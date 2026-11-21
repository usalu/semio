# Descriptor Pipeline Alternatives

No existing supported command was found that selects only Note and builds the descriptor emitter as a prerequisite in the same warm browser component pipeline. No build or code change was performed in this audit.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts::runRepositoryComponentDriverV1` lines 81–83 skips the installed emitter capability check only when a plan contains the descriptor emitter’s managed build job and makes `describe` depend on it. The root `📋️project.json` release pipeline lines 2196–2222 declares precisely such a `describe-tool` job and dependency. The driver retains the compiler store, product, descriptor, and browser writer ownership paths.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts::ReleaseScript` lines 69–73 accepts no arguments and calls `buildReleasePipelineV1` without a component selection, so the exposed release driver selects every discovered component. Although the internal `buildReleasePipelineV1` accepts a component set, this caller does not supply one.

The existing `components iterate <plugin-id>` entry (`release/🟦️.ts` line 56) selects one plugin through `buildComponentClosureV1`, whose `componentPassPlanV1` plan has no pipeline jobs or `stageAfter`; the driver therefore checks the installed emitter before proceeding. `components describe` explicitly awaits `ensureDescriptorEmitterV1` before running its driver. Neither supplies the requested integrated Note-only emitter prerequisite.
