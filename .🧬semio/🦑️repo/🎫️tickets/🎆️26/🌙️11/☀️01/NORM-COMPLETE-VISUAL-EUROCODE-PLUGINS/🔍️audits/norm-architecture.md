# 🧭 Norm Plugin Architecture

Read-only audit of `/home/deck/semio/✏️s/🔌️plugins/📕️norm` (about 11,150 tracked files), with EN 1993 as the worked example. Paths below are relative to the plugin root `P = /home/deck/semio/✏️s/🔌️plugins/📕️norm` unless marked absolute. Artifact root: `E93 = P/🗿️artifacts/🔩️en1993`. Subset root: `S93 = E93/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 0. Layout at a glance

| Area | Path (relative to `P`) | Role |
|---|---|---|
| Plugin root | `AGENTS.md`, `Cargo.toml`, `Cargo.lock` | Cargo workspace (only 4 members: compliance, contract, app-surface, results-config). Artifacts are excluded (`exclude = ["**/🏅️standards/**", "**/🔮️oracles/**", "**/🏭️bridge/**", "🗿️artifacts", ...]`). |
| Shared compliance kernel | `⚖️compliance/` | `NormFamily`, `CheckResult`, `CheckReport`, `Quantity`, `ClauseId`, `NormError`, annex tables. |
| Registry contract | `📇️registry/🧬️contract/` | Package identity (`NormArtifactPackage`), TS wire readers (`🟦️.ts`), `🖥️app-surface` (shared editor machinery). |
| Results config | `🪟️results/🎚️config/` | Results-window config owner (`norm_results_window_config_owner!`) and io. |
| Bridge | `🏭️bridge/` | Language-neutral production mutation inventory for the test platform (`📜️script.ts list-mutations ...`). |
| Oracles (shared) | `🔮️oracles/` | Python reference engine `🏃️execution/` (`semio_norm_vocabulary_*`), shared manifest `🔣️.json`. |
| Root TS package | `📦️packages/🟦️typescript/` | `@semio-tech/norm-js` (exports `./registry/contract`, `./testing/document-contract`). |
| Root tests | `🧪️tests/` | `🏷️mutation-label-census`, `🧪️wire-twins`, `🪪️document-contract`, `🔬️typescript-package-commands`, fixtures. |
| Artifacts (15) | `🗿️artifacts/{⚖️en1990,🏋️en1991,🏛️en1992,🔩️en1993,🧩️en1994,🪵️en1995,🪨️en1996,🌍️en1997,🫨️en1998,🪶️en1999,🌬️din16798,⚡️din18599,🧱️din4108,📇️iso16757,🏭️vdi3805}/` | Each is one `s.norm.<x>` artifact. |
| Stray mirror | `P/../../🧑‍💻dev/📕️norm/` (i.e. `/home/deck/semio/✏️s/🧑‍💻dev/📕️norm/`) | Tracked dev copy with only `📦️packages/🟦️typescript/` and `🧪️tests/🔬️typescript-package-commands/`. Not part of the plugin tree; do not add to it without reason. |

Per-artifact structure (EN 1993 shown):

- `E93/📜️artifact-definition.json`: `id: s.norm.en1993`, `rust_package: semio-s-artifact-norm-en1993`, `nx_project: @semio-tech/norm-en1993-rs`, `dependencies: [semio-s-artifact-norm-en1992]`. Validated by `⚖️compliance`-adjacent `📇️registry/🧬️contract/🦀️.rs::package_from_schema` (rejects unknown members, enforces naming).
- `E93/🟦️.ts` (re-export barrel) and `E93/🦀️.rs` (`definition()` declaration: schema, inference, composer, codec, capabilities and localization claims).
- `E93/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts}` and `E93/📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts}`.
- `E93/Cargo.toml`, `E93/Cargo.lock`.
- `S93/🔣️.json` (subset declaration, `subsetPolicy: single`, subset `*`).
- `S93/🧬️schema/` (snapshot model, inferences, mutations, diff), `S93/✏️editor/`, `S93/👁️viewer/`, `S93/🚪️io/`, `S93/🔮️oracles/`, `S93/🧪️tests/`, `S93/📚️examples/`, `S93/🖼️assets/`.

## 1. The pipeline: schema-first sources and sync

### 1.1 Source-of-truth ordering

The repo rule is schema-first. In practice the files split into three classes.

**Declared (hand-authored, the contract):**
- `S93/🧬️schema/🔣️.json` (JSON Schema draft-07, about 60 KB for the snapshot facet, with `x-semio-ui` labels in en/de). Snapshot, diff and mutation payload schemas are each `🔣️.json` under `🧬️schema/📸️snapshot/`, `🧬️schema/🔺️diff/`, `🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json`.
- `S93/🧬️schema/🔗️.graphql` and `S93/🧬️schema/🛰️.proto`: the GraphQL file header literally says `# 🧬 EN 1993 snapshot GraphQL (handcrafted SI schema).` The proto is also hand-written. Treat these as declared siblings of the JSON Schema, not generated outputs.
- `S93/🔮️oracles/🔣️.json`: the oracle and mutation-catalog manifest (`en1993-1-any`, capability `en1993-1-mutate`, comparison `ordered-json-v1`, `mutationManifests` list the 49 mutation kinds).
- `S93/🧬️schema/🧬️mutations/<leaf>/🔣️.json`: per-mutation `semanticKind`, `binaryTag`, `invertibility`, `outcomeClasses`, `requiredLanguageSurfaces`. Example `➕️insert-member/🔣️.json`: `semanticKind: insert-member`, `binaryTag: 22`, `invertibility: explicit-mutation`, `diffParticipation: detect`.
- `S93/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` (payload), e.g. `index` is a stepper with en/de labels.
- `E93/🏅️standards/🔖️1/🪆️subsets/🔣️.json`: subset declaration.

**Authored Rust (runtime truth):**
- Snapshot model: `S93/🧬️schema/📸️snapshot/🦀️.rs`. `pub struct En1993Snapshot` with `#[derive(DslRecord, ArtifactSchema, ToValue, FromValue, ...)]`, `#[dsl(id = "norm.en1993", layout = "lines")]`, `#[artifact_schema(id = "s.norm.en1993")]`, and `#[state(artifact)]` plus `#[dsl(table)]` per collection.
- Mutations: `S93/🧬️schema/🧬️mutations/🦀️.rs` (`enum En1993Mutation`, `#[derive(dsl::Mutations)]`, `#[mutations(snapshot = En1993Snapshot, diff = En1993Diff, schema = "s.norm.en1993", ...)]`), and one `<leaf>/🦀️.rs` per mutation.
- Diff: `S93/🧬️schema/🔺️diff/🦀️.rs`.
- Inferences: `S93/🧬️schema/💡️inferences/🦀️.rs` (`En1993Inference` with `#[derived] outline`), `💡️inferences/🧾outline/🦀️.rs`, and checks under `💡️inferences/⚖️checks/`.

**Generated TypeScript twins:**
- Each `🟦️.ts` under the schema folders (snapshot, diff, mutations, inferences, and `🚪️io/📝️text/*`, `🚪️io/💾️binary/*`) starts with a docstring stating "Generated from `./🔣️.json` by `🧪️s2-norm-ts-twins.ts`". It imports readers from `@semio-tech/norm-js/registry/contract`.
- **Caveat:** that generator is NOT in the repo as a plugin file. `git ls-files` finds `🧪️s2-norm-ts-twins.ts` only at `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-norm-ts-twins.ts` (a closed ticket folder), and `🧪️tests/🧪️wire-twins/🟦️.ts` references that same path. Implementers should treat the twin files as hand-maintained against the wire tests, and should not assume `repo run` regenerates them.

**Rust twins/facets:** `🦀️.rs` artifacts reference their facets via `include_str!("🔣️.json")`, `include_str!("🛰️.proto")` etc. in the `*_artifact_*_descriptor()` functions (see `💡️inferences/🦀️.rs`), so the schema files are compiled into the descriptor registry.

### 1.2 Generated vs handwritten

| Kind | Files | Status |
|---|---|---|
| Declared schema | `🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `📖️*.grammar.semio`, `🅰️.g4`, `🔤️.ebnf`, `🥋️.ksy`, `🌶️.spicy`, `📡️*.protocol.semio`, `🔠️.abnf` | Handwritten |
| Runtime model | `🦀️.rs` in schema folders | Handwritten |
| Wire twins | `🟦️.ts` in schema and io folders | Header says generated (external ticket script); verified against Ajv by tests |
| Examples | `📚️examples/**/🖼️assets/*.semio`, `✏️editor/📚️examples/**/🖼️assets/` | Committed fixtures |
| Snapshot SQLite | `🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql`, `🗄️.d.ts` | Handwritten |
| Test fixtures | `🧪️tests/🧫️fixtures/🧬️mutations/<leaf>/{✅apply,⛔dupe}/{📸️snapshot/⬅️before,➡️after, 🦠️mutation, 🔺️diff, 🎯️outcome}` | Produced by production dispatch (see mutate-en1993-1 feature text) |

### 1.3 Keeping them in sync

- Rust sources are checked against their schemas by the document contract (`🧪️tests/🪪️document-contract/🟦️.ts`, `testNormDocumentContractOracle`). It loads each artifact's `🔣️.json` for the artifact, snapshot and diff facets, runs Ajv (`semioSchemaAjvV1`) and the twin parsers, and feeds valid and invalid documents.
- `🧪️tests/🧪️wire-twins/🟦️.ts` asserts every committed mutation, snapshot and diff in the fifteen artifacts meets its TS twin and the strict Ajv oracle, and that an admitted wire re-encodes byte-for-byte (two-space JSON, Python float repr).
- `🧪️tests/🏷️mutation-label-census/🟦️.ts` checks every mutation `label()` speaks both locales (no field identifiers, no English reuse in German).
- `kinds_match_the_enum_and_the_catalog` in `S93/🧬️schema/🧬️mutations/🦀️.rs`'s test folder keeps `KINDS` (in `S93/🧪️tests/🔩️mutate-en1993-1/🦀️.rs`) aligned with the enum and the manifest catalog.
- The framework's catalog `🧰️framework/.../🔣️schema-catalog.json` is referenced by the document-contract test (`CATALOG`).

### 1.4 Build targets (`📋️project.json`)

Rust package (`E93/📦️packages/🦀️rust/📋️project.json`), Nx name `@semio-tech/norm-en1993-rs`, tags `language:rust, role:artifact, family:norm`:
- `build`, `check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`. Each uses `executor: repo:script` and runs `repo ./📜️script.ts <cmd>` in `E93/📦️packages/🦀️rust`.
- `build` has `cache: true` and output `{projectRoot}/.🧾️store/build.json`. Inputs include `S93/**` standards, `🦀️.rs`, `🟦️.ts`, and framework `sqlite-snapshot` and caching modules.

TypeScript package (`E93/📦️packages/🟦️typescript/📋️project.json`), name `@semio-tech/norm-en1993`, targets `build`, `check`, `test`, cwd in that folder. Build has `metadata.semio.artifactPackage` pointing at `../../🟦️.ts`.

Script routers (`📜️script.ts`) call framework helpers:
- Rust: `runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-norm-en1993", { packages: [...16 crate names...], snapshotSqliteTests: [...] })`. The package list is the authoritative map of which crates a subset builds: `-checks`, `-diff`, `-editor`, `-examples`, `-inferences`, `-io`, `-model`, `-mut-connections`, `-mut-cranes`, `-mut-fire-fatigue`, `-mut-general`, `-mut-thin-walled`, `-mut-towers`, `-mutations`, `-schema`, `-viewer`.
- TS: `runArtifactTypeScriptPackageMain(import.meta.dir, "@semio-tech/norm-en1993", { bundleRuntime: "runtime", suites: [...] })`.

Plugin root project (`P/📦️packages/🟦️typescript/📋️project.json`), name `@semio-tech/norm-js`, targets `test` (runs `🧪️tests/🔬️typescript-package-commands`) and `test-wire-twins` (runs `🧪️tests/🧪️wire-twins/🟦️.ts` via `bun test`).

Contract Rust package `P/📇️registry/🧬️contract/📦️packages/🦀️rust/📋️project.json`, name `@semio-tech/norm-artifact-contract-rs`, targets `build`, `check`, `test`, `test-byte-property`.

No `📋️project.json` exists for `⚖️compliance`, `🖥️app-surface`, `🪟️results/🎚️config` as separate projects. They are built as Cargo workspace members. `🏭️bridge/📜️script.ts` has no `📋️project.json` either.

Hint for the stray `P/../🧑‍💻dev/📕️norm/📦️packages/🟦️typescript/📋️project.json` project: it also carries a TS package and a test script, but I did not verify whether it is a live project or a duplicate. Avoid it.

## 2. Snapshot, inferences, mutations, diff

### 2.1 Snapshot (`S93/🧬️schema/📸️snapshot/`)

- `🦀️.rs`: `En1993Snapshot` with fields `annex: AnnexChoice`, and collections `materials`, `sections`, `members`, `load_cases`, `member_actions`, `joints`, `fatigue_details`, `fire_exposures`, `cold_formed_members`, `plated_panels`, `silo_shells`, `tension_components`, `bridge_fatigue`, `tower_legs`, `piles`, `crane_runways` (all `Vec<T>`, each `#[dsl(table)] #[state(artifact)]`). Record types: `SteelMaterial`, `SteelSection`, `SteelMember`, `LoadCase`, `MemberAction`, `ForceAction`, `JointForceAction`, `FatigueBand`, `DesignAction`, `SteelJoint`, `FatigueDetail`, `FireExposure`, `ColdFormedMember`, `PlatedPanel`, `SiloShell`, `TensionComponent`, `BridgeFatigue`, `TowerLeg`, `SteelPile`, `CraneRunway`.
- `🔣️.json` (JSON Schema, `$id` `https://json.schemas.assets.semio-tech.com/s/norm/en1993/...`), `🔗️.graphql`, `🛰️.proto`, `🟦️.ts`, `🪪️capability` tests under `📸️snapshot/🧪️tests/🪪️capability/`.
- Package: `📸️snapshot/📦️packages/🦀️rust/Cargo.toml` (crate `semio-s-artifact-norm-en1993-model`, `lib.path = ../../🦀️.rs`).
- SQLite: `S93/🚪️io/🪶️sqlite/📸️snapshot/` (`🗄️.sql`, `🗄️.d.ts`, `🦀️.rs`, `🟦️.ts`, `🛂️admission`, `🧪️tests/`).
- Text and binary IO: `S93/🚪️io/📝️text/📸️snapshot/` (`📖️component.grammar.semio`, `🔤️.ebnf`, `🅰️.g4`, plus schema twins) and `S93/🚪️io/💾️binary/📸️snapshot/` (`🥋️.ksy`, `🌶️.spicy`, `📡️.protocol.semio`, `🔠️.abnf`). The grammar is a placeholder (`payload = OCTET+`), so text round-trip is compared at the envelope and digest level only.

Also: `E93/📦️packages/`'s model crate is the one every other facet depends on.

### 2.2 Inferences (`S93/🧬️schema/💡️inferences/`)

- `En1993Inference` (`🦀️.rs`) has one derived field, `outline: En1993Outline`. It implements `protocol::Inference<En1993Snapshot>` and `InferenceSpec` with `fields()` listing every snapshot field it reads, and `inference_schema_id = "s.norm.en1993.inference"`.
- `🧾outline/🦀️.rs` builds the document outline.
- `⚖️checks/` is where all compliance computation lives:
  - `⚖️checks/🦀️.rs`: entry, `pub fn evaluate(document) -> CheckReport` is in `💡️inferences/🦀️.rs` and delegates to `semio_s_artifact_norm_en1993_checks::check_full_steel_structure`.
  - `⚖️checks/🧱️member-checks/🦀️.rs`: `check_full_steel_structure(document: &En1993Snapshot) -> CheckReport`. It runs `push_referential_integrity`, returns a single `en1993.empty` not-applicable check when the document has no elements, then loops members and resolves section and material via `find_section` and `find_material`. Sub-modules: `🏷️section-class-checks/` and `🧮️action-resistance-checks/` (ULS actions and SLS deflection).
  - `⚖️checks/🔗️special-element-checks/` with `🏛️shell-and-structure-checks/`, `⏳️fatigue-fire-and-thin-walled-checks/`, `🔩️joint-checks/`.
  - `⚖️checks/📏️parts/🦀️.rs`: clause formulas as `pub mod part_1_1 { ... }` (ε, class tables, resistances, buckling curves). Also `part_1_2`, `part_1_8`, `part_1_9` are referenced by the compliance oracle test.
  - `⚖️checks/🧰️structure-helpers/🦀️.rs`: `AnnexParams` (EN vs DE gammas: `GAMMA_M1_DE = 1.1`, others from EN 1993-1-1), `find_section`, `find_material`, `governing_effects`, `CombinedEffects`.
- Check identity pattern (verbatim from `🧮️action-resistance-checks`):

  ```
  CheckResult::assess(
      format!("en1993.6.2.4.n.{}", action_row.combination_id),
      "DIN EN 1993-1-1",
      ClauseId::new("EN 1993-1-1", "6", "6.2.4"),
      subject_member(member, &format!("members[id={}].id", member.id)),
      loc("Cross-section compression", "Querschnitt Druck"),
  )
  .annex(annex)
  .utilization(force_n(a.n), force_n(n_rd))
  ...
  ```

  Subjects are `SubjectRef { entity_id, path, label }` with value-tree paths like `members[id=...].id`.

### 2.3 Mutations (`S93/🧬️schema/🧬️mutations/`)

- Root `🦀️.rs` declares the leaves via `use semio_s_artifact_norm_en1993_mut_<group>::<leaf>` (groups: `mut-general`, `mut-connections`, `mut-fire-fatigue`, `mut-thin-walled`, `mut-cranes`, `mut-towers`), then `enum En1993Mutation { ChangeAnnex(...), UpdateMemberProperties(...), ... }`. 49 variants: 1 `change-annex`, 16 `update-*-inputs`/`update-member-properties`, 16 `insert-*`, 16 `remove-*` (plus the `update-*` set). Verify against `KINDS`.
- Leaf anatomy (`➕️insert-member/🦀️.rs`): a payload struct (`InsertMember { index: Option<usize>, member: SteelMember }`) with `dsl::MutationLeaf` and `#[mutation_leaf(contract = ::protocol)]`; `impl MutationKind<En1993Snapshot, Op>` with `SEMANTICS` (`verb`, `entity`, `kind`, `record`), `label()` (en/de via `LocalizedLabel::native`) and `target()`; `impl ColdMutationKind` with `diff()` and `inverse()` delegating to submodules `🔺️diff/🦀️.rs` and `↩️inverse/🦀️.rs`.
- Diff for insert: rejects a duplicate id with `protocol::MutationOutcome::fatal("mutation.duplicate-id", ...)`, otherwise `En1993MemberDelta::insertion(index, member)`.
- Each leaf directory has `🔣️.json` (catalog metadata), `🧬️schema/🔣️.json` and `🧬️schema/🟦️.ts` (payload), `🔺️diff/`, `↩️inverse/`.
- Shared group code: `mut-*` crates are physically in the directories named `🏗️general`, `🪜️general-2`, `🔩️connections`, `🔥️fire-fatigue`, `🪝️cranes`, `🥶️thin-walled`, `🗼️towers`, `🧭️edit-rules`. They are the crates named by the package list.
- Edit rules (`🧭️edit-rules/🦀️.rs`): `EDIT_RULES: NormEditRules` mapping value-tree paths to kinds. Example: `set_field: [{path: "annex", kind: "change-annex", value: "newAnnex"}]`, `insert_item: [{path: "joints", kind: "insert-joint", index: "index", item: "joint"}, ...]`.

### 2.4 Diff (`S93/🧬️schema/🔺️diff/`)

- `🦀️.rs` (`En1993Diff`, with `En1993MemberDelta` and siblings as delta containers, `insertion(index, row)`), `🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🟦️.ts` (wire twin), `📦️packages/🦀️rust` (crate `semio-s-artifact-norm-en1993-diff`), `🧪️tests/`.
- A diff is sparse: `..Default::default()` leaves other lanes empty.

### 2.5 Recipe: add a new compliance check to an existing artifact

Example: a new EN 1993 member check, say section-class limit on web.

1. Clause formula: add pure fn to `S93/🧬️schema/💡️inferences/⚖️checks/📏️parts/🦀️.rs` (in the right `pub mod part_x_y`). No comments inside definitions, docstring starts with an emoji.
2. Check body: in the matching module (`🧱️member-checks/🏷️section-class-checks/🦀️.rs` or `🧮️action-resistance-checks/🦀️.rs`), call `CheckResult::assess(id, part, ClauseId::new(family, part, section), subject, loc(en, de))` with `.annex(annex).utilization(computed, limit)` (or `.minimum(...)`, `.status(...)`, `.not_applicable(...)`, `.remedy(Remedy::at_least|at_most|exactly|one_of(...))`), then `.build()` and `report.push(...)`. Use `Quantity::force_kn`/`stress_mpa`/`length_m` etc. for units.
3. Wire the new function into `check_full_steel_structure` (`🧱️member-checks/🦀️.rs`). Respect the `members.is_empty() && ...` not-applicable gate if you add a new collection.
4. If the check needs a new input field: add it to the snapshot record in `📸️snapshot/🦀️.rs`, update `📸️snapshot/🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🟦️.ts` twin (keep field order identical; twin tests check order byte-for-byte), SQLite `🗄️.sql` and `🟦️.ts` in `🚪️io/🪶️sqlite/📸️snapshot/`, and add `reads` to `En1993Inference::fields()`.
5. Add `reads` for the inference via `InferenceFieldSpec` (listed in `💡️inferences/🦀️.rs`).
6. Add a unit test in `⚖️checks/.../🧪️tests/🔬️unit/🦀️.rs` (or `💡️inferences/🧪️tests/🔬️unit/🦀️.rs`), and a compliance-report test in `💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs` if the report shape changes.
7. Language-agnostic test: add a `.feature` scenario in `S93/🧪️tests/🔩️mutate-en1993-1/🥒️.feature` only if you add a mutation. For a pure check, the independent oracle is `S93/🔮️oracles/⚖️compliance/🐍️.py` (EN 1990 combinations plus governing steel checks, pure Python, uses `json`, `math`, `pathlib`). Add the expected value there and the matching case in `S93/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs`, which compares Rust output with the oracle.
8. Results window: remedies and utilisation flow automatically through `render_report` (app-surface `🌳️report-tree`); no UI work needed for a plain check.

### 2.6 Recipe: add a new entity (collection) to EN 1993

Modelled on `insert-member` / `remove-member` (`mut-general`) or `insert-joint` (`mut-connections`).

1. Model: add the record struct and the `Vec<Record>` field (`#[dsl(table)] #[state(artifact)]`) to `📸️snapshot/🦀️.rs`, plus the child record in model re-exports. Then update the JSON Schema, GraphQL, proto and TS twin, and SQLite (`🗄️.sql`, `🟦️.ts`, `🦀️.rs`) in the same change.
2. Diff: add a `<Entity>Delta` with `insertion` and `removal` in `🔺️diff/🦀️.rs` and its `🔣️.json`, `🟦️.ts`, `🛰️.proto`, `🔗️.graphql`.
3. Mutations: create `🧬️mutations/➕️insert-<entity>/` and `➖️remove-<entity>/` each with `🦀️.rs`, `🔣️.json` (catalog metadata with `binaryTag`, unused tags must not collide), `🧬️schema/{🔣️.json,🟦️.ts}`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`. Also add to the group crate (`mut-connections`, etc.) and its `Cargo.toml`, then register in `🧬️mutations/🦀️.rs` (`use` plus enum variant) and `🧬️mutations/🔗️.graphql`, `🛰️.proto`, `🔣️.json`, `🟦️.ts`.
4. Oracle manifest: add the two kind names to `S93/🔮️oracles/🔣️.json` `mutationCatalogs` (`en1993-1-any`) and to `KINDS` in `S93/🧪️tests/🔩️mutate-en1993-1/🦀️.rs`. The test `kinds_match_the_enum_and_the_catalog` enforces this.
5. Fixtures: create `S93/🧪️tests/🧫️fixtures/🧬️mutations/<emoji><kind>/{✅apply,⛔dupe}/...` produced by production dispatch. Also add a row to `mutationManifests` and to the `.feature` Examples table.
6. Editor: add `InsertItemRule`/`RemoveItemRule` to `S93/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs` (or the `edit-rules` file under `✏️editor/`). Add a `commands` path if needed.
7. Field metadata: add choice tables and paths in `S93/✏️editor/🏷️field-meta/🦀️.rs` (`lookup_norm_field_meta`, wildcards `[]`).
8. Label census: every new leaf's `label()` must pass `🧪️tests/🏷️mutation-label-census/🟦️.ts`.
9. Bridge: production inventory is read by `🏭️bridge/🦀️.rs`; no edit needed unless the new leaf lives in a new group crate (then add it to `🏭️bridge/` manifests if it is listed there).

### 2.7 Recipe: add a new mutation to an existing entity

Same as 2.6 steps 3 to 8, skipping model and diff creation unless the diff lane changes.

## 3. The compliance kernel and the registry contract

### 3.1 `⚖️compliance` (crate `semio-s-plugin-norm-compliance`)

Root `⚖️compliance/🦀️.rs` mounts (via `#[path]`) these modules, all `pub use`d:

- `📐️quantities-clauses-and-text/🦀️.rs`: `QuantityKind` (Length, Area, Force, Stress, Dimensionless, ThermalResistance, UValue, Acceleration, ...), `Quantity { kind, value }` with constructors `length_m`, `area_m2`, `force_kn` (converted to N), `stress_mpa` (converted to Pa), `thermal_resistance_m2k_w`, `u_value_w_m2k`, `acceleration_m_s2`. `ClauseId { family, part, section }` with Display `"EN 1993-1-1 6 §6.2.4"`. `LocalizedText`.
- `✅️check-results/🦀️.rs`: `CheckStatus`, `LocalizedCopy::new(en, de)` with `resolve(locale)`, `SubjectRef { entity_id, path, label }` (`whole(label)` for report-level), `RemedyBound` (AtLeast, AtMost, Exactly, OneOf), `Remedy` with `at_least/at_most/exactly/one_of` constructors, `CheckResult` (id, part, clause, subject, status, title, explanation, computed, limit, utilization, annex, remedies), `CheckBuilder` (`CheckResult::assess(...)` is the only construction path; methods `utilization`, `minimum`, `status`, `warn_above`, `annex`, `explanation`, `remedy`, `not_applicable`, `build`), `PartVerdict`, `CheckReportSummary`, `CheckReport` (`push`, `extend`, `complies`, `worst_utilization`, `failing`, `by_part`).
- `📜️annex-and-tables/🦀️.rs`: `AnnexChoice { En, De }` with `label()`, trait `NationalAnnex`, `TableEntry1D`, `table_lookup_linear`, `table_lookup_bilinear`.
- `🏗️design-situation-and-shared/🦀️.rs`: `DesignSituation`, `ConsequenceClass`, `ImposedCategory` (Q_k table), `LimitState`, `LoadDuration`, `ClimateZoneDe` (DIN climate values), `OccupancyType`.
- `🏛️norm-error-and-family/🦀️.rs`: `NormError { IncompleteInput{field}, OutOfScope{clause}, InvalidValue{field, reason} }` (implements Display and Error). `NormFamilyId` (Din4108, DinEn16798, DinV18599, En1990..En1999, Iso16757, Vdi3805) with `label()`. Trait `NormFamily { type Document; type Mutation; fn family_id(); fn evaluate(&Document) -> CheckReport }`. Report cache `cached_report_for`, `store_cached_report_for`, `invalidate_cached_report_for`, keyed by `document_revision_key` (FNV-like hash of canonical JSON). `NormHost<F>` (`from_artifact`, `document`, `report`, `apply`, `replace_document`, `evaluate`).
- `🔤️op-text-and-pack-codec/🦀️.rs`: text escaping, `NormArtifactRecord` trait, `norm_parse_dsl`, `norm_print_dsl`, `norm_encode_pack`, `norm_decode_pack`, plus macro `impl_norm_artifact_record!` (macro-export).
- `🧪️tests/🔬️child-identity-oracle/🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs` (feature `compliance-testing` enables serde).
- `🧬️schema/` in compliance holds its own schema (`🔣️.json`, `🛰️.proto`, `🔗️.graphql`, `🟦️.ts`) for the kernel types, and `🧪️tests/🧪️check-report-fixture`, `🧫️fixtures/🔬️check-report/🔣️.json`.
- Kernel package: `⚖️compliance/📦️packages/🦀️rust/Cargo.toml`, `lib.path = ../../🦀️.rs`, feature `compliance-testing`.

### 3.2 Registry contract `📇️registry/🧬️contract/`

- Crate `semio-s-artifact-norm-contract` (`🦀️.rs`): `NormArtifactPackage` and `package_from_schema(source)` for `📜️artifact-definition.json`. Reads with `JsonMemberPolicy::Reject` and enforces `definition_version == 1`, id `s.norm.<artifact>`, rust package `semio-s-artifact-norm-<artifact>`, nx `@semio-tech/norm-<artifact>-rs`. Also `pub mod payload_op_binary`, `pub mod definition` (`🧾️definition/`), `pub mod sqlite_native` (`🪶️sqlite/🫳️admission/`).
- TS half `🟦️.ts`: `NormWireRefusal`, `NormWireReader<T>`, helpers `normWireArray`, `normWireInteger`, `normWireLiteral`, `normWireNumber`, `normWireObject`, `normWireRange`, `normWireRef`, `normWireRequired`, `normWireString`. Exported via `@semio-tech/norm-js/registry/contract`.
- `🧪️tests/` contains `🔬️rust-package-commands/🟦️.ts`, `🔬️unit/🦀️.rs`, fixtures `✅️valid/`, `❌️invalid/`, `🧫️retained-command-dispositions/`.

### 3.3 `🖥️app-surface` (crate `semio-s-plugin-norm-app-surface`)

Shared editor machinery for all fifteen apps. Its header states the rule: declarations shared by all apps live once here, and each `✏️editor/` only states what varies. Mounted modules (`🦀️.rs`):

- `🏷️ids-chrome-and-labels` (`MODE_EDIT`, window ids, labels, `edit_mode_definition`, `window_definition(id, label, body, icon)`).
- `🧭️value-path` (value-tree path helpers).
- `🔖️field-meta` (`NormFieldMeta { label_en, label_de, unit, choices }`, `NormFieldChoice`, `NormFieldMetaFn = fn(&str) -> Option<NormFieldMeta>`, `lookup_norm_field_meta` with longest-prefix and `[]` wildcard matching).
- `✏️edit-rules` (`NormEditRules`, `SetFieldRule`, `InsertItemRule`, `RemoveItemRule`, `ApplyRemedy` rules).
- `🌳️report-tree` (`render_report(report, windows, locale, controller)`, used by the results window).
- `📝️document-editors` (`render_document_editor(document, locale, controller_id, meta_fn, windows)`, object/array/value editors; `NORM_LIST_VIRTUALIZE_THRESHOLD = 64`; section ids `norm-inputs-<path>`). This is a structured tree editor, not a JSON dump.
- `📚️catalogue-and-inspection`, `🪟️manifest-and-media` (`export_media`, `import_media`, ports `model:in`, `report:out`, `artifact:out`, `artifact:in`), `⌨️command-args-and-dispatch` (`dispatch_norm_command`, `dispatch_set_field`), `🏭️command-and-tool-job-macros` (`norm_owned_tool_job_factory!`, `build_norm_tool_job`, `norm_bounded_contract`), `🧵️retained-commands` (`NormRetainedEditor` trait), `🛡️store-ownership-macros` (`norm_exact_store_ownership!`, `norm_artifact_store_preparation`).
- Tests: `🧪️tests/🔬️retained-disposition-oracle/` and `🧪️tests/🔬️unit/`.

### 3.4 Results config `🪟️results/🎚️config/`

`norm_results_window_config_owner!(ResultsWindowConfigOwner, WINDOW_RESULTS)` supplies window config (`selected_check_index`), with `mutations::NormResultsWindowConfigMutation`. Its io is in `🚪️io/` (binary and text, snapshot and mutations).

## 4. Editor and viewer composition (EN 1993)

Path base: `S93/✏️editor/`.

- `🦀️.rs` (app root): imports the commands and windows, declares `LABEL = "EN 1993"`, `VARIANT = "en1993"`, `CONTROLLER_ID = "s.norm.en1993@1/*#editor"`, `DOCUMENT_SCHEMA = "semio.norm.en1993/v1"`.
  - `semio_framework_plugin::app_commands!` defines `En1993Command` with six rows: `evaluate`, `setSelectedCheckIndex`, `setField`, `insertItem`, `removeItem`, `applyRemedy`. Row order is the wire ordinal. Appending is safe; reordering is a wire break.
  - `impl ArtifactEditor for En1993PlayApp`: sets `Snapshot`, `Mutation`, `Config = NoConfig`, `Draft = NoDraft`, `Presence = NoPresence`, `Transient = NoTransient`. Its `handle` delegates to `dispatch_norm_command`. Its `render(body_key, ...)` matches the body keys `norm.en1993.play.inputs`, `.results`, `artifact`, `catalogue`, `inspection` and calls `inputs::render`, `results::render`, `document_panel`, `catalogue_panel`, `inspection_panel`. `export_media`/`import_media` delegate to app-surface. `reference_birth` is an empty `En1993Snapshot::default()`. `examples()` returns `heb240_compliant` and `high_strength_connection` sources.
  - `bounded_first_step_tool_proofs!` lists the six tools.
  - `NormRetainedEditor` impl routes `setSelectedCheckIndex` to `selected_check::window_mutation`.
  - `NormFamily` impl `En1993Family` (`family_id = En1993`, `evaluate` calls `semio_s_artifact_norm_en1993_inferences::evaluate`).
  - `create_en1993_app()` returns `Editor::builder(EN1993_DIALECT)...` with `.mode_def(edit_mode::definition())`, `.default_mode_id(MODE_EDIT)`, `.window_kind_def(inputs)`, `.window_kind_def(results)`, `.default_layout(edit_mode::layout())`, `.panel_tab_def(document|catalogue|inspection)`, `.action_with(...)`, `.action_interactive_job(...)`. Actions: `evaluate` (View), `setSelectedCheckIndex` (view action), `setField` (catalog action with args `path` text and `value` any), and the rest.
- `🎮️commands/<name>/🦀️.rs` (one per command: `🧮️evaluate`, `☑️selected-check`, `✏️set-field`, `➕insert-item`, `➖remove-item`, `🩹apply-remedy`). Each has a `Payload` struct with `#[dsl(keyword = "...")]` and `ToValue/FromValue/DslRecord`, and a `handle(payload, doc, cfg) -> Result<Emit<Mutation, ConfigMutation>, Fault>`. Examples:
  - `evaluate`: payload is empty; `handle` returns `Emit::default()` (no mutation). The report is derived on every read.
  - `set-field`: `SetField { path, value_json }`, `handle` calls `dispatch_set_field(doc.snapshot, path, value_json, |doc, edit| EDIT_RULES.resolve(doc, edit))`.
- `🎚️config/🧬️schema/` (empty config schema for the editor), `👥️presence/` and `🫧️transient/` (empty placeholders, `📌️.empty.md`), `🧪️tests/🔬️unit/`.
- `🏷️field-meta/🦀️.rs`: `en1993_field_meta` plus choice tables (`ANNEX`, `ACTION_KIND`, `ACTION_CATEGORY`, `MEMBER_TYPE`, `LOAD_APP`, `ANALYSIS`, `MOMENT_DIAGRAM`, `MATERIAL_KIND`, `JOINT_KIND`, `JOINT_CATEGORY`, ...), each choice with `value`, `label_en`, `label_de`. Used by `render_document_editor` through `meta_fn` for display labels, units and enums.
- `🎭️modes/✏️edit/🦀️.rs`: `definition()` delegates to `edit_mode_definition()`; `layout()` is a row split 42/58 of `inputs` and `results` windows.
- `🪟️windows/📥️inputs/🦀️.rs`: `WINDOW_INPUTS = "norm-en1993-inputs"`, `BODY_INPUTS = "norm.en1993.play.inputs"`, icon `download`. `render` calls `render_document_editor` with field meta. Its header comment still says "rendered as JSON"; this is stale (see section 7).
- `🪟️windows/📊️results/🦀️.rs`: `WINDOW_RESULTS = "norm-en1993-results"`, `BODY_RESULTS = "norm.en1993.play.results"`, icon `bar-chart-3`, config owner `ResultsWindowConfigOwner`, `render` uses `render_report(host.report(), ...)`.
- Panels: `📌️panels/🗿️artifact/`, `📚️catalogue/`, `🔍️inspection/` (each `🦀️.rs` plus `🧪️tests/`).
- `✏️editor/📦️packages/🦀️rust/Cargo.toml`, `🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/🟦️.ts` (typed view-model twin), `🧬️schema/` for editor state, examples in `📚️examples/🎬️demo-session/`.

**Event flow:** UI action → `Command` (`En1993Command`, decoded from action id or keyword by `norm_command_from_action!`) → `handle` in `🎮️commands/*` → `Emit<En1993Mutation, ...>` (applied by the store as event-sourced mutations; mutations come from `EDIT_RULES` via `dispatch_set_field`/`insertItem`/`removeItem`) → snapshot changes → next `render` re-runs `NormHost::from_artifact` and `evaluate`. Results are derived; they are never stored.

**Viewer:** `👁️viewer/🦀️.rs` implements `ArtifactViewer` only (no mutation emit). `👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/🦀️.rs` is the report window. Viewer config/schema mirror the editor's, minus commands.

## 5. Field meta, commands and events (summary)

- A command emits events only through `Emit<Mutation, ConfigMutation, DraftMutation>`. `Emit::default()` means no event (used by `evaluate`).
- `field_meta` is a plain table lookup (`NormFieldMetaFn`): exact keys, `[]` list wildcards, longest prefix wins. It is data, not schema; schema labels come from the `x-semio-ui` blocks in `🔣️.json`.

## 6. Tests

### 6.1 Unit tests

- Rust unit tests are mounted with `#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` in each node. Examples: `💡️inferences/🧪️tests/🔬️unit/`, `💡️inferences/🧪️tests/🔬️compliance-report/`, `✏️editor/🧪️tests/🔬️unit/`, `🪟️windows/📥️inputs/🧪️tests/🔬️unit/`, `📇️registry/...app-surface/🧪️tests/🔬️unit/`.
- Compliance oracle: `S93/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` runs `check_full_steel_structure` on DE and EN annex variants and compares to the Python oracle values.
- Kind/catalog consistency: `S93/🧪️tests/🔩️mutate-en1993-1/🦀️.rs` (test `kinds_match_the_enum_and_the_catalog`), `S93/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🦀️.rs`.

### 6.2 Language-agnostic `.feature` (Gherkin)

- `S93/🧪️tests/🔩️mutate-en1993-1/🥒️.feature`: "Apply every typed EN 1993 mutation against an independent Python implementation". Tags `@capability-en1993-1-mutate @oracle-en1993-1-python-independent @comparison-ordered-json-v1 @mutations-en1993-1-any`. Scenario Outline `Apply <id> to its committed specification vector`. Reads `shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json` and `shared://.../🦠️mutation/🔣️.json`. 49 `✅apply` rows, 16 `⛔dupe` refusal rows (`mutation.duplicate-id` Fatal, `mutation.target-missing` Error, `mutation.no-op` Warning). The `.feature` is the spec; `🦀️.rs` and `🐍️.py` in the same folder are the step adapters (Rust SUT adapter with `#[cfg(feature = "sut")]`, Python reference adapter importing `semio_norm_vocabulary`).
- Other fifteen artifacts have equivalent `🥒️.feature` files (`mutate-<artifact>-1`).

### 6.3 Python oracles (`🔮️oracles/`)

- Shared engine `P/🔮️oracles/🏃️execution/🐍️.py` (module `🏃️execution`): imports `semio_repo_test` (`Adapter`, `Outcome`, `digest`) and three packages `semio_norm_vocabulary_document`, `_verbs`, `_carrier`. Helper files: `🔧️verbs/🐍️.py` (mutation verbs, inverse), `📝️carrier/🐍️.py` (DSL carrier, digest), `📄️document/🐍️.py`. The module docstring documents that no third-party library reads or writes `s.norm.*`, and that the `.dsl.semio` carrier has no published grammar.
- Per-subset: `S93/🔮️oracles/⚖️compliance/🐍️.py` (independent EN 1990 combinations and governing steel checks, SI units, pure stdlib: `json`, `math`, `pathlib`).
- `S93/🔮️oracles/🧬️snapshot-schema/🐍️.py`: validates a snapshot against its JSON Schema using `jsonschema` (imported; the script prints an error and exits if it is missing). This is the only third-party Python library used for oracle validation. Note that the EN 1993 manifest `oracles[0].package` is `""` and `license` is AGPL-3.0-only (the oracle entry itself is the in-repo Python package; `jsonschema` is the external dependency used by the snapshot-schema oracle).
- Ajv (JavaScript, `ajv`) is the strict JSON Schema oracle used by TS tests (`semioSchemaAjvV1` from `@semio-tech/framework/schema/oracles/validator`).
- Test-only libraries: `bun:test` (TS), `node:assert/strict`.

### 6.4 Fixtures

- Mutation fixtures: `S93/🧪️tests/🧫️fixtures/🧬️mutations/<emoji><kind>/` as above.
- Snapshot SQLite fixtures: `S93/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/{🎛️semantic.json,🔣️.json}`.
- Examples used as fixtures and demos: `S93/📚️examples/{✅️heb240-compliant, 🔩️high-strength-connection}/` (`🦀️.rs`, `🟦️.ts`, `🧪️tests/🧩️example`, `🖼️assets/*.semio`).
- Root fixtures: `P/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` and `P/🧪️tests/🧫️fixtures/🪪️document-contract/🔣️.json` (vectors: `artifacts`, `invalidChildren`, `committed`). `P/🧪️tests/🧫️fixtures/` also holds a `🔬️check-report/🔣️.json` under `⚖️compliance/🧪️tests/`.

### 6.5 Plugin-root tests (`P/🧪️tests/`)

- `🏷️mutation-label-census/🟦️.ts`: `bun:test` census over every norm leaf's `label()` across fifteen families (locales, no field identifiers, no English reuse).
- `🧪️wire-twins/🟦️.ts`: every committed `🦠️mutation`, `📸️snapshot`, `🔺️diff` vs TS twin vs Ajv (`semioSchemaAjvV1`), byte round-trip, negative witnesses, undeclared member and unknown tag refusal.
- `🪪️document-contract/🟦️.ts`: `testNormDocumentContractOracle()` for each owner in `🧫️fixtures/🪪️document-contract/🔣️.json`, calls `assertDocumentContractOracle` from `@semio-tech/framework/schema/oracles/document`, checks child kind `s.stdio.semio`, mutation roots.
- `🔬️typescript-package-commands/🟦️.ts`: `runTestScript` (invoked by root `📜️script.ts test`), checks the TS package and fixture contract.

### 6.6 Running tests (repo MCP)

Use the `mcp__repo__test` tool (`project`, optional `test_level`, `skip_cache`, `language`) or `mcp__repo__run` (`target` in `project:target` form). Exact names taken from the `📋️project.json` files:

| Purpose | Project | Target |
|---|---|---|
| EN 1993 Rust unit tests | `@semio-tech/norm-en1993-rs` | `test` |
| EN 1993 SQLite snapshot (both) | `@semio-tech/norm-en1993-rs` | `test-snapshot-sqlite` |
| SQLite native / source | same | `test-snapshot-sqlite-native` / `test-snapshot-sqlite-source` |
| Rust build and check | same | `build` / `check` |
| EN 1993 TS package | `@semio-tech/norm-en1993` | `test` / `build` / `check` |
| Kernel-independent Rust contract | `@semio-tech/norm-artifact-contract-rs` | `test`, `test-byte-property` |
| Shared TS suites (wire twins, label census, document contract) | `@semio-tech/norm-js` | `test`, `test-wire-twins` |

Examples:
- `mcp__repo__test` with `project: "@semio-tech/norm-en1993-rs"`.
- `mcp__repo__run` with `target: "@semio-tech/norm-en1993-rs:test-snapshot-sqlite-native"`.
- `mcp__repo__run` with `target: "@semio-tech/norm-js:test-wire-twins"`.

Not verified in this audit: the exact invocation that runs the Gherkin `🥒️.feature` plus Python oracle pair (`mutate-en1993-1`). It is probably part of the framework test platform (`testContributionDirName`, `🏭️inventory`) rather than a per-project target. Check with `mcp__repo__show_targets` and the `🏭️bridge` inventory before relying on a specific command. Also do not run `cargo`, `nx` or `bun` directly (AGENTS.md).

## 7. Rules and conventions

### 7.1 `P/AGENTS.md` (the only plugin AGENTS.md; no nested AGENTS.md exists under the plugin)

It declares `technology: norm`, `emoji: 📏`, and describes the families. Its link targets (`core/rs/lib.rs`, `din/`, `en/`, `iso/16757/rs/lib.rs`, `vdi/3805/rs/lib.rs`) do NOT match the actual tree (`⚖️compliance/`, `🗿️artifacts/...`). Treat as stale; do not copy its paths.

### 7.2 Repo-level rules that bind norm work (from `/home/deck/semio/AGENTS.md`, mandatory)

- No modifying git commands (commit, stash, checkout, reset, worktree). Others edit the same files; never revert them.
- Build and test only through repo MCP tools or the `repo` binary.
- Schema-first, test-driven, at least one language-agnostic test per feature, validated against a third-party library (Python oracles, Ajv).
- No runtime external libraries; no comments inside definitions; docstrings start with a unique emoji; concise code.
- English and German labels; `LocalizedLabel::native(en, de)` for UI and mutation labels; `LocalizedCopy::new(en, de)` for check text.
- Event-sourced CQRS; no CRUD, no CRDT; no legacy, no compatibility layers, no migration scripts.
- File paths below 256 characters. Ticket output and reports go in the ticket folder; generated logs in `TICKET/🗑️generated/`.

### 7.3 Conventions observed in the code

- Emoji-prefixed `//#region` markers and `#[path = "<emoji>/🦀️.rs"]` mounts. Every taxonomy node is a folder with `🦀️.rs` (Rust), `🟦️.ts` (TS twin), `🔣️.json` (schema), `🔗️.graphql`, `🛰️.proto`, `📌️.empty.md` (placeholder). Keep the sibling set consistent.
- Per-leaf mutation folders must carry `requiredLanguageSurfaces` (rust, json-schema, text, binary).
- Mutation kinds are kebab-case `verb-entity`; the verb set is fixed: `change`, `insert`, `remove`, `update`.
- Binary tags in `🔣️.json` are unique per subset.

## 8. Discrepancies and risks observed (for implementers)

1. The EN 1993 inputs-window header says "rendered as JSON", but the code calls `render_document_editor` (structured tree). Header is stale.
2. `✏️editor/🦀️.rs` has mojibake in comments (`ð®ï¸commands`), so the taxonomy paths in comments there are not reliable. Use the real path names from the tree (`🎮️commands`, `🎭️modes`, `🪟️windows`, `📌️panels`).
3. The wire-twin generator `🧪️s2-norm-ts-twins.ts` is referenced by generated-looking twins but lives only in a closed ticket folder. Edit twins by hand and keep the wire tests green.
4. `P/AGENTS.md` link paths are out of date.
5. `P/../🧑‍💻dev/📕️norm/` is a tracked mirror with its own `project.json`. Unclear status.
6. The policy allowlist `✏️s/🔌️plugins/🔒️policy-allowlist.json` still names paths under `📕️norm/🎛️apps/...`, which do not exist (artifacts are under `🗿️artifacts/`). Verify before adding new allowlist entries.
7. `⚖️compliance` and `🖥️app-surface` have no own `📋️project.json`. Tests run via the Cargo workspace or the artifact projects only.
8. Results are derived on each read (`NormHost::from_artifact`), with a cache keyed by document hash. Do not persist results as document state.

## 9. Worked checklist for implementers (EN 1993)

- New check: sections 2.5 steps 1 to 8.
- New entity: section 2.6 steps 1 to 9, and run the EN 1993 Rust and TS targets listed in 6.6 plus `@semio-tech/norm-js:test-wire-twins`.
- New UI field: add choice rows to `✏️editor/🏷️field-meta/🦀️.rs`, plus an `x-semio-ui` label in `🔣️.json`. No app-surface change is needed.
- New editor command: add a `🎮️commands/<name>/🦀️.rs` payload, append a row to `app_commands!` in `✏️editor/🦀️.rs` (append only), register the action in `create_en1993_app()` and `bounded_first_step_tool_proofs!`, and update `🧪️tests/🔬️unit`.
