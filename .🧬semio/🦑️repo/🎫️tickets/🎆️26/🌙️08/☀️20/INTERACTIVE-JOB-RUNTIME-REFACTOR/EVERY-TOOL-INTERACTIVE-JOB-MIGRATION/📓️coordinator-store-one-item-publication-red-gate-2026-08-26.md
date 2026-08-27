# Store One-Item Publication Red Gate

## Result

The tool-job verifier now rejects every route while the shared Store publication boundary can enter whole-operation replay, command application, outbound flushing, mutation diffing, or diff application in one actor turn.

The guard requires a retained `ArtifactStoreOneItemPublication` state machine with explicit begin, advance, cancel, close-step, terminal-emptiness, generation, item-budget, and byte-budget witnesses. It also requires a fail-closed `ArtifactStoreOneItemPreparationFactory` seam and a plugin-owned pending publication that begins once and advances once per turn without calling `apply_one`.

## Red-First Validation

`bun ./📜️script.ts verify interactivity tool-jobs --self-test` exited successfully with `self-tests=468 clean`. Hostile fixtures were rejected when the preparation factory, terminal witness, retained plugin publication, or bounded Store entry points were removed; when monolithic Store/plugin calls were restored; when an entry phase hid resizable-vector growth or collection materialization; or when close restored a whole-vector `clear` instead of one item under an explicit byte grant.

`git diff --check` was clean for `📜️script.ts` and the Playbook retained-view owner qualification.

## Current Acceptance

This is intentionally a red production gate. It does not claim the Store implementation or any app route is green. Route admission remains fail-closed until the Store state machine and app/domain preparation implementations satisfy the full-operation boundary.
