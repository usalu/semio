# Split of `semio-plugin-decl-app` (2026-11-24)

## Finding
- Admission failed at 264,887 bytes against 262,144 (28 files). The measure is the sum of the `.rs` files reachable from the crate's lib root through `#[path]` modules, including `../👷️job`.
- Another agent shrank `🗂️natural-file-and-media-artifacts/🦀️.rs` from 19,365 to 16,317 bytes. That left 261,839 bytes, 305 under the limit.
- The same agent also raised the crate's `maxSourceBytes` in `📦️packages/🦀️rust/Cargo.toml` to 393,216. I restored it to 262,144.
- I split anyway. A 305-byte margin is not a real fix.

## Boundary chosen
The natural file and media artifact contract moved to a new crate, `semio-plugin-decl-natural-file-media`. It holds:
- `NaturalFileCodec` and the natural file decode cursors, steps and owner.
- `document_load_emit` and `document_reference_load_emit`.
- The mounted natural file actions.
- `MediaArtifactDescriptor`, `MediaArtifactError`, `MediaArtifactOperationFailure` and `MediaConsumption`.
- The `DOCUMENT_LOAD_*` and `MEDIA_SCHEMA_MISMATCH` codes, `document_load_fault`, `whole_document_archive` and `media_artifact_fault`.

It has no dependency on `ArtifactApp`, `tool_contract` or the other lifted modules. It only needs `semio-plugin-decl-media`, `semio-plugin-decl-emission` and framework crates. The dependency direction is `decl-app -> decl-natural-file-media`, so it stays acyclic.

The other candidates (`👷️job`, `lifted/*`) all reference `ArtifactApp` or `crate::tool_contract`. Moving them would have created a cycle.

`decl-app` reexports the whole API with `pub use semio_plugin_decl_natural_file_media::*;`. Its public API is unchanged and no code was edited.

## Sizes (sum of reachable `.rs`)
| crate | files | bytes | headroom to 262,144 |
|---|---|---|---|
| `semio-plugin-decl-app` | 27 | 245,425 | 16,719 |
| `semio-plugin-decl-natural-file-media` | 1 | 19,787 | 242,357 |

The natural-file source is 19,787 bytes because it now carries its own import header. Concurrent edits by the other agent can still change both numbers.

## Files
- New: `🧩️app/🗂️natural-file-and-media-artifacts/📦️packages/🦀️rust/Cargo.toml` and `📋️project.json`. The crate's lib root is the existing `🗂️natural-file-and-media-artifacts/🦀️.rs`.
- Edited: `🗂️natural-file-and-media-artifacts/🦀️.rs`. `use super::*` became explicit imports plus the `protocol` and `store` extern crate aliases. `crate::media_operation_refusal` became `semio_plugin_decl_media::media_operation_refusal`.
- Edited: `🧩️app/🦀️.rs` (the `mod` block became a `pub use`) and `🧩️app/📦️packages/🦀️rust/Cargo.toml` (new dependency, `maxSourceBytes` back to 262144).
- Edited: root `Cargo.toml` (workspace member) and `📋️build-owners.json` (`@semio-tech/plugin-decl-natural-file-media-rs`).
- Edited fixtures: `native-build/paths/tests/fixtures/🔣️.json` (admitted unit) and `wgpu/compiler/wasm/tests/fixtures/🔣️.json` (domain package).
- Lock files: root `Cargo.lock` was updated by cargo. A script added the new `[[package]]` and the `decl-app` dependency entry to 146 more `Cargo.lock` files that list `semio-plugin-decl-app`, including `🌎️hub/Cargo.lock`.
- 20 `✏️s/**/Cargo.lock` files were not touched: they already lack `semio-plugin-decl-media`, so they were stale before this change. They need a normal `cargo metadata` refresh when someone builds them.
- The script is `/private/tmp/claude-501/-Users-ueli-Documents-semio/3d55bb15-005d-4498-88da-1487da3fa74c/scratchpad/lock.py`.

## Verification
- `repo cargo --store=shared --configuration=prototype -- check -p semio-plugin-decl-natural-file-media` passes.
- `repo cargo --store=shared --configuration=prototype -- check -p semio-plugin-decl-app -p semio-plugin-instance-app` finishes with no error.
- I did not run the Draw component build or any tests.
