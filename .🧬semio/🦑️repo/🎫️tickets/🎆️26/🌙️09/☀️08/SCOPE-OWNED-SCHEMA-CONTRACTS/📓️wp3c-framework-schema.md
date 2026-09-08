# WP3c — `framework.schema` facet, the os-kernel-free registry crate, the Go emitter path

Partition: `🧰️framework/🔨️modules/🧬️schema/**` plus the two root `Cargo.toml` lines the new crate needs.
Continues `📓️wp2-rust-schema-registry.md` and `📓️wp3b-validator-keywords.md`. Closes cross-partition rows
**68 (framework side)**, **70**, **76**.

A predecessor (W3c, stopped mid-task) had already landed the row-68 `🔣️.json` facet, the `🟦️.ts` twins,
`ValidationDiagnostic`, `SchemaExportEntries`, the `framework.schema` registration constants and the row-70
emitter path edit. This packet completed row 76, found and fixed the breakage row 68 caused in this module,
and added the missing verification.

## 1. Files changed

| File | Change |
|---|---|
| `🧬️schema/📇️registry/🦀️.rs` | **new** — the whole `(scope id, export id, format id)` contract, **zero dependencies** |
| `🧬️schema/📇️registry/📦️packages/🦀️rust/Cargo.toml` | **new** — `semio-framework-schema-registry`, one `[[test]]` (ASCII `name`, emoji `path`) |
| `🧬️schema/📇️registry/📦️packages/🦀️rust/🦀️.rs` | **new** — package glue (`#[path]` to the owner component) |
| `🧬️schema/📇️registry/📦️packages/🦀️rust/📋️project.json` | **new** — `@semio-tech/schema-registry-rs`, `test`/`test-quick`/`test-long`/`test-exhaustive`, mirrors the `✨️derive` sibling |
| `🧬️schema/📇️registry/📦️packages/🦀️rust/📜️script.ts` | **new** — `bun ./📜️script.ts test` → `runCargoTestBudgeted(["semio-framework-schema-registry"])` |
| `🧬️schema/📇️registry/🧪️tests/📇️catalog/🦀️.rs` | **new** — process-wide catalog seen from a crate that links **only** the registry crate |
| `🧬️schema/⚛️component.rs` | `🔖️SchemaExportResolution` reduced to what needs the validator/kernel; explicit re-exports of the registry crate; `ArtifactSchemaDescriptor::facet_leaves()`; `register_artifact_schema_descriptor` mirrors its facets into the registry; new/rewritten tests |
| `🧬️schema/📦️packages/🦀️rust/Cargo.toml` | `semio-framework-schema-registry` path dependency |
| `🧬️schema/📦️packages/🦀️rust/📜️script.ts` | entity-catalog source path → `🔣️entity-kinds.json` (see §4); Go target path already at `⌨️cli/🐹️entity_kinds.g.go` |
| `🧬️schema/🔣️entity-kinds.json` | **new** — the 58-entry entity catalog, byte-identical to what `🔣️.json` held at HEAD (§4) |
| `🧬️schema/🧪️vitest.config.ts` | **new** — module-local vitest project, `include: ["🧪️tests/**/🟦️.ts"]` |
| `🧬️schema/🧪️tests/📤️schema-export-entries/🟦️.ts` | **new** — ajv oracle for the real runtime dump against `🔣️.json` |
| `🧬️schema/🧪️tests/📤️schema-export-entries/🦀️.rs` | dump now rendered by `SchemaExportEntries::to_json()`, validated by the owned validator, and compared byte-for-byte with the committed fixture |
| `🧬️schema/🧫️fixtures/📤️schema-export-entries-dump.json` | **new** — the real runtime dump, produced by the Rust test, read by the ajv spec |
| root `Cargo.toml` | `members` line + `[workspace.dependencies] semio-framework-schema-registry` (the framework worker's `ui-contract` already uses `{ workspace = true }`, so the second line was load-bearing) |

A concurrent repo sweep moved both crates' `#[cfg(test)] mod tests` bodies to
`🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs` and `🧬️schema/📇️registry/🧪️tests/🔬️unit/🦀️.rs`, leaving
`#[cfg(test)] #[path = …] mod tests;` behind. Both crates were re-tested after that sweep and are green (§6).

## 2. Row 76 — the os-kernel-free leaf crate

### 2.1 Crate layout

```
🧰️framework/🔨️modules/🧬️schema/📇️registry/
  🦀️.rs                                  owner component (taxonomy `componentFileKinds.🦀️rust`)
  📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}
  🧪️tests/📇️catalog/🦀️.rs                [[test]] name = "schema-registry-catalog"
  🧪️tests/🔬️unit/🦀️.rs                   unit tests (moved there by the repo sweep)
```

Mirrors the `✨️derive` sibling exactly (owner component at the module root, package glue mounting it with
`#[path]`, `📋️project.json` + `📜️script.ts` in the package). No root registration was added, because
`nx.json`'s repo plugin already declares `include: ["**/📋️project.json", "**/Cargo.toml", …]`.

⚠️ **That last sentence is inferred from `nx.json`, not observed.** `bun nx show project
@semio-tech/schema-registry-rs --json` was attempted and **failed**: `NX   The daemon timed out while
processing REQUEST_PROJECT_GRAPH` after 600 s — the graph was never computed, so the run says nothing about
this project either way (the same timeout is a known repo-wide condition, not specific to this change). The
crate itself is fully verified through `cargo` (§6.1–6.2); only the **nx target wiring** is unproven. Whoever
picks up R-4 should run `bun nx run @semio-tech/schema-registry-rs:test` once against a warm daemon.

### 2.2 What moved, what stayed

| Moved to `semio-framework-schema-registry` | Stayed in `semio-framework-schema` |
|---|---|
| `SchemaFormat`, `FacetLeaves`, `SchemaExport`, `ScopeSchemaExports`, `RESERVED_FACET_EXPORT_IDS` | `SchemaError`, `ValidationDiagnostic`, `OwnedJsonSchemaValidator` |
| `SchemaExportRegistryError`, `SchemaResolveError`, `SchemaExportEntry`, `SchemaExportEntries` | `SchemaBoundaryError` |
| `SchemaExportRegistry` (minus structural validation) | `structural_validator_in(&SchemaExportRegistry, scope, export)`, `structural_validator_for(scope, export)` |
| the two `OnceLock<Mutex<…>>` catalogs, `register_scope_schema_exports`, `register_scope_facet_leaves`, `scope_schema_exports_registered`, `scope_schema_facets_registered`, `with_schema_export_registry`, `resolve_schema_export`, `schema_export_catalog_entries` | `ArtifactSchemaDescriptor` (+ `SchemaVersion`, the kernel round-trip, `register_artifact_schema_descriptor`), `FRAMEWORK_SCHEMA_*` |

`semio-framework-schema` re-exports every moved item **explicitly** (never `pub use …::*`), so
`semio_framework_schema::{SchemaFormat, register_scope_schema_exports, …}` keeps working unchanged — the
`framework.interaction` registration in `🕹️interaction/🧬️schema/🦀️.rs` needed no edit.

### 2.3 Two accepted API changes (report, not silent)

1. **`SchemaExportRegistry::register_descriptor(ArtifactSchemaDescriptor)` →
   `register_facet_leaves(scope, [FacetLeaves; 4])`.** `ArtifactSchemaDescriptor` carries `SchemaVersion`
   methods that need `pack` (content hash + canonical JSON), and `pack` pulls
   `semio-framework-replication` + `semio-framework-async`, so it cannot live in a zero-dependency crate.
   The registry stores the descriptor's *resolution projection* instead — the four leaves positionally keyed
   by `RESERVED_FACET_EXPORT_IDS` — and `ArtifactSchemaDescriptor::facet_leaves()` is the one conversion.
   No duplicated struct.
2. **The fixed-facet side is no longer read back out of the kernel catalog.** `with_schema_export_registry`
   previously called `with_kernel_artifact_schema_catalog` + `descriptor_from_kernel`. It now reads the
   registry crate's own facet catalog, which `register_artifact_schema_descriptor` writes alongside the
   kernel one. This is lossless: `register_kernel_artifact_schema_descriptor` has exactly **one** caller in
   the tree (`⚛️component.rs:351` — verified by grep), so the two catalogs cannot diverge. Conflict handling
   is unchanged (the mirror's error is ignored exactly as `let _ = registry.register_descriptor(…)` ignored
   it before). Proven by `artifact_schema_descriptor_registration_mirrors_its_facets_into_the_export_registry`.

### 2.4 `SchemaExportEntries` sort order corrected

`from_catalog` sorted by the `SchemaFormat` **enum** discriminant (rust, typescript, graphql, jsonschema,
protobuf), which contradicts the `📓️wp3b` §5 contract "sorted … by that triple" where `format` is the ascii
id the dump renders. Changed to `sort_unstable_by_key(|e| (e.scope, e.export, e.format.id()))`, restoring the
pre-`SchemaExportEntries` behaviour of the hand-rolled renderer. The ajv spec asserts the rendered order is
lexicographic, so this is now gated from both sides.

`to_json` no longer routes the `generator` string through `pack::json`; the registry crate carries an owned
`escape_json_string` (the only serialization it performs), covered by
`entries_dump_renders_the_declared_contract_and_escapes_the_generator`.

### 2.5 Dependency proof

```
$ cargo tree -p semio-framework-schema-registry
semio-framework-schema-registry v0.1.0 (…/🧬️schema/📇️registry/📦️packages/🦀️rust)
```

A single node: not just os-kernel-free — **dependency-free**. `cargo tree … | grep -c os-kernel` → `0`.
Full log `🗑️generated/wp3c-cargo-tree.txt`.

`🧪️tests/📇️catalog/🦀️.rs` is the runtime half of that proof: it links only the registry crate, registers
`framework.ui.contract`'s exports, resolves them in all declared formats, asserts `FormatAbsent` for the two
absent ones, and renders the dump — exactly the sequence a dependency-restricted scope performs.

### 2.6 `framework.ui.contract` — already registered by the framework worker

The exact call requested in `📓️wp4b-framework-modules.md` §7.3 is **already on disk**, written by the
framework worker against this crate while it was being built
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs`, region `🔖️ScopeSchemaExports`):

```rust
use semio_framework_schema_registry::{register_scope_schema_exports, FacetLeaves, SchemaExport, ScopeSchemaExports};
const LEAVES: FacetLeaves = FacetLeaves { rust: include_str!("🦀️.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" };
const EXPORTS: [SchemaExport; 2] = [SchemaExport { id: "ConformanceCatalogFixture", leaves: LEAVES }, SchemaExport { id: "ContractFixture", leaves: LEAVES }];
pub fn register_scope_exports() {
    register_scope_schema_exports(ScopeSchemaExports { scope: "framework.ui.contract", exports: &EXPORTS }).expect("framework.ui.contract scope schema exports");
}
```

Its `📦️packages/🦀️rust/Cargo.toml:42` already says `semio-framework-schema-registry = { workspace = true }`
— which is why the root `[workspace.dependencies]` line, not just the `members` line, had to land. That crate
is outside this partition and outside this packet's cargo scope, so it was **not** compiled here; row 40 for
`framework.ui.contract` is unblocked and needs one `cargo check -p semio-framework-ui-contract` from its owner.

## 3. Row 68 — the `framework.schema` facet

`🧬️schema/🔣️.json` is now the module's real draft-07 facet
(`$id https://semio.tech/schema/framework/schema/schema.json`, root `$ref: #/$defs/SchemaExportEntries`).

### Export table — `framework.schema`

| Export id | `$defs` | Rust | TypeScript | JSON Schema | GraphQL / proto |
|---|---|---|---|---|---|
| `SchemaFormat` | ✔ | `📇️registry/🦀️.rs` | `🟦️.ts` | `🔣️.json` | absent |
| `FacetLeaves` | ✔ | `📇️registry/🦀️.rs` | `🟦️.ts` | `🔣️.json` | absent |
| `SchemaExport` | ✔ | `📇️registry/🦀️.rs` | `🟦️.ts` | `🔣️.json` | absent |
| `ScopeSchemaExports` | ✔ | `📇️registry/🦀️.rs` | `🟦️.ts` | `🔣️.json` | absent |
| `SchemaExportEntry` | ✔ | `📇️registry/🦀️.rs` | `🟦️.ts` | `🔣️.json` | absent |
| `SchemaExportEntries` | ✔ | `📇️registry/🦀️.rs` | `🟦️.ts` | `🔣️.json` | absent |
| `SchemaResolveError` | ✔ | `📇️registry/🦀️.rs` | `🟦️.ts` | `🔣️.json` | absent |
| `ValidationDiagnostic` | ✔ | `⚛️component.rs` | `🟦️.ts` | `🔣️.json` | absent |

The rust leaf is now **per export** rather than one blanket `⚛️component.rs`: the seven resolution types are
carried by the registry crate's file, `ValidationDiagnostic` by the validator-side file. GraphQL and proto
leaves do not exist in this module, so all ten `(export, format)` pairs resolve to `FormatAbsent` — asserted.

`framework_schema_exports_match_the_modules_json_schema_defs_and_resolve` parses `🔣️.json` at test time and
asserts **set equality** between its `$defs` keys and `FRAMEWORK_SCHEMA_EXPORTS`, so a `$defs` key added
without a registration (or the reverse) fails the build.

### The dump, checked three ways

`🧫️fixtures/📤️schema-export-entries-dump.json` is a **real** dump, written by
`cargo test -p semio-framework-schema --test schema-export-entries` with
`SEMIO_SCHEMA_EXPORT_ENTRIES_OUT` pointing at it (27 entries: `framework.schema` × 8 exports × 3 formats,
plus the test's own `framework.schema.entries` × 3). It is validated by:

1. the **owned validator** — `structural_validator_for("framework.schema", "SchemaExportEntries")` inside the
   Rust test, plus three negative instances (wrong `contractId`, missing `entries`, unknown `format`);
2. **ajv** — `🧪️tests/📤️schema-export-entries/🟦️.ts`, same document, same facet, six negative instances
   (unknown contract id, missing member, unknown format, extra property, non-conforming scope id, empty
   generator);
3. a **byte-equality** assertion against the committed fixture, so the export table cannot drift silently.

## 4. Row 68 fallout this packet had to fix — the entity catalog had a reader

`📓️wp3b-validator-keywords.md` §7.7 recorded that `🧰️framework/🔨️modules/🧬️schema/🔣️.json` (the 58-entry
technology/bundle/folder/file catalog) had "**no** path reader anywhere in the tree". That is wrong. The
reader is in this very module:

```
🧬️schema/📦️packages/🦀️rust/📜️script.ts:23   readEntityKinds() → join(ownerRoot, "🔣️.json")
```

It is the single source of the `schema-entity-catalog` generator, whose three outputs are
`🤖️generated/🟦️entity-kinds.ts`, `🤖️generated.rs` (`include!`d by `⚛️component.rs` region
`🔖️EntityCatalog`) and the Go CLI file. With the slot taken over by the schema facet, `generate`,
`preview-generated` and `check` all died with `TypeError: kinds.map is not a function`.

Fixed inside the partition: the data was restored **byte-identical from HEAD** to
`🧬️schema/🔣️entity-kinds.json` (taxonomy `🔣️` = `.json` file kind; same `🔣️<slug>.json` shape as
`📚️library/🔣️taxonomy.json` and `🔣️schema-catalog.json`) and the emitter reads it there. The schema slot
stays the facet. Verified in §6.3.

This does not contradict row 68's *other* half — the vscode duplicate — but it does invalidate its premise;
see request R-1.

## 5. Row 70 — Go emitter target

`generatedTargets()` writes
`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go` (the emoji
`⚡️implementations/🐹️go/` directory Go cannot compile is gone from the emitter). The target file exists on
disk at the new path; the old path does not. Verified with the package's own `check` and
`preview-generated` (§6.3).

## 6. Verification — real output

### 6.1 `cargo test -p semio-framework-schema-registry -p semio-framework-schema`

`CARGO_TARGET_DIR=…/scratchpad/target-w3 RUSTC_WRAPPER=""`, full log `🗑️generated/wp3c-cargo-test.txt`.

```
running 25 tests
test component::tests::artifact_schema_descriptor_registration_mirrors_its_facets_into_the_export_registry ... ok
test component::tests::framework_schema_exports_match_the_modules_json_schema_defs_and_resolve ... ok
test component::tests::framework_schema_facet_validates_a_real_runtime_entries_dump ... ok
test component::tests::validation_diagnostics_round_trip_through_the_validation_error ... ok
test component::tests::structural_validator_resolves_cross_scope_refs_by_document_id ... ok
test component::tests::scope_schema_exports_register_into_the_os_wide_catalog ... ok
… (19 pre-existing tests) …
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running ../../🧪️tests/📤️schema-export-entries/🦀️.rs
running 1 test
test schema_export_catalog_entries_dump_is_sorted_and_matches_the_declared_contract ... ok
test result: ok. 1 passed; 0 failed; …

     Running ../../🧪️tests/🧩️module-compile/🦀️.rs
running 1 test
test every_draft07_schema_module_compiles_through_the_owned_validator ... ok
test result: ok. 1 passed; 0 failed; … finished in 16.09s

     Running unittests 🦀️.rs (semio_framework_schema_registry)
running 6 tests
test component::tests::entries_dump_renders_the_declared_contract_and_escapes_the_generator ... ok
test component::tests::schema_export_registration_is_duplicate_safe_and_conflict_fatal ... ok
test component::tests::schema_export_registration_rejects_a_duplicate_export_id_inside_one_scope ... ok
test component::tests::schema_export_registry_reports_every_scope_with_its_exports ... ok
test component::tests::schema_export_resolution_distinguishes_every_failure ... ok
test component::tests::schema_format_ids_mirror_the_taxonomy_schema_formats_keys ... ok
test result: ok. 6 passed; 0 failed; …

     Running ../../🧪️tests/📇️catalog/🦀️.rs
running 1 test
test a_dependency_restricted_scope_registers_resolves_and_dumps_through_the_registry_crate_alone ... ok
test result: ok. 1 passed; 0 failed; …
```

34 tests, 0 failures.

### 6.2 `cargo clippy -p semio-framework-schema-registry -p semio-framework-schema --all-targets`

```
warning: `semio-framework-replication` (lib) generated 2 warnings
warning: `semio-framework-os-kernel` (lib) generated 1 warning
    Finished `dev` profile [unoptimized] target(s) in 4.98s
```

**Zero** warnings attributed to either crate. The three remaining are pre-existing `large_enum_variant`
warnings in peer crates outside this partition. One clippy finding **was** attributed to this partition and
is fixed: `ValidationDiagnostic::json_pointer` used consecutive `str::replace` calls →
`.replace(['.', '['], "/")`.

### 6.3 Entity-catalog emitter (row 70 + §4)

```
$ bun ./📜️script.ts check
entity catalog is fresh (58 entity kinds).

$ bun ./📜️script.ts preview-generated
schema-entity-catalog 1 staleRemovals: []
  🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs                                    9996
  🧰️framework/🔨️modules/🧬️schema/🤖️generated/🟦️entity-kinds.ts                    8176
  🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go      8232
```

`check` is green against the three committed outputs, so the emitter and the tracked Go file agree.

### 6.4 ajv, through the new module-local vitest config

`bunx vitest run --config 🧰️framework/🔨️modules/🧬️schema/🧪️vitest.config.ts --reporter=verbose`
(full log `🗑️generated/wp3c-vitest.txt`)

```
 ✓ 🧪️tests/📤️schema-export-entries/🟦️.ts > … > declares the draft-07 dialect and its own $id
 ✓ 🧪️tests/📤️schema-export-entries/🟦️.ts > … > accepts the real runtime dump
 ✓ 🧪️tests/📤️schema-export-entries/🟦️.ts > … > keeps the dump sorted and deduplicated by (scope, export, format)
 ✓ 🧪️tests/📤️schema-export-entries/🟦️.ts > … > rejects an unknown contract id, a missing member, an unknown format and an unknown property
 ✓ 🧪️tests/✅️draft07-oracle/🟦️.ts > draft-07 structural validation vectors > … (17 cases)

 Test Files  2 passed (2)
      Tests  21 passed (21)
```

### 6.5 Repo-wide module compile, after the facet landed

```
[schema-module-compile] modules=3186 compiled=3158 draft07Failures=12 otherDialectFailures=16 collidingIds=1
```

Was `modules=3161 compiled=3108 draft07Failures=33 otherDialectFailures=20` in `📓️wp3b` §4. This module's own
`🔣️.json` no longer appears as `<undeclared>` / `$: schema must be an object or boolean` (wp3b §6 open item),
and the 32 path-style `$ref`s are down to 11 as peers land row-65/66 fixes. Still open and owned elsewhere:
`📇️directory` `discriminator` (1), `🖥️host/🧪️tests/🕸️media-projection` `prefixItems` (1, **new** since wp3b),
`🌊️flow/🌿️vcs` mutation leaves + diff test `$ref`s (8), `🌍️gis` and `🗄️stdio` lowercase local `$ref`s
(`#/$defs/field`, `#/$defs/path`, 2).

### 6.6 TypeScript

```
$ bunx tsc --noEmit --skipLibCheck --strict --resolveJsonModule --types node \
    🧬️schema/🧪️vitest.config.ts 🧬️schema/🧪️tests/📤️schema-export-entries/🟦️.ts 🧬️schema/🟦️.ts
(no output)
```

## 7. Cross-partition requests

| # | Owner | Request |
|---|---|---|
| R-1 | coordinator + **W9c repo client** | **Row 68's ownership half rests on a wrong premise.** The catalog in `🧬️schema/🔣️.json` had a reader all along — `🧬️schema/📦️packages/🦀️rust/📜️script.ts:23`, the `schema-entity-catalog` generator that emits the TS, Rust **and** Go projections. It is a framework-owned single source (`package.json`: "single-source entity catalog codegen … for CLI, VS Code and Rust consumers"), not vscode data. It now lives at `🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json`. The vscode copy in `🧩️vscode/📦️packages/🟦️typescript/🟦️.ts` should become a **consumer** of `🤖️generated/🟦️entity-kinds.ts` (or a fourth emitter target), not the owner. If the coordinator still wants vscode to own it, this emitter has to read across the framework→product boundary, which inverts the layering — say so explicitly and I will not guess. |
| R-2 | **W2c tooling** (taxonomy) | `🔣️taxonomy.json` `generatorContracts.schema-entity-catalog` needs two edits: `inputPatterns[2]` `"🧰️framework/🔨️modules/🧬️schema/🔣️.json"` → `"🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json"`, and `outputRoots[2].path` `"…/💻️client/⌨️cli/⚡️implementations/🐹️go/🐹️entity_kinds.g.go"` → `"…/💻️client/⌨️cli/🐹️entity_kinds.g.go"` (the second is already row 71's item; the first is new and blocks nx input hashing today). |
| R-3 | **root `📜️script.ts` worker** | `schema test`'s `oracleArguments()` (`:17452-17456`) writes a throwaway vitest config into `tmpdir()` from `SCHEMA_DRAFT07_ORACLE_SPEC` (`:17291`). Replace both with the tracked module config: `private oracleArguments(): string[] { return ["vitest", "run", "--config", join(this.root, "🧰️framework/🔨️modules/🧬️schema/🧪️vitest.config.ts")]; }` and delete `SCHEMA_DRAFT07_ORACLE_SPEC` plus the now-unused `tmpdir`/`writeFileSync` there. The module config's `include` is `🧪️tests/**/🟦️.ts`, so `schema test` also picks up the new `📤️schema-export-entries` ajv oracle with no further edit. |
| R-4 | **root `📜️script.ts` / `📋️project.json` / `.vscode/launch.json` worker** | Register the new crate's target. `.vscode/launch.json`, following the existing `🧬️schema` grouping/naming: `{ "name": "📇️registry schema test", "type": "node", "request": "launch", "runtimeExecutable": "bun", "runtimeArgs": ["nx", "run", "@semio-tech/schema-registry-rs:test"], "cwd": "${workspaceFolder}" }`. Root `package.json` should gain a `schema-registry:test` script calling the same nx target. `nx.json`'s repo plugin include (`**/📋️project.json`) should discover the project with no `nx.json` edit, but **this was not verified** — `nx show project` died on a 600 s daemon `REQUEST_PROJECT_GRAPH` timeout (§2.1). Run `bun nx run @semio-tech/schema-registry-rs:test` once before closing this row. |
| R-5 | **W5b os / boot-path owner** | `register_scope_schema_exports` is still not idempotent across *different* declarations, so each scope needs exactly one boot-time call. Three now exist and none is called from production: `semio_framework::interaction::schema::register_scope_exports()`, `semio_framework_ui_contract::schema_metadata::register_scope_exports()`, and `semio_framework_schema::register_framework_schema_exports()`. They belong next to wherever the plugin assembly calls `register_artifact_schema_descriptors`. (Supersedes `📓️wp4b-framework-modules.md` §7.4 with the two new entries.) |
| R-6 | **W2c tooling** | `schema verify --rust-entries` now has a committed reference dump: `🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/📤️schema-export-entries-dump.json`, regenerated with `SEMIO_SCHEMA_EXPORT_ENTRIES_OUT=<that path> cargo test -p semio-framework-schema --test schema-export-entries`. Note the sort key is the **ascii** `SchemaFormat::id()` spelling (§2.4), not the enum order the first `from_catalog` used. |
| R-7 | **`ui-contract` owner** | `cargo check -p semio-framework-ui-contract` (and its `cargo tree` invariant assertion) has not been run against the new crate from here — it is outside this partition's cargo scope. The dependency is dependency-free, so the invariant holds by construction, but the assertion should be re-run once. |
| R-8 | **W5b os** (new since wp3b §7.4) | `🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🕸️media-projection/🧬️schema/🔣️.json` uses `prefixItems` (2020-12) in a module the walk reaches; contract §B says migrate to `items` array + `additionalItems: false`. |

## 8. Open questions

- **Is `📇️registry` a schema scope of its own?** It is a nested module directory that is a crate root, which
  contract §A makes eligible. It deliberately carries **no** `🧬️schema/` module: its Rust types *are*
  `framework.schema`'s exports, declared once in `🧬️schema/🔣️.json`. If the catalog generator insists a crate
  root must be a scope, it will report `framework.schema.registry` as scope-less; the honest answer is that
  the package split is a Rust packaging concern, not a second contract.
- **`RESERVED_FACET_EXPORT_IDS` is now positional.** `register_facet_leaves` takes `[FacetLeaves; 4]` indexed
  by that constant. Reordering the constant silently reorders every registered facet. It is a `const` in the
  same file as its only consumer, and `schema_export_registry_reports_every_scope_with_its_exports` pins the
  order — but if a fifth fixed facet is ever added, the array and the constant must change together.
- **`schema_export_catalog_entries()` is still only as complete as the linked crates.** Unchanged from
  `📓️wp3b` §5: three scopes now register (`framework.interaction`, `framework.ui.contract`,
  `framework.schema`), and `--rust-entries-complete` will keep reporting every other catalogued scope as
  `rust-scope-unregistered` until WP4/WP5 add their calls.
- **Should `structural_validator_for` be in the registry crate at all?** It cannot be — the draft-07 compiler
  is `semio-framework-schema`'s. A dependency-restricted scope can therefore *register* and *resolve* but not
  *structurally validate* from its own crate. No scope needs that today; if one does, the validator itself
  would have to become a third dependency-free crate.
