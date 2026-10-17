# Remaining Authored Neutral Command Surfaces

Read-only current Source audit, 2026-10-17. No command/task/daemon/installer, Native/protected operation, dependency/cache traversal, AGENTS edit or runtime mutation was performed. The existing ticket stays open under Root. This report records actionable remaining surfaces; no implementation or runtime acceptance is implied.

## Current Executable Census

`rg --files` selected authored `package.json`, `project.json`, `📋️project.json` and `.vscode/launch.json`, excluding node_modules, build output, ticket/generated and Git trees. All 1,692 selected JSON files parsed: 311 package manifests, 1,378 emoji project descriptors, two plain project descriptors and one launch document. Structured inspection covered 694 package script rows, 3,518 target rows and 13,375 launch configurations. The target executor inventory is 3,374 `repo:script`, one `repo:noop`, and 143 targets without an explicit executor; no foreign executor was found in that inventory. The descriptors include test/fixture declarations, so counts are a Source census rather than an accepted runtime project graph.

No direct Nx or Bun-install executable was found in those package script strings/project JSON strings; an extended script pattern also checked direct Nx at shell boundaries and node_modules Nx executable spellings. Parsed launch program/runtimeExecutable/runtimeArgs/args had no direct split Nx/install candidate. Package script rows all contain the neutral `repo` route, except the necessary root `repo` Bun bootstrap itself. This is finite literal/structured coverage, not a proof against arbitrary dynamically generated commands, inferred targets, task behavior or every shell grammar.

The saved census contains every selected input path, bytes and SHA256: `🗑️generated/bazel-planner/neutral-surface-current-census.json`. It is point-time evidence; no old frozen manifest is promoted to current runtime authority.

## Concrete Remaining Executable Bypass

| Source | Current behavior | Required next change |
| --- | --- | --- |
| `.github/workflows/architecture-quiz.yml:63` | Site job invokes `bun install --frozen-lockfile --ignore-scripts` before neutral test/publish tasks. | Route dependency synchronization through the existing neutral dependency owner, preserving frozen locks/disabled lifecycle scripts and genuine funded/canonical admission. |
| `.github/workflows/architecture-quiz.yml:109` | Proctor job repeats the same direct installation. | Apply the same owner route; verify both clean-runner jobs before claiming zero-touch CI. |

These are actual authored workflow executions, not backend adapter strings or negative fixture DATA. The current defining build entry at `library/🏗️build/📜️script.ts:119–122` exposes `bun repo dependencies sync --all` and `--scope <owner>`; the dependency owner builds sync argv with `--frozen-lockfile` (`dependencies/🟦️.ts:92`) and owns its installation environment/lifecycle policy. `dependencies/📜️script.ts:12–39` handles the selection. The appropriate exact CI selection must be chosen from the actual quiz owners and current ALL completeness, then tested on a clean runner. Merely changing the workflow string is not a claim that today's Dependencies257-path/physical Source-leaf blockers are solved. The old `~/.bun/install/cache` action cache also belongs in that owner-routing review, because the canonical installation owner controls its own funded cache; do not silently assume this generic cache remains effective.

No workflow was executed or edited here. This audit does not replace the ongoing genuine funded producer work.

## Remaining Public Guidance, Without A Backend Bypass Claim

The previously resolved dashboard introductory run/build examples (`dashboard/README.md:14,25`) and clean-skill daemon-stop command remain exactly correct. Their current hashes still match `2026-10-17-dashboard-neutral-command-resolution.md`; do not repeat those edits.

Later portions of that same dashboard README still spell the old root package alias:

- Lines 99–101 explain `semio` using `bun run dashboard` and show `bun run dashboard run os-hub:dev --detach --wait-ready`.
- Lines 141–142 direct agents/editors to `bun run dashboard run <id> ...` and `bun run dashboard commands <words> --json`.

The root README also retains alias examples at lines 131,145–152,166–167,656–673,843,880–884. Root `setup` and `dashboard` aliases themselves already dispatch neutral `bun repo run` commands, so this is an incomplete public spelling/documentation conversion, not a demonstrated task backend escape. The defining exact launch route remains `bun repo run @semio-tech/repo-dashboard-rs:run --output=stream` with native arguments forwarded; dedicated daemon operations must retain the already-qualified declared target rather than blindly replacing text. Reference the existing command-resolution report when normalizing the remaining guide examples.

All four current `.agents/skills/*/SKILL.md` files were scanned as Source guidance without invoking any skill. No direct Nx/Bun-install guidance remained there. Their direct `bun ./📜️script.ts clean`, commit and micro-commit routes are explicitly owned Git/cleanup implementation entry points, not a task-build backend selector. No clean/commit action was run. No AGENTS file was edited.

A separate archived-planning search found 209 matching lines in 99 `.cursor/plans/*.plan.md` files and zero matches outside that plan tree in the inspected `.github/.agents/.cursor/.claude/.codex` Markdown/rule scope. Those plans contain historical Nx commands, obsolete package identities and direct install suggestions. They are not live launch/skill declarations. Treat them as historical task context and do not copy their old commands into current executable guidance; a blanket rewrite cannot establish current target identity. The bounded raw match list is `🗑️generated/bazel-planner/neutral-agent-guidance-current-matches.txt`.

## Intentionally Preserved Boundaries

- Root package `repo` must still bootstrap the first-party build script with Bun; runtime choice is distinct from build-backend choice. Nx remains the configured primary adapter until complete current verification permits changing it.
- The devcontainer `postCreateCommand` is already `[bun,repo,run,workspace:setup]`; start/attach call owned setup-runtime hooks. No raw install/Nx task candidate was found there.
- Dashboard IPC/provider binding and Rust command selection are covered by previous Source tests/reports; this audit found no new direct literal backend launch in the current parsed surfaces and did not rerun the live dashboard.
- Private Nx SDK adapters, Bun installer implementations, graph projection, negative command fixtures, taxonomy descriptions and historical fixture packageCommand strings are not user-facing dispatch authority. No renaming or removal is recommended based on substring matches.
- Complete installed-Nx/Bazel parity, accepted authored ALL/audit/plans, live platform/dashboard/agent qualification, and the current Dependencies producer/Source-binding transition remain separate gates. No runtime test was run for this read-only audit.

## Comparison And Point-Time Inputs

Compared `2026-10-17-neutral-current-coverage-followup.md`, `2026-10-17-current-neutral-interface-followup.md`, `2026-10-17-public-interface-bounded-followup.md`, `2026-10-17-dashboard-neutral-command-resolution.md`, `2026-10-17-authored-task-coverage-audit.md` and `final-neutral-public-interface-audit.md` against current files. This extends the earlier bounded command census; it does not reopen already-resolved dashboard authority/default selection or the clean skill.

| File | Bytes | SHA256 |
| --- | ---: | --- |
| `.github/workflows/architecture-quiz.yml` | 5198 | 9dda05437fa538835d0b2b23c4dce27bd76ffff06f9e64c840260822c0627f89 |
| `README.md` | 58495 | 3afcbd2ca4db03e67ea7028eec5439a930d8c65c861b1cdd407aee9e3aedf089 |
| `dashboard/README.md` | 27564 | c09dbfefaab086d4c3163c920d1b3bf15b3aa333e447b6dc1e1f5faf6d39e5f8 |
| `.agents/skills/clean/SKILL.md` | 2942 | 0af7764572861a8ec2358f051fc5560dbe38e72ac2830df9087639fc5a3e0ee0 |
| `.devcontainer/devcontainer.json` | 11928 | 775d0049adf4b386408cc10e35c095d4d0de64e20a5c4b89a7f0f4fdacc24fa9 |

Only this Markdown and the two named Generated audit files were written. All runtime/Probe/DS/ADMIN/bindings remained untouched.
