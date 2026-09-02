# Artifact Duplication Matrix - Exploration Report

**Date**: 2026-09-02  
**Scope**: All plugins under `s/🔌️plugins/*/🗿️artifacts/*/` vs semio v1 subsets  
**Method**: Schema inspection, ArtifactKindSpec extraction, composition analysis (store::ArtifactChild references)

---

## Executive Summary

**19 semio v1 subsets baseline**: any, animation, audio, brep, cad, document, drawing, flow, graph, image, kit, mesh, model, object, presentation, table, text, value, video

**Confirmed Duplicates Found**: 13 plugin artifacts are semantic duplicates or compositions of semio v1 subsets.

---

## Detailed Findings by Plugin

### Confirmed Duplicates (IDENTICAL / NEAR-IDENTICAL schema)

| Plugin | Artifact | ID | Schema | MediaType | Composed Semio | Overlap | Evidence |
|--------|----------|----|---------|-----------|----|---------|----------|
| 🎞️animate | 🎬️present | animate.present | animate.present | Presentation/Deck | presentation, animation | **Identical** | Bridges FigureTileSource/Draft → SemioPresentationSnapshot; documented composition in code |
| ✒️writer | ✒️writer | text.document | text.document | Text/Document | document | **Identical** | WriterDocumentChild = ArtifactChild<SemioDocumentSnapshot> |
| 🖍️draw | 🖍️draw | (no artifact_kind yet) | draw.document | (extraction incomplete) | (pending) | **Near-identical** | Comment states "2d.drawing document type"; v1 subset exists |
| 🌍️gis | 🏔️gisterrain | gis.terrain | gis.terrain | (compound mesh child) | mesh | **Identical** | Composes: ArtifactChild<SemioMeshSnapshot>; removed duplicate 3d.mesh kind per ticket 26/08/12 |
| 💠️lowpoly | 💠️lowpoly | 3d.lowpoly | lowpoly.fixture | ThreeD/Mesh | mesh | **Identical** | mesh_child_handle() → ArtifactChild<SemioMeshSnapshot> |
| 📐️cad | 📐️cad | 3d.cad | (compound children) | ThreeD/Mesh | model, drawing | **Identical** | CadModelChild<SemioModelSnapshot>, CadDrawingChild<SemioDrawingSnapshot> |
| 🏭️process | 🧊️process3d | 3d.process | process.3d | ThreeD/Brep | brep, flow | **Identical** | brep_child_handle() → ArtifactChild<SemioBrepSnapshot> |
| 💡️reasoning | 🔌️wires | s.reasoning.wires | reasoning.wires.fixture | (compound graph) | graph | **Identical** | WiresContentChild = ArtifactChild<SemioGraphSnapshot>; documented composition replacing inline board_fixture |
| 🕸️dag | 🕸️dag | s.dag.dag | dag.dag | (compound graph) | graph | **Identical** | DagContentChild = ArtifactChild<SemioGraphSnapshot>; nodes/edges now in composed child |
| 🔱️trinity | 🔌️jack | s.trinity.graph | (compound graph) | (compound graph) | graph | **Identical** | JackContentChild = ArtifactChild<SemioGraphSnapshot> |
| 🌊️flow | 🌊️flow | computation.flow | flow.artifact | Computation/Flow | flow | **Identical** | FlowContentChild = ArtifactChild<SemioFlowSnapshot> |
| 🎬️sequence | 🎬️sequence | (artifact_kind pending) | sequence.sequence | (compound flow) | flow | **Identical** | SequenceContentChild = ArtifactChild<SemioFlowSnapshot> |
| 📖️playbook | 📖️playbook | (artifact_kind pending) | playbook.playbook | (compound flow+document) | flow, document | **Identical** | PlaybookFlowChild<SemioFlowSnapshot>, PlaybookDocumentChild<SemioDocumentSnapshot> |
| 📜️imperative | 📜️imperative | imperative.imperative | imperative.imperative | (compound flow+text) | flow, text | **Identical** | ImperativeFlowChild<SemioFlowSnapshot>, ImperativeTextChild<SemioTextSnapshot> |
| 🏛️architect | 🏛️program | architecture.program | architect.program | (compound table) | table | **Identical** | ProgramBenchmarksChild<SemioTableSnapshot>, ProgramKnowledgeChild<SemioTableSnapshot> |
| 🗒️note | 🗒️note | 2d.note | note.document | TwoD/Document | text | **Identical** | NoteTextChild with handle:ArtifactChild<SemioTextSnapshot> |

### Related but Distinct (NOT direct duplicates)

| Plugin | Artifact | ID | MediaType | Notes |
|--------|----------|----|-----------|----|
| 🌍️gis | 🗺️gismap | 2d.map | TwoD/Vector | Related to semio drawing but distinct (map-specific, not general drawing) |
| 🧩️puzzle | ◻️2d, 🖐️5d, 🧊️3d | puzzle.2d/5d/3d | TwoD/Kit/ThreeD Design | References s.stdio.semio.kit but as independent design specs, not direct schema duplication |
| 🧱️block | ◻️2d, 🖐️5d, 🧊️3d | block.2d/5d/3d | Kit/Type | Related to semio kit/object type catalogs; references s.stdio.semio.kit |
| 🌀️procedural | 🌀️procedural2d, 🧊️procedural3d, 🧩️assembly | 2d/3d.procedural, data.assembly | TwoD/ThreeD/Data Flow | Procedural generation (independent schemas, not semio reuse) |
| 🖨️raster | 📷️png, 📷️jpg, 🖼️bmp, etc. | stdio.* | (format converters) | Pure format adapters (not artifact schemas); duplicated from semio image path but as IO bridges |

### Non-duplicates (Independent schemas, format converters)

| Plugin | Artifact | Category | Notes |
|--------|----------|----------|-------|
| 📋️forms | 📋️forms | Form dictionary | Independent from semio table/value |
| 📏️layout | 📏️layout | Layout document | Independent from semio drawing (procedural layout vs general drawing) |
| 🗄️stdio | (24+ format adapters) | Format I/O | Pure format converters (json, pdf, svg, dxf, dwg, csv, xlsx, md, etc.) — not artifact schemas |
| 🎥️shooting | 🎥️shooting | Camera/shot capture | Independent video editing artifact |
| ➗️mathematical | ➗️mathematical | Math/equation graph | Has subsets (equation, geometry, graph) but distinct from semio graph |
| 🌿️vcs | 🌿️vcs | Version control | Metadata/diff document — independent |
| 🎪️demonstrator | 🎪️playground | Interactive demo | Independent from core artifacts |
| 📸️remodel | 📸️remodel | Reconstruction | Independent point-cloud/mesh processing |
| 🔋️energy | 🔋️model | Energy simulation | Independent domain model |
| 📕️norm | (10 norm subsets) | Engineering standards | Standard profiles/tables — independent from semio table (domain-specific) |
| 🪐️space | 🪐️space | Spatial indexing | Independent artifact catalog |
| 🪵️sourcing | 🗂️curate | Sourcing curation | References s.stdio.semio.kit but as catalog metadata |

---

## Interpretation Key

- **Identical**: Plugin artifact is a thin wrapper or direct composition of semio subset (store::ArtifactChild reference confirmed in code)
- **Near-identical**: Schema semantically equivalent but may have been created before unified design or not yet migrated
- **Merely related**: Shares domain (e.g., both graphs) but different structure/use case
- **Independent**: Unrelated schema or pure format converter

---

## Recommendations

### Priority 1: Remove/Consolidate (Confirmed duplicates)
1. animate/present → remove, reuse semio presentation + animation
2. writer/writer → remove, reuse semio document
3. gis/gisterrain → (already consolidates mesh via child composition)
4. lowpoly/lowpoly → (already consolidates mesh via child composition)
5. cad/cad → (already consolidates model + drawing via children)
6. process/process3d → (already consolidates brep + flow via children)
7. reasoning/wires → (already consolidates graph via child composition)
8. dag/dag → (already consolidates graph via child composition)
9. trinity/jack → (already consolidates graph via child composition)
10. flow/flow → (already consolidates flow via child composition)
11. sequence/sequence → (already consolidates flow via child composition)
12. playbook/playbook → (already consolidates flow + document via children)
13. imperative/imperative → (already consolidates flow + text via children)
14. architect/program → (already consolidates table via child composition)
15. note/note → (already consolidates text via child composition)

**Note**: Most of these have already been consolidated per ticket 26/08/12 (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM). Code inspection confirms they use store::ArtifactChild composition pattern rather than standalone artifact kinds.

### Priority 2: Verify & Migrate (Near-identical, pending completion)
1. draw/draw → verify schema vs semio drawing; complete artifact_kind extraction

### Priority 3: Validate (Related but potentially consolidatable)
1. puzzle/* → clarify relationship to semio kit vs independent design specs
2. block/* → clarify relationship to semio kit/object type system
3. gis/gismap → evaluate if should be general semio drawing or stay specialized

---

## Sources

- **Direct schema inspection**: grep for `pub fn artifact_kind()` in `/🏅️standards/🔖️1/🦀️.rs`
- **Composition verification**: grep for `store::ArtifactChild<SemioXxxSnapshot>` patterns
- **Ticket references**: Code comments citing ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
- **Semio v1 baseline**: `s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/` (19 subsets confirmed)

