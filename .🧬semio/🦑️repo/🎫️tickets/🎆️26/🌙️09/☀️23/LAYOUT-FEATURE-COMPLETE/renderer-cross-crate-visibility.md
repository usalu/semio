# Renderer Cross-Crate Visibility

Read-only audit of `cargo-artifacts-GArScM.log` and `cargo-artifacts-pLyO8L.log`; source verified live. Beyond `TextEditorExplicitDraftStatus`, these warnings affect actual other-crate callers:

- `EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🧾️receipts-and-explicit-draft/🦀️.rs`: `TextEditorRetainedActionBatch` is `pub(crate)` but public `TextEditorRetainedActionStep::Ready` exposes it. The renderer frame transaction matches the variant at `🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs:358`.
- `EngineCanvas/🎯️targets/🧊️wgpu/🧩️engine-surface-attach/🖌️paint-editor-and-composite/🦀️.rs`: `RasterCompositeProgress` is `pub(crate)` while `advance_raster_composite` is public. `Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs:323` consumes it in the separate `🪞️scene-render` crate.
- `Scenes/🎯️targets/🧊️wgpu/🧩️board2d-and-vfs/🦀️.rs` exposes private `VfsVisibleRow` and `VfsMetrics` via public functions. `Scenes/🎯️targets/🧊️wgpu/📰️event-feed-and-timeline/🦀️.rs` exposes private `EventFeedLayout`. The separate scene-render crate imports these functions and uses the returned values at `🎨️scene-render-entry/🦀️.rs:107–143`; fields used there are already public.

The logs also warn about Gumball helper types, but identified direct external uses of those particular helpers are tests; production uses `*_into` APIs. No build was run.
