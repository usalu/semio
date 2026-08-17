# Residue Sweep — First-Class Hover & Selection Mechanism

Read-only, adversarial. Verifies the claim "hover and selection are now a first-class framework
mechanism that fully replaced per-app hover/selection" against the actual repo state, not the wave
summaries. **Verdict: the claim is false for a large slice of the app surface.** Two distinct classes
of residue survive:

1. **10 plugin crates were never migrated at all** (flow, dag, reasoning/wires, sequence, vcs,
   imperative, forms, architect, playbook, animate/present) — full duplicate per-app selection/hover
   state, exactly the pattern the ticket set out to dissolve.
2. **The `♾️infinite` (world/3D) domain was scaffolded but never wired up** — the framework `"world"`
   `InteractionDefinition` exists and is unit-tested, but zero apps call `.interaction()` with it and
   the UI (`World3dHost`) never dispatches the injected verbs; the pre-migration `worldSelect`/
   `worldHover`/`setSelection` command path is what actually runs today, unconditionally, for every
   3D-world app (cad, gis-3d, procedural-3d, puzzle-3d, block-3d, remodel, process-3d, shooting, space).

The 17 crates the ticket's own W4 task list *did* cover (writer, procedural, gis, shooting, process,
lowpoly, layout, cad, remodel, trinity, draw, raster, note, puzzle, block, space, sourcing) are migrated
cleanly — no residue found there beyond documented, intentional retentions.

---

## 1. Category (b) — genuine leftover per-app hover/selection state

### 1a. Ten plugin crates outside the W4 scope, fully unmigrated

The master doc's own "Context" section (`📋️master.md:5`) names `flow`, `dag`, `playbook`, `sequence`
by name as apps with hand-rolled hover/selection that duplication the mechanism was meant to dissolve.
None of them appear in the actual W4 work-list (`📋️master.md:94`, 17 crates), and none were touched:
every one still has its own config field(s), presence-facet duplicate(s), `Set*` config-mutation
variant(s), and `set-selection`/`set-hover`/`select-all`/`clear-selection` command dirs, wired through
the app's own `handle_action`/mutation-application path — not through `InteractionView`/
`dispatch_interaction_action`. None of these 10 crates call `AppBuilder::interaction(...)` anywhere
(`grep -c "\.interaction(" == 0` for all ten).

| Crate | Config field(s) | Mutation | Command dir(s) |
|---|---|---|---|
| `🌊️flow` | `selected_node_ids`/`selected_edge_ids`/`selected_handle_ids` (config + presence + schema) | `FlowConfigMutation::SetSelection` | `🎮️commands/🗂️set-selection`, `🗂️clear-selection`, `🗂️select-all`, `🗂️select-node`, `🗂️node-graph-select` |
| `🕸️dag` | `selected_node_ids` (config + presence + schema), `hovered_node_id`/`hovered_edge_id` (presence) | `DagConfigMutation::SetSelection` | `🎮️commands/🗂️set-selection`, `🕸️delete-selection`, `🗂️select-node`, `🗂️node-graph-select` |
| `💡️reasoning` (`wires`) | `selected_ids` (config + presence + schema) | `WiresConfigMutation::SetSelection` | `🎮️commands/🗂️set-selection`, `🗑️delete-selection`, `🗂️document-select` |
| `🎬️sequence` | `selected_step_ids` (config + presence + schema) | `SequenceConfigMutation::SetSelection` | `🎮️commands/🗂️selection` (`SetSelection`), `🕸️node-graph` |
| `🌿️vcs` | `selected_checkpoint_ids` (config + schema) | `VcsDemoConfigMutation::SetSelection` | `🎮️commands/🗂️set-selection` |
| `📜️imperative` | `selected_step_ids` (config + presence + schema) | (via `set-selection` handler) | `🎮️commands/👁️set-selection` |
| `📋️forms` | `selected_ids` (config + presence + schema) | `FormsConfigMutation::SetSelection` | `🎮️commands/🗂️set-selection`, plus `question`/`step` commands push `SetSelection` directly |
| `🏛️architect` | `selected_ids` (config + presence + schema) | `ArchitectConfigMutation::SetSelection` (has a hand-decoded wire-varint test asserting its byte shape) | `🎮️commands/🗂️selection` |
| `📖️playbook` | `selected_ids` (config + presence + schema) | `PlaybookConfigMutation::SetSelection` | `🎮️commands/🗂️set-selection` |
| `🎞️animate` (`present`) | `selected_ids` (config + presence + schema) | `PresentConfigMutation::SetSelectedIds` | `🎮️commands/👁️set-selected-ids`, `🀄️delete-selection` |

Representative evidence (full grep output cross-checked line-by-line, not just directory names):
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎚️config/🦀️component.rs:37,39,41,194` — `selected_node_ids`/`selected_edge_ids`/`selected_handle_ids` fields + `SetSelection { node_ids, edge_ids, handle_ids }` variant, still applied at line 301 and consumed by `🎮️commands/🕸️spotlight-commit`, `🗂️context-menu-at`, `🗂️graph-pointer-down`, `🕸️node-graph-edit`, `🪟️remove-widget`, `🪟️rename-flow-widget`, `🗂️select-all`, `🗂️select-node`, `🪟️add-widget`.
- `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎚️config/🦀️component.rs:138,224` — `SetSelection { node_ids }` still the only way `dag` mutates selection (`🔧️add-node:25`, `🕸️delete-selection:24`, `🗂️select-node:16`, `🗂️node-graph-select:16`, `🗂️graph-pointer-down:14`, `🕸️node-graph-edit:24`, `🔧️remove-node:24`, `🔧️rename-dag-node:28`).
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎚️config/🦀️component.rs:110-116` — `PresentConfigMutation::SetSelectedIds { ids }` alongside `SetEngagementInput`/`SetLocale`; command dir `👁️set-selected-ids` still exists.

These are not stray dead fields — they are the live, sole selection/hover path for these apps today:
`FlowCommand::SetSelection`/`DagCommand::SetSelection`/`WiresCommand::SetSelection`/etc. are dispatched
from real UI event handlers (pointer-down, node-graph-select, etc.), not just test fixtures.

**Impact**: this contradicts both the ticket's stated goal ("dissolve hover/selection out of per-app
commands... for ~14 apps: flow, lowpoly, writer, dag, procedural, cad, process, remodel, raster,
trinity, playbook, sequence, layout") and the closing claim of a full replacement. 4 of those 14 named
apps (`flow`, `dag`, `playbook`, `sequence`) were simply dropped from the execution plan between the
design doc and the W4 task list, and 6 more apps that also had the same duplication (`vcs`,
`imperative`, `forms`, `architect`, `animate`/`present`, plus `wires`) were never named or migrated
either.

### 1b. `♾️infinite` / world domain: declared, tested, never connected

This is the more serious finding because `♾️infinite` **is** explicitly in scope (`📋️master.md:94,104`:
"plus OS `♾️infinite`"; W3b names it one of "the hardest three" reference-traced migrations) and W3b's
own summary claims the interception plumbing was built. What's actually there:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs:2683` defines
  `pub fn world_interaction_definition() -> InteractionDefinition`, with its own doc comment: *"declared
  once here so any app mounting a world3d surface can push it onto its `AppDefinition.interactions`
  (**wave 4**)"* — and a unit test
  (`world_interaction_definition_declares_path_delimited_item_domain`, line 3939) confirming its shape.
- **`grep -rn "world_interaction_definition()" .` (whole repo) returns exactly two lines: the
  definition itself and its own unit test.** No app builder anywhere calls
  `.interaction(world_interaction_definition())`. The domain is dead code — declared, never attached.
- `🌍️world/🦀️component.rs:2714-2724` still has live, reachable match arms for the OLD verb strings
  `"worldSelect"`, `"worldHover"`, `"setSelection"` that mutate `state.selected_ids`/
  `state.local_hover_id` directly.
- `🌍️world/🦀️component.rs:2784,2793` also has NEW match arms for `"interactionSelect"`/
  `"interactionHover"` gated on `WORLD_INTERACTION_DOMAIN_ID` — explicitly commented as *"still the
  OPTIMISTIC LOCAL PREVIEW only; the framework's `next_selection`/`next_hover` machine (not this file)
  is the source of truth once the round-trip settles."* This code is unreachable in production: nothing
  ever sends `interactionSelect`/`interactionHover` with `domainId: "world"` (see next point), and even
  if it did, `dispatch_interaction_action`'s registry lookup (`plugin/🦀️component.rs:7384` etc.) would
  fault with "undeclared interaction domain" since no app's `AppActionRegistry.interactions` contains
  `"world"`.
- **The UI never dispatches the new verbs for world objects.** `World3dHost/🟦️component.tsx:4102`
  (`handleInstancePointerDown`) and `:4519` (marquee finalize) both call `dispatch("worldSelect", ...)`
  — the pre-migration action. `grep -n '"interactionSelect"\|"interactionHover"'` on that file returns
  nothing. Hover dispatch (`worldPick`/`worldHover`) is likewise untouched.

**Net effect**: every app that renders through `World3dHost` — `cad`, `gis` (3d), `procedural` (3d),
`puzzle` (3d), `block` (3d), `remodel`, `process` (3d), `shooting`, `space` — still does its world-level
pick/marquee/hover selection through the old `WorldSunConfig`-adjacent `World3dState.selected_ids`/
`local_hover_id`/`selection_method` fields (`🦀️component.rs:363,376` in both
`♾️infinite/🦀️component.rs` and `♾️infinite/🌍️world/🦀️component.rs`, plus
`🎲️board/🔌️ports/➡️directed/🦀️component.rs:76` `selection_exit_highlight_ids` and
`➕️normal/🦀️component.rs:299-301` `hovered_id`/`hovered_kind`, and the `🕸️dag`-flavored board port at
`➡️directed/🕸️dag/🦀️component.rs:2112-2228`), not through the framework mechanism. This is the single
largest surviving instance of the exact duplication pattern the ticket exists to remove — bigger than
any one of the 10 crates in §1a, because it's shared low-level infrastructure used by nine app surfaces
at once.

Component-level (vertex/edge/face) picking inside a world (`worldPick`/`setSelection` for
`component_ids`/`granularity`) is separately, explicitly flagged as out-of-scope in the same doc comment
("a separate, unconverted mechanism... out of this wave's named scope") — that one is a **documented**
gap, not a silent residue, so it is not counted as a bug here. The instance-level `worldSelect`/
`worldHover` gap is *not* documented as out-of-scope anywhere — the doc comment's own language ("so any
app... can push it onto its interactions (wave 4)") implies it was expected to land in wave 4 and did
not.

---

## 2. Category (a) — legitimately retained, checked individually

These matched the grep patterns but are not residue; each was read in full context:

- **`📐️cad` `selected_node_ids`/`hovered_reference_id`** (`🎚️config/🦀️component.rs:74,78`) — explicit
  doc comment: mesh selection/hover *is* migrated (`selected_object_ids`/`hovered_object_id`/etc. were
  deleted per the comment at line 66-71); `selected_node_ids` is the document-tree ("node") selection, a
  deliberately separate, still-app-owned granularity distinct from the framework `"cad"` mesh domain;
  `hovered_reference_id` is a per-pane background-image reference hover, unrelated to mesh hover. Both
  are named and justified in-file, not silent leftovers.
- **`💠️lowpoly` engine `selected_face_ids`/`selected_vertex_ids`/`selected_edge_ids`/
  `selection_vertex_ids`** (`⚙️engine/🦀️component.rs:188-230`) — pure functions/an internal engine mesh
  cache (`self.selection.mode`/`self.selection.ids`) fed by `apply_selection(mode, ids)` called from
  outside per-dispatch; the app-level command dir (`🎮️commands/🗂️selection/component.rs:4-5`) explicitly
  documents `SetSelection`/`ToggleSelectionKind`/etc. as DELETED and framework-owned. Engine-internal
  cache, not a second source of truth.
- **`📏️layout` `SceneQuery.selected_ids`/`hovered_id`** (`⚙️engine/🎬️scene/🦀️component.rs:364-365`) —
  a borrowed-slice render/hit-test argument struct (`pub struct SceneQuery<'a>`), populated by the
  caller from `InteractionView` each call; not stored state.
- **`🧩️puzzle` `selected_*_ids` getters** (`🎛️apps/🧊️3d/🦀️component.rs:1795-1807`,
  `🖐️5d/🦀️component.rs:1280-1286`, `◻2d/🦀️component.rs:788`) — read-only accessor methods over
  `InteractionView`-derived state for WASM/test consumption; all three apps also declare
  `.interaction(...)` and dispatch `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/
  `setSelectionMode`/`setInteractionGranularity` explicitly (`🧊️3d/🦀️component.rs:2876,2888,2898`,
  `🖐️5d/🦀️component.rs:2123,2135`, `◻2d/🦀️component.rs:1477,1489`).
- **`🗒️note` `selected_block_ids`** (`🦀️component.rs:94`) and **`🎥️shooting`
  `selected_shot_ids`/`selected_asset_ids`** — both crates independently declare `.interaction(...)` and
  dispatch `interactionSelect` (`note/🦀️component.rs:444`, `shooting/🦀️component.rs:643` and
  `📌️panels/📄️artifact/🦀️component.rs:28`); the remaining fields are for domains/asset-lists distinct
  from the migrated interaction domain (spot-checked, not a duplicate selection surface).
- **`gis`, `layout`, `draw`, `space`, `trinity` (`jack`/`rewrite`), `raster` `SetSelection`/`SetHover`
  string matches** — every one of these is inside a `//` or `///` doc comment explaining what used to be
  there and was deleted (e.g. `🖍️draw/🦀️component.rs:656`: *"instead of writing a
  `DrawConfigMutation::SetSelection`"*; `🪐️space/…/🧩️delete-selection/component.rs:14`: *"`SetSelection`
  config mutation needed afterwards, the framework auto-prunes..."*) — no live code path uses the old
  mutation.
- **TS `selectedIds`/`highlightedIds`/`hoverAction`/`unhoverAction`** — the wire-schema types that the
  master doc specifically calls out (`UiTreeNode`/`UiTreeItemNode` in
  `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:2168-2266`) were correctly cut: no
  `selected_ids`/`highlighted_ids`/`selection_change` fields remain, replaced by
  `interaction_domain: Option<String>`, with an explicit doc comment: *"Replaces the deleted per-app
  `selected_ids`/`highlighted_ids`/`selection_change` wire surface."* The 256 TS-side grep hits are
  almost entirely (a) the generic, reusable `🪵️Tree` React component's own controlled-prop API (used by
  non-interaction-domain consumers like `📁️VirtualFileSystem`), or (b) an unrelated `hoverAction`/
  `hoverArgs` field on a generic clickable-item spec in `🔺️mesh/🟦️component.ts:43` (icon-catalog rows
  with a hover-preview action, e.g. `"hoverSuggestion"`/`"hoverFlag"` — nothing to do with selection
  domains).

**Borderline, worth a second look but not counted as a bug**: `🪵️Tree/🟦️component.tsx:966,984` —
`normalizeTreeSelectedIds`/`getTreeNextSelectionState`, which the master doc says should be *deleted*
("Tree... deletes `normalizeTreeSelectedIds`/`getTreeNextSelectionState` and imports the interaction
TS"), are still present as named exports. In practice they are now thin wrappers that fully delegate to
`validateState`/`nextSelection` from `🕹️interaction` (no independent logic, no drift risk) — the
in-file doc comment explains the deviation was deliberate, keeping the two call shapes because
"`📁️VirtualFileSystem`/tests already call it this shape." This is a real (if minor, low-risk) deviation
from the "no pragmatism, no compatibility layers" instruction, since the stated reason to keep the old
name is caller convenience rather than necessity — flagged for awareness, not listed as a residue bug
above because it doesn't retain any of the old *state*, only the old *function name*.

- **`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs:328`** and its TS
  counterpart `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts:40` — the CAD
  artifact-DSL `SelectionSpec` (`accept`/`multiple`/`prompt` for engagement-step target filtering), per
  the task's own instruction, confirmed unrelated to the framework mechanism and correctly excluded.

---

## 3. Single-definition check (Rust + TS)

Excluding the CAD `SelectionSpec` (confirmed unrelated, §2):

| Type | Rust definitions | TS definitions |
|---|---|---|
| `InteractionState` | 1 (`📡️spr/📡️wire/🦀️component.rs:1070`) | 1 (`🕹️interaction/🟦️component.ts:92`) |
| `PresenceInteraction` | 1 (`:1373`) | 1 (`🕹️interaction/🧬️schema/🟦️component.ts:37`) |
| `DomainSelection` | 1 (`:1047`) | 1 real (`🕹️interaction/🟦️component.ts:79`) + 1 typegen re-export alias (`🛂️manifest/🟦️component.ts:540 = GeneratedDomainSelection`) |
| `DomainHover` | 1 (`:1060`) | 1 (`🕹️interaction/🟦️component.ts:86`) |
| `MergeMode` | 1 (`:1015`) | 1 real (`🕹️interaction/🟦️component.ts:65`) + 1 typegen alias (`manifest.ts:538`) |
| `SelectionSpec` | 1 (`:962`) + 1 excluded CAD | 1 real (`🕹️interaction/🟦️component.ts:49`) + 1 typegen alias (`manifest.ts:535`) + 1 excluded CAD (`spatial-kernel/geometry/🟦️component.ts:40`) |
| `next_selection`/`next_hover`/`validate_state` | 1 each, all in `📡️spr/📡️wire/🦀️component.rs` (`:1190,~1250,1330`) | 1 each, `🕹️interaction/🟦️component.ts` |

All single-definition, confirmed. Note the Rust definitions live in `os_spr::wire`
(`semio-framework-os-kernel`), not in the `🧰️framework/🔨️modules/🕹️interaction` module the master doc
originally proposed — `🕹️interaction/🦀️component.rs:4-16` documents this as a deliberate crate-layering
fix (avoids a `semio-framework → os-kernel → semio-framework` cargo cycle) and re-exports everything
`pub use protocol::{...}` so call sites are unaffected. This is a legitimate, self-documented
architecture deviation, not a duplication bug — flagged here only because it means "the interaction
module" as named in the design doc is a thin re-export shell, with the real logic one layer down in the
OS product tree rather than in the domain-neutral framework tree (worth confirming with the team whether
that's the intended final layering for a domain-neutral mechanism, per CLAUDE.md's "domain-neutral
framework, domain-specific extensions" rule — this crate is OS-product-scoped by necessity of the orphan
rule, not domain-neutral).

The TS manifest re-exports (`manifest.ts:535,538,540`) are typegen aliases of the same generated type,
not hand-duplicated definitions — same pattern as every other manifest-mirrored type in that file.

---

## 4. Migrated-crate interaction-domain coverage (17 + infinite)

All 17 W4-scoped crates declare at least one `.interaction(...)` domain — none silently dropped
selection without replacement:

`✒️writer`(1) · `🌀️procedural`(6) · `🌍️gis`(2) · `🎥️shooting`(1) · `🏭️process`(1) · `💠️lowpoly`(1) ·
`📏️layout`(1) · `📐️cad`(1) · `📸️remodel`(1) · `🔱️trinity`(2, jack+rewrite) · `🖍️draw`(1) ·
`🖨️raster`(1) · `🗒️note`(1) · `🧩️puzzle`(3, 3d/5d/2d) · `🧱️block`(3, 3d/5d/2d) · `🪐️space`(2) ·
`🪵️sourcing`(2) — counts are `.interaction(` call-site counts per crate, confirmed by grep, spot-checked
above for `cad`/`lowpoly`/`puzzle`.

**`♾️infinite` declares zero** — see §1b. This is the one crate in the nominal 18-crate scope that fails
this check outright.

The other 14 plugin crates (`➗️mathematical`, `🎪️demonstrator`, `🏗️fem`, `📕️norm`, `🗄️stdio`,
`🔋️energy`, plus the 10 in §1a) were never in the W4 scope and show no `.interaction()` calls; the first
six of those also show zero `SetSelection`/config-field hits, i.e. no evidence they ever had selectable
state to begin with (not independently verified further — out of this sweep's budget, flagged as an
assumption, not confirmed).

---

## 5. Framework-verb shadowing check

No migrated app re-declares the six injected verbs (`interactionSelect`, `interactionHover`,
`clearSelection`, `selectAll`, `setSelectionMode`, `setInteractionGranularity`) as its own command-enum
variant. Every occurrence of these strings in the 17 migrated crates is either (a) the app *dispatching*
the framework-injected action (`app.handle_action("interactionSelect", ...)`,
`dispatch(app, "interactionSelect", ...)`), which is the correct consumption pattern, or (b) referencing
the action id in a menu/keybinding builder that points at the framework's own catalog entry.

`🌊️flow` and `✒️writer` also reference `"selectAll"`/`"clearSelection"` as action ids, but `flow`
declares its *own* command-enum arms for them (`"selectAll" as "select-all" => select_all::SelectAll`,
`"clearSelection" as "clear-selection" => clear_selection::ClearSelection`,
`🌊️flow/🦀️component.rs:159,172`) — since `flow` has no `.interaction()` domain, the framework never
injects these actions for it, so there is no live collision, but this is one more concrete symptom of
§1a: `flow` still owns hand-rolled `SelectAll`/`ClearSelection` commands that the framework verb of the
same name was built to replace. `writer` only *references* the framework's own injected `"selectAll"` in
a menu item — no shadowing.

---

## 6. Compatibility-shim keyword sweep (`git status --porcelain` touched set)

Grepped all touched `.rs`/`.ts`/`.tsx` files (791 total, repo-wide — includes unrelated concurrent work)
for `#[deprecated]`, `\blegacy\b`, `\bcompat\b`, `\bbackwards\b`, `\bfallback\b`:

- `#[deprecated]`: 0 hits anywhere.
- `backwards`: all hits are the standard `MutationDiff::backwards()` inverse-mutation method
  (pre-existing store convention, unrelated to compatibility).
- `legacy`: all hits belong to the unrelated, pre-existing `26/08/11/CLEAN-ARCHITECTURE-LAYERING-
  ENFORCEMENT` media-format-enum retirement, in files that happen to also carry interaction-mechanism
  edits. None reference the old selection/hover mechanism.
- `compat`: unrelated (`kindCompatibility` puzzle-fixture metadata, WebCodecs profile/compat/level
  bytes, forward-compat metadata comments on unrelated store fields).
- `fallback`: none reference keeping the old hover/selection API alive; all are ordinary
  default-value/UX fallback logic (e.g. `mesh_selection_ids_typed(ids, fallback)` in `procedural`'s
  gumball commands, which falls back to the *current framework* selection, not an old field).

**No explicit, keyword-labeled compatibility shim was introduced.** The one real dual-path found in this
sweep — `♾️infinite`/`🌍️world`'s live `worldSelect`/`worldHover` handling running alongside dead
`interactionSelect`/`interactionHover` handling (§1b) — doesn't use any of these keywords in its
comments, so it would not have surfaced from a keyword grep alone; it was found by tracing actual
dispatch reachability.

---

## Summary for the ticket

- **Confirmed bug (large, cross-cutting)**: `♾️infinite`'s `"world"` interaction domain is unreachable
  dead code; every 3D-world app still runs pre-migration `worldSelect`/`worldPick`/`worldHover`/
  `setSelection`. Fix: wire `.interaction(world_interaction_definition())` +
  `.window_kind_interactions(...)` onto every app that mounts a `World3dHost` surface, then delete the
  old `worldSelect`/`worldHover`/`setSelection` arms in `🌍️world/🦀️component.rs` and repoint
  `World3dHost`'s `handleInstancePointerDown`/marquee-finalize dispatches at `interactionSelect`/
  `interactionHover`.
- **Confirmed bug (10 crates)**: `flow`, `dag`, `reasoning`(`wires`), `sequence`, `vcs`, `imperative`,
  `forms`, `architect`, `playbook`, `animate`(`present`) never received the W4 migration at all — full
  duplicate config/presence/command state survives per §1a. Four of these (`flow`, `dag`, `playbook`,
  `sequence`) are explicitly named in the ticket's own problem statement.
- Everything inside the 17-crate W4 scope is clean; no silent domain-dropping, no verb-shadowing, no
  keyword-labeled compat shim, exactly one definition of every core type.
- One low-risk, self-documented deviation worth a second look: `🪵️Tree`'s
  `normalizeTreeSelectedIds`/`getTreeNextSelectionState` wrapper functions were kept (fully delegating
  to the new machine) instead of deleted, for caller convenience.
- One architecture note, not a bug: the interaction state machine's canonical Rust home is
  `semio-framework-os-kernel`'s `📡️spr/📡️wire` module, not the domain-neutral `🧰️framework/🔨️modules/
  🕹️interaction` module the design doc originally specified — a documented orphan-rule workaround.
