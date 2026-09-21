# Native132 Retained-Window Fixture Audit

## Scope and evidence

Read-only source audit of the Native132 window activation, retained capture, and focus cluster. No build or test was run. The findings below are fixture publication defects in the current source; they do not establish a production routing defect.

## Confirmed fixture defects

### 1. Accessibility targets must read the accepted generation

`retained_pane_focus_follows_the_last_focus_event_and_ignores_an_old_surface_blur` constructs every `AccessibilityTarget` with `window_generation: 1` at [wgpu-shell-input/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:579). The related single-surface focus tests do the same at lines 546 and 635.

That is no longer a valid identity. `Ui` allocates `document_generation` globally and assigns it only on first admission of a window at [ui engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1553); the value is retained through ordinary refresh. Accessibility dispatch accepts only the published generation at [ui engine](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2870). A shared native runner may therefore give the fixture any nonzero generation, even though its local shell begins empty.

**Fixture repair:** after the one all-documents `paint_tree_pointer_documents` call has sealed and acknowledged the candidate, obtain each row's advertised `windowGeneration` from the existing accessibility dump/projection and use it to form that row's target. Do not reset the engine or weaken the generation comparison. The dump is already built from `engine.surface_generation` at [Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:3650). If a native test-only query is needed, expose that same exact surface-generation projection rather than duplicate or guess `1`.

Fail-first law: publish all four body/Measures/Actions/Search documents in one candidate; retrieve their accepted generations; focus them in fixture order; then send a blur with the first row's same accepted generation. Each focus must activate the owning dock window and clear the prior sibling, while the late blur must not clear the latest focus.

### 2. The modal world probe mutates state after its authority was accepted

`retained_world_sequence_probe` paints, seals, and acknowledges all retained-world rows at [wgpu-shell-input/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:319). Its `modal` step subsequently assigns `context_menu` directly at line 356.

The ingress correctly consults only the accepted geometry: `pointer_owner_at` reads `presented_input_geometry.modal` at [Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13342). That field is copied from `candidate_pointer_input_is_modal` only while sealing at [Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13000). It is therefore expected that an unpainted, post-ACK menu does not block the world.

**Fixture repair:** make modal creation a pre-paint state transition, then invoke the existing all-documents `paint_tree_pointer_documents` helper again before the press. That helper performs one visibility pass, body rendering, retained-hit registration, seal, and ACK for every current row. `candidate_pointer_input_is_modal` already derives the required flag from `context_menu` at [Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13321). No production change is indicated.

Fail-first law: with both world rows already published, open the fixture menu, publish one new complete candidate containing both rows, then press and release in the overlap. The accepted modal geometry must cause zero World intents, no active-window change, and no pointer capture.

### 3. One candidate must cover every simultaneously visible document

The all-documents helper starts one accessibility staging pass, renders every supplied document, and seals/ACKs once at [wgpu-shell-input/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:765). This is the correct shared chrome seam for the four-pane focus fixture. Publishing each pane as an isolated candidate and then manually adjusting the accessibility-visible set has no corresponding presented frame and can make the input registry, visibility set, and generation witness disagree.

The helper must receive the dock plan before it seals. `activate_window_under_pointer` reads the accepted `presented_input_geometry.dock_window_plan`, not mutable current layout, at [Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13388). The current retained-world probe now assigns its plan before the helper at [wgpu-shell-input/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:317), which is the required ordering.

## Conclusion

Use the complete-all-documents helper as the sole fixture presentation seam, derive accessibility generation from the accepted projection, and republish after every modal-state transition. These repairs preserve the production invariant: input can observe only a GPU-accepted candidate.
