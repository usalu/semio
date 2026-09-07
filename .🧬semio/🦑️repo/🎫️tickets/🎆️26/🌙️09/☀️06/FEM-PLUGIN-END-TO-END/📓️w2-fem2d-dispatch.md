# 🏗️ W2 — fem2d descriptors, dispatch and boot

Scope: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/**` (editor, viewer, artifact schema).
**NOTHING WAS COMPILED.** No `cargo`, `bun`, `nx`, or any build/test command was run (host swap exhausted per the
coordinator's instruction). Every claim below is static: read of the framework source with file:line, grep, and a
brace/paren-balance pass over each edited file. The coordinator must compile.

---

## 1. Deliverable A — the `DESCRIPTORS` / `descriptor()` compile blocker

### Contradiction resolved against the trait itself

`📓️explore-fem2d-editor.md` §6a said the two impls are missing required associated items; `📓️explore-history-drift-gates.md`
§3.f said the class was "present in 3 files … not independently verified — flag as unverified, not a known defect" and
its §2 table row said `Descriptor = OK`. Read the trait directly:

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:145-153`

```rust
pub trait Mutation<P>: Clone + crate::value::ToValue + crate::value::FromValue {
    type Diff: MutationDiff<P>;
    const DESCRIPTORS: &'static [MutationLeafDescriptor];   // line 148 — NO default
    fn descriptor(&self) -> &'static MutationLeafDescriptor; // line 151 — NO default
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Vec<Self>;
    // everything below line 155 has a default body
}
```

Both items are required. `📓️explore-fem2d-editor.md` is correct; the drift-gates note's "Descriptor = OK" row was a
stale/derived table row, and its own §3.f already labelled the class unverified. **The defect was real** (E0046 on both
`Fem2dConfigMutation` and `Fem2dPresenceMutation`) and is now fixed.

### Fix

| File | Added |
|---|---|
| `✏️editor/🎚️config/🦀️.rs` | `DESCRIPTORS` with **4** leaves + a 4-arm `descriptor()` |
| `✏️editor/👥️presence/🦀️.rs` | `DESCRIPTORS` with **1** leaf + a 1-arm `descriptor()` |

Shape copied from block2d (`🧱️block/…/✳️any/✏️editor/{🎚️config,👥️presence}/🦀️.rs`), but the **`semantic_kind`/`owner`
spellings follow fem3d's already-landed W3 fix** (`🏗️fem/🗿️artifacts/🧊️3d/…/✏️editor/🎚️config/🦀️.rs:179-183`,
`👥️presence/🦀️.rs:57-59`) so the two fem artifacts read identically:

| Enum | Variant | `semantic_kind` | `owner` leaf | `aggregate_variant` |
|---|---|---|---|---|
| `Fem2dConfigMutation` | `Snapshot` | `set-snapshot` | `🎚️config/🟤️set-snapshot` | `Snapshot` |
| | `SetResultDisplay` | `set-result-display` | `🎚️config/👁️set-result-display` | `SetResultDisplay` |
| | `SetCamera` | `set-camera` | `🎚️config/🎥️set-camera` | `SetCamera` |
| | `SetLocale` | `set-locale` | `🎚️config/🗣️set-locale` | `SetLocale` |
| `Fem2dPresenceMutation` | `Noop` | `presence-noop` | `👥️presence/🚫️presence-noop` | `Noop` |

Every other field is the block2d/fem3d constant set (`schema_version: 1`, `payload_schema: "🔣️.schema.json"`,
`text_opcode: None`, `binary_tag: None`, `ExplicitMutation`, `Detect`, `[Applied]`, `Atomic`,
`[Rust, JsonSchema]`).

New tests: `every_config_mutation_variant_has_its_own_descriptor` (config) and
`the_presence_mutation_variant_has_its_own_descriptor` (presence, in a new `🧪️Tests` region).

### Open risk (documented, not fixed)

These `owner` paths name directories that **do not exist on disk**, so `protocol::validate_mutation_leaf_descriptor`
would reject them: `mutation_leaf_descriptor_owner` (`🎮️mutation/🦀️.rs:475-487`) requires the owner string to contain
the marker `"/🧬️mutations/"` (`:472`). Config/presence mutation enums have no `🧬️mutations` leaf directory anywhere in
the repo, so **block2d, block3d, puzzle3d, fem3d and now fem2d all carry owner paths that fail that predicate**. Nothing
calls the validator on these rosters (`validate_mutation_leaf_descriptor_roster` is invoked only per-artifact, and
`::DESCRIPTORS` is otherwise read only by tests — grep across `🧰️framework/**`), so this is metadata debt, not a
runtime fault. fem3d's own W3 comment calls it "⚠️ PROVISIONAL". Flagging it as a cross-plugin follow-up.

### `factory_type` — already present

`bounded_first_step_tool_proofs!` (`🔌️plugin/🦀️.rs:12704-12731`) makes `factory_type:` optional and only that arm
calls `.with_factory_type::<Owner, F>()`. fem2d **already declared it** (`✏️editor/🦀️.rs`, pre-existing line 453), so the
`interactive-job.missing-owned-reducer` "bare bounded factory" failure mode was NOT fem2d's problem. Left as-is,
tools list extended (below).

---

## 2. Deliverable B — dispatch: 19/19 `Migrated`

### Two blockers the exploration report did not name

Migrating the 16 actions to `Migrated` was **not** a one-line classification flip. Two additional, independently
verified gaps had to be closed first:

**(a) No document-lane publication authority.** `ArtifactToolFactoryRegistry::register`
(`🔌️plugin/🦀️.rs:12906-12911`) demands an exact per-tool publication-lane contract, and the app build
(`:19563-19581`) marks a declared lane *unsupported* when the matching one-item preparation factory is `None`;
`qualified_tool_proof` (`:19405-19407`) then rejects every dispatch of that verb with
`interactive-job.publication-authority-missing`. fem2d declared **only**
`build_config_store_one_item_preparation_factory`. Declaring the 15 structural editors on the `Artifact` lane without a
document-lane factory would therefore have made them dead in a *different* way. Added
`Fem2dArtifactPreparationFactory` + `Fem2dArtifactPreparation` (new `📬️ArtifactStorePreparation` region, modelled on
block2d's `Block2dStorePreparationFactory`, `🧱️block/…/✏️editor/🦀️.rs:260-390`) and
`build_artifact_store_one_item_preparation_factory`.

**(b) No `command_from_action` bridge.** `ArtifactEditor::command_from_action`
(`🔌️plugin/🦀️.rs:26831-26838`) defaults to rejecting **every** app action with `app.command.unsupported`, and
`command_from_intent` (`:26843`) funnels the React/wgpu `{action, args}` wire through it. fem2d had **no override**
(grep: zero hits for `command_from_action` under `🏗️fem/` before this change), so no fem2d action could ever reach
`dispatch` from a shell button — including the navbar example switcher. Added a 19-arm bridge, plus
`fem2d_dof`/`fem2d_dofs` helpers (new `🔖️ActionArgHelpers` region) for the typed `FemDof` args.

Also: `Fem2dConfigPreparation::advance` and `fem2d_config_publication_bytes` previously rejected
`Fem2dConfigMutation::Snapshot` with `fem2d-config-unsupported-mutation` — which is exactly the mutation
`setActiveExample` emits. Both now handle `Snapshot` (and their `_ =>` catch-alls are gone, so the match is exhaustive
over the 4 variants).

### Per-action classification, before → after

| # | Action | Before | After | Lane | Reducer (unchanged `🎮️commands/*` handler) |
|---|---|---|---|---|---|
| 1 | `addNode` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `⚪️add-node::handle` |
| 2 | `addBar` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `➖️add-bar::handle` |
| 3 | `addBeam` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🌉️add-beam::handle` |
| 4 | `addMaterial` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🧱️add-material::handle` |
| 5 | `addSection` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `📐️add-section::handle` |
| 6 | `addSupport` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🛡️add-support::handle` |
| 7 | `addNodalLoad` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `📍️add-nodal-load::handle` |
| 8 | `addMemberUdl` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `📏️add-member-udl::handle` |
| 9 | `addAreaLoad` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🏋️add-area-load::handle` |
| 10 | `addRegion` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🗺️add-region::handle` |
| 11 | `addLoadCase` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `📋️add-load-case::handle` |
| 12 | `addCombination` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🔗️add-combination::handle` |
| 13 | `setSelfWeight` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `⚖️set-self-weight::handle` |
| 14 | `setAnalysisSettings` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🧮️set-analysis-settings::handle` |
| 15 | `removeSelection` | BatchOnlyPendingRewrite | **Migrated** | Artifact | `🗂️remove-selection::handle` |
| 16 | `setActiveExample` | BatchOnlyPendingRewrite | **Migrated** | **Config** | `📚️set-active-example::handle` |
| 17 | `setCamera` | Migrated | Migrated | Config | `🎥️set-camera::handle` |
| 18 | `setResultDisplay` | Migrated | Migrated | Config | `👁️set-result-display::handle` |
| 19 | `setLocale` | Migrated | Migrated | Config | `🗣️set-locale::handle` |

**No action is left un-retained.** Grep for `BatchOnlyPendingRewrite` under `🗿️artifacts/◻️2d/` returns 2 hits, both
inside doc comments (`✏️editor/🦀️.rs:954` and `:1277`); zero classification call sites.

`setActiveExample` is on the **Config** lane, not Artifact: its handler emits no `artifact_mutations` at all — it
replaces the document with a non-history `Effect::LoadDocument` (`reset_document_effect`) and publishes only
`Fem2dConfigMutation::Snapshot`. Effects are not a store lane in the emit-lane check (`🔌️plugin/🦀️.rs:22945-22951`),
so `[Config]` is the exact contract. block2d declares its own `setActiveExample` as `Artifact`, which is wrong for
fem2d's handler shape and would trip the "emitted a store lane absent from its publication contract" fault.

### Factory shape (`✏️editor/🦀️.rs`, `🧵️RetainedCommands` region)

```
FEM2D_RETAINED_TOOL_IDS        = all 19 ids (was 3)
FEM2D_RETAINED_PAYLOAD_SCHEMA  = "fem.2d.tool-command.v1"           (unchanged)
FEM2D_RETAINED_RAW_BYTES       = 65_536      (was 8_192)
FEM2D_RETAINED_WORK_ITEMS      = 4_096       (was 1)
FEM2D_MAXIMUM_DOCUMENT_ITEMS   = 4_096       (new)
FEM2D_PUBLICATION_CONTRACTS    = 19 rows, 15 × [Artifact] + 4 × [Config]   (new const)
fem2d_retained_contract()      = bounded_first_step(65_536, 4_096, 1, 262_144, 7_500)
fem2d_document_items(snapshot) = 1 + nodes+elements+regions+materials+sections+supports+load_cases+combinations
fem2d_retained_extent(..)      = None if the id is not a route or the document exceeds the cap, else Some(1)
```

`ArtifactOwnedToolJobFactory::PUBLICATION_CONTRACTS` now aliases `FEM2D_PUBLICATION_CONTRACTS`, and the
`bounded_first_step_tool_proofs!` `contract:` literal was updated to match `fem2d_retained_contract()` byte for byte
(the two must agree). `fem2d_retained_reduce` is unchanged apart from the route table it guards against.

`extent` returning `None` is the framework's rejection path — `🧵️retained-command/🦀️.rs:470-472` faults with
"retained command exceeds semantic work capacity" — so an oversized document is refused before any reduction, matching
`Fem2dArtifactPreparationFactory::begin`'s identical `FEM2D_MAXIMUM_DOCUMENT_ITEMS` admission.

### Manifest arg declarations added

`addBar`, `addBeam` (start/end/materialId/sectionId), `addMaterial` (name/e), `addSection` (name/area/iy),
`addSupport` (nodeId + a text `fixed`, default `"tx,ty"`), `addCombination` (name), and `addNodalLoad`/`addMemberUdl`
gained their non-`caseId` args (they previously declared `caseId` only). Without these the staged palette form could
only ever send empty strings into a now-live route.

### Actions that cannot be fully staged from the palette (documented, still Migrated)

- **`addCombination.terms`** (`Vec<FemCombinationTerm>`) — no `ActionArgDef` control maps to a typed struct list
  (`🛂️manifest/🦀️.rs:331-380`: text/number/slider/toggle/select/vec3/artifact_kind/surface_app only). The action is
  retained and creates the named combination with **no terms**; term rows need a follow-up editor surface.
- **`removeSelection.ids`** (`Vec<String>`) — same reason; declared with no args. The bridge reads an `ids` **array**
  if a caller (canvas selection, not the palette) sends one, else an empty list, which the handler already treats as a
  no-op.

---

## 3. Deliverable C — boot snapshot and example switching

### Boot

`🧬️schema/🦀️.rs` gained `default_fem2d_snapshot()` next to `empty_fem2d_snapshot()` (block2d precedent:
`🧱️block/…/✳️any/🧬️schema/🦀️.rs:259-261`): parse `crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT`, fall back to the
empty document on a parse error. Both surfaces now boot on it:

- `Fem2dPlayApp::initial_snapshot()` — was `empty_fem2d_snapshot()`, now `default_fem2d_snapshot()`.
- `Fem2dViewer::initial_snapshot()` — was an inline duplicate of the same parse, now calls the shared function.

`[DEBUG]` console lines (the crate's logging idiom is `eprintln!("[DEBUG] …")`, per
`🔱️trinity/…/✏️editor/🌍️world/🦀️.rs:403,412,418,572`; the fem crate had no runtime logging at all before):

```
[DEBUG] fem2d boot snapshot: loaded the bundled example — nodes=… elements=… regions=… materials=… sections=… supports=… loadCases=… combinations=…
[DEBUG] fem2d boot snapshot: the bundled example failed to parse, falling back to the empty document — <TextError>
[DEBUG] fem2d setActiveExample '<id>': loading nodes=… elements=… regions=… loadCases=…
```

The coordinator can prove non-empty first paint from the first line alone.

### Example id drift — a second dead-switcher bug

The shell's navbar switcher dispatches the example's **own manifest id** verbatim:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:6439`
→ `onAction({ action: "setActiveExample", args: { exampleId } })`, where `exampleId` comes from
`PluginManifest.examples`. fem2d's example leaf declares `ID = "demo"`
(`📚️examples/🎬️demo/🦀️.rs:5`) and the subset root already publishes it
(`🪆️subsets/🌐️any/🦀️.rs`, `examples()` → `crate::artifacts::fem2d::examples::demo::source()`), but:

- the manifest select option was `"default"`, and
- the handler only recognised the literal `"default"`.

So even with `setActiveExample` migrated, the switcher would have dispatched `"demo"` and been silently reset to an
**empty** document. Both are now `crate::artifacts::fem2d::examples::demo::ID`, with a new test
(`the_declared_example_option_is_the_bundled_example_id_2d`) pinning option id == handler id == `ExampleSource::id()`.

The wire-baseline row in `every_command_keeps_its_pre_migration_bytes` still encodes the string `"default"` — that is
a free-text `String` payload and its bytes are unchanged; only the *accepted* id moved.

### Example list the switcher reads

Already correct at the live path (`SubsetDeclaration.examples` in the subset root — W4's file, untouched). The
`EditorBuilder` has no `.example(...)`; the stale `🚧️ SDK GAP` doc comment on `create_fem2d_app` claiming the
registration was "dropped, not ported" was rewritten to point at the subset-root path. The test-only
`fem2d_app_manifest_for_testkit` wrapper now carries the same single example instead of `Vec::new()`.

---

## 4. Bonus fix: 75 stale `resolve_ready(…)` wrappers (de-async fallout)

`📓️explore-fem2d-editor.md` §6 checked for `async fn diff/inverse/handle/print_dsl/parse_dsl` **declarations** and
found fem clean. It did not check **call sites**. `semio_framework_plugin::resolve_ready` is the io module's
`resolve_ready<F: std::future::Future>` (`🧰️framework/🔨️modules/🚪️io/🦀️.rs:891`) and requires a real future, but the
fem2d editor subtree wrapped it around functions that are **sync** on the current traits:

| Call | Definition | Verdict |
|---|---|---|
| `HistoryView::empty()` | `🔌️plugin/🦀️.rs:10128` `pub fn` | stale wrapper |
| `ArtifactView::new(..)` | `:8205` `pub fn` | stale wrapper |
| `VcsArtifactApp::snapshot()` | `:20559` `pub fn` | stale wrapper |
| `ArtifactEditor::io()` | `:26921` `fn` | stale wrapper |
| `ArtifactEditor::config_spec()` | `:26857` `fn` | stale wrapper |
| `ArtifactEditor::export_media` / `import_media` | `:26931` / `:26946` `fn` | stale wrapper |
| `ArtifactViewer::initial_snapshot()` | `:27079` `fn` | stale wrapper |
| `testkit::new_app` / `new_app_with_registry` / `paired_apps` / `assert_undo_redo_round_trip` | `:6791/6797/6827/6855` `pub async fn` | **kept** |
| `PluginApp::render` / `handle_action` / `load_document_pack` / `export_media(&mut self)` | `:11960/11814/11952/11985` `async fn` | **kept** |
| `store::print_document_spr` | `🏪️store/🦀️.rs:10971` `pub async fn` | **kept** |

block2d — the already-fixed reference — calls all of the first group directly
(`🧱️block/…/✏️editor/🦀️.rs:805-806,878`), confirming the direction. 75 stale wrappers were stripped across the fem2d
editor subtree + viewer (31 in `✏️editor/🦀️.rs` and `👁️viewer/🦀️.rs`, 44 in `🎮️commands/*` and
`🎭️modes/✏️edit/🪟️windows/*`). All are `#[cfg(test)]` code, so this does not affect `cargo check` (DoD 1) but does
affect `cargo test` (DoD 2). **fem3d and the other plugins were NOT swept — check them.**

---

## 5. Files changed

All under `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/`:

| File | Change |
|---|---|
| `✏️editor/🎚️config/🦀️.rs` | `DESCRIPTORS` + `descriptor()` (4 leaves), descriptor test |
| `✏️editor/👥️presence/🦀️.rs` | `DESCRIPTORS` + `descriptor()` (1 leaf), new `🧪️Tests` region |
| `✏️editor/🦀️.rs` | route table → 19, publication contracts, doc-lane preparation factory + laws test, `Snapshot` arms in the config preparation, `command_from_action`, dof helpers, boot snapshot, 19 × `Migrated`, new arg declarations, example option id, 3 new route/bridge tests + 1 boot test, testkit example list, doc-comment rewrite, `resolve_ready` sweep |
| `✏️editor/🎮️commands/📚️set-active-example/🦀️.rs` | accept `demo::ID`, `[DEBUG]` line, test rename + new option-id test, `resolve_ready` sweep |
| `🧬️schema/🦀️.rs` | `default_fem2d_snapshot()` with `[DEBUG]` logging |
| `👁️viewer/🦀️.rs` | boot via `default_fem2d_snapshot()`, `resolve_ready` sweep |
| `✏️editor/🎮️commands/{⚪️add-node,📍️add-nodal-load,🗂️remove-selection,🧮️set-analysis-settings,🎥️set-camera,👁️set-result-display,🗣️set-locale}/🦀️.rs` | `resolve_ready` sweep only |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/{🧱️model,📊️results}/🦀️.rs` | `resolve_ready` sweep only |

Untouched by design: `🚪️io/**` and the subset root's `io:` field (W4), `🗿️artifacts/🧊️3d/**` (W3),
`◻️2d/…/📈️analysis/🧪️tests/` (W5), `🧪️tests/`/`🔮️oracle/` case-name literals (W1), the crate entry (W1).

---

## 6. Verification performed (no compiler)

1. **Trait contract** read directly at `🎮️mutation/🦀️.rs:145-153` — both associated items required, no defaults.
2. **Variant ↔ descriptor coverage**: `Fem2dConfigMutation` has 4 variants / 4 descriptors / 4 match arms;
   `Fem2dPresenceMutation` 1 / 1 / 1. Asserted at runtime by the two new tests as well.
3. **Zero `BatchOnlyPendingRewrite`** under `🗿️artifacts/◻️2d/` outside doc comments (grep, 2 doc hits listed above).
4. **Route-set equality**: `FEM2D_RETAINED_TOOL_IDS` (19) == `app_commands!` rows (19) == `bounded_first_step_tool_proofs!`
   tools (19) == `FEM2D_PUBLICATION_CONTRACTS` (19) == `.action_interactive_job(_, Migrated)` calls (19), checked by hand
   and pinned by the new `retained_routes_cover_every_command_exactly_once`.
5. **Every referenced symbol grepped to a definition**, notably:
   `store::ARTIFACT_STORE_ONE_ITEM_{ID_BYTES,MAXIMUM_BYTES}` (`🏪️store/🦀️.rs:13023,13021`),
   `ActionArgDef::{text,number,select,toggle,required,default_value,control}` (`🛂️manifest/🦀️.rs:341-402,405`),
   `ActionArgOption { value, label }` (`:231-239` — the field is `value`, **not** `id`),
   `ExampleSource::id()` (`🔌️plugin/🦀️.rs:7993` — the field is private, accessor only),
   `semio_framework::ActionArgControl::Select` (used at `🔌️plugin/🦀️.rs:656`),
   `ActionDefinition.semantics.execution.interactive_job` (the classification's actual home, written by
   `action_interactive_job` at `:5227` — `ActionDefinition` has **no** `interactive_job` field, `🛂️manifest/🦀️.rs:978-1002`),
   `DslValue::{as_str,as_f64,as_bool,as_array,get,float,Object}` (`🌱️value/🦀️.rs:134-196`),
   `crate::artifacts::fem2d::examples::demo::{ID,label,source}` (mounted at crate entry `🦀️.rs:714-720`),
   every command payload struct field name/type (all 17 read directly).
6. **Brace/paren/bracket balance** = 0 for every file I edited (script over the whole subset tree). Two imbalances
   reported by that pass are **not mine**: `🚪️io/📥️import/…/🔣️json/🔖️rfc8259/✳️any/🦀️.rs` (W4, in flight) and
   `✏️editor/🧵️session/🦀️.rs` (`git status` shows it unmodified — pre-existing, parens inside strings/comments).
7. **`resolve_ready` classification**: each of the 10 call targets checked against its `fn`/`async fn` declaration
   (table in §4).

## 7. Open risks for the coordinator

1. **Nothing compiled.** Expect first-compile fallout; the highest-risk new code is
   `Fem2dArtifactPreparation::advance` (block2d-shaped but retyped) and the 19-arm `command_from_action`.
2. **Descriptor `owner` paths fail `validate_mutation_leaf_descriptor`** (no `/🧬️mutations/` marker) — shared with
   block2d/block3d/puzzle3d/fem3d; nothing invokes the validator on config/presence rosters today. Cross-plugin
   follow-up, not a fem2d regression.
3. **`fem2d_app()` uses the registryless `testkit::new_app`.** Per the repo's own experience, a plugin carrying
   `bounded_first_step_tool_proofs!` must use `new_app_with_registry` and bind an instance id or dispatch faults with a
   catalog-authority error. fem2d already had the proofs macro before this change, so the exposure is pre-existing —
   but it now covers 19 routes instead of 3, so any such failure will look new. `fem2d_app_with_registry()` already
   exists next to it if the suite needs switching over.
4. **fem3d was not swept for the stale `resolve_ready` class** (§4) and almost certainly carries it.
5. **`addCombination` terms and `removeSelection` ids** have no staged palette form (§2) — retained but only partially
   drivable from the action pane.
6. `FEM2D_RETAINED_RAW_BYTES`/`WORK_ITEMS` were raised 8× / 4096× to block2d's numbers; if the coordinator wants
   fem2d's wire budget tighter, `fem2d_retained_contract()` and the `bounded_first_step_tool_proofs!` `contract:`
   literal must be changed **together**.
