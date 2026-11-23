# L17a 3D lifecycle/retirement cluster (partial; blocked on tmpfs space)

## Status
- Fix 1 (ordered-map retirement) written, NOT yet compiled or run: the runner tmpfs (4g) is 100% full (2 feature-set generations of every dependency, ~391 crates duplicated, plus L17b builds in the same namespace) and a `rm` cleanup of the stale generation and a `mount -o remount,size=` were denied by the permission layer. /home also hit 0 free for a while (the 3D runner log writes were lost; builds ran from /dev/shm instead).
- Re-measured baseline (current framework, old fixture, built exe run directly): `editor-panels` 40 pass / 51 fail (earlier full run: 63 / 28). 7 fail with "ordered-map root must be explicitly retired before drop", ~42 with "puzzle3d app never quiesced: pending typed operations outlived the settle budget" (new, framework in flux; owner: framework/L17b), 1 "registered fixture ... terminal-empty witness", 1 "reload exact window pack", 1 "maintenance step ...".

## Cause A: "ordered-map root must be explicitly retired before drop" (19 tests) - test fixture
`ViewModel.active_utility_by_window_id` and `tool_run_trace_cursor_by_window_id` are `OrderedMap`s (🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs). Their `Drop` asserts empty roots; clones are aliases that must be retired or returned (`ScopedWindowView`). The fixture kept the host session `ViewModel` in `Puzzle3dApp.view`, handed out owned clones (`window_view`), wrapped them in `Arc` inside `ActionMeta`, and reset the map with `= Default::default()`. As soon as a test activated a utility (`setActiveUtility`, so the map is non-empty) every clone/the app view dropped a live root and aborted. Tests that never activate a utility passed (empty roots). Not a puzzle product bug, not a framework bug.

Fix (in 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/):
- 🧰️context/🏗️raw-app-fixture/🦀️.rs: new `retire_view` (cold-retires both roots via `OrderedMap::retire_cold`), `WindowView` (Deref + retiring Drop) returned by `Puzzle3dApp::window_view`, `WindowMeta` (ActionMeta whose `Arc<ViewModel>` is unwrapped and retired on drop) via `Puzzle3dApp::window_meta`, `activate` retires the replaced utility map, `Drop for Puzzle3dApp` retires `self.view`.
- 🧰️context/🎛️settle-and-dispatch-drivers/🦀️.rs and 🧰️context/📏️measure-and-tool-run-probes/🦀️.rs: `ActionMeta { view_state: Some(Arc::new(app.window_view(..))), .. }` replaced by `app.window_meta(..)`.
- 🔬️gesture-kinds/🦀️.rs:257: `Arc::new(view.into_inner())`.
Risk to verify on first compile: `Arc::try_unwrap` in `WindowMeta::drop` panics if the framework keeps a clone of the call's view past the call; `retire_cold` on an alias root (non-final owner) must not panic.

## Cause B: terminal-empty witness / history close / "unretired closing session" (12 + 6) - framework (not fixed)
`context::history_edits_end_to_end` etc: close of the time-travel discard session stops at Replaying with `value.refusal unsupportedOwner "terminal ticket has no explicit native header lifecycle"`, raised by `factory_ticket_demands` (🧰️framework/🔨️modules/🌱️value/♻️retirement/🎟️tickets/🦀️.rs:4). `ErasedSnapshotRetirement::native_header_release` defaults to `None` (♻️retirement/🧬️contract/🦀️.rs:128); `FactoryTicket` returns `None` unless `T::close_state_terminal_native_release_safe()`, and the file already carries another agent's `[DEBUG]` backtrace probes (tickets file and factory/🦀️.rs:184 "native header unsafe type"). Candidate: `TimeTravelDiscardCursor` (🧰️framework/🔨️modules/⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs:109) overrides no `native_header_release`. Owned by the framework retirement agent; I did not touch it.

## Cause C: "local interaction read N never terminated", "original Puzzle work remains in close custody", "context root retains its original projection or Store issuer", "cancelled caller must release its empty mount shell" - unresolved
Same family as B plus the new "never quiesced" regression; needs a build with symbols (the runner strips symbols, so backtraces are empty) and fresh framework state. Not diagnosed further.

## Runner notes
- Started the 3D runner (pid 122687 namespace); it is still alive in its request loop and L17b is using it. Files in /dev/shm (l17-go.sh, l17-exe.sh: run cargo / built test exes inside that namespace with logs and spill in RAM) are not in the repo.
- Required: ZIG_GLOBAL_CACHE_DIR must not live on a full /home (`cc` is zig; "unable to create compilation: NoSpaceLeft").
- Next steps once tmpfs has room (>= 7g or delete stale dependency generation): `cargo test --no-run -p semio-s-artifact-puzzle-3d-editor --test editor-panels`, run `gumball_inactive_when_every_handle_flag_is_off` exact; expect Cause A tests to pass, then rerun all editor targets.
