# WP2 — Rust side of the `(scope id, export id, format id)` resolver

Partition: `🧰️framework/🔨️modules/🧬️schema/**` only. No plugin, hub, os, harness or root-script file
was touched. Crate: `semio-framework-schema`
(`🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`, unchanged — no new dependency).

## 1. Files changed

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs` | new region `🔖️SchemaExportResolution` (resolver, registry, errors, boundary entry point) + new test region `🔖️SchemaExportResolution` / `🔖️Draft07OracleVectors` |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator.rs` | cross-document `$ref` by `$id`, tuple `items` + `additionalItems`, `pattern` (owned engine), `StructuralValidation` trait, compiled-pattern cache |
| `🧰️framework/🔨️modules/🧬️schema/🟦️.ts` | TS twins: `SCHEMA_FORMATS`, `SCHEMA_FORMAT_TAXONOMY_KEYS`, `SCHEMA_FORMAT_LEAVES`, `RESERVED_FACET_EXPORT_IDS`, `SchemaExport`, `ScopeSchemaExports`, `SchemaExportEntry`, `SchemaResolveError` |
| `🧰️framework/🔨️modules/🧬️schema/🧪️fixtures/✅️draft07-validation-vectors.json` | new — language-agnostic draft-07 vectors (7 cases, 22 valid + 32 invalid instances with expected error paths) |
| `🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts` | new — vitest spec running the same vectors through `ajv` draft-07 |

No file was deleted; nothing was superseded by this change.

## 2. API surface (signatures)

### 2.1 Named exports and formats — `⚛️component.rs`

```rust
pub enum SchemaFormat { Rust, Typescript, Graphql, JsonSchema, Protobuf }
impl SchemaFormat {
    pub const ALL: [Self; 5];
    pub fn id(self) -> &'static str;            // "rust" | "typescript" | "graphql" | "jsonschema" | "protobuf"
    pub fn taxonomy_key(self) -> &'static str;  // "🦀️rust" | "🟦️typescript" | "🔗️graphql" | "🔣️jsonschema" | "🛰️protobuf"
    pub fn parse(id: &str) -> Option<Self>;     // accepts either spelling
    pub fn leaf(self, leaves: &FacetLeaves) -> &'static str;
}

pub struct SchemaExport      { pub id: &'static str, pub leaves: FacetLeaves }
pub struct ScopeSchemaExports { pub scope: &'static str, pub exports: &'static [SchemaExport] }
pub const RESERVED_FACET_EXPORT_IDS: [&str; 4]; // ["artifact","snapshot","diff","mutations"]

pub enum SchemaExportRegistryError {
    ConflictingScope { scope: String },
    DuplicateExportId { scope: String, export: String },
}

pub enum SchemaResolveError {
    UnknownScope  { scope: String },
    UnknownExport { scope: String, export: String },
    FormatAbsent  { scope: String, export: String, format: SchemaFormat },
    AmbiguousScope{ scope: String, export: String },
}

pub enum SchemaBoundaryError { Resolve(SchemaResolveError), Schema(SchemaError) }

pub struct SchemaExportEntry { pub scope: &'static str, pub export: &'static str, pub format: SchemaFormat }

pub struct SchemaExportRegistry { /* fixed facets by scope + named exports by scope */ }
impl SchemaExportRegistry {
    pub fn new() -> Self;
    pub fn register_descriptor(&mut self, ArtifactSchemaDescriptor) -> Result<(), SchemaExportRegistryError>;
    pub fn register_exports(&mut self, ScopeSchemaExports)          -> Result<(), SchemaExportRegistryError>;
    pub fn scopes(&self) -> Vec<&'static str>;                                   // sorted, deduped
    pub fn exports(&self, scope: &str) -> Result<Vec<&'static str>, SchemaResolveError>;
    pub fn leaves(&self, scope: &str, export: &str) -> Result<FacetLeaves, SchemaResolveError>;
    pub fn resolve(&self, scope: &str, export: &str, format: SchemaFormat) -> Result<&'static str, SchemaResolveError>;
    pub fn structural_validator(&self, scope: &str, export: &str) -> Result<OwnedJsonSchemaValidator, SchemaBoundaryError>;
    pub fn entries(&self) -> impl Iterator<Item = SchemaExportEntry> + '_;       // (scope, export, format)
}

// OS-wide catalog (same OnceLock<Mutex<…>> shape the kernel catalogs use)
pub fn register_scope_schema_exports(ScopeSchemaExports) -> Result<(), SchemaExportRegistryError>;
pub fn scope_schema_exports_registered(scope: &str) -> bool;
pub fn with_schema_export_registry<R>(visit: impl FnOnce(&SchemaExportRegistry) -> R) -> R;
pub fn resolve_schema_export(scope: &str, export: &str, format: SchemaFormat) -> Result<&'static str, SchemaResolveError>;
pub fn schema_export_catalog_entries() -> Vec<SchemaExportEntry>;
pub fn structural_validator_for(scope: &str, export: &str) -> Result<OwnedJsonSchemaValidator, SchemaBoundaryError>;
```

Resolution is **exact**: no nearest-parent search, no glob, no fixture-local fallback, exactly as
`📋️execution-contract.md` §A requires. The four fixed facets of a registered
`ArtifactSchemaDescriptor` are resolvable as the reserved export ids `artifact`/`snapshot`/`diff`/
`mutations`; a facet whose leaf is empty resolves to `FormatAbsent`, never to `UnknownExport`.

### 2.2 Validator — `✅️validator.rs`

```rust
pub trait StructuralValidation {
    fn validate_structure(&self, instance_json: &str) -> Result<ValidationProgress, SchemaError>;
    fn validate_structure_with_control(&self, instance_json: &str, control: &ValidationControl) -> Result<ValidationProgress, SchemaError>;
}
impl StructuralValidation for OwnedJsonSchemaValidator { … }

pub const JSON_SCHEMA_DRAFT_07_DIALECT: &str = "http://json-schema.org/draft-07/schema#";

impl OwnedJsonSchemaValidator {
    pub fn compile(schema_json: &str) -> Result<Self, SchemaError>;                                   // unchanged
    pub fn compile_with_control(&str, &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError>; // unchanged
    pub fn compile_with_documents(schema_json: &str, documents: &[&str]) -> Result<Self, SchemaError>; // NEW
    pub fn compile_with_documents_and_control(&str, &[&str], &ValidationControl) -> Result<(Self, ValidationProgress), SchemaError>; // NEW
    // validate_json / validate_json_with_control / is_valid_json unchanged
}

pub struct PatternMatcher;                                    // NEW, owned regex for `pattern`
impl PatternMatcher {
    pub fn compile(pattern: &str) -> Result<Self, String>;
    pub fn is_match(&self, text: &str) -> bool;               // unanchored search, JSON Schema semantics
}
```

Keyword coverage after this change (all of the WP2 required scope):
`type`, `properties`, `required`, `additionalProperties`, `enum`, `const`, `items` (single **and**
tuple) + `additionalItems`, `minItems`/`maxItems`, `minimum`/`maximum`/`exclusiveMinimum`/
`exclusiveMaximum`/`multipleOf`, `minLength`/`maxLength`, `pattern`, `oneOf`/`anyOf`/`allOf`/`not`,
`uniqueItems`, `$defs`/`definitions`, `$ref` local **and across registered documents by `$id`**.
Unknown keywords are still a hard compile error, `x-*` extensions still pass through.

## 3. Decisions and deviations

### 3.1 Deviation — `exports` is a sibling declaration, not a field on `ArtifactSchemaDescriptor`

`📋️execution-contract.md` §C says `ArtifactSchemaDescriptor` "gains `exports: &'static [(&'static str,
FacetLeaves)]`". Adding a required field to that struct breaks **235 handcrafted struct literals across
115 files** (`grep -rn --include='*.rs' 'ArtifactSchemaDescriptor {'`), all of which are outside this
partition and are being edited concurrently by WP3–WP5 workers. Rust offers no way to add a required
field without touching every literal.

Implemented instead: `ScopeSchemaExports { scope, exports: &'static [SchemaExport] }` as a **sibling
registration unit**, exactly the precedent the repo already set for `ArtifactInferenceDescriptor`
(documented at `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs:395-402`: "a SIBLING registry …
not a field on it: the four-facet descriptor already has ~107 handcrafted call sites … and none of
them need to change"). `SchemaExportRegistry` unifies both sides, so every consumer sees one
`(scope, export, format)` surface and never has to know which half a leaf came from.

A second, independent reason the field could not have worked as written: the descriptor round-trips
through `KernelArtifactSchemaDescriptor` in `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`
(outside this partition), so a new field would have been silently dropped by
`descriptor_from_kernel` unless the wire crate were edited too. The sibling registry has its own
`OnceLock<Mutex<…>>` store in this module and therefore round-trips losslessly with no cross-crate
edit.

### 3.2 `AmbiguousScope` semantics

Reserved facet ids and named export ids are separate namespaces, each unique on its own side.
Registration therefore cannot detect a cross-namespace collision (the two declarations arrive
independently, in either order). `resolve`/`leaves` report `AmbiguousScope` when one export id is
served by both a fixed facet and a named export of the same scope. Covered by
`schema_export_resolution_distinguishes_every_failure`.

### 3.3 Owned `pattern` engine rather than reusing `semio-framework-math`

An in-house Rust regex engine does exist —
`🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️.rs` region `🔖️Automata`, `Dfa::from_pattern` /
`RegexConstraint`, Thompson NFA → subset construction, no external crate. It was **not** reused:

- it is byte-level with no anchors (`^`/`$`) and no `\d`/`\w`/`\s` shorthand — both are ubiquitous in
  JSON Schema `pattern` values, so the required subset is not expressible;
- its API is `async fn step(&self, state, byte)` per byte, driven by an LLM token-masking loop;
- `semio-framework-schema` is a boundary module every plugin links; depending on the sampling/LLM
  math crate for a string predicate inverts the layering.

`PatternMatcher` in `✅️validator.rs` is a compact backtracking matcher over `char`s covering
literals, `.`, `\d \D \w \W \s \S`, `\n \r \t \f \v \0 \xHH \uHHHH`, classes with ranges and negation,
groups `(…)`/`(?:…)`, alternation, `* + ? {n} {n,} {n,m}` with lazy variants, and `^`/`$`.
Backreferences, lookaround and `\b` are rejected at compile time rather than silently ignored.
Patterns are compiled once during schema compilation and cached on the validator, so validation of an
instance never recompiles a regex.

### 3.4 `$schema` dialect is *not* enforced at compile time

`JSON_SCHEMA_DRAFT_07_DIALECT` is exported, but the compiler accepts a document declaring any
`$schema` and implements the draft-07 keyword subset regardless. Enforcing it here would break live
MCP runtime today: `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs:291`
(`compile_validator`) compiles `schemars`-derived documents that are converted to the 2020-12 dialect
by `convert_draft07_to_2020_12` in the same file. Dialect hygiene is a **file** check and belongs in
WP7's `schema check`, not in a per-instance runtime path. Flagged as a cross-partition request below.

## 4. Tests and real output

### 4.1 Rust — `cargo test -p semio-framework-schema`

`CARGO_TARGET_DIR=…/scratchpad/target-w3 RUSTC_WRAPPER="" cargo test -p semio-framework-schema`
(full log: `🗑️generated/wp2-cargo-test.txt`). Tests added by this work packet, all in
`⚛️component.rs`'s test module:

- `schema_format_ids_mirror_the_taxonomy_schema_formats_keys`
- `schema_export_registration_is_duplicate_safe_and_conflict_fatal`
- `schema_export_registration_rejects_a_duplicate_export_id_inside_one_scope`
- `schema_export_resolution_distinguishes_every_failure`
- `schema_export_registry_reports_every_scope_with_its_exports`
- `structural_validator_resolves_cross_scope_refs_by_document_id`
- `scope_schema_exports_register_into_the_os_wide_catalog`
- `owned_validator_agrees_with_the_shared_draft07_vectors`
- `owned_pattern_matcher_covers_the_supported_ecma_subset`

```
running 26 tests
test component::tests::artifact_composition_fields_default_to_empty_for_leaf_artifacts ... ok
test component::tests::artifact_composition_fields_derive_emits_expected_slot_tables ... ok
test component::tests::app_schema_registry_accepts_placeholder_owner_for_wave_structure ... ok
test component::tests::artifact_composition_projection_walks_aliases_nested_options_and_cancels ... ok
test component::tests::artifact_inference_registry_registers_independently_of_the_snapshot_diff_mutations_descriptor ... ok
test component::tests::artifact_inference_graphql_sdl_composes_shared_preamble_with_facet_leaf ... ok
test component::tests::graphql_state_preamble_matches_normative_sdl ... ok
test component::tests::owned_pattern_matcher_covers_the_supported_ecma_subset ... ok
test component::tests::derived_fields_leave_the_state_class_axis_entirely ... ok
test component::tests::artifact_composition_projection_real_child_alias_has_fixed_admission_bounds ... ok
test component::tests::schema_export_registration_is_duplicate_safe_and_conflict_fatal ... ok
test component::tests::registry_descriptors_carry_valid_snapshot_state_and_match_field_states ... ok
test component::tests::owned_validator_preserves_supported_keyword_corpus ... ok
test component::tests::retired_state_vocabulary_no_longer_parses ... ok
test component::tests::schema_export_registration_rejects_a_duplicate_export_id_inside_one_scope ... ok
test component::tests::schema_catalog_still_registers_json ... ok
test component::tests::owned_validator_preserves_every_exercised_keyword_family ... ok
test component::tests::state_class_kebab_round_trips_exactly_the_four_lanes ... ok
test component::tests::scope_schema_exports_register_into_the_os_wide_catalog ... ok
test component::tests::schema_export_registry_reports_every_scope_with_its_exports ... ok
test component::tests::structural_validator_resolves_cross_scope_refs_by_document_id ... ok
test component::tests::owned_validator_diagnostics_progress_and_cancellation_are_deterministic ... ok
test component::tests::owned_validator_agrees_with_the_shared_draft07_vectors ... ok
test component::tests::schema_export_resolution_distinguishes_every_failure ... ok
test component::tests::schema_format_ids_mirror_the_taxonomy_schema_formats_keys ... ok
test component::tests::schema_versions_ignore_whitespace_and_detect_drift ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

The 17 pre-existing tests are unchanged and still pass, so the validator rewrite did not regress the
existing keyword corpus, diagnostics-path or cancellation contracts.

### 4.2 Rust — `cargo check` / `cargo clippy`

```
$ cargo check -p semio-framework-schema --all-targets
    Checking semio-framework-schema v0.1.0 (…/🧬️schema/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 4.81s
```

`cargo clippy -p semio-framework-schema --all-targets` finishes with **zero warnings attributed to
`semio-framework-schema`**. The three warnings emitted during that run belong to peer crates outside
this partition and are pre-existing: `semio-framework-replication` (lib) 2 ×
`large_enum_variant` at `🔗️causal/🦀️.rs:371,428`, `semio-framework-os-kernel` (lib) 1 ×
`large_enum_variant` at `📇️directory/🧬️schema/🦀️.rs:364`.

### 4.3 Third-party oracle — `ajv` draft-07 via vitest

`bunx vitest run "🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts" --reporter=verbose`
(full log: `🗑️generated/wp2-ajv-oracle.txt`):

```
 RUN  v4.1.10 /Users/ueli/Documents/semio

 ✓ …/✅️draft07-oracle.test.ts > draft-07 structural validation vectors > declares the draft-07 dialect on every case 5ms
 ✓ … > agrees with ajv on object-required-and-additional-properties 38ms
 ✓ … > agrees with ajv on string-pattern-and-enum-and-const 21ms
 ✓ … > agrees with ajv on array-list-items-and-bounds 294ms
 ✓ … > agrees with ajv on array-tuple-items-and-additional-items 134ms
 ✓ … > agrees with ajv on combinators-one-of-any-of-all-of 5ms
 ✓ … > agrees with ajv on local-ref-into-definitions 114ms
 ✓ … > agrees with ajv on cross-document-ref-by-id 89ms

 Test Files  1 passed (1)
      Tests  8 passed (8)
```

Oracle contract: both implementations must agree on the **verdict** for every one of the 54
instances. On top of that, the owned validator's reported error path must equal the vector's
`errorPath` exactly (rendered `$`, `$.name`, `$.items[0]`), and ajv must report an error at that path
**or at one of its ancestors** — ajv attributes `additionalItems`/`uniqueItems`/`oneOf` failures to
the containing array or object rather than the offending child, so exact path identity is not a
property either implementation can be held to.

### 4.4 TypeScript twin

```
$ bunx tsc --noEmit --skipLibCheck --target es2022 --module esnext --moduleResolution bundler --strict "🧰️framework/🔨️modules/🧬️schema/🟦️.ts"
(no output)
```

## 5. Cross-partition requests

1. **Plugin / scope descriptor sites (WP3–WP5).** A scope that publishes named exports must call,
   next to its existing `register_artifact_schema_descriptor(…)`:

   ```rust
   const EXPORTS: [semio_framework_schema::SchemaExport; N] = [
       semio_framework_schema::SchemaExport {
           id: "<ExportId>",
           leaves: semio_framework_schema::FacetLeaves {
               rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"),
               graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"),
               proto: include_str!("🛰️.proto"),
           },
       },
       // …
   ];
   semio_framework_schema::register_scope_schema_exports(
       semio_framework_schema::ScopeSchemaExports { scope: "<scope id>", exports: &EXPORTS },
   ).expect("scope schema exports");
   ```

   `scope` must equal the module `$id` path segment form (`s.trinity.jack`, `hub.inference`) and, for
   a scope that also registers an `ArtifactSchemaDescriptor`, must equal that descriptor's `id`.
   Export ids must not reuse `artifact`/`snapshot`/`diff`/`mutations` — those four are already
   resolvable from the descriptor, and declaring them again makes the scope resolve to
   `AmbiguousScope`.

2. **Catalog generator (WP2 TS side / root `📜️script.ts`).** `schema verify` can cross-check the
   generated `🔣️schema-catalog.json` against the runtime truth with
   `semio_framework_schema::schema_export_catalog_entries()` (or `with_schema_export_registry(|r|
   r.scopes()/r.exports(scope))`), which yields exactly the `(scope, export, format)` triples that
   have a non-empty leaf. Format ids in the catalog should be the ascii `SchemaFormat::id()` spelling;
   `SCHEMA_FORMAT_TAXONOMY_KEYS` in `🟦️.ts` maps them to the taxonomy `schemaFormats` emoji keys.

3. **Root `📜️script.ts` / `📋️project.json` / `.vscode/launch.json` (WP7).** The ajv oracle spec
   `🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts` currently has no nx target; it was run
   directly with `bunx vitest run`. It should become part of `schema test`.

4. **Root `package.json` (WP7).** `ajv` resolves today only as a hoisted transitive dependency
   (`node_modules/ajv` 8.20.0). It should be declared explicitly in `devDependencies` so the oracle
   spec cannot break from an unrelated dependency change. It must stay a **devDependency** — no
   production code may reach it.

5. **`🌉️mcp` (WP4-os).** `convert_draft07_to_2020_12` + `compile_validator`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs:225-291`) feed 2020-12-labelled
   documents into the owned validator. Once the repo is uniformly draft-07 (WP7), decide whether MCP
   keeps its 2020-12 wire label while validating against the draft-07 compiler, or whether the label
   conversion moves to the MCP serialization boundary only. Until that is settled the validator
   deliberately does not enforce `$schema`.

## 6. Open questions

- **Should `SchemaExportRegistry::exports()` order named exports by declaration or sort them?**
  Currently: four reserved facet ids in fixed order, then named exports in declaration order.
  Declaration order makes the generated catalog stable only if the descriptor arrays are stable; if
  WP2's catalog generator wants a byte-stable diff regardless, it should sort at the generator.
- **Cross-scope `$ref` document set.** `structural_validator` offers every registered JSON Schema
  leaf as a sibling document keyed by its `$id`, and fails with "two schema leaves declare `$id` X"
  if two scopes collide. Whether `schema check` should also enforce `$id` uniqueness statically
  (rather than only at validator-compile time) is a WP7 decision.
- **`ValidationControl` node budget vs `pattern`.** Pattern matching is not metered by the traversal
  node budget (only schema/instance nodes are). A pathological pattern plus a long string can burn
  CPU without hitting `LimitExceeded`. If a scope ever accepts untrusted `pattern` values this needs
  a step budget inside `PatternMatcher::is_match`; today every `pattern` is handcrafted repo-owned
  text, so it was left out rather than guessed at.
