# Window Pane Owner Parity

## Evidence and Scope

Checkpoint 17 step 21 recorded the active window as `puzzle3d-main-top/framework.section.engagements` after Abort. Actions then carried that document identifier in `windowId`. The checkpoint raw output was removed by concurrent workspace cleanup on September 21; the observed values remain recorded in the checkpoint reports and conversation tool receipts.

Source inspection confirms that pointer activation, accessibility focus, action normalization, and keyboard focus currently recognize only the Measures surface. Actions and Search use separate retained documents with reserved suffixes but belong to the same concrete dock window. The fix must preserve the retained surface for event delivery while keeping activation and guest action scope on the concrete window.

## Shared Contract and Regressions

`Shell/🧫️fixtures/🪪️window-surface-owner/🔣️.json` and its JSON Schema describe body, Measures, Actions, and Search for one concrete window. The native registered shell-input laws exercise pointer activation and real published Select accessibility focus. The React registered Window law mounts the actual Window component and exercises activation capture, focus, and typing for all four body kinds; Ajv validates the shared contract.

New native filters:

- `retained_pane_pointer_activation_preserves_the_concrete_window_owner`
- `retained_pane_accessibility_and_keyboard_focus_keep_surface_and_window_distinct`

The React oracle passed under the existing `@semio-tech/ui-react:test` Nx target: 1/1 selected test, all four fixture rows, 566 unrelated tests outside the filter, 3.93 seconds Vitest / 4.5 seconds Nx, zero cache hits. Receipt: `🗑️generated/astra-runtime/pane-owner-react-1.log`.

A third native law, `retained_pane_actions_address_the_concrete_window_before_guest_admission`, calls the actual action funnel with a window-scoped utility change. It stops at the intentional missing-program admission boundary and verifies that the preceding host-side utility interception used only the concrete window ID.

Native66 ended at compile failure after 11m50s: six missing chrome metric API errors were expected from the fail-first registration, and one `active_stack` fixture assertion needed `Some(vec![1])` instead of `vec![1]`. That test-only type mismatch is corrected. No tests ran in Native66. Chrome production is released to its owner; the owner-routing production fix remains held for Native67 runtime failure.

## Panel Focus Extension

The shared fixture now also names an ordinary Preferences panel. `retained_panel_focus_routes_keyboard_without_activating_an_application_window` uses a real published Select, actual accessibility focus, the open anchor, and a subsequent hide. It requires keyboard ownership while open, unchanged application activation, and no keyboard route once hidden. The actual React `Panel` counterpart mounts a tree input beside a real active Window and checks pointer capture, focus, typing, and unmount on hide.

The two selected React laws passed together through Nx with zero cache hits: 2/2 selected, 566 outside the filter, 8.14 seconds Vitest / 8.9 seconds Nx (`pane-owner-react-2.log`). Native67 stopped at compile failure after 4m52s because the in-flight settings page queue's service exports were not yet available. No native tests ran. The queue owner is completing that API before Native68. No native owner result or production fix is claimed yet. The read-only audit requires a single FocusChanged-driven surface tracker, exact sibling clearing, and live document validation rather than choosing among stale entries in a map.

## Focus Sequence

The fixture now orders focus through Measures, Search, Actions, and body, then sends a late blur from Measures. The actual mounted React Window counterpart passes that sequence: all four pointer activations name the same concrete window, DOM focus always follows the newest input, and an old sibling blur leaves the latest input focused. The selected React owner suite is now 3/3 passed, 566 unrelated tests outside the filter, 4.13 seconds Vitest (`pane-owner-react-3.log`).

The native counterpart `retained_pane_focus_follows_the_last_focus_event_and_ignores_an_old_surface_blur` publishes four real retained Select documents and routes actual accessibility events. It additionally requires logical sibling focus to be cleared and closes all document owners before assertions. This new law was registered while Native68 was still compiling dependencies. Root's new Rust laws parse under rustfmt; they remain unverified until the native result.
# Native68 Failure and Source Repair

All five new native owner/focus laws executed and failed in Native68. The existing mounted React oracle passed all three selected tests (566 outside filter), and the root implementation now resolves only the reserved Measures, Actions, and Search pane suffixes to a concrete window. Pointer activation, accessibility activation, and action admission share that resolution.

Keyboard focus tracks the newest real FocusChanged event. A previous surface's late blur cannot erase the newer focus. Lookup requires a visible, currently focused retained tree and the same live generational node; open panels may own keyboard input without changing the active application window. True document removal clears the tracking entry; ordinary keyed document replacement preserves it for validation against the successor. Native69 is running; production behavior is not yet accepted as passing.
