# W3b — Plugin SDK Owns Hover/Selection at Runtime

## What changed

### `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (crate `semio-framework-plugin`)

- **`ArtifactApp` trait** (region `🔖️` around `handle`/`copy_fragment`/`cut_operations`):
  - `handle`, `copy_fragment`, `cut_operations` each gain a new `interaction: &InteractionView<'_>` parameter (inserted right after `cfg`, matching the trait's existing no-`&self`, positional-view-args convention).
  - New defaulted `fn interaction_topology(doc, cfg) -> protocol::InteractionTopology` (default: empty topology — correct for `Flat`/`UiTree`/`PathDelimited`, which the wrapper self-derives; an app declaring `HierarchyProvider::Topology` must override it).
- **`InteractionView<'a>`** (new, region `🔖️InteractionView`, beside `ConfigView`/`DraftView`/`PresenceView`/`TransientView`): `selection(domain) -> &DomainSelection`, `hover(domain, channel) -> &DomainHover`, `active_granularity(domain) -> Option<&str>`, `active_mode(domain) -> Option<SelectionMode>`. Empty domains return a static empty default, never `None`/panic.
- **`VcsArtifactApp<A>`** gains three fields: `interaction_store: ConfigStore<protocol::InteractionState, InteractionConfigMutation>` (persisted-local, its own `ArtifactStore` instance — see "Design deviation" below), `interaction_hover: InteractionHoverState` (`BTreeMap<String, DomainHover>`, ephemeral, never persisted), `interaction_ui_topology: HashMap<String, DomainTopology>` (cache populated by `render`'s post-pass, for `HierarchyProvider::UiTree` domains).
- **`InteractionConfigMutation`** (new, region `🔖️InteractionConfig`): the one mutation `interaction_store` ever applies — a whole-`InteractionState` replace (`SetState`), with hand-written `MutationDiff`/`Mutation`/`OpText`/`OpBinary` impls (JSON-line/JSON-bytes encoding, no `dsl_derive`).
- **`AppActionRegistry`** gains `interactions: HashMap<String, InteractionDefinition>` (from `AppDefinition.interactions`) plus `interaction(id)`/`interactions()` accessors — the runtime source of domain declarations `dispatch_interaction_action` resolves against.
- **Dispatch interception** (`dispatch_action`, region `🔖️InteractionDispatch`): the six framework verbs (`interactionSelect`, `interactionHover`, `clearSelection`, `selectAll`, `setSelectionMode`, `setInteractionGranularity`) are checked FIRST, before history/revert/filter/noteShellCommand/clipboard, and routed to `dispatch_interaction_action`. That method runs the pure `next_selection`/`next_hover` machine, then always calls `revalidate_and_persist_interaction_state` (validate_state + persist), then `record_command(action, ActionKind::Interaction, …)`, then returns `UiDirtyScope::Full`.
- **Post-document-dispatch pruning** (task 4): `dispatch_emit`'s document-mutation branch calls `revalidate_interaction_state_after_document_change` right after `self.store.dispatch(vcs_command)` succeeds — a no-op (skips all topology/validate work) when the app declares no interaction domains at all.
- **`finish_recorded`**: `skip_history_panel` now also matches `ActionKind::Interaction` (was `ActionKind::View` only) — an interaction verb's dispatch never dirties the history panel body.
- **`render`** (`PluginApp::render`): after `A::render(...)` produces a `UiNode` (both the live-cache and `snapshot_override_json` branches), `stamp_and_cache_interaction_ui` recursively walks it (through `Stack`/`Section`/`Group`/`Field`); for every `UiNode::Tree` carrying `interaction_domain: Some(id)` it (a) derives/caches a `DomainTopology` from the tree's own `sections`/`items` nesting into `interaction_ui_topology` and (b) calls `ui_tree_stamp_presence` from the combined `InteractionState`, OVERWRITING whatever `selected`/`highlighted` the app itself stamped via `PanelTreeBuilder`.
- **`AppBuilder::build_definition`**: added the `transitive ⇒ hierarchy != Flat` assertion for both `hover.transitive` and `selection.transitive` (the unique-domain/non-empty-granularity/method/merge and window-kind-interaction-ref checks already existed from W1).
- **Topology resolution** (free fns + `resolve_domain_topology`/`build_full_interaction_topology`): `Flat` → empty (and OMITTED from the `InteractionTopology` map built for `validate_state`, so pruning is skipped for it — see design note below); `Topology` → `A::interaction_topology(doc, cfg)`; `UiTree` → the `interaction_ui_topology` cache; `PathDelimited{delimiter}` → self-derived from every id already known to the current dispatch (batch targets ∪ current selection/hover ids) by splitting on `delimiter` — a documented approximation, not a full-universe derivation.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs` (crate `semio-framework-os-kernel`)

- Added `impl store::ArtifactPack for InteractionState` (region `🔖️InteractionStorePack`, right after `🔖️PresenceInteraction`) — bridges through the existing `impl ArtifactPack for serde_json::Value`. **Required to live here, not in the plugin crate**: the orphan rule needs `ArtifactPack` (defined in this crate's `os_store` module) or `InteractionState` (defined in this crate's `os_spr::wire` module) to be local to the impl's crate; both are local to `semio-framework-os-kernel`, neither is local to `semio-framework-plugin` (which only sees them through its `store`/`protocol` aliases).

## Design deviation from the master doc (read this before W4)

The master doc says persisted selection/mode/granularity goes "through **the existing** ConfigStore via `ApplyInLane{lane: HistoryLane::Interaction}`" — read literally as `VcsArtifactApp.config_store: ConfigStore<A::Config, A::ConfigMutation>`, the app's OWN config store. That is **not** what got built. Two independent blockers ruled it out:

1. **Orphan rule**: `InteractionConfigMutation`'s `Mutation<InteractionState>`/`MutationDiff<InteractionState>` impls need `InteractionState` reachable as the trait's type parameter, which is fine — but folding `InteractionState` INTO `A::Config` would need a generic wrapper `FrameworkConfig<C> { app: C, interaction: InteractionState }` wrapping the app's config type, with a matching `FrameworkConfigMutation<M>` wrapping `A::ConfigMutation`. That wrapper is the only way to keep it zero-touch for apps.
2. **Blast radius**: adopting that wrapper means changing `VcsArtifactApp.config_store`'s type parameters everywhere — `ConfigView<'_, Self::Config>` construction, `A::initial_config()`, config pack import/export, `test_config()`, `dispatch_emit`'s config-mutation branch, media serialization — dozens of call sites across an already 11.9k-line file, for every one of the ~17 downstream apps' `Config`/`ConfigMutation` types.

Instead, `VcsArtifactApp` owns a **separate** `interaction_store: ConfigStore<InteractionState, InteractionConfigMutation>` — same `ConfigStore<C, M>` type alias (`= ArtifactStore<C, M>`), same `ApplyInLane{lane: HistoryLane::Interaction}` mechanism, just instantiated with the framework's own type parameters instead of the app's. This is zero-touch for every app's `Config`/`ConfigMutation` (matches "no app ever implements this again") and behaviorally equivalent for everything that matters today: the general "undo"/"redo" actions only ever dispatch against `self.store` (the DOCUMENT store) — never `config_store`, and now never `interaction_store` either — so a pick is unreachable from undo/redo by construction, with or without the `HistoryLane` tag. `revertToCommand`'s two edit-linked branches (`self.store`/`self.config_store`) are similarly untouched; an interaction verb's `CommandLogEntry` carries neither `edit_id` nor `config_edit_id`, so it is simply never `revertible` (confirmed by `interaction_verbs_are_recorded_under_the_interaction_action_kind`).

If a future wave specifically wants interaction state folded into the app's visible `A::Config` (e.g. so a host that only knows how to read `config_pack()` sees it too), that requires the `FrameworkConfig<C>` wrapper and the full call-site sweep described above — flagged here, not attempted in this wave.

## `HierarchyProvider::Flat` and `validate_state` pruning

`validate_state` prunes a domain's ids by checking `InteractionTopology.domains.get(domain_id)`: `None` → keep every id (no membership info); `Some(topology)` → keep only ids `topology.contains(id)`. An **empty** `DomainTopology` is therefore NOT the same as "no pruning" — it means "prune everything, every time." `Flat` domains are deliberately **omitted** from the `InteractionTopology` built for `validate_state` (see `build_full_interaction_topology`), not inserted empty, so a `Flat` domain's selection is simply never auto-pruned by this mechanism. An app that wants deleted-node pruning for a nominally-flat domain must declare `HierarchyProvider::Topology` and return one root `TopologyNode` per currently-valid id from `interaction_topology` — `Flat` genuinely has no structure to check staleness against by its own definition.

## Acceptance

`cargo test -p semio-framework-plugin`: **165 passed, 0 failed** (156 pre-existing + 9 new). Real output: `w3b-cargo-test-semio-framework-plugin.txt` (same folder). Also spot-checked (not required by this task, but touched by it): `cargo test -p semio-framework`: 105 passed (unaffected — this wave never touched that crate's own files); `cargo check -p semio-framework-os-kernel`: 0 errors. `cargo test -p semio-framework-os-kernel` has one PRE-EXISTING/unrelated failure (`os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures`, "found 0 usable grammar+fixture pairs") — a repo-wide example-fixture sweep test with zero code path through anything this wave touched (no DSL grammar/example asset file was edited); `git status` shows ~35 already-modified files outside this ticket's W3b scope (earlier waves' uncommitted work), the much likelier cause. Not chased further per this task's scope.

## New signatures every app must implement/call (W4 — copy-pasteable)

```rust
// ArtifactApp::handle — interaction is the 3rd param, before draft/engines:
fn handle(
    command: &Self::Command,
    doc: &ArtifactView<'_, Self::Snapshot>,
    cfg: &ConfigView<'_, Self::Config>,
    interaction: &InteractionView<'_>,
    draft: &DraftView<'_, Self::Draft>,
    engines: &EngineHandles,
) -> Result<Emit<Self::Mutation, Self::ConfigMutation, Self::DraftMutation>, Fault> {
    // read current selection/hover instead of storing it:
    let selected: &[String] = &interaction.selection("mesh").ids;
    let hovered: &[String] = &interaction.hover("mesh", "pointer").ids;
    let granularity: Option<&str> = interaction.active_granularity("mesh");
    // ... existing match on `command` ...
}

// ArtifactApp::copy_fragment / cut_operations — interaction is the 3rd param:
fn copy_fragment(
    doc: &ArtifactView<'_, Self::Snapshot>,
    cfg: &ConfigView<'_, Self::Config>,
    interaction: &InteractionView<'_>,
) -> Result<ClipboardFragment, ClipboardError> {
    let selected = &interaction.selection("mesh").ids;
    // ... build fragment from `selected` instead of `cfg`/app-owned selection fields ...
}

fn cut_operations(
    doc: &ArtifactView<'_, Self::Snapshot>,
    cfg: &ConfigView<'_, Self::Config>,
    interaction: &InteractionView<'_>,
) -> Vec<Self::Mutation> { /* same idea */ }

// ArtifactApp::interaction_topology — ONLY needed for a HierarchyProvider::Topology domain
// (Flat/UiTree/PathDelimited domains: do not override, the wrapper self-derives):
fn interaction_topology(
    doc: &ArtifactView<'_, Self::Snapshot>,
    cfg: &ConfigView<'_, Self::Config>,
) -> semio_framework::InteractionTopology {
    let mut domains = std::collections::BTreeMap::new();
    domains.insert(
        "graph".to_string(),
        semio_framework::DomainTopology {
            ordered: doc.snapshot.nodes.iter().map(|node| semio_framework::TopologyNode {
                id: node.id.clone(),
                granularity: "node".into(),
                parent: node.group_id.clone(), // None for a root
            }).collect(),
        },
    );
    semio_framework::InteractionTopology { domains }
}
```

Migration mechanics per app (see the ticket's `📓️inventory.md` for the per-crate table):
1. Delete `selected_*_ids`/`hovered_*`/`selection_method`/`selection_mode_default` fields from `🎚️config` + schema leaves; delete `🎮️commands/{set-selection,set-hover,select-*,*-hover,clear-selection,select-all}`.
2. `AppBuilder::interaction(InteractionDefinition { .. })` + `.window_kind_interactions(id, vec![InteractionRef::new("domain")])` — the six actions auto-inject.
3. Add the `interaction: &InteractionView<'_>` param to `handle`/`copy_fragment`/`cut_operations`; read `interaction.selection(domain)`/`interaction.hover(domain, channel)` wherever the deleted config fields used to be read.
4. `PanelTreeBuilder`: add `.interaction_domain("domain")`; `.selected()`/`.highlighted()` calls on that SAME tree become dead code (the wrapper overwrites them) — safe to delete, harmless to leave.
5. u32 ids (lowpoly, cad, process, …): stringify at the boundary — `InteractionTarget.id`/`DomainSelection.ids` are always `String`; parse back to `u32` only inside the app's own `interaction_topology`/`handle` bodies.

## Known gap (flagged, not fixed this wave)

`revalidate_interaction_state_after_document_change` is wired into the SOLITARY document-dispatch path (`dispatch_emit`'s non-`child_emits` branch) only. `dispatch_emit_group` (the UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM composite/child-artifact dispatch path) does not call it — a composite gesture that deletes a selected id purely through a child document edit will not prune that id until the NEXT solitary dispatch touches this same app. None of the 17 inventoried apps use composition-groups for their interaction domains today, so this is not expected to bite W4, but a future wave adding it should touch `dispatch_emit_group`'s tail the same way.
