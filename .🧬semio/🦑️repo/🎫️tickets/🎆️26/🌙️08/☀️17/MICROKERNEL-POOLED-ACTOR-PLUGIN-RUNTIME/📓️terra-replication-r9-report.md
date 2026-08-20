# 📓️ terra-replication-r9 — report

**Verdict: R9 does NOT apply to `store::ByteReader`/`ByteWriter` as scoped. Changes made, verified
against a real compiler error, then REVERTED COMPLETELY.** This is the packet's own stated success
condition ("a clean revert plus a written reason is a SUCCESS for this packet"). All edits are gone
— `git status --porcelain` on every touched path returns empty, confirmed as the last action before
writing this report.

## The blocker, stated precisely

`store::ByteReader`/`ByteWriter` (`🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`) are
consumed — among ~200 files repo-wide (106 in `✏️s/🔌️plugins/🗄️stdio/**`, ~16 in other framework/
os-kernel modules, the remainder scattered across other plugin families) — by **two production
functions in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`**:
`variants_binary::encode_op` (line 366) and `variants_binary::decode_op` (line 372). That file sits
under `🗣️dsl/**`, which the standing rules name explicitly: *"🗣️dsl/\*\* and 💡️inference/\*\* belong
to a peer interactive session — never edit them."*

`decode_op`/`encode_op` are **not** gated by `#[cfg(test)]` or any feature/target cfg — they compile
unconditionally as part of `semio-framework-os-kernel --lib` (confirmed: the real compiler error
below was produced by a plain `--lib` check, not `--all-targets` or `--tests`). Making `ByteReader`/
`ByteWriter` sync is therefore not a "consumer can become async instead" situation (R9 rule 3) — the
consumer already **is** async and merely calls the now-sync fn with a now-illegal `.await`, a
one-line fix (`ByteReader::new(bytes).await` → `ByteReader::new(bytes)`) that I am contractually
forbidden from making. There is no third option: a fn is either `async` (awaitable, and every one of
~200 caller sites needs `.await`) or not (never awaitable) — nothing bridges the two without editing
every caller, and this one caller is off-limits.

## Genuine attempt, with real evidence (not just static reasoning)

I did not stop at the static read. I made the edit, chased the fallout through two full layers of
the dependency graph with the shared `remove-bad-await.py` tool, and got rustc to actually say it:

1. **Edited the target file**: stripped `async` from all 59 fns/methods (`zigzag_encode/decode`,
   `write_varint_u64/i64`, `read_varint_u64/i64`, `is_minimal_varint`, `ByteReader`'s 11 methods,
   `ByteWriter`'s 10 methods, the `CompressionCodec` trait + `NoCompression`/`DeflateCodec` impls,
   `crc32c`, `deflate_compress`/`deflate_decompress`), tagged each with
   `// 🚫️async: E1 ... — see R9`, and stripped the now-invalid `.await` from the file's own 62
   internal call sites (all in its `#[cfg(test)]` module, all the uniform `).await` shape, verified
   safe before touching via `grep -o` pattern-uniqueness check).
2. **`cargo check -p semio-framework-os-kernel --lib`** → broke inside `semio-framework-replication`
   itself first: **47 `E0277` "is not a future"** across `🔢️scalar` (32), `📐️format` (13), `🧾️wire`
   (2) — all in-scope siblings within `📡️replication`, not forbidden. Ran
   `remove-bad-await.py --crate semio-framework-replication --scope 📡️replication --apply` →
   fixpoint in 2 passes, 46 removed mechanically (the 47th was a `#[cfg(feature = "deflate")]`
   branch invisible to a default-features check; found and hand-fixed once `--all-features` was run,
   plus a matching test-only site — 2 one-line manual fixes, `📐️format/🦀️component.rs:374,895`).
   `cargo check -p semio-framework-replication --lib --all-features` → **EXIT 0**.
3. Next layer: **`semio-framework-pack`**, 42 more `E0277` sites, all in its own `📐️format` module
   (in-scope, not forbidden). Same tool, same scope pattern → fixpoint in 2 passes, 42 removed.
   `cargo check -p semio-framework-pack --lib --all-features` → **EXIT 0**.
4. Next layer: **`semio-framework-os-kernel --lib` itself → 228 `E0277` sites**:

   | file | sites | in scope? |
   |---|---:|---|
   | `📡️spr/📜️history/🦀️component.rs` | 94 | yes |
   | `🎒️pack/🔢️value/🦀️component.rs` | 84 | yes |
   | `🏪️store/🦀️component.rs` | 41 | yes |
   | `📡️spr/🔌️io/🦀️component.rs` | 4 | yes |
   | `📡️spr/💎️materialize/🦀️component.rs` | 3 | yes |
   | **`🗣️dsl/🦀️component.rs`** | **2** | **NO — forbidden** |

   The real compiler output for the two blocking sites:

   ```
   error[E0277]: `()` is not a future
      --> …/🗣️dsl/🦀️component.rs:366:52
   366 |         write_varint_u64(&mut out, ordinal as u64).await;
       |         ------------------------------------------ ^^^^^ `()` is not a future
   help: remove the `.await`

   error[E0277]: `protocol::ByteReader<'_>` is not a future
      --> …/🗣️dsl/🦀️component.rs:372:49
   372 |         let mut reader = ByteReader::new(bytes).await;
       |                          ---------------------- ^^^^^ `protocol::ByteReader<'_>` is not a future
   help: remove the `.await`
   ```

   Both are trivially fixable in isolation (rustc even hands the exact one-line fix), and both are
   in the one file I am not allowed to touch. The other 226 sites were left unfixed — no point
   burning further time on them once the unconditional blocker was confirmed with a real diagnostic,
   since the revert is mandatory regardless of how tractable the rest is.

## Revert — verified complete

Restored all 5 touched files (`⚙️codec`, `📡️replication/📐️format`, `📡️replication/🔢️scalar`,
`📡️replication/🧾️wire`, `🎒️pack/📐️format`) from `git show HEAD:<path>` byte-for-byte (via `cp`, not
`git checkout` — no git-modifying command was used per the standing rule). `git status --porcelain`
on `📡️replication`, `🎒️pack`, and `🗣️dsl` all return **empty** — confirmed as the literal last
action before this report was written, and again just now for this line.

Post-revert, freshly re-measured (R14, target named on every line):

| check | result |
|---|---|
| `cargo check -p semio-framework-os-kernel --lib` | **EXIT 0** |
| `cargo check -p semio-framework-os-kernel-db --lib` | **EXIT 0** |
| `cargo test -p semio-framework-os-kernel --lib` | **779 passed / 0 failed** |
| `cargo test -p semio-framework-os-kernel-db --lib` | **424 passed / 0 failed** |
| `cargo check -p semio-framework-os-kernel --lib --target wasm32-unknown-unknown` | **EXIT 0** |
| `cargo check -p semio-framework-replication --lib --all-features` | **EXIT 0** |
| `cargo check -p semio-framework-pack --lib --all-features` | **EXIT 0** |

Regression baseline holds on every gate I could independently reach.

## Gates I could NOT independently reach — a pre-existing, unrelated blocker, not mine

`cargo test -p semio-framework-plugin-host --lib`, `cargo check -p semio-framework-plugin --lib` /
`--all-features`, `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features
component-guest`, and `cargo check -p semio-s-plugin-stdio --lib` **all fail identically**, before
and after my (now fully reverted) edit, with:

```
error[E0432]: unresolved import `semio_framework_ui_contract`
   --> 🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:873:9
873 | pub use semio_framework_ui_contract::{UiPatch, UiPatchOp};
error: could not compile `semio-framework` (lib) due to 1 previous error
```

This is `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`, an unrelated file, currently **live and
uncommitted** (`git status` shows `M`, 38 lines changed on top of the last commit `cb9bcce7a4`,
file mtime 12:19:23 — ~7 minutes old when first observed, unchanged since, confirmed again at
12:35). I never touched this file, `semio-framework-plugin`/`plugin-host`/`stdio` all depend on the
top-level `semio-framework` facade crate (verified via each `Cargo.toml`), and this break was
present in a **preflight check I ran before making any edit at all** — it is not something my
packet caused or can fix (out of scope, another session's in-progress work; the ticket's own
"Concurrent Cargo Workspace Churn" pattern). **These four gates are therefore reported as
"unmeasurable right now," not as pass or fail** — re-run them once that peer edit lands or is
reverted.

## stdio own-error count — BEFORE / AFTER

Cannot be freshly measured either direction, for the same reason: `cargo check -p
semio-s-plugin-stdio --lib` hits the identical `semio_framework_ui_contract` blocker pre-edit and
post-revert alike. Since my change is a complete no-op relative to `git HEAD` (confirmed clean),
stdio's true count is **unaffected by this packet** — it remains at the sibling `stdio-await`
packet's last independently-measured **18,591** (itself flagged there as a likely modest overstate,
171 parse-fix sites applied after that measurement, never re-verified — see
`📓️terra-stdio-await-report.md`). **No payoff was realized because the fix was reverted; report
18,591 → 18,591 (unchanged, by construction of the revert), not a new number.**

## Dropped-future census on replication (R12/R17), forced rebuild

```
cargo clean -p semio-framework-replication   # 1031 files, 616.2 MiB removed
cargo check -p semio-framework-replication --lib --all-features   # EXIT 0
grep -c "unused implementer of" <output>   # 0
```

**0 dropped futures**, matching the pre-existing (unchanged, since fully reverted) state.

## Why the earlier DictReader/prev_frame burn and this one are different failure shapes

Worth recording since both are now data points for future R9 candidates in this crate: the earlier
burn was a **depth** problem (~220 sites deep, cascading because intermediate consumers were
themselves consumed further up by something language-barred). This one was NOT a depth problem — the
fallout from `ByteReader`/`ByteWriter` alone was shallow and entirely mechanical (46 + 42 + 226 = 314
sites across 8 files, all fixed or fixable in 2 tool passes each, zero hand-judgement needed, zero
E0382/corruption residue). It was a **single specific forbidden-file consumer** problem instead —
the chain never got deep, it just touched one file I am not allowed to edit. Both are valid reasons
to revert; they should not be conflated when a future packet asks "is this the same kind of trap."

## Recommendation

The structural fix genuinely would work — `write_varint_u64(&mut out, ordinal as u64).await;` →
`write_varint_u64(&mut out, ordinal as u64);` and `ByteReader::new(bytes).await` → `ByteReader::new(bytes)`
are the ONLY two edits standing between this R9 revert and a clean `semio-framework-os-kernel`
build (226 further in-scope framework sites are separately fixable in the same pass with the same
tool, already dry-run-verified tractable). **Whoever owns `🗣️dsl/**` needs to either make those two
one-line changes themselves, or explicitly release those two lines from the no-edit restriction for
a follow-up packet.** Once either happens, re-running this exact packet (same file, same method,
tools already left in the ticket folder from this run's use of them) should land cleanly and,
per the stdio packet's own estimate, eliminate a large fraction of stdio's remaining ~18,591 errors.

## Files touched (production) — NONE remain modified

All reverted: `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`,
`🧰️framework/🔨️modules/📡️replication/📐️format/🦀️component.rs`,
`🧰️framework/🔨️modules/📡️replication/🔢️scalar/🦀️component.rs`,
`🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️component.rs`,
`🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️component.rs`. `🗣️dsl/**` was never edited.

## Ticket-folder artifacts from this packet

- `terra-replication-r9-codec-ORIGINAL-backup.rs.txt` — pristine backup of the target file taken
  before any edit (redundant with `git show HEAD:`, kept for convenience).
- Raw check/test logs: `terra-r9-preflight-*.txt`, `terra-r9-experiment-oskernel*.json/.stderr.txt`,
  `terra-r9-repl-allfeat.txt`, `terra-r9-pack-allfeat.txt`, `terra-r9-final-*.txt`,
  `terra-r9-census-repl.txt`, `terra-r9-stdio-before.json/.stderr.txt`.
- `remove-bad-await.py` was reused as-is from the ticket folder (already present from a prior
  packet) — not modified, not re-authored.
