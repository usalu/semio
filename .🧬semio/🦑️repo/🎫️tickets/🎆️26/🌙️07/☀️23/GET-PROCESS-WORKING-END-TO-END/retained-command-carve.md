# Retained Command Carve

`semio-plugin-retained-command` exceeded the 256 KiB source boundary (262349 of 262144 bytes, `cfg(test)` files excluded). It is now two crates along its module graph. No source file moved.

## Seam
The module graph is a DAG with two independent halves. `typed_operation` (`🔄️full-operation`, including `mounted_page`, `ui_copy`, `view_retirement`, `original_emission` and the `include!`d `🪟️mounted/♻️frontier`) imports nothing from `admission`, `factory`, `producer`, `checkpoint_cursor`, `retained_command_job` or `reserved_route_capacity_lift`, and the job half does not name `typed_operation`. So the halves split without an edge between them. `semio-plugin-result-page` and `semio-plugin-retained-work` were already carved earlier.

| Crate | Modules | Bytes (no test files) |
|---|---|---|
| `semio-plugin-retained-command` | `admission`, `factory`, `producer` (+`custody`), `checkpoint_cursor`, `retained_command_job`, `raw_allocation_close_oracle`, `reserved_route_capacity_lift` | about 99 KB |
| `semio-plugin-retained-command-operation` (new) | `typed_operation` | about 163 KB |

Both crates sit below 64% of the limit.

## Changes
- New crate root `🔄️full-operation/🧱️owner/🦀️.rs` (`#[path]`-mounts `../🦀️.rs` as `typed_operation`, keeps the `store` and `protocol` kernel aliases), with `Cargo.toml` and `📋️project.json` under `🔄️full-operation/🧱️owner/📦️packages/🦀️rust`. It carries the `artifact-app-testing` feature and the `mounted-ui-copy` test target.
- Old crate: `typed_operation` mount removed from `🦀️.rs`; the `cfg(test)` mount of `🧪️tests/🔬️unit/🦀️.rs` was removed too, because those laws name `crate::typed_operation` and are already hosted by `semio-plugin-tests-20`, whose root now also re-exports `typed_operation` from the new crate. Eleven dependencies only the operation half used were dropped from its `Cargo.toml`.
- 39 `.rs` files: `semio_plugin_retained_command::typed_operation` became `semio_plugin_retained_command_operation::typed_operation`.
- `🪝️mounts/👥️wp3-owners/🦀️.rs` (`typed_operation` facet) points at the new crate; `plugin/🦀️.rs` re-exports `semio_plugin_retained_command_operation`; the plugin `artifact-app-testing` feature forwards to it.
- 37 dependent `Cargo.toml` files gained the new dependency (same `features` as their old line). Root and `🧰️framework` workspace `members` list the new path. There is no `[workspace.dependencies]` entry for the old crate, so none was added for the new one.

## Hand-pinned lists
`📋️feature-consumers.json`, `📋️surface-fixture*.json`, the native-build paths fixture and the wgpu fixture name `semio-plugin-retained-command` as a dependency record; they were not regenerated for the new crate (same status as in the ownership carve).
