# CI and Environment Entry Points

## Current Source Inspection

Repository test commands already enter Nx. Remaining CI/environment integration is incomplete:

- `.github/workflows/repo-test.yml` repeats a full root `bun install --frozen-lockfile` before Nx in six jobs, and gives dependency/native state an invocation-specific agent identity. Its baseline environment currently uses the triggering SHA rather than an independently resolved last successful validation base.
- `playwright.yml` installs root dependencies before invoking broad `workspace:setup`; the eventual browser and Storybook consumers still need explicit preparation scopes.
- `play-sites.yml` manually lists site owners and physical dist locations and verifies output files with shell commands outside Nx. Its presentation path still names the removed implementation directory. The comment about invoking the old raw OS Dev build path is also stale.
- `gh-pages.yml` installs dependencies and builds through Nx, while artifact promotion/deployment remains owned by GitHub actions. Its compose docs owner and physical upload path require validation against the current external/nested workspace boundary.
- Bun is configured as `latest` in three workflows and is not explicitly selected in the repository-test workflow. Toolchain identity must come from the repository's pinned policy.
- `.devcontainer/devcontainer.json` still routes create/start/attach through shell files and supplies a separately versioned global Nx feature. The minimal runtime/Nx acquisition boundary and application dependency synchronization need distinct commands.

No hosted CI, deployment, container creation, signing or publication was run. These observations are source findings, not an executed cross-platform qualification. A minimal Nx bootstrap must be implemented before replacing installation lines; simply renaming the full install step would leave the same boundary violation.

## 2026-09-08 continued qualification

The current root target is `workspace:deps-javascript`, not the earlier `deps-bun` name. The setup target depends on nine explicit dependency targets. The pinned Bun policy is `packageManager: bun@1.3.14`. The setup-bun action documents both automatic packageManager selection and explicit `bun-version-file: package.json` ([official source](https://github.com/oven-sh/setup-bun)). The workflows with no version already select this pin under the current action behavior; the three `latest` overrides do not. Nx documents last-successful relevant CI baselines ([official source](https://nx.dev/docs/features/ci-features/affected)); no affected-baseline implementation has been validated yet.

At approximately 21:15 local time, generated logs and fixtures disappeared during running ticket validations. Free disk changed from about 12 GiB to 196 GiB. This task performed no such cleanup. The Hub admin retry passed, but the native build exited 1 after 7m05s; the native failure log was removed before inspection. No native Hub success is claimed. Print/repo/editor processes were still live when inspected. This invalidates assumptions that their generated fixtures and logs survived; final statuses must be qualified accordingly.

## JavaScript dependency leaf qualification

RED: the language-neutral dependency-bootstrap contract rejected `workspace:deps-javascript` because it imported the root application script. GREEN: the target now calls the small `🚀️bootstrap/📦️dependencies/📜️script.ts sync` executor. It remains uncached with no deliverable outputs and refuses installer overrides. The root-script JavaScript install branch was removed. The installer runs asynchronously, reports progress, and forwards cancellation to its owned process tree. Cancellation branches have not yet received a native fixture proof. Existing dependency editor entry remains `bun nx run workspace:deps-javascript`.

The contract was executed through pinned native Nx with NODE_PATH supplying Nx from an external tooling installation. Its private application entry throws if imported. Native Bun first produced the expected installed local-package bytes; installed node_modules was removed twice, and both subsequent Nx target invocations reinstalled matching bytes without changing the lockfile or reporting a cache hit. Both native task executions succeeded. The fixture is schema-validated with jsonschema and the import boundary is checked with esbuild; workflow structure is parsed by YAML. All four workflow files now explicitly select `bun-version-file: package.json`; the fixture checks every setup-bun step. Shared node_modules was not installed or changed by this proof. These changes are a prerequisite for minimal tooling acquisition; that acquisition and replacing direct workflow installs are still pending.

## 21:46 local concurrent state

All four inspected workflow files disappeared during this turn, and `.github/workflows` is now empty. `git ls-files .github` contains no workflow files, so those earlier live files were not represented in the current index. No matching renamed workflow was found by a scoped filename search. This task did not delete them and will not recreate the obsolete Compose/deployment pipelines from memory. The earlier pin changes and first YAML proof describe historical working-tree files only. The later dependency fixture completed native synchronization but failed its hardcoded workflow-path check. Current CI configuration needs a fresh owned entry point after its report/output contract is settled.

Tooling RED: the fixture could not import the missing acquisition module. GREEN: a frozen, ignored-lifecycle-script Bun install acquired four exact direct tooling dependencies (Nx, @nx/js, @nxlv/python, TypeScript; 368 transitive packages) in private storage. It did not install an intentionally invalid application dependency. The public launcher then matched native Nx from its owned `.nx/installation` link, ran `workspace:deps-javascript` against a valid private application fixture, preserved its lockfile, and rejected installer overrides. The acquisition cache includes platform, architecture, Linux libc family, locked recipe and patch bytes; existing installations are immutable. Activation uses the existing cross-process SQLite lease and refuses foreign directories. Darwin behavior was executed; Windows junction replacement and Linux behavior remain unqualified. Shared application node_modules has not been installed or changed by these probes. Tooling version retention and installation-versus-consumer leases remain pending.

## Fresh checks and invalidated long run

The current existing-installation bootstrap proof, uncached dependency-sync proof and four cancellation-fallback vectors all printed PASS after the tooling changes. The cancellation probe initially failed because the extracted class contained import.meta; acquisition loading was moved to a module-level function and the mock filesystem seam was made explicit. The fixture now concerns dependency synchronization only; its stale references to removed workflows were deleted. CI remains unimplemented, not silently passed.

The older repo:test process (bootstrap PID 8068) was positively identified and sent SIGTERM after more than an hour. Its output file had been unlinked by another operation, its generated fixture set was removed, and newer source changes invalidated its loaded test harness before its remaining checks. It had printed passing native contracts and a 448-project inventory before log removal; this is not a complete repository-suite pass. A one-second owned-process sample showed active filesystem reads rather than an idle wait, but the sample did not identify a particular source-level hot path. Do not infer the exact cause of its duration from that sample. Registry editor generation exited 0, and generated launch.json contains all three os-hub build, os-hub-admin build and workspace:deps-javascript commands.

## Installer process-tree proof

A real Bun installer fixture spawned a child; both explicitly ignored SIGTERM and published readiness atomically. Cancelling the shared Bun runner triggered its five-second forced shutdown. Both PIDs disappeared from the native process table, verified using process.kill(pid, 0) and macOS ps. The probe printed PASS and exited 0. This is actual Darwin cancellation evidence, not Windows qualification.

The first repository-plugin fixture failed because it used a Bun file: dependency, which the current monorepo lock-graph parser deliberately rejects. The fixture has been corrected to a workspace:* dependency matching the repository dependency model; the graph parser has not been weakened. The plugin-level retry is pending.

## Repository-plugin cold setup qualified

The corrected workspace-dependency fixture passed the repository nx.json configuration, both external plugins, the repository project plugin, the test plugin and emoji project discovery. Public Nx synchronized the private application using the independent patched toolset; the frozen application lockfile was unchanged. The bootstrap additionally selects independent tooling for direct or batched setup/deps-javascript target selections, including configured target spelling. The activation lease was moved outside Nx workspace-data so Nx workspace-data reset cannot remove a live lease database. The active-generation link stays in Nx’s standard `.nx/installation` location. No root installation link was created by these private probes.

Remaining bootstrap limitations: native Windows and Linux qualification, runtime acquisition on machines without Node/Bun, safe retention of obsolete immutable tooling generations, and task-lifetime leases preventing application installs from racing consumers. CI baselines, platform lanes and workflow creation are still pending. No GitHub workflow was triggered or deployed.

## Extracted test registration and import repair

The cache contract tests were extracted by another worker from the caching script into `🧪️tests/⚡️cache-contracts/🟦️.ts` while this task was active. The three dependency/tooling/cancellation functions are now registered in that module. An esbuild import-graph check failed on twelve misbased imports after extraction. Relative sibling, lease, playground and bootstrap imports were corrected against their existing filesystem locations; the self-import was replaced with a direct call to testCommandInputs. The same esbuild check then exited 0 and printed PASS. The unrelated factory extraction was retained. A fresh full repo:test run has been started with terminal output plus a ticket log, so loss of a log pathname does not also erase the captured failure evidence.

The final standalone tooling proof after activation-lease relocation and batched setup selection exited 0, including the real repository-plugin fixture.

The first fresh full suite exited 1 in 821ms: the extracted test factory’s source parameter was shadowed by a local source binding, causing a temporal-dead-zone ReferenceError at the first schema-validator import. The factory metadata parameter and its references were renamed to testSource; the extraction and test logic were retained. No contract pass was claimed for that run.

## Full suite after factory repair

The fresh full suite ran 6m17s and exited 1. All native command, tooling bootstrap, dependency synchronization and real installer cancellation contracts passed; 448 projects were collected; editor/playground and lifecycle/compiler contracts also passed. The next materializer cancellation assertion failed because process.kill(childPid, 0) did not throw. An isolated native probe is investigating whether the recorded PID was valid and whether shutdown had actually completed. No full-suite pass is claimed.

## Materializer cancellation isolation and new suite failure

The unchanged materializer spawn implementation passed twelve isolated real-process cancellations, with valid nonzero PIDs and absent native ps results after each shutdown (about 2.0–2.2 seconds). Its full-suite readiness fixture previously published the PID file nonatomically and accepted Number(emptyString) as zero. The fixture now publishes via rename and asserts a positive integer PID before cancellation; this is a race correction, not proof that it caused the earlier failure. The next full suite exited 1 after 5m59s earlier in generator input coverage: schema-entity-catalog target inputs did not contain the current entity catalog path. Native contracts and the 448-project inventory passed again. The materializer section was not reached in that run. The schema file exists; current graph inputs need inspection before another full retry.

The attempted native schema-generator project inspection failed during graph discovery: a browser-bundle actor-import fixture project.json disappeared between discovery and loading; Nx also reported a missing external source node after that plugin error. This does not establish a schema-generator input defect. No source fixture was recreated and no shared Nx cache/daemon reset was performed. A later fresh graph attempt is pending.

## 2026-09-09 Baseline Selection Contract

A schema-first baseline selector was added under caching/ci/baseline. It accepts an explicit repository, workflow, run, target branch and checkout SHA. It filters completed successful push/manual/scheduled runs from that same workflow, branch and head repository, excludes the current run and PR/PR-target results, and checks candidate ancestry before selecting the latest workflow sequence. Missing successful ancestry selects full validation; it never invents a previous-commit or main-branch fallback. Explicit full validation bypasses history lookup.

RED: the language-neutral fixture could not import the missing selector. GREEN: eleven selection vectors, explicit/no-baseline full fallback, invalid SHA rejection and an actual read-only Git ancestry check passed. Ajv validates fixture/result schemas; lodash independently filters/orders expected candidates. No Git history was modified. This test is registered in the repository suite; the preceding full suite passed before this new registration, and the new focused check passed separately.

This implements the selection contract only. GitHub history retrieval, the Nx CI command, platform lanes and the workflow still need implementation. The [Nx affected guidance](https://nx.dev/docs/features/ci-features/affected) supports selecting the latest successful relevant base. The [GitHub workflow-run API](https://docs.github.com/en/rest/actions/workflow-runs#list-workflow-runs-for-a-workflow) provides workflow/branch/status filtering, 100-item pages and at most 1000 filtered results. If no trustworthy ancestor can be established from available history, full validation remains the required fallback.

## GitHub History Retrieval

The new GitHub provider discovers the workflow ID from the current run after matching both run and repository identities, then reads successful branch history in 100-item pages up to GitHub’s 1000-result search limit. It normalizes head-repository IDs so fork runs cannot become trusted baselines. HTTP/auth/rate-limit/schema failures discard partial history and require full validation. Cancellation is propagated rather than converted to a successful fallback. The HTTP transport bounds response bodies to 8 MiB and requests to 30 seconds, keeps authorization on the configured origin and does not follow redirects.

RED: the fixture could not import the missing provider. GREEN: the focused fixture exited 0 and printed PASS for pagination, schema/lodash projection, current-run identity, malformed responses, authentication/rate-limit failures, history limits, cancellation, and a real local HTTP server exercising response bounds and redirect refusal. No hosted workflow or authenticated GitHub request was executed. Provider registration in the full suite is new and not yet covered by a complete retry. The current full suite began before that registration.

Official API contract: [workflow runs](https://docs.github.com/en/rest/actions/workflow-runs). The current API documentation specifies version 2026-03-10, which the transport sends explicitly. The CI command, graph-selected platform lanes and workflow remain pending.

## Baseline Command and Native Execution

The uncached repo:ci-baseline target now calls the dedicated small CI script. It reads the actual Git checkout, validates GitHub repository/run/event/checkout identity, uses a pull request’s destination branch, and chooses complete validation for local, scheduled, tagged or explicitly full runs. Privileged pull_request_target events are rejected. The composed resolver connects event selection, paginated workflow history and actual Git merge-base ancestry. The command emits JSON and, when invoked by GitHub Actions, scalar mode/base/head step outputs; it accepts no manually invented base override. An editor entry was added and regenerated.

RED: environment and composed-resolution fixtures failed to import their missing implementations. GREEN: both focused suites passed. A native command fixture passed Ajv result validation, matching GitHub output fields via lodash, actual Git checkout identity, argument rejection, bounded esbuild import ownership and one matching seed/generated editor command. Its first ticket probe used an incorrect generated-output parent and failed before creating files; the path was fixed inside the existing ticket and the retry passed.

The first native Nx invocation could not find the newly authored target. A subsequent graph inspection, after waiting for another graph constructor, exposed the correct uncached target; no cache reset or daemon deletion was performed. The retry ran successfully in 1.3 seconds (124 ms target execution) and printed a complete-validation baseline with the current commit. The cause of the first stale target lookup is not proven.

The full repository suite after the materializer dependency-contract correction passed in 4m20s; that run preceded GitHub-provider and command test registration. A new full suite covering all CI tests is active. The workflow, graph-derived platform lanes, native Windows/Linux qualification and shared-cache trust configuration are still unfinished.

## Complete CI Contract Suite

The full repository suite containing the GitHub history, event selection, composed baseline resolver and native command tests exited 0 in 3m42s (3m41s test task). All four new CI contract groups printed PASS. The suite also passed the existing native tooling, restoration, graph, lifecycle and cancellation contracts across 449 projects. This proves local test execution, not hosted CI or non-Darwin platform behavior.

## Consolidated Validation at 23:53 UTC

The complete repo:test suite containing CI, container-context and isolated WASM-fingerprint contracts exited 0 in 2m30s (2m28s test task). Native repo:toolchain exited 0 in 891 ms and emitted valid installed-tool/pinned-Binaryen identity JSON. This remains local macOS arm64 validation.
