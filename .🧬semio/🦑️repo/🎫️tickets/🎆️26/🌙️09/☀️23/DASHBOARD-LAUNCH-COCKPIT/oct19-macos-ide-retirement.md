# macOS IDE Retirement Completion

The remaining active build inputs depended on removed IDE assets. The .NET named source group still required `Monorepo.sln`; the architecture course default group still required `.vscode/launch.json` and `.vscode/tasks.json`.

## Changes

- The .NET source group retains C# sources and project manifests and now includes the existing neutral build and cache policies. These policies declare the graph inputs and .NET toolchain, environment and cache contract. The removed solution is no longer an input.
- The architecture course retains its dashboard registry schema input and no longer requires removed launch or task configuration.
- The dashboard control regression contract now declares language-neutral named-input vectors, checks both source groups, rejects solution and editor-workspace entries, and confirms every literal workspace input retained by these vectors physically exists. The repository schema compiler and third-party Ajv compare acceptance of shared vectors; TypeScript independently parses the current JSON policies/manifests before their input lists are compared. Gherkin records the expected behavior.
- The local `.agents/skills/build/SKILL.md` guidance now requires owner manifest dashboard metadata and explicitly gives the newer dashboard request precedence over older launch-registration guidance. This skill is ignored/untracked locally (`git ls-files` returned no entry). The tracked regression deliberately does not read that local-only skill, so fresh clones gain no dependency on it. No AGENTS file changed.

## Audit

A read-only inventory excluding ticket, dependency, cache and build-output directories found no active `.vscode`, `.idea`, `.vs`, Fleet, Zed, run, Nova or similar IDE configurations, and no solution or code-workspace files. The extension product's `.vscodeignore` remains product packaging source. Mutable Markdown outside AGENTS, historical `.cursor/plans`, tickets and build/cache output contained no retired launch, solution or generation guidance after the local skill update. Historical plans remain evidence.

The scoped active script and documentation audit found no additional editor launch generators. The parent separately owns the process build-environment schema/test references found during the concurrent audit.

## Verification

The schema, shared vectors, Gherkin and test were authored before changing the build/course inputs. `bun repo run @semio-tech/repo-lib:test-dashboard-control` was invoked through the required build runner. It remained in shared bootstrap/graph resolution and emitted repository progress before any test body ran. Implementation proceeded after the parent confirmed the same bootstrap condition. No observed red count or passing test count is claimed.

Read-only `git diff --check` passed for this slice. Runtime test execution remains pending the shared build runner. The temporary runner log is under the ticket `🗑️generated` folder and will be removed during ticket completion.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎛️dashboard-control/🥒️.feature`
- Local ignored guidance: `.agents/skills/build/SKILL.md`

No dashboard package script, install, Git state, worktree or unrelated concurrent source was modified by this slice.

## Canonical Dashboard Bootstrap Routing

The parent reproduced a separate runtime defect: `bun run dashboard commands --check` reached generic build argument validation when no installation record existed, and `bun run dashboard --help` printed generic build help.

The build owner now recognizes eligible dashboard native arguments independently of installation presence. The native launcher owns installation/rebuild and receives the unchanged Original invocation, selected build mode, workspace, network authority and issued cancellation/deadline control. Generic build help only consumes help when the request is not an eligible native dashboard invocation. Build/compile/install/test selections and explicit graph/batch/parallel/exclude-dependencies selections retain backend routing; arguments after `--` stay opaque.

Shared schema/vector/Gherkin contracts were authored before the implementation. The regression executes the actual extracted build dispatch branch and actual outer build entry behind bounded import/resource ports, with genuine issued Original/phase controls. It compares shared route vectors against SQLite's independent relational result, validates vectors with Ajv and the repository schema compiler, and uses TypeScript to parse/transpile the actual source boundaries. Cases cover missing/present installation, prototype/release, native help, opaque backend-looking payload, each native target, authoring/backend fallback and nonzero native exit.

The existing ordinary test runner still waits behind the live `dependencies lock --all` holder; no test body has executed. `git diff --check` passes. The additional implementation file is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`.

The ordinary `bun repo --help` command executed after the routing edit and exited 0, confirming the build entry still loads and generic build help remains available. This is a CLI boot check, not a claim that the dashboard route regression suite executed.

The parent then executed the canonical dashboard command against the missing prototype installation. Its runtime output reached `[dashboard] Rebuilding prototype because it is not installed`, confirming the native route no longer falls into generic build argument rejection. The next failure was `Undeclared original Repo build selection` during the rebuild child projection; the separate execution owner is fixing that caller/metadata closure. The canonical log is `🗑️generated/macos-dashboard-current.log`. This observation confirms dispatch, while full bootstrap and test-suite completion remain pending.

## Final Runner Settlement

At the parent's cleanup request, only this slice's dashboard-control runner PID 2966 received SIGINT after more than 24 minutes of shared bootstrap waiting. Session 88398 settled with exit 1 and `Original Script process interrupted`. The test body never executed, so there is no red count or passing suite count. A read-only queue inspection confirmed PID 2966's own dependency-image ticket was removed by normal cancellation cleanup. The unrelated PID 99823 dependency producer remained live with its exclusive ticket and was not signalled or otherwise modified.

The parent separately confirmed the final canonical cold route now reaches lazy prototype rebuild and the exact declared dashboard `:build` child without the former undeclared-selection error, then waits behind the same unrelated dependency producer. The parent owns the final caller/metadata fix and its evidence. Runtime acceptance remains unfinished; the ticket stays open. The parent will remove temporary generated logs after retaining the Markdown audits.
