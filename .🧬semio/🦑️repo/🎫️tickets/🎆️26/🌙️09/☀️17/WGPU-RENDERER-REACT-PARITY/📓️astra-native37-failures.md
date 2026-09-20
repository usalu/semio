# Native37 Runtime Failures

Full1220-test run completed in45.57s:1210passed,10failed,0skipped. Both new kernel wake laws and the previously deadlocked host-panel law completed. Two Sol source owners and root are repairing the exact failures; no gate was skipped.

```text
        FAIL [   0.041s] ( 208/1220) semio-framework-os-renderer-wgpu engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack
  stdout ───

    running 1 test
    test engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack ... FAILED

    failures:

    failures:
        engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.00s
    
  stderr ───

    thread 'engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack' (20844130) panicked at 🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/../../🦀️.rs:1512:5:
    assertion `left == right` failed: 🧱️ guard 'renderer::engine_canvas': measured slot tables differ from the committed budget
      left: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 75704, owner_bytes: 16 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
     right: [FixedSlotTableBudget { owner: "engine_canvas::EngineSurfaceRegistry", capacity: 256, element_bytes: 75104, owner_bytes: 16 }, FixedSlotTableBudget { owner: "engine_canvas::StagedEngineScenes", capacity: 256, element_bytes: 352, owner_bytes: 24 }, FixedSlotTableBudget { owner: "engine_canvas::EngineCanvasBuildContext", capacity: 256, element_bytes: 384, owner_bytes: 72 }]
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.043s] ( 371/1220) semio-framework-os-renderer-wgpu interpreter::ui_command_wiring_tests::ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing
  stdout ───

    running 1 test
    test interpreter::ui_command_wiring_tests::ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing ... FAILED

    failures:

    failures:
        interpreter::ui_command_wiring_tests::ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.00s
    
  stderr ───

    thread 'interpreter::ui_command_wiring_tests::ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing' (20844728) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:439:5:
    assertion failed: SCENE_INTENTS.with(|cell|
            cell.borrow().slots.iter().flatten().any(|intent|
                    intent.pointer_id == Some(pointer) && intent.ink_job.is_some()
                        && !intent.retiring))
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.068s] ( 372/1220) semio-framework-os-renderer-wgpu interpreter::ui_command_wiring_tests::ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle
  stdout ───

    running 1 test
    test interpreter::ui_command_wiring_tests::ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle ... FAILED

    failures:

    failures:
        interpreter::ui_command_wiring_tests::ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.01s
    
  stderr ───

    thread 'interpreter::ui_command_wiring_tests::ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle' (20844732) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:396:5:
    InkCanvas interaction did not reach a terminal state
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.038s] ( 524/1220) semio-framework-os-renderer-wgpu scenes::ink_canvas_tests::shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks
  stdout ───

    running 1 test
    test scenes::ink_canvas_tests::shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks ... FAILED

    failures:

    failures:
        scenes::ink_canvas_tests::shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.01s
    
  stderr ───

    thread 'scenes::ink_canvas_tests::shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks' (20845301) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs:116:5:
    assertion `left == right` failed
      left: Number(56.0)
     right: Number(56)
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.043s] ( 613/1220) semio-framework-os-renderer-wgpu settle_pump_tests::an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch
  stdout ───

    running 1 test
    test settle_pump_tests::an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch ... FAILED

    failures:

    failures:
        settle_pump_tests::an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.01s
    
  stderr ───

    thread 'settle_pump_tests::an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch' (20845622) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧪️tests/🫀️settle-pump/🦀️.rs:200:13:
    a-chrome-pointer-button-declares-and-returns: pub async fn handle_pointer_button declares the chain with self.owe_settle();
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.060s] ( 614/1220) semio-framework-os-renderer-wgpu settle_pump_tests::a_gesture_runs_its_user_activation_bound_work_before_it_returns
  stdout ───

    running 1 test
    test settle_pump_tests::a_gesture_runs_its_user_activation_bound_work_before_it_returns ... FAILED

    failures:

    failures:
        settle_pump_tests::a_gesture_runs_its_user_activation_bound_work_before_it_returns

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.01s
    
  stderr ───

    thread 'settle_pump_tests::a_gesture_runs_its_user_activation_bound_work_before_it_returns' (20845625) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧪️tests/🫀️settle-pump/🦀️.rs:222:9:
    a-pointer-gesture-runs-its-file-picker-before-it-returns: pub async fn handle_pointer_button must run self.drain_gesture_bound_work().await; — a picker handed to the settle pump has lost the gesture that was allowed to open it
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.064s] ( 988/1220) semio-framework-os-renderer-wgpu shell::shell_input_tests::published_tree_handle_routes_through_shell_and_preserves_label_selection
  stdout ───

    running 1 test
    test shell::shell_input_tests::published_tree_handle_routes_through_shell_and_preserves_label_selection ... FAILED

    failures:

    failures:
        shell::shell_input_tests::published_tree_handle_routes_through_shell_and_preserves_label_selection

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.02s
    
  stderr ───
    [DEBUG] ui-doc ingress window=tree-pointer-host-boundary generation=1 nodes=3

    thread 'shell::shell_input_tests::published_tree_handle_routes_through_shell_and_preserves_label_selection' (20848349) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🔬️wgpu-shell-input/🦀️.rs:150:9:
    the tree pointer document faulted in phase fault
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   1.901s] (1069/1220) semio-framework-os-renderer-wgpu shell::settings_general_layout_tests::appearance_option_commit_uses_the_retained_router_and_retires_its_popup
  stdout ───

    running 1 test
    test shell::settings_general_layout_tests::appearance_option_commit_uses_the_retained_router_and_retires_its_popup ... FAILED

    failures:

    failures:
        shell::settings_general_layout_tests::appearance_option_commit_uses_the_retained_router_and_retires_its_popup

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 1.83s
    
  stderr ───
    [DEBUG] ui-doc ingress window=framework.settings.general.appearance-commit-law generation=1 nodes=35
    [DEBUG] wgpu-shell pointer button x=1350.4 y=912.8 down=true button=0 targets=18 staged=0 gen=1 hit=Some((Select, Some("framework.settings.general.appearance-commit-law/framework.settings.appearance"), None))
    [DEBUG] wgpu-shell retained press window=framework.settings.general.appearance-commit-law kind=Select down=true action=None
    [DEBUG] wgpu-shell content focus window=framework.settings.general.appearance-commit-law node=Some(NodeId { index: 3, generation: 0 })
    [DEBUG] wgpu-shell pointer button x=1350.4 y=912.8 down=false button=0 targets=18 staged=0 gen=1 hit=Some((Select, Some("framework.settings.general.appearance-commit-law/framework.settings.appearance"), None))
    [DEBUG] wgpu-shell retained press window=framework.settings.general.appearance-commit-law kind=Select down=false action=None
    [DEBUG] wgpu-shell pointer button x=1350.4 y=884.8 down=true button=0 targets=21 staged=18 gen=2 hit=Some((Button, Some("dark"), Some("setAppearance")))
    [DEBUG] wgpu-shell retained press window=framework.settings.general.appearance-commit-law kind=Button down=true action=Some("setAppearance")
    [DEBUG] wgpu-shell content focus window=framework.settings.general.appearance-commit-law node=Some(NodeId { index: 37, generation: 0 })
    [DEBUG] wgpu-shell pointer button x=1350.4 y=884.8 down=false button=0 targets=39 staged=21 gen=3 hit=Some((Button, Some("dark"), Some("setAppearance")))
    [DEBUG] wgpu-shell retained press window=framework.settings.general.appearance-commit-law kind=Button down=false action=Some("setAppearance")
    [DEBUG] wgpu-shell content focus window=framework.settings.general.appearance-commit-law node=Some(NodeId { index: 3, generation: 0 })
    [DEBUG] ui-doc ingress window=framework.settings.general.appearance-commit-law generation=2 nodes=35

    thread 'shell::settings_general_layout_tests::appearance_option_commit_uses_the_retained_router_and_retires_its_popup' (20847898) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/⚙️settings-general-layout/🦀️.rs:387:5:
    assertion `left == right` failed: the same commit retires every popup option before the next paint
      left: 3
     right: 0
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.266s] (1144/1220) semio-framework-os-renderer-wgpu shell::window_actions_search_pane_tests::the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it
  stdout ───

    running 1 test
    test shell::window_actions_search_pane_tests::the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it ... FAILED

    failures:

    failures:
        shell::window_actions_search_pane_tests::the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.20s
    
  stderr ───
    [DEBUG] wgpu-shell dock plan canvas=1434x576 windows=2 pane-top@470x547+6,61 pane-perspective@947x547+486,61

    thread 'shell::window_actions_search_pane_tests::the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it' (20849569) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/../../🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:138:32:
    mounted Actions row 'action.clearSelection' missing from the published viewport
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        FAIL [   0.058s] (1210/1220) semio-framework-os-renderer-wgpu winit_app::callback_latency_tests::mounted_refused_event_preserves_the_frame_generation
  stdout ───

    running 1 test
    test winit_app::callback_latency_tests::mounted_refused_event_preserves_the_frame_generation ... FAILED

    failures:

    failures:
        winit_app::callback_latency_tests::mounted_refused_event_preserves_the_frame_generation

    test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1219 filtered out; finished in 0.00s
    
  stderr ───

    thread 'winit_app::callback_latency_tests::mounted_refused_event_preserves_the_frame_generation' (20850049) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../🪟️winit-app/../../../🧪️tests/🔬️wgpu-winit-app-callback-latency/🦀️.rs:59:5:
    assertion `left == right` failed
      left: 1
     right: 0
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

────────────
     Summary [  45.570s] 1220 tests run: 1210 passed, 10 failed, 0 skipped
        FAIL [   0.041s] ( 208/1220) semio-framework-os-renderer-wgpu engine_canvas::engine_surface_attach_tests::engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack
        FAIL [   0.043s] ( 371/1220) semio-framework-os-renderer-wgpu interpreter::ui_command_wiring_tests::ink_pointer_cancel_retires_only_the_exact_in_progress_owner_without_publishing
        FAIL [   0.068s] ( 372/1220) semio-framework-os-renderer-wgpu interpreter::ui_command_wiring_tests::ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle
        FAIL [   0.038s] ( 524/1220) semio-framework-os-renderer-wgpu scenes::ink_canvas_tests::shared_clipboard_fixture_copies_order_and_publishes_text_svg_and_recursive_blocks
        FAIL [   0.043s] ( 613/1220) semio-framework-os-renderer-wgpu settle_pump_tests::an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch
        FAIL [   0.060s] ( 614/1220) semio-framework-os-renderer-wgpu settle_pump_tests::a_gesture_runs_its_user_activation_bound_work_before_it_returns
        FAIL [   0.064s] ( 988/1220) semio-framework-os-renderer-wgpu shell::shell_input_tests::published_tree_handle_routes_through_shell_and_preserves_label_selection
        FAIL [   1.901s] (1069/1220) semio-framework-os-renderer-wgpu shell::settings_general_layout_tests::appearance_option_commit_uses_the_retained_router_and_retires_its_popup
        FAIL [   0.266s] (1144/1220) semio-framework-os-renderer-wgpu shell::window_actions_search_pane_tests::the_mounted_actions_tree_keeps_fixed_rows_clips_the_terminal_row_and_scrolls_to_it
        FAIL [   0.058s] (1210/1220) semio-framework-os-renderer-wgpu winit_app::callback_latency_tests::mounted_refused_event_preserves_the_frame_generation
error: test run failed
Warning: command "bun ./📜️script.ts test-wgpu-unit --no-fail-fast" exited with non-zero status code


 NX   Running target test-wgpu-unit for project @semio-tech/framework-renderer-wgpu and 4 tasks it depends on failed

Failed tasks:

- @semio-tech/framework-renderer-wgpu:test-wgpu-unit


Output of 4 successful tasks were not shown. Run with --verbose or --output-style=static to see it.
Hint: run the command with --verbose for more details.

  Run duration:      1m 48s
  Cache:             Skipped (--skip-nx-cache)
  Critical path:     1m 47s (2 tasks)
  Recoverable time:  <1ms

  Recommendations:
    - Cache: drop --skip-nx-cache to restore unchanged tasks instantly.
    - Speed up or split the longest tasks on the critical path:
        @semio-tech/framework-renderer-wgpu:test-wgpu-unit    1m 42s
error: script "nx" exited with code 1

```

## General Frame Harness Repair

The appearance commit law omitted the normal InputFrame retirement between paint walks. HitRegistry swaps the completed registry with staging; without retiring staging, old popup entries are appended to the next build and reappear after the following swap. Production already retires one entry per InputFrame step. The test helper now drives that same retirement to completion before its synthetic panel walk, retaining all popup, capture, action, theme and ordinary-button assertions. Verification remains pending Native38.

Native39: General and Winit17/17 selected laws pass,1,205outside selection. Native38: Ink still3failures;9selected pass,8notexecuted due fail-fast,1,200outside selection. General and Winit are verified at native harness level; Ink and remaining scene laws are still under repair.

Native40 stopped before test execution at one new catalogue harness E0616: private InputState.pending_actions access in wgpu-canvas2d:98. Compiler rebuilt shared kernel dependencies, then reported this exact error. Sol is retaining the no-partial-publication assertion via the existing public queue boundary before Native41.

UI55 passes642/642 native UI tests with zero skipped, including the real62-full-document byte-limit law. Native41 stopped at E0499 in the extracted frame input collection seam: disjoint mutable fields were borrowed through a MutexGuard without first reborrowing its AppRuntime. This is a compile failure, not the intended batch-refusal RED witness. Native42 will execute after that mechanical borrow correction.

## Native43 Full Execution

Native42 also stopped at E0499 before any tests: AppRuntime itself dereferences to AppInteractionState, so reborrowing the MutexGuard target was not sufficient. The collector now explicitly borrows `app.interaction.as_mut().expect(...)` separately from `app.frame_actions`, matching the existing worker-owned interaction invariant. Native43 then compiled and ran all1,226 tests:1,223passed,3failed,0skipped in44.299seconds. All three old Ink defects, General, Winit admission, surface-size, settle, and Tree fixture laws passed. The failures are the intended real catalogue terminal-pair downstream admission RED, a new catalogue test fixture exhausting byte credits before slot credits, and the existing mounted Actions clearSelection viewport law. Sol owns the bounded batch repair and fixture correction; the second Sol owns Actions. Log: `🗑️generated/astra-runtime/native-renderer-43.log`.

UI56 passes645/645 native UI laws, zero skipped in3.817seconds, including new batch countdown/removal/claims and padding memory checks. The renderer batch/backpressure implementation reached its source-coherent marker; Native44 full census is running next with the corrected Actions publication harness. No new renderer pass is claimed until this actual run finishes.

## Native44 And45

Native44 executed1,228tests:1,226passed,2failed,zero skipped in44.168seconds. Catalogue batch capacity, source-pressure fixture, live deferred-owner draining and staged close laws pass. Actions still lacks its first visible row after the bounded publication harness; the new complete-frame diagnostics will identify exact lease/content/hit state. Tree hostinput intermittently faulted during document ingress; the same law passed in Native45.

Native45 selected3laws:1passed,2failed (1,226outside selection). It proves the new post-stage source-fault defect: after staging canvasDragLeave and injecting a source authority fault, cleanup discarded the staged prefix but left canvasDrop as the next dispatchable action instead of independent later. Sol is repairing bounded source-tail retirement. The Actions failure persists; its detailed diagnostic source landed after this compiler snapshot.

Root extracted the existing page-application branch unchanged and added deterministic production-seam tests using the shared neutral document fixture: exhausted-fuel/deadline refusal must retain the cursor and retry the same page, while a live wrong generation must remain terminal. These tests are pending Native46. UI engine currently maps budget refusal and generation mismatch to the same Generation page-rejection enum; the renderer currently terminal-faults every page rejection. This is a concrete source defect; its relation to intermittent Tree44 is still under diagnostic verification.

## Native46 Deterministic Receipts

Focused Native46 executes4laws:2pass,2fail (1,227outside selection). The repaired catalogue post-stage fault cleanup passes, preserving independent successor and suppressing the orphan terminal action. The live wrong-generation control passes. The new page-refusal law supplies deterministic RED: actual engine page rejection is Generation with cancelled=false/yielded=true, and the retained cursor incorrectly becomes terminal. Root added the precise retry condition only for Generation rejection with an exhausted/cancelled context; live generation errors remain terminal. The same-page retry law now covers fuel, deadline and cancellation. Final green is pending Native47.

Actions diagnostics prove an independent pre-ingress issue: all8completedchrome walks carry a58-node lease at generation1, but retained content height is absent, surface-fault false, and only21chrome hits exist; none are Actions body rows. No ui-doc ingress ever occurs. Sol is tracing why this mounted fixture never enters the body paint phase; increasing walk counts is not a repair.

## Native47 And Window-Chrome Root Cause

Native47 selected4laws:3pass,1fails (1,227outside selection). The real document page refusal/retry law now passes for fuel, deadline and cancellation, and a live wrong generation still faults. Catalogue source-tail cleanup remains green. Actions diagnostics show folded=false and a complete shell walk without ever entering phase12. Root traced the exact production skip: absent window content at main-window phase4 advanced to the next window, bypassing pane phases8–13. React Window mounts its pane controls independently of the optional body. Sol repaired the missing-body branch to advance to phase8 and removed all temporary Actions diagnostics and guessed multi-frame retries. Native48 full census is now running against that source marker.

## Native48 Full Green

Native48 passes all1,231renderer tests with zero skips in35.987seconds (1m34sNx target). This includes exact Actions fixed geometry/scroll/Abort click from one complete chrome publication without a body lease, Tree hostinput, page refusal/retry, real generation rejection, catalogue source/target atomicity and fault retirement, and all prior Ink/General/Winit laws. Evidence: `🗑️generated/astra-runtime/native-renderer-48.log`. Canonical WGPU activation16 is running next; browser parity is not inferred from native green.
