# WGPU Browser Media Implementation

The browser host now mounts real audio/video controls from the accepted WGPU slot list and uses the same neutral media export lifecycle and AppChannel adapter as React. Temporary concealment preserves the operation, player, playback position and object URL. Actual owner/node/resource retirement cancels a running export or drains its completed output, removes the player and revokes its URL exactly once.

## Revalidated Audit39

The original missing browser media lane, discarded typed plugin handle and canvas-only browser boot were still present. The lifecycle existed beneath the React element. The audit's old canonical resource-validator findings were stale; the Rust/schema lane was already being repaired in current source. This change extracted shared lifecycle ownership rather than adding a second export implementation.

## Implemented Source

All engine paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

- `🎬️media/🚚️lifecycle/🟦️.ts`: neutral serial submit/poll/cancel/drain lifecycle, exact submitted owner/revision/generation and five-field subsequent handle checks. Refused submitted owners are never polled or cancelled. Completed output drains before MIME/size/length refusal; only validated terminal bytes become a URL.
- `🎬️media/📡️channel/🟦️.ts`: one shared typed AppChannel adapter for React and WGPU. Missing/ambiguous replies and guest faults fail closed, and cancellation requires its correlated Done receipt. The page bundle imports the small neutral lifecycle authority assertion and does not pull the full Os runtime codec into the browser-boot closure.
- `🎬️media/🌐️browser/🟦️.ts`: exact accepted descriptor parser and worker resource registry. Limits are 32 slots, 64 KiB descriptor JSON, 8 KiB commands, one operation in flight per token, 4096-byte output pages, and the existing 32 MiB/8192-page segmented export caps. Descriptor/resource owners match exactly; decimal node IDs and bigint revision/generation/operation IDs preserve u64 authority. Scalar string bounds agree with Rust and JSON Schema.
- `🎬️media/🌐️browser/🎛️host/🟦️.ts`: production imperative overlay keyed by accepted token/resource identity, CSS-pixel accepted bounds and clips, opaque theme background, isolated canvas input, explicit localized audio/video/play/pause/seek/selection/progress/cancel labels, no autoplay, known/unknown duration behavior, two independent selection sliders, native decode/play refusal, and exactly-once cleanup. Initial media position is applied on loaded metadata. Legitimate AbortError from pausing an unresolved play request does not destroy the player.
- `🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`: correlated lifecycle/request/token media command/result lane, bigint structured clones, bounded pending commands and deadlines, immediate rejection on retired authority/fault/quarantine/close, and worker release on timeout. Accepted descriptors are published only with accepted frame generation.
- `🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`: preserves typed plugin handles for initial and lazy mounts, resolves media ports from the owning plugin, transfers chunk ArrayBuffers, publishes registry-admitted accepted slots, serializes owner retirement before runtime teardown, and clears registry ownership on renderer/worker quarantine.
- `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`: uses the shared adapter and adds trusted app document identity query to the typed and serialized JSON Rust bridge.
- `🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`: mounts/reconciles/disposes the overlay at production page lifecycle boundaries.
- `🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`: suppresses duplicate native mirror media nodes while the DOM host owns them, updates its ownership signature, and restores fallback ownership on actual removal. Concealed hosts remain owned but are hidden/inert/aria-hidden so covered controls are not exposed.
- `🧱️elements/🎬️MediaTransportHost/🟦️.tsx` and `🎛️react/🤖️PluginRuntime/🟦️.tsx`: consume the neutral lifecycle/adapter. The React host receives the same initial-seek and legitimate interrupted-play corrections.
- Canonical media kit `🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts`: Unicode scalar length limits now match Rust/Ajv for controller, document, MIME and capability reason.

The sibling Rust lane owns `🎯️targets/🧊️wgpu/🎬️media-slots/🧬️contract/🔣️.json`, accepted Shell/snapshot/browser-slot collection and publication, stable SHA-256 resource owner tokens, trusted document lease transitions, concealment/occlusion geometry, native accessibility and the shared `🧫️fixtures/🎬️presented-media-slots/🔣️.json`. The parser admits zero accepted extents only when required `occluded` is true. Conceal/reveal never changes resource authority.

## Trusted App Document Identity

The canonical Os TS codec and AppChannelClient implement channel v20 ReadDocumentIdentity command tag41 and DocumentIdentity reply tag31. The identity uses u32 app instance and nullable nonempty document ID bounded to512 Unicode scalars. Sequence values remain exact u64; strict optional tags, malformed UTF8, overflow, trailing bytes and foreign/missing/ambiguous receipt identity are rejected. WGPU returns serialized camelCase JSON to the Rust ProgramBridge, matching the existing document bridge ABI. The query comes from the trusted guest document envelope rather than the media resource's own owner claim. The sibling Rust lane supplies the paired codec, envelope query, registry and neutral fixture/schema.

## Executed Validation

Executed through permanent Bun/Nx routes with task dependencies excluded and cache skipped:

| Gate | Observed result | Generated log |
| --- | --- | --- |
| `@semio-tech/framework-renderer-wgpu:test-browser long 🧪️tests/🎬️wgpu-browser-media/🟦️.ts 🧪️tests/🎬️presented-media-slots/🟦️.ts --run --silent=false --reporter=verbose --maxWorkers=1` | 2 files,27 tests passed,latest resumed gate,36.1s Nx | `🗑️generated/media-focused.log` |
| `@semio-tech/framework-renderer-react:test long ../../../../🧱️elements/🎬️MediaTransportHost/🧪️tests/♻️lifecycle/🟦️.tsx --run --silent=false --reporter=verbose --maxWorkers=1` | 1 file,8 tests passed,23:37:08 local,3.0s Nx | `🗑️generated/react-media-focused.log` |

The focused WGPU gate covers actual registry/channel/frame transport source, all five foreign subsequent handle fields, refused submitted resource owners, serial retirement during an in-flight submit, geometry preservation, worker chunk ownership transfer, fault retirement, pending rejection on close/fault/quarantine, duplicate AX suppression/restoration, native controls/input isolation, stale completion without URL publication, and shared trusted identity vectors/receipt behavior. The latest gate also validates the aggregate descriptor byte ceiling and real-app acceptance CLI/neutral journey fixture. Independent Ajv validates the same descriptor and identity schemas; the existing WebAssembly LEB128 library checks exact wire varints; node:crypto checks the shared SHA-256 token fixture. Shared language-neutral fixtures are persisted in domain folders.

Real Chromium executes an esbuild bundle of the production imperative host, not a copied DOM mock. Its transport bytes come from a bounded deterministic MediaTransportPort oracle. Chromium decodes the1second WAV and the existing canonical MP4 demo (24.016667seconds, positive native video dimensions). Both journeys check initial position250ms, no autoplay, play/pause, seek500ms, both selection sliders, accepted20px/30px placement, playing conceal/reveal with the same node/time and no URL revoke, followed by exactly one revoke on actual removal. Observed runtime receipts:

```text
[DEBUG] browser-media Chromium decoded 1s 20px/30px
[DEBUG] browser-media Chromium conceal/reveal playing=true same-player=true revoke=0
[DEBUG] browser-media Chromium retired submit -> poll -> take -> take revoke=1
[DEBUG] browser-media Chromium decoded 24.016667s 20px/30px
[DEBUG] browser-media Chromium conceal/reveal playing=true same-player=true revoke=0
[DEBUG] browser-media Chromium retired submit -> poll -> take -> take -> take -> take -> take -> take -> take -> take -> take -> take -> take revoke=1
[DEBUG] React media Chromium decoded 1s initial=0.25s
```

The existing React language-neutral fixture had a44byte zero-sample WAV header. Its real browser decoder correctly rejected it, so the fixture now contains a valid1second8044byte WAV split into two bounded pages. The production React host mounts and decodes that fixture in Chromium.

The focused Chromium oracles establish actual DOM/native decoder/control/resource behavior. They do not claim a complete guest export through Rust WASM, Shell collection and frame worker in one browser session; the parent owns broad Cargo/browser/package integration and activation evidence. The final worker quarantine retirement change is covered by the parent's full integration/source guard gate, not by a directly instantiated Worker in this focused suite.

## Ownership and Remaining Gate Coordination

No new runtime external dependency, compatibility shim, auxiliary script or modifying Git command was introduced. The old React-nested lifecycle source was removed and all live consumers moved to the canonical neutral path. Parent ownership covers permanent task/launch registration, generated source closure metadata, channel v20 census, broad Cargo/full browser/package gates and ticket cleanup/closure. Generated logs stay beneath the active ticket until that cleanup; this Markdown report remains.
