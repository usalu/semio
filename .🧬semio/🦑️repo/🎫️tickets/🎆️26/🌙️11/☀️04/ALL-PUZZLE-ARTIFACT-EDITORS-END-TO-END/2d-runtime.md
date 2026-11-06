# Puzzle 2d Runtime

Owned the 2d snapshot import boundary. The old admission predicate accepted an explicit foreign schema, null schema, numeric schema, or malformed nodes whenever the other branch of its schema-or-nodes predicate passed. Extended the existing language-neutral payload input corpus before changing admission. A direct evaluation of the original predicate reproduced four new rejecting vectors as incorrectly accepted. Admission now validates any explicit schema and any present nodes array, preserving schema-free arrays and schema-only empty boards.

The existing native snapshot_import_requires_the_explicit_payload_contract law consumes the expanded corpus and compares every accepted native projection against serde_json as a third-party JSON oracle. A native suite is queued with repo run @semio-tech/puzzle-2d-rs:test --lang=en -- --lib; it has not yet run because repository capacity is saturated by other agents. No runtime test success is claimed.

Files changed:
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📥️import-snapshot/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧊️document/📥️import-snapshot/🧪️tests/🧫️fixtures/📄️payload-input/🔣️.json

The corpus now includes its language-neutral admission JSON Schema. The retained 2d-import-oracle.cjs ticket input compiles that schema with the existing third-party Ajv installation; all 14 vectors pass. A mounted runtime law invalid_snapshot_import_keeps_the_live_board_and_its_history now dispatches malformed object imports after creating a node, requires a visible notice, prohibits LoadDocument and edits, checks the live snapshot remains identical, and undoes the seed afterward to prove the history stays usable. Native execution is still pending.

## Mounted Persistence Law and Gate

Added `created_and_edited_board_reopens_with_undo_and_redo` to the mounted editor unit laws. It creates a node, translates it, serializes to text and pack in separate runs, reopens each in a fresh registered app, and checks the edited board, undo, and redo. This is a test addition; execution and compilation are not yet established.

Both `repo run @semio-tech/puzzle-2d-rs:test --lang=en -- --lib` and `repo cargo --store=shared --configuration=prototype --parallelism=4 -- test -p semio-s-artifact-puzzle-2d --lib` failed before Rust compilation at `source.capture`: authorizer source closure capacity exceeded, 1,831/2,048 files and 16,788,269/16,777,216 bytes. The failure path is the build graph memo warm source. This replaced the earlier missing/ambiguous icon module failure. UI and persistence behavior remain unverified at runtime.

Additional file changed:
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧮️operations-and-command-envelope-laws/🦀️.rs

## Browser Persistence Journey

Added step 24 to the real Puzzle 2D time-travel scenario, with a language-neutral JSON input fixture. On the React `puzzle2d` variant, it attaches a local folder, creates a node by duplicating a selected board node, drags the new node, waits for an archive, reloads, reattaches the folder, then checks the reopened board and undo/redo using keyboard input. The browser wgpu variant currently has no local folder transport and records that limitation. This journey has not been executed; TypeScript `transpileModule` returned no syntax diagnostics, which is not a typecheck or runtime result.

The browser invocation `repo run @semio-tech/framework-os-dev:time-travel --lang=en -- --scenario ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts --renderer react --locales en,de --only 24 --out <ticket>/🗑️generated` stopped at the same authorizer source-capacity gate before the serve started. The native law is owned by the `semio-s-artifact-puzzle-2d-unit-tests` crate, whose test module includes the law under `#[cfg(test)]`; the prior artifact-root crate invocation does not select it. Run it with `repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-puzzle-2d-unit-tests --lib created_and_edited_board_reopens_with_undo_and_redo` when admission is repaired.

Additional files changed:
- ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts
- ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/💾️persistence-journey/🟦️.ts
- ✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/💾️persistence-journey/🔣️.json

## Readback and Routing Review

Step 24 is registered in the scenario's step list, execution order and dispatch table. Before reloading, it now opens a fresh second peer, attaches the same local folder and waits until the peer's actual hydrated board matches every edited node position. A failed boot, attachment or readback records a failed verdict and stops the reload. This replaces the prior file-existence check, which could observe an archive from before the edit. The TypeScript compiler's transpile parser reported no syntax diagnostics; browser execution has not yet verified the new path.

The normal `@semio-tech/puzzle-2d-rs:test` package list explicitly includes `semio-s-artifact-puzzle-2d-unit-tests`, which owns the new mounted lifecycle law. A separately attempted direct `repo cargo` run was cancelled during source admission in favor of the normal owned target. Neither the native law nor browser step 24 has completed in this run.

The normal target was then started with `-- --lib created_and_edited_board_reopens_with_undo_and_redo`; live command session 16617 passed Source admission and waits at dependency preparation for the active dependency producer. At 10:25:22 the live fleet reported that producer (PID 34296, lease holder 29656) at `dependencies:store-input-replay`. The test result is pending, not a passing test; the command was left running for the parent to monitor.

Session 16617 later disappeared without an observable terminal result. Replacement normal target session 21731 is live and waiting for the dependency producer. A separate supported `repo cargo` invocation with the 2D workspace manifest and the exact lifecycle test filter passed source and Cargo planning, then exited 1 in `ensureCargoLockV1` before compilation. The tool response truncated the causal lock diagnostic, so its underlying missing input or resolver error is unknown; no native test result is claimed.

One controlled retry, session 92087, captured its complete output in `🗑️generated/2d-cargo-lock.log`. The normal lock producer refreshed the 2D lock and reported it current. Cargo source admission then exited 1 at unit 65 of 864: `semio-framework-canvas-assets` requires the missing or ambiguous Rust module `icon_pack_digest`. No compiler or test ran. This establishes lock convergence for this retry, not the cause of the earlier truncated failure or the separate 5D refusal. The lock owner forwards its explicit probe and refresh argument lists without adding `--locked` to the refresh. The real asset build is already running separately and awaits the shared dependency producer.

One React activation was started as session 80930 with the supported Bazel closure install mode, command-local only, after a live process query found no existing matching activation. The source inferrer defines the variant/profile target pattern and the selected-runtime fixture names `puzzle2d`; no generated catalog or sealed graph was available to confirm the emitted target independently before execution. The activation initially queued for capacity. Browser step 24 remains unexecuted.

At the coordinator's instruction, normal native session 21731 was deliberately rescheduled to release its idle capacity reservation for activations. Before cancellation its process tree was PID 55636 → 55024; Ctrl+C returned exit 1 while still waiting for dependencies, with zero tests run. Both PIDs subsequently returned ESRCH, the capacity grant and child leases disappeared, and the unrelated dependency producer PID 29656 remained alive. The exact native acceptance must be rerun on current source after the genuine asset prerequisite is ready. Activation session 80930 remains queued; the released reservation did not immediately satisfy the shared capacity gate.

Activation 80930 later passed capacity and source boot and reached parser source listing. No graph selection, activation completion or browser result has yet been observed.

It subsequently reached `build:dependencies` and explicitly reported waiting for another dependency build. The authored closure mode narrows selection while retaining the common build lease; it did not independently pass the current owner. The activation target prepares runtime rather than serving HTTP; after successful completion, the time-travel runner's supported server path must start or reuse the 2D server before step 24 can run.

## Portable Native Lifecycle Oracle

The native create/edit/reopen/history law now consumes an adjacent neutral JSON fixture with explicit create, translate, exact created/edited coordinates and both text/pack encodings. It parses the fixture independently with serde_json and independently parses the produced node JSON before exact coordinate assertions. Full node equality still guards reopen/undo/redo. Fixture arithmetic and source wiring were checked; this is not a native execution pass. The native run remains behind genuine asset generation. The new fixture absolute path is 177 UTF-16 units.
