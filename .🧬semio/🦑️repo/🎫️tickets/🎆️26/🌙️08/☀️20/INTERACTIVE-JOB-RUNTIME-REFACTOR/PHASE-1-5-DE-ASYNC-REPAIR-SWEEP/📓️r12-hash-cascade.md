# R12 — Hash Crate De-Async + Cascade (Phase 1.5 final repair)

## Target

`🧰️framework/🔨️modules/#⃣hash/🦀️component.rs` (`semio-framework-hash`, 29 errors → 0).

All ten functions were `async fn` with zero real suspension points: five pure blake3/merkle/float
helpers (`hash_parts`, `hash_bytes`, `format_number_for_hash`, `merkle_node`, `merkle_collection`)
and five `#[test] async fn` (illegal Rust — never compiled, never ran). Fix: dropped `async` from
all ten, dropped the one `.await` (`merkle_collection` → `merkle_node`), switched the five tests
from `#[test] async fn` to plain `#[test] fn` (not the `async_test` harness — the functions
themselves are correctly non-suspending, so a plain synchronous test is the honest fix).

## De-async list (functions whose signature changed)

| Function | File |
|---|---|
| `hash_parts`, `hash_bytes`, `format_number_for_hash`, `merkle_node`, `merkle_collection` | `#⃣hash/🦀️component.rs` |
| `MediaFingerprint::of` | `🛂️manifest/🦀️component.rs` |
| `os_extension::content_hash` | `🧩️extension/🦀️component.rs` |
| `backbone_pack_hash` (`#[cfg(not(target_arch="wasm32"))]`) | `🏪️store/🔄️sync/🦀️component.rs` |

Everything else touched kept its existing `async` signature because it still had at least one
other genuine `.await` in its body after the hash-related one was dropped (`DepHash::chain`,
`infer_field_after_diff`, `install_extension_package`, `FolderSqliteStorage::put`,
`InferenceCache`/session driver functions, the three `media_fingerprint` trait-default impls in
`🔌️plugin/🦀️component.rs`, etc.) — the cascade only propagates when a function's *only* `.await`
was the one removed, and in this crate's real call graph that happened just three times beyond the
hash crate itself.

## Call sites touched, by crate

**`semio-framework`** (top-level; also covers `semio-framework-os-kernel`'s `#[path]`-included
copies of the same files):
- `🛂️manifest/🦀️component.rs:5139` — dropped `.await` on `hash_parts`, de-asynced `MediaFingerprint::of` (its only await); dropped `.await` at its 4 test call sites and converted `media_fingerprint_structured_hashes_json_binary_reuses_blob_hash` from `#[semio_framework_async_macros::async_test] async fn` to plain `#[test] fn`.
- `🔁️workflow/🦀️component.rs:2556-2573` — pre-existing, hash-unrelated bug uncovered by reachability (see below): `media_contract_to_record`/`media_contract_from_record`/`workflow_media_port_to_record`/`workflow_media_port_from_record` are already plain `fn` (de-asynced by an earlier packet) but two tests still `.await`ed them. Dropped the stray `.await`s; both test functions keep `async fn` because they still genuinely await `media_port_spec`/`placeholder_media_contract`.
- `💡️inference/🦀️component.rs:48,311` — dropped `.await` on `merkle_node`/`merkle_collection`; both enclosing functions (`DepHash::chain`, `infer_field_after_diff`) keep `async` (other real awaits remain: `hex::encode`/`hex::decode_to_slice` calls, `diff.touches()`, `infer_field`, etc.).
- `🧩️extension/🦀️component.rs:256-257` — de-asynced `content_hash` (its only await); fixed its 3 test call sites (`content_hash_is_stable_blake3`, `pack_unpack_verify_round_trip`).
- `🔌️plugin/🦀️component.rs:10108,14244,14347` — dropped `.await` on 3 identical `MediaFingerprint::of(&media)` call sites inside the `media_fingerprint` trait-default methods; each stays `async fn` (still awaits `Self::export_media`).
- `🖥️host/🦀️component.rs:1103` (`install_extension_package`) — dropped `.await` on `store::extension::content_hash`; corrected the docstring's now-false "every store::extension fn it calls is async" claim; function stays `async` (still awaits `verify`/`extends_matches_primary_dependency`).

**`semio-framework-os-mcp`** (also part of `semio-framework-os-kernel`'s tree):
- `🏪️store/🔄️sync/🦀️component.rs:335` — de-asynced `backbone_pack_hash` (its only await was `hash_bytes`); fixed its 3 call sites (`setup`, `persist_write`, `handle_external_change` — all keep `async`, other awaits remain).
- `🏪️store/🔄️sync/🦀️component.rs:2252` — dropped `.await` on `hash_bytes` inside `FolderSqliteStorage::put`; stays `async` (still awaits `self.connection()`).

**No change needed** (already written assuming synchronous hash functions — presumably prepared
ahead of the eventual fix, and simply invisible until now because the hash crate blocked
compilation of everything downstream): `🏃️run/🦀️component.rs` (5 call sites), `🌉️mcp/🏠️workspace/🦀️component.rs:308`, `✏️s/🔌️plugins/🎞️animate/…/🎥️video/🦀️component.rs`, `…/🎛️config/🦀️component.rs`, `✏️s/🔌️plugins/🧩️puzzle/…/🎛flat-position/🦀️component.rs`. `🖱️ui/🖼️render/element.rs`'s `fxhash_bytes` and `🛢️db/🔘️state/🦀️component.rs`'s `hash_bytes` are unrelated same-named local functions (fxhash / a local blake3 wrapper), not this crate — confirmed by reading both bodies before touching anything (the exact "generic name, don't blind-replace" risk the brief called out).

Total genuine hash-cascade edits: **6 files**, **~14 call sites** + the 10 functions in the hash
crate itself + 1 stale docstring. The originally estimated ~86 call sites turned out to already be
either already-fixed (run/component.rs, mcp/workspace, animate, puzzle) or non-existent duplicates
of the coordinator's raw grep count; the real edit surface after excluding comments/unrelated
same-named functions was much smaller.

## Where the cascade stopped, and why

The cascade is bounded by "does the enclosing function still have another real `.await`". In this
codebase almost every hash call sits inside a larger async function that also does real async I/O
(file reads, sqlite connections, actor sends) or calls other still-legitimately-async helpers, so
after dropping the hash-related `.await` the enclosing function almost always still compiles as
`async`. Only 3 functions outside the hash crate itself had the hash call as their *sole* await:
`MediaFingerprint::of`, `os_extension::content_hash`, `backbone_pack_hash`. None of their own
callers had the hash call as *their* sole await either, so the cascade terminated after one hop in
every branch — it never needed to propagate a second level outward.

## Unrelated pre-existing breakage uncovered by reachability (NOT part of this cascade, NOT fixed)

`cargo check -p X` only type-checks `X` after all of `X`'s dependencies build successfully. While
`semio-framework-hash` failed, nothing that depended on it (directly or transitively) was ever
type-checked, so several crates have accumulated their *own*, hash-unrelated compile errors that
only became visible once hash (and everything gated behind it) started compiling. Confirmed by
`grep` that none of these reference any hash-crate function:

- **`semio-s-plugin-stdio`** — 5,545 errors across dozens of `🗿️artifacts/*/🧬️schema/…` files
  (dwg, gltf, pdf, docx, png, semio-graph subsets, etc.) — mostly `impl Future<Output=T>` vs `T`
  mismatches, i.e. the same general "wrongly-async" bug class but an entirely separate, much larger
  body of work than this packet's scope. Blocks `semio-framework-os-run`, `semio-s-plugin-animate`,
  and `semio-s-plugin-puzzle` from compiling even though none of their own hash call sites have any
  remaining issue (verified: zero errors reference `🏃️run/🦀️component.rs`, the animate `🎥️video`/`🎛️config` files, or the puzzle `flat-position` file in the full error logs for those three crates).
- **`semio-framework-os-mcp`** — 22 remaining errors in `🏠️workspace/🦀️component.rs`,
  `🔀️dispatch/🦀️component.rs`, `🧭️protocol/🦀️component.rs`: missing types (`AppCommand`, `AppFrame`,
  `Fault`, `SearchHit`, `RevisionStamp`, `PreparedActionReport`, `InvocationReport`,
  `ArtifactChannel`) and several `dyn_enum_close!`-generated methods reported as
  "should be async or return a future, but it is synchronous" — an async/sync trait-impl mismatch
  of the same general bug class, but against a trait/types this packet never touched and that
  aren't reachable from any hash call site.
- **`semio-s-imperative`** — 6 errors: async-trait-method signature mismatches
  (`ContributedExtensionStub::evaluate`, `Step::id`) and two `E0733` "recursion in an async fn
  requires boxing" in `📇️registry` and `⚙️engine`. No hash reference.
- **`semio-framework-os-infinite`** — 927/1228 errors, not investigated in depth (clearly its own
  large pile, same shape as stdio); no hash reference confirmed by grep.

These four are recommended as separate, dedicated follow-up tickets (same treatment R1/R8 gave
`semio-framework-ui`/`semio-framework-machine`), not folded into R12.

## Test results

`cargo test -p semio-framework-hash` (debug) and `--release`: **5/5 pass**, both profiles. These
tests had never compiled before (illegal `#[test] async fn`), let alone run; all five passed on
first execution with no implementation bugs found — no honest-failure diagnosis was needed.

`cargo test -p semio-framework --lib`: 160/160 pass, including the edited
`workflow::tests::media_contract_dsl_round_trips`/`workflow_media_port_dsl_round_trips` and
`manifest::media_vocabulary_tests::media_fingerprint_structured_hashes_json_binary_reuses_blob_hash`.

`cargo test -p semio-framework-os-kernel --lib`: 779/779 pass, including
`os_extension::tests::content_hash_is_stable_blake3`, `pack_unpack_verify_round_trip`, and all 18
`inference::` tests (the `DepHash`/`InferredField` cache-transparency suite).

Release-mode re-runs of both crates were launched; see follow-up note if not yet landed by hand-off.

## Wasm targets

`semio-framework-hash` and `semio-framework-os-kernel` (`--lib`, the wasm-safe document model —
`[[bin]]` targets `pack`/`spr` are native-only by design and correctly fail on wasm32 for unrelated
reasons, gated-out `cli` modules) both check clean (warnings only) on `wasm32-unknown-unknown` and
`wasm32-wasip2`. `semio-framework --lib` (top-level) also checks clean on `wasm32-unknown-unknown`.

## Dependency ratchet

`bun ./📜️script.ts verify dependencies` → `baseline: 238; current: 238` — clean, no new
third-party dependencies.

## Phase gate

`cargo check --workspace --all-targets 2>&1 | grep "could not compile"` (no `--keep-going`, matching
the mandated command exactly) currently returns:

```
error: could not compile `semio-compose-rs` (lib) due to 18 previous errors; 89 warnings emitted
error: could not compile `semio-compose-rs` (lib test) due to 36 previous errors; 160 warnings emitted
error: could not compile `semio-framework-os-infinite` (lib) due to 927 previous errors
error: could not compile `semio-framework-os-infinite` (lib test) due to 1228 previous errors; 19 warnings emitted
error: could not compile `semio-s-imperative` (lib) due to 6 previous errors
error: could not compile `semio-s-imperative` (lib test) due to 6 previous errors; 1 warning emitted
```

`semio-compose-rs` is the expected, explicitly-out-of-scope entry. `semio-framework-os-infinite` and
`semio-s-imperative` are the pre-existing, hash-unrelated breakage described above, now visible for
the first time. Because `cargo check --workspace` (without `--keep-going`) stops scheduling new
units after a failure, this single run never reached `semio-s-plugin-stdio`,
`semio-framework-os-mcp`, `semio-framework-os-run`, `semio-s-plugin-animate`, or
`semio-s-plugin-puzzle` — their errors (documented above from dedicated `-p` runs) are real but did
not surface in this particular ordering of the workspace-wide command. The hash crate's own
cascade — everything this packet was actually scoped to — is complete and verified; the phase gate
is not fully green only because of the four newly-unmasked, pre-existing crates above, none of
which reference any hash-crate function.

## Lessons for the repo-wide de-async codemod

1. **Reachability masking compounds.** Every crate that depends (even transitively) on a crate with
   compile errors is *completely* unchecked, not partially — its own, unrelated errors accumulate
   invisibly. Fixing the "last" visible crate in a workspace check reliably unmasks 1+ more "last"
   crates; budget for this recursively rather than assuming a fix is final because the gate command
   currently shows only the expected entry.
2. **The cascade is usually much shallower than raw grep-count suggests.** ~86 raw textual matches
   of the five function names implied a large blast radius; the real edit surface was ~14 call
   sites because (a) most call sites live inside functions with other genuine awaits, so dropping
   one `.await` doesn't force further de-asyncing, and (b) several call sites were already written
   correctly (no `.await`) ahead of time, apparently anticipating this exact fix, and were
   themselves invisible-but-broken until the hash crate compiled.
3. **Generic function names are a real landmine.** `hash_bytes` and `merkle_node`-shaped names
   recur as unrelated local helpers (fxhash wrapper, a separate blake3 wrapper returning a
   different `ContentHash` type) in at least two files touched by this packet's grep sweep. Always
   read the callee's actual definition before editing a call site with a generic name — never
   trust the name alone, even under time pressure.
4. **`cargo check --workspace` without `--keep-going` under-reports.** It stops scheduling after
   the first crate failure it reaches, so a single run's "could not compile" list is not the full
   set of broken crates — it's whatever cargo's build-graph scheduler happened to reach first
   (order is not fully deterministic under parallel scheduling). Any repo-wide codemod's "are we
   done" check should use `--keep-going` (or iterate `-p` over every workspace member) to get a
   true picture, even though the phase gate as specified here intentionally uses the plain command.
5. **`#[test] async fn` is a silent no-op, not a loud error, in the wrong context** — Rust actually
   does reject it at compile time (E0752-adjacent "async functions cannot be used for tests"), but
   because it sits inside a crate whose *other* functions already had real compile errors, the
   test-illegality error was just one more line in a wall of 29, easy to miss when triaging by
   error count alone. Grep for `#[test]\s*\n\s*async fn` repo-wide as a cheap independent check
   before assuming a crate's tests "just need the dependency fixed."

## Files touched

- `🧰️framework/🔨️modules/#⃣hash/🦀️component.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
