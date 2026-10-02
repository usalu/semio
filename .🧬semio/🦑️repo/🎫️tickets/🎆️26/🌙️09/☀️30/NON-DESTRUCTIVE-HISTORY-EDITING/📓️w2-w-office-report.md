# 📓️ W2-W-office — wire-witness conversion (xlsx, pptx, zip, binary, deflate) + presentation set-snapshot order

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-office, 2026-09-30. Status: DONE (one verification blocked by a peer, §6). Contract: design §6/§11, plan "W2-W
brief", recipe `📓️w2-s-report.md` F10. Extra items from the coordinator: the presentation `set-snapshot` parity
disagreement and the `office-schema-contract` TS test.

## 1. Outcome

| scope | `schema mutation-payloads` before → after | `schema mutation-inputs` | leaves witnessed |
|---|---|---|---|
| 📕️xlsx (base, strict, transitional) | 26 → **0** (45/45 rows+fixtures clean) | 0 (27/27) | 23/23 |
| 📽️pptx (base, strict, transitional) | 24 → **0** (47/47) | 0 (32/32) | 24/24 |
| 🎒️zip (base, iso21320) | 26 → **0** (27/27) | 0 (22/22) | 13/13 |
| 💾️binary | 12 → **0** (18/18) | 0 (6/6) | 4/4 |
| 🗜️deflate | 10 → **0** (9/9) | 0 (6/6) | 4/4 |

Every touched case, `bun ./📜️script.ts parity exhaustive --case <case>` (run, results seen):

| case | executed / passed | oracle-vs-subject parity |
|---|---|---|
| `🔀️mutate-binary-raw` | 18/18 | no-oracle feature (subject cross-checks the spec implementation in role) |
| `🗜️mutate-deflate-rfc1950` | 18/18 | 9/9 |
| `🔀️mutate-zip-2-0` | 26/26 | 13/13 |
| `🔀️mutate-zip-2-0-iso21320` | 30/30 | 15/15 |
| `🔀️mutate-xlsx-ecma-376` | 38/38 | 19/19 |
| `🔀️mutate-xlsx-ecma-376-strict` | 34/34 | 17/17 |
| `🔀️mutate-xlsx-ecma-376-transitional` | 26/26 | 13/13 |
| `🧱️mutate-pptx-ecma-376` | 34/34 | 17/17 |
| `🔒️mutate-pptx-ecma-376-strict` | 42/42 | 21/21 |
| `🌉️mutate-pptx-ecma-376-transitional` | 26/26 | 13/13 |
| `📽️mutate-semio-presentation` | 92/92 | **46/46** (was 40/46: `mutate-/inverse-set-snapshot` 6 differences) |

Crate lib tests and the oracle crate: see §6.

## 2. The adapter surface (one decision for all five crates)

The generated test host links the artifact crate alone, so an adapter could not name `protocol::Mutation` — which is why
every adapter hand-mapped params and hand-transcribed the inverse (and why the xlsx/pptx/zip subjects had not compiled since
the leaf fields became `pub(crate)` on 09-25). Each in-scope artifact crate root now re-exports, explicitly:

```rust
pub use protocol::json::{from_json_str, to_json_string};
pub use protocol::{DslValue, Mutation};
```

Every subject adapter decodes a row as

```rust
let payload: DslValue = from_json_str(&params.to_string())?;
let mutation = <Agg as Mutation<Snap>>::from_payload_value(&kind, payload)?;
params_are_wire(&kind, &params, &to_json_string(&<Agg as Mutation<Snap>>::payload_value(&mutation)))?;
```

and undoes it with `<Agg as Mutation<Snap>>::inverse(&mutation, &base)` — the production law under test, never a
transcription. The now-unused `inverse_zip_iso21320_mutation` wrapper (it existed only so an adapter could avoid naming
the trait) was deleted.

New shared law `⚖️law::params_are_wire(kind, params, emitted)` (`✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/🦀️.rs`, + unit
test): the subject's re-emitted payload must equal the row member for member (order-free). It caught the zip oracle still
reading `new_name` after the camelCase fix (a parity diff) and guarantees no row carries a member the decoder ignores.

Note for peers: the docx executor chose per-aggregate `decode_<agg>_mutation_payload` bridges instead; both reach the same
derive-generated `from_payload_value`.

## 3. Per artifact

- **binary**: rows → wire (`remove_len`, snapshot `schema`); `no-mutation` scenarios removed (the identity round trip is
  the baseline); oracle reads the wire, `no-mutation` arm replaced by `oracle_round_trip`; subject inverse through
  `Mutation::inverse`, cross-checked against the oracle's own undo.
- **deflate**: rows → wire (payload texts as UTF-8 byte arrays, `window_bits`/`level_hint`/`dict_id`); oracle reads the
  wire and gains `oracle_inverse_spec(base, forward)` + `oracle_round_trip` (the adapter no longer parses header fields
  out of the projection); `no-mutation` scenarios removed.
- **zip base + iso21320**: rows → wire (full `ZipEntry` with `metadata`; iso members declare method 0/8). Rust was the
  outlier on two leaves: `RenameEntry` and `SetArchiveComment` emitted `new_name`/`comment_utf8` while schema/TS/graphql
  said camelCase → `#[value(rename_all = "camelCase")]` (base + iso) and the rename schema → `newName`. The add-entry leaf
  schemas carried a stale local `ZipEntry` without `metadata` → they now `$ref` the snapshot's `ZipEntry`. The iso feature's
  stale "ZipSnapshot has no method slot" finding was rewritten (the metadata facet exists). Both oracles read the wire.
- **xlsx base**: `RenameSheet` camelCase (TS twin said `newName`). Rows → wire: `XlsxSheet` with tagged cell values,
  `set-cell`/`remove-cell` carry the lineage-bound `XlsxCellAddress` the subject takes on the real workbook (captured once
  with a temporary `[DEBUG]` generator test, since removed), `set-snapshot` carries a whole replacement `XlsxSnapshot`
  (`build_minimal_xlsx` of one sheet "Ersatz"). Oracle: an independent cell-address resolver (own XML tree walk of
  `nodePath`, `r` reference, sheet name via the main part and its relationships), `set-snapshot` through the new shared OPC
  writer, `oracle_apply_inverse(original, mutated, forward)` (addresses are lineage-bound to the original), and — because
  production now refuses to remove a referenced pool entry and every entry of the real pool is referenced — a reference
  guard (`shared_strings::is_referenced`) plus `oracle_arrange` (appends one unreferenced entry; the row removes index 229).
  The `sharedStringCount` exemption for `set-snapshot` is gone: every inverse is held to the whole projection.
- **xlsx strict/transitional**: PRODUCTION FIX. Since the snapshot became XML-authoritative (`xml_parts`), every class edit
  read/rewrote `opc.parts` XML bytes and was a silent no-op (parity showed the subject unchanged). The diff builders now edit
  a copy — logical XML parts, content-type table, the opaque VML part — and diff it with the base subset's
  `diff_set_snapshot`; the hand-rolled sparse OPC diff helpers are gone (`🧪️w2-w-office-xlsx-conformance-authority.py`).
  New unit test `class_edits_move_the_authoritative_main_part` in both subsets.
- **xlsx/pptx strict + transitional (all four)** (`🧪️w2-w-office-conformance.py`): rows → wire (`insert-vml-part` carries
  its `markup`, `set-worksheet-content-type` uses `content_type`); the whole-package class stamp, whose payload is the
  entire stamped package, is the plain `mutate-set-snapshot`/`inverse-set-snapshot` pair (ids kept so the catalog's
  kind-coverage contract still holds): subject records `stamp_conformance_class_mutation` (new production helper per
  aggregate, mirroring docx), reference stamps with `oracle_stamp`; oracle `KINDS` drop `no-mutation`/`set-snapshot`; the
  `set-snapshot` leaves gained wire witnesses `🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json`
  (base quintet snapshot); dead `vml_markup()` and `VML_NS` removed from xlsx/pptx strict.
- **pptx base**: `InsertShape`/`RemoveShape`/`SetShapeText`/`SetShapePosition` camelCase (TS twin said
  `slideIndex`/`shapeIndex`/`textFrame`) + `x-semio-ui` en/de labels for the two targets. Rows → wire (`PptxShape` wire,
  `set-snapshot` a typed-presentation `PptxSnapshot`); oracle reads/writes the wire; its undo spec is fallible (no silent
  `no-mutation` fallbacks).

Shared engine (`🔮️oracles/📃️document/🦀️.rs`, region `🔖️PackageWire`, additive): `write_snapshot_package(snapshot)` — an
independent OPC writer (content types, one `.rels` per source, opaque parts, XML parts serialized from their wire DOM).
`set-worksheet-content-type` reads/emits `content_type` (xlsx-only kind). The `set-snapshot {conformanceClass}` arm of
`apply_conformance_mutation` is untouched: only docx still drives it (docx executor's call).

## 4. Presentation `set-snapshot` order

The leaf's contract is "the document becomes this snapshot", so the Python oracle (reversed deck) was right and Rust wrong:
the slide collection is index-keyed and `SlideDiff` had no identity slot, so `between(base, reversed)` moved content but
left the base's ids at their indices. Fix at the source: `SlideDiff.id: Option<String>` (skip when unchanged) —
`diff_slide`/`apply_slide`/`inverse_slide`/`absorb_slide_diff`, the text codec (`slide-diff` leads with `option-hex`), the
grammar, the diff JSON schema, the TS twin and graphql. Pinned by fixture `🧫️fixtures/🧬️mutations/📸️set-snapshot/
🔃️reverses-slide-order` (reversed two-slide deck) + test `🔃️reverses-the-slide-order` (apply, inverse, committed diff,
diff-applies-to-after, codec round trip). The feature's "RED, left red" paragraph now records the fix.

## 5. office-schema-contract TS test

`✏️s/🔌️plugins/🗄️stdio/🧪️tests/📚️office-schema-contract/`: the import moved `🧩️composition` → `🏘️composition`; strict
Ajv then rejected `x-semio-ui` → every validator now comes from the shared `semioSchemaAjvV1` (the registered vendor
vocabulary); the xlsx public-facet proof still used the removed `workbook` field → `xmlParts`. Passes (run).

## 6. Verification (all run)

| check | result |
|---|---|
| `schema mutation-payloads --under <artifact>` (5 artifacts + semio presentation) | 0 findings each; presentation 31/31, 14/14 witnessed |
| `schema mutation-inputs --under <artifact>` (5 artifacts) | 0 findings each |
| `parity exhaustive --case …` (11 cases, §1) | all executed scenarios passed; every parity comparison equal |
| `cargo test -p semio-s-artifact-stdio-xlsx --lib` | 75 passed, 2 ignored — incl. `semio_payload_law_xlsx_{,strict_,transitional_}mutation` (the new wire witnesses) and the new `class_edits_move_the_authoritative_main_part` ×2 |
| `cargo test -p …-pptx -p …-zip -p …-binary -p …-deflate --lib` | pptx 81 (+1 ignored), zip 68, binary 43, deflate 51 — all pass, payload laws included |
| `cargo test -p semio-s-artifact-stdio-semio --lib presentation` | 63 passed, incl. the 5 `set_snapshot_reverses_the_slide_order` tests and `semio_payload_law_semio_presentation_mutation` |
| same with `--features conversion-presentation` | 78 passed, incl. `diff_grammar_conformance_law` over a new slide-reorder demo diff (id slot) |
| `testStdioOfficeSchemaContracts()` (bun) | passes |
| `tsc --noEmit` presentation diff TS twin | exit 0 |
| oracle crate `cargo test --features oracles --lib` | NOT RUN: the crate's lib-test build is blocked by a peer's dxf r12 header smoke test (`include_bytes!` of the missing `📚️examples/🚏️bus-shelter/🖼️assets/🧪️bus-shelter-r12/🖊️.dxf`). My oracle unit-test edits (binary, zip iso, xlsx base/strict/transitional, pptx base/strict/transitional, law `params_are_wire`) are WRITTEN BUT UNVERIFIED; the same oracle code paths are exercised by the 11 green case runs (the pptx unit test reads the feature rows the case runs). |

Build notes: one `cargo test` was SIGKILLed (137) under memory pressure (swap 6.8/8 GB); reruns used `CARGO_BUILD_JOBS=4`.
Scratch and logs: `🗑️generated/w2w-office/` (kept for the coordinator's sweep).

## 7. Open items / notes for others

- docx executor: its `stamp-conformance-class` scenario ids do not satisfy the catalog's `mutate-<kind>` coverage contract
  for `set-snapshot`; xlsx/pptx use `@id-mutate-set-snapshot`/`@id-inverse-set-snapshot` plain scenarios instead.
- Rust was fixed to camelCase only where a sibling contract (schema, TS twin, graphql) already said camelCase (zip
  `newName`/`commentUtf8`, xlsx `newName`, pptx `slideIndex`/`shapeIndex`/`textFrame`). Snake-case root keys that Rust and
  schema agree on with no camelCase twin were left: xlsx/pptx conformance `content_type`, deflate
  `window_bits`/`level_hint`/`dict_id`, binary `remove_len` — candidates for the W3 camelCase sweep.
- Taxonomy: the new fixture/test/witness dirs report `directory-kind-unresolved`/`path-too-long`/`mutation-fixture-unpaired`,
  the same classes their existing siblings (`🔤️rewrites-second-slides…`, peers' `🧾️wire-witness`) already report.

## 8. Files

- **Shared stdio test code:** `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}` (`params_are_wire`);
  `🔮️oracles/📃️document/🦀️.rs` (`🔖️PackageWire`, `relationships_part_path` pub, `content_type`);
  `🧪️tests/📚️office-schema-contract/{🟦️.ts,🧬️public-facets/🟦️.ts}`.
- **Crate roots (re-exports):** `🗿️artifacts/{💾️binary,🗜️deflate,🎒️zip,📕️xlsx,📽️pptx}/🦀️.rs`.
- **binary:** `…/✳️any/🧪️tests/🔀️mutate-binary-raw/{🥒️.feature,🦀️.rs}`, `…/✳️any/🔮️oracles/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `…/🧬️mutations/📸️set-snapshot/↩️inverse/🦀️.rs` (unused import).
- **deflate:** `…/✳️any/🧪️tests/🗜️mutate-deflate-rfc1950/{🥒️.feature,🦀️.rs}`, `…/✳️any/🔮️oracles/🦀️.rs`, `…/↩️inverse/🦀️.rs`.
- **zip:** `…/🧱️base/🧪️tests/🔀️mutate-zip-2-0/{🥒️.feature,🦀️.rs}`, `…/🌐️iso21320/🧪️tests/🔀️mutate-zip-2-0-iso21320/{🥒️.feature,🦀️.rs}`,
  `…/{🧱️base,🌐️iso21320}/🔮️oracles/🦀️.rs`, `…/🌐️iso21320/🔮️oracles/🧪️tests/🔬️live-unit/🦀️.rs`,
  `…/{🧱️base,🌐️iso21320}/🧬️schema/🧬️mutations/{🏷️rename-entry,💬set-archive-comment}/🦀️.rs`, both `🏷️rename-entry/🧬️schema/🔣️.json`,
  `➕add-entry`/`📦add-stored-entry`/`🗜️add-deflated-entry` `🧬️schema/🔣️.json`, `…/🌐️iso21320/🧬️schema/🧬️mutations/🦀️.rs`,
  `…/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/↩️inverse/🦀️.rs`.
- **xlsx:** `…/🧱️base/🧪️tests/🔀️mutate-xlsx-ecma-376/{🥒️.feature,🦀️.rs}`, `…/🧱️base/🔮️oracles/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `…/🧱️base/🧬️schema/🧬️mutations/🏷️rename-sheet/{🦀️.rs,🧬️schema/🔣️.json}`; for `🔒️strict` and `🌉️transitional`: `🧪️tests/*/{🥒️.feature,🦀️.rs}`,
  `🔮️oracles/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `🧬️schema/🧬️mutations/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  new `🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json`.
- **pptx:** `…/🧱️base/🧪️tests/🧱️mutate-pptx-ecma-376/{🥒️.feature,🦀️.rs}`, `…/🧱️base/🔮️oracles/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `…/🧱️base/🧬️schema/🧬️mutations/{🔷insert-shape,🔶remove-shape,✍️set-shape-text,📐set-shape-position}/{🦀️.rs,🧬️schema/🔣️.json}`;
  for `🔒️strict` and `🌉️transitional`: `🧪️tests/*/{🥒️.feature,🦀️.rs}`, `🔮️oracles/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `🧬️schema/🧬️mutations/🦀️.rs`, new `🧫️fixtures/🧬️mutations/📸️set-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json`.
- **semio presentation:** `…/📽️presentation/🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,📝️text/📖️.grammar.semio}`,
  `…/🧬️schema/🧬️mutations/🦀️.rs`, new `…/🧬️mutations/📸️set-snapshot/🧪️tests/🔃️reverses-the-slide-order/🦀️.rs`, new fixture
  `…/🧫️fixtures/🧬️mutations/📸️set-snapshot/🔃️reverses-slide-order/` (5 files),
  `…/🧪️tests/📽️mutate-semio-presentation/🥒️.feature`.
- **Ticket scripts (kept):** `🧪️w2-w-office-align.py`, `🧪️w2-w-office-conformance.py`, `🧪️w2-w-office-xlsx-conformance-authority.py`.

## Session 2 — 2026-10-01 (S2-STDIO-A, WP-2)

Status: **IN PROGRESS**. The authoritative WP-2 record is `📓️w3-stdio-cases-2-report.md` § Session 2.

- §6, oracle crate `cargo test --features oracles --lib`, which session 1 could not run: queued behind the coordinator's CARGO HOLD (rule 26).
- The pptx diff copy of `deserialize_double_option` moves to the stdio contract crate (in progress).
