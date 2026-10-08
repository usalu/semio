# Exact Schema Facet Admission and Scene Format Audit

Read-only current evidence; no runtime/schema-inventory rerun or source edit. Disappearance actor remains unidentified.

## Defining schema grammar

Discovery schemaScopeOwnerLevel3081 rejects excluded owners and fixture collections, then checks declared owner-level patterns. Framework-module pattern admits General Value retirement, Record retirement and Pack JSON owners unless an excluded/test/fixture ancestor applies. inventorySchemaScopes3437 enumerates directories whose own name is 🧬️schema, derives their owner, and extracts canonical facets.

Crucial additional rule3465–3468: canonical filename is accepted only when every directory segment below a schema module is in taxonomy.schemaChildDirs. Current exact list is 📸️snapshot, 🔺️diff, 🧬️mutations, 💡️inferences. Arbitrary capability directories below schema are not canonical facets. Thus schema/🎮frontier, schema/🎮erased-controlled, schema/🎮retirement, schema/🧩members, schema/🧫finite are not admitted via this predicate. A 🧫 segment can additionally mark an example collection. Merely putting a leaf outside fixtures does not settle facet eligibility.

Stable canonical layout is defining capability owner/<capability>/🧬️schema/🔣️.json, with capability outside schema and not named a test/fixture collection, or one root defining retirement/🧬️schema/🔣️.json containing appropriate $defs. Root canonical schema document must declare addressable $id under taxonomy idBase, valid scope segments and canonical facet; schema-module-id-missing otherwise. Nested actual schema modules own themselves separately and need their own root identity. Do not expand taxonomy to admit arbitrary fixture-specific paths merely to bypass this contract.

Present snapshot: Value retirement schema module exists with erased-controlled/frontier leaves; Record retirement exists with finite leaf; JSON schema exists with members leaf but retirement leaf absent. This neither confirms prior body equivalence nor explains disappearance. Current validator diagnostics are not deletion evidence. No actor/process can be attributed from these observations.

## Actual framing authority

General Pack intrinsic::decode already accepts explicit IntrinsicFormat::{Body,Document}, DecodeOptions, NativeDecodeControl, returns moved DslValue and typed PackRefusal. Use it directly; no additional cloned value bridge/fallback required.

Surface GraphHost Rust471 and wasm GraphSession808 currently accept bytes only; wrapper809 forwards without framing. Native original fixture350–359 encodes with OS Rust encode_pack_value (document container), therefore must choose Document while retaining all scene assertions.

Runtime NodeGraph TS sceneToSyncPack2220 imports encodePackValue from framework-os76. Its definition OS TS1977 writes symbol count/symbols, fieldcount1/fieldid1/TAG_VALUE then payload, with no document header: actual runtime framing is Body. Calls873/884 must explicitly choose Body. Do not infer Document from the name encodePackValue. The shared entrypoint needs declared format and actual caller controls; expose a canonical format wire enum/schema to WASM/TS rather than try another parser.

Update Rust owning API, WASM wrapper, renderer interface100, calls873/884, renderer mock931, styling builder mock136, Infinite demand-frames mock57 and generated bindings by their actual build owner. EditorSession styling mock137 is a different session; inspect its owning API before changing it by textual resemblance. Keep existing native Document law plus Body runtime contract, wrong-format refusal, immutable inputs, cancellation/owned limits and exact output. Generated d.ts is derived declaration evidence, not a substitute for canonical source edits.

## Source roster

```text
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:136:export class GraphSession { lodScaleJson() { return dagLodScaleJson(); } syncFromSceneJson() {} syncFromScenePack() {} labelOverlayPaintStateJson() { return '{"labels":[]}'; } selectionUnionBoundsScreenJson() { return '{}'; } selectionPreviewPointsJson() { return '[]'; } selectionPreviewCrossing() { return false; } selectionPreviewMethod() { return 'rectangle'; } selectedNodeIdsJson() { return '[]'; } hoveredNodeId() { return null; } hoveredChannelJson() { return '{}'; } viewport() { return { x: 0, y: 0, zoom: 1 }; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScreen() {} }
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:137:export class EditorSession { syncFromSceneJson() {} syncFromScenePack() {} setText() {} text() { return ''; } caret() { return 0; } anchor() { return 0; } pointerDownScreen() {} pointerMoveScreen() {} pointerUpScreen() {} wheelScrollScreen() {} insertText() {} backspace() {} deleteForward() {} selectAll() {} replaceSelection() {} selectionText() { return ''; } hoverTokenRangeJson() { return 'null'; } setHoverRange() {} cameraJson() { return '{}'; } }
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface_bg.wasm.d.ts:43:export const graphsession_syncFromScenePack: (a: number, b: number, c: number, d: number) => void;
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:84:    syncFromScenePack(bytes: Uint8Array): void;
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts:279:    readonly graphsession_syncFromScenePack: (a: number, b: number, c: number, d: number) => void;
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:471:    pub fn sync_from_scene_pack(&mut self, bytes: &[u8]) -> Result<(), NodeGraphError> {
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:807:        #[wasm_bindgen(js_name = syncFromScenePack)]
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:808:        pub fn sync_from_scene_pack(&self, bytes: &[u8]) -> Result<(), JsValue> {
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:809:            self.state.borrow_mut().host.sync_from_scene_pack(bytes).map_err(|e| JsValue::from_str(&e.to_string()))
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs:350:fn graph_host_sync_from_scene_pack_decodes_pack_shell() {
🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs:359:    host.sync_from_scene_pack(&bytes).expect("sync");
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🧪️tests/🪶️demand-frames/🟦️.tsx:57:          syncFromScenePack: () => undefined,
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:100:  syncFromScenePack?(bytes: Uint8Array): void;
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:873:      sessionRef.current?.syncFromScenePack?.(scenePack);
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:884:        sessionRef.current.syncFromScenePack?.(scenePack);
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx:931:      syncFromScenePack: () => {},

```
