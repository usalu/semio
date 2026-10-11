# Dep-info directory audit (read-only)

Date: 2026-10-11. Scope: find the source of `failed to compute checksum, omitting it from dep-info ... Is a directory (os error 21)` in the cold release build. No code was edited, no git command was run, and no cargo build was run.

## Verdict

The culprit is the compile-time resource observer in `🧮️compiler/🦀️.rs`. Its `read_dir` registers every directory it enumerates with `proc_macro::tracked::path`. The only caller is the DSL derive's `$id` document index, which recursively walks the whole plugin or module tree for every `#[derive(MutationLeaf)]`. With `checksum-freshness` enabled, rustc tries to hash each registered directory, fails with EISDIR, logs ERROR and omits the entry.

The flow extension crates do not contain the derive themselves. They print these lines because their cargo runs compile the flow SDK and the flow artifact crate, and those crates expand `MutationLeaf`.

## Mechanism, with file:line

All paths are relative to `/Users/ueli/Documents/semio/`.

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:588` calls `mutation_leaf_referenced_documents` from `expand_mutation_leaf`, once per `#[derive(MutationLeaf)]`.
2. Same file, `:722-723`: `mutation_leaf_referenced_documents` calls `mutation_schema_document_index(&mutation_schema_search_root(payload_schema))`.
3. Same file, `:742-752`: `mutation_schema_search_root` returns the nearest ancestor directly under `🔌️plugins` or `🔨️modules`. The walk root is therefore the whole plugin or module, not the schema directory.
4. Same file, `:754-767`: `mutation_schema_document_index` calls `walk`, which calls `read_dir(directory)` on every directory at `:762` and recurses into every non-hidden, non-excluded child at `:767`. The exclusions are dot-dirs, `target`, `node_modules`, `dist`, `🗑️generated`, `📦️packages`, and example collections (`🧫️fixtures`, `🧪️fixtures`, `🧪️tests`). Non-schema directories are walked too, because the walk must find `🧬️schema` children.
5. `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs:92-99`: `read_dir` calls `(capture.track)(&path)` on the directory itself, on the error branch at `:95` and on the success branch at `:99`.
6. `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs:9`: `track` is `|path| proc_macro::tracked::path(path)`. Each directory therefore becomes a dep-info entry of the crate that expands the macro.
7. `/Users/ueli/Documents/semio/.cargo/config.toml:18`: `checksum-freshness = true` under `[unstable]`. The nightly cargo binary (`nightly-2026-07-20`) contains the literal `checksum-hash-algorithm=blake3` next to its rustc argument list, and the nightly rustc binary contains the `failed to compute checksum` text. Together these match: cargo passes the checksum flag to rustc, and rustc fails to hash a directory.

For contrast, `read` at `compiler/🦀️.rs:71` and `:75` tracks only files actually read, which is the correct behaviour. The value derive at `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs:9` uses the same `track` but only calls `read` and `read_to_string`, so it tracks files only and is not a source.

## Why the flow extension logs show it

- None of the nine extension packages (`📐️brep`, `📃️list`, `🖍️draw`, `📖️dictionary`, `📝️text`, `🧠️logic`, `🏗️bim`, `🧮️math`, `🔤️primitive`) contains a `MutationLeaf`, `DslArtifact`, `Mutations`, or `CompositeMutation` derive, and none uses `owned_json_file!`. Grep over `✏️s/🔌️plugins/🌊️flow/🧩️extensions` found none.
- Each of the nine depends on `semio-framework-os-flow` (the flow SDK, `flow_extension_sdk`). Its Cargo.lock graph includes `semio-framework-artifact-flow-flow`. That is the only consumer whose walk root is the flow framework module (`🛍️products/💻️os/🔨️modules/🌊️flow`).
- Cargo prints the output of every dependency compile under the task that triggered it, so the flow artifact crate's errors appear under the extension's nx task label (`@semio-tech/flow-extension-brep-rust`).
- The wasm directory from the report, `🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🧬️schema/📥️request`, is one of the 144 flow-module directories the walk registers. Nothing in `🕸️wasm` itself tracks paths.

## Evidence (measured)

- Compiler-resource observation files written by earlier builds: 20,010 under `.🧬semio/🦑️repo/⚡️cache/`, of which 713 contain directory rows. These rows exist only when `SEMIO_COMPILER_RESOURCE_ROOT` is set (`🏗️native-build/🟦️.ts:566`). The track call itself does not depend on that variable, so a build without it still registers the directories and still hits the error.
- 59 distinct consumer crates produce directory rows. 14,842 distinct directory paths are registered in total.
- Flow module root: 144 directories, identical to the count from `find` with the same exclusions. The flow artifact crate (`semio_framework_artifact_flow_flow`) contributes 144 entries per compile.
- Stdio plugin root (`✏️s/🔌️plugins/🗄️stdio`): 10,388 directories by `find`, 10,389 observed at maximum. This is the largest single source of lines.
- Cargo's binary dep-info (`.🧬semio/🦑️repo/⚡️cache/cargo/build`) contains no entry for these directories. That matches rustc omitting them, so dep-info grep cannot show the culprit.
- Dependency closure: brep's Cargo.lock contains 9 of the 59 consumer packages: `semio-framework-artifact-flow-flow` (144 per compile), `semio-framework-artifact-infinite-dag` (117), `semio-framework-os-kernel` (290), `semio-framework-plugin` (525), and five stdio crates (`semio-s-artifact-stdio-binary`, `-dwg`, `-semio`, `-step`, `-txt`), each up to 10,389. At the observed maxima, a brep cold build emits roughly 53,000 lines. The other eight extensions each contain 4 of the 59 consumer packages.

## Affected crates

- Workspace-wide: 59 consumer crates expand `MutationLeaf` and register directories.
- Flow extensions: 9 of 9 (`bim`, `list`, `brep`, `dictionary`, `text`, `primitive`, `draw`, `logic`, `math`) pull in these consumers transitively, through the flow SDK and the plugin crates.

## Minimal clean fix

File: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs`

- Remove the directory-level track calls in `read_dir`: `(capture.track)(&path);` on the error branch at `:95` (keep `capture.complete = false`) and on the success branch at `:99`. Keep the directory observation row, which is written to `observation.json` and not to dep-info.
- Keep file tracking in `read` (`:71`, `:75`). That is "track only the files actually read", which the code already does.

Freshness impact under the current configuration: none. Under `checksum-freshness`, rustc already omits directory entries from dep-info, so removing them changes no rebuild decision. The only mode in which they have effect is mtime freshness, which this repo has disabled.

Residual gap (exists today under checksum mode too): adding a new schema JSON file under a walked directory changes the `$id` index without invalidating any tracked file, so the derive will not rebuild. The complete fix is to have the document index come from a generated, tracked index file, following the existing `dsl-derive-rs:generate` authority-projection pattern. The derive would then read one file and never walk the tree, which also removes the 10,388-directory walk per stdio compile. That is larger than this fix and should be its own ticket.

## Not the source (checked)

- `.cargo/config.toml`: the `-Z` flags are `threads=8` and `unstable-options`. The checksum setting is `checksum-freshness = true` under `[unstable]`, not a command-line `-Z`. `binary-dep-depinfo` is not used.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts` (9 lines) is a router only, and `🟦️.ts` in that folder contains no `-Z` flag.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️builder/🦀️.rs:43-44` emits `cargo:rerun-if-changed` on a directory. That goes to cargo's rerun logic, not rustc dep-info, so it is not this error.
- `include_str!`/`include_bytes!` uses in the flow extensions all name files.

## Verification plan (not run here)

After the fix, run `cargo build -p semio-s-plugin-flow-extension-brep` in the foreground, capture the log under `🗑️generated`, and check that `rg -c "failed to compute checksum"` returns 0. Also run the derive crate's tests in `🗣️dsl/✨️derive/🧪️tests/🔬️mutation-leaf-derive`, because the walk result feeds `MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS`.

## Caveats

- No captured log of the failing cold build exists in the ticket. Line counts come from the observation files of earlier builds, and per-compile maxima are used where runs differ.
- Gating of `-Z checksum-hash-algorithm` under `checksum-freshness` is inferred. The string is confirmed in the cargo binary, but cargo's source was not available to confirm the condition.
- One ERROR line per registered directory per compile is the expected rustc behaviour. It is not counted from a rustc log.
