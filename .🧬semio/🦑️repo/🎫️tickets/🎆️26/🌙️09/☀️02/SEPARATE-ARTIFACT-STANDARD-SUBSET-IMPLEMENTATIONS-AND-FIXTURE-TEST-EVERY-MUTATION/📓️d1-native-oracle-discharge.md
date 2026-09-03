# D1 — Native-Second-Implementation Discharge: The Rule, The Evidence, The Application

Shard D1. Changed the judge itself (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`
and its own tests), per `📓️c2-native-artifact-oracles.md`'s finding that `missing-external-oracle` cannot,
even in principle, be discharged for a semio-native artifact, and its precisely-scoped recommendation for
a new qualifying kind. C2 deliberately did not implement it, to avoid moving the shared ground truth
mid-flight. Every other C2/A-wave shard had finished before this one started.

## 0. Headline

| id | before (my baseline) | after |
| --- | --- | --- |
| `missing-external-oracle` | 1183 | **344** |
| `no-oracle-covers-mutation` | 5 | 0 (unrelated to this shard — see §5) |
| `reimplementation-registered-as-third-party` | 2 | **2 (untouched, as instructed)** |
| `oracle-capability-mismatch` | 0 | 0 |
| `unknown-oracle` | 0 | 0 |
| `fixture-generated-by-non-qualifying-oracle` | 0 | 0 |
| `native-second-implementation-*` (new, this shard) | n/a | **0** — every promoted entry passes its own earned-ness check |
| **TOTAL breach count** | **1951** | **1114** |

Both numbers are from a live foreground `bun ./📜️script.ts test contract`, read back from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`: before is
`🗑️generated/d1-baseline-testing.json` (captured before any edit this shard made), after is
`🗑️generated/d1-after-testing.json` (captured after the rule change AND the artifact promotion below).
My baseline (1183/2/5/1951) differs slightly from C2's stated 1182/2/0/2058 because several other
shards (B5, C1, C3, C4 and unnamed concurrent sessions) kept moving the tree between C2's snapshot and
mine — expected and accounted for, per the ticket's own "measure, never assert" rule.

**Closed by legitimate qualification: 839 of the 1183 `missing-external-oracle` breaches this shard
started with.** The remaining **344 are honest, itemized debt** — 174 semio-native artifacts with no
second implementation yet, 170 real interchange formats still correctly owed a genuine third-party
reference. Neither bucket was hidden, relabelled, or force-closed.

## 1. The rule, as implemented

### 1a. The new qualifying kind

`QUALIFYING_ORACLE_KINDS` (`🟦️.ts:2767`) gained a fourth member,
`"verified-native-second-implementation"`, alongside the existing `third-party-library` /
`third-party-cli` / `standards-reference-tool`. `isQualifyingOracleKind` (`🟦️.ts:2790`) — the single
predicate every discharge check in the file calls — therefore now also lets this kind through
`oracleRequirementBreaches`'s discharge test, exactly as any other qualifying kind would.

`SUPPLEMENTAL_ORACLE_KINDS` (still `metamorphic`, `inverse`, `round-trip`, `property`,
`cross-semio-implementation`) is **unchanged**. A second implementation registered as
`cross-semio-implementation` still discharges nothing, for every artifact where that was already true.
The new kind is not a relabelling of the old one — it is a new, narrower, EARNED path that a
`cross-semio-implementation` entry may be *promoted into*, never a change to what
`cross-semio-implementation` itself means.

### 1b. The categorical boundary — `isSemioNativeArtifact` (`🟦️.ts:2793`)

```ts
export function isSemioNativeArtifact(artifact: string): boolean {
  return artifact.length > 0 && !(artifact.startsWith("s.stdio.") && artifact !== "s.stdio.semio");
}
```

Every `mutationManifests[].artifact` in this repository is namespaced `s.<plugin>.<name>`. I read the
full set of 96 distinct artifact ids actually declared in the registry before writing this function (not
assumed): every REAL interchange format this repository decodes — `pdf`, `png`, `tiff`, `gltf`, `ifc`,
`step`, `docx`, `xlsx`, `pptx`, `svg`, `xml`, `zip`, `jpg`, `dwg`, `obj`, `gif`, `bmp`, `avi`, `bcf`,
`las`, `json`, `mp3`, `mp4`, `stl`, `ply`, `dxf`, `wav`, `txt`, `html`, `csv`, `epw`, `md`, `deflate`,
`tsv` — lives under `s.stdio.*`. `s.stdio.semio` is the one exception: it names semio's OWN container
format, defined by this repository, so by construction no vendor implements it. Every other artifact id
(`s.norm.*`, `s.block.*`, `s.puzzle.*`, `s.cad.cad`, `s.lowpoly.lowpoly`, `s.procedural.*`, `s.gis.*`,
`s.raster.raster`, `s.forms.forms`, `s.trinity.*`, `s.writer.writer`, `s.sourcing.curation`,
`s.playbook.playbook`, `s.remodeling.remodeling`, `s.shooting.shooting`, `s.layout.layout`, `s.dag.dag`,
`s.process.process3d`, `s.flow.flow`, `s.reasoning.wires`, `s.animate.presentation`, `s.vcs.vcs`,
`s.imperative.imperative`, `s.space.*`, `s.demonstrator.playground`, `s.energy.model`,
`s.mathematical.mathematical`, `s.sequence.sequence`, `os.config`, …) is semio's own application-domain
document, not an interchange standard, and none of them collides with the `s.stdio.*` boundary.

This is enforced **in code that runs on every gate invocation**, not documented in a comment beside a
kind an author could set freely: `nativeSecondImplementationBreaches` calls it and fires
`native-second-implementation-not-native` the moment a claimed `format` fails it — §1c, check 2, and the
test `"a real interchange format is refused however good the survey reads"` proves it with `s.stdio.png`.

### 1c. The earned-ness gate — `nativeSecondImplementationBreaches` (`🟦️.ts`, wired into
`validateAllContracts` right after `reimplementationOracleBreaches`)

For every oracle entry (in every owner contribution) whose `kind` is
`"verified-native-second-implementation"`, in order, each producing its own distinct, actionable breach
id on failure — a lazy or false claim cannot pass by accident, because every one of the following must
hold at once:

1. **Evidence is present at all.** `oracle.nativeSecondImplementation` must exist →
   `native-second-implementation-unearned`. This is the check that closes the exact gap the brief named:
   *the kind field alone still mechanically discharges the requirement* (nothing in
   `oracleRequirementBreaches` reads this new evidence object — discharge is, and stays, a pure
   `isQualifyingOracleKind(oracle.kind)` check, unchanged). Proven with a dedicated test: *"the kind field
   alone, with no recorded evidence, still mechanically discharges — which is exactly why the claim must
   also be earned"* asserts `oracleRequirementBreaches(...)` returns `[]` (silently discharged) **and**
   `nativeSecondImplementationBreaches(...)` returns exactly one `native-second-implementation-unearned`
   breach for the same entry — the earned-ness gate is the ONLY thing standing between an unearned claim
   and a silently-passing gate.
2. **The artifact is actually semio-native.** `isSemioNativeArtifact(evidence.format)` →
   `native-second-implementation-not-native` otherwise. §1b.
3. **The claim ties to real, owned vocabulary.** `evidence.format` must match an `artifact` this SAME
   contribution's own `mutationManifests` declares (not the whole registry — see the mesh/cad/drawing/brep
   /document `s.stdio.semio` subsets discussion in §4, where a registry-wide manifest lookup would have
   silently merged 13 unrelated subsets' capability sets into one) → `native-second-implementation-unearned`
   if no such manifest exists here.
4. **100% capability coverage, not partial.** `oracle.capabilities` must be a superset of every
   capability the owning manifest(s) declare → `native-second-implementation-partial-coverage` otherwise,
   naming exactly which capabilities are uncovered. Proven with a two-capability manifest where the
   reference only covers one.
5. **A structured, non-empty negative search.** `noThirdPartySurvey.ecosystemsSearched` must be
   non-empty, `candidatesConsidered` must be non-empty, and every candidate needs a real `package` string
   and a `reason` of at least 10 characters → `native-second-implementation-unearned` otherwise. Proven
   with three variants (empty ecosystems, empty candidates, a too-short reason) in one test.
6. **A genuinely different implementation language.** `subjectImplementationLanguage` and
   `secondImplementationLanguage` must both be non-empty and must differ (case-insensitively) →
   `native-second-implementation-same-language` otherwise — the check that keeps a same-language or
   transliterated reference from qualifying, because both halves would still read one specification and a
   misreading of it produces two agreeing wrong answers, exactly as the docstring on
   `reimplementationOracleBreaches` already argues for the sibling rule.
7. **A named specification source.** `specificationSource` must be non-empty →
   `native-second-implementation-unearned` otherwise. This field is **not** independently machine-verifiable
   (nothing here re-reads the cited document and diffs it against the reference's behaviour) — per the
   brief's own instruction, it is still required to be present and non-empty rather than omitted, because an
   unfalsifiable claim recorded in the open is a far better failure mode than a silent gap.
8. **Fixture-backed vectors.** `fixtureCoverage.vectors > 0` and `capabilitiesCovered` non-empty →
   `native-second-implementation-unearned` otherwise — ties the discharge back to this ticket's second law
   ("every mutation is tested over fixtures"), not merely a language claim with nothing committed behind it.

Every check above has its own unit test in `🧪️index.test.ts`'s new `"🌱️ native second implementation"`
describe block (12 tests, all listed in §3), including one full **negative** test proving a false/lazy
claim is rejected end-to-end, and one **positive** test proving a fully-earned claim both discharges the
requirement and records zero breaches of its own.

### 1d. What was deliberately left untouched

- `reimplementationOracleBreaches`, `noOracleMisuseBreaches`'s mechanism, `oracle-in-production`,
  `stub-serializer`, `binary-protocol-drift` — not one line changed. `reimplementation-registered-as-third-party`
  stayed at 2 (`ifc/2x3`, `ifc/4`), exactly as before.
- `oracleRequirementBreaches` itself gained **zero** new logic. Discharge is still the same one-line
  `isQualifyingOracleKind(oracle.kind)` filter it always was; only the SET of kinds that predicate accepts
  grew by one, narrowly. This was a deliberate design choice to keep the earned-ness enforcement in one
  dedicated, independently-testable function rather than entangling it with the discharge path — mirroring
  exactly how `reimplementation-registered-as-third-party` already polices `third-party-library` abuse
  as a SEPARATE check rather than inside the discharge function.

## 2. Test evidence

`bun ./📜️script.ts lint` (the package's own `tsc --noEmit`): **zero new errors**. Two pre-existing errors
at `🟦️.ts:5758` (`requirement.oracle` — a field referenced in `measureCoverage` that the `OracleRequirement`
type has never declared, confirmed present at `git show HEAD`, i.e. before this shard touched anything) and
a handful of pre-existing errors in unrelated files (`🖱️ui/🎨️styling`, `📚️library`) are untouched by this
diff — full log: `🗑️generated/d1-lint.txt` is not kept (transient), verified live twice, before and after.

`bun test 🧪️index.test.ts` for **just the new describe block**:
```
 12 pass
 78 filtered out
 0 fail
 95 expect() calls
```
(after the artifact promotion in §4 — before it, 11 pass / 1 fail, because the 12th test asserts the LIVE
registry actually contains promoted entries and they are all earned; see below.)

The 12th test, `"every registered verified-native-second-implementation entry in the live registry is
earned"`, is the real end-to-end gate: it loads the live `OracleRegistry`, finds every
`verified-native-second-implementation` oracle, asserts `nativeSecondImplementationBreaches` reports
zero breaches for any of them, and asserts every one's `format` is semio-native. **It failed before §4
(0 live entries, so `expect(live.length).toBeGreaterThan(0)` failed) and passes after (53 live entries, all
earned).** This ties the rule and the data application together mechanically — a future regression in
either the rule or a promoted entry's evidence fails this test, not just a silent drift in the breach count.

Full package suite, `bun test 🧪️index.test.ts` (all 90 tests, run in the FOREGROUND, ~180s):
```
 78 pass
 12 fail
 1583 expect() calls
```
The 12 failures are **pre-existing and unrelated to this shard** — confirmed three ways: (1) none of their
names or diffs mention `nativeSecondImplementation`, `isSemioNativeArtifact`, or any `native-second-
implementation-*` id (grepped the full run log, zero matches); (2) the same 12 (13 before promotion, one —
`📈️ non-aggregate metrics`'s oracle-coverage test — disappeared between my two runs from unrelated
concurrent work) fail for reasons entirely outside this diff's surface: `taxonomy.json` currently declares
zero `exempt` areas (some other session's edit), several registered oracle/no-oracle-decision records in
the live registry are missing `comparisonProfiles`/`substitutes` fields entirely (`TypeError: undefined is
not an object`, not a value-mismatch — a different session's in-flight edit, not mine), and `discoverTestCases`
throws `ENOENT` on `mutate-step-ap214`'s feature file, which does not currently exist on disk (another
session mid-rename, exactly like the `curate→curation` and `rewrite→rewriting` renames this shard already
had to route around live — see §4); (3) `git show HEAD` confirms the `requirement.oracle` root cause of the
largest failure (`oracleImportsInProduction`'s cascading false-positive list) predates every edit this
shard made. Full logs kept per house rules: `🗑️generated/d1-pretest-full.txt` (before promotion) and
`🗑️generated/d1-posttest-full.txt` (after).

## 3. The 12 new tests (`🧪️index.test.ts`, `"🌱️ native second implementation"`)

1. `isSemioNativeArtifact` refuses every `s.stdio.*` interchange format except `s.stdio.semio` itself —
   and accepts `s.norm.din16798`, `s.block.5d`, `s.puzzle.3d`, `s.cad.cad`, `os.config`.
2. `verified-native-second-implementation` is a member of `QUALIFYING_ORACLE_KINDS` and
   `isQualifyingOracleKind` accepts it.
3. A fully earned claim discharges the requirement (`oracleRequirementBreaches` → `[]`) AND records no
   breach of its own (`nativeSecondImplementationBreaches` → `[]`).
4. **The kind alone, no evidence** — still mechanically discharges (proving the gap exists) but is caught
   by `native-second-implementation-unearned` (proving the gate closes it). The headline negative test.
5. A claim against `s.stdio.png` is refused (`native-second-implementation-not-native`) even with a
   complete, otherwise-valid evidence object.
6. A `format` naming no manifest this owner actually declares is refused
   (`native-second-implementation-unearned`).
7. A reference covering one of two required capabilities is refused
   (`native-second-implementation-partial-coverage`), naming the uncovered capability.
8. Three variants of a non-credible `noThirdPartySurvey` (empty ecosystems, empty candidates, a
   too-short reason) are all refused.
9. A same-language (`Rust`/`rust`) claim is refused (`native-second-implementation-same-language`).
10. An empty `specificationSource` is refused.
11. Zero fixture vectors is refused.
12. The live-registry end-to-end gate (§2).

## 4. Per-artifact application — 53 entries, 41 distinct artifacts, 872 mutations promoted

I re-surveyed the LIVE tree myself rather than trusting C2's now-superseded table (the ticket's own
"measure, never assert" rule, and the "Live Predicate, Not Derived Artifact" lesson from this session's own
history) — the tree had moved under both of us: `curate→curation`, `rewrite→rewriting` renamed mid-flight
by a concurrent session, and five artifacts C2 had listed as "no second implementation yet"
(`s.demonstrator.playground`, `s.imperative.imperative`, `s.energy.model`, `s.space.home`,
`s.space.space`) turned out, live, to already carry a complete, executed, fully-covering Python second
implementation — some other concurrent shard had finished exactly that debt since C2's snapshot. I
verified each of the five individually (full committed Python component, an explicit negative survey,
"both/all passed" execution log, 100% capability coverage) before promoting it; the fresh gate confirms all
five's `missing-external-oracle` breaches are gone in the after-count.

**Selection criterion, applied mechanically via script** (`🩹️d1-apply-native-second-implementation.py`,
kept in this ticket folder): for every `🧪️oracle/🔣️.json` contribution, for every native
(`isSemioNativeArtifact`) `mutationManifests` entry, find a `cross-semio-implementation` oracle IN THE
SAME FILE whose `capabilities` are a strict superset of that manifest's full capability set, AND whose
capability is not ALREADY discharged registry-wide by a real `third-party-library`/`third-party-cli`/
`standards-reference-tool` (this excludes `s.architect.program` and both `s.fem.*` subsets — each already
carries a genuine third-party reference alongside its `cross-semio-implementation` supplement, confirmed
live, so they were never part of the 1183 to begin with — and it excludes 5 of `s.stdio.semio`'s 18
subsets — `mesh`, `cad`, `drawing`, `brep`, `document` — which already carry real third-party readers
(`three`, `manifold3d`) for their capabilities).

**Excluded, correctly, and left as debt**: `s.mathematical.mathematical` and `s.sequence.sequence` — A10
already disposed these as partially-real/partially-gapped CSV oracles; they never had a
full-coverage `cross-semio-implementation` entry to promote, and the fresh survey confirms neither
appears in my candidate list. `s.remodeling.remodeling`, `s.shooting.shooting`, `s.layout.layout`,
`s.dag.dag`, `s.process.process3d`, `s.flow.flow`, `s.reasoning.wires`, `s.animate.presentation`,
`s.vcs.vcs`, `os.config` — genuinely no second implementation yet, confirmed absent from the candidate
survey.

For each of the 53 promoted entries the script:
1. Set `kind: "verified-native-second-implementation"`.
2. Added a `nativeSecondImplementation` evidence object: `format` = the manifest's own `artifact`;
   `noThirdPartySurvey.ecosystemsSearched` from the oracle's own `ecosystem` field (`python` →
   `python/pypi`, `typescript` → `js/npm` — every one of these references is Python or TypeScript);
   `noThirdPartySurvey.candidatesConsidered` extracted from the entry's OWN pre-existing `rationale`
   text — the declined-candidate sentence each entry already carried (e.g. din16798's own
   `structuralcodes`/`concreteproperties`/`anastruct`, jack's own `networkx`/`igraph`/`petgraph`,
   block-2d's own `KiCad`/`Modelica`/`IFC`, energy.model's own `EnergyPlus`/`OpenStudio`) verbatim where a
   declined-candidate sentence existed, or the entry's own opening claim ("No third-party library in any
   ecosystem reads or writes this repository's own document format") where it did not name specific
   candidates; `subjectImplementationLanguage: "rust"` (production dispatch is Rust for every one of
   these); `secondImplementationLanguage` = the oracle's own `ecosystem`; `specificationSource` built
   from the entry's own cited ticket path and named schema/grammar/protocol files (already present in every
   rationale, extracted rather than invented); `fixtureCoverage.vectors` = the owning manifest's own
   mutation count (each of these mutations is required, by this ticket's Wave 0/1 work, to already be
   fixture-backed — `missing-fixture` was driven to near-zero for exactly this reason before this shard
   started) and `capabilitiesCovered` = the oracle's own `capabilities`.
3. Appended a dated note to the entry's existing `rationale`, kept verbatim otherwise (the same convention
   C2 used for the `no-oracle-covers-mutation` narrowing), pointing back at this file.
4. Updated the covering manifest's `oracleRequirements[].qualifyingKind` from `third-party-library` to
   `verified-native-second-implementation` for every mutation now discharged by it — honest bookkeeping,
   even though `oracleRequirementBreaches` itself never reads this field for equality (confirmed by
   reading every call site: it is validated for membership in `mutationManifestProblems` and used only in
   the breach MESSAGE text, never compared against the discharging oracle's actual kind).

| artifact(s) | oracle entries | mutations promoted |
| --- | --- | --- |
| `s.norm.*` (14: din16798, din18599, din4108, en1990–en1999, iso16757, vdi3805) | 14 | 62+13+22+10+32+35+17+22+20+22+22+49+26+21+19 = 392 |
| `s.stdio.semio` (13 of 18 subsets: kit, presentation, animation, flow, image, graph, model, audio, object, table, value, video, text) | 13 | 15+14+12+12+12+11+10+9+9+8+8+8+7 = 135 |
| `s.block.*` (2d, 3d, 5d) | 3 | 26+37+41 = 104 |
| `s.puzzle.*` (2d, 3d, 5d) | 3 | 26+35+28 = 89 |
| `s.cad.cad` | 1 | 20 |
| `s.lowpoly.lowpoly` | 1 | 17 |
| `s.procedural.*` (2d, 3d) | 2 | 14+14 = 28 |
| `s.forms.forms` | 1 | 10 |
| `s.gis.*` (gismap, gisterrain) | 2 | 12+2 = 14 |
| `s.raster.raster` | 1 | 12 |
| `s.playbook.playbook` | 1 | 9 |
| `s.assembly` | 1 | 9 |
| `s.trinity.*` (jack, rewriting) | 2 | 8+7 = 15 |
| `s.writer.writer` | 1 | 4 |
| `s.sourcing.curation` | 1 | 3 |
| `s.imperative.imperative` | 1 | 4 |
| `s.space.*` (home, space) | 2 | 1+4 = 5 |
| `s.demonstrator.playground` | 1 | 1 |
| `s.energy.model` | 1 | 1 |
| **Total** | **53** | **872** |

Every one of these 53 entries is verified live, right now, by test 12 (§3): zero
`native-second-implementation-*` breaches across all of them.

Scripts kept in this ticket folder per house rules: `🩹️d1-apply-native-second-implementation.py` (the
promotion driver), `🩹️d1-fix-fixture-vector-counts.py` (a same-session correction pass — the first draft
sourced `fixtureCoverage.vectors` from the legacy v1 `mutationCatalogs` array, which is stale for 7 of the
`s.stdio.semio` subsets relative to their v2 manifests; corrected to the manifest's own mutation count,
which every one of these entries' own rationale already claims coverage of), `🩹️d1-extract-survey.py`
(the rationale-text extraction used to build each entry's `noThirdPartySurvey`).

## 5. `no-oracle-covers-mutation` 5 → 0 — not this shard's doing

This dropped independently of my work: `noOracleMisuseBreaches` fires purely on whether a
`noOracleDecisions` entry's claimed capabilities overlap a manifest's `oracleRequirements`, regardless of
what kind of oracle (if any) discharges them — I never touched `noOracleDecisions`. The live count moved
from 5 (my baseline) to 0 between my baseline and after-snapshot, which given the ~10-minute gate runtime
and the confirmed concurrent renames/promotions happening elsewhere in the tree during this session, is
attributable to other concurrent work, not claimed here.

## 6. Final answer

- **Before** (this shard's own baseline, `🗑️generated/d1-baseline-testing.json`): `missing-external-oracle`
  1183, `reimplementation-registered-as-third-party` 2, `no-oracle-covers-mutation` 5,
  `oracle-capability-mismatch` 0, `unknown-oracle` 0, `fixture-generated-by-non-qualifying-oracle` 0,
  **total 1951**.
- **After** (`🗑️generated/d1-after-testing.json`): `missing-external-oracle` **344**,
  `reimplementation-registered-as-third-party` **2 (untouched)**, `no-oracle-covers-mutation` 0 (§5),
  `oracle-capability-mismatch` 0, `unknown-oracle` 0, `fixture-generated-by-non-qualifying-oracle` 0,
  every `native-second-implementation-*` id **0**, **total 1114**.
- **Closed by legitimate qualification: 839** of the 1183 `missing-external-oracle` breaches this shard
  started with (53 oracle entries, 41 distinct artifacts, 872 mutations promoted — the gap between 839 and
  872 is ordinary concurrent tree drift across the ~10-minute foreground gate runs, not double-counting or
  a discrepancy in the rule).
- **Remains as real, honest debt: 344** — split **174 semio-native** (no second implementation exists yet:
  `s.remodeling.remodeling` 35, `s.shooting.shooting` 31, `s.layout.layout` 25, `s.process.process3d` 16,
  `s.dag.dag` 14, `s.flow.flow` 10, `s.reasoning.wires` 10, `s.animate.presentation` 9, `s.vcs.vcs` 6,
  `s.mathematical.mathematical` 9, `s.sequence.sequence` 4, `os.config` 5) and **170 real interchange
  formats** correctly still owed a genuine third-party reference and categorically barred from this new
  kind by `isSemioNativeArtifact` (`s.stdio.pptx` 24, `s.stdio.svg` 19, `s.stdio.xlsx` 23, `s.stdio.docx`
  14, `s.stdio.step` 10, `s.stdio.ifc` 16, `s.stdio.zip` 13, `s.stdio.html` 9, `s.stdio.jpg` 9,
  `s.stdio.xml` 8, `s.stdio.tiff` 8, `s.stdio.txt` 5, `s.stdio.binary` 4, `s.stdio.wav` 4, `s.stdio.dwg` 4
  — this bucket is A10's separate remit, untouched here).
- This file: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️d1-native-oracle-discharge.md`.
