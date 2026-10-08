# 📓️ Executor `stdio-pdf` — Report

Scope `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf` (crate `semio-s-artifact-stdio-pdf`, workspace root `✏️s/`). Paths are relative to
`…/📖️pdf/🏅️standards/` (`S/`): `7️⃣1.7/🪆️subsets/🧱️base` is `B17`, `4️⃣1.4/🪆️subsets/🧱️base` is `B14`.

Status: ALL CODE WRITTEN, NOTHING COMPILED OR RUN. Every `cargo check` through the gate died in `semio-framework-value`
(`🌱️value/🗂️ordered/🚦️native/🦀️.rs:19: expected RetainedCloneGrant, found Grant`, a peer's mid-change that the coordinator named), before any
pdf code is reached; the one earlier attempt was SIGKILLed (exit 137) while compiling framework deps. What did run: `rustfmt --check`
parse of all 309 changed/new `.rs` files of the crate (zero parse errors). Every claim below about behaviour is therefore WRITTEN BUT
UNVERIFIED (see "Verification" for the exact command to run first).

## Design decisions (pdf-specific)

1. **Graph edits are a declared application semantic, not a diff-builder computation.** The 76+7 callers of `support::graph_edit_diff` /
   `diff::diff_graph_edit` cloned `base`, edited the clone and diffed it, because an edit of the retained COS graph (`objects`, `trailer`)
   must also move every typed lane the edited graph reads differently (`io::carry_graph_edit`). `PdfDiff.graph_edit: bool` (wire
   `graphEdit`, skipped when false): a kind builds only its sparse `objects`/`trailer` rows and marks them with `diff::graph_edit(rows)`;
   the central `MutationDiff::apply` (the only caller of `carry_graph_edit`) then carries the moved typed lanes. `absorb` ORs the flag
   (cleared when no graph rows remain), `inverse` keeps it, `between` never sets it. Limit: a flagged diff overwrites an explicit typed-lane
   row of the same diff when the graph moved that lane (no kind builds such a diff).
2. **`PdfDiff.info` is field-wise** (`Option<PdfInfoDiff>`, tri-state per field): `set-info-title` / `set-info-author` emit one field.
3. **Concrete `DiffAlgebra::inverse` on both `PdfDiff` types** (1.7 and 1.4), lane by lane from `base`: index triples through one shared
   final-layout function, keyed triples, dict / array / value diffs, objects (sorted), pages, info. The "apply then `between`" inverse is gone.
4. **Central apply only**: `MutationDiff::apply(&self, base, ApplyCapability)`; the dispatch fns call `mutations::apply_outcome` (per
   standard, in the base aggregate), tests go through `protocol::apply_diff`.
5. **Seams deleted** from `B17/🧬️schema/🏅️conformance-support`: `graph_edit_diff`, `insert_object`, `remove_object`, `set_entry`, `remove_entry`,
   `set_catalog_entry`, `remove_catalog_entry`, `set_acro_form_fields`, `set_output_intent`, `insert_file_spec`, `insert_signature_field`,
   `remove_signature_field`, `set_dpart_root`, `set_dpart_job` (all `&mut PdfSnapshot`) and `diff::diff_graph_edit`. Replaced by pure row builders
   returning `PdfDiff` (`Insertion`, `insert_object_rows`, `remove_object_rows`, `set_entry_rows`, `remove_entry_rows`, `set_catalog_entry_rows`,
   `remove_catalog_entry_rows`, `remove_catalog_entry_owned_rows`, `acro_form_rows`, `*_signature_field_rows`, `output_intent_rows`,
   `struct_tree_root_rows`, `insert_file_spec_rows`, `remove_file_spec_rows`, `dpart_root_rows`, `dpart_job_rows`, `embed_font_file_rows`).
6. **Removal kinds drop what the entry exclusively owned** (`support::owned_objects`): remove-struct-tree-root, -output-intent, -dpart-root,
   -embedded-file. `embed-font-file` drops the other `FontFile*` keys.
7. **No whole-document mutations (coordinator ruling b).** `set-snapshot` and `patch-snapshot` are deleted in BOTH standards: leaf dirs
   (schema, text io, binary io, fixtures, facets), aggregate variants, binary tags, the `snapshot_patch_leaf!` use, the 1.4 fixture
   `📸️set-patch-exact-words`. The editors (10 files) call `snapshot_edit_net_exact(event, snapshot, schema::mutations::net_mutations)`, and
   the 1.7 base editor's `import_media` is `import_media_as_load` (import is the genesis/load path, not a mutation).
8. **`net_mutations(base, next) -> Vec<PdfMutation>`** (1.7: `B17/🧬️schema/🧬️mutations/🦀️.rs`; 1.4: `B14/…`) builds the concrete net per lane:
   keyed lanes (objects, trailer, catalog_extra, fonts, images, forms, ext-g-states, shadings, patterns, colour spaces, properties,
   embedded files, named destinations) as removals + in-place sets + sets at final index ascending (a lane whose surviving keys changed
   relative order is rebuilt whole); pages as one `move-page` for a single moved page, else `replace-page` (1.4: `resize-page` /
   `replace-page-text`) per changed page, `remove-page` from the end, `insert-page` at final positions; every whole-value lane as its
   `set-*` leaf. `snapshot_edit_net_exact` replays the leaves through the central applier and rejects an edit a leaf does not address
   (the `schema` / `declaredVersion` markers).
9. **Position-exact inverses (coordinator ruling a).** The inverse of every remove / delete on an ordered collection restores the item at
   its ORIGINAL index (and, for created objects, its original object number); every creating kind carries the optional position
   (absent = append):
   * 7.1.7 base, keyed document lanes — `set-font`, `-image`, `-form`, `-ext-g-state`, `-shading`, `-pattern`, `-color-space`,
     `-properties`, `-embedded-file`, `-named-destination` gained `index: Option<usize>`; `remove-<lane>` inverts to the set kind with
     `index: Some(position)`.
   * 7.1.7 base, raw graph — `insert-object`, `set-object-value`, `set-dict-entry`, `set-trailer-entry`, `set-catalog-entry` gained
     `index`; `remove-object` / `-dict-entry` / `-trailer-entry` / `-catalog-entry` invert with `index: Some(position)` (dict entry
     position within its container at `path`). Page lists, content operators and annotations were already index-explicit.
   * 7.1.7 conformance (79 kinds, six subsets): object-creating kinds (`insert-javascript-action`, `-launch-action`,
     `-encryption-dictionary`, `-media-annotation`, `-embedded-file`, `-signature-field`, `set-struct-tree-root`, `set-output-intent`,
     `set-dpart-root`) gained `placements: Vec<ObjectPlacement { id, index }>` (one per created object, creation order; missing = next
     free number appended); catalog-/dict-entry kinds (`set-mark-info`, `-lang`, `-display-doc-title`, `-trim-box`, `-af-relationship`,
     `-dpart-metadata`, `embed-font-file`, and the entry written by `set-struct-tree-root` / `set-output-intent` / `set-dpart-root` /
     `insert-signature-field`) gained `entry_index`; `insert-signature-field` also `field_index` (position in `/AcroForm/Fields`).
     The remove kinds read ids and positions from `base` (`placements_of`, `entry_position`, `catalog_entry_position`,
     `file_spec_creation_ids`, `output_intent_creation_ids`, `dpart_root_creation_ids` — the creation order of the canonical shape).
10. **`replace-page`** (new concrete kind, 1.7 base, `🪄️`, binary tag 60): whole-page entity replace used by the editor's
    `set_page_extra` / `set_page_transition` (which used to emit `SetSnapshot`); diff = the differing page fields only, inverse =
    `replace-page` of the base page (accepted by ruling a).

## Per-family result (before → after)

Codes: `V1` snapshot/generic diff, `V2` derived/restore/empty inverse, `V3` leaf apply or `&mut` writer, `V4` no law test.
"law test" = `assert_mutation_inverse_sum_law` in the leaf's own `🧪️tests/🔬️unit/🦀️.rs`.

| Family (kinds) | Before | After | Files per kind |
|---|---|---|---|
| 7.1.7 conformance `♿️ua` 11, `⚕️h` 10, `📐️e` 12, `🖨️x` 14, `🗄️a` 14, `🧾️vt` 18 (= 79) | V1+V3+V4 (`graph_edit_diff` of a base clone), restore/empty inverses | rows via `support::*_rows` + `diff::graph_edit`; concrete position-exact inverses (decision 9); law test + middle-row law test on every remove kind, explicit-placement law test on every creating kind; info kinds via one-field `PdfInfoDiff` | leaf `🦀️.rs`, leaf `🧬️schema/🔣️.json`, leaf tests |
| 7.1.7 base raw graph: insert-object, remove-object, set-object-value, set-dict-entry, remove-dict-entry, set-trailer-entry, remove-trailer-entry, set-catalog-entry, remove-catalog-entry (9) | V1+V3 (+ V2-EMPTY insert-object) | `diff::diff_*` rows (+ `graph_edit` for the COS kinds), `index` on the creating kinds, position-exact remove inverses; insert-object refuses a duplicate id | leaf `🦀️.rs`, 5 facets for the 5 gaining kinds, tests |
| 7.1.7 base keyed document lanes (10 set + 10 remove) | clean diff; remove inverse re-added at the END | `index` on the set kinds; remove inverses restore the original index; middle-row law test | leaf `🦀️.rs`, 5 facets (`🔣️.json`, `🟦️.ts`, `🛰️.proto`, `🔗️.graphql`, payload schema), tests |
| 7.1.7 base index-explicit kinds (pages, content, annotations, page boxes, whole-value lanes, set-info) (31) | clean | unchanged diff/inverse; `set-info` field-wise; middle-row law test for remove-page / -annotation / -content | tests |
| 7.1.7 base `replace-page` (new) | n/a | see decision 10 | new leaf + text/binary io leaves + 4 aggregate facets |
| 7.1.7 base `set-snapshot`, `patch-snapshot` | V1+V2-RESTORE / V1+V3 | DELETED (decision 7) | dirs, aggregate, io, fixtures, facets |
| 4.1.4 base insert/remove/move-page, resize-page, replace-page-text (5); `🗄️a` 2; `🖨️x` 2 | clean | unchanged diff/inverse; middle-row law test on insert/remove/move-page; `net_mutations` | tests, aggregate |
| 4.1.4 base `set-snapshot`, `patch-snapshot` | as 1.7 | DELETED | dirs, aggregate, io, fixtures |
| Both standards: `PdfDiff` | generic `between`-based inverse, leaf apply | concrete `inverse`, `apply(&self, base, ApplyCapability)`, `graph_edit`, field-wise `PdfInfoDiff` | `🔺️diff/🦀️.rs` + 4 facets (1.7) |

Totals: 7.1.7 = 61 base + 79 conformance = 140 kinds; 4.1.4 = 5 base + 4 conformance = 9 kinds.

## Verification

Nothing below was run; commands to run in this order once `semio-framework-value` compiles again (each through
`"$T/🚦️gate.sh" stdio-pdf -- …` from `…/📖️pdf/📦️packages/🦀️rust`, foreground):

1. `cargo check -p semio-s-artifact-stdio-pdf --target wasm32-wasip2 --message-format=short` (this is `🗑️generated/stdio-pdf/run.sh`).
2. `cargo test -p semio-s-artifact-stdio-pdf --lib --message-format=short` — the new/changed tests: per-leaf `inverse_diffs_sum_to_the_negative_diff`,
   `inverse_restores_a_middle_row` (38 conformance remove kinds + 10 keyed lanes + remove-page / -annotation / -content / -object /
   -dict-entry / -trailer-entry / -catalog-entry + 1.4 insert/remove/move-page), `inserts_at_the_placements_it_is_given` (25 creating
   conformance kinds), `conformance_support` unit tests (placements, entry positions, creation ids), and
   `the_net_of_an_edit_replays_to_exactly_the_edited_document` in both aggregates.
3. `cargo test -p semio-s-artifact-stdio-pdf --test '*'` for the mutate-pdf-1-7 and lopdf-vector suites.

Done: `rustfmt --edition 2021 --check` parse of all 309 changed/new `.rs` files — zero parse errors (syntax only; no type check).

## Open issues

1. **Nothing compiled.** Expect type/borrow errors in the first compile (new `Insertion`/`ObjectPlacement` API, ~250 struct-literal
   sites patched mechanically, 76 conformance leaves rewritten by script). The struct-literal patcher could have touched a pattern or
   an enum variant of the same name (PdfOp::SetFont etc. were reverted explicitly).
2. **Set-over-existing is not position/id exact for the object-creating conformance kinds.** `set-output-intent` on a document that
   already has an output intent writes NEW intent objects and leaves the old ones orphaned; its inverse (`set-output-intent` with the
   previous identifier) recreates canonical objects at the end, not the old ones. Same for `set-struct-tree-root` / `set-dpart-root`
   over an existing root. The law fixtures cover create-then-remove, remove-in-the-middle and set-over-existing of scalar entries.
3. **Multi-object removals are exact only for the canonical shape the insert builds** (file spec + exactly one payload stream; output
   intent + optional profile; dpart node + root). A shape with extra owned objects inverts with fresh placements (empty `placements`).
4. **`graph_edit` + explicit typed lane in one diff** (decision 1): a flagged diff overwrites an explicit typed-lane row of the same diff
   when the graph moved that lane. `net_mutations` therefore emits the COS graph leaves first, typed lanes after.
5. **Keyed `absorb` is position-approximate** (`PdfKeyedDiff::absorb` cannot transport `added` indices without a base).
6. **`net_mutations` is coarse for pages**: a changed page is one `replace-page` (1.7) rather than the finer `set-page-*` leaves; a
   multi-page reorder other than a single move is a sequence of replacements.
7. **Not regenerated:** `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🔣️.json`, `🧰️framework/…/📚️library/🔣️schema-catalog.{json,md}`, the generated TS
   `dist/🧬️types`, and the schema-first docs of the new `placements` / `entryIndex` / `fieldIndex` / `index` properties beyond the per-leaf payload
   schema (they carry `x-semio-ui` labels in en + de). The emoji `🪄️` of `replace-page` is unique among directories; the repo-wide emoji
   registry was not consulted.
8. **Layout.** Leaves keep diff and inverse inline in the leaf `🦀️.rs` (the pdf taxonomy); no per-leaf `🔺️diff/` / `↩️inverse/` folders were
   split out, so gate rule R15 (which keys on a leaf owning `🔺️diff`) does not see them even though every leaf has the law test.
9. `EmbedFontFile` / `SetAfRelationship` / `Set|RemoveTrimBox` still answer a missing target with an empty diff and no message; unchanged.
10. Shared-machine events: killed my own hung cargos twice earlier (job-queue lock deadlock with 0 rustc children) and one runaway test
    child; no peer processes other than those were touched.
