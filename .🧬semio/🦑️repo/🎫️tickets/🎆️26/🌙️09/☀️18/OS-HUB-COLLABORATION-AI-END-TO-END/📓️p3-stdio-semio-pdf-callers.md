# P3 — stdio-semio PDF callers ported to `PdfSnapshot`

Slice P3 (fleet 3, 2026-09-19). Peer ticket owning the pdf crate: `26/09/18/PDF-ARTIFACT-SPEC-COMPLETE`.
Captures: `🗑️generated/p3-*.txt`.

## 1. Measured state (before any edit)

The "~45 compile errors" figure was already stale when this slice started.

| measurement | command | result |
|---|---|---|
| default features | `cargo check -p semio-s-artifact-stdio-semio` | **green** (44.3 s) — `semio-s-artifact-stdio-pdf` is an OPTIONAL dep, only pulled in by `conversion-document` / `conversion-drawing`, so a default check never saw the break |
| pdf features, lib only | `cargo check -p semio-s-artifact-stdio-semio --features conversion-document,conversion-drawing` | **4 errors** |
| unstaged tree | `git diff --stat` on the crate | 2 files, +12/−12 — a previous (dead) P3 worker had already ported the **drawing** export/import leaves; nothing else |

The 4 remaining lib errors (before my edits):
- `…/📑️document/🚪️io/📥️import/…/📖️pdf/🔖️1.7/✳️any/🦀️.rs:38` ×2 — `E0615` `page.text` is a method now, not a field
- `…/📑️document/🚪️io/📤️export/…/📖️pdf/🔖️1.7/✳️any/🦀️.rs:74` — `E0063` missing 25 of `PdfSnapshot`'s 30 fields in a struct literal
- `…/📑️document/🚪️io/📤️export/…/🦀️.rs:46` — `E0615` `page.text` assignment

Tests were never counted by that check (`cargo check` without `--all-targets`); five further call sites in test modules were broken as well (§3).

## 2. The new `PdfSnapshot` API

Read from the peer's live code, not from the memo:

- `PdfSnapshot` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:1918`) now has **30** fields (pages/fonts/images/forms/ext_g_states/shadings/patterns/color_spaces/properties/outlines/named_destinations/page_labels/embedded_files/output_intents/acro_form/optional_content/page_layout/page_mode/viewer_preferences/open_action/language/mark_info/metadata/document_id/encryption/info/catalog_extra/objects/trailer + schema/declared_version). `Default` (line 1983) already sets `schema: STDIO_PDF17_DOCUMENT_SCHEMA` and `declared_version: "1.7"`, so no caller needs to spell those.
- `PdfPage` no longer carries a `text: String` authoring field. It carries `content: Vec<PdfOp>` — real content-stream operators — plus `media_box` and the other box/annotation/group slots. `PdfPage::text()` (line 1874) *derives* the page's Unicode text from those operators in painting order.
- The supported way to author text pages is `semio_s_artifact_stdio_pdf::io::text_document(&[(width, height, &str)]) -> PdfSnapshot` (`…/🧱️base/🚪️io/🦀️.rs:443`): one page per tuple, Helvetica 12 pt, ≤72 pt margins, line breaking from the font's real metrics, and it registers the font on `snapshot.fonts` when any page has content. Overflow lines are dropped by `PdfTextLayout::show_paragraph`.
- Crate-root aliases `semio_s_artifact_stdio_pdf::schema` / `::io` forward to the 1.7 base modules; `::PdfSnapshot` is the 1.7 type. `standards::v1_4::` (the small `PageDoc` stub) is untouched by the peer.

## 3. Fixes (file:line)

All inside `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/`. No shims, no compat layer, no edit to the pdf crate.

1. `📑️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs`
   - `:44` `make_page()` (which set `page.text`) replaced by `page_texts(blocks) -> Vec<String>` — the PageBreak split, now returning plain text per page.
   - `:74` the 6-field `PdfSnapshot { … }` literal replaced by `Ok(text_document(&pages))` with `PAGE_WIDTH/PAGE_HEIGHT = 612/792`.
   - module doc rewritten: the export now paints real operators via the pdf crate's own layout, and the new honest loss (a group longer than one page drops its overflow lines, because `DocBlock::PageBreak` is the only pagination signal the source carries) is documented.
2. `📑️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs:38` — `page.text.clone()` → `let text = page.text();`; module doc updated to `PdfPage::text()`.
3. `📑️document/🚪️io/📥️import/…/🧪️tests/🔬️unit/🦀️.rs:5` — `sample_pdf()` built two `PdfPage`s by assigning `.text`; now `io::text_document(&[(612,792,"Page one text."), (612,792,"Page two text.")])`.
4. `📑️document/🚪️io/📤️export/…/🧪️tests/🔬️unit/🦀️.rs:18` — `pdf.pages[n].text` → `.text()`, plus a new `assert_eq!(pdf.fonts.len(), 1)` (the font registration is part of the new contract).
5. `📑️document/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs:136` — `pdf_round_trip_is_stable` built its input by field assignment; now `io::text_document(…)`, and the local `PdfPage`/`PdfSnapshot` import is gone.
6. `🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs:43` — the dead worker's version built a `Vec<(f64,f64,String)>` then a borrowed copy then re-stamped `schema`/`declared_version` over `..built`; simplified to one `texts: Vec<String>` + `Ok(text_document(&pages))` (Default already carries the schema id). Stale module doc (“`encode_pdf` only regenerates a content stream FROM `PdfPage.text`”) rewritten with the two real losses.
7. `🖊️drawing/🚪️io/📥️import/🧩️deserializers/…/🦀️.rs:2` — stale module doc quoting the removed `text` authoring field rewritten.
8. `🖊️drawing/🚪️io/📥️import/…/🧪️tests/🔬️unit/🦀️.rs:6` — `PdfPage { media_box: …, text: … }` literal → `io::text_document(&[(200.0, 100.0, "hello semio")])`.
9. `🖊️drawing/🚪️io/📤️export/…/🧪️tests/🔬️unit/🦀️.rs:34,39` — `pdf.pages[0].text` → `.text()` on both the serialized and the `decode_pdf`-round-tripped snapshot.

## 4. Checks and tests (real output)

| # | command | result |
|---|---|---|
| 1 | `cargo check -p semio-s-artifact-stdio-semio --features conversion-document,conversion-drawing --all-targets` | **exit 0, 0 errors** (`p3-check-semio-conv.txt`) |
| 2 | `cargo test -p semio-s-artifact-stdio-semio --features conversion-document,conversion-drawing --lib -- pdf` | **9 passed, 0 failed**, 2570 filtered out (`p3-test-semio-pdf.txt`) |
| 3 | `cargo check -p semio-s-plugin-stdio --features full-artifact-catalog` | **exit 0, 0 errors** (`p3-check-plugin-stdio.txt`) — this is the exact feature set `semio-hub`'s `native-artifact-execution` forwards, i.e. all 13 `conversion-*` features at once |
| 4 | `cargo check -p semio-hub` (default features) | **exit 101, 18 errors — none from pdf or stdio** (`p3-check-hub.txt`); see §5 |
| 5 | `cargo check -p semio-s-artifact-stdio-semio --features component-app-assembly,conversion-document,conversion-drawing --all-targets` | **exit 0, 0 errors** (`p3-check-semio-caa.txt`) |
| 6 | `cargo test -p semio-s-artifact-stdio-semio --features conversion-document,conversion-drawing --lib` (whole suite) | 2579 run, **2531 passed, 47 failed** (`p3-test-semio-lib-full.txt`) — see the baseline below |
| 7 | `cargo test -p semio-s-artifact-stdio-semio --lib` (default features: the pdf leaves are not compiled at all) | 2533 run, **2485 passed, the SAME 47 failed** (`p3-test-semio-lib-default.txt`) |

Checks 6 vs 7 are the honesty gate on the 47 failures: the two runs differ by exactly 46 tests (the pdf leaves' own), **all 46 pass**, and the failing set is byte-identical in both — 32 `brep`, 6 `object`, 3 `kit`, 2 `value`, 1 `base`, 1 `drawing::schema::mutations::binary`, 2 envelope tests. None of them is an io/pdf test and none is in a file this slice touched, so they are pre-existing crate debt, not a regression from this port.

The 9 pdf-filtered tests (check #2) include the two that exercise the new operator lane end to end:
- `drawing::…::export::…::real_byte_round_trip_through_pdf_codec` — serialize → `encode_pdf` → real PDF bytes → `decode_pdf` → `pages[0].text() == "hello\nsemio"`.
- `document::io::component::derived_composition::tests::pdf_round_trip_is_stable` — pdf → semio → pdf → semio is a fixed point.

### Every other dependent of the pdf crate

`grep -rln semio-s-artifact-stdio-pdf --include=Cargo.toml` names 16 dependent crates besides the pdf crate itself. All of them except `stdio-semio` use `semio_s_artifact_stdio_pdf::standards::v1_4::…` (the small `PageDoc` stub the peer did not touch), so none needed porting — confirmed by checking each one:

| crate | command | result |
|---|---|---|
| `semio-s-artifact-writer-writer` | `cargo check -p` | 0 errors |
| `semio-s-artifact-animate-presentation` | `cargo check -p` | 0 errors |
| `semio-s-artifact-vcs-vcs` | `cargo check -p` | 0 errors |
| `semio-s-artifact-note-note` | `cargo check -p` | 0 errors |
| `semio-s-artifact-layout-layout` | `cargo check -p` | 0 errors |
| `semio-s-artifact-draw-drawing` | `cargo check -p` | 0 errors |
| `semio-s-artifact-shooting-shooting` | `cargo check -p` | 0 errors |
| `semio-s-artifact-puzzle-2d` | `cargo check -p … --features component-app-assembly` | 0 errors |
| `semio-s-artifact-trinity-rewriting` | `cargo check -p … --features component-app-assembly` | 0 errors |
| `semio-s-artifact-procedural-generation2d` | `cargo check -p … --features component-app-assembly` | 0 errors |
| `semio-s-artifact-gis-gismap` | via `semio-hub` | **18 errors, unrelated to pdf** — §5 |
| `semio-s-plugin-{stdio,writer,animate,note,layout,shooting}` | `semio-s-plugin-stdio` checked directly (#3); the others only re-export their artifact crate | — |

Note: `writer`, `animate`, `vcs`, `note`, `layout`, `draw` and `shooting` artifact crates **have no `component-app-assembly` feature** (cargo: "the package … does not contain this feature"), so the plain check is the complete check for them — the slice brief's assumption that they take that flag is wrong for these seven.

## 5. Honest gaps

- **`cargo check -p semio-hub` is still red, for a reason outside this slice.** All 18 errors are in `semio-s-artifact-gis-gismap`, in `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧬️schema/🧬️mutations/…`: 6 × `MutationLeaf source authority failed: descriptor owner does not exactly match source owner` and 12 consequent `E0277`. Zero of them mention pdf or stdio; the whole stdio/pdf dependency path compiles (check #3). This looks like the `⚙️config`→`🎚️config` rename drift G3 is auditing, in B3a's crate. **The pdf break is no longer what blocks the hub.**
- The export's overflow behaviour is a genuine behaviour change, not a regression I can avoid: `text_document` lays one block-group out on one page and drops lines that do not fit. Documented in the module doc rather than papered over by inventing page breaks.
- Verified by compile + unit tests only. No runtime hub boot, no live import/export of a real `.pdf` through the running app was done in this slice. The strongest runtime-shaped evidence is the byte round trip through the pdf crate's own `encode_pdf`/`decode_pdf` inside check #2.
- 47 pre-existing `semio-s-artifact-stdio-semio` lib-test failures remain (brep mutation fixtures dominate). Proven pre-existing by the feature-off baseline (§4, checks 6/7) but **not fixed** — they are outside this slice and belong to whoever owns the brep/object/kit mutation fixtures.
- `semio-s-artifact-gis-gismap` was left untouched. Its 18 errors block `semio-hub`; the file paths (`🎚️config`) match the `⚙️config`→`🎚️config` rename G3 is auditing, and the crate is B3a's. Handing it back rather than editing a peer's live crate.
- The pdf crate itself was **not edited** — the peer owns it and is still working in it (30 modified files under `📖️pdf/` at the time of this slice).

## 6. Files changed

Nine files, all under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/` (+74/−72). Two of them (`🖊️drawing` export + import leaf) carried a dead worker's partial port that this slice finished and simplified.

```
📑️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs
📑️document/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🧪️tests/🔬️unit/🦀️.rs
📑️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs
📑️document/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🧪️tests/🔬️unit/🦀️.rs
📑️document/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs
🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs
🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🧪️tests/🔬️unit/🦀️.rs
🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs
🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🧪️tests/🔬️unit/🦀️.rs
```

Plus this report. No `Cargo.toml`, no `launch.json`, no `📜️script.ts` change was needed (no new runnable command). Captures in `🗑️generated/p3-*.txt`.
