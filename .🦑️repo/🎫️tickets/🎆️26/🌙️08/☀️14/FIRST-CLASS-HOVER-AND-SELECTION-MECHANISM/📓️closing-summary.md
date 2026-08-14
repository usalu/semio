# Closing Summary — First-Class Hover and Selection Mechanism

Hover and selection are no longer per-app code. They are one declared framework mechanism, owned end to end by the framework, with declarative transitive hover, selection modes/granularities/methods, uniform presence broadcast, and event-sourcing-correct state classification.

## What the mechanism is

**Declaration (manifest).** `AppDefinition.interactions: Vec<InteractionDefinition>` and `WindowKindDefinition.interactions: Vec<InteractionRef>`, siblings of actions/utilities/tools/commands. One `InteractionDefinition` per domain carries `granularities`, a `HierarchyProvider` (`Flat` | `Topology` | `UiTree` | `PathDelimited{delimiter}`), a `HoverSpec{enabled, transitive, channels, broadcast}` and a `SelectionSpec{modes, methods, merges, transitive, broadcast}`. `AppBuilder.interaction()` / `.window_kind_interactions()` declare and bind them, validated at build time (unique domains, non-empty granularities, `transitive ⇒ hierarchy != Flat`, refs resolve).

**Algebra (os-kernel).** `next_selection` / `next_hover` / `validate_state` plus `InteractionState`, `DomainSelection`, `DomainHover`, topology types — one implementation, in `📡️spr/📡️wire/🦀️component.rs`, mirrored handcrafted in `🕹️interaction/🟦️component.ts` for the UI. Merge modes: Replace, Additive, Subtractive, Invertive, Range.

**Runtime (plugin SDK).** `VcsArtifactApp` intercepts the six framework-injected verbs (`interactionSelect`, `interactionHover`, `clearSelection`, `selectAll`, `setSelectionMode`, `setInteractionGranularity`) *before* app dispatch, runs the pure machine, persists, records under `ActionKind::Interaction`, and emits the dirty scope. Apps read state through `InteractionView` on `handle`/`copy_fragment`/`cut_operations`; only `HierarchyProvider::Topology` domains implement `interaction_topology`. `validate_state` runs after every document dispatch, so ids deleted from the document fall out of selection with zero app code.

**Persistence.** Selection/mode/granularity are persisted-local through a new store `HistoryLane` mechanism: entries are tagged `HistoryLane::{Document, Interaction}`, default undo/redo walks past the Interaction lane to the nearest document edit, and `UndoInLane`/`RedoInLane` exist for walking a lane deliberately. Hover is never persisted. This is a general mechanism, not a hover/selection special case.

**Presence.** `PresencePeer.interaction: Option<PresenceInteraction>` on wire bit 7, assembled from `InteractionState` on the existing heartbeat. Every app broadcasts selection and hover with no app code; the Shell renders any peer's state generically via `derivePeerInteractionByDomain`/`peerIdsSelecting`/`peerIdsHovering`. Only explicit ids cross the wire — receivers expand transitive closures with their own topology.

**UI.** `UiTreeNode.interaction_domain` binds a rendered tree to a domain; the framework stamps `UiPresence{hover, selected}` onto its items, overwriting anything the app stamped. `UiTreeNode.selected_ids`/`highlighted_ids`/`selection_change`, `UiTreeItemNode.hover_action`/`unhover_action` and `ViewModel.selection_json` are deleted. Tree delegates to the shared machine instead of its private one. One modifier→merge policy lives in the React host (shift=Range, mod=Invertive, alt=Subtractive).

## Scope delivered

31 plugin crates migrated (17 from the original inventory + 14 the inventory missed), plus the OS `♾️infinite` world surface. Per-app selection/hover config fields, mutations, command directories and presence mirrors are deleted; retained verbs that operate *on* the selection (delete/duplicate/focus/zoom/rotate/scale/translate-selection, all nine `nudge-selection` variants, select-same-kind, set-selected-opacity, select-generation) now read `InteractionView`.

Headline behaviors now generic rather than bespoke:
- **writer**: `jack_ast_node_for_selection`'s covering-node logic dissolved into declarative `transitive` on a `Topology` AST domain.
- **lowpoly**: ten config fields (four granularity booleans, merge mode, method, keys, ids…) replaced by one declaration.
- **flow**: dual-tracked selection (config store *and* presence mirror, guaranteed to drift) collapsed to a single framework-owned source.
- **Tree**: its two distinct clamps preserved deliberately — `nextSelection` clamps a live pick to the last target, `validateState` normalizes persisted ids to the first.

## Deliberate design decisions

1. **Interaction state lives in a sibling `interaction_store`**, not folded into `A::Config`. Folding it in needed a generic `FrameworkConfig<C>` wrapper touching dozens of call sites across every app. Same `ConfigStore` + `HistoryLane` machinery, zero-touch for apps. A future wave wanting it inside `config_pack()` must do that wrapper sweep.
2. **The runtime layer lives in os-kernel, the declaration layer in `semio-framework`.** `semio-framework` depends on os-kernel and the reverse edge would be a cycle; `InteractionDefinition`/`GranularityDefinition` carry `LocalizedLabel`/`IconName`, which os-kernel cannot see. `validate_state` therefore takes `InteractionOutline` (via `InteractionDefinition::outline()`).
3. **`HierarchyProvider::Flat` domains are excluded from pruning.** An empty topology means "prune everything", not "don't prune". A flat domain wanting deleted-id pruning declares `Topology` returning root-only nodes.
4. **Navigation is not selection.** The playbook generation picker and similar "which thing is active" controls keep plain per-item actions and declare no domain.
5. **The world surface emits into the domain bound to its window**, not a hardcoded `"world"`. Hardcoding produced two selection universes for one entity — CAD's world picks were silently invisible to CAD's own `interaction.selection("cad")`. `world_interaction_definition()` remains the fallback for a window binding no app domain.

## Verification

`cargo check --workspace --keep-going` is clean apart from `semio-compose-rs` (another session's `os_vcs` refactor in `compose/client/`, untouched by this work). `semio-framework` 105 tests green. `semio-framework-os-kernel` 862 green with one pre-existing `os_dsl::fixture_sweep::m5_cross_artifact_rejection` failure that scans example fixtures and is independent of this work. Framework TS package 146 vitest green.

## Outstanding

- **`dispatch_emit_group`** (composite/child-artifact dispatch) does not yet run post-dispatch revalidation. No inventoried app's domains are affected, but a composite app adding one would not get automatic pruning.
- **`♾️infinite` lib tests cannot compile** — another session's in-flight change removed `Index` from `DslValue`, breaking the `args["key"]` idiom used throughout that file's pre-existing tests. `cargo check` was the gate for that crate.
- **`PathDelimited` topology self-derivation** only knows ids already seen by the current and prior dispatches. A full-universe path topology needs `HierarchyProvider::Topology` with an explicit `interaction_topology`.
- **Unbound world domains**: block-3D's world window and puzzle-3D/5D were left unbound on purpose — they already emit their own interaction verbs from bespoke logic, and binding risked double-emission. Worth a follow-up.
- **Runtime smoke testing was not performed.** Compilation and unit tests pass; the OS dev-preview scenarios in the plan (two-peer presence, transitive hover on a group, undo-skips-selection, marquee granularity switching) have not been exercised in a running app.
