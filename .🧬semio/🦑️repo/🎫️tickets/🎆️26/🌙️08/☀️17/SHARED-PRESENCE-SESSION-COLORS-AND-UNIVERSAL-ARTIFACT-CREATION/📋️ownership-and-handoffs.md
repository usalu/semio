# Ownership and handoffs

One writer per file per wave. Everyone else files a `sharedFileRequest:` block in their report.
Waves have hard barriers: the coordinator merges every report and runs the audit before the next wave
starts.

## Wave 1 — wire · hub · channel/ABI · manifest · transport · palette

| Lane | Model | Lease (exclusive write) | Gate commands |
|---|---|---|---|
| **P1** wire + client actor | Sonnet 5 | `💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs` · `📡️spr/🦀️component.rs` · `💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` · `💻️os/🟦️component.ts` **presence + ServerFrame/ClientFrame + BackboneWorkerResponse regions only** · `💻️os/🟦️backbone-worker.ts` · `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` (re-export list) · `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (re-export list) · `💻️os/🖥️host/🦀️component.rs` (delete dead `presence_peers_json` + its test) | `cargo test -p semio-framework-os-kernel --lib --features sync` · `cargo check -p semio-framework-os` · `cargo check -p semio-framework` · `bun nx run @semio-tech/framework-os:test` |
| **P2** hub | Sonnet 5 | `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` · `💻️os/🔨️modules/📇️directory/🧬️schema/{🦀️component.rs,🟦️component.ts,🔣️component.json}` | `cargo check -p semio-hub` · `cargo test -p semio-hub --lib` · `cargo test -p semio-hub --bin os-hub` (default features only) |
| **P3** channel + plugin ABI + wrapper + UiPresence | Sonnet 5 | `💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` · `🧫️fixtures/📡️channel/**` · `💻️os/🔨️modules/🔌️plugin/🦀️component.rs` **except** the `PluginBuilder`/testkit regions (→ 1-A) · `💻️os/🟦️component.ts` **`AppChannelCodec` + `AppChannelClient` regions only** · `📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx` · `📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs` · `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/**` (`UiPresence`, `UiPeerMark`, scene `domain_id`, every `presence()` call site) · `🛂️manifest/🟦️component.ts` (UiTree TS twin only) | `cargo test -p semio-framework-os-kernel --lib channel` · `cargo test -p semio-framework-plugin --lib` · `cargo check -p semio-framework-ui` · `cargo check -p semio-framework-os-renderer-wgpu` · `bun nx run @semio-tech/framework-os:test` |
| **1-A** manifest + builder | Sonnet 5 | `🧰️framework/🔨️modules/🛂️manifest/{🦀️component.rs,🟦️component.ts,🤖️generated/**}` · `💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` · `💻️os/🔨️modules/🔌️plugin/🦀️component.rs` **PluginBuilder + testkit regions only** | `cargo test -p semio-framework --lib` · `cargo test -p semio-framework-plugin --lib` · `bun nx run @semio-tech/framework:check` |
| **1-B** worker transport | Sonnet 5 | `💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts` · `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` **`PluginWorkerClient` region only** | `bun nx run @semio-tech/framework-renderer-react:test` |
| **1-C** native plugin path | Sonnet 5 | `📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs` **`load_wasm_plugins` only** (coordinate with P3) · `📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{📦️glue.rs,📦️bin.rs,📜️script.ts}` | `cargo check --bin semio-wgpu-native --features native-bin` · `cargo test -p semio-framework-os-renderer-wgpu --lib program_bridge` |
| **P6** palette + tokens | Sonnet 5 | `🖱️ui/🎨️styling/**` · `🖱️ui/🧱️elements/👥️PresenceBar/{🧊️component.rs,🟦️component.tsx}` **`🔖️Palette` region only** · wgpu `🦀️theme.rs` | `bun nx run @semio-tech/ui-styling:test` · `cargo check -p semio-framework-ui-styling` · `cargo test -p semio-framework-ui --lib presence` |

Cross-lane notes for W1: P1 lands the type/frame skeleton first (~1 h) so P3 compiles against it;
P3 owns every `UiPresence` `Copy → Clone` call site in one pass; 1-C touches `load_wasm_plugins`
only, P3 touches the rest of `ProgramBridge` — coordinate through the coordinator if both need the
file at once.

## Wave 2 — shells · overlay core · creation

| Lane | Model | Lease | Gate |
|---|---|---|---|
| **R-A** overlay core | Sonnet 5 | new `🖱️ui/👥️presence/{🦀️component.rs,🟦️component.ts}` + glue mounts · new `🖱️ui/🧱️elements/👥️PeerOverlay/{🟦️component.tsx,🧊️component.rs,🧪️story.tsx}` · ui-react `📦️index.tsx` `👥️PeerOverlay` region · `📺️renderer/…/🧱️elements/Interpreter/{🟦️component.tsx,🧊️component.rs}` · `Scenes/🧊️component.rs` (signature only) · `🔺️mesh/🟦️component.ts` (`ComponentSceneHostProps.uiPath`) | ui + renderer tests |
| **R-F** coloring | Sonnet 5 | wgpu `🦀️paint.rs`/`🦀️draw.rs`/`🦀️shaders.rs`/`🦀️gpu.rs` · `🪵️Tree/🟦️component.tsx` · ui-react inline presence rings · `World3dHost` palette hook | ui + renderer + react tests |
| **2-B** React shell | Sonnet 5 | `📺️renderer/…/🧱️elements/ShellHost/🟦️component.tsx` · `Shell/🟦️component.tsx` · `ShellHelpers/🟦️component.tsx` (dialog/arg regions) · `⚛️react/🧪️index.test.ts` | `bun nx run @semio-tech/framework-renderer-react:test` |
| **2-C** wgpu shell | Sonnet 5 | `📺️renderer/…/🧱️elements/Shell/🧊️component.rs` | `cargo test -p semio-framework-os-renderer-wgpu --lib` |
| **2-A** space plugin | Sonnet 5 | `✏️s/🔌️plugins/🪐️space/**` | `cargo test -p semio-s-plugin-space --lib` · `cargo check -p semio-s-plugin-space --target wasm32-wasip2` |
| **2-D** STEP-2 diagnosis | Sonnet 5 | read-everything; edits only via `sharedFileRequest` to 2-B/2-C; **owns the `verify collab` runner this wave** | `bun ./📜️script.ts verify collab` |
| **G** PresenceBar | Sonnet 5 | `🖱️ui/🧱️elements/👥️PresenceBar/**` · `📚️I18n` presence keys | ui tests |

## Wave 3 — per-surface overlays · kind coverage · harness

| Lane | Lease |
|---|---|
| **D1** React 2D | `Canvas2dHost` · `Board2dHost` · `NodeGraph` · `Paint2dHost` · `InkCanvasHost` (`🟦️component.tsx`) |
| **D2** React World3d | `World3dHost/🟦️component.tsx` · `♾️infinite/🌍️world/🎨️r3f/🟦️component.tsx` |
| **D3** React map + text | `TiledMapHost` · `TextEditor` (`🟦️component.tsx` wiring) |
| **D4** React tabular | `Table` · `BlockListHost` · `DiffViewHost` · `EventFeedHost` · `GraphTimelineHost` · `📜️HistoryTable` |
| **E1** wgpu 2D | `Scenes/🧊️component.rs` Canvas2d/InkCanvas/Paint2d/NodeGraph/Board2d regions · `EngineCanvas/🧊️component.rs` |
| **E2** wgpu World3d | `♾️infinite/🌍️world/🦀️component.rs` |
| **E3** shared engines + wgpu map/text | `🗺️surface/{🕸️node-graph,🗺️tiled-map,🎨️paint}/🦀️component.rs` · `✍️editor/🦀️component.rs` · puzzle `BoardHost` · `Scenes` TextEditor/TiledMap regions |
| **E4** wgpu tabular | `Scenes` Table/BlockList/DiffView/EventFeed/GraphTimeline · `🦀️paint.rs paint_tree_item` |
| **3-B** kinds | `✏️s/🔌️plugins/{✒️writer,🖍️draw,🕸️dag,📐️cad,🌍️gis}/**` (view reporting + broadcast declarations only) |
| **3-A** harness | dev `📜️script.ts` `🔖️CollabE2e` region · `📋️project.json` · `.vscode/🧩️launch.seed.jsonc` + registry generate |

## Wave 4 — runs, native, closure

`4-A` native wgpu e2e · `4-B` wgpu browser e2e · `4-C` matrix run ×3 · `4-D` storybook + parity spec ·
`4-H` final audit (Haiku, read-only).

## Permanently coordinator-owned

`📋️contract-freeze.md` · `📋️worker-brief.md` · `📋️ownership-and-handoffs.md` · `📌️important.md` ·
every `ticket_*` call.
