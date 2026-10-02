# Inventory Request and Collector Reentrancy

Read-only source inspection, 2026-10-02. No runtime behavior or tests executed. Evidence records the source observed before Root's callback fix. `N` denotes repository-relative `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization`.

## Inventory Request Snapshot

`N/🟦️.ts:259-269` defines inventory request fields. A shallow own-data capture before `sourceAdmissionPrepareOptions` is sufficient to preserve string scalars, workers number and progress function identity against ordinary mutation of the caller's original object. It must happen before prepare because that function already invokes user progress. Capturing only after prepare cannot repair the first callback boundary.

It is insufficient for nested arrays. Copy/freeze `structuralDirectoryNames`; if retaining the complete request, copy each `excludedTreeDigests` record and freeze that roster. The current inventory implementation does not read `excludedTreeDigests` or `baselineCommit`; those fields alone do not create an observed inventory mutation effect. Prefer explicit captured fields actually used to an indiscriminate deep clone of callbacks and class values.

Observed umbrella live reads: line 4602 setup progress; line 4603 workers validation; line 4607 setup progress; lines 4636/4662/4666/4689/4751/4775/4830 progress or cancellation; line 4813 context ticketDir. Workers is validation-only in the inspected inventory function. Ordinary mutable request changes can therefore change which worker value is validated, redirect later cancellation, replace callbacks, and alter retained ticket context relative to prepared admission. These are temporal consistency gaps in the current source, not executed runtime exploit claims.

Use a fresh frozen explicit request capture at function entry, and reference only that local capture afterward. Passing it through preparation then deriving prepared canonical path values gives one stable operation request. Use prepared.progress/cancelFile/ticketDir for admission-related orchestration where canonical path semantics are intended. Preserve separate orchestration fields such as workers. A function reference capture preserves identity but cannot freeze user code or its external state.

## Direct Collector Inputs

`N/🚪️source-admission/📁️io/🟦️.ts:240-292` accepts both caller-owned loaded taxonomy and prepared values. Existing checks establish only matching taxonomy path and presence of input. It does not prove that a structurally fabricated prepared object came from preparation; the private TypeScript interface is not a runtime capability.

At entry it copies repoRoot/scope/cancelFile scalars and detaches opaquePrefixes and generatorOutputRoots. Later callback mutation cannot modify those already detached local rosters. Repository fences and index rows remain shared array references. Index entries are also shared references when added to row state. After callbacks, collector rereads prepared.ticketDir, prepared.taxonomyPath and taxonomy.input.contentHash. It passes live taxonomy to helpers after callbacks. Those helpers recompute exclusions (`sourceAdmissionUntrackedRows:151`, `sourceAdmissionWalk:163`, `sourceAdmissionStructuralDirectories:195`) and read fixed-directory contracts/matcher (`walk:179-180`, structural:215-216`). Mutation of a held taxonomy reference can thus produce mismatched phase vocabularies even though initial opaque and generator rosters are stable. This is a source-reachable effect only for a direct caller that retains those objects across its progress callback.

The regular public inventoryTaxonomySources path loads a fresh taxonomy internally and never exposes it before collection; ordinary options callbacks cannot obtain this local taxonomy reference from the inspected public API. Root's prepared snapshot closure therefore addresses the ordinary public request bug without automatically requiring a second deep clone of internally owned taxonomy.

## Bounded Clean Contract

Make the public operation entry accept ordinary mutable request values and own a detached snapshot. Preparation returns a frozen record with detached/frozen structural roster, immutable Git entry rows and immutable fence roster. Capture the callback itself once. Collection accepts only `(taxonomy, prepared)` and treats both as trusted owner-produced session authority. Freeze prepared nested facts to the contract depth, not just its outer record. This also removes callback-held prepared aliases returned by prepare from the normal operation.

Loaded taxonomy deliberately contains a fresh mutable matcher and detached facts per session; indiscriminate deep freezing or structured cloning of that full object conflicts with that existing matcher contract. Explicitly establish that the owner-produced taxonomy session is exclusively owned for the duration of collection. The direct collector's mutation of that session by its caller is then a violation of a declared ownership precondition, not evidence of a bug in the normal public inventory API. Current readonly types alone do not state or enforce this runtime precondition, so document it in the collector's native docstring and a focused contract test if this is the chosen boundary.

If direct collection must support adversarial callers mutating their taxonomy during progress, detach only the immutable facts used by admission once before callbacks: exclusions, generator roots, fixed nested-Git contract, taxonomy path/hash and repository rows/fences; provide a new private session matcher for those facts. This is a larger explicit API guarantee with independent tests. Do not claim it is already guaranteed by readonly or freeze of prepared.

Do not introduce runtime nominal tokens or compatibility aliases merely to prevent fabricated internal inputs unless the architecture explicitly requires capabilities. The bounded practical contract is stable ordinary request inputs, owned immutable prepared facts, and an exclusively owned taxonomy session. Filesystem races and external state changes remain separate validation/cancellation concerns.
