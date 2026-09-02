# Artifact Semantics & Naming Analysis

## Artifact 1: Mathematical

**Root struct:** `MathematicalSnapshot` (from artifact root 🦀️.rs line 127)

**Top-level fields:**
- `notation: MathematicalNotationChild` (composed `SemioTextSnapshot`)
- `results: MathematicalResultsChild` (composed `SemioTableSnapshot`) 
- `computed: MathematicalComputedChild` (composed `SemioValueSnapshot`)
- `equation: EquationSnapshot`

**Sub-components (from doc comment lines 1-2):**
- `MathematicalGraph`: nodes/edges with directed flag and algorithm selection
- `MathematicalGeometry`: point cloud (convex-hull/centroid demonstration)

**Doc summary:** "a graph playground (nodes/edges/algorithm) and a geometry playground (a point cloud), combined into one snapshot"

**Real-world model:** An interactive mathematical computation system combining graph algorithms (topological sort, traversal) with 2D geometry operations (convex hull, centroid).

**Noun candidates (ranked):**
1. **playground** — Directly from the doc comment; the artifact is an interactive "mathematical playground" for experimentation
2. **computation** — Reflects the mathematical algorithms (graph, geometry) being composed
3. **graph** — Too vague (many artifacts have graphs); ambiguous with procedural/flow

**Reference counts:**
- Module path `artifacts::mathematical`: 292 total (290 inside, 2 outside)
- Schema `computation.mathematical`: 13 total (13 inside, 0 outside)

---

## Artifact 2: Procedural2D

**Root struct:** `Procedural2dSnapshot` (re-exported from snapshot module)

**Top-level fields:**
- Generated from `flow::Widget` vocabulary (Neuron, InputSlider, InputNote, InputImage, Variable, OutputPreview, OutputAction, OutputExport, Cluster)
- Composed into `s.stdio.semio.flow` child

**Doc summary (lines 1-2):** "snapshot re-exports, widget id helper, and artifact kind"

**Real-world model:** A visual procedural graph editor for 2D operations using node-based UI composition. Nodes represent computational units (neurons, inputs, outputs) connected by data flow edges.

**Noun candidates (ranked):**
1. **graph** — It's a procedural node-graph (visual DAG of computational nodes)
2. **workbench** — Could emphasize the 2D-specific nature; less likely
3. **diagram** — Too generic; overlaps with other artifact names

**Reference counts:**
- Module path `artifacts::procedural2d`: 371 total (369 inside, 2 outside)
- Schema `s.procedural.procedural2d`: 18 total (18 inside, 0 outside)

---

## Artifact 3: Procedural3D

**Root struct:** `Procedural3dSnapshot` (re-exported from snapshot module)

**Top-level fields:**
- Same `flow::Widget` vocabulary as procedural2d
- Composed into `s.stdio.semio.flow` child

**Doc summary (lines 1-2):** "snapshot re-exports, widget id helper, and artifact kind"

**Real-world model:** A visual procedural graph editor for 3D operations (mesh generation, geometry, rendering). Same node-graph paradigm as procedural2d but with 3D-specific output formats (STL, GLB, OBJ).

**Noun candidates (ranked):**
1. **graph** — Same as procedural2d (procedural node-graph)
2. **workbench** — Could distinguish from 2D, less likely
3. **engine** — Could work for procedural generation context, less clear

**Reference counts:**
- Module path `artifacts::procedural3d`: 533 total (531 inside, 2 outside)
- Schema `s.procedural.procedural3d`: 22 total (19 inside, 3 outside)

---

## Artifact 4: Imperative

**Root struct:** `ImperativeSnapshot` (from schema module)

**Top-level fields:**
- `schema: String`
- `flow: ImperativeFlowChild` (composed `SemioFlowSnapshot` - control flow tree)
- `text: ImperativeTextChild` (composed `SemioTextSnapshot` - seed dictionary)

**Doc summary (lines 1-4):** "the document entities this plugin's app edits: a Path of control-flow Steps (state.set/log.print/control.if/control.while/math.add/…), each addressable by a PathRef"

**Real-world model:** An imperative programming system combining a control-flow execution graph (if/while/sequence) with variable seed dictionary. Steps are executable operations in a procedural context.

**Noun candidates (ranked):**
1. **program** — The artifact represents an executable imperative program with steps and control flow
2. **script** — Same as program; less formal
3. **procedure** — Could work, but less specific than program

**Reference counts:**
- Module path `artifacts::imperative`: 304 total (298 inside, 6 outside)
- Schema `computation.imperative`: 11 total (11 inside, 0 outside)

---

## Artifact 5: Writer

**Root struct:** `WriterSnapshot` (from artifact root line 76)

**Top-level fields:**
- `schema: String`
- `id: String`
- `language_id: String`
- `uri: String`
- `document: WriterDocumentChild` (composed `SemioDocumentSnapshot`)

**Doc summary (lines 1-2):** "✒️ Writer artifact — the document entity this plugin's app edits"

**Real-world model:** A text editor artifact holding plain text/code with language specification (syntax highlighting), supporting multiple file URIs. The composed document child holds structured markup representation.

**Noun candidates (ranked):**
1. **document** — The artifact's schema is "writer.document" (line 9); already has the right name
2. **file** — Could work (represents a file), but less precise than document
3. **textfile** — Too compound; schema is "document"

**Already a noun:** YES, "Writer" is agent-noun form. Schema uses "document" as the real noun.

**Reference counts:**
- Module path `artifacts::writer`: 281 total (279 inside, 2 outside)
- Schema `text.document`: 13 total (8 inside, 5 outside)

---

## Artifact 6: Shooting

**Root struct:** `ShootingSnapshot` (from artifact root line 16)

**Top-level fields:**
- `schema: String`
- `assets: Vec<ShootingAsset>` (id, name, url, format, origin, orientation, scale)
- `shots: Vec<ShootingShot>` (id, label, dimensions, format, shape, camera_id)
- `saved_cameras: Vec<ShootingSavedCamera>` (id, label, camera params)
- `scene: ShootingSceneLighting` (sun, ambient, shadow, material)
- `emblem: Option<ShootingEmblemChild>` (composed `SemioImageSnapshot`)

**Doc summary (lines 1-4):** "the real icon-studio snapshot (assets, shots, saved cameras, scene lighting)"

**Real-world model:** A 3D product/icon rendering studio. Manages 3D assets (meshes), multiple camera angles and shots (render configurations), lighting setup, material properties. Generates product photography for ecommerce/marketing.

**Noun candidates (ranked):**
1. **scene** — The artifact represents a 3D scene with assets, lights, cameras; `ShootingSceneLighting` is already a component
2. **studio** — Emphasizes the "icon-studio" purpose (ecommerce product rendering)
3. **session** — Could work; less descriptive than scene

**Reference counts:**
- Module path `artifacts::shooting`: 588 total (562 inside, 26 outside)
- Schema `2d.shooting`: 7 total (5 inside, 2 outside)

---

## Artifact 7: Forms

**Root struct:** `FormsSnapshot` (from artifact root line 25)

**Top-level fields:**
- `schema: String`
- `id: String`
- `version: String`
- `title: Option<String>`
- `structure: FormsStructureChild` (composed `SemioValueSnapshot` - lossless form tree)
- `results: FormsResultsChild` (composed `SemioTableSnapshot` - flattened tabular projection)

**Doc summary (lines 1-4):** "the document entity this plugin's app edits" (from artifact root)

**Real-world model:** A form/questionnaire designer with hierarchical steps containing questions (blocks) with 15+ config fields (required, placeholder, min/max, options, conditions, validation). Composes structured data representation (form tree) with tabular results projection.

**Noun candidates (ranked):**
1. **dictionary** — Artifact kind schema is `form.dictionary` (line 430); already uses the right noun
2. **questionnaire** — Could work; emphasizes form-filling use case
3. **survey** — Similar to questionnaire; less technical

**Reference counts:**
- Module path `artifacts::forms`: 283 total (283 inside, 0 outside)
- Schema `form.dictionary`: 26 total (17 inside, 9 outside)

---

## Artifact 8: Flow, Sequence, Playbook Comparison

### Flow (`🌊️flow`)

**Snapshot structure:**
- `schema: String`
- `camera: CameraJson` (viewport state)
- `content: FlowContentChild` (composed `s.stdio.semio.flow`)

**Real-world model:** A visual node-graph editor for procedural/computational workflows. Nodes are widgets (neurons, inputs, outputs, clusters); edges are data synapses. Framework-kernel plugin that IS the canonical editor for stdio's flow subset.

**Name status:** "Flow" is already a noun. Represents a data-flow network.

### Sequence (`🎬️sequence`)

**Snapshot structure:**
- `schema: String`
- `content: SequenceContentChild` (composed `s.stdio.semio.flow`)

**Real-world model:** A directed acyclic graph (DAG) of imperative execution steps. Steps have kinds (state.set, log.print, control.if/while), dynamic params (Dictionary), and optional slot nesting for control flow bodies. Simpler than playbook—no form vocabulary.

**Name status:** "Sequence" is already a noun. Represents an ordered/conditional sequence of operations.

### Playbook (`📖️playbook`)

**Snapshot structure:**
- `schema: String`
- `id: String`
- `version: String`
- `title: Option<String>`
- `document: PlaybookDocumentChild` (composed `s.stdio.semio.document` - narrative projection)
- `flow: PlaybookFlowChild` (composed `s.stdio.semio.flow` - procedural source of truth)

**Real-world model:** A structured questionnaire/workflow with steps containing blocks (form questions) with ~18-field config vocabulary (required, options, fields, conditions). Dual representation: narrative (document for reading) and procedural (flow for execution).

**Name status:** "Playbook" is already a noun. Represents an executable workflow book or guide.

### Key Differences

| Aspect | Flow | Sequence | Playbook |
|--------|------|----------|----------|
| **Primary Use** | Visual node-graph for data/procedural workflows | Imperative step execution with conditionals | Form-driven questionnaire workflows |
| **Node/Step Vocab** | Widget variants (9 types) | Generic steps w/ dynamic params | PlaybookBlock (form question) w/ 18 fields |
| **Configuration** | Per-widget fields flattened into params | String-encoded Dictionary params | Structured condition trees + option lists |
| **Composition** | 1 child (`flow` only) | 1 child (`flow` only) | 2 children (`document` + `flow`) |
| **Nesting Model** | Cluster widgets with nested flows | SlotRef to parent steps | Nested block conditions |
| **Editor Intent** | Dataflow/computational composition | Procedural automation/workflows | Survey/form collection + execution |
| **Names Already Nouns?** | YES | YES | YES |

### Assessment

All three names are **already good nouns**:
- **Flow** = data-flow network (noun)
- **Sequence** = ordered sequence of steps (noun)
- **Playbook** = executable workflow guide (noun)

None require renaming. They are conceptually distinct and their names reflect their primary use cases accurately.

---

## Summary Table: Artifacts 1-7

| Artifact | Models | Ranked Noun Candidates | Ref Count (Total / Out) | Schema Ref Count (Total / Out) |
|----------|--------|------------------------|------------------------|------------------------------|
| Mathematical | Graph algorithms + geometry computation system | playground, computation, graph | 292 / 2 | 13 / 0 |
| Procedural2D | 2D visual node-graph for procedural operations | graph, workbench | 371 / 2 | 18 / 0 |
| Procedural3D | 3D visual node-graph for procedural operations | graph, workbench | 533 / 2 | 22 / 3 |
| Imperative | Imperative program with control flow and seed state | program, script, procedure | 304 / 6 | 11 / 0 |
| Writer | Plain text/code editor with language specification | document*, file | 281 / 2 | 13 / 5 |
| Shooting | 3D product/icon rendering studio with assets and scenes | scene, studio, session | 588 / 26 | 7 / 2 |
| Forms | Hierarchical questionnaire with conditions and validations | dictionary*, questionnaire, survey | 283 / 0 | 26 / 9 |

*Writer and Forms already use the correct noun in their schema (`writer.document`, `form.dictionary`)
