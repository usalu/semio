# 🅰️ Batch A (T07): txt, md, json, xml, html

Root `A` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`. `A/🔣️.json` is the collection manifest of the artifacts directory, not an artifact. `json` is `🧾️json`.

## Phase 1 findings

Architecture (matches coordinator fact): editors are per-subset surfaces `A/<kind>/🏅️standards/<std>/🪆️subsets/<subset>/✏️editor` (plus `👁️viewer`), mounted by the stdio family component (`🌎️hub/🧩️compositions/🗄️stdio`), reached via `os.open-artifact` on the explicit coordinate. They are not generic textareas.

| Artifact | Surface(s) | Window kit | Edit commands | Natural file codec | Native codec |
| --- | --- | --- | --- | --- | --- |
| txt | utf-8/any | text | SpliceText (revision-guarded range splices; one `splice-text` mutation, inverse restores), snapshot-edit details (set-line, insert-line, remove-line, set-line-ending, set-trailing-newline) | `.txt` `text/plain` | yes |
| md | commonmark/any | text | SpliceText, snapshot edits (insert/remove/replace-block, set-inlines) | `.md` `text/markdown` | yes |
| json | rfc8259/base, i-json | tree | SetNode (revision-guarded subtree source), snapshot edits (set-member, remove-member, insert/remove-array-element, set-scalar) | `.json` `application/json` | yes |
| xml | 1.0/base, valid | tree | SetNode, snapshot edits (insert/remove-element, set-attribute, set-text, set-doctype, set-declaration) | `.xml` `application/xml` | yes |
| html | 5/any | document | LoadText (whole-document DSL apply, a document load, not a mutation), snapshot edits (insert/remove-node, set-attribute, set-comment, set-doctype, set-element-name, set-raw-text, set-text) | `.html` `text/html` | yes |

All five are in the natural-file roster (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json`: md, html, txt, xml base and valid, json base and i-json). Existing unit tests per surface already cover: window declaration, op-text round trip, example picker, shell action pair, kit verb through its retained factory, natural-file export + reopen with a third-party oracle (txt: bstr, md: comrak, json: serde_json, xml: quick-xml, html: html5ever), and revision-guarded/invalid SetNode (json).

Open observations (no code change needed from me, reported for owners):
- html `textEdit` previously applied the whole DSL envelope as `Effect::LoadDocument` (no undo). Fixed in phase 2b below.
- `📜️artifact-definition.json` `support_ledger.read/write` still say `unimplemented` for html (and similar) while codecs are `implemented`; this is generated metadata (registry/T01, T19).
- No invalid-input natural-file fixtures existed for txt/md/json/xml/html decode; added below.

## Phase 2 changes (this worker)

Added unit tests (test-first, third-party cross-check where applicable) after the existing natural-file test in each surface's `🧪️tests/🔬️unit/🦀️.rs`:
- txt: `bom_and_leading_whitespace_survive_the_dsl_and_natural_file_round_trips`, `natural_file_import_refuses_invalid_utf8`.
- md: same two names for CommonMark.
- json: `natural_file_import_refuses_malformed_json_and_invalid_utf8` (serde_json must agree each input is invalid), `leading_whitespace_and_nested_values_survive_the_dsl_round_trip` (serde_json equality of exported bytes).
- xml: `natural_file_import_refuses_malformed_xml_and_invalid_utf8`, `leading_whitespace_text_survives_the_dsl_round_trip`.
- html: `natural_file_import_refuses_invalid_utf8_and_keeps_padded_text_through_the_dsl`.

These answer the coordinator's wrap_text note (BOM and leading whitespace preserved by DSL printing). Files:
`A/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `A/📝️md/…/🔖️commonmark/🪆️subsets/✳️any/…`, `A/🧾️json/…/🔖️rfc8259/🪆️subsets/🧱️base/…`, `A/📰️xml/…/🔖️1.0/🪆️subsets/🧱️base/…`, `A/🌐️html/…/🔖️5/🪆️subsets/✳️any/…` (same relative tail `✏️editor/🧪️tests/🔬️unit/🦀️.rs`).

## Test status

Each artifact is its own cargo workspace, so one `-p` list cannot span them. One queued job at a time via `🅰️batch-a-chain.sh` (shared store, results in `🗑️generated/a-<pkg>.log` and `a-chain.log`):
txt `semio-s-artifact-stdio-txt-tests`, md `semio-s-artifact-stdio-md-tests`, json `semio-s-artifact-stdio-json-mutations`, xml `semio-s-artifact-stdio-xml-mutations`, html `semio-s-artifact-stdio-html-mutations`, each `--lib editor`.
Results: pending in the capacity queue at the time of writing (load above 30). The first (isolated) attempt was cancelled per the coordinator rule. New tests are uncompiled until these finish; assertions on malformed JSON/XML inputs (for example `[1,]`, empty XML) are assumptions about decoder strictness and may need adjusting to the real decoder behavior.

## Heavy queue / shared contract

- Needs the heavy queue: the five cargo test runs above; browser journeys `stdio-txt|md|json|xml|html` editor playgrounds (T06).
- No shared-contract change required for batch A. Optional: undoable text apply for html (see above).

## Phase 2b (coordinator review changes)

A = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, H = `A/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/`, J = `A/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/`.

### html: reversible splice source edit (replaces destructive LoadText)
- New mutation leaf `splice-source` (`SpliceSource { splices: [{offset, delete, insert}] }`, scalar offsets into `write_html_document(base)`): diff re-parses the edited source and emits `HtmlDiff { doctype, root: Replace }`; an unparsable source is a fatal outcome (editor fault `stdio.html.invalid-source`, document untouched); inverse is one splice over the whole edited source that restores the base source. Files: `H/🧬️schema/🧬️mutations/✂️splice-source/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`, aggregate `H/🧬️schema/🧬️mutations/{🦀️.rs,🛰️.proto,🔣️.json,🔗️.graphql,🟦️.ts}`, text codec `H/🚪️io/📝️text/🧬️mutations/{🦀️.rs,📖️.grammar.semio}`, binary tag 9 in `H/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio`.
- Editor: `HtmlEditCommand::LoadText` replaced by `SpliceText { revision, splices }`; kit verb lane is now `Artifact`; `html_emit(command, snapshot, canonical_revision)` revision-guards (`stdio.html.stale-text-edit`), no-ops on empty/identical change sets; window uses `TextWindowKit::render_editable_by_splices` with the revision. Files: `H/✏️editor/🦀️.rs`, `H/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs`.
- Oracle: `H/🔮️oracles/🦀️.rs` implements splice-source via html5ever (apply ranges to the oracle's serialization, reparse), catalog `H/🔮️oracles/🔣️.json` (kinds + manifest), sweep case in `H/🔮️oracles/🧪️tests/🔬️oracles-unit/🦀️.rs`.
- Tests: language-agnostic corpus `H/🧪️tests/🧫️fixtures/✂️splice-source/🔣️.json` (edit text with non-ASCII, add attribute, multi-range rename, malformed source refused), consumed by `H/✏️editor/🧪️tests/🔬️unit/🦀️.rs` with html5ever cross-check and inverse check; kit-verb law (one Apply = one history edit labelled en/de, one undo restores); invalid source refused with the document unchanged; stale/empty/identical/out-of-range change sets; op-text/binary round trip and KINDS order in `H/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`.
- Scaffold cleanup: removed the "scaffolded by W1b" markers in `H/🚪️io/🦀️.rs` and `H/🚪️io/🟦️.ts` (the `<!DOCTYPE html` sniff is a real case-insensitive, whitespace-tolerant byte inspection; TS twin `sniffRealBytes` added); `H/🚪️io/💾️binary/🔺️diff/📡️.protocol.semio` now describes the real frame (one opaque UTF-8 `print_diff` payload) instead of a magic byte.
- Not done: no html5lib-generated applied-before/after fixture pair for `splice-source` (html5lib is not installed here; the oracle manifest entry lists the kind, the real-document differential scenario in `🥒️.feature` is not extended because offsets are relative to each side's own serialization). Generated catalogs (`native-codec-projection`, `composition-prepare`) are untouched and may need regeneration via T06.

### json i-json: I-JSON verbs reachable from the editor
- The i-json editor's mutation type is now `JsonIJsonMutation` (was the base `JsonMutation`). New resolver `J/🛜️i-json/✏️editor/🧭️edit-rules/🦀️.rs` (reuses the base locator, now `pub(crate)` in `J/🧱️base/✏️editor/🧭️edit-rules/🦀️.rs`): member insert -> `upsert-member`, member remove -> `remove-member`, key rename -> single `rename-member` (position kept, duplicate refused), element insert/remove -> `insert-array-element`/`remove-array-element`, string write -> `set-string`, number write -> `set-safe-number`, root write -> `set-top-level` (a scalar root is refused per RFC 7493 s2.1), other whole-value writes -> `upsert-member` or element remove-then-insert. Move goes through the same replace path.
- Editor file `J/🛜️i-json/✏️editor/🦀️.rs`; tests `J/🛜️i-json/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (scalar roots refused, `details_edits_raise_the_i_json_leaves` including duplicate-rename refusal).
- Definition `A/🧾️json/📜️artifact-definition.json` now declares the ten json mutation kinds (`s.stdio.json.mutation.<kind>.v1`, status implemented, no new executable registration) in `mutations` and `support_ledger.mutations` instead of `[]`.

### Test status
Chain runs (shared store, one at a time): `🅰️batch-a-chain.sh` (txt, md, json base, xml, html) then `🅰️batch-a-chain2.sh` (html again with the latest edits, i-json `semio-s-artifact-stdio-json-mutations-2`). Logs `🗑️generated/a-*.log`, `a-chain.log`, `a2-chain.log`. All uncompiled until they finish; edits were made without compiler feedback, so expect fix-ups.
