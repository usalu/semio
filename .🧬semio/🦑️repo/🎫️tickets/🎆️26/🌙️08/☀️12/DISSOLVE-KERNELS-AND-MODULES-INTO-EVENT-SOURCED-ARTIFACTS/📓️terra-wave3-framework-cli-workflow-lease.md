# Terra Wave 3 Framework CLI Workflow Command Lease

## Baseline And Boundary

- Applicable instructions were read from repository root, `🧰️framework/🛍️products`, and `🧰️framework/🛍️products/🦑️repo`; no deeper instruction file applies to the CLI command path.
- Source fingerprint: `dbf5f965d607c2bbc50b8e398e7a5a715da101097c1c9897c7c4575371d28450` for `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs`, matching the packet baseline exactly.
- Both the workflow source and the new `🎮️commands` destination were clean before the lease. The surrounding worktree contains extensive concurrent changes outside this narrow source boundary and remains untouched.
- Referrer sweep found `workforce`, `agent_runner`, and `workflow_command` only in the workflow implementation, workflow dispatcher, and two local tests in `📦️glue.rs`. No second production component consumes this behavior, so no `🔨️modules` extraction qualifies.

## Source Decision

- Create exactly one command component: `framework.repo.command.workflow` at `🎮️commands/🌊️workflow/🦀️component.rs`.
- Move the workflow command, scheduler, workflow JSON contract/types, runner probing/launch, and tests into that component. The component publicly exposes only `run(root, parsed)` to the CLI crate assembly; all implementation types stay private.
- The runner probe selects `where` on Windows and `which` elsewhere through a private repository-owned `executable_on_path` helper, with a pure platform-selection test.
- `📦️glue.rs` becomes mechanical assembly for this component and dispatches directly to `workflow::run`; no forwarding compatibility layer is retained.

## Applied Move

- Added `🎮️commands/🔣️component.json` with exactly one `command` member: directory `🌊️workflow`, ID `framework.repo.command.workflow`, and the packet responsibility verbatim.
- Moved the workflow command, workflow JSON loading/types, scheduler, agent runner selection/launch, and all three focused tests into `🎮️commands/🌊️workflow/🦀️component.rs`.
- Added the exact local CLI mount `#[path = "../../../../🎮️commands/🌊️workflow/🦀️component.rs"] pub mod workflow;` and replaced dispatch with `workflow::run(&root, &parsed)`.
- Removed the old `workforce` and `agent_runner` modules, `workflow_command` wrapper, and the two former glue-local workflow tests. No forwarding alias or compatibility export remains.
- Removed the unconsumed `save_workflow` helper rather than retaining a zero-consumer private behavior surface.

## Validation And Evidence

- Post-move SHA-256:
  - CLI glue: `e9c4aa79957b6d270f3bc3d40bd2dfad71c97b1e3f9a72d882b40f7ec44af59e`.
  - Command manifest: `f1f19500fb7e51e3f6f9e4c413cf19f8d61c71be439d29ec3b5f07c9efd771ac`.
  - Workflow component: `36020cb8963b6d2cd31b221590dd511890e7b2cecaccc17cba3ab44aaa003424`.
- Command manifest JSON parsing passed.
- Post-move source sweep found no `workforce`, `agent_runner`, `workflow_command`, or `crate::workforce`/`crate::agent_runner` reference in repository-product Rust source. Glue contains exactly the required local mount and `workflow::run` dispatch.
- `bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.workflow` passed: 1 component, 0 errors, 0 warnings, no findings.
- `bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.workflow` passed with the same clean result.
- `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache` passed: 18/18 tests, including scheduler ordering, unavailable-runner filtering, and platform probe selection. It emits only the packet-baseline UI qualification warnings and CLI `Read`/`Session.variant` warnings.
- `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache` passed. It emits those same pre-existing warnings only.
- `git diff --check -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` passed; no-index whitespace checks for both new command files and this report produced no diagnostics (their expected new-file comparison exit status is 1).

## Changed Paths And Release Verdict

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json`
- `🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️component.rs`
- This report

No protected library index, root/taxonomy/script/launch file, generated artifact, global registrar, or unrelated CLI command was modified. The workflow-command lease is source-complete and ready for the next independent CLI slice.
