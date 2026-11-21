# Generate form navigation

The Generate form windows in both editors now render the page selected for that window. Previous and Next are shared semantic buttons with bilingual labels, and the page indicator announces updates. The navigation stays above the scrollable fields. Each page has a separate viewport identity, so it enters at the top and keeps the original question and generation bindings.

The host intercepts page activation before artifact command dispatch. The existing body refresh scheduler carries the explicit one-page request separately from scroll observations. Each request addresses the original body and mounted window. The default first page releases its request. Two windows of the same form kind can browse different pages.

## Verification

- Chromium mounted the production React interpreter, scheduler, window chrome and stylesheet with the same 70-field partition used by the native composition fixture. In English and German, keyboard activation reached both pages, disabled the unavailable direction, and reset the newly mounted field viewport. Editing the first question on the second page sent the original generation and question IDs. Page actions sent no document commands.
- All 136 editable-control and window-scheduling tests passed. The new test rejects a mismatched window address, retains independent pages while a scroll report crosses, and validates the produced view with AJV against the shared view-context schema.
- The pinned native harness compiled the actual shared page presenter and WGPU scheduler source. It passed localized controls, the window-addressed binding, refresh coalescing, return to page one, and complete retirement of the real UI arena.
- The registered 70-field native composition law now renders through the actual Generation3d form entry. Native shell routing and app owners passed Rust parsing. The linked composition law and a mounted canonical WGPU journey remain unexecuted. The working volume held 0.289 GiB free against the 5.5 GiB linked build reserve.
