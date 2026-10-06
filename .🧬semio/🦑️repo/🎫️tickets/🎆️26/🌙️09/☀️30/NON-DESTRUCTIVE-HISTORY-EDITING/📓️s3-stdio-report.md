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

## Session 5 — 2026-10-05

S5-TEXT-STDIO (one executor for S4-TEXT + S4-STDIO; the text half is `📓️w3-t2-text-report.md` § Session 5). Scratch
`🗑️generated/s5-text-stdio/`. Ticket inputs (keep at close): `🧪️s5-text-stdio-input-ui.py` (engine: `--check` default, explicit
file list, fails closed on an empty root / a row without an input / an input without a row / a refused annotation),
`🧪️s5-text-stdio-input-ui-table.py` (the reviewed table: one hand-authored row per input MEANING), `🧪️s5-text-stdio-input-ui.files.txt`,
`🧪️s5-text-stdio-input-ui-check.ts` (pre-flight with the framework's own reader). Per-input table: `📓️s5-text-stdio-input-table.md`.

### S5.0 Status log (newest first)

- 09:36 RESUME after the 07:45 cut. Repair-first: the six files of the interrupted wave were written atomically at 07:31:02 and are
  complete; the verifying check had run across the cut (07:43:35–07:47:30, `check-semio-tests-1.txt`): **`cargo check
  --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-semio --lib --tests --all-features` exit 0** (lib 393 warnings, lib test
  452 = type-check proof) → the 14 lib-test errors are gone. Re-check on the current tree (73 stdio files changed after 07:48, a
  Codex peer's sqlite-native wave) is running cold after the prune (`check-semio-tests-2.txt`).
- 07:31 stdio-semio LIB-TEST repaired (14 errors in 5 stale tests, all model drift, none a production red):
  - image ⇄ tiff: `TiffIfd` has no `pixels`, `TiffTag` no `kind`, `TiffSnapshot` no `pixels`, `TiffValues::Ascii` is bytes. The import
    fixture is now a real one-strip RGB page (`storage.chunks`), the export law projects the decoded page with
    `decode_tiff_page_rgba` and compares all four samples.
  - **Production defect found by rewriting that law**: `SemioImageToTiff` wrote a 4-sample page without `ExtraSamples` (338), which
    the tiff crate's own projector refuses ("four-sample display projection requires one unassociated alpha ExtraSample") — an
    exported image could not be re-imported. Fixed: the exporter plants `ExtraSamples = [2]` (unassociated alpha); stale module doc
    corrected.
  - presentation ⇄ pptx: `PptxSnapshot::from_parts` takes two arguments and `presentation` is a projection method — the fixtures
    build through the crate's own `build_minimal_pptx`, the export law reads `pptx.presentation()`.
- 02:14 CUT. Landed and checked before it (each under one `stdio` hold, `cargo check … --lib` exit 0 with warnings):
  b1 las, zip, mp4, tsv, txt, stl (37 files; 01:50–01:52) · b2 csv, mp3, png, wav, tiff, ply (25; 01:56–01:57) · b3 md, xml, json,
  obj, jpg, svg (82 + codemod 11 sites; 02:08–02:10) · b4 xlsx, pptx, docx, step, binary, deflate (68 + codemod 9 sites;
  02:13–02:14). Text crates (`jack`, `rewriting`, `writer`, `vcs` + the stdio closure) `--lib` exit 0 at 02:40.

### S5.1 P1 census (before, 01:05) — `🗑️generated/s5-text-stdio/census.py`

Unit: top-level payload properties of every leaf schema on disk (`…/🧬️mutations/<leaf…>/🧬️schema/🔣️.json`, fixtures/tests excluded,
`mutation` discriminator excluded). "Bare" = no `x-semio-ui` on the property: it read only through type inference and the label
glossary — no description, no step, no unit, no role.

| tree | leaves | explicit | bare | no input |
|---|---:|---:|---:|---:|
| stdio | 1031 | 890 | **821** | 45 |
| trinity | 16 | 11 | 17 | — |
| writer | 10 | 9 | 10 | — |
| vcs | 6 | 3 | 3 | — |

Bare by stdio crate: semio 335, pdf 166, svg 50, step 44, xlsx 29, json 25, obj 21, zip 20, xml 19, jpg 17, pptx 16, las 13, ply 11,
md 11, mp4 9, tiff 7, stl 7, txt 7, wav 5, tsv 5, png 2, csv 1, mp3 1 (the audit's 644 excluded the 190 `$ref` inputs).
What was missing, by kind (top 10 of 723 distinct leaf/input rows): `index` integer ≥ 0 ×130 (a different thing in every leaf:
page, point, track, block, row, keyframe …), string `id` ×55, `name` ×38, `key` ×26, child-index `path` ×19, whole `snapshot` ×62,
`value` ×14, text ×11, `descriptorOrdinal` ×12, script ×10. The full per-crate table (leaf, input pointer, JSON type, what was
missing, what is declared) is `📓️s5-text-stdio-input-table.md`.

### S5.2 P1 method — a table of meanings, applied surgically

- One constructor per judgement (`AT`/`IDX` which item an index addresses, `POS` where an insert lands, `ENT` an entity named by id
  → reference chip with `role: target` + `ref.kind`, `KEY` a key the mutation may create → text in group `target`, `TXT`/`MULTI`/
  `FLAG`/`INT`/`NUM`/`VEC` the value with its unit/step/precision/soft bounds/snaps, `REC` a structured value whose members carry
  their own declarations, `ADDR` a structured address, `BLOB` an opaque member beside real parameters, `DISC` a fixed discriminator).
  Examples of the distinctions the table makes: pdf `set-page-rotation.rotation` = dial, `°`, step 90, snaps 0/90/180/270; pdf page
  `width`/`height` = stepper in `pt`, snaps at A5/A4/Letter/Legal/A3/A2; b-rep `tol` = log slider 1e-9…1e-1 with decade snaps;
  mp4/semio-video `width`/`height` = `px` with 640…3840 / 360…2160 snaps; mesh `new_metallic`/`new_roughness` = slider 0–1 step 0.01;
  camera `zoom` = log slider 0.1–8 with snaps; graph node `x`/`y`/`width`/`height` snap to the window's `gridFactor`;
  png/bmp/tiff colour channels = sliders 0–255; `ExtraSamples`-style code integers = steppers with the legal values in the description.
- **Design §22.20 (01:50)**: a leaf whose whole VALUE is opaque is declared WITHDRAW-ONLY in its descriptor (`"editable": false`),
  never hidden. The table's `W` rows name them with the reason; a leaf that takes no parameter at all is withdraw-only by shape.
  `hidden` stays only for an opaque member beside real parameters (wav `patch-data.data`).
- Earlier declarations that lacked a step are AMENDED (`A` rows: 372 inputs, gltf 166, gif 43, ifc 25, …) so every interactive
  number declares `step` or snaps.
- Engine guarantees: span-surgical insert (every other byte kept; the parsed document must equal the original plus exactly the
  written members), `InputUi` meta-schema validation with third-party `jsonschema`, the reader's widget/shape/bounds rules, and the
  pre-flight `bun 🧪️s5-text-stdio-input-ui-check.ts <bundle>` = the framework reader over every patched leaf before the write
  (last run: 746 files, 0 findings added).
- Findings fixed on the way (semio graph subset, were gate reds): `remove-node-property` `/node_id` reference on an object and
  `/key` `role: target` without `ref.kind`; `create-edge`/`create-node`/`add-node-port` unlabelled `source_port`, `target_port`,
  `ports`, port `kind` options, `category`.

### S5.3 State at 10:45 (what is on disk, what was run)

| Family (one `stdio` hold each) | On disk | Verifying run |
|---|---|---|
| b1 las zip mp4 tsv txt stl | declarations (37 files) | `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-{las,zip,mp4,tsv,txt,stl} --lib` **exit 0** 01:52 |
| b2 csv mp3 png wav tiff ply | declarations (25) | same form, **exit 0** 01:57 |
| b3 md xml json obj jpg svg | declarations (82) + tool-mismatch codemod 11 sites | **exit 0** 02:10 |
| b4 xlsx pptx docx step binary deflate | declarations (68) + codemod 9 sites | **exit 0** 02:14 |
| stdio-semio tests | 5 test files + image→tiff exporter (`ExtraSamples`) | `… -p semio-s-artifact-stdio-semio --lib --tests --all-features` **exit 0** 07:47 and again on the current tree **exit 0** 09:52 (lib 393 / lib-test 452 warnings) |
| b5 avi bmp dwg dxf epw gif | declarations + amendments (58 files incl. 7 `editable: false` descriptors) + codemod 8 sites | **NO VERDICT** — gate v6 closed 15 min (09:52–10:07), second run sat 26 min childless behind shared-dir locks and was stopped by me (exit 143), third gate closed 10:41. Pre-flight with the framework reader: 0 findings added. **OWED**: `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-avi -p semio-s-artifact-stdio-bmp -p semio-s-artifact-stdio-dwg -p semio-s-artifact-stdio-dxf -p semio-s-artifact-stdio-epw -p semio-s-artifact-stdio-gif --lib --message-format=short` |

Totals of the plan (`python3 T/🧪️s5-text-stdio-input-ui.py`, 775 table rows → 1226 inputs in 862 files, 0 problems): **526 inputs
declared on disk**, 493 to declare + 207 to amend still STAGED (semio 343, pdf 166, gltf 166, ifc 23, html 2); withdraw-only
leaves **8 marked of 115** (avi, bmp, dwg, dxf, epw, gif ×2, rewriting `edit-before-fixture`), 106 staged, 1 blocked (gltf
`default-scene/unbind` also needs `payload = Apply` dropped from its `#[mutation_leaf]` attribute, same wave); tool-mismatch codemod
**28 of 72 sites** rewritten (`python3 T/🧪️s5-gates-tool-mismatch.py --root ✏️s/🔌️plugins/🗄️stdio` → 44 left in 39 files).

Gates on the disk state (bun, repo root, 10:42): `schema mutation-inputs --census --under ✏️s/🔌️plugins/🗄️stdio` = 2011 findings
(was 4705 at 01:33 under the same tightened gate): `numericUndeclared` 2489 → **1882**, `labelInferred` 2119 → counted (GATES
narrowed the rule), `inputless` 27, 7 leaves withdraw-only, `labelMissing` 12 / `uiInvalid` 1 / `widgetIncompatible` 1 /
`optionLabelMissing` 2 (all semio graph, cleared by the staged semio family), catalogue 86. `python3 T/🧪️s4-stdio-fixture-digests.py
--check` = 0 stale. **Projection when the staged families land** (computed from the gate's JSON against the plan): `inputless` 0,
label/widget findings 0, `numericUndeclared` 1882 → **911, none top-level**.

### S5.4 The remainder of clause 5 in stdio — 632 nested numeric members

After the plan, every top-level input is declared; what stays `numericUndeclared` are the numeric MEMBERS of structured values
(`REC` inputs): 911 findings = **632 distinct schema nodes in 176 files** (469 integers, 162 numbers) — e.g. semio `SemioPoint2.x/y`
and `Point3` (70 + 70 + 39 findings), pdf `PdfDate` parts and `ObjRef.num/gen`, zip entry fields, mp4 track boxes, array items of
address paths. The node list with label / widget / unit / type / bounds is `🗑️generated/s5-text-stdio/nested-numeric-nodes.json`
(keep until that layer is authored). Next table layer = `D` rows on those shared definitions (integers: step 1 + precision 0 where
the member is a count, index, size or code; coordinates and factors by their unit). Not started.

### S5.5 Design §22.20 in stdio (withdraw-only)

- 78 whole-document leaves (`set-snapshot` ×77, gltf `snapshot/set`), 9 opaque-value leaves (las `set-vlr-data`, zip
  `set-entry-data` ×2, jpg `replace-pixels`, wav `set-data`, pdf `set-document-id`, semio video `set-sample-data`, semio mesh
  `replace-primitive-geometry`, bcf `set-viewpoint-snapshot`), 27 parameterless leaves (remove-/clear-/collapse-/strip-…;
  marked by shape, no table row). All of them carry `#[mutation_leaf(contract = ::protocol)]` only, except gltf
  `default-scene/unbind`. The 56 `patch-snapshot` leaves stay editable (`input_schema = Self::input_schema_at_path`).
- The semio base `apply-<subset>` forwarding leaves are NOT parameterless (their `mutation` member is the nested subset
  mutation, not the aggregate tag) and stay untouched.
- Found by reading before marking, reported 10:03, fixed by S5-GATES 10:09: the derive's withdraw-only arm emitted only
  `input_schema() -> None`, so the derived payload law would have failed every marked leaf with a fixture ("declares no input
  schema yet rebuilds from its payload"); it now also emits a refusing `with_input_value`.

### S5.6 Owed, in order (each = one `stdio` hold: `zsh T/🔐️lock.sh acquire stdio S5-TEXT-STDIO`, then
`zsh T/🗑️generated/s5-text-stdio/batch.sh <name> <artifacts> [extra -p …]`, which applies the table to those artifacts, runs the
codemod on their roots, passes gate v6 and checks their libs; release on exit 0)

1. b5 check (above). 2. `batch.sh b6 gltf,html,ifc,bcf` — first Edit gltf `🏠️default-scene/✂️unbind/🦀️.rs:27` to
`#[mutation_leaf(contract = ::protocol)]`. 3. `batch.sh b7 pdf` (also settles the 07:33 `apply_validated_snapshot_patch` E0603:
the fn was private at HEAD, the Codex wave made it `pub` at 08:12 — not standing in source). 4. `python3
T/🧪️s5-gates-tool-mismatch.py --root ✏️s/🔌️plugins/🗄️stdio/📇️registry --apply`, then `batch.sh b8 semio "-p
semio-s-artifact-stdio-contract"`. 5. markers + codemod of the families landed before the marker existed: `batch.sh m1
las,zip,mp4,tsv,txt,stl`, `m2 csv,mp3,png,wav,tiff,ply`, `m3 md,xml,json,obj,jpg,svg`, `m4 xlsx,pptx,docx,step,binary,deflate`.
6. `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-stdio --target wasm32-wasip2 --lib` → "COMPOSITION GREEN stdio".
7. Tests (≥ 25 GiB, `zsh T/🚦️gate.sh 3 25`): `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-semio --lib --
image::io presentation::io` (the five repaired laws; the tiff round trip now asserts all four samples), then the derived payload
law of one marked crate (`-p semio-s-artifact-stdio-gif --lib -- mutation`), the `--features oracles` check of the oracle crates
and `test parity exhaustive` for the D4 rows (unchanged from S4, not started).
8. After everything: `bun …/🧪️test/📜️script.ts schema mutation-inputs --census --under ✏️s/🔌️plugins/🗄️stdio`, `schema
mutation-payloads --json --under ✏️s/🔌️plugins/🗄️stdio`, `schema mutation-editability --json --under ✏️s/🔌️plugins/🗄️stdio`.

### S5.7 Open items (P5) — recommendations

- **Composition ledger** (`🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧫️fixtures/🧩️composition/🔣️.json`, 218 of 265 `callers` rows
  stale): there is NO refresh command — `🔮️oracles/📜️script.ts` registers tests only, and the law (`🧪️tests/🧩️composition/🟦️.ts:247–258`)
  pins the sha256 of every caller test file with the oracle-crate rename undone. That is a migration pin over files that are
  edited every wave; refreshing 218 digests buys one green run. Recommendation (hub = coordinator/INFRA decision, design §21.3
  "compat pins are deleted"): delete the per-caller digest assertion and keep the structural rows (source exists, names the
  current package, no retired path). Audit meanwhile: `python3 T/🧪️s4-stdio-composition-ledger-audit.py`.
- **gltf `♾️any` set/patch-snapshot ownership**: both leaves address the whole asset, never one of the eight subsets, so neither
  "smallest subset owner" nor a subset policy `single` is true. Recommendation: own them in the `♾️any` manifest as an explicit
  typed compound over the eight subsets (the escape `wildcard-subset-owner` itself names), one row each, instead of sixteen
  duplicated subset rows that would test the same dispatch arm; with §22.20 `snapshot/set` is withdraw-only and `snapshot/patch`
  is typed per instance by its pointer, so the compound is exact.
- **hub trinity lib-test macro gap**: untouched (S5-INFRA's macro). The hub crate lists `semio-framework-async-macros` but the
  `plugin_exports!` test arm names `crate::semio_framework_async`; owed check after INFRA's fix: `cargo check --manifest-path
  🌎️hub/Cargo.toml -p semio-hub-trinity --lib --tests`.
- **Tool census (c)**: png / tiff / bmp paint and pdf page editors publish a plain edit outside a tool transaction — port onto the
  framework gesture slot after "PRESS ON DISK" (S5-TOOLS §22.29); not started.

### S5.8 Coordinator actions

- Central `schema generate` (stdio: 66 `leafUncatalogued` + 20 `malformed`; every edited leaf schema changes its catalogue hash).
- Describe at the final wave: `stdio`, `trinity`, `writer`, `vcs`, and every composition that publishes `s.stdio.semio` member
  leaves (reasoning, dag, sequence, flow, mathematical, imperative, playbook, cad, space) — the input descriptors and the
  withdraw-only flags travel in `INPUT_SCHEMAS`.
- Rule 49 (live proof): the probe exercises puzzle 2d only; none of the stdio declarations is reachable live. The missing
  user-visible step is "open a stdio document, edit a parameter leaf in the history editor in both renderers" (e.g. pdf
  `set-page-rotation`: a dial with four snaps; gif `set-screen-size`: two `px` steppers; a `set-snapshot` row offering Withdraw
  only).

- 10:49 PARKED (rule 66): gate v6 closed again at 10:48:55 (shared-cargo 3). `stdio` lock released 10:44; no cargo of mine is running; b5 and the rewriting marker stay WRITTEN BUT UNVERIFIED by cargo (commands in S5.3 / S5.6 and `📓️w3-t2-text-report.md` S5.5).

### S5.9 USAGE STOP 11:35 — state for the 14:20 resume

- ALL families are APPLIED (8 apply-only `stdio` holds, train lines 11:01–11:04, restore = `tar -xf 🗑️generated/s5-text-stdio/pre-<name>.tar -C <repo>`): **1226 / 1226 inputs declared, 114 / 115 `editable: false` markers** (gltf `default-scene/unbind` stays blocked until S5-GATES' derive hunk accepts `payload = <Variant>` on a withdraw-only leaf), **tool-mismatch codemod 72 / 72** stdio sites.
- VERIFIED in the private build dir (`zsh T/🗑️generated/s5-text-stdio/check.sh <name> ✏️s/Cargo.toml -p … --lib`, target `🗑️generated/s5-text-stdio/target`): b5 avi bmp dwg dxf epw gif + trinity-rewriting marker **exit 0** 11:11 (`check-b5.txt`); m2 csv mp3 png wav tiff ply **exit 0** 11:16 (`check-m2.txt`); m3 md xml json obj jpg svg **exit 0** 11:17 (`check-m3.txt`); m4 xlsx pptx docx step binary deflate: start 11:17:19 end 11:26:56 exit=0     Finished `dev` profile [unoptimized] target(s) in 9m 35s  (`check-m4.txt`).
- NO VERDICT m1 (las zip mp4 tsv txt stl): exit 101 at 11:13 on a framework mid-save that is not stdio — `🔌️plugin/🦀️.rs:13676:43 E0425 tool_once_emit` not found in `semio_framework_tool_machine` (`check-m1.txt`). OWED: `zsh T/🗑️generated/s5-text-stdio/check.sh m1 ✏️s/Cargo.toml -p semio-s-artifact-stdio-las -p semio-s-artifact-stdio-zip -p semio-s-artifact-stdio-mp4 -p semio-s-artifact-stdio-tsv -p semio-s-artifact-stdio-txt -p semio-s-artifact-stdio-stl --lib`.
- APPLIED BUT UNVERIFIED by cargo (rule 68 stopped new cargos at 11:18): b6 gltf html ifc bcf, b7 pdf, b8 semio + contract. OWED, same script: `check.sh b6 ✏️s/Cargo.toml -p semio-s-artifact-stdio-gltf -p semio-s-artifact-stdio-html -p semio-s-artifact-stdio-ifc -p semio-s-artifact-stdio-bcf --lib`; `check.sh b7 ✏️s/Cargo.toml -p semio-s-artifact-stdio-pdf --lib`; `check.sh b8 ✏️s/Cargo.toml -p semio-s-artifact-stdio-semio -p semio-s-artifact-stdio-contract --lib`; then jack / rewriting / writer / vcs `--lib` and `semio-hub-stdio` wasip2. (The cold b5 run compiled semio, pdf and docx as dependencies while those families were being applied — not a verdict for them.)
- Gate on the disk state, 11:22: stdio `mutation-inputs` **4705 → 997** (`numericUndeclared` 910 all nested, `inputless` 1, catalogue 86; label / widget / option findings 0; 111 leaves withdraw-only, 0 refused); trinity 16 (catalogue only).
- STAGED, NOT APPLIED — the record-member layer (`N` rows in `🧪️s5-text-stdio-input-ui-table.py`, region `RecordMembers`): 1003 member declarations in 100 shared records, dry check 0 problems, covers 568 of the 631 nested numeric nodes (837 of 910 findings). Before applying: `python3 T/🧪️s5-text-stdio-input-ui.py --emit-files T/🧪️s5-text-stdio-input-ui.files.txt --emit-bundle <json>` (the committed file list predates this layer), `bun T/🧪️s5-text-stdio-input-ui-check.ts <json>`, then `apply.sh` per family. The other 63 nodes are leaf-level byte / index arrays of earlier declarations (binary, deflate, docx, gif, gltf `order` ×17, html `path`, semio bytes …: amend rows or withdraw-only by the value rule) and three inline `SemioVideoStream` members.
- NOT STARTED: ledger per-caller pin deletion (hub lock), gltf `♾️any` typed compound manifest, paint / page tool ports (after "PRESS GREEN"), every test build. The private target dir stays for the owed checks; delete it once they are green.

### S5.10 Base-crate red triage and the patch-snapshot inverse-rows law (2026-10-06, 03:34–03:55)

Sources: `🗑️generated/coord/base-check.txt` (coordinator runs 03:30 = 2923 located errors and 03:44 = 2906, the second
stopped before deflate), my private runs `🗑️generated/s5-text-stdio/check-p1-gltf.txt` (03:52, txt 78 errors) and
`check-p1-pdf.txt` (03:53, deflate 17 errors). `cargo check --lib` of dwg / txt / deflate / semio is **NOT green**.

- **Errors mine 0 / the Codex peer's all** — deflate 17, txt 78, dwg 1064, semio 1773. 0 errors sit in the 934 files my
  10-05 waves touched and 0 error lines mention `MutationLeaf`, withdraw-only, `editable`, `x-semio`, tool-mismatch or
  `FaultCode`. Caveat: every error is a name-resolution error (E0433 1363, E0425 1120, E0422 284, E0255 39, E0119 39,
  E0432 27, E0428 18, unreadable include 14), so type-check has not reached these crates: m1 (txt) and b8 (semio) of my
  waves stay WRITTEN BUT UNVERIFIED by cargo, nothing more can be said about them until resolution is clean.
- **Where** (03:44 run): dwg `🚪️io/🪶️sqlite/📸️snapshot` 1040; semio `🚪️io/📝️text/🔺️diff` 915, `🚪️io/💾️binary/🧬️mutations` 254,
  `🚪️io/💾️binary/🔺️diff` 219, `🚪️io/🪶️sqlite/📸️snapshot` 135, `🚪️io/📝️text/📸️snapshot` 99, `🚪️io/💾️binary/📸️snapshot` 67,
  `🚪️io/📝️text/🧬️mutations` 41; txt `🚪️io/💾️binary/🧬️mutations` 42 + `🚪️io/📝️text/🧬️mutations` 36; crate roots semio 16, dwg 14,
  deflate 14; old-tree leftovers 14 includes + 4.
- **Cause classes — one relocation in flight** (codecs moved into `🚪️io/{📝️text,💾️binary,🪶️sqlite}/{📸️snapshot,🔺️diff,🧬️mutations}`):
  1. crate roots written by a script at 03:05:12 (all four in the same second): semio mounts `io` twice per subset
     (`🧿️semio/🦀️.rs:1374, 1424, 1427, 1471, 1474, 3320, 3527, 3530, 3689, 3692, 3956, 3959, 4106, 4109, 4316, 4319` E0428),
     dwg and deflate re-export from an `io` their root never mounts (`🖊️dwg/🦀️.rs:143…`, `🗜️deflate/🦀️.rs:113–156` E0433);
  2. the relocated codec modules glob-import the old module but need its PRIVATE items — e.g. animation
     `🚪️io/💾️binary/🧬️mutations/🦀️.rs:24–36` uses `TAG_*` / `OP_BINARY_FORMAT`, which stay private `const`s in
     `🧬️schema/🧬️mutations/🦀️.rs` region `OpCodecs` (E0425 207 `TAG_*`, 894 functions / values, E0422 284 records);
  3. that leftover region still includes the protocol from its old place — `include_str!("💾️binary/📡️.protocol.semio")`
     in 13 semio subsets + dwg, while the file now lives at `🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio`;
  4. new sqlite / diff files name snapshot types without importing them (`DwgEntityBody`, `DwgLogicalObjectBody`,
     `SemioDiff`, `SemioSubsetSnapshot`, … E0433) and import `sqlite_native`, which no crate declares (deflate
     `🧬️schema/📸️snapshot/🦀️.rs:110`, `🚪️io/💾️binary/📸️snapshot/🦀️.rs:13`, `🚪️io/📝️text/📸️snapshot/🦀️.rs:19`);
  5. generated per-leaf payload codecs in txt import `…::io::{binary,text}::mutations::<Leaf>Payload`, which nothing
     exports, and each leaf module is declared twice (E0255 `insert_line` … at `🚪️io/💾️binary/🧬️mutations/🦀️.rs:62–77`);
  6. old and new codec impls both mounted (E0119 `OpText` / `OpBinary` / `ArtifactDsl` / `ArtifactPack`, semio kit / graph).
- **First error per crate**: semio `🧿️semio/🦀️.rs:1374:1 E0428 the name io is defined multiple times`; dwg
  `🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:44:69 couldn't read …/💾️binary/📡️.protocol.semio` (then
  `🖊️dwg/🦀️.rs:143:35 E0433 io`); txt `…/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs:62:1 E0255 insert_line`; deflate
  `…/✳️any/🧬️schema/📸️snapshot/🦀️.rs:110:9 E0432 sqlite_native` (then `🗜️deflate/🦀️.rs:113:35 E0433 io`).
- **No adaptation of ours applies.** Nothing in my files consumes an API the peer moved; the breaks are inside the
  peer's generated modules, the script-written roots and the regions its cut left behind. The peer is writing right now
  (deflate `🧬️schema/📸️snapshot/🦀️.rs` 03:46:17, semio animation `🚪️io/🪶️sqlite/📸️snapshot` tests 03:35–03:41, gltf
  `♾️any/🚪️io` 03:3x; `Codex` processes alive), so a hand edit of those files would collide with its generator and be
  overwritten or break its anchors. **Only the peer can finish**: mount `io` exactly once per subset in the four roots;
  move the `OpCodecs` leftovers (tags, keywords, format) into the `🚪️io` module or make them `pub(crate)` and re-point the
  14 includes; add the missing imports in the sqlite / diff files and declare `sqlite_native`; export the `<Leaf>Payload`
  types and drop the duplicate leaf mounts in txt; delete the old impls duplicated in `🚪️io`.
- **Payload law red (pdf op 44, gltf op 76: "answers 128 inverse row(s) where its leaf schema declares 1") — FIXED AT
  THE SOURCE.** Cause: `#[mutation_leaf(input_schema = Self::input_schema_at_path)]` answers the per-operation schema
  built by `snapshot_patch_input_schema_text` (`📇️registry/🧬️contract/✏️editing/🩹️patch/🦀️.rs`), whose root carried no
  `x-semio-inverse-rows`; the kernel law (`📡️spr/🎮️command/🦀️.rs` `mutation_inverse_rows_declaration_failures`) then reads
  "1" while the leaf answers `SNAPSHOT_PATCH_MAX_INVERSE_PARTS`. All 56 static leaf schemas already declare
  `{ "bounded": 128 }`; only crates whose snapshot schema resolves at test time (pdf, gltf) reach the generated text.
  Hunk: the generated root now carries `x-semio-inverse-rows { bounded: SNAPSHOT_PATCH_MAX_INVERSE_PARTS }`; new contract
  test `a_patch_input_schema_declares_the_inverse_rows_its_leaf_answers` (`🩹️patch/🧪️tests/🦀️.rs`). Train line 03:40:52
  `p1-patch-schema-inverse-rows`, restore `tar -xf 🗑️generated/s5-text-stdio/pre-p1.tar -C <repo>`.
  - VERIFIED 03:45: `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-contract --lib -- a_patch_input_schema`
    → `2 passed; 0 failed` (the new test + the existing reader / validator test over the new text), `check-p1.txt`.
  - NOT RUN — the derive-emitted laws themselves: gltf's test closure contains txt (red, 78), pdf's contains deflate
    (red, 17). Owed once those two compile, private dir, one at a time:
    `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-gltf --lib -- semio_payload_law` and
    `… -p semio-s-artifact-stdio-pdf --lib -- semio_payload_law`.
- Private target dir deleted at the end of this turn (951 MiB); every owed check of S5.9 (m1, b6, b7, b8) stays owed and is
  blocked by the same four crates.
