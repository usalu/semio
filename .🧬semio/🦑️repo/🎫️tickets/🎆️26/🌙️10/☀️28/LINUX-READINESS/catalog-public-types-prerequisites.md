# Catalog Public Types Prerequisites

Actual semantic receipt `🗑️generated/catalog-public-types.json` covers 5273 files and reports 31 diagnostics. All diagnostics belong to the framework shell dependency: 18 TS2305 missing exports and 10 TS2459 private exports in the public shell façade, two TS2307 missing-module diagnostics in the reducer (lines 18–19), and one TS2322 exhaustiveness failure at line 553. These are consistent with one missing generated shell type module; no independent catalog façade root error appears in this receipt. Resolving the missing authority projection must precede interpreting the downstream export and never-type diagnostics. This audit did not rerun semantic checking.

## Authentic Producer

Missing file: `🧰️framework/🔨️modules/🐚️shell/🤖️generated/🟦️.ts`. The handwritten reducer imports `./🤖️generated/🟦️.js`; ordinary TypeScript extension substitution resolves that specifier to the generated `.ts` file. Do not create a `.js` stub or hand-maintained replacement types.

The owning manifest is `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/📋️project.json`, project `@semio-tech/framework-os-shell-rs`. Its `typegen` target explicitly declares exactly the missing generated directory as output and `schemaModule` inputs. Canonical producer command:

```sh
SEMIO_LANGUAGE=en repo run @semio-tech/framework-os-shell-rs:typegen
```

The router delegates to `runTypegenScript` in `🖥️shell/🧪️tests/🔬️rust-package-commands/🟦️.ts:64`. It requires all three owned schema files, computes the framework shell output path, then invokes admitted Cargo test `schema::tests::exports_typescript_bindings` with the `typegen` feature and `SEMIO_TYPEGEN_OUT` bound to that exact output. The Rust schema test validates the registry and renders it before writing. This command genuinely compiles/runs Rust and publishes generated output; it is not a lightweight read-only check. Existing owned Cargo tooling, capacity admission, received Original and generated-output custody remain prerequisites.

Then the existing cross-language authority check is:

```sh
SEMIO_LANGUAGE=en repo run @semio-tech/framework-os-shell-rs:schema-check
```

That command compares owned draft-07 JSON public `$defs`, Rust registry export IDs, rendered mirror exports and TypeScript parsers, then runs the actual Rust registry/schema gate. It does not generate an absent mirror and currently has no `dependsOn: typegen`. `preview-generated` is the existing bounded captured-output alternative for publication machinery; it emits the shell-typegen projection and does not itself install it.

## Durable Target Dependency Gap

The inspected specific catalog `test-catalog-exports` depends on its own `generate` only; the latter produces catalog declarations, not framework shell types. The separate `@semio-tech/framework-os-shell:test` default inputs reference the rendered mirror but its manifest also declares no producer dependency. Neither reference is a zero-touch producer edge. Consumers needing the shell reducer/type closure should acquire the owning shell `typegen` prerequisite through the build graph, rather than patching façade exports or casting the exhaustiveness check. Preserve acyclic ownership: shell typegen owns framework schema projection, while specific catalog generate owns the specific catalog output.

After authentic projection and schema agreement, rerun the same scoped semantic gate that produced the receipt. Only that actual result can establish whether all 31 cascading diagnostics disappear. No producer, tests, compiler, Native tool, process or lease operation was launched by this audit.
