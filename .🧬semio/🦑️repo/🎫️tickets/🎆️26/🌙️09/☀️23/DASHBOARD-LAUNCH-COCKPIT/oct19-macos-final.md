# Dashboard Control Plane: Implementation and Verification Status

## Current Status

The requested source changes are implemented and independently reviewed. Full runtime completion is **not verified**. The existing Dashboard Launch Cockpit ticket remains open because the native dashboard build, installed command check and focused suites still need to finish through the repository build runner.

The repository MCP was connected through the configured stdio transport and `repo://goals` was read. Reopening the covering ticket returned `ticket is already open`; the associated Running Repo goal was preserved. No goals were opened, closed or reopened. No external issue/PR publication or modifying Git command was performed.

## Changes

- Removed the ten remaining tracked historical `launch.json` files. Independent physical tracked-file inventory now finds zero conventional IDE configuration files. Reports, captured historical inventory data and unrelated ticket inputs remain. The sole direct fixture consumer's obsolete launch-checker branch was removed while preserving its neutral tokenizer checks.
- Removed .NET's dependency on the deleted solution file and the course's dependencies on deleted editor launch/task files. Neutral build/cache policy and dashboard schema inputs remain.
- Retired the process environment schema/test protocol that read IDE catalogs. The existing shared vectors now exercise the actual compiler environment in both build modes with SQLite, Ajv and the repository validator. A localized `test-runtime-environment` target exposes this verification through the dashboard.
- Made every dashboard test subprocess await the owned execution helper, retaining the original invocation, cancellation signal and installed executable reader through completion. Corrected native window test ownership after the dashboard/UI crate split and removed the terminal manifest's duplicate dependency on a nonexistent core path.
- Repaired canonical fresh-checkout dashboard routing. Native arguments and help reach the native launcher even without an installation; authoring and explicit build/backend scheduling stay with the build runner.
- Declared exact conditional build/compile descendants in the dashboard owner metadata before the original invocation is issued. The initial expansion and child projection use the same authored closure. Runtime admission remains strict; it was not weakened to accept undeclared work. The nonexistent standalone workflow target was retired; workflow arguments under the run target remain supported.
- Missing installations now rebuild for observational/control commands too. Existing stale installations remain usable for stopping or inspecting running work.
- Updated the developer guide to the dedicated Debugger Availability and Debug a Program forms. The local ignored build skill now describes owner metadata rather than editor launch generation. AGENTS files remain unchanged; the current explicit user request supersedes the protected root launch-registration instruction.

The existing dashboard already supplies typed owner targets/tools, service readiness and lifecycle controls, logs, debugging, source navigation and read-only Git tools. Advanced shell and Git write operations remain reachable through its supervised terminal; no dedicated Git write forms were added in this change.

## Observed Validation

- `bun repo --help` exited 0 after the final source changes, preserving the ordinary build help route.
- Scoped `git diff --check` and owned file/path/whitespace checks passed. These are structural checks, not application tests.
- The first real `bun run dashboard commands --check` attempt reproduced `Unexpected build argument before --: commands` on a missing installation.
- After routing repair, the same command logged `[dashboard] Rebuilding prototype because it is not installed`, then reproduced `Undeclared original Repo build selection`.
- After the exact declaration repair, the canonical command logged automatic rebuilding and entered `repo run @semio-tech/repo-dashboard-rs:build`. Neither earlier error recurred before it queued behind the shared dependency-image producer. This verifies those routing/admission transitions only; native compilation and command-registry validation did not complete.
- Cancelling that owned queued bootstrap produced `dashboard:rebuild stopped: cancelled`. Its wrapper and descendants settled; the owned process inventory was empty afterward. This is queued-bootstrap cancellation evidence, not native task/daemon cancellation evidence.
- The dashboard-control, dispatcher and runtime-environment test commands were invoked through `bun repo`, but no test body ran. No passing tests, red/green counts, native UI success, compiler success or full cross-platform success is claimed.

## Verification Blocker and Cleanup

The shared dependency image was held exclusively by other tasks' live dependency producers. The first course bootstrap producer settled without this fleet intervening; a separate `bun repo dependencies lock --all` producer subsequently held it for more than 25 minutes while this fleet's suites waited. Both independent investigations found the expected exclusive maintenance scope, with no demonstrated safe dashboard-local lock repair. No other task's process, image, lease, mount, source change or Git state was disturbed.

Only this fleet's queued test/bootstrap processes were interrupted. Their cancellation settled through the original controls. Generated logs, samples and command outputs were removed after this Markdown evidence was recorded; authored inputs and reports remain.

The fleet used the maximum four concurrent slots: coordinator, GPT 6.1 Sol High execution agents and GPT 6.1 Sol Low read-only agents. Low is the supported tool label corresponding to the requested Light. No tool is exposed to change the active main conversation's model/reasoning setting, so the existing main configuration was retained rather than claiming it was switched.

## Required Remaining Verification

When ordinary shared dependency preparation settles, run the focused dashboard-control, dispatcher, bootstrap/launch and runtime-environment suites through `bun repo`, then compile/install the current dashboard and complete `bun run dashboard commands --check`, native terminal interaction and service/task lifecycle checks. Resolve any resulting failures before closing this ticket or claiming the dashboard is fully operational across all supported hosts.

Detailed evidence: [implementation plan](oct19-macos-plan.md), [IDE retirement](oct19-macos-ide-retirement.md), [execution and declarations](oct19-macos-execution.md), [runtime environment](oct19-macos-runtime-environment.md), [bootstrap investigation](oct19-macos-bootstrap.md), [independent audit](oct19-macos-independent-audit.md).
