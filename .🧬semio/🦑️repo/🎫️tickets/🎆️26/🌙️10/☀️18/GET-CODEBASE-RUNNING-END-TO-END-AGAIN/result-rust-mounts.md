# Result rust-mounts (2026-10-19)

Input: `result-tst3b.md` "Peer work in flight" items (a) to (e).

## Findings per item

- (a) `os/🌿️vcs/🗃️history-ledger/🦀️.rs`: already fixed by a peer. `🔗️read/🦀️.rs` exists (tracked, read slot with its own schema/fixtures/tests) and the mount reads `#[path="../🔗️read/🦀️.rs"]` (working tree, 14:40). Nothing to create. The mount resolves to an existing file. `crate-boundaries` still fails (93 pass / 23 fail): `semio-framework-os-vcs` sources `🦀️.rs`, `🪪️checkpoint-records`, `🚨️vcs-errors`, `🧮️diff-and-mutation` still use the forbidden tokens `semio_framework_os_kernel` / `semio_framework_plugin` / `os_store` (design debt of the vcs crate cut, not a missing module). Open item for the crate-cut owner.
- (b) `plugin/📜️declaration/👷️job/🧵️reserved-tool-job/🦀️.rs`: already fixed by a peer (14:40). Both `#[path]` mounts now read `../🎟️request/🔒️command/🦀️.rs` and `../🎟️request/♻️retirement/🦀️.rs`; both targets exist (checked with a path loop).
- (c) 5d gestures `start_result_insert` / `advance_result_insert`: they never existed in any revision (HEAD 695 used plain `HashMap::insert`; HEAD 696 landed the retained-ordered-map call sites without the two methods). No splitter touched the file since 12:29 and `git status` is clean for it. Implemented both in `Puzzle5dKindWeightWork`:
  - `start_result_insert(ordinal, value, resume, cx) -> Result<bool, OriginalTurnFailure>`: funds the `RetainedOrderedMapInsertCursor<String,f64>` birth through `semio_framework_action_bus::admit_original_tool_construction_scoped(cx, &mut self.insert, demand, ...)`, reserves the owner quote with `scope.admit_owner`, hands the map and key back on refusal (key kept in `refused_key` for controlled retirement), returns `false` while the turn grant cannot fund it.
  - `advance_result_insert(cx) -> Result<bool, OriginalTurnFailure>`: gates every action on `cursor.next_advance_demands(...)`, spends the receipt (`consume_retained` + fuel), moves the finished map back into `result`, then closes the cursor via `close_step` until `terminal_is_empty()`, then resumes `stage`. A duplicate-key refusal takes the workspace back (`take_refused_workspace`) and starts the close before returning the error.
  - New field `insert_closing`; imports added (`Puzzle5dKindWeights`, grant/insert types, `ToolJobAdmissionDemand`); unused `HashMap`/`HashSet` import removed.
  - The catalog test `🗂️catalog/🧪️tests/🟦️.ts` (its source-shape law names exactly `cursor.next_advance_demands(`, `admit_original_tool_construction_scoped(cx, &mut self.insert`, `insert: self.insert.take()`) pointed at the now 25-line `👆️gestures/🦀️.rs` and the retired `retained-command/🦀️.rs`; re-pointed to `🎛️selection-and-weights/🦀️.rs` and `🧵️retained-command/🧵️retained-command-job/🦀️.rs`.
- (d) `geometry-service/🧪️tests/🧫️fixtures/🔣️.json` is `{}`: it was committed as `{}` at HEAD 675 (`251fa724c4a`) and never had content; it must be produced by running the Rust service (dependency hashes are SHA-256 of the engine state, outputs carry kernel-computed details), so it cannot be handwritten. NOT DONE: needs cargo (blocked, see below). Plan once cargo works: add a `[DEBUG]` test to `🧪️tests/🔬️unit/🦀️.rs` dumping `{request: <payload parsed>, complete: json(once(document(2.0,1.5), 1000)), stepped: json(once(.., 2)), faulted: json(once(document(2.0,-1.0), 1000))}` to `C/🗑️generated/rust-mounts-*`, write it as the fixture, delete the test, then `bun test` the sibling `🟦️.ts` (Ajv oracle) plus `cargo test -p semio-s-artifact-procedural-3d-geometry-service`.
- (e) `fixture-caller` fixture: the sixth value was a defect. The grant has five currencies (the fixture's own `currencies` array lists items, copy, capacity, release, depth; `RetainedCloneGrant` has five fields; every Rust consumer reads indices 0..4 only, so depth was silently read as 1048576). HEAD 696 turned `[1,16384,65536,262144,64]` into `[1,16384,65536,262144,1048576,4096]`, evidently meaning release 1048576 / depth 4096 (identical to `fixture_mounted_policy()` and to the time-travel `.max(1_048_576)` release). Fixed the fixture to the 5-value `[1,16384,65536,1048576,4096]` (schemas stay 5, matching design) and updated the stale expectation in `plugin/⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts` (`toEqual([1,16384,65536,1048576,4096])`).

## Files changed

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👆️gestures/🎛️selection-and-weights/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗂️catalog/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts`

## Verification

- `bun test ./…/🗂️catalog/🧪️tests/🟦️.ts` (5d): 1 pass / 0 fail (35 expects), includes the source-shape law on the new methods.
- `bun test ./…/⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts`: 2 pass / 0 fail.
- `bun test ./…/🔬️plugin-runtime-runtime-close-budget/🟦️.ts`: 26 pass / 1 skip / 4 fail. The fixture assertions (`native.grant` Ajv 5-item schema and SQLite round trip, lines 78 to 79) now pass; the 4 failures are source-text laws on unrelated Rust (identity authority signatures, envelope quotes, preparation branch), SOURCE-LAW-DRIFT for their owners.
- `bun test ./…/🏪️store/🧩️composition/🚪️open/…/📸️snapshot-custody/🟦️.ts`: 28 pass / 0 fail (was red on the unmounted `read` module per tst3b; run from the repo root because it uses `process.cwd()`).
- `bun test ./…/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts`: 93 pass / 23 fail (see (a); not caused by a missing mount).
- `…/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`: 17 pass / 9 fail (source-shape laws, unchanged character).
- `rustfmt --edition 2021 --emit stdout --config skip_children=true` on the edited 5d Rust file: parses (syntax only; no type check).
- `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <crate> --lib --tests`: NOT RUN TO COMPLETION. Attempts for `semio-framework-os-vcs` (15:4x): `Rust crate Source refused before compiler: semio-framework-artifact-reference (Bounded unique parent kernel roster required)`; for `semio-s-artifact-puzzle-5d-puzzle5d` (15:43 to 16:0x, four attempts): `cargo fetch --locked` fails with `cannot update the lock file Cargo.lock because --locked was passed` (Cargo.lock 15:32 older than manifest edits by running splitters) and once `failed to get semio-framework-value-persistent as a dependency of semio-framework-value-edit` (crate cut in flight). `cargo metadata --offline --locked` times out or fails the same way. A `dependencies lock --all` was running during part of this time (I did not start any lock/sync). So (c) is syntax-checked and test-law-checked only, not compiled.

## Open items

- Re-run `check -p semio-s-artifact-puzzle-5d-puzzle5d --lib --tests` (and `-p semio-framework-os-vcs`, `-p semio-framework-plugin`) once `Cargo.lock` is re-locked and the artifact-reference / value-persistent cuts settle; fix any type errors in `start_result_insert` / `advance_result_insert` (written against `RetainedOrderedMapInsertCursor`, `admit_original_tool_construction_scoped`, `StepContext::{retained_grant,consume_retained,consume_fuel}`, `OriginalTurnFailure::debit_original`).
- (d) geometry-service fixture generation (plan above), then also run the Rust service tests.
- For the crate-cut owner: `semio-framework-os-vcs` sources still use `semio_framework_os_kernel` / `semio_framework_plugin` / `os_store` tokens (crate-boundaries law).
- For the tick-wire splitter: `tick/📦️payload/🦀️.rs` and its `🧪️tests/🦀️.rs` were being edited (15:20) while I only touched the TS test of that directory.
