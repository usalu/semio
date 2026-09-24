# Session 9 Preamble

Session 9 of the repo goal. Canonical ticket with full history: `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`
(ASCII entry: `/Users/ueli/Documents/semio/.tmp-ticket-0918/`). Its `📓️worker-preamble.md` rules apply, with these session-9 specifics.
This folder (ASCII entry `/Users/ueli/Documents/semio/.tmp-ticket/`) is session 9's lane.

1. Never type emoji characters in tool input — not in paths, not in file contents, not in heredocs. Resolve paths with ASCII globs (`ls -d /Users/ueli/Documents/semio/*hub`) and edit through FILE-level ASCII symlinks (`links/world3d.tsx`, never `links/world3d/<emoji file>`) under `.tmp-ticket/wp-<id>/links/`. New files: `touch "$DIR/$(ls "$DIR" | grep -m1 'ts$')"`-style names copied by the shell, then link and edit. Import paths or docstrings that need emoji: copy them with StrReplace from existing lines of the same file, or construct them in the shell with `printf` from bytes already on disk. Agents that typed emoji crashed (`[unavailable] Error`) and created misnamed folders (e.g. `🎫️fixtures` for `🧫️fixtures`).
2. Temporary scripts only under `.tmp-ticket/wp-<id>/`; captures under `.tmp-ticket/wp-<id>/generated/` as `.txt`, ≤ 1 MB.
3. Every wasm32 plugin build, `describe`, `component dev/release`, `materialize`, `activate`, `dev s` cold boot: `zsh /Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh wasm <slice> -- <command …>` in ONE foreground call.
4. Every `cargo test|nextest|build` of `-p semio-hub` and every `os-hub:test*` nx target: `zsh /Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh hub <slice> -- <command …>`. `cargo check -p semio-hub` is free.
5. Every cargo invocation: `-p <crate>` only, `CARGO_INCREMENTAL=0`, at most one cargo from you at a time; binary-producing commands with a private uplift dir `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-<id>/target` (never change `build.build-dir`).
6. Never `pkill`/`killall` by name; kill only pids you started. Never kill peer cargo.
7. Servers: pick free ports in 7700–7799 (hubs) and 6200–6299 (serves); record pids.
8. Write your report early and update it after every landed item.
9. Lock order is wasm → hub, never the reverse. Never hold the `hub` mutex around anything that builds wasm or runs `trusted-catalog-bootstrap`/`describe`/plugin cdylib builds (those take the repo's wasm prebuild flock internally). Build wasm first under `wasm`, then take `hub` only for hub-only cargo/test steps. The full trusted catalog is published by TC5 — consume its output instead of rebuilding catalog packages yourself.
