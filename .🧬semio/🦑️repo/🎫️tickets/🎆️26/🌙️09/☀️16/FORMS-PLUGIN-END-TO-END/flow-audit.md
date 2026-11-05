# Flow and Procedural Extension Audit

Read-only light exploration on 2026-11-04. No runtime checks performed. Paths below are relative to the repository root.

## Verified Architecture

- Flow plugin instructions describe separate graphical flow and neural-tree data. The runtime extension implementation family is under ✏️s/🔌️plugins/🌊️flow/🧩️extensions/ (primitive, math, text, list, dictionary, logic, draw, brep, bim). Each has Rust implementation, component descriptor and standalone guest tests.
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs registers BRep geometry operators behind the spatial-kernel session and BrepKernel interfaces. NODE_KERNEL_METHOD maps operators to kernel methods and exposes quality metadata.
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧩️component/🔣️.json contributes topic flow.extension, extensionId brep, an extension manifest with schemas and operators. Geometry uses opaque handles; mesh schema includes data and preview text fields.
- BRep operator families include primitives, curves, surfaces, sweeps, booleans, transforms, solid features, intersection and evaluation. Registration implementation lives in its 🧾register-operators children.

## Forms Connection

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📝️forms/🧬️schema/🦀️.rs defines FlowFormProducer with slider, note, image, variable and finish. It lets the caller own its actual field/form types.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs contains forms_bridge::flow_host_snapshot_to_form_spec. It projects InputSlider, InputNote, InputImage and Variable widgets only, dropping other widgets. This bridge extracts a form; it does not execute a flow or embed a graph question. Its adjacent apply_generation_values_to_host_snapshot patches those four input kinds.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📝️forms/🧬️schema/🧪️tests/🦀️.rs tests selected widget projection and original source loans using ../../🧪️tests/🧫️fixtures/🔣️.json. Uses serde_json as third-party validation of selectedInputs.

## Procedural3d Connection

Let P = ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/.

- P/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs derives a PlaybookFlowFormProducer spec from the graph snapshot and renders generation_form with the selected generation values, dispatching updateGenerationValues. It returns a localized hint with no selected generation. The TypeScript twin is view-model shape only.
- P/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs evaluates a separately patched generation graph using generation_host_snapshot_for. It evaluates nothing until a generation is selected, then delegates to the surface-neutral preview_eval chain. Emitted extension invocations and publication update the addressed preview. The inline continuation preserves progress and cancellation opportunities.
- P/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs refuses direct dispatch: original replies require retained command work and initiating caller authority. A new consumer must preserve that receiver/authority route.
- P/🧬️schema/🟦️.ts reexports FlowHostSnapshot, FormGeneration, GenerationPlayState and neural types from the procedural foundation schema. It models Generation3dArtifact as hostSnapshot plus generation.

## Existing Test Evidence

- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️evaluate-budget/🦀️.rs exercises the real evaluate handler/kernel and a shared generation3d evaluate-budget fixture. Long boolean work must yield inside caller allowance and use multiple round trips.
- P/✏️editor/🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️budget/🟦️.ts is the caller-side budget contract route.
- P/✏️editor/🧪️tests/💥️extension-evaluate-fault/🟦️.ts pins flow.extension-evaluate-failed publication, faulted-phase semantics and both en/de labels using the shared fault fixture.
- P/✏️editor/🧪️tests/🔬️generate-interactions/🟦️.ts and P/🧬️schema/💡️inferences/📐️geometry children are useful follow-up test routes.

## Concrete Gap and Follow-up Boundary

The inspected bridge and procedural generate-form implement extraction of input widgets and separate preview evaluation. They do not provide a forms-question-owned embedded flow state, execution lifecycle, extension contribution installation, graph output binding or graph editor/preview inside a question. This is a verified limitation of these inspected routes, not proof of absence across the uninspected forms plugin. A forms implementation should reuse flow snapshot/evaluation/extension ownership rather than treating the extraction bridge as an evaluator. Inspect the forms question schema and runtime next to establish its actual missing cases. Procedural3d already supplies reusable BRep extension operators and a budgeted retained-reply evaluation pattern; no new procedural extension taxonomy is justified by this audit.
