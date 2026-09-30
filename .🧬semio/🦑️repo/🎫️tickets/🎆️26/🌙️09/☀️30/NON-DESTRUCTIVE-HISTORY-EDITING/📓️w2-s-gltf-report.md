# 📓️ W2-S-B1 — glTF schema/payload parity (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, parity executor W2-S-B1, 2026-09-30.

- **Contracts:**
  - `📋️design.md` §6 and §11, where the 07:50 evidence rule says wrapped phase leaves witness their `Apply` payload;
  - the W2-R and W2-S parity briefs in `🧭️plan.md`.
- **Scope:** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf`, i.e. the `♾️any` subset: 121 leaves, the aggregate, the shared
  snapshot and diff documents, the fixture corpus, the wire-form feature rows and the TypeScript twins.

## 1. Outcome

**Done.** Both lints report 0 findings for glTF.

For the payload lint this now means **every glTF leaf witnessed by the real wire**: 121 committed fixtures plus 254
wire-form feature rows. Before this WP, 0 fixtures were visible to it.

| Check, strict, `--under ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf` | Before this WP | After |
|---|---|---|
| `schema mutation-payloads` | 7 `opaque`; 0 fixtures seen (glTF committed `🧬️operation/🔣️.json`, a layout no lint reads) | **0 findings; 375/375 fixtures and rows clean; 121/121 leaves witnessed** |
| `schema mutation-inputs` | 0 findings (W2-R-stdio-a) | **0 findings; 228/228 inputs declared** |
| Strict Ajv oracle (`🧪️w2-s-strict-compile.ts`) | 0/121 before W1-D removed `x-semio`/`x-semio-mutation` | **121/121 leaves, 9/9 aggregates** |

The design changed mid-WP. At 07:08 the W2-S executor marked all 120 glTF phase leaves `#[mutation_leaf(contract = ::protocol,
payload = Apply)]`, and the W1-D derive (07:33) now makes `payload_value()`/`input_schema()` the editable `Apply` content, with
`Restore` inert. The final shape follows that:

- **Leaf document root** = the flat `Apply` payload, exactly what `payload_value()` yields and the reader reads.
- **Leaf `$defs/<PhaseEnum>`** = the leaf's whole wire: `{phase: apply, value: {$ref: "#"}} | {phase: restore, value: {$ref:
  diff.json}}`.
- **Aggregate branch** = `{mutation: const W, payload: {$ref: <leaf $id>#/$defs/<PhaseEnum>}}`.

### Schema size

| | HEAD | Before this WP (after W2-R + W1-D) | After |
|---|---|---|---|
| 121 leaf schemas | 4,041,374 B | 6,586,907 B | **271,385 B** |
| `📸️snapshot/🔣️.json` | 13,010 B | 33,741 B | 62,479 B |
| `🔺️diff/🔣️.json` | 3,790 B | 3,790 B | 61,649 B |
| **Total** | 4,058,174 B | 6,624,438 B | **395,513 B** (−94.0 % against before, −90.3 % against HEAD) |

The 51 inlined `GltfDiff` copies (about 125 KB each) and the duplicated nested labels under the hidden restore diffs are gone.
There is now one shared `diff.json`, which every leaf's wire enum references, and the records are defined once in
`snapshot.json#/$defs`. The aggregate document grew from 48,484 B to 52,387 B through the `#/$defs/<PhaseEnum>` pointers.

## 2. What changed

### 2.1 Shared schema documents

Both documents were written from the Rust `ToValue` wire.

- **`📸️snapshot/🔣️.json`**:
  - Previously loose: 7 opaque objects, no `additionalProperties`, and most `extras`/`extensions` missing.
  - Now the `GltfSnapshot` root plus every record as a `$defs` export, following the hand-rolled wire impls:
    - `componentType` is its numeric code;
    - `type` is `"SCALAR"`…;
    - a camera is `{type, perspective|orthographic}` flattened with its own members;
    - the attribute and morph maps are objects.
  - `required` is exactly the set of fields without `skip_serializing_if`, and the root keeps its `x-semio-state` markers.
  - W2-R's labels were harvested once and moved here (`🧪️w2-s-gltf-labels.json`), so set-snapshot's nested inputs still read.
- **`🔺️diff/🔣️.json`**: the complete `GltfDiff`.
  - An `Option<Option<T>>` slot is `anyOf [T, null]`.
  - The JSON slots use the `json_presence` wire `{state}` (`$defs/GltfJsonPresence`).
  - It has 14 index-keyed collection triples, whose items and changes are `$ref`s to the snapshot records, plus
    `GltfDiffDerivation` and `GltfTouchedRegion`.
  - The diff text document's broken `diff.json#/$defs/derivation` ref was fixed.
- **Export parity.** The 11 exports that exist in fewer formats by design carry `x-semio-formats`. The Rust snapshot facet
  gained per-name `pub use` of `GltfAccessorType`/`GltfComponentType`. The TS twin gained `GltfComponentType`,
  `GltfAccessorType`, `GltfMorphTarget` and `GltfCameraProjection`. `schema-export-incomplete` for the scope is down to 1, and
  that one is pre-existing: `GltfDiffTextDocument` in the text sub-document.

### 2.2 Leaves and aggregate

- **All 120 wrapped leaves.** The root is now the `Apply` struct, titled with the Rust type name. `$defs/<PhaseEnum>` is the
  wire enum; `node/rename` keeps its own `GltfChangeNodeNameRestore` restore. The script checks every leaf against its Rust
  source, and refuses to write if any of these fail:
  - `payload = Apply` marker present;
  - field names;
  - scalar types;
  - nullable `Option`s;
  - no `skip_serializing_if`.
- **Corrections.**
  - `change-alpha` and `change-sides`: now `{material, alphaMode}` and `{material, doubleSided}`. They used to describe the
    rejection struct.
  - Node, scene, mesh and primitive extras/extensions: `data` is the `{state}` presence over any JSON. It used to be an
    externally tagged `{"Bool": …}` enum.
  - Document and asset extras/extensions: `data` is required any-JSON.
  - `accessor/create`: `componentType` is the numeric code (with the 6 codes as snaps) and `kind` is `"SCALAR"`…. It used to be
    `UnsignedInt`/`Scalar`.
  - `camera/create`: `projection` is `{type, perspective|orthographic}`.
  - `node/transform`: the trs fields are required and nullable.
  - Missing `required` entries were filled, e.g. `mesh/rename` and `scene/rename` `value`, and `change-description`.
- **Aggregate.** The 120 wrapped branches now point at `<leaf>#/$defs/<PhaseEnum>`; set-snapshot keeps its plain `$ref`.
- **Lint rule extended (repo test module), in step with the W2-S wrapped-leaf support.**
  - `mutationAggregateBranch(layout, wire, leafId, wrapper)` refers a wrapped leaf's branch to `<leaf>#/$defs/<wrapper>`.
  - The aggregate check passes the wrapper's enum name.
  - The corpus gained 2 cases: an apply and an inert restore through a wrapped adjacent aggregate.
  - `bun test 🧪️tests/🧪️mutation-payload-parity/🟦️.ts` passes: 29 tests, 126 expects.

### 2.3 Fixtures and feature rows

- **Re-homed.** All 120 `🧬️operation/🔣️.json` became `🦠️mutation/🔣️.json` holding the aggregate wire.
- **camera/create** was corrected to the Rust wire.
- **Diffs regenerated.** 78 committed `🔺️diff` were replaced by the production-produced diff:
  - 71 were legacy per-leaf shapes;
  - 6 predated `json_presence`;
  - 1 restated an unchanged field.
- **New set-snapshot case** `📸️snapshot/📸️set/📸️replaces-the-document`. It is the witness set-snapshot lacked.
- **Feature rows.** The two `change-node-transform` rows in `🎬️scene/🧪️tests/🎬️mutate-gltf-2-0-scene/🥒️.feature` were
  canonicalized to the emitted `null`s.

### 2.4 Rust (`semio-s-artifact-stdio-gltf`)

- **Wire bug fixed.** 26 `Option<Option<T>>` slots of `GltfDiff` decoded a clear (`null`) back as "unchanged". A restore diff
  therefore lost its clears on every wire round trip (persisted history, replication): undoing a transform kept the scale.
  They now carry the repo idiom `deserialize_with = "deserialize_double_option"`.
- **Spurious derive removed** from the 67 apply payload structs and the 2 rejection structs: `dsl::MutationLeaf` +
  `#[mutation_leaf]`.
- **Dead tests replaced.** The 120 unmounted per-case leaf test directories were deleted. They referenced nonexistent modules and
  the old `🧬️operation` paths. They are replaced by one mounted law, `🧬️mutations/🧪️tests/🧪️fixture-corpus/🦀️.rs`, with
  5 tests:
  1. **Coverage:** every one of the 121 leaf kinds owns a committed case.
  2. **Each `🦠️mutation`:**
     - is a decode→encode fixed point;
     - is admitted by the aggregate schema (Rust `OwnedJsonSchemaValidator` over the aggregate, all 121 leaf schemas and both
       shared documents);
     - its `payload_value()` is the editable content, valid against `input_schema()`;
     - `with_payload_value` round-trips.
  3. **Each case:**
     - its snapshots are valid against `snapshot.json` and are fixed points;
     - an applied case lands before on after through the committed diff, which equals the produced diff;
     - a rejected case changes nothing and emits the committed code's class.
  4. **Each computed inverse:**
     - is admitted by the aggregate, so `diff.json` is judged against real Rust restore diffs;
     - an inert restore has `input_schema() == None`;
     - it restores before after a wire round trip.
  5. **Reader:** `semio_framework::mutation_input_defs` declares one input per payload member for all 121 `INPUT_SCHEMAS`.
- `semio-framework` was added as a dev-dependency.

### 2.5 TypeScript twins

- **Aggregate `GltfMutation`:** every payload is `GltfPhase<P>`, i.e. `apply P | restore GltfDiff`.
- **Diff twin:** the JSON slots are `GltfJsonPresence`.
- **Snapshot twin:** the new entities are in place, and a precedence bug was fixed: `GltfCamera` used to attach `name` only to
  orthographic cameras.
- **Transform twin:** the trs fields are `T | null`.
- tsc reports 0 errors in these files.

## 3. Verification (all run, results seen)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-payloads --under ✏️s/…/🧊️gltf` (strict) | exit 0: 375/375 clean, 121/121 witnessed, 0 findings |
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/…/🧊️gltf` (strict) | exit 0: 228/228, 0 findings |
| `bun 🧪️w2-s-strict-compile.ts ✏️s/…/🧊️gltf` and `--aggregates` | 121/121, 9/9 |
| `.venv/bin/python 🧪️w2-s-gltf-check.py`: Python `jsonschema` 4.26 Draft7 + `referencing` | 0 failures. Checked: 121 aggregate wires, 121 leaf contents, 242 snapshots, 117 committed diffs, 2 carrier wires; 635 `x-semio-ui` valid against manifest `$defs/InputUi` |
| `bun test 🧪️tests/🧪️mutation-payload-parity/🟦️.ts` (repo test module) | 29 pass, 0 fail |
| `cargo check -p semio-s-artifact-stdio-gltf` (gated) | ok. 2 pre-existing warnings (`component::*` in the crate root). |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2s-gltf cargo test -p semio-s-artifact-stdio-gltf --lib`, final state | **265 passed, 0 failed**, including the 5 `fixture_corpus_tests` |
| `bunx tsc` over the aggregate, diff, snapshot and transform twins | 0 errors in those files |

## 4. Open items (not fixed here)

1. **Do not `--apply` `🧪️w2-s-aggregate-rule.py` to glTF yet.** The tool still emits `$ref <leaf>` for wrapped leaves and would
   undo the aggregate (a dry run says "would write 1"). It needs the same wrapper rule as the lint (`<leaf>#/$defs/<wrapper
   enum>`). It is W2-S's script, so I did not edit it.
2. **Taxonomy (`verify taxonomy report`).**
   - Now that the bundles use `🦠️mutation`, the taxonomy recognizes them. It reports 121 `mutation-fixture-unpaired` and 14
     `mutation-payload-schema-authority-invalid` on the fixture entity directories.
   - The root cause is pre-existing: glTF's two-level `<entity>/<verb>` leaf layout is not a registered projection. Even a
     recreated per-case `🧪️tests/<case>` directory stays "directory-kind-unresolved" and unpaired (probed and removed).
     `♾️any` already had 597 `directory-kind-unresolved`.
   - The fix is registering the glTF layout, e.g. as mutation domain owners, in the hot `🔣️taxonomy.json`. That is
     coordinator-owned.
3. **The TS parsers are missing (`schema-export-parser-missing`, 66; 7 pre-existing).** Sharing the records as `$defs` makes
   them exports, and each TS export owes a `parse<Export>()` (contract §A). This debt is repo-wide: 397 such findings across
   stdio. It belongs in a TS-twin WP.
4. **The glTF leaf TS twins do not compile, and did not before this WP.** tsc reports 2,309 errors:
   - `'./🟦️'` self-imports of helpers that live in `♾️any/🔨️modules/🧬️mutation-support/*/🟦️.ts`;
   - `'../../📸️snapshot/🟦️.ts'`, which should be `../../../`.

   This looks like codemod damage. It belongs in the same TS-twin WP.
5. **The schema catalog needs one central `schema generate`.** It is needed for the new `$defs` exports and the leaves'
   `dependsOn` on `s.stdio.gltf.2.0.any`, and the hashes are stale.
6. **Build blockers seen and cleared.**
   - Peer compile breaks in `semio-framework-plugin` and `semio-framework-ui` were transient.
   - A fleet-wide fine-grain-lock deadlock: my run sat in `prebuild_lock_exclusive` for 21 min. I killed my own cargo, and the
     coordinator cleared the rest.
   - The final run is green.

## 5. Files

- **Ticket scripts** (kept):
  - `🧪️w2-s-gltf-schemas.py` (generator; idempotent, checks against Rust)
  - `🧪️w2-s-gltf-labels.json` (harvested W2-R labels)
  - `🧪️w2-s-gltf-rust.py` (derive strip, dead-test removal)
  - `🧪️w2-s-gltf-diffs.py` (committed-diff regeneration)
  - `🧪️w2-s-gltf-check.py` (third-party check)
- **Scratch**, in `🗑️generated/w2s-gltf/`: lint and test logs, a backup of the pre-change schemas and fixtures, the tsc config.
- **Schemas**, under `♾️any/🧬️schema/`:
  - `📸️snapshot/🔣️.json`
  - `🔺️diff/🔣️.json`
  - `🔺️diff/📝️text/🔣️.json`
  - `🧬️mutations/🔣️.json` (the aggregate)
  - the 120 leaf `🧬️schema/🔣️.json`
- **Rust**:
  - `🔺️diff/🦀️.rs`
  - `📸️snapshot/🦀️.rs`
  - the 69 leaf `🦀️.rs` files
  - `🧬️mutations/🦀️.rs`
  - the new `🧬️mutations/🧪️tests/🧪️fixture-corpus/🦀️.rs`
  - `📦️packages/🦀️rust/Cargo.toml`
  - removed: the 120 `🧬️mutations/<entity>/<verb>/🧪️tests/<case>/` directories
- **TypeScript**:
  - `🧬️mutations/🟦️.ts`
  - `🔺️diff/🟦️.ts`
  - `📸️snapshot/🟦️.ts`
  - `🌳️node/📐️transform/🟦️.ts`
- **Fixtures**, in `♾️any/🧫️fixtures/🧬️mutations/**`:
  - 120 re-homed to `🦠️mutation`
  - 78 diffs regenerated
  - 1 new set-snapshot case
  - camera/create fixed
- **Feature**: `🎬️scene/🧪️tests/🎬️mutate-gltf-2-0-scene/🥒️.feature` (2 rows)
- **Repo lint**, under `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/`:
  - `🧬️schema/📋️orchestration/🟦️.ts` (`mutationAggregateBranch` and the wrapper at the aggregate check)
  - `🧪️tests/🧪️mutation-payload-parity/🟦️.ts`
  - `🧫️fixtures/🧫️mutation-payload-parity/🔣️.json` (+2 cases)
