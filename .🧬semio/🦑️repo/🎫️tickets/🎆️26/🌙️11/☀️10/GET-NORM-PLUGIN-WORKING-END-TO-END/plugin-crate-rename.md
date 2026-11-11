# Plugin Crate Rename Alias Removal

- Removed every `semio-framework-plugin-<x>` dependency alias key (with `package = "semio-plugin-<x>"`); keys are now `semio-plugin-<x>`.
- Rust paths `semio_framework_plugin_<x>` became `semio_plugin_<x>` (including composition_emission, instance_history, instance_render, instance_typed_ops, time_travel_turns, tool_run_turns, tests_NN).
- Three crates still carried the old package name and were renamed: `semio-plugin-reactor-staged-forms`, `semio-plugin-native-identity-tests`, `semio-plugin-tests-16`.
- Files changed: 223 (Cargo.toml, Rust, project.json, build-owners.json, docs/scripts); listed in `🗑️generated/rename-files.log`.
- `Cargo.lock` is left to cargo to regenerate.
- No `semio_framework_plugin_` / `semio-framework-plugin-` remain outside lock files, target, .cache and tickets.

## Redundant package attribute removal and metadata verification

- Scope: root, framework, ✏️s, 🌎️hub Cargo.toml files (excluding target/.cache and .🧬semio).
- Matched 844 files / 4405 lines where key equals package value (task estimate was ~65); removed only the `package = "..."` attribute.
- Metadata (`repo cargo --store=shared --configuration=prototype --parallelism=4 -- metadata --format-version 1 --no-deps`): root exit 0, framework exit 0, hub exit 0.
- Hub Cargo.lock `semio-framework-plugin` count after run: 0.
- Logs: 🗑️generated/pkgkey-root-metadata.log, pkgkey-framework-metadata.log, pkgkey-hub-metadata.log, pkgkey-exit-codes.log.
