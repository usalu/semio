# Result testfix-defect (2026-10-20) - STAGED, waiting for GO (nothing written in the repo outside C/)

Scope: 21 real-defect suites + 168 envelope-gated suites of `🗑️generated/tst8-census.json`, vitest reds that are real defects. Phase rule held: no repo write, no `bun repo`, one process at a time in the overlay copy `/private/tmp/testfix-defect/r` (copy of 🧰️framework + 🧪️tests + root files, workspace links in `node_modules` re-pointed into the copy, no symlinks into the repo). Law drift belongs to testfix-law (no result file of theirs existed at 17:30; no overlap touched).

## 0. The census is stale (important for planning)
The tst8 classes were produced before the 14:17 install generation and several peer batches. Re-reading the sources showed that many "real-defect" notes are no longer true (paths fixed since, bare imports now linked: `framework-filesystem`, `framework-transient`, `framework-hash` resolve in generation 81b8...). `exact` cargo suites do import their helpers (the note "lost its imports" is a heuristic artefact); the dashboard ENOENT paths are from an older test text (current `bootstrap` test uses `dashboard/📦️packages/🦀️rust`). Truth requires the live run below.

## 1. Harness: genuine Original birth for bun suites (staged, verified in overlay)
Root cause of the envelope-gated class: census runner minted an envelope with empty `capabilities` and no `SEMIO_BUILD_MODE`/provider map/SDK scope, so every suite reading native caller capabilities, the Nx tooling identity or the build mode failed before its first assertion (and a failed first test consumes the single process birth, producing the cascade `Previous build test law remains unsettled` / `Original process admission closed`).
New repo module `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌱️genuine-birth/` (schema-first):
- `🔣️.json` + `🧬️schema/🔣️.json`: closed birth ladder (setup -> setup-sdk -> orchestration -> model); a birth with `after` is only tried when the previous failure output matches it.
- `🟦️.ts`: `runGenuineBunTestV1` births the envelope with the REAL `createBuildBootstrapCapabilitiesV1(root, args, env)` (same function as `runBuildCliV1`), issues script providers, default artifact dir via `repoTestArtifactEnvironment` (short path, fixes the 256-char leases), `SEMIO_BUILD_MODE`, optional installed-Nx SDK scope (`withRepositoryToolingStoreV1` + `withNxToolingUseV1` -> `SEMIO_NX_TOOLING_ROOT/IDENTITY`), runs `bun test ./<file> [--test-name-pattern]` as owned child; plus `readGenuineBirthPolicyV1`, `nextGenuineBirthV1`, `genuineTestNamePatternV1`.
- `📜️script.ts`: `bun <dir>/📜️script.ts --file=<test> [--birth=<id>] [--pattern=<regex>]`.
- `🧪️tests/🟦️.ts` (+ `🧫️fixtures/🔣️.json`, probe `🧫️probe/🟦️.ts`): language-agnostic fixture test; third-party cross-checks: Ajv strict schema, RegExp oracle and bun's own `--test-name-pattern` for the name escape, real `createBuildBootstrapCapabilitiesV1` per birth, end-to-end harness run observing the issued envelope.
- Verified in overlay: `bun test ./…/🌱️genuine-birth/🧪️tests/🟦️.ts` 5 pass / 0 fail / 36 expects; CLI on the probe, birth `setup`: 3 pass.
- Runner `C/testfix-defect-tools/run.py` (reads the ladder from the same `🔣️.json`; ladder by failure signature; failing file is re-run one process per failing test name; JSONL results; <= 2 jobs): replaces `tst8-tools/run.py`.
- Overlay warning: suites that need rustup/images (cargo profile) start downloading a toolchain and mount images under the overlay `.cache`; I detached and deleted them (hdiutil clean, 0 images). Do not run those in an overlay.

## 2. Real-defect fixes staged (all via `C/testfix-defect-tools/apply.py`, anchored by sha256 of the current file, refuses on drift, dry-run default, all-or-nothing; staged copies in `staged/NNN.ext`, map in `manifest.json`)
| # | target | defect | state |
| --- | --- | --- | --- |
| 001 | `🧪️tests/🧫️fixtures/🧫️diff-only-law-gate/🫧️lanes/🧬️schema/🔣️.json` (new) | fixture schema was never committed; `lanes` test reads it | verified overlay: 4 pass |
| 002/003 | `🎒️pack/🔤️json/🧪️tests/🚮️absence` test + fixture | isolated workspace lacked `📁️filesystem` + `📚️compiler` roots (process module now imports them) and workspace links for bare `@semio-tech/*` imports | verified overlay under the genuine harness: 1 pass (reference route typechecks and runs with products/S/Hub absent) |
| 004-010 | genuine-birth module | harness (see 1) | verified overlay |
| 011 | `…/🏪️store/🧪️tests/📥️phase/📋️callers.json` | roster path `🎛️dashboard/📦️installation/🟦️.ts` moved under `📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/` | static; the only missing of 122 roster paths |
| 012/013 | workspace-exports fixture + teaching `📦️resolution` vitest | `@semio-tech/framework-manifest/generated/ui-axes` is now the generated projection itself (source == target), no longer a re-export wrapper; row moved to `generatedExports` with a projection test (header `@generated from 🖱️ui/🎚️axes/🔣️.json`, exact exported values/types via TS AST, resolve.exports + TS resolution) | static, vitest not run |

vit4 staged edits (storage/fixed lazy path, gis bridge hostManifest, engine react fixture path) are ALREADY APPLIED (`vit4-tools/staged-apply.py` reports "already applied" x4).

## 3. Diagnosed, not yet staged (need the live run to finish)
- `⚡️caching/🧪️tests/🧾️cargo-provenance` test 3: root `Cargo.toml` no longer defines `[profile.*]` (profiles are rendered by `🏗️build/🎚️mode/🦀️profile` selectors), so `manifest.profile[name]` is undefined. Rewrite: resolve incremental from the rendered configuration (`protocol.settings.*.incremental === false`) and run the cargo unit-graph oracle with `appendCargoProfileConfigurationArgumentsV1` selectors. Needs live cargo.
- `🏃️process/🎛️owned-execution/🧪️tests` (+ 6 siblings: `🧪️testing/🦀️cargo/🎯️exact`, `🧭️routing/…/📤️fractional`, `⚡️caching/…/📬️test-body`, `🎛️owned-execution/…/📥️original-control`, `🗂️workspaces/🦀️cargo/…/📥️original-control` x2) fabricate structural `ScriptInvocation` literals; the issued-Original law (`requireIssuedScriptInvocationV1`) now refuses them ("Live Original process issuance required"). Real drift: re-derive from `withBuildTestInvocationV1` + `withScriptInvocationSignalV1` for cancellation; publication-refusal injection needs a different seam. Reproduced in overlay (2 of 6 fail even with the genuine birth).
- `🧪️markdown-inline-references`: `snapshot()` refuses the oracle `package.json` inside the installed store (`Unsafe named input`); verify live, then either allow the declared store root in `ancestry()` or read the oracle through a copied snapshot.
- `🧪️storybook-discovery` / `🧱️owned-script-routes` / `🔗️import-edges`/`🏛️graph` / `🛑️coordinator/🔮️oracle` (`import.meta` in non-module: runs through `testNxCoordinator` `new Function`?) / `🏗️build/🏃️execution/🧪️tests` (times out waiting for "prepared execution inventory" under the setup birth) / `🕸️graph/🧭️portable-cli` (needs orchestration birth): overlay results are polluted by missing `.storybook`, `🏢️semio-tech`, tickets; decide live.
- Dashboard suites (launch, bootstrap, authority, installation, registry, coverage, cli, journeys): the dashboard module is peer-active (DASHBOARD-LAUNCH-COCKPIT); the "Dashboard is not installed" six need `bun run dashboard:install` (host state), not a code fix.
- vitest: teaching ui-axes row staged (012/013); ui-react `react-package-export` (layered renderer imported through `@semio-tech/ui/elements/LayeredOverview`, external rule hides it) and wgpu/engine react Rust-text laws are peer-active (ui split, renderer split): re-run first, fix only what stays red.

## 4. After GO (exact commands)
```
python3 C/testfix-defect-tools/apply.py            # dry run: 13 ready, 0 drift expected
python3 C/testfix-defect-tools/apply.py --write
C/testfix-defect-tools/live.sh real                 # 21 real-defect suites, 2 jobs
C/testfix-defect-tools/live.sh env                  # 168 envelope-gated suites, 2 jobs
# vitest: C/vit4-tools/one.sh "<config>" for: teaching architecture course, ui-react, gis inference bridge, engine react, wgpu
```
Then triage `🗑️generated/tfd/*.jsonl` (`isolated` map = test passes alone -> test-order/birth-consumption issue, otherwise real defect), fix the 7 structural-invocation tests and cargo-provenance, re-run, record here.

## 5. Verification actually run (pre-GO, overlay only)
- `bun test ./🧪️tests/🧪️diff-only-law-gate/🫧️lanes/🟦️.ts` with 001: 4 pass.
- genuine harness on `…/🚮️absence/🟦️.ts` with 002/003: 1 pass (earlier iterations: missing filesystem root, bare imports, missing compiler root).
- `bun test ./…/🌱️genuine-birth/🧪️tests/🟦️.ts`: 5 pass.
- runner `run.py` on 12 of the 21 real-defect suites in overlay (setup birth): 1 green, rest need live repo state (see 3).
- NOT run: anything live, vitest, cargo, 011-013, the 168 envelope suites at scale.

Files: tools `C/testfix-defect-tools/{stage.py,apply.py,run.py,live.sh,manifest.json,suites-*.txt,staged/}`; overlay `/private/tmp/testfix-defect/` (744 MB, no mounted images; delete when done). No `[DEBUG]` lines added.
