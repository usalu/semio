# result-cisource (lane cisource `cs`, wave 1027b, 10-27 ~18:45): the dashboard CI Source gate follows its manifest; batch `cisrc` staged

`C` = this ticket folder. Tools `C/cisource-tools/`. Nothing live edited, no product run, no sub-agent, no modifying git command. Mirror `cs1` (mine, one) was used for every law and is dropped at the end.

## FOR coordinator
- Apply: `python3 -I C/cisource-tools/staged/cisrc/stage.py check|apply|revert` (3 files, all `base` on live of 18:39; keeps modes; `python3 -I C/xplat5-tools/tests/test_stage_mode.py C/cisource-tools/staged/cisrc/stage.py` OK).
- Rebuild on the current live bases (anchored edits, exit 2 on a moved anchor): `python3 -I C/cisource-tools/mkbatch.py`. Mirror: `M=$(sh C/cisource-tools/mirror-up.sh); python3 -I C/cisource-tools/staged/cisrc/stage.py apply --root $M`.
- Laws (mirror only): `python3 -I C/mirror-tools/mirror.py run cs1 -- sh C/cisource-tools/staged/cisrc/verify.sh $M` (law file, dynamic oracle, 5 negative controls, stage mode law). Result today: all green.
- Product runs: none prove it (a bun test of one file, no graph). It also makes the `dashboard CI test 4/1` of `result-cicover.md` 6/0.
- ORDER with cicover's `ci1`: both edit `🏗️ci/🔣️.json` and `🧬️schema/🔣️.json` (other lines: host runner pins vs `sourceGate.inputs`). Whichever lands first makes the other's `stage.py check` say `drifted`; rebuild the second (`python3 -I C/cicover-tools/mkbatch.py --batch ci1` or `python3 -I C/cisource-tools/mkbatch.py`). Proven both ways of content: ci1 applied first, then `cisrc` built on top (`--root <mirror>`) gives dashboard CI law 6/0, negative controls OK. After `cisrc` lands the NOTE in cicover's `verify.sh` ("known pre-existing set") is obsolete.

## Decision: the law was stale, the input is right
1. The test `🧪️tests/🏗️ci/🟦️.ts` itself does `readFileSync(join(root(),gate.owner,"📋️project.json"))` and asserts on `targets["test-ci-source"]` (executor, cache, `inputs`, `metadata.semio...`). The cached result is a function of that manifest: an edit of the target (or of any key in the file) must re-run the gate. A declared input is therefore required.
2. keys3-g's Seatbelt oracle observed it (`result-keys3-g.md` item 3 table: `repo-dashboard-rs:test-ci-source | clean, passed | its own 📋️project.json (FIXED in the batch)`), and the batch added `{projectRoot}/📋️project.json` to the target.
3. My own oracle agrees: `cisource-tools/trace-reads.ts` (a `mock.module("node:fs")` wrapper, Bun's preload; patching the `fs` object does NOT reach named imports under Bun) records what the test opens. On the pre-existing tests: `.github/workflows/dashboard.yml`, `.github/actions/native-repo/action.yml`, `.github/actions/devcontainer-repo/action.yml`, `📋️project.json` of the owner (readFileSync) and `.github/workflows/build-system.yml` (existsSync probe of `root()`), nothing else (imports of `🔣️.json`/schema/coverage fixtures and the task-routes script come through the module closure: the script-inputs provider resolves JSON imports, `resolveJsonModule: true`, and the nearest `package.json`/`tsconfig.json`/`bun.lock`).
4. So four of the five data reads were declared and the fifth (the manifest) was not covered by the law: the fixture (`inputs` 9 entries), the schema (`minItems/maxItems 9`, pattern `^\{workspaceRoot\}/`) and the test (`inputs:9`) were the drift. Reproduced on base in the mirror: `4 pass 1 fail`, diff `+ "{projectRoot}/📋️project.json"`.

## What `cisrc` changes (3 paths, all in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🏗️ci/`)
| path | change |
|---|---|
| `🔣️.json` | `sourceGate.inputs` gains `{projectRoot}/📋️project.json` (10 entries) |
| `🧬️schema/🔣️.json` | `inputs`: `minItems/maxItems` 10, `contains {const "{projectRoot}/📋️project.json"}`, item pattern `^\{(?:workspaceRoot\|projectRoot)\}/` (schema-first: the manifest is mandatory by schema, not only by a count) |
| `🟦️.ts` | (a) `inputs:9` -> `inputs:target.inputs.length` (uniqueness law instead of a magic number; the count is closed by the schema); (b) the schema must refuse the first 9 inputs; (c) NEW test "declares every workflow and manifest file its test reads": extracts every `.github/**.yml` literal and every `gate.owner,"<file>.json"` literal of the test source, requires each as a declared input (`{workspaceRoot}/..` / `{projectRoot}/..`, at least 5, the manifest among them), and requires every declared input to exist on disk (a typo can no longer pass as long as fixture and manifest agree) |

## Laws and evidence (mirror `cs1`)
- Red on base: 4/1 (the pre-existing red). Green with the batch: 6/0 (145 expects). Composed on cicover `ci1`: 6/0 (147).
- Independent oracle (`check-observed.py`): the files the pre-existing tests open (recorded by the fs wrapper, not by the law's regexes) are all declared inputs AND equal the law's static extraction (5 files). Run by `verify.sh`.
- Negative controls (`negative.py`, seeded on the mirror, each fails EXACTLY the named tests, files restored byte for byte): fixture entry removed only -> Source gate + closed-vectors test; manifest input removed only -> Source gate + new law; removed everywhere -> closed-vectors (schema) + new law; a new `document(".github/dependabot.yml")` read without input -> new law only; dangling input spelled identically in fixture and manifest -> new law only.
- Parse check (`gate-g-tools/scan.ts`): the changed TS 1/0 bad; both JSON files parse.

## Other products' CI-source gates (same drift?)
Searched the tracked tree (`git grep`) for tests that read `.github/**` and for fixtures/laws that pin a target's `inputs` list.
| gate | reads | declared | verdict |
|---|---|---|---|
| dashboard `test-ci-source` | 3 yml + manifest + build-system probe | now 10 inputs | FIXED here; the only target named `*ci-source` in the repository |
| course `test` (deploy/preflight/native readiness: `architecture-course.yml`) | that workflow only | named input `default` lists `{workspaceRoot}/.github/workflows/architecture-course.yml` | no drift; no test reads another `.github` file (checked: `🚀️deploy/🟦️.ts`, `🧪️deploy/*`, `🩺️preflight`, `🚀️readiness` fixtures) |
| workflow law + CI report tests + direct-tool law (library `🧾️ci-report`, `⚖️laws`) | whole `.github` | no cached target owns them (suite sweep, no inputs list) | not applicable |
| other laws pinning `inputs` (hub foundation/socket-grant, catalog, pixels, os store, staging-root, library command-source, native-orchestration, nx-plugin) | named inputs of their own projects | they compare to named inputs, none touched by keys3-g's two manifest edits | no drift found by reading; not run (no staged change to them) |
| FINDING, not fixed: repo-lib `test-build-surfaces` (`🧪️tests/🔧️build-surfaces/🟦️.ts`) | `.github/workflows/architecture-course.yml`, `.devcontainer/devcontainer.json`, root `package.json`, root `📜️script.ts`, several project manifests | `default`, `^default`, `testEnv`, `reads` = `library/**` only | the same class (cached test, data read outside the key): a course-workflow or devcontainer edit is served from the cache. The suite is also one of tst4's stale suites (`result-tst4.md`: 20/1 under the envelope). Fix belongs with that test-debt/inputs lane (needs the full read set from a passing run; one entry per file in the target's `inputs`, and the same fs-wrapper oracle, `trace-reads.ts`, works for `bun test`). I did not edit the 3908-line hot repo-lib manifest |

## Not verified
Nothing here runs the graph; the cache-key effect (target key moves when the manifest changes) follows from the declared input, which keys3-g's batch already landed in the manifest (the manifest line is in the index); I did not run an official `repo-dashboard-rs:test-ci-source`.

## Files
`C/cisource-tools/`: `mkbatch.py`, `staged/cisrc/` (manifest, files, base, `stage.py`, `verify.sh`), `verify.sh`, `negative.py`, `check-observed.py`, `trace-reads.ts`, `mirror-up.sh`. No `🗑️generated` output.
