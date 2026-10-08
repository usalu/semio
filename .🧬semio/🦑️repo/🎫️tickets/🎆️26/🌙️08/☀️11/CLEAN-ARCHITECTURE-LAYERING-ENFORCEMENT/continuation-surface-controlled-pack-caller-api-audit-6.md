# Surface Controlled Pack Caller API Audit

Read-only fresh source, no edits or runtime reruns.

Current GraphHost sync_from_scene_pack471 accepts bytes only, calls Store wire decode then document fallback, projects to Serde JSON, and clones JSON subtrees through NodeGraphScenePayload::from_json. No NativeDecodeControl is supplied by its current caller API. WASM808 simply forwards bytes. This is the exact active boundary to replace.

Minimal owning Rust signature proposal: sync_from_scene_pack(&mut self, bytes:&[u8], format:semio_framework_pack::intrinsic::IntrinsicFormat, options:&semio_framework_pack::record::DecodeOptions, control:&mut NativeDecodeControl)->Result<(),NodeGraphError>. Use existing intrinsic::decode directly; it moves exact value with explicit Body/Document and requires neither cloned one-field bridge nor retry parser. If JSON text is needed, its output needs separate caller NativeEncodeControl/units or a retained preparation operation, not hidden unbounded conversion inside this signature.

Existing General JSON JsonWriteCursor<DslValue>::new takes original decoded owner without copying, advances bounded with NativeEncodeControl, and take_source transfers only after complete. Generic to_json_string_controlled instead calls to_value_controlled, which projects/clones the DslValue; avoid it for this no-copy slice. Borrowed write_json_source_into is also available but must retain exact source custody and explicit sink limits. Cancellation closes the original decoded owner and writer, independently of output bytes. Typed scene construction needs borrowed/moving General Value conversion rather than today's Serde subtree clones if full owner projection is claimed.

Runtime format provenance: renderer sceneToSyncPack2220 calls framework-os encodePackValue1977, which writes Body. Runtime873/884 should choose Body. Native fixture350–359 calls Rust encode_pack_value document container, so original test should choose Document. Do not change its encoded bytes just to avoid the extra format parameter. WASM source and authored TS interface need an explicit closed format parameter or typed prepared request plus limits; every caller selects a known producer format. Unknown format refuses. No byte-only legacy shim or sniff/fallback.

Native Serde oracle opportunity: same neutral nodes/edges/viewport payload encoded independently to Body and Document, then exact chosen decoder yields identical scene facts and retained graph snapshot. Opposite format refusal, truncation/extra bytes, integer/float carrier preservation, Unicode ids, original output/owner retirement/cancel and exact limits need schema-first cases. Serde is oracle, not new canonical exported runtime API.

Caller inventory below includes current Rust owner/WASM/native fixture, renderer interface and calls/mocks, styling builder GraphSession mock, Infinite demand-frames mock, and generated bindings. Generated files must be regenerated through actual owner build; don't handcraft API shims. Styling EditorSession similarly named method is a distinct owner: inspect independently before changing it mechanically. Full Surface owning/alltargets execution must qualify the cut; no source-only success claim.

```text
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:136:export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:137:export class EditorSession { syncFromSceneJson() {} syncFromScenePack() {} setText() {} text() { return ''; } caret() { return 0; } anchor() { return 0; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScrollScreen() {} insertText() {} backspace() {} deleteForward() {} selectAll() {} replaceSelection() {} selectionText() { return ''; } hoverTokenRangeJson() { return 'null'; } setHoverRange() {} cameraJson() { return '{}'; } }
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface_bg.wasm.d.ts:43:export const graphsession_syncFromScenePack: (a: number, b: number, c: number, d: number) => void;
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:84:    syncFromScenePack(bytes: Uint8Array): void;
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:279:    readonly graphsession_syncFromScenePack: (a: number, b: number, c: number, d: number) => void;
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.js:859:    syncFromScenePack(bytes) {
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.js:864:            wasm.graphsession_syncFromScenePack(retptr, this.__wbg_ptr, ptr0, len0);
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:472:    pub fn sync_from_scene_pack(&mut self, bytes: &[u8]) -> Result<(), NodeGraphError> {
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:808:        #[wasm_bindgen(js_name = syncFromScenePack)]
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:809:        pub fn sync_from_scene_pack(&self, bytes: &[u8]) -> Result<(), JsValue> {
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:810:            self.state.borrow_mut().host.sync_from_scene_pack(bytes).map_err(|e| JsValue::from_str(&e.to_string()))
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs:361:fn graph_host_sync_from_scene_pack_decodes_pack_shell() {
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs:370:    host.sync_from_scene_pack(&bytes).expect("sync");
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🧪️tests/🪶️demand-frames/🟦️.tsx:57:          syncFromScenePack: () => undefined,
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:100:  syncFromScenePack?(bytes: Uint8Array): void;
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:873:      sessionRef.current?.syncFromScenePack?.(scenePack);
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:884:        sessionRef.current.syncFromScenePack?.(scenePack);
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:931:      syncFromScenePack: () => {},

```
