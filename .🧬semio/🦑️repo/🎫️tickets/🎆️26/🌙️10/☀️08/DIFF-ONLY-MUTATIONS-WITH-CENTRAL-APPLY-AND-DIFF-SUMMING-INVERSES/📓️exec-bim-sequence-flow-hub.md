# 📓️ exec-bim-sequence-flow-hub — 🏙️bim, 🎬️sequence, 🌊️flow plugin facets and 🌎️hub space engine

Status: WRITTEN, GATE-GREEN, BUILD NOT YET RUN (foundation was RED the whole time; see "Test status"). Scope gate (all rules R8-R16,
rule 1 outcomes, rule 2 message codes, R15 sum-law, `apply_to`) over the four scope paths: 0 breaches (was 11 R-rule breaches + 74 rule-1 breaches).
Scratch checker (not in repo): `scratchpad/scope-gate.ts` imports `policy*` from `📜️script.ts` and filters to the four scope paths.

## Audit first (before changes)

### 🏙️bim `🗿️artifacts/🏢️model` (88 kinds, `ModelDiff` = `Option<KeyedDelta<T, Patch>>` per id-keyed `BTreeMap` collection + `ProjectPatch`)
- The mutation leaves were already authored diff-only (BIM-PLUGIN ticket): every `🔺️diff` builds `Entry::{Created,Deleted,Replaced,Patched}` rows from payload + base reads, every
  `↩️inverse` builds concrete mutations from payload + base (no `Vec::new()` for state-changing kinds, no `diff(`/`*_inverse(` helper, no `between(`, no `.apply(`, no `&mut`), cascades are
  shared concrete code (`🌊️cascade`), `KeyedDelta::absorb` coalesces per key (created∘deleted cancels, deleted∘created replaces, patch∘patch merges), `ModelDiff::inverse` is concrete.
- 205 applied fixture cases, each with a `assert_mutation_inverse_sum_law` test; 335 rejected cases (no law needed: empty diff). Every one of the 88 kinds has >= 1 applied case.
- No ordered collection exists at collection level (all `BTreeMap<String, _>`), so a "middle row" case is not applicable (storeys order by `level`, a field). Intra-record vectors (layers, outline) are replaced as absolute field values.
- Gate rule 1 (74 files) was a FALSE POSITIVE of the textual gate: the diffs report through the typed `protocol::OutcomeCode::X` (`MutationOutcome::refuse`), which spells the frozen codes
  only inside the enum, so `content.includes("mutation.…")` never matched. Fixed in the gate (`policyMutationOutcomeBreaches` also accepts `OutcomeCode::<Variant>`), not by gaming the diffs.
- Real breaches: `🖌️render/🪟️window-config` macro (R10 `between(` in the mutation diff, R14 `Snapshot` restore inverse); the editor window configs were additionally on the deleted
  `impl_whole_record_config!` (whole-state diff) and `BimWindowTransient` on the pre-sparse `transient_root!` form (no `diff:`/`fields:`), neither visible to the gate (compile errors).

### 🎬️sequence `🗿️artifacts/🎬️sequence`, 🌊️flow `🗿️artifacts/🌊️flow`
- Parent mutation vocabularies are uninhabited by design (content edits are child-lane leaves owned by stdio/semio): no leaf, no leaf test possible.
- `SequenceDiff`/`FlowDiff`: old `apply(&self, base)` signature (no `ApplyCapability`), no `DiffAlgebra`, whole-artifact `artifact` slot (`V1-SNAPSHOT-DIFF` whole-document replace) with `apply_to_artifact` and `diff_set_snapshot`.
- Script window transient (sequence): hand-written whole-state diff + `Snapshot` restore inverse (R13/R14). Flow presence: whole-state diff + `Snapshot` restore inverse (R13/R14), old `apply` signature.
- `FlowBuilderConstruction` (io) and three flow editor tests called `MutationDiff::apply` directly (L4).
- Main window configs of both plugins (sequence `📽️main`, flow `🌊️main`) were converted concurrently at 17:48-17:50 by another executor (per-field `Set…` via `config_record!`, fixtures,
  `addressed(view, base, config) -> Vec<_>`, editor callers); I only completed their window-ownership tests (see below).

### 🌎️hub space `⚙️engine/🪐️space`
- `🎚️config`: already converted (sparse `SpaceConfigDiff`, keyed camera rows, concrete `SetCamera`/`RemoveCamera` inverse) by the framework wave before my start; the gate lines in `gate-run-2.log` were stale.
  Remaining flaw: every `Set…` emitted its slot unconditionally (non-minimal diff); no law test.
- `👥️presence`: whole-state diff `SpacePresence: MutationDiff<SpacePresence>`, `Snapshot` restore inverse, old `apply` signature, no tests.

## Per kind: before -> after

| Facet | Before | After |
|---|---|---|
| bim `🖌️render/🪟️window-config` (5 window configs: viewer world/plan, editor section/world/plan) | `Snapshot { config }`, diff via `between(base, config)` or whole config; inverse `Snapshot { base }`; editor ones on deleted `impl_whole_record_config!` | one macro `bim_window_config!` (single arm, `diff:`/`fields:` required): `store::sparse_record_diff!` sparse diff, `ConfigRecord` mark, `Replace { config }` (a `replace-<entity>` kind per the wave-2 ruling: diff = `Diff::changing(base, config)` = only the differing slots; inverse = `Replace { base }`); `assert_window_config_laws` now always checks the `between` law |
| bim editor `🫧️transient` | `transient_root!` without `diff:`/`fields:` | `diff: BimWindowTransientDiff`, `fields: { engagement_input, pointer_generation, preview }`; law + sparse tests added |
| bim `ModelDiff` apply error code | `mutation.apply.duplicate-target` | `mutation.apply.duplicate-id` (design); test updated |
| bim TS twins (3 editor window configs) | `{kind:"snapshot"}` + `applyBim…Mutation` returning the parsed payload | `{kind:"replace"}`, `Bim…Diff = Partial<Bim…>`, `diffBim…Mutation(base, m)` (differing keys only), `applyBim…Mutation` through the diff |
| sequence `SequenceDiff` | `artifact` slot, `apply_to_artifact`, old apply, no algebra | `{ schema, content }` only; `apply(.., ApplyCapability)`, concrete `DiffAlgebra` (inverse = base values of the set slots, `between`, `is_empty`), slot-wise `absorb`; schema twins (`🔣️.json`, `🟦️.ts`, `🛰️.proto`, `🔗️.graphql`) rewritten to exactly this shape (stale step/edge delta types removed) |
| flow `FlowDiff` | same | same; `diff_set_snapshot`/`apply_to_artifact` and the `artifact` slot deleted; codec fixture 4 -> 3 cases; twins rewritten |
| sequence script `🫧️transient` | hand `Snapshot` mutation, `type Diff = Self` whole-state diff, hand owner | `transient_root!` (sparse `SequenceScriptWindowTransientDiff`, framework-owned `Snapshot { transient }` ephemeral-transfer exception) + `window_transient_owners!` (generates `register`/`from_snapshot`/`current`/`addressed`) |
| flow `👥️presence` | `FlowPresence: MutationDiff<FlowPresence>`, `Snapshot { presence }` | `sparse_record_diff!` `FlowPresenceDiff` + `field_set_mutations!` `FlowPresenceMutation::{SetPreviewOffNodeIds, SetCamera}` (JSON op wire `{"kind","value"}`), no whole-record variant |
| hub space `👥️presence` | `SpacePresence: MutationDiff<SpacePresence>`, `Snapshot { presence }` | `SpacePresenceDiff` (sparse slots + keyed camera rows) + hand `SpacePresenceMutation::{SetActiveNode, SetFocusedNode, SetCollapsed, SetPreviewOff, SetCamera, RemoveCamera}`, minimal diffs, `RemoveCamera` of an absent row = `mutation.target-missing` refusal, concrete inverse (`SetCamera` of an absent row inverts to `RemoveCamera`) |
| hub space `🎚️config` | unconditional slots, string code, no law test | minimal diffs (`(base != x).then(..)`), typed `OutcomeCode::TargetMissing`, law test over all 12 variants incl. a middle camera row |
| flow `🚪️io` `FlowBuilderConstruction`, flow editor tests (3) | `MutationDiff::apply(..)` | `protocol::apply_diff(..)` |
| flow/sequence main window ownership tests | `.diff().apply()`, forward-order inverse replay | `protocol::apply_diff`, inverse replayed `.rev()`, `assert_mutation_inverse_sum_law` per fixture row |
| gate `📜️script.ts` rule 1 | literal-only | also accepts `OutcomeCode::<Pascal>` |

## Files touched
- Gate: `📜️script.ts` (`policyMutationOutcomeBreaches`).
- bim (`✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`): `🖌️render/🪟️window-config/🦀️.rs`; `✏️editor/🧰️kit/🦀️.rs`; `✏️editor/🫧️transient/🦀️.rs` + `🧪️tests/🔬️unit/🦀️.rs`;
  `✏️editor/🎭️modes/✏️edit/🪟️windows/{📐️section,🧊️world,🗺️plan}/🎚️config/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs,🧬️schema/🟦️.ts}`; `👁️viewer/🎭️modes/👁️view/🪟️windows/{🧊️world,🗺️plan}/🎚️config/🧪️tests/🔬️unit/🦀️.rs`;
  `🧬️schema/🔺️diff/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`.
- sequence (`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/`): `🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🟦️.ts,🛰️.proto,🔗️.graphql,🧪️tests/🔬️unit/🦀️.rs}`;
  `✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🫧️transient/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`; `…/📽️main/🎚️config/🧪️tests/🔬️window/🦀️.rs`.
- flow (`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/`): `🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🟦️.ts,🛰️.proto,🔗️.graphql,🧪️tests/🔬️unit/🦀️.rs,🧫️fixtures/🔁️codec/🔣️.json}`; `✏️editor/👥️presence/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`;
  `🚪️io/🦀️.rs`; `✏️editor/🧪️tests/🔬️unit/🦀️.rs`; `✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs`; `…/🌊️main/🎚️config/🧪️tests/🔬️window/🦀️.rs`.
- hub space (`🌎️hub/🧩️compositions/🪐️space/`): `⚙️engine/🪐️space/👥️presence/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`; `⚙️engine/🪐️space/🎚️config/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`; `📦️packages/🦀️rust/Cargo.toml` (dev-dependency `semio-framework-os-kernel` with `protocol-laws` for the law tests).

## Wave 4 additions (translators T15, W11, BT1)

| Row | Before | After |
|---|---|---|
| flow T15 `set-active-example` | `flow_scene_replacement(composed, widgets, synapses, layout)` diffed the live content child against the example (`flow_content_leaves(base, next)`) into node/edge/param leaves | example switch = load: `set_active_example_edit(payload)` returns `Emit::effect(Effect::LoadDocument { pack, spr })` (`load_document_effect`, `set_active_example_document`); no mutation rows, no history row, no differencing; `flow_scene_replacement` deleted; the retained graph route calls the new signature; both tests rewritten (one `LoadDocument` of the demo graph / of every shipped example) |
| sequence W11 `replace_snapshot` | host method used by `move-step` and node-graph `move` rows as a mutate-then-diff vehicle | callers verified (2 production: `🪜️step` MoveStep, `🕸️node-graph` move): MoveStep emits ONE concrete `drag-nodes` leaf (offset between the published step position and the requested one, nothing when absent/unchanged); node-graph `move` records emit one `drag-nodes` per record (targets filtered to existing steps) ahead of the remaining rows; `SequenceHost::replace_snapshot` and its two tests deleted |
| bim BT1 inference cache | `ModelDiff::between(previous, snapshot)` per refresh | no `between`: the editor step records the concrete diffs of the mutations it emits (`inference::record_mutations`, sequential diffs summed with `absorb`); `ModelInferenceSession::refresh` trusts the pending sum only if `apply_diff(pending, previous) == snapshot` (then `touches()` selects the recomputed fields), otherwise (undo, peer edit, load, rejected emit) recomputes every field; 3 new tests (unexplained change, untrusted diff, summed sequence) and the two existing incremental tests now record their mutation |


## Wave 4b: mutate-then-diff translators deleted (sequence and flow)

Deleted: `sequence_content_leaves`, `sequence_scene_leaves`, `sequence_child_leaves_from_host_mutation`, `sequence_child_emit_from_host_mutation` (+ `SEQUENCE_DRAG_OFFSET_TOLERANCE`, `sequence_retained_{serial,next_id,is_control,default_slot,create_step,remove,delete_ids}`);
`flow_content_leaves`, `flow_node_leaves`, `flow_scene_publication`, `host_scene_edit`, `renamed_fixture`. No scene or content is differenced against another any more; every gesture emits the leaves of its own rule.

**Sequence** (`✏️editor/📐️edit-rules`, mounted as `crate::editor::sequence::edit_rules`): `SceneEdit` applies a gesture to a working copy of the scene and records the leaf of each step.

| Gesture | Leaves |
|---|---|
| add-step / add-step-to-slot / add-step-dropped / import media (2 sites) | one `insert-node` (fresh `step-N`; dropped steps join the picked expanded control's default slot) |
| remove-step / delete-selection | `remove-edge` per touching edge, then `remove-node` per step; the removal set is the transitive closure of the roots' control slots (`removal_closure`) |
| move-step | one relative `drag-nodes` (published position -> requested) |
| node-graph `move` records | one `drag-nodes` per record; connect / disconnect / delete rows run through the same rules in row order |
| set-step-params / set-step-collapsed | one `set-node-param` (`params` / `collapsed`), nothing when restated; collapse only for controls |
| connect-steps | `remove-edge` for the edge entering the target, one `insert-edge`; refuses self loops, cycles, a second outgoing edge, (typed command) mismatched slot scopes |
| disconnect-steps | `remove-edge` of every matching edge |
| reorganize (typed command: host layout engine; retained job: its own layering) | one `drag-nodes` per distinct offset (`move_to`) |

The retained lanes (`sequence_retained_artifact_emit`, `SequenceNodeGraphState`, `SequenceReorganizeState`) build a `SceneEdit` and publish `edit.leaves`; `SequenceNodeGraphState` drops its `base` scene (one scene copy fewer), keeps the cold-retirement bookkeeping on `edit.scene`.

**Flow** (`✏️editor/📐️edit-rules`, `crate::editor::flow::edit_rules::ContentEdit` over the content child): the `FlowHost` stays the oracle of port compatibility, id minting and layout only.

| Gesture | Leaves |
|---|---|
| disconnect | `remove-edge` (refused `mutation.target-missing` when absent) |
| remove-widget / delete-selection | `remove-edge` per named or touching edge, then `remove-node` (`flow_removal_leaves` is now this rule) |
| connect-media-ports / node-graph `connect` | host validates and mints the synapse id; `remove-edge` for edges entering the same target port, one `insert-edge` |
| rename-flow-widget | `insert-node` of the renamed copy at the old index, `set-edge-endpoints` per touching edge, `remove-node` of the old node |
| reorganize | `set-node-position` per node the host layout moves |
| node-graph `insertPort` | `set-node-param` of the side's port list + `set-edge-endpoints` for wires at or past the insertion point |
| node-graph `delete` / `disconnect` | the remove rules above |

**Tests**: new rule suites `sequence .../📐️edit-rules/🧪️tests/🔬️unit` (10 tests: every gesture above, incl. a middle-step removal, nested control removal, grouped layout offsets) and `flow .../📐️edit-rules/🧪️tests/🔬️unit` (5 tests: middle-node removal, rename keeps child position, connect replaces the entering wire, middle port insertion, absolute position/param leaves). Each test checks (1) the expected leaf kinds, (2) folding the leaves over the base content through the stdio flow `diff`/`apply_diff` reproduces the gesture's working content, (3) undo (each leaf's concrete `inverse`, rows reversed, leaves last-to-first) restores the base. The old whole-scene "leaves reproduce edited content" tests were rewritten around the rules (`child_intent_leaves_reproduce_the_edited_content`, `graph_gestures_land_as_the_concrete_leaves_their_rules_name`). Command-level tests (`step`, `layout`, `connection`, flow `disconnect`/`remove-widget`/... ) are unchanged end-to-end dispatch tests.

Known behaviour deltas: sequence typed `connect-steps` keeps the host's same-slot rule while node-graph / retained `connect` (as before) does not; removal of nested controls now follows the transitive closure; an invalid `set-step-params` JSON is ignored as before.

## Wave 4c: strengthened gate (R8/R11/R12 now scan every fn under `🧬️mutations` and `🔺️diff`)

The gate grew at ~22:20 (34 new bim breaches, all textual: `&mut` anywhere, `.negate(`, `let mut = base.clone()`). Fixed in the code, not the gate:
- `Patch::negate` renamed `Patch::restoring` (trait, ~150 patch impls, 17 inverse leaves, `KeyedDelta::inverse`, `ModelDiff::inverse`).
- `ModelDiff` algebra without `&mut`: `KeyedDelta::then(self, later) -> Self` (was `absorb(&mut self)`), `region_paths(&self, collection) -> Vec<String>` (was `touches(.., &mut Vec)`), pure `combine`/`combine_patch` (were `absorb_delta(&mut Option)`), `merge_slot!` macro (was `fn merge_slot(&mut Option)`), `write_into` builds the next collection from iterators instead of `let mut next = base.clone()`; `MutationDiff::absorb` stays the thin exempt wrapper.
- `🌊️cascade` `Removal::with_root(self, ..) -> Option<Self>` (was `root(&mut self)`), `🧵️elements` `patched(slot, id, patch) -> Option<KeyedDelta>` (was `&mut` slot). Diff tests use `then`.
Scope gate after this: 0.

## Schema catalog
`bun ./📜️script.ts schema generate` ran (exit 0, 3646 scopes): `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` regenerated (+105/-98), the removed `FlowDiff`/`SequenceDiff` `artifact` slots and sequence delta types are gone.

## Test status
- Policy gate over the four scope paths (all diff-only rules R8-R16, rule 1 outcomes, rule 2 message codes, R15 sum-law, `apply_to`): 0 breaches (scratch checker `scratchpad/scope-gate.ts`, run after the last edit).
- `rustfmt` syntax parse of every touched Rust file: OK. Macro expansion, types and tests: NOT compiled, NOT run. `foundation.status` stayed RED from 17:35 through 20:01 (last: unresolved import in `os/📦️packages/🦀️rust/../🎒️pack/🌱️value/🏷️symbols/🎮️decode`, peers' `🌱️value` retirement rewrite), so no cargo call was made (waited > 60 min, then the usage-limit restart).
- Run once GREEN (one cargo at a time via the gate, per artifact dir, `--target wasm32-wasip2` for the check):
  1. `"$T/🚦️gate.sh" bim-sequence-flow-hub -- cargo check -p semio-s-artifact-bim-model --target wasm32-wasip2 --message-format=short`, then `cargo test -p semio-s-artifact-bim-model window_config inference transient presence diff` (cwd `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`);
  2. same for `semio-s-artifact-sequence-sequence` (filters: `diff`, `transient`, `window_ownership`, `node_graph`, `step`) and `semio-s-artifact-flow-flow` (`diff`, `presence`, `window_ownership`, `set_active_example`, `node_graph`);
  3. `semio-hub-space` (filters: `presence`, `config`).
- Likely first compile findings to expect: field-type imports at the `sparse_record_diff!`/`field_set_mutations!` sites, `OutcomeCode`/`KeyedRow` paths via the `protocol` alias, the `FlowSnapshot`/`Effect` paths in the new flow tests.

## Open issues
- Shared `📜️script.ts` edit (rule 1 accepts `OutcomeCode::<Variant>`): fold into the gate owner's change; also update the "9 frozen codes" wording (vocabulary is 11).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` still lists the removed `FlowDiff`/`SequenceDiff` `artifact` slot and sequence delta types: regenerate with `bun ./📜️script.ts schema generate`.
- Wire changes (greenfield, no migration): flow presence ops now `{"kind":"set-…","value":…}` JSON; hub space presence op keywords `active-node`, `focused-node`, `collapsed`, `preview-off`, `camera`, `remove-camera`; BIM window-config diff keys are snake_case (`sparse_record_diff!`); `Replace { config }` replaces `Snapshot { config }`.
- BIM window configs deliberately use one `Replace` kind (existing callers and tests assume one mutation per gesture) rather than per-field sets; switching to `config_record!`-style per-field sets would change `addressed` to `(view, base, config) -> Vec<_>` as sequence/flow main windows now do.
