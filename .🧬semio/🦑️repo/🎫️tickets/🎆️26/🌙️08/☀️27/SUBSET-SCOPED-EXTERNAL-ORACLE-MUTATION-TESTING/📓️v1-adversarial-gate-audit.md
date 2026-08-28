# Adversarial Gate Audit — Repository Test Protocol v2

Independent verifier pass against the central claim:

> No gate can be satisfied by a run in which nothing was measured, and no mutation can read as covered
> unless a QUALIFYING third-party oracle actually produced its expected result.

Scope read in full: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts` (4019 lines,
all regions), `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` (1253 lines, all commands), and
the existing 81-check harness at `🧪️verify/📜️script.ts` in this ticket, to avoid duplicating what it
already proves.

All demonstration scripts referenced below live in this ticket's
`🧪️v1-adversarial-probe/` folder and were executed with `bun run <path>`; their actual output is
pasted inline, not predicted.

---

## CONFIRMED findings, ranked by severity

### 1. [CRITICAL] The dependency ratchet compares the committed baseline against itself — it can never see a new production dependency, in any language

`script.ts` `DependencyScript.run()` (script.ts:892-916):

```ts
const sorted = loadClassifiedBaseline(this.repoRoot);
...
const verdict = ratchetDependencies(sorted, sorted, registry);
```

`ratchetDependencies(baseline, candidate, registry)` is documented as "the shrink-only ratchet: a new
production-reachable external dependency is always forbidden" — but `baseline` and `candidate` here
are the *same array*, both loaded from the committed `🔒️dependencies.json`. `index.ts` exports exactly
the functions that would compute a real, freshly-scanned candidate from the live source tree —
`goProductionClosure`, `pythonRuntimeImports`, `dotnetPackageReferences` — and `script.ts` imports all
three (lines 38, 81) but **never calls any of them, anywhere**. No Rust or TypeScript equivalent is
called either. The only thing `bun ./📜️script.ts dependency` measures is whether the committed file is
internally consistent with the oracle registry — never whether it still matches what production code
actually imports.

Demonstration: `🧪️v1-adversarial-probe/demo8-dependency-ratchet-compares-itself.ts`

```
ratchetDependencies(sorted, sorted, registry) — as script.ts calls it: ok=true, newProduction=[]

ratchetDependencies(sorted, REAL-candidate, registry) — what SHOULD run: ok=false, newProduction=[js:sneaky-eval-based-plugin-loader]
```

The ratchet function itself is sound (second line) — proven by feeding it a genuinely different
candidate. The vacuous pass is entirely in how `script.ts` invokes it: `newProduction` and
`unregisteredTestDeps` are provably always `[]` given identical inputs, regardless of what a developer
adds to `package.json`, `Cargo.toml`, `go.mod`, or `requirements`. `oracleImportsInProduction` (run by
`test contract`, a *different* command) separately grep-scans for imports of *registered oracle*
packages specifically — it does not cover the general "any new production dependency" case this
ratchet exists for.

### 2. [CRITICAL] `optional: true` is not restricted to unqualified probes — it silently excuses a fully qualified probe's hard failure

The `ComparisonStage.optional` docstring (index.ts:3075-3076) states: "A stage whose probe is not yet
qualified. It RUNS and REPORTS; no release gate may claim its guarantee." Nothing in `evaluatePipeline`
or `evaluateStageAssertions` checks that a stage marked `optional` actually uses an unqualified probe.
`optional` is a free boolean the pipeline author sets per stage.

Demonstration: `🧪️v1-adversarial-probe/demo7-optional-excuses-a-qualified-gating-stage.ts` — a stage
using a probe whose `qualification.status === "qualified"` (with a real `criteria` entry) is marked
`optional: true`, and its probe reports a hard `status: "failed"`:

```
pipeline.equal despite the QUALIFIED probe reporting status="failed": true
unqualifiedStages (should be empty — the probe IS qualified): []
the failing verdict itself: {"stage":0,"probe":"real-qualified-probe","key":"status","expected":"ok","actual":"failed","ok":false,"optional":true,"reason":"probe reported failed"}
```

This is the sharpest possible violation of the central claim: a QUALIFYING third-party probe reported
failure, and the pipeline still reads `equal: true`. The safety property described in the docstring is
a comment, not code — and (finding 7 below) no validator exists for `ComparisonPipeline` records at
all, so nothing would reject this shape at ingest either.

### 3. [CRITICAL] A capability with no `MutationManifest` is invisible to the v2 release gate, not merely "missing"

`buildCoverageMatrix` (index.ts:3816) enumerates exclusively `for (const manifest of
registry.mutationManifests)`. An owner who never authors a v2 `MutationManifest` for a capability —
while a *different* capability elsewhere in the registry is fully, correctly covered — contributes
**zero rows** to the matrix. Because the six release-gated dimensions' denominators are pooled across
the whole registry (not per-owner), the other capability's real coverage keeps every denominator
non-zero, so `enforceReleaseGates`'s explicit "empty denominator fails" guard (index.ts:3963, which
*does* work correctly — see the "attacked and could not break" section) never fires.

Demonstration: `🧪️v1-adversarial-probe/demo1-manifest-omission.ts` — a registry with one fully-covered
manifest (`demo.visible`, complete fixture, oracle, runtime inventory, dispatch-verified result) and a
second, entirely undeclared mutation (`hidden-owner`'s `delete-everything`, never given a
`MutationManifest`):

```
rows in coverage matrix: 1
rows mention "hidden-owner": false
...
release gate failures: 0

>>> VACUOUS PASS CONFIRMED: enforceReleaseGates reports ZERO failures while a whole capability with an
un-oracled, unmeasured, un-inventoried mutation exists in production and simply has no MutationManifest
entry.
```

Root cause, structural: `mutationCatalogs` (v1's kind/vector completeness vocabulary, checked by
`mutationCoverageBreaches` via `@mutations-<id>`) and `mutationManifests` (v2's oracle/dispatch/outcome
vocabulary, the only thing `buildCoverageMatrix` reads) are **two independent registries with no
cross-validation**. `validateAllContracts`'s `unregistered-mutation-vocabulary` check
(index.ts:1778-1791) only requires a physical `🧬️mutations` tree to have a matching `mutationCatalogs`
entry — it never requires a matching `mutationManifests` entry. An owner can therefore be 100%
"v1-complete" (every `mutate-<kind>`/`inverse-<kind>` scenario present) while being *totally absent*
from `test matrix --enforce`, the actual v2 release gate.

### 4. [HIGH] `buildCoverageMatrix`/`measureCoverage` trust `registry.mutationManifests` with zero validation — a contract-invalid mutation reads as fully oracle-covered

Unlike `contribution.fixtureManifests` (whose invalidity **does** reduce `fixtureProvenanceCoverage` —
see the contrast noted below), nothing calls `mutationManifestProblems` before `buildCoverageMatrix` or
`measureCoverage` consume a manifest. A `ManifestMutation` with `oracleRequirements: []` is
contract-invalid (`mutationManifestProblems` returns `"oracleRequirements must name at least one
qualifying capability"`), but `measureCoverage`'s `externalOracleCoverage` computes
`mutation.oracleRequirements.every(...)` — vacuously `true` on the empty array.

Demonstration: `🧪️v1-adversarial-probe/demo2-empty-oracle-requirements.ts`, registry with **zero
oracles registered anywhere**:

```
mutationManifestProblems(brokenManifest):
  - mutations[0].oracleRequirements must name at least one qualifying capability

externalOracleCoverage: 100.0% (1/1), missing=[]
oracleRequirementBreaches on the same mutation directly: 0 breach(es)
```

`oracleRequirementBreaches` (used by `test contract`) also emits **zero breaches** for this same
mutation, because `mutationInventoryBreaches` `continue`s past a manifest the moment
`mutationManifestProblems` finds any problem (index.ts:3354-3358) — it never runs
`oracleRequirementBreaches` for it either. So the same defect is invisible to *both* `test contract`
and `test matrix --enforce`, via two different mechanisms.

Contrast that closes the gap conclusively: `fixtureProvenanceCoverage`'s `withoutProvenance` **does**
filter on `fixtureManifestProblems(fixture).length > 0` (index.ts:3913, 3937) — proving the pattern
"exclude contract-invalid records from the coverage numerator" was known and applied to fixtures. It
was simply never applied to mutation manifests.

### 5. [HIGH] A subset that never runs `test inventory` is invisible to `runtimeMutationCoverage`, not reported as missing

`MatrixScript` (script.ts:1172) builds its inventories list as
`registry.mutationManifests.map(readRuntimeInventory).filter(non-null)`. `measureCoverage`'s
`runtimeMutationCoverage` denominator is `runtimeIds.length`, derived *only* from inventories that
exist in that filtered array. A manifest whose owner never ran `bun ./📜️script.ts inventory` for that
subset contributes **nothing** — not even a "missing" entry — to this dimension, as long as some other
manifest elsewhere was inventoried (giving a non-zero pooled denominator).

Demonstration: `🧪️v1-adversarial-probe/demo3-uninventoried-subset.ts` — two manifests, identical shape,
identical oracle backing; only one has a runtime-inventory file:

```
runtimeMutationCoverage: 100.0% (1/1), missing=[]
does "delete-everything" or "demo.never-inventoried" appear ANYWHERE in the measurement? false
runtimeMutationCoverage gate failure: (none — gate is satisfied)

For comparison — what `test contract` sees for the SAME manifest: compareInventories(...).runtimeMissing = true
```

`test contract`'s `mutationInventoryBreaches` (via `compareInventories`) correctly flags
`runtime-inventory-missing` for the un-inventoried manifest — but that is a separate command with a
separate exit code from `test matrix --enforce`, the command that actually names itself the release
gate.

### 6. [HIGH] `computeRunKey` is blind to `OracleEntry.kind` and `ProbeEntry.qualification.status`

`RunKeyComponents` hashes `oracleLockDigest` (package@version or `lockDigest`), `oracleEngineDigest`
(engine family@version), and `probeDigest` (package@version or `lockDigest`) — never the oracle's
`kind` (the single field `isQualifyingOracleKind` reads) or the probe's `qualification.status` (the
single field `isQualifiedProbe` reads).

Demonstration: `🧪️v1-adversarial-probe/demo6-runkey-blind-spots.ts` — same package/version/engine,
different `kind`/`qualification`:

```
run key with oracle.kind="third-party-library":       0be7153b198c2de4540869c71ce9bc1a
run key with oracle.kind="cross-semio-implementation": 0be7153b198c2de4540869c71ce9bc1a
run key with oracle.kind=undefined:                    0be7153b198c2de4540869c71ce9bc1a
all three identical: true

run key with probe qualification="qualified":  c90ea6bb9256b668dcc86cfb7220e8d6
run key with probe qualification="provisional": c90ea6bb9256b668dcc86cfb7220e8d6
run key with probe qualification="rejected":    c90ea6bb9256b668dcc86cfb7220e8d6
all three identical: true
```

Consequence: a parity result cached while an oracle was registered as a fully qualifying
`third-party-library` survives byte-for-byte if that same registry entry is later reclassified to
`cross-semio-implementation` or to no `kind` at all — the cache has no way to know it must
re-execute, because the key that addresses it never changed. Symmetrically, a comparison run while a
probe was `provisional` (non-gating) is keyed identically to the same comparison after the probe is
promoted to `qualified` — so promoting a probe does not, by itself, force any stage that ran under it
to be re-verified.

### 7. [MEDIUM] No contract validator exists for `ComparisonPipeline`, `ProbeEntry`, or `ToleranceProfile` at all

Confirmed by exhaustive grep across index.ts: `mutationCatalogProblems`, `mutationManifestProblems`,
`fixtureManifestProblems`, and `probeReportProblems` all exist and are wired into
`validateAllContracts`/`fixtureProvenanceBreaches`. No `comparisonPipelineProblems`,
`probeEntryProblems`, or `toleranceProfileProblems` function exists anywhere, and `readContribution`
(index.ts:818-846) casts `parsed.comparisonPipelines`, `parsed.probes`, and `parsed.toleranceProfiles`
directly with no shape check of any kind.

Two direct consequences, demonstrated in `🧪️v1-adversarial-probe/demo4-empty-pipeline-and-fake-probe.ts`:

```
(a) empty-stage pipeline: equal=true, verdicts=0
(b) ghost-probe pipeline: equal=true, missingProbes=[ghost-probe-nobody-registered]
```

(a) A `ComparisonPipeline` with `stages: []` is the literal empty-collection `.every()` vacuous pass
the audit was asked to hunt for (`verdicts.every(v => v.ok || v.optional)` on an empty array is `true`
by definition — index.ts:3148). (b) `missingProbes` is computed by `evaluatePipeline` but never folds
into `equal`; a stage naming a probe that exists in no probe table still evaluates a fabricated
`ProbeReport` as if it were legitimate.

### 8. [MEDIUM] `productionDispatch.invoked` and probe `qualification` are bare self-reported fields, verified nowhere

Per item 8 of the brief: **what this does and does not prove.** It proves the *shape* of a claim is
present; it proves nothing about whether the claim is true. `vectorReplayBreaches` and the
`productionBridgeCoverage` dimension read `result.productionDispatch?.invoked === true` with no
independent check — no trace id, no cross-reference against a manifest mutation id that actually
exists, no signature. Symmetrically, `isQualifiedProbe` reads `qualification?.status === "qualified"`
with no check that `criteria` exist or are all `met: true`, nor that `evidence` says anything
substantive.

Demonstration: `🧪️v1-adversarial-probe/demo5-fake-dispatch-and-fake-qualification.ts`:

```
(a) vectorReplayBreaches on a fully fabricated dispatch claim: 0 breach(es)
    productionBridgeCoverage counts it as dispatched: 100% (1/1)
(b) isQualifiedProbe(selfCertified with 0 criteria, evidence="x"): true
```

The fabricated result in (a) claims `mutation: "a-mutation-id-that-appears-in-no-manifest-anywhere"`
and `operation: "totally-made-up-operation"` — neither is cross-checked against anything.

---

## PLAUSIBLE hypotheses (not fully demonstrated as exploits)

- **NaN tolerance propagation.** `resolveTolerance`/`resolveToleranceProfile` compute
  `Math.max(absolute, relative * Math.abs(reference))`; if `reference` (e.g. `diagonal`) is `NaN` —
  plausible from a degenerate upstream geometry calculation — the result is `NaN`, confirmed directly:
  ```
  {"length":null, ...}   // NaN silently serializes to JSON `null`
  length is NaN: true
  ```
  I could not find, within index.ts/script.ts, the call site that would consume a resolved tolerance
  and decide pass/fail from it (that logic appears to live in a host/adapter file outside the reviewed
  scope), so I cannot show whether a `NaN` bound reads as an automatic pass or an automatic fail
  downstream. Flagging because the JSON-serializes-to-`null` behavior would make a `NaN` tolerance
  silently indistinguishable from "field absent" in any cached/reported artifact.
- **`noOracleMisuseBreaches` inherits the manifest-omission blind spot.** `mutationCapabilities` is
  built from `registry.mutationManifests` (index.ts:3451) — the same set finding 3 shows can be
  incomplete. A no-oracle decision naming a capability that is real in production but never manifested
  would not be flagged as "covers a mutation capability," for the same structural reason as finding 3.
  Not separately demonstrated; it is a corollary of finding 3 rather than an independent bug.
- **`oracleImportsInProduction`'s pattern-based import probes** (index.ts:2069-2084) are regexes per
  ecosystem; a sufficiently indirect import (dynamic `require(variable)`, reflection, a re-export
  chain) would not match any `importProbe` pattern. I did not attempt to construct a working bypass
  file and run the real scan against it, so this remains a hypothesis rather than a demonstrated gap.

---

## Attacked and could NOT break

- **`enforceReleaseGates`'s empty-denominator guard** (index.ts:3963) for the six release-gated
  dimensions genuinely works: when I first built demo 1 with *no* fixtures/inventories at all, it
  correctly failed with `"has an EMPTY denominator — nothing was measured"` for every gated dimension
  with `total === 0`. I had to give the "visible" side full, real coverage before the manifest-omission
  bypass (finding 3) became visible — the guard is not vacuous for the case it was written for.
- **`compareInventories`** (runtime/manifest/test three-way diff): tried runtime-only, manifest-only,
  test-only, outcome-mismatch, variant-mismatch, and missing-runtime cases beyond what the existing
  harness covers; all detected correctly, matching the harness's own thorough coverage of this
  function.
- **`fixtureProvenanceCoverage`**: unlike `externalOracleCoverage` (finding 4), an invalid fixture
  manifest *does* correctly reduce this dimension's numerator — confirmed by reading
  `measureCoverage`'s `withoutProvenance` filter and cross-checking against the harness's
  `fixture/all-valid` checks.
- **Lease reclamation** (`leaseReclaimable`): tried an agent's own stale lease (never reclaimed), a
  live peer process with a stale heartbeat (never reclaimed), a `failed`-state lease (never
  reclaimed, by design — failure evidence outlives cleanup), and dead+stale+foreign (correctly
  reclaimable). All matched the harness's existing checks; found no additional bypass.
- **GC symlink/path-escape guard** (`collectGarbage`'s `resolvedCacheRoot`/`resolvedMetaRoot` check):
  reasoned through the realpath-on-both-sides comparison; it closes the exact TOCTOU-style bypass
  (comparing a resolved path against an unresolved literal) the code comment says it was fixed for
  I did not find a variant that reopens it within the code as written.
- **`withAtomicRunDir`**: rename-based publish with a `.creating` suffix; an interrupted `body()`
  leaves only the uniquely-suffixed temporary directory, never a half-written final one. Consistent
  with the harness's `lease/interrupted-leaves-no-published-dir` check; found no race.

---

## Demonstration scripts (all executed, output pasted above)

All under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/🧪️v1-adversarial-probe/`:

- `demo1-manifest-omission.ts` — finding 3
- `demo2-empty-oracle-requirements.ts` — finding 4
- `demo3-uninventoried-subset.ts` — finding 5
- `demo4-empty-pipeline-and-fake-probe.ts` — finding 7
- `demo5-fake-dispatch-and-fake-qualification.ts` — finding 8
- `demo6-runkey-blind-spots.ts` — finding 6
- `demo7-optional-excuses-a-qualified-gating-stage.ts` — finding 2
- `demo8-dependency-ratchet-compares-itself.ts` — finding 1

Each is self-contained, imports only from the reviewed `📦️index.ts`, and can be re-run with
`bun run <path>`.
