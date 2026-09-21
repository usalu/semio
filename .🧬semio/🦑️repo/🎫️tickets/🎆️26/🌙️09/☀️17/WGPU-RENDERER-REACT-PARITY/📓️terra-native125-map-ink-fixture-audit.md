# Native 125 Map and Ink Fixture Audit

This is a read-only source review. Native 125's reported Map failure was not rerun.

## Map sibling rebase

The original fixture had one setup-only defect, now corrected in the source snapshot. TiledMap produces HitKind::ScrollRegion, not HitKind::ComponentScene, in the retained-hit factory ([input](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:712)). Filtering the local hit list for ComponentScene therefore could never find a painted Map receiver.

[The law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:827) constructs exactly the neutral sibling sequence from the shared fixture: `[2]`, then `[2,4]`, then `[2,3,4]`. It mounts every record as a retained document, so receiver protocol id 4 must move to a different arena node when id 3 is inserted before it. Its final `assert_ne!(presented.node, accepted.node)` is therefore a real arena-rebase check, not a fabricated node substitution.

The fixture reaches the actual retained document path. [`paint_retained_map_document`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:155) repeatedly invokes [`render_ui_document_step`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2596), which owns document ingress, reconciliation, layout, and `frame_into_step` paint ([phase transitions](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2632)). That path writes the UI engine's retained hit registry. The law then:
1. calls [`register_clipped_retained_hit_targets`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:848) before presentation;
2. seals and acknowledges the same candidate ([lines 849–853](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:849));
3. selects receiver id 4 from the returned scene registration ([lines 855–859](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:855)).

That order matches the Shell's production body paint: it registers the candidate retained hits before sealing ([Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12970)), then promotes on GPU acceptance ([Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13018)). The registration helper intentionally derives a scene target from the candidate tree ([Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:833)). Reordering it after ACK would be less production-faithful and would hide a real candidate/presentation or retained-hit failure.

The receiver's wire `surface_id` is intentionally different from the document/window id. The document is owned by `map.lifecycle.sibling-rebase`, while the scene wire retains `map.lifecycle.shared-wire`; that is the documented host/wire separation. Do not collapse those ids in the fixture.

**Current result:** the corrected law consumes register_clipped_retained_hit_targets' returned scene target and matches protocol document id 4 directly ([lines 848–859](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:848)). This is the production-faithful fix. It retains the real candidate/presented protocol and must not be replaced with a direct fabricated target or an after-ACK lookup. No further Map fixture defect is evident statically.

## Ink2 fixture state

There is one concrete cleanup defect. [`presented_ink_intent_finishes_before_an_unchanged_same_host_sibling_refresh_is_accepted`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:1146) and [`presented_ink_intent_uses_old_authored_content_before_a_changed_same_host_successor_starts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:1185) drain their own queued intent at entry and cancel their pointers at exit, but neither closes its unique `UI_ENGINE` document window. `publish_ink_intent_document` admits a retained window ([lines 612–620](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:612)); it remains in the global UI surface registry after each test.

This is not isolated local state: [`UI_ENGINE`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:711) owns retained windows across these helpers, and the bounded surface registry is the same owner whose close route is exercised by [the existing close law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs:1224). Repeated tests on one worker can retain the new `presented-ink-intent-*` windows until the fixed registry capacity is consumed.

After each Ink intent law, use the existing test route:
- `assert!(request_ui_document_close(window_id));`
- bounded `while ui_document_close_pending() { assert!(close_ui_document_one()); }`
- assert `UI_ENGINE.with(|cell| cell.borrow().surface_token(window_id).is_none())`
- preserve the current pointer cancellation before closing, then assert `scene_interaction_terminal_is_empty()`.

That is a test teardown only. It must not replace the intent/ACK assertions or use a global reset.
