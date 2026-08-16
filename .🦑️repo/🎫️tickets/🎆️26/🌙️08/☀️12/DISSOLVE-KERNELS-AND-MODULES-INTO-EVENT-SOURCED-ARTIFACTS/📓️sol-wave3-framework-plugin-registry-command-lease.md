# Wave 3 Repository Plugin-Registry Command Lease

## Decision

Lease the next independent CLI action, `semio plugin registry check|generate`, after the clean workflow-command extraction. This is a narrow repository-product command boundary; it does not enter any protected kernel, machine, platform, renderer, or repo-library-index path.

```text
current owner: 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli
destination:   🧰️framework/🛍️products/🦑️repo/🎮️commands/🔌️plugin-registry/🦀️component.rs
semantic id:   framework.repo.command.plugin-registry
```

## Semantic Evidence And Disposition

| Current implementation | Evidence | Disposition |
|---|---|---|
| `plugin_registry_command`, glue lines 386–422 | The only runtime entry is the `plugin registry` arm in the CLI dispatcher. It validates generated registry JSON or forwards to the canonical Nx generator. | Move into the named command component. |
| `catalog::check_registry`, lines 156–170 | Used only by that command; its two current tests are command behavior tests. | Move as private command implementation, renamed specifically for generated plugin-registry validation. |
| `registry`, lines 428–509 | Its only non-test call is the successful `plugin registry generate` branch. A complete non-cache scan finds no reader, mount, schema, generator input, launch flow, or package entrypoint for `🤖️generated/🎛️dashboard.json`; the only other use is its unit test. | Delete the output builder, data types, write, console line, and test after this proof. It is neither a qualified shared module nor a live serializer boundary. |
| `proc::spawn_inherit` | The command and the separate dev action both use it. | Leave it in its present assembly for the subsequent two-consumer shared-capability lease; do not copy or move it in this command slice. |
| catalog directory path resolver | Remaining catalog import/raw-output behavior still owns it. | Change only to crate visibility so this command can validate the already generated files; no duplicate resolver and no new generic helper. |

The dashboard file writer has no production terminal consumer after traversing its reverse closure. The direct caller is this one command, and its written artifact has no consumer or registration. In a greenfield repository with no speculative external consumers, retaining the write would preserve dead active behavior. The command's supported `generate` outcome is therefore the canonical `@semio-tech/plugin-registry:generate` Nx generation alone.

The command continues to own its in-process freshness checks privately. Those checks read `🔣️plugins.json` and `🔣️playgrounds.json` only to report missing/invalid generated registry output; they do not establish a reusable shared I/O abstraction.

## Terra Write Lease

The worker owns exactly these paths and no generated output, root registrar, or protected library index path.

| Path | Operation | Required result |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` | Modify | Mount `plugin_registry`; remove `plugin_registry_command`, the entire `registry` builder, dashboard write invocation/console output, command-check tests from the root test module, and the old dispatcher reference. Keep unrelated catalog, process, dev, daemon, TUI, and test code in place. Make only the existing generated-directory resolver `pub(crate)` for this direct command dependency. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔣️component.json` | Modify | Append exactly the `🔌️plugin-registry` command member. Preserve the existing workflow member byte-for-byte. |
| `🧰️framework/🛍️products/🦑️repo/🎮️commands/🔌️plugin-registry/🦀️component.rs` | Create | Own `check|generate` command dispatch, generated-registry JSON validation, and the two moved freshness tests. Its only public crate API is `run(root: &Path, subcommand: &str) -> i32`; all validation helpers remain private. |

Pre-edit fingerprint:

```text
e9c4aa79957b6d270f3bc3d40bd2dfad71c97b1e3f9a72d882b40f7ec44af59e  🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs
```

The exact local assembly mount is:

```rust
#[path = "../../../../🎮️commands/🔌️plugin-registry/🦀️component.rs"]
pub mod plugin_registry;
```

The CLI dispatch changes directly to `plugin_registry::run(...)`; do not leave a forwarding module, re-export, old command name, or dashboard-file alias.

The command collection manifest member is:

```json
{
  "directory": "🔌️plugin-registry",
  "id": "framework.repo.command.plugin-registry",
  "kind": "command",
  "responsibility": "Verifies and regenerates the canonical plugin and playground registry."
}
```

## Consumer Moves And Test Ownership

- `run` is the sole production consumer move: its `plugin registry` arm changes from `plugin_registry_command::run` to `plugin_registry::run`.
- The old command's call to `catalog::check_registry` becomes a private `check_generated_plugin_registry` implementation in the new component. It uses the existing catalog generated-directory resolver by direct crate import.
- `check_registry_reports_missing_generated_files` and `check_registry_reports_invalid_json_and_passes_when_valid` move into the command component with local ticket-temp-root helpers.
- `registry_build_includes_agents_and_verbs` is removed with the zero-consumer dashboard registry builder. Tests/examples/glue do not keep that behavior alive.
- No package, Cargo member, Cargo lock, Nx project, launch configuration, runtime mount, external caller, generated file, or shared repo-library index changes belong to this lease.

## Validation

The source owner has just passed the previous command lease's quick suite (18/18) and release build according to the release handoff. The Terra release must independently run:

```text
bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.plugin-registry
bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.plugin-registry
bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache
bun nx run @semio-tech/repo-cli-rs:run -- plugin registry check
```

For the registered generation surface, run the command's `generate` branch through Nx only after recording the generated-file baseline, then verify the canonical plugin-registry target reports fresh output and that no dashboard output is recreated:

```text
bun nx run @semio-tech/repo-cli-rs:run -- plugin registry generate
bun nx run @semio-tech/plugin-registry:generate
rg -n 'Dashboard(Registry|Task|Agent)|dashboard_json_path|registry::(build|generate)|🤖️generated/🎛️dashboard\.json' -- 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust 🧰️framework/🛍️products/🦑️repo/🎮️commands
```

The final scan must be empty. If the real generator changes tracked artifacts, regenerate through its established Nx target and report every resulting generated path; no generated file may be hand-edited.

## Exclusions

Do not touch `Cargo.toml`, `Cargo.lock`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, taxonomy/discovery SSOT, Nx caches, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`, or any dirty/quarantined framework/OS owner. The next lease for the genuinely shared process invocation and catalog import remains separate.

