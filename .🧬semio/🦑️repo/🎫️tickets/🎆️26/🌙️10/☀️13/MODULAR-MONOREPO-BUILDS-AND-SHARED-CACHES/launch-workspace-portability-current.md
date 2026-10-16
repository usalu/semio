# Launch Workspace Portability

The existing launch catalog embedded the current developer workspace prefix in 120 string occurrences, including cwd, already-quoted command paths and environment JSON/capture paths. The preceding real JSON check failed before this edit (`launch-workspace-portability-red1.log`). This narrow configuration repair replaces that exact prefix with the already-established `${workspaceFolder}` syntax. No script, adapter, migration, new command, fixed cache cap or command registration was added.

A recursive JSON projection verified that every changed value differs only by that exact workspace-prefix substitution. All 12815 configuration entries, their order, names and other fields are preserved. The edit reread the live file immediately before writing to detect intervening peer edits. Absolute `/home/...` and `/Users/Example` deliberate test strings were not altered.

This repair makes those launch paths relocatable; it does not prove every launch works on all platforms or that a literal Cargo destination escapes a genuine managed compiler. Existing actual repository Cargo test/launcher admissions replace caller CARGO_TARGET_DIR and CARGO_BUILD_BUILD_DIR with their issued store paths. Other concrete consumers still require their own qualification. No launch, compiler or Native effect ran for this change.

Before SHA-256: `800b699f1ceea8f53583cd7b5656d886cfd0f9fd614300658e2efe7d7e5f6572`. After SHA-256: `9f0b428abad751bfb7bc4eb87e7194277be995fae47bfb72bd925ba76d700dc6`.

## Executed Checks

The after-edit Python JSON gate exited0 with12,815 registrations and zero developer-specific workspace literals. Independent installed TypeScript `parseConfigFileTextToJson` exited0 with the same12,815 registrations and no parse diagnostic. These are read-only configuration checks; no production/Native runtime or platform execution is claimed. The full pre-edit Cargo destination classification remains in `launch-cargo-destination-observation-current.md` as its earlier Source epoch.

| Evidence | SHA-256 |
| --- | --- |
| `launch-workspace-portability-red1.log` | `8178a9cc71b951b45af4faa073b452bfdb0c15f2a2f6fae8b87b9f6be2f57a31` |
| `launch-workspace-portability-green1.log` | `3bac36b824859da0c3c9aa4da66fd11db2a8030b489486b6788b49584d2d664c` |
| `launch-workspace-portability-oracle1.log` | `55d47d398ff3e201da090ead15d0a645f1c1b84feed9a2bbd09b1d5e5ee3fdaa` |
