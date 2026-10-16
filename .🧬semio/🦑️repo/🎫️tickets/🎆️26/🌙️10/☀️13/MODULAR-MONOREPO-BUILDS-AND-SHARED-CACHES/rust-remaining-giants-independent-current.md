# Remaining Rust Giants Independent Inventory

Read-only repository Source/manifests inventory on 2026-10-15. No compilation, tests, mounted caches, process probes or implementation edits. Existing Error/Crypto extractions are retained and are not proposed again.

## Method and Limits

Traversed 321 framework Cargo manifests and each declared library's literal mod/path/include Source graph using Python's TOML parser. Excluded paths in test directories; counted exact on-disk bytes, files and physical lines for the discovered sets. This is an all-feature approximation, not an actual cfg-selected compiler closure: inline tests, mutually exclusive features, macro-generated paths and unresolved default modules can alter active Source. Consequently these are the exact top five of this static census, not a claim about exact active compiler rankings or boundary violations. Physical lines are not maxActiveLines measurements.

## Top Five Static Production-Path Roots

| Root | Bytes | Files | Physical lines | Authored local boundary bytes/files/active lines | Unresolved paths |
|---|---:|---:|---:|---|---:|
| semio-framework-plugin | 6,041,531 | 185 | 85,910 | 1,048,576 / 128 / 12,000 | 1 |
| semio-framework-os-renderer-wgpu | 5,859,042 | 139 | 107,578 | none in package metadata | 1 |
| semio-framework-os-kernel | 3,949,667 | 150 | 61,433 | 1,048,576 / 128 / 12,000 | 0 |
| semio-framework-os-infinite | 2,238,125 | 69 | 43,772 | none in package metadata | 0 |
| semio-framework-ui | 2,206,054 | 82 | 44,119 | 1,048,576 / 128 / 12,000 | 10 |

Package manifests and direct nominal dependencies:

- Plugin: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`. Direct families include job publication/fault/worker/authority, composition admission, plugin ownership/mounted owner, retirement owner, Pack/Pack JSON, replication, DSL/record/derive, schema state/composition/registry, value/derive, async/job/actor, tool-run/time-travel/tool-machine, kernel, UI WGPU/contract/runtime/scene, geometry/2d, diagnostics/trace/hash, base64, serde/json and wit-bindgen. Largest discovered files: reactor runtime 391505 bytes and plugin time-travel 209731 bytes.
- Renderer: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`. Direct nominal dependencies: job worker, worker authority, Pack, value derive, canvas, value, DSL record, Pack JSON and UI locale. Shell command dispatch is 143210 bytes; chrome paint/navbar is 137976 bytes.
- Kernel: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`. Direct families include extracted store link/blob/ephemeral/one-item/envelope/command/cursor/projection/message-ledger owners; directory client/history/pack/VCS/command/inference/identity/channel/directory; Pack source/JSON/retained-schema, mutation diff, IO control/sqlite snapshot/schema/base64; composition admission, value snapshot/retirement/value/derive; plugin component receipt; schema registry/state/composition/validator; DSL/record/derive; UI viewport/locale; job publication/worker/authority/job, async/diagnostic/hash/trace, serde/json. Store root remains 774556 bytes and sync remains 317454 bytes.
- Infinite: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`. Direct families: already-extracted directed presentation, Pack/Pack JSON, canvas, artifact reference, UI locale/viewport/runtime/contract/styling, value/derive, kernel, job/tool-run/async/framework, geometry, graph manifest/graph/drawing, serde/json/base64/hash/pixels. World scene is 126082 bytes; directed normal scene-delete-links is 124271 bytes.
- UI: `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`. Direct families: job worker/authority, existing surface identity/draw contract/prepared/raster owner/retained tree/select behavior/arena/action/theme/text/declaration; value/derive, styling/scene/contract, geometry/2d, job/pixels/async/locale, serde/json, bytemuck, wgpu and taffy. Draw implementation is 303828 bytes; WGPU engine is 223496 bytes.

## Focused Pack and Replication

Pack (`framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml`): 804976 bytes / 33 files / 11292 physical lines, no local boundary metadata, no unresolved literal paths. Direct dependencies: Pack source, IO schema/sqlite snapshot, diagnostic, Pack error, DSL record, Pack JSON, replication, value, async and hash. Format is 209299 bytes; value layer is 184343 bytes.

Replication (`framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml`): 1039702 bytes / 57 files / 16737 physical lines, no local boundary metadata, no unresolved literal paths. Direct dependencies: already-extracted replication Error and Crypto, Pack source/JSON/error, mutation diff, trace/diagnostic, schema registry/state, value/derive, hash/deflate/base64 and serde/json. Wire is 166382 bytes; mutation is 87836 bytes.

## Small Domain-Owned Next Cuts

1. **Kernel document sync session/transport owner.** `store/🔄️sync/🦀️.rs` explicitly owns actor scheduling, remote mutation sessions, file watching and socket integration. Move actual SyncSession/host state machine and its IO ports, rather than a reexport, into a store-sync domain crate. This removes a 317454-byte implementation from kernel's owner closure and offers a concrete location for async/replication transport dependencies. Existing store command/envelope/cursor/projection extractions should remain dependencies, not be recreated. A cycle/dependency-use audit is required before claiming any kernel dependency can disappear entirely.
2. **UI WGPU renderer ownership.** Move actual GPU pipeline/resource/draw implementation from the 303828-byte draw module plus required shader/resource ownership into a target-specific renderer crate consuming existing draw-contract/prepared/raster contracts. Pure declaration users can then avoid the WGPU implementation edge. Do not recreate existing draw-contract or raster-owner facades; move owned resources and render execution.
3. **Pack binary format owner.** The format module documents a complete byte-level SPK container, writer/file pair, verification/recovery and codec contract. Move those implementations plus their required primitive format types/limits into a leaf Pack-format crate using existing PackSource/Sink and PackError. This gives pure format readers a dependency route that does not require Pack's replication-facing layer. Merely moving the existing module behind a reexport would not achieve that dependency cut.
4. **Replication wire frames/codecs owner.** Wire already treats presence and mutation payloads as opaque bytes and owns lane/frame enums plus binary dispatch. Move the actual frame types and encoder/decoder implementation into a wire domain crate with narrow primitive codec dependencies; keep causal mutation/session logic in replication. Error/Crypto are already extracted and are not candidates.
5. **Plugin time-travel runtime extension.** The existing general time-travel dependency is not the plugin's 209731-byte state owner. Isolate actual plugin-specific time-travel state/commands with explicit plugin ownership ports. This is smaller than cutting the entire reactor and can remove code from plugin's root closure; prove dependency separation before asserting kernel/UI weight disappears.

Renderer shell dispatch and Infinite directed scene-delete logic remain additional large domain owners; Infinite directed-presentation already exists, so repeating that extraction would be a facade rather than a new cut. This inventory is planning evidence only and authorizes no edits or boundary increases.
