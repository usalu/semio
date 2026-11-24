# O20-patched: path-module units for the play snapshot (REQUESTER-ONLY, HOT-LOADABLE)

Base: the play clone `/Users/ueli/.sinc/play` as it is now, O19 already applied (sealed digest `4114c4d6a52d7c25`, created 11:54, is what the running `cargo build` (pid 62034) uses through `RUSTC_WRAPPER=.../u/bin/rustc-wrapper-4114c4d6a52d7c25`; its `inputs.ts` is the O19 post-image `f6c2c16f`). Nothing was written to the clone or its sealed copies.

## Apply: sealed mode FIRST (picked up by every wrapper process spawned afterwards, no restart of cargo, driver or anything)
```
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/wave-1122-dist/O20-patched/apply.sh /Users/ueli/.sinc/play/.cache/b/r/m/u/sealed/4114c4d6a52d7c25 sealed
```
Tree mode (the clone's source tree, taken by the next seal; tests included): the same script with `/Users/ueli/.sinc/play` and no second argument. Both modes: stage 1 checks every target against its pre-image sha256 and refuses on any mismatch before it changes anything, a file that already has the post-image is skipped (idempotent), stage 2 writes `<name>.o20tmp` and renames. The older sealed digests (`05fd3e08470211af` and the other four) hold the same `paths` pre-image; patch them the same way only if a process still spawns wrappers from them.

## What the sealed mode hot-loads (exactly ONE file: `🗺️paths/🟦️.ts`)
The wrapper (`📜️script.ts`) imports the planner module at each rustc call (`moduleRemapped()` dynamic import, `offload.ts` static import), so all four behaviours are live for the next wrapper process:
1. **Mutation-derive units decline before upload** (`not-portable`, `offload-declined`, no store call, 1 to 10 ms on the real crates; 133 ms for the 666-file layout model scan). All 113 crates of the 11:15-12:00 `incomplete-inputs` fallbacks (7,466 s wasted) are such units; run against the live tree every one of them now declines at plan time (113/113, 0 planned).
2. **Workspace-root module directory is covered**: `moduleRemapArgumentsV1` no longer refuses when the common module directory IS the workspace root; it adds `--remap-path-prefix=<workspace>=.` (before Cargo's own remaps, which still win), also to the LOCAL compile, so local, remote and any other checkout record the same names. Cures `unmapped-path: no --remap-path-prefix covers the module directory /Users/ueli/.sinc/play` (norm_inspector_forms, fem_app_surface, hub_stdio_catalog_projection, layout_diff: all planned now).
3. **No 512-file cap on the module scan** (`REACHED_FILES` = `limits.maximumInputFiles` 8192): `source outside CARGO_MANIFEST_DIR` for units that reach more than 512 files (layout model: 666) is gone; exactly the reached files ship, never a directory walk.
4. The `climbAboveV1` text of the live tree (the clone's `paths` lacked it, post-image = live file).
Side effect to expect: the units of (2) get a new local key once (their command line changed), and their outputs become path-independent.
Not hot-loaded / not in the set: tests (tree mode only), the live cluster test (peers' edits make its pre-image differ from the clone), nothing in the driver.

## Files
| file | kind | pre sha256 (16) | post sha256 (16) |
|---|---|---|---|
| `paths.ts` | patched | `16e43dce84937ed8` | `51b08c42d7889d3c` |
| `determinism-test.ts` | patched (tree only) | see `list.tsv` | see `list.tsv` |
| `paths-test.ts` | patched (tree only) | see `list.tsv` | see `list.tsv` |
| `derive-test.ts` | new (tree only) | `NEW` | see `list.tsv` |
`edit-paths.py` (in `O20-scratch/`) is the scripted edit of the clone's `paths` file (clone + live `climbAboveV1` + `edit-paths.py` == `paths.ts`, checked with cmp).

## Verified (sealed mode on a scratch copy, scratchpad `o20/sealed`, 5.2 MB, no write to the real one)
- `apply.sh <copy> sealed`: 1 file written (tests are skipped as absent), second run `0 file(s) to write`.
- `bun build` of the copy's `script.ts` (snapshot bun): ok.
- Wrapper smoke from the copy (snapshot bun, `SEMIO_OFFLOAD=never`, unit in a package directory whose `#[path]` module is in the workspace root's other tree): the inner rustc received `--remap-path-prefix=<ws>=.`; the same run from an unpatched copy of 4114: 0 such argument.
- The copy's planner on the live tree: `semio-s-artifact-fem-3d-material` `not-portable: the mutation derive in ../../🩹️change-material/🦀️.rs ...`, `semio-s-fem-app-surface` and `layout-diff` planned.
