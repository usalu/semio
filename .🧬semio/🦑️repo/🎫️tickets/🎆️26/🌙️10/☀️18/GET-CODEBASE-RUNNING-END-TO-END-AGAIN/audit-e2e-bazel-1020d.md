# 🧭 Audit: end-to-end chain on Bazel for course, play and demonstrator (wave 1020d)

Auditor `aud-e2e-1020d`, read-only, 2026-10-20 17:53 CEST. `C` = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Course ticket `K` = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL`. Repo root `R` = `/Users/ueli/Documents/semio`.

Tags: VERIFIED-READ = read or checked on disk now (file, hash, or port probe). INFERRED = derived from a document or code path, not run. Nothing was built, run or changed. Only commands run: reads, sha256 of files, `ps`, `/dev/tcp` port probes, `node`/`bun`/`df` version checks, `python3 -I` reads of JSON manifests.

## 0. Bottom line

1. Nothing can run yet. `bun repo show projects` (chain14, pid 6822/6823, started 17:48) is queued behind peer pid 15549, an exclusive `bun` graph build at 1h23m elapsed. VERIFIED-READ `C/🗑️generated/chain14/show-projects.err` ("position 3 of 3, behind the request of process 15549 (exclusive, queued 4956 s ago)") and `ps`. Every `bun repo --tool=bazel` step depends on show green.
2. Play and demonstrator can run cold today through the `--tool=bazel` chain. Their warm (cache-hit) re-activation is not achievable yet: the `pg-chain` set that makes the 35 + 9 lane `prepare` tasks cacheable is not staged. VERIFIED-READ `C/pg-chain-tools/edits.py` (empty `EDITS={}`, `NEW={}`, `STAGED=""`), `C/pg-chain-tools/bases.json` (`{}`), no `C/result-pg-chain.md`.
3. The play and demo staged sets (`C/play-bzl-tools`, `C/demo-bzl-tools`) are NOT applied. VERIFIED-READ: the sha256 of all 3 play and all 6 demo base files equals the live file (`bases.json` = live, so pending).
4. Course on Bazel: the chain is wired, but the course closure paths are 250–262 UTF-16 units against the AGENTS 256 cap, and the fix (`x-paths`) is staged only. VERIFIED-READ `K/audit-xplat.md` P0, `K/result-x-paths.md` (status "DONE, staged only"). The proctor is still built in-process on the live default (`PROCTOR_ACQUISITION_DEFAULT = "build"`, `K/result-x-proctor.md` §5).
5. The earlier B1 blocker (`--release` leaking into play/demo) is fixed in the live `C/e2e-tools/run-e2e.sh` (line 229, scoped to `test-e2e`; line 70 rejects `--release` for non-course). VERIFIED-READ.

## 1. Commands (exact)

Entry point: `C/e2e-tools/run-e2e.sh <play|demonstrator|course> [--tool=nx|bazel] [options] [-- <extra>]`. VERIFIED-READ `C/e2e-tools/run-e2e.sh:4-22` (usage), `:74` (`bun repo ${tool[@]} "$@"`, so `--tool` precedes the subcommand as `bun repo --tool=bazel run …`).
Options: `--install-browsers` (runs `workspace:deps-browsers`, about 300 MB, only if missing), `--skip-activate`, `--no-serve` (play/demo default is serve on), `--serve` (course), `--topology=dev|rehearsal|both`, `--proctor=<exe>`, `--release` (course only, exports `SEMIO_BUILD_MODE=release` for the `test-e2e` step only, `:229`), `--warm-proctor` (course: runs `@teaching/proctor:build` first, `:212-214`), `--keep`, `--serial`, `--check` (preflight only). Env: `E2E_SERVE_TIMEOUT` (default 1800 s), `E2E_MIN_FREE_GB` (default 15). Logs: `C/🗑️generated/e2e-<product>-<stamp>-<step>.log`. VERIFIED-READ `run-e2e.sh:9-22, :32, :78`.

Play (`@semio-tech/semio-tech-play`, port 6033), `run-e2e.sh play --tool=bazel` runs in order:
- `bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype`
- detached `bun repo --tool=bazel run @semio-tech/semio-tech-play:serve` (smoke, polled on `http://127.0.0.1:6033/`)
- `bun repo --tool=bazel run @semio-tech/semio-tech-play:test-e2e`, then the serve is stopped
VERIFIED-READ manifest targets `activate-prototype, prepare-prototype, serve, serve-e2e, prepare-e2e, test-e2e` in `R/🏢️semio-tech/🎡️play/📋️project.json`; script `run-e2e.sh:207-233`.

Demonstrator (`@semio-tech/mit-bestand-demonstrator`, port 6029): same shape with `activate-prototype`, `serve`, `test-e2e` (`R/♻️mit-bestand/🧺️demonstrator/📋️project.json`; `prepare-prototype`, `activate-lanes-prototype`, `prepare-e2e`, `serve-e2e` also present). VERIFIED-READ.

Demonstrator print report (`@semio-tech/mit-bestand-bericht`, manifest `R/♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json`; targets `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht`, `build`): `bun repo --tool=bazel run @semio-tech/mit-bestand-bericht:build-kompaktbericht` then `…:build` (aggregate). Not on the `run-e2e` chain; driven by `C/demo-bzl-tools/cold-run.sh report cold|warm|probe`. VERIFIED-READ `C/demo-bzl-tools/cold-run.sh:9`. Deck (`@semio-tech/mit-bestand-praesentation-projektetage`, targets `build, canonical-architecture, dev, test`): `…:build` twice (`cold-run.sh deck`). VERIFIED-READ manifest and `cold-run.sh:12`.

Course (`@teaching/architecture-course`, dev topology, port 6061 by the gate, not the serve): `run-e2e.sh course --tool=bazel` → `bun repo --tool=bazel run @teaching/architecture-course:test-e2e -- --topology dev --`. VERIFIED-READ `run-e2e.sh:218-225` (`args+=(-- "${gate[@]}" --)`), manifest `R/🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json` (targets `build, check, dev, dev-site, publish, test, test-e2e, typecheck, deploy-check, preflight, …`). Rehearsal/both need `--topology=rehearsal|both --release`. VERIFIED-READ `C/audit-e2e-readiness-1020.md` §2c.7 and `run-e2e.sh:70`. Course proctor: `@teaching/proctor:build` (receipt output `{projectRoot}/.🧾️store/build.json`, `cache:true`), manifest `R/🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`. VERIFIED-READ.

Direct (no wrapper), each one as `bun repo --tool=bazel run <project>:<target>`: `…:activate-prototype`, `…:serve`, `…:test-e2e`, and for the course `…:build`, `…:dev`, `@teaching/proctor:build`. VERIFIED-READ manifests. Course `build` is currently `cache:false`, `managed-output` (`R/🎓️…/📋️project.json`, `targets.build`); `K/result-x-site` stages `cache:true` + receipt, not applied. VERIFIED-READ `K/result-x-site.md` §2.

## 2. Chains per product, with dependencies

Play, activation (`R/🏢️semio-tech/🎡️play/📋️project.json`): `activate-prototype` dependsOn `prepare-prototype` + 35 `@semio-tech/framework-os-dev:activate-<lane>-react-prototype`. Measured closure (`C/result-play-bzl.md` §1): 35 lanes, 68 components, 35 `prepare-<lane>` (driver-owned, uncached), 35 `session-<lane>`, `s-dev-composition:generate` (91,035 declared files). VERIFIED-READ (document). Staged fix E1–E3 is in `C/play-bzl-tools/staged/`, not applied.
- serve `:6033` (continuous), test-e2e: `serve-e2e` → `prepare-e2e` → `activate-prototype`, `workspace:deps-browsers`; `test-e2e` `cache:false` (Playwright, 11 tests, 1 worker). VERIFIED-READ `C/audit-e2e-readiness-1020.md` §2a.3, `C/result-play-bzl.md` table row 16.
- readiness budget: `R/🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🧪️e2e/🔣️.json` `readyTimeoutMs: 600000` (estimate); the waiter uses it (`R/🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🟦️.ts:27`). VERIFIED-READ. The demonstrator uses the same budget (`R/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🟦️.ts:18`). VERIFIED-READ. So the 60 s default risk (R1) is resolved.

Demonstrator, activation (`R/♻️mit-bestand/🧺️demonstrator/📋️project.json`): `activate-prototype` dependsOn `prepare-prototype` (9 `framework-os-dev:prepare-<variant>-react-prototype` + 4 wasm targets) + `activate-lanes-prototype` (3 lanes). VERIFIED-READ `C/audit-e2e-readiness-1020.md` §2b. Staged set in `C/demo-bzl-tools/apply.py` (rows 4–9 of `C/result-demo-bzl.md` §2). Not applied (VERIFIED-READ, bases equal live).

Course, test-e2e (`R/🎓️…/📋️project.json` `targets.test-e2e`; `K/audit-closure.md` table row 246): dependsOn `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate`; router `R/🎓️…/📜️script.ts:131` → `runCourseEndToEnd` (`…/🎭️e2e/🟦️.ts:333`) → proctor (`acquireProctor` when staged, live default `buildProctor` = `bun repo cargo --store=shared … build --package teaching-proctor --bin proctor`, `…/🛂️proctor/🏗️bootstrap/🟦️.ts:259-263`) → dev stack (proctor on a fresh loopback port, `PROCTOR_MODE=development`, `dev-site` Vite) → Playwright, 14 spec files (`C/audit-e2e-readiness-1020.md` §2c.4–6). VERIFIED-READ (document). `test-e2e` is `cache:false`; it re-runs on every invocation by design.

## 3. Staged sets: what must be applied first

Status key: APPLIED = verified in tree; PENDING = staged and not in tree; MISSING = no set.

| set | scope | status now | evidence |
|---|---|---|---|
| integrate round 1 (groups A–G, 371 files) | generic adapter, caching, rexec, bworkers, zt-e2e-fix, graph-perf, uncached-owners, bzl-lazy | APPLIED 15:29 | `C/result-integrate.md` §1; `K/coordination.md` 15:29 entry. Note: integrate `fnsplit4b` Rust split not compiled |
| policy-admit | policy | APPLIED 14:47 | `C/coordination.md` 14:47 (6 writes) |
| bzl-stage (no `.bazelignore`) | filegroups, transitive inputs | APPLIED (live adapter line 448–450 removes `.bazelignore`; no `.bazelignore` in `R`) | `R/🧰️…/🏗️bazel/📜️script.ts:448-450`; `ls R/.bazelignore` absent. VERIFIED-READ |
| zt-e2e-fix, per-product readiness | e2e budget | APPLIED (budget file present, waiter uses it). Frozen part NOT applied: `R/🧰️…/⚡️caching/🚀️bootstrap/🛠️tools/package.json:45,54` still lists `@semio-tech/framework-filesystem` (needs dependency lock) | VERIFIED-READ |
| integrate round 2 (`integrate2`, agent a24bfde5ce7488156, incl. the course x-* sets) | all of the above remainder | NOT REPORTED. No `C/integrate2*` output | `K/coordination.md` 17:47 table; `C` listing. INFERRED: running or not yet started |
| play-bzl E1–E3 (`C/play-bzl-tools/apply.py`) | `prepare-prototype` cached, `activate-prototype` cached, `s-dev-composition:generate` inputs, `repo:generator-inputs` uncached | PENDING (bases = live: `R/🏢️semio-tech/🎡️play/📋️project.json`, `R/✏️s/🧑‍💻dev/📋️project.json`, `R/…/⚡️caching/📋️project.json`) | VERIFIED-READ (sha256) |
| demo-bzl (6 files, `C/demo-bzl-tools/apply.py`) | demonstrator prepare/activate cached, transitive restore on serve/dev/serve-e2e/prepare-e2e, report output slots, deck dev | PENDING (6 of 6 bases = live) | VERIFIED-READ (sha256) |
| pg-chain (component targets, lane prepare verdict, wasm fingerprint) | required for warm lanes | MISSING: no `C/result-pg-chain.md`; `C/pg-chain-tools/edits.py` is an empty stub; `bases.json` = `{}`. Owner executor acda06302c1e657ff (resumed) | VERIFIED-READ |
| hub-bzl (Go toolchain identity) | hub Go tasks | PENDING; not on any of the three chains. `R/🧰️…/🏗️bazel/🐹️go` does not exist | VERIFIED-READ (absence); INFERRED not on chain (no Go task in play/demo/course test-e2e closure) |
| course x-* sets: x-release (adapter P1-1 mode axis, staged adapter part), x-proctor (product path + flip), x-dev (live course code + staged manifests), x-site (receipt build, deploy-check), x-inputs, x-keys, x-ingress follow-up, x-graph, x-paths | course closure | MIXED: course-side code partly live (x-dev `R/🎓️…/🧱️stack/🟦️.ts`, x-proctor `…/🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts` exists); staged manifests and adapter parts PENDING; x-paths not live | `K/result-x-*.md` status lines; `ls` of `🧾️product` VERIFIED-READ; the rest INFERRED from their status lines |

Order to apply (after show green and the pause; never during show): (1) integrate round 2 (carries course x-* and remaining peer sets; the course depends on it); (2) `python3 C/play-bzl-tools/apply.py` dry run then `--write` (anchored, refuses on drift); (3) `python3 C/demo-bzl-tools/apply.py` dry run then `--write`; (4) pg-chain set once it exists; (5) `python3 -I C/demo-bzl-tools/verify-independent.py <root>` (expect 0 FAIL); (6) `bun repo --tool=bazel run …` runs only after the coordinator's GO. VERIFIED-READ `C/result-play-bzl.md` §2, `C/result-demo-bzl.md` §6; order (1) INFERRED from `K/coordination.md` 17:47.

Important: play `E2` (`s-dev-composition:generate` inputs) changes the catalog's key. `C/result-play-bzl.md` §4.1 requires `C/play-bzl-tools/trace-generate.sh` before trusting a warm hit. VERIFIED-READ.

## 4. Known blockers, ranked

1. Show projects queued behind peer pid 15549 (exclusive). Blocks everything. VERIFIED-READ.
2. pg-chain not staged: play and demo warm re-activation re-runs 35 (+9) driver-owned `prepare-<lane>` tasks. VERIFIED-READ `C/result-play-bzl.md` §0.2, §3.
3. Course path length: course closure 250–262 UTF-16 units, AGENTS cap 256; fix `x-paths` (aliases `a/<10hex>/`, 260→231 on mac, 262→198 win) staged only and was reverted once after a live slip (15:52–16:19). VERIFIED-READ `K/audit-xplat.md` P0, `K/result-x-paths.md`. Windows also needs machine-wide `LongPathsEnabled` today (`K/audit-xplat.md` P0). Not a blocker on this mac once x-paths is live.
4. Course proctor built inside the test run: live default is `build` (in-process `bun repo cargo` nested inside a Bazel action, risk R5). Nested invocation blocking on the Bazel server lock: unverified. VERIFIED-READ `K/result-x-proctor.md` §5; INFERRED for the lock (`C/audit-e2e-readiness-1020.md` R5).
5. Play stdio component: nine `stdio-*` lanes depend on `stdio`, memory says its wasm cannot link (1,000,000-function ceiling). If it fails, its lanes fail (`C/result-play-bzl.md` §4.4). Unverified in this tree. Use `--keep_going`.
6. Serve teardown under Bazel: `run-e2e.sh` kills the process group (`run-e2e.sh:82-96`); under `bazel run` the server can outlive it. Check `lsof -nP -iTCP:6033 -sTCP:LISTEN` (play) and `:6029` (demo) after each run. R3, unverified.
7. `workspace:deps-browsers` is `cache:false` with no `no-sandbox` (R4). Browsers are present, so only a cache wipe makes this matter. Unverified under Bazel.
8. Demonstrator release `build` downloads OSM/OpenFreeMap tiles at build time (`C/result-demo-bzl.md` §0.5), uncached; not on the e2e path.
9. Course test is `cache:false` and re-derives the selected graph on each call: "course test 429 s" (`K/coordination.md`, x-graph entry). x-graph staged, not applied. Measured by peer, not me.
10. `bun` local 1.3.14 vs `packageManager` `bun@1.4.2` (`R/package.json:146`). VERIFIED-READ; the readiness audit did not re-check it.
11. Cosmetic: course output lands in `C/🗑️generated/nx/e2e/` even under Bazel (`C/audit-e2e-readiness-1020.md` R2).
12. Stale items: readiness audit B1 cites `run-e2e.sh:207`, which no longer exports release (fixed, see §0.5); R7 (`.vscode` AGENTS conflict) is stale because the user removed the launch.json rule from AGENTS on 10-20 01:24 (`C/coordination.md` 06:00). VERIFIED-READ.

No blockers found for: the wrapper flags, port availability (6033, 6029, 6061, 6063, 8791, 8793 all free, VERIFIED-READ `/dev/tcp` probe), disk (238 GB free, VERIFIED-READ), node (v24.15.0 at `/opt/homebrew/bin/node`, VERIFIED-READ), Playwright CLI (`R/node_modules/playwright/cli.js`, VERIFIED-READ).

## 5. External downloads

- Playwright chromium-1248, chromium_headless_shell-1248, ffmpeg-1013: present with `INSTALLATION_COMPLETE` in `R/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/`. VERIFIED-READ. The approved download is therefore not needed now (only after a cache wipe: `--install-browsers`).
- Bazel 8.4.2 binary: `R/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel` (58 MB) present. VERIFIED-READ. `.bazelversion` = 8.4.2 (`R/.bazelversion`).
- Tectonic 0.16.9 (aarch64-apple-darwin) and two TeX bundle directories under `R/.🧬semio/🦑️repo/⚡️cache/tools/tectonic/`. VERIFIED-READ dirs. That the bundle hash matches the pinned one (`C/result-demo-bzl.md` §4): INFERRED.
- Binaryen (wasm-opt): `R/.🧬semio/🦑️repo/⚡️cache/tools/binaryen` present. VERIFIED-READ dir; INFERRED usable.
- Cargo registry crates: a cold proctor and component build fetches locked crates over the network. INFERRED.
- Demonstrator release `build` tiles (OSM/OpenFreeMap): network, not on the e2e path. VERIFIED-READ `C/result-demo-bzl.md` §0.5.
- Docker: not needed for play, demo or course test-e2e (course docker steps are not on the path). VERIFIED-READ `C/audit-e2e-readiness-1020.md` §0 and §2c.
- Host tools needed: cargo `1.99.0-nightly` (course `.cargo/config.toml`), node v24.15.0 (course gate `spawn("node")`). VERIFIED-READ `/Users/ueli/.cargo/bin/cargo`, `/opt/homebrew/bin/node`; cargo nightly version INFERRED from the audit.

## 6. Estimated cold durations (no run done)

| phase | estimate | basis |
|---|---|---|
| play `activate-prototype` (68 components, 35 lanes, wasm) | about 3 h cold | estimate, `C/result-play-bzl.md` §3; memory cited by `C/audit-e2e-readiness-1020.md` §4 |
| play `serve` boot | 1–3 min cold, 30–60 s warm | estimate |
| play `test-e2e` (11 tests) | 15–30 min each run (`cache:false`) | estimate |
| demonstrator `activate-prototype` (9 + 4 wasm + 3 lanes) | about 41 min (measured 2026-09-16 by memory); about 11 min warm (before pg-chain) | `C/audit-e2e-readiness-1020.md` §4 |
| demonstrator `test-e2e` (7 blocks, GPU) | 10–20 min each run | estimate |
| demonstrator report `build-kompaktbericht` + `build` | not measured; Tectonic offline | INFERRED |
| course proctor cargo build (teaching-proctor) | 10–25 min cold, 1–3 min warm | estimate, `C/audit-e2e-readiness-1020.md` §4 |
| course `test-e2e --topology dev` (14 specs) | 20–40 min each run (`cache:false`) | estimate |
| course `--topology rehearsal` | 35–60 min cold | estimate |

Warm: play and demo activation approx. minutes after pg-chain (`C/result-play-bzl.md` §3: "tens of seconds" of Bazel overhead, plus two probes); without pg-chain the 35 (+9) prepares still re-run. INFERRED. Warm e2e for all three products is never a cache hit (`cache:false`). VERIFIED-READ manifests.

## 7. How to verify cache hits on the warm run

1. Before and after each run: `bun repo --tool=bazel cache status` (activity ledger: builds, actions, executed, cacheHits). VERIFIED-READ `C/hub-bzl-tools/prove.py` (spawns = delta of this ledger) and `K/x-keys-tools/run-twice.sh` (writes `cache-status-{0,1,2}.txt` and `verdict.txt`). Live output format not checked.
2. Bazel summary line in the run log: `1 process: 1 internal` (no spawn) means 100 % action-cache hit; `N processes: … disk cache hit` means a restore. VERIFIED-READ `C/result-bzl-real.md` lines 28–34 (measured in scratch).
3. Play: `C/play-bzl-tools/cold-run.sh warm` expects 0 executed except `deps-wasm-opt` and `generator-inputs` (after pg-chain). `cold-run.sh probe comment|code` edits one component, then only that lane's chain runs, and a revert is a disk hit. VERIFIED-READ `cold-run.sh:9-10` and `C/result-play-bzl.md` §3.
4. Demo: `C/demo-bzl-tools/cold-run.sh warm` (only the two probes), `probe comment|code`, `probe brand` (no Rust/wasm action), `report warm`, `deck` twice (second is a hit). VERIFIED-READ `C/demo-bzl-tools/cold-run.sh:4-11`.
5. After-clean: `bazel clean` / `cache clear` then rerun: expect disk-cache hits for all actions. `cache clear` also removes the disk cache (no keep-actions scope yet). VERIFIED-READ `C/result-bzl-lazy.md:77`, `C/hub-bzl-tools/prove.py` `--after-clean`.
6. Dry plan: `bun repo --tool=bazel run <target> --dry-run` lists planned tasks, no execution. VERIFIED-READ `C/hub-bzl-tools/prove.py` use, `C/result-hub-bzl.md` proof plan.
7. Serve teardown and port check: `lsof -nP -iTCP:6033 -sTCP:LISTEN` (play), `:6029` (demo), `:6061` (course). INFERRED from `run-e2e.sh:96`.
8. Demo report output slot: `.cache/b/p/m/print/<mode>/documents/report/<id>` (staged fix, `C/result-demo-bzl.md` §0.3); cross-check script `C/demo-bzl-tools/layout.ts`. VERIFIED-READ (script and report text present; slot not checked live, it is pending).

## 8. What was not verified (needs a real run)

Bazel behaviour of `serve`/`serve-e2e` teardown (R3), nested `bun repo cargo` under Bazel (R5), `workspace:deps-browsers` sandbox (R4), the stdio component link (play §4.4), course Windows (`K/audit-xplat.md`), any warm hit count, and every duration above except the demo activation figure (memory, dated 2026-09-16).
