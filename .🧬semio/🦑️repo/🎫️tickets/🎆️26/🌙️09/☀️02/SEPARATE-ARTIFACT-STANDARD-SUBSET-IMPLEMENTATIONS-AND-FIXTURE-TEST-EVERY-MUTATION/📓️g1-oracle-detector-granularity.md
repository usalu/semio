# G1 — `reimplementation-registered-as-third-party` Made Entry-Granular

Shard G1. Territory: the file owned this wave —
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` (and its test file)
— specifically `reimplementationOracleBreaches`, which E2 diagnosed as the cause of
`reimplementation-registered-as-third-party` rising 2 → 20. Also disposed the honest remainder of
`missing-external-oracle` (45) assigned to this shard.

## 0. Headline

| id | before | after |
| --- | --- | --- |
| `reimplementation-registered-as-third-party` | **20** | **0** |
| `missing-external-oracle` | 45 | 45 (unchanged — genuine debt, re-verified, see §3) |
| `oracle-in-production` | 301 | 301 (**not risen** — the hard constraint) |
| `oracle-capability-mismatch` | 0 | 0 |
| `oracle-profile-mismatch` | 0 | 0 |
| `unknown-oracle` | 0 | 0 |
| `fixture-generated-by-non-qualifying-oracle` | 0 | 0 |
| `fixture-generator-unregistered` | 0 | 0 |
| `native-second-implementation-*` (all ids) | 0 | 0 |
| **TOTAL breach count** | **805** | **784** |

Both numbers are from live foreground `bun ./📜️script.ts test contract` runs, read back from
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`: before is `🗑️generated/g1-before-testing.json`
(captured immediately before this shard's edit — the cache's own mtime, and its
`reimplementation-registered-as-third-party` count of exactly 20, confirm it reflects the pre-edit
state), after is `🗑️generated/g1-after-testing.json` (captured after the detector fix, via a full
foreground `test contract` run). The only two ids that moved between the two snapshots are
`reimplementation-registered-as-third-party` (20 → 0, this shard) and `mutation-without-fixture`
(5 → 4, a concurrent shard's unrelated work, per the ticket's own "other sessions are editing the same
tree" rule). Every other id — including `oracle-in-production`, the hard constraint — held exactly
still.

## 1. Verifying the 18-versus-2 split myself, before acting on it

E2's diagnosis: `reimplementationOracleBreaches` only opens a contribution's `🧪️oracle/🦀️.rs` once
that contribution has at least one qualifying oracle, then — if the file's text admits or predicts a
mutation's result — flags **every** qualifying oracle in the contribution, including ones the file
says nothing about. I did not take this on faith. I dumped the live 20 breaches
(`🔍️g1-dump-breaches.ts`, kept in this ticket folder) and, for each, cross-referenced the flagged
owner's own `🧪️oracle/🔣️.json` and `🧪️oracle/🦀️.rs`:

- **Every one of the 20 flagged qualifying entries has `ecosystem` `python` or `javascript`** — never
  `rust` — e.g. `python-pptx-pptx-ecma-376-strict-mutate-reader` (python), `yauzl-zip-2-0-base-mutate-
  reader`/`yazl-zip-2-0-base-mutate-writer` (javascript), `ifcopenshell-ifc-2x3-base-differential`
  (python), `steputils-ifc-4-any-mutate-reader` (python).
- **Every one of those owners' `🦀️.rs` files independently contains a real, hand-written mutation
  dispatch** — a `match kind { … other => Err(format!("mutation kind {other:?} has no oracle
  implementation")) }` shape — computing the mutation's expected RESULT itself: verified directly by
  reading `pptx/strict`, `ifc/2x3/base`, `zip/base`, `step/base`'s `🦀️.rs` (grep excerpts in this
  session's own log; `pptx/strict`'s `oracle_inverse_spec` doc comment literally says "read out of
  `base` by the independent implementation alone" — the exact phrase the `admits` regex matches).
- **Every one of those owners' `🔣️.json` already carries a SEPARATE `cross-semio-implementation`
  entry** (ecosystem `rust`) whose own rationale documents shard A10's original reclassification —
  e.g. `pptx-ecma-376-strict-mutate`, `zip-2-0-mutate`, `ruststep-ifc-2x3-base-mutate` — proving the
  Rust reimplementation and the third-party reader are two DIFFERENT registry entries that merely
  share a file, not one entry wearing two hats.
- The genuine reader's own rationale text records real, measured verification against this
  repository's own committed fixture (E2's transcript: real slide counts, real namespaces, real
  `IfcBuildingStorey` records, real DEFLATE compression methods) — none of the 20 is a cosmetic
  registration.

**Verdict: all 20 (the 18 E2's registrations activated, plus A10's original 2) are genuine third-party
references, none is a re-implementation dressed as third-party.** This matches E2's own claim exactly,
now independently confirmed by ecosystem cross-reference rather than by re-reading the same doc
comments E2 already quoted.

## 2. The fix — entry-granular, keyed on `ecosystem`

`reimplementationOracleBreaches` (`🟦️.ts:5145`) previously read: any qualifying oracle in a
contribution → open the shared `.rs` file once → if it admits/predicts, flag **all** qualifying
oracles in that contribution by id.

The fix filters to `implicable = qualifying.filter((oracle) => oracle.ecosystem === "rust")` **before**
even reading the file, and only that filtered set is opened for reading and named in the breach
message. The rule's own literal wording — "a specific registered oracle entry is a re-implementation
when the code that answers for THAT entry predicts mutation output in **our own Rust**" — makes this
the correct predicate, not a heuristic approximation of it: an entry whose own `ecosystem` is not
`rust` executes in a runtime this shared `.rs` file never touches (a Python venv, a Node subprocess),
so the file's content is categorically not evidence about that entry, however incriminating the same
file is about a Rust sibling sitting in the very same contribution. I considered E2's own two
suggested fixes (§5 of `📓️e2-interchange-format-oracles.md` — exempt on the entry's own `rationale`
text, or scan for the entry's `id` mentioned near the predicting code) and rejected both: `rationale`
is free text an author writes and could write dishonestly (gameable, and not itself proof); scanning
for the entry's own id verbatim in the file fails on real data — I checked `pptx/strict`'s `🦀️.rs` and
it never mentions any oracle id at all, by either entry. `ecosystem` is a structural, non-gameable fact
already required on every `OracleEntry`, and it is exactly true for a genuine cross-language reference:
the code answering for a Python or JavaScript reader is provably not "our own Rust."

I did **not** widen this to exempt `ecosystem: "native"` or `"typescript"` speculatively — no currently
flagged (or previously flagged) entry uses either, so there was nothing in the live data to justify
generalizing past what `rust` already, precisely, explains. `admits`/`predicts`/`judgedByProbes` and
the breach shape are otherwise untouched.

Diff: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`, inside
`reimplementationOracleBreaches` — replaced the unconditional `qualifying` filter used for the file
gate and the breach message with `implicable` (rust-ecosystem qualifying entries only); added a
doc-comment paragraph explaining the entry-granularity and why `ecosystem` is the correct
discriminator.

## 3. Why this does not weaken the rule — proof the mechanism still catches a real one

Nothing about `admits`/`predicts`/`judgedByProbes` changed. A genuinely mislabeled Rust
reimplementation is, by construction, `ecosystem: "rust"` (it is registered against the very crate the
file wraps — `quick-xml`, `ruststep`, `html5ever`, …), so it always survives the new `implicable`
filter and is caught exactly as before. This is not merely argued — it is what shard A10's original 20
reclassifications already were (all `ecosystem: "rust"`), and it is proven fresh by three new unit
tests in `🧪️index.test.ts` (`describe("🎯️ reimplementation-registered-as-third-party is
entry-granular, not file-granular")`), each against a real temp-directory `🦀️.rs` file (not a mock):

1. **POSITIVE** — a `third-party-library` entry with `ecosystem: "rust"`, in a file carrying the real
   `apply_kind`/`"has no oracle implementation"` shape, still fires
   `reimplementation-registered-as-third-party` and names that entry.
2. **NEGATIVE** — the identical file, but the ONLY qualifying entry has `ecosystem: "python"`: zero
   breaches. This is the exact shape of all 20 of this ticket's flagged owners.
3. **BOUNDARY** — one contribution, one shared file, TWO qualifying entries (one `rust`, one
   `python`): exactly one breach fires, and it names only the `rust` entry — the python sibling is
   never named. This is the direct proof of entry granularity: same owner, same file, same detector
   invocation, different verdict per entry.

```
$ bun test 🧪️index.test.ts -t "entry-granular"
 3 pass
 104 filtered out
 0 fail
 7 expect() calls
Ran 3 tests across 1 file. [18.24s]
```

`bun ./📜️script.ts lint` (package `tsc --noEmit`): zero new errors. The same pre-existing errors D1
already documented remain, unchanged by this diff — `🟦️.ts:5880` (`requirement.oracle`, a field never
declared on `OracleRequirement`, present at `HEAD` before this shard touched anything), plus unrelated
pre-existing errors in `📡️replication`, `🖱️ui/🎨️styling`, `📚️library`.

Full package suite, `bun test 🧪️index.test.ts` (all 107 tests, foreground, 622s):
```
 92 pass
 15 fail
 1591 expect() calls
Ran 107 tests across 1 file. [622.36s]
```
Full log kept at `🗑️generated/g1-full-suite.txt` (deleted per house rules once this ticket closes; this
run also carries the package's growth since D1's 90-test/12-fail snapshot — other shards added tests
concurrently, including this shard's own +3). All 15 failures read individually — every one is
pre-existing and unrelated to this diff; confirmed by grepping the full log for `reimplementation`,
`entry-granular` and `implicable` (this diff's own vocabulary): **zero matches in any failure block**:

- `every exempt area is excluded by the discovery library itself` — `taxonomy.json` currently declares
  zero `exempt` areas (a concurrent session's edit, matching D1's own documented pre-existing failure
  verbatim).
- `every registered oracle is test-only and declares its license and capabilities` /
  `every recorded no-oracle decision names its rationale and its substitutes` — `TypeError: undefined
  is not an object` on `oracle.comparisonProfiles`/`decision.substitutes`: a live registry record
  missing a field entirely, a data issue in another owner's `🔣️.json`, not a value mismatch and not
  touched by this diff (this diff edits zero `🔣️.json` files).
- `discovery finds the committed cases and never returns a compose path` — timed out at the default 5s
  bun budget; unrelated to detector logic, plausibly slow from concurrent load (this run overlapped
  with the repo-wide `test contract` gate run in §0).
- `every committed case satisfies the frozen contract` — `validateAllContracts` returning a non-empty
  list is expected while ANY breach anywhere in the repo is open (784 remain, nearly all other shards'
  territory), per the ticket's own "it exits non-zero... that is NOT your signal" rule — not a
  regression.
- `the migration backlog is a shrink-only ratchet` / `no tracked fixture, source file or compose path is
  ever a clean candidate` / `the committed baseline classifies every ecosystem it tracks and keeps
  oracles out of production` — none mention `reimplementation`, `ecosystem`, or
  `nativeSecondImplementation` in their diff; unrelated registry/ratchet-baseline drift from concurrent
  shards.
- `the only live case-above-subset violation is the one C4 documented as deliberately blocked` — a
  `s.stdio.obj` violation another shard evidently closed or reopened since C4's snapshot; nothing to do
  with oracle classification.
- `no production source imports a registered oracle` / `narrowing a run to one case must not make other
  cases' adapters look like production source` — the same `requirement.oracle`-rooted cascading
  false-positive list D1 already traced to `HEAD` before this ticket started (dominated by
  `serde-json-equation-carrier-reader` hits across `puzzle/2d`/`puzzle/5d`, an unrelated artifact this
  shard never touched).
- `the committed baseline classifies every ecosystem it tracks and keeps oracles out of production` —
  `serde_json` (linked by `serde-json-equation-carrier-reader`) classifies as
  `production-build`/`production-runtime`/`test-runner` in `🔒️dependencies.json` rather than
  `test-oracle` — the exact "serde_json reachability drift" the brief told me to expect, in the
  `equation` plugin I never registered an oracle for.
- `the committed baseline classifies every external host package as a test-only dependency` —
  `python:pypdf` is on a generated host's import path but absent from `🔒️dependencies.json` — a `pdf`
  plugin dependency-baseline gap, unrelated to any format this shard touched.
- `an oracle claiming testOnly while already production-reachable must record the debt` /
  `only the recorded paths are excused` — both root in the same `serde_json` production-reachability
  drift, plus one unrelated import at
  `♻️mit-bestand/recherche/_neo4j/review/2026-08_akteursnetz_faktencheck/beziehungsprofil_review/
  audit_sources.py` — a research script entirely outside this ticket's plugin tree.
- `every registered oracle names its capabilities, comparison profiles and a rationale that scopes it` —
  the same missing-`comparisonProfiles`-field registry record as §3's `oracle registry` failure above.

None of the 15 failure blocks contain `reimplementation`, `entry-granular`, or `implicable` — this
diff's own vocabulary — confirmed by grepping the full log for all three (zero matches). Every failure
traces to one of three pre-existing root causes the brief itself named in advance (taxonomy exempt-areas,
registry records missing fields, serde_json reachability) plus their direct siblings in the same test
file (the `pypdf` baseline gap and the unrelated `audit_sources.py` import share the exact same
`requirement.oracle`/dependency-baseline mechanism, not a new one) — none is a regression this shard
introduced, and none is a false negative this shard's own new tests should have caught: this shard's
own 3 new tests (§3) all pass.

## 4. `missing-external-oracle` — the honest 45, re-verified rather than reused

This shard's assigned remainder (`🧿️semio` 19, `➗️equation` 9, `🧰️framework/…/os/🎚️config` 5,
`🎬️sequence` 4, `💾️binary` 4, `🖊️dwg` 4) was already investigated by name in four prior shards' own
reports (A10 for equation/sequence, D1 for os.config's exclusion, F1 for semio's base envelope, E2 for
binary/dwg). Rather than take those dispositions on faith, I re-read every one directly against the
LIVE registry state:

- **`🧿️semio@v1/base` (19)** — one capability, `semio-v1-base-mutate`, required by all 18 wrapped arms'
  `apply-<arm>` verbs plus `set-snapshot`. Confirmed live: `oracles: []` for this subset, and a
  `noOracleDecisions` entry `semio-envelope-routing` with `capabilities: []` (correctly narrowed, not
  masking the gap) and a substantial rationale — F1 already investigated building a routing-level
  second implementation and rejected it for two concrete reasons still true today: the wrapped arm
  snapshot/mutation types have no JSON bridge reachable outside the subject crate, and even if they
  did, a routing-level implementation could say nothing about whether a DELEGATED verb actually changed
  the arm it reached (i.e. it could at best re-check envelope SHAPE, never the wrapped mutation's own
  correctness — the exact thing an oracle exists to judge). D1's `verified-native-second-implementation`
  promotion (my own territory this wave, §2 of `📓️d1-native-oracle-discharge.md`) deliberately excluded
  `base` from its 53-entry promotion for the same reason: no candidate `cross-semio-implementation`
  entry exists for it to promote (`oracles: []`). **Genuine, deep debt — left open.**
- **`➗️equation` geometry (4) + graph (5)** — confirmed live: `geometry` subset has `oracles: []`
  (point-cloud geometry edits — `replace-points`/`insert-point`/`remove-point`/`move-point` — have no
  registered reference at all); `graph` subset has one qualifying entry (`csv-rfc4180-equation-1-mutate`)
  that discharges only `equation-1-mutate` (node create/delete/label/move), leaving a correctly-narrowed
  `noOracleDecisions` entry (`equation-mutation-semantics`, `capabilities: []`) for the graph-attribute
  edits (`directed`/`algorithm`/arbitrary edge lists) it cannot reach. This is exactly A10's own
  `mathematical-1-mutate-uncarried` finding (9 mutations, "genuinely required... and nothing discharges
  it", after a real survey of `petgraph` and external CAS candidates, both declined) — nothing has
  changed since. **Genuine debt — left open.**
- **`🎬️sequence` dependency (2) + step (2)** — confirmed live: both subsets have `oracles: []`.
  A10's own `sequence-1-mutate-uncarried` finding stands unchanged (4 mutations —
  `connect-steps`/`disconnect-steps`/`change-step-collapsed`/`move-step` — genuinely required, nothing
  discharges them). **Genuine debt — left open.**
- **`🧰️framework/…/os/🎚️config` (5)** — confirmed live: `oracles: []` across `opening`,
  `merge-policy`, `identity`; three correctly-narrowed `noOracleDecisions` entries
  (`os-config-opening-preferences-mutation-semantics`, `os-config-merge-policy-mutation-semantics`,
  `os-config-identity-mutation-semantics`, all `capabilities: []`), each rationale explaining this is
  repository-owned application configuration (a preference-pin upsert, a single-field policy replace, a
  two-kind session-establish record) with no independent third party that could hold an opinion about
  it — there is no external "operating-system preference manager" whose own semantics this repo's
  upsert/replace/establish rules could be checked against. Matches D1's own final classification
  verbatim (`os.config 5`, "genuinely no second implementation yet"). **Genuine debt — left open.**
- **`💾️binary` (4) and `🖊️dwg` (4)** — E2's own §6 (`📓️e2-interchange-format-oracles.md`) already
  searched both: `binary` has no format at all (a reused opaque byte blob; `bsdiff`/`xdelta3` rejected
  because they verify a delta-PATCH model, not this vocabulary's direct splice/append/truncate-at-offset
  model); `dwg`'s only independent implementation of weight, LibreDWG, is GPL-3.0-only, and a prior
  shard's dated, reasoned licensing judgment call (recorded in the feature files themselves) declined it
  — E2 independently re-verified LibreDWG's license and availability on this machine before leaving it
  as-is, rather than trusting the note blindly. I re-read both feature files' recorded rationale and the
  current `noOracleDecisions` (`capabilities: []` in both `dwg` subsets, confirmed live) and found
  nothing has changed. **Genuine debt — left open, per E2's already-thorough search.**

No new third-party candidate was found for any of the 45, and none was force-registered. All 45 remain
open, each with an already-narrowed `noOracleDecisions` entry (or, for `semio/base`, an explicit
routing-level rejection) that does not mask the gap — exactly this ticket's second law working as
intended: a real, visible gap instead of a hidden one. **0 of the 45 closed; all 45 verified as genuine,
not relabelled.**

## 5. Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` —
  `reimplementationOracleBreaches`: entry-granular `implicable` filter + doc comment. No other function
  changed.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts` — added
  `reimplementationOracleBreaches` to the import list; added
  `describe("🎯️ reimplementation-registered-as-third-party is entry-granular, not file-granular")`
  with the 3 tests in §3.
- No `🔣️.json` registry file touched — this shard registered no new oracle and reclassified none; the
  20-breach reduction is entirely the detector fix, not a data change.

Scratch scripts kept in this ticket folder: `🔍️g1-dump-breaches.ts` (dumps live
`reimplementationOracleBreaches` output with each flagged owner's qualifying-entry ecosystems — used
for §1's verification), `🔍️g1-debug-wav-txt.ts` (probe used to confirm why `wav`/`txt`'s own
predicting-shaped `.rs` files were never flagged, ruling out a second, unrelated false-positive
mechanism before trusting the fix).

## 6. Final answer

- **All 20 flagged entries (18 E2's registrations activated + A10's original 2) are genuine
  third-party references — 0 were re-implementations dressed as third-party.** Verified independently
  (§1): every flagged qualifying entry has `ecosystem` python/javascript, every owner's `.rs` file
  independently contains real Rust reimplementation logic under a SEPARATE, already-correctly-classified
  `cross-semio-implementation` entry, and every reader's own rationale records real, measured
  verification against this repo's own fixtures.
- **Fix**: `reimplementationOracleBreaches` now filters to `ecosystem === "rust"` qualifying entries
  before even opening the shared `.rs` file, and only that filtered set is named in the breach — a
  structural, non-gameable predicate that matches the rule's own stated boundary ("predicts mutation
  output in our own Rust") exactly, rather than an approximation of it.
- **The rule is not weakened**: a genuine Rust-ecosystem reimplementation mislabeled third-party — the
  exact shape A10's original 20 were — still fires, proven by a dedicated positive unit test against a
  real temp-directory `.rs` file; a boundary test proves the SAME shared file, SAME contribution can
  produce different verdicts per entry, which is the entry-granularity requirement made concrete.
- **Before/after**: `reimplementation-registered-as-third-party` **20 → 0**; `missing-external-oracle`
  **45 → 45** (genuine, re-verified debt, 0 closed, 0 relabelled); `oracle-in-production` **301 → 301**
  (not risen); every other listed id held at 0; **TOTAL 805 → 784**.
- This file:
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️g1-oracle-detector-granularity.md`.
