# MCP Pre-Admission Producer Identity Boundary

## Concrete Finding

No current public interface supplies an authenticated desired MCP artifact Source/product identity before stageMcpBinaryAsync spawns the build frontdoor. Implementing grouping from argv, a PID, target name, a manifest hash or current mtime freshness would manufacture the missing equivalence proof. No production edit or lock acquisition was made.

The reusable domain-neutral exclusion mechanism is process acquireResourceLease/acquireQueuedResourceLease. It preserves actual per-caller control and cancellation but requires a genuinely materialized lease path and does not itself define producer identity or readiness. The dependency acquireConcurrentLeaseV1 has a ready/join protocol, but its contract is keyed by dependency lease kind and workspace and is entered under the authenticated dependency store; using a pretend dependency kind for an MCP artifact would be an ownership error.

## Current Call Path

- repo library commands/dev DevScript.runMcpOs has this.invocation, but calls serveWarmMcpStdio(root,args,env) without forwarding Original or an issued phase control.
- OS mcp stageMcpBinaryAsync receives root, progress and AbortSignal. Every gateway independently starts repo run @semio-tech/framework-os-mcp-rs:build. WarmMcpLaunch likewise carries no Original/control or desired artifact identity.
- mcpBinaryFreshness validates a sources record containing builtAtMs and source paths through current mtimes. It has no admitted source-byte closure digest, toolchain/mode identity or producer recipe digest. A missing cold publication cannot provide desired identity.
- The normal MCP BuildScript calls neutral buildAdmittedCargoArtifactsV1. CargoAdmissionProviderV1 delegates through repository buildRepositoryCargoArtifacts, which authenticates Original workspace/mode and owns dependency inputs, compiler store, Source boundaries, compilation and publication. That proper path is reached only inside the already-spawned build target.

## Existing Acquisition APIs Are Not an Earlier Join Port

process/native-build/recipe acquireCargoTargetRecipeV1 canonicalizes a compiler recipe and serializes its target under actual compiler store/profile custody. Its private cargoArtifactTargetRecipeV1 input set is the manifest plus ancestor Cargo manifests/locks/config and toolchain files, Cargo-home config, compiler/toolchain identity and selected environment. It does not hash the Rust source closure; its documented freshness owner is Cargo. It therefore cannot certify identical admitted Source merely by being hoisted earlier. The SEMIO_BUILD_OWNER_PID staging value is outside the CARGO_BUILD_ recipe-environment prefix, so that particular caller ownership field is not a recipe input; it still must remain genuine per caller.

repository withRepositoryPublishedCargoExecutableV1 retains a genuine existing compiler store and artifact SH reader and verifies a qualified publication. It neither plans nor produces a missing artifact. CargoAdmissionProviderV1 currently exposes buildArtifacts and withCompilerStore, not an authenticated plan/acquire operation. CompilerStoreView carries sourceBoundary/withCargoInputs/environment, but assembling a separate OS-owned planner from those effects would duplicate the canonical compiler admission/selection protocol.

## Smallest Required Contract

Extend the existing neutral Cargo admission owner with an authenticated artifact planning/acquisition operation, implemented by its existing repository provider. It must derive exact selected source-closure identity and compile recipe/platform/mode/environment identity under the received Original/control, then expose only the owned plan/consume continuation needed for same-product single-producer acquisition. Source changes must invalidate the plan through the existing Source owner checks. No caller-constructed hash may stand in for admission.

The MCP route then forwards its actual Original and issued phase control to that operation before it spawns another build frontdoor. Each waiter keeps its own deadline, cancellation and custody. A waiter cancellation must not terminate a foreign producer; producer cancellation settles its own scope and releases ownership before another waiter can become producer. Joined success requires a matching qualified receipt revalidated by each consumer, never a successful exit or mtime alone. Existing lower leases can implement exclusion once this identity/authority boundary exists.

Schema-first cases for that extension should distinguish equal Source/product plans, changed Source/toolchain/mode/recipe, missing/stale publication, canceled waiter, canceled producer and re-admission, producer failure, and competing distinct products. A SQLite-backed independent state-transition oracle and genuine inherited two-process acquisition test can prove single publication without PID caches or synthetic Original. This is a precise contract plan, not an implemented coalescer or a request to relax policy.

## Inspected Source Owners

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📡️stdio/🚪️gateway/🟦️.ts
- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts
- process 🧪️testing/🦀️cargo/🔌️provider/🟦️.ts and repository 🏃️process/🦀️cargo/🔌️provider/🟦️.ts
- process native-build/🎯️recipe/🟦️.ts and native-build/🟦️.ts
- repository caching artifacts native-build/🟦️.ts and 📥️published/🟦️.ts
- dependency 🚦️concurrent/💾️storage/🟦️.ts and process 🔒️leases/🟦️.ts

The actual managed restoration 87807 and 42-read recovery remain higher-priority execution work; no new heavy job was started for this investigation.
