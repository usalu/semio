# Remaining Rust Giants Current Refresh

2026-10-15. Read-only current authored Source/manifests census after the independent prior inventory. No compilation, tests, caches, targets, vendor traversal or implementation edits. Exact measured bytes below are sums of current existing files reachable through literal path-module/include declarations from each manifest's lib root, excluding test-directory paths. This bounded parser does not resolve every implicit module or cfg branch, so these are measured discovered closures, not exact active compiler closures or directly comparable historical deltas. Missing literal candidates are reported honestly. Dependency counts are distinct nominal keys in the ordinary dependencies section (including following target sections before dev-dependencies); no transitive compiler fanout is claimed.

| Current crate | Discovered Source bytes | Files | Unresolved literal paths | Nominal dependency keys |
|---|---:|---:|---:|---:|
| semio-framework-plugin | 6064270 | 187 | 0 | 44 |
| semio-framework-os-renderer-wgpu | 5879777 | 140 | 0 | 9 |
| semio-framework-os-kernel | 3949667 | 150 | 2 | 68 |
| semio-framework-os-infinite | 2218746 | 68 | 1 | 32 |
| semio-framework-ui | 2206054 | 82 | 10 | 35 |
| semio-framework-replication | 1039702 | 57 | 0 | 15 |
| semio-framework-pack | 17516 | 7 | 0 | 21 |

## Current Ownership And Prior Cuts

Pack is no longer the old 804976-byte owner. Its current manifest directly depends on separate format, format-catalog, format-retained, record, record-encode, record-preflight, record-retained, record-schema, async/http/IO/intrinsic owners. It now declares its own 262144-byte/32-file/4000-active-line boundary. Recommending the former Pack format/value extraction again would ignore current peer work. The small root still has 21 nominal dependency edges: root Source shrink is not proof that every caller avoids the complete aggregate graph. Root Cargo also now explicitly lists wasm-prototype optimization overrides for new Pack format children, so profile policy must follow the new ownership rather than the old package snapshot.

Replication Error and Crypto remain genuine separate manifest edges. Wire remains 166382 bytes and mutation 87836 bytes; no separate wire owner is declared in the inspected replication manifest. A naive whole-wire leaf is not immediately clean: ClientFrame embeds causal MutationEnvelope/FrontierSummary, and artifact bootstrap metadata likewise embeds causal frontiers. A cut must assign those schema owners deliberately rather than create a dependency cycle or a second copied type family.

Plugin's largest current implementation remains reactor runtime (391525 bytes) and plugin-specific time-travel (209942 bytes). Time-travel directly imports kernel HistoryMutation and action/schema families and uses super::*; it requires explicit plugin domain ports before an independent crate can own it. Renderer shell command dispatch (143210 bytes) and chrome paint/navbar (137976 bytes) remain distinct UI domain seams beneath the target renderer's nine direct nominal edges.

Kernel still owns store root (774556 bytes) and document sync (317454 bytes) despite existing store link/blob/ephemeral/command/envelope/cursor/projection/message-ledger cuts. Its 68 nominal dependency keys show broad aggregation, not proof that each is used by every active unit. Infinite still owns world scene (126082 bytes) and directed scene-delete-links (124302 bytes); directed-presentation was already extracted and should not be recreated. UI still owns WGPU draw (303828 bytes) and WGPU engine (223496 bytes), alongside already extracted draw contracts/prepared/raster owners.

## Selected Next Cut: Target-Owned UI GPU Rendering

Create one actual target-specific UI WGPU render-execution owner under `framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/📦️packages/🦀️rust/Cargo.toml`. Its domain is GPU pipeline/resource creation, draw execution, owned buffer/texture lifetime and matching shader programs. Move actual implementation from draw and directly required shader ownership, not a facade/reexport layer. Existing target-neutral draw types, prepared/raster contracts, theme values and scene resource/lease contracts remain their existing owners and become direct dependencies. Public API must expose own repository contracts; framework GPU ABI objects stay behind the target implementation interface.

The inspected draw imports clearly establish this boundary: prepared raster pages/content identity, existing draw_types, authored/painted/blur/glass/vector/world shader programs, Theme, scene pass/profile/material and mesh source lease. The mesh lease/return ownership must remain exact through resource retirement; do not replace it with integer identities or copied receipts. `super::*` or references to the aggregate UI crate must not enter the new child. If a needed scene contract currently lives only inside aggregate UI, move its genuine schema/contract owner first in the same cut; do not make the child depend back on UI.

Affected Cargo manifests are the new draw owner manifest, `framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` (consume the new actual renderer owner and remove dependencies only once no remaining own implementation uses them), and the root workspace `Cargo.toml` member/profile registration. Current UI engine can remain the higher orchestration owner and directly call this renderer. The product renderer manifest `products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml` should change only if its actual imports require direct render ownership after the API move; no speculative dependency removal is justified.

This is one semantic execution/resource ownership cut, not a line threshold split. TDD should retain language-neutral draw output/resource lifetime laws and independent raster/output oracle while testing cancellation, detached accepted rendering drain and lease retirement. No compilation-speed, runtime-speed, binary-size or dependency removal claim follows from this Source-only inventory.
