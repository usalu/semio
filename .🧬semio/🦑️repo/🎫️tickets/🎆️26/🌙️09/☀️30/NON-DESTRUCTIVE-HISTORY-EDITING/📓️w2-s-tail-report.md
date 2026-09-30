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
