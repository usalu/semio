# Draw drawing native tests, 2026-11-23

Result: BLOCKED. No drawing test executed; no drawing source changed. Reason: three independent environment/source blockers outside the drawing slice (listed below).

## Example parse check (static, no run possible)
- `examples()` registers exactly `demo` and `blank`.
- `set-active-example` parses each via `DrawingSnapshot::parse_dsl(&source.document_json())`.
- Existing coverage already exercises every registered example through that exact path: `set_active_example_resolves_the_registered_catalogue` (editor `command-laws`, loops over all `examples()`), `registered_demo_source_parses_the_authored_emblem` (demo) and `registered_blank_starts_with_no_layers_or_assets` (examples crate).
- None of these could be run, so the browser `drawing.example.parse` for `demo` is NOT confirmed fixed or reproduced.

## Commands run (SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en), logs in 🗑️generated/native-tests-2026-11-23-*.log
1. `repo ./📜️script.ts test` -> `Script provider designation required in SEMIO_SCRIPT_PROVIDERS` (wrong entry form; script must run through `repo run`).
2. `repo run @semio-tech/draw-drawing-rs:test` (3 attempts) -> repo graph service exits ("lost the service exited: adopted (code 0) ... shutdown settled") after ~15 s every time.
3. Raw `cargo test` -> refused by the compiler ingress ("requires the repository launcher").
4. `repo cargo --store=isolated --configuration=prototype -- test --workspace`:
   - first two tries: "The disk is full" guard (20 GiB free); fixed by `repo cache cas gc` (5.6 GiB freed).
   - then: `semio-s-artifact-draw-drawing-editor exceeds its source boundary: 22 of 32 files, 262845 of 262144 bytes` (cfg(test) closure of the editor lib, 701 bytes over).
5. Same with `--exclude semio-s-artifact-draw-drawing-editor`:
   - `semio-s-artifact-draw-drawing-model exceeds its source boundary: 33 of 32 files` (cfg(test) closure).
6. `repo cargo ... -- test -p semio-s-artifact-draw-drawing-examples` (admission passed) ->
   `error[E0061]` in `semio-plugin-window-kits`: `scene_surface` now takes 5 args (`name`, `description`), callers pass 3
   (`🪟️window-kits/📝️text/🦀️.rs:159,235`, `🪟️window-kits/🧊️mesh/🦀️.rs:51`, plus `📊️table/🦀️.rs:166,205,239`, tests in `🧪️tests/🔬️tool-run` and `🧪️tests/🪟️surface-carriers`).
   `🪟️window/🦀️.rs` was modified at 01:13 on 2026-11-23, i.e. another agent's in-flight API change; stale callers were still present for 4+ minutes afterwards.

## Pass/fail counts
0 passed, 0 failed (nothing reached test execution).

## Fixes
None to drawing sources. Only action: `repo cache cas gc` to clear the disk-full guard.

## Still failing / needed
- Owner of the `scene_surface(id, kind, scene, name, description)` change must update window-kits callers; this blocks every cargo build depending on `semio-plugin-window-kits` (the drawing examples/editor crates included).
- Editor lib cfg(test) closure is 701 bytes over the 262144-byte boundary; model lib cfg(test) closure is 1 file over 32. These need real splitting (move test modules/files into the companion crates) before the lib unit tests can be admitted; the companion crates compile the libs without cfg(test) and are unaffected.
- repo graph service dies on startup; `repo run ...:test` cannot be used until it is fixed (`repo service host status`).
