# Graph load BUILD_SOURCE_UNPARSEABLE after the pdf crate move (2026-11-23)

## Symptom
`repo run semio-hub-flow-component:component ...` stopped in "Reading the project graph slice" with
`BUILD_SOURCE_UNPARSEABLE: ENOENT ... lstat '.../pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/💾️binary/🧬️mutations/🗑️remove-mark-info/📦️packages/🦀️rust/Cargo.toml' (imported by 🌎️hub/🧩️compositions...)`
(logs `🗑️generated/pub-occ-component-r2.txt`, `-r4.txt`). The older `react-v15b.txt` hit (missing `🧭️format-catalog/🤖️generated/stdio-format-claims.json`) is a separate, transient case: the file exists again (regenerated 00:56).

## Root cause
A pdf peer is moving the binary mutation crates to `📝️text/🧬️mutations/...` (workspace `pdf/Cargo.toml` members were already updated; git still tracks the deleted `💾️binary` files, so the move is in progress). Two registries were not updated:
- `📋️build-owners.json`: rows `...pdf-mut-base-5`, `-family-6`, `-family-7`, `-x-2` still pointed at the vanished `💾️binary` roots, and rows for the new `📝️text` crates and some new crates were missing.
- `🔒️dependencies.json`: users rows of the dependency audit baseline named the old Cargo.toml paths (serde_json) and one removed laws crate.
The owner registry is the source of the owner list; reading a row whose manifest is gone lstat-fails inside the hub composition's closure, which is classified (`ownerBoundaryFailureV1`) as the withheld source failure of that owner. Because the hub composition depends on the pdf plugin, that owner is in the requested closure, so the refusal is fatal for the run.

## Fixes
1. `repo ./📜️script.ts verify build-owners sync` (the real reconciler): relocated/added the rows. It left one finding, the stale `mut-base-5` row (git still tracks paths of the in-progress move, so the reconciler refuses to drop it); its replacement row for `base/🚪️io/📝️text/🧬️mutations/📦️packages/🦀️rust` already existed, so the stale row was removed by hand (4849 -> 4848 owners). `verify build-owners` now passes.
2. `🔒️dependencies.json`: users paths of the moved crates rewritten `💾️binary` -> `📝️text` in place (existing order kept); the row of the removed `🔬️app-typed-command-full-operation/...laws` crate dropped (9 diff lines).
3. Not touched: `pdf/🏅️standards/7️⃣1.7/.../🧮️function-colour/🌈️cmyk-image-clut` is a new crate of another peer that is not yet a workspace member of `pdf/Cargo.toml` (their work in progress).

## Robustness (item 3)
The design already isolates a single broken owner: `ownerBoundaryFailureV1` (`🏗️build/🕸️graph/📥️inference/📦️project-nodes/🚧️source-failure`) turns a missing source into a failure of that owner alone, fatal only when the owner is in the closure asked for. Projects outside the closure load normally. Here the closure legitimately contained the pdf owners, so refusing was by design. No code change; a possible later improvement is to treat a registry row whose manifest is absent while the move is within the replica settle window as `replicaUnsettled` (wait and re-read) instead of a stable refusal. Not done: it needs a design decision on the settle window for registry rows, not a contained fix.

## Verification
`repo run semio-hub-flow-component:component -c prototype --exclude-dependencies --dry-run` (log `🗑️generated/graph-unparseable-verify-1.txt`): graph sealed and published ("publishing the sealed graph to the fleet"), "Planning 1 targets", zero `BUILD_SOURCE_UNPARSEABLE`; killed after build tool storage planning started. Caveat: the pdf peer is still mid-move, so new stale rows can reappear; rerun `repo ./📜️script.ts verify build-owners sync` first.
