# 📓 Agent Brief (FEM 3D end to end)

Ticket: `26/11/25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE` (this folder, `$M`).
Prior ticket with all history, scripts and the failing matrix: `26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END` (`$T22`), see `feature-matrix-3d-v2.md` (F1-F14), `fem-interactive-features.md`, `fem2d-plugin-fixes.md` (2D twins of several 3D fixes).

## Paths
- 3D plugin: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/` (crate `semio-s-artifact-fem-3d-editor`, workspace root `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d`).
- 2D twin: `.../◻️2d/...`.
- Browser: vite 3D on http://127.0.0.1:6087/ (2D on 6086), served component in `.cache/b/p/m/browser/prototype/modules/🏗️fem` (wasm from 11-25 12:30; the coordinator rebuilds it, never you).
- Headless probing: `$T22/page-driver.mjs <url> <port>` + `$T22/d.sh <port>` (POST JS body). Start your own driver on a free port 62xx; kill only your own PIDs.

## Rules
- Follow `C:\semio\CLAUDE.md`. No git command that modifies the tree. Re-read files before editing; others edit concurrently (last writer wins).
- Test-driven: add or extend a law (Rust `#[test]`) for each fix; 3D fixes mirror 2D fixes where one exists.
- Native cargo only, private dirs: `CARGO_TARGET_DIR=C:\t\<you>` AND `CARGO_BUILD_BUILD_DIR=C:\t\<you>` (the SAME dir: a different build dir splits the layout and overflows the Windows command line with `os error 206`; leaving it unset falls back to the repo's shared `.cargo/config.toml` build dir and serializes every agent on one lock), `TMP/TEMP/TMPDIR=C:\t\<you>tmp`, `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, `RUST_MIN_STACK=134217728`, `-j 3`, `--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""'`. Single test targets only. Example script: `$T22/fem3d-resolve-law.sh`. Delete your `C:\t\<you>*` dirs when done.
- rustc may crash with STATUS_ACCESS_VIOLATION: just retry.
- Long cargo runs MUST be launched detached from your shell (`powershell -NoProfile -c "Start-Process -WindowStyle Hidden -FilePath 'C:\Program Files\Git\usr\bin\bash.exe' -ArgumentList '/c/t/<you>-run.sh'"`, an ASCII-path launcher that writes its own log and `rc=` line). The Claude app's `taskkill /T` of stopped tasks walks stale parent PIDs and silently kills unrelated cargo trees (rc=1, no error): see `cargo-killer.md`.
- Scripts in `$M`, outputs in `$M/🗑️generated/<you>/`. Final report: `$M/<topic>.md` (root cause, fix, files, law, status). `[DEBUG] ` prefix for temporary prints, remove them before finishing.
- Concise code, no comments inside definitions, docstrings start with a unique emoji.
