# 📓️ W2-S-E — schema/payload parity: layout + tail (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, parity executor W2-S-E, 2026-09-30. Contract: `📋️design.md` §6, the W2-R and
W2-S briefs in `🧭️plan.md`. Scope: `✏️s/🔌️plugins/{📏️layout,➗️mathematical,📜️imperative,🏭️process,🎬️sequence,🌊️flow,🕸️dag}`
and the os leaves under `🧰️framework/🛍️products/💻️os/**` (workflow, store, flow vcs, config).

## 1. Outcome

| Gate | Before | After |
|---|---|---|
| `schema mutation-payloads` findings in scope | 165 (layout 63, process 21, math 21, os 11, imperative 10, flow 12, sequence 12, dag 15) | **0** — every owner clean: layout 28/28, math 17/17, process 16/16, imperative 10/10, sequence 8/8, flow 10/10, dag 17/17, os 27/27 fixtures |
| `schema mutation-inputs` findings in scope (collecting reader) | 3 (`replace-flow-host-snapshot` `/hostSnapshot/{schema,widgets,synapses}`) + 14 layout stubs | **0** — layout 131/131, process 26/26, math 33/33, imperative 14/14, sequence 17/17, flow 27/27, dag 36/36, os 136/136 inputs declared |
| Rust struct ↔ leaf schema probe (`🧪️w2-s-e-leaf-parity.py`, also covers leaves with no fixture) | 40 layout + 3 math + 3 Option-null + 10 workflow snake drifts | 0 real drifts (6 remaining lines are probe false positives: one-line struct bodies, and `required`+nullable `Option`s that Rust always emits) |
| Python `jsonschema` Draft 7 oracle (`🧪️w2-s-e-check.py`) | — | 189 leaf schemas valid, 901 `x-semio-ui` nodes valid against manifest `$defs/InputUi`, 122 fixture wires: leaf + aggregate verdicts 0 rejections |
| Layout second implementation (`📐️mutate-layout-1/🐍️.py`, run by `🧪️w2-s-e-layout-oracle.py`) | snake keys | **52/52** mutate + inverse scenarios pass on the camelCase vectors |

## 2. What changed

### 2.1 Layout (`📏️layout`)

- **Rust**: every one of the 45 `LayoutMutation` leaf structs gained `#[value(rename_all = "camelCase")]` plus the test-serde twin
  `#[cfg_attr(test, serde(rename_all = "camelCase"))]` (the leaf tests decode fixtures through serde). The aggregate stays
  externally tagged (`{"ChangePageWidth": {...}}`, PascalCase keys); the approved rule accepts external tagging, and the binary
  codec (`VariantTag::Key`) and text codec keep working unchanged.
- **Fixtures**: 20 of 28 committed `🦠️mutation/🔣️.json` renamed to camelCase (8 had no multi-word keys).
- **14 stub leaves** now carry full payload schemas with canonical `$id`s
  (`…/s/layout/layout/mutation/<kind>/schema.json`), `additionalProperties: false`, exact `required`, local `$defs`
  (`LayoutRect`, `TextStyleRun`, `PageOverride`, `LayoutBounds`) and en/de `x-semio-ui` on every field (DTP terms: Zeichenformat,
  Mustervorlage, Druckbogen, Hilfslinien, Formatbereiche, Abweichungen, Beschriftungsindex, Auflösung/dpi, Farbprofil, mm/pt
  units, rotation dial rad→deg): `create-/delete-/update-character-style`, `update-parent-page`, `update-spread`,
  `set-page-parent`, `set-page-guides`, `set-story-runs`, `update-link`, `set-page-overrides`, `create-layer`,
  `set-frame-layer`, `set-drawing-text`, `reorder-frame`.
- **Option fields admit `null`** (value_derive emits `null`): 10 full leaves incl. nested `link.colorProfile/state/proxyDataUrl`
  and `story.styleRuns[].paragraphStyleId/characterStyleId`.
- **Aggregate** `🧬️mutations/🔣️.json`: rewritten by `🧪️w2-s-aggregate-rule.py` into 45 externally tagged branches (it was
  unresolvable before: stub leaves had no `$id`).
- **Surfaces regenerated from the Rust structs** (`🧪️w2-s-e-layout.py surfaces`): aggregate `🔗️.graphql` leaf types
  (camelCase), TS twin leaf interfaces (camelCase, `T | null` for `Option`, `?` for `#[value(default)]`) + corrected header,
  text-facet `📝️text/🔣️.json` (all 45 kinds, was 28), text `🔗️.graphql` inputs, `🛰️.proto` messages (proto snake_case idiom,
  `optional` for `Option`), and the positional grammar productions of the 14 stubs in `📖️.grammar.semio`, `🅰️.g4`, `🔤️.ebnf`.
- **Second implementation** `🧪️tests/📐️mutate-layout-1/🐍️.py`: payload and inverse keys camelCase.

### 2.2 Mathematical (`newDirected` vs `new_directed` → camelCase rule)

- All 15 `EquationMutation` leaf structs gained `#[value(rename_all = "camelCase")]` (3 change wire: `newDirected`,
  `newAlgorithm`/`newAlgorithmSeed`, `newLabel`); 3 fixtures renamed; the kebab text codec (`new-directed=`) and positional
  binary codec are independent and unchanged.
- `replace-graph.graph.algorithmSeed` and `update-graph-algorithm.newAlgorithmSeed` admit `null`.
- TS twin: camelCase fields, corrected header, and the pre-existing broken import of `EquationGraph`/`EquationPoint` from
  `../🟦️.ts` (never exported there) replaced by local record interfaces; `CreateNode`/`ConnectNodes` gained `index?`.

### 2.3 Process (`🏭️process`)

- The tag-only stubs became the real internally tagged unions with every variant field and en/de annotations:
  `ProcessMeasure` (`measure: cut|drill|attach`, tool/component `WorkingSolid`, `Pose`), `MeasureRecipe` (`recipe:
  discCut|bladeCut|pocketCut|boreDrill|cylinderAttach|boxAttach`, parameter-id fields), `CapabilityRule` (`kind: min|max`,
  quantity select, parameter, margin m→mm) in `create-step`, `replace-step-measure`, `create-machine`,
  `replace-machine-capabilities`; `create-machine.machine.catalogId` added.
- `replace-stock-solid.newSolid.target` → `$ref` framework io `ArtifactRef` (its `dialect` is an object).
- `change-step-origin.newOrigin`, `change-cursor.newResolvedUpTo` admit `null`.
- **Rust**: `WorkingSolid` gained `rename_all_fields = "camelCase"` (its multi-word variant fields `mesh_url`, `solid_handle`,
  `reference_id` wired snake under serde semantics; no JSON/TS consumer used the snake names).

### 2.4 Imperative, sequence, dag, flow

- **imperative**: `PathRef.owner/slot` (`Option<String>`, emitted `null`) admit `null` in all 4 leaves; `newParams` and
  `Step.params` are neural dictionaries (`additionalProperties: {$ref: NeuralValue}`, `NeuralValue` = atom or nested dictionary,
  mirroring `neural_engine::{Value, Atom, Dictionary}`) instead of bare objects.
- **sequence**: `step.params` / `params` (`StepParams`, transparent `Dictionary`) the same.
- **dag**: `replace-node-kind.newKind` is the real `DagNodeKind` union (11 `kind` branches, `$defs` `IoPortSpec`, `DagMedia`,
  `DagPreviewContent`); the payload keeps the Rust wire `variadic_inputs`/`variadic_outputs` (explicit `#[value(rename)]` in the
  shared os `DagNodeKind`, see §4). `change-node-operator-kind.newOperatorKind` admits `null`.
- **flow plugin**: `create-widget`/`replace-widget.widget` → `$ref` the flow host wire `Widget` union of the os flow vcs
  snapshot schema (one definition for plugin and vcs leaves).
- **os flow vcs** (`🌿️vcs/🧬️schema`): `$defs/Widget` neuron ports corrected to the Rust wire `inputPorts`/`outputPorts`
  (`rename_all_fields = "camelCase"`); `HostDocument` members labelled (schema, camera, widgets, synapses, layout) — clears the
  `/hostSnapshot/*` findings. All 10 vcs leaves dropped `propertyNames`: after the aggregate tag-const sweep its enum excluded
  the required `operation` tag, so no instance could validate (additionalProperties false already closes them).

### 2.5 os workflow and store

- **workflow** (18 leaves): member names camelCase as the `rename_all = "camelCase"` leaf structs emit them (`nodeId`, `portId`,
  `inputId`, `edgeId`, `parameterId`, `fieldPath`; were snake), and the 8 opaque records are full: `node` (`WorkflowNode` +
  `WorkflowMediaPort`/`MediaPortSpec`/`MediaType`), `edge` (`WorkflowEdge` + `MediaContract`, `MediaWireFormat`, conversion pair
  with option labels), `binding` ×3, `input`, `parameter` ×2 (`type`-tagged `WorkflowParameter` union).
  **Rust**: `WorkflowInput`, `WorkflowInputBinding`, `WorkflowOutputBinding` gained `#[value(rename_all = "camelCase")]`
  (they wired snake inside camelCase payloads).
- **store**: `commit-space-checkpoint` authors items are `Author` records; `parentId` (skipped when `None`) is no longer
  required. Demo fixture leaf `delete-n` (since flattened by a peer) drops `propertyNames: false`, which refused its required
  `operation` tag (see §6).
- `os/config change-merge-policy` (the 1 `invalid` in the baseline) was already clean on re-run (peer fix).

## 3. Verification (all run, foreground)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-payloads --json` (test module), filtered by `🗑️generated/w2s-e/scope.py` | 0 findings in scope (206 repo-wide, all outside) |
| `bun ./📜️script.ts schema mutation-inputs --json`, filtered by `inputs-scope.py` | 0 findings in scope (857 repo-wide, all outside) |
| `python3 🧪️w2-s-e-leaf-parity.py <scope roots>` | 172 leaves, 0 real drifts (6 probe false positives, §1) |
| `.venv/bin/python 🧪️w2-s-e-check.py` | 189 leaves, 901 `x-semio-ui` valid, 0 invalid schemas, 122 fixture wires, 0 rejections |
| `.venv/bin/python 🧪️w2-s-e-layout-oracle.py` | 52 passed, 0 failed |
| `bunx tsc --noEmit --strict … layout + math mutation twins` | 0 errors (the layout package entry still shows 2 pre-existing errors in `📸️snapshot/📝️text` and `🔺️diff/📝️text` twins, untouched) |
| `cargo check -p semio-s-artifact-layout-layout -p semio-s-artifact-mathematical-equation -p semio-s-artifact-process-process3d --target wasm32-wasip2` | **Finished** (warnings only, incl. layout/math crate warnings as proof of type-check) |
| `cargo check -p semio-framework-artifact-workflow-workflow` | Finished |
| `cargo test -p semio-framework-artifact-workflow-workflow` (private target, `CARGO_INCREMENTAL=0`) | 37 passed, 0 failed |
| `cargo test -p semio-s-artifact-layout-layout --lib` | **WRITTEN BUT UNVERIFIED**: does not compile natively because of peer in-flight errors outside this WP — first `semio-framework-plugin` (`time_travel::Supersede*` imports, `HistoryEntry` missing `author`/`transition_id`), then `semio-framework-ui` (`UiInputNode` missing `snaps`). The math/process leaf tests are blocked by the same crates. W3-G should rerun `cargo test -p semio-s-artifact-layout-layout -p semio-s-artifact-mathematical-equation -p semio-s-artifact-process-process3d --lib` |

## 4. Open items (outside this WP or deliberately bounded)

1. **Schema catalog hashes** (`📚️library/🔣️schema-catalog.json`) are stale for every touched schema until the central
   `schema generate` (the lints read the documents from disk; the 14 layout stubs are already catalogued).
2. **Snake wire left in shared records** (described exactly, not renamed): os `DagNodeKind.Computation.variadic_inputs/outputs`
   (explicit `#[value(rename)]`, used by every dag snapshot) and framework `MediaWireFormat::Binary.format_kind`
   (`🛂️manifest/🦀️.rs`, hot file). `WorkflowSnapshot` root fields (`parameter_bindings`, `input_bindings`, `output_bindings`)
   are snake too — snapshot parity, not a mutation payload.
3. **Loose nested records** (not findings; `additionalProperties: true`): layout `create-frame.frame` and `create-page.page`
   only type `kind/id` and `id/name/width/height`; dag `create-node.node` types the flattened kind as a string enum.
   Candidates for `$ref` into the precise layout snapshot `$defs` once those carry `x-semio-ui`.
4. The flow vcs `🔺️diff/🧫️fixtures/🧾️ownership` fixture still names `input_ports` (diff domain, not a payload).
5. The process artifact schema (`🧬️schema/🔣️.json` `$defs` `Process3dPose`/`Process3dStep`) is itself a bare-object stub and
   `ArtifactChildHandle.target` is typed `string` — snapshot parity for the owner of that document.

## 5. Files

- **Ticket inputs (kept)**: `🧪️w2-s-e-leaf-parity.py` (Rust↔schema probe), `🧪️w2-s-e-layout.py` (rust/fixtures/schemas/surfaces),
  `🧪️w2-s-e-mathematical.py`, `🧪️w2-s-e-process.py`, `🧪️w2-s-e-schemas.py` (imperative, sequence, dag, flow, os),
  `🧪️w2-s-e-check.py` (Python oracle), `🧪️w2-s-e-layout-oracle.py` (standalone second-implementation run),
  `🧪️w2-s-e-strict-ajv.ts` (strict x-semio Ajv oracle over every leaf and aggregate of the scope).
- **Rust**: 45 layout leaf `🦀️.rs`; 15 math leaf `🦀️.rs`; `🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs` (`WorkingSolid`);
  `🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs` (3 records).
- **Schemas**: 24 layout leaves + aggregate + `📝️text/🔣️.json`; 3 math leaves; 7 process leaves; 4 imperative; 2 sequence;
  2 dag; 2 flow plugin; os flow vcs `🔣️.json` + 10 leaves; 18 workflow leaves; 2 store leaves.
- **Fixtures**: 20 layout + 3 math `🦠️mutation/🔣️.json`.
- **Surfaces**: layout `🔗️.graphql`, `🟦️.ts`, `📝️text/{🔗️.graphql,🛰️.proto,📖️.grammar.semio,🅰️.g4,🔤️.ebnf}`; math `🟦️.ts`;
  layout `📐️mutate-layout-1/🐍️.py`.
- **Scratch**: `🗑️generated/w2s-e/` (lint JSON, filters, layout pre-edit tarball).

## 6. Follow-up (coordinator request): strict Ajv and `NeuralValue`

- `$defs/NeuralValue` in the imperative `create-step`/`edit-step-params` and sequence `create-step`/`edit-step-params` leaves is
  now an `anyOf` of single-type schemas (`null`, `boolean`, `number`, `string`, object of `NeuralValue`) instead of the
  multi-type `type: [null, boolean, number, string]` the strict oracle refuses (`strictTypes`: union type other than with
  `null`). The `x-semio-ui` on `params`/`newParams` is unchanged. Source: `NEURAL_VALUE` in `🧪️w2-s-e-schemas.py`.
- The store demo fixture leaf `delete-n` had been flattened by a peer into `propertyNames: false` + `required: ["operation"]`,
  which no instance can satisfy; `propertyNames` is dropped (same fix as the 10 flow vcs leaves). A scope-wide scan finds
  0 remaining `propertyNames`/`required` conflicts.
- Negative control: the strict oracle still refuses `{type: [null, boolean, number, string]}` and compiles the `anyOf` form.

| Command (re-run after the follow-up) | Result |
|---|---|
| `bun 🧪️w2-s-e-strict-ajv.ts` (`semioSchemaAjvV1({strict: true})`, every leaf + aggregate `🧬️mutations/🔣️.json` of the scope, incl. the imperative and sequence aggregates) | **228 compiled, 0 failed** |
| `schema mutation-payloads --json`, scope filter | 0 findings; census clean: dag 17/17, flow 10/10, imperative 10/10, layout 28/28, math 17/17, os 27/27, process 16/16, sequence 8/8 |
| `schema mutation-inputs --json`, scope filter | 0 findings; declared = inputs: dag 36, flow 27, imperative 14, layout 131, math 33, os 136, process 26, sequence 17 |
| `.venv/bin/python 🧪️w2-s-e-check.py` | 189 leaves, 901 `x-semio-ui` valid, 0 invalid schemas, 122 fixture wires, 0 rejections |

## 7. Follow-up (coordinator requests): witnesses, negative fixtures, dag rows, flow

### 7.1 Layout wire witnesses (design §11)

- 17 payload-only witnesses at `✳️any/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json` for the leaves no quintet
  covered (the 14 former stubs plus `update-paragraph-style`, `update-text-frame`, `update-layer`): canonical aggregate wire
  (`{"<Variant>": <camelCase payload>}`, `Option` as `null`, `f64` with a fraction, f32 colour channels exactly representable).
  Written by `🧪️w2-s-e-layout-witnesses.py`.
- Proven canonical by `committed_wire_witnesses_are_the_canonical_wire` in the aggregate unit tests
  (`store::os_store::test_support::assert_wire_witness::<LayoutMutation>`) and picked up by the derived
  `semio_payload_law_layout_mutation`. Layout census: 45/45 leaves witnessed, 45/45 fixtures clean.
- Stale layout tests found while running the crate (pre-existing, not caused by this WP) and fixed at the source:
  - 17 committed `🔺️diff/🔣️.json` lacked the `null` members the `LayoutDiff` patch structs gained with the later leaves
    (page `frame_layer`/`frame_order`/`guides`/`layer_*`/`overrides`/`parent_page_id`, frame `inset_*`/`story_id`/`thread_next`,
    link/story patch members). `🧪️w2-s-e-layout-diff-fixtures.py` inserts exactly the missing members from the produced diff and
    refuses to write unless the result equals it (17/17 equal).
  - `semantic_kinds_cover_every_variant` asserted 28 kinds; it now ties `kinds()` to `KINDS` and pins 45.

### 7.2 dag feature rows

`🌳️mutate-dag-1/🥒️.feature`: the `replace-node-kind` rows (mutate and inverse) and the `create-node` rows carried
`variadicInputs`/`variadicOutputs`, but the Rust wire of the shared os `DagNodeKind` is `variadic_inputs`/`variadic_outputs`
(explicit `#[value(rename)]`); FromValue silently ignored the camelCase keys. Rows now carry the exact wire (F10). The subject
already decodes `params` generically (`decode_dag_mutation_json`); the Python oracle does not read `params`.

### 7.3 Flow editor source contract

`✏️editor/🧪️tests/🔬️source-contract/🟦️.ts` (run by `bun ./📜️script.ts test-source` in `🌊️flow/📦️packages/🦀️rust`) now passes:
- the demo asset is the host-grammar content genesis (`demo_example_ships_the_laid_out_default_graph_as_its_content_genesis`),
  not a JSON snapshot: the test asserts the host grammar and its three widgets instead of `JSON.parse`-ing it;
- the `duplicate-widget` check uses the repo's strict x-semio oracle (`semioSchemaAjvV1`) on the full tagged wire (the leaf
  declares the `mutation` const) instead of a bare Ajv that refused `x-semio-ui`;
- the interactive-job catalogue check knows the fifth factory `FlowContributionsJobFactory` (`FLOW_CONTRIBUTIONS_TOOL_IDS`).

### 7.4 Negative witnesses (F16), fixed at the source

| Fixture | Kind | Fix |
|---|---|---|
| flow `duplicate-widget` onto a taken id | state-dependent | New `PlanError::Refused(MutationMessage)` (replaces the string-only `Invalid`) in `📡️spr/🎮️command`: `fold_plan_diff` emits a composite precondition's own coded refusal; any other planning failure stays `mutation.invariant`. The flow precondition answers `mutation.duplicate-id` at `new_id`, `mutation.target-missing` at the source, `mutation.invariant` for `new_id == source_id`. Outcome → `duplicate-id` at `["note-beta"]`; leaf + plan tests updated. |
| forms `create-block` for a missing step | state-dependent | Error `mutation.target-missing` at the step (like its three siblings); outcome, leaf docs, leaf test, unit law (`assert_missing_target_is_error`) and the Python second implementation (its sibling rules also said `invariant` against the Rust) updated; positive witness `➕create-block/🧾️wire-witness` + `committed_wire_witnesses_are_the_canonical_wire`. |
| trinity jack `create-edge` endpoints absent | state-dependent | Error `mutation.target-missing` addressed by the absent node ids (`["shaft","capsule-a"]`); a malformed port key stays the payload invariant and the leaf schema now states it (`pattern: ^[^@]+@.+$` on `source`/`target`); positive witness `🌉️create-edge/🧾️wire-witness` + test in the aggregate's structural suite. |
| animate `resize-tile-crop` zero-width crop | range | `newCrop.width`/`height` `exclusiveMinimum: 0` — a clean negative. |
| dag `reorder-nodes` duplicate id in the order | range | `order` `uniqueItems: true` — a clean negative. |
| sequence `connect-steps` step to itself | cross-field | leaf `x-semio-invariant: [{id: "no-self-loop", description {en, de}}]`, outcome `"invariant": "no-self-loop"` — counted as a declared invariant. |

### 7.5 `FlowMutation` cold retirement

Both flow aggregates now declare `retire_cold = retire_flow_mutation`: the plugin `FlowMutation` retires the `Widget` of
`create-widget`/`replace-widget`, the os vcs `FlowMutation` the `Widget` of `add-widget`/`change-widget` and the
`FlowHostSnapshot` of `replace-flow-host-snapshot` (both own fail-closed `Dictionary`/`OrderedSet`/`OrderedMap`/`Tree` roots).

### 7.6 Verification (follow-up)

| Command | Result |
|---|---|
| `schema mutation-payloads --json` | 0 findings for layout, math, process, imperative, sequence, flow, dag, os, trinity, forms, animate; layout 45/45 witnessed, sequence 1 declared invariant, animate/dag 1 clean negative each |
| `schema mutation-inputs --json` | 0 findings for the same owners |
| `cargo check -p semio-s-artifact-flow-flow -p semio-s-artifact-forms-forms -p semio-s-artifact-trinity-jack -p semio-s-artifact-layout-layout --target wasm32-wasip2` (compiles the os kernel `PlanError` change and the os flow retire) | Finished |
| `cargo test -p semio-s-artifact-flow-flow --lib` | **259 passed**, 0 failed (duplicate-widget refusal/plan tests, payload law) |
| `cargo test -p semio-framework-artifact-flow-flow --lib` | 45 passed, 1 failed: `slider_label_tests::authored_slider_labels_survive_json_dag_and_chrome` (`WidgetDescriptor` accepts a widget without `label`), pre-existing and unrelated; `vcs::mutations::semio_payload_law_flow_mutation` passes |
| `bun ./📜️script.ts test-source` (flow package) | exit 0 |
| `cargo test -p semio-s-artifact-forms-forms -p semio-s-artifact-trinity-jack --lib` | forms **230/230**, jack **176/176** (recoded refusals, new witnesses) |
| `cargo test -p semio-s-artifact-layout-layout --lib` | **482 passed**, 2 failed — both editor behaviour tests outside the mutation vocabulary and untouched by this WP: `patch_document::…patch_frame_sets_flags` (a locked frame now refuses the visibility toggle) and `panels::document::…granularity…` (a page row lost its action); witnesses, payload law, all 28 quintets and the kinds test pass |
| `cargo test -p semio-s-artifact-mathematical-equation --lib` | **393 passed**, 1 failed: `viewer::…geometry::render_produces_a_table_scene_with_one_row_per_point` (renderer table JSON contract, peer area); the two leaf tests that asserted snake_case keys now assert the camelCase wire |
| `cargo test -p semio-s-artifact-process-process3d --lib` | **361 passed**, 1 failed: `every_example_fixture_carries_its_canonical_child_handles` (the drilled-plate stock B-Rep handle digest drifted; the handle hashes B-Rep content, not the `WorkingSolid` wire) |
| `cargo test -p semio-s-artifact-sequence-sequence -p semio-s-artifact-dag-dag --lib` | sequence **207/207**, dag **214/214** |
| `cargo test -p semio-s-artifact-animate-presentation --lib` | 336 passed, 1 failed: `video::program::…a_demo_scene_is_one_still_scene_for_every_captured_frame` (video frame sampling, unrelated to the crop schema bound) |
| `bun 🧪️w2-s-e-strict-ajv.ts` (scope + jack, forms, animate) | **275 compiled, 0 failed** (incl. `x-semio-invariant`, `pattern`, `uniqueItems`, `exclusiveMinimum`) |
| `.venv/bin/python 🧪️w2-s-e-check.py` | 189 leaves, 901 `x-semio-ui` valid, 192 fixture wires (2 negatives rejected or declared), 0 rejections |
| `.venv/bin/python 🧪️w2-s-e-layout-oracle.py` | 52/52 |

Open from this round: the layout diff JSON Schema (`🧬️schema/🔺️diff/🔣️.json`) does not describe the patch members the Rust
`LayoutDiff` gained (`frame_layer`, `frame_order`, `guides`, `layer_*`, `inset_*`, …) — diff parity, outside the payload lint.
The jack `create-edge` `source`/`target` annotations are node-reference pickers although the wire is a `nodeId@portId` port key.

