# 🧬️ Repository Test Protocol v2 — Specification

Ticket `2026/08/27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING`. Baseline
`a8d1caf41f68204e73ff5e47ce40c5f543ed442d`. Everything below was read out of the implementation, not
proposed — every claim cites the file it came from. Owner: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

The v1 platform this replaces is described in
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️w0-baseline.md`
(G1–G5). It is cited inline below wherever v2 closes a specific v1 gap.

## 1. The unit of coverage

`🧬️schema/🔣️.json`'s top-level `description` states it directly, and `📈️CoverageV2` in
`📦️packages/🟦️typescript/📦️index.ts:3766` (`CoverageRow`) implements it literally as one report row:

```
artifact × standard × smallest owning subset × runtime mutation × expected outcome class
  × fixture × subject implementation × platform
```

`buildCoverageMatrix` (`index.ts:3801`) enumerates this matrix from the **manifests**, never from
results — a mutation nobody wrote a test for becomes a `status: "missing"` row instead of vanishing
from the denominator (`index.ts:3827`, comment at `3796`). Sharding or reporting at artifact level is
what `CoverageRow`'s own doc comment (`index.ts:3766`) says the shape exists to prevent.

## 2. Mutation manifests and the three-way equality gate

**Why v1 could not do this.** `📓️w0-baseline.md` G3: v1's `mutationCoverageBreaches` compared
`catalog.kinds` against scenario ids, and its own source comment said outright "Runtime mutation kinds
are deliberately not consulted." Production already exposed an inventory
(`protocol::SemanticMutation::kinds()`, `MutationLeafDescriptor`) that v1 never read, so a mutation
reachable through production dispatch but absent from a catalog left **no trace anywhere**.

v2 closes this with three independent sources that must agree exactly:

| Source | Produced by | Schema |
| --- | --- | --- |
| Manifest | The domain owner, handwritten in `🧪️oracle/🔣️.json` | `MutationManifest` (`schema/🔣️.json` `$defs.MutationManifest`) |
| Runtime | Running the owner's production bridge | `RuntimeMutationInventory` (`$defs.RuntimeMutationInventory`) |
| Test | The claimed test catalog kinds | plain string ids |

`compareInventories` (`index.ts:2671`) diffs all three and returns `InventoryEquality`
(`runtimeOnly`, `manifestOnly`, `testOnly`, `outcomeMismatches`, `variantMismatches`,
`runtimeMissing`). `mutationInventoryBreaches` (`index.ts:3333`) turns every non-empty field into a
blocking `BreachRecord`, including the case where no runtime inventory was ever produced
(`runtimeMissing` — "Runtime completeness is a measurement, not a claim").

The runtime side comes from a **language-neutral production mutation bridge**: a plain executable at
`🏭️bridge/📜️script.ts` beside the owner, discovered by walking up from the owner
(`mutationBridgeFor`, `📜️script.ts:326`) so a subset inherits its artifact's bridge without its own
copy. `InventoryScript` (`📜️script.ts:979`, command `test inventory`) runs it, validates
`schema === "semio.repository-test.runtime-inventory/v2"`, and writes the result under
`.🧬semio/🦑️repo/⚡️cache/tests/🏭️inventory/` (`runtimeInventoryPath`, `index.ts:2631`) — **cache
state, never committed source**, so a stale checked-in copy can never be mistaken for what the runtime
offers today.

The BRep pilot's bridge, `✏️s/…/📐️step/…/✳️cc6/🏭️bridge/🦀️component.rs`, is the concrete pattern: it
enumerates `StepCc6Mutation::every_variant()` exhaustively by construction (a new enum variant fails to
compile until added there) and reports the outcome classes each kind can reach, read from the class
guard's own behaviour rather than restated by hand. Dispatch equality is **name-checked, not
order-checked**: `ManifestMutation.productionDispatch.variant` (`schema/🔣️.json`
`$defs.ManifestMutation`) must name the exact enum variant, and `compareInventories` flags a
`variantMismatches` entry the moment a variant is renamed under the manifest (`index.ts:2687`).

## 3. Subset ownership — no wildcards

`SubsetTarget` (`schema/🔣️.json` `$defs.SubsetTarget`, `index.ts:2442`) is `{artifact, standard,
subset, compound?, selector?}` with **no wildcard field representable at all**.
`WILDCARD_SUBSET_IDS = ["*", "any", "all", "unconstrained", ""]` (`index.ts:2451`) and
`isWildcardSubset` reject them wherever a subset id is read: `mutationManifestProblems`
(`index.ts:2558`), `fixtureManifestProblems` (`index.ts:2767`), and `mutationInventoryBreaches`'
`wildcard-subset-owner` breach (`index.ts:3373`).

A cross-subset operation declares an explicit typed `compound: string[]` (minItems 2) — "falling back
to the whole artifact is a contract failure rather than a default" (`schema/🔣️.json`
`$defs.SubsetTarget.compound` description). `subsetCoordinate` (`index.ts:2459`) renders a compound as
`artifact@standard/(subset-a+subset-b)`, sorted, so the coordinate is stable regardless of declaration
order.

**Known outstanding wildcard** (not yet migrated, per `📓️w0-baseline.md` G4): the CAD artifact's
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/🔣️component.json` declares one
subset keyed `"*"` owning all 20 `CadMutation` variants. `subsetOwnershipCoverage` (one of the 16
dimensions, §10) reports this as missing until it is split.

## 4. Oracle qualification

Two disjoint kind families (`schema/🔣️.json` `$defs.QualifyingOracleKind` /
`$defs.SupplementalOracleKind`, `index.ts:2478`/`2487`):

| | Kinds | Discharges a mutation's oracle requirement? |
| --- | --- | --- |
| **Qualifying** | `third-party-library`, `third-party-cli`, `standards-reference-tool` | Yes |
| **Supplemental** | `metamorphic`, `inverse`, `round-trip`, `property`, `cross-semio-implementation` | No, never |

`isQualifyingOracleKind` (`index.ts:2493`) is the single predicate every gate consults.
`oracleRequirementBreaches` (`index.ts:3395`) requires at least one **qualifying** oracle per
`OracleRequirement.capability`, and independently counts **engine families**
(`EngineFamily.family`, `$defs.EngineFamily`) among the qualifying set against
`distinctEngineFamilies` (default 1) — "Two wrappers around one kernel are one oracle, not two"
(`schema/🔣️.json` `$defs.EngineFamily` description).

`cross-semio-implementation` is registered as a **required supplement, never a substitute**: both
halves of a second in-repo implementation read the same specification, so a misreading produces two
agreeing wrong answers (`index.ts:3389` doc comment, and `📓️w0-baseline.md` G5, which found exactly
this pattern in the CAD subset's `cad-python-independent` oracle — "the SUBJECT half does not run this
subset's codec … replays the committed vectors"). `noOracleMisuseBreaches` (`index.ts:3435`) further
forbids a `NoOracleDecision` from covering any mutation capability at all —
`NoOracleDecision.coversMutations` is schema-pinned to `const: false`
(`schema/🔣️.json` `$defs.NoOracleDecision`) — v2 keeps no-oracle decisions only for non-mutation
capabilities.

`engineIndependenceBreaches` (`index.ts:3517`) is the closing check: if every qualifying oracle for a
capability sits on the **same** engine family as the subject's own declared engine, the comparison is
circular ("the reference and the code under test would agree on the kernel's own defects") and the
finding is reported rather than silently accepted.

**Worked example** — `✳️cc6`'s `🧪️oracle/🔣️.json` registers two oracles on different engine families:
`ruststep-step-ap214-cc6-mutate` (`kind: third-party-library`, `engine.family: stepcode-independent`,
a pure-Rust EXPRESS/Part-21 reader with **no writer**, used for structural/inverse evidence) and
`brepjs-occt` (`kind: third-party-library`, `engine.family: opencascade`, the exact-BRep half and the
fixture-corpus generator). `set-shape-representation`'s `oracleRequirements` names
`brep.step.import` with `distinctEngineFamilies: 2` — satisfied by ruststep's `stepcode-independent`
family beside brepjs's `opencascade` — while `brep.measure.volume` stays on brepjs alone "because
claiming ruststep for those would be claiming a measurement it cannot make" (oracle `rationale`
field). `brepjs-occt` also carries a recorded `productionDebt`: it **is** production-reachable from
`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️brep-implementation.ts`, which makes it a genuine
oracle for the STEP codec subset (no geometry kernel in the subject) but not for a CAD subject that
adopts OpenCASCADE — `isolationBreaches` (`index.ts:3480`) requires exactly this kind of debt to be
recorded, never silently accepted, and it is shrink-only (`$defs.ProductionDebt` description: "a path
may leave this list, never join it").

## 5. Probes: external measurement only

`ProbeRegistryEntry` (`schema/🔣️.json` `$defs.ProbeRegistryEntry`, `index.ts:2924`) doc comment states
the rule the whole pipeline is built around: "A probe computes; the framework only evaluates the typed
JSON it emits. No geometry, no format parsing and no metric arithmetic may live in the orchestrator."
The BRep pilot's `🔬️probes/📜️script.ts` states the same rule from the other side in its own header
comment: "Everything here MARSHALS and INVOKES; nothing here computes geometry. Every number this file
emits comes out of `brepjs`'s OpenCASCADE kernel." `ProbeReport.measurements` (`$defs.ProbeReport`) is
declared `probe-defined`, deliberately open — the schema does not know what a probe measures, only
that it must emit a typed report.

`ProbeQualification` (`schema/🔣️.json` `$defs.ProbeQualification`, `index.ts:2951`) has three states:
`qualified`, `provisional`, `rejected`. `isQualifiedProbe` (`index.ts:2954`) is the single predicate a
release gate must consult before claiming a probe's strongest guarantee. A `ComparisonStage.optional:
true` (`$defs.ComparisonStage`) stage **runs and reports**; `evaluatePipeline` (`index.ts:3118`)
collects it into `unqualifiedStages` but `PipelineVerdict.equal` still requires every verdict to be
`ok || optional` — an optional stage cannot fail the gate, but it also cannot silently vanish from the
report.

The cc6 registry carries **8 probes total**, 6 `qualified` (`brepjs-step-import`,
`brepjs-brep-validity`, `brepjs-measure`, `brepjs-topology`, `brepjs-reimport-compare`,
`brepjs-tessellate` — each cites `📓️w4-brepjs-qualification.md` as its evidence) and 2
`provisional` (`step-external-canonicalizer`, `cgal-mesh-comparison`) whose stages are marked
`optional: true` in the `semantic-brep-solid-v1` pipeline and therefore gate nothing (confirmed live:
`.../🧪️verify/📤️report.json` `registry/unqualified-probes-are-marked` — "unqualified:
step-external-canonicalizer, cgal-mesh-comparison").

## 6. Fixture manifests

Three classes (`schema/🔣️.json` `$defs.FixtureClass`, `index.ts:2704`): `real-world`,
`handcrafted`, `third-party-generated`. `FixtureManifest` (`$defs.FixtureManifest`, `index.ts:2723`)
is immutable by contract — "Updating a fixture creates a NEW identity and digest; the manifest is
never edited in place" — enforced at runtime by `verifyFixture` (`index.ts:2819`), which re-hashes
every file against its manifest's `sha256` and fails on any mismatch
(`fixtureProvenanceBreaches`, `index.ts:3459`, `fixture-digest-mismatch`).

**Mandatory provenance**: `FixtureProvenance.license` is required unconditionally
(`fixtureManifestProblems`, `index.ts:2786`: "a fixture without an acceptable licence is a contract
failure, not an undocumented exception"). A `third-party-generated` fixture additionally requires a
complete `FixtureGenerator` record — `oracle`, `packageVersion`, `engineFamily`, `engineVersion`,
`command`, `platform` all non-empty (`index.ts:2789`) — "a fixture with no reproducible generator
record is a fixture nobody can audit" (`$defs.FixtureGenerator` description). The generating oracle
must itself be a **qualifying** one (`fixtureProvenanceBreaches`,
`fixture-generated-by-non-qualifying-oracle`, `index.ts:3464`): "An expected result produced by our
own second implementation is not third-party evidence."

**Content-addressed store**: `fixtureBlobRoot` (`index.ts:2840`) is
`.🧬semio/🦑️repo/⚡️cache/tests/fixtures/blobs/sha256/`, sharded by the first two hex digits so no
directory grows unbounded (`fixtureBlobPath`, `index.ts:2850`). `installFixtureBlob` (`index.ts:2861`)
writes a private staging file and `renameSync`s it into place — two agents generating the same fixture
concurrently is "the normal case, not the exception," and a reader never observes a partial blob.
`materializeFixtureBlob` (`index.ts:2890`) prefers a copy-on-write reflink, falls back to a hard link,
and falls back again to a real copy only when `mutable: true` — a mutation scenario handed a hard link
would write into shared storage otherwise.

**Generation vs. execution are separate operations, deliberately.** `FixtureScript`
(`📜️script.ts:1029`, command `test fixture <subcommand>`) has four subcommands:

| Subcommand | Effect |
| --- | --- |
| `generate` | Re-runs the recorded generator, installs the produced files into the CAS, publishes the manifest. Never runs during a normal test. |
| `reproduce` | Re-runs the recorded generator into a scratch dir and diffs the digests against the **committed** bytes — never overwrites the expectation. |
| `verify` | Re-hashes every committed file against its manifest. |
| `audit` | Prints/`--json`s the full provenance table (class, target, licence, `reproducible`, generator, contract problems). |

"A normal test run must never be able to rewrite the expectation it is being measured against"
(`📜️script.ts:1112` comment). `reproduce` mirrors this on the read side: "a 'reproduce' that overwrote
its own expectation would pass unconditionally, which is the whole failure mode it guards"
(`📜️script.ts:1073`).

**Worked example**: `✳️cc6`'s `🧪️oracle/🔣️.json` `fixtureManifests` array carries exactly **24**
entries (verified: `grep -c '"schema": "semio.repository-test.fixture/v2"'` = 24), spanning the
`family` values `spatial-relationship`, `robustness`, `failure`, `mechanical` (e.g.
`cut-bored-box-through`, `fuse-edge-touching-boxes`, `intersect-disjoint-operands`,
`mechanical-filleted-bracket`). Every one declares `"reproducible": false` and cites `brepjs-occt` as
its generator, honestly, per §12 below.

## 7. Comparison pipelines and the assertion vocabulary

`ComparisonPipeline` (`schema/🔣️.json` `$defs.ComparisonPipeline`, `index.ts:3066`) replaces v1's
single-projection `ComparisonProfileSpec` for any case producing more than one artifact or needing a
measured metric — `ComparisonProfileSpec.pipeline` (`$defs.ComparisonProfileSpec`) is the escape hatch
that still allows a plain profile for simple cases. A pipeline is an **ordered** list of
`ComparisonStage`s, each naming one `probe`, the artifact `inputs`/`outputs` (by `role`, never by
path), and `assertions`.

`evaluateStageAssertions` (`index.ts:3081`) is the exact rule set:

| Key suffix | Bound | Check |
| --- | --- | --- |
| `<key>Max` | `max` | measured ≤ declared |
| `<key>Min` | `min` | measured ≥ declared |
| `<key>Equal` | `equal` | `JSON.stringify(canonicalize(actual)) === JSON.stringify(canonicalize(expected))` |
| anything else | `value` | same structural equality, no suffix stripped |

The **exact-key-wins** rule (`index.ts:3090`–`3094`): the lookup tries the assertion key **verbatim**
against `report.measurements` first, and only falls back to the suffix-stripped key
(`connectedComponentsEqual` → `connectedComponents`) if the exact key is absent. The comment records
why: "A probe may legitimately name its own measurement `connectedComponentsEqual`, and stripping
unconditionally looked up `connectedComponents`, found nothing, and failed a stage whose measurement
was right there." A stage whose probe reported anything other than `status: "ok"` fails immediately
without evaluating any assertion (`index.ts:3084`). A stage with **no report at all** is a failure,
not a skip — "an unmeasured assertion that reads as green is the exact defect this pipeline replaces
v1's single generic tolerance to prevent" (`index.ts:3114`).

**Worked example**: `semantic-brep-solid-v1` (`✳️cc6`'s `🧪️oracle/🔣️.json` `comparisonPipelines`) has
six stages in deliberate order — import, then validity on the exact shape ("a mesh cannot see a lost
cavity"), then the scale-normalized metric comparison via `brepjs-reimport-compare`
(`relativeVolumeErrorMax: 1e-8`, `connectedComponentsEqual: true`, …), then topology, then the two
`optional: true` stages (`step-external-canonicalizer`, `cgal-mesh-comparison`) that run and report but
gate nothing.

## 8. Scale-relative tolerance

`resolveTolerance` (`index.ts:3024`) is the one rule: `max(absolute, relative × reference)`. A single
absolute number cannot serve both a micro-scale fixture and one translated a kilometre from the origin
(`$defs.ToleranceProfile` description). `resolveToleranceProfile` (`index.ts:3036`) resolves a
profile's five dimensional/metric fields against a measured `{diagonal, area, volume}` reference and
an optional `ToleranceOverride`, **capped** by the profile's own `maxOverrideFactor` — an override
cannot exceed the cap regardless of what the fixture requests (`Math.min(override.factor, cap)`,
`index.ts:3038`), and `fixtureProvenanceBreaches`' `tolerance-override-exceeds-cap`
(`index.ts:3470`) catches a manifest that tries anyway. Every override is **always reported**:
`ResolvedTolerances.overridden` is a plain boolean the coverage matrix surfaces as
`toleranceOverridden` on every affected row (`index.ts:3844`).

The **7 core profiles** (`CORE_TOLERANCE_PROFILES`, `index.ts:3008`), owned by the framework — an
owner may contribute further ones by manifest (`toleranceProfileTable`, `index.ts:3019`):

| Profile | `absoluteLength` | `relativeLength` | `maxOverrideFactor` | Intent |
| --- | --- | --- | --- | --- |
| `analytic-strict` | 1e-9 | 1e-12 | 10× | Closed-form analytic answers; only representation noise permitted |
| `mechanical-standard` | 1e-7 | 1e-9 | 10× | Ordinary mm-scale solids — the Boolean corpus default |
| `contact-sensitive` | 1e-6 | 1e-8 | 5× | Exact contact/tangency, where kernels legitimately disagree at the boundary |
| `epsilon-degenerate` | 1e-5 | 1e-7 | 4× | Slivers/near-coplanar; result CLASS is the assertion, metric secondary |
| `large-coordinate` | 1e-4 | 1e-9 | 5× | Geometry far from the origin — only the relative term is meaningful |
| `micro-scale` | 1e-11 | 1e-8 | 5× | Sub-millimetre geometry, where an mm-sized absolute floor would swallow the model |
| `real-world-import` | 1e-4 | 1e-6 | 3× | Third-party models with unknown, larger authoring tolerance |

## 9. Leases, retention, atomic publication, GC

`RetentionClass` (`schema/🔣️.json` `$defs.RetentionClass`, `index.ts:3149`): `ephemeral-success`,
`last-success-proof`, `failure-evidence`, `fixture-generation`, `toolchain`, `downloaded-source`,
`pinned`. `PROTECTED_RETENTION_CLASSES` (`index.ts:3153`) — `failure-evidence`, `last-success-proof`,
`pinned`, `toolchain` — are never removed by routine stale cleanup.

`RunLease` (`$defs.RunLease`, `index.ts:3159`) records one agent's claim on one run directory:
`state` (`creating|active|complete|failed|abandoned`), `heartbeatAt`, `pid`, `retention`.
`leaseReclaimable` (`index.ts:3230`) requires **both** a stale heartbeat (`LEASE_STALE_MS` = 45 min,
`index.ts:3178`) **and** `!processAlive(lease.pid)` before an `active` lease is even a candidate for
reclaim — "a slow run and a dead one look identical from the timestamp alone, and deleting the slow
one destroys a peer agent's work" (`index.ts:3225`).

`withAtomicRunDir` (`index.ts:3244`) is the publication primitive: work happens in a private
`<final>.<runId>.creating` directory, and the whole thing is published with **one** `renameSync` — a
reader observes either the previous complete directory or the new one, never a half-written one. On
error the lease is rewritten `state: "failed", retention: "failure-evidence"` before the error
propagates, so a failed run's evidence survives routine GC.

`collectGarbage` (`index.ts:3638`) is mark-and-sweep, **dry by default everywhere it is offered**
(`GcScript`, `📜️script.ts:1191`, flag `--apply` to act). It refuses to run at all unless the resolved
cache root sits inside the resolved repository meta dir (`index.ts:3647`–`3651`, guarding against both
a symlink escaping the repo and a false rejection from `/var` → `/private/var`-style repo symlinks).
`markReferencedBlobs` (`index.ts:3588`) marks every blob any committed fixture manifest, cached
fixture manifest, or retained run manifest's `artifacts[].sha256` references; an unreachable blob is
one nothing reaches, never merely one that looks old.

## 10. The 16 coverage dimensions, 6 release-gated

`COVERAGE_DIMENSIONS` (`index.ts:3743`) — all 16, measured by `measureCoverage` (`index.ts:3881`):

`runtimeMutationCoverage`, `subsetOwnershipCoverage`, `externalOracleCoverage`,
`oracleCapabilityCoverage`, `productionBridgeCoverage`, `fixtureClassCoverage`,
`fixtureProvenanceCoverage`, `expectedOutcomeCoverage`, `inverseCoverage`, `metamorphicCoverage`,
`comparisonMetricCoverage`, `determinismCoverage`, `implementationCoverage`, `platformCoverage`,
`fixtureReproducibilityCoverage`, `dependencyIsolationCoverage`.

`RELEASE_GATED_DIMENSIONS` (`index.ts:3764`) — the 6 that must be **exactly** 100%:
`runtimeMutationCoverage`, `subsetOwnershipCoverage`, `externalOracleCoverage`,
`productionBridgeCoverage`, `fixtureProvenanceCoverage`, `dependencyIsolationCoverage`.

**An EMPTY denominator fails a gate.** `measure` (`index.ts:3792`) reports `ratio: 1` for `total === 0`
so a *display* reads "n/a" rather than a false 0%, but `enforceReleaseGates` (`index.ts:3936`)
explicitly special-cases it: "a release gate that accepted it would be satisfied by a run in which
nothing was measured at all — which is precisely the failure the gate exists for" (`index.ts:3945`).
This is exactly the state the platform is in today for `runtimeMutationCoverage` at the repository
level — the release-gate check confirms it live (`🧪️verify/📤️report.json`
`coverage/release-gate-blocks-today`: "runtimeMutationCoverage has an EMPTY denominator — nothing was
measured, and an unmeasured dimension cannot be 100%").

## 11. Commands and the full selector set

Router registrations (`📜️script.ts:1231`–`1249`):

`discover`, `contract`, `oracle <level>`, `subject <level>`, `parity <level>`, `run`/`test` (contract
then parity), `report`, `clean [--dry] [--stale] [--over <bytes>]`, `dependency`, `metrics`, `nx`,
`doctor`, `inventory` (new v2 — runs the production bridge, §2), `fixture
generate|reproduce|verify|audit` (new v2, §6), `probe [--json]` (new v2 — lists/qualifies probes, §5),
`matrix [--json] [--enforce]` (new v2 — the coverage report and release gates, §10), `gc [--apply]
[--older-than] [--over-size] [--agent] [--retention]` (new v2, §9).

`Selectors` (`📜️script.ts:240`) is the full v2 selector set every phase accepts, read by
`readSelectors` (`📜️script.ts:258`) as `--flag value` pairs:

`--artifact`, `--standard`, `--subset`, `--mutation`, `--outcome`, `--case`, `--fixture-class`,
`--fixture-family`, `--oracle`, `--probe`, `--implementation`, `--platform`, `--agent`, `--run`,
`--status`.

"Every phase accepts every selector, so a CI shard is expressible at the exact coordinate a report row
is keyed by — never merely at artifact level" (`📜️script.ts:237`).

## 12. The STEP canonicalization position

**Raw byte equality is NOT a gate, in either direction.** Measured, not assumed:
`📓️w4-brepjs-qualification.md` ran the qualification spike and found OCCT is not even
**self**-deterministic — two `exportSTEP` calls on the *same shape in the same process* differ:

```
#7 = PRODUCT('Open CASCADE STEP translator 8.0 1',   ← first  export
#7 = PRODUCT('Open CASCADE STEP translator 8.0 2',   ← second export
```

OCCT stamps a monotonically incrementing translator counter into `PRODUCT`, and `FILE_NAME`
additionally carries a wall-clock timestamp, a producer string and an originating-system string — none
of it semantic. 11 of 12 qualification criteria were met; `step-self-determinism` is the one failure.

The operative gate is therefore **semantic equivalence**, implemented as the
`semantic-brep-solid-v1` pipeline's non-optional stages (§7): independent reimport succeeds, both
shapes are valid solids, the scale-normalized volume/area/centroid/bounding-box error and component
count agree within `mechanical-standard`/`analytic-strict`/`contact-sensitive` tolerance depending on
the fixture. **Canonical byte equality may not be claimed** until an external canonicalizer is
qualified: `step-external-canonicalizer` (candidate: STEPcode) is registered in `✳️cc6`'s
`🧪️oracle/🔣️.json` with `qualification.status: "provisional"` and every criterion `met: false`
("not installed... AP242 coverage, deterministic entity ordering and header normalization have not
been demonstrated here"). Its pipeline stage is `optional: true`, so `isQualifiedProbe` refuses it to
any gate that would claim canonical byte equality (`index.ts:2954`, `2954`–`2956`), and it runs and
reports rather than silently disappearing.

---

## 🧑‍🔬️ Oracle-authoring rules

1. **A mutation's `oracleRequirements` names a capability and a `qualifyingKind`, never an oracle id
   directly** (`$defs.OracleRequirement`) — the registry, not the manifest, decides which registered
   oracle currently supplies it, so a new independent oracle for the same capability needs no manifest
   edit.
2. **`kind` must be one of the three qualifying kinds to discharge anything.** Registering a second
   in-repo implementation is useful and required as a supplement (`cross-semio-implementation`), but it
   never satisfies `oracleRequirementBreaches` (§4).
3. **Independence is accounted at the `engine.family` level, never at the wrapper/package level.** Two
   bindings over one kernel are one oracle. Declare `EngineFamily.family` honestly — `opencascade`,
   `cgal`, `stepcode`, `parasolid`, or a new one — and expect `engineIndependenceBreaches` to fire the
   day the subject adopts the same family (§4 worked example).
4. **`hostPath` decides provisioning, and it is the only thing that does** (`README.md` "Reaching a
   reference library"). A path means LOCAL in-repo source, linked by Rust `Cargo.toml` path only — a
   crates.io coordinate is refused as an unreviewed dependency of a generated host. No path means an
   EXTERNAL distribution: a cache-local Python venv (`--system-site-packages`, reused if the machine
   already provides the pinned version, never touching the system interpreter), or resolution from the
   repository's own `node_modules` for TypeScript — one lockfile, one version, repository-wide.
5. **`productionReachable: true` requires a recorded `productionDebt`, or it is a blocking breach**
   (`isolationBreaches`, `index.ts:3484`). Debt is shrink-only (`ProductionDebt` schema) and must name
   `reachableFrom`, `owner`, `plan` — never silently accepted, never allowed to grow.
6. **`networkDuringExecution: true` is always a breach.** Provision ahead of the run and read from the
   fixture store; a network call during execution makes a run non-reproducible and CI-flaky.
7. **A composed reference's further linked packages are pinned and licensed in their own right**
   (`OracleLinkedPackage` — `package`, `version`, `license`, `role` all mandatory), not folded silently
   into the primary package's licence.

## 🧫️ Fixture-provenance rules

1. **`provenance.license` is mandatory, unconditionally** — an unlicensed fixture is a contract
   failure, never an undocumented exception.
2. **`class: "third-party-generated"` requires a complete `FixtureGenerator` record**: `oracle`
   (must resolve to a registered, qualifying oracle), `packageVersion`, `engineFamily`,
   `engineVersion`, `command` (re-runnable, exactly as `fixture reproduce` will invoke it), `platform`.
   `seed` and `sourceDigest` are optional but should be set whenever the generator is seedable.
3. **`reproducible` must be stated honestly, not aspirationally.** When the generating engine is not
   byte-self-deterministic (§12), the fixture declares `reproducible: false` and
   `fixtureReproducibilityCoverage` (one of the 16 dimensions) reports it rather than hiding it behind
   a metric-only pass.
4. **A file's `sha256` is its identity.** Editing committed bytes without minting a new fixture `id`
   is a `fixture-digest-mismatch` breach the moment `fixture verify` or `test contract` runs. There is
   no "update in place."
5. **`toleranceOverride` is all-or-nothing and always reported.** `reason` must be ≥ 20 characters
   (state WHY, not just THAT), `measuredBaseline` and `factor` (≥ 1) must be the actual numbers the
   override is sized against, and `approvedBy` must name an owner. The override is capped by the
   tolerance profile's own `maxOverrideFactor` regardless of what is requested.
6. **Generation and execution never share a code path.** `fixture generate` is the only command
   permitted to write into the CAS and publish a manifest; a normal `test run` only reads. `fixture
   reproduce` diffs against the committed bytes and never overwrites them.
7. **Fixtures are immutable after review.** A scenario needing to mutate one copies it into the case
   work directory first (`README.md` "Rules that are enforced, not advisory") — `local://` never
   shadows `shared://`.

## 📐️ BRep tolerance policy

* Default pipeline tolerance profile is declared per pipeline (`semantic-brep-solid-v1` →
  `mechanical-standard`), but **each fixture may declare its own `toleranceProfile`**, and the cc6
  corpus does: `analytic-strict` for fixtures with a closed-form answer (`cut-bored-box-through`,
  `intersect-overlapping-boxes`, the empty/disjoint cases), `contact-sensitive` for exact face/edge
  contact (`fuse-face-touching-boxes`, `fuse-edge-touching-boxes`) — "kernels legitimately disagree
  about the last bits of a classification boundary" there.
* `referenceScale: "bounding-box-diagonal"` on the pipeline means the normalizing `D` for a relative
  metric is **measured by a probe**, never computed by the orchestrator (`$defs.ComparisonPipeline`
  `referenceScale` description) — consistent with the "probe computes, orchestrator compares" rule
  (§5).
* Only the **connected-component count** is asserted generally in the topology stage; face and edge
  counts are asserted only when a mutation's `normativeTopologyCounts` is `true` — "two valid kernels
  may split a periodic surface differently" (`$defs.ManifestMutation.normativeTopologyCounts`). The
  `fuse-edge-touching-boxes` fixture is the measured example: this kernel leaves **two** solids (12
  faces, 23 edges) for edge-only contact, so its declared outcome is `disjoint`, not `applied` — the
  volume is identical either way and only the component count distinguishes the two answers.
* The exact-shape metrics (`brepjs-measure`) are asserted as **stronger** than mesh-derived
  equivalents on purpose: measured directly on the BRep, the bored-box volume agrees with the analytic
  closed form to a relative error of `2.83e-16`, versus a mesh comparison that would be limited by
  tessellation tolerance.

## 🧹️ Clean/CAS operations

| Operation | Command | Scope |
| --- | --- | --- |
| Generic output cleanup | `test clean [--dry] [--stale] [--over <bytes>]` | Marked generated test output roots (hosts, work, results, diffs) discovered via `discoverTestCases`; refuses to remove anything resolving inside an excluded/taxonomy-protected area. |
| CAS mark-and-sweep | `test gc [--apply] [--older-than <s>] [--over-size <bytes>] [--agent <id>] [--retention <class>]` | Fixture blob store + run directories under the repository cache root. Dry by default; `--apply` is required to actually remove anything. |

Both are lease-aware: a directory with an `active` lease from a *different* agent, or one whose
retention class is in `PROTECTED_RETENTION_CLASSES`, is reported as `held`, never removed, regardless
of age or size flags (`index.ts:3679`–`3681`). The blob store itself is content-addressed
(`.🧬semio/🦑️repo/⚡️cache/tests/fixtures/blobs/sha256/<first-2-hex>/<64-hex>`) — "the store's whole
safety argument is that a blob's name IS its content" (`index.ts:288` comment on `sha256_hex`).
Materialization into a run directory prefers reflink, then hard link, then a real copy only when the
target must be writable (`materializeFixtureBlob`, §9).

## ✅️ Migration checklist — moving a subset onto v2

1. **Confirm the smallest semantic subset.** If the owner still uses a wildcard subset (`*`, `any`,
   `all`, `unconstrained`), split it into real semantic subsets first — `subsetOwnershipCoverage` will
   otherwise report the mutation as uncovered no matter what else is done.
2. **Expose (or confirm) the production mutation bridge** at `🏭️bridge/📜️script.ts`, answering
   `listMutations(artifact, standard, subset)` by enumerating the dispatch enum exhaustively (compile
   error on a missed variant, per the cc6 bridge pattern, §2) — never by restating a hand list.
3. **Author the mutation manifest** in the owner's `🧪️oracle/🔣️.json`: one row per dispatch variant,
   `outcomes` matching exactly what dispatch can reach, `productionDispatch.variant` naming the exact
   enum variant, and `oracleRequirements` naming a qualifying capability (with
   `distinctEngineFamilies` raised wherever a single kernel ancestry would make the comparison
   circular).
4. **Run `test inventory --artifact <a> --standard <s> --subset <sub>`** to produce the runtime side,
   then `test contract` to check the three-way equality gate — fix `runtime-only`, `manifest-only`,
   `test-only`, outcome and variant mismatches before proceeding.
5. **Qualify or register an external oracle** for every required capability. Run a qualification
   spike, write its evidence into a ticket doc (the `📓️w4-brepjs-qualification.md` pattern: a
   criteria table plus a stated failure and its consequence), and register the oracle with an honest
   `engine.family`. Do not fabricate a `NoOracleDecision` for a mutation capability — it is
   schema-refused (`coversMutations: false`).
6. **Build or reuse probes** for whatever the comparison pipeline needs; qualify each one the same
   way. Any stage backed by an unqualified probe is declared `optional: true`.
7. **Define the comparison pipeline** (or reuse an existing `ComparisonProfileSpec` for a
   single-artifact case) and pick a `toleranceProfile` — one of the 7 core profiles, or a
   subset-contributed one, per fixture where scale varies.
8. **Generate fixtures** with `test fixture generate`, review the produced bytes by hand, commit them,
   then `test fixture verify` and `test fixture audit` to confirm provenance is complete and
   `reproducible` is stated honestly.
9. **Run `test run`** (contract, then parity) end to end for the subset, then `test matrix --enforce`
   and confirm the 6 release-gated dimensions are either 100% or the remaining gaps are understood and
   tracked.
10. **Delete the replaced legacy v1 hierarchy in the same change** — catalog, vector bundles, the old
    comparison profile — never leave two test hierarchies alive for one owner (`README.md` lifecycle
    step 12).
11. **Track, do not chase, upstream compile blockers.** If a shared crate is mid-refactor by another
    session (the concrete case: `cargo check -p semio-s-plugin-stdio --lib --offline` currently fails
    because `semio-framework`'s `🔁️workflow/🦀️component.rs` is missing `DESCRIPTORS`/`descriptor` on
    `protocol::Mutation` impls — `📓️h5-no-mutation-blast-radius.md`), record the blocked work as a
    visible, gate-blocking breach (e.g. `test-only-mutation`) and move on; do not report a claim
    nothing has compiled.

## 🚧️ What is not yet working

* **The `✳️cc6` production bridge cannot be built or run today.** `🏭️bridge/🦀️component.rs` depends on
  `semio_s_plugin_stdio::artifacts::step::mutations::cc6::StepCc6Mutation`, and
  `cargo check -p semio-s-plugin-stdio --lib --offline` currently fails because of an unrelated,
  in-progress refactor in `semio-framework` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`
  is missing `DESCRIPTORS`/`descriptor` on two `protocol::Mutation` impls, plus missing `os_spr`
  imports — error count grew from 2 to 75 during this ticket's own session, confirming it is a live
  concurrent refactor and not a static defect here). This is why `runtimeMutationCoverage` reports an
  empty denominator today (§10) rather than a passing three-way equality.
* **`step-external-canonicalizer` (STEPcode) is registered but unqualified.** Not present in this
  environment; AP242 coverage, deterministic entity ordering and header normalization are undemonstrated.
  Its pipeline stage is `optional: true` and canonical STEP byte equality may not be claimed (§12).
* **`cgal-mesh-comparison` (CGAL) is registered but unqualified.** Not installed in this environment
  (no CGAL headers under `/opt/homebrew` or `/usr/local`); the independent mesh-side Hausdorff/volume/
  area/self-intersection stage is `optional: true` and gates nothing.
* **`no-mutation` retirement is scoped but not executed** (`📓️h5-no-mutation-blast-radius.md`): 151
  call sites across 7 STEP subsets, one of which (`#[derive(Default)]` removal on 7 enums) needs a
  compiler-driven call-site repair that cannot be verified while `semio-framework` fails to compile.
  `mutationInventoryBreaches` reports it as a live `test-only-mutation` breach on every contract run.
* **The CAD artifact's wildcard subset (`✳️any`) has not been split** (`📓️w0-baseline.md` G4) —
  `subsetOwnershipCoverage` reports all 20 `CadMutation` variants as wildcard-owned.

## 📎️ Verification of this specification

Every claim above was checked against source, not assumed:
`🧬️schema/🔣️.json`, `📦️packages/🟦️typescript/📦️index.ts` (regions `🪆️Subset` through `📈️CoverageV2`),
`📜️script.ts`, `🧬️protocol/🦀️component.rs`, `🏃️runner/🦀️component.rs`, the `✳️cc6` pilot
(`🧪️oracle/🔣️.json`, `🔬️probes/📜️script.ts`, `🏭️bridge/🦀️component.rs`), and the ticket's own
evidence (`📓️w0-baseline.md`, `📓️w4-brepjs-qualification.md`, `📓️h5-no-mutation-blast-radius.md`,
`🧪️verify/📜️script.ts` + `📤️report.json`, 81/81 checks passing at this baseline).
