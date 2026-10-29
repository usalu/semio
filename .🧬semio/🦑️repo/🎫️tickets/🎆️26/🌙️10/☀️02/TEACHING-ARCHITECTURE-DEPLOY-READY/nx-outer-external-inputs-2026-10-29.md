# Outer Build Selection External Inputs — 2026-10-29

Persistent serve-release owner44094 exited1 before target execution: Nx could not find externalDependency nx for @teaching/proctor:serve-release. The actual published Nx graph had458 external nodes and no npm:nx, although both injected private engine input groups requested nx/typescript. The same outer-selection omission subsequently blocked root semantic12990 at ui-rs:generate. Neither target started; no service or course TypeScript success is claimed.

The earlier selected-root fix existed in the bootstrap NxScript branch used only when SEMIO_BUILD_GRAPH_PATH is absent. The normal command runtime now constructs and pins the graph first through loadBuildInvocationGraphV1. That outer path did not supply backend engine roots, so the later Nx branch correctly reused an already-pruned graph.

The neutral BuildBackend interface now exposes optional graphExternalInputs; the Nx adapter declares existing nxBuildEngineExternalInputsV1. Normal command runtime obtains them before either direct or isolated-child planning. The invocation graph loader carries the explicit roots into fresh, reused and ancestor-validation graph loads. Existing exact admitted-root checks and bounded generation normalization remain authoritative; no installed-package supplement, all-global-facts inclusion, mutable receipt edit or hash bypass was added. Other callers may omit a root request; explicitly supplied empty requests remain exact.

Schema-first neutral invocation corpus now covers proctor:serve-release→build and architecture-course:typecheck→ui-rs:generate. The actual outer loader runs with genuine issued phase/control, reads the selected sealed Bun-lock graph, projects it, and executes the qualified Nx native task graph and HashPlanner. SQLite independently closes external dependencies; canonical locked node identities are compared unchanged, unrelated packages stay pruned, and omitted root admission refuses. Existing raw/npm/scoped normalization cases remain.

Baseline after declaring the backend port but before threading it:3pass1fail, exact Pinned build graph lacks external input:nx. Corrected focused suite:4pass0fail364assertions,1.86s; four changed production TypeScript files parse with0diagnostics. Logs: generated/nx-outer-external-red.log and nx-outer-external-green.log. An earlier first test attempt lacked the existing SEMIO_TEST_ARTIFACT_DIR test harness input and failed initialization; it was not treated as causal evidence.

Changed production Source (below repo library):
- 🏗️build/🔌️backends/🟦️.ts
- 🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts
- 🏗️build/🧭️invocation/🕸️graph/🟦️.ts
- 🏗️build/⌨️command/⚙️runtime/🟦️.ts

Existing generation external test owner changed coherently: 🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️stream/🔌️external/{🟦️.ts,🔣️.json,📋️schema.json}. No generated SDK bytes were edited.

Next execution is the same normal qualified persistent release target with standard proctor-dev data and authored course. Focused tests do not substitute for qualified acquisition or actual HTTP/browser readiness.
