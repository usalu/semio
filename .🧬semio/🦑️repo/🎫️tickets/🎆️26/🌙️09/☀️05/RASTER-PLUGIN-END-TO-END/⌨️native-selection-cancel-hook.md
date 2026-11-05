# Native Selection Escape Hook

Read-only source finding, sent to the native-selection implementer and parent.

The key route is `PointerCtx::handle_keyboard_async` in renderer engine `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs:303`. Existing retained-content dispatch at 445–453 consumes a key before idle Escape deactivates utility/tool at 456–467. Pending selection cancellation must be offered before those consumers, with the existing modal/field precedence preserved.

The active retained focus is already obtained by `self.as_session().retained_keyboard_focus()`. Resolve its scene through `semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::retained_scene_target(window_id, node)`; implementation is Interpreter `🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📕️close-scene-pointer-and-dispatch/🦀️.rs:101`. Its ScenePointerTarget identifies surface kind and host_id. `scene_pointer_target_is_live(&target)` at 152 validates window/component generation and retirement before acting.

Existing engine-board `paint2d::paint2d_pointer_cancel_into(surface_id:&str)->bool` is in EngineCanvas `🎯️targets/🧊️wgpu/🖌️paint2d/🦀️.rs:601`. It resets marquee, removes/cancels PAINT2D_PIXEL_CANDIDATES and cancels host pointer work. Its return currently means a raster host existed, not that pending selection was cancelled; a focused Escape-selection API should return true only when it removed actual pending work, so idle Escape can still deactivate utility normally afterward.

Interpreter `cancel_scene_pointer` only traverses SCENE_POINTER_OWNERS (same file:206–247). Pointer release removes that owner, so that route alone cannot cancel a still-running selection candidate after release. Generic Scenes `🎯️targets/🧊️wgpu/⌨️scene-input/🦀️.rs:635` cancel_canvas_interactions handles Canvas2d gesture/catalogue state and does not include Paint2d candidates. No source changes or runtime verification were performed.
