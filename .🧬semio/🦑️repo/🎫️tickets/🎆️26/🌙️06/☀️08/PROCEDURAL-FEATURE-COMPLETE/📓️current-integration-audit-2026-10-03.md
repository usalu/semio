# Current Procedural Integration Audit — 2026-10-03

Read-only source audit under the existing open ticket. Read the current coordination and widget-plan reports before inspecting production. No heavy targets, browser sessions, git mutations, production changes or test executions were performed. Findings describe current source, not successful assembled runtime behavior. Paths below are relative to the repository root.

## Concrete Remaining Authoring Gap

The mesh source inspector cannot author material or texture assets. Let `I` denote `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`, and `C` denote that editor's `🎮️commands/🎚️set-widget-input/🦀️.rs`.

- `I:260` explicitly enumerates only vertices, faces and attributes. Material/texture tables never enter its structured tree.
- `C:148` explicitly rejects any mesh-source path whose first component is not vertices, faces or attributes. This is a command refusal, not simply a hidden control.
- `I:277` can create UV, normal, color, number, text, boolean and vector channels, but offers no material asset or texture asset creation.
- Existing asset retention is real: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs:111` retains materials/textures in PolygonMeshSource, and lines 115–116 restore surface assets/attributes. Importing an already authored asset therefore does not establish user authoring of that asset in the inspector.

Recommended next bounded slice: extend the existing meshSource facet/tree and existing mutation owner for canonical material/texture entries and assignments, with neutral fixture, third-party schema/render oracle, actual undo/redo and EN/DE controls. Coordinate with root's rich glTF ownership and mesh execution's canonical texture-reference ownership. Do not introduce a second material model, state owner or evaluator.

## Concrete Remaining Bounded-Work Gaps

Let `M` denote `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs`.

- `M:433` explicitly returns no operator job for exportObj and toBrep. `M:612` invokes whole-mesh `to_obj`; `M:614–617` converts through that whole OBJ string and calls kernel.import_obj inside SessionCapture.with_kernel. These paths have no retained work/cancellation phases in this operator. Mesh-derived BRep quality is declared at `M:626`; this is not analytic BRep recovery. A job wrapper around final publication alone would not resolve the expensive pre-publication work.
- `M:434` runs prepare before constructing the retained output job. `M:496–502` decodes arbitrary construct data and builds primitive meshes synchronously. Consequently the output job's bounded steps do not bound source decode/construction. Sphere is capped at five subdivisions and cylinder/cone at 1024 segments, so do not call them unbounded in size; the missing guarantee is per-turn work/cancellation during construction/admission.
- `M:571–577` flips selected faces synchronously in prepare; flip is absent from the retained modeling allowlist at `M:432`. This is separate from retained dissolve/weld already assigned to the mesh owner. Verify its actual kernel implementation before deciding whether existing retained affine/topology machinery can own the work.

Recommended next slice: source admission/construction, OBJ serialization and mesh-to-BRep conversion through existing operator/kernel job authorities. Add cancellation before publication and phase/budget laws. This audit did not measure latency and does not claim every synchronous call is independently slow.

## Selected Analysis Presentation Gap

`I:103–152` builds the selected neuron's kind and editable input fields. It does not reflect that neuron's evaluated outputs. `I:161–163` consumes eval_json only for Widget::Variable and selects its first output. Thus selecting brep.mesh.analyze itself does not expose its area, topology or bounds outputs in this inspector; selecting inspectFace does not show the computed corner list/normal/center there either. The existing Flow window receives eval_json (`🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:321`), so this finding is specifically about selected inspection, not a claim that results are inaccessible everywhere. Browser verification should determine whether existing output previews already provide an adequate authoring experience before adding UI. If necessary, project bounded scalar/vector/list outputs from the existing evaluation into the selected inspector rather than recomputing analysis.

## Corrected Historical Claims

Do not repeat that fromBrep or mesh analysis lacks retained jobs. `M:429–430` now dispatches both through MeshOperatorJob. `M:190–240` implements retained corner/topology/bounds/tessellation/triangle phases; `M:222` advances tessellation by one work unit. The owning unit test source contains mesh_analysis_retains_topology_tessellation_and_measurement at line 157 and the actual fromBrep dispatch-job cancellation/publication law around lines 204–212. Their presence is source evidence only; this audit did not rerun them.

The constructor/property editor is also already authored: `I:119–139` handles collection cardinality and point/vector components; `I:267–310` uses tree-window indexing for structured mesh arrays, add/remove/reorder, and attribute rename/presets. Do not replace it with raw JSON editing or label it absent. Slider accessibility/blur Commit and actual all-13 mutation/browser gates remain their assigned lanes, not new findings from this audit.

## Verification Dependencies

The shared selected component closure must build against the current typed SQLite/ValueError authorities before fresh native/app/browser claims. Preserve root's sole served editor port 6018. Kernel/provider success and historical coordination receipts do not prove current catalog admission, port metadata parity, selected inspector interactions, emitted quick actions, cancellation or history. This audit found no source evidence warranting ticket/goal completion.

Root's current IO attempt completed portable/strict TypeScript but Cargo metadata refused the MP3 test-oracle path before native assertions (receipt `🗑️generated/sol-2026-10-03/io-surface-native-current-red.log`). A fresh source read shows MP3 `📦️packages/🦀️rust/Cargo.toml:47` now uses `../../🔮️oracles/📦️packages/🦀️rust`, which resolves within the MP3 artifact to the existing oracle Cargo.toml. The observed earlier path resolving under the parent artifacts directory is no longer present in this manifest; this is consistent with a concurrent manifest correction, not proof of its author or timing. No manifest repair was made by this audit or root. This metadata failure is not a feature-red assertion receipt.
