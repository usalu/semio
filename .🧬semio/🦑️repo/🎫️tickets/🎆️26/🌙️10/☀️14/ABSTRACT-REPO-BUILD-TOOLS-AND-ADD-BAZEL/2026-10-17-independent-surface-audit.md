# Independent Public Surface Audit — 2026-10-17

Read-only source audit of root dispatch, package scripts, build invocation authoring, dashboard launch and session propagation, setup routing, agent-client sources, and editor launch entries. No native build, tooling store operation, or runtime gate was run by this auditor. Findings below are source-proven control-flow defects; runtime confirmation remains outstanding.

## Material Finding: Dashboard Reuses a Session Across Selected Build Tools

Priority P2. In `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/🦀️.rs:327`, `start` searches live sessions using `same_launch`; when all members match, lines 333–337 return those existing sessions without submitting a new launch. The predicate at lines 359–361 compares command ID, executable, arguments, and label parameters, and explicitly ignores environment.

The target launcher in `🎛️dashboard/🎮️registry/🦀️.rs:1217–1220` emits `bun repo run <task>` regardless of selected tool. The tool therefore travels through environment rather than argv for the dashboard: `🎛️dashboard/📎️connection/🦀️.rs:99–103` sends the requesting process environment, and `🎛️dashboard/🌀️daemon/🧠️supervisor/🦀️.rs:127–134` preserves `SEMIO_BUILD_TOOL` (only `SEMIO_BUILD_INVOCATION_PID` is dropped, line 41). The neutral build dispatcher consumes that environment to choose its adapter.

Concrete trigger: leave a dashboard task live with `SEMIO_BUILD_TOOL=nx`, then request the same task using `--env SEMIO_BUILD_TOOL=bazel` (or a second dashboard client inheriting Bazel). Its command identity, argv, and label parameters still match. The existing Nx session is returned; Bazel never runs. The reverse transition has the same defect. Explicit request environment is a particularly direct reproducer because it removes ambiguity about the requested tool. The same predicate also ignores selected build mode, but that broader environment identity issue is outside this finding.

A repair should include effective build-tool selection in semantic launch/session identity, including the default `nx`, before deciding reuse. Comparing every ambient variable would unnecessarily prevent reuse. Add a focused regression for same task plus different explicit tool and same task plus same effective tool; validate inherited selection separately because client environment is merged later by the daemon.

## Inspected Coverage Without Additional Material Findings

- Root `package.json` exposes neutral `repo`, `setup`, dashboard and dashboard-install invocations. Root `📜️script.ts` registers `repo` lazily against the same build script. Bun is the host runtime and is not itself evidence of an Nx adapter leak.
- Build caller capabilities declare `tool: nx | bazel`; child task environment explicitly sets `SEMIO_BUILD_TOOL` in `🏗️build/📥️caller/🟦️.ts:68`. Repo-to-Nx authorizer references remain specifically within the Nx adapter path and do not warrant indiscriminate renaming.
- Editor entries around `.vscode/launch.json:196565–196700` expose a build-tool input for run, plan, watch, and reset. Watch executes a neutral child command, relying on the selected outer environment. Entries around 196008–196041 expose setup for all tools or either tool. No separate editor selection-loss defect established.
- Dashboard fresh launch preserves selected tool through client environment to exact child process environment; the issue above is reuse, not fresh launch propagation.
- Agent-client implementation scan found no direct Nx/build invocation to contrast with a neutral equivalent. This is bounded source evidence, not proof that every externally generated configuration has been executed.
- Setup routes dependency preparation through the repository build graph. Mandatory Nx dependency declarations remain consistent with Nx being the primary supported adapter; they are not evidence that Bazel tasks are dispatched to Nx.

## Limits

Concurrent changes to Bazel planning, arguments, and watch/affected selection were owned by other agents and excluded from this audit. Parent-owned public build/cache receipt failure and source/runtime gates were not rerun. This report adds the dashboard reuse finding independently; it does not certify end-to-end parity.
