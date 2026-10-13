# Schema Document Index

## Cause

`.cargo/config.toml` sets `checksum-freshness = true`. `compiler_resources::read_dir` called `proc_macro::tracked::path` on every directory it listed. The only caller was the `#[derive(MutationLeaf)]` `$id` walk, which recursed the plugin or module root once per derive (59 crates, about 14.8k directories). rustc then logged `failed to compute checksum, omitting it from dep-info <dir> Is a directory (os error 21)` until the build log reached 1.78 GB. File reads stay tracked. Directory tracking was the only new-file signal, so deleting it alone would leave new schemas invisible to cargo.

## Design

The owning script `framework/products/os/modules/dsl/derive/packages/rust/script.ts` command `generate-schema-index` writes `derive/schema-document-index.json` (`semio.dsl.mutation-schema-document-index/v1`). Shape: `{ schema, roots }`. Each root key is the workspace-relative search root (plugin child, else module child, else the parent of a `schema` directory that contains `mutations`). Each value maps `$id` to a workspace-relative JSON path.

The walk matches the old derive: skip dot directories, `target`, `node_modules`, `dist`, `generated`, `packages`, and example collections (`fixtures`, `tests`) unless that collection's parent is `modules`. Symlinks are skipped. JSON is indexed only after a `schema` path segment. Sibling names are ordered by UTF-8 bytes. Duplicate `$id`s keep the first path in that order and do not fail the generator. The two `wfc/2d` config documents share an `$id` and differ in bytes; the previous `read_dir` `or_insert` was unstable across machines.

The derive reads that one file through `compiler_resources::read` (thread-local cache per workspace) and resolves paths with the authority locator. It does not call `read_dir`. `read_dir` no longer registers the directory as a compiler dependency; the observation row remains. `#[allow(dead_code)]` keeps the enumerator for callers that still want a listing.

Nx targets on `@semio-tech/dsl-derive-rs`: `generate-schema-index`, `preview-schema-index` (`schema-index preview`), `check-schema-index`, `test-schema-index-source`. Taxonomy contract `mutation-schema-document-index` owns the tracked output, lists the schema globs that cover every current root prefix (`s/plugins`, `s/modules`, `framework/modules`, `framework/products`, `mit-bestand/bericht`, `mit-bestand/demonstrator`, `semio-tech/play`), and sets `nativeConsumers` to the derive package so an Nx build regenerates the index first. Launch entries sit in `4_build` (206.1695, 206.1696) and `4_gate` (900.04735). The plugin rebuild chain step `schema-index` runs `check-schema-index` immediately after `mutation-authority`.

The orchestration lint walker is unchanged. Its comment no longer claims to be the twin of a live derive walk.

## Verification

- `schemaDocumentIndex` matches a `fast-glob` `**/schema/**/*.json` oracle on `derive/fixtures/schema-search` (3 ids, and the collection root stays empty). Log: `[DEBUG] schema document index case=domain-schema-index-excludes-examples-before-read indexed=3`.
- `cargo test -p semio-framework-os-kernel-dsl-derive schema_document_index_reads_one_file_and_ignores_unlisted_schemas` passed. The test lists one id, ignores an unlisted schema beside it, and returns an empty map for an example collection.
- `validateTaxonomy` and `validateGeneratorContractsAgainstWorkspace` report no problems for this contract. Owned-generator inventory length is 32 and matches the sorted owned contract ids. `check-schema-index` prints `schema-document-index: fresh`. The index has 126 roots and 4811 documents.
- Cold `cargo build -p semio-s-artifact-stdio-txt` (stdio txt workspace): 86 crates compiled, 0 `os error 21` / checksum lines.
- Cold `cargo build -p semio-s-plugin-flow-extension-brep`: 187 crates compiled, including `semio-framework-artifact-flow-flow` and the brep extension, 0 checksum directory lines.
- A second stdio build was fresh (0 compiling). Adding `s/plugins/stdio/schema/freshness-probe.json` and running `generate-schema-index` recompiled `semio-framework-os-kernel`, `semio-framework-plugin`, `semio-s-artifact-stdio-contract`, `semio-s-artifact-stdio-binary`, and `semio-s-artifact-stdio-txt`, still with 0 directory checksum errors. The probe was deleted and the index regenerated back to 4811 documents; the same five crates recompiled onto that restored file.
