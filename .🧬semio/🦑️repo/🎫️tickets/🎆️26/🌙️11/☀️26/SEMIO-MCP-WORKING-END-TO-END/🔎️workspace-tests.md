# Workspace crate tests (semio-os-mcp-workspace)

Status: BLOCKED, no quick test could be run.

## Changed
- `🌉️mcp/🪟️workspace-host/📦️packages/🦀️rust/Cargo.toml`: added dev-dependency `semio-os-store-test-support` (path `../../../../🏪️store/🧰️test-support/📦️packages/🦀️rust`; it provides `tempdir()`), fixing the missing `semio_os_store_test_support` in `🏠️workspace/🧪️tests/🔬️long/🦀️.rs`.

## Not done
- `exchange_blocking(&mut channel, ..)`: current signature is `exchange_blocking<C: ArtifactChannel>(channel: &mut C, instance: u32, commands)` in `🔀️dispatch/🔌️channel-port/🦀️.rs:362`, and `open_artifact_channel` returns a concrete `PluginArtifactChannel`, so the call shape looks right; the real error needs the compiler output.
- `DslValue == &str`: not examined (no compiler output).

## Blockers (not caused by this ticket)
Three build attempts:
1. Failed in other agents' in-flight edits: `semio-plugin-instance-stores` (unresolved `semio_plugin_retained_command_operation`, suggests `semio_plugin_retained_command`) and `semio-plugin-instance-app` (unresolved `semio_framework_trace`, suggests `semio_framework_value`).
2. Host memory watchdog killed the build (critically low memory).
3. Source boundary gate: `semio-framework-job-context exceeds its source boundary before the compiler: 17 of 32 files, 262309 of 262144 bytes`.

## Test results
None. Rerun once the above are fixed:
`repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-os-mcp-workspace`
