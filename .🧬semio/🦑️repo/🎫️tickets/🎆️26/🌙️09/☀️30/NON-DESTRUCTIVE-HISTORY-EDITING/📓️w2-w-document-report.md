# 📓️ W2-W-document — wire-witness conversion: pdf 1.7 + 1.4, semio, docx (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-document, 2026-09-30.

- **Contract:** `📋️design.md` §6 and §11, `🧭️plan.md` "W2-W brief", `📓️w2-s-report.md` F10.
- **Scope:** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📖️pdf,🧿️semio,📜️docx}`.
- **Coordinator additions handled:** the semio object/kit TS twins (F13 helper); the drawing/image fixture-name note.

## 1. Outcome

| lint, `--under <artifact>` | before | after |
|---|---|---|
| pdf `schema mutation-payloads` | 71: 24 `invalid`, 2 `undescribed`, 45 `unwitnessed` | **0**; 258/258 payloads, 149/149 leaves witnessed |
| semio `schema mutation-payloads` | 32 `unwitnessed` | **0**; 320/320 payloads, 241/241 leaves witnessed |
| docx `schema mutation-payloads` | 28: 16 `invalid`, 6 `unmapped`, 6 `unwitnessed` | **0**; 55/55 payloads, 32/32 leaves witnessed |
| `schema mutation-inputs` (pdf / semio / docx) | 0 / 0 / 0 | **0 / 0 / 0**: 201, 450 and 51 inputs |

**Every in-scope adapter now decodes its rows generically.**

- The per-kind `params`→op `match` is deleted from all 13 in-scope adapters (10 pdf, 3 docx), and so is every helper that only served it.
- Every row decodes through the derive's `from_payload_value`.
- The oracles read the same wire.

**Case results:**

| case | executions | parity | status |
|---|---|---|---|
| pdf 1.4 base | 22/22 | 11/11 | green |
| pdf 1.4 x | 10/10 | 5/5 | green |
| pdf 1.4 a | 10/10 | 5/5 | green |
| docx strict | 38/38 | 19/19 | green |
| docx transitional | 22/22 | 11/11 | green |
| semio image | 80/80 | 40/40 | green |
| pdf 1.7 base | 65/66 | 15/33 | red: the pdf encoder (§4.1) |
| pdf 1.7 ua | 46/46 | 15/23 | red: the pdf encoder (§4.1) |
| pdf 1.7 h | 42/42 | 19/21 | red: the pdf encoder (§4.1) |
| pdf 1.7 e | 50/50 | 23/25 | red: the pdf encoder (§4.1) |
| pdf 1.7 x | 58/58 | 25/29 | red: the pdf encoder (§4.1) |
| pdf 1.7 a | 58/58 | 27/29 | red: the pdf encoder (§4.1) |
| pdf 1.7 vt | 74/74 | 31/37 | red: the pdf encoder (§4.1) |
| docx base | 50/50 | 0/25 | red: whole-package docProps digests (§4.2) |
| semio drawing (runs again) | 103/104 | 33/52 | red: pre-existing projection mismatch (§4.3) |

- **The red cases are pre-existing product divergences, not the conversion.** The pdf 1.7 and docx base subject hosts did not compile before this WP: the base pdf adapter still built `PdfPage { text }`, and the docx adapters named a `subsets::any` module and set a `pub(crate)` field.
- **Evidence that the decode path is not the cause:** every row decodes, and every oracle and subject execution runs. For example, the ua subset's `SetMarkInfo { marked: true }` is the same op before and after the conversion.

## 2. What changed

### 2.1 Generic decode bridge (one per in-scope aggregate)

- **Why a bridge is needed:** a test host links only the artifact crate. It cannot name `DslValue`, `pack` or the kernel, so the recipe's `from_payload_value(kind, DslValue::from(params))` needs a production entry point.
- **The bridge:** each aggregate's Delegation/Apply region gains `decode_<aggregate>_mutation_payload(kind: &str, payload: &str) -> Result<Aggregate, String>`. It runs `from_json_str` and then `<Aggregate as Mutation<_>>::from_payload_value(kind, value)`.
- **Precedent:** this follows W2-S-B2's `decode_semio_*_mutation_json`.
- **Aggregates covered (13):**
  - pdf 1.4: `base`, `x`, `a`;
  - pdf 1.7: `base`, `ua`, `h`, `e`, `x`, `a`, `vt`;
  - docx: `base`, `strict`, `transitional`.
- **Adapter side:** each adapter's `mutation_from_spec` is one line: `decode_…(&spec.str("kind"), &spec.get("params")…to_string())`.
- **Script:** `🧪️w2-w-document-decoders.py`.

### 2.2 pdf

**1.7 base feature rows are now the leaf wire** (`🧪️w2-w-document-pdf17-rows.py`):

- `insert-page` → `page: {mediaBox, rotate, content: [PdfOp…]}`.
- `append-page-content` / `set-page-content` → `content: [PdfOp…]`, instead of the `text` shorthand. The ops are the same `BT /F1 12 Tf 72 720 Td (…) Tj ET` the old adapter rendered.
- `set-info` → `{info: {title, author}}`.
- `insert-object` / `set-object-value` → `PdfObject` newtypes under `value`, with strings as byte arrays.
- The feature prose about "PdfPage's only content field is text" is rewritten: the vocabulary carries typed ops, and the carve-out is the reference's lossy `Tj`-only undo.

**1.7 base oracle** (`🔮️oracles/🦀️.rs`) reads the Rust wire:

- the `PdfObject` grammar in both directions: `real` as `PdfDecimal`, `str` as bytes, `stream` with its `dict`/`data`/`filters`;
- `PdfPathSegment` `arrayIndex`/`dictKey`;
- `PdfOp` text-object operators, encoded with lopdf `Content`; any other operator is refused;
- `SetInfo.info`;
- `cropBox: null`;
- members outside what it reproduces are refused (`only_members`).
- **Unit tests** read their parameters from the feature rows (`parse_json`), not from a restated copy.
- The subject's `protocol::Mutation::inverse`, which the host cannot link, is replaced by `inverse_pdf_mutation`.

**Conformance subsets** (`🧪️w2-w-document-pdf-conformance-adapters.py`):

- Rows: `set-display-doc-title` → `{display}`.
- `embed-font-file` → `program: {num: 5, gen: 0}`. That is the program `programOrdinal: 0` resolved to on every seed, computed with pypdf the same way the arrange step does.
- The shared `document::pdf_conformance` engine speaks `display` and requires an exact `program`.
- Deleted as dead code:
  - the `programOrdinal` lookup;
  - `font_programs` in the oracle;
  - `font_programs` in production `conformance_support`, whose only users were the adapters.
- The six unit tests use `program {803, 0}`, the thesis equivalent of the old `programOrdinal`.

**1.4 base / x / a:** the rows were already wire-valid; only the adapters changed.

**Witnesses:** 45 for 1.7 base (§2.5).

### 2.3 docx

**base feature** (`🧪️w2-w-document-docx-base-feature.py`):

- **Run addresses:** `set-run-text` / `set-run-formatting` carry the canonical `DocxXmlAddress` the old `{path, runIndex}` resolved to: `nodePath [0,177,1]`, revision `1dc1281dba7711ea`. It was printed by the crate's own `docx_block_run_address` in a temporary `[DEBUG]` probe, since removed; the output is `🗑️generated/w2w-document/docx-probe.txt`.
- **Field names:** `set-style-based-on` carries `based_on`, and `set-part` carries `content_type` plus `bytes`, as Rust emits them.
- **`no-mutation`:** its baselines are deleted, because the identity round trip is the baseline.
- **`set-snapshot`:** its payload is a whole package, so it cannot sit in a table cell. It becomes the `set-snapshot` / `set-snapshot-inverse` scenario pair.
  - The payload is the snapshot decoded from the committed `🧾️readme-afters/📸️set-snapshot/➡️after.docx`.
  - The Rust reference answers with `oracle_replace_package`.
  - The TS jszip oracle answers with the committed after-document.

**base oracle** (`🧪️w2-w-document-docx-base-oracle.py`):

- **Run addresses:** it resolves the wire address in its own quick-xml tree (`resolve_run_address`). Each `nodePath` step becomes a `w:p`/`w:tbl`/`w:tr`/`w:tc`/`w:r` ordinal, and `expectedName` is checked.
- **Other fields:** it reads `based_on`, and `content_type` + `bytes`.
- **Undo:** the computed undo is now a list of wire specs, and the internal `no-mutation` is gone.
- **New entry points:** `oracle_replace_package` and `oracle_round_trip`.
- **Unit tests:** updated, plus a new replace/replace-back law.

**base adapter:**

- The block/style/path/run codec is deleted.
- `SetSnapshot.snapshot` is now `pub`. The adapter builds the op from the decoded after-document, and the field was the only reason the host did not compile.

**strict / transitional** (`🧪️w2-w-document-docx-conformance-{adapters,features}.py`):

- **Rows:**
  - `set-snapshot {conformanceClass}` leaves the tables: its wire is the entire stamped README.
  - `no-mutation` baselines are deleted.
  - `insert-vml-part` carries `markup`.
- **New scenarios:** `stamp-conformance-class` / `-inverse`.
  - Subject: the new production helper `stamp_conformance_class_mutation(base, strict)`, one `SetSnapshot` of the stamp.
  - Oracle: the new `oracle_stamp`, the engine's own stamp.
- **Oracle `KINDS`:** `no-mutation`/`set-snapshot` dropped.
- **Shared ooxml engine:** the inverse of `remove-vml-part` now carries the removed part's own `markup` (the subject decodes that undo).

### 2.4 semio

**Witnesses:** 32 (§2.5).

**Object / kit TS twins (coordinator item):**

- **What changed:** both now use `semioSchemaAjvV1({allErrors: true})` and `addSemioMutationLeafSchemasV1`. Hand-registered vendor keywords and the fast-glob leaf loading are gone.
- **What that exposed, and the fixes:**
  - object `📸️snapshot/🔣️.json` lacked `additionalProperties: false`, which the Rust `FromValue` enforces ("unknown Object field") and the twin expects. Added.
  - kit `parseSemioKitMutation` refused `SetSnapshot`: it was missing from its allowed-key list although the parser had a branch for it. Added.
  - the kit twin's corpus counts are now 16/16.
- **Result:** both twins PASS in the sweep.

**Drawing / image:** every fixture reference in both features resolves on disk now, so no feature edit was needed. Both cases run again (§1, §4.3).

### 2.5 Wire witnesses — 85 files at `<owner>/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json`

Script: `🧪️w2-w-document-witnesses.py`. Plans are in `🗑️generated/w2w-document/plan-*.json`, with sources in `plan-semio-sources.json`.

- **pdf 1.7 base (45):** schema-first instances of each leaf payload, internally tagged. Each one takes every member of the top two object levels and the required members below.
- **semio (32):**
  - the 8 subset `set-snapshot` witnesses are `{"SetSnapshot":{"snapshot":<committed ⬅️before>}}`. The kit snapshot has its `objects`/`models` emptied: the lint cannot describe `allOf` child references inside arrays (§4.5);
  - the 17 base `apply-*` witnesses are `{"mutation":"applyX","payload":{"mutation":<committed subset op>}}`;
  - the 7 value witnesses come from the value feature's rows.
- **docx (8):**
  - 6 base leaves from the crate's own `demo_mutation_cases()` wire;
  - the strict/transitional `set-snapshot` witnesses are the ToValue of a stamped minimal snapshot.
- **Decode verified, not assumed.** `mutation_fixture_ops` silently drops undecodable files, so temporary probes (since removed) asserted `ops == files` for every witness: pdf 45/45, semio 32 × 1/1. The docx witnesses are Rust ToValue output, and `semio_payload_law_docx_*` passes.

## 3. Verification (all run, results seen)

| check | result |
|---|---|
| both lints, `--under` pdf / semio / docx | 0 findings each (§1) |
| `cargo check -p …-pdf` | ok |
| `cargo check -p …-pdf -p …-docx -p …-semio --target wasm32-wasip2` | ok |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2w-document cargo test -p semio-s-artifact-stdio-pdf --lib` | 517 passed; all 10 `semio_payload_law_*` ok |
| same, `-p semio-s-artifact-stdio-docx --lib` | 76 passed, 1 ignored; `semio_payload_law_docx_{mutation,strict_mutation,transitional_mutation}` ok |
| same, `-p semio-s-artifact-stdio-semio --lib` | 2596 passed, 1 ignored (incl. every `semio_payload_law_*`) |
| oracle crate `cargo test --features oracles --lib -- artifacts::pdf artifacts::docx` | 52 passed |
| `bun 🧪️w2-s-ts-twin-sweep.ts` (object, kit document contracts) | PASS, PASS |
| `SEMIO_TEST_BUDGET_MS=7200000 bun ./📜️script.ts parity exhaustive --case <case>` | per §1 table; outputs in `🗑️generated/w2w-document/parity-*.txt`, `pdf17-run3.txt` |

- **Why the raised budget:** the pdf 1.7 base subject decodes the 6.3 MB thesis about 30 times under fleet load. The default budget killed it (`pdf17-run2.txt`).

## 4. Open items (pre-existing, surfaced now that the subject hosts compile — routed, not mine)

### 4.1 pdf encoder: typed lanes override COS-lane edits (routed to the pdf artifact owner)

Diffs are in `.🧬semio/🦑️repo/⚡️cache/tests/diffs/*pdf*`.

1. **Catalog rewritten on every forward mutation.** `encode_pdf` rewrites the catalog: it inlines `/Names /Dests` and adds `/OpenAction /Type /Action`. This accounts for 2 differences in each of the 13 pdf 1.7 base mutate rows.
2. **COS-level edits are reverted by typed lanes.** Edits to objects that typed lanes re-emit are lost:
   - `set-object-value #145` (OpenAction);
   - `remove-object #3015` (Outlines);
   - `set-dict-entry`/`remove-dict-entry` on the catalog.

   The same applies in every conformance subset: MarkInfo/StructTreeRoot/Lang/ViewerPreferences (ua), AcroForm signature fields (h), OutputIntents (e/x/a/vt), TrimBox (x/vt), DPartRoot (vt).
3. **`set-trailer-entry` is dropped.** A custom trailer key is not written; the subject observability law fails.
4. **The 3 known-red inverses remain** (`inverse-remove-page/append/set-page-content`): the reference's `Tj`-only undo. Documented in the feature and the oracle.

### 4.2 docx base: jszip compare vs whole-package re-serialisation

- **docProps digests:** all 25 scenarios, the identity round trip included, differ only in the `docProps/core.xml` and `docProps/app.xml` byte digests. The subject re-serialises every XML part, while the compare hashes non-main parts.
- **`set-run-formatting`:** the subject writes `<w:b w:val="0"/>`, which is correct per ECMA-376, but the jszip probe (and the Rust oracle's `run_from_xnode`) reads the presence of `w:b` as bold.

### 4.3 semio drawing: projection vocabulary mismatch (not touched here)

- Rust projects `nodes.group-nodes` and different path counts; the Python oracle projects `nodes.group`.
- This hits the identity round trip itself.

### 4.4 Shared ooxml engine (office peer)

- **Still-needed dialect:** `apply_conformance_mutation` keeps `set-snapshot {conformanceClass}`, `no-mutation` and the `markup_or_default` fallbacks. Their only remaining users are the not-yet-converted xlsx/pptx rows.
- **Recommendation:** the office peer should adopt the same `stamp-conformance-class` scenario shape; the last converter then deletes those fallbacks.

### 4.5 Lint and taxonomy (routed to W2-S / W3-G)

- **The `undescribed` check does not merge `allOf` branches for child references inside arrays.** Example: kit `objects[].target.artifactId` via `child.json#/$defs/object`.
- **`mutation_fixture_ops` drops undecodable fixtures without a count check.** So a malformed witness passes the payload law vacuously.
- **Taxonomy `directory-kind-unresolved`** fires on:
  - every new `🧾️wire-witness` directory, exactly as on the D-scope store witnesses;
  - the pdf 1.7 base `🧫️fixtures/🧬️mutations/<leaf>` level, which that subset never had.

  These directories follow the brief's mandated path. The registration belongs to the taxonomy owner.

### 4.6 Pre-existing warnings in touched docx files

The warnings: unused `DocxParagraph`/`DocxRun` and zip imports, unused `sweep_a`, and in the docx oracle an unused `RELS_CONTENT_TYPE`. They predate this WP.

## 5. Files

**Ticket scripts (kept):**

- `🧪️w2-w-document-decoders.py`
- `🧪️w2-w-document-pdf-conformance-adapters.py`
- `🧪️w2-w-document-pdf17-rows.py`
- `🧪️w2-w-document-witnesses.py`
- `🧪️w2-w-document-docx-conformance-adapters.py`
- `🧪️w2-w-document-docx-conformance-features.py`
- `🧪️w2-w-document-docx-base-oracle.py`
- `🧪️w2-w-document-docx-base-adapter.py`
- `🧪️w2-w-document-docx-base-feature.py`

**pdf** (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/…`):

- the aggregates `{4️⃣1.4/{🧱️base,🖨️x,🗄️a},7️⃣1.7/{🧱️base,♿️ua,⚕️h,📐️e,🖨️x,🗄️a,🧾️vt}}/🧬️schema/🧬️mutations/🦀️.rs`;
- the 10 adapters `…/🧪️tests/<case>/🦀️.rs`;
- the 7 features of 1.7;
- `7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`;
- the six conformance `🔮️oracles/🧪️tests/🔬️unit/🦀️.rs`;
- `🧱️base/🧬️schema/🏅️conformance-support/🦀️.rs`, from which `font_programs` is deleted;
- 45 witnesses under `7️⃣1.7/🪆️subsets/🧱️base/🧫️fixtures/🧬️mutations/`.

**docx** (`…/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/`):

- `{🧱️base,📏️strict,🔄️transitional}/🧬️schema/🧬️mutations/🦀️.rs`;
- `🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs` (the `pub` field);
- the 3 features;
- the 3 Rust adapters and the base `🟦️.ts` adapter;
- the base, strict and transitional `🔮️oracles/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`;
- 8 witnesses.

**semio** (`…/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`):

- `📦️object/🧬️schema/📸️snapshot/🔣️.json`;
- `📦️object/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`;
- `🧰️kit/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`;
- `🧰️kit/🧬️schema/🧬️mutations/🟦️.ts`;
- 32 witnesses.

**Shared oracle crate:** `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/🦀️.rs`:

- `pdf_conformance`: `display`, exact `program`, `font_programs` deleted;
- `ooxml`: the `remove-vml-part` undo carries its `markup`.

**Scratch:** `🗑️generated/w2w-document/` (lint JSON before and after, the probe output, the plans, and case and cargo logs).
