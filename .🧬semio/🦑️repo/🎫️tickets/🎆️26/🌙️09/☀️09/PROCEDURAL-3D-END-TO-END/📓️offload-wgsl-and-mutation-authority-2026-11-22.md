# Offload WGSL inputs and mutation-authority errors

## Remote offload

`reachedFilesV1` now follows `include!` and `include_<name>!` (`include_str!`, `include_bytes!`, `include_wgsl!`, `include_spirv!`, `include_glsl!`, and a `macro_rules!` of that shape). The path may be a plain string, a raw string `r#*"..."#*`, or `concat!(env!("CARGO_MANIFEST_DIR"), "...")`. Only paths that exist are kept, and only `.rs` paths are queued.

When the shipped directory is the crate manifest and `src/` exists, `planOffloadV1` also ships non-`.rs` files from that `src/` tree, so a registry crate cannot drop a shader that an arbitrarily named macro embeds.

An extern whose spec is `semio_os_kernel_dsl_derive`, or whose resolved basename contains `libsemio_os_kernel_dsl_derive`, declines offload with `unmapped-path` (`proc-macro source authority ...`) before the dylib is stat-ed. Those units compile locally.

The cargo unit-cache source key calls the same `reachedFilesV1`.

## Mutation source authority

`symlink_metadata`, `lstat`, and `read` failures in the no-follow walks and the workspace-anchor read now name the path component and `ErrorKind`, for example `absent (NotFound): ... [NotFound]`. The existing `[NotFound]` and `[PermissionDenied]` substrings stay. `NotFound` on a taxonomy marker is still classified as missing.

`mutation_authority_raw_no_follow` is `pub(super)` and imported by the derive tests. `aggregate_metadata_failure_names_the_path_component_and_error_kind` checks a missing component and, when not root, a mode `000` directory (permissions restored in `Drop`).

## Source boundary

`semio-os-kernel-dsl-derive-expand` was refused at 265509 of 262144 bytes before the compiler. The mutation-leaf JSON fixture was compacted from 49389 bytes to 45623 bytes. Parsed JSON is unchanged. The expand tests then compiled.

## Tests

Memory free was 51% to 58% (gate 40%). Cargo used `--store=shared --configuration=prototype --parallelism=2`.

| Command | Result |
| --- | --- |
| `repo tools exec --light bun -- test` offload `tests/package.ts` | 36 pass, 0 fail, 307 `expect()` |
| `repo tools exec --light bun -- test` unit-cache sources `tests/package.ts` | 6 pass, 0 fail, 44 `expect()` |
| `repo cargo test -p semio-os-kernel-dsl-derive` | 0 pass, 0 fail (proc-macro harness has no tests); doc-tests 0; exit 0 |
| `repo cargo test -p semio-os-kernel-dsl-derive-expand` | 22 pass, 1 fail, 0 ignored; doc-tests 0 pass, 0 fail |

The expand failure is `validates_mutation_source_authority_fixture`, case `dependency-scope-anchor`: the fixture expects acceptance and the run returns `workspace package.json declaration lacks paired project.json`. That missing-project arm is unchanged from HEAD. The same assertion fails alone with `--test-threads=1`. `raw_no_follow_refusal_names_the_path_component_and_io_error_kind` and `aggregate_metadata_failure_names_the_path_component_and_error_kind` passed.
