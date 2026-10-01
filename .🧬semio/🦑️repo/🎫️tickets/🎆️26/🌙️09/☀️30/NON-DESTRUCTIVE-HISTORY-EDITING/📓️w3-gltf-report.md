# 📓️ W3-GLTF — glTF TypeScript twins and the two-level leaf taxonomy (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W3-GLTF, 2026-09-30 → 2026-10-01.
Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf` (subset `♾️any` = `A` below), plus the repo-tool gaps the glTF layout exposed.

## 1. Outcome

| Target | Before | After | Evidence (all run, results seen) |
|---|---|---|---|
| tsc, every glTF `🟦️.ts` (260 files, strict, `🗑️generated/w3-gltf/tsconfig.json`) | 2,857 errors (2,307 leaves, 525 contract probes, 25 other) | **0** | `bunx tsc -p …` exit 0 |
| `schema-export-parser-missing` / `-incomplete` (`test schema --under 🧊️gltf`) | 66 / 1 | **2 / 0** | the 2 left are generated surface twins, §4.1 |
| TS witness `A/🧬️schema/🧬️mutations/🧪️tests/🧪️fixture-corpus/🟦️.ts` | — | **486 pass, 0 fail** | `bun nx run @semio-tech/stdio-gltf:test` (owner runner) |
| `schema mutation-payloads` / `mutation-inputs` (glTF) | 0 / 0 | **0 / 0** | 375/375 witnessed rows, 121/121 leaves; 228/228 inputs |
| repo payload law `🧪️tests/🧪️mutation-payload-parity` | — | **42 pass** | `bun test` |
| `cargo test -p semio-s-artifact-stdio-gltf --lib` (gated, private target) | 265 pass | **265 pass, 0 fail** (2026-09-30 17:05) | incl. 121 case tests + 3 corpus laws; final rerun blocked by a peer, §4.2 |
| `verify taxonomy report --scope A` | 1,026 errors | **805** | `mutation-fixture-unpaired` 121→**0**, `mutation-payload-schema-authority-invalid` 14→**0**, `directory-kind-unresolved` 596→408, `path-too-long` 293→293, new `mutation-pair-path-budget` 102 (§3.3) |
| engine vector test `📚️library/🧪️tests/🧬️mutation-case-pair` | unregistered | **3 pass**, registered | `bun nx run @semio-tech/repo-lib:test-mutation-case-pair` |

## 2. TypeScript twins

**Decision: a leaf TS twin is a wire twin.** The 120 leaf files carried a TypeScript re-implementation of the mutation
semantics (`validate…`/`apply…`, 54 `📜️contract/🟦️.ts` law probes calling never-written `derive…Diff/Inverse`, one
`🔒️private/🟦️.ts`, four `🔨️modules/🧬️mutation-support/*/🟦️.ts`). Git history shows it never compiled: since its first
commit it imported nonexistent paths (`../../🔨️modules/…` from inside `🧬️schema`), called `run` with five arguments for
four parameters, and imported a `moveItem` no module defines; nothing imports it, nothing tests it, and the reference
artifacts (puzzle 2d, energy) have no per-leaf TS semantics at all. The Rust leaf is the implementation; its independent
checks are the fixture corpus, Ajv/npm `jsonschema`/Python validators and the three.js reader oracles. All 59 files were
deleted; the leaves now hold exactly their wire.

- **Shared readers** (`A/🧬️schema/📸️snapshot/🟦️.ts`, region `🚪️Wire`): `GltfWireRefusal`, `GltfWireReader<T>`, and
  combinators (`gltfWireObject/Tagged/Array/Tuple/Map/Nullable/Literal/Integer/…`). `gltfWireObject<T>` takes a member
  table typed `GltfWireMembers<T>`, so tsc proves every interface member has a reader and required/optional match;
  it builds a fresh object in wire order and refuses undeclared members.
- **Snapshot twin**: types corrected to the wire (members the Rust `ToValue` skips when empty/default are optional, as
  in `snapshot.json`), 37 `parse<Export>()`.
- **Diff twin** (`🔺️diff/🟦️.ts`): 25 parsers, `GltfPhase<P, R>` + `gltfWirePhase`. Text twin: `parseGltfDiffTextDocument`;
  its JSON export now declares `x-semio-formats` (Rust has no such struct) → `schema-export-incomplete` 1→0.
- **Leaves, aggregate, subset views**: generated schema-first by `🧪️w3-gltf-twins.ts` from the committed JSON Schemas
  (`--check` = 0 drift): each leaf exports the flat `Apply` payload, its `$defs` records, the phase enum
  (`GltfPhase<Payload, GltfDiff>`; `node/rename` keeps `GltfChangeNodeNameRestore`) and a parser for each. The aggregate
  `GltfMutation` mirrors `🧬️mutations/🔣️.json` branch for branch with a typed tag→parser table. The eight subset views
  (`🦴️skin/🧬️schema/🧬️mutations/…`) are `Extract<GltfMutation, …>` + parser; their JSON Schemas referenced the leaf root
  (flat payload) instead of the phase wire — fixed to `<leaf>#/$defs/<PhaseEnum>`.
- **Other compile fixes**: 11 inference `🧪️contract/🟦️.ts` imported vectors from a moved file (`../🧫️fixtures/🔣️.json`);
  two cast their fixture context; `parseGltfSnapshotText` returned an object for a string.
- **Witness test** (beside the Rust law, `🧪️fixture-corpus/🟦️.ts`): every committed `🦠️mutation`, both snapshots and the
  diff of all 121 cases decode through the twins and re-encode **byte-equal**; Ajv (`semioSchemaAjvV1`, strict) admits
  each, and for each wire a member spliced into every closed object (root, `payload`, `payload.value`, `document`,
  `asset`) is refused by both Ajv and the twin; an unknown tag likewise; every aggregate branch owns a case; a rejected
  case's diff is `🚫️.absent`. Negative control: swapping two snapshot members in the twin fails 243 tests.
- **Registration**: `runArtifactTypeScriptPackageMain(packageRoot, name, { suites })` (shared runner, optional): `check`
  type-checks each suite, `test` also runs it with `bun test`. The glTF package registers the witness. Negative control:
  a type error in a leaf twin fails `nx run @semio-tech/stdio-gltf:check`.
- **Fixture spelling**: 9 committed files spelled integral numbers `1.0` (Rust-regenerated diffs, the set-snapshot case);
  the other ~600 use the ECMAScript/RFC 8785 form `1`. JSON has one number type and the Rust law compares by value, so
  the 9 were rewritten value-preservingly (`🧪️w3-gltf-canonical-numbers.ts`).

## 3. Taxonomy: the layout was registered; the engine ignored it

### 3.1 Diagnosis (decided from how the other artifacts are laid out)

- The two-level `<entity>/<verb>` layout **is** registered in `🔣️taxonomy.json`:
  - `mutationDomainOwners[A/🧬️schema/🧬️mutations]` lists 30 domains and 121 leaves;
  - `mutationCatalogSourceOwners` routes the eight sibling catalogs to `A`;
  - `subsetDirectoryOverrides` names `♾️any` the any-subset of this owner.

  The architect program uses the same layout. glTF was itself `✳️any` and single-level until a deliberate, registered
  move on 2026-09-05. So there is nothing to register and nothing to move back: **no `taxonomy.json` edit**.
- The findings had three other causes.
  1. **Unpaired (121).** The normalization engine never consulted `subsetDirectoryOverrides`, so `♾️any` (and all 12
     override owners' subsets) stayed `directory-kind-unresolved`. The canonical-pair projection requires kind `subset`,
     so no glTF implementation case could pair. On top of that, W2 had deleted the 120 per-case implementation
     directories when it consolidated them into one law.
  2. **Authority-invalid (14).** `validateMutationPayloadSchemas` treated `🧫️fixtures/🧬️mutations/<x-y>` mirrors as
     mutation owners (`📣️used-extension` matches the owner-name pattern). This also hit puzzle 2d (12), architect (47),
     fem (29+29), note (33), draw (16) and others.
  3. **Fixture-side domain/verb directories** (148) never resolved; only the schema side knew `mutationDomainOwners`.

### 3.2 Changes

- **Engine** (`📚️library/🧹️normalization/🟦️.ts`):
  1. A declared `subsetDirectoryOverrides` directory under its exact `🪆️subsets` owner resolves to kind `subset`
     (name kept), and the projection captures its id through `subsetIdForDirectoryName`.
  2. The `🧫️fixtures/🧬️mutations` mirror of a registered domain owner resolves like the schema side
     (`mutationFixtureMirrorOwnerPath`).
  3. Fixture mirrors are never payload-authority owners.
  4. A catalog of a registered catalog owner outside the scope is read from disk instead of reported missing.
  5. A pair over the path budget still consumes its vector. "Unrealized: no physical bundle" was false for it, and the
     budget finding stays.
- **Rust (glTF crate)**:
  - 121 canonical implementation cases `A/🧬️schema/🧬️mutations/<entity>/<verb>/🧪️tests/<case>/🦀️.rs`, mounted by their
    leaf in place of `🧪️tests/🔬️direct-leaf`. Each keeps the leaf's semantic-identity assertion and calls
    `fixture_corpus_tests::assert_case("<entity>/<verb>/<case>")`. The rich `node/rename` module and the two material
    `🔬️unit` tests moved into their cases intact.
  - The corpus law (`🧪️fixture-corpus/🦀️.rs`) now holds `assert_case` (wire fixed point + aggregate + payload schema;
    outcome/diff/after; inverse restore). It caches the aggregate validator in a `OnceLock` and adds the law
    `every_committed_case_is_mounted_by_its_leaf_implementation_case`.
  - Generator: `🧪️w3-gltf-cases.py`, idempotent (`--check` = 0).
- **Catalogs**: the nine `🔮️oracles/🔣️.json` named long scenario directories that do not exist (the bundles had been
  shortened); `directoryName` now equals the committed bundle. `set-snapshot` gained its vector, and its case was renamed
  `📸️replaces-the-document` → `📸️replaces-all` to fit the budget (`🧪️w3-gltf-catalogs.ts`, `--check` = 0).
- **Engine witness**: `🧫️fixtures/🧬️mutation-case-pair/🔣️.json` became `cases[]` (schema updated). It now covers the energy
  single-level owner and glTF `📣️used-extension/➕️add/🔬️t056` (a domain owner under a subset override, a hyphenated
  domain). Ajv validates the vector. The test is registered:
  - `test mutation-case-pair` in the library `📜️script.ts`;
  - the nx target `test-mutation-case-pair` and its `package.json` script;
  - the `.vscode/launch.json` row `⚖️test-mutation-case-pair📚️library🟦️`.

### 3.3 What remains in the glTF report, and why

- **`mutation-pair-path-budget` (102), plus their 102 implementation directories still "unresolved".** The pairs now
  exist, but `A/🧫️fixtures/🧬️mutations/` alone is 151 bytes, and the bundle reserve is 42. 103 of 121 pairs therefore
  exceed 240 bytes under any layout; single-level would save about one segment and still exceed for most. The finding
  states the same root cause as the 293 pre-existing `path-too-long` (stdio depth versus the budget), which puzzle 2d
  (the reference) also shows (64). The 19 within-budget cases pair cleanly.
- **Pre-existing and not layout-caused**:
  - 120 `📜️contract` leaf child directories;
  - 66 inference and 5 module directories;
  - 2 `fixed-source-disposition` findings.
- **Engine impact elsewhere**, measured HEAD engine against the working engine, before fix 5.
  - Removed everywhere: authority-invalid, unpaired and unresolved findings. Examples: architect unresolved 626→293;
    fem 2d unpaired 117→0.
  - Override subsets now reach the pairing engine and surface masked defects in their owners' trees. Pair budget:
    fem 68/63, note 29. Catalog coverage: sequence 16. Member-unresolved: svg 9, xml 6. Draw: 1 fixture-invalid and
    2 implementation-invalid.
  - Fix 5 removes the "unrealized" double counts there. Logs: `🗑️generated/w3-gltf/engine-impact.txt`.

## 4. Open items (not mine; noted, not waited on)

1. **`NoConfig`/`NoPresence` `parse…()`** (the 2 parser findings): generated by the plugin registry's `surface-schema`
   projection (`🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts`, "do not edit", 527 files). The generator must emit parsers.
2. **The glTF crate does not compile right now** because of a peer's in-flight rollout. That uncommitted
   `🏪️store/🦀️.rs` change makes `ArtifactCodec::of` require `ArtifactSqliteSnapshot`. Handwritten `📸️snapshot/🪶️sqlite`
   impls landed for csv/tsv/html/svg/binary (01:42), not yet for glTF. The errors are at `🧊️gltf/🦀️.rs:55,99`, a file I
   did not touch.
3. **Every taxonomy command currently aborts** while loading the taxonomy. A peer simplified
   `semanticDescendantContracts.draw-editor-command-bundle-v1` (committed between 12:00 and 23:43). The engine validator
   (line 1350, unchanged since August) still requires three source-named Rust nodes. The 805-finding report and the
   engine test above were run before that.
4. **W2's scripts would regress the corpus.** `🧪️w2-s-gltf-diffs.py` writes Rust `1.0` spellings, and
   `🧪️w2-s-aggregate-rule.py` emits flat-payload `$ref`s. Do not re-run either as is.
5. **Repo-tool gap left as is**: the `🧫️fixtures/🧬️mutations` mirrors of *single-level* owners (energy and the others)
   remain unresolved. Resolving them lexically without an existence check would admit stray directories.

## 5. Files

- **Ticket scripts** (kept): `🧪️w3-gltf-twins.ts`, `🧪️w3-gltf-cases.py`, `🧪️w3-gltf-catalogs.ts`,
  `🧪️w3-gltf-canonical-numbers.ts`, `🧪️w3-gltf-taxonomy-inventory.ts`, `🧪️w3-gltf-engine-impact.ts`.
- **Scratch**: `🗑️generated/w3-gltf/` (logs, tsconfigs, pre-change backups `backup/*.tar`).
- **glTF, under `A/🧬️schema`**:
  - `📸️snapshot/🟦️.ts`, `🔺️diff/🟦️.ts`, `🔺️diff/📝️text/{🟦️.ts,🔣️.json}`, `📸️snapshot/📝️text/🟦️.ts`;
  - `🧬️mutations/🟦️.ts`, `🧬️mutations/🦀️.rs` (`pub(super)` law module), `🧬️mutations/🧪️tests/🧪️fixture-corpus/{🦀️.rs,🟦️.ts}`;
  - 121 leaf `🟦️.ts`, 121 leaf `🦀️.rs`, 121 `🧪️tests/<case>/🦀️.rs`;
  - 11 `💡️inferences/**/🧪️contract/🟦️.ts`.
- **glTF, elsewhere**:
  - the 8 subset views `🧬️schema/🧬️mutations/{🟦️.ts,🔣️.json}` and the 9 `🔮️oracles/🔣️.json`;
  - `📦️packages/🟦️typescript/📜️script.ts`;
  - fixtures: 9 number spellings, plus the set-snapshot case rename.
- **Removed**:
  - 54 `📜️contract/🟦️.ts`;
  - `🎬️scene/🌱️create/🔒️private/`;
  - 4 `🔨️modules/🧬️mutation-support/*/🟦️.ts`;
  - 120 `🧪️tests/🔬️direct-leaf/` and 2 `🧪️tests/🔬️unit/` (folded into their cases).
- **Repo**:
  - `📚️library/🧹️normalization/🟦️.ts`;
  - `📚️library/⚡️caching/📦️artifacts/🟦️typescript/📜️script.ts`;
  - `📚️library/{🧫️fixtures,🧬️schema,🧪️tests}/🧬️mutation-case-pair/*`;
  - `📚️library/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json,package.json}`;
  - `.vscode/launch.json` (one row).
