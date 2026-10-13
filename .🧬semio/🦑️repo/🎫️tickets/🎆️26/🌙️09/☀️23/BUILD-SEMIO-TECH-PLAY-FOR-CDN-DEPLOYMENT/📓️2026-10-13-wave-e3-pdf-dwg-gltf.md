# 🧱️ Wave E3: pdf, dwg and gltf format-family decoupling (2026-10-13)

Executor: Lane D (E3). Scope: `semio-s-artifact-stdio-pdf`, `-dwg` and `-gltf` under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, following laws L3, L5, L6 and L8 and §10 E3 of `📓️2026-10-13-scalable-build-architecture.md`, and the family method of `📓️2026-10-13-stdio-semio-split-plan.md` (decouple inside the crate first, then cut).

## Rules I followed

- Every cargo call ran in the foreground through `🚦️cargo-slot.sh`, with the private target dir `.🧬semio/🦑️repo/⚡️cache/e3/target`. The build-dir stayed the shared one. The private dir is deleted at the end.
- I used no git-modifying command, opened no ticket and started no sub-agent. I did not touch `🗑️generated`.
- I touched no file of another lane. While I worked, three of my gates were blocked by other lanes' in-flight edits: `semio-framework-plugin` (E2), `semio-framework-os-kernel` (E1) and the stdio-semio split crates (Lane C). I waited for green windows instead of editing their files (see Verification).

Inputs I created, all kept in `e3-format-split/`:

| Script | What it does |
|---|---|
| `🕸️module-graph.py` | Module-graph census. It walks the `#[path]` tree, resolves `crate::`/`super::` paths (including use groups, component modules and cfg(test) exclusion), folds files onto L6 layers, and reports layer sizes, the layer graph and forbidden edges. |
| `🕸️{dwg,gltf,pdf}-layers.json` | The target crate graph of each format, written as census rules. |
| `🔀️item-split.py` | Item-level split of one file. It reaches items from per-layer seeds, sends shared items to the base, emits one file per layer and makes cross-file items `pub`. |
| `🧭️canonical-paths.py` | Retargets every path that reaches an item through a re-export barrel (root `pub use`, `schema`/`engine` shims, moved items) to the owning module. It runs crate-internally and on external consumers. |
| `🧭️{dwg,gltf,pdf}-paths.json` | The barrel and search lists of each format. |
| `🧩️flatten-uses.py` | Flattens nested `use crate::{a::{..}}` groups so the path rewrite can see every leaf. |
| `✂️unqualify.py` | `unused_qualifications` fix for imported names. |
| `🏗️cargo.sh` | One slot-gated cargo call with the private target dir. |
| `⏳️await-upstream.sh` | Re-runs a check until no upstream framework crate fails. |
| `🧪️each-test.sh` | One process per test, `RUST_MIN_STACK=67108864`, alarm per test. |

## 1. Result in one table

| Crate | Step 1 (decouple inside the crate) | Step 2 (crate cut) |
|---|---|---|
| dwg | **Done.** The in-crate graph is a DAG: model ← read, model ← write, editor → {model, read}, aggregator → {model, editor}. The census reports 0 forbidden edges. | **Not done.** It is blocked by the composition contract (no `symbolPackages`, S3) and by the per-artifact workspaces (WS1). The cut plan is in §6. |
| gltf | **Done.** model ← {read, write, mutations, inferences}; facets and aggregator sit on top. The census reports 0 forbidden edges. | Not done, same blockers. Cut plan in §6. |
| pdf | **Done, except one cycle.** model ← cos ← read ← write; facets → read; conformance → facets; legacy → read/write. One forbidden edge remains, model → conformance: the base 1.7 SQLite codec dispatches the six subset conformance checks. | Not done. Same blockers, plus this cycle, which needs a framework-level dependency inversion (§6.3). |

Step 2 is not clean in the current layout, for three reasons. Each would force a hack I may not ship:

1. **The composition contract binds every symbol to `package`.** `CompositionContributionV1` has only `package`. The hub stdio generator rejects any app `type`/`factory` or mutation `snapshot`/`mutation` path that does not start with `<package>::`:
   - `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts` lines 87–88 and 146–147.

   Every cut moves `PdfSnapshot`, `DwgSnapshot` and `GltfSnapshot` (the mutation descriptors) and every editor type out of `package`. The only ways around this are re-export shims (forbidden) or the `symbolPackages` contract extension. That extension is split-plan wave 3 (S3), owned by Lane C, and it has not landed (`rg symbolPackages` finds nothing).
2. **Each artifact is its own Cargo workspace with its own lockfile.** Cutting 4–7 crates per format means new `[workspace.dependencies]` rows in every consumer workspace: 36 manifests and 45 lockfiles for pdf, 49 and 49 for dwg, 26 and 33 for gltf. §10 orders E3's cut after WS1 for exactly this reason ("makes adding crates a generator step").
3. **The pdf model → conformance cycle** (§4.3).

## 2. Baseline, measured before any edit

Lib tests ran one process per test (`🧪️each-test.sh`, 6 parallel, 300 s alarm, `RUST_MIN_STACK=67108864`). Binaries came from `cargo test --lib --no-run` per artifact workspace.

| Crate | Tests | ok | FAILED | ABORT/TIMEOUT | Wall |
|---|---:|---:|---:|---:|---:|
| pdf | 1018 | **943** | 75 | 0 | 323 s |
| dwg | 112 | **109** | 3 | 0 | 139 s (dwg and gltf together) |
| gltf | 330 | **266** | 64 | 0 | (see dwg) |

Failure clusters before any edit:

- **dwg:**
  - 2 × `history_edit_inputs_resolve`: leaf payload schemas do not resolve.
  - 1 × the TypeScript sqlite interop law: missing node module.
- **gltf:**
  - mutation-corpus `committed_case_holds_the_corpus_law` diffs;
  - `absorb_law_insert_then_set_field_patches_into_added`;
  - an `edit_rules` path-missing case.

Other baseline checks, all measured:

- **Native `--lib` checks:** pdf, dwg and gltf all green. Own warnings: pdf 2, dwg 1, gltf 5.
- **wasm32-wasip2 `--lib` checks:** all three green.
- **Hub wasm workspace check:** `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2` was green (323 s).
- **Hub stdio `editor_catalog`:** 177 tests. The pdf/dwg/gltf rows plus the catalog law gave **27/27 ok** (one process per test, 4 parallel).

## 3. dwg (step 1 done)

### 3.1 What changed

**`🚪️io/🦀️.rs` (11,685 lines) was split by item reachability** (`🔀️item-split.py`, seeds per layer):

| Destination | Layer | Contents |
|---|---|---|
| `🚪️io/📥️import/🦀️.rs` (new, module `io::import`) | read | 5,445 lines: R2004 header and page decrypt, LZ77 decompress, section map, R2007 container, every `decode_*`, `dwg_from_bytes`, `decode_dwg` |
| `🚪️io/📤️export/🦀️.rs` (new, module `io::export`) | write | 5,150 lines: LZ77 compress, R2004 writer, `DwgBitWriter`, every `encode_*`, `dwg_to_bytes`, `encode_dwg`, `polylines_to_dwg_bytes`; it also mounts the round-trip test tree |
| `🧬️schema/🧮️geometry/🦀️.rs` (new, admitted `members-of-schema` slug) | model | the structural drawing model (`DwgDrawing`/`DwgLayer`/`DwgColor`/`DwgEntity`/`DwgGeometry`/`DwgPathSegment`) and its pure mesh, path and SVG projections |
| `🚪️io/💾️binary/📸️snapshot/🦀️.rs` | model | the pack codec, plus a new `🧶️Wire` region with the vocabulary both directions share: version stamp, header decrypt, CRC16, sentinels, `DwgBitReader`, evaluation-graph indexing. The `native_snapshot_codec` private-module-plus-glob was flattened. |
| `🚪️io/🦀️.rs` | model | the facets (composition, analysis, construction), `io_registry`, and the representation mounts |

A missing `//#endregion 🧱️R2004DirectoryDecode` in the source was fixed first, so every emitted region nests.

**Root (aggregator) changes:**

- Removed the legacy shims: `pub mod schema`, `pub mod engine`, both `standards::*::engine` barrels, every root `pub use` of model, io and contract items, and the trailing `pub use …io::{DwgBuilder…}`.
- Moved `STDIO_DWG_DOCUMENT_SCHEMA` and `DWG_ARTIFACT_SCHEMA_ID` into the ac1024 schema (model).
- Rewrote the stale declaration doc.

**Tests:** the four io test trees moved under `📤️export/🧪️tests/` and their relative includes got one more `../`. The section codecs that the round-trip laws read (`decode_r2004_sections`, `r2004_section_data`, `DwgRawSection`, …) are now `pub` read-side API. In a cut, write's tests take read as a dev-dependency.

**Consumers:** every path was canonicalized. That covers 30 external files plus the two `mutate-dwg-*` sources. Examples:

- `semio_s_artifact_stdio_dwg::DwgDrawing` → `…::standards::v_ac1024::subsets::any::schema::geometry::DwgDrawing`
- `…::dwg_to_bytes` → `…::io::export::dwg_to_bytes`
- `…::io::binary::snapshot::decode_dwg` → `…::io::import::decode_dwg`
- `…::apply_mutation_checked` → `semio_s_artifact_stdio_contract::apply_mutation_checked`

### 3.2 Census before and after

`🕸️module-graph.py` with the same final rules `🕸️dwg-layers.json` on both trees. "Before" is the `HEAD` tree exported by `git archive` into the scratchpad. Non-test own lines:

| Layer | Before (HEAD) | After | Target crate |
|---|---:|---:|---|
| model | 20,512 (incl. the 11.7 k mixed io file) | **9,746** | `…-dwg-model` |
| read | 34 | **5,445** | `…-dwg-read` |
| write | 26 | **5,150** | `…-dwg-write` |
| editor + viewer + examples | 1,410 | 1,410 | `…-dwg-editor` |
| aggregator | 631 | **558** | `semio-s-artifact-stdio-dwg` |

- **Forbidden edges:** 24 before (model → aggregator 21 through the root `schema`/`engine` shims and root re-exports, read → aggregator 2, write → aggregator 1); **0 after**.
- **Remaining edges:** aggregator → model 2, editor → model 12, editor → read 1 (the `architectural` example decodes), read → model 6, write → model 6.

## 4. gltf (step 1 done)

### 4.1 What changed

**`🚪️io/🦀️.rs` split:**

| Destination | Layer | Contents |
|---|---|---|
| `🚪️io/📥️import/🦀️.rs` (new) | read | `parse_gltf_document`, `decode_glb`, buffer resolution |
| `🚪️io/📤️export/🦀️.rs` (new) | write | `serialize_gltf_document(_owned_controlled)`, `encode_glb`, the pretty JSON writer; the io unit tests moved under it |
| `🚪️io/🦀️.rs` | model | base64 `data:` URIs, accessor decode, the GLB constants and the accessor model's value and serde mapping. These are inherent and trait impls on model types, so they must stay with the model. |
| `🪆️subsets/♾️any/🦀️.rs` (the subset leaf, now mounted `pub use component::*`) | aggregator | the facets: construction needs mutations, analysis needs the reader, plus `derive_artifact_facets!` and `io_registry` |

**The document mutation bridge** (`gltf_mutated_document`, `gltf_inverse_restored_document`, read → apply → write) moved from the mutations text codec to the subset leaf. It needs read, write and mutations, so it sits in the aggregator. The `mutate-gltf-2-0-*` corpus sources were retargeted.

**Root changes:**

- Removed the shims: `pub mod schema`, `pub mod engine`, the `standards::v2_0::engine` barrel, and the root `pub use` of contract fns, model types and facet types.
- Moved `STDIO_GLTF_DOCUMENT_SCHEMA` into the schema.
- Rewrote the stale declaration doc.

**Consumers:** 27 external files were canonicalized.

### 4.2 Census before and after

| Layer | Before (HEAD, final rules) | After | Target crate |
|---|---:|---:|---|
| model | 9,310 (with mixed io) | **8,655** | `…-gltf-model` |
| read | 38 | **132** | `…-gltf-read` |
| write | 31 | **159** | `…-gltf-write` |
| mutations | 12,809 | **12,782** | `…-gltf-mutations` |
| inferences | 6,582 | 6,582 | `…-gltf-inferences` |
| editor + viewer + examples | 1,205 | 1,205 | `…-gltf-editor` |
| aggregator (root + subset leaf) | 1,218 | **1,629** | `semio-s-artifact-stdio-gltf` |

- **Forbidden edges:** 143 before (mutations → aggregator 127, model → aggregator 6, inferences → aggregator 5, editor → aggregator 3, read/write → aggregator 2, all through root shims and re-exports); **0 after**.
- **The mutations axis is the real one for gltf.** It holds 37 % of the crate. read and write together are under 300 lines, which is below the ≈ 3–5 s per-crate overhead. §6.2 therefore cuts read and write into the model crate's siblings only when they grow (glTF extension decoders).

## 5. pdf (step 1 done, one cycle left)

### 5.1 What changed

**1.7 base `🚪️io/🦀️.rs` split:**

| Destination | Layer | Contents |
|---|---|---|
| `🚪️io/📥️import/🦀️.rs` (new) | read | `sniff_pdf`, `decode_pdf(_with_password)`, and the `page_text` free function. It replaces `impl PdfSnapshot { page_text }`: an inherent impl on a model type cannot live in the reader crate. |
| `🚪️io/📤️export/🦀️.rs` (new) | write | `encode_pdf(_with)`, the graft and reconcile writer, `DocumentStream`, and the builders `text_document`, `PdfTextLayout`, `embedded_true_type_font`, `unresolved_resources`, plus `blank_pdf_snapshot` and `demo_pdf17_snapshot` (both were model-file code that round-trips through the codec). The io unit tests moved under it. |
| `🚪️io/🦀️.rs` | model | gained the shared vocabulary: `PdfEngineError`/`PResult`/`malformed`, moved from the lexer, and the PDF text-string codec `decode_text_string`/`encode_text_string`/`PDF_DOC_ENCODING`, moved from fonts and fonts/tables. Before this move the model needed the COS engine. `derived_construction` stays in the model: it needs only the model. |
| `🪆️subsets/🧱️base/🦀️.rs` (new subset leaf) | facets | analysis, composition and `derive_artifact_facets!` (reader-level). The six conformance subsets extend these facets. |
| `🏅️standards/7️⃣1.7/🦀️.rs` (new standard leaf) | aggregator | `io_registry`, which composes the base and all six conformance composers |

**Other moves:**

- `WRITER_TRAILER_KEYS` moved from `📝️writer` to `🔗️xref` (cos), so the reader no longer reaches the writer.
- The root page-text helpers (`replace_page_unicode_text`, `page_text_agent_revision`, `page_text_edit_mutation` and their tests) moved to `editor::page`.
- `STDIO_PDF_DOCUMENT_SCHEMA` moved to the 1.4 schema and `PDF_ARTIFACT_SCHEMA_ID` to the 1.7 schema.
- Removed the `pub mod schema` shim and every root `pub use`.
- 157 files had nested `use` groups flattened by `🧩️flatten-uses.py`, and 30 external files were canonicalized.

### 5.2 Census before and after

| Layer | Before (HEAD, final rules) | After | Target crate |
|---|---:|---:|---|
| model (1.7 base schema + native codecs) | 18,343 (incl. the mixed io file) | **16,997** | `…-pdf-model` |
| cos (lexer, filters, encryption, xref, fonts, content, colour, images) | 5,968 | **5,911** | `…-pdf-cos` |
| read (lift + `io::import`) | 1,467 | **1,557** | `…-pdf-read` |
| write (lower, writer, `io::export`, builders) | 1,794 | **2,946** | `…-pdf-write` |
| facets (1.7 base subset leaf) | – | **128** | conformance crate |
| conformance (1.7 a/x/e/ua/vt/h) | 11,595 | 11,595 | `…-pdf-conformance` |
| legacy (1.4 base/a/x) | 4,129 | 4,130 | `…-pdf-1-4` |
| editor + viewer + examples | 8,423 | 8,567 | `…-pdf-editor` |
| aggregator (root + 1.7 standard leaf) | 1,520 | **1,448** | `semio-s-artifact-stdio-pdf` |

- **Forbidden edges:** 59 before (editor → aggregator 35, model → cos 7, legacy → aggregator 6, model → conformance 6, model → write 2, model → aggregator 2, model → read 1); **1 after**. The one left is model → conformance, which the census reports as → root because the code reaches the subsets through `use crate::standards::v1_7::subsets;`.
- write → read is allowed for pdf, and is the only such pair in the three crates. The writer reconciles typed lanes onto the retained graph by lifting it (`Reading::of → lift_document_with`), which is the documented `decode(encode(s)) == s` design. It is not a misplaced helper.

### 5.3 The remaining cycle: model → conformance

`🧱️base/🚪️io/🪶️sqlite/📸️snapshot/📄️document/🦀️.rs` implements `store::ArtifactSqliteSnapshot::validate_sqlite_snapshot_subset` for `PdfSnapshot`. That impl must live in the model crate (orphan rule), and it calls `subsets::{a,x,e,ua,vt,h}::io::check_*_conformance`, which are conformance-layer code. There are three ways out:

1. Move the six rule sets into the model. That grows it to ≈ 23 k lines, over L3's 15 k target.
2. Merge model and conformance. That gives ≈ 28.6 k lines, over L3's 25 k fail line.
3. **Invert the dependency in the framework.** `validate_sqlite_snapshot_subset` should resolve the subset validator registered for the dialect: the declaration already carries the same six `SubsetValidatorEntry`s through `pdf_1_7_subset_validators()`. The model codec then stops naming the subsets.

Option 3 is the clean one. It is a framework (os-kernel store / framework-os io) change, so it is out of this lane's file scope and is a prerequisite of the pdf cut (§6.3).

## 6. Cut plan (step 2), precise

**Common prerequisites, in order:**

1. **S3 contract.** `CompositionContributionV1.symbolPackages: string[]`, plus the hub stdio generator's prefix checks generalized to `package ∪ symbolPackages`. The editor and viewer extension rows (`🧩️extensions/{📘️pdf,🛠️cad,🔺️mesh}`) take their dependencies from `package + symbolPackages`. This is Lane C split-plan §4.7 and serves these three artifacts unchanged.
2. **WS1.** A product workspace with generated `members` and `[workspace.dependencies]`. Without it, each new crate needs a row in 26–49 consumer workspaces and regenerating 33–49 lockfiles. That is possible but not clean, and §10 orders E3's cut after WS1.
3. **Taxonomy admission** of `📦️packages/🦀️rust` at these nodes, gated by `bun nx run workspace:verify-taxonomy-report`:
   - `🚪️io/📥️import`
   - `🚪️io/📤️export`
   - `🧬️schema`
   - `🧬️schema/🧬️mutations`
   - `🧬️schema/💡️inferences`
   - `🔨️modules`
   - the subset and standard leaves

   Precedent for the first two is `🏙️bim/🏢️model`, which has `🚪️io/📥️import/🦀️.rs` and `📤️export/🦀️.rs`.

**Common mechanics.** The in-crate module paths were chosen so that the cut is a pure crate-prefix codemod:

- Each new crate root replicates the extern aliases (`dsl`, `protocol`, `store`, `framework_schema`, `value_derive`) and `#![allow(async_fn_in_trait, long_running_const_eval)]`.
- Each root `#[path]`-mounts its own files at the same `standards::…` module paths.
- Paths from another layer become `semio_s_artifact_stdio_<f>_<layer>::standards::…`.
- Every crate is rlib-only (L5), has no features (L2) and has a package name of 48 characters or fewer (L4; the longest is `semio-s-artifact-stdio-pdf-conformance`, 38).

### 6.1 dwg: 5 crates

| Crate | Package dir | Root | Deps |
|---|---|---|---|
| `semio-s-artifact-stdio-dwg-model` | `🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust` | `🧬️schema/🦀️.rs` mounting schema, geometry and the ac1024 `🚪️io/{💾️binary,📝️text,🪶️sqlite}` + io root, and the ac1018 schema/io | framework, contract, binary, mesh-engine |
| `…-dwg-read` | `🚪️io/📥️import/📦️packages/🦀️rust` | `📥️import/🦀️.rs` + the deserializer leaves (both standards) | model |
| `…-dwg-write` | `🚪️io/📤️export/📦️packages/🦀️rust` | `📤️export/🦀️.rs` + the serializer leaves; tests use `[dev-dependencies] …-dwg-read` and `acadrust` | model |
| `…-dwg-editor` | the existing package dir would collide, so a new `✏️editor/📦️packages/🦀️rust` at ac1024 | the editor and viewer trees of both standards + examples | model, read |
| `semio-s-artifact-stdio-dwg` (aggregator) | existing | root `🦀️.rs` | model, editor |

- **Composition `symbolPackages`:** `[-dwg-model, -dwg-editor]`.
- **Consumers by layer** (computed from canonical paths):
  - cad: model
  - presentation: model
  - note: model
  - layout: model, read, write
  - generation3d: model, read, write, aggregator
  - stdio-semio: model, read, write
  - framework doc comment: write
  - hub stdio: model, editor, aggregator
- **Expected sizes:** model 9.7 k, read 5.4 k, write 5.2 k. Today's 7.0 GB / 165–241 s unit came from one 11.7 k-line function soup. Using the architecture note's 0.6 GB + 38 MB per k-line model, the largest new unit is ≈ 1.0–1.5 GB (estimate; M0 measures it).

### 6.2 gltf: 5 crates now, 7 once read and write grow

| Crate | Package dir | Contents | Deps |
|---|---|---|---|
| `…-gltf-model` | `🧬️schema/📦️packages/🦀️rust` | snapshot, diff, schema root, io (base64, accessors, GLB constants), native codecs except the mutation and inference facets, and **read + write** (`io::import`/`io::export`, together < 300 lines) | framework, contract, json |
| `…-gltf-mutations` | `🧬️schema/🧬️mutations/📦️packages/🦀️rust` | schema mutations, `🔨️modules/{🧬️mutation-support,🧭️mutation-dispatch}`, `io/{💾️binary,📝️text}/🧬️mutations` | model |
| `…-gltf-inferences` | `🧬️schema/💡️inferences/📦️packages/🦀️rust` | schema inferences, the measurement and topology modules, the inference codecs, the decoded-accessor projection | model |
| `…-gltf-editor` | `✏️editor/📦️packages/🦀️rust` | editor, viewer, examples | model, mutations |
| `semio-s-artifact-stdio-gltf` | existing | root + subset leaf (facets, document mutation bridge, inference services) | all |

- The census already keeps read and write separable (`io::import`/`io::export` have no back-edges). They become their own crates as soon as either passes ≈ 1.5 k lines.
- **`symbolPackages`:** `[-gltf-model, -gltf-mutations, -gltf-editor]`.
- **Consumers:**
  - bim model: model, read, write
  - stdio-semio: model, read, write
  - generation3d: model, aggregator
  - process3d: aggregator
  - hub stdio: model, mutations, editor, aggregator

### 6.3 pdf: 8 crates (after the framework inversion of §5.3)

| Crate | Contents | Deps |
|---|---|---|
| `…-pdf-model` | 1.7 base schema, native codecs, io vocabulary (error, text strings), foreign-artifact port, mutation bridge, construction (17.0 k; split `-pdf-mutations` out if M0 measures > 3 GiB) | framework, contract |
| `…-pdf-cos` | `🔨️modules/{lexer,filters,encryption,xref,fonts,content,colour,images}` | model, binary, deflate |
| `…-pdf-read` | `🔨️modules/⬇️lift`, `io::import` (+ deserializer leaves) | model, cos |
| `…-pdf-write` | `🔨️modules/{⬆️lower,📝️writer}`, `io::export` (+ serializer leaves) | model, cos, read |
| `…-pdf-conformance` | 1.7 a/x/e/ua/vt/h + the 1.7 base facets leaf | model, cos, read, write |
| `…-pdf-1-4` | 1.4 base/a/x | model, cos, read, write |
| `…-pdf-editor` | every editor, viewer and example | all above |
| `semio-s-artifact-stdio-pdf` | root + `🏅️standards/7️⃣1.7/🦀️.rs` registry | all |

- **`symbolPackages`:** `[-pdf-model, -pdf-conformance, -pdf-1-4, -pdf-editor]`. The 1.4 snapshot and the conformance mutation descriptors are named in `🧩️composition/🔣️.json`.
- **Consumers:**
  - bim model: model, read, write
  - stdio-semio: read, write
  - draw: 1-4
  - note: 1-4
  - hub stdio: model, 1-4, editor, aggregator
- **Largest units:** model 17.0 k, conformance 11.6 k, editor 8.6 k. All are below L3's 25 k fail line, and model is over the 15 k target until `-pdf-mutations` (≈ 3.8 k schema + codecs) is cut.

### 6.4 L8 (the generic boundary at the aggregator)

These crates instantiate the following themselves in the aggregator:

- `ArtifactCodec::of`/`bare::<S, M>`
- `document_codec_bare::<S, M>`
- `composer_entry_of::<…>`
- `subset_validator_entry_of::<…>`

The hub-side monomorphizations are `VcsArtifactApp<EditorApp<…>>` in `🧩️extensions/{📘️pdf,🛠️cad,🔺️mesh}` and `typed_snapshot_owner!` in `🚢️shipped-fleet`. They stay in the hub until C1 adds the concrete `create_*_app()` constructors to each aggregator (or editor crate). This is recorded for C1 and is not done here, because C1 owns the hub-side change.

## 7. Verification (after the change, all measured)

Upstream was a moving target for the whole session. `semio-framework-plugin` (E2), `semio-framework-os-kernel` (E1), the stdio-semio split crates (Lane C) and the hub stdio catalog move (another lane) each failed to compile for long stretches. Every gate below was run in a window where its upstream compiled; `⏳️await-upstream.sh` re-ran the gate until it got one.

| Gate | Result |
|---|---|
| Native `--lib` check | **pdf, dwg and gltf green.** Own warnings are equal to or fewer than baseline: pdf 2 → 2, dwg 1 → 1, gltf 5 → 3. No warning was introduced. |
| wasm32-wasip2 `--lib` check | **pdf, dwg and gltf green.** Own warnings are the same as native. |
| `--lib --tests` check | **pdf, dwg and gltf green** after one upstream fix, described next. |
| Lib tests, one process per test (6 parallel, 300 s alarm, `RUST_MIN_STACK=67108864`) | **dwg 109/112** (baseline 109/112, identical per test). **gltf 265/330** (baseline 266/330). **pdf 936/1018** (baseline 943/1018). Every other test has the same verdict. The renamed tests were matched by stripping `::component`/`io::{import,export}`/subset-leaf path segments, and the 6 moved tests (gltf facets, pdf page text) all pass. |
| The 8 differing tests | All are load-flaky history laws, not regressions. They are gltf `editor::gltf…::history_edits_end_to_end` and pdf `editor::pdf17{,a,e,h,ua,vt,x}…::conflict_history_edits_end_to_end`. Rerun alone, the after binary passed gltf 2/2 and each pdf17 law at least once. Run concurrently on the same machine, the **before** binary fails them too: `pdf17e` before failed 2 of 3, after failed 2 of 3; gltf before failed 1 of 2, after failed 0 of 2. The failing pdf assertion is the plugin time-travel close (`time travel close retains an undeclared original session editor preview or authoring frontier`); my edits did not touch that code path. |
| Hub wasm workspace check | **Green.** `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2` finished in 7 min 57 s with 0 errors and 899 warning lines. Earlier attempts were red only because of other lanes' in-flight edits: Lane C's `base_foundation` API change, which had not yet reached 17 consumers, and the private `semio_hub_stdio_catalog::artifact_catalog_contribution`. Even in those red runs, my three crates and their direct consumers were green. |
| Hub stdio `editor_catalog` | **177/177 ok** (all tests, one process per test, 4 parallel, 300 s alarm). Built once the other lane's `semio-hub-stdio-catalog` move compiled. The 26 pdf, dwg and gltf rows plus the catalog law are 27/27, the same as the baseline. |
| Taxonomy gate (`workspace:verify-taxonomy-report`) | **No verdict.** It was stopped by the 1 h owner-command cap (`native:owner-command stopped: timeout 3600000ms`) while scanning 155,588 files under fleet load. The new paths only use precedented kinds: `🚪️io/📥️import/🦀️.rs`, `📤️export/🦀️.rs` (as in `🏙️bim/🏢️model`), `🧬️schema/🧮️geometry` (registered `members-of-schema`), the subset and standard leaves `🦀️.rs` (stated precedent: 29 of 31 subset/standard leaves), and `🧪️tests` (global kind). |

**Upstream fix made outside my lane (cross-lane, minimal).** E2 made `app_testing` `pub(crate)` but left `history_edit_acceptance_law!` naming `$crate::app_testing::history_edit_acceptance::…`. That broke every downstream artifact's editor tests with E0603: 12 errors in dwg, 6 in gltf, 60 in pdf. I changed the 5 macro paths in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` to the public re-export `$crate::app::history_edit_acceptance::…`; `pub mod app` globs `app_testing` under the same cfg. No other plugin file was touched.

**Unit timing and RSS.** No crate was cut, so there is nothing to compare. The planned before numbers are those in the architecture note §2.8: pdf 152 s / 5.4 GB, dwg 165–241 s / 7.0 GB, gltf 160 s / 3.6 GB. M0 measures the cut crates.

## 8. Files touched

- **dwg** (46 files): root `🦀️.rs`; ac1024 `🚪️io/🦀️.rs`, `🚪️io/📥️import/🦀️.rs` (new), `🚪️io/📤️export/🦀️.rs` (new), `🧬️schema/🧮️geometry/🦀️.rs` (new), `🚪️io/💾️binary/📸️snapshot/🦀️.rs`, `🧬️schema/🦀️.rs`, `🧬️schema/📸️snapshot/🦀️.rs`; the io tests moved to `🚪️io/📤️export/🧪️tests/{🔬️unit,🔬️standalone,🔬️polyline-io,🔮️acadrust-oracle}`; canonicalized paths in the schema, codec, sqlite, editor, viewer and examples files and in both `🧪️tests/🖊️mutate-dwg-*`.
- **gltf** (178 files): root `🦀️.rs`; `🪆️subsets/♾️any/🦀️.rs` (facets and document mutation bridge); `🚪️io/🦀️.rs`, `🚪️io/📥️import/🦀️.rs` (new), `🚪️io/📤️export/🦀️.rs` (new); `🧬️schema/🦀️.rs`; `🚪️io/📝️text/🧬️mutations/🦀️.rs`; `🚪️io/🧪️tests/🔬️unit` moved to `🚪️io/📤️export/🧪️tests/🔬️unit`; canonicalized paths across schema, mutations, inferences, modules, codecs and editor; the `mutate-gltf-2-0-*` sources.
- **pdf** (284 files): root `🦀️.rs`; `🏅️standards/7️⃣1.7/🦀️.rs` (new); 1.7 base `🦀️.rs` (new), `🚪️io/🦀️.rs`, `🚪️io/📥️import/🦀️.rs` (new), `🚪️io/📤️export/🦀️.rs` (new), `🚪️io/📦️foreign-artifacts`, `🚪️io/📝️text/📸️snapshot` and `🪪️retained-text`, `🔨️modules/{🔤️lexer,🔤️fonts,🔤️fonts/🅰️tables,🔗️xref,📝️writer,⬆️lower,…}`, `✏️editor/🖼️page/🦀️.rs`, `🧬️schema/🦀️.rs`; 1.4 `🧬️schema/🦀️.rs`; `🚪️io/🧪️tests/🔬️unit` moved to `🚪️io/📤️export/🧪️tests/🔬️unit`; 157 files with flattened `use` groups; canonicalized paths in conformance, legacy, editors and viewers.
- **External consumers** (canonical paths only, no behaviour change):
  - `🌎️hub/🧩️compositions/🗄️stdio/{🧩️extensions/🛠️cad/🦀️.rs, 🧪️tests/🚢️shipped-fleet/🦀️.rs}`
  - `✏️s/🔌️plugins/{📐️cad, 📏️layout, 🗒️note, 🎞️animate/🎬️presentation, 🌀️procedural/🧊️generation3d, 🏙️bim/🏢️model, 🏭️process/🧊️process3d, 🖍️draw, 💠️lowpoly}` io/serializer/deserializer/test files
  - `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{🖊️drawing,🔺️mesh,🧊️brep,📐️cad,…}` conversion leaves and tests
  - the `🧰️framework/📦️packages/🦀️rust/🦀️.rs` doc comment
  - 87 files in all: 30 dwg, 27 gltf, 30 pdf
- **Upstream:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs` (macro path, see §7).
- **Ticket inputs (kept):** `e3-format-split/{🕸️module-graph.py, 🕸️dwg-layers.json, 🕸️gltf-layers.json, 🕸️pdf-layers.json, 🔀️item-split.py, 🧭️canonical-paths.py, 🧭️dwg-paths.json, 🧭️gltf-paths.json, 🧭️pdf-paths.json, 🧩️flatten-uses.py, ✂️unqualify.py, 🏗️cargo.sh, ⏳️await-upstream.sh, ⏳️await-condition.sh, 🧪️each-test.sh}`.
- **Private cache:** `.🧬semio/🦑️repo/⚡️cache/e3/` (target, binaries, per-test logs) is deleted at the end.

## 9. Corrections entry

Appended as entry **86** (`wave-e3`) to `📓️2026-10-10-migration-corrections.md`.
