# Registered Staged Form Script Ingress

Read-only source trace; no planner, test or script process launched. Leaf remains held for descriptor forms39823 verification.

Grounded public route from repository root:

~~~sh
repo exec --tool=semio --graph-policy=current --projects=@semio-tech/framework --parallel=1 -- repo ./📜️script.ts test-staged-form
~~~

This is an existing public exec route, not an invented authored test target. Source evidence under 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build:

- 🧭️selection/🟦️.ts:104–108 requires explicit command after -- and creates an uncached repo-exec command target for each selected real project, with program/args preserved and cwd equal to that project root. Actual authored project name is @semio-tech/framework, root 🧰️framework/📦️packages/🟦️typescript. Therefore ./📜️script.ts resolves the actual registered framework router.
- 🔧️tools/🐙️semio/🔌️backend/🟦️.ts:84–87 issues every selected task environment using caller.issueBuildTaskEnvironmentV1; scope100–107 preserves that route for ordinary exec tasks. Executor 🚀️exec/🟦️.ts:29–30 preserves program/arguments,51 resolves the declared cwd,55 starts under the issued environment.
- 📥️caller/🟦️.ts:56–74 reads the genuine issued invocation, validates original build caller/workspace/native authority, then scriptProcessEnvironment at70 projects the actual parent envelope into the selected child. No fabricated environment or ScriptInvocation required.
- Framework 📦️packages/🟦️typescript/📜️script.ts:510 registers test-staged-form to StagedFormScript,512 receives the original parent invocation and passes it to runScriptMain. StagedFormScript466–491 rejects extra arguments and executes its20 explicit absolute test-file paths through runOwnedCommand, including newly registered commit/reference/session/lifecycle/execution suites. It is not reachable by forwarding test-staged-form to the different TestScript handler439–444.

The authored framework project does not declare test-staged-form. Thus repo run @semio-tech/framework:test-staged-form would require a real missing target declaration; do not claim it works from router registration alone. Existing repo run @semio-tech/framework:test invokes its different broad Vitest handler and dependencies; it is not this requested suite ingress. Direct repo /absolute/script.ts has no parent issued task envelope and already refused before tests, as root observed. Tool-runner alone does not manufacture this ScriptInvocation.

Graph scope/caveat: graph/🚪️admission buildInvocationGraphSelectionV1 maps literal exec --projects to selected owner closure. Current policy is a real supported before-- flag; it can serve a usable sealed generation without workspace verification. This avoids explicitly requesting a global refresh, but a missing/unusable generation can still require the normal planner. This read-only trace does not prove the available snapshot is usable; root should retain any actual refusal rather than bypass custody or force an unproven pin. No execution/pass is claimed.
