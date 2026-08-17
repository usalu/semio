# Hover & Selection as First-Class Mechanisms

## Context

Hover and selection are today hand-rolled per app (~14 apps: flow, lowpoly, writer, dag, procedural, cad, process, remodel, raster, trinity, playbook, sequence, layout, plus the OS `♾️infinite` module): each has its own config fields (`selected_node_ids` vs `selection_ids` vs `selected_object_ids`, String vs u32 ids), its own `set-selection`/`set-hover` command handlers, merge modes only in lowpoly, transitive behavior only in writer (AST covering node), and presence broadcast in only 4 apps. This duplication and drift is exactly what the framework's mechanism pattern (actions/utilities/tools/commands in `🛂️manifest`) exists to prevent.

Goal: dissolve hover/selection out of per-app commands into one first-class framework mechanism with declarative transitive hover, selection modes/granularities/methods, clean app-config (manifest) declaration, uniform presence broadcast for every app, and event-sourcing-doctrine-compliant state classification. Greenfield: no back-compat, no adapters; all apps and fixtures fixed by hand in one sweep.

**User-confirmed decisions:**
1. **Selection is persisted-local** (config axis) — survives reload, never shared-persisted — but **ignored by default undo/redo**; a proper history-lane mechanism is added to the store (no hacks). Hover stays ephemeral (local + shared pointer channel).
2. **Unified `InteractionDefinition`** per domain (hover + selection sub-specs sharing one target universe/hierarchy), not split definitions.
3. **Short workspace-red window accepted** between the breaking SDK wave and the parallel per-app migration wave (briefs pre-staged, one focused session).

Execution model (user-specified): 1 Opus 5 coordinator agent, N Sonnet 5 executors, N Haiku 4.5 read-only verifiers, orchestrated via Workflow scripts from the main session. **No worktrees, no git commands** (shared live tree, concurrent human devs, auto-commit).

**Ticket (already opened):** `26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM`, goal `r2602/runningsketchpad`, [issue #2555](https://github.com/usalu/semio/issues/2555). The design doc is mirrored to the ticket folder as `📋️master.md` so every agent reads one source of truth. No code has been written yet.

## Design

### New manifest types (in `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, mirrored to TS via typegen)

```rust
pub struct InteractionDefinition {           // one per interaction domain: "graph", "mesh", "ast", "world"
    pub id: String,
    pub label: LocalizedLabel,
    pub granularities: Vec<GranularityDefinition>,  // non-empty; first = default
    pub hierarchy: HierarchyProvider,               // Flat | Topology | UiTree | PathDelimited{delimiter}
    pub hover: HoverSpec,                           // { enabled, transitive, channels (default ["pointer"]), broadcast }
    pub selection: SelectionSpec,                   // { modes: [Single|Multiple], methods: [Pick|Rectangle|Lasso], merges: [Replace|Additive|Subtractive|Invertive|Range], transitive, broadcast }
}
pub struct InteractionRef(String);           // mirrors ActionRef/UtilityRef
```

- `AppDefinition` gains `interactions: Vec<InteractionDefinition>` (sibling of actions/utilities/tools/commands); `WindowKindDefinition` gains `interactions: Vec<InteractionRef>`. No mode-level scoping (modes scope window kinds already).
- `ActionKind` gains `Interaction` (framework-injected, never app-declared, mirroring History/Clipboard). Injected actions for any app with interactions: `interactionSelect{domainId,targets,merge,method}`, `interactionHover{domainId,channel,targets}`, `clearSelection` (esc), `selectAll` (mod+a), `setSelectionMode`, `setInteractionGranularity`. All per-app selection/hover View actions and their icon-catalog entries (manifest ~236–248) are deleted; app verbs *over* the selection (`delete-selection`, `focus-selection`) survive as ordinary actions reading framework state.
- Marquee (rectangle/lasso) is a **method**: surfaces do geometric hit-testing and emit one batched `interactionSelect`; no geometry in the state machine.

### New framework module `🧰️framework/🔨️modules/🕹️interaction/`

`🦀️component.rs` + handcrafted `🟦️component.ts` parity + `🧬️schema/{🔣️component.json,🦀️component.rs,🟦️component.ts,🔗️component.graphql}`, wired via `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` like `🛂️manifest`. Pure state machine:

- `InteractionTarget{granularity,id:String}` (u32 ids stringified at the boundary), `DomainSelection{granularity,ids,anchor_id}`, `DomainHover{channel,ids}` (transitive closure, root first), `InteractionState{selection,hover,active_mode,active_granularity}` (BTreeMaps keyed by domain).
- `TopologyNode{id,granularity,parent}` / `DomainTopology{ordered}` (pre-order = range order) / `InteractionTopology`.
- `next_selection(spec,state,topo,input)` — generalizes Tree's `getTreeNextSelectionState` (Tree/🟦️component.tsx:946–968): single clamp, Range via anchor+order, Additive/Subtractive/Invertive set algebra, batch merge, transitive closure, granularity filter. `next_hover(...)`, `validate_state(...)` (prunes stale ids). TS mirror replaces Tree's private machine.

### Store: persisted-local selection with history lanes (user decision 1)

In `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`:
- Add a **history-lane mechanism** to the config-side store: mutations carry `HistoryLane::{Document, Interaction}`; the default undo/redo cursor **skips `Interaction`-lane entries** (both directions), while they remain persisted, replayable, and inspectable. This is a first-class store feature with its own tests, usable by future view-ish-but-persisted state.
- Framework-owned persisted-local `InteractionState` per app instance, written only by the framework machine via `Interaction`-lane mutations (single writer → no drift with presence). Hover is never persisted (ephemeral store field beside `PresenceStore`, :757).
- After every artifact dispatch, `validate_state` prunes ids deleted from the document.

### Runtime interception (no per-app handlers)

`VcsArtifactApp` (`🔌️plugin/🦀️component.rs`) already routes framework-reserved verbs through `dispatch_command_frame`/`dispatch_action` (~:7105) before `A::handle` — verified. Add the interaction verbs there:
- Topology via new trait method `ArtifactApp::interaction_topology(doc,cfg) -> InteractionTopology` (default empty; wrapper self-derives for Flat/UiTree/PathDelimited).
- `ArtifactApp::handle` (and `copy_fragment`/`cut_operations`) gain an `interaction: &InteractionView` parameter — read-only accessor (`selection(domain)`, `hover(domain,channel)`). Breaking change applied to SDK + all apps in the sweep.
- Session command log rows use kind `Interaction`; every accepted semantic interaction allocates one distinct live-history row and is delivered in the invocation's `HistoryPatch`. Raw pointer samples remain telemetry and never create rows. `UiDirtyScope` is emitted without widening it for history; presence is marked dirty.
- `AppBuilder` gains `.interaction(def)` / `.window_kind_interactions(id, refs)` with build-time validation (unique domains, non-empty granularities, `transitive ⇒ hierarchy != Flat`, method/merge refs declared).

### UI

- `UiPresence{state,status,hover,selected}` unchanged as the universal paint contract; the wrapper stamps it from `InteractionState` for `UiTree`-bound domains (existing `ui_tree_stamp_presence` path, plugin ~:2108).
- `UiTreeNode`: delete `selectedIds`/`highlightedIds`/`selectionChange`, add `interactionDomain?: string`; `UiTreeItemNode`: delete `hoverAction`/`unhoverAction`. Renderers translate clicks/hover + modifiers into injected interaction actions; modifier→merge policy lives in one framework place (shift=Range, mod=Invertive, alt=Subtractive).
- `ViewModel.selection_json` deleted; `TutorialUiChange::Selection` re-pointed to replay interaction verbs. Tree (`🖱️ui/🧱️elements/🪵️Tree/🟦️component.tsx`) deletes `normalizeTreeSelectedIds`/`getTreeNextSelectionState` and imports the interaction TS.
- Surfaces (`🗺️surface/{🕸️node-graph,🎨️paint,🗺️tiled-map}`, 3d world hosts): delete push-setters (`set_hovered_id`, `set_selection_ids_json`); scene payloads fed from `InteractionView` in render; marquee gathers → batched select.

### Presence (uniform, all apps)

`PresencePeer` (`📡️spr/📡️wire/🦀️component.rs:713`, bitmask codec verified, **bit 7 free**) gains a typed field:

```rust
pub struct PresenceInteraction { pub app_id: String, pub domains: Vec<PresenceDomain> }
pub struct PresenceDomain { pub domain: String, pub granularity: String, pub selected: Vec<String>, pub hovered: Vec<String> }
// PresencePeer += interaction: Option<PresenceInteraction>  (bit 7)
```

Typed (not inside app-opaque `presence_pack`) so the Shell renders every peer's selection/hover generically — zero app code. Assembled by the wrapper from `InteractionState` on the existing heartbeat (`ClientFrame::Presence`, sync :930/:1476); hover shares the cursor throttle; only explicit ids broadcast (receivers expand closures via own topology). `presence_pack` keeps genuinely app-specific presence (e.g. FlowPresence keeps camera + preview_off_node_ids, loses selection fields). Schema leaves registered under owner `framework.interaction` with `x-semio-state` annotations.

### State-axis classification (final)

| State | Axis |
|---|---|
| Interaction declarations | static manifest |
| Own selection + active mode/granularity | **persisted-local** (Interaction history lane, default-undo-skipped) |
| Own hover | ephemeral-local; pointer channel mirrored ephemeral-shared |
| Own selection broadcast mirror | ephemeral-shared (framework-assembled, single writer) |
| Peer selection/hover | ephemeral-shared (PresencePeer roster) |

## Per-app migration pattern (17 crates / ~24 apps — verified inventory)

Scope is larger than first estimated. Full inventory (fields, mutation variants, command dirs, presence facets, readers, implied domains) is in the ticket folder; the W4 work-list is one task per crate:

`writer` · `procedural` (3d, 2d) · `gis` (3d, 2d) · `shooting` · `process` (3d) · `lowpoly` · `layout` · `cad` · `remodel` · `trinity` (jack, rewrite) · `draw` · `raster` · `note` · `puzzle` (3d, 5d, 2d — ~20 selection/hover commands, the heaviest) · `block` (3d, 5d, 2d) · `space` · `sourcing` · plus OS `♾️infinite`.

Crate names follow `semio-s-plugin-<plugin>`. Hierarchy sources for transitive hover exist in: writer (AST parents), procedural (DAG), puzzle/block (vortex→part→kind), trinity rewrite (AST + var refs), space (media node graph); the rest are Flat. Every one of the 17 already has a `👥️presence` facet mirroring config selection/hover — all of which collapse into the framework's typed presence field.

Common pattern:

Remove selection/hover fields + mutations from `🎚️config/🦀️component.rs` + schema leaves; delete `🎮️commands/{🗂️set-selection,🗂️set-hover,🗂️select-*,👁️*-hover,🗂️clear-selection,🗂️select-all}` dirs + `declare_command_enum` rows; shrink `👥️presence` facet to app-specific fields; add `.interaction(...)` + `interaction_topology` impl; thread `InteractionView` through `handle`/clipboard; fix example/fixture DSL containing selection fields. Hardest three (reference traces validated by design agent):
- **flow**: domain `graph` (node/edge/handle, Topology hierarchy from group nodes, modes [Multiple,Single], methods [Pick,Rectangle], all merges, transitive hover). FlowPresence keeps camera+preview only.
- **lowpoly** (richest today — acceptance bar): domain `mesh` (object/face/edge/vertex, u32→String, methods [Pick,Rectangle,Lasso], merges incl. Range); `🛠️options/🗂️select` re-renders framework state, dispatches injected mode/granularity actions.
- **writer**: caret/range stays app-side (moved out of undo via the same Interaction lane); domain `ast` with Topology hierarchy + transitive select/hover — `jack_ast_node_for_selection` covering logic dissolves into generic transitivity.
- **♾️infinite**: `worldSelect`/`worldHover` → framework verbs; domain `world` (surface/item, PathDelimited "surfaceId/id").

## Waves & agent workforce

Repo/ticket discipline (applies to every wave): open ONE ticket via `mcp__repo__ticket_open` (goal `r2602/runningsketchpad`, titleized title, e.g. "First-Class Hover and Selection Mechanism"); all scratch/logs as `.txt` inside the ticket folder; research/summaries as `.md` in the ticket folder (referenced in chat, not pasted); every subagent brief says **do NOT call ticket_close**; `ticket_close` at the end with explicit path + files list (first entry an ASCII path); `📌️important.md` cleared last. **Never** pass `isolation:"worktree"`; no git commands. Regions (`#region`) for all added code; extend existing test files only; docstrings start with emoji.

Orchestration: main session spawns **1 Opus coordinator agent** (persistent, via Agent tool, model `opus`) that owns wave sequencing judgment: reviews each wave's outputs, adapts W4 briefs, times the W3 red window, staggers cargo acceptance runs (≤3 concurrent to avoid target/ lock thrash). Main session runs one **Workflow script per wave**; inside scripts, `agent()` calls use `model:'sonnet'` for execution tasks and `model:'haiku', effort:'low'` for read-only verification. Coordinator reviews between workflow invocations (SendMessage). W4 exceeds the 15-agent guideline deliberately (user requested a parallel workforce).

- **W0 — `🕹️interaction` module + store lanes (sequential, 1 Sonnet).** New module + schema leaves + glue; `HistoryLane` in `🏪️store` (cursor-skip semantics + in-file tests). Accept: `cargo test -p semio-framework` (+ store crate tests). Haiku: Rust/TS state-machine parity read.
- **W1 — manifest (sequential, 1 Sonnet; contended file — coordinator announces window, re-read before edit).** InteractionDefinition family, ActionKind::Interaction, AppDefinition/WindowKind fields, injected actions, icon purge, ViewModel/UiTree field changes, TS regen. Accept: `cargo test -p semio-framework --features typegen`, manifest in-file serde/injection tests extended.
- **W2 — parallel lanes (2 Sonnet + Haiku verifiers).**
  - W2a: wire `PresenceInteraction` bit-7 codec + `InteractionStore`/hover ephemeral + sync heartbeat + regenerated wire fixtures (sync :2320; codec tests after :829).
  - W2b: TS UI — Tree delegation, React host + Shell TSX modifier→merge policy, peer-interaction rendering. Accept: `bun vitest run` (extend existing suites).
- **W3 — plugin SDK breaking pass (sequential, 1 Sonnet; RED WINDOW opens).** Reserved-verb interception, `InteractionView` param on handle/clipboard, AppBuilder methods + validation, wrapper topology derivation + presence stamping + UiTree stamping, Shell 🧊 Rust, surface Rust API changes, taxonomy gate. Accept: `cargo test -p semio-framework-plugin` green; workspace intentionally red. W4 briefs pre-staged during W2.
- **W4 — per-app migrations (parallel, 18 Sonnet tasks; RED WINDOW closes).** One task per plugin crate (17 above) + `♾️infinite`; multi-app plugins (puzzle, block, procedural, gis, trinity) migrate all their apps in the one task since they share a crate. Files strictly within one plugin dir each → no cross-executor conflicts. Accept per task: `cargo test -p semio-s-plugin-<x>` green (coordinator staggers to ≤3 concurrent cargo runs) + Haiku residue grep: `selected_.*_ids|hovered_|set-selection|setHover|selection_mode|selection_method` in config/commands (allowlist genuinely app-specific presence like `preview_off_node_ids`).
- **W5 — verification sweep (parallel Haiku + 1 Sonnet).** `cargo check --workspace`; `cargo test -p semio-framework -p semio-framework-os-kernel -p semio-framework-plugin` + per-app; `bun vitest run`; fixture grep over `📚️examples` for stale selection/hover DSL; dev-preview smoke; write closing summary `.md` in ticket folder; close ticket.

Risks: concurrent human churn on manifest/plugin (re-read immediately before editing, keep W1/W3 short); shared `target/` cargo flock (stagger); repo-wide build failures may be another session's refactor (check git log vs ticket start commit before assuming ours); auto-commit means every wave except the declared W3→W4 window must leave framework crates check-green.

## Verification

- Extend in-file `mod tests` only: manifest (~:4300+), plugin (dispatch/EphemeralEmit ~:9200+, taxonomy ~:2599), wire codec (after :829, bit-7 round-trip; old fixtures regenerated, no legacy decode), sync fixture writer (:2320), store (new lane tests beside existing), Tree vitest, per-app config/command round-trips (e.g. flow :355–397 — delete selection cases, add topology tests).
- Commands: `cargo check --workspace`; targeted `cargo test -p …`; `bun vitest run`; typegen via `cargo test -p semio-framework --features typegen`.
- OS dev preview smoke (via launch.json dev server, one tab, patient cold boot ~20 WASM plugins): flow — marquee select, shift-range, mod-toggle, hover a group → descendants highlight; lowpoly — switch granularity via injected action, lasso faces; **two tabs on one hub doc → both peers' selection/hover render in every app** (the uniform-presence acceptance test); reload → own selection restored (persisted-local); mod+z after a pick → reverts last document edit, not the pick (lane mechanism acceptance); storybook Tree regression (`bun nx run workspace:dev-storybook-framework`).
