# Launch Reconciliation And Wave 3 Daemon Command Lease

## Protected Launch Reconciliation

The canonical plugin-registry generation left a split index/worktree state for the protected `.vscode/launch.json` entry `⚖️gate🗄️stdio-catalog`:

| Version | Command |
|---|---|
| Generator-owned index result | `bun nx run workspace:stdio-quick` |
| Working-tree inverse before reconciliation | A direct `bun -e` call to `policyStdioCatalogBreaches` |
| Seed template | The old direct call, intentionally not hand-edited in this central reconciliation |

The working tree has been reconciled to the generated Nx command with `apply_patch`, preserving the staged generator result. The working-tree delta is now empty; the intended staged central delta remains. Current hash:

```text
ad448f785a16be680c97c04ec93a6e63dc9ed84ed65d6bb38cc9658c7e18abaa  .vscode/launch.json
```

Validation passed:

```text
bun nx run workspace:stdio-quick --skip-nx-cache
[stdio] quick passed (36 artifacts, 40 dialects, 6 codecs).
```

The seed is generator-owned input and was not directly modified. Its different template command is an ordered generator-owner reconciliation item, not permission for a manual compatibility edit.

## Shared Capability Assessment

The next candidate shared capabilities do **not** yet meet the formal module rule:

| Capability | Current terminal consumers | Decision |
|---|---|---|
| `proc::spawn_inherit` | `framework.repo.command.plugin-registry` is the only classified terminal semantic component. The current dev implementation is still glue, which does not count. | Do not extract a module yet; one confirmed consumer fails the minimum of two. |
| generated plugin/playground catalog | The new plugin-registry command only validates the generated directory. Its data loader remains consumed by unclassified dev/TUI/catalog glue, so there are no two declared independent terminal consumer components. | Do not extract a module or I/O component prematurely. |

Their prospective common owner is the repository product, but prospective consumers and a possible future owner do not satisfy the declared-consumer graph. The next lease must therefore be a separate bounded command, not a speculative shared extraction.

## Selected Next Lease

Extract the terminal-dashboard daemon lifecycle command from the active repository CLI:

```text
current implementation: 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
destination:             🧰️framework/🛍️products/🦑️repo/🎮️commands/🖥️terminal-dashboard-daemon/🦀️component.rs
semantic ID:             framework.repo.command.terminal-dashboard-daemon
```

The existing `daemon_command` at glue lines 1368–1399 is one user-facing lifecycle action (`start`, `serve`, `stop`, `status`, and `attach`) and has no production referrer beyond the `semio daemon` dispatch arm. It uses the existing daemon/TUI implementation but does not share a new capability or require a global registration. The no-op `ctrlc_stub` at lines 1401–1403 has no consumer or behavior and is deleted with the wrapper; no compatibility forwarder is legal.

## Terra Write Lease

| Path | Operation | Required result |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` | Modify | Add the direct daemon command mount; replace the `daemon_command` dispatcher call with `terminal_dashboard_daemon::run`; remove only `daemon_command` and `ctrlc_stub`. Leave daemon service, TUI, IPC, catalog, process, dev, tests, and all other dispatches untouched. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` | Modify | Append exactly one terminal-dashboard-daemon command member, preserving workflow and plugin-registry members. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🖥️terminal-dashboard-daemon/🦀️component.rs` | Create | Own the exact lifecycle command parser/dispatcher. It imports repository-owned `ParsedArgs`, daemon service, and TUI application directly; it has no public API other than `run(root: &Path, parsed: &ParsedArgs) -> i32`. Add a no-side-effect unknown-subcommand test. |

Pre-edit source and manifest fingerprints:

```text
88ffdbd8394725a74aeca1b77eb7e68c62720b89a2b1e0953b1552536c28b2cb  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
3d07f1c9e303aa5c2d7797e48941f19df1b677a00652bdd44c2eddbe42d65e9b  🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json
```

Exact local assembly wiring:

```rust
#[path = "../../../../🎮️commands/🖥️terminal-dashboard-daemon/🦀️component.rs"]
pub mod terminal_dashboard_daemon;
```

Exact manifest member:

```json
{
  "directory": "🖥️terminal-dashboard-daemon",
  "id": "framework.repo.command.terminal-dashboard-daemon",
  "kind": "command",
  "responsibility": "Controls the terminal dashboard daemon lifecycle and attachment."
}
```

## Required Validation

```text
bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.terminal-dashboard-daemon
bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.terminal-dashboard-daemon
bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:daemon -- status
rg -n 'daemon_command|ctrlc_stub|terminal_dashboard_daemon' -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs 🧰️framework/🛍️products/🦑️repo/🎮️commands
```

The final scan must retain only the direct component mount and direct dispatch, never a wrapper alias or stale function. The runtime status invocation may report that no daemon is running, but it must execute through the registered Nx daemon surface without starting or stopping a process.

## Exclusions

Do not modify `.vscode/launch.json` or its seed further, Cargo files, locks, project files, global taxonomy/discovery files, repo-library index, generated output, framework kernel/machine/platform/renderer, or any currently dirty OS source. No root registrar is required for this package-local source decomposition.

