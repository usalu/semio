# 📓️ S3-STDIO — non-text stdio artifacts: path-scoped snapshot patches, leaf labels, agnostic findings

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, WP S3-STDIO (design §6, §11, §16.2, §20.3, §20.6). Owner files: the non-text
stdio artifacts under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/` (everything except md / html / txt, which are S3-TEXT's) and the shared
contract crate `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract`. Scratch: `🗑️generated/s3-stdio/`. Nothing committed; no ticket touched.

## Session 3 — 2026-10-02

### S3.0 Status log (newest first)

- 10-03 06:45 gltf `MutationLeaf source authority failed` (reported by S3-W2D via coordinator): gltf's mutation root is a registered
  domain-operation root, so `📸️snapshot/🩹️patch` must be registered — added `"🩹️patch": "patch-snapshot"` to `mutationDomainOwners`
  in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` and regenerated `✨️derive/🔣️mutation-authority.json`
  (`bun nx run @semio-tech/dsl-derive-rs:generate`, one-line diff, `check-generated` fresh). Proof: ticket input
  `🧪️s3-stdio-leaf-authority.ts` (taxonomy twin `mutationOwnerIdentity`) → gltf `patch-snapshot`, bmp flat ok; no other patch root is a
  registered domain root; `verify taxonomy report` clean for bmp/pdf patch dirs (gltf `📸️snapshot` scope: 2 errors on the existing
  `📸️set/📸️replaces-all`, not on `🩹️patch`). Gated `cargo check` gltf+bmp blocked by the red kernel (`IoError` refactor, 06:44).
- 10-03 06:45 source work complete (S3.2/S3.2a); gates: labels 0, payloads 0, editability 0, inputs 48 (all `leafUncatalogued`,
  coordinator action). Lib tests ran for 10 crates (S3.3); the rerun after fixes and the 11 red crates + dependents wait on a green
  kernel (peer `IoError` refactor in flight 06:37) and S3-INFRA's `ValueError`/`dsl::json` sweep. One-off edit helpers used for the
  codec-arm pass are kept as ticket inputs in `🧪️s3-stdio-codec-tools/`.
- 10-03 05:50 resumed on "TREE GREEN (core)". The tiff baseline editor edit (S3.2a) was already complete and compile-atomic.
  Remaining before cargo: semio image/animation + crate-root codec arms, bmp + gltf leaves, editor conversions.
- 10-02 ~21:00 cut by the usage limit during the per-aggregate codec-arm pass (gates measured 19:35, S3.3).
- 18:45 resumed after the usage cut (~13:05) and the reboot (~17:00). The 17:04 auto-commit `202c4b7b5b1` holds every edit below.
  The load-example verb edit (S3.2 item 1) was verified complete and compile-atomic (callees `app::CATALOGUE_EXAMPLE_ACTION_ID`,
  `app::runtime_verb_label` exist at `🔌️plugin/🦀️.rs:11974,11979`). Cargo waits on the peer schema-split fix (S3-INFRA).
- 12:40 started: read fleet rules, design, plan, closure census, agnostic report, stdio case reports. Baseline gates running.

### S3.1 Baseline (before any change, 12:45)

| Gate (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, `--under ✏️s/🔌️plugins/🗄️stdio`) | Result |
|---|---|
| `schema mutation-labels` | exit 1, **178** findings, all `labelHandwritten` (115 `Load example {example_id}`; 3 of them html / md / txt = S3-TEXT) |
| `schema mutation-inputs` | exit 1, **14** findings: 12 `prologPosition` `labelMissing` (xml/svg/docx/xlsx set-snapshot, xml set-doctype), 2 pptx `set-shape-position` `cx`/`cy` |
| `schema mutation-payloads` | exit 1, **10** findings: 2 wav `🔊️resamples` `chunkOrder[2]`, 8 pptx `🥒️.feature` rows (see S3.5: peer integer→string conversion of pptx in progress since 11:18) |

### S3.2 Changes

1. **Load-example verb = the framework verb with the framework-localized label (§20.6).** Contract `🦀️.rs`:
   `SET_ACTIVE_EXAMPLE_ACTION_ID = semio_framework_plugin::app::CATALOGUE_EXAMPLE_ACTION_ID`; `set_active_example_action()` takes
   its label from `app::runtime_verb_label` ("Load example" / "Beispiel laden") instead of a stdio-local literal. Since the
   runtime labels a row from the verb (`authored_row_label` = verb label first), every `description: Some(format!("Load example …"))`
   was dead weight: removed from all 59 owned editor files (112 sites; list `🗑️generated/s3-stdio/load-example-files.txt`).
2. **Every other hand-written emission label in owned stdio files removed (§20.6):** pdf 1.4/1.7 `Set page {page}` (20) and the page
   tool's `Select` / action-id labels, docx `Set DOCX run in …` (8), pptx `Set slide … shape …` (6), xlsx `Set {sheet}!{row},{col}` (3),
   csv/tsv command-id labels, epw `Set {column}`, zip `Set archive comment` / `Rename archive entry`, xml `Replace XML source` /
   `Set node …`, png `Paint PNG region …`, wav `edit-audio` action-id label. Remaining: the contract's two snapshot-edit helpers and
   the png/jpg/tiff/mp4/wav snapshot-edit helpers, which §20.3 rewrites (item 4).
3. **S3-AGNOSTIC findings.** (a) xml base `🔺️diff/🔣️.json`: the three relative `snapshot.json#…` refs are now the absolute
   `https://json.schemas.assets.semio-tech.com/s/stdio/xml/1.0/base/snapshot.json#…` (no other relative `$ref` in stdio).
   (b) `XmlDoctype.prologPosition`: ONE type, schema-first from the Rust serializer (`📸️snapshot/🧭️position`, canonical u64
   decimal TEXT): `string` + `pattern ^(0|[1-9][0-9]*)$` + `maxLength 20`, with `x-semio-ui` label/description en+de, in the xml base
   snapshot schema and the xml base `set-doctype` leaf; svg base `set-doctype` (which wires the xml `XmlDoctype` type) moved from
   `integer`+stepper to the same string type. pptx: a peer converted the snapshot schema to string at 11:18 (diff / set-snapshot leaf
   still `integer`) — left to that peer (S3.5). (c) wav `set-snapshot` leaf schema `WavChunkRef.other.value`: `integer` → `$ref` the
   snapshot schema's `Unsigned64` (the Rust `WavChunkRef::Other(u64)` serializes decimal text; the `🔊️resamples` fixture was right).
4. **§20.3 path-scoped patch leaf — mechanism (contract crate + derive), source complete; per-artifact rollout in progress (S3.2a).**

### S3.2a §20.3 design and mechanism

- **One operation per leaf.** `editing::SnapshotPatch` (`📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs`) is now ONE RFC 6901 pointer
  operation, internally tagged by `operation`: `set {path, value}`, `insert {path, value, index?}`, `remove {path}`,
  `move {from, path, index?}`, `rename {path, key}` (`index` only on an exact inverse restoring an object member's place). It replaces the
  old `{edits: [≤2 primitive edits]}` (no compat). `prepare_snapshot_patch` canonicalizes (appending `-` → the index it lands on, source
  replacement → `set` of the root), `inverse_snapshot_patch` is exact (set→prior value, insert→remove, remove→insert at the former
  object position, move→move back, rename→rename back in place), `snapshot_patch_label` names operation + pointer in en/de, `target()` is the
  pointer. Wire schema `📇️registry/🧬️schema/🔣️.json` `$defs/SnapshotPatch` (+ `SnapshotPointer`, `SnapshotMemberIndex`, labelled).
  Refusals map to the 9-code vocabulary (`path-missing`/`index-out-of-bounds` → `target-missing`, `key-exists` → `duplicate-id`,
  `not-object`/`not-container`/`lossy-conversion`/`schema-invalid`/`invalid-move` → `target-mismatch`, malformed → `invariant`).
- **Input schema = the snapshot sub-schema at the pointer (generic).** `snapshot_schema_location(document, segments, resolve)` walks the
  snapshot schema from the schema alone (properties, `items`/tuple items, `additionalProperties`, `$ref` across documents, every
  `oneOf`/`anyOf`/`allOf` branch that admits the next segment; ambiguous branches → `None`). `snapshot_patch_input_schema_text` builds the
  per-operation leaf payload schema `{patch: {operation: const, path: hidden, value: {$ref: <snapshot $id>#<location>}}}` (rename: a `key`
  text input; remove/move: hidden pointers only), so the time-travel reader renders the control the snapshot declares there.
  `snapshot_patch_input_schema(snapshot_id, patch)` resolves through `registered_input_schema_document`, admits the text with
  `mutation_input_defs`, and interns it (bounded by schema locations); `None` falls back to the leaf's structural payload schema.
- **Per-instance `input_schema` (framework derive, region `🪪️MutationLeaf`, compile-atomic, additive):**
  `#[mutation_leaf(contract = ::protocol, input_schema = path)]` emits `fn input_schema(&self) { path(self) }`; exclusive with `payload = …`.
  `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`.
- **Leaf macro.** `semio_s_artifact_stdio_contract::snapshot_patch_leaf! { leaf, snapshot, mutation, diff, snapshot_schema }` implements
  `MutationKind` (diff via `DiffAlgebra::between`, exact inverse, leaf label, pointer target) and `input_schema_at_path`. Contract root
  re-exports `semio_framework_ui_locale as locale` for the label type.
- **Editors.** `snapshot_edit_patch` publishes the leaf with no description; `snapshot_edit_set_snapshot` lost its description and is
  deleted once its last caller is converted.
- **Language-agnostic evidence.** Fixture `🩹️patch/🧫️fixtures/🔣️.json` gains `patches` (canonical patch + exact inverse per case) and
  `locations` (two schema documents, 11 cases incl. unions, cross-document refs, nullable branches, an ambiguous union → `null`). TS twin
  `🩹️patch/🟦️.ts` (prepare/apply/inverse + `snapshotSchemaLocation`), TS tests with **fast-json-patch** (RFC 6902) and **ajv** oracles: each
  resolved location admits exactly the values the whole-document schema admits at that place. Rust tests (`🩹️patch/🧪️tests/🦀️.rs`):
  canonical patches + inverses, locations, input schema compiles/validates and reads as `/patch` input, wire + label round trip.
- **New patch leaves (47 aggregates) generated schema-first** by ticket input `🧪️s3-stdio-patch-leaves.py` (idempotent, resumable):
  leaf descriptor + payload schema (`{patch: $ref registry SnapshotPatch}`, the aggregate tag `const` for internally tagged
  aggregates, `$defs.Snapshot` → the snapshot schema so the derive embeds and the runtime publishes it), Rust leaf over the macro,
  wire witness, aggregate mount (in-file or crate-root `mutations` module) + variant + `KINDS`, catalog `oneOf` branch per layout,
  TS union member, binary protocol record, text grammar rule, oracle catalog kind + manifest row. Aggregates: stl, las, epw, zip,
  gif 87a/89a, svg base/basic/tiny, mp3, ifc 4, ifc 2x3, bcf, step, tsv, xlsx, pdf 1.7 (used by every pdf 1.4/1.7 editor), docx, xml
  base/valid, jpg baseline, avi, pptx, dwg, dxf, tiff baseline, obj, ply, semio ×19. Conformance-subset editors (step cc1–6, ifc 2x3
  cobie/cv20/sav, xlsx/docx/pptx strict/transitional, pdf subsets) publish the BASE aggregate, so they need no own leaf.
  Plus, by hand (10-03): **bmp** (generator run + leaf `💾️binary`/`📝️text` `CODEC` modules + both registries, png style), **gltf**
  (taxonomy `📸️snapshot/🩹️patch`: descriptor, payload schema, Rust leaf, TS twin `parsePatchSnapshot`, crate-root mount, variant,
  catalog branch, TS union + parser table, witness, oracle catalog kind) and **pdf 1.7** leaf `binary`/`text` identity modules +
  `BINARY_TAG_REGISTRY`/`TEXT_OPCODE_REGISTRY` rows; xml base / svg base `BINARY_TAGS`/`TEXT_OPCODES` rosters.
- **Codec + law arms per aggregate (all 50 aggregates):** diff/inverse routed to the leaf (`MutationKind::diff/inverse`), print/parse
  (`snapshot_patch_text`/`_from_text`, semio `<tag>:<hex>` via new `snapshot_patch_hex`/`_from_hex`), binary encode/decode (the
  patch's canonical JSON after the tag), `TAG_PATCH_SNAPSHOT` consts, `kind()`/`KINDS`, demo cases, every test `kind_of` match and
  one-per-variant sample list, KINDS counts (obj 22, ply 10, gif87a 12, gif89a 21, mp3 5, dwg 3, dxf 19, semio flow 14 / cad 16 /
  document 18). The 8 semio subsets whose tests pin `KINDS[wire_tag]` list `patch-snapshot` LAST (tag order).
- **Whole-snapshot invariants on patches** (`snapshot_patch_leaf! { …, check: f }` → `apply_snapshot_patch_checked`, refusal
  `snapshot-edit.schema-invalid` → `mutation.target-mismatch`): xlsx + docx `validate_authority`, gif 87a/89a raster guard
  (`raster_check`, factored out of the `SetSnapshot` raster arm), dwg version sentinel (`version_sentinel_check`).
- **Retained retirement:** `RetireOwned for SnapshotPatch` (pointer + value tree through framework cursors); zip's retirement cursor
  retires `PatchSnapshot` incrementally (`RetiredOwner::Patch` → `RetirementStep::Child`).
- **Editors (§20.3) — 72 generic snapshot editors + 11 earlier ones** now call `editing::snapshot_edit_patch(event, snapshot, |patch|
  E::PatchSnapshot(..), Some(|snapshot| E::SetSnapshot(..)))` (ticket input `🧪️s3-stdio-convert-editors.py`, idempotent; bcf by hand):
  every path event → ONE patch leaf; only `ReplaceSource` → `SetSnapshot` (S3.4). `snapshot_edit_set_snapshot` is now a private
  helper of `snapshot_edit_patch`; no editor calls it. Field-granular domain leaves kept where they exist (§19.3): jpg baseline
  (`/sofMarker`, `/arithmetic`, `/frame/precision`), tiff baseline (single-value Compression/Photometric, whole BitsPerSample /
  StripOffsets lists); the two-field `SetComponentSampling` / `InsertTileTags` shortcuts were dropped (they masked sibling history).
  bcf table GUID cell → `set /topics/{row}/guid` patch instead of a whole-snapshot `SetSnapshot`.
- **Labels (§20.6):** jpg/tiff baseline `Edit baseline JPEG/TIFF details` descriptions removed; on the coordinator's request (10-03 06:30)
  also the last three `Load example {example_id}` descriptions in the html / md / txt editors (shared contract path) — **0 `Emit
  { description: … }` sites remain in stdio**, `schema mutation-labels` 0 findings.
- **Fixes from the first lib-test run (10-03 06:18):** dxf + ply text parse / binary decode arms and semio value binary decode arm were
  missing; DslOps aggregates (obj, gif 87a/89a; zip already had it) print the patch as a structured block, so their grammar rule is now
  the structured `patch-snapshot … { { patch-fields } }` form (sorted keys, the framework `print_dsl_value` order); pdf `kinds()` 61 → 62
  (+ a `PatchSnapshot` sample), xml base roster 7 → 8, dwg ac1018 catalog (re-exports the ac1024 vocabulary) lists `patch-snapshot`.
- **semio mesh** material texture slots (`baseColorTexture` … `emissiveTexture`, added by a peer 10-03 01:39) labelled en/de.
- **Findings fixed (10-03):** pptx `set-shape-position` + snapshot `PptxTransform.cx`/`cy` `x-semio-ui` labels (en/de); wav
  `🥒️.feature` rows 65/85 `chunkOrder[2].value` `0` → `"0"` (the Rust `WavChunkRef::Other` reads decimal text; the sqlite law
  already rejects a number); oracle manifest `payloadSchema` of 7 patch rows `#SetSnapshot` → `#PatchSnapshot` (generator fixed).
- **Existing patch leaves converted** (json base, csv, png, wav, mp4, jpg document, tiff document): macro + `input_schema`, wire witnesses
  and `🥒️.feature` rows rewritten to the single-op wire (mp4's two-edit row became one `set`), the png/jpg/tiff/mp4/wav third-party oracles
  read the single op, mp4/wav TS twin tests updated, oracle catalog notes reworded.

### S3.3 Verification

| Command (repo root unless noted) | Result |
|---|---|
| `bun test ./✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🧪️tests/🟦️.test.ts` (19:40) | **33 pass / 0 fail** (267 expects; fast-json-patch + ajv oracles, 7 native pilots) |
| `tsc --noEmit --strict … ✏️editing/🟦️.ts ✏️editing/🩹️patch/🟦️.ts` (the package's typecheck command) | **0 errors** |
| `bun test …/🩹️patch/🧪️tests/🟦️.test.ts` (10-03 06:00, + `parseSnapshotPatch` ajv-parity test) | **34 pass / 0 fail** (303 expects) |
| `cargo check -p semio-s-artifact-stdio-contract` (10-03 06:03) | **exit 0** (own warnings fixed: unused `pack`, `RetireOwned`, one qualification) |
| `cargo check` 20 stdio crates `--keep-going -j 2` (06:07) | **exit 101 only from the known DSL-extraction classes** (`From<ValueError>` for `String` in zip/step `🪶️sqlite` + zip iso21320/step cc schema); contract, binary, semio, txt, deflate, xml, pdf, csv, stl, dxf, json, ply, obj, tiff, dwg, las, bmp, tsv **compile**; xlsx/docx/pptx/ifc/bcf blocked behind zip/step |
| `cargo check --lib --tests` 16 crates (06:10–06:15) | **exit 101 only from known classes** (`dsl::json` removed: bmp/csv/las/tsv tests; zip/step); contract, pdf, xml, dwg, dxf, tiff, obj, ply, stl, json lib tests compile (semio/png tests depend on red dev-deps — skipped by `--keep-going`, NOT verified) |
| `cargo test --lib` contract, dwg, dxf, json, obj, pdf, ply, stl, tiff, xml (private target, 06:18–06:35) | contract **89/0**; dwg 94/1, dxf 46/3, json 107/2, obj 57/2, pdf 557/3, ply 59/3, stl 53/2, tiff 117/1, xml 93/1 — own failures (dwg catalog, dxf + ply parse, obj grammar, pdf + xml counts) fixed above; the rest are peer `🪶️sqlite` controlled-admission tests (dxf 2, json 1, obj 1, pdf 2, ply 2, stl 2, tiff 1) and json's snapshot-DSL `grammar_conformance_law` (untouched by S3-STDIO) |
| rerun dwg/dxf/obj/ply/xml (06:37) | **blocked**: `semio-framework-os-kernel` red from a peer's in-flight `IoError` refactor (52 errors in `🚪️io`/`🏪️store`) |
| `bun …/🧪️s3-stdio-audit-patch-leaves.ts` (10-03 06:40; the gate's reader `mutationInputAudit` over every patch leaf incl. uncatalogued) | **56 leaves, 0 findings** (input `/patch`) |

#### Gates after (10-03 06:35, `--under ✏️s/🔌️plugins/🗄️stdio`)

| Gate | Before (10-02 12:45) | After | Remaining |
|---|---|---|---|
| `schema mutation-labels` | 178 | **3** | html / md / txt `Load example` = S3-TEXT's files |
| `schema mutation-inputs` | 14 | **48** → 0 owned | all 48 `leafUncatalogued` (new patch leaves, S3.6 item 1); the 12 `prologPosition`, 2 pptx `cx`/`cy` and the 10 new semio-mesh texture-slot `labelMissing` (peer added the slots 01:39) are fixed; 1661/1661 inputs labelled |
| `schema mutation-payloads` | 10 | **0** | 2047/2047 fixtures + feature rows, 1038/1038 leaves witnessed |
| `schema mutation-editability` | 0 | **0** | 1038/1038 leaves of 87 aggregates editable |

### S3.4 `snapshot_edit_set_snapshot` / `SetSnapshot` survivors

| Survivor | Where | Reason |
|---|---|---|
| `ReplaceSource` → `E::SetSnapshot(next)` | the `replace` arm of `editing::snapshot_edit_patch`, every stdio snapshot editor with a whole-snapshot leaf (81 editors) | the one genuine whole-document intent (paste/replace the source); a root `set` patch would be capped by `SNAPSHOT_PATCH_MAX_BYTES` (1 MiB) while sources reach `SNAPSHOT_EDIT_MAXIMUM_RAW_BYTES` (16 MiB), and its exact inverse (the old document) would be capped too |
| json base / i-json (`None`) | `🧾️json` editors | `JsonMutation` has no whole-snapshot leaf; `ReplaceSource` becomes a root `set` patch (≤ 1 MiB, S3.5) |
| jpg baseline / tiff baseline `SetPixelRegion` → `SetSnapshot` | `✏️editor/🦀️.rs` handlers | a pixel-region tool, not a snapshot-editor event; the raster exceeds the patch item bound and neither aggregate has a pixels leaf (S3.5) |
| `agg_inverse` → `SetSnapshot(base)` | ifc 2x3 / docx / xlsx non-patch kinds | pre-existing whole-snapshot inverses of their own domain leaves (unchanged; patch leaves invert exactly) |

### S3.5 Open items

1. **Third-party oracle + `🥒️.feature` row per new `patch-snapshot` kind** (47 new leaves): catalogs list the kind and every leaf has a
   wire witness, but no Examples row / oracle `"patch-snapshot"` arm exists yet (the existing json/csv/png/wav/mp4/jpg/tiff patch kinds
   have both). Mirror tests (`mp3` oracle `KINDS`, pdf/docx/pptx oracle `KINDS`) stay green because they compare subsets; adding rows
   needs a per-oracle generic JSON-pointer apply over each oracle's own wire projection.
2. **Patch item bound vs. large inverses:** a `remove` of a subtree > 1 MiB has no exact inverse patch (`inverse_snapshot_patch`
   refuses → empty inverse); likewise a JSON `ReplaceSource` > 1 MiB. Needs either a streamed patch value or a whole-snapshot fallback
   inverse — a contract decision.
3. **Snapshot-schema labels:** time-travel inputs at deep pointers fall back to the structural payload schema wherever the snapshot
   sub-schema lacks `x-semio-ui` labels (admission via `mutation_input_defs` fails) — many stdio snapshot schemas are unlabelled.
4. **pdf / xml base / svg base TS unions** have no `SetSnapshot` member either; `PatchSnapshot` not added there (no leaf `🟦️.ts`).
5. **pptx**: `set-snapshot`/diff leaf `prologPosition` still `integer` while the snapshot is `string` (peer conversion since 10-02 11:18);
   GraphQL `Int!` for `prologPosition` in xml/pptx `🔗️.graphql` not regenerated.
6. **Cargo verification incomplete:** epw, step, gif, avi, jpg, mp4, wav, zip, mp3, svg, gltf (+ dependents xlsx, docx, pptx, ifc, bcf,
   and semio/png tests) wait on S3-INFRA's `ValueError`/`dsl::json` sweep. Payload laws (`semio_payload_law_*`) run with the lib tests.

### S3.6 Coordinator actions

1. Central `schema generate` (catalog): the 49 new `…/mutation/patch-snapshot/schema.json` leaves are `leafUncatalogued` in
   `schema mutation-inputs` until the catalog lists them (47 of the 49 current input findings).
2. `describe` / `materialize` for every stdio plugin: new variants, catalog branches, the "Load example" verb label and the removed
   descriptions change every stdio descriptor; then re-activation of the stdio dev lanes.
3. After S3-INFRA reports green: S3-STDIO reruns `cargo test --lib` for the 11 red crates + dependents and the four gates.

## Session 4 — 2026-10-04

S4-STDIO (Opus executor), successor of S3-STDIO. Scratch: `🗑️generated/s4-stdio/`. Private test target `⚡️cache/cargo/target-nde-s4-stdio`.

### S4.0 Status log (newest first)

- 20:55 RESUME (python/bun only, no cargo; kernel red after a peer's 18:27 stash/pop — every file of mine re-verified
  intact first: all ticket scripts `--check` 0, oracle/schema/fixture spot checks present). F10 finished:
  - **xml base**: oracle arm (`xnode_to_wire` + `patch-snapshot` apply/invert) + rows (`/doc/root/children/0/…/attrs/0/value`
    → Heading3) + catalog kind/manifest; old pptx entry dropped from `🧪️s4-stdio-patch-rows.py` (pptx rows come from
    `🧪️s4-stdio-pptx-rows.py`).
  - **shared OOXML reading helper** (`🔮️oracles/📃️document/🦀️.rs` `ooxml` region `🔖️PackageReading`): `XmlPartsShape {Tree,
    Retained}`, `xml_parts_reading`, `patched_xml_parts` — logical XML parts in archive order, wire `XmlNode` trees (text +
    entity references coalesced as the subject keeps them) or the retained arena (post-order nodes, pre-order attributes),
    changed parts re-serialized between their own prologue/epilogue text, container rebuilt.
  - **xlsx base**: oracle arm via `patched_xml_parts(Tree)` (inverse = original round trip) + rows
    (`/xmlParts/5/…sharedStrings si 0 text` → "Kennung (gepatcht)").
  - **docx base**: python-docx (repo `.venv`) wrote `🧾️readme-afters/🩹️patch-snapshot/➡️after.docx` from the row
    (`/xmlParts/0/document/attributes/1/value` → Heading2, the first pStyle) and re-read it against the patched retained reading
    before committing (`🧪️s4-stdio-docx-patch-after.py`, byte-identical rerun); fixture manifest
    `patch-snapshot-readme-applied`; zip+quick-xml oracle arm via `patched_xml_parts(Retained)`; `KINDS` 14 + unit-test counts.
  - **semio base**: catalog vector `🩹️patch-snapshot/✏️edits` (before = `✉️replaces` before, after = host `patched_snapshot`, exact
    inverse checked) + fixture manifest + `mutate-/inverse-patch-snapshot` scenario pair + adapter handlers both roles + oracle
    `route` arm (`🧪️s4-stdio-semio-base-patch.py`).
  - **semio mesh**: general pointer arm + exact inverse in the TS three oracle (replaces the peer's insert-only arm, keeps the
    DeleteTexture inverse working); payload + rows (`/materials/0/roughness` → 0.5); `tsc --strict` exit 0;
    `🧪️s4-stdio-mesh-patch-probe.ts` 7 passed / 0 failed.
  - **gltf ♾️any check**: no naming mismatch — the row resolves to leaf `📸️snapshot/🩹️patch` (semanticKind `patch-snapshot`,
    variant `PatchSnapshot`). The contract still reports "test catalog claims set-snapshot/patch-snapshot, which no manifest
    owns" (pre-existing): owning them in the ♾️any manifest trips `wildcard-subset-owner` (8 real subsets) → owner decision
    (typed compound or subset policy), left unchanged.
  - **all Python oracle arms run** (`🧪️s4-stdio-python-oracle-arms-run.py`, host Context, oracle role, no runner): 34 passed / 0
    failed (17 cases × mutate+inverse). It caught a bug in my S3/S4 arming: table/text/object kept `KINDS` and `TAG_OF_KIND`, and
    only `KINDS` had been extended → `tagged()` refused `PatchSnapshot`; fixed in `🧪️s4-stdio-semio-python-arms.py` and the 3 oracles.
  - **contract gate** (`contract --owner 🗄️stdio`, static): fixed what my changes caused — bmp duplicate scenario ids (removed
    the small-document paint rows), bmp catalog (kinds/manifest/fixture links to the 4-kind vocabulary), bmp vectors rewritten
    to `{schema, bytes}` + new `patch-snapshot-applied` + manifests for the history-edit paints (`🧪️s4-stdio-bmp-vectors.py`,
    Pillow decodes every reading); 10 stale fixture digests refreshed after legitimate schema evolutions (tiff storage/kind,
    mesh texture slots, wav) via `🧪️s4-stdio-fixture-digests.py`. Remaining stdio breaches are pre-existing or generated-artifact
    staleness: "production dispatch" (runtime mutation inventory — coordinator regeneration), "no fixture-backed vector" for
    most `patch-snapshot` kinds, tiff `paint-region` / xlsx `insert-cell` without scenarios (no tiled TIFF / vacancy fixture).
  - **composition ledger** (`🌎️hub/…/🔮️oracles/🧫️fixtures/🧩️composition/🔣️.json`, relayed from S4-FLOWCAD): the
    `🏛️mutate-semio-model` caller row matches revision 202c4b7b5b1 (10-02 17:04); the only change since is the fleet-wide
    `semio_model_mutation_inverse(…).expect(…)` sweep (inverse became fallible) — neither FLOWCAD nor S4-STDIO. The ledger is
    stale wholesale: 218 of 265 caller rows (97 of 126 stdio), so `bun test …/🧩️composition/🟦️.ts -t "caller laws"` fails on its
    first row (writer). Read-only audit: `🧪️s4-stdio-composition-ledger-audit.py [--trace <caller>]`. Refreshing the digests is
    the composition owner's action.
  - **Gates**: `schema mutation-payloads --under ✏️s/🔌️plugins/🗄️stdio` exit 0, findings `{}` (1031 leaves witnessed, 1446 rows,
    2081 clean); host Python parity 16/0. **Every Rust edit (xml/xlsx/docx/semio base oracles + document helper + semio base
    adapter) is uncompiled — OWED (cargo freeze)**.
- 14:20 PARKED (coordinator usage limit). Exact state and next steps:
  - **Payload gate GREEN**: `bun ./📜️script.ts schema mutation-payloads --json --under "✏️s/🔌️plugins/🗄️stdio"` (cwd test module)
    exit 0, findings `{}` — 1031 leaves, 1031 witnessed, 1442 rows, 2072 clean (was 72 at 08:46). Fixes, by group:
    - 5 layout: ifc2x3 cobie/cv20/sav `📋️rename-file` + xlsx strict/transitional `🧮️widens` fixtures retagged to `{"SetSnapshot": {...}}`.
    - bmp (15): aggregate `🔣️.json` branches → leaf `$id` refs (dropped broken `$defs`); `🪟️mutate-bmp-v3` feature + `🔮️oracles/🦀️.rs`
      rewritten to the byte-authoritative 4-kind vocabulary (set/patch-snapshot, paint-indexed/direct-region; oracle recomputes the
      FNV-1a revision; inverse = original); 5 empty orphan leaf dirs removed; text grammar `op` gained `patch-snapshot`.
    - png (20): 196 dead files + uncompiled `🧪️tests/🛡️mutation-regressions/🦀️.rs` deleted (coordinator GO; list in
      `🗑️generated/s4-stdio/png-dead-files.txt` — 14 leaf dirs ✏️replace-text-chunk 🌈️change-chromaticities 🎨️replace-palette
      👁️change-transparency 📏️change-physical-dims 📐️change-header 📤️remove-unknown-chunk 📥️insert-text-chunk 📦️insert-unknown-chunk
      🔲️replace-pixels 🕰️change-timestamp 🖌️change-srgb-intent 🖼️change-background 🗑️remove-text-chunk, 9 files each, + their
      `🧫️fixtures/🧬️mutations/<leaf>` 5 files each); `🔀️mutate-png-1-2` feature/oracle/adapter rewritten (5 kinds; subject identity
      law now `carrier_is_exact`); new fixture `🧫️fixtures/🟥️rgba8-swatch/🖼️.png` (stdlib-authored 2x2 RGBA8); catalog/generator
      script+codec/contract fixture/exports (`T/🧪️s4-stdio-mutation-exports.py`, also bmp text json) fixed; F19 done (codec match
      comments → docstring).
    - tiff document (8): leaf schemas replace-tag/insert-ifd → snapshot `$defs` refs; feature rows + oracle on the `{storage}` wire;
      `🔲️replace-pixels` dead leaf (14 files, `🗑️generated/s4-stdio/tiff-dead-files.txt`) deleted (GO); catalog/generator/contract fixed;
      `paint-region` wire witness added (no case row: tiled-only + runtime revision — OWED). Codec match comments → docstring.
    - xlsx base: `➕️insert-cell` wire witness. docx strict/transitional: witnesses → retained OPC/XML arena shapes.
    - semio graph (8): create-edge schema `source_port`/`target_port` (wire is snake_case), floats → Binary64Transport anyOf in
      create-node/aggregate/snapshot schemas, add-node-port port gains category/properties, set-snapshot witness width/height/properties.
    - pptx (16): case rewritten to the revision-bound address vocabulary — rows derived by `T/🧪️s4-stdio-pptx-rows.py` (`--check` 0),
      oracle apply = XML-tree edits on its own zip/quick-xml model + `package_of` for set-snapshot + `{xmlParts}` reading for
      patch-snapshot; `KINDS` 9 (unit test updated).
  - **AUDIT-TOOLS F10 (D4)**: done — xml base (oracle arm + row pending, see next steps), semio presentation/drawing/image
    (`T/🧪️s4-stdio-semio-d4-cases.py`, `--check` 0; arms run directly in Python: 3 passed / 0 failed). svg base already had row+arm.
  - **COMPOSITION GREEN stdio**: NOT reached — `semio-hub-stdio` wasip2 check was blocked (flock queue, then fingerprint ENOENT in the
    shared build dir), my cargo stopped at RULE 44. OWED.
  - **Every Rust edit of this session is uncompiled** (bmp/png/tiff/xml/pptx oracles + adapters): OWED (rule 44).
  - **Next steps**, in order:
    1. Add the xml base row: `patch("set", "/doc/root/children/0/children/0/children/0/children/0/attrs/0/value", value="Heading3")`
       to `🧪️s4-stdio-patch-rows.py` for outlines "Apply <id> to the real document"/"Undoing <id> restores the document" (oracle arm is
       already in `📰️xml/…/🧱️base/🔮️oracles/🦀️.rs`); drop that script's stale pptx entry (the pptx rows script owns pptx now).
    2. D4 remaining: xlsx base (outline 2 + inverse; needs a shared `{xmlParts}` reading helper in `🔮️oracles/📃️document`), docx base
       (needs a committed `🧾️readme-afters/🩹️patch-snapshot/➡️after.docx` + fixture manifest; python-docx not installed), semio ✉️base
       (Rust envelope oracle), semio 🔺️mesh (TS three oracle); gltf `♾️any` naming check.
    3. When CARGO OPEN: gated `cargo check --lib --tests` for bmp png tiff xml pptx docx xlsx semio (+ test-oracle crates with
       `--features oracles`), wasip2 check of `semio-hub-stdio` → send COMPOSITION GREEN stdio; then owed lib TEST batches.
    4. Re-run `schema mutation-payloads`, `verify mutation-outcome-law`, and the test platform `test parity exhaustive --case` for
       the rewritten bmp/png/tiff/pptx cases and every D4 row.
  - Coordinator actions: central `schema generate` (graph schemas, tiff leaf schemas), `describe`/materialize of the stdio plugins.

- 08:45 RULE 43 (checks only): owed lib TEST batches paused — batch 1 (epw step gif avi jpg mp4 mp3 svg gltf) ran to compile errors
  in TEST files only (peer value refactor fallout: `protocol::ToValue`/`dsl::DslValue` now private, `MutationDiff::inverse` returns
  `Self`, `native_decoding` moved to `semio_framework_value`, `WavFmt` import) — all fixed (epw/mp3/jpg sqlite tests, mp4 codec test,
  wav editor test, 13 files carrying the S3 patch sample `dsl::DslValue`); `cargo check --lib --tests` of the 9 crates: only those
  errors, now fixed (re-check pending). zip + wav production `🪶️sqlite/🚦️native` reds belong to their owner (edited 07:16). Lib TEST
  runs of all stdio crates: **OWED (rule 43)**.
  D4: Python test host gains `patched_snapshot` + `snapshot_patch_inverse` (region `🩹️SnapshotPatch`, RFC 6901 twin of the Rust
  helper; ticket input `🧪️s4-stdio-python-patched-snapshot.py` **16 passed / 0 failed** on python 3.9 and 3.14); 14 semio Python oracles
  armed (`🧪️s4-stdio-semio-python-arms.py`, idempotent): uniform set-snapshot vocabularies (animation, video, cad, document, audio) and
  flow/model/value restore through `set-snapshot`; externally tagged ones (table, object, text, graph, brep, kit) restore through the host's
  exact inverse patch; svg base oracle reads `/doc` through its own quick-xml tree. Rows added: 14 semio + svg base pending row.
  Payload gate: 74 `opaque` (my splice branch) fixed → splice `value` `anyOf [{array, items: true}, {string}, {object,
  additionalProperties: true}]`; remaining 77 (bmp/png/tiff/pptx/graph/docx/xlsx/ifc 2x3 peer-model drift) assigned to me 08:30.
- 08:00 Outcome-law routing (S4-GATES): png/bmp diff codes → `mutation.apply.invalid-bytes`; pptx canonical-address fatal →
  `MutationOutcome::refuse(OutcomeCode::TargetMismatch, …)`; `bun ./📜️script.ts verify mutation-outcome-law` **passed**. Owed batch 1
  diagnosis: zsh does not word-split `$P`, so the earlier `cargo test … $P` calls ran the WHOLE ✏️s workspace with a junk filter (that is
  why wfc-engine / jack-lsp compiled); fixed with `${=P}`; rerun gated by rule 42 (cargo < 8). D4 continued: bcf (reuses the committed
  `🔢️set-version-applied` pair), glTF (oracle reads `/document` as the GLB JSON chunk; an omitted default member is added; reuses
  `🪞️change-material-double-sided`), bmp (pointer into the file's own octets, re-parsed by the `image` decoder; small indexed outline),
  jpg baseline (`/sofMarker`, `/frame/precision`, `/arithmetic` → T.81 axes), tiff baseline (IFD 0 entry order read from the bytes →
  `/ifds/0/entries/{i}/values/value` → Baseline axis) = **35 of 56** subsets with row + arm; left: svg base, xml base, xlsx, docx, 19 semio.
- 06:45 RESUMED after the usage cut (~04:15). State re-established from disk: the staged D4 scripts had already been APPLIED at 04:10
  (FREEZE LIFTED): `🧪️s4-stdio-oracle-arms.py --check` 0 of 16 pending, `🧪️s4-stdio-patch-rows.py --check` 0 pending, `🧪️s4-stdio-inverse-rows.py
  --check` 0 of 56 pending. Owed batch 1 (11 crates) was stopped by zip compile errors (peer value refactor: `close_step` now returns
  `ValueError`) — fixed in `🎒️zip/…/✏️editor/📬️preparation/🦀️.rs` (3 impls, 5 error sites), zip editor test (`MutationDiff::inverse` now
  returns `Self`), zip sqlite test import (`semio_framework_value::ToValue`, coordinator-assigned). Batch 1b (10 crates without zip)
  aborted on `semio-s-plugin-wfc-engine (lib test)` (peer `semio_framework_os_kernel::json` removal in wfc tests, not stdio).
- 03:52 ACTIVATION FREEZE (rule 40): no source saves; owed stdio lib batch 1 (11 crates) running; D4 staged in `🗑️generated/s4-stdio/`.
- 03:48 D3 VERIFIED (TREE GREEN 03:21): `cargo test --manifest-path ✏️s/Cargo.toml --lib -p semio-s-artifact-stdio-contract` (private target)
  **100 passed / 2 failed** — both failures were test bugs (serde_json re-sorted the fixture's object members → parity compared against a
  sorted base; a continued-part test spliced at offset 1 of an empty array), fixed (order-preserving fixture read via pack-json, the
  RFC 6902 replay now also asserts agreement after every part); rerun `-- editing::patch:: --skip sixteen` **17 passed / 0 failed**. The
  laws `an_inverse_at_the_patch_budget_plus_or_minus_one_byte_restores_exactly` and `sixteen_mebibyte_prior_values_undo_exactly_within_the_part_bound`
  (16 MiB text, 16 MiB octets, ≥ 16 MiB of typed rows; ≤ 128 parts of ≤ 1 MiB; exact restore) PASS in the full run (279 s debug). No
  warnings left in `🩹️patch/🦀️.rs` / `✏️editing/🦀️.rs` (40 + 8 peer-refactor `unnecessary qualification` + unused `pack` fixed).
- 03:40 D4 shared piece: repo test host `⚖️law/🦀️.rs` region `🩹️SnapshotPatch` — `law::patched_snapshot(snapshot, patch)` applies ONE
  pointer operation (set/insert/remove/move/rename/splice) to a reference's own snapshot reading, written from RFC 6901 alone (no subject
  code); unit law `a_patch_snapshot_row_is_its_one_pointer_operation_on_the_reference_reading`; `cargo test -p semio-repo-test-host --lib`
  **19 passed / 0 failed**. Markup family oracle (`🔮️oracles/📰markup`) gains `node_to_wire` + `patched_markup` (doctype kept raw, pointers
  into it refused).
- 03:30–03:50 D4 rows + oracle arms (source, unrun — cases need `test parity exhaustive`): stl, las, epw, csv, tsv, zip, gif 87a, gif 89a
  (new `snapshot_to_json` wire), svg tiny, svg basic, xml valid, avi, dxf (header-var reading), obj, ply = **15 of 50**; rows written by ticket
  input `🧪️s4-stdio-patch-rows.py` (idempotent, `--check`), adapters' own inverse rules extended where a case computes the oracle inverse
  (csv, tsv, epw, obj, ply).
- 03:20 D3 source complete (coordinator approved the design 03:15): contract `🩹️patch` gains the `splice` operation
  (`{path, offset, remove, value, continued?}` over array items / octets / UTF-8 bytes of text / object members by position, common prefix
  replaced in place so struct members and items never vanish transiently) and `continued` (a non-final part of one multi-part inverse; the
  leaf's whole-snapshot `check` runs on the run's final part). `inverse_snapshot_patches[_within]` plans the EXACT inverse in ≤ 128 parts of
  ≤ 1 MiB (shell = containers emptied / struct-like objects keep every key / oversized maps keep a prefix, placed by a parent splice — a root
  `set` only on a root kind change — then refilled by greedy splices); the leaf macro's `inverse` returns the parts (errors instead of the old
  silent empty inverse) and its `diff` refuses a patch without an exact inverse (`snapshot-edit.inverse-limit` → `target-mismatch`).
  Schema-first: registry `$defs/SnapshotPatch` splice branch + `SnapshotSpliceUnits`; `x-semio-inverse-rows: {bounded: 128}` on all 56
  patch-snapshot leaf schemas (ticket input `🧪️s4-stdio-inverse-rows.py`, idempotent, `--check` 0 pending); `patch-splice` rule in the 4
  structured mutation grammars (obj, gif 87a/89a, zip). TS twin mirrors splice + planner (compact canonical sizing, also fixing the twin's
  pretty-printed size check). Language-agnostic fixture `chunkedInverses` (8 cases, 80 parts, budget 160; generated by ticket input
  `🧪️s4-stdio-chunked-inverse-fixture.ts`, `--check`): TS `bun test …/🩹️patch/🧪️tests/🟦️.test.ts` **43 pass / 0 fail (658 expects)** incl. the
  fast-json-patch replay of every part; strict tsc **0 errors**. Rust laws written (twin parity + `json_patch` replay, 1 MiB − 1/= /+ 1,
  16 MiB text/octets/typed rows, part bound refusal, continued check deferral, canonical length) — cargo blocked by the peer-red plugin crate
  (`🔌️plugin/🦀️.rs:27457/27637` E0061, owners fixing).
- 02:20 started: read fleet rules 1–38, AGENTS.md, design §11/§20.3/§20.6, plan Session 4 roster, `📓️s4-resume.md` §0/§0.1/§2.3/§2.5/§4 D3–D4/§7 S3-STDIO, this report.
  Rule 34 repair check: `git diff HEAD --stat -- ✏️s/🔌️plugins/🗄️stdio` = 4 792 files (HEAD is 10-02 17:04, so it holds all S3-STDIO work);
  mtime census of the 25 842 tracked stdio files since S3-STDIO's last report (10-03 06:45): every change between 06:45 and 12:30 is a
  peer's (Codex sqlite-snapshot conversion: semio ×19 `🪶️sqlite`/`🛬️native`/`🛫️native` 08:00, gltf/csv/tsv/binary/deflate/png sqlite
  09:22–11:00, step 11:32–12:28; S3-GRAPHS semio graph `drag-nodes` 11:48–12:07); contract files touched 10-03 15:32–23:53 + 10-04 01:04
  (raster/bytes/details/part21/`🩹️patch` imports) are peer value/DSL refactor waves. No half-finished S3-STDIO edit found.
