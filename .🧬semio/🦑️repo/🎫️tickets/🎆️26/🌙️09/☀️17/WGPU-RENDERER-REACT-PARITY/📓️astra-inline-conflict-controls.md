# Inline Conflict Controls

The new language-neutral fixture and closed JSON schema live under the renderer engine's `🎛️inline-tree-controls` fixture/schema folders. They require one conflict TreeItem with a horizontal Toolbar containing two real labeled Buttons, available before selecting the row. The existing WGPU projection instead creates three TreeItems and hides the verbs under disclosure.

The React oracle calls the real Settings panel factory, mounts its conflict control with the existing UI testing library, and verifies labels, ids, two inline Buttons and resolution callbacks. It passed one focused test (three unrelated tests skipped) in `react-inline-controls-oracle/run-4.log`. Earlier runs fixed test-harness path and query mistakes; they are not feature regressions.

The WGPU test exercises the Shell's actual panel record assembler and requires canonical Toolbar/Button records with Activate bindings. Native 150 first encountered a test-only ActionId field typo; that is corrected and its second run is pending. The implementation packet follows the owned-wire Tree decoder repair and Dock clip validation.

The bounded design recommendation is recorded in `📓️terra-settings-conflicts-inline-preview-audit.md`. Icon-only Tree row actions cannot substitute for the React labeled buttons because they lack the required separate input/accessibility ownership. A selected conflict diff preview remains a separate required detail layout.

Native 150's second run establishes the actual RED: three TreeItem records were published where the fixture requires one. One test failed in 0.137 seconds; 1,370 unrelated tests were skipped. The React oracle is green for the same fixture.

The implementation now publishes one TreeItem row with an explicit direct-child horizontal Toolbar containing real labeled Buttons. TreeItem owns its inline-toolbar child id in the shared schema, Rust record, TypeScript decoder, builder, copy and validation paths. Reconcile preserves that semantic relation; layout keeps the toolbar in the row; hit testing gives the buttons the row's pointer band before row selection; accessibility projects separate button controls.

Focused receipts:

- retained TypeScript wire ownership: 1 passed, 1 skipped;
- actual React inline-controls oracle: 1 passed, 1 skipped;
- native laws handed to the coordinated Cargo runner: `inline_tree_toolbar_relation_validates_and_copies_from_the_shared_fixture`, `conflict_resolution_buttons_are_inline_controls_before_row_selection`, and `conflict_resolution_buttons_share_the_row_and_win_its_pointer_band`.

The full native result remains owned by the coordinated root build because this source landed while the shared Cargo graph was compiling.
