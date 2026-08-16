# Terra Wave 3 Framework Terminal Dashboard Daemon Command Lease

## Baseline And Boundary

- Root and repository-product instructions were reread before this lease. The CLI glue SHA-256 is `88ffdbd8394725a74aeca1b77eb7e68c62720b89a2b1e0953b1552536c28b2cb`, exactly matching the packet; the commands manifest SHA-256 is `3d07f1c9e303aa5c2d7797e48941f19df1b677a00652bdd44c2eddbe42d65e9b`, also matching.
- The current source boundary is the prior command extraction only. `.vscode/launch.json` is dirty but protected and is excluded from all reads/writes under this lease.
- The daemon lifecycle has one production entrypoint, the `semio daemon` dispatcher arm. Its helper `ctrlc_stub` is a no-op with no independent consumer; no module extraction qualifies.

## Source Decision

- Add `framework.repo.command.terminal-dashboard-daemon` as one exact command member beside the preserved workflow and plugin-registry members.
- Move only the lifecycle parser/dispatcher (`start`, `serve`, `stop`, `status`, `attach`) to a private implementation behind `run(root, parsed)`.
- Delete the no-op `ctrlc_stub` without a replacement, wrapper, or alias. Daemon service, TUI, IPC, catalog, process, and all other CLI behavior stay in their current owners.

## Applied Move

- Appended only the exact `🖥️terminal-dashboard-daemon` command member to the existing commands collection, preserving workflow and plugin-registry membership.
- Added `🎮️commands/🖥️terminal-dashboard-daemon/🦀️component.rs`, exposing only `run(root, parsed)`; it owns the exact existing `start`, `serve`, `stop`, `status`, and `attach` lifecycle dispatch.
- Mounted the component locally from CLI glue and changed the `daemon` arm to `terminal_dashboard_daemon::run(&root, &parsed)`.
- Deleted `daemon_command` and its no-op `ctrlc_stub`. The component uses the existing daemon service and terminal dashboard directly; no alias, forwarder, module, registrar, or service/TUI move was introduced.
- Added a no-side-effect unknown-subcommand test in the command component.

## Validation And Runtime Evidence

- Commands manifest JSON/membership audit passed: exactly workflow, plugin-registry, and terminal-dashboard-daemon command members in canonical order.
- Post-move wrapper sweep finds only the required CLI mount and direct `terminal_dashboard_daemon::run` dispatch; no `daemon_command` or `ctrlc_stub` reference remains.
- `bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.terminal-dashboard-daemon` passed: 1 component, 0 errors, 0 warnings, no findings.
- `bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.terminal-dashboard-daemon` passed with the same clean result.
- `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache` passed: 18/18 tests, including `terminal_dashboard_daemon::tests::unknown_subcommand_returns_usage_without_side_effects`.
- `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache` passed.
- `bun nx run @semio-tech/repo-cli-rs:daemon -- status` passed through the registered daemon target and printed `daemon not running`; no process was started or stopped.
- The build/test/runtime commands emit only the known pre-existing UI unnecessary-qualification and CLI `Read`/`Session.variant` warnings.
- `git diff --check` passed for the tracked glue and command manifest paths.

## Fingerprints, Scope, And Registrar

- CLI glue SHA-256: `a3904c911c312efa7883a3f5b66e4f1019516aba778355552f798c26f4feaf68`.
- Commands manifest SHA-256: `239fe894cdb7d0130efed62ae92b8029b8f0ed5f1bc4e4b8cd80b27a61b7d5aa`.
- Daemon command SHA-256: `589085630dd89b9956b8d7649e9df7e2f6b520e5be67e8dab8e4513af02bd2a6`.
- Changed paths: CLI glue, command collection manifest, the new terminal-dashboard-daemon command component, and this report only.
- `.vscode/launch.json` remains protected and dirty (`M`) with no edit under this lease. No generated artifact, central taxonomy/discovery/script/Cargo path, root glue, or protected owner was touched.
- Registrar request: none. The command remains in the existing CLI crate and needs only its packet-approved local `#[path]` mount.

The terminal-dashboard daemon lifecycle command lease is complete.
