# result-td-misc (lane td-misc, mirror prefix tm, wave 1027b test-debt)

FOR coordinator: 5 batches READY, all `base` on live 15:50, disjoint from each other, any order. Apply: `for b in texts artdir resident windows consumption; do python3 -I C/td-misc-tools/staged/$b/stage.py apply; done` (revert: same with `revert`, reverse order). Each has `verify.sh <root>`; one-shot proof on a fresh mirror: `sh C/td-misc-tools/verify-all.sh tmv` (creates the mirror with the repository's Cargo manifests from `td-misc-tools/extra-only.txt`, applies, verifies; windows/consumption need a MIRROR root because they run inside the official envelope via `td-misc-tools/envrun.sh`).
FOR coordinator: no product run needed (test/fixture files + 2 product files: process suite runner refusal text, resident state `since`). Suites that prove them: `repo run @semio-tech/framework-process:test-testing` (texts, artdir), the build-resident suite (resident), course:test unaffected.
FOR coordinator: FINDING for EVERY lane: `backlog-tools/live-laws.sh` (source of a1-laws.txt) runs `cd "$(dirname "$f")" && bun test ./🟦️.ts`. Outside the repo root Bun does not load the root `bunfig.toml`: the run uses the ambient bun 1.3.14 without the runtime-pin and orphan preloads (logs start `bun test v1.3.14` only; root runs re-exec into `v1.4.2`); and it concatenates the digits of all `N pass`/`N fail` lines, so ci-report suites (a nested fixture run prints `5 pass`/`1 fail` before the real `16 pass`/`0 fail`) reads `516 pass 10 fail`. Truth = `bun test ./<repo-relative file>` from the repo root.
FOR td-leases: the declared birth (`declareBuildTestEnvelopeV1`, capabilities `{artifacts?,command?}`) covers 5 of my 9 envelope-only files but NOT the library build laws that need the native-caller build grant (`createBuildBootstrapCapabilitiesV1` of their target, and WITHOUT `artifacts`: with it they refuse `/artifacts is not allowed`). Requirements table below. I did NOT stage `declare…` lines: a declaration that differs from what the official parent grants is a hard refusal beneath that parent, and the library grants are not expressible yet.

## Method
Mirror `tm1` (`--only library process launcher 🧰️framework/🔨️modules/⚙️runtime` + the 557 Cargo manifests/locks the fixtures name, `td-misc-tools/extra-only.txt`). Each file run 4 ways (`td-misc-tools/run.sh`): `plain` (repo root, bare), `art` (+ `SEMIO_TEST_ARTIFACT_DIR`), `a1` (the a1 recipe: cwd = test dir), `env` (`td-misc-tools/envrun.sh <mirror> <project:target> <file>`: ver-f proc-run with the OWNING target's bootstrap grant, mirror root realpath'd (else `Exact Original workspace required`), `NOART=1` = no artifacts capability, `ONLYART=1` = only `{artifacts}` = td-leases' declared birth). Logs `🗑️generated/td-misc/p0` (base), `p1` (Cargo mirror), `p2` (all batches applied).

## Classification (base = live code in the mirror)
| suite | a1 | bare (root) | classification / evidence | status |
|---|---|---|---|---|
| 🧾️ci-report/🗂️suites | 516/10 | 16/0 | NOT failing: a1 digit concatenation (nested fixture run) | none needed |
| 🚀️launcher/📌️runtime-pin-contract | 13/1 | 15/0 | NOT failing: a1 recipe (no root bunfig → bun 1.3.14, no pin preload; a1-mode 14/0) | none needed |
| 💾️storage/🎛️phase | 0/8 | 0/8 | envelope-only: 7/0 under the process grant, ONLYART, and the build grant w/o artifacts | needs declared birth `{}` or `{artifacts}` |
| 💾️storage/🚪️admission | 0/2 | 0/2 | envelope-only: 1/0 in every envelope | needs declared birth `{}`/`{artifacts}` |
| 💾️storage/📒️ledger | 4/4 | 3/5 | envelope (build grant w/o artifacts + SEMIO_TEST_ARTIFACT_DIR env); then the first law HANGS (60-180 s) waiting on the shared `node` lease (`graph:incremental-ledger`) on a freshly created leases image in the mirror | UNRESOLVED: needs one official `repo run repo-graph-maintenance:test` on live to tell mirror-environment from bug |
| 📥️inputs/🧮️digest | 3/3 | 3/3 | envelope-only: 5/0 with `{artifacts}` (w/o artifacts: `Ticket Source fixture sink required`) | needs declared birth `{artifacts}` |
| 📥️inputs/📏️windows | 2/2 | 2/2 | envelope + STALE FIXTURE `leaf-transient` (reader retries interrupted reads since 10-25 → accepted with oracle digest, 351 vs 297 ops, 3/3 runs) | batch `windows` (envelope run 3/0); then birth `{artifacts}` |
| 📊️reporting | 2/4 | 2/4 | envelope (build grant w/o artifacts) → 5/0 once the mirror's leases partition is an image; ENOTEMPTY on `.cache/b/l/m` was mirror state (`l/m/data` written at 14:47 without `image.json`; not reproduced by later bare/envelope runs) | needs build-grant birth |
| ⌨️command | 2/4 | 2/4 | envelope (build grant) then REAL: `runCapturedBuildSourceV1` with a fake proof resolves instead of `Authenticated captured…`, and `requireCapturedBuildCommandV1` does not refuse changed identities — its identity check is skipped while `implementationPinnedV1()` is true; bare (law 1 refused early) law 2 passes, so the pin state of law 1 leaks into law 2. Module is under active edit (📥️entry adopt/farm worker, uncommitted) | UNRESOLVED → owner of `📥️source/🔏️closure/🚪️command` + `📥️inputs/🔏️revision` pin |
| 🎚️mode/🦀️profile/📥️consumption | 14/11 | 20/5 (22/3 with manifests) | a1 recipe adds 6; real TEST BUG: hard-coded sink in the 10-13 ticket folder (ENOENT anywhere else + cascade); envelope = build grant w/o artifacts | batch `consumption` (24/0 in that envelope); then build-grant birth |
| 🏡️resident/🏛️service | 31/3 | 30/4 | REAL BUG + test env: `dirty` answered complete from a non-exact journal (backend none/poll/portable); the 3 laws silently ran with backend `none` because no watcher binary is configured bare; protocol-abuse law flaky under load (3.07 s vs 3 s wait, passes otherwise) | batch `resident` (35/0, 1 skip = crash safety without fixture tools dir) |
| 🏛️sealed/🗂️store | 10/2 (flaky) | 15/0 ×4 | NOT failing at the root: the 60 s `repo leases status did not end` self-check timeout reproduces only in the a1 recipe (bun 1.3.14 without pin) | none needed |
| 🧪️tests/🏗️native | 4/2 | 2/4 | envelope-only + SEMIO_TEST_ARTIFACT_DIR: 5/0 with `{artifacts}`; "All Cargo workspaces…" was mirror-only (manifests absent) | needs birth `{artifacts}` |
| ⚙️runtime/🧪️tests | 32/2 | 31/3 | NOT failing on live data: 34/0 with `SEMIO_REPOSITORY_ROOT=<live>` both at the root and a1-style; mirror reds = `git ls-files` without `.git` (0 files compared) | none needed |
| ⚙️runtime/🟢️node/🧲️native | 0/1 | 11/0 | NOT failing: a1 recipe (bun 1.3.14 FFI `strlen` TypeError) | none needed |
| 🏃️process/🎛️owned-execution | 5/1 | 2/4 | test needs SEMIO_TEST_ARTIFACT_DIR (green with any dir incl. tmp) | batch `artdir` (6/0 bare) |
| 🏃️process/📖️texts | 2/1 | 2/1 | REAL: `script.suite-unknown` added (xplat4 platforms) but the process runner still throws an uncatalogued English `Unknown test suite` | batch `texts` (3/0) |
| 🏃️process/🧪️testing/🎛️execution | 2/1 | 1/2 | needs SEMIO_TEST_ARTIFACT_DIR + the budgeted-command law had no timeout (Bun 5 s default; siblings declare 60 s) | batch `artdir` (3/0 bare) |

After all 5 batches, bare root run (`p2`): every remaining red is `Original parent Script process invocation required` (+ the artifact dir for ledger/native) in phase, ledger, admission, digest, windows, reporting, command, consumption, native; runtime = mirror-without-git only.

## Batches (`C/td-misc-tools/staged/<batch>/`, built by `td-misc-tools/build.py` from `edits/<batch>.py` against live; stage.py = xplat5 template, mode law OK; `gate-g-tools/scan.ts` 10 files bad 0)
- `texts` (1 file): `🧪️testing/🗂️suites/🏃️run/🟦️.ts` refuses an unknown suite with `refuseProcessV1("invalid","script.suite-unknown",{suite,known})`. Law: texts 2/1 → 3/0. The process suites law keeps its pre-existing red (`every test file of the process module belongs to exactly one suite…`, same on base).
- `artdir` (2 files): owned-execution + testing/execution tests use `SEMIO_TEST_ARTIFACT_DIR ?? tmpdir()` (house idiom, 80 uses); budgeted-command law `60000`. Laws: bare 2/4 → 6/0 and 1/2 → 3/0; with an artifact dir 6/0, 3/0.
- `resident` (6 files): `🗃️state/🟨️.mjs` `since()` returns `{complete:false,reason:"trust"}` unless trust is exact (+ `.d.mts` union); state law row (verify mode → since proves nothing) and service law row (backends none/poll: `dirty` after a silent edit is incomplete) — both RED on base code, GREEN after; test helper `residentWatcherV1()` = configured binary else `ensureResidentWatcherV1` (the provisioned `repo-watch`, cached on live, 3.5 s compile in a fresh mirror); service + watch laws use it; crash-safety describe gated on `SEMIO_REPO_NATIVE_DIR` (its `--bin` tools). Laws bare: state 13/0, watch 5/8skip → 13/0, service 30/4 → 35/0/1skip, graph-hooks 12/0; other resident files unchanged (journal bench 0/1 pre-existing).
- `windows` (2 files): fixture `leaf-transient` → `fdStable:true, accepted:true` (the SQLite oracle `accepted = fdStable∧namespaceStable∧completed` holds), law prices a healed race `readOperations >= single pass` and still demands the oracle digest. Envelope run 2/2 → 3/0 (twice).
- `consumption` (1 file): sink = `realpathSync(SEMIO_TEST_ARTIFACT_DIR ?? tmpdir())`. Envelope (build grant w/o artifacts) 22/3 → 24/0.

## Declared-birth requirements (for td-leases / the coordinator)
| file | minimal birth that makes `bun test ./file` green |
|---|---|
| 💾️storage/🎛️phase, 🚪️admission | `{}` or `{artifacts:true}` |
| 🧮️digest, 📏️windows, 🏗️native | `{artifacts:true}` (native also needs the published SEMIO_TEST_ARTIFACT_DIR) |
| 📥️consumption, 📊️reporting, ⌨️command | build bootstrap grant of a library target (`tool/workspaceRoot/native…`), NO `artifacts` key |
| 📒️ledger | build grant, no `artifacts` key, SEMIO_TEST_ARTIFACT_DIR set — and the hang above |

## Leftovers / notes
- Mirror `tm1` dropped at the end (its two partition images detached first). `🗑️generated/td-misc/` holds the evidence logs.
- `td-misc-tools/`: run.sh, envrun.sh, errs.py, build.py, verify-all.sh, edits/*.py + *.verify.sh, files.txt, extra-only.txt.
