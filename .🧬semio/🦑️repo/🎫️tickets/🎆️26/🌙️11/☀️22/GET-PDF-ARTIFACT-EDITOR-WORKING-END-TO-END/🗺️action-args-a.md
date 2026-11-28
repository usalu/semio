# Action argument decoding for the PDF 1.7 base page editor (read-only audit)

Nothing was executed and nothing was changed. The destination path `.../GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END/🗺️action-args-a.md` could not be written, so the full content is returned here for you to save. Examples are derived from reading the decoders and are not run. Where I could not confirm something, it is marked **unknown**.

## Legend (file keys used in citations)

Base directory `B` = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base`

| Key | Absolute path | Contents read |
|---|---|---|
| ACT | `B/✏️editor/🖼️page/🎬️actions/🦀️.rs` | `edit_from_action` 7-82 (arg tuple per action), `apply_payload` 95-205, `emit_page_edit` 208-232 |
| ARG | `B/✏️editor/🖼️page/🧰️arguments/🦀️.rs` | `text_arg` 8, `req_text` 12, `committed_argument` 23, `req_page` 39, `opt_page` 44, `req_num` 48, `opt_num` 57, `parse_pdf_number` 81, `typed_locale` 96 |
| WK | `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🪟️window-kit/🦀️.rs` | `window_kit_text_argument` 54, `required_text` 96, `required_index` 110, `window_kit_document_text_edit` 426 |
| MODEL | `B/✏️editor/🖼️page/🧱️model/🦀️.rs` | `PAGE_ACTIONS` 13-104, `is_page_action` 184 |
| DOC | `B/✏️editor/📄️document/🦀️.rs` | `set_outline` 25, `set_page_rotation` 40, `set_page_box` 49, `page_box_kind` 63, `set_page_boundary` 74, `set_page_user_unit` 97, `page_layout` 103, `page_mode` 116, `resolve_page` 140, `set_page_label` 190, `set_mark_info` 210, `set_info_field` 215, `set_viewer_preference` 239, `set_open_action` 317, `set_document_id` 328 |
| OBJ | `B/✏️editor/🖼️page/🖱️object-edits/🦀️.rs` | `payload_edit` 6, `edit_object` 10 (set-text 35, set-font-size 41, delete 47, move 49, resize 50, set-fill 51, set-stroke 52, fallback 53), `insert_object` 58 (text 63, rectangle 86, line 87, image 94), `replace_image_samples` 115, `set_font_of_text` 144, `font_for_base` 169 |
| CONTENT | `B/✏️editor/🖼️page/🧮️content-ops/🦀️.rs` | `ensure_font` 6 (Helvetica fallback), `delete_paint` 53, `translate_object` 72, `resize_object` 82, `set_fill` 122, `set_stroke` 134 |
| AUTH | `B/✏️editor/🖼️page/✍️authoring/🦀️.rs` | `parse_page_selection` 374, `decode_source` 396, `import_pages` 645, `retarget_pages` 601, `insert_page_map` 613, `remove_page_map` 617, `extract_effects` 1027, `pick_source_effect` 1036, `merge_document` 1049, `replace_page_with` 1056 |
| ANNOT | `B/✏️editor/🖼️page/🔖️annotation-edits/🦀️.rs` | `set_page_transition` 417 (Di written as Name at 426), `annotation_at` 6 |
| DOCEDIT | `B/✏️editor/🖼️page/📄️document-edits/🦀️.rs` | `set_page_extra` 86-88 (`box.*` routes to `set_page_boundary`) |
| PNG | `B/✏️editor/📄️document/🖼️image-authoring/📥️admission.rs` | `png_invocation` 65-88, `png_actions` 91-120, `PngTarget::placement` 48 |
| LOCAL | `B/✏️editor/📥️local-image/🦀️.rs` | `from_action` 17, `native_work` 30, `command_id` 44 |
| CTRL | `B/✏️editor/🦀️.rs` | controller 201, `command_from_action` 231-246 (set-page 235, PNG branch 239, edit branch 240-243) |
| KIT17 | `B/✏️editor/🧰️kit/🧾️pdf17/🦀️.rs` | `command_from_action` 158-172 (no PNG branch); reused via `pdf17_editor!` by subsets `🗄️a`, `📐️e`, `⚕️h` (`/editor/🦀️.rs` line 6 of each) |
| PAGETEXT | `B/✏️editor/🖼️page/📝️page-text/🦀️.rs` | `page_text_agent_revision` 32, `page_text_edit_mutations` 41, inline tests 62+ |
| WIN | `B/✏️editor/🖼️page/🪟️windows/🦀️.rs` | action declarations 64-146 (rail arguments) |
| VIEW | `B/✏️editor/🖼️page/🔭️view/🔢️page-number/🦀️.rs` | `page_index` 13-14 |
| GEOM | `B/✏️editor/🖼️page/🧭️geometry/🦀️.rs` | object id format 12 (annotations), 216-222 (objects) |
| FAULTS | `B/✏️editor/📄️document/🚨️faults/🦀️.rs` | `PdfFault` codes 10-105 |
| RESRC | `B/✏️editor/🖼️page/🎨️resource-edits/🦀️.rs` | `image_color_space` 355-364 |
| LOW_PAGE | `B/🔨️modules/⬆️lower/🧭️pages-and-navigation/🦀️.rs` | page keys 10-29 (MediaBox, CropBox, BleedBox, TrimBox, ArtBox, Rotate, UserUnit, Trans, Dur) |
| LOW_CAT | `B/🔨️modules/⬆️lower/📚️catalog-and-info/🦀️.rs` | catalog keys 40-120 (PageLabels 52, PageLayout 73, PageMode 81, ViewerPreferences 82-99, OpenAction 105, Lang 107, MarkInfo 115, Metadata 117) |
| TEST_PAGE | `B/✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs` | Rust unit tests (line numbers cited) |
| TEST_AUTH | `B/✏️editor/🖼️page/🧪️tests/🔬️unit/✍️authoring/🦀️.rs` | authoring tests |
| TEST_EDITOR | `B/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | controller-level tests (command_from_action, shell picker) |

## General calling conventions (verified in code)

1. **Page numbers are one-based.** `req_page` (ARG:39) reads the number, then `resolve_page` → `page_index` (VIEW:13-14) maps `n` to `n-1` and refuses `0` or `> count`. Insertion-type actions accept `count+1` through `page_slots` (ACT:90-92): `insert-page`, `merge-document`, `merge-pick`. `move-page`'s `x` is also a one-based destination (ACT:136).
2. **Object ids use a zero-based page index.** Format is `p{page0}:{t|v|i|f|s}{ordinal}` for content objects and `p{page0}:a{index}` for annotations (GEOM:220 and GEOM:12). Example: `p1:t0` is the first text object on the **second** page. The first page is `p0:t0`. Ordinals count per kind per page in discovery order.
3. **Argument JSON types are enforced differently per helper.**
   - `req_page` requires a JSON **number** (WK:110-122). A string such as `"2"` is refused, except for `move-page`, which falls back to parsing a string (ACT:19).
   - `req_text` (for `object` and `text` in the actions that use it) requires a JSON **string** (WK:96-106). An empty string passes.
   - `req_num` and `opt_num` accept a number or a numeric string (ARG:48-58).
   - `text_arg` is lenient: a missing key returns `""` (ARG:8-10). Several actions depend on this (see Notes N3).
4. **Field/value convention (inspector and rail).** If the request contains `field: "<name>"`, the argument named `<name>` is read from `value` (ARG:19-34, `committed_argument`). Example: `{page:1, object:"p0:t0", field:"size", value:18}` (TEST_EDITOR:1644).
5. **Locale-typed numbers.** Only a typed value stamped with `locale` may use a decimal comma (ARG:92-99, `parse_pdf_number` ARG:81-89). Agents should send canonical `.` notation.
6. **Units.** Coordinates are passed straight into PDF operators, so they are PDF default user space (points, origin bottom-left). Colours (`red`, `green`, `blue`) go unchanged into `rg`/`RG` or `/C`. No range clamp was found (not verified).
7. **Non-finite numbers.** `ArgumentNotFinite` (FAULTS:80) is effectively unreachable, because `parse_pdf_number` filters out non-finite values first (ARG:88). A non-finite value surfaces as `NumberInvalid`.

## Per-action table

Abbreviations: "obj" = object id, "1-based" = page number. "Fault" lists the code where one is attached and the message otherwise. Codes come from FAULTS:10-105 (`stdio.pdf.*`); plain `fault(...)` calls use `stdio.pdf.page-edit`.

| # | Action | Arg meaning (page/object/text/extra/x/y/width/height/red/green/blue/size) | PDF structure written (reader check) | Example arg set (blank 612x792, unexecuted) | Faults on invalid input | Decoded at | Rust test |
|---|---|---|---|---|---|---|---|
| 1 | set-page | `page` 1-based Number; `item` must be `0` (Number); `revision` String (hash of current page text, from `page_text_agent_revision`); `text` String (new page Unicode text) | Text-only edit of page content (Tj/TJ). No single key. Check with pymupdf `page.get_text()`. | `{page:1, item:0, revision:"<token from agent path>", text:"Hello"}` | `stdio.pdf.set-page.invalid-item` (item != 0); `stdio.pdf.set-page.stale-target`; `stdio.pdf.set-page.conflict`; window-kit `argument-*` codes | CTRL:235-237; PAGETEXT:41-60 | Yes: TEST_EDITOR:1310, 1321, 1345-1359; inline PAGETEXT tests 62+ |
| 2 | set-text | `page`; `object` String = text obj `p{n}:t{k}` (or annotation `p{n}:a{k}`); `text` String (required) | Text run string replaced (glyph check). Annotation: `/Contents`. | `{page:1, object:"p0:t0", text:"World"}` after insert-text | `stdio.pdf.glyphs-missing` (font cannot show text); "pdf object '…' is not on the page"; "unsupported PDF annotation edit"; `stdio.pdf.content-preserved` | ACT:21; OBJ:35-40 | Yes: TEST_PAGE:128, 138, 691, 700, 1412; TEST_EDITOR:1964 |
| 3 | move | `page`; `object`; `x`,`y` = **new absolute** bbox origin (not a delta) | Text: text matrix moved. Image/Form: `cm` translated. Vector: path coordinates rewritten. Annotation: `/Rect=[x,y,x+w,y+h]`. Shading: `edit_shading`. Check with pymupdf `get_drawings()` / `get_text("dict")`. | `{page:1, object:"p0:v0", x:100, y:500}` | Not found: object not on page, content preserved | ACT:22; OBJ:49, 10-28; CONTENT:72-80 | Yes: TEST_PAGE:164, 183-185, 271, 276, 360, 483, 693, 938, 1232, 1310, 1861; native-page-edit:109; paint-fidelity:37 |
| 4 | resize | `page`; `object`; `x`,`y`,`width`,`height` = absolute bbox | Vector/Image/Form: `cm` scale+translate (CONTENT:82-98). Text: `resize_text` (**unknown**, not read). Annotation: `/Rect` only when width>0 and height>0. | `{page:1, object:"p0:v0", x:72, y:500, width:300, height:100}` | Annotation with width<=0 → "unsupported PDF annotation edit" | ACT:25; OBJ:50 | Yes: TEST_PAGE:938, 1211, 1293-1294; native:22; `move-text-run.json:15` |
| 5 | delete | `page`; `object` | Text: `delete_text` (**unknown**, not read). Vector: path ops removed, or EndPath when clipped. Image/Form/Shading: paint op removed. Annotation: `/Annots` entry removed. | `{page:1, object:"p0:v0"}` | content-preserved (`stdio.pdf.content-preserved`) when object is not displayed | ACT:23; OBJ:47-48; CONTENT:53-70 | Yes: TEST_PAGE:194, 702, 949, 1067, 1284 |
| 6 | set-fill | `page`; `object` (Text or Vector); `red`,`green`,`blue` (Number, required) | Inserts `rg` before the paint op (restores prior colour after). Annotation: `/C=[r,g,b]`. | `{page:1, object:"p0:t0", red:1, green:0, blue:0}` | Image/Form/Shading → misleading "unknown pdf page action 'set-fill'" (OBJ:51, 53) | ACT:27; OBJ:51; CONTENT:122-132 | Yes: TEST_PAGE:1145 |
| 7 | set-stroke | `page`; `object` (Vector only); `width` = line width (required); `red`,`green`,`blue` (required) | Inserts `w` and `RG`; FillStroke conversion when needed. | `{page:1, object:"p0:v0", width:2, red:0, green:0, blue:1}` | Non-vector → misleading unknown-action fault (OBJ:52, 53) | ACT:28; OBJ:52; CONTENT:134 | Yes: TEST_PAGE:1163 |
| 8 | insert-text | `page`; `text` String (required); `x`,`y` (required); `height` = font size (**required by dispatcher**, `<=0` → 12); `red`,`green`,`blue` optional (default 0 = black) | Appends `BT /Fx size Tf rg Tm Tj ET` to page content. Newlines use leading 1.2×size (`TL`, `T*`). Adds a `/Font` resource: Helvetica (`F1`…) when no admitted font shows the text (CONTENT:6-15). Check with pymupdf `get_text()`. | `{page:1, text:"Hello", x:72, y:700, height:24}` | "no admitted PDF font can show the requested text" | ACT:29, 147; OBJ:58-85; CONTENT:6-15 | Yes: TEST_PAGE:203, 1980, 1998 |
| 9 | insert-rectangle | `page`; `x`,`y`,`width`,`height` (required); `red`,`green`,`blue` optional (black) | Appends `rg x y w h re f`. Check with pymupdf `get_drawings()`. | `{page:1, x:72, y:600, width:200, height:50, red:1, green:0.8, blue:0}` | Missing or non-numeric arg → `stdio.pdf.argument-missing` / `stdio.pdf.number-invalid` | ACT:26; OBJ:86 | Yes: TEST_PAGE:204 |
| 10 | insert-line | `page`; `x`,`y` = start; `width`,`height` = **delta** to end point (end = `x+width`,`y+height`); colours optional; stroke width fixed at 1.0 | Appends `RG w m l S` | `{page:1, x:72, y:500, width:200, height:0}` | Missing or non-numeric → argument faults | ACT:26; OBJ:87-93 | **No test found by id** |
| 11 | set-image | `page`; `object` (must be an Image object, e.g. `p0:i0`); `text` = samples: decimal 0-255 separated by commas or whitespace (**not hex**, despite the param name); `width`,`height` = **pixel** dimensions (rounded, min 1) | Replaces XObject (or inline) samples. `/Width /Height /BitsPerComponent 8 /ColorSpace /DeviceRGB` or `/DeviceGray`, inferred from byte count = w·h·3 or w·h. | `{page:1, object:"p0:i0", text:"255,0,0, 0,255,0, 0,0,255, 255,255,0", width:2, height:2}` **requires an existing image object; a blank doc has none.** | "image samples are N bytes; expected P gray or 3P rgb"; "pdf object '…' is not an image"; "image samples must be logical values from 0 to 255" | ACT:33; OBJ:115-141; RESRC:355-364 | Yes: TEST_PAGE:239, 891 |
| 12 | insert-page | `page` = slot 1..=len+1 (insert **before** that number; len+1 appends); `width`,`height` (required, min 1) | Adds page to page tree with `/MediaBox [0 0 w h]`. Retargets links and form widgets after the index (AUTH:601-617). | `{page:2, width:612, height:792}` on a 1-page doc (appends) | "pdf page N does not exist, valid pages are 1..M" (`stdio.pdf.page-number-out-of-range`) for out-of-range | ACT:30, 125-129; ACT:90-92; AUTH:601-617 | Yes: TEST_AUTH:380, 502; TEST_PAGE:221, 1396, 2011, 2013 |
| 13 | remove-page | `page` 1..=len | Removes page, retargets links and fields (remove_page_map). Removing the last page: **unknown**. | `{page:2}` on a 2-page doc | `stdio.pdf.page-number-out-of-range` | ACT:24, 130-134; AUTH:617 | Yes: TEST_AUTH:382, 492, 501, 510; TEST_PAGE:1396 |
| 14 | move-page | `page` = source (Number, or numeric String fallback); `x` = destination, one-based, against the current count | Reorders `/Kids`; retargets references (move_page_map AUTH:625). | `{page:2, x:1}` on 2 pages | `x=0` or `x>len` → out-of-range | ACT:19, 135-140; AUTH:625 | Yes: TEST_AUTH:381, 470, 481, 507; TEST_PAGE:230, 677, 1396, 2015 |
| 15 | set-page-size | `page`; `width`,`height` (min 1) | `/MediaBox=[left,bottom,left+w,bottom+h]` (keeps existing origin). CropBox and other boxes **not touched** (see N11). Check with pymupdf `page.mediabox`. | `{page:1, width:595, height:842}` | Range fault only | ACT:30, 141-146 | Yes: TEST_PAGE:228, 720, 723; native-page-edit:23; TEST_AUTH field-controls:15 |
| 16 | set-info | No page. `object` → `/Subject`, `text` → `/Title`, `extra` → `/Author`. An empty argument **keeps** the existing value. | Info dict entries (pymupdf `doc.metadata`) | `{text:"Report", extra:"Example Author"}` | None | ACT:31, 112-124 | Yes: TEST_PAGE:217, 725 |
| 17 | set-info-field | No page. `object` = field name: `title`, `author`, `subject`, `keywords`, `creator`, `producer`, `trapped`, `creationDate`, `modificationDate`. `text` = value (empty clears). Dates `D:YYYYMMDDHHmmSS`. | Info dict key | `{object:"keywords", text:"alpha, beta"}` | "unknown info field 'x'"; "a document date needs D:YYYYMMDDHHmmSS" | ACT:46, 166; DOC:215-237 | Yes: TEST_PAGE:493-494, 663, 721, 814 |
| 18 | set-font | `page`; `object` = text obj; `text` = base font name (standard 14: `Helvetica`, `Times-Roman`, `Courier-BoldOblique`, `Symbol`, `ZapfDingbats`, …) or the BaseFont of an existing doc font | Adds a `/Font` resource with `/BaseFont`; run's `Tf` rebound; glyphs re-encoded (rebind_show). Check with pymupdf `page.get_fonts()`. | `{page:1, object:"p0:t0", text:"Times-Roman"}` | "a text run needs a font name"; "pdf object '…' is not text"; "the PDF font is not admitted"; "the selected PDF font cannot preserve the shown text" | ACT:34, 149; OBJ:144-176 | Yes: TEST_PAGE:286-290, 701-706, 1255; standard names in `B/🔨️modules/🔤️fonts/🅰️tables/🦀️.rs` |
| 19 | set-font-size | `page`; `object` (Text); `size` (Number, required, >0) | `Tf` size changed (restyle_text). Check with pymupdf `get_text("dict")` span size. | `{page:1, object:"p0:t0", size:18}` or `{page:1, object:"p0:t0", field:"size", value:18}` | "a text run needs an active font and a positive finite size"; non-text → misleading unknown-action fault (OBJ:41-53) | ACT:20; OBJ:41-46 | Yes: TEST_EDITOR:1644 (field/value form). No page-level unit test by id. |
| 20 | set-outline | `page` = destination, 1-based; `text` = title (required); `x` = item index (0-based); `-1` (default) or past the end → **append** | Outline item `/Title` plus `/Dest [page /Fit]` (lowering of `/Outlines` **not read**). Check with pymupdf `doc.get_toc()` / pypdf `reader.outline`. | `{page:1, text:"Chapter 1", x:-1}` | "an outline needs a title" | ACT:35, 150; DOC:25-38 | Yes: TEST_PAGE:293, 295, 607 |
| 21 | set-page-rotation | `page`; `x` = degrees (rounded; must be 0/90/180/270) | `/Rotate` integer on page; omitted when 0. Check with pymupdf `page.rotation` / pypdf `page.rotation`. | `{page:1, x:90}` | "page rotation must be 0, 90, 180, or 270" (`-90` refused, not normalized) | ACT:36, 151; DOC:40-47; LOW_PAGE:16-18 | Yes: TEST_PAGE:303, 1397 |
| 22 | set-page-box | `page`; **`text` = kind** (`crop`, `bleed`, `trim`, `art`; the decoder reads it from the `object` slot, which holds the `text` value, see ACT:37); `x`,`y`,`width`,`height`: rect `[x,y,x+w,y+h]`; `width=height=0` clears the box | `/CropBox`, `/BleedBox`, `/TrimBox`, `/ArtBox`. Check with pymupdf `page.cropbox` etc. **No `media` kind** (see N4). | `{page:1, text:"crop", x:36, y:36, width:540, height:720}`; clear with `x:0,y:0,width:0,height:0` | "page box must be crop, bleed, trim, or art"; "a page box needs a positive width and height" | ACT:37, 152; DOC:49-71; LOW_PAGE:11-15 | Yes: TEST_PAGE:305 |
| 23 | set-page-user-unit | `page`; `x` = unit (`<=0` removes) | `/UserUnit` number on page. pypdf: `page.get("/UserUnit")`. | `{page:1, x:2}` | None | ACT:36, 153; DOC:97-101; LOW_PAGE:19 | Yes: TEST_PAGE:307; native-page-edit:79 |
| 24 | set-language | `text` (lenient: missing or empty removes) | Catalog `/Lang` (text string). No BCP-47 validation seen. pypdf: `reader.trailer["/Root"]["/Lang"]`. | `{text:"de-CH"}` | None. **Omitted `text` silently deletes `/Lang`** (N3). | ACT:38, 154; LOW_CAT:107 | Yes: TEST_PAGE:309 |
| 25 | set-page-layout | `text` ∈ `singlePage`, `oneColumn`, `twoColumnLeft`, `twoColumnRight`, `twoPageLeft`, `twoPageRight`; empty removes | Catalog `/PageLayout` name (`/TwoColumnLeft` etc.). pypdf: `reader.page_layout`. | `{text:"twoColumnLeft"}` | "unknown page layout 'x'". Omitted `text` removes (N3). | ACT:38, 155; DOC:103-114; LOW_CAT:73-80 | Yes: TEST_PAGE:311 |
| 26 | set-page-mode | `text` ∈ `useNone`, `useOutlines`, `useThumbs`, `fullScreen`, `useOc`, `useAttachments`; empty removes | Catalog `/PageMode`. pypdf: `reader.page_mode`. | `{text:"useOutlines"}` | "unknown page mode 'x'". Omitted `text` removes (N3). | ACT:38, 156; DOC:116-127; LOW_CAT:81 | Yes: TEST_PAGE:313 |
| 27 | set-page-label | `page` = range start (1-based, required); `text` = style ∈ `""`, `decimal`, `romanUpper`, `romanLower`, `lettersUpper`, `lettersLower`; `extra` = prefix; `x` = start number (default 1, min 1) | Catalog `/PageLabels << /Nums [idx << /S /P /St >>] >>`. `idx` is the **zero-based** page index; `/St` omitted when 1. pymupdf: `doc.get_page_labels()` (**verify**). | `{page:1, text:"lettersLower", extra:"app-", x:1}` | "unknown page label style 'x'" | ACT:43, 162; DOC:190-208; LOW_CAT:40-52 | Yes: TEST_PAGE:333; TEST_AUTH field-controls:63 |
| 28 | set-mark-info | No page. `x` = Marked, `y` = UserProperties, `width` = Suspects; each `>=0.5` means true | Catalog `/MarkInfo << /Marked true … >>`; removed when all false | `{x:1}` | None | ACT:44, 163; DOC:210-213; LOW_CAT:115 | Yes: TEST_PAGE:336 |
| 29 | set-metadata | `text` = whole XMP packet (lenient: missing or empty removes) | Catalog `/Metadata` XML stream (`/Type /Metadata /Subtype /XML`). pypdf: `reader.xmp_metadata`. No XML validation seen. | `{text:"<x:xmpmeta …>…</x:xmpmeta>"}` | None. **Omitted `text` silently deletes XMP** (N3). | ACT:38, 164; LOW_CAT:117-119 | Yes: TEST_PAGE:338 |
| 30 | set-viewer-preferences | `object` = flag: `hideToolbar`, `hideMenubar`, `hideWindowUi`, `fitWindow`, `centerWindow`, `displayDocTitle`, `pickTrayByPdfSize`, `numCopies`, `nonFullScreenPageMode`, `printPageRange`, `direction`, `viewArea`, `viewClip`, `printArea`, `printClip`, `printScaling`, `duplex`. `x` = value (**required even for text-only keys**; flags `>=0.5` = true; `numCopies` = round(x) when `>=0`). `text` = mode, list (`1,2`) or name. | Catalog `/ViewerPreferences` dict entries. Check with pypdf `reader.trailer` Root. | `{object:"fitWindow", x:1}`; `{object:"printPageRange", text:"1,2", x:0}`; `{object:"numCopies", x:2}` | "unknown viewer preference 'x'"; "a print range needs whole page numbers"; "unknown page mode" | ACT:45, 165; DOC:239-263; LOW_CAT:82-99 | Yes: TEST_PAGE:340, 515-517 |
| 31 | set-page-transition | `page`; `text` = `/S` style name (e.g. `Split`, `Blinds`, `Box`, `Wipe`, `Dissolve`, `Glitter`, `R`, `Fly`, `Push`, `Cover`, `Uncover`, `Fade`; not validated); `extra` = `/Di` (**written as a Name**, see N1); `x` = `/Dur` seconds (`<0` removes, `0` leaves unchanged, `>0` sets) | Page `/Trans << /S /… >>` and page `/Dur`. Check with pypdf `page.get("/Trans")`, `page.get("/Dur")`. | `{page:1, text:"Fade", x:1.5}` | "pdf page is gone" when page missing | ACT:67, 189; ANNOT:417-436; LOW_PAGE:28-29 | Yes: TEST_PAGE:422 (Di value not asserted) |
| 32 | set-open-action | `object` = kind: `""` (none), `uri`, `page`; `text` = URI (required non-empty for `uri`); `page` = one-based destination for `page` kind (**defaults to page 1 when omitted**, N8) | Catalog `/OpenAction [pageRef /Fit]` or `<< /S /URI /URI (…) >>` (URI form lowering **not read**). | `{object:"page", page:1}`; `{object:"uri", text:"https://example.org"}`; clear: `{object:""}` | "an open uri needs an address"; "unknown open action 'x'"; page out of range | ACT:58, 179; DOC:317-326; LOW_CAT:105 | Yes: TEST_PAGE:379 |
| 33 | set-document-id | `text` = permanent id string (**bytes are the UTF-8 of the string, not hex**, N12); `extra` = changing id. Empty permanent → changing used for both. Both empty → `/ID` removed. | Trailer `/ID [<perm> <change>]`. Encoding in the lowered trailer **not read**. pypdf: `reader.trailer["/ID"]`. | `{text:"abc123", extra:"def456"}` | None | ACT:57, 180; DOC:328-337 | Yes: TEST_PAGE:382 |
| 34 | insert-image (base controller) | `page` (1-based, Number, required); `x`,`y`,`width`,`height` required, width and height `>0`. Routed to the **PNG file picker**, not a placeholder (CTRL:239). | After picker: `cm [w 0 0 h x y]` + image XObject with the decoded PNG (`Insert` placement, PNG:48-52). Check with pymupdf `page.get_images()`. | `{page:1, x:72, y:500, width:200, height:200}` **opens a file picker; not scriptable headless** | "Image width and height must be positive"; "Choose a PNG within the local image import limits"; "Invalid PDF image operator"; `stdio.pdf.image-placement-invalid` | PNG:65-88, 91-120; CTRL:239; LOCAL:17-27 | Picker flow: TEST_EDITOR:2232, 2247. Placeholder path only: TEST_PAGE:205. |
| 35 | insert-image (subsets a, e, h via KIT17) | Same args. Routed to `edit_from_action` placeholder (KIT17:158-172 has no PNG branch). | 8×8 grey RGB placeholder image, `cm` sized `w,h` at `x,y` (OBJ:94-99). **Not the user's image** (N2). | `{page:1, x:72, y:500, width:200, height:200}` | `argument-*` codes | ACT:30; OBJ:94-99 | TEST_PAGE:205 |
| 36 | replace-image | Base controller only. `page` (Number); `operator` (Number, content-op index of the image draw; `u32::MAX` reserved for insert); `text` = revision guard (**min 16 chars**, required). Opens the picker. | Replaces the draw at `operator` with the picked PNG on the import step (see #37). | `{page:1, operator:<n>, text:"<revision ≥16 chars>"}` | "Invalid PDF image operator" when operator == `u32::MAX` | PNG:65-71; `replace-button.rs:16-22`; `admission.rs:110` | **No test found by id.** Revision token source: picker work (**unknown**, not read). |
| 37 | import-local-image | Host callback only, not an agent action (`in_palette(false)`, PNG:110-118). `page`; `operator`; `revision` (String, exactly 64 chars); `payload` (String: `data:image/png;base64,…` or `data:application/octet-stream;base64,…` containing PNG, size-capped); `name` optional; `x`,`y`,`width`,`height`. | Image XObject with decoded PNG samples. `ImportImage` retained job (LOCAL:17-27, 53-65). | Not scriptable without the host picker. | "Choose a PNG within the local image import limits" | PNG:65-88; LOCAL:17-27, 53-65 | **Partial**: TEST_EDITOR:2215 asserts only the import action name. No end-to-end import test found. |
| 38 | extract-pages | No page. `extra` = selections separated by `;`, each `1-3,5` (1-based, at least one group required). **No mutation.** Emits one download (PDF) per group. | Separate PDF per group. File-name pattern: `extract_file_name` (**not read**). | `{extra:"1"}` on 1 page (`"1-2;3"` needs 3 pages) | "extract needs a page selection"; "page range 'x' is outside the N pages"; "'x' is not a page range"; "a page selection needs at least one page" | ACT:77, 222-226; AUTH:374-389, 1027-1033 | Yes: TEST_AUTH:318, 330, 336 |
| 39 | replace-page | `page` 1..=len (Number); `payload` String (`data:application/pdf;base64,…` or raw base64; **empty → blank page**); `extra` = source page in payload (1-based; default first page; **multi-page refused**, test:298); `width`,`height` used only when payload is empty (default: existing page size) | `ReplacePage` at index: the old page's content and annotations are **dropped**; imported page resources copied (AUTH:1056-1065, 645-659). | Blank: `{page:1, width:612, height:792}`. From file: `{page:1, payload:"data:application/pdf;base64,…", extra:"1"}` | "the picked pdf is not readable"; "the picked pdf is not a valid pdf"; "page range … outside"; "pdf page N does not exist" | ACT:75, 200; AUTH:374-401, 1056-1065 | Yes: TEST_AUTH:291-298, 336 |
| 40 | replace-page-pick | `page` 1..=len; `extra` = optional source selection. **Emits `RequestFileOpen`** (accept `application/pdf,.pdf`, read dataUrl) that re-dispatches `replace-page` with `{page, extra}` (AUTH:1036-1046). No PDF mutation itself. | None | `{page:1, extra:"1"}` (then the host must supply `payload`; **host injection not verified**, N13) | As `replace-page` | ACT:76, 215-221; AUTH:1036-1046 | Yes: TEST_AUTH:323, 336 |
| 41 | merge-document | `page` = insert-before position 1..=len+1; `payload` String (required, min 1, same forms as #39); `extra` = source pages (default all) | `ImportPlan` inserts picked pages at index, copies resources, retargets references (AUTH:645-659, 601-611) | `{page:2, payload:"data:application/pdf;base64,…", extra:"1-2"}` | As `replace-page`; `page=len+1` allowed | ACT:75, 201; AUTH:645-659, 1049-1053 | Yes: TEST_AUTH:244, 258, 265-267, 278, 323, 336 |
| 42 | merge-pick | `page` 1..=len+1; `extra` optional. **Emits `RequestFileOpen`** that re-dispatches `merge-document` with `{page, extra}` (AUTH:1036-1046). | None | `{page:2}` | As `merge-document` | ACT:76, 215-221 | Yes: TEST_AUTH:323, 336 |

## Notes and defects found

**N1. set-page-transition writes `/Di` as a PDF name (defect).** `ANNOT:426` builds `PdfObject::Name(direction)`, so `extra:"90"` yields `/Di /90`. ISO 32000-1 defines `Di` as a **number** (0, 90, 180, 270, 315), or the name `/None` for Fly. A name value is invalid per spec. Confirm with `qpdf --check` (not run). Fix: parse `direction` as a number and emit a Number.

**N2. insert-image behaves differently between subsets (defect, inconsistent).** The base controller (`CTRL:239`) sends `insert-image` to the PNG picker. The kit macro (`KIT17:158-172`, used by subsets a, e, h) falls through to the `edit_from_action` placeholder: an 8×8 grey image (`OBJ:94-99`). A user on subset a, e, or h gets an unrelated grey square, not the image they asked for. This is silent content corruption if reached. Fix: one shared dispatch for the image path.

**N3. Omitted text silently deletes entries (defect).** `set-language`, `set-page-layout`, `set-page-mode` and `set-metadata` read `text` with the lenient `text_arg` (`ACT:38`). A missing argument therefore removes `/Lang`, `/PageLayout`, `/PageMode` or the whole XMP stream without an error. Fix: use `req_text`, or require an explicit clear value.

**N4. Inconsistent page-box API.** `set-page-box` has no `media` kind (`DOC:63-71`). `set_page_boundary` does handle `media` (`DOC:74-94`), but it is reachable only through `set-page-extra` with `object: "box.media"` (`DOCEDIT:86-88`). That route takes four coordinates (`left bottom right top`) in `text`, while `set-page-box` takes `x,y,width,height`. Two conventions for one concept.

**N5. Misleading faults for the wrong object kind.** `set-font-size` on a non-text object, and `set-fill` or `set-stroke` on an incompatible object, fall through to `other => "unknown pdf page action '<action>'"` (`OBJ:53`). The action is known; the object kind is wrong. Fix: return a typed refusal naming the object kind.

**N6. set-info cannot clear fields.** `ACT:112-124` only assigns non-empty values, so an empty argument keeps the old value. `set-info-field` (`DOC:215-230`) can clear. Asymmetric. Document it or make both clearable.

**N7. set-viewer-preferences requires a dummy `x`.** `ACT:45` uses `req_num(args, "x")` for every key, including text-only ones such as `direction`, `printArea`, `duplex`. Callers must send `x:0`. Make `x` optional for text-only keys.

**N8. set-open-action silently targets page 1.** With kind `page` and no `page` argument, `opt_page` yields `None`, and `apply_payload` uses index 0 (`ACT:97-100`). Refuse instead.

**N9. set-outline out-of-range index appends silently.** `DOC:30-36`: any `x` past the end appends. Probably intended, but undocumented.

**N10. ArgumentNotFinite is unreachable.** `parse_pdf_number` filters non-finite values (`ARG:88`), so `NumberInvalid` is returned first (`ARG:53`). Minor: `FAULTS:80` never fires.

**N11. set-page-size and other boxes (unverified).** `set-page-size` changes only `/MediaBox` (`ACT:141-146`). I found no clamp of CropBox, BleedBox, TrimBox or ArtBox to the new MediaBox. Possible box outside the media box. Not verified by execution.

**N12. set-document-id uses raw UTF-8 bytes.** `DOC:332-334` uses the string's bytes directly. Acceptable for an ID, but it is not hex-encoded, and the trailer encoding was not read.

**N13. Host payload injection for picker re-dispatch (unknown).** `pick_source_effect` (`AUTH:1036-1046`) sends `{page, extra}` with `read_as: "dataUrl"`. The `payload` key that `replace-page`/`merge-document` read (`ACT:75`) must be injected by the host. I searched `/plugins` for `RequestFileOpen`/`import_action` and found only emitters, not the host side. **Unknown** whether the key is `payload` in the host.

**N14. Page number versus object id base.** Action `page` is one-based. Object id prefix `p{n}` is zero-based (`GEOM:220`). The rail and agents must translate. Document this, or make object ids one-based.

**N15. set-image parameter name.** The sample bytes come in `text`, and the decoder names them `hex` (`OBJ:115`), but the format is decimal 0-255. Rename for clarity.

**N16. insert-text height requirement.** `height` is required by the dispatcher (`ACT:29`) but defaults to 12 when `<=0` (`OBJ:68`). Make it optional, or drop the fallback.

**N17. set-mark-info uses positional names.** `x`, `y`, `width` stand for Marked, UserProperties and Suspects (`ACT:44`). Unclear to agents.

**N18. set-page-user-unit removes silently** for `x<=0` (`DOC:99`). Intentional but silent.

**N19. set-page-rotation rejects -90 and 450** (`DOC:43-45`), instead of normalizing. Design choice, document it.

**N20. replace-page with empty payload drops page content and annotations** (`AUTH:1058-1060` plus `ReplacePage`). Destructive by design, but silent.

## Test coverage gaps (by action)

- No Rust test by id for **insert-line**.
- No Rust test by id for **replace-image**.
- **import-local-image**: only the import action name is asserted (`TEST_EDITOR:2215`). No end-to-end PNG import test found.
- **set-font-size**: only the field/value form is tested (`TEST_EDITOR:1644`). No page-level unit test by id.
- **set-page**: command-level tests exist (`TEST_EDITOR:1310-1359`). PAGETEXT inline tests cover the function directly.
- **set-page-transition**: tested (`TEST_PAGE:422`), but the `/Di` value is not asserted, so N1 is not caught.
- No test exercises the subsets a, e, h `insert-image` placeholder path (N2).
- Feature file `B/🧪️tests/📑️mutate-pdf-1-7/🥒️.feature` exists. **Not inspected in this pass.**

## Unknowns (not read or not verified)

- Host-side `RequestFileOpen` handling and the payload key (N13).
- Picker work: revision token generation, operator semantics for `replace-image` (`PngPickerWork`, not read).
- `resize_text`, `delete_text`, `translate_text`, `restyle_text` internals.
- Lowering of `/Outlines`, the URI form of `/OpenAction`, `/ID` encoding, and `/Metadata` details beyond the cited lines.
- Behaviour of `remove-page` on the last remaining page.
- Colour range validation (none found).
- Output file names for `extract-pages` (`extract_file_name`, not read).
- Whether CropBox and related boxes are clamped on `set-page-size` (N11).
- Whether any example in the table was executed. **None were.** All examples are derived from decoder reading only.

## Example sequence on a blank 612×792 document (unexecuted)

1. `insert-text` `{page:1, text:"Hello", x:72, y:700, height:24}` → object `p0:t0`.
2. `insert-rectangle` `{page:1, x:72, y:600, width:200, height:50, red:1, green:0.8, blue:0}` → vector `p0:v0`.
3. `set-text` `{page:1, object:"p0:t0", text:"World"}`.
4. `set-font` `{page:1, object:"p0:t0", text:"Times-Roman"}`.
5. `set-font-size` `{page:1, object:"p0:t0", size:18}`.
6. `move` `{page:1, object:"p0:v0", x:100, y:500}`.
7. `set-stroke` `{page:1, object:"p0:v0", width:2, red:0, green:0, blue:1}`.
8. `insert-page` `{page:2, width:612, height:792}`, then `move-page` `{page:2, x:1}`.
9. `set-page-box` `{page:1, text:"crop", x:36, y:36, width:540, height:720}`.
10. `set-page-rotation` `{page:1, x:90}`; `set-page-user-unit` `{page:1, x:2}`.
11. `set-language` `{text:"de-CH"}`; `set-page-layout` `{text:"twoColumnLeft"}`; `set-page-mode` `{text:"useOutlines"}`.
12. `set-page-label` `{page:1, text:"lettersLower", extra:"app-", x:1}`.
13. `set-outline` `{page:1, text:"Chapter 1", x:-1}`.
14. `set-open-action` `{object:"page", page:1}`.
15. `set-info` `{text:"Report", extra:"Example Author"}`; `set-info-field` `{object:"keywords", text:"alpha, beta"}`.
16. `set-viewer-preferences` `{object:"fitWindow", x:1}`.
17. `set-page-transition` `{page:1, text:"Fade", x:1.5}`.
18. `set-document-id` `{text:"abc123", extra:"def456"}`.
19. `set-metadata` `{text:"<x:xmpmeta …>…</x:xmpmeta>"}`.
20. `extract-pages` `{extra:"1"}`.

Steps 1-19 are expected to work on the base controller. Steps that need a pre-existing image (set-image) or a host picker (insert-image, replace-image, import-local-image, replace-page-pick, merge-pick) are not in this sequence.