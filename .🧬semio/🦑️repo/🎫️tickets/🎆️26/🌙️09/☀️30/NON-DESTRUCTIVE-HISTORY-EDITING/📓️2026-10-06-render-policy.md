# Rendering Policy Trace — 2026-10-06

Current source trace only; this is not runtime proof.

The ordinary guest request context renders the current body through instance.app.render(body, None, view_state) before publishing presence. All four ordinary ArtifactView seams use the shared time-travel snapshot and child-content selection. Explicit snapshot overrides enter a separate render branch through plugin_render_with_document, a public helper which also permits documentJson inside its private WindowRenderInput payload. Repository rg finds no callers of plugin_render_with_document other than plugin_render, whose explicit snapshot argument is None, and no renderer caller directly constructing this private payload. That reduces the risk of an ordinary active viewport bypass but does not prove the helper cannot be used externally.

The explicit branch currently constructs ArtifactView::new and has no child render context. It should be treated as an isolated projection render rather than the active editing viewport. A future explicit public render-purpose contract should own snapshot selection before this helper is used for active editor requests. For the current task, native/React acceptance must observe the ordinary request-context path and retained scenes, plus history inspector bodies. Do not change export/thumbnail semantics solely from an unused branch audit.
