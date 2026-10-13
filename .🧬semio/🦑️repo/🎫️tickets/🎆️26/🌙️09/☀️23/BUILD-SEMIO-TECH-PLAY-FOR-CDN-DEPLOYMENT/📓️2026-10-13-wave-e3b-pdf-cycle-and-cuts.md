# ✂️ Wave E3b: pdf conformance cycle, hub stdio laws and the dwg/gltf crate cuts (2026-10-13)

Executor: E3b. Follows `📓️2026-10-13-wave-e3-pdf-dwg-gltf.md` (step 1, §6 cut plans) and `📓️2026-10-13-lane-c-semio-split.md` (`symbolPackages`, S3).

## Rules I followed

- Every cargo call ran in the foreground of its shell through `🚦️cargo-slot.sh` (`e3b-pdf-cycle/🏗️cargo.sh`), with the private target dir `.🧬semio/🦑️repo/⚡️cache/e3b/target`. The dir is deleted at the end.
- I used no git-modifying command, opened/closed no ticket, started no sub-agent and did not touch `🗑️generated`. Restores used `git show :<path>` (read-only).
- Upstream crates of other lanes (`semio-framework-plugin` E2b, `semio-framework-os-kernel` E1b, framework-3d E4) were repeatedly mid-edit. Gates waited for green windows with `e3b-pdf-cycle/⏳️await-upstream.sh` instead of editing their files. One exception, a one-line import of an E1b rename in the media-export contract, is listed under incidents.

Inputs I created (kept):

| Script | What it does |
|---|---|
| `e3b-pdf-cycle/🔀️snapshot-validators.py` | Rewrites the pdf subset io modules to `SnapshotSubsetValidator` impls and removes the dead per-subset registration. |
| `e3b-pdf-cycle/🏗️cargo.sh` | One slot-gated cargo call, private target dir, log under `⚡️cache/e3b/logs/<tag>.log`. |
| `e3b-pdf-cycle/⏳️await-upstream.sh` | Re-runs a cargo call while an upstream framework/contract/semio crate fails. |
| `e3b-format-cut/✂️crate-cut.py` | `plan`/`apply` crate cut from census layer rules: moves each layer's `#[path]` mounts under a new crate root, flattens and retargets `crate::` paths across crates (attributes repeated on flattened uses), relocates nested cross-layer mounts, rewrites external consumers and writes a consumer map. Audits upward edges. |
| `e3b-format-cut/🧾️consumer-manifests.py` | Gives each consumer package exactly the layer crates its sources name (normal vs dev) and adds the workspace rows. |
| `e3b-format-cut/{🖊️dwg,🧊️gltf,📖️pdf}-cut.json`, `{🧊️gltf,📖️pdf}-layers.json` | Cut configs (crate name, package dir, layer membership). |
| `e3b-format-cut/📏️measure-cuts.sh` | wasm32-wasip2 build of one artifact workspace with a 2 s per-rustc RSS sampler and `--timings`. |

## 1. Result

| Task | Result |
|---|---|
| 1. pdf model → conformance cycle | **Fixed at the framework level.** Census: pdf forbidden edges **1 → 0**. |
| 2. `Binary64Transport` unresolved in the pdf 1.7 schema | **Fixed at the root** (framework value schema is now a registered export; pdf keyword renamed to the admitted `x-` form). The TypeScript owned validator gained cross-document `$ref` resolution, so the pdf retirement parity test passes too. |
| 3. hub `native_openable_provider` 7/9 | **Both root causes fixed** (tsv preamble-less parse; the law asserted a canonicalisation the pack codec does not promise). |
| 4. dwg cut | **Done:** `-dwg-model`, `-dwg-read`, `-dwg-write`, `-dwg-editor` + aggregator. |
| 4. gltf cut | **Done:** `-gltf-model` (read/write inside), `-gltf-mutations`, `-gltf-inferences`, `-gltf-editor` + aggregator. |
| 4. pdf cut | **Not done, not clean in the current layout** (§5). |

## 2. Task 1: the pdf conformance cycle

The base 1.7 SQLite codec called the six subset checks (a, x, e, ua, vt, h) directly. That is a model → conformance edge; the subsets themselves sit above the model.

Framework inversion (`🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs`):

- `SnapshotSubsetValidator: SubsetValidator` with `type Snapshot` and `validate_snapshot(snapshot, database, control)`.
- `SubsetValidatorEntry.validate_snapshot: Option<SnapshotSubsetValidation>` (a type-erased fn pointer with a downcast thunk). `snapshot_subset_validator_entry_of::<V>()` builds it; `subset_validator_entry_of` keeps `None`.
- `validate_registered_snapshot_subset::<S>(dialect, snapshot, database, control)` looks the validator up in the existing process-global registry by dialect. `*` (base) is clean; a dialect without a snapshot validator refuses with `UnsupportedOwner`.

pdf side:

- The base codec (1.7 and 1.4) checks kind, standard and checkpoint, then calls `validate_registered_snapshot_subset`. It names no subset.
- The shared row checks moved into `validate_conformance_profile(snapshot, database, control, diagnostics, PdfConformanceProfile { encryption_forbidden, action_severity, page_box_required })` in the base snapshot module. Each subset passes its own profile.
- Each subset io module implements `SnapshotSubsetValidator`. The dead per-subset `VALIDATOR_ENTRY`/`register` code is gone; the root declaration registers `snapshot_subset_validator_entry_of::<…>()` for every 1.7 and 1.4 subset.

## 3. Task 2: `Binary64Transport`

Root cause: the pdf 1.7 snapshot schema references `framework/value/schema.json#/$defs/Binary64Transport`, but no scope registered the framework value schema as an export, so `structural_validator_in` could not resolve the `$id`. A second fault hid behind it: the pdf schema used a bare `semioPrimitive` keyword, which the validator rightly refuses (only `x-` extensions are admitted).

- `🌱️value/🦀️.rs`: `VALUE_SCHEMA_EXPORTS` (scope `framework.value`) and `register_value_schema_exports()`; the value crate depends on the schema registry crate.
- `publish_framework_schema_documents` (plugin registration) registers the value exports first.
- pdf 1.7 `🧬️schema/📸️snapshot/🔣️.json` + `🟦️.ts`: `semioPrimitive` → `x-semio-primitive`.
- TypeScript parity: `validateJsonSchemaSubset(schema, value, root, documents)` (`🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts`) now resolves `<$id>#/…` into sibling documents, mirroring Rust `compile_with_documents`. The portable corpus gained three cross-document vectors (AJV, Bun, bundled node and both Rust corpus tests agree). The two Rust corpus tests read the corpus' real shape (`input.schema`, `input.documents`); they read the absent `schema` key before.

## 4. Task 3: `native_openable_provider`

1. **tsv fixture.** The tsv `ArtifactDsl::parse_dsl` required the `semio ` DSL preamble, so a plain TSV payload failed. It now decodes plain TSV when the preamble is absent. The separate `read_tsv_source_text` helper is deleted; all readers call `parse_dsl`.
2. **Kernel wire encoding.** The law asserted that permuting an object's fields yields identical encodings. The pack codec documents the opposite: intrinsic `DslValue` objects retain occurrence order; only closed-schema maps sort. The law now asserts the encodings differ and that each decodes back to its own permutation. A stale "canonical encoding sorts" comment in the pack unit test is corrected.

## 5. Task 4: crate cuts

### dwg

| Crate | Dir (under `🏅️standards/🔟ac1024/🪆️subsets/✳️any/`) | Files | Lines (non-test / all) |
|---|---|---:|---:|
| `semio-s-artifact-stdio-dwg-model` | `🧬️schema/📦️packages/🦀️rust` | 83 | 9 864 / 10 625 |
| `semio-s-artifact-stdio-dwg-read` | `🚪️io/📥️import/📦️packages/🦀️rust` | 4 | 5 690 / 5 690 |
| `semio-s-artifact-stdio-dwg-write` | `🚪️io/📤️export/📦️packages/🦀️rust` | 7 | 5 325 / 6 208 |
| `semio-s-artifact-stdio-dwg-editor` | `✏️editor/📦️packages/🦀️rust` | 35 | 1 583 / 2 025 |
| `semio-s-artifact-stdio-dwg` (aggregator) | `📦️packages/🦀️rust` | 2 | 198 / 213 |

Graph: model ← read, model ← write, editor → {model, read}, aggregator → {model, editor}. Composition `symbolPackages: [model, editor]`; oracle `subjectHostPackages: [model, read, write]`.

### gltf

| Crate | Dir (under `🏅️standards/🔖️2.0/🪆️subsets/♾️any/`) | Files | Lines (non-test / all) |
|---|---|---:|---:|
| `semio-s-artifact-stdio-gltf-model` | `🧬️schema/📦️packages/🦀️rust` | 44 | 9 032 / 10 275 |
| `semio-s-artifact-stdio-gltf-mutations` | `🧬️schema/🧬️mutations/📦️packages/🦀️rust` | 284 | 13 092 / 15 262 |
| `semio-s-artifact-stdio-gltf-inferences` | `🧬️schema/💡️inferences/📦️packages/🦀️rust` | 161 | 6 672 / 7 490 |
| `semio-s-artifact-stdio-gltf-editor` | `✏️editor/📦️packages/🦀️rust` | 21 | 1 303 / 1 490 |
| `semio-s-artifact-stdio-gltf` (aggregator) | `📦️packages/🦀️rust` | 6 | 1 162 / 1 447 |

read and write stay in the model crate (under 300 lines together, below per-crate overhead; E3 §4). The binary/text codec mounts of mutations and inferences moved into their own crates under the same module path. The serde oracle in the model is `cfg_attr(any(test, feature = "testing"))`; only the mutations crate enables `testing`, as a dev-dependency. Composition `symbolPackages: [model, mutations, editor]`.

### Shared changes

- Workspace manifests: members via the membership generator, workspace dependency rows for every new crate, unused aggregator rows dropped. Consumers get exactly the layer crates their sources name (`🧾️consumer-manifests.py`). Root `🌎️hub/Cargo.toml` rows for the symbol packages.
- Semio conversion authority (`🧿️semio/🧩️composition/🔗️conversions/`): owners require `symbolPackages`, and a target may live in the owner package or one of its symbol packages. Schema, fixture and law updated (45 checks pass); generator rerun (26 targets).
- Hub stdio generator rerun (36 contributions, 176 apps, 30 receipts).
- Scripts: `📜️script.ts` of dwg and gltf list the family packages; metadata/grammar/native-schema tests target the crate that owns them.

### pdf: why the cut is not clean yet

The cycle is gone, but four test-structure facts would each force a shim or a cross-crate test hack:

1. 136 test files use `cfg(test)` conformance-support helpers built on read + write. They live in the model layer, so a model crate cannot compile them without depending on read/write.
2. The 61 base mutation tests are round-trip integration tests (write → read). They belong in the aggregator's integration tests, not in the model crate.
3. Full-stack tests sit inside the model's SQLite inline module.
4. The aggregator's `register_sqlite_test_declaration` is used by model and 1.4 tests.

The clean order is: move the conformance-support helpers into a `testing` feature of the read crate (dev-dependency only), move round-trip and full-stack tests to aggregator-mounted test modules, then cut with the existing `📖️pdf-cut.json`/`📖️pdf-layers.json`. My pre-cut test moves were reverted, so pdf is unchanged apart from Tasks 1 and 2.

## 6. Verification

PENDING

## 7. Unit peaks

PENDING

## 8. Incidents

- **Truncated manifest.** A first `🧾️consumer-manifests.py` opened the workspace root for writing before reading it, which truncated `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml`. Restored from the index (`git show :path`) and the script fixed (read, then write).
- **Census strip shifted offsets.** An early rewrite used a census strip that shortened raw strings by 2 bytes, which corrupted rewrites in `🌎️hub/…/🚢️shipped-fleet/🦀️.rs` (two lines, repaired by hand) and the gltf export unit test's conformance block (moved to the mutations tests). The tool now strips length-preserving.
- **Attributes on flattened uses.** Flattening `#[cfg(test)] use a::{…}` lost the attribute (dwg write, layout consumer), and once leaked a `#[cfg(test)]` onto the next item. Both are fixed in the tool.
- **Cross-lane import.** `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🎬️media-export/🦀️.rs` imported a store function E1b had renamed; I pointed it at `os_store::snapshot_read::artifact_retirement_admit_owned/box_close_step` so the hub could compile.

## 9. Open items

PENDING
