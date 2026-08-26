# S-Core Engine

## Outcome

Implemented the repository-owned, zero-external-runtime taxonomy normalization engine in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` against pinned baseline `9f449b10659b95148c8bcb3f91ce583bf7446973` and taxonomy schema v7.

The final module is 2,220 lines / 128,948 bytes. Its SHA-256 is:

```text
ae588c254b7125b9ac7ee541bc6692d8461bf07e7fefdd86954eba7195ed6da8
```

## Public API

The module exports the normative records `TaxonomyInventoryEntry`, `TaxonomyMove`, `ReferenceEdit`, `TaxonomyPlan`, `TaxonomyViolation`, `TaxonomyRegeneration`, `OpaqueTreeDigest`, verification, progress, journal, option, failure-stage and apply-result records. The implemented entry points are:

```ts
inventoryTaxonomy(options: TaxonomyInventoryOptions): TaxonomyInventory
planTaxonomy(inventory: TaxonomyInventory, options: TaxonomyPlanOptions): TaxonomyPlan
applyTaxonomyPlan(plan: TaxonomyPlan, options: TaxonomyApplyOptions): TaxonomyApplyResult
verifyTaxonomy(options: TaxonomyInventoryOptions): TaxonomyVerification
canonicalJson(value: unknown): string
taxonomyPlanDigest(plan: TaxonomyPlan): string
opaqueTreeDigest(root: string, relativeRoot: string): OpaqueTreeDigest
```

`taxonomyPlanDigest` hashes canonical plan bytes with `planDigest` omitted. No deterministic artifact contains a generated timestamp.

## Implemented behavior

- Loads and strictly validates v7 physical `fileKinds`, contextual `fileKindResolutionRules`, ticket-scoped file kinds, global and exact-owner semantic directory registries, structured fixed filename/directory contracts, configurable contracts, recursive package boundary/glue grammar, Unicode, VS16, collision and area-enforcement records.
- Inventories present cached and read-only untracked Git paths plus explicitly admitted active-ticket paths. Source spellings retain their original Unicode bytes; normalized destinations use NFC and required VS16.
- Applies lexical opaque exclusion before filesystem metadata/content access, never follows symlinks, and retains `compose` as a registered exclusion when absent. A digest is required only while the excluded root exists.
- Resolves opaque exclusions first, then most-specific fixed contracts, scoped kinds and schema-owned physical extension rules. Fixed names and directories are preserved; equal-specificity matches block. Exact semantic IDs, parent context and nearest-owner member overlays establish directory context without guessed emoji mappings.
- Uses longest physical extension chains, generic-stem drops, legacy `.test` source-role stripping into semantic test directories, contextual test/asset children and schema-owned package-language boundaries. Extensionless files use fixed contracts first, then unambiguous shebang detection or deterministic UTF-8 text/binary sniffing into registered `.txt`/`.bin` kinds; contradictory shebangs block.
- Proves package roles with Rust, TypeScript, Go, Python and .NET grammars. Proven implementation is extracted beside the package owner; an uncertain role or destination blocks the plan.
- Detects byte, NFC, case-fold, VS16-fold and same-kind collisions across files/directories, along with UTF-8 path budget, Windows-reserved and trailing-dot/space hazards.
- Produces structured Rust, TypeScript/JavaScript, Go, Python, .NET, native/CMake, JSON, JSONC, TOML, YAML, XML and Markdown reference edits. Every edit carries adapter, structured offset location, old/new value and preimage hash; no global text replacement is used. Incoming references to binary targets block only when a structured source reference is unaccounted; generated targets remain fail-closed pending an explicit regeneration contract.
- Precomputes exact, NFC, extensionless and Python-module reference indexes, and hashes directories through a parent-to-children index. Reference lookup and directory hashing therefore avoid repository-wide scans per token/node.
- Applies collision-safe two-phase staged moves under `<ticket>/🧾️taxonomy-transaction/🔖️<plan-digest>/`, retaining the kind-only `🔣️.json` journal while removing staging and backup trees after commit. It verifies per-file backups, mode preservation, regeneration output backups, cancellation and in-progress journal resume, and rejects any plan mutation path overlapping the semantic transaction root.
- Supports injected failures at `after-staging`, `after-moves`, `after-edits` and `before-verify`; every mutation is rolled back in two phases, including swap/cycle moves. Transaction scratch data is removed only after successful post-state and opaque-digest verification.
- Leaves the Git index untouched. Post-apply convergence comes from read-only cached-plus-untracked worktree discovery.

## Opaque-tree safety

`opaqueTreeDigest` is an explicit SHA-256 Merkle operation. It hashes directory entries, regular-file bytes and symlink target text without following the target. Normal inventory never calls it implicitly. Apply accepts only digests whose lexical root is registered in schema v7, compares them before and after mutation, and treats an absent registered root as valid.

No development command for this packet traversed, inventoried, moved, restored or digested the real `compose/` or `temp/compose` trees.

## Verification evidence

Module import and Bun bundling:

```text
$ bun build 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts --target bun --outfile /dev/null
Bundled 15 modules in 14ms
exit 0

$ bun -e '<import module and assert frozen exports>'
missing=[]
exports=[applyTaxonomyPlan,canonicalJson,inventoryTaxonomy,opaqueTreeDigest,planTaxonomy,taxonomyPlanDigest,verifyTaxonomy]
```

The strict TypeScript entry check reports no diagnostics in the engine or discovery module. Its only remaining diagnostics are two pre-existing `ImportMeta.env`/`ImportMeta.glob` declarations in the transitively imported styling package.

Final scoped runtime inventory:

```text
elapsedMs=2016
entries=7
violations=0
pathExclusions=[compose]
activePathExclusions=[]
inventoryDigest=2df01ea324c3e5a0112b9651cbbc1546c82fb53f3c257e2e64ea8abd78263d6c
```

Full non-Compose performance probe after the asymptotic fixes:

```text
directories=37,987
files=64,721
referenceCandidates=48,336
entries=102,708
elapsedMs=60,897
pathExclusions=[compose]
activePathExclusions=[]
```

This full probe was a census/readiness measurement before the final physical-leaf registry collapse; its counts and timing are retained as deterministic performance evidence, while the final-schema acceptance result is the scoped inventory above.

Focused language-agnostic and third-party-oracle suite against the final physical-leaf schema:

```text
$ bun test 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts -t '^taxonomy normalization'
14 pass
196 filtered out
1 fail
169 expect() calls
```

The 14 passing cases cover canonical bytes/digests, no-follow opaque symlinks, absent exclusions, deterministic census versus `fast-glob`, exact D3D12/PDF/BMP destinations, Unicode/collisions/platform hazards, all reference adapters, recursive package purity, stale preimages, four injected failure stages, cancellation and successful empty-second-plan convergence. The sole shared-test failure is the superseded expectation that `component.test.ts` normalize to role-special `🧪️.test.ts`; the final schema intentionally emits physical TypeScript `🟦️.ts` beneath semantic test directories. Updating that shared assertion belongs to `S-TEST-PHYSICAL-LEAVES`, not this engine-owned lane.

Canonical self-digest smoke:

```text
canonical={"a":2,"b":1}
digest=9b1b601c903c4696d56ca871824273264be13c9abe8e36f252882ff8417c8d3f
stable=true
```

## Acceptance checks

- [x] Normative v7 inventory/move/edit/plan shapes and frozen function signatures exported.
- [x] Opaque filtering precedes admitted-path metadata/content access; symlinks are never followed.
- [x] Source spelling, NFC, VS16, contextual semantic ownership and physical longest extension chains covered.
- [x] Structured fixed file/directory, scoped/configurable and exact-member contracts fail closed.
- [x] Extensionless content sniffing preserves fixed names and blocks ambiguous shebangs.
- [x] Every supported structured adapter emits preimage-verified edits.
- [x] Collision/platform/path-length groups block unresolved plans.
- [x] Canonical plan digest, expected post-state digest and opaque Merkle digest are deterministic.
- [x] Canonical semantic transaction paths, two-phase moves, journal/resume, scratch cleanup, backups, cancellation, injected failure and rollback verified.
- [x] Successful apply converges to an empty second plan without modifying Git state.
- [x] No TODO, FIXME or placeholder remains in the engine.

## Touched paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️s-core-engine.md`
