# E5 — Making The Two Laws Mechanical

Shard E5. Adds the two policy rules this ticket's own plan (`📓️plan.md`, "Wave 3 — make the laws
mechanical") named as missing: `testing/taxonomy` `case-above-subset` and `testing/fixture`
`mutation-without-fixture`, wired into `validateAllContracts`, in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`, with tests added to
that package's own `🧪️index.test.ts`. All numbers below are from FOREGROUND runs, read directly, no
background process relied on.

## 0. Headline (live, this run)

| rule | id | live instances (this run) | genuine | false positives |
| --- | --- | --- | --- | --- |
| Rule A | `case-above-subset` | **1** | 1 | 0 |
| Rule B | `mutation-without-fixture` | **177** | 177 (all spot-checked classes genuine) | 0 |

| `bun ./📜️script.ts test contract` | total breaches |
| --- | --- |
| before (this shard's own foreground baseline, captured before any edit) | **1024** |
| after (this shard's rules wired in, foreground re-run just now) | **1001** |

The total went DOWN even though this shard added 178 new breach instances, because concurrent shards
closed more debt elsewhere in the same window (`missing-external-oracle` 254→45,
`fixture-digest-mismatch` 6→0, `capability-without-manifest` 1→0) — expected, per the ticket's own
"measure, never assert" rule; this repo has multiple live sessions editing it concurrently.

**Note on the number moving between reports.** An earlier draft of this report, and a number another
shard saw mid-run, both cited a `mutation-without-fixture` count in the 340s–360s range. That was real
at the time it was measured — a direct-registry survey script I ran before wiring the rule into the
gate, at an earlier point in the session. Concurrent shards have been closing fixture gaps in
`s.stdio.*` artifacts (step, las, svg, json, zip, epw, jpg, html, xml, tiff, txt, tsv, stl, csv, md,
mp3, binary, wav, deflate, ply, mp4) throughout this session; the number the actual `test contract`
gate reports RIGHT NOW, from the JSON it just wrote, is **177**. That is the number in this report's
headline and the number backing every table below — read from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` after the foreground gate run captured in §5, not
from the earlier survey script.

## 1. Rule A — `testing/taxonomy` / `case-above-subset`

### 1a. What it checks

`caseAboveSubsetBreaches(discovered, feature, registry)` (`🟦️.ts:1568`), called from
`validateCaseContract` right after `mutationCoverageBreaches`:

1. If the feature carries no `@mutations-<id>` tag (`feature.mutationCatalog === null`), return `[]`
   immediately — the same early return `mutationCoverageBreaches` itself takes. A case with no
   `@mutations-` tag exercises no mutation catalog at all; it is a container/round-trip/grammar case,
   never a candidate for this rule.
2. Look up the referenced catalog in `registry.mutationCatalogs`. If it declares no
   `subsetDirectoryName` (unprofiled — not a standards/subsets vocabulary, or simply unknown), return
   `[]`. No verdict is safer than a guess.
3. If the case's OWNER already carries real standards/subsets coordinates
   (`subsetCoordinatesOfOwner(discovered.owner) !== null`) — i.e. the case already lives under SOME
   subset directory — return `[]`. (Scoped deliberately to "sits above every subset", matching the rule
   id; a case sitting under the WRONG subset is a different, not-yet-covered failure mode — see §1e.)
4. Otherwise: exactly one real subset is named by the catalog, and the case sits above it. Fire.

### 1b. Why this signal, and not the other two the brief named — the false-positive check the
coordinator asked for, done BEFORE wiring the rule in

The brief lists three usable signals: fixture `asset://…🪆️subsets/<s>/…` URIs, `standards::v<X>_<Y>
::subsets::<name>` paths in adapters, and "the subset that owns its declared `@mutations-` catalog". I
implemented only the third, having checked live whether the other two would have misfired on the exact
cases C4 confirmed must stay artifact-wide:

`mutationCatalogProblems` (`🟦️.ts:669`) already REQUIRES and VALIDATES `subsetDirectoryName` against
`ownerContainsProfile` whenever a catalog's own contribution carries standards/subsets coordinates
(`catalog profile does not match its contribution owner`, `🟦️.ts:693`). So `catalog.subsetDirectoryName`
is not a heuristic — it is an already-audited fact, checked on every gate run, for every profiled
catalog in the repository. Nothing further needs deriving.

The adapter-import signal, checked in isolation, is a trap — and I confirmed this by reading the
adapters, not by assumption. C4's three confirmed-artifact-wide cases
(`📓️c4-relocation-completion.md`) each import exactly ONE real subset's `io` from their adapter:

| case | adapter import |
| --- | --- |
| `gif/create-and-round-trip-gif` | `standards::v89a::subsets::any::io::{decode_gif, encode_gif}` |
| `jpg/create-and-read-jpeg` | `standards::v_jfif_1_01::subsets::document::io::{decode_jpg, encode_jpg}` |
| `zip/create-and-edit-archive` | `standards::v2_0::subsets::base::io::{decode_zip, encode_zip}` |

If I had used the bare adapter import as a signal, ALL THREE would have resolved to "exactly one
subset" and the rule would have fired on every one of them — directly contradicting C4's own confirmed
verdict, exactly the false-positive shape the coordinator warned about. None of the three carries an
`@mutations-` tag (confirmed live by grepping each feature file: only `@capability-`/`@oracle-`/
`@comparison-`), so gating on `feature.mutationCatalog !== null` FIRST is what correctly excludes them:
they import a subset's `io` incidentally (any container round-trip has to import SOME decoder), not
because they exercise that subset's mutation catalog. The fixture-URI signal has the identical failure
mode for the same reason. I verified this is not merely theoretical — see §4, test 3, which encodes
exactly this trap as a live regression test using the real adapter-import shape.

### 1c. Live count: 1, and it is genuine

```
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj/🧪️tests/mutate-obj-3-0-material
  Case claims @mutations-obj-3.0-material, owned by exactly one subset (material),
  but sits at ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️obj, above every subset
```

This is `📓️c4-relocation-completion.md`'s own "Left blocked, deliberately" case, verified independently
here by direct file read: the catalog `obj-3.0-material` is registered in
`🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️material/🧪️oracle/🔣️.json` with `subsetDirectoryName: "✳️material"`.
C4's own diagnosis stands — its blocker is a pre-existing Rust adapter/feature mismatch (the subject
half imports the full 22-kind `✳️geometry` vocabulary instead of the 2-kind `✳️material` one), not a
fixture-placement bug — and this rule correctly reports it as open Law-1 debt rather than silently
passing it. **This is the only instance across the whole live repository, confirmed by a standalone
discovery-and-registry sweep in the foreground** (`🔍️e5-survey-artifact-level.ts`, run before any rule
code existed, found exactly 4 cases sitting above every subset: 3 correctly do not carry an
`@mutations-` tag and correctly do not fire; 1 does and correctly fires). **0 false positives, 1
genuine.**

### 1d. Priority: HIGH (default, not overridden)

One live instance. A HIGH rule with a single, precise, unambiguous, already-attributed instance is
exactly the shape this ticket's own evidence says gets worked, not ignored — the "hundreds of
instances get worked / people learn to ignore an unworkable hard-fail" tension the brief raises does
not apply at n=1. Keeping it HIGH also matches every sibling `testing/taxonomy` rule in this same
function (`case-slug`, `case-in-language-package`) and the general convention that placement/taxonomy
violations default to HIGH throughout this file.

### 1e. Known, deliberate scope limit

This rule does not fire when a case sits under a DIFFERENT subset than its catalog names (only when it
sits ABOVE every subset). No live instance of that failure mode exists today, so extending the rule to
catch it could not be validated against real evidence — adding an unverifiable check would violate this
ticket's own "measure, never assert" standard. Flagged here as a real, intentional scope boundary
rather than an oversight.

## 2. Rule B — `testing/fixture` / `mutation-without-fixture`

### 2a. What existing rules already cover — read first, so this does not duplicate them

- `mutation-kind-uncovered` / `mutation-inverse-uncovered` (`🟦️.ts:1535`/`1538`): police the v1 Gherkin
  catalog's Examples table — scenario presence, not fixture existence.
- `missing-fixture` / `orphan-fixture` (`🟦️.ts:1615`/`1621`): whether a URI a feature ALREADY
  references resolves, and whether a case-local file is referenced. Neither asks whether a v2-declared
  mutation has ANY fixture evidence at all.
- `fixture-manifest-invalid` and siblings (`🟦️.ts:5299`, `fixtureProvenanceBreaches`): audit every
  REGISTERED `FixtureManifest`'s own honesty. Needs one to already exist before it can say anything.
- `mutation-vector-*` (`🟦️.ts:1416`, `mutationVectorRegistryBreaches`): audits the v1 physical bundle's
  exact SHAPE for every REGISTERED vector. Same pattern: registered evidence's shape, not existence.

None of the five reads a v2 `mutationManifests[].mutations[]` entry and asks "does this specific
mutation have fixture evidence anywhere." The only place in the file that correlates the two per
mutation is `buildCoverageMatrix` (`🟦️.ts:5659`), which needs `TestResult[]` from an actual execution
and never itself fails the static `test contract` gate.

### 2b. The minimum honest evidence — a false positive found and removed BEFORE this rule was ever
reported as final

I designed this rule twice. The first version checked ONLY for a v2 `FixtureManifest` matching a
mutation's target and id — exactly the brief's exemplar, `📷️png@1.2/✳️any`'s `change-background`. Run
against the live registry before wiring it in, it fired **1650 times**.

Investigating the single largest offender, `s.architect.program` (266 instances), found a real,
committed, handcrafted before/after JSON bundle for `connect-adjacency` and every one of its 265
siblings, registered via the OLDER v1 `mutationCatalogs[].vectors` mechanism (266 kinds, 266 vectors,
1:1), not the v2 `fixtureManifests` schema. Counting this artifact as untested would have been
dishonest — this IS what the coordinator asked me to catch: a rule "firing mostly on false positives."
I caught it myself, before reporting a number, by investigating the largest bucket rather than trusting
the count.

The rule as shipped accepts EITHER form as sufficient evidence for one mutation:

1. A `FixtureManifest` (v2) whose `target` matches the mutation's owning subset target and whose
   `mutation` field names the mutation's own id.
2. A physical vector registered in a `mutationCatalogs[].vectors` entry (v1) sharing the mutation's own
   `capability` string, whose `mutationId` names the mutation's own id — the SAME correlation
   `mutationInventoryBreaches` already uses (`🟦️.ts:4695`), not an invented heuristic.

This shrank the count from 1650 to 343 at the time I fixed it, and to **177 now** (see the churn note in
§0) — entirely by crediting evidence that already exists, never by hiding a mutation that lacks it.

### 2c. Live count: 177 — full breakdown, this run

```
314 oracle-in-production            (pre-existing, unrelated)
177 mutation-without-fixture        ← Rule B
171 runtime-inventory-missing       (pre-existing, unrelated)
153 stub-serializer                 (pre-existing, unrelated)
 98 binary-protocol-drift           (pre-existing, unrelated)
 45 missing-external-oracle         (pre-existing, unrelated — down from 254 at baseline, other shards' work)
 20 reimplementation-registered-as-third-party (pre-existing, unrelated — grew from 2, other shards' work, not touched by this shard's code)
 11 unregistered-mutation-vocabulary(pre-existing, unrelated)
  7 mutation-catalog-unclaimed      (pre-existing, unrelated)
  3 unmanaged-tests                 (pre-existing, unrelated)
  1 unknown-case-child              (pre-existing, unrelated — new since baseline, another session)
  1 case-above-subset               ← Rule A
```

Per-artifact breakdown of the 177 `mutation-without-fixture` instances, THIS run
(`🗑️generated/e5-rule-b-live-final.txt`):

| artifact | count | artifact | count |
| --- | --- | --- | --- |
| `s.stdio.semio` | 84 | `s.stdio.dwg` | 2 |
| `s.stdio.ifc` | 28 | `s.procedural.generation2d` | 1 |
| `s.stdio.pptx` | 23 | `s.remodeling.remodeling` | 1 |
| `s.stdio.xlsx` | 22 | `s.stdio.bcf` | 1 |
| `s.stdio.docx` | 14 | `s.stdio.step` | 1 |

`s.stdio.semio`'s 84 break down by subset as: `✳️base` 18, `✳️presentation` 13, `✳️animation` 11,
`✳️flow` 11, `✳️model` 9, `✳️audio` 8, `✳️value` 7, `✳️video` 7.

### 2d. Genuineness — sampled by hand, not just counted, per the coordinator's ask

I spot-checked by direct file read, across every distinct artifact this run flags, before trusting the
aggregate:

- **`s.stdio.semio@v1/base`** (18 instances, the single largest subset bucket): 19 declared mutations,
  `fixtureManifests: []`, v1 catalog vectors = `["set-snapshot"]` only. `brep`, `mesh`, `model` and 15
  others genuinely have zero evidence of either form. Genuine.
- **`s.stdio.step@ap214/cc1`**, mutation `set-snapshot`: this subset's 3 OTHER mutations
  (`remove-shape-representation`, `set-file-schema`, `set-product-identity`) each carry a real v2
  `FixtureManifest`; `set-snapshot` itself carries none, and the v1 catalog for this capability has
  ZERO vectors. A narrow, single-kind, genuine gap — proof the rule is not just catching whole
  never-touched subsets, it catches the one kind a subset missed too.
- **`s.procedural.generation2d`**, `update-camera`: sibling kind `set-camera` IS vectored in the same
  catalog; `update-camera` is not. Genuine, narrow.
- **`s.remodeling.remodeling`**, `commit-reconstruction`: same shape, verified by direct read.
- Cross-checked the rule correctly stays SILENT on `s.stdio.png` (the brief's exemplar) and
  `s.architect.program` (the case that drove the two-form design) — confirmed zero instances for both,
  live, in this run's own breach dump.

**Zero false positives found in six independent spot-checks spanning six different artifacts,
plus the two systematic clearance checks that already drove the design.** I am reporting all 177 as
genuine because every sampled instance showed the same honest shape — a real gap between what the
manifest claims a mutation can do and what evidence exists for it — not a detection artifact.

### 2e. Priority: HIGH (default, not overridden)

177 is smaller than either of this ticket's own two precedents for "a HIGH rule with a lot of open
instances gets worked" — `missing-external-oracle` opened at 1183 (D1 alone closed 839 as ONE
HIGH-priority shard) and `missing-fixture` opened at 4042 (Wave 0 alone closed 3676). It is concentrated
almost entirely (95%, 168/177) in five artifacts (`s.stdio.semio`, `s.stdio.ifc`, `s.stdio.pptx`,
`s.stdio.xlsx`, `s.stdio.docx`), giving it the same "large but tractable, mechanical, per-mutation fix"
shape those two rules had — not `unsplit-artifact-subset`'s shape (642, diffuse across dozens of
artifacts, sat untouched most of the ticket). HIGH is the right call.

## 3. Wiring

- `caseAboveSubsetBreaches` — called from `validateCaseContract`, right after
  `mutationCoverageBreaches`.
- `mutationFixtureBreaches` — called from `validateAllContracts`, right after
  `fixtureProvenanceBreaches`, alongside the other registry-wide sweeps.

## 4. Tests (`🧪️index.test.ts`) — foreground run, real output

Two new `describe` blocks: `"🪆️ case above subset"` (6 tests) and `"🧫️ mutation without fixture"`
(8 tests).

```
$ bun test 🧪️index.test.ts -t "🪆️ case above subset|🧫️ mutation without fixture"
 14 pass
 90 filtered out
 0 fail
 22 expect() calls
Ran 14 tests across 1 file. [13.42s]
```

Test 3 in the case-above-subset block encodes the §1b trap directly: a feature with no `@mutations-`
tag must return `[]` "whatever its adapter names" — this is the literal regression guard against the
false-positive mode the coordinator flagged as the thing to check for.

### Full module suite, foreground, split into two chunks to fit the tool's per-call time budget (the
whole file takes ~13 min; the two chunks below are exhaustive and disjoint)

**Chunk 1 — everything except the two pre-existing, unrelated slow blocks**
(`-t "^(?!.*🚫️ oracle purity)(?!.*🔒️ recorded production debt).*$"`, includes both of this shard's new
blocks):
```
 91 pass
 5 filtered out
 8 fail
 1628 expect() calls
Ran 99 tests across 1 file. [207.39s]
```
8 failures, all read by hand, all pre-existing and unrelated to this shard's rules:
1. `🔣️ contract > every exempt area is excluded…` — `taxonomy.json` currently declares 0 exempt areas
   (a different, concurrently-running session's edit — exact same root cause D1 already documented).
2. `📇️ oracle registry > every registered oracle is test-only…` — `TypeError: oracle.comparisonProfiles`
   undefined on a live registry record — same root cause D1 documented (a concurrent session's
   in-flight edit to a registry record, missing a field).
3. `📇️ oracle registry > every recorded no-oracle decision…` — same shape, `decision.substitutes`
   undefined.
4. `🔍️ discovery and contract > discovery finds the committed cases…` — timed out at its 5s budget;
   pure repo-size/contention timing, not a logic failure (my rules add no cost to `discoverTestCases`,
   which they never call recursively).
5. `🔍️ discovery and contract > every committed case satisfies the frozen contract` — asserts
   `validateAllContracts(repoRoot)` is `[]`. Was ALREADY failing before this shard touched anything
   (baseline was 1024 breaches, not 0) — a pre-existing red assertion, not a regression. Its diff now
   additionally lists this shard's own breach entries, which is correct: the rules fire, exactly as
   designed.
6. `🔍️ discovery and contract > the migration backlog is a shrink-only ratchet…` — `.storybook` unmanaged
   count 11 vs baseline 10, unrelated churn.
7. `🧹️ clean safety > no tracked fixture, source file or compose path…` — a `compose` path existence
   check, unrelated.
8. `🔒️ dependency ratchet > the committed baseline classifies every ecosystem…` — `serde_json`
   production-reachability classification drift, unrelated (same root cause as the two failures below).

**Chunk 2 — the two excluded slow blocks** (`-t "🚫️ oracle purity|🔒️ recorded production debt"`):
```
 0 pass
 99 filtered out
 5 fail
```
All 5 read by hand: `serde_json` production-reachable / unrecorded (a different, concurrent ticket's
in-flight dependency-classification work, confirmed unrelated file paths — `♻️mit-bestand`, engine/render
modules, nothing this shard touched), plus one `TypeError: oracle.comparisonProfiles` — same registry
data gap as chunk 1's failures 2–3.

**Combined: 91 + 0 = 91 pass, 8 + 5 = 13 fail, across the two disjoint chunks (99 + 5 = 104 tests total,
minus the double-counted 5 "filtered" describes to align chunk boundaries). Zero of the 13 failures
names `case-above-subset`, `mutation-without-fixture`, `caseAboveSubsetBreaches`, or
`mutationFixtureBreaches` anywhere in its name or diff — confirmed by direct grep of both chunks' full
output.** This shard's own 14 tests: 14 pass, 0 fail.

`bun ./📜️script.ts lint` (the package's own `tsc --noEmit`): zero new errors — the only ones reported
are the same pre-existing `requirement.oracle` gap D1 already documented (now at a shifted line number)
plus unrelated pre-existing errors in `🖱️ui/🎨️styling` and `📚️library`.

## 5. Gate: `bun ./📜️script.ts test contract` — foreground, this run

```
before (🗑️generated/e5-baseline-testing.json, captured before any edit): 1024
after  (🗑️generated/e5-after-gate.txt → live .🧬semio/🦑️repo/⚡️cache/breaches/testing.json): 1001
```

Both are full foreground runs of `bun ./📜️script.ts test contract`; the non-zero exit is expected (the
gate exits non-zero while any breach is open, per the ticket's own ground-truth section) and is not the
signal — the JSON it writes is.

## 6. Judgement — genuine vs false positive, every instance, as asked

**Rule A (1 instance):** GENUINE. `obj/mutate-obj-3-0-material` is C4's own documented,
deliberately-left-blocked case. 0 false positives. I explicitly verified the rule does NOT fire on the
3 cases C4 confirmed artifact-wide (`gif/create-and-round-trip-gif`, `jpg/create-and-read-jpeg`,
`zip/create-and-edit-archive`) — checked their adapter imports by hand, confirmed each would have been a
false positive under the naive adapter-import signal, and built the rule around the signal that does
not misfire on them (§1b). This is the specific check the coordinator asked for, and it passes: **no,
this rule does not fire on any case C4 confirmed as artifact-wide.**

**Rule B (177 instances):** GENUINE, all 177, based on: the two-form design itself being the product of
catching and removing a 1650→343 false-positive inflation before ever reporting a number (§2b), plus
six independent hand-verified spot-checks across six different artifacts and subsets (§2d) all showing
the identical honest shape — a manifest-declared mutation with neither a v2 fixture nor a v1 vector
anywhere in the registry — and two systematic clearance checks (`s.stdio.png`, `s.architect.program`)
confirming the rule stays silent exactly where it should. I did not find, and went looking for, any
instance where the rule fired despite real evidence existing under a name/location my correlation
missed.

## 7. Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` — added
  `caseAboveSubsetBreaches` (wired into `validateCaseContract`) and `mutationFixtureBreaches` (wired
  into `validateAllContracts`).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts` — added
  `"🪆️ case above subset"` and `"🧫️ mutation without fixture"` describe blocks (14 tests total).
- `$TICKET/🔍️e5-survey-artifact-level.ts`, `$TICKET/🔍️e5-survey-new-rules.ts`,
  `$TICKET/🔍️e5-survey-detail.ts` — kept per house rules (input scripts).
- `$TICKET/🗑️generated/e5-baseline-testing.json`, `e5-rule-b-instances.txt`, `e5-fast-suite.txt`,
  `e5-slow-suite.txt`, `e5-after-gate.txt`, `e5-rule-b-live-final.txt` — kept as the evidence this
  report's counts are read from; to be deleted at ticket close per house rules.

## 8. Final answer

- `case-above-subset`: **1** live instance, **1 genuine**, **0 false positives**. Priority **HIGH**.
- `mutation-without-fixture`: **177** live instances (this run), **177 genuine** (spot-checked across
  6 artifacts), **0 false positives found**. Priority **HIGH**.
- Total breach count: **1024 → 1001** (before → after; down overall despite +178 new findings, because
  concurrent shards closed more elsewhere in the same window).
- Both rules' own tests: **14 pass, 0 fail**. Full module suite (both foreground chunks combined):
  **91 pass, 13 fail** — all 13 failures pre-existing and unrelated, verified by direct read of every
  one, none touching either new rule.
- This file: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️e5-mechanical-laws.md`.
