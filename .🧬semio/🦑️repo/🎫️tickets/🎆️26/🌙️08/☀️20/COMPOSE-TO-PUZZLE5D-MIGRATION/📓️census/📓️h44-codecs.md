# H44 Census: Puzzle5d Codec Surface

Snapshot timestamp: 2026-08-20, commit 5904ebe289.

---

## 1. Snapshot Codecs

**Location:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/`

| Format    | Text Directory | Text Files | Binary Directory | Binary Files |
|-----------|---|---|---|---|
| ABNF      | `📝️text/` | ❌ | `💾️binary/` | ✅ 🔠️component.abnf |
| EBNF      | `📝️text/` | ✅ 🔤️component.ebnf | `💾️binary/` | ❌ |
| GraphQL   | `📝️text/` | ✅ 🔗️component.graphql | `💾️binary/` | ✅ 🔗️component.graphql |
| JSON      | `📝️text/` | ✅ 🔣️component.json | `💾️binary/` | ✅ 🔣️component.json |
| ANTLR4    | `📝️text/` | ✅ 🅰️component.g4 | `💾️binary/` | ❌ |
| Kaitai    | `📝️text/` | ❌ | `💾️binary/` | ✅ 🥋️component.ksy |
| Protobuf  | `📝️text/` | ✅ 🛰️component.proto | `💾️binary/` | ✅ 🛰️component.proto |
| Spicy     | `📝️text/` | ❌ | `💾️binary/` | ✅ 🌶️component.spicy |
| Semio Grammar | `📝️text/` | ✅ 📖️component.grammar.semio | `💾️binary/` | ❌ |
| Semio Protocol | `📝️text/` | ❌ | `💾️binary/` | ✅ 📡️component.protocol.semio |
| TypeScript | `📝️text/` | ✅ 🟦️component.ts | `💾️binary/` | ✅ 🟦️component.ts |
| Rust | `📝️text/` | ✅ 🦀️component.rs | `💾️binary/` | ✅ 🦀️component.rs |

**Summary:** Text + binary pairs exist for JSON, GraphQL, Protobuf, TypeScript, and Rust. Text grammars (G4, EBNF, Semio) exist. Binary encodings (ABNF, Spicy, Kaitai, Semio Protocol) exist. Asymmetry is intentional per multi-format strategy.

---

## 2. Mutation Codecs

**Location:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`

### Text Codecs (`📝️text/`)

- **Grammar:** 🅰️component.g4
- **Semio Grammar:** 📖️component.grammar.semio
- **Semio Protocol:** 🛰️component.proto
- **GraphQL:** 🔗️component.graphql
- **JSON:** 🔣️component.json
- **EBNF:** 🔤️component.ebnf
- **TypeScript:** 🟦️component.ts
- **Rust:** 🦀️component.rs
  - **Defines:** Not a direct type definition. Imports `Puzzle5dMutation` enum and provides `OpText` trait implementation for text parse/print.
  - **Trait implemented:** `protocol::OpText` — async `parse_op(&str)` and `print_op()` for DSL mutation language dispatch.

### Binary Codecs (`💾️binary/`)

- **ABNF:** 🔠️component.abnf
- **Spicy:** 🌶️component.spicy
- **Kaitai:** 🥋️component.ksy
- **Semio Protocol:** 📡️component.protocol.semio
- **TypeScript:** 🟦️component.ts
- **Rust:** 🦀️component.rs
  - **Defines:** Not a direct type definition. Re-exports `Puzzle5dMutation` from text module and provides `OpBinary` trait implementation.
  - **Trait implemented:** `protocol::OpBinary` — async `encode_op(&self) -> Vec<u8>` and `decode_op(&[u8]) -> Self` via `dsl::variants_binary::encode_op/decode_op`.

### Mutation Payload Codecs (Per-mutation modules, e.g., `⚓change-part-anchor/`)

Each of 31 mutation kinds has a three-part codec structure:

1. **🦠️mutation/** — Payload struct definition
   - **Rust:** 🦀️component.rs (defines the `MutationKind` payload struct, e.g., `ChangePartAnchor`)
   - **TypeScript:** 🟦️component.ts

2. **🔺️diff/** — Diff computation
   - **Rust:** 🦀️component.rs (implements `MutationKind::diff()`)

3. **↩️inverse/** — Inverse computation
   - **Rust:** 🦀️component.rs (implements `MutationKind::inverse()`)

### Derive Chain

**`Puzzle5dMutation` enum** (`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:line 18-47`)

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Puzzle5dSnapshot, diff = Puzzle5dDiff, schema = "puzzle.puzzle5d")]
pub enum Puzzle5dMutation { ... }
```

- **`dsl::DslEnum`** — Generates `DslVariants` trait for keyword-keyed variant dispatch (text grammar parsing).
- **`dsl::Mutations`** — Generates `impl Mutation<Puzzle5dSnapshot>` and `impl SemanticMutation<Puzzle5dSnapshot>`:
  - `async fn diff(&self, base: &Puzzle5dSnapshot) -> MutationOutcome<Puzzle5dDiff>` — dispatches to each variant's `MutationKind::diff()`.
  - `async fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Self>` — dispatches to each variant's `MutationKind::inverse()`.
  - Also generates `apply()`, `label()`, `target()`, and `foreign_steps()` dispatch arms.

---

## 3. Diff Codecs

**Location:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/`

### Text Codecs

- 📝️text/🅰️component.g4
- 📝️text/📖️component.grammar.semio
- 📝️text/🔗️component.graphql
- 📝️text/🔣️component.json
- 📝️text/🔤️component.ebnf
- 📝️text/🛰️component.proto
- 📝️text/🟦️component.ts
- 📝️text/🦀️component.rs

### Binary Codecs

- 💾️binary/🔠️component.abnf
- 💾️binary/🌶️component.spicy
- 💾️binary/🥋️component.ksy
- 💾️binary/📡️component.protocol.semio
- 💾️binary/🟦️component.ts
- 💾️binary/🦀️component.rs

**CRITICAL FINDING:** Binary diff codec files **exist on disk**. The subset root declares `diff: LanguagePair { text: Some(...), binary: None }` in the migration spec, but actual `.semio` protocol files are present:
- `📡️component.protocol.semio` (binary)

**Files needed to complete binary diff protocol:** None — already complete. The `.protocol.semio` file is already authored and present.

**Note on Diff Type Definition:** The diff is NOT an enum mutation dispatch like `Puzzle5dMutation`. It is a struct `Puzzle5dDiff` (defined in `🦀️component.rs:line 4-54`) with optional sparse field patches per artifact layer (artifact, presence, config). Diffs are generated by mutation dispatch (`Mutation::diff()`), not hand-written; there is no text/binary encoder/decoder pair for the diff type itself — it is internal to VCS layer, never serialized standalone.

---

## 4. Language Registry (`pilot_languages()`)

**Location:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️component.rs:line 44+`

Function signature: `pub async fn pilot_languages() -> &'static [dsl::LanguageSpec]`

Returns a 5-entry `Vec<dsl::LanguageSpec>`:

| Index | ID | Role | Extension | Grammar | Grammar Path | Protocol | Protocol Path |
|-------|---|---|---|---|---|---|---|
| 0 | `puzzle.puzzle5d` | Document | `Some("puzzle5d")` | `crate::artifacts::puzzle5d::dsl::COMPONENT_GRAMMAR_SEMIO` | `crate::artifacts::puzzle5d::dsl::COMPONENT_GRAMMAR_PATH` | `crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO` | `crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_PATH` |
| 1 | `puzzle.puzzle5d.op` | Ops | `None` | `crate::artifacts::puzzle5d::op::COMPONENT_GRAMMAR_SEMIO` | `crate::artifacts::puzzle5d::op::COMPONENT_GRAMMAR_PATH` | `crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_SEMIO` | `crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_PATH` |
| 2 | `puzzle.puzzle5d.diff` | Diff | `None` | `crate::artifacts::puzzle5d::diff::COMPONENT_GRAMMAR_SEMIO` | `crate::artifacts::puzzle5d::diff::COMPONENT_GRAMMAR_PATH` | `None` | `None` |
| 3 | `5d.pack` | Pack | `None` | `None` | `None` | `crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO` | `crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_PATH` |
| 4 | `5d.spr` | Spr | `None` | `None` | `None` | `crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_SEMIO` | `crate::artifacts::puzzle5d::spr::COMPONENT_PROTOCOL_PATH` |

**Key observations:**
- Index 2 (diff) has `protocol: None`, consistent with the migration spec declaring binary: None.
- But the actual `.protocol.semio` file exists on disk — wiring is incomplete or declared incorrectly.
- Ops and Spr entries point to the same snapshot::pack protocol (`crate::artifacts::puzzle5d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO`) — likely incorrect; spr should have its own protocol.

---

## 5. File Extension Convention

**Existing uses in repo:**

| Extension | Count | Examples | Producer/Consumer |
|-----------|-------|----------|---|
| `.dsl.semio` | 20+ | `space.studio.dsl.semio`, `os.workflow.dsl.semio`, wave5 examples | Handcrafted DSL; parsed by `parse_dsl()` → Snapshot |
| `.op.semio` | 20+ | `space.studio.op.semio`, `os.workflow.op.semio`, capsule-dream examples | Text operation logs; parsed by `OpText::parse_op()` |
| `.pack.semio` | 20+ | `space.studio.pack.semio`, binary pack files | Binary snapshot; decoded by `ArtifactPack` trait |
| `.spr.semio` | 20+ | `space.studio.spr.semio`, binary state-patch files | Binary operation log; decoded by ops decoder |
| `.patch.semio` | 0 | **Not found** | Not yet established |

**Conclusion:** `.patch.semio` does NOT exist. The migration can safely introduce it without collision.

---

## 6. DSL Derive Machinery

**Locations:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` (proc-macro implementation)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (public attribute macros)

### `#[derive(dsl::DslRecord)]`

**Generates (for each struct field):**
- `pub fn __dsl_spec() -> RecordSpec` — field type shape metadata.
- `pub fn __dsl_to_record(&self) -> RecordValue` — struct → RecordValue.
- `pub fn __dsl_from_record(record: &RecordValue) -> Result<Self, TextError>` — RecordValue → struct.
- `impl DslField for Self` — shape introspection and conversion.

**Does NOT generate text/binary codec entry points.**

### `#[derive(dsl::DslEnum)]`

**Generates (for each enum variant):**
- `pub fn variants() -> Vec<(String, fn() -> RecordSpec)>` — variant keyword → field specs.
- `pub fn from_named_record(keyword: &str, record: &RecordValue) -> Result<Self, TextError>` — keyword + RecordValue → enum variant.
- `pub fn to_named_record(&self) -> (String, RecordValue)` — enum variant → keyword + RecordValue.

**Does NOT generate text/binary codec entry points. Those are hand-written (`impl OpText`, `impl OpBinary`).**

### `#[derive(dsl::Mutations)]` (with `#[mutations(snapshot = ..., diff = ..., schema = ...)]`)

**Generates:**
- `impl Mutation<SnapshotType>` with arms:
  - `async fn diff(&self, base: &SnapshotType) -> MutationOutcome<DiffType>` — dispatches each variant's `MutationKind::diff()`.
  - `async fn inverse(&self, base: &SnapshotType) -> Vec<Self>` — dispatches each variant's `MutationKind::inverse()`.
  - `async fn apply(&self, base: &SnapshotType) -> Result<SnapshotType, MutationError>` — dispatches each variant's `MutationKind::apply()`.
  - `async fn label(&self) -> String` — dispatches each variant's `MutationKind::label()`.
  - `async fn target(&self) -> MutationTarget` — dispatches each variant's `MutationKind::target()`.
  - `async fn foreign_steps(&self, base: &SnapshotType) -> Vec<ForeignStep>` — dispatches each variant's `MutationKind::foreign_steps()`.
- `impl SemanticMutation<SnapshotType>` — standard adapter.

**Does NOT generate the text/binary operation encoding/decoding. Those are hand-written on the enum itself (`impl OpText`, `impl OpBinary`).**

**Summary:** The derive macros generate metadata and dispatch routing, NOT serialization. Serialization is `OpText`/`OpBinary` hand-written on the mutation enum. The `Mutations` macro ensures each enum variant corresponds to a payload struct implementing `MutationKind<Snapshot, Mutation>` — it does NOT generate `MutationKind` itself.

---

## 7. Round-Trip Helpers

**Location:** `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:line 1524+` (in tests module)

**Name:** `round_trip!(name, Type, value_expr)` macro

**Definition:**
```rust
macro_rules! round_trip {
    ($name:ident, $ty:ty, $value:expr) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            let value: $ty = $value;
            let mut bytes = Vec::new();
            value.pack_encode(&mut bytes).await;
            let mut pos = 0usize;
            let decoded = <$ty>::pack_decode(&bytes, &mut pos).await.unwrap();
            assert_eq!(pos, bytes.len());
            assert_eq!(decoded, value);
        }
    };
}
```

**What it does:** Encode value → bytes, decode bytes → value, assert decoded == original and consumed all bytes.

**Signature:** `fn round_trip!(name: ident, Type: ty, value: expr) -> TokenStream`

**Usage:** Invoked 30+ times in the actor tests module for pack-encode/pack-decode round-trip coverage.

**Applicability to migration:** The `FixtureHarness` should build a similar generic pattern for DSL parse/print round-trip over each codec variant:
- DSL text → Snapshot, Snapshot → DSL text, assert equality and print idempotence.
- Pack bytes → Snapshot, Snapshot → pack bytes, assert equality.
- Ops bytes → Mutation sequence, Mutation sequence → ops bytes, assert equality.

**Note:** No framework-level round-trip helper exists for `OpText`/`OpBinary` pairs over mutations yet — `round_trip!` only covers `ArtifactPack` (whole-document binary). The migration must author test helpers for granular operation codecs.

---

## 8. Example Test Convention

**Location:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/`

**Files:**
- 🦀️test.rs (Rust tests)
- 🟦️test.ts (TypeScript tests — content not examined)

**Rust test convention (from 🦀️test.rs):**

1. **Asset loading:** `include_str!("../🖼️assets/<file>")` for text, `include_bytes!("../🖼️assets/<file>")` for binary.

2. **DSL round-trip test:**
   ```rust
   #[semio_framework_async_macros::async_test]
   async fn dsl_asset_parses_and_round_trips() {
       let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
       assert!(text.len() > 64, "dsl fixture must carry real payload");
       let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text)
           .expect("example dsl parses");
       assert_eq!(projection.parts.len(), 2880);
       assert_eq!(projection.fasteners.len(), 2864);
       semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&projection);
   }
   ```
   - Parse fixture → Snapshot, assert Snapshot state, call framework helper `assert_dsl_round_trip(&snapshot)`.

3. **Golden-data comparison test:**
   ```rust
   #[semio_framework_async_macros::async_test]
   async fn flatten_matches_golden_poses_to_1e4() {
       let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
       let mut projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text)
           .expect("example dsl parses");
       // ... transform ...
       let golden: serde_json::Map<String, serde_json::Value> =
           serde_json::from_str(include_str!("../🖼️assets/🏅golden-poses.json"))
           .expect("golden json");
       // ... assert projection against golden ...
   }
   ```
   - Parse fixture, apply transformation, load golden data, assert numeric equivalence to tolerance.

4. **Asset non-empty test:**
   ```rust
   #[semio_framework_async_macros::async_test]
   async fn op_pack_and_spr_assets_are_nonempty() {
       assert!(include_str!("../🖼️assets/🔧️dream.op.semio").len() > 64);
       assert!(include_bytes!("../🖼️assets/🎒️dream.pack.semio").len() > 64);
       assert!(include_bytes!("../🖼️assets/📡️dream.spr.semio").len() > 64);
   }
   ```
   - Sanity-check that example files are non-empty.

5. **Inference determinism test:**
   ```rust
   #[semio_framework_async_macros::async_test]
   async fn inference_determinism_law() {
       let text = include_str!("../🖼️assets/🗣️dream.dsl.semio");
       let projection = crate::artifacts::puzzle5d::dsl::parse_dsl(text)
           .expect("example dsl parses");
       assert_eq!(Puzzle5dInference::infer(&projection),
                  Puzzle5dInference::infer(&projection));
   }
   ```
   - Assert that inference is idempotent (infer twice, expect same result).

**Asset files in example:**
- 🗣️dream.dsl.semio (handcrafted DSL)
- 🏅golden-poses.json (golden numeric data)
- 🔧️dream.op.semio (operation log text)
- 🎒️dream.pack.semio (binary snapshot pack)
- 📡️dream.spr.semio (binary operation state-patch record)

**Migration must extend this convention:**
- Every new test file type (`.patch.semio` etc.) must have:
  - A non-empty asset fixture.
  - A round-trip parse/print test.
  - A forward/inverse/diff dispatch test (if mutation-related).
  - Determinism/idempotence assertions where applicable.

---

## 9. `store::ArtifactCodec` API

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:line 28–126`

### Structure

```rust
pub struct ArtifactCodec {
    pub schema: String,
    pub extension: &'static str,
    pub pack_schema_hash: [u8; 32],
    pub compile_dsl: for<'a> fn(&'a str, &'a str) -> Pin<Box<...>>,
    pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> Pin<Box<...>>,
    pub edit_text_from_envelope: for<'a> fn(&'a MutationEnvelope) -> Pin<Box<...>>,
    pub apply_ops_binary: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> Pin<Box<...>>,
}
```

### Creation

**Method:** `pub async fn ArtifactCodec::of<P, Mutation>(schema: impl Into<String>) -> Self`

where:
- `P: Clone + PartialEq + Serialize + DeserializeOwned + ArtifactDsl + ArtifactPack + Send + 'static`
- `Mutation: self::Mutation<P> + PartialEq + Serialize + DeserializeOwned + OpText + OpBinary + Send + 'static`

**Returns:** A monomorphized `ArtifactCodec` with five `fn` pointers, one per entry point.

### Entry Points

1. **`compile_dsl(dsl_text: &str, ops_text: &str) -> Result<(ArtifactPackFiles, String), VcsError>`**
   - Parse DSL + ops text → `ParsedDocumentText<P, Mutation>`.
   - Serialize to pack files (pack bytes, spr bytes).
   - Return pack files + re-printed canonical DSL mirror.
   - Used by: Handcrafted text import path.

2. **`print_mirror(pack_bytes: &[u8], spr_bytes: &[u8]) -> Result<ArtifactTextFiles, VcsError>`**
   - Decode pack + spr → `ParsedDocumentText<P, Mutation>`.
   - Serialize to text files (dsl text, ops text).
   - Return text files (logging mirror).
   - Used by: Schema-agnostic callers that never touch concrete `P`/`Mutation`.

3. **`edit_text_from_envelope(envelope: &MutationEnvelope) -> Result<String, VcsError>`**
   - Decode opaque `OpBinary` payload → concrete `Mutation`.
   - Serialize to ops text (one edit block: header + indented op line).
   - Return op text line.
   - Used by: `FolderTextStorage::append_ops()` hot-path logging append.

4. **`apply_ops_binary(pack: &[u8], spr: &[u8], ops_vec: &[u8]) -> Result<(Vec<u8>, Vec<u8>, String), VcsError>`**
   - Decode ops_vec → `Vec<Mutation>`.
   - Load baseline from pack + spr (or empty).
   - Dispatch mutations to `ArtifactStore::dispatch()`.
   - Serialize result to pack files + re-printed ops text.
   - Return (pack, spr, ops text).
   - Used by: Host-authoritative Emit apply (VCS coordinator).

### No Direct Encode/Decode

The `ArtifactCodec` does **not** expose direct `encode_snapshot()` or `decode_snapshot()` methods. Those are trait-bound methods on `P` (`ArtifactPack`), not on `ArtifactCodec`. The codec's job is to coordinate _bridges_ between text and binary, and between individual ops and whole-document state.

---

## Summary of Findings

### Item 3 (Diff Codecs) — Files Needed for Binary Diff

**Already complete.** The binary diff protocol file is authored and present:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`

However, the `pilot_languages()` registry **declares** `protocol: None` for the diff role (index 2), which is incorrect. The wiring must be corrected: set `protocol: Some(crate::artifacts::puzzle5d::diff::COMPONENT_PROTOCOL_SEMIO)` and provide the path.

**Critical discrepancy:** Declared vs. implemented. The spec says `binary: None`, but the file exists. Clarify intent: should binary diff codec be supported, or should the file be removed?

### Item 5 (File Extensions) — Already Exist

- `.dsl.semio` — 20+ existing uses (handcrafted DSL documents)
- `.op.semio` — 20+ existing uses (text operation logs)
- `.pack.semio` — 20+ existing uses (binary snapshots)
- `.spr.semio` — 20+ existing uses (binary operation logs)
- `.patch.semio` — **Not found.** Safe to introduce.

---

## Cross-References

- Puzzle5dMutation enum: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:18-47`
- pilot_languages(): `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️component.rs:44+`
- ArtifactCodec::of(): `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:61–126`
- DslRecord derive: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:1–50`
- DslEnum derive: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:51–60`
- Mutations derive: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:61–150`
- round_trip! macro: `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs:1524–1540`
- Example tests: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌙️capsule-dream/🧪️tests/🦀️test.rs`
