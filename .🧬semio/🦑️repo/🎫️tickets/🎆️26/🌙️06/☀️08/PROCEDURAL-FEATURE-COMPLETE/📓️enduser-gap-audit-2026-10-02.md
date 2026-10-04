# End-User Gap Audit — 2026-10-02

Read-only source audit of current procedural generation3d and its existing flow BRep/mesh extension. No builds or runtime tests were executed. Browser/native verification belongs to the parent task. Paths below are repository-relative; line numbers refer to the inspected checkout and may move under concurrent edits. Operator counts and preceding reports were not used as evidence of end-user completeness.

## Confirmed Task-Sized Gaps

### P1 — Mesh Surface Data Cannot Enter or Survive the Procedural Mesh Contract

Evidence: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🟦️.ts:3` defines only vertices/faces; its parser at lines 10–14 actively rejects every other field. Rust counterpart `🦀️.rs:75–81` does the same. `PolygonData` at lines 105–119 serializes only positions/topology, and `mesh_output` at 139–149 reconstructs a new HalfedgeMesh before tessellation. Existing kernel/preview normals and UV buffers therefore do not imply editable or retained surface data. Custom corner normals, UV seams, colors, material assignments, and arbitrary domain attributes cannot be represented by these operators.

Concrete task: extend the existing mesh schema and existing mutation/inference owners with attribute domains and explicit interpolation/remapping rules, first covering a topology-preserving transform and construct round trip. Add cross-language fixtures plus an existing third-party oracle. Follow with topology-changing operator remapping tests; do not silently discard attributes where remapping is unsupported. This is a completeness requirement under the assigned arbitrary mesh/surface scope; exact domain design is a recommendation.

### P1 — Import and Export Silently Strip Surface Channels

Evidence: generation3d `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:60–88`, `semio_mesh_from_mesh_data`, exports normals but explicitly sets `uvs: Vec::new()`, `colors: Vec::new()`, `material_id: None`, and empties materials/textures. `merge_meshes` at lines 92–108 appends colors but not UVs; export then discards those appended colors. `import_mesh_data` at lines 127–143 converts only positions/indices into the construct payload. Geometry-only import/export can be valid, but successful rich-format IO currently provides no authored loss report here.

Concrete task: preserve supported channels end to end through the existing IO/schema inference system and supply a localized explicit loss diagnostic for target formats or attributes that cannot be represented. Validate glTF/OBJ/PLY color/normal/UV fixtures and export after a topology-preserving edit. This is a requirement for full-fidelity surface workflows; choosing diagnostics for incapable formats is a recommendation.

### P1 — Retained Modeling Jobs Still Finish in an Unbudgeted Encoding/Tessellation Phase

Evidence: flow mesh `🦀️.rs:208–225`, `MeshOperatorJob::step`, calls synchronous `mesh_output` as soon as the retained kernel job returns Done; that function at lines 139–149 serializes all polygons, reconstructs the entire halfedge mesh, tessellates, mesh-pack encodes, and base64 encodes before returning. The supplied budget governs `job.step` only. Cancellation at lines 229–232 returns immediately once output is present. Large completed jobs therefore have an expensive final phase that does not yield or admit cancellation.

Concrete task: keep preparation, tessellation, and encoding in the same retained job as explicit budgeted phases; expose truthful phase progress and cancel between chunks. Assert cancellation during output encoding and bounded stepping for a large fixture. This follows the explicit AGENTS requirement to support progress/cancellation for all expensive operations.

### P2 — Other Expensive Mesh Operators Are Still Synchronous

Evidence: `MeshOperation::step_plan` at lines 239–251 returns a job only for bevel/decimate. Current `evaluate` includes merge-by-distance, mirror, loopCut up to 256 cuts, subdivide, fromBrep tessellation, and analysis. The existence of retained bevel/decimate jobs does not verify cancellation or bounded work for those other routes. Inputs are size bounded at 100000 vertices/faces and 600000 corners, but that is not a bound on each operation's work or generated output; no shared output cap is enforced in `mesh_output` itself.

Concrete task: measure these routes under the current harness, then convert routes that exceed an interaction turn budget into retained jobs using the existing owner. Add explicit generated-output limits before committing output and a recoverable fault. Bounded progress/cancel is a requirement; priority based on measurement is recommended. No runtime performance claim is made by this audit.

### P2 — Structured/List Inputs Require Wiring Rather Than Direct Property Editing

Evidence: generation3d `✏️editor/📌️panels/🔍️inspection/🦀️.rs:112–136` edits noncollection point/vector components and noncollection number/text/boolean. Collection or other schema inputs show `input_connect_hint`. This is an explicit intentional surface limitation, not a missing scalar editor. OutputExport format and Variable name/schema are read-only rows at lines 139–153. Raw mesh text/note payloads larger than UI_TEXT_MAX_BYTES are also intentionally read-only.

Concrete task: provide a validated structured/list editor through existing `change-widget-input` mutations, starting with an ordered points/list input and a selection index list; add format editing for OutputExport through its existing mutation. Large mesh authoring should use a file-backed existing import interaction with a clear limit, rather than expanding general UI text fields. These are recommended usability tasks unless the product specification explicitly requires direct editing of every port.

## Existing Capabilities That Must Not Be Misreported as Absent

- Import/export commands are real: `✏️editor/🎮️commands/📤️export-document/🦀️.rs:40–73` exports prepared retained preview or full-fidelity txt, and explicitly faults when prepared geometry is absent. The artifact registers glTF/OBJ/PLY/STL/DWG geometry IO. Interaction success still requires parent runtime verification.
- Scalar and point/vector property inputs exist, and connected ports intentionally show the source instead of overwriting a wired value.
- BRep catalog summaries use quality tags via `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:16`. Their presence is stronger than claiming every operator is exact. This audit did not independently verify every quality assignment or prove that the UI exposes those summaries accessibly.
- The existing generation3d inference schema at `🧬️schema/💡️inferences/🟦️.ts:11–14` contains topology only. Per-result quality/provenance could be added there and surfaced in the inspector if required; this is a recommendation, not a confirmed bug. Do not introduce a parallel provenance/analysis module.

## Recommended Completion Order

1. Attribute-preserving contract and rich-format IO (the most blocking feature-completeness gap).
2. Budgeted output phase and measured remaining expensive operator jobs.
3. Structured property editing, then result quality/provenance visibility through existing inference owners.

No code or AGENTS files were edited. This report is the only created artifact.
