# result-ciwf (lane ciwf, wave 1027b): why `build-system.yml` runs fail in 0 s

`C` = this ticket folder. Read-only `gh`, no remote write, nothing live edited, no mirror needed (no product code touched).

## FOR coordinator
- Nothing to apply: the root cause is already fixed in the live tree (lane ci-g, 10-26 16:55, `C/result-ci-g.md` line 8, `coordination.md` 1192). The fix and its tracked law are only in the git INDEX/working tree (`M  .github/workflows/build-system.yml`, `A  .../🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts`), never committed, so the five failing pushes (commits 701, 702, 703) all carried the broken file. It must be committed and pushed.
- Verify (no product run): `sh C/ciwf-tools/run-law.sh /Users/ueli/Documents/semio` (3 pass; 0 findings from the TS law, the regex model and the schema oracle). The tracked law `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🧪️tests/🔀️workflow/🟦️.ts` is 7 pass 0 fail on live.
- After the push the workflow stops being named by its path: `gh workflow list` shows `build system`, and the instant failures end. CAUTION: with the file valid, a push to the default branch `⛳wip` starts NO run at all (`on.push.branches: [main]`, the default branch is `⛳wip`); only a pull request or a manual `workflow_dispatch` runs it. See open decision D1.

## Root cause (certain)
`jobs.bench.env` of the pushed file (the 430-line version, identical in all five failing commits `8b89a8878`, `c31b126a1`, `7b2365ac0`, `6b24f2780`, `ec6b0673e`) was
```yaml
    env:
      BENCH_BASELINE: ${{ runner.temp }}/bench-baseline.json   (also BENCH_REPORT, BENCH_MARKDOWN)
```
The `runner` context is not available in a job-level `env` (`jobs.<id>.env` knows github, needs, strategy, matrix, vars, secrets, inputs; docs: contexts, "Context availability"). GitHub rejects the WHOLE file ("Unrecognized named-value: 'runner'"), before any filter, matrix or job exists.

Evidence (all read-only):
1. The five runs (ids 39873920242, 40227563985, 40244799446, 40301257402, 40301732436) are the only runs of workflow id 393477301; each took 0 s, `jobs` total 0, `gh run view`: "likely failed because of a workflow file issue".
2. The workflow's `name` is the PATH `.github/workflows/build-system.yml`, not the declared `name: build system`: GitHub falls back to the path when it cannot parse the file. It was created at the first push of the file (2026-10-22 23:45Z).
3. The runs exist although the file says `push: branches: [main]` and every push was to `⛳wip`: only a file whose trigger section cannot be evaluated produces a run for every push. The sibling files `dashboard.yml` and `architecture-course.yml` carry the same filter, are valid, and have no run and no registered workflow.
4. The pushed file is red under the local law (3 findings, exactly these three env lines); the live file is green (the first step of `bench` now writes the names into `$GITHUB_ENV`).
5. The annotation text is not exposed by the API (`check-suites/<id>/check-runs` total 0); only the Actions UI shows the literal message. Not needed: two independent readings of the table agree.
Why `C/result-linuxdc.md` missed it: it validated against the JSON schemas, which cannot see context availability (re-proved: the schemastore workflow schema reports 0 errors on the red file).

## Other checks (all clean)
- Every `uses:` pin of the live files exists on GitHub (`gh api repos/<o>/<r>/commits/<sha>`): actions/cache, checkout, deploy-pages, setup-node, upload-artifact, upload-pages-artifact, devcontainers/ci, docker/login-action.
- Every `./.github/actions/*` reference exists (cache-proof, devcontainer-repo, devcontainer-suites, native-repo, repo-launcher); composite run steps have `shell`; no duplicate YAML keys; `needs` acyclic and defined; step ids unique; `steps.*`, `needs.*.outputs.*`, `matrix.*`, `inputs.*` references resolve; status functions and `hashFiles` only where allowed; expression syntax parses (own recursive-descent parser).
- Other instantly failing workflows: none. The only other failures in `gh run list` are `Dependabot Updates` runs (dynamic workflow, a dependabot updater error in uv/cargo/go/npm, 1 min, not a workflow file issue; the updater log needs write access).

## Law and tools (`C/ciwf-tools/`)
- `law/check.ts`: GitHub's accept-checks as a Bun law: expression parser, context-availability table per location (workflow/job/step/composite), reference resolution, local `uses` existence, duplicate keys. `bun law/check.ts <root>` (exit 1 on findings).
- `law/check.test.ts`: red on `fixtures/pushed-build-system.red.yml` (the rejected file, exactly the 3 `runner` findings), green on live, 6 negative controls bite (unknown `needs`, `env` in `runs-on`, missing local action, `secrets` in a job `if`, unknown matrix key, broken expression syntax) plus duplicate-key and parser cases.
- `law/ctxmodel.py` (independent regex reading of the availability table, PyYAML) and `law/oracle.py` (PyYAML strict loader + jsonschema against the vendored schemastore schemas in `fixtures/`; exit 3 = modules missing, never a silent pass). actionlint is not installed (not installed on purpose).
- `run-law.sh [root]` runs all of it. Cross-platform: Bun for the law, `sh` wrapper only convenience (the Python oracles are optional second opinions).
- `🗑️generated/ciwf` removed.

## Open decisions (not staged, they change CI policy)
- D1: trigger on the default branch. `push: branches: [main]` in `build-system.yml`, `dashboard.yml` (and `DISPATCH_BASE origin/main`) while the repository default is `⛳wip`: pushes to `⛳wip` never run CI. One-line options: `branches: [main, ⛳wip]` in both workflows, or keep PR + dispatch only. The `default_branch` expressions (cache write, bench baseline) already follow the repository default, so they would start working on `⛳wip` pushes.
- D2: GitHub announced `ubuntu-latest` migrates to Ubuntu 26 from 2026-10-19 (annotation on the Dependabot runs). The Linux host jobs (smoke, storage, host, suites, affected, parity, bench) would move off the 24.04 the devcontainer and `result-linuxdc.md` checks assume; pin `ubuntu-24.04` there if that is unwanted (devcontainer jobs build their own image, unaffected).
- D3 (cosmetic, from linuxdc L5): `devcontainer-suites` and `suites` use `timeout-minutes: 350` while `SEMIO_BUILD_BUDGET_MS=21600000` (360 min); 360 is the hosted maximum.
