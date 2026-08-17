# Contract freeze — Shared Presence, Session Colors and Universal Artifact Creation

Frozen by the coordinator at W0. Every lane implements against THIS document. A lane that needs a
change here STOPS and writes a `sharedFileRequest:` block in its report; only the coordinator edits
this file.

Predecessor contract (still binding, not repeated in full): `26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END/📋️contract-freeze.md`
sections **C0** (ports/env/ids/test-id grammar), **C1** (directory schema, event-sourced), **C2** (hub
HTTP/WS), **C3** (client identity `os.config.identity`), **C4** (`s.space` artifact), **C5** (save and
check-in), **C6** (directory command flow). Amendments below extend it.

Ports unchanged: hub **8787**; `s` react **6072** (user1) / **6073** (user2); wgpu **6067** / **6068**;
admin vite 8790; e2e scan pool 7400–7498. Actor id `user:{user_id}#{shell_session_id}`. Surface id
`<kind>@<standard>/<subset>#<role>`.

---

## C7 Presence

### C7.0 Scope model (the core rule)

- **Artifact presence** — rendered from *every* peer of the document, regardless of which app/surface
  they run: `interaction` (selection + hover per interaction domain, matched by `domain` id; `app_id`
  is informative only) and `views` (cameras + in-view pointer, in artifact coordinates, matched by
  `space` id).
- **App presence** — rendered only from peers whose `surface` equals the local surface:
  `presence_pack` (app-typed `ArtifactApp::Presence`), `drag_ghost_json`, `ui` (data-ui-path
  hover/focus/press).
- The own actor is excluded from peer rendering everywhere. The local user's own hover/selection ring,
  marquee and 3D select color use the **own** session color.
- The roster is **document-wide** at the hub. `?surface=` stays on the document WS URL (the hub needs
  it for `ConnectionView` and directory presence) **and** the peer carries `surface` inside
  `PresencePeer` (stamped by the client actor, never by a shell).

### C7.1 `PresencePeer` v3

Rust `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs` region `🔖️Presence`;
TS twin `🧰️framework/🛍️products/💻️os/🟦️component.ts` `ArtifactPresencePeer`.

```
PresencePeer {
  actor: String,                        // always
  connected_at_ms: i64,                 // always
  label?: String,                       // bit 0
  presence_pack?: Vec<u8>,              // bit 1   APP scope (ArtifactPack of ArtifactApp::Presence)
  user_id?: String,                     // bit 2
  role?: String,                        // bit 3
  drag_ghost_json?: String,             // bit 4   APP scope
  interaction?: PresenceInteraction,    // bit 5   ARTIFACT scope (type unchanged)
  color?: u8,                           // bit 6   hub-assigned palette index, stamped by the client actor
  surface?: String,                     // bit 7   canonical surface id
  views: Vec<PresenceWindowView>,       // bit 8   ARTIFACT scope; set iff non-empty
  ui?: PresenceUi,                      // bit 9   APP scope
}

PresenceWindowView { window_id: String, space: String, kind: PresenceViewKind, size: [f64; 2], pointer: Option<[f64; 3]> }
PresenceViewKind =
    Canvas { x: f64, y: f64, zoom: f64 }
  | Orbit  { position: [f64; 3], target: [f64; 3], up: [f64; 3], fov: f64 }
  | Geo    { lng: f64, lat: f64, zoom: f64, bearing: f64, pitch: f64 }
PresenceUi { hovered_path: Option<String>, focused_path: Option<String>, pressed_path: Option<String> }
```

- `space` is the coordinate-space id the surface host reports: `"world"` (3D scenes), `"canvas"`
  (2D canvases / boards / node graphs), `"geo"` (tiled maps); an app may declare a finer id. An
  overlay renders a peer view only in local surfaces with the **same `space`**; `window_id` is for
  de-duplication and labels.
- `size` is the reporting surface's pixel size (needed to draw a peer's viewport rectangle).
- `pointer` is in view coordinates: world point (Orbit), `[x, y, 0]` canvas point (Canvas),
  `[lng, lat, 0]` (Geo).
- `hovered_path`/`focused_path`/`pressed_path` use the `data-ui-path` grammar `type[idx]#id/...`.
- `PresencePoint` and `PresenceViewport` and the fields `cursor`/`viewport` are **deleted**.
  Every consumer must be updated (checklist in `📌️important.md`).

**Binary layout** (`encode_presence_peer`/`decode_presence_peer`, TS `encodePresencePeer`/`decodePresencePeer`):

```
actor str | flags varint_u64 | connected_at_ms varint | fields strictly in bit order:
  [label str] [presence_pack bytes] [user_id str] [role str] [drag_ghost_json str]
  [interaction: encode_presence_interaction]
  [color u8] [surface str]
  [views: count varint × ( window_id str | space str | kind u8 {0 Canvas, 1 Orbit, 2 Geo}
                         | kind f64 fields in declared order | size 2×f64
                         | pointer present u8 (0/1) + 3×f64 when 1 )]
  [ui: opt_str hovered | opt_str focused | opt_str pressed]   // opt_str = present u8 + str
```

The decoder returns `ProtocolError::Malformed { what: "presence peer flags", .. }` when any bit ≥ 10
is set — drift guard, no silent forward compatibility.

Serde: camelCase everywhere; `views` `#[serde(default, skip_serializing_if = "Vec::is_empty")]`;
`PresenceViewKind` `#[serde(tag = "kind", rename_all = "camelCase")]` → `{"kind":"orbit",…}`;
`presence_pack` keeps its base64 `presence_pack_serde`.

`PresenceInteraction` / `PresenceDomain` are unchanged. `encode_presence_interaction` /
`decode_presence_interaction` become `pub` and gain exported TS twins.

### C7.2 Wire frames

- `ClientFrame::Presence { peer: Vec<u8> }` — unchanged (tag 4, preview lane).
- **New** `ServerFrame::Session { actor: String, color: u8 }` — **tag 9**, command lane, encoded
  `str | u8`. The hub sends it exactly once per connection, after `Welcome` (and its follow-up
  bootstrap frames) and before any `Presence` frame.
- `ServerFrame::Presence { peers }` — shape unchanged; now document-wide.
- Fixtures under the kernel's wire fixture dir: regenerate `📦️client-presence.bin` and
  `📦️server-presence.bin` (a peer carrying `color`, `surface`, 3 interaction domains, 2 views, `ui`),
  add `📦️server-session.bin`.

### C7.3 Hub (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`)

- `HubState.presence: DashMap<(scope_key, actor), PresenceSession>` with
  `PresenceSession { surface: String, user_id: Option<String>, color: u8, peer: Option<Vec<u8>> }`
  replaces the `(scope_key, surface, actor) → Vec<u8>` map. `surface_fanout`, `surface_fanout_for`
  and the `surface_rx` select arm are **deleted**; `ServerFrame::Presence` fans out on the
  document-wide `fanout`.
- `presence_peers(key)` returns the entries whose `peer.is_some()`.
- **Session colors**: `HubState.session_colors: DashMap<space_id, SpaceColors>`,
  `SpaceColors { by_actor: BTreeMap<String, ColorLease { index: u8, refs: u32 }> }`.
  - `acquire(space, actor) -> u8` — an existing lease for that actor: `refs += 1`, return its index;
    otherwise the **lowest index in `0..=255` not held by any live actor of the space** (beyond 256
    live actors, reuse `n % 256`).
  - `release(space, actor)` — `refs -= 1`, drop the lease at 0 (freed on the last disconnect of that
    shell session across all of its document sockets).
  - Acquired after successful `Hello`/auth and before `Welcome`; released at handler exit. Never
    persisted.
- Handshake: after `Welcome` + follow-ups the hub sends `ServerFrame::Session { actor, color }` and
  inserts `PresenceSession { peer: None, .. }`, so `ConnectionView.presence_known` = `peer.is_some()`.
- On `ClientFrame::Presence`: set `peer`, broadcast the roster, publish directory presence.
  On disconnect: remove the entry, broadcast the roster, release the color, publish directory presence.
- The hub **never decodes peer bytes**.

> **Amendment 3 to C1.** `DirectoryStreamMessage::Presence { spaceId, documentId, actors: Vec<DirectoryPresenceActor> }`
> with `DirectoryPresenceActor { actor, userId?, surface, color }` — replaces the previous
> `{ surface, actors: Vec<String> }` shape. Schema triad (`🦀️`/`🟦️`/`🔣️`) updated together.
> Published on every roster change of a document; the hub knows all four fields without decoding.

### C7.4 Client actor (kernel `🏪️store/🔄️sync/🦀️component.rs`, TS `🟦️backbone-worker.ts`)

- Actor state gains `session_color: Option<u8>` and `surface: Option<String>` (from
  `PersistenceBinding::Hub.surface`).
- On `ServerFrame::Session { actor, color }`: store `session_color`, emit
  `ArtifactEvent::Session { actor, color }` (Rust) / `{ kind: "session", actor, color }` (the TS
  `BackboneWorkerResponse` event union).
- On outbound `ArtifactActorMsg::PresenceHeartbeat { peer }`: stamp `peer.color = session_color` and
  `peer.surface = surface` **before** `presence_to_bytes`. Shells never fill these two.
- `PresenceHeartbeatProducer` / `PRESENCE_HEARTBEAT_INTERVAL_MS = 100` unchanged; shells additionally
  offer an immediate (still coalesced) beat when `views`/`ui` change.
- `assemble_presence_interaction` **moves** (pure, same signature, tests with it) from `🔄️sync` into
  `📡️wire`'s `🔖️PresenceInteraction` region — guests never enable the kernel's `sync` feature, and
  `VcsArtifactApp` must be able to call it.

### C7.5 Palette

Source of truth `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🔣️tokens.json` →
generated CSS / TS / Rust; pure twins in `👥️PresenceBar/{🧊️component.rs,🟦️component.tsx}` region
`🔖️Palette`.

```
hues  = [0, 210, 120, 30, 270, 180, 330, 60, 240, 150, 300, 90]     // 12 base hues
light = { s: 0.68, l: 0.42 }        dark = { s: 0.72, l: 0.62 }

presence_color(index: u8, appearance: Light | Dark) -> Hsl { h: u16, s: f64, l: f64 }
    base = index % 12;  k = index / 12
    h = hues[base]
    s = appearance.s - 0.25 * (k >= 2)
    l = appearance.l + 0.14 * (k odd)   // light: +, dark: -
presence_css_var(index) -> "var(--presence-{index % 12})"            // k == 0; k > 0 renders inline hsl
```

- CSS variables `--presence-0 … --presence-11` for `:root` (light) and `.dark`, plus `--color-presence-N`
  in the `@theme` block. Rust `presence::HUES: [u16; 12]`, `presence::LIGHT/DARK`. TS
  `STYLING_PRESENCE_PALETTES`.
- wgpu `Theme` gains `presence: [Rgba; 12]` and `local_presence: Option<u8>` plus
  `Theme::presence_color(index)`.
- `presence_hue_for_actor` / `presenceHueForActor` (FNV) are **deleted**.
- A peer with no `color` (folder-only session, no hub) renders index 0.
- Accessibility: ≥ 3:1 contrast against the base surface in both appearances, pairwise oklab ΔE ≥ 0.12;
  every color-coded overlay also carries a text/initials label (never color alone).

### C7.6 Plugin ABI

- `CHANNEL_VERSION` **12** (`🧫️fixtures/📡️channel/channel-version.json`, TS `APP_CHANNEL_VERSION`).
- `AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction: Vec<u8> }`
  — trailing length-prefixed bytes, the output of `encode_presence_interaction`; empty = none.
- **New** `AppCommand::Presence { seq: u64, own_color: Option<u8>, peers: Vec<Vec<u8>> }` — **tag 33**;
  `peers` are `encode_presence_peer` blobs (the whole roster; the wrapper drops its own actor).
  Reply `AppFrame::Done { in_reply_to: seq }`. This is the ONLY plugin ingress for peers.
- Object-safe app trait:
  ```
  fn ephemeral_snapshot(&self) -> EphemeralSnapshot;   // { presence, presence_generation, transient_generation, interaction }
  fn adopt_presence(&mut self, own_color: Option<u8>, peers: &[PresencePeer], now_ms: i64) -> Result<(), Fault>;
  ```
- `VcsArtifactApp` gains `own_color: Option<u8>` and
  `peer_presence: BTreeMap<String, PeerPresence { color: Option<u8>, surface: Option<String>, interaction: Option<PresenceInteraction> }>`.
  `adopt_presence` = for each peer ≠ own actor: `presence_store.adopt_peer(actor, A::Presence::decode_pack(pack)?, now)`
  when `presence_pack` is present, upsert `peer_presence`; actors absent from the roster are removed
  from both.
- `ephemeral_snapshot().interaction` = `encode_presence_interaction(&assemble_presence_interaction(app_id, &self.interaction_state(), &hover_specs, &selection_specs))`,
  the spec maps taken from the app's declared `AppDefinition.interactions` (`def.id → def.hover / def.selection`).
  Empty domains ⇒ empty bytes. **Zero app-side code.**
- `InteractionView` gains `peers: &BTreeMap<String, PeerPresence>` and
  `peers_selecting(domain, id) -> Vec<PeerMark>`, `peers_hovering(domain, id) -> Vec<PeerMark>`
  with `PeerMark { actor: &str, color: Option<u8> }`, sorted by actor.
- UI stamping: `UiPresence { state, status, hover, selected, color: Option<u8>, peers: Vec<UiPeerMark> }`
  with `UiPeerMark { actor: String, color: Option<u8>, hovered: bool, selected: bool, label: String }`.
  **`UiPresence` becomes `Clone` and is no longer `Copy`**; `UiNode::presence()`/`UiControlNode::presence()`
  return `&UiPresence`. The existing `ui_tree_stamp_presence` pass (ticket 26/08/14) stamps the own
  color and the peer marks for every `interaction_domain`-bound tree.
- Scene structs `TableScene` / `BlockListScene` / `DiffViewScene` / `EventFeedScene` gain
  `domain_id: Option<String>` (mirroring `World3dScene`) so tabular surfaces can look up marks.

### C7.7 Shell state (both shells)

- `local_views: BTreeMap<window_id, PresenceWindowView>` — surface hosts report
  `report_view(window_id, space, kind, size, pointer)`; entries cleared when the window closes.
- `local_ui: PresenceUi` — renderer-owned. React: one delegated `pointerover` / `focusin` /
  `pointerdown`+`pointerup` listener resolving `closest("[data-ui-path]")`. wgpu:
  `ui_presence_paths(&UiTree)` derived from the HOVERED / FOCUSED / ACTIVE node flags.
- `session_color: Option<u8>` from `ArtifactEvent::Session`.
- Heartbeat per open document every 100 ms:
  `PresencePeer { actor, label: displayName, presence_pack: snapshot.presence, connected_at_ms, user_id,
  role: None, drag_ghost_json: None, interaction: decode(snapshot.interaction), views: local_views.values(),
  ui: Some(local_ui) }` — `color`/`surface` are stamped by the actor.
- On `ArtifactEvent::Presence`: store the typed roster per document, push `AppCommand::Presence`
  (with `own_color`) to the plugin instance, feed `PresenceBar` rows with `color` + `surface`.

### C7.8 Peer-overlay derivation (pure, one implementation + one twin)

Rust `🧰️framework/🔨️modules/🖱️ui/👥️presence/🦀️component.rs`, handcrafted TS twin `🟦️component.ts`
beside it. Regions `🔖️PeerColor` / `🔖️Spec` / `🔖️Geometry` / `🔖️Paths`.

```
peers_for_window(roster, window_id, my_surface, my_actor, local_color) -> PeerOverlaySpec
PeerOverlaySpec {
  window_id, local_color: u8,
  artifact_peers: Vec<PeerView { actor, label, color, view: PresenceViewKind, pointer }>,   // views filtered to window_id's space
  app_peers:      Vec<PeerUi   { actor, label, color, hovered_path, focused_path, pressed_path }>,  // same-surface only
  marks: BTreeMap<domain, BTreeMap<id, Vec<UiPeerMark>>>,                                   // all peers
}
peer_marks_for(spec, domain, id) -> &[UiPeerMark]
ui_peer_marks_by_path(spec) -> BTreeMap<String, Vec<UiPeerMark>>      // hovered_path → hovered; focused/pressed → selected

canvas_peer_viewport_rect(peer_view, local_view, local_size_px) -> [f32; 4]
canvas_point_to_screen(local_view, local_size_px, world: [f64; 2]) -> [f32; 2]
orbit_frustum_corners(position, target, up, fov_deg, aspect, depth) -> [[f32; 3]; 5]   // apex + 4 far corners
orbit_frustum_segments(corners) -> [([f32; 3], [f32; 3]); 8]
geo_peer_viewport_polygon(peer_geo, viewport_px) -> [[f64; 2]; 4]                      // lnglat corners, web mercator

peer_overlay_path(scene_path, kind, index, key) -> String
    Camera   -> "{scene_path}/peerCamera[{i}]#{actor}"
    Cursor   -> "{scene_path}/peerCursor[{i}]#{actor}"
    Marks    -> "{scene_path}/peerMarks[{i}]#{domain}:{id}"
    Caret    -> "{scene_path}/peerCaret[{i}]#{actor}"
    Playhead -> "{scene_path}/peerPlayhead[{i}]#{actor}"
```

Both twins must produce byte-identical output over the pinned test fixtures.

### C7.9 DOM / dump id grammar — identical in BOTH renderers, **no `data-testid`**

Every overlay node carries `data-ui-path` (grammar above), `data-peer-actor`, `data-peer-color`
(the palette **index** as a decimal string) and exactly one kind attribute:

| Attribute | Meaning |
|---|---|
| `data-peer-camera` | a peer's camera/viewport indicator (World3d, TiledMap) |
| `data-peer-viewport` | a peer's viewport rectangle (Canvas2d, Board2d, NodeGraph, Paint2d, InkCanvas) |
| `data-peer-cursor` | a peer's pointer marker |
| `data-peer-marks` + `data-peer-mark="hover"\|"selection"\|"row"\|"caret"` | a peer's hover/selection mark on an item |
| `data-peer-caret` | a peer's text caret |
| `data-peer-playhead` | a peer's timeline playhead |

Local chrome: `[data-selection-ring][data-peer-color]`. Roster: `#s-presence-peers[data-self-color]`,
rows `[data-row-id="peer:<actor>"][data-peer-color][data-peer-surface]`, own row `[data-peer-self]`.
The wgpu renderer exposes the same set through `window.wasmBindings.dumpStructure()` — `ParityNode`
gains `attrs: Record<String, String>` and the peer nodes appear with `kind` = `peerCamera` etc.
Every camera / cursor / caret also renders a name tag; every mark renders an initials chip;
`aria-label`s in en + de.

---

## C8 Universal artifact creation

### C8.1 Manifest controls (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`)

- `ActionArgControl` gains:
  - `ArtifactKind { roles: Vec<AppRole> }` — the host resolves the option list from its live plugin
    catalogue (every app whose role ∈ `roles` and whose `io.document_schema` is non-empty).
  - `SurfaceApp { roles: Vec<AppRole>, dialect_arg: String }` — the host lists `(pluginId, appId, role)`
    for the dialect found in the dialog's seed argument named `dialect_arg`.
- New region `🔖️HostResolvedArgs`:
  `ArtifactKindChoice { kind_id, schema, dialect: ArtifactDialect, label: LocalizedLabel }`,
  `SurfaceAppChoice { app: AppRef, role: AppRole }`, `encode_*` / `decode_*` (the option `value` is
  JSON), and the pure
  `artifact_kind_choices(manifests: &[PluginManifest], roles: &[AppRole]) -> Vec<ArtifactKindChoice>`
  (deduped by dialect coordinate, sorted by coordinate).
  Option value shape (frozen):
  ```json
  {"kindId":"s.draw.draw","schema":"draw.document",
   "dialect":{"artifactKind":"s.draw.draw","standard":"1","subset":"*"},
   "label":{"en":"Draw","de":"Zeichnung"}}
  ```
- `ActionArgDef::artifact_kind(id, label, roles)` and `ActionArgDef::surface_app(id, label, roles, dialect_arg)`;
  `validate_arg_defs` and the empty-effective-value check treat both like `Select`.
- ts-rs mirror regenerated (`bun nx run @semio-tech/framework:generate`, then `:check`).
- `ArtifactKindSpec` is **not** touched (200 literals across 108 files).

### C8.2 Schema stamping

`PluginBuilder::editor::<E>(def)` / `viewer::<V>(def)` stamp `def.io.document_schema = E::DOCUMENT_SCHEMA`
when it is empty — the schema-first source of truth for "which document schema does this surface open".

### C8.3 Relay args (frozen, both shells)

`os.open-artifact` and `os.open-artifact-with` carry
`{ artifactRef, documentId, spaceId, schema, role?: "editor" | "viewer", pluginId?, appId? }`.

- `schema` comes from the space row (`SpaceArtifactRow.schema`).
- `role` default: author ⇒ `editor`, spectator ⇒ `viewer` (the directory read model's role), unless a
  pinned/preferred app says otherwise.
- App resolution: `pluginId && appId` when present, else `resolveOpeningApp(router, dialect, role, prefs)`
  (`os.config.opening`).
- Then `openDocument({ documentId, schema })` with bindings `[hub { baseUrl, spaceId, token, surface }, folder]`.
- React must stop requiring `pluginId`/`appId` and must stop parsing `role` as `=== 0`.
- wgpu must resolve the app and switch plugin (`switch_to_app(plugin_id, app_id, view_state)`),
  not reuse the current session's plugin.

### C8.4 Space plugin

- `KNOWN_ARTIFACT_KINDS` and `known_artifact_kind` are **deleted**.
- `CreateArtifact { name, kind /* encoded ArtifactKindChoice */, now_ms, actor }` — decode failure ⇒
  `Fault` code `s.space.unknown-kind`; the row records `{ kind_id, schema, dialect }`; the relay
  carries `schema`. The dialog arg id is `kind` (the e2e harness is updated accordingly).
- New `requestOpenArtifactWith { id }` command + `openArtifactWith` dialog with a
  `SurfaceApp` arg, wired as an "Open with…" row action.
- The space index renders a per-row peer roster from the directory presence stream; the ad-hoc
  `presence_peers_json` scene field is deleted.

### C8.5 wgpu staged dialogs

`ChromeDialogRequest` gains `args: Vec<ActionArgDef>`, `seed: Option<Value>`, `submit_action`,
`controller_id`; `HostEffect::OpenDialog { dialog_id, args }` is handled in both effect loops by
looking the dialog up in `session.app.dialogs`; the overlay renders `render_staged_arg` rows keyed
`window_id = "dialog:<id>"`; Submit builds `effective_action_args(defs, staged, seed)` and pushes the
`submit_action`; hit ids `shell.dialog.submit::<id>` / `shell.dialog.cancel::<id>`. A
`🔖️HostArgCatalog` region resolves `ArtifactKind` / `SurfaceApp` into `Select { options }` before
painting. `os.config.opening` is folded into `ShellState.opening_preferences` from the folder lane.

---

## C9 End-to-end steps

Steps 1–8 of the predecessor harness are unchanged. Appended (names are the log text):

```
 9 both peers carry distinct data-peer-color; each user's own [data-selection-ring] color equals its #s-presence-peers[data-self-color]
10 writer: user1 hovers an item; user2 shows [data-peer-mark="hover"][data-peer-actor=<user1>]
11 writer: user1 selects; user2 shows a selection mark plus [data-peer-caret]
12 draw: created and opened by both; user1 pans; user2 shows [data-peer-viewport] and [data-peer-cursor]
13 draw: user1 selects a stroke; user2 shows the peer selection mark
14 dag: created and opened by both; node selection propagates
15 cad (fallback lowpoly): user1 orbits; user2 shows [data-peer-camera] (World3d)
16 gis map: user1 pans the map; user2 shows [data-peer-camera] (TiledMap)
17 space index: user2's row for the artifact user1 is in shows a peer mark; row hover propagates
18 cross-surface: user1 in the editor and user2 in the viewer of the same artifact; user2 still sees user1's selection
19 creation smoke over every wasm-buildable non-stdio kind (create -> user1 editor mounts -> user2 row -> user2 viewer mounts)
20 wgpu-in-browser: steps 1-5 and 9-11 replayed with SEMIO_RENDERER=wgpu via the dump driver
21 wgpu native smoke: the native binary against the live hub reports identity, folded directory, open space and >=1 presence peer
22 every step above logged real failure text; no step skipped without naming its upstream cause
```

Harness scope: `COLLAB_E2E_REQUIRED_PLUGINS = ["s", "writer", "draw", "dag", "cad", "gis"]`.
Explicitly **out of scope**, to be reported as FAIL-with-reason rather than skipped:
`🗄️stdio/**` (the `FULL-STDIO` ticket is open and owns it) and the wasm-broken crates
`animate`, `layout`, `note`.
