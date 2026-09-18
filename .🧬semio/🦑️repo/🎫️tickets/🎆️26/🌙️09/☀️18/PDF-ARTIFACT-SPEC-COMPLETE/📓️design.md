# 📖️ PDF artifact — spec-complete design

Goal: `s.stdio.pdf` (crate `semio-s-artifact-stdio-pdf`) becomes a full ISO 32000-1 typed artifact —
vectors, text, fonts, images, colour, transparency, annotations, outlines, metadata — importable
(COS → typed), exportable (typed → COS → bytes), editable over mutations, and the ONLY PDF writer any
plugin uses (draw, layout, note, document, raster, presentation, …).

## 1. Where we start (2026-09-18 survey)

- 1.7 base: real COS lexer/parser, xref (classic/stream/hybrid/brute force), filters (Flate/AHx/A85/RL +
  predictors), page-tree inheritance, text extraction, minimal writer. `PdfPage = {media_box, crop_box,
  rotate, text}` — **text only**; `encode_pdf` regenerates a `BT … Tj ET` stream from `text`.
- 1.4: separate flat `PageDoc {width,height,text}` model; reuses 1.7's COS layer.
- Subsets a/x/e/ua/vt/h layer conformance checks and their own mutation vocabularies over the retained
  `objects`/`trailer` COS graph — the graph lane MUST stay.
- Every plugin that exports PDF hand-paints bytes (draw: 620-line painter; layout: 4.5k-line streaming
  job with a CIDFontType2 embed) or writes text-only pages (note, document, drawing, writer, …); raster
  refuses outright.
- Baseline: `cargo test -p semio-s-artifact-stdio-pdf --lib` = 389 pass / 20 fail (simple-font text decoded
  as UTF-16 → "䡥汬漠卥浩"; missing oracle catalogs; absorb law; thesis byte-preservation).

## 2. Architecture

Three layers, one direction each, with a fixed-point law between them:

```
bytes ──decode──▶ COS graph (objects/trailer)  ──lift──▶ typed model (pages/fonts/images/…)
bytes ◀──write──  COS graph                     ◀──lower── typed model
law:  lift(lower(t)) == t          (typed lane survives a write/read round trip)
law:  write(decode(b)) preserves every retained object (lossless carrier, existing thesis law)
```

- **COS layer** (exists, extended): `PdfObject`, lexer, xref, object streams, filters (+LZW, DCT/JPX/
  CCITT/JBIG2 retained as encoded), standard security handler (RC4 40/128, AESV2, AESV3 — decrypt on read
  with empty/user password, encrypt on write when `encryption` is set), writer (classic xref; xref
  stream + object streams when `declared_version >= 1.5` and the snapshot asks for it).
- **Typed model** (new, `🧬️schema/📸️snapshot`): see §3.
- **lift/lower** (new, `🚪️io` sub-modules): one emitter used both for fresh documents (`objects` empty)
  and for reconciling an edited typed lane back into a retained graph (replace that lane's objects,
  keep everything else).
- **Streaming writer**: `PdfObjectWriter` emits header / one object at a time / xref+trailer so a guest
  job (layout) can write a page per step without buffering the document.

Engine sub-modules (taxonomy: shared code in `🔨️modules`; 1.4 reuses 1.7's engine as it already reuses
its COS grammar):

```
🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/
  🔤️lexer        COS tokens/objects ⇄ bytes (moved out of 🚪️io)
  🗜️filters      Flate (stdio deflate), LZW, AHx, A85, RL, predictors, encoded-image passthrough
  🔗️xref         classic/stream/hybrid xref, object streams, brute-force scan, resolver
  🔐️encryption   MD5, RC4, AES-128/256-CBC, SHA-256/384/512, standard security handler R2–R6
  🖋️content      content-stream operators ⇄ bytes (all ISO 32000-1 Table 51 operators, inline images)
  🔤️fonts        standard-14 AFM metrics + encodings (Standard/WinAnsi/MacRoman/PDFDoc/Symbol/ZapfDingbats
                 + AGL), 🔠️truetype (sfnt parser: head/hhea/hmtx/maxp/cmap 0/4/6/12/loca/glyf/post/OS2/name,
                 glyf subsetter), 🗺️cmap (embedded CMap parse/print, Identity, ToUnicode bfchar/bfrange)
  🖼️images       image XObject lift/lower (bpc 1/2/4/8/16, colour spaces, masks, SMask, decode, DCT/JPX passthrough)
  🎨️colour       colour spaces, functions (0/2/3/4), shadings 1–7, patterns
  ⬇️lift         COS graph → typed model (catalog, page tree, resources, annots, outlines, names, …)
  ⬆️lower        typed model → COS objects (+ reconcile into a retained graph)
```

## 3. Typed model (ISO 32000-1 coverage)

```
PdfSnapshot {
  schema, declared_version,
  pages: Vec<PdfPage>,
  fonts: Vec<PdfFont>,               // id-keyed document resources (shared across pages)
  images: Vec<PdfImage>,
  forms: Vec<PdfFormXObject>,
  colour_spaces / functions / shadings / patterns / ext_g_states: Vec<…>   // id-keyed
  outlines: Vec<PdfOutlineItem>,     // tree (children nested)
  named_destinations: Vec<PdfNamedDestination>,
  page_labels: Vec<PdfPageLabelRange>,
  embedded_files: Vec<PdfEmbeddedFile>,
  acro_form: Option<PdfAcroForm>,    // fields tree (text/button/choice/signature) + DA/DR/NeedAppearances
  optional_content: Option<PdfOptionalContent>,   // OCGs + default config
  page_layout, page_mode, viewer_preferences, open_action, language, tagged (MarkInfo), metadata (XMP)
  info: PdfInfo (typed dates), document_id, encryption: Option<PdfEncryption>,
  catalog_extra: Vec<PdfDictEntry>,  // unknown catalog keys, lossless
  objects, trailer                   // retained COS carrier (subsets a/x/… read it)
}
PdfPage { media_box, crop_box, bleed_box, trim_box, art_box, rotate, user_unit, content: Vec<PdfOp>,
          resources: PdfResources (name → id maps), annotations: Vec<PdfAnnotation>, group, extra }
PdfOp — one variant per operator (73), typed operands; text as PdfTextString { Unicode text | raw codes }
        + TJ arrays; inline images typed.
PdfFont { id, kind: Standard14 | TrueType | Type1 | Type3 | Type0(CID Type0/2) , encoding, widths,
          descriptor, embedded program (FontFile/2/3), cmap (Identity/embedded), to_unicode, extra }
PdfImage { id, width, height, colour_space, bpc, samples: Raw | Dct | Jpx | Ccitt | Jbig2, image_mask,
           soft_mask, mask (colour key | stencil id), decode, interpolate, intent, extra }
PdfAnnotation { subtype-typed enum (25 subtypes) + common fields, appearance streams by form id, actions }
```

Numbers in the typed lane are `f64`, written with a canonical exponent-free formatter so parse∘print is
identity for finite values; COS keeps `PdfDecimal` for lossless retained values.

## 4. Mutations (1.7 base vocabulary)

Existing 16 stay (the COS-level ones keep operating on `objects`/`trailer`). New, all with diff + inverse:
page boxes/user-unit, `SetPageContent(ops)`, `InsertContentOp`, `RemoveContentOp`, `ReplaceContentOp`,
`AppendContentOps`, resource put/remove (font, image, form, ext-g-state, shading, pattern, colour space,
page resource name bindings), annotation insert/remove/set/move, outline insert/remove/set, named
destination set/remove, page label set/remove, embedded file insert/remove, acro-form field set/remove,
metadata set, viewer-preferences set, page layout/mode set, open-action set, language set, encryption
set/remove, document-id set. `AppendPageContent`/`SetPageContent` become typed-op mutations (the text
convenience becomes a builder helper `add_text` that lowers to ops with a real font).

Diff: `PdfPageDiff` gains index-keyed triples for `content` and `annotations`, tri-states for boxes; new
id-keyed triples for every document collection; rich payloads ride the schema-derived value codec
(`store::pack_rt::encode_wire_value`) inside the hand-rolled frame instead of per-field hand codecs.

## 5. Consumers

| plugin | today | target |
|---|---|---|
| draw | own painter → bytes | `DrawingSnapshot → PdfSnapshot` typed (paths, shadings, ext-g-states, images, text via standard-14/embedded) → `encode_pdf` |
| layout | own streaming job | job lowers pages through the artifact's streaming writer with its TTF as `PdfFont::TrueType` CID font |
| note / document / writer / drawing(semio) | text-only pages | real text layout ops with standard-14 metrics (line breaking by AFM widths) |
| raster | refuses | one page, one image XObject |
| presentation / gis / procedural / puzzle / playbook / shooting / trinity | JSON coercion / stubs | honest typed mappings |

## 6. Validation

- Unit laws: lift∘lower fixed point per feature; parse∘print for every operator; font encode/decode;
  TrueType subset re-parse; encryption known-answer vectors (RC4/AES/MD5/SHA); lopdf independent reads via
  the existing oracle host; PyMuPDF renders exported PDFs (ink probes) for draw/layout/raster.
- Third-party oracles: lopdf/pdf-writer (Rust, registered), PyMuPDF + pdftotext (host tools) for exports.
- Gates: `cargo check/test -p semio-s-artifact-stdio-pdf`, wasm32-wasip2 build of stdio plugin, dependent
  plugin crates check, `bun ./📜️script.ts schema verify`, `verify dependencies`, browser export probes
  for draw (6064) and layout (6079).
