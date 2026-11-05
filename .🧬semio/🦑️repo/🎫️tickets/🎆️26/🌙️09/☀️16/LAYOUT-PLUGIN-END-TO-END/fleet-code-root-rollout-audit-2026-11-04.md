# Remaining Native Code Root Rollout Audit

Read-only current implementation audit; no edits or gates.

## Concrete Omission

`library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🟨️.mjs:92` reconstructs emitted NativeOwner capability DATA from caller.buildMode, repositoryRoot=context.root, artifactDirectory, command, transport, child and network, but omits mandatory codeRoot. Normal mandatory owner schema therefore refuses its native child wire. Carry admitted caller.codeRoot, preserving distinct Source identity rather than resetting it to execution workspace. Host/root notified.

## Inspected Propagation

Build caller ordinary factory28 and authoring38 carry mandatory codeRoot; captured target NativeOwner factory68 carries capabilities.codeRoot. Native Nx caller factory47 declares it; build Nx envelope29 preserves role.codeRoot. Authority reader50–55 exposes it across both roles. Generic process wire scriptInvocationEnvironment and resource deadline projection preserve capabilities DATA wholesale; lifecycle transport/child projections under publication owner envelope/metadata/initial only extract bounds, do not replace/reemit Native capability DATA and need no codeRoot field of their own.

Tracked Rust/Go production search for Native owner command, repositoryRoot/workspaceRoot/artifactDirectory and caller policy fields found no closed Native role deserializer/reconstruction beyond TypeScript/MJS owners; dashboard Rust only intentionally drops full invocation environment for detached daemon births. Cargo metadata workspace_root records are compiler graph facts, not Native callers, and must not gain codeRoot. Generic capability Value carriers need no structural change. No invented bridge or broad workspaceRoot replacement recommended.


## Complete Explicit Projector Recheck

Read-only AST inspection of tracked production TS/JS/MJS object literals under framework, excluding tests/generated sources, matched explicit workspaceRoot+native or repositoryRoot+command/artifactDirectory constructors. All current actual Native shapes carry codeRoot:

- Native Nx caller factory47.
- Native owner authority projection invocation55.
- Nx plugin Native wire92 (fixed first omission).
- Build orchestration projector33 (host fixed second omission).
- Ordinary build caller factory34; authoring caller44; captured target Native owner74.
- Build Nx child envelope29.

One syntactic match without codeRoot is tools/sdk27, which is an SDK object exposing native transport methods, not Native invocation capabilities (no arguments/selection/buildMode/grants); no field addition needed.

Spread projectors preserve complete existing Native DATA: selected Nx envelope53–60 spreads original capabilities and replaces only selection; neutral build child56 spreads caller then replaces arguments/selection/launches; service declaredCapabilities153–162 destructures only service then spreads rest/launches. Generic process envelope serialization projects policy/deadline while preserving carried capabilities; lifecycle bounds-only views do not reemit Native capabilities. Read-only key-filter/fromEntries/JSON replacer search found no additional Native property whitelist that drops codeRoot. Graph receipt declarations and workspace inference contexts are separate DATA, not caller reconstruction. Current recheck finds no third explicit projector omission. No gates or edits.


## Private Compiler Infrastructure Source Trace

Read-only current path trace found no additional concrete missing framework Source locator in requested chain. withRepositoryCompilerStoreOwned passes mutable storage-backed paths to native compiler. withRepositoryNativeCompilerV1 sourcePlan additional input is fileURLToPath(import.meta.url); core nativeCompilerSourcePlan115 uses its own import.meta URL/schema plus actual selected SDK command/include/lib roots, not privateworkspace/framework joins. Native shim writes to admitted private storage n/digest/cc. Unit-cache launcher UNIT_CACHE_SCRIPT14–15 resolves own source by import.meta URL; shims/cache output remain private.

Cargo ingress effective-configuration guard now explicitly resolves framework compiler script from authority.codeRoot. Remaining layout/configuration-view/lock/event paths resolve storage root and are mutable owned outputs, correctly private. Cargo app manifest/config/lock inputs stay private execution workspace, not blanket codeRoot. Rustup pinRoot correction is Medium-owned. No edits/gates and no new infrastructure-path defect sent.

Direct package route confirmation from root supersedes requested codeSuites ambiguity: private-native-birth is correctly owned by package runtime-and-path branch, no caching/build registry entry required or to be re-added.
