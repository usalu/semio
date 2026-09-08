# Selection & Hover Mechanism — Definitive Map (puzzle3d render/write access)

## TL;DR

- **Selection/hover is already a first-class framework mechanism** (ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`, closed). Framework-owned `InteractionState` (persisted-local, history-lane-excluded) + ephemeral hover, mutated ONLY by six reserved verbs (`interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity`) intercepted by `VcsArtifactApp` before app dispatch, read via `InteractionView` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8176-8233`).
- **puzzle3d already declares and binds its `vortex` domain** (`✏️s/…/🧊️3d/…/✏️editor/🦀️.rs:7098-7144`), already reads it in `handle()` (`:6836-6846`, `Puzzle3dActionCtx.selection` at `:1943-1990`) and already implements `interaction_topology` (`:6872-6905`). So selection READS inside command handlers (dispatch, select-same-kind's precondition check, etc.) already work.
- **The render-time gap is real but the fix needs NO framework change.** The framework already ships an opt-in `render_with_request_context(..., interaction: &InteractionView<'_>)` trait method (`plugin/🦀️.rs:9946-9964` on `ArtifactApp`, `:23712-23730` on `ArtifactEditor`) that the runtime ALWAYS calls with a real, populated `InteractionView` (`plugin/🦀️.rs:22403-22442`, wrapper's `self.interaction_state()`/`self.interaction_hover`/`self.peer_presence`). Its default body silently discards `interaction` and falls back to plain `render`. **puzzle3d implements only plain `render` (`editor/🦀️.rs:6955`), never overriding `render_with_request_context`, so the interaction data the wrapper already built is thrown away.** This is puzzle3d's own gap, not a framework limitation — `process3d` and `generation3d` already prove the read path works (see below).
- **The host draws selection/gumball/hover from the PLUGIN's own JSON, not from its own independent state.** `World3dHost.tsx:2325` (`selectedIds = selection.ids ?? []`) and `:2346` (`gumballVisible = Boolean(selection.gumballActive) && …`) read straight out of `parseSelection(scene.selectionJson)` — the exact JSON puzzle3d's `world_selection_json()` builds (`…/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:425-459`), which currently hardcodes empty `vortexIds`/`targetVolumeIds` and `gumballActive: false` (`gumball_active` at `:114`). So the host is not silently overriding puzzle3d's `false` — it is faithfully rendering what puzzle3d sends it. Fixing `render_with_request_context` fixes the host paint immediately, no host change needed.
- **`context_menu` genuinely has no interaction-aware variant anywhere in the framework** (`ArtifactApp`/`ArtifactEditor`/`ArtifactViewer` — grepped, zero `context_menu_with_request_context`-style method exists). This is a real, framework-wide gap; the clean fix mirrors the `render_with_request_context` pattern exactly (additive, default-falls-back, zero forced blast radius).
- **App-initiated selection WRITES (select-same-kind, duplicate re-select, accept re-select) are architecturally impossible today** — `dispatch_interaction_action`, the only writer of `InteractionState`, is private to `semio-framework-plugin` and only reachable via the six reserved verbs (`plugin/🦀️.rs:8880-8886`, `:18637`). puzzle3d's own `select-same-kind` command already documents this and aborts (`🎮️commands/🧬️select-same-kind/🦀️.rs`). This needs a small, additive framework change (new opt-in `Emit` field), not a client-side workaround.
- **Presence is already fully wired and app-agnostic** (`PresencePeer.interaction`, wire bit 7) — since puzzle3d's `vortex` domain is declared and bound, presence should already work for puzzle3d/5d/2d without further changes; only needs a runtime smoke check (the 26/08/14 ticket's own closing summary flags "runtime smoke testing was not performed").
- **Blast radius of the recommended fix is small and entirely opt-in**: 0 forced changes to other plugins for both the render fix and the context-menu fix (same additive-default-method pattern already used for `render_with_request_context`); ~1 framework file touched for the write-channel addition, plus puzzle3d's own command handlers.

---

## 1. Ticket `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM` — design & outcome

Source: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM/📋️master.md` and `🎫️ticket.json`/`📓️closing-summary.md` (status: **closed**).

### Decided design
- **Framework owns selection**, not the app. One `InteractionDefinition` per domain (hover + selection sub-specs sharing one target universe), declared in the manifest as a sibling of actions/utilities/tools/commands: `AppDefinition.interactions: Vec<InteractionDefinition>`, `WindowKindDefinition.interactions: Vec<InteractionRef>`.
- **`vortex`-style interaction domains**: each domain declares `granularities` (non-empty, first = default), a `HierarchyProvider` (`Flat | Topology | UiTree | PathDelimited`), `HoverSpec{enabled,transitive,channels,broadcast}`, `SelectionSpec{modes,methods,merges,transitive,broadcast}`.
- **`interactionSelect{domainId,targets,merge,method}` / `interactionHover{domainId,channel,targets}`** are framework-injected actions (`ActionKind::Interaction`), the ONLY way selection/hover ever change. Marquee (rectangle/lasso) is a **method** argument on `interactionSelect`, not separate state — surfaces do geometric hit-testing client-side and emit one batched call; merge mode (`Replace|Additive|Subtractive|Invertive|Range`) travels as the `merge` argument.
- **Persistence**: selection/mode/granularity are persisted-local via a new store `HistoryLane::{Document,Interaction}` mechanism — default undo/redo skips the `Interaction` lane. Hover is never persisted.
- **Presence**: typed `PresenceInteraction` on `PresencePeer` (wire bit 7), assembled generically by the framework from `InteractionState` on the existing heartbeat — zero per-app code.
- **Runtime interception**: `VcsArtifactApp` intercepts the six verbs before app dispatch; apps read state through `InteractionView` on `handle`/`copy_fragment`/`cut_operations`.

### What was left unfinished (from the ticket's own closing summary)
- `ArtifactApp::render`/`context_menu` were explicitly called out as **not** gaining an `interaction: &InteractionView` parameter in that wave (this is the origin of the "framework gap" language duplicated across puzzle3d's own doc comments).
- **"block-3D's world window and puzzle-3D/5D were left with unbound world domains on purpose — they already emit their own interaction verbs from bespoke logic, and binding risked double-emission. Worth a follow-up."** — **This is now stale**: puzzle3d/5d/2d and block3d/5d/2d all now DO declare and bind their domains (`.interaction(...)`, `.window_kind_interactions(...)` — see §2 below), evidently done in later, uninventoried work. What is still true from that note is that the World3d **scene's `domain_id` field is explicitly set to `None`** by puzzle3d/block3d (see §2), keeping the host's generic click-dispatch path off and routing clicks through legacy `worldPick`/`worldSelect`/`setHover` verbs instead — verified NOT handled anywhere in puzzle3d's own Rust (`grep "worldPick"` outside tests/comments returns nothing in the puzzle crate; the only real handler is the unrelated OS `♾️infinite` world module, `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9801`). **This strongly suggests basic click-to-select in the puzzle3d 3D viewport may not currently dispatch anything real** — flagged as a concrete runtime-verification item for the implementation wave, not confirmed by a live run (read-only audit).
- `dispatch_emit_group` (composite/child dispatch) doesn't revalidate interaction state post-dispatch (no inventoried app affected).
- Runtime smoke testing (two-peer presence, transitive hover, undo-skip, marquee) was never performed.

---

## 2. Framework types & the actual render-time flow

### `InteractionView` (read-only accessor)
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8176-8233`:
```
pub struct InteractionView<'a> { state: &'a protocol::InteractionState, hover: &'a InteractionHoverState, peers: &'a PeerPresenceRoot }
impl<'a> InteractionView<'a> {
    pub fn selection(&self, domain: &str) -> &protocol::DomainSelection   // :8188
    pub fn hover(&self, domain: &str, channel: &str) -> &protocol::DomainHover // :8196
    pub fn active_granularity/active_mode/peers_selecting/peers_hovering(…)
}
```
Never `None`-typed — undeclared/never-touched domains resolve to empty structs, so callers never unwrap.

### `handle()` already gets it; `render`/`context_menu` do not, by trait shape
- `ArtifactApp::handle(..., interaction: &InteractionView<'_>, ...)` — `plugin/🦀️.rs:9837`. Also `copy_fragment`/`cut_operations` (`:9910/9913`).
- `ArtifactApp::render(body_key, doc, cfg, view_state: &ViewModel)` — `:9935` — **no interaction param**, required (no default body).
- **`ArtifactApp::render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction: &InteractionView<'_>)`** — `:9952-9964` — has a default body (`let _ = (transient, interaction); Self::render_with_instance_operation_owner(...)`) that falls through to plain `render`. Doc comment (`:9946-9951`) explicitly references this: *"`interaction` reads the framework-owned interaction mechanism (hover + selection) so a window can paint its own hovered/selected state instead of ever storing it again itself — see `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`."*
- Identical shape on `ArtifactEditor` (`:23651` handle, `:23712` render, `:23723-23730` render_with_request_context) — this is the trait puzzle3d actually implements.
- `context_menu` (`:9990`, `:23744`) has **no** request-context counterpart anywhere — confirmed by `grep -n "context_menu_with_request_context"` returning nothing repo-wide.

### The wrapper ALWAYS calls `render_with_request_context`, with a real `InteractionView`
`plugin/🦀️.rs:22403-22442` (`VcsArtifactApp::render`):
```rust
let interaction_state = self.interaction_state().await;               // :22403
let interaction_hover = self.interaction_hover.clone();                // :22405
let interaction_peers = std::sync::Arc::clone(&self.peer_presence);    // :22406
let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref() }; // :22407
...
A::render_with_request_context(&self.instance_operation_owner, body_key, &doc, &cfg, view_state, &transient, &interaction).await // :22421 and :22442
```
So **every** app's real render call already gets a fully-populated `InteractionView` — an app only needs to override `render_with_request_context` (instead of only `render`) to receive it. **This is the entire fix for puzzle3d's render-time gap. Zero framework change required.**

### `ViewModel` carries none of this (confirms the audit)
`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4372-4413`: `ViewModel` fields are `active_mode_id, active_window_kind_id, active_utility_id, active_utility_by_window_id, active_tool_id, panel_json, contributions_json, locale, terminology, window_id, window_instances`. No selection/hover/interaction field, confirming `render(..., view_state: &ViewModel)` alone truly cannot see live selection.

### Host draws selection/gumball entirely from the plugin's own JSON
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`:
- `:1057` `parseSelection(selectionJson)` — parses `scene.selectionJson` (the exact string `world_selection_json()` builds).
- `:2325` `const selectedIds = selection.ids ?? []` — instance selection is 100% JSON-sourced.
- `:2346` `const gumballVisible = Boolean(selection.gumballActive) && transformGumballMode && !paintMode` — gumball visibility is 100% JSON-sourced (`selection.gumballActive` ← puzzle3d's hardcoded `false`).
- `domainId` (`scene.domainId`, `:3869-3871`) is used ONLY on the OUTGOING path — to build `interactionSelect`/`interactionHover` action args when the user clicks (`:4387`, `:4407`, `world3dSelectionActionArgs`/`world3dHoverActionArgs`, `:3837-3847`). It is never read to derive the DISPLAYED selection; there is no host-side fallback to `InteractionState`/presence for painting a plugin's own world scene.
- When `interactionDomainId` is falsy (puzzle3d's case — see below), `handleInstancePointerDown` (`:4376-4396`) dispatches legacy `worldPick`/`worldSelect` instead of `interactionSelect`.

**Conclusion for the audit's question 2**: the host does NOT independently draw selection/gumball; it trusts the plugin's JSON completely. Puzzle3d's hardcoded `false`/`[]` is not overridden or made irrelevant by the host — it is the direct, sole cause of the missing gumball/highlight in the running app.

### puzzle3d's domain IS already declared and bound (contradicts the stale 26/08/14 closing-summary note)
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:
- `:7098-7118` `puzzle3d_interaction_definition()` — domain `PUZZLE3D_INTERACTION_DOMAIN` ("vortex"), granularities `object/vortex/attraction/targetVolume/reference/kind`, `HierarchyProvider::Topology`, hover `{enabled:true, transitive:false, channels:["pointer"], broadcast:true}`, selection `{modes:[Multiple,Single], methods:[Pick,Rectangle], merges:[Replace,Additive,Subtractive,Invertive], transitive:false, broadcast:true}`.
- `:7143-7144` `.interaction(puzzle3d_interaction_definition())` + `.window_kind_interactions(main::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE3D_INTERACTION_DOMAIN)])`.
- `:6872-6905` `interaction_topology()` — builds the object→vortex-marker nesting.
- `:6836-6846` `handle()` already reads `interaction.selection(PUZZLE3D_INTERACTION_DOMAIN)` and threads it into `Puzzle3dActionCtx.selection` (`:1943-1990`, with `selected_object_ids/selected_vortex_ids/selected_attraction_ids/selected_target_volume_ids/selected_reference_ids` helpers) — so **command-handler-time reads already work**; only render-time reads are missing.
- Same pattern confirmed for puzzle5d (`🖐️5d/…/✏️editor/🦀️.rs:8736-8929`, domains bound to both `board2d` and `world3d` window kinds) and puzzle2d (`◻️2d/…/✏️editor/🦀️.rs:3843-3846`, three window kinds bound) and block3d/5d/2d (`🧱️block/…`, same `.interaction(...)`/`.window_kind_interactions(...)` shape).
- BUT the World3d scene explicitly opts out of the host's generic pick path: `…/🪟️windows/🧊️main/🦀️.rs:484` `scene.domain_id = None;` (puzzle3d) and `…/block/.../🌐️world/🦀️.rs:67` `scene.domain_id = None;` (block3d) — both with a comment claiming the app "already emits its own `interactionSelect`/`interactionHover` … from bespoke vortex-fit pick logic elsewhere in this crate" (`🧊️main/🦀️.rs:476-481`). **No such emission was found anywhere in the puzzle3d crate** (only doc-comment references, no actual dispatch/handler for `worldPick`/`worldSelect`/`worldVortexHover`/`setHover` in real code) — this claim should be verified at runtime before the implementation wave assumes clicking already works via some other path.

---

## 3. Reference-plugin comparison — what actually solves render-time selection today

Adoption census: only **3 of ~34 distinct plugin `render(body_key...)` implementers** override `render_with_request_context` at all (grep excludes the `🗄️stdio` mirror tree, which is 176 of the 288 raw hits and not a distinct hand-written implementation):
```
✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/…/✏️editor/🦀️.rs
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/…/✏️editor/🦀️.rs
```

**`process3d` — the pattern to copy** (`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1616-1627`):
```rust
fn render_with_request_context(_owner, body_key, doc, cfg, view_state, _transient, interaction: &InteractionView<'_>) -> UiAssemblyResult<ComponentTree> {
    let selected_ids = interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids.clone();
    process3d_render_body(body_key, doc.snapshot, cfg.snapshot, view_state, &selected_ids)
}
```
`process3d_render_body` (`:1297-1309`) routes `selected_ids` into `inspection::render(doc, selected_ids, labels)` for the inspection-panel body key — but **not** into `workpiece::render(doc, config, active_utility)` (`…/🪚️workpiece/🦀️.rs:154`), the actual 3D viewport body. So process3d proves the read mechanism works end-to-end for a panel, but has NOT itself solved the "paint gumball/highlight in the World3d scene" half of this problem — puzzle3d would be the first to close that half.

**`generation3d`** (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:1006-1016`) similarly reads `interaction` into a `PreviewInteractionMarks::from_interaction(interaction)` value fed to its render body — a real, working read-to-render-prop conversion, worth reading as a second precedent for the "thread interaction into a scene-prop builder" shape.

**`lowpoly`** (`…/💠️lowpoly/…/✏️editor/🦀️.rs:1756-1767`) — override exists but **discards** `interaction` (`_interaction: &InteractionView<'_>`, unused) and instead renders selection from `LowpolyScratch::from_transient(transient.snapshot, crate::LowpolySelection::default())` — i.e. lowpoly currently renders with an explicitly EMPTY selection at the point it overrides this method, contradicting its own status as the "acceptance bar" in the 26/08/14 design doc. Its `handle()` DOES correctly consume `interaction` via `selection_from_interaction(&active, interaction)` (`:1747`), so lowpoly's write/action-time path is correct; its render-time selection paint appears to be a separate, still-open gap in lowpoly itself — flagged for cross-reference, out of scope for puzzle3d's own fix but worth a follow-up ticket note.

**`cad`**: implements `context_menu` (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/…/✏️editor/🦀️.rs`) but not `render_with_request_context` — no additional pattern beyond what's above.

**Net finding**: there is no plugin today that fully solves "paint live selection/gumball in a World3d viewport from `InteractionView`." puzzle3d would be the pattern's first real-world completion, using `process3d`'s read shape (`interaction.selection(DOMAIN).ids`) but threading the result all the way into the world-scene body (`main::render`/`world_selection_json`/`gumball_active`), not just an inspection panel.

---

## 4. Peer ticket `26/09/08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS` — no collision

`📋️plan.md:34`: *"The existing `semio_framework::ViewModel` is the canonical render-time context. Its immutable reference now passes through the shared renderer/editor/viewer/context-menu seams. No duplicate `RenderContext` type or compatibility method is needed."*

This ticket's scope is **locale/terminology/active-utility/camera/LOD ownership** — moving config fields plugins shouldn't own onto the OS-shared `ViewModel`/host context. It does **not** touch entity selection/hover (`InteractionState`/`InteractionView`) at all — "selection" in its `📋️plan.md:38` ("Camera, LOD, and selection cannot be classified by their names alone…") refers to per-window UI OPTIONS (e.g. a window's selection **method**/mode toggle, analogous to camera/LOD), not the picked-entity ids from the 26/08/14 mechanism. `ViewModel` staying the sole render-time context type is consistent with the existing repo pattern where `InteractionView` is a **separate, additional** parameter alongside `ViewModel` (already true for `handle`/`render_with_request_context`) — there is no indication this peer ticket intends to fold `InteractionState` into `ViewModel`, and doing so would contradict its own "no duplicate context type… needed" framing only if someone tried to invent a second context type, not if `InteractionView` (already established) continues riding alongside it. **No architectural collision; puzzle3d's fix does not need to wait on or coordinate with this ticket.**

---

## 5. Recommended architecture

### (a) Render-time READ — puzzle3d, puzzle5d, puzzle2d, block3d/5d/2d
No framework change. Override `render_with_request_context` (mirroring `process3d`'s shape) instead of relying on plain `render`:
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:6955` — replace/augment `fn render(...)` with `fn render_with_request_context(..., interaction: &InteractionView<'_>)`, extract `interaction.selection(PUZZLE3D_INTERACTION_DOMAIN)` and `interaction.hover(PUZZLE3D_INTERACTION_DOMAIN, "pointer")`, thread into:
  - `…/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs:114` `gumball_active(runtime, active_utility)` → take a real selection slice instead of `_runtime`.
  - `:425-459` `world_selection_json(envelope)` → populate `vortexIds`/`targetVolumeIds` from selection-by-granularity (mirror `Puzzle3dActionCtx::selected_ids` grouping logic at editor `🦀️.rs:1968-1990`), and hover-driven `hoveredId`.
  - `:674` `puzzle3d_brush_target_vortex` → accept an interaction/selection+hover slice and restore the "explicit selection, else hovered vortex, else first vortex of hovered object" fallback the doc comment describes.
  - `📌️panels/🔍️inspection/🦀️.rs` → mirror `process3d`'s `inspection::render(doc, selected_ids, labels)` shape exactly.
- Same treatment for puzzle5d (`🖐️5d/…/✏️editor/🦀️.rs:8736-8929`) and puzzle2d, and for block3d/5d/2d if they have the analogous gap (not audited line-by-line here, but same `.interaction(...)` + `scene.domain_id = None` shape was confirmed present).
- **Blast radius: 0 forced changes elsewhere** — `render_with_request_context` already exists with a safe default; only the crates that opt in are touched.

**context_menu** — real framework gap, needs an additive method (same non-breaking shape as `render_with_request_context`):
- Add `context_menu_with_request_context(_request, doc, cfg, view_state, interaction: &InteractionView<'_>, registry) -> Vec<ContextMenuItemSpec>` to `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer` (`plugin/🦀️.rs`, alongside the existing `context_menu` at `:9990`/`:23744`/`:24004`), default body falls back to plain `context_menu` (same pattern as `:9952-9964`). Update the ONE wrapper call site that currently invokes `context_menu` to call this instead (pass the same `InteractionView` the wrapper already builds for render, `:22407`).
- **Blast radius: 1 framework file, 0 forced plugin changes** (15 non-test `context_menu` implementers keep working unmodified); puzzle3d opts in to fix "duplicate/accept can't re-select"-adjacent menu items and any selection-count-aware menu text.

### (b) App-initiated selection WRITES — select-same-kind, duplicate re-select, accept re-select
Real, additive framework change needed — today `dispatch_interaction_action` (`plugin/🦀️.rs:8880-8886` doc, body at `:18637`) is the sole writer and is private, reachable only via the six reserved client verbs.
- Add an opt-in field to `Emit` (`plugin/🦀️.rs:8852-8875`, mirroring the existing `child_emits: Vec<ChildEmit>` pattern), e.g. `interaction_writes: Vec<InteractionWrite>` where `InteractionWrite{ domain: String, targets: Vec<InteractionTarget>, merge: MergeMode }` (reuse `protocol::SelectionInput`'s shape).
- `VcsArtifactApp::dispatch_emit` applies these through the SAME single-writer path `dispatch_interaction_action` already uses (`next_selection` + `revalidate_and_persist_interaction_state`, `:8858-8875`), immediately after the triggering action's own document mutations land, recorded under the same `ActionKind::Interaction` lane. This preserves the "six reserved verbs are the only EXTERNAL trigger" invariant — `interaction_writes` is an internal side-channel of an app's own action, not a new client-facing verb, and still goes through the one pure state machine.
- puzzle3d changes: `🎮️commands/🧬️select-same-kind/🦀️.rs` populates `Puzzle3dActionCtx`'s emit with an `interaction_writes` entry (`domain: PUZZLE3D_INTERACTION_DOMAIN, granularity: object, ids: <same-kind ids>, merge: Replace`) instead of aborting; likewise for duplicate/accept re-select command handlers (not individually audited by file:line here — same shape applies wherever a command currently can't express "select the thing I just created").
- **Blast radius: 1 framework file** (`Emit` struct + `dispatch_emit`), **0 forced plugin changes** (new field defaults to empty `Vec`); puzzle3d's own select-same-kind/duplicate/accept handlers opt in.

### (c) Presence for multi-user
Already framework-generic and already wired for puzzle3d's bound `vortex` domain (`PresencePeer.interaction`, wire bit 7, assembled by the framework heartbeat — no app code). No further design work needed; treat as a **runtime verification** item (two-tab smoke test), not an implementation item — this matches the 26/08/14 ticket's own "not done" list.

### Summary of files touched
- **Framework** (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`): add `context_menu_with_request_context` (additive, ~3 trait sites + 1 wrapper call site); add `Emit.interaction_writes` + apply it in `dispatch_emit` (~2-3 sites). No signature changes to any existing required method — zero forced edits to the 30+ other plugin crates.
- **puzzle3d** (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…`): `✏️editor/🦀️.rs` (add `render_with_request_context` override, wire `Puzzle3dActionCtx`/select-same-kind to `interaction_writes`), `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` (`gumball_active`, `world_selection_json`, `puzzle3d_brush_target_vortex`, `world_brush_preview_json`), `✏️editor/📌️panels/🔍️inspection/🦀️.rs`.
- **puzzle5d/puzzle2d/block3d/block5d/block2d**: same shape, not individually line-audited here — flagged for the same treatment once puzzle3d's pattern is proven.
- **Host (`World3dHost.tsx`)**: no change required for (a)/(c); no change required for (b) either (writes already flow as ordinary `Emit` mutations the host doesn't need to know about).

## Open items for the implementation wave (not resolved by this read-only audit)
1. Verify at runtime whether `worldPick`/`worldSelect`/`setHover` (the verbs the host currently dispatches for puzzle3d's world viewport, since `scene.domain_id = None`) are handled at all, or whether basic click-to-select is currently a no-op in the running app.
2. Decide whether to bind `scene.domain_id` to `PUZZLE3D_INTERACTION_DOMAIN` (enabling the host's generic `interactionSelect`/`interactionHover` dispatch path) as part of this wave, or keep the bespoke pick path — the "double-emission" risk the original comment cites was never substantiated by any actual bespoke emission code found in this audit.
3. lowpoly's `render_with_request_context` override currently discards `interaction` and renders an explicitly empty selection — likely a separate, pre-existing lowpoly bug worth its own ticket, not blocking puzzle3d.
