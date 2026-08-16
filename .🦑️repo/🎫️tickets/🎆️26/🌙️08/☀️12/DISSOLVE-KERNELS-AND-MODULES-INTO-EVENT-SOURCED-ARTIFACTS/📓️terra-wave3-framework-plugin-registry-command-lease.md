# Terra Wave 3 Framework Plugin-Registry Command Lease

## Baseline And Boundary

- The root and repository-product instructions were reread before this lease. The CLI glue SHA-256 is `e9c4aa79957b6d270f3bc3d40bd2dfad71c97b1e3f9a72d882b40f7ec44af59e`, exactly matching the packet fingerprint.
- The sole dirty source in scope is the prior, completed workflow-command extraction in CLI glue plus its two new command paths. No concurrent path conflicts with the plugin-registry command slice.
- The live registry command currently comprises the dispatcher arm, `plugin_registry_command`, and `catalog::check_registry`; its two freshness tests are local glue tests.
- The dashboard JSON builder/output graph is closed: the only non-test caller is the successful old generation branch, the sole remaining references are builder code and its own test, and no reader/mount/schema/generator/package/launch path consumes `🤖️generated/🎛️dashboard.json`. It is deleted rather than preserved as a module or serializer.

## Source Decision

- Add `framework.repo.command.plugin-registry` as the second exact member of the existing commands collection.
- Move the user-facing `check|generate` command, generated JSON validation, and the two freshness tests to `🎮️commands/🔌️plugin-registry/🦀️component.rs`.
- Preserve the existing catalog generated-directory resolver as a direct crate-visible dependency; retain catalog raw-output behavior and the shared process helper for their later independent leases.
- The new component exports only `run(root, subcommand)`. No alias, wrapper, dashboard writer, or old dispatcher name remains.

## Pending Validation

- JSON manifest parsing and old-symbol/dashboard-output sweep.
- Scoped taxonomy report/enforce, configured CLI quick test/build, and CLI check/generate execution through Nx.
- Generator-output observation and diff whitespace checks.
