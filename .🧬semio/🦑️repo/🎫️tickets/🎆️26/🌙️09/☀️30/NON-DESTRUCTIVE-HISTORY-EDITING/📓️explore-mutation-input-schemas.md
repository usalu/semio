# 📓️ Explore: how mutations and their inputs are declared (schema, Rust, TS, registry, UI metadata)

Ticket: `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING` — read-only audit, no source edited.
Requirement under audit: "Every input of a mutation has information about the UI element (such as slider, stepper, min, max, snapping points on a slider, etc)".

Path aliases used below (all relative to the repo root, all with the real emoji spelling):

| Alias | Path |
| --- | --- |
| `FW` | `🧰️framework/🔨️modules` |
| `OS` | `🧰️framework/🛍️products/💻️os/🔨️modules` |
| `REPO` | `🧰️framework/🛍️products/🦑️repo/🔨️modules` |
| `UI` | `🧰️framework/🔨️modules/🖱️ui` |
| `P2D` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any` |
| `MUT` | `P2D/🧬️schema/🧬️mutations` |
| `LP` | `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` |
| `DRW` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets` |
| `NOTE` | `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets` |

Statistics were computed by scanning the generated scope catalog `REPO/📚️library/🔣️schema-catalog.json` (3,540 scopes, 2,834 of level `mutation-leaf`) and the leaf directories it points at (python, read-only). "Verified" means I read the file or ran the scan; "inferred" is marked as such. Nothing was compiled or run in cargo.

---

## 0. Conclusions in ten lines

1. Mutation payloads are declared in **three parallel hand-written forms per leaf** — a draft-07 JSON Schema (`🧬️schema/🔣️.json`, normative, 2,834/2,834), a Rust struct (`🦀️.rs`, 2,764 direct + 70 facet layout, all `ToValue`/`FromValue`), and an optional TS mirror (`🟦️.ts`, ~624 leaves = 22 %). There is **no generator** from JSON Schema to Rust/TS/GraphQL/proto; parity is enforced by repo laws, fixtures and oracles. Python/C# have **zero** mutation implementations (only JSON Schema is consumable there).
2. Per-leaf metadata is a **strict 14-field descriptor** (`🔣️.json` next to the leaf, read at compile time by `#[derive(dsl::MutationLeaf)]`); it carries identity/codec facts (`semanticKind`, `displayName`, `binaryTag`, `invertibility`, ...) and **no input information at all**. Extra keys are a hard error in three places.
3. **No mutation input carries UI metadata today.** Across 5,025 real inputs (2,834 leaves) only 25 of 2,541 `type: number` properties (any depth) have any bound, 0 carry `multipleOf`, 7 a `default`, 0 an `x-semio-*` UI annotation; `title`/`description` exist on 891/600 nodes, none localized.
4. The UI vocabulary **already exists but only for actions/config/UI nodes**, in at least seven unconnected shapes: `ActionArgDef`/`ArgSchema` (best: localized label, min/max/step/integer/unit/options/format/presentation), `ConfigFieldShape` (nearly dead), `WindowEngagementControl` (slider/stepper/ring/toggleGroup/select), `ui_contract::SliderProps`/`NumberStepperProps`/`UiInputNode`, `dsl::Shape` (Quantity/Angle/Range/Ref/Coord/Dir/Dim/Enum), React `Slider.snapValues` (the only snap-point support, not on the wire) and hand-written per-app inspector field tables (draw: `opacity` 0..1 step 0.01 hard-coded in the panel, not in the schema or the mutation).
5. Generic value machinery is strong: every `Mutation`/`MutationKind` is `ToValue + FromValue` → `DslValue` tree with typed **path edits** (`FromValue::edit_value_at_path`, `ValueEdit::{Set,Insert,InsertAt,Remove}`); the derive refuses editing an enum tag in place. `SemanticMutation::{kinds,semantics,label,target}` give kind/label(en+de)/target ids without knowing the type.
6. Blockers for a generic editor: (a) leaf JSON Schemas are **not embedded at runtime** (`payload_schema` is only a relative path string; nothing reads it), (b) only 888/2,831 leaves derive `DslRecord` (no universal `RecordSpec`), (c) aggregate **wire layouts are inconsistent** (internal tag `mutation`, adjacent `mutation`+`payload`, tag `kind`/`operation`, externally tagged; camelCase vs snake_case).
7. The generic schema-driven property grid that exists (`stdio` "snapshot details", used by ~20 stdio editors) is JSON-Schema+`ToValue` driven with localized `x-semio-title: {en,de}`, but renders every number as a **text input** (min/max only feed validation/templates) — no slider, stepper, snaps.
8. Puzzle-2d style gestures **lose their inputs**: `translateSelection {dx,dy,step}` mutates a scratch fixture and the committed leaves are `move-node id newX newY` derived by snapshot diff (`puzzle2d_snapshot_mutations`). Editing "the drag" needs composite leaves (only 1 of 2,834 is composite today); editing a leaf is what the descriptor can support.
9. Recommendation: put the input descriptor **in the leaf payload JSON Schema** as one new annotation `x-semio-ui` (+ standard `minimum/maximum` for hard bounds), share it through PascalCase `$defs` scalar types (top-100 field names cover 60 % of inputs), register the keyword in the strict-Ajv vocabulary fixture, add one `PAYLOAD_SCHEMA: &str` to `MutationLeaf` (derive `include_str!`s it), and read it with one Rust reader + one TS twin (Python/C# read JSON directly). Do **not** add a 15th field to the 14-field descriptor.
10. Generic edit = `to_value` → strip/normalize tag (needs a small `payload_value`/`with_payload_value` pair generated by `#[derive(Mutations)]`) → `edit_value_at_path` → owned draft-07 validation → `FromValue` → `MutationKind::diff(base)` outcome class → **recompute `inverse` (persisted `.spr` `HistoryEdit.inverse` goes stale)**.

---

## 1. The `🧬️schema` framework module and per-artifact schema directories

### 1.1 Two levels: schema-of-schemas and artifact schemas

`FW/🧬️schema` is the *registry and contract* for schemas, not a schema of any artifact:

| File | Role | Key lines |
| --- | --- | --- |
| `FW/🧬️schema/🔣️.json` | draft-07 schema of the scope catalog contract: `SchemaFormat` (rust, typescript, graphql, jsonschema, protobuf), `FacetLeaves`, `SchemaExport`, `ScopeSchemaExports`, `SchemaExportEntry(ies)`, `SchemaResolveError`, `ValidationDiagnostic`, `EntityKind(Catalog)` | root L1-20 |
| `FW/🧬️schema/🟦️.ts` | TS twins: `FacetLeaves` L3-9, `ArtifactSchemaDescriptor{id,artifact,snapshot,diff,mutations}` L12-18, `ArtifactInferenceDescriptor` L26-29 (inference is a *sibling*, not a 5th facet), `GRAPHQL_STATE_PREAMBLE` L38, `STATE_CLASSES` L47, `SCHEMA_FORMATS` L171, `RESERVED_FACET_EXPORT_IDS` L193, registries L80-166, `parseEntityKind*` L280-323 | |
| `FW/🧬️schema/⚛️component/🦀️.rs` | Rust: `SchemaCatalog{register_json,load_json,validate}` L73-119, `ArtifactSchemaFields` L143, `JSON_SCHEMA_DERIVED_KEY="x-semio-derived"` L153, `ArtifactSchemaDescriptor` (4 facets × 5 leaves) L192-200, `register_artifact_schema_descriptor(s)` L353/L381 (kernel catalog + `schema://` export registry) | |
| `FW/🧬️schema/✅️validator/🦀️.rs` | **owned draft-07 validator** (no external crate). `$ref` siblings may only be annotations; keys starting with `x-` never count as constraining siblings (L512, L597); `multipleOf` uses an 8·ε float tolerance (L1098-1105); only 7 `format`s assert (`ASSERTED_STRING_FORMATS`) | L152, L512, L1098 |
| `FW/🧬️schema/📇️registry/🦀️.rs` | `schema://<scope>/<Export>?format=` resolver, `register_scope_facet_leaves` | |
| `FW/🧬️schema/✨️derive/🦀️.rs` (+`⚙️expansion`) | `#[derive(ArtifactSchema)]` with `#[artifact_schema(id)]`, `#[state(artifact|config|presence|transient)]`, `#[derived]`, `#[child(kind)]`, `#[link_slot(roles)]` — the **only** per-field Rust attribute set that mirrors a JSON annotation (`x-semio-state`, `x-semio-derived`, `x-semio-child*`, `x-semio-link*`) | L11 |
| `FW/🧬️schema/📽️projection/🦀️.rs` | Rust→TS type projection for *framework wire types* (`SchemaMetadata{name,version,typescript}`); includes `ActionArgDef`, `ArgSchema`, `ConfigFieldShape`, ... (2,206 lines, TS text embedded in Rust) | |
| `FW/🧬️schema/🏷️entity-kinds`, `🤖️generated` | the only schema-driven **code generation** in the module: `🔣️.json` catalog → generated `🟦️.ts`/`🦀️.rs` (+ Go) via `schema-entity-catalog` | |

`REPO/📚️library/🔣️taxonomy.json` (30,847 lines) is the single authority for the layout. Relevant keys (verified by reading the JSON):

- `roles`: `plugin, framework, product, hub, s-module, extension, tool`.
- `langs`: `🦀️rust, 🟦️typescript, 🟨️javascript, 🐹️go, 🐍️python, 🔷️dotnet`.
- `schemaFormats`: rust (snake), typescript (camel), graphql (camel), jsonschema (camel), protobuf (snake), wit (kebab). `schemaFacetKinds`: `🧬️data` (normative `🔣️jsonschema`, formats jsonschema+rust+ts+graphql+proto) and `📜️interface` (normative `wit`). `schemaJsonDialect` = draft-07. `schemaChildDirs` = `📸️snapshot, 🔺️diff, 🧬️mutations, 💡️inferences`.
- Mutation keys: `mutationDirectoryPattern` (`^.+️[a-z][a-z0-9]*(?:-[a-z0-9]+)+$`), `mutationBehaviorFacetDirs` (`🦠️mutation, 🔺️diff, ↩️inverse`), `mutationOrganizationalFacetDirs` (`🧩️plan, 📝️text, 💾️binary, 🧬️schema`), `mutationPayloadSchemaAuthority` (`descriptorField: payloadSchema`, `one-canonical-no-competing-descriptor`, dialect draft-07), `mutationAggregateSources` (subset→aggregate assembly, e.g. draw/note/fem/sequence), `mutationDomainOwners` (verb-per-domain rosters, architect), `mutationCatalogProjection` (canonical case pair for tests).
- `schemaExportResolution`: `idBase https://json.schemas.assets.semio-tech.com/`, `mutationLeafFacetFilename: schema.json`, `mutationAggregateFacetFilename: mutations.json`, `mutationScopeSegment: mutation`, `exportsKeyword: $defs`, generator `bun ./📜️script.ts schema generate`.
- `artifactSchemaSpecFileKinds` also lists the text facets `🚪️io/🧬️mutations/📝️text: json` — i.e. text/binary wire grammars are per-aggregate spec dirs.

### 1.2 Per-artifact layout (verified on puzzle 2d)

```
<plugin>/🗿️artifacts/<artifact>/🏅️standards/🔖️<v>/🪆️subsets/<subset>/
  🧬️schema/
    🔣️.json 🦀️.rs 🟦️.ts 🔗️.graphql 🛰️.proto            # artifact facet (5 leaves)
    📸️snapshot/  🔺️diff/  🧬️mutations/  💡️inferences/    # 4 facets x 5 leaves + 📝️text + 💾️binary spec dirs
    🧬️mutations/<emoji><verb>-<noun>/                      # one directory per mutation kind (leaf)
  ✏️editor/  👁️viewer/  🚪️io/  📚️examples/  🧫️fixtures/  🧪️tests/  🔮️oracles/
```

- `P2D/🧬️schema/🦀️.rs` L63-95: `puzzle2d_artifact_schema_descriptor()` builds `ArtifactSchemaDescriptor` from **twenty `include_str!` leaves** (docstring: "twenty handcrafted schema leaves"). The Rust artifact struct derives `ArtifactSchema` (`#[state(artifact)]` per field, L7-25).
- Every facet ships 5 formats + text (`📖️.grammar.semio`, `🔤️.ebnf`, `🅰️.g4`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`, `🟦️.ts`, `🦀️.rs`) and binary (`📡️.protocol.semio`, `🥋️.ksy`, `🌶️.spicy`, `🔠️.abnf`) spec files (`P2D/🧬️schema/🧬️mutations/📝️text`, `💾️binary`).
- Plugin-level `✏️s/🔌️plugins/🧩️puzzle/🧬️schema/🔣️.json` (`$id …/s/puzzle/schema.json`) is *not* a mutation schema (publication authority table).
- `🚪️io` (per artifact `P2D/🚪️io/🦀️.rs`, `🟦️.ts`) = import/export dialect glue (`stdio.json`, `stdio.dxf`, ...), plus io text/binary specs; framework `FW/🚪️io/🦀️.rs` = dialect vocabulary + typed io dispatch registry. Neither declares mutation inputs.
- State lanes below the artifact (`✏️editor/🎚️config|👥️presence|🫧️transient/🧬️schema`, `OS/../🎚️config/🧬️schema/🧬️mutations`) have their own leaf mutations (177 `surface-state-lane` scopes; e.g. `set-camera`, `set-engagement-input`).

### 1.3 Codegen pipeline: what is generated from what (verified)

| Direction | Exists? | Evidence |
| --- | --- | --- |
| JSON Schema → Rust / TS / GraphQL / proto types for artifacts | **No.** All five leaves are handcrafted; the descriptor `include_str!`s them. | `P2D/🧬️schema/🦀️.rs` L62; taxonomy `_languageNeutralityComment` |
| Parity checking between formats | Yes, as repo laws/tests: `repo-schema-artifact-field-parity-law`, `-state-parity-law`, `-diff-coverage-law`, `-type-name-parity-law`, surface + inference law families (taxonomy `semanticDirectoryKinds`), lint codes `schema-export-incomplete`, `schema-instance-invalid`, `schema-mutation-leaf-id`, `schema-mutation-leaf-schema-absent` (`REPO/🧪️test/🟦️.ts` L4600-4640). Mutation **leaf** payload ↔ Rust struct parity is enforced only *indirectly* (fixtures validated against the leaf schema + Rust round-trip tests). | |
| `bun ./📜️script.ts schema generate` | Regenerates only the **scope catalog** (`REPO/📚️library/🔣️schema-catalog.json` 3,540 scopes, `📓️schema-catalog.md`); no type generation. | taxonomy `schemaExportResolution`; catalog `level` histogram: mutation-leaf 2,834, product-module 180, artifact-standard-subset 180, surface-state-lane 177, framework-module 136, plugin-root 13, hub-area-module 12, ... |
| Rust → TS for framework wire types | Yes (`FW/🧬️schema/📽️projection/🦀️.rs`; generators `actor-typegen`, `shell-typegen`, `framework-manifest`, `ui-contract`, `ui-axes`, ... in taxonomy `generatorContracts`). This is **code-first** (Rust struct is the source). | |
| Python / C# | **None** for `s` plugins: 0/2,834 mutation leaves have a `.py`/`.cs` file. The `.🧬semio/🦑️repo/📊️metrics/schema-py|net|ts.json` files are a Dec-2025 legacy scan of an older schema. JSON Schema is the only artifact a Python/C# implementation can consume today. | scan |

Caveat found while reading: the per-aggregate GraphQL/proto mutation facets are not trustworthy — `P2D/🧬️schema/🧬️mutations/🔗️.graphql` L3-9 and `🛰️.proto` L6-16 declare a snapshot-shaped `Puzzle2dMutation { schema, camera, nodes, edges, meta }`, not the 33 kinds (verified for puzzle 2d only; only 371 of 2,834 leaves have their own graphql/proto).

---

## 2. Anatomy of one mutation kind (with the exact contracts)

### 2.1 Files of a leaf (`P2D/🧬️schema/🧬️mutations/📍move-node/`)

| File | Content |
| --- | --- |
| `🔣️.json` | **14-field descriptor** (below) |
| `🦀️.rs` | payload struct + `MutationKind` impl (label, target, semantics), builder fn |
| `🧬️schema/🔣️.json` | payload JSON Schema (`$id …/s/puzzle/puzzle2d/mutation/move-node/schema.json`) |
| `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs` | behaviour facets (`mutationBehaviorFacetDirs`); 1,860 leaves have `🔺️diff/🦀️.rs`; 70 leaves put the payload in `🦠️mutation/🦀️.rs` instead |
| `🧪️tests/<case>/🦀️.rs` | + fixtures in `P2D/🧫️fixtures/🧬️mutations/<kind>/<case>/{📸️snapshot/⬅️before,➡️after,🦠️mutation,🔺️diff,🎯️outcome}/🔣️.json` |
| optional `🟦️.ts` | TS payload mirror (draw also has `🔺️diff/🟦️.ts`, `↩️inverse/🟦️.ts`) |

### 2.2 The 14-field descriptor (strict)

`FW/📡️replication/🎮️mutation/🦀️.rs` L371-386 `MutationLeafDescriptor {schema_version, owner, semantic_kind, display_name, emoji, aggregate_variant, payload_schema, text_opcode, binary_tag, invertibility, diff_participation, outcome_classes, composition, required_language_surfaces}`; hand-written `ToValue` L391-410; validation L429+; `MutationLeaf` trait (`DESCRIPTOR`, `PROVENANCE`) L928-931. The derive parses the JSON and **rejects any key outside the fourteen** (`OS/🗣️dsl/✨️derive/🦀️.rs` L376 `MUTATION_LEAF_DESCRIPTOR_KEYS: [&str; 14]`, L378-`parse_mutation_leaf_descriptor`: "must contain exactly the fourteen schema fields"; `expand_mutation_leaf` L540 reads the descriptor file at compile time and `include_str!`s the taxonomy projection + descriptor for rebuild tracking). The same contract is also pinned in `OS/📡️spr/🎮️command/🧬️schema/🔣️.json`. Adding a field means: kernel struct + derive + that schema + **2,834 descriptor files**. Profile of the descriptors: 2,832 atomic / 1 composite (`s.cad.aec-building.mutation.create-building-storey`); invertibility 2,824 explicit-mutation / 6 self / 3 plan; diffParticipation 2,611 detect / 220 apply-only; `textOpcode` set on 1,067, `binaryTag` on 2,350; top verbs change 708, set 470, remove 275, delete 229, create 226, replace 170, insert 166, rename 127, move 71, add 57, update 56, reorder 46, resize 19, scale 14, rotate 10.

### 2.3 Rust side

- Payload (`P2D/…/📍move-node/🦀️.rs` L9-19): `#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)] #[mutation_leaf(contract = ::protocol)] #[value(rename_all = "camelCase")] #[dsl(keyword = "move-node")] pub struct MoveNode { id: String, new_x: f64, new_y: f64 }`.
- `impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation>` L26-41: `SEMANTICS: SemanticDescriptor{verb:"move", entity:"node", kind:"move-node", record:"MovedNode"}`, `diff`, `inverse`, `label() -> LocalizedLabel::native("Move node \"{id}\"", "Knoten \"{id}\" verschieben")`, `target() -> vec![id]`.
- Trait definitions: `OS/📡️spr/🎮️command/🦀️.rs` `SemanticDescriptor` L198-203, `MutationKind` L219-254 (`SEMANTICS`, `diff`, `inverse`, `label`, `timestamp`, `target`, `may_emit_foreign_steps`, `foreign_steps`), `SemanticMutation` L261-267 (`kinds`, `semantics`, `label`, `target`), `APPROVED_VERBS` L112-157 (compile-time gate), `CompositeMutationKind` L789 (`plan`), `MutationDescriptor` + process-wide `MutationDescriptorRegistry` L437/L525-585 (id, schema version, state class, leaf descriptor, semantics, fingerprint).
- Kernel trait `Mutation<P>` (`FW/📡️replication/🎮️mutation/🦀️.rs` L174-234): `DESCRIPTORS`, `descriptor()`, `diff`, `inverse`, `conflict_target`, `undo_policy`, `state_class`, `foreign_steps`; **bound `ToValue + FromValue`**. `OpText` L1209 / `OpBinary` L1223 / `DiffCodec` L1245 (per-aggregate codecs), `MutationOutcome` L1066 with classes applied / no-op / empty / disjoint / rejected (L295).
- Aggregate: `P2D/🧬️schema/🧬️mutations/🦀️.rs` L31-70 `#[derive(… ToValue, FromValue, dsl::DslEnum, dsl::Mutations)] #[value(tag = "mutation", rename_all = "camelCase")] #[mutations(snapshot = Puzzle2dSnapshot, diff = Puzzle2dDiff, schema = "puzzle.puzzle2d")] enum Puzzle2dMutation { …33 variants }`, `KINDS` L77-111, re-exports L115-147. `#[derive(Mutations)]` (`OS/🗣️dsl/✨️derive/🦀️.rs` L1596 `expand_derive_mutations`, region L1552-1840) generates the per-variant arms for `diff`, `inverse`, `descriptor`, `semantics`, `label`, `target`, `may_emit_foreign_steps`, `foreign_steps`, `timestamp`, the `DESCRIPTORS` table, compile-time assertions (`SEMANTICS.kind == kebab(variant)`, approved verb) and registration calls. **It has no arm for inputs — this is the natural extension point.**
- Registry/dispatch: plugin `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs` L73 `.editor_mutation_roster::<Puzzle2dPlayApp>()` (37 files under `✏️s` mention the roster API) → builder `OS/🔌️plugin/🏗️builder/🦀️.rs` L527 → `commit_owner_mutation_roster` `OS/🔌️plugin/🦀️.rs` L4956 → `WireMutationRosterEntry {mutation_id "<schema>#<kind>", verb, entity, kind, record, contributor?, artifact_kind?}` L4823 → `wire_list_artifact_mutations` (L34620/L39218). **The roster row has no `inputs`.** Contributed (plugin-supplied) mutations use `ContributedMutationSemantics` with the same shape.
- Storage of applied edits: `Edit<Op>{id, actor, forwards, inverse, mutation_meta, description, coalesce_key, ...}` `FW/📡️replication/🎮️mutation/🦀️.rs` L1488; `MutationMeta{…, semantic_kind, label, group_id, origin, payload_hash}` L1362; persisted `HistoryEdit{ops: Vec<OpPayload{text?,binary?}>, inverse: Vec<OpPayload>, meta, lane, coalesce_key}` `OS/📡️spr/📜️history/🦀️.rs` L105-170 (`OpPayload` L139, `HistoryOpMeta` L145). Payloads are opaque validated bytes to that layer.

### 2.4 JSON Schema side

- Leaf (`…/📍move-node/🧬️schema/🔣️.json`): `type object`, `additionalProperties false`, `required [mutation,id,newX,newY]`, `mutation: {const:"moveNode"}`, `id: string`, `newX: number`, `newY: number`. The aggregate `P2D/🧬️schema/🧬️mutations/🔣️.json` is `oneOf` of 33 `$ref`s to `https://json.schemas.assets.semio-tech.com/s/puzzle/puzzle2d/mutation/<kind>/schema.json`.
- **Runtime does not read leaf schemas.** `MutationLeafDescriptor.payload_schema` (`"🧬️schema/🔣️.json"`) is only checked non-empty (`FW/…/🎮️mutation/🦀️.rs` L456); no non-test code loads it. The runtime-embedded schema material is the 20 facet leaves per artifact (incl. the aggregate with unresolved external `$ref`s) via `with_artifact_schema_registry`.
- Vocabulary of annotations: strict Ajv oracle `OS/🧪️tests/🧬️schema-oracle/🟦️.ts` registers exactly the keys in `OS/🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json` (24 keys: `x-semio-binary, -child, -child-kind, -child-standard, -child-subset, -constants, -demo-tables, -derived, -family-options, -formats, -invariant, -link, -link-roles, -mutation, -mutationKinds, -note, -owner, -persistence, -retained-discriminator, -state, -taxonomy-groups, -taxonomy-sections, -toolRun, -transfer-retry`). "an undeclared or misspelled `x-semio-*` keyword still fails at compile time". Note `x-semio-unit`, `-shape`, `-ref`, `-format`, `-entity-kind`, `-roles`, `-title` are used by Rust emitters/tests but are **not** in this vocabulary (they only appear in generated MCP tool schemas / unit-test schemas).
- Field-level annotation precedent: snapshot schema properties carry `"x-semio-state": "artifact"` next to `$ref` (e.g. `P2D/🧬️schema/📸️snapshot/🔣️.json` L13-46) — the same sibling-annotation placement a UI annotation would use.

### 2.4b Statistics (all 2,834 mutation-leaf payload schemas, scanned)

| Fact | Count |
| --- | --- |
| Leaf payload schemas / total property nodes (any depth) | 2,834 / 36,489 |
| Top-level payload fields incl. `mutation` discriminator / real inputs | 5,608 / 5,025 (avg ≈ 2 per leaf, max 16; 2,624 leaves have ≤ 3 fields) |
| Top-level field types | string 1,520 · integer 1,455 · number 660 · const 583 · `$ref` 413 · object 395 · array 220 · oneOf 197 · boolean 111 · enum 31 |
| Most frequent names | `id` 849, `mutation` 601, `index` 458, `newName` 106, `snapshot` 76, `path` 75, `name` 72, `newValue` 71, `value` 68, `memberId` 57, `zoneId` 50, `key` 45, `layerId` 34, `x`/`y` 22 |
| Distinct real-input names / coverage by top 10 / 25 / 100 / 200 names | 1,484 / 37.5 % / 45.2 % / 60.0 % / 68.1 % |
| Keywords present on property nodes | `minimum` 4,669 (almost all integer `0`), `maximum` 921, `title` 891, `description` 600, `enum` 498, `minItems` 313, `maxItems` 311, `format` 213, `exclusiveMinimum` 21, **`default` 7**, `pattern` 3, **`multipleOf` 0** |
| `type: number` properties with any bound | **25 of 2,541** (wfc 13, layout 5, energy 2, reasoning 2, gis 1, mathematical 1, draw 1) |
| `x-semio-*` on properties | 1 (`x-semio-state`); top-level `x-semio-mutation` on 120 gltf leaves only |
| Leaves with `$ref` / local `$defs` | 14 / 431 |
| Leaves deriving `dsl::MutationLeaf` / implementing `MutationKind` / with `label()` en+de / with `target()` / deriving `DslRecord` | 2,831 / 2,823 / 2,805 (`LocalizedLabel::native`) / 2,412 / 888 |
| Language files: direct `🦀️.rs` / facet `🦠️mutation/🦀️.rs` / `🟦️.ts` (direct+facet) / diff `🟦️.ts` / inverse `🟦️.ts` / own graphql or proto | 2,764 / 70 / 499+125 / 103 / 103 / 371 |
| Leaves still deriving serde outside tests (heuristic regex, approximate) | ~75 (note, ...) |
| Files with an aggregate enum deriving `dsl::Mutations` (non-test, grep) | ~213 |
| Wire layouts by single-line `#[value(tag…)]` grep (approximate) | `tag="mutation"` camelCase 89 · `tag="mutation", content="payload"` 20 · `tag="kind"` 14 · `tag="operation"` 7 · ~108 without a single-line tag (externally tagged or multi-line attribute) |

---

## 3. Inventory: does any mutation/action argument carry UI metadata?

**Mutation inputs: no.** Actions/config/UI nodes: yes, in disconnected vocabularies.

| # | Vocabulary | Where | Widget kinds | min/max/step | unit | snaps | labels i18n | Attached to |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A | `ActionArgDef` + `ArgSchema` + `ArgPresentation` + `ActionArgControl` | `FW/🛂️manifest/🦀️.rs` `ArgFormat` L120, `ArgSchema` L158 (String{options,min_len,max_len,pattern,format} / Number{min,max,step,integer,unit} / Boolean / Vec3{unit} / Array / Object / Any), `ArgPresentation` L239 (Slider, IconSelect, Multiline, Hidden), `ActionArgControl` L275, `ActionArgDef` L335 (`id, label: LocalizedLabel, schema, presentation, required, default, description`), `control()` L492, `json_schema()` L521; TS twin `argControl()` `FW/🛂️manifest/🟦️.ts` L559; TS resolution helpers `FW/🧩️action-argument-resolution/🟦️.ts` | text, number, slider, toggle, select, vec3, iconSelect, artifactKind, surfaceApp | yes | yes (free string) | **no** | **yes** (`LocalizedLabel` = locale × terminology matrix, axes `en,de` × `native,reuse` from `UI/🎚️axes/🔣️.json`) | **actions** (`ActionDefinition.args`), rendered by shell dialogs in wgpu (`OS/📺️renderer/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` L6528-6549, L21686-21707) and React (`…/🛠️ShellHelpers/🟦️.tsx` L1915-1956). ≈ 800 calls of `ActionArgDef::number|index|select|toggle|text` plus 33 `::slider` (includes tests). JSON Schema projection emits `minimum/maximum/multipleOf(step)/x-semio-unit/x-semio-format`. |
| B | `ConfigFieldShape` / `ConfigFieldSpec` / `CommandFieldSpec` | `FW/🛂️manifest/🦀️.rs` L5963/L5991/L6029 | Number, Toggle, Text, Select, Record | yes | no | no | **no** (`label: String`) | app config + command grammar; only 2 `Number` uses in the tree → effectively dead duplicate of A |
| C | `WindowEngagementControl` | `UI/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` L1331 (`Slider{min,max,step,unit}`, `Stepper{min?,max?,step?,unit}`, `Ring{options}`, `ToggleGroup`, `Select`); TS `EngagementSliderControl`… `UI/🎯️targets/⚛️react/🟦️.tsx` ~L7343 | slider, stepper, ring (dial), toggleGroup, select | yes | yes | no | plain `String` (already resolved) | interactive tool "engagement" panel (the closest existing set of *numeric gesture controls*) |
| D | `ui_contract::SliderProps{value,min,max,step,unit}`, `NumberStepperProps{value,step,uniform,min,max}`, `UiInputNode{input_kind,min,max,step}`, `UiFieldNode`, `UiGroupNode` | `UI/🧬️contract/🧩️component/🦀️.rs` L439/L454; `UI/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` L2025-2319 | slider, stepper, input | yes | slider only | no | `Label` | rendered UI nodes (values embedded) |
| E | React `Slider` element | `UI/🧱️elements/🎚️Slider/🟦️.tsx` L73, L216-237 | slider | yes | – | **`snapValues?: number[]`, nearest-snap logic** | – | element only; **not** in the Rust wire contract (`SliderProps` has no snaps) |
| F | `dsl::Shape` / `FieldSpec` / `RecordSpec` (text grammar of records) | `OS/🗣️dsl/🧬️schema/🦀️.rs` L33 (`Quantity(unit)`, `Angle`, `Range` = `(lo..hi[,step])`, `Ref(kind)`, `Coord`, `Dir`, `Dim`, `Count`, `Enum`, …), L120, L181; JSON projection `shape_json_schema` L218 (`x-semio-unit`, `x-semio-ref`, `x-semio-shape`) ; derive attrs `#[dsl(unit="GPa" / angle="deg" / refs="material" / coord / dir / key / positional / table / block)]` `OS/🗣️dsl/✨️derive/🦀️.rs` L590-620; unit catalog `UNITS` with dimension + SI factor `OS/🗣️dsl/🔤️token/🦀️.rs` L285-360 | value kinds only | `Range` has step | ✔ (validated catalog) | no | no | **record fields of the 888 `DslRecord` leaves** (text grammar), not widgets |
| G | Hand-written per-app inspector tables | e.g. draw `DRW/✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs` L36-71 (`Field{key,label,kind,toggle,min,max}` table: `("opacity", …, Some(0.0), Some(1.0))` at L50), L118 `step(0.01)` for `opacity`/`traceThreshold` else `0.1`, L65/L87 rotation shown in degrees (`to_degrees()`) and sent as `rotationDegrees` though stored in radians; framework helpers `ui_inspector_stepper_field/vec3_group/toggle_field/mixed_*` `UI/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` L2634-2790 | stepper, vec3 group, toggle, select | partly | – | no | app `LabelText` | the only place today that knows opacity is 0..1 |
| H | Schema-driven inspector annotations | stdio `🪟️details` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs`, see §6): `x-semio-title` / `title` as `{ "en": …, "de": … }` (or plain string = English) via `localized_schema_annotation` L848 | – | uses `minimum/maximum/enum/default/examples` for validation and templates only | – | – | **yes**, the only JSON-Schema-level i18n convention in the repo | JSON Schema properties |

No `ui:widget`, `x-ui`, `x-widget`, `x-control` or `x-semio-widget` annotation exists anywhere (searched `🧰️framework` and `✏️s`; hits were unrelated strings). "Snapping points" exist in exactly three unrelated forms: React `snapValues` (E), the puzzle window runtime `grid_factor`/`set-grid-snap-enabled` command (window state, `P2D/✏️editor/🎮️commands/📐️set-grid-factor`), and note's document field `snapGridSpacing` (`NOTE/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing`, payload `new_spacing: Option<f64>`, schema `{type:number}`), i.e. the snap source can be **document data, config/window state, or a static list** — the descriptor must be able to say which.

Tickets: `26/07/21/SELECTION-DETAILS-AS-PROPER-EDITABLE-TREES-ACROSS-ALL-APPS` (closed) introduced the stepper/vec3-group inspector helpers (G) and dot-path `field` patch actions (`value` absolute / `delta` relative) for puzzle 3d/2d/5d, compose, cad; ~20 other apps were explicitly deferred. Its API names (`UiVec3Node`, `dispatchUiAction`) have since been replaced; only the `ui_inspector_*` helpers survive.

---

## 4. Value model and generic read / edit / re-encode

1. **`DslValue`** (`FW/🌱️value/🦀️.rs`): schema-erased JSON-equivalent tree (`Null, Bool, Number{UInt|Int|Float}, String, Array, Object(Vec<(String,DslValue)>)`) — ordered objects, integer fidelity kept. `UiValue` (`UI/🧬️contract/🎬️action/🦀️.rs` L1815) is a separate bounded type for UI wire args and deliberately has **no** `ToValue`/`FromValue`; conversions live in the os-kernel.
2. **`ToValue`/`FromValue`** (`FW/🌱️value/🔁️codec/🦀️.rs` L21/L42): `to_value`, `value_at_path`, `value_shape_at_path`, `value_key_at_path`; `from_value`, **`edit_value_at_path(path, ValueEdit)`** with `ValueEdit::{Set, Insert, InsertAt{index,value}, Remove}` (L63), `ValueShape` (L73), `edit_through_value` (L123: edit through the value tree, decode back so every `from_value` invariant holds, target unchanged on failure). Path edit wire schema: `FW/🌱️value/🔁️codec/🧭️path/🧬️schema/🔣️.json` (`{path:[…], edit:{operation:set|insert|remove, value}}`, RFC 6901 segments, `maxItems 128`). `#[derive(ToValue, FromValue)]` (`FW/🌱️value/✨️derive/⚙️expansion/🦀️.rs`) implements typed path edits for structs and for internally / adjacently / externally tagged enums; it returns `"cannot edit an enum tag in place"` for the tag segment (L1470-1481). Attributes: `rename_all`, `tag`, `content`, `deny_unknown_fields`, `default`, `skip_serializing_if`, `with`, `crate`.
3. **Bounds**: `Mutation`, `MutationKind`, `MutationDiff`, `Inference`, `CompositeMutationKind` are all bound on `ToValue + FromValue` (serde is being removed: ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`); ~75 leaves still derive serde outside tests (heuristic).
4. **Reflection descriptors** (`dsl::RecordSpec`): only `#[derive(dsl::DslRecord)]` leaves (888/2,831 = 31 %; puzzle, energy, block, fem, remodel, note, cad, draw, gis: ~100 %; norm, architect, wfc, lowpoly, layout: ~0 %) expose `__dsl_spec()`/`__dsl_to_record()`/`__dsl_from_record()` (`OS/🗣️dsl/✨️derive/🦀️.rs` L1200) and the aggregate's `DslVariants::variants()` (used by `OpText`). Not universal, so it cannot be the descriptor of record.
5. **Encodings**: `OpText::print_op/parse_op` (one line, positional grammar, e.g. `move-node = "move-node" SP id SP number SP number` in `MUT/📝️text/📖️.grammar.semio`), `OpBinary::encode_op/decode_op` (`dsl::variants_binary::encode_tagged_op(include_str!("…📡️.protocol.semio"), self)`, `MUT/📝️text/🦀️.rs` L30-40), canonical JSON via `pack::json` / `dsl::json` (`from_json_str`, `to_json_string`), `.spr` frames. Annotations on a schema do not change any of these wire formats.
5b. **Wire layouts of the tagged aggregate are not uniform** (see §2.4b): puzzle 2d/draw are internally tagged `{"id":"node-a","mutation":"moveNode","newX":5.0,"newY":7.0}`; lowpoly is externally tagged `{"MoveObject":{"id":"obj-hull","newPosition":[2.5,0.0,-1.5]}}`; forms uses snake_case keys (`step_id`, `block_id`, `to_step_id`) while most artifacts use camelCase. A generic editor must therefore **not** assume where the payload sits inside `aggregate.to_value()`.

Generic edit recipe that works with today's contracts (verified pieces, composed = inferred):

```
stored op (typed Op | OpPayload{text|binary}) ──decode──▶ Op
Op ──SemanticMutation::semantics()/label()/target()──▶ kind, en/de title, target ids
Op ──to_value()──▶ DslValue (layout-dependent!)
   ──edit_value_at_path([field], ValueEdit::Set(v))──▶ (aggregate derive handles tag; leaf structs handle payload)
   ──from_value()──▶ Op' (invariants of FromValue enforced)
Op' ──MutationKind::diff(base)──▶ MutationOutcome{applied|no-op|empty|disjoint|rejected + messages}
Op' ──inverse(base)──▶ new inverse   (persisted HistoryEdit.inverse and MutationMeta.payload_hash become stale)
```

---

## 5. Four concrete declaration chains

### 5.1 Puzzle 2d `move-node` (absolute reposition; 33-kind aggregate)

- Schema: descriptor `MUT/📍move-node/🔣️.json` (binaryTag 2, textOpcode `move-node`, explicit-mutation, detect, outcomes applied/no-op/rejected, surfaces rust/json-schema/text/binary); payload schema `MUT/📍move-node/🧬️schema/🔣️.json` (`mutation` const `moveNode`, `id`, `newX`, `newY`, all bare types); TS `MUT/🟦️.ts` `interface MoveNode {id:string; newX:number; newY:number}`; text grammar `MUT/📝️text/📖️.grammar.semio`.
- Rust: `MUT/📍move-node/🦀️.rs` L9-41 (struct + `MutationKind`), `🔺️diff/🦀️.rs` L6-20 (patches one node `replacement`; `mutation.target-missing` error, `mutation.no-op` warning), `↩️inverse/🦀️.rs` L6-11 (`move_node(id, node.x, node.y)`, missing target ⇒ `[]`).
- Registry/dispatch: aggregate `MUT/🦀️.rs` L31-70 → `apply_puzzle2d_mutation` L322 → `vcs::apply_mutation`; roster registration `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs` L73; bridges for the play snapshot `MUT/🦀️.rs` L420-560. **How it is produced**: action `translateSelection` args `dx, dy, step` (`P2D/✏️editor/🎮️commands/🚀️translate-selection/🦀️.rs`; declared with `ActionArgDef::number("dx"…).default_value(&0.0)` — no min/max/step, `P2D/✏️editor/🦀️.rs` L5548-5551) mutates a scratch `serde_json::Value` fixture, then `puzzle2d_document_delta_operations(before, after)` (`MUT/🦀️.rs` L396, call site `P2D/✏️editor/🦀️.rs` L2994) → `puzzle2d_snapshot_mutations` (L152; `move_node` at L164) emits **absolute** positions; a streamed drag is folded into one `Edit` through `puzzle2d_gesture_coalesce_key` (`P2D/✏️editor/🦀️.rs` L2862).
- Inputs carried: target `id` + final `newX/newY`. The gesture parameters (`dx,dy,step`, rotate `angle`, scale `factor`) never reach history.

### 5.2 Lowpoly `move-object` (3D transform, f32 vector)

- `LP/↗️move-object/🦀️.rs`: `MoveObject { id: String, new_position: [f32; 3] }`, derives `MutationLeaf/ToValue/FromValue` only (no `DslRecord`, `textOpcode: null`, `binaryTag: 6`); label `Move object "{id}"` / `Objekt "{id}" verschieben`; `target()=[id]`.
- Schema `LP/↗️move-object/🧬️schema/🔣️.json`: `newPosition: array<number>, minItems 3, maxItems 3`; **no `mutation` const** (the lowpoly aggregate `LP/🦀️.rs` derives `Mutations, ToValue, FromValue` without `#[value(tag)]` → externally tagged, fixture `{"MoveObject":{…}}`). Inputs: object id, vec3 position, no unit/step/bounds; f32 in Rust vs JSON number.

### 5.3 Draw `set-layer-opacity` (slider-shaped scalar; facet layout)

- Location `DRW/🎨️style/🧬️schema/🧬️mutations/🌫️set-layer-opacity/`: payload in **`🦠️mutation/🦀️.rs`** (`SetLayerOpacity{layer_id, opacity: f64}`; derives `ToValue, FromValue, DslRecord, MutationLeaf`), TS mirrors `🦠️mutation/🟦️.ts`, `🔺️diff/🟦️.ts`, `↩️inverse/🟦️.ts`; descriptor `🔣️.json` (`diffParticipation: apply-only`, binaryTag 2, textOpcode `set-layer-opacity`).
- Schema: `layerId: string`, `opacity: number` — **no `minimum`/`maximum`**. The Rust diff only checks `is_finite()` (`🔺️diff/🦀️.rs`). The 0..1 range and step 0.01 live only in the properties panel (`DRW/✳️any/✏️editor/📌️panels/🔍️properties/🦀️.rs` L50, L118) and in an action help string ("from 0 (invisible) to 1", `✏️editor/🦀️.rs` L2133).
- Inspector → mutation bridge is a hand-written per-artifact function: `drawing_op_for_layer_field(doc, layer_id, field, value)` (`DRW/✳️any/🧬️schema/🧬️mutations/🦀️.rs` L53) with `parse_layer_field_input` L41 — exactly the generic "field key → mutation kind" mapping a descriptor should make declarative (also handles storage/display unit mismatch: `transformRotation` shown in degrees, sent as `rotationDegrees`).

### 5.4 Note `drag-blocks` (relative multi-target delta) and `change-snap-grid-spacing`

- `NOTE/🧱️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🦀️.rs`: `DragBlocks { ids: Vec<String>, dx: f64, dy: f64 }`, `SemanticDescriptor{verb:"drag", entity:"blocks", record:"DraggedBlocks"}`, `target() = ids`; schema `ids: array<string>, dx/dy: number` (required order `dx, dy, ids`); still derives serde outside tests. This is a genuine **relative offset** input; `move-block` (same subset) is the absolute twin.
- `NOTE/🎨️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing`: `new_spacing: Option<f64>`; schema `{"newSpacing":{"type":"number"}}` (no minimum although spacing must be positive). This is the document datum that would parameterize snapping of the drag inputs.
- Additional shapes seen: forms `move-block-to-step` (`step_id, block_id, to_step_id, index: usize`; `target() = [step_id, block_id]` = outermost-first address; schema `index: integer, minimum 0`), cad `move-objects` (`pane` enum + `#[dsl(table)] placements: Vec<{objectId, newOrigin: [f64;3]}>`, "gumball-drag op").

---

## 6. Existing generic editors of schema-described values

1. **stdio "snapshot details" tree** — `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs` (2,016 lines): `DslSnapshotDetailsProvider<S: ArtifactDsl + ToValue>` L921 reads the snapshot's `ToValue` tree and the bundled snapshot JSON Schema (`snapshot_schema(id)` L899 via the schema registry; `bundle_snapshot_schema`, `$ref`/`allOf`/`oneOf` resolution, discriminated unions `variant_label`), derives enum/select, toggle, scalar inputs (`scalar_control` L1593), add/remove/insert/rename/move controls with `minItems/maxItems/minProperties` limits, templates from `default/const/examples/enum`, localized labels through `localized_schema_annotation(schema, "title", locale)` L848 (`x-semio-title` or `title` as `{en,de}`), fallback `humanized_schema_key` L867. Edits go through the `SnapshotPatch`/`SnapshotValuePatch{path, edit: Set|Insert|InsertAt|Remove}` contract (`…/✏️editing/🩹️patch/🦀️.rs`, 1 MiB cap, exact inverse patches) which is built on `ValueEdit`. Used by ~20 stdio artifact editors (xml, zip, xlsx, wav, txt, tsv, tiff, svg, …). **Limitations**: numbers become `InputKind::Text` (L1600-1603), min/max/step are not turned into widgets, no snaps/unit/grouping, only stdio consumes it.
2. **Shell action dialogs** (A above): fully declarative, both renderers, localized, with slider/number/select/toggle — but the descriptor is per *action*.
3. **Per-app inspector builders** (G above) with the framework `ui_inspector_*` helpers (mixed-state aware stepper/vec3/toggle fields, dot-path patch actions with `value`/`delta`).
4. **DSL language service** (`OS/🗣️dsl/🧠️lsp`, `🪟️viewport`) edits text against `RecordSpec`.
5. MCP gateway: `ActionArgDef::json_schema()` (+ `x-semio-format/-unit/-roles`) already publishes action inputs as JSON Schema for agents (ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`).

None renders a stored *mutation payload*.

---

## 7. Gaps and inconsistencies that matter for the requirement

1. Mutation input schemas lack essentially all UI-relevant facts (§2.4b); bounds are sometimes only in panel code (draw opacity).
2. Seven overlapping "value + widget" vocabularies (A-H); A is the only one with i18n **and** JSON Schema projection; B is dead; C/D/E are wire/element shapes; F is a text-grammar shape; nothing links them.
3. Leaf JSON Schemas are not available at runtime; `WireMutationRosterEntry` and the `Mutations` derive have no inputs slot.
4. Descriptor contract is closed (14 fields, three enforcement sites, 2,834 files) — adding a field is expensive.
5. Wire layout and key casing differ across aggregates; leaf schemas sometimes include the tag const (`mutation`: 601 fields) and sometimes not (lowpoly).
6. Only 31 % of leaves have `RecordSpec`; only ~22 % have a TS mirror; 0 % Python/C#.
7. Gestures do not preserve their inputs (§5.1); composite leaves are nearly unused.
8. `x-semio-*` keys used by Rust emitters (`-unit`, `-shape`, `-ref`, `-format`, `-title`) are not in the strict vocabulary fixture.
9. Existing JSON-Schema i18n convention (H) accepts a plain string as English — contradicts the repo rule "multiple languages with no default language"; A/`LocalizedLabel` requires the full matrix.
10. `ActionArgDef::json_schema()` maps `step → multipleOf`; the owned validator tolerates float error (8 ε) but a third-party validator might reject e.g. `0.3` for `multipleOf 0.1` (known Ajv behaviour, **not verified in this repo**). A UI step is not a validation constraint.
11. Persisted `.spr` history keeps `inverse` and `payload_hash` next to the forward op: editing a payload without recomputing them yields inconsistent history (`HistoryEdit.inverse`, `HistoryOpMeta.payload_hash`).
12. Minor: mutation-facet GraphQL/proto for puzzle 2d are snapshot-shaped placeholders; `aggregate.x-semio-mutationKinds` is a lint error (`schema-mutation-aggregate-kinds-redundant`).

---

## 8. Recommendations

### 8.1 Where the descriptor lives (schema-first, multi-language)

Put each input's UI element description **in the leaf payload JSON Schema** (`<leaf>/🧬️schema/🔣️.json`), because it is already: normative (`mutationPayloadSchemaAuthority`), present for 100 % of leaves, catalogued (`schema-catalog.json`), draft-07 (every language has a validator), and the sole artifact Python/C# can read today. Do **not** extend the 14-field descriptor (cost: 2,834 files + kernel struct + derive + `spr` schema).

Split responsibilities so no fact is stated twice and no third-party validator needs our vocabulary:

- **Hard constraints in standard keywords**: `type`, `minimum`, `maximum`, `exclusiveMinimum`, `minItems`, `maxItems`, `enum`, `const`, `pattern`, `format`, `default`. (Every validator enforces them; the strict-Ajv oracle and the owned validator already agree on them.)
- **UI-only facts in one annotation `x-semio-ui`** (annotation-only, never affects validity; sits beside `$ref` exactly like `x-semio-state`):

```jsonc
"opacity": {
  "$ref": ".../s/draw/drawing/1/style.json#/$defs/UnitInterval",   // hard bounds live in the def
  "x-semio-ui": { "label": { "en": "Opacity", "de": "Deckkraft" } }
}
// $defs/UnitInterval
{ "type": "number", "minimum": 0, "maximum": 1,
  "x-semio-ui": {
    "widget": "slider",                    // slider | stepper | toggle | select | segmented | dial | color | text | textarea | vec | entity | index | readonly | hidden
    "role": "value",                       // value | target (entity ref / index) | discriminator (const tags: skip)
    "step": 0.01, "precision": 2,          // UI step; NOT multipleOf
    "softMin": 0, "softMax": 1,            // optional narrower slider range than the hard bounds
    "snaps": [0, 0.25, 0.5, 0.75, 1], "snapRadius": 0.02,
    "snapSource": { "step": true },        // or { "snapshot": "/canvas/snapGridSpacing" } | { "config": "gridFactor" }
    "scale": "linear",                     // linear | log
    "unit": "1", "displayUnit": "%", "displayFactor": 100,     // storage unit validated against dsl UNITS; display conversion (draw rotation rad→deg)
    "label": { "en": "Opacity", "de": "Deckkraft" },
    "description": { "en": "0 = invisible, 1 = opaque", "de": "0 = unsichtbar, 1 = deckend" },
    "group": "appearance", "order": 20,
    "ref": { "kind": "layer" }             // role=target: entity picker over the base snapshot (same idea as ArgFormat::EntityId / dsl Shape::Ref)
  } }
```

Rules: `label`/`description` objects **must contain every locale** of `UI/🎚️axes/🔣️.json` (en, de; optional `native`/`reuse` terminology nesting = the existing `LocalizedLabel` JSON matrix) — no plain string, no default language (this also fixes the i18n gap of convention H). Reuse the localized-object shape `localized_schema_annotation` already understands so the stdio details editor benefits immediately.

### 8.2 Minimum boilerplate

- **Shared scalar/semantic `$defs`** (PascalCase export ids, satisfying `schema-export-id-invalid` and `schema-ref-unresolved`: fragments must be `#/$defs/<ExportId>`): one framework scope (e.g. `FW/🧬️schema` sibling `mutation-input`) publishes `EntityId`, `ListIndex`, `Name`, `UnitInterval`, `Angle`, `Length`, `Vec3`, `Ratio`, … and each artifact adds a handful of domain scalars (`Puzzle2dCoordinate`, …). Data: 5,025 inputs, 1,484 distinct names; top 25 names cover 45 %, top 100 cover 60 % → ~100 shared defs annotate the majority; only the ~660 `number`, 31 `enum` and irregular objects need per-leaf choices. Because the validator forbids constraining keywords next to `$ref`, bounds/step/snaps/labels live in the `$def`; a leaf may add a sibling `x-semio-ui` that overrides `label`/`order`/`group` only.
- **Inference when absent** (implemented once, mirrors `ActionArgDef::control()`): `integer`+`minimum` → stepper; `integer` named `index`/`*Index` → index stepper; `number` with both bounds → slider; `number` → stepper; `string`+`enum` → select; `boolean` → toggle; `array<number>` len 3 → vec; `role` from name/`x-semio-ref` (`id`, `*Id`, `ids` → target); `const` → hidden discriminator. Labels are **not** inferable into German → the lint below forces authoring them (the existing `humanized_schema_key` only knows three German words).
- **Lint/law** (in `REPO`, next to `schema-mutation-leaf-*`): `schema-mutation-input-ui-missing` (every non-const input must have a resolvable `x-semio-ui` with all locales, or an inference rule that yields a widget), `schema-mutation-input-ui-invalid` (widget vs type, `snaps ⊂ [min,max]`, unit in catalog), `schema-mutation-input-field-parity` (Rust struct fields ↔ schema properties, closing today's indirect-only parity gap).
- **Register the keyword** in `OS/🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json` (`"x-semio-ui": {"type":"object", …}`) and ship the meta-schema of its value as a `$defs` export in a framework schema module with the five formats (`schema-export-incomplete` demands all declared formats, incl. TS `parse<Export>()`).

### 8.3 How every language and every mutation kind gets it

- **Rust**: add `const PAYLOAD_SCHEMA: &'static str` to `MutationLeaf` (`FW/📡️replication/🎮️mutation/🦀️.rs` L928); `#[derive(dsl::MutationLeaf)]` already opens the descriptor beside the leaf at compile time, so it also `include_str!`s `payloadSchema` (same dependency tracking it uses for taxonomy+descriptor; `OS/🗣️dsl/✨️derive/🦀️.rs` L540). Add one framework function `mutation_inputs(schema: &str) -> Vec<MutationInputSpec>` (resolves `$ref` through the schema registry / bundle logic the stdio details editor already has) and one arm in `#[derive(Mutations)]` (`L1552-1840`) exposing `SemanticMutation::inputs(kind)`; extend `WireMutationRosterEntry` (`OS/🔌️plugin/🦀️.rs` L4823) and `ContributedMutationSemantics` with `inputs`. Zero per-leaf Rust edits: all 2,831 leaves already derive `MutationLeaf`.
- **TS**: one reader over the same JSON (the `ArtifactSchemaDescriptor` carries the aggregate JSON only, so also register leaf schemas — the catalog already has 2,834 leaf scopes with `schema://` addresses). **Python / C#**: read JSON Schema directly (draft-07: `jsonschema`, `json-everything`); the only implementation each needs is the ~100-line `x-semio-ui` reader.
- **Unify vocabularies (greenfield, no compat layers):** make `ActionArgDef`/`ArgSchema` the single Rust shape of an input descriptor (it already has localized label, min/max/step/integer/unit/options/format/presentation, `control()` and `json_schema()`); extend it with `snaps`, `snap_source`, `scale`, `display_unit`, `group`, `soft_min/max`; **delete `ConfigFieldShape`/`CommandFieldSpec`** (2 uses); derive `WindowEngagementControl` and the renderers' `SliderProps` from `control()` and add `snap_values` to `SliderProps`/`Engagement Slider` (React already supports `snapValues`). Then a mutation input, an action argument and a config field share one vocabulary and one renderer (shell dialog + stdio details editor + inspector helpers).
- **Rollout order** (keeps the tree green each step): (1) vocabulary fixture + meta-schema + reader + oracle tests; (2) `PAYLOAD_SCHEMA` + `inputs()` plumbing; (3) shared `$defs` and inference; (4) migrate leaves plugin by plugin driven by the lint's counts, starting with puzzle 2d/3d/5d, draw, lowpoly, cad, note, forms (they have actions with arg defs today, and 5 of the 7 gestures above); (5) delete the per-app field tables (draw panel `fields()`, `drawing_op_for_layer_field` becomes descriptor-driven).

### 8.4 Generic editing of a stored payload

Use the contracts that already exist and add three small pieces:

1. `#[derive(Mutations)]` also emits `fn payload_value(&self) -> DslValue` (the leaf payload without any tag/wrapper) and `fn with_payload_value(&self, DslValue) -> Result<Self, ValueError>` (rebuild the variant from the same kind). This removes the dependence on the six wire layouts and on camelCase/snake_case, and it works for hand-written codecs through `edit_through_value`.
2. An **editor session object** (framework, language-neutral in behaviour): `read(op) -> {kind, title(en,de), inputs: [{path, spec, value, error?}]}`, `set(path, value) -> op'` = `edit_value_at_path` on the leaf payload → validate against the leaf schema with the owned validator (hard bounds/enums; UI clamps/snaps by `x-semio-ui`) → `FromValue` → `MutationKind::diff(base)` and surface `MutationOutcome` class/messages (`mutation.target-missing`, `mutation.no-op`, …) next to the input. `role=target` inputs use `ref.kind` to offer ids from the base snapshot at that history position.
3. **Recompute derived persisted facts** after any edit: `inverse` (`Mutation::inverse(base)`), `MutationMeta.payload_hash`, `label`, and re-run every later op of the same alternative (each has an `outcomeClass`; a later `rejected` must be reported, not hidden). Persisted `.spr` keeps `inverse` explicitly (`HistoryEdit.inverse`), so the edit must rewrite it in the same append.
4. Gesture-level edits (rotate angle, translate dx/dy, scale factor) need the gesture inputs recorded: either a composite leaf (`CompositeMutationKind::plan`, `composition: composite`, `invertibility: plan`; only `create-building-storey` exists) or edit metadata on the `Edit` (`description`, `coalesce_key`, `group_id`) that names the gesture and its parameters. Without this, "edit the drag" degrades to editing N `move-node` leaves.

### 8.5 Tests (AGENTS.md: test-driven, language-agnostic, third-party oracle)

- Language-agnostic vectors under a new `🧫️fixtures` folder: `{leafSchema, expectedInputs[]}` and `{op, path, value, expectedOp | expectedError}`; Rust and TS readers must produce byte-equal `inputs[]` (canonical JSON).
- Third-party oracle: strict Ajv (already the repo oracle, `semioSchemaAjvV1`) with `x-semio-ui` registered validates every leaf schema and the meta-schema; a second oracle in Python `jsonschema` (or `json-everything`) proves the same fixtures validate without knowing the vocabulary.
- Law tests: every locale present in every label; `snaps` within bounds; every `role=target` `ref.kind` resolves to an entity kind of the artifact; parity Rust field ↔ schema property.

---

## 9. Open questions / risks for the coordinator

- `step` as UI-only (`x-semio-ui.step`) vs `multipleOf`: this report recommends UI-only; confirm with the owner of `ActionArgDef::json_schema()` (MCP gateway) before changing its current `multipleOf` output.
- Snap sources that depend on document data (note `snapGridSpacing`) or window/config state (puzzle `gridFactor`) make descriptors *contextual*; specify the resolution API (`SnapSource::{Fixed, Step, Snapshot(pointer), Config(key)}`) before authoring leaves.
- Whether a shared framework `$defs` scope may be depended on by every plugin scope (`schema-cross-scope-dependency-forbidden` requires a declared `dependsOn`; catalog `dependsOn` is generated, but the dependency-direction taxonomy (`dependencyDirections`, `areaLayers`) must admit plugin→framework schema refs; existing precedent: leaf/snapshot `$ref`s inside the same plugin scope only — 14 leaves use `$ref` today).
- Aggregate wire-layout normalization (§2.4b) is a prerequisite only if descriptors are read from `aggregate.to_value()`; the `payload_value`/`with_payload_value` pair avoids it.
- Legacy: ~75 leaves still derive serde outside tests and the note/forms leaves keep inconsistent casing; not blocking, but the parity lint will surface them.
- Not covered here (other agents): history storage/transition semantics, alternatives, conflict handling, UI for the history table (`UI/🧱️elements/🕰️HistoryTable`, `OS/📡️spr/📜️history`).

---

## Appendix A: file index (most relevant first)

- `FW/📡️replication/🎮️mutation/🦀️.rs` — `Mutation`, `MutationLeafDescriptor`, `MutationLeaf`, `MutationOutcome`, `OpText/OpBinary`, `Edit`, `MutationMeta`
- `OS/📡️spr/🎮️command/🦀️.rs` — `SemanticDescriptor`, `MutationKind`, `SemanticMutation`, `CompositeMutationKind`, `MutationDescriptorRegistry`, `APPROVED_VERBS`
- `OS/🗣️dsl/✨️derive/🦀️.rs` — `MutationLeaf`, `Mutations`, `DslRecord`, `#[dsl(...)]` attributes
- `OS/🗣️dsl/🧬️schema/🦀️.rs`, `OS/🗣️dsl/🔤️token/🦀️.rs` — `Shape`/`FieldSpec`/`RecordSpec`, unit catalog
- `FW/🌱️value/🦀️.rs`, `FW/🌱️value/🔁️codec/🦀️.rs`, `FW/🌱️value/✨️derive/⚙️expansion/🦀️.rs` — `DslValue`, `ToValue/FromValue`, path edits
- `FW/🧬️schema/{🔣️.json,🟦️.ts,⚛️component/🦀️.rs,✅️validator/🦀️.rs,✨️derive}` — descriptors, validator, `ArtifactSchema`
- `FW/🛂️manifest/{🦀️.rs,🟦️.ts}`; `FW/🧩️action-argument-resolution/🟦️.ts` — `ActionArgDef` family
- `UI/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`, `UI/🧬️contract/🧩️component/🦀️.rs`, `UI/🧱️elements/🎚️Slider/🟦️.tsx` — UI control vocabularies, inspector helpers, snap values
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/{🦀️.rs,🩹️patch,🪟️details}` — generic details editor and patch contract
- `OS/📡️spr/📜️history/🦀️.rs` — persisted history model
- `OS/🔌️plugin/{🦀️.rs,🏗️builder/🦀️.rs}` — mutation roster registry and wire listing
- `REPO/📚️library/{🔣️taxonomy.json,🔣️schema-catalog.json,🏗️authoring/🧬️mutation-tree/🟦️.ts}` — taxonomy, catalog, new-mutation scaffolder (`newMutationDescriptor` writes the 14 fields)
- `OS/🧫️fixtures/🧬️schema-vendor-annotation-vocabulary-v1/🔣️.json`, `OS/🧪️tests/🧬️schema-oracle/🟦️.ts` — strict Ajv vocabulary
