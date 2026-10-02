# JPG Owned Output and Raster State

## Actual authority and failure

The owned JpgSnapshot has seventeen fields: schema; u32 width/height; RGBA bytes; optional re-encode quality; two version bytes; density enum and two u16 density fields; optional thumbnail; optional frame; marker; arithmetic flag; ordered quantization definitions; ordered Huffman definitions; optional restart interval; ordered intrinsic segments. Its actual schema permits independent dimensions and byte arrays, including partial final pixel channels. Native logical state codecs retain those fields; an ordinary JPEG encoder is a distinct wire boundary.

The neutral intermediate fixture declares width zero, height u32MAX, five RGBA octets and a zero-width/255-height thumbnail with five RGB octets. Both its authored JSON schema and the actual owner snapshot schema admit it through Ajv. The registered source suite executed five laws: four passed and one genuinely failed,36assertions /14.6s uncached Nx, rejecting `JPEG pixels must fill their declared grid`. Evidence: `🗑️generated/jpg-owned-intermediate-raster-source-red.log`.

The matching native gate executed eleven laws: eight passed and three genuinely failed (Nextest `f97925e2-16a6-4bba-926c-4f4120169987`,0.566s assertions /1m19 uncached Nx). Failures were the missing controlled output in actual erased bridges and the long-octet law, plus the grid restriction in the literal partial-state fixture. The previous native/typed SQLite results predate mandatory controlled output dispatch and remain historical.

## Mounted literal ownership

The explicit relational RGBA/RGB entities have dense stream ordinals and literal channel values. A partial final entity retains nullable trailing channels, with no gaps and no intermediate partial entities. Snapshot dimensions remain independent scalar fields. This preserves every octet and permits independent SQL channel edits without any JPEG carrier or hidden JSON. Named `baseline` still invokes its exact owner semantic checker and preserves its diagnostics separately from wildcard logical state.

The native output owner explicitly forecasts domain rows before materializing metadata: document, channel entities, optional thumbnail/frame, unique authored quantizer identities, frame components,64coefficients per quantization definition,16code-length entries plus each Huffman symbol, and each ordered segment/octet. Literal seventeen-field controlled construction uses one native control, exact first-party ToValue constructors and intrinsic byte helpers. The existing guarded physical RecordSpecProducer/Text/Pack helper emits the actual native vocabulary. The old approximate preflight remains a separately tested direct guard, without factory authority.

## Current owned paths

- JPG document snapshot ownership fixture/schema and source/native SQLite laws.
- Executed controlled output131072-octet neutral fixture/schema/law.
- Handwritten SQL channel shape and matching Rust/TypeScript projection/reconstruction; one shared authored SQL text asset, with explicit TypeScript string export and declaration.
- Explicit literal native controlled record projection and whole-owner output hook; four owned intrinsic byte serializer annotations.

The repaired selected native gate executed eleven laws, all passed (Nextest `4f335fbe-e13d-486f-9941-054d55953b64`,4.036s assertions /1m35 uncached Nx). It exercises full erased Binary/Text state, long-octet output cancellation and partial raster preservation. The complete owning quick suite subsequently executed133/133,zero skips (Nextest `61c32832-1fe0-46ef-96ba-540d41263881`,4.244s assertions), including factory structural identity, shipped fixtures and ordinary JPEG wire regressions. Log: `🗑️generated/jpg-full-owner-controlled-current-quick.log`.

The strengthened source suite executed5/5,41assertions,including malformed trailing-channel refusal,24.2s uncached Nx. The public package check genuinely reported TS2307 for the imported SQL asset; the explicit adjacent declaration reference now matches the established PDF asset boundary. Its fresh registered uncached check passed in6.0s. Logs: `🗑️generated/jpg-partial-channel-malformed-source-current.log` and `🗑️generated/jpg-source-public-declarations-current-green.log`.
