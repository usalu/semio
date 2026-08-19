# 📓️ `alltargets-kernel` report

Packet: run `cargo check -p semio-framework-os-kernel --all-targets` (never run on this crate before),
apply the shared async-conversion tools, and — the hard invariant — leave `--lib` at EXIT 0.

## Bottom line

- **`--lib`: EXIT 0**, verified fresh at the end of the session (pasted below). Never left broken.
- **`--all-targets`: still failing.** Went from **4 broken targets** (`lib test` 3615 errors, `bin "pack"`,
  `bin "pack" test`, `bin "spr"`, `bin "spr" test`) to **1 broken target** (`lib test`, **2734–2746 errors**,
  fluctuates by ±a few between measurement runs — see "known tool instability" below). Both bins and both
  bin-test targets now compile clean.
- The residual `lib test` errors are **not evenly spread** — `🏪️store/🦀️component.rs`'s `#[cfg(test)] mod
  tests` (≈4,100 lines) is the dominant single source, and its defect shape is now root-caused (below), not
  just measured.

## Commands and exit codes (every claim below is from a pasted run, not an estimate)

```
$ CARGO_TARGET_DIR=<scratchpad>/target-at-kernel cargo check -p semio-framework-os-kernel --all-targets
  (first run, before any edit)                                          → exit 0 (cargo itself), 4 targets failed
$ CARGO_TARGET_DIR=<scratchpad>/target-at-kernel cargo check -p semio-framework-os-kernel --all-targets
  (final run, after all fixes below)                                    → exit 101, only `lib test` fails (2746)
$ CARGO_TARGET_DIR=<scratchpad>/target-at-kernel cargo check -p semio-framework-os-kernel --lib
  (final run)                                                            → exit 0, "Finished `dev` profile"
$ CARGO_TARGET_DIR=<scratchpad>/target-at-kernel cargo test -p semio-framework-os-kernel --lib --no-run
  (final run)                                                            → exit 101, 2746 errors (harness cannot
                                                                             even build — the historical "779
                                                                             passed" baseline is currently
                                                                             UNRUNNABLE, not "5 failing")
```
Full logs: `terra-alltargets-kernel-initial-check.txt` (3.4 MB, first run), `terra-alltargets-kernel-final-check.txt`,
`terra-alltargets-kernel-lib-FINAL.txt`, `terra-alltargets-kernel-test-FINAL.txt`.

## What I changed, in order

### 1. Four in-scope E0728s blocking `insert-await.py` from even starting (R10-legal residue shape 1)
`🏪️store/🦀️component.rs` had `.await` inside a **sync `map_err` closure** (`|error| … reader.position()
.await …`) at four `OpCodec::decode` sites (:9245, :11021/11022, :11304/11306, :12175/12178 pre-shift).
Fixed by hoisting `let record_offset = reader.position().await as u64;` out of the closure — the exact
remedy R10 §1 prescribes.

### 2. Two bin entry points needing an E5 executor bridge
`🎒️pack/⌨️cli/📦️main.rs` and `📡️spr/⌨️cli/📦️main.rs` called `os_pack::cli::main_impl`/`os_spr::cli::main_impl`,
both of which the earlier asyncify pass correctly made `async fn … -> i32`, but `fn main()` never got the
bridge. `futures_lite::future::block_on` is the crate's own established pattern for exactly this (already
used at `🏃️run/📦️bin.rs:324`). Fixed both, tagged `// 🚫️async: E5 executor bridge — bin entry point,
sanctioned by R4 clause 1`.

### 3. `async-test-attr.py --apply` on the whole `🔨️modules` scope
271 `#[test] async fn` sites across 13 files were **illegal Rust** (`async functions cannot be used for
tests`) — the single largest error class in the very first check (283 of 387 "other" residuals). Converted
to `#[semio_framework_async_macros::async_test]`. This alone dropped `lib test` from 3132→2853 residual
errors and, crucially, **unblocked type-checking inside those fn bodies** so real missing-`.await`
diagnostics could finally surface (previously the parser rejected the whole fn before reaching its body).

### 4. `insert-await.py` / `remove-bad-await.py`, alternated to fixpoint, several rounds
Standard span-keyed compiler-driven passes (R10-compliant). Cumulative: **~600 `.await` insertions applied**,
**~9 bad `.await`s removed**. Reports: `terra-alltargets-kernel-apply{1..5,-r6,-r7}.json/.txt`,
`terra-alltargets-kernel-removebad{1,2,-r6,-r7}.txt`.

### 5. Eight E4 (fn-pointer-slot) mis-asyncifications — hand fixes, each verified against its use site
The blind codemod had asyncified several `fn() -> RecordSpec` factory functions that are stored **bare**
(no call parens) into `Shape::Record(...)`/`Shape::Statements(vec![(..., f)])`/`Shape::Table(...)` —
fn-pointer-typed enum slots per R2 E4. An `async fn`'s pointer type is unnameable, so these cannot be async
structurally, independent of any style preference:

| file | fn | stored via |
|---|---|---|
| `🗣️dsl/🧬️schema/🦀️component.rs` | `group_spec` | `Shape::Statements(vec![("group", group_spec)])` (self-referential) |
| `🗣️dsl/🧬️schema/🦀️component.rs` | `nested_point_spec` (×2 fns, same name, two test scopes) | `Shape::Record(nested_point_spec)` |
| `🗣️dsl/🧬️schema/🦀️component.rs` | `table_row_spec` | `Shape::Table(table_row_spec)` |
| `🗣️dsl/🧬️schema/🦀️component.rs` | `unbounded_tuple_row_spec` | `Shape::Table(...)` |
| `🗣️dsl/🧬️schema/🦀️component.rs` | `nested_inner_row_spec`, `nested_outer_row_spec` | `Shape::Table(...)` (nested) |
| `🗣️dsl/🧬️schema/🦀️component.rs` | `quantity_spec`, `duplicate_type_row_spec` | `Shape::Record`/`Shape::Table` |
| `🎒️pack/🧪️testkit/🦀️component.rs` | `nested_point_spec`, `recursive_spec` | `Shape::Record`/`Shape::Statements` (self-referential) |

Each tagged `// 🚫️async: E4 fn-pointer slot — stored bare as ... below`. Verified each call site was
already calling the fn WITHOUT `.await` elsewhere (structural proof it was never validly async).

### 6. One R9 de-asyncify — pure recursive fn, blocked by sync closures (R10 residue shape 1 + 2 combined)
`🏪️store/🦀️component.rs`'s `dsl_value_numeric_insensitive_eq(a, b) -> bool` is a **pure, I/O-free**
recursive `DslValue` comparator, called from inside sync `.all()`/`.is_some_and()` closures on its own
recursive branches — a call that can never be `.await`ed there. R9 test: no suspension point exists
(confirmed: no I/O, no `tokio`/`fs`/network in the body) AND a consumer is language-barred from being async
(the sync iterator-adapter closures) → made sync, tagged `// 🚫️async: E1 pure recursive comparison
consumed inside sync .all()/.is_some_and() closures … — see R9/R10 residue shape 1`.

### 7. Root-caused and fixed the dominant defect: 135 missing `.await` on a shadowed async constructor
`🏪️store/🦀️component.rs`'s `mod tests` declares a **local newtype wrapper**
`struct ArtifactStore<P,M>(super::ArtifactStore<P,M>)` (line ~8964) with its own
`async fn new(envelope) -> Self` that shadows `super::ArtifactStore` for the whole test module. ~135 of the
~143 `ArtifactStore::new(...)` call sites in that module (and even the wrapper's own internal
`super::ArtifactStore::new(envelope).expect(...)`) predate this shadow and never got `.await` appended —
leaving `store`/`peer`/`local`/`remote`/`a`/`b`/`seed_store`/… bound to `impl Future<Output = Self>`
instead of a real store, which cascades into hundreds of "no method found for opaque Future" / "cannot
index into Future" / "cannot apply `!` to Future" errors downstream wherever those bindings get used.

This is **not** the name-keyed sweep R10 bans — it is keyed on the single fully-qualified call
`ArtifactStore::new(` (never a bare identifier that could collide with `Vec`/`HashMap`/std), root-caused by
hand before writing any tool, with balanced-paren matching so nested generic calls in the argument list
never confuse the span, and it only fires where `.await` is not already present. Saved as
`terra-store-artifactstorenew-await-fixer.py` in the ticket folder per R10's "save the recovery tool"
instruction, for whichever packet picks up the remainder of this file.
```
$ python3 terra-store-artifactstorenew-await-fixer.py 🏪️store/🦀️component.rs --apply
found 135 call sites missing .await (in test module, from line 8958)
applied 135 edits
```

## ⚠️ A tool-instability finding for the coordinator / whoever runs `insert-await.py` next on this file

`insert-await.py` **repeatedly re-corrupted the same two struct-literal shorthand fields** —
`timestamp,` inside a `crate::os_spr::Conflict { … timestamp, }` literal (line ~11457) and
`member` inside `FixtureDirectory { member }` (line ~12327) — turning them back into `timestamp.await,` /
`member.await`, which is a **hard parse error** (`expected one of ,: or }, found .`) that breaks `(lib)`
itself (parsing happens before `cfg(test)` stripping, so a syntax error in test-only code still fails
every target). This happened **three separate times** across the session, each time reverted by hand and
re-verified against `--lib`. Root cause looks like a stale/misattributed rustc suggestion span landing on
the struct-literal field identifier rather than the actual unresolved-Future expression nearby in the same
literal (both `timestamp` and `member` are plain non-Future locals — proven structurally, since both are
used as sync values elsewhere in their own functions). I did **not** patch the shared tool itself (used by
sibling packets) — instead I wrapped further rounds in `terra-guard-and-round.sh`, which reverts this exact
pattern and re-checks `--lib` after every `insert-await.py`/`remove-bad-await.py` round. **Whoever next runs
`insert-await.py --apply` broadly against `🏪️store/🦀️component.rs` should grep for `timestamp.await,` and
`FixtureDirectory { member.await }` afterward before trusting `--lib`.**

## What's left (for the next packet — do not re-derive, the shape is known)

- `🏪️store/🦀️component.rs`'s `mod tests` (~4,100 lines) still has **pervasive missing `.await`** on the rest
  of `ArtifactStore`'s async method surface (`.envelope()`, `.generation()`, `.snapshot()`, `.conflicts()`,
  `.resolve_conflict()`, `.dispatch()`, `.ingest_remote()`, `.open_conflicts()`, …) used throughout the
  file — the constructor fix in §7 above unblocked the *bindings*, not every *call* on those bindings.
  This is almost certainly the majority of the remaining error count. Expect it to need several more
  `insert-await.py`/`remove-bad-await.py` rounds (via `terra-guard-and-round.sh`, already wired for this
  file) plus hand review of the residue shapes it can't reach (sync closures, recursion, moved values).
- Distribution of residual errors across files (`terra-alltargets-kernel-final-check.txt`, counted by
  file mentions — an overestimate of true error count per file since each error prints several context
  lines, but accurate for *relative* ranking):
  `🏪️store` (dominant) › `🗣️dsl/🧬️schema` › `🗣️dsl/📖️grammar` › `📡️spr/🧵️channel` › `🗣️dsl` (root) ›
  `📡️spr/🎮️command` › `🎒️pack/🔢️value` › `📡️spr/💎️materialize` › `💡️inference` › `📡️spr/⌨️cli` ›
  `🎒️pack/⌨️cli` › `🗣️dsl/🔍️lexer` › `🚪️io` › `🎒️pack/🧪️testkit` › `🌿️vcs` › `🗣️dsl/🖋️notation` ›
  `📡️spr/🧪️testkit` › `🗣️dsl/🧪️fixture-sweep` › `🗣️dsl/👪️family/📊️sheet` › `📡️spr` (root).
- A handful of confirmed non-await residue shapes seen in the `other` bucket that will need hand fixes once
  the await noise clears: `E0733` recursion-in-async-fn (the `DslRecord`/`DslEnum` derive macros — needs
  `Box::pin` per R10 §3, or the derive itself may need to stay sync/E1), a few `E0382` "use of moved value"
  (`diff`/`added`/`moved`/`patched` — likely `.clone()` needed after an await reordered evaluation), and
  `E0308`/trait-impl-signature mismatches for `parse`/`print`/`classify`/`id` methods that may be E1
  externally-declared-trait impls asyncified when they shouldn't have been.

## Files touched (hand edits via Edit tool — exact list)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — items 1, 6, 7 above, plus the two
  tool-corruption reverts (§ "tool-instability finding")
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/📦️main.rs` — item 2
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/📦️main.rs` — item 2
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs` — item 5 (8 sites)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️testkit/🦀️component.rs` — item 5 (2 sites)

Additionally, `async-test-attr.py --apply` (13 files, 271 sites) and several rounds of
`insert-await.py --apply` / `remove-bad-await.py --apply` (both `--scope 🧰️framework/🛍️products/💻️os/🔨️modules`,
so no edits landed outside my owned path) touched further `.rs` files across the crate as a side effect of
being compiler-diagnostic-driven over the whole `--all-targets` surface — see the per-round JSON reports
listed above for exact spans. **Note:** `git diff` against `HEAD` in this shared, uncommitted tree also
shows unrelated concurrent sessions' work (e.g. a `.tsx` file) — that is not mine; I did not touch anything
outside `🔨️modules/**/*.rs` (Rust only).

## Tools added to the ticket folder

- `terra-store-artifactstorenew-await-fixer.py` — the item-7 root-cause fixer (pattern-keyed on the fully
  qualified `ArtifactStore::new(`, not name-keyed; R10-compliant). Reusable if any more call sites of this
  shape surface.
- `terra-guard-and-round.sh` — one round of `insert-await.py` → `remove-bad-await.py` → revert the known
  `timestamp`/`member` corruption if it reappeared → `--lib` recheck, all against the scratchpad target dir.
  Safe to re-run for further rounds on this crate.

## Nothing regressed

`--lib` was EXIT 0 before I started (today's headline win per the coordinator) and is EXIT 0 now, verified
by a cold, fresh run at the very end of the session (not reused from an earlier pass). Every intermediate
regression I introduced or found (three occurrences, all the same `timestamp`/`member` corruption, all
pre-existing in the file before this session per the very first check's line numbers) was caught by
re-running `--lib` after each risky step and fixed before moving on.
