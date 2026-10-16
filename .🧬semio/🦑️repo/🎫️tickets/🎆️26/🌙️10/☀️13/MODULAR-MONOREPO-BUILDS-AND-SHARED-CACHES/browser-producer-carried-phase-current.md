# Browser Producer Carried Phase Callgraph

2026-10-16 Source census only. No compiler, product, Native, GPU or graph runtime was executed for this report.

The recomputation remains required: `verifyBrowserActorProducerInputsV1` compares the actual executable, JCO package version, factory-owned runtime, physical inputs and freshly bundled compiler module against the surviving original observations. A receipt/hash alone must not replace this check.

## Actual Upstream Paths

| Entry | Current chain | Missing authority seam |
| --- | --- | --- |
| Hub `🌎️hub/📦️packages/🦀️rust/📜️script.ts:172` | `RuntimeGraphScript.run` → `verifyRuntimeFixtureGraphV1(..., hubRuntimeActorPublicationPortV1(root), ..., this.invocation)` → selection owner `observeCurrent(control)` → Hub `loadCurrentDevelopmentActorsV1` → `runtimeDevelopmentActorContextV1` → `runtimeSelectedActorsV1:50` | Graph owns actual Original and diagnostics phase, but the publication port/context currently carries only physical observation DATA/control. |
| Hub document sweep `🌎️hub/🧪️tests/🗂️document-sweep/🟦️.ts:331` | optional `publicationProof` → `loadCurrentDevelopmentActorsV1(root, observation())` → same Hub context | Must carry the actual enclosing operation Original and phase, or explicitly refuse compiler recomputation; never receive an ambient new root. |
| Hub foundation source tests lines459–468 | direct `runtimeSelectedActorsV1` and `loadCurrentDevelopmentActorsV1` missing-ledger/cancellation refusal laws | Preserve refusal-before-compiler behavior while updating required actual ports. |
| Repository runtime discovery tests | direct producer verification with a supplied invocation | Required actual carried phase must join the existing invocation; current structural/data-only observation is not compiler authority. |

## Actual Downstream Compute

`runtimeSelectedActorsV1` → `verifyBrowserActorProducerInputsV1` → `buildBrowserCodegenModule` → `captureBrowserCodegenSources` → `ownedBrowserBundleWorker` → `bundleVirtualJavaScript`. The existing optional invocation path receives ambient `process.env` and the bundler creates a fresh120s phase. Both must be removed together: required explicit actual Original plus actual compiler phase, authenticated before effects, then requested derivative only.

Other actual compute entrances are `buildClosedBrowserActorArtifactV1` (Hub materialization and actual browser tests), `closedBrowserActorBundle` (canonical async-import route), and private test ports into codegen/capture/bundling. Pure finite string/component factory DATA helpers do not gain authority arguments.

Root owns the graph/publication-port/context propagation. This lane owns the required producer/worker signatures and domain callers. No actual graph-context fallback, structural phase, receipt-derived authority or new clock is allowed. Existing scratch allocation/toolchain and process descendant settlement remain unqualified runtime surfaces.
