# Native Selection Frame Scheduling Audit

Read-only source audit; no runtime result is claimed.

The first inspected Ui::frame_step has a DIRTY_PAINT cache gate (`ui/🎯️targets/🧊️wgpu/⚙️engine/🎞️frame-step/🦀️.rs:54–58`), but this is not the production window-body path. It is insufficient evidence that native selection advancement stalls behind a cached scene. This distinction was corrected promptly to the implementing agent.

Production shell body rendering calls render_ui_document_step (`renderer engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs:703`), which calls Ui::frame_into_step (`Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs:320`). Production `ui/🎯️targets/🧊️wgpu/⚙️engine/🖥️frame-into-step-and-frame/🦀️.rs:25` opens a new RetainedPaintFrame unconditionally when none exists; it has no early DIRTY_PAINT cache return. The shell resets document cursors between pane walks, and ShellChromeFrameCursor starts with a default child on each new frame. Therefore phase5 is reachable again on a genuinely new frame with an unchanged visible document.

Both native redraw_on_original_caller and browser redraw_offscreen_worker enter build_and_publish_snapshot_on_original_caller and then FrameBuildHandle.poll_runtime_on_original_caller (`engine/🎯️targets/🧊️wgpu/🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs:223–279`). No production cached-scene bypass was identified in this traced chain. Accepted animated border primitives arm AcceptedAnimationClock for native/browser deadlines; the browser pending-selection predicate additionally requests continued frame work.

The proposed once-per-frame phase5 grant plus phase8 progress rendering is source-consistent for a visible mounted pane. Verify with an unchanged-document regression using actual production frame_into_step/document consumer, rather than the test helper frame/frame_step, which deliberately behaves differently. It must execute multiple new frames, show intermediate progress and complete/cancel without a new document revision. Also verify tab/window visibility transitions: a retained host that is still valid but no longer painted must not leave the pending-work predicate requesting frames forever.

Exact paths and the corrected production/test distinction were sent to native_selection and parent. No invalidation API or cache-bypass change is justified solely by the alternate frame_step gate.
