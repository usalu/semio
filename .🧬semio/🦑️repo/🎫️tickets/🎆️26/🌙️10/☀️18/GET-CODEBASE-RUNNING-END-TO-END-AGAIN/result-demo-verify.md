# result-demo-verify (executor demo-verify, wave 1022a) - WIND-DOWN 22:10

Projects: `@semio-tech/mit-bestand-{demonstrator,bericht,praesentation-projektetage}`. Tools `C/demo-verify-tools/` (`run.sh`, `wait.sh`, `chain-bazel.sh`); logs `C/🗑️generated/demo-verify/`.

| project | target | nx | bazel |
|---|---|---|---|
| demonstrator | test | BLOCKED (Nx tool path, below) | GREEN rc 0 (PASSED 26 s, after 2 fixes) |
| demonstrator | canonical-architecture | BLOCKED | GREEN rc 0 (4 tests) |
| demonstrator | test-runtime-catalog | BLOCKED | GREEN rc 0 |
| bericht | akteursnetz-validate | BLOCKED | GREEN rc 0 |
| bericht | check-generated | BLOCKED | RUNNING (chain bz1) |
| bericht | test (incl. rewritten PDF.js part) | BLOCKED | PENDING (chain bz1, last) |
| praesentation-projektetage | test | BLOCKED | PENDING (chain bz1) |
| praesentation-projektetage | canonical-architecture | BLOCKED | PENDING (chain bz1) |

## HANDOVER: detached Bazel chain bz1
Wrapper pid 66743 (`chain-bazel.sh bz1 ...`), summary `C/🗑️generated/demo-verify/bz1-summary.txt` (one line per target, `done` at the end), per-target logs `bz1-*.log/.rc`. Runs are slow only because of the fleet capacity queue (load 20-37). Risk: praesentation `test` may need `resolveTestLevel(segments, "quick")` too (15 s fundamental budget) if its log shows `[budget] ... exceeded 15000ms`.

## Fixes (live edits)
1. `♻️mit-bestand/🧺️demonstrator/📜️script.ts`: `resolveTestLevel(segments, "quick")` (suite fixed cost 17-20 s exceeded the 15 s fundamental budget; same floor as the react product scripts).
2. `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🎭️e2e/🟦️.ts`: harness for `commandUrlReferences` now injects `parserTypescriptV1` (the function stopped using `require`), fixing `parserTypescriptV1 is not defined`.
(Earlier demo-libs fixes unchanged; see result-demo-libs.md.)

## Nx lane (all targets)
`bun repo run <p>:<t>` on Nx never reaches the target: (a) first run: daemon child dies `Original native caller capabilities required` (nx@23.2.0.patch `server/start.js` -> `withNxDaemonServiceV1`; likely a service-envelope capabilities object lacking `native` hitting `readNativeOwnerCapabilities`/`readNativeCallerAuthorityV1`), then parent repo spins 150 % CPU in `claim-retirement-scan` (killed by me: 29822/10744/10742/10738); (b) second run: `Original process admission closed` thrown from `evictIdleNamespacesV1` (Nx graph pool out of room) after "nx-exit storage closed"; the primary error is masked. course-rest2 is working this live (logs course-rest2/nxfail4-6, `[DEBUG] nx-exit` prints in `⚡️caching/🚀️bootstrap/📜️script.ts` are theirs). No successful Nx run exists in any fleet log. My temporary [DEBUG] in `readNativeOwnerCapabilities` was removed. Re-run Nx after their fix: `SEMIO_BUILD_GRAPH_POLICY=current nice -n 10 bun repo run <project>:<target>`.
