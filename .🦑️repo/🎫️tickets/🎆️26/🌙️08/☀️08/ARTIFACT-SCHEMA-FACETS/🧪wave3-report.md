# 🧪 Wave 3 Report — Kernel Snapshot Rename + Framework Schema Module

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W3 owns §4 / §5 / §7 / §8 / §9 surfaces inside
`semio-framework-os-kernel` and `semio-framework-schema` only.

## 1. Kernel `Projection` → `Snapshot`

### Renamed symbols (kernel crate)

| From | To |
| --- | --- |
| `ArtifactEngine::Projection` | `ArtifactEngine::Snapshot` |
| `ArtifactEngine::projection` | `ArtifactEngine::snapshot` |
| `DocumentVcs::initial_projection` | `DocumentVcs::initial_snapshot` |
| `materialize_document_projection` | `materialize_document_snapshot` |
| `materialize_document_projection_with_conflicts` | `materialize_document_snapshot_with_conflicts` |
| `DocumentStore::projection_json` | `DocumentStore::snapshot_json` |
| `DocumentStore::projection_with_conflicts` | `DocumentStore::snapshot_with_conflicts` |
| `SpaceHost::meta_projection` | `SpaceHost::meta_snapshot` |
| `BaseProjection` | `BaseSnapshot` |
| `ProjectionRecord` | `SnapshotRecord` |
| `ProjectionBodyKind` | `SnapshotBodyKind` |
| `ProjectionHeader` | `SnapshotHeader` |
| `encode_projection` / `decode_projection` / `parse_projection` | `encode_snapshot` / `decode_snapshot` / `parse_snapshot` |
| `find_*_projection*` / `read_projection_at` | `find_*_snapshot*` / `read_snapshot_at` |
| `latest_projection_offset_at_or_before` | `latest_snapshot_offset_at_or_before` |
| `SEC_PROJECTION_OFFSETS` | `SEC_SNAPSHOT_OFFSETS` |
| `record_projection` / index `projections` field | `record_snapshot` / `snapshots` |
| `DemoProjection` / `SpaceHistoryProjection` (kernel tests) | `DemoSnapshot` / `SpaceHistorySnapshot` |
| local `projection` / `pre_projection` / `_projection` idents | `snapshot` / `pre_snapshot` / `_snapshot` |

`DocumentStore<P, Mutation>` keeps the generic parameter letter `P`; all projection-named
accessors, locals, and docs inside the store/vcs/spr/engine modules were renamed to snapshot.

`Mutation<P>` / `MutationDiff<P>` docs now speak of snapshot / base state (no projection wording).

### `ArtifactEngine` associated type

```rust
pub trait ArtifactEngine: Send + Sync {
    type Artifact;   // full state — all StateClass values
    type Snapshot;   // persisted subset only
    type Mutation: Mutation<Self::Snapshot>;
    type Diff: MutationDiff<Self::Snapshot>;
    fn artifact(&self) -> &Self::Artifact;
    fn snapshot(&self) -> &Self::Snapshot;
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, EngineFault>;
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation>;
}
```

**Plugin consequence:** every plugin `ArtifactEngine` impl must now provide `type Artifact`,
`fn artifact(&self)`, and rename `Projection`/`projection` → `Snapshot`/`snapshot`. No default
was added — breakage of plugin crates is expected and owned by later waves.

### Deliberately excluded from the rename

| Path | Why |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📽️projection/` | DB read-model `Projection` — unrelated concept; keeping the word is the disambiguation. Not part of the kernel crate. |
| `DocumentApp::Projection` in `🔌️plugin/🦀️component.rs` (`semio-framework-plugin`) | Lives outside `semio-framework-os-kernel`. §8 says kernel-only; W6 sweeps consumers. |
| Plugin artifact types (`LowpolyProjection`, `CadProjection`, …) referenced from `🗣️dsl/🧪️fixture-sweep` | Artifact leaf renames are later fan-out waves; fixtures still name today's types. |
| `✏️s/🔌️plugins/**`, renderer, hub, os apps | Explicitly out of W3 scope. |

## 2. Framework `🧬️schema` module

### Derive crate

- Location: `🧰️framework/🔨️modules/🧬️schema/✨️derive/`
- Package: `semio-framework-schema-derive` at `✨️derive/📦️packages/🦀️rust/`
- Registered in workspace `Cargo.toml` members next to the DSL derive
- Nx: `@semio-tech/schema-derive-rs` (`📋️project.json` + `📜️script.ts`)
- Emits `ArtifactSchemaFields` with camelCase field keys

### Descriptor / registry API

```rust
pub struct FacetLeaves {
    pub rust: &'static str,
    pub typescript: &'static str,
    pub graphql: &'static str,
    pub json_schema: &'static str,
    pub proto: &'static str,
}
pub struct ArtifactSchemaDescriptor {
    pub id: &'static str,
    pub artifact: FacetLeaves,
    pub snapshot: FacetLeaves,
    pub diff: FacetLeaves,
}
pub struct ArtifactSchemaRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, descriptor: ArtifactSchemaDescriptor);
    pub fn get(&self, id: &str) -> Option<&ArtifactSchemaDescriptor>;
    pub fn iter(&self) -> impl Iterator<Item = &ArtifactSchemaDescriptor>;
}
pub const GRAPHQL_STATE_PREAMBLE: &str; // §5 SDL
pub trait ArtifactSchemaFields {
    fn artifact_schema_id() -> &'static str;
    fn field_states() -> &'static [(&'static str, StateClass)];
}
#[derive(ArtifactSchema)] // + #[artifact_schema(id=…)] / #[state(…)]
```

`StateClass` is re-exported from `semio-framework-os-kernel` (no second enum). Schema depends on
kernel; kernel does not depend on schema — no cycle. Kebab parse helpers
`parse_state_class_kebab` / `state_class_kebab` live in schema for JSON Schema leaves.

TypeScript twin: `🧬️schema/🟦️component.ts`, mounted from
`🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`.

Table-driven test in `//#region 🔖️Tests` registers synthetic `s.wave3.synthetic` so the registry
walk is non-vacuous (W4 wires real artifacts).

## 3. Gate tails (verbatim)

### `cargo check -p semio-framework-os-kernel`

```
2180 | |     Mutation: Clone + Serialize + DeserializeOwned + self::Mutation<P> + OpBinary + OpText,
     | |___________________________________________________________________________________________- method in this implementation
...
2283 |       pub(crate) fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Mutation>, applied_edit_ids: Vec<String>) {
     |                     ^^^^^^^^^^^^

warning: `semio-framework-os-kernel` (lib) generated 45 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 22 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.18s
```

### `cargo test -p semio-framework-schema`

```

warning: `semio-framework-os-kernel` (lib) generated 45 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 22 suggestions)
    Finished `test` profile [unoptimized] target(s) in 0.21s
     Running unittests 📦️glue.rs (target/debug/deps/semio_framework_schema-04eb66f90ba82f5a)

running 3 tests
test component::tests::graphql_state_preamble_matches_normative_sdl ... ok
test component::tests::registry_descriptors_carry_valid_snapshot_state_and_match_field_states ... ok
test component::tests::schema_catalog_still_registers_json ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests semio_framework_schema

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## 4. Left for later waves

- Plugin `ArtifactEngine` / `DocumentApp` fan-out (W5/W6)
- Real artifact facet registration into `ArtifactSchemaRegistry` (W4+)
- Policy scanners / taxonomy (W1/W2 already; root script concurrent)
