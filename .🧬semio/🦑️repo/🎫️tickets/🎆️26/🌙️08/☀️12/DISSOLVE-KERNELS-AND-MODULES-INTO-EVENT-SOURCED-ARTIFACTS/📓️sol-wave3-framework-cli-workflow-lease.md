# Wave 3 Framework CLI Workflow Command Lease

## Decision

The next conflict-free Wave 3 lease is the workflow command inside the active repository-product CLI:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli
```

This is deliberately a small first slice of the owner, not a broad rewrite of the remaining CLI/TUI/daemon monolith. The owner is non-dirty, outside the kernel, machine, platform, renderer, and repo-library-index quarantines, and it has a closed source and production-referrer boundary.

## Evidence

| Check | Result |
|---|---|
| Active scope | `🧰️framework/🛍️products/🦑️repo` is `clean` in the taxonomy, so it is structurally included. |
| Current census record | `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli`, kind `module`, marked `delete` only because the static adapter has no Cargo-edge evidence. |
| Cargo evidence | Offline `cargo metadata --offline --format-version 1` resolves the package as the live `semio` binary and library, with zero reverse **crate** dependencies. Thus it is an application/command entrypoint, not dead code and not a reusable module. |
| External production referrers | Root `Cargo.toml` workspace member plus four `.vscode/launch.json`/`.vscode/🧩️launch.seed.jsonc` command paths. None is touched by this slice. |
| Source scope | `📦️glue.rs` lines 425–651 implement only workflow scheduling and coding-agent launch. Lines 1732–1790 are the workflow dispatch arm. The two associated tests are lines 2010–2042. |
| Reverse closure | `workforce` and `agent_runner` are used only by the `semio workflow` command; tests are excluded. Neither meets the two-independent-production-component threshold for a `🔨️modules` extraction. |
| Baseline | `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache` passed 17/17 tests on 2026-08-16. It emitted pre-existing UI unnecessary-qualification warnings and two CLI warnings (`Read` import and `Session.variant`). |

The prior broad source partitions also show why this is the correct small frontier: `workforce` is 171 lines, `agent_runner` is 56 lines, while the remaining terminal dashboard is 931 lines and has direct daemon/catalog/I/O dependencies. No protected owner or global registration needs to move for this command leaf.

## Semantic Disposition

Create one command component at the repository-product owner:

```text
🎮️commands/🌊️workflow/🦀️component.rs
```

with semantic ID `framework.repo.command.workflow` and responsibility:

> Executes a ticket-scoped dependency workflow using locally detected coding-agent process runners.

`Scheduler`, workflow file loading, task/status types, runner detection, and runner launch remain private implementation of this one command component. They must not be promoted to `🔨️modules`: one command's internal call sites and its tests do not qualify as independent consumers. The command component receives the root and parsed invocation from the CLI dispatcher and returns the command exit code. It owns the `🌊️workflow.json` read contract; no compatibility export or forwarding alias is permitted.

The agent executable probe must be made platform-specific while moving it: use `where` on Windows and `which` on Unix, behind a repository-owned private `executable_on_path` function. Add a pure platform-probe test; do not add an external runtime dependency or expose process-library types.

## Terra Write Lease

The following is the complete writable set. The worker must re-read the three applicable `AGENTS.md` files and rehash the existing source before editing.

| Path | Operation | Required result |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` | Modify | Reduce this source to mechanical crate assembly for the new `workflow` component; remove the old `workforce` and `agent_runner` definitions, the `workflow_command` wrapper, and only their imports/tests. Dispatch calls `workflow::run`. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` | Create | Canonical `x-semio` collection manifest declaring exactly `🌊️workflow` as the `framework.repo.command.workflow` command member. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🌊️workflow/🦀️component.rs` | Create | Own the command entrypoint, scheduler, workflow specification/status types, agent runner implementation, and their tests, organized with regions/subregions. |

Pre-edit source fingerprint:

```text
dbf5f965d607c2bbc50b8e398e7a5a715da101097c1c9897c7c4575371d28450  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
```

The local crate assembly mount is exact and does not require a global registrar:

```rust
#[path = "../../../../🎮️commands/🌊️workflow/🦀️component.rs"]
pub mod workflow;
```

The component must expose only `workflow::run(root: &Path, parsed: &ParsedArgs) -> i32` to that assembly. `workforce`, `agent_runner`, `Scheduler`, `WorkflowSpec`, and `AgentRunner` become non-public outside the component unless an actual production component consumer is proved later.

## Explicitly Out of Lease

Do not write these paths or create a compatibility layer:

- `Cargo.toml`, `Cargo.lock`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, or any Nx cache/generated workspace graph: this slice preserves the package path and registered launch command.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts` and all taxonomy/discovery SSOT files.
- Framework `🎠️kernel`, `🔄️machine`, `🖥️platform`; OS `📺️renderer`; and every dirty source path recorded by the current lease map.
- The CLI's args, workspace, process, catalog, dev, registry, IPC, daemon, and TUI partitions. They require later graph-colored leases; none is a consumer move for this command slice.

## Required Validation

After source/referrer hashes remain stable and after the command collection manifest is in place, the Terra worker must run:

```text
bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.workflow
bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.workflow
bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache
```

It must also prove the source move mechanically:

```text
rg -n 'pub mod (workforce|agent_runner)|workflow_command|crate::(workforce|agent_runner)' 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
```

The result must contain no stale old implementation/forwarding reference. The scoped tests must include scheduler dependency/order behavior, unavailable-runner filtering, and Windows/Unix probe-command selection. No generator run is required because the crate/package and registered launch location do not change.

## Registrar Queue

None for this slice. A later owner-level CLI reclassification may relocate the package from the invalid `🔨️modules/⌨️cli` owner into a specifically named application/command owner. That larger migration must be separately leased with the root Cargo and launch configuration registrar; it is intentionally not bundled with this source leaf.

