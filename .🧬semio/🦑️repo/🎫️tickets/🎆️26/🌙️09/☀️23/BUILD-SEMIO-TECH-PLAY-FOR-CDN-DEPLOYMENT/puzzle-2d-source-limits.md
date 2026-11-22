# Puzzle 2D snapshot crate source limits

## Findings

Both refusals in `job-puzzle-wasm.log` have one cause: the crate `semio-s-artifact-puzzle-2d-snapshot` exceeded the per-crate ceiling (32 files, 262,144 bytes).

- `process.source.boundary-exceeded` is the per-crate total: 16 files, 263,979 bytes against 262,144.
- `process.source.read-bytes-exceeded` is not a per-file limit. The admission reader (`native-build/source/admission/read`) receives `maximumBytes` as the crate's remaining byte budget at the moment a file is read (262,144 minus the bytes already admitted). The window file (54,421 bytes) was read with 52,586 remaining, so it was the file that tipped the crate over.
- Admission counts every file the crate's module tree reaches, including `#[path]` mounts outside the crate directory. It does not count dependency crates. The ticket probe counted differently, so it reported the crate OK.

## Resolution

Moved the editor window module into its own sibling crate `semio-s-artifact-puzzle-2d-window`, following the records and tambour carve pattern.

- New crate at `editor/window/packages/rust` (`Cargo.toml` and `lib.rs`). The lib mounts the unchanged module file as `editor::puzzle2d::window` and re-exports `records`' `config` so the module's `crate::editor::puzzle2d::config` paths resolve unchanged.
- Dependencies: records plus the externs the window module uses. No cycle (window depends on records only; snapshot depends on window).
- Snapshot now re-exports `semio_s_artifact_puzzle_2d_window::editor::puzzle2d::window`, so `crate::editor::puzzle2d::window` stays stable for the inferences crate and the others.
- Registered in the `2d` workspace members and in `build-owners.json`.

## Sizes as admission counts them

| Crate | Files | Bytes |
| --- | --- | --- |
| snapshot before | 16 | 263,979 |
| snapshot after | 15 | 209,541 |
| window (new) | 2 | 55,027 |

Both are below the 220 KiB (225,280 bytes) target.

## Verification

`repo cargo --store=shared --configuration=prototype -- check --target wasm32-unknown-unknown -p semio-s-artifact-puzzle-2d-window -p semio-s-artifact-puzzle-2d-snapshot --message-format=short` from the `2d` workspace passed (exit 0). Source admission ran in the same invocation and admitted both crates. Only warnings remain, including one unused `extern crate` alias in the new crate.
