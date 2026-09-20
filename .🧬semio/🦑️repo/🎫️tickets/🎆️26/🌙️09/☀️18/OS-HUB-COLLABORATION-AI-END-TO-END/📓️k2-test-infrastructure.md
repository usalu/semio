# K2 — Test infrastructure defects (ticket 26/09/18)

Six test-infrastructure defects sibling slices found but did not own. Every claim below was executed;
captures are in `🗑️generated/k2-*.txt`.

| # | item | status |
|---|---|---|
| 1 | `vitest-configuration-ownership` "machine-specific `cacheDir`" | DONE — diagnosis **refuted**, real drift fixed, gate 7/7 |
| 2 | orphaned test suites census + registration | WIP |
| 3 | `4_gate` launch families | DONE — 34 rows added, census 0 missing in both files |
| 4 | `foundation-source-check` `:371` `HUB_DEV_BINARY_TARGET` | DONE — gate 11/11 |
| 5 | `documentId`→`artifactId` descriptor rename | WIP |
| 6 | three `🔋️energy` `sim::` unit tests | WIP |

---

## 1. `vitest-configuration-ownership` — the `cacheDir` diagnosis is wrong

**Verified state.** U2's `📓️u2-…md` §6.5 reports "42 of 43 owners already mismatch their fixture
`projectionSha256` … the projection includes `cacheDir`, which `repoCacheDirectory()` resolves to a
machine-specific absolute path *outside* `repoRoot`, so `portable()` cannot relativize it".

Both halves are false:

1. `repoCacheDirectory()` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts:8`) is
   `join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", …)` — **inside** `repoRoot`, so `portable()`
   rewrites it to `./.🧬semio/…` like every other repo path.
2. The 42/43 number is an artifact of U2's own probe. `🐍️u2-vitest-projection-hash.ts:41` calls
   `projectionHash(config, resolve(repoRoot, owner.configurationRoot))` — an **absolute** root — while
   the gate (`🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts:104`) passes the **relative**
   `owner.configurationRoot`. Every row therefore hashed differently except the single owner whose
   `configurationRoot` is `"."` (`resolve(repoRoot, ".") === repoRoot` → `portable()` → `"."`). Hence
   exactly 43 − 1 = 42 "drifted".

**Measurement.** `🐍️k2-ownership-projection.ts` replays the gate's own inputs for *every* owner (the
real gate short-circuits on the first rejected `expect` inside `Promise.all`) and additionally scans the
canonical projection string for any absolute path. Capture `🗑️generated/k2-ownership-census.txt`:

```
owners=43 rootBad=1 nameBad=0 hashBad=8 machineBad=0
```

`machineBad=0` — **no owner's projection contains a single machine-derived path**. The real defects were
one `configurationRoot` regression and eight content drifts from peers' legitimate config edits.

**Fixes.**

- `🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts:53-59,111-113` — the projection is split out as
  `behavioralProjection()` and the gate now asserts `ABSOLUTE_PATH_IN_PROJECTION` matches nothing, i.e.
  no quoted value in the hashed projection may open with a POSIX root or a Windows drive. This is the
  root-level guard for the class U2 feared; it did not exist before, which is why nobody could tell the
  difference between a machine-dependent hash and ordinary drift.
- `🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts:75-88` — `routerRoot()` now also accepts the
  `📦️packages/🟦️typescript` bundle of the nearest semantic ancestor. `*Script` classes are routinely
  extracted out of `📜️script.ts` into sibling owners (`🧑‍💻dev/🧪️tests/🏃️execution/🟦️.ts:42`); the old
  walk found no `📜️script.ts` above such a file, fell through to `repoRoot`, and reported a correct
  `runVitest` selection as `wrong`.
- `🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` — the `🧰️framework` owner's
  `configurationRoot` was still `🧰️framework/📦️packages/🟦️typescript`; O1 deliberately re-rooted that
  config at `🧰️framework` (`🧰️framework/🧪️tests/🎚️config/🟦️.ts:12`, with the reason in its docstring:
  `includeSource` globs cannot walk upwards, so the suite collected zero tests). Fixture follows source.
- Eight `projectionSha256` rows regenerated from the gate's own hash function (there is **no** owning
  generator for this fixture — `projectionSha256` appears nowhere but the fixture, the schema and the
  test; the honest regeneration path is a replay of the gate, which `🐍️k2-ownership-projection.ts --write`
  is). Later edits in this slice moved two more, regenerated the same way.
- Three `runVitest` selections that pointed nowhere (see §2 — they are the same defect class as item 2).

**Proof.** `🗑️generated/k2-ownership-after.txt`:

```
 7 pass   0 fail   526 expect() calls   Ran 7 tests across 1 file. [35.44s]
```

Owner count 43 → 46 (schema `minItems`/`maxItems` and the two count assertions moved with it).

---

## 2. Tests that run nowhere

_in progress — census script and results below_

### 2.1 Found through the ownership gate's `runVitest` law

| owner | defect | fix |
|---|---|---|
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📜️script.ts:26` | selected `"🧪️vitest.config.ts"`, **a file that does not exist** — `nx run …projektetage:test` could never have run | repointed at `../../🧪️tests/🎚️config/🟦️.ts` |
| `✏️s/🔌️plugins/🀄️wfc/🧪️tests/🎚️config/🟦️.ts` | a real, registered owner absent from the fixture | fixture row added |
| `🧰️framework/🛍️products/🎤️presentation` (×2) | both vitest configs sat at the predecessor path `<package>/🧪️tests/🟦️.ts`, inside `📦️packages/` | relocated to the semantic owners (below) |

### 2.2 `@semio-tech/presentation-react` — a 147-test suite that collected **nothing**

`🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (was
`…/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts`) aliased `@semio-tech/ui-react` to
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — the **inverted**
path; the real entry is `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`.
The whole in-source file failed to transform, so `bun ./📜️script.ts test` reported
`1 failed | 2 passed (3)` with **11 tests** and a green-looking "only one file failed".

Root fixes:

- the `ui-react` alias corrected, plus the missing `@semio-tech/ui-react/test` and
  `@semio-tech/animate-presentation-core` aliases (the latter is what the projektetage spec needs), and
  the projektetage spec alias repointed from `🟦️.ts` (a browser entry that only side-effect-imports) to
  `🔖️spec.ts`.
- `🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:66` — the **production** module
  imported `act` (a React test utility) from `@semio-tech/ui-react`, which has never exported it. Moved
  into the `import.meta.vitest` block as `await import("@semio-tech/ui-react/test")`.

Result: **147 tests collected (was 0), 136 pass, 11 fail.** The 11 are genuine, pre-existing product
failures in a suite that has never run; classified in `🗑️generated/k2-presentation-react-failures.txt`
(6 × `presentation.chapters` undefined, 4 × `*.closest` on a null disposition, 1 × PDF canvas state).
`@semio-tech/presentation` itself: **2 files / 56 tests, all pass** (`🗑️generated/k2-presentation-tests.txt`).

### 2.3 `🎭️actor/📥️cold-pair` — an orphan in-source suite hiding a live schema violation (session 4)

`🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts:146-149` carries an `import.meta.vitest` block that
registers `🧪️tests/🧪️cold-pair-wit-status-codec-…/🟦️.ts`. No owner collected it: the only owner whose
`test.root` is `🧰️framework` (`🧰️framework/🧪️tests/🎚️config/🟦️.ts`) listed **three** files in
`includeSource` and cold-pair was not one of them. The suite had therefore never run — and it was
failing. See §5; the census (§2.4) is what makes this class visible at all.

**Fix.** `🧰️framework/🧪️tests/🎚️config/🟦️.ts:34-35` — `🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts` added to
`includeSource` **and** `coverage.include` (the file's own docstring is the law: in-source files go in
those two keys only, never in `include`). The framework owner's `projectionSha256` regenerated
(`🐍️k2-ownership-projection.ts --only 🧰️framework/🧪️tests/🎚️config --write`, a new `--only` filter so a
single owner can be replayed without paying `loadConfigFromFile` for all 46).

**Proof.** `🗑️generated/k2-framework-vitest.txt` — `bun ./📜️script.ts test` in
`🧰️framework/📦️packages/🟦️typescript` goes **3 files / 163 tests → 4 files / 164 tests, all pass**, and
`--reporter=verbose` shows the suite by name:

```
 ✓ |@semio-tech/framework| 🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts > cold pair WIT status codec agrees with
   the neutral schema and rejects every hostile authority 914ms
cold-pair-status-codec: schema=1 valid=6 structural-hostile=3 semantic-hostile=5 unknown-fields=refused
```

### 2.4 The census script

`🐍️k2-orphan-suite-census.ts` was rewritten this session. Two defects in the inherited version:

1. It loaded each owner through vite's `loadConfigFromFile`, which esbuild-bundles the config file.
   Under fleet load that is ≈20 s per owner, and `✏️s/🔌️plugins/📐️cad/🧪️tests/🎚️config/🟦️.ts` never
   returns at all (it calls `createWorkspaceViteResolveConfig(repoRoot)`, a whole-repo walk). Both
   inherited runs died there with a **0-byte** capture, which is why §2 was still `WIP`. It now
   imports each owner in a **child `bun` process** (`--owner <path>`; `defineConfig` is identity, so
   the module's default export *is* the resolved user config and no glob changes) with a 240 s
   deadline and four lanes, and reports an owner that does not resolve as `UNRESOLVED` instead of
   hanging the whole census.
2. Candidates were collected under `🧰️framework` and `✏️s` only. It now walks the whole repo
   (`🌎️hub`, `♻️mit-bestand`, `🧪️tests`, … minus `node_modules`/`🗑️generated`/`target`/`.🧬semio`).

It also buffered every line to the end, so a killed run left nothing; progress now goes to stderr
(`🗑️generated/k2-orphan-progress.txt`) line by line.

**Census result** (`🗑️generated/k2-orphan-census.txt`):

```
candidates=1104 vitest=94 direct=660 transitive=135 orphans=309 unloadable=2
```

Two owners do not resolve inside the 240 s child deadline under fleet load
(`✏️s/🔌️plugins/📐️cad/…` and `💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/…`); both call
`createWorkspaceViteResolveConfig(repoRoot)`, a whole-repo walk. Their suites are therefore counted
as unreached and are inside the 309. **The 309 is an upper bound, not a final residual.**

By area (`ORPHAN` lines, first three path segments):

| area | orphans | owner |
|---|---:|---|
| `🧰️framework/🛍️products/📓️print` | 106 | **§2.5 — root-caused and fixed here** |
| `🧰️framework/🛍️products/💻️os` | 63 | T4c |
| `🧰️framework/🛍️products/🦑️repo` | 49 | dev infrastructure (repo MCP); preamble rule 20 — not slice work |
| `🧰️framework/🔨️modules/🖱️ui` | 16 | unowned |
| `✏️s/🔌️plugins/🎞️animate` | 10 | F1 / B1a |
| `✏️s/🔌️plugins/🌀️procedural` | 7 | B3c |
| `🏢️semio-tech/🎡️play` | 9 | ticket 26/09/19 SEMIO-TECH-PLAY-GRID |
| `🌎️hub/🔨️modules/🛡️admin` | 5 | H1b |
| `✏️s/🔌️plugins/{📐️cad,🧩️puzzle,🗄️stdio,🀄️wfc}` | 15 | plugin slices (5 of these are the unresolved cad owner) |
| rest (`🏗️fem`, `🌐️spatial-kernel`, `🎠️kernel`, `⏳️async`, `📡️replication`, `♻️mit-bestand`, …) | 29 | unowned |

### 2.5 `@semio-tech/print` — a whole product's test target has not been able to start since 2026-09-14

**106 of the 309 orphans are one defect.** `bun ./📜️script.ts test` in
`🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript` does not run zero tests — it does not even
load (`🗑️generated/k2-print-test.txt`, exit 1):

```
5 | export class PrintPipelineVerificationCommand extends BundleScript {
ReferenceError: BundleScript is not defined
  at 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🟦️.ts:5:55
```

**Root cause**, `git show bb961413d4` (2026-09-14 16:57, a 524-file commit): that commit deleted the
file's **three import lines** —

```
-import { verifyPrintVisualizationBuild } from "./🧪️tests/🖨️pipeline/🟦️.ts";
-import { BundleScript, TEST_LEVELS, resolveTestLevel } from "../../../🦑️repo/…/🟦️.ts";
-import { verifyPrintMacroStagingNative, verifyPrintPipelineLong, verifyPrintPipelineQuick } from "./🧪️tests/🖨️pipeline/🟦️.ts";
```

— **and** the two statements that were the entire body of the default (level) branch:

```
     const { level } = resolveTestLevel(segments);
-    await verifyPrintPipelineQuick();
-    if (TEST_LEVELS.indexOf(level) >= TEST_LEVELS.indexOf("long")) await verifyPrintPipelineLong();
```

while adding calls to `regeneratePrintGalleryFixtures(…)` and `runPrintPlatformCases(…)`, **neither of
which exists anywhere in the repo** (`grep -rn` over `🧰️framework` finds only these two call sites).
So even with the imports restored the `viz fixtures` and `viz` paths would still throw.

The file has not been touched since (`ls -la` → Sep 15 01:41, clean in `git status`), so
`@semio-tech/print:test` has been dead for five days, and with it every one of the 106 print suites.

---

## 3. The `4_gate` launch family

**Verified state.** K1 restored one row and reported "at least eight" missing families. The real number
is **34**. `📜️script.ts:9003-9009` documents that commit `6f33e313da` (2026-09-15) emptied
`INTERACTIVITY_ALL_APP_REQUIRED_GATES` *and* deleted the `⚖️gate…` rows from both launch files; the
per-gate fixtures and their tests were never updated, so each of those gates has been asserting a row
that does not exist.

**Measurement.** `🐍️k2-gate-launch-rows.ts` walks every JSON under `📚️library` and `🌎️hub`, collects
each object carrying a `4_gate` launch registration, and derives the command the owning test asserts
(`launchCommand` → `nxCommand` → `bun nx run @semio-tech/repo-lib:<target> --skip-nx-cache`). Before:
`declared=35 present=1 missing=34`.

**Fix.** `.vscode/🧩️launch.seed.jsonc` — 34 configurations appended to the `configurations` array
(re-read immediately before the write; Z1/V2 hold the same file). `.vscode/launch.json` regenerated with
`bun nx run @semio-tech/plugin-registry:generate` (`🗑️generated/k2-registry-generate.txt`, exit 0,
"`.vscode/launch.json regenerated`"). Never hand-edited.

**Proof.** `🐍️k2-gate-launch-census.ts` (`🗑️generated/k2-gate-census.txt`):

```
declared=35 missing=0
.vscode/🧩️launch.seed.jsonc: 4_gate rows = 37
.vscode/launch.json:         4_gate rows = 37
```

Every target referenced exists: 25 `@semio-tech/repo-lib:test-*` targets verified present in
`📚️library/📦️packages/🟦️typescript/📋️project.json`; `@semio-tech/value-resident`,
`@semio-tech/value-resident-rs`, `@semio-tech/ui-host-rs` and the two `os-hub` socket-grant targets are
real projects.

Owning gates re-run (`🗑️generated/k2-gate-tests.txt`, `k2-gate-failures.txt`):
`🔤️taxonomy-leading-grapheme` **9/9 pass**. Five other gates still fail, but **none** of their failing
tests is the launch-registration test (`🫙️artifact-empty-facet-authority`, `♻️taxonomy-pattern-compiler-reuse`,
`🕰️historical-json-source-encoding` — failures are semantic/oracle ones);
`🪶️artifact-empty-facet-authoring` and `📍️draw-destination-observation` need `SEMIO_TEST_ARTIFACT_DIR`
which only their nx target sets. `🛟️transaction-recovery-authority`'s launch assertion is unreachable
behind a pre-existing broken router-path assertion at `:94`
(`expect(router).toContain('🧪️tests/🧪️' + row.id + '../🛟️transaction-recovery-authority/🟦️.ts')` — a
malformed concatenation). All of these are pre-existing and outside this item.

---

## 4. `foundation-source-check` at `:371`

**Verified state.** Reproduced: `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:83` exports
`HUB_DEV_BINARY_TARGET` and `🌎️hub/📦️packages/🦀️rust/📜️script.ts:107` imports it, but the owner's row
in `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json` listed neither.

**Fix.** That fixture row — `HUB_DEV_BINARY_TARGET` added to `declarations` (source order, after
`hubBinaryPath`) and to `rootImports` (the router's import-statement order, after `freeLoopbackPort`).

**Proof.** `🗑️generated/k2-foundation-source.txt`:

```
 11 pass   0 fail   262 expect() calls   Ran 11 tests across 1 file. [3.45s]
```

---

## 5. The descriptor `documentId` / `artifactId` half-rename

**The premise as handed over is backwards, and the measurement says so.** AU2 §7 reported
"the `announce-document` descriptor schema now requires `artifactId` while the test's fixture still
sends `documentId`", and the slice brief asked for a repo-wide `documentId`→`artifactId` rename.
Measured at HEAD, the descriptor family agrees on **`documentId`** in all four places:

| twin | field |
|---|---|
| `💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json:2679` `$defs/DocumentDescriptor` | `documentId` (required, `additionalProperties:false`) |
| `…/🧬️schema/🦀️.rs:1459` `struct DocumentDescriptor` | `document_id` |
| `…/🧬️schema/🟦️.ts:1179` `interface DocumentDescriptor` | `documentId` |
| `…/🏛️administration/🧫️fixtures/🛂️command-admission/🔣️.json:20`, `💻️os/🧫️fixtures/📇️directory/🏛️administration-worker-wire-v1.json:35`, `🌎️hub/📇️directory/🧫️fixtures/🎯️admin-intent-v1/🔣️.json:30`, `…/🚻️space-journey-v1/🔣️.json:42` | `documentId` |

and `@semio-tech/framework-os:test` — the suite AU2 saw fail — is **5 files / 361 tests, all pass**
(`🗑️generated/k2-os-test.txt`). The test AU2 named (`🧪️tests/🧪️backbone-envelope-io/🟦️.ts:2172`) compiles
the directory schema with Ajv `strict:true` and validates every fixture command against it, so it is
itself the regression gate for that pair. Renaming 953 TypeScript `documentId` occurrences to
`artifactId` would *introduce* the drift T4 §10a/§10i deliberately removed. **Not done, on purpose.**

**What was actually still broken.** One fixture in the same rename family, invalid against its own
schema and its own parser, protected by the orphan suite of §2.3:

`🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧫️fixtures/🔣️.json` keyed the `applied` receipt's
`baselineFrontier` as **`artifactId`** (two rows: `statusRows[4]`, `hostileRows[3]`) while
`🧬️schema/🔣️.json:74-89` `#/definitions/frontier` requires `documentId` with
`additionalProperties:false`, `🟦️.ts:80` `exactRecord(value, ["documentId", …])` refuses anything else,
and `🦀️.rs:13` is `pub document_id: String`. Renamed to `documentId`, by hand, at both rows.

**Proof both halves were real.** `🗑️generated/k2-cold-pair-proof.txt` — the same Ajv instance and the
same exported parser the gate uses, run against the pre-fix value and the post-fix value:

```
HEAD-1 (artifactId) valid = false ["must NOT have additional properties", …,
                                   "must have required property 'documentId'", …]
HEAD   (documentId) valid = true
HEAD-1 parser threw: cold-pair.frontier
HEAD   parser kind: applied
```

So the suite could not have passed had anyone run it — which is the §2 defect and the §5 defect being
the same defect. The Rust twin (`🧪️tests/📥️cold-pair/🦀️.rs:39`) reads the same fixture but only asserts
`statusRows.len() == 6` and that each row has a `kind`, so it never touched the field. Not re-run
(cargo; see §7).

## 6. `🔋️energy` `sim::` unit tests

**Verified state.** K1 §9.1 left three failures without an owner and without a before-measurement:
`sim::tests::p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one`,
`p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total`,
`p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology` (32 pass / 3 fail).

**Result: 35 passed, 0 failed** — `🗑️generated/k2-energy-sim.txt`, `cargo test -p
semio-s-artifact-energy-model --lib sim::`:

```
test result: ok. 35 passed; 0 failed; 0 ignored; 0 measured; 6258 filtered out; finished in 0.18s
```

Landed in commit `48a8c69cdb` (`git log --date=iso` → 2026-09-19 13:48). Three distinct root causes,
none of them a test edit for its own sake:

1. **Production bug** — `⚡️simulation/⚙️engine/🧪️sim/🦀️.rs:3111`: the commit-reservation admission
   compared `aggregate_pages > self.numerical_census.pages`. The census bounds the pages of **one**
   commit, not the running aggregate, so the second commit of a four-fuel chronology was refused and
   `p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology` never saw its fourth fuel. Now
   `pages > self.numerical_census.pages`; the two genuinely aggregate bounds
   (`JOB_PAYLOAD_OPERATION_PAGES`, `…_BYTES`) are untouched.
2. **`p7c2_preview_typed_view_…`** asserted a substantive facility total from
   `test_model_single_zone()`, which has no electric end use — it can never produce
   `facility_electricity_kwh > 0`. Moved to `test_model_full_topology()`, and the test now drives
   `begin_close`/`close_step` to `Complete` and closes the retained state/output payloads on the
   `Complete` arm instead of `break`ing with them still mounted (the store-drop-witness law).
3. **`p7c1_weather_owner_…`** called `job.step()` **once** and asserted `StepOutcome::Fault`. The
   weather slot rejection is reached through `begin_fault()`, which first yields to open the fault
   wire, so the fault surfaces a few turns later. Now loops up to 1024 steps, breaking on the first
   `Fault`, and the `weather_fault == Some(WeatherFault::SlotRejected)` assertion is unchanged.

Also in that commit, from K1's slice: the eleven `eprintln!("[DEBUG] …")` probes and the
`fn fault(error: &Error)` parameter that existed only to feed them are gone.

## 7. Honest gaps

1. **The orphan census has not produced a final number.** Three runs: two inherited ones died with a
   0-byte capture, and the session-4 run is still walking owners at the time of writing. The residual
   list will be whatever `🗑️generated/k2-orphan-census.txt` ends with; §2.3 is one orphan found by
   hand from the same evidence the census uses. **Not closed.**
2. **`test-vitest-configuration-ownership` now blows its 120 s bun-test budget.**
   `🗑️generated/k2-ownership-after.txt`: `this test timed out after 5000ms` →
   `exceeded 120000ms — killed`. This is **load**, not a regression: the same gate ran 7/7 in 35.4 s
   earlier in this ticket, and the machine was at `load average 146.16` on 10 cores when it was
   re-run. The gate calls vite's `loadConfigFromFile` 46 times, so it is inherently the most
   load-sensitive gate in the repo. Two honest options, neither taken here: move it to the `long`
   level, or make it import owners directly the way §2.4's census now does (≈46× cheaper, same globs).
   **Re-measure on a quiet machine before believing either number.**
3. **The cold-pair Rust twin was not re-run.** `cargo test -p …` for the actor crate was not started;
   the Rust test reads the fixture but asserts only its row counts and `kind`s, so the key rename
   cannot affect it — that is an argument, not a measurement.
4. **§5's TypeScript side is proved by a standalone replay**, not by the Rust-side language-agnostic
   twin. The schema, the TS parser and the Rust struct agree on `document_id`/`documentId` by
   inspection.
5. **Nothing in this slice re-ran `@semio-tech/framework-os:test` after the peers' session-4 edits.**
   The 361/361 capture is from 11:20 on 2026-09-19.

## 8. Files changed

Session 4 (this pass):

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🧫️fixtures/🔣️.json` | `artifactId` → `documentId` in both `baselineFrontier` rows (§5) |
| `🧰️framework/🧪️tests/🎚️config/🟦️.ts:34-35` | cold-pair in-source module registered in `includeSource` + `coverage.include` (§2.3) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` | framework owner `projectionSha256` regenerated for the line above |
| `🐍️k2-orphan-suite-census.ts` | rewritten: child-process owner loading with deadline, repo-wide candidates, streaming progress (§2.4) |
| `🐍️k2-ownership-projection.ts` | `--only <substring>` owner filter |

Earlier passes (§1, §3, §4, §6) are listed in their own sections; all of them are in commit
`48a8c69cdb` or earlier.

---

# Session 5 (2026-09-20)

Inherited state re-checked first: §2.5's `@semio-tech/print` restoration is in the working tree
(uncommitted, `🗑️generated/k2-print-test.txt` at 00:40 shows the target loading and passing), so the
predecessor landed it before the 01:15 cut but never wrote it up. Everything below was executed this
session.

## 9. A self-referential symlink made two vitest owners unloadable — and can hang any dev server

**This is the root cause behind §7 gap 2 and behind the census's two `UNRESOLVED` owners.**

`findWorkspacePackages()` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:1566`)
walks the whole repo to build `optimizeDeps.exclude` for `createWorkspaceViteResolveConfig()`, which
every playground/dev/test vite config in the repo calls **at config-module load**. The walk classified
directory entries with `statSync(full).isDirectory()` — which **follows symlinks**.

`🌎️hub/📦️packages/🦀️rust/🗑️generated/test-artifacts/` contains three links a hub test leaves behind:

```
linked-ancestor-publication-owner-17675-0 -> …/🗑️generated/test-artifacts
linked-ancestor-publication-owner-39889-0 -> …/🗑️generated/test-artifacts
linked-ancestor-publication-owner-45643-0 -> …/🗑️generated/test-artifacts
```

Each points at **its own parent directory**. The walk therefore recursed into `test-artifacts` forever,
with no error and no timeout. `macOS sample` of the hung `bun` (pid 14835, 7 min in) shows one JIT frame
(`0x118680ac8`) repeating to the bottom of the stack — the `scan` recursion.

**Measured before/after**, same command, same machine (load ≈ 84):

| | `bun 🐍️k2-owner-patterns.ts --owner …📐️cad/🧪️tests/🎚️config/🟦️.ts` |
|---|---|
| before | did not return in **> 400 s** (killed; the inherited census gave it 240 s and recorded `UNRESOLVED`) |
| after | **23.5 s**, full config printed |

**Fix.** `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:1566-1595` — entries are now
classified from `readdirSync(dir, { withFileTypes: true })`'s own `Dirent`, which never follows a link,
and `entry.isSymbolicLink()` is skipped outright. Build output (`🗑️generated`, `🤖️generated`) is added to
the skip set as `WORKSPACE_PACKAGE_SCAN_SKIP` — it can carry no workspace package, and it is exactly
where the links live.

## 10. The gate now asserts that an owner collects something, and costs a third of the work

Three laws added to `🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts`, all inside the test that already
loads every owner, so they cost nothing extra:

1. **`patternsSelectingNothing()`** (`:124-139`) — every `include`/`includeSource` pattern of every owner
   must select **at least one file on disk**. This is the §2.1/§2.3 defect class made permanent: a
   pattern that matches nothing is not an error to vitest, it just narrows the run, which is how
   `…/📅️33.projektetage` could select `"🧪️vitest.config.ts"` (never existed) and how `🧰️framework`
   could ship an `includeSource` reaching zero files while its `test` target stayed green.
   Patterns are matched **absolute**: many legitimately climb above their own `test.root`
   (`../../📥️cold-pair/🟦️.ts`), and matching them relative is precisely the bug §11 found in the census.
2. **zero-pattern owners** (`:189-190`) — only the root owner (`configurationRoot === "."`, which
   documents its own no-tests policy and is separately gated) may declare no patterns at all.
3. **Nx input coverage** (`nxInputPatterns()`, `:141-162`) — replaces a literal substring search over
   `📋️project.json` **text** with a real expansion of each project's `namedInputs` / target `inputs`,
   `{workspaceRoot}`/`{projectRoot}` resolved and glob-matched. The old check failed this session on
   `@semio-tech/framework-replication`: commit `03b1a41483` replaced its explicit owner path with the
   equivalent `{workspaceRoot}/…/📡️replication/🧪️tests/**/*`, which registers the same file with Nx.
   That was a false failure of the gate, not a real regression.

**Cost.** `loadConfigFromFile` (esbuild-bundles each of the 46 configs) replaced by plain `import()`;
`defineConfig` is identity, so the default export *is* the user config, and all 46 `projectionSha256`
rows still match — which is the proof the two loaders agree. The three separate whole-repo walks
(`walkProductFiles` ×2 + the new file list) became one memoized `repositoryTree()`, with `🗑️generated`,
`🤖️generated` and `.🧬semio` (shared cargo build dir + every ticket folder) never walked.

**Budget.** `📜️script.ts:160` — `budgetMs: 120_000` → `TEST_LEVEL_BUDGET_MS.long`. Measured: the gate
costs **9.2 s of user CPU**, stable across every run; wall clock was 118 s, 194 s, 201 s and 125 s on the
same tree at load 121–153, i.e. it is contention-bound, not work-bound, and 120 s was unmeetable on this
machine. §7 gap 2's two options were "move it to the long level, or make it import owners directly" —
both are now taken.

**Proof.** `🗑️generated/k2-ownership-s5.txt`, load average 122:

```
 7 pass   0 fail   618 expect() calls   Ran 7 tests across 1 file. [125.21s]
```

(was 526 expect() calls). One owner hash regenerated on the way
(`🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`, a peer's config edit — `🗑️generated/k2-ownership-s5-write.txt`,
`owners=46 rootBad=0 nameBad=0 hashBad=1 machineBad=0`).

## 11. `@semio-tech/ui-styling`: 39 → 78 tests

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` carries a 39-test `import.meta.vitest`
block (`registerTests1`, the `🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts` suite). The styling owner
listed only `📽️projection/🟦️.ts` and `🌓️theme/🟦️.ts` in `includeSource`, so **no owner in the repo
collected it** — the same defect as §2.3, and the reason the §9 symlink bug could live in a tested
function. The census did not flag it: `reachedByBunTest` counts a literal path mention in any
`📜️script.ts`/`📋️project.json`/`🔣️.json`, and the path appears in
`🧫️fixtures/🧱️framework-source-topology/🔣️.json` — a topology fixture, not a runner. A false positive.

**Fixes.**

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎚️config/🟦️.ts:23-24` — the module added to
  `includeSource` **and** `coverage.include`, never `include` (in-source law).
- `🏗️builder/🌐️vite/🟦️.ts:1` — `// @vitest-environment node`. The owner's base environment is `jsdom`,
  and the first collected run failed outright: esbuild refuses to start there
  (`Invariant violation: "new TextEncoder().encode("") instanceof Uint8Array" is incorrectly false`).
  This is the repo's existing idiom for per-file environments since vitest 4 dropped
  `environmentMatchGlobs` (precedent: `📐️cad/…/📺️renderer/🟦️.tsx:3`).
- the owner's `projectionSha256` regenerated.

**Proof.** `🗑️generated/k2-styling-vitest.txt`:

| | files | tests |
|---|---|---|
| before | 2 passed (3) | **39** |
| after | 3 passed (3) | **78 passed** |

A regression test for §9 was added to that suite —
`🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts`, `"terminates on a self-referential symlink and never
reads through it"`: a temp tree with a link to its own parent plus a `🗑️generated` package manifest, and
`findWorkspacePackages` must return exactly the one real package. `symlinkSync` threaded through the
suite's injected dependencies. On the pre-fix code this test does not fail — it never returns.

## 12. The orphan census was wrong three ways: 309 → 115

None of the 194 difference is a suite that got wired up. All three are defects in the inherited census
(`🐍️k2-orphan-suite-census.ts`), each found by cross-checking it against an independent survey of the
same owners (`🐍️k2-owner-patterns.ts`, new this session, which prints every owner's patterns *and how
many files each one matches*).

| # | defect | effect |
|---|---|---|
| 1 | owner globs were matched against the candidate **relative to `test.root`**, and anything resolving above it was skipped (`if (local.startsWith("..")) continue`) | every `../../…` pattern was ignored — that is most of the `🖱️ui/🎯️targets/⚛️react` (27 patterns, all `../../../../🧱️elements/…`) and `🎭️actor` (11 patterns, all `../../…`) owners. Vitest-reached suites **94 → 195** |
| 2 | two owners were recorded `UNRESOLVED` and their suites counted as unreached | §9's symlink hang. **unloadable 2 → 0** |
| 3 | `🧪️test`'s **case discovery** was not modelled at all | `discoverTestCases()` generates one cacheable Nx project per `🧪️tests/<case>/` directory holding a feature file. The whole `📓️print` visualization gallery (105 suites, each a `defineTestAdapter` beside a `🥒️.feature`) read as orphaned. **directly reached 711 → 861** |

A fourth, smaller correction: a `BundleScript` router is a runner wherever it lives, not only in a
`📜️script.ts` — `📓️print` routes its suites from `🎮️commands/🧪️print-pipeline-verification/🟦️.ts`.

**Result** (`🗑️generated/k2-orphan-census.txt`):

```
candidates=1113 vitest=195 direct=861 transitive=137 orphans=115 unloadable=0
```

Residual by area — this is the real list, and it is now small enough to own:

| area | orphans | owner |
|---|---:|---|
| `🧰️framework/🛍️products/💻️os` | 47 | T4c |
| `🧰️framework/🛍️products/🦑️repo` | 11 | dev infrastructure; preamble rule 20 |
| `🧰️framework/🔨️modules/🖱️ui` | 11 | unowned — §13 |
| `✏️s/🔌️plugins/🌀️procedural` | 7 | B3c |
| `🏢️semio-tech/🎡️play` | 11 | ticket 26/09/19 SEMIO-TECH-PLAY-GRID |
| `✏️s/🔨️modules/🏗️fem` + `✏️s/🔌️plugins/🏗️fem` | 5 | fem slices |
| `✏️s/🔌️plugins/🎞️animate` | 3 | F1 / B1a |
| rest (`🌐️spatial-kernel`, `🧩️puzzle`, `🗄️stdio`, `🀄️wfc`, `🎒️pack`, `🌱️value`, `🧵️job`, `🕹️interaction`, `♻️mit-bestand`, `.storybook`, …) | 20 | plugin/module slices |

**Known remaining looseness, stated rather than hidden.** `reachedByBunTest` still counts a literal path
mention in any `🔣️.json` as a runner registration. That is how §11's suite read as reached while nothing
ran it — the path appears in a source-topology fixture. The census is therefore still an **upper bound on
reachability**, i.e. its orphan count is a **lower** bound. Every residual above is a real orphan; there
may be more that a fixture mention is hiding.

## 13. `@semio-tech/cad-js`: the spatial-kernel `🧠️semio` suite had never run either

Same class as §11, found by the corrected census. `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts:778`
imports `🧪️tests/🧪️semio-tech-cad-js-spatial-kernel-semio/🟦️.ts` from its `import.meta.vitest` block, and
the cad owner's `DOMAIN_FILES` listed its three siblings (`🧱️brepjs`, `📐️geometry`, `🗺️spatial`) but not it.

**Fix.** `✏️s/🔌️plugins/📐️cad/🧪️tests/🎚️config/🟦️.ts:31` — added to `DOMAIN_FILES`, which feeds both
`includeSource` and `coverage.include`; the config's docstring count corrected 9 → 10 domain files.

**Measured, and NOT green** (`🗑️generated/k2-cad-vitest.txt`): the suite now collects —
`../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts (6 tests | 5 failed) 424ms` — and 5 of its 6
tests fail (`createBoxFromCorners` volume, `createBoxFromCornersDiff` face buckets, …). These are
genuine pre-existing product failures in a suite nothing has ever run, exactly like §2.2's
`presentation-react`. The whole cad owner is **111 failed / 148 passed of 259**, so 106 of those failures
predate this change; they belong to the cad slice, not here. Surfacing them is the point — they were
invisible before.

That run also shows a defect class **the §10 law deliberately does not cover**: two of the cad owner's
`includeSource` entries resolve to real files that contain no tests at all —
`✏️editor/⚙️engine/🏃️runtime/🟦️.ts (0 test)` and `✏️editor/⚙️engine/📺️renderer/🟦️.tsx (0 test)`. A pattern
pointing at a real file is all a static gate can check; whether that file still carries an
`import.meta.vitest` block is a property of the run. Noted as a gap (§14.6), not papered over.

## 14. Honest gaps (session 5 — supersedes §7 where they overlap)

1. **§7 gap 1 (census had no final number) is closed.** 115 orphans, `unloadable=0`, and the number is
   derived from a census whose three reachability defects are fixed (§12). It remains a **lower** bound:
   `reachedByBunTest` still counts a literal path mention in any `🔣️.json`, which is how §11's suite hid.
2. **§7 gap 2 (gate over budget) is closed**, but by two changes and a budget raise (§10), not by making
   the gate fast. Its CPU cost is 9.2 s; its wall clock on this machine is 2 minutes because the machine
   carries load 120–150. **Re-measure on a quiet machine** before concluding anything about the 300 s
   budget.
3. **§7 gap 3 (cold-pair Rust twin) is still open.** No cargo was run in this session at all — the load
   average never fell below 84 and preamble rule 14 says take the narrowest check that proves the point.
   The argument in §7 gap 3 stands and is still an argument, not a measurement.
4. **§7 gap 5 (`@semio-tech/framework-os:test` not re-run) is still open**, same reason plus peer churn:
   T4c owns that tree and was editing it this session.
5. **The 115 residual orphans are not fixed, only measured and attributed** (§12 table). Two classes:
   105 are plain assert-style suites — predominantly discovered-case adapters with **no feature file
   beside them**, so `discoverTestCases()` skips them (several are `🟦️.ts`/`🦀️.rs` twins where only the
   Rust half runs, e.g. `🖱️ui/🪟️viewport/🧪️tests/🪟️poses`, `🌱️value/📋️list/🧪️tests/📋️list`). Authoring
   those feature files is a decision for the `🧪️test` taxonomy owner, not a drive-by. The other 10 are
   `registerTests` suites whose importing module is in no owner's `includeSource` — the §11/§13 class,
   one fix each, and all ten sit in other slices' trees (`🎡️play` ×5, `🎞️animate`, `🗄️stdio`,
   `💻️os/📺️renderer`, `♻️mit-bestand`, and the one taken in §13).
6. **A pattern can select a real file that carries no tests.** §10's law cannot see that; the cad owner
   has two such entries today (§13). Catching it needs a collected-count assertion in the *run*, i.e. in
   `runVitest`, against a per-owner expected minimum — a fixture that does not exist yet.
7. **`findWorkspacePackages` is still a whole-repo walk** on every `createWorkspaceViteResolveConfig`
   call (§9 made it terminate, not cheap: 23.5 s for the `📐️cad` owner under load). The exact list is
   declared in the root `package.json` `workspaces` array; deriving it from there would make the call
   O(#workspaces). Not taken here — a mismatch would change `optimizeDeps.exclude` for every dev server
   in the fleet, and peers are running those right now.

## 15. Files changed (session 5)

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:1566-1595` | `findWorkspacePackages` no longer follows symlinks; `WORKSPACE_PACKAGE_SCAN_SKIP`; `@vitest-environment node` pragma; `symlinkSync` threaded to the suite (§9, §11) |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts` | self-referential-symlink regression test; `symlinkSync` dependency (§9) |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎚️config/🟦️.ts:23-24` | vite builder registered in `includeSource` + `coverage.include` (§11) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` | `patternsSelectingNothing`, zero-pattern law, `nxInputPatterns`, `ownerConfiguration` (import, not esbuild), one memoized `repositoryTree` (§10) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts:160` | ownership gate budget → `TEST_LEVEL_BUDGET_MS.long` (§10) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` | `projectionSha256` regenerated for the `🖱️ui/🎨️styling` and `🧊️wgpu` owners |
| `✏️s/🔌️plugins/📐️cad/🧪️tests/🎚️config/🟦️.ts:31,35` | `🧠️semio` domain file registered; docstring 9 → 10 (§13) |
| `🐍️k2-orphan-suite-census.ts` | absolute glob matching, `BundleScript` routers as runners, case discovery modelled (§12) |
| `🐍️k2-owner-patterns.ts` | **new** — per-owner pattern survey with on-disk match counts, the cross-check that found §12.1 |

Captures: `🗑️generated/k2-ownership-s5.txt`, `k2-ownership-s5-write.txt`, `k2-styling-vitest.txt`,
`k2-cad-vitest.txt`, `k2-orphan-census.txt`, `k2-orphan-progress.txt`, `k2-owner-patterns.txt`,
`k2-owner-patterns-progress.txt`, `k2-gate-census.txt`.

No `git commit`/`stash`/`checkout` was run; nothing in `🗑️generated` was deleted; `📌️important.md` and
`🎫️ticket.json` untouched. The `4_gate` census was re-verified unchanged: `declared=35 missing=0`, and
both launch files agree at 48 `4_gate` rows (peers added 11 since §3).

## 16. Session 5b (06:15, quiet machine) — the two gaps that needed a calm machine

### 16.1 §14.4 closed, and it is a regression: `@semio-tech/framework-os:test` 361/361 → 349/359

`bun ./📜️script.ts test long` in `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`
(`🗑️generated/k2-os-test-s5.txt`, 39.7 s on an idle machine):

```
 Test Files  2 failed | 3 passed (5)
      Tests  10 failed | 349 passed (359)
```

§5's `361 tests, all pass` capture was 11:20 on 2026-09-19. **Ten tests have broken since**, and the
total moved 361 → 359, so the suite itself changed too. None of the ten is in this slice's tree; all ten
are peers' live edits, and they are listed here so the owning slice sees them rather than discovering
them at the end:

| failure | error | owner |
|---|---|---|
| `tags every AppCommand variant per the agreed contract order` | `local-interaction.invalid-u64` from `📡️replication/📡️wire/🏠️local-interaction/📡️transport/🟦️.ts:18` via `encodeAppCommand` (`💻️os/🟦️.ts:2916`) — an `AppCommand` variant now carries a u64 field whose value does not match `^(0\|[1-9][0-9]{0,19})$` | replication / os |
| `browser document actor reservation activates only after an exact current socket Session` | `Error: session` | W3c / C1c |
| `browser document actor transfers one verified cold pair only after lifecycle ACK and exact page receipts` | `Error: expected` | W3c / C1c |
| `browser document first open rejects hostile assets and retired owners before socket authority` | `Error: browser` | W3c / C1c |
| `browser document open rejects mismatched and max-plus-one plans and cancels before receipt exchange without leaking authority` | `foreign-plan-scope` | W3c / C1c |
| `browser execution target lease rejects every single-field substitution without publication` | `Error: expected` | os |
| `browser execution target lease verifies GIS wasm bytes before plan exchange` | `Error: hub` | B3a / os |
| `browser GIS viewer exposes localized renderer-unavailable after verified lease` | `Error: expected` | B3a |
| `execution target body reader prevents stale owners publishing a lease or grant` | `Error: expected` | os |
| `allowlists only one canonical safe-decimal event-page route and keeps bootstrap on the TypeScript owner` | `Error: expected` | os |

Worth noting against §5: this is the suite that is the regression gate for the `documentId` descriptor
family, and it still validates every fixture command against the directory schema with Ajv `strict:true` —
**none of the ten failures is a `documentId`/`artifactId` failure**, so §5's conclusion is unaffected.

Also measured: the default `fundamental` level cannot run this suite at all — `bun ./📜️script.ts test`
with no level is killed at 15 000 ms before collecting, on an idle machine. It needs `long`.

### 16.2 §14.3 still open: the cold-pair Rust twin never got the artifact-directory lock

`cargo test -p semio-framework-actor --lib cold_pair`, started 06:19 on the calm machine, spent
**42 minutes** in `Blocking waiting for file lock on artifact directory` and never reached a test.
`🗑️generated/k2-cold-pair-rust.txt` contains that one line and nothing else.

This is **not** the 06:12 deadlock of preamble rule 23(a): the check was run repeatedly and 17–45 `rustc`
processes were live on the machine throughout, i.e. the lock holder was genuinely building. It is
ordinary contention behind the parallel fleet, which preamble rule 7 says to wait out and rule 14 says to
report rather than outlast. The run was killed by pid (10363, mine, proven by its capture path) so it
would not sit in the queue after this turn.

So §7 gap 3 stands exactly as written: the Rust twin (`🧪️tests/📥️cold-pair/🦀️.rs:39`) reads the fixture
this slice edited but asserts only `statusRows.len() == 6` and that each row carries a `kind`, so the
`artifactId` → `documentId` key rename cannot affect it. **That is still an argument, not a measurement.**
It is a one-command job for whoever next has the artifact lock.

### 16.3 §14.7 closed by memoization — and the `workspaces` shortcut is refuted

Re-running the §11 suite after the peers' session-5 churn in `🖱️ui` caught it red: **2 failed / 76
passed**, both failures the two *pre-existing* `findWorkspacePackages` tests timing out on their own
20 000 ms budget. Not a regression from §9 — §14.7 arriving: the walk terminates now, but it is slow, and
the tree grew past the budget.

**The declared-`workspaces` shortcut §14.7 proposed is wrong, measured.** A probe comparing the walk's
result with the root `package.json` `workspaces` array resolved to package names:

```
walked=148 in 14409ms   declared=124 in 16ms
declared-only: (none)
walked-only: 21 × @semio-tech/compose-*  +  @semio-tech/framework-graph-layout-run-rs
             @semio-tech/framework-tool-run-rs  @semio-tech/print-viz-kernel
```

Three of those 24 are live packages absent from `workspaces`. Deriving the list from the array would
silently drop them out of `optimizeDeps.exclude` for every dev server in the fleet. **Not taken — §14.7's
caution was correct, and this is the measurement behind it.**

**Fix taken instead**: `🏗️builder/🌐️vite/🟦️.ts:1571-1581` — `workspacePackagesByRoot`, a per-repo-root
memo. The set cannot change under a running dev server (its vite config is evaluated once at boot), so one
walk per process is the whole truth, and a process loading several configs now pays it once instead of
once per config. The two tests' budgets went 20 000 → 90 000 ms against the measured 14.4 s cold / 27 s
loaded, with the reason in the comment.

**Proof** (`🗑️generated/k2-styling-vitest.txt`, idle machine):

```
 Test Files  3 passed (3)
      Tests  78 passed (78)
   Duration  29.78s
```

— green, and the whole owner went **74.4 s → 29.8 s**. No ownership hash changed: the owner *config* was
not touched, only the module it collects and that module's suite.

**Session 5b files changed** (adds to §15): `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`
(memo), `…/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts` (two measured budgets).
Captures added: `k2-os-test-s5.txt`, `k2-cold-pair-rust.txt`, `k2-styling-vitest.txt` (re-run).
