# result-course-bzl (wave 1020d): architecture course on Bazel, takeover of the dead peer session

`C` = this ticket (`26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`), `P` = `26/10/20/ARCHITECTURE-COURSE-ON-BAZEL`, `S` = `🎓️teaching/🏛️architecture/📚️course`, `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.
Status 17:55: PRE-GO work finished (read-only on the repo; scratch only under `/private/tmp/course-bzl`). Nothing applied, no `bun repo`, no bazel/cargo run. Waiting for GO.

## 1. What I found

- The peer session (8 executors x-*, 5 audits, 1 reviewer) finished all eight sets; `x-graph` last (17:19). `x-integrate` died at 17:35 without `result-x-integrate.md`, but its tool `P/x-integrate-tools/apply-all.py` (ordered series on an overlay, all-or-nothing live write, idempotent) is complete. I ran it read-only on the live tree: every step resolves, no anchor refusal (`C/🗑️generated/course-bzl/apply-all-check-1.txt`): peer-ops already applied (the uncached-owners rows are live), the other 9 steps pending, 50 files.
- Defect in that series: step `x-graph` looks for `x-graph-tools/staged/apply.py` (does not exist, the script is `x-graph-tools/apply.py`), so it silently skips. Fixed in my wrapper (below).
- Codex peer (`26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY`, files `2026-10-20-current-course-checkpoint.md`, `native-transition.md`, `native-build-regression.md`, `shell-typegen-admission.md`) is working on the same course from the native Go `repo` frontend and a Linux builder. Overlaps:
  1. It repaired the shell `typegen` own-output refusal (owner-discovery per target, admission schema `taskSourceDemand`, graph fixture, source-demand test, shell manifest `sourceInputObservation: freshness`). Its frozen hashes: shell manifest 6d8a92a9..., owner-discovery 88976061..., admission schema e21558fc..., graph fixture 03c6154...; regression session 82722 (`workspace:test-workspace-build --configuration=release --skip-cache`) is live and bound to them. Do not touch those five files and do not apply `own-output` remainder or `x-graph` until 82722 is terminal.
  2. It will edit `.github/workflows/architecture-course.yml` (native provisioning) and the offline stack bundle (`pull_policy: never`, `🚀️deploy/🎁️stack-bundle`). The course manifest lists the workflow and `🚀️deploy/**` as inputs, so both invalidate `test`/`check`; harmless. x-site edits `🚀️deploy/🌐️site`, `⚖️readiness`, not the bundle.
  3. It holds the graph-facts lease (pid 15549, 85 min) and queues our `show projects`. Never kill it.
  4. Its goal needs a Linux x64 course image; `docker-image-*` / `docker-stack-*` are Linux-only by design. On macOS they cannot pass on Bazel (gap G3).

## 2. State per staged set (`P/x-<name>-tools`)

| set | live now | staged / pending | needs before the course runs | live proof still missing |
| --- | --- | --- | --- | --- |
| x-ingress | main set applied by peer 16:28 (24 ops); generation-receipt one-token fix live | follow-up `x-ingress-tools/followup/apply.py` 13 ops pending, anchors OK (`--check`) | apply follow-up (shared-only miss path, no exclusive `.cache/b/d` walk on a warm run) | 2x `--dry-run`, second must not take the exclusive lease |
| x-graph | nothing | `x-graph-tools/apply.py` 21 ops, anchors OK today (touches `B/🕸️graph/🟦️.ts`, `🚪️admission/🟦️.ts`, `🧬️schema/🚪️admission.json`, same schema file Codex froze) | course test `--dry-run` re-derives 258 owner observations (367 CPU-s) per run; apply AFTER Codex session 82722 ends; invalidates every graph receipt once | `measure.sh` pass 2: zero `build:selected-declaration` |
| x-inputs | 6 test files made hermetic (no `SEMIO_TICKET_DIR`) | 12 files: declared closure (modules/TypeScript projects), scanner in `🧭️script-inputs`, project schema `metadata.semio.closure`, manifest rows, two law suites | needs inference change together with the manifest (inert otherwise); touches `script-inputs/🟨️.mjs` that Codex edited: apply after 82722 | gap files-read minus declared = 0 on real run; `test` stays a hit on an unrelated `🖱️ui` edit |
| x-site | budget-derived typecheck timeout, sorted `siteFiles`, tsbuildinfo removed, tests | 12 files: cached `build` (store receipt output), `publish` on `build`, `deploy-check` graph tasks, readiness steps, README | peers' manifest rows first (peer-ops already live), x-inputs (default loses `🖱️ui/**`), reconcile R2 (site tsconfig closure) | second `build` = hit, no `Bundling` line; `PROCTOR_URL` switch = one rebuild; `publish:release` in Bazel |
| x-release | EN/DE conflict message in `B/🧭️invocation`, course mode gate + tests | 6 files: adapter build-mode axis (`request.authority`), `B/📜️script.ts` dry-run plan, README, `📥️mode` suite | adapter apply; then dry-run `-c release` of `test-e2e`, `docker-image-*`, `deploy-check` (R2 mode-fixed risk) | real `publish:release` (gate passes inside the Bazel action) |
| x-paths | nothing (incident reverted 16:19) | 12 files: `a/<10hex>/` aliases, per-OS path budget (mac 260 -> 231, Windows 262 -> 198), readiness no longer demands LongPathsEnabled | adapter + readiness apply, after x-release (same adapter) | real Bazel run with budget table printed; Windows/Linux never run |
| x-proctor | product module, `acquireProctor` in dev/check/e2e/image/capacity, tests (26 pass) | 3 files: proctor + course manifests (`proctorRust` input, `dist/build/<mode>`, `dependsOn: build`, receipt input in `check`), default flip to `product` | manifests apply; then ONE cold proctor build | cold/warm seconds, dep-info coverage law against the real receipt, `cleanup` step afterwards |
| x-dev | ready line after probe (`awaitAnnounced`), env contract, README, 11 tests | 3 files: `stack=beside` parameter on `dev-site` and `proctor:dev` | manifest apply | `drive.ts` for `dev-site`, `dev`, `proctor:dev`: ready ordering, ambient env (`TEACHING_ARCHITECTURE_COURSE_PORT=6063`), cancel frees 6061/8791 |
| x-keys | nothing (no defect confirmed; 4 key risks refuted offline) | `📥️keys` regression suite (tests only) | none | `run-twice.sh` on `test` and `typecheck`: runner cwd, six dependency snapshots |
| reconcile / cleanup | n/a | in the series: R1 (`check` keys on the receipt, not Rust sources), R2 (build closure over `tsconfig.site.json`); cleanup deletes `buildProctor` + `PROCTOR_ACQUISITION_*` | cleanup only after the cold proctor build + e2e passed | n/a |

## 3. Course-specific residue I found and staged (G1)

`test-e2e --proctor <relative>` resolved against the package directory the build tool runs in (audit-proctor P2-7). Staged `C/course-bzl-tools/staged/course_step.py` (anchored, idempotent, `--check`): pure `endToEndProctorPath` in `S/🎭️e2e/🟦️.ts`, run uses it, test in `S/🧪️tests/🧱️local-stack/🟦️.ts`, language-agnostic fixture + schema `S/🧫️fixtures/🎭️e2e-proctor/` (Python `posixpath`/`ntpath` oracle, Ajv, Node `path.posix`/`path.win32`). Offline check: function against the fixture 0 failures; byte-copy mirror apply + re-check idempotent.

Wrapper `C/course-bzl-tools/staged/apply-course.py` = x-integrate series + `course-bzl` (+ corrected `x-graph` only with `--with-graph`). Mirror proof (`mirror-check.py /private/tmp/course-bzl [--with-graph]`): live apply writes 54 files (62 with graph), second `--check` finds 0 differences, rc 0 both.

## 4. READY section for integrate2 (ROUND 2 ordering)

Apply with the wrapper, not with `P/x-integrate-tools/apply-all.py` (it skips x-graph):

```
python3 -I C/course-bzl-tools/staged/apply-course.py --check --with-cleanup      # read only; expect 10 pending, 54 files, exit 0
python3 -I C/course-bzl-tools/staged/apply-course.py --live  --with-cleanup      # all or nothing (replaces files by rename: no hard links)
```
Drop `--with-cleanup` for the first apply if the cold proctor build has not run yet (cleanup deletes the transitional switch); add it afterwards by running the wrapper again with `--only cleanup --with-cleanup`.

Must apply BEFORE the course runs, in this order (the wrapper keeps it): peer-ops (live), x-inputs, x-site, x-dev, x-release, x-paths, x-proctor, x-keys, reconcile, course-bzl. Plus, outside the series:
1. x-ingress follow-up: `python3 -I P/x-ingress-tools/followup/apply.py --check`, then `--live` (13 ops; independent of everything else).
2. Generic dependencies already named by the coordinator for ROUND 2: own-output remainder is NOT needed for the course (Codex's repair is live; applying both would collide on shell `typegen` freshness: do not apply `own-output-tools/staged/apply.sh` before diffing against the live shell manifest), `bzl-lazy`/`cache-fix`/`uncached-owners` already applied in round 1.
3. `x-graph` (`--with-graph`): optional performance set; apply last and only after Codex session 82722 / 63416 are terminal. Without it a cold `--dry-run` of the course closure re-derives 162 owners (about 7 min CPU) once per process.
4. Gate for x-inputs, x-release, x-paths, x-graph: they change inference/adapter files that are part of the graph authority hash; every existing graph receipt is invalidated once and Codex's running native sessions (82722, 63416, 86151, 96264) are bound to the old hashes. Ask the coordinator to confirm those sessions are terminal first.
5. Order against other ROUND 2 sets: none of `input-precision`, `uncached-owners`, `sync-incr`, `mem-gov`, `hub-bzl`, `demo-bzl`, `play-bzl`, `bzl-rbe`, `orphan-child` reference the course/proctor manifests (grep of their tools folders), so they can go before or after. `docs-build` edits the README: apply after x-release/x-dev README edits (soft edits report drift instead of failing). `bzl-incr` has course scenarios (vite-comment, test-body, lockfile-typecheck on `@teaching/architecture-course:build|test|typecheck`): its harness is the right incremental regression once the course is green.
6. After the apply: `sh P/x-integrate-tools/check-all.sh live` (semantic checks, type diagnostics, tests, at most 2 files at a time).
7. Preconditions for the cold run: `show projects` green, `tools setup bazel` green (chain14), exclusive `.cache/b/d` lease free (Codex pid 15549 holds graph-facts), disk >= 60 GB free (239 GB now), load < 12.

## 5. Targets and commands after GO

Closure (all through `bun repo --tool=bazel run <label>`): `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate`, `@semio-tech/framework-rs:generate`, `@semio-tech/ui-contract-rs:generate`, `@semio-tech/ui-rs:generate`, `@semio-tech/framework-actor-rs:typegen` (deps, pulled automatically), `@teaching/proctor:build|test|check|dev`, `@teaching/architecture-course:typecheck|test|build|check|publish|test-e2e|deploy-check|dev-site|dev|preflight`. Cached after the series: proctor `build` (product + receipt), `test`, `check`; course `typecheck`, `test`, `build` (receipt), `check` (keys on the receipt). Uncached by contract: `publish` (policy name `publish`), `test-e2e`, `deploy-check`, `dev*`, `docker-*`, `preflight`, `ci-browser-prepare`.

Driver: `C/course-bzl-tools/course-run.sh` (refuses without `COURSE_BZL_GO=1`; foreground; logs `C/🗑️generated/course-bzl/run/<phase>-<n>-<label>.log`, one TSV per phase with rc, seconds and the Bazel process line):
```
COURSE_BZL_GO=1 sh course-run.sh preflight   # --dry-run of 12 targets (plan only)
COURSE_BZL_GO=1 sh course-run.sh cold        # proctor:build -> typecheck -> test -- quick -> build -> check -> publish -c release -> proctor:test; cache status before/after
COURSE_BZL_GO=1 sh course-run.sh warm        # same list; each cacheable target must report only cache hits and finish in seconds
COURSE_BZL_GO=1 sh course-run.sh edit        # reversible comment edit of S/🏗️builder/⏱️budget/🟦️.ts: typecheck/test/build/check/publish miss, proctor build/test hit (PROCTOR_EDIT=<framework .rs> also proves the proctor miss)
COURSE_BZL_GO=1 sh course-run.sh e2e         # test-e2e --configuration=release --topology both (Chromium downloads once into the repo tool cache: network)
COURSE_BZL_GO=1 sh course-run.sh dev         # x-dev drive.ts: proctor:dev 8791, dev-site 6061, dev 6061+8791: ready order, cancel, ports freed
```
Equivalent single commands: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`, `bun repo --tool=bazel run @teaching/architecture-course:test -- quick`, `bun repo --tool=bazel run @teaching/architecture-course:test-e2e --configuration=release`. Cache evidence: Bazel prints `N processes: M disk cache hit` per run, `bun repo --tool=bazel cache status` gives the counters; for key stability also `P/x-keys-tools/run-twice.sh <label>`. `--skip-cache` forces a miss. Success criteria: cold green; warm = hits only, no Vite `Bundling`, no cargo compile line; edit = only the affected tasks miss; e2e = both topologies pass; cancel (Ctrl+C / SIGINT) frees the ports.

## 6. Remaining course-specific gaps (not fixed, with owner)

- G2 Chromium: first `test-e2e` downloads Chromium (`playwright install`) into the repo tool cache; no pinned/offline source, none found in `.cache/b/t/m` today. Needs network once; `ci-browser-prepare` is apt-only (Linux). Owner: course.
- G3 `docker-image-*`, `docker-stack-*` are Linux x64 only (runtime throw, `🐳️docker-image`): on macOS/Windows they should be reported "not applicable" instead of failing in an action; the Linux course image is the Codex peer's builder work. Not changed.
- G4 `docker-image-publish` has no `dependsOn docker-image-build` (ordering by the user; safe, check exists). Not changed (Codex edits the same deploy area).
- G5 `test-e2e` spawns `node` (port probe, Playwright cli): Node must be on PATH inside the Bazel action; `x-keys` showed the action env carries the caller PATH, not run live.
- G6 `proctor:dev` / course `dev` rebuild the proctor on Rust edits through the product module (`{rebuild:true}`), one cargo compile outside Bazel's action graph by design (continuous, uncached).
- G7 Windows: nothing run (path budget, junction link, `.cmd` credential helper only matter with a remote cache). Linux/devcontainer: `processwrapper` sandbox for unprivileged containers is still staged-only (bzl-xplat).
- G8 `deploy-check` loses early typecheck-before-build ordering (graph tasks run in parallel before the gate script); intended trade.
- G9 two pre-existing red tests in `🧱️local-stack` (Vite proxy upgrade under Bun, "proctor goes away while it proxies a socket"): unrelated to the sets, baseline-compare will show them as not new.

## 7. Files

Created (all under `C/course-bzl-tools/`, plus this file): `staged/apply-course.py`, `staged/course_step.py`, `staged/files/e2e-proctor.fixture.json`, `staged/files/e2e-proctor.schema.json`, `mirror-check.py`, `course-run.sh`. Scratch: `C/🗑️generated/course-bzl/` (`apply-all-check-1.txt`, `probe/`), `/private/tmp/course-bzl` (mirror, deletable). Backend edits: none.

## 8. READY2 (21:50, stage only, rebased on the live tree)

Codex gate: checkpoint 19:36 (+ `next-capture-critical-source-delta.md` 21:40): sessions 82722, 63416, 86151 are terminal 1 (before collection), 98366 exited 1 at 19:30, 38213 cancelled, "no owned command is live"; `ps` shows none of 82722/63416/86151/96264/15549. The freeze on graph/inference files is lifted. Codex now edits `⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts`, `🧪️tests/🟦️.ts`, `🔗️activation/*` (new), `🦀️rustup/*`, `🔏️authorizers/*`, `⚙️operations`, `🌡️host-capacity`: NO file overlap with course-B (we touch only `🛠️tools/🏗️bazel/{🧬️schema/🔣️.json,🩺️readiness/*}`; the owner file mentions neither bazel nor readiness).

Rebase result (`apply-course.py --check --with-cleanup`, live tree): applied already = peer-ops, x-dev, x-keys, course-bzl (course-A); pending = x-inputs, x-site, x-release, x-paths, x-proctor, reconcile, cleanup (46 files); x-graph optional.
- x-paths was REFUSED on the live tree: bzl-hermetic added `trees:[...]` after `groups:tree.groups,` in the Bazel adapter. Anchor rebased (`C/course-bzl-tools/rebased/x-paths/edits.py`, one edit).
- x-release anchor `run` breaks once conc-bazel is applied (`return withBazelStorageV1(` becomes `return withBazelSlotV1(root,bazelSelectionV1(graph,request),...`). Anchor rebased to end at `checkResourceLeaseControl(phaseControl);return ` (`C/course-bzl-tools/rebased/x-release/mode_patch.py`); the authority binding runs before the slot is chosen, so the slot key sees the mode-bound request.
- `apply-course.py` now uses the rebased copies; both roots resolve and are idempotent: live tree and live + conc-bazel (group all) overlay (`compat-check.py`, `try-roots.sh`; mirror apply writes 46 files, re-check 0 differences).
- ORDER RULE for integrate2: several staged sets replace whole files guarded by a base hash (conc-leases, mem-gov, cache-gc, conc-cargo copy `B/📜️script.ts` or the adapter script as staged files). An x-release edit applied BEFORE them makes them refuse ("changed since staging"). So apply the POST-SHOW BATCH (mem-gov, conc-nx, conc-deps, conc-bazel, conc-leases, conc-cargo, cache-gc, conc-graph) first, then re-run `apply-course.py --check`, then course-B. Only conc-bazel (adapter) and conc-graph/conc-nx accept `--root`, so the combined order was proven for conc-bazel only; the other overlaps are `B/📜️script.ts` where x-release edits one 38-character expression (`adapter.planBazelTasksV1(graph,request)`).
- x-graph: complementary to conc-graph (its own note: either order, anchors disjoint); keep it out of course-B by default (`--with-graph` only if conc-graph's closure measure is still slow). Round-2 group 21 should drop `--with-graph`.
- Still gated on tools that need a booted CLI: tsc of the patched adapter (x-integrate `check-all.sh live` after apply), the `📥️mode`/`📥️paths`/`📥️keys` suites, and real runs.

`course-run.sh` for the new world (still refuses without `COURSE_BZL_GO=1`): `SEMIO_CARGO_LANE=course` default (private conc-cargo lane; no `repo:compiler-writer` wait), `show projects --graph-policy=current` in preflight (task commands always plan fresh; `current` applies to `graph`/`show` only), mem-gov and lease evidence per phase (`capacity status`, `leases status`, `cache status` before/after into `capacity-*`, `leases-*`, `cache-status-*`), slot/wait lines (`session slot`, `server slots in use`, `Waiting for access`) in a fifth TSV column, new phase `parallel` (typecheck + proctor:test + check at once: expect slots s0..s2, no wait lines), new phase `lanes` (proctor:build `--skip-cache` in lane-a and lane-b at once: second lane compiles from the shared unit cache), `dev` unchanged (service slots `d0..` keep long sessions off the build slots once conc-bazel group 2 is in). `SEMIO_HOST_CAPACITY=off` only as the documented escape hatch.

## 9. Round-3 reds fixed (22:55, applied live, test files + one verifier line; `nice -n 10 bun test ./S/🧪️tests/🧪️deploy/🟦️.ts`: 37 pass, 1 fail, 5 s)

- Coverage law (141 uncovered `🖱️ui/**` files): the law only looked at the glob inputs and the router closure, but reconcile R2 declares the site's source ownership as `metadata.semio.closure.typescript: tsconfig.site.json`, which the inference expands with the TypeScript program. `S/🧪️tests/🧪️deploy/🕸️site-graph/🟦️.ts` now computes that program with the TypeScript API (independent of the inference), asserts the manifest declares the fixture's `closureTypescript` (new fixture field + schema), and counts program files as covered. No exemption list; the law still names any value-imported file outside inputs, router closure and the declared program.
- Image identities (`timestamp-pax`): the bundle verifier `S/🚀️deploy/🎁️stack-bundle/🟦️.ts` required a PAX `path` to equal the header name, but tar-stream (and Docker for long names) writes the real name only in the PAX record. Effective entry name is now `pax.path ?? header name`; a PAX `size` must still equal the body, and `path-pax` (override to `foreign.json`) is still refused because `manifest.json` then is missing (fixture rows and the tar-stream/Ajv oracle agree). File untouched by Codex since 19:47.
- Still red, not mine: `🩺️preflight` alignment test counts 0 workflow steps containing `bun repo preflight` (expects 3): Codex's workflow change (`repo preflight`, native) landed without the matching test update; it is part of their CI/offline RED/GREEN, left alone.

## 10. After GO (log)

(pending: no GO for the course run yet)
