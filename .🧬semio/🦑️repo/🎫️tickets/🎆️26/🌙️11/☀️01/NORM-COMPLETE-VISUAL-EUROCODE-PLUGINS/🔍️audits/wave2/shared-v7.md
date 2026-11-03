# Shared v7 (wave 2)
Nothing compiled (no cargo). Python run: twins generator, sectionproperties beta_p smoke test.

## 1. Viewer ephemeral state
The framework needed no change: `ArtifactViewer` already has `Transient`, `ephemeral` and `render_with_request_context`; the viewers used `NoTransient`. New `app-surface/👁️viewer-surface` (live-editor crate): `NormViewerSurface`, `view_lane_mutations`, `viewer_ephemeral`, generic `NormViewBoundedCommandJobFactory<V>` over `ViewerApp<V>`, macros `norm_viewer_lane_stores!` (transient only), `norm_viewer_commands!`, `norm_view_command_from_action!`, `norm_viewer_surface!`, `norm_owned_view_tool_job_factory!`, ownership arm `(lanes)`. All ten en19xx viewers wired by `🧰️scripts/shared-v7-wire-viewers.py`: `view_commands!` row `viewportAct`, transient lane `crate::lanes`, `ephemeral`, `render_with_request_context` (checks window selects the stepped check, viewport renders the lane), manifest action, tool proofs, Cargo deps; window fns `render_lane`, `render_selected`. Tests: viewer-surface unit tests (check, governing, combination steps, wrap, no-op) and two tests per viewer.
## 2. Requests
See requests.md "Shared v7": done 831, 1137, move item, traffic-group lane, entity tools; obsolete 585, 971; kept with reasons.
## 3. EDITOR-UX wiring
LiveCommand copy/duplicate/paste + tools (ten editors, fixture), scene snap in `gesture_handles`, `shortcut_route`, `touch_menu`, `bulk_rows`, clip JSON; tests in entity-editing, command-args, live-acceptance macro. Host key handler and UI renderers remain.
## 4. Compile review
Reviewed by reading: all new shared code and the viewer consumer files. NOT reviewed (no capacity): entity-editing internals, editor-wiring older parts, viewport3d parts/mesh/lowering, focus-order, forms, handles schema, ndp tables (changed since integrate/4 by other agents).
## Risks
`semio_framework_job`/`action_bus` paths in the viewer factory macro need the viewer crate to resolve them like the editors do (editors declare no such dependency either); `ViewerApp` retained reducer type coercion; `ChecksViewState`/`ViewportView` fields; `ActionKind::Mutation` use for duplicate/paste; panels budget unchanged except ~120 lines in entity-editing.
