# Final Contracts Audit — 2026-10-04

## Scope and Method

Read-only static audit of the current print schema, TypeScript inference barrel and read-side inference, Rust product/plugin/service, and guarded TypeScript diff/mutations. Root and products AGENTS.md were read. No compilation, Nx task, runtime test, production edit, test edit, Git mutation, or ticket/goal mutation was performed. Reported test counts from the parent were not independently verified here. Native grammar and full catalogue rendering are outside this audit and remain separate gates.

## Material Findings

1. **Inference output schema does not describe its owned numerical contracts.** `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🔣️.json:12` leaves scene node, fill, stroke and clip objects unrestricted; line 16 leaves plan frame, theme and every item unrestricted. Consequently `{tikz:"",diagnostics:[],complete:true,plan:{width:1,height:1,frame:{},theme:{},items:[{}]}}` satisfies the written shape despite not implementing `VizRenderPlan` (render TypeScript lines 22–34). Consumers cannot validate required item discriminators, geometric fields, paint contracts, clipping, or theme structure using the published canonical schema. This is a schema-first contract hole, not a request to narrow arbitrary authored option maps.

2. **TypeScript cancellation is synchronous and cannot receive ordinary same-thread UI cancellation during computation.** `inferVizChart` (`💡️inferences/🟦️.ts:13–37`) performs validation, planning, TikZ rendering and scene rendering in one synchronous call. `planVizChart` checks a signal and invokes callbacks (`🖼️render/🟦️.ts:72–74,140–143`) but never yields. These checks support pre-aborted signals and aborts triggered directly by progress callbacks; they do not let browser input/timer events execute while inference occupies the thread. No async/worker wrapper was established in the audited production entry point. Large UI invocations therefore need an integration layer that actually yields or runs in a worker before interaction-friendly cancellation can be claimed. The Rust service does poll the shared OS cancellation registry independently (print Rust lines 71–74), so this finding is specific to TypeScript's public synchronous path.

## Confirmed Static Contracts

- Shared output requires `tikz` string, structured `{code,path,message}` diagnostics, and boolean `complete`, with optional plan/scene. Empty TikZ is intentionally representable for invalid results. Rust `ChartInference` and TS `VizChartInference` match these required fields. No earlier string-diagnostics mismatch remains.
- Rust declares the real `s.print.chart` artifact and inference capability, then builds `Plugin::<NoPluginApp>::builder("print")` at `print/🦀️.rs:56`. Declaration supplies schema, inference descriptor and executable service; `print_plugin` publishes the service through the existing registry at lines 57–58.
- Controlled native execution admits cancellation identity/budgets, refuses undeclared incremental cache mode, checks decoding depth/work/allocation, invokes cancellation checkpoints, and returns owned packed inference payload (`print/🦀️.rs:79–120`). Static inspection cannot prove shell startup calls this factory: repository searches found its invocations in print tests and its own convenience wrapper, but no OS/s host callsite. This is an integration verification limit, not a demonstrated defect in the reusable product factory.
- TypeScript guarded replay clones the snapshot, enforces canonical/prototype-safe paths, checks before-values sequentially, and returns the original snapshot on any failure. Post-edit validation is atomic. Inverse uses snapshot-to-snapshot diff, preserving array deletions by restoring the containing array. These properties were inspected, not executed.
- Current inference barrel lives beneath `schema/💡️inferences` and exports repository-owned schema/math/render/mutation contracts. DrawingScene originates in the repository's 2D module. Rust runtime dependencies are repository framework crates; listed D3 dependencies are TypeScript devDependencies.
- Catalogue requests intentionally return complete TikZ with a structured scene-unavailable diagnostic and no plan/scene (`💡️inferences/🟦️.ts:31–35`). This is allowed by the shared output contract; it does not demonstrate catalogue numerical scene parity.

## Completion Boundary

This audit establishes static contract observations only. It does not establish full D3 scope, compiled catalogue coverage, numerical equivalence, plugin startup execution, or runtime responsiveness. The retained report is the sole generated audit artifact.
