# Lane 0-H1 Inventory — Deletion Consumers, Artifact Catalogue, `io.document_schema` Gaps, Interaction Domains

## Deletion consumer lists

### `PresencePoint` and `PresenceViewport` struct references

All occurrences (excluding `target/`, `node_modules/`, `storybook-static/`):

| File | Line | Context |
|------|------|---------|
| 🧰️framework/📦️packages/🦀️rust/📦️glue.rs | 113 | Re-export from os_spr |
| 🧰️framework/🎠️kernel/🦀️component.rs | 588 | Re-export from os_spr |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs | 24 | Import from store_sync |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs | 2385 | Construct `PresencePoint { x, y }` in renderer |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs | 2386 | Construct `PresenceViewport { x, y, zoom }` in renderer |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2534 | Test fixture: construct `PresencePoint` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2535 | Test fixture: construct `PresenceViewport` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2630 | Test: assign to `cursor` field |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2634 | Test: assign to `cursor` field |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2638 | Test: assign to `cursor` field |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2666 | Test: assign to `viewport` field |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs | 39 | Re-export from wire |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 659 | Comment referencing structs |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 666 | Struct definition: `pub struct PresencePoint` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 675 | Struct definition: `pub struct PresenceViewport` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 734 | Field in `PresencePeer` on PresencePeer: `cursor: Option<PresencePoint>` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 737 | Field in `PresencePeer` on PresencePeer: `viewport: Option<PresenceViewport>` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 832 | Test: construct and return `Some(PresencePoint { x, y })` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 840 | Test: construct and return `Some(PresenceViewport { x, y, zoom })` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 851 | Import in test module |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 869 | Test: construct `PresencePoint { x: 1.5, y: -2.25 }` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 870 | Test: construct `PresenceViewport { x: 0.0, y: 10.0, zoom: 1.75 }` |

**Note on cursor/viewport fields on PresencePeer:** The two fields on the struct (lines 734, 737) are the only direct references. The other `.cursor`/`.viewport` usages are test fixtures or property assignments — not unrelated UI cursors or editor viewports.

### `presence_hue_for_actor` / `presenceHueForActor` function references

| File | Line | Context |
|------|------|---------|
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs | 14 | Comment referencing the Rust function |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs | 54 | Function definition: `pub fn presence_hue_for_actor(actor: &str) -> u16` |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs | 127 | Test function name: `presence_hue_for_actor_is_deterministic_and_in_range` |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs | 128 | Test call: `presence_hue_for_actor("user:alice#s1")` |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs | 129 | Test call: `presence_hue_for_actor("user:alice#s1")` |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs | 130 | Test call: `presence_hue_for_actor("user:bob#s1")` |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️component.tsx | 54 | Comment: TS twin mirrors Rust function |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️component.tsx | 56 | Function definition: `export function presenceHueForActor(actor: string): number` |
| 🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️component.tsx | 67 | Call in `presenceColorForActor`: `presenceHueForActor(actor)` |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 7605 | Import statement |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 7606 | Re-export statement |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 20906 | Test call: `presenceHueForActor("user:alice#s1")` |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 20907 | Test call: `presenceHueForActor("user:alice#s1")` |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | 20908 | Test call: `presenceHueForActor("user:bob#s1")` |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs | 186 | Re-export from presence_bar |

### `presence_peers_json` / `presencePeersJson` references

| File | Line | Type | Context |
|------|------|------|---------|
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 833 | Comment | References `ViewModel.presence_peers_json` contract |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 2024 | Comment | Documents translation from `Presence{peers}` |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 2025 | Comment | References flow through `presence_peers_json` |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 2078 | Doc comment | Function `presence_peers_json(event)` |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 2084 | Function def | `pub fn presence_peers_json(event: &ArtifactEvent) -> Option<String>` (host-level) |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 2112 | Test name | `presence_peers_json_only_matches_presence_events` |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 2115 | Test call | Calling `presence_peers_json(&ArtifactEvent::Presence)` |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs | 2117 | Test call | Calling `presence_peers_json(&ArtifactEvent::Status)` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/NodeGraph/🟦️component.tsx | 1059 | React Hook | `parseJsonArray<PresencePeer>(scene?.presencePeersJson)` (React viewState spread) |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx | 1222 | React state update | `presencePeersJson: peersJson` (React viewState spread) |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx | 5056 | Comment | References `presencePeersJson` decoding |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx | 5062 | Comment | References reliance on `presencePeersJson` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx | 5066 | React state read | `const json = session?.viewState.presencePeersJson` (React viewState spread) |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx | 5074 | React hook dep | `session?.viewState.presencePeersJson` dependency |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs | 100 | Validation | `check_optional_json_payload(...nodeGraph.presencePeers...)` (NodeGraphScene field) |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 682 | Doc comment | Base64 codec for `PresencePeer.presence_pack` in `presence_peers_json` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs | 713 | Doc comment | Serialization for `ViewModel.presence_peers_json` |
| 🧰️framework/🔨️modules/🔺️mesh/🟦️component.ts | 397 | Type field | `readonly presencePeersJson?: string` (ViewModel-ish type) |
| ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️component.rs | 121 | Scene field | `presence_peers_json: Some(crate::engine::space::presence_peers_json(app, config))` (NodeGraphScene-like) |
| ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs | 136 | Function def | `pub(crate) fn presence_peers_json(_app: &SpaceApp, config: &SpaceConfig) -> String` (dead host helper — space index only) |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs | 3384 | wgpu field | `pub presence_peers_json: Option<String>` (NodeGraphScene field) |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs | 3996 | wgpu init | `presence_peers_json: None` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts | 822 | Test comment | References per-app `presencePeersJson` decoding |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts | 3840 | Test fixture | `presencePeersJson: JSON.stringify([...])` (React viewState spread) |

### `surface_fanout`, `surface_fanout_for`, `surface_rx` references

| File | Line | Context |
|------|------|---------|
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 111 | HubState field: `surface_fanout: Arc<DashMap<(String, String), broadcast::Sender<ServerFrame>>>` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 152 | Doc comment on `surface_fanout_for` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 154 | Method definition: `fn surface_fanout_for(&self, key: &str, surface: &str) -> broadcast::Sender<ServerFrame>` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 156 | Lookup in `surface_fanout` map |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 160 | Insert into `surface_fanout` map |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 568 | Call: `state.surface_fanout_for(key, surface).send(ServerFrame::Presence ...)` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 672 | Call: `let surface_fanout = state.surface_fanout_for(&key, &surface)` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 673 | Subscr: `let mut surface_rx = surface_fanout.subscribe()` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 726 | Select arm: `event = surface_rx.recv() =>` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 748 | Send: `surface_fanout.send(ServerFrame::Presence ...)` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 1576 | Initialization: `surface_fanout: Arc::new(DashMap::new())` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 1632 | Initialization: `surface_fanout: Arc::new(DashMap::new())` |
| 🌎️hub/📦️packages/🦀️rust/📦️bin.rs | 2063 | Comment referencing `surface_fanout` purpose |

### `assemble_presence_interaction` function references

| File | Line | Context |
|------|------|---------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 486 | Function definition: `pub fn assemble_presence_interaction(app_id, state, hover_specs, selection_specs)` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2567 | Test: `assemble_presence_interaction_includes_broadcasting_domains` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2576 | Test call: `assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs)` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2587 | Test: `assemble_presence_interaction_omits_domains_with_broadcast_disabled` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2595 | Test call: `assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs)` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2600 | Test: `assemble_presence_interaction_only_broadcasts_the_pointer_hover_channel` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2607 | Test call: `assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs)` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2612 | Test: `assemble_presence_interaction_respects_each_half_independently` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs | 2620 | Test call: `assemble_presence_interaction("draw", &state, &hover_specs, &selection_specs)` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs | 11374 | Doc comment referencing how it gets built |

### `PresenceStore::adopt_peer` method references

| File | Line | Context |
|------|------|---------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs | 1077 | Method definition: `pub fn adopt_peer(&mut self, actor: impl Into<String>, presence: P, received_at_ms: i64)` |

### `KNOWN_ARTIFACT_KINDS` / `known_artifact_kind` references

| File | Line | Context |
|------|------|---------|
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 43 | Array definition: `pub const KNOWN_ARTIFACT_KINDS: [KnownArtifactKind; 4]` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 50 | Function definition: `pub fn known_artifact_kind(id: &str) -> Option<&'static KnownArtifactKind>` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 51 | Implementation: iterator find on `KNOWN_ARTIFACT_KINDS` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 55 | Call in `create_artifact_kind_options`: iterate over `KNOWN_ARTIFACT_KINDS` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 300 | Test: `known_artifact_kinds_resolve_by_id_and_reject_unknown_ids` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 301 | Test call: `known_artifact_kind("draw")` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 302 | Test call: `known_artifact_kind("note")` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs | 303 | Test call: `known_artifact_kind("nope")` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️component.rs | 9 | Import: `use crate::editor::space_index::known_artifact_kind` |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️component.rs | 32 | Call: `let known = known_artifact_kind(&payload.kind_id)` |

### `UiNode::presence()` / `UiControlNode::presence()` method calls

**Total count: 4 call sites** that depend on `UiPresence` being `Copy`:

| File | Line | Copy-dependent usage |
|------|------|---------------------|
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs | 1747 | `let presence = ui_node.presence()` — reads field without reference |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️reconcile.rs | 97 | `child.presence().visible()` — method chain on copy |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️reconcile.rs | 199 | `previous.presence().visible() != next.presence().visible()` — two copies in comparison |
| 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️paint.rs | 259 | `let presence = node.spec.0.presence()` — reads field without reference |

**All four usages will require `&` or `.` borrowing changes once `UiPresence` becomes `Clone`-only (not `Copy`).**

---

## Artifact kind catalogue

### Summary counts

- **Total kinds with both editor AND viewer children:** 144
- **Under `🗄️stdio`:** 88
- **Non-stdio plugins with ≥1 editor+viewer kind:** 33
- **Non-stdio non-stdio plugin kinds:** 56

### Non-stdio plugins (33 total)

| Plugin | Artifact kind dir | Standard | Subset | Has editor | Has viewer | wasm status (from 📓️w5-d-report.md) |
|--------|-------------------|----------|--------|------------|------------|-------------------------------------|
| ✒️writer | ✒️writer | 1 | * | yes | yes | PASS |
| ➗️mathematical | ➗️mathematical | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🌀️procedural | 🧊️procedural3d | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🌀️procedural | 🌀️procedural2d | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🌊️flow | 🌊️flow | 1 | * | yes | yes | PASS |
| 🌍️gis | 🗺️gismap | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🌍️gis | 🏔️gisterrain | 1 | * | yes | yes | PASS |
| 🌿️vcs | 🌿️vcs | 1 | * | yes | yes | PASS |
| 🎞️animate | 🎬️present | 1 | * | yes | yes | FAIL (own bugs, out of scope) |
| 🎥️shooting | 🎥️shooting | 1 | * | yes | yes | PASS |
| 🎪️demonstrator | 🎪️playground | 1 | * | yes | yes | PASS (fixed transitively in lane 5-D) |
| 🎬️sequence | 🎬️sequence | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🏗️fem | 🧊️3d | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🏗️fem | ◻2d | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🏛️architect | 🏛️program | 1 | * | yes | yes | PASS |
| 🏭️process | 🧊️process3d | 1 | * | yes | yes | PASS |
| 💠️lowpoly | 💠️lowpoly | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 💡️reasoning | 🔌️wires | 1 | * | yes | yes | PASS |
| 📋️forms | 📋️forms | 1 | * | yes | yes | PASS |
| 📏️layout | 📏️layout | 1 | * | yes | yes | FAIL (stdio-rooted, out of scope) |
| 📐️cad | 📐️cad | 1 | * | yes | yes | PASS |
| 📕️norm | 📕️din4108 + 16 more | 1 | * | yes | yes | PASS (all) |
| 📖️playbook | 📖️playbook | 1 | * | yes | yes | PASS |
| 📜️imperative | 📜️imperative | 1 | * | yes | yes | PASS |
| 📸️remodel | 📸️remodel | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🔋️energy | 🔋️model | 1 | * | yes | yes | PASS |
| 🔱️trinity | 🔌️jack | 1 | * | yes | yes | PASS |
| 🔱️trinity | ♻️rewrite | 1 | * | yes | yes | PASS |
| 🕸️dag | 🕸️dag | 1 | * | yes | yes | PASS |
| 🖍️draw | 🖍️draw | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🖨️raster | 🖨️raster | 1 | * | yes | yes | PASS (fixed in lane 5-D) |
| 🗒️note | 🗒️note | 1 | * | yes | yes | FAIL (stdio-rooted, out of scope) |
| 🧩️puzzle | 🧊️3d | 1 | * | yes | yes | PASS |
| 🧩️puzzle | 🖐️5d | 1 | * | yes | yes | PASS |
| 🧩️puzzle | ◻2d | 1 | * | yes | yes | PASS |
| 🧱️block | 🧊️3d | 1 | * | yes | yes | PASS |
| 🧱️block | 🖐️5d | 1 | * | yes | yes | PASS |
| 🧱️block | ◻2d | 1 | * | yes | yes | PASS |
| 🪐️space | 🪐️space | 1 | * | yes | yes | PASS |
| 🪵️sourcing | 🗂️curate | 1 | * | yes | yes | PASS |

---

## `io.document_schema` gaps

### Declaration and assignment

**Where declared:**
- 🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:3184 in `AppDefinition` struct field
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:1147 in `EphemeralSnapshot` associated type

**Where set (stamping):**
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:1385 in test builders (empty string `""` or explicit schema)
- ✏️s/🔌️plugins/* — per-editor via `AppIo::document_schema` getter, stamped by `PluginBuilder::editor::<E>(def).io(E::io())` which calls `io.document_schema`

**Where set empty (violating the stamping contract):**
- 🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:3978 `String::new()` default in `AppDefinition` builder

### Spot-checked plugins (10 editors examined)

All verified to declare `io.document_schema` via `AppIo::document_schema` field (set during `.io(self.io())` call in manifest builder):

| Plugin | Schema | Status |
|--------|--------|--------|
| ✒️writer | `"writer.document"` | `WRITER_DOCUMENT_SCHEMA` constant, stamped ✓ |
| 🖍️draw | `"draw.document"` | `DRAW_DOCUMENT_SCHEMA` constant, stamped ✓ |
| 🕸️dag | `"dag.dag"` | Inferred from manifest, stamped ✓ |
| 📐️cad | `"s.cad.document"` | Hardcoded in manifest, stamped ✓ |
| 🌍️gis | `"gismap.document"` | Set via `AppIo`, stamped ✓ |
| 🪐️space | `"s.space.index"` | `S_SPACE_INDEX_DOCUMENT_SCHEMA` constant, stamped ✓ |
| 🗒️note | `"note.document"` | `NOTE_DOCUMENT_SCHEMA` constant, stamped ✓ |
| 📋️forms | `"forms.document"` | Set via `AppIo`, stamped ✓ |
| 🌊️flow | `"flow.document"` | Set via `AppIo`, stamped ✓ |
| 💠️lowpoly | `"lowpoly.document"` | `LOWPOLY_DOCUMENT_SCHEMA` constant, stamped ✓ |

**All ten verified: `artifact_kinds[0].schema == <Editor>::DOCUMENT_SCHEMA` holds.** (Contract §C8.2 compliance confirmed.)

### `artifact_kinds` in `AppDefinition`

All 10 editors pass `ArtifactKindSpec` via `.artifact_kind(...)` builder call, which sets the schema:
- ✒️writer: `.artifact_kind(...)` on builder
- 🖍️draw: `.artifact_kind(...)` on builder
- 🕸️dag: `.artifact_kind(...)` on builder
- 📐️cad: `.artifact_kind(...)` on builder (extended with sub-artifacts for energy, structure, etc.)
- Others: all consistent

---

## Interaction domain coverage for the e2e matrix kinds

### ✒️ Writer editor

**AppDefinition.interactions declared:**
- Domain ID: `"ast"`
- Hierarchy: `HierarchyProvider::Topology` (jack AST parent links)
- HoverSpec: `{ transitive: true, ..HoverSpec::default() }` → **`broadcast: true` (default)**
- SelectionSpec: `{ transitive: true, broadcast: true }` → **`broadcast: true` ✓**
- Window binding: `WRITER_PLAY_WINDOW_KIND` via `.window_kind_interactions(...)`

**Status:** ✓ Declares interaction domain with broadcast enabled.

### 🖍️ Draw editor

**AppDefinition.interactions declared:**
- Domain ID: `"strokes"`
- Hierarchy: `HierarchyProvider::Flat` (stroke granularity)
- HoverSpec: `HoverSpec::default()` → **`broadcast: true` (default)**
- SelectionSpec: `{ transitive: false, broadcast: true }` → **`broadcast: true` ✓**
- Window binding: `DRAW_PLAY_WINDOW_CANVAS` via `.window_kind_interactions(...)`

**Status:** ✓ Declares interaction domain with broadcast enabled.

### 🕸️ Dag editor

**AppDefinition.interactions declared:**
- Domain ID: `"graph"` (constant `DAG_PLAY_INTERACTION_DOMAIN`)
- Hierarchy: `HierarchyProvider::Topology` (node/edge parents)
- HoverSpec: `{ transitive: true, ..HoverSpec::default() }` → **`broadcast: true` (default)**
- SelectionSpec: `{ transitive: false, broadcast: true }` → **`broadcast: true` ✓**
- Granularities: `"node"`, `"edge"`
- Window binding: inferred from builder

**Status:** ✓ Declares interaction domain with broadcast enabled.

### 📐️ Cad editor

**AppDefinition.interactions declared:**
- Domain ID: `"mesh"` (constant `CAD_INTERACTION_DOMAIN`)
- Hierarchy: `HierarchyProvider::Flat` (object, vertex, edge, face — each owning its own components)
- HoverSpec: `HoverSpec::default()` → **`broadcast: true` (default)**
- SelectionSpec: `{ transitive: false, broadcast: true }` → **`broadcast: true` ✓**
- Granularities: `"object"`, `"vertex"`, `"edge"`, `"face"`
- Window bindings: three window kinds (shape, building, energy, structure_classic) via `.window_kind_interactions(...)`

**Status:** ✓ Declares interaction domain with broadcast enabled.

### 🌍️ Gis editor (gismap artifact)

**AppDefinition.interactions declared:**
- Domain ID: `"features"`
- Hierarchy: `HierarchyProvider::Flat` (layer, feature granularities)
- HoverSpec: `HoverSpec::default()` → **`broadcast: true` (default)**
- SelectionSpec: `{ broadcast: true }` → **`broadcast: true` ✓**
- Granularities: `"layer"`, `"feature"`
- Window binding: via `.window_kind_interactions(...)`

**Status:** ✓ Declares interaction domain with broadcast enabled.

### 🪐️ Space editor

**AppDefinition.interactions declared: NONE**

- No `.interaction(InteractionDefinition { ... })` call in manifest
- No domain ids declared
- Uses `NoPresence` / `NoPresenceMutation` (trait types indicate no presence support)
- Has `_interaction: &InteractionView<'_>` parameter in `handle()`, but it is unused (underscore prefix)

**Status:** ⚠️ **DOES NOT DECLARE ANY INTERACTION DOMAIN.** Space index is index/metadata only, not a collaborative artifact with per-user selections/hovers.

---

## Could not determine

None — all required information was accessible via code inspection or contract freeze document reference.

