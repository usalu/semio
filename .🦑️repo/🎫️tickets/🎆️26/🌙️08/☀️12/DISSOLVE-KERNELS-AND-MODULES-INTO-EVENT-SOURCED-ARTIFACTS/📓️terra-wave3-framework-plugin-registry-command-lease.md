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

## Applied Move And Deletion

- Appended only the exact `🔌️plugin-registry` command member to the existing command collection; the pre-existing workflow member remains semantically unchanged.
- Added `🎮️commands/🔌️plugin-registry/🦀️component.rs`, exposing only `run(root, subcommand)` and keeping generated-registry validation plus test helpers private.
- Mounted the component from CLI glue with `#[path = "../../../../🎮️commands/🔌️plugin-registry/🦀️component.rs"] pub mod plugin_registry;`; the `plugin registry` dispatch arm calls `plugin_registry::run` directly.
- Moved generated JSON checking and its two tests from catalog/glue into the command component. `catalog::generated_dir` is now `pub(crate)` solely for that direct command dependency; raw catalog output behavior and `proc::spawn_inherit` remain in their current owners.
- Deleted `plugin_registry_command`, the zero-consumer dashboard registry types/builder/writer, its generate-branch console output, and the dashboard builder test. No dashboard writer or compatibility alias remains.

## Validation And Generator Evidence

- Commands manifest JSON parsing passed, and exact membership is `framework.repo.command.workflow` plus `framework.repo.command.plugin-registry`, both `command` members with their canonical responsibilities.
- Pre-generation `🤖️generated/🎛️dashboard.json` was absent. It remains absent after both generator invocations.
- The post-move stale sweep found no `DashboardRegistry`, `DashboardTask`, `DashboardAgent`, `dashboard_json_path`, `registry::build`, `registry::generate`, dashboard-output path, `plugin_registry_command`, or `check_registry` reference in CLI or command source. Glue contains only the expected direct mount/dispatch.
- `bun ./📜️script.ts verify taxonomy report --scope framework.repo.command.plugin-registry` passed: 1 component, 0 errors, 0 warnings, no findings.
- `bun ./📜️script.ts verify taxonomy enforce --scope framework.repo.command.plugin-registry` passed with the same clean result.
- `bun nx run @semio-tech/repo-cli-rs:test-quick --skip-nx-cache` passed: 17/17 tests, including both moved `plugin_registry::tests`.
- `bun nx run @semio-tech/repo-cli-rs:build --skip-nx-cache` passed.
- `bun nx run @semio-tech/repo-cli-rs:run -- plugin registry check` passed both before and after generation, printing `plugin registry catalog is fresh.`
- `bun nx run @semio-tech/repo-cli-rs:run -- plugin registry generate` passed and invoked the canonical target. `bun nx run @semio-tech/plugin-registry:generate` then passed independently; both reported refreshed output for 59 plugin crates, 58 playgrounds, and 23 framework packages.
- The canonical generator reported `.vscode/launch.json` regenerated. After generation it is `MM`; it is protected and was neither opened nor edited under this lease. No registry-generated output path appears in `git status`.
- Build/test/run emit only the pre-existing UI unnecessary-qualification and CLI `Read`/`Session.variant` warnings.
- Tracked `git diff --check` and untracked component no-index whitespace checks produced no diagnostics.

## Post-Move Fingerprints And Scope

- CLI glue SHA-256: `88ffdbd8394725a74aeca1b77eb7e68c62720b89a2b1e0953b1552536c28b2cb`.
- Commands manifest SHA-256: `3d07f1c9e303aa5c2d7797e48941f19df1b677a00652bdd44c2eddbe42d65e9b`.
- Plugin-registry command SHA-256: `dce34526a1de42be81b8412fcb987b2db602c48854a6df52efea56633df07144`.
- Changed paths: CLI glue, command collection manifest, new plugin-registry command component, and this report only. The generator-owned `.vscode/launch.json` is recorded as an observed protected output, not a source edit.

No central registrar, root/taxonomy/script/Cargo/launch source, protected library index, or unrelated command was modified. The plugin-registry command lease is complete.
