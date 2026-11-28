# PDF page-action argument map (PDF 1.7 base subset)

Read-only research. Nothing was built, run, or edited. Every "example" below is derived from code and existing test bodies, not executed. Third-party checks are listed but were not run.

## Files (absolute paths)

- BASE = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base`
- ACT = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🎬️actions/🦀️.rs`. Decode table at lines 7-82; apply at 95-205; emit at 208-232.
- ARG = `.../✏️editor/🖼️page/🧰️arguments/🦀️.rs` (`text_arg` 8, `req_text` 12, `req_page` 39, `req_num` 48, `opt_num` 57, `parse_pdf_number` 81)
- ANN = `.../✏️editor/🖼️page/🔖️annotation-edits/🦀️.rs` (annotation, form-field, style and kind handlers)
- DOC = `.../✏️editor/📄️document/🦀️.rs` (`set_form_field` 297, `set_encryption` 265, `set_output_intent` 279, `set_optional_content` 156, `set_embedded_file` 172, `set_named_destination` 183, `set_page_extra` in DDE)
- DDE = `.../✏️editor/🖼️page/📄️document-edits/🦀️.rs` (`set_font_program` 6, `set_image_mask` 32, `set_form_content` 65, `set_page_extra` 86)
- RES = `.../✏️editor/🖼️page/🎨️resource-edits/🦀️.rs` (glyph, indirect object, mesh, ExtGState, pattern, colour space, properties, font metrics, `remove_named`)
- OBJ = `.../✏️editor/🖼️page/🖱️object-edits/🦀️.rs` (`payload_edit` 6, `edit_annotation` 106)
- AUTH = `.../✏️editor/🖼️page/✍️authoring/🦀️.rs` (`create_annotation` 83, `markup_form` 141, `fill_form_field` 278, `set_entry` 1077, `quads_of` 58, `ink_of` 70, `rect_of` 51)
- FAULT = `.../✏️editor/📄️document/🚨️faults/🦀️.rs` (`PdfFault` codes)
- PAGENUM = `.../✏️editor/🖼️page/🔭️view/🔢️page-number/🦀️.rs` (`page_index` 13)
- PTESTS = `.../✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs`
- ATESTS = `.../✏️editor/🖼️page/🧪️tests/🔬️unit/✍️authoring/🦀️.rs`
- NPTESTS = `.../✏️editor/🖼️page/🧪️tests/🔬️unit/📝️native-page-edit/🦀️.rs`
- ETESTS = `.../✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`qpdf_accepts` 1401; `create-annotation` 2633; `fill-form-field` 2636)
- FC = `.../✏️editor/🖼️page/🧪️tests/🔬️unit/✍️authoring/🧪️field-controls.json` (UI control groups and inspector bindings)
- WK = `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🪟️window-kit/🦀️.rs` (`text_arg` path 54; required argument 78-93; `window_kit_required_text_argument` 96; `window_kit_required_index_argument` 110)
- Fixtures: `BASE/🧪️tests/🧫️fixtures`

Reading the mapping: `edit_from_action` (ACT:7) builds a JSON payload `{page, object, text, extra, x, y, width, height, red, green, blue}` (OBJ:6-8). `apply_payload` (ACT:95) decodes that payload with the same names. Positional arguments below are the names used in ACT:14-80 and the handler signatures.

## Shared argument rules

- `page`: required via `req_page` (ARG:39) for actions that need it. It goes through `window_kit_required_index_argument`, which accepts only a JSON number (WK:113). A JSON string `"1"` is refused. Optional `opt_page` (ARG:44) accepts strings. Input is one-based; `resolve_page` (PAGENUM:13) does `checked_sub(1)` and refuses 0 or above the page count. The stored value is zero-based.
- `object` and `text` (required form): `req_text` (ARG:12). An absent key or a JSON number or bool is refused. An empty string is accepted (WK:96-101 checks only type and presence).
- `text`/`extra`/`object` (optional form): `text_arg` (ARG:8) never faults. It coerces JSON numbers and bools to strings and returns "" when absent.
- Numeric `x`, `y`, `width`, `height`, `red`, `green`, `blue`: `req_num` (ARG:48) or `opt_num` (ARG:57). Locale rules: `.` is the decimal separator for `en` and for agent calls (`locale` absent). `,` is the separator for `de`. A foreign separator or a repeated separator fails (ARG:81-89).
- Directed UI form: if args contain `field == <name>`, the value is read from `args.value` (ARG:19-34). This is how the inspector commit is shaped. The sender was not read.
- Object ids: annotation `p<zeroBasedPage>:a<index>` (GEOM:12). Note the page prefix is zero-based while the `page` argument is one-based.

## Table 1: argument meaning per action

R = required, O = optional. "Decode at" gives ACT lines unless noted.

| # | Action | page | object | text | extra | numeric args | Decode at |
|---|---|---|---|---|---|---|---|
| 1 | set-annotation | R | R: annotation id `p<zp>:a<i>`, strict prefix match (ANN:6-8) | R: new /Contents (empty string stored as `Some("")`) | - | x,y,width,height all R. Rect becomes `[x,y,x+w,y+h]` only if w>0 and h>0; otherwise the rect is silently unchanged (OBJ:109-111) | 32 / 196; OBJ:106-113 |
| 2 | set-annotation-style | R | R: id. Only the `:aN` suffix is parsed, page prefix ignored (ANN:404) | R aspect: `color`, `flags`, `name`, `state`, `modified`; others fault | O: value for name/state/modified ("" clears) | color uses red/green/blue, value `clear` empties it. `flags` = x, rounded, floored at 0 | 48 / 168; ANN:403-415 |
| 3 | set-annotation-border | R | R: id (strict) | O: style name; `clear` removes border, "" = no style | O: dash csv of numbers | x = line width (default 0). y = radius_x (default -1, negative = no radii). width = radius_y. Naming is inverted; tests confirm (PTESTS:532, 539 expects radii [1,2]) | 49 / 169; ANN:34-42 |
| 4 | set-annotation-markup | R | R: id (strict) | R aspect: `layer`, `structParent`, `entry`, `title`, `subject`, `richContents`, `replyType`, `intent`, `opacity`, `popup`, `inReplyTo`, `creationDate`; others fault | O: value; `entry` takes `key=value` | x = number: structParent (>=0), opacity (clamped 0..1), popup and inReplyTo (object number, not checked), creationDate ignored | 50 / 170; ANN:44-74 |
| 5 | set-annotation-kind | R | R: id (strict) | R aspect: `kind` (value in extra) or a per-kind aspect | O: value. For kind: name (text, link, freeText, line, square, circle, polygon, polyLine, highlight, underline, squiggly, strikeOut, stamp, caret, ink, popup, file, sound, movie, widget, screen, printerMark, trapNet, watermark, threeD, redact). Other aspects use uri, csv quads, `x,y,x,y` points, ink paths `x,y,x,y;x,y,...`, names | Aspect-dependent: x = page (link.page, one-based), flag (open, caption, repeat), number (quadding, leader, popup.parent). x,y,w,h = differences rect. r,g,b = interior (value `clear` = none). Wrong aspect for kind faults | 53 / 173; ANN:177-355 |
| 6 | set-annotation-appearance | R | R: id (only `:aN` suffix checked) | O: form XObject resource name; "" clears /AP | - | none | 68 / 192; ANN:445-450 |
| 7 | create-annotation | R | R = kind: `note`, `highlight`, `underline`, `strikeOut`, `squiggly`, `ink`, `link`. Other values fault | Note and markup: /Contents. Link: URI, or `page:N` with one-based N. Empty link text faults | Markup: quads csv in groups of 8 (empty = rect corners). Ink: strokes `x,y,x,y;...`, each stroke at least two points (AUTH:70-80) | x,y,w,h: positive required for markup and link (AUTH:51-55). Note defaults 20x20. Ink ignores x,y. r,g,b: all zero means default colour, so pure black cannot be requested (AUTH:85) | 73 / 198; AUTH:83-139 |
| 8 | set-form-settings | none | O (text_arg): setting name. Empty faults. Known names: `signatureFlags`, `defaultAppearance`, `quadding`, `defaultFonts`, `entry`. Any other value is a top-level field name | O: value for the setting, or a field sub-aspect (`appearance`, `quadding`, `flags`, `alternate`, `mapping`, `action`, `entry`) | O: secondary value (entry key, field DA, action `key=value`) | x: signatureFlags, quadding, field quadding and flags | 51 / 171; ANN:76-108. Creates /AcroForm if absent (ANN:80) |
| 9 | set-field-data | none | R: exact top-level field name (ANN:370) | R aspect: Choice `options`, `values`, `defaults`, `top`. Text `default`, `maxLength`, `rich`. Button `default`, `options`. Signature `entry` | O: value (options: `export=display,...`; csv for values/defaults; name=value for entry) | x: `top` index, `maxLength` | 54 / 174; ANN:365-401. Requires AcroForm |
| 10 | set-form-field | none | O: field name. Empty removes the whole /AcroForm (DOC:298-299) | O: value (empty = none) | O kind: `""`/`text` (default), `button`, `choice`; other faults | none | 57 / 178; DOC:297-315. Sets NeedAppearances. Replaces kind of existing field; otherwise appends field with `widgets: []` (DOC:312) |
| 11 | fill-form-field | none | R: field name. Dotted hierarchical names resolved (AUTH:193-204) | O: value (text_arg, may be empty) | unused | none | 74 / 199; AUTH:278 |
| 12 | set-form-content | none | R: form XObject id | O: content text (empty = filled rectangle) | unused | x,y origin (0). width,height default 8, min 1 | 66 / 188; DDE:65-84 |
| 13 | set-extra-entry | none | O: owner, `info` or `viewer`. Other faults | O: VALUE (name; empty removes) | O: KEY (empty faults "an entry needs a key") | none | 52 / 172; ANN:110-124. Note extra=key, text=value |
| 14 | set-resource-detail | none | R: resource id. Outline: index as string (`"0"`). Image, ExtGState, font, form, shading: their id. Layers: must be non-empty but unused | R aspect `owner.field`; no dot faults | O: value for csv and name aspects | x,y,w,h used per field (font.bbox, shading.bbox, group knockout = y). r,g,b for outline.color | 55 / 175; RES:70-177. Owners: outline, image, graphics, font, form, shading, layers |
| 15 | set-page-extra | R | O: kind: `box.<name>` (text parsed as canonical quad), `thumbnail`, `metadata`, `structParents`, `group`, `action`, `entry`. Other faults | O: value (name, xmp, `clear`, colour space) | O: key for `action` and `entry` | structParents = x. group: isolated = x>=0.5, knockout = y>=0.5 | 47 / 167; DDE:86-124 |
| 16 | set-optional-content | none | R: OCG id (string, not object number) | R: display name | unused | x = on flag (default 1; >=0.5 on, else off) | 39 / 157; DOC:156-170 |
| 17 | set-embedded-file | none | R: embedded-file id (name-tree key) | O: file name (empty = remove) | O: contents, as UTF-8 bytes of the string only (DOC:180) | none | 40 / 158; DOC:172-181 |
| 18 | remove-embedded-file | none | R: id | - | - | none | 41 / 159 |
| 19 | set-named-destination | R | R: destination name | unused | unused | none (page taken from `page`) | 42 / 160; DOC:183-188 (fit = /Fit) |
| 20 | remove-named-destination | none | R: name | - | - | none | 41 / 161 |
| 21 | set-encryption | none | O: OWNER password (text_arg) | O: algorithm `rc4-40`, `rc4-128`, `aes128`, `aes256`; "" removes encryption; other faults | O: USER password | x = permissions (default -1), rounded to i32, unchecked. encrypt_metadata = true | 56 / 176; DOC:265-277 |
| 22 | set-output-intent | none | O: subtype (e.g. `GTS_PDFA1`). "" clears all intents | R (when subtype set): OutputConditionIdentifier | O: Info text | none. Profile is kept from an existing intent of the same subtype | 57 / 177; DOC:279-295 |
| 23 | set-font-reference | none | R: existing font id, must exist | R: program kind `type1`, `truetype`, `cff`, `cidcff`, `opentype` | O: artifact id (external store, not validated) | none | 59 / 181; DDE:6-30 (`place_font_program` 16) |
| 24 | set-graphics-state | none | R: ExtGState id (created if absent) | O: blend mode name (""=keep; stored as one-element list, unvalidated) | unused | x R = fill alpha, clamped 0..1. y R = stroke alpha, clamped 0..1 | 60 / 182; RES:179-190 |
| 25 | set-pattern | none | R: pattern id | O: shading id. "" = tiling pattern; non-empty must be an existing shading | unused | x,y = tiling bbox origin (0). width,height = bbox and step (default 8; <=0 becomes 8). Ignored for shading patterns | 61 / 183; RES:192-207 |
| 26 | set-color-space | none | R: colour-space resource name | R: `deviceGray`, `deviceRgb`, `deviceCmyk`, `separation`; other faults | R when separation: colorant name | none. Separation alternate fixed to DeviceRgb with fixed tint transform (RES:217) | 62 / 184; RES:209-223 |
| 27 | set-properties | none | R: property list name | R: KEY | O: VALUE. Empty stores empty name, not removal (RES:230) | none | 63 / 185; RES:225-237 |
| 28 | set-font-metrics | none | R: simple font id (Type1 or TrueType only) | O: encoding `winAnsi`, `macRoman`, `macExpert`, `standard` (""=keep) | O: widths csv (""=keep) | x = FirstChar (default -1 keeps; >=0 sets). y = FontDescriptor ascent (0 keeps) | 64 / 186; RES:239-284 |
| 29 | set-image-mask | none | R: image id, or page-object address `p<page>:<...>` that resolves to an image (DDE:54-63) | O: kind: "" clears masks; `stencil`; `soft`; `colorKey` | R for stencil/soft: mask image id (not checked to exist). colorKey: csv of non-negative integers | none | 65 / 187; DDE:32-52 |
| 30 | set-catalog-entry | none | R: catalog key | O: name value (""=remove) | ignored (passed through but unused) | none | 65 / 190; ANN:438-443 |
| 31 | set-trailer-entry | none | R: trailer key | O: name value (""=remove) | ignored | none | 65 / 191; ANN:438-443 |
| 32 | set-glyph | none | R: Type3 font id (others fault, RES:20) | R: glyph (CharProc) name | unused | x,y origin (0). width,height default 10. Content is a rectangle + fill, not a glyph outline | 69 / 193; RES:6-23 |
| 33 | set-indirect-object | none | ignored (empty) | O: value as PDF name ("" = remove existing, else faults "pdf object is gone") | unused | x R = object number (u32). y = generation (default 0) | 70 / 194; RES:25-36 |
| 34 | set-mesh-reference | none | R: mesh shading id (must be a Mesh, RES:40) | O: decode csv (""=keep; length not checked) | O: artifact id for mesh stream ("" = keep) | none | 71 / 195; RES:38-46 |
| 35 | set-entry | none | R: family: `catalog`, `trailer`, `info`, `viewer`. Other faults | O: VALUE (empty removes) | O: KEY. If empty, text is parsed as `key=value` (AUTH:1078-1083) | none | 72 / 197; AUTH:1077-1104. Same extra=key, text=value convention as set-extra-entry |

## Table 2: PDF structure, blank-document example, checks, tests

"Writer key" = how the snapshot is serialized. The writer was not read, so keys marked "spec" are the PDF spec names only and are not confirmed in this codebase.

| Action | Expected PDF structure | Works on blank doc? (by code reading) | Example args | Third-party check (not run) | Rust test refs |
|---|---|---|---|---|---|
| set-annotation | Page /Annots entry; /Contents, /Rect (spec) | No: needs annotation | after create-annotation: `{page:1, object:"p0:a0", text:"Reviewed", x:72, y:700, width:30, height:20}` | pymupdf `page.annots()` contents and rect | PTESTS:225 |
| set-annotation-style | annotation /C, /F, /NM, /AS, /M (spec); writer keys unknown | No: needs annotation | `{page:1, object:"p0:a0", text:"color", red:1, green:0, blue:0}` | annot.colors, annot.flags | PTESTS:509-511 |
| set-annotation-border | /Border array or /BS dict (spec); writer keys unknown | No: needs annotation | `{page:1, object:"p0:a0", text:"S", x:1}` | annot.border | PTESTS:532, 539 |
| set-annotation-markup | /T, /Subj, /CA, /OC, /StructParent (spec); writer unknown | No: needs annotation | `{page:1, object:"p0:a0", text:"title", extra:"Reviewer"}` | annot.info["title"] | PTESTS:533-536 |
| set-annotation-kind | /Subtype changes with kind; link: /A /S /URI; ink: /InkList; line: /L (spec) | No: needs annotation | `{page:1, object:"p0:a0", text:"kind", extra:"link"}` then `{text:"uri", extra:"https://example.test"}` | annot.type | PTESTS:571-589 (asserts snapshot, not bytes) |
| set-annotation-appearance | annotation /AP /N name (spec). Form XObject not checked to exist | No: needs annotation | `{page:1, object:"p0:a0", text:"Mk0_0"}` | widget/annot appearance stream | PTESTS:440 |
| create-annotation | note: /Subtype /Text, /Rect, /F 28, /C, /Contents, /Name /Note. highlight/underline/strikeOut/squiggly: /QuadPoints, /C, /AP -> form `Mk<page0>_<at>`, plus ExtGState "Multiply" for highlight (AUTH:131-136). ink: /InkList, /BS. link: /Subtype /Link, /A (URI) or /Dest (page). Note: the page reference is `text` for links and no appearance is added | Yes | note: `{page:1, object:"note", text:"Review", x:72, y:700, width:20, height:20}` -> id p0:a0. highlight: `{page:1, object:"highlight", x:72, y:700, width:200, height:14}` (default quads). ink: `{page:1, object:"ink", extra:"72,600,172,650;172,650,200,600"}` | pymupdf `page.annots()` types Text, Highlight, Ink, Link | ATESTS:133, 336; ETESTS:2633 (name-level) |
| set-form-settings | /AcroForm: /SigFlags, /DA, /Q, /DR fonts (spec); field-level: /DA /Q /Ff /TU /AA (spec); writer unknown | Yes: creates AcroForm (ANN:80) | `{object:"defaultAppearance", text:"/Helv 12 Tf 0 g"}` | pypdf `reader.trailer["/Root"]["/AcroForm"]` | PTESTS:546-550, 556 |
| set-field-data | /Fields entry: choice /Opt, /V, /DV, /TI; text /MaxLen, /DV; button /Opt; sig /V (spec) | No: needs field (after set-form-field) | `{object:"City", text:"options", extra:"Bern=Bern,Zug=Zug"}` | pymupdf `page.widgets()` field options | PTESTS:595-597 (Choice only; other kinds not tested) |
| set-form-field | /AcroForm /Fields with /FT /Tx|/Btn|/Ch, /T, /V, and /NeedAppearances true (spec). New fields have no widgets (DOC:312) | Yes | `{object:"Name", text:"Ada", extra:"text"}` | pypdf `reader.get_fields()["Name"]["/V"]` | PTESTS:374, 545, 594 |
| fill-form-field | field /V updated. Widget appearances regenerated only for `field.widgets` (AUTH:285), which editing actions never populate (populated only by reader/tests). Blank doc: value only, no visible widget | Partial: value set, no widget | `{object:"Name", text:"Grace"}` after set-form-field | pypdf `reader.get_form_text_fields()`; pymupdf `widgets()` (expected empty on blank doc) | ATESTS:178, 202-203, 274, 336, 419-424; ETESTS:2636 |
| set-form-content | Form XObject with /BBox [x y x+w y+h] and content stream. Not referenced from any page by this action; page /XObject registration unknown | Yes (use empty text): text mode uses font "F1" or first font, which may not exist | `{object:"Fm1", x:72, y:600, width:100, height:20}` | pymupdf page.get_xobjects() (only if registered) | PTESTS:420 |
| set-extra-entry | Info dict key = /Kept (a PDF name, not a text string) (ANN:113-114); viewer: ViewerPreferences key (ANN:117-120) | Yes | `{object:"info", extra:"Custom", text:"Kept"}` | pymupdf `doc.xref_get_key(-1,...)` or raw trailer /Info (metadata shows standard keys only) | PTESTS:557-558 |
| set-resource-detail | outline: /Outlines item /F (bold/italic), /C, /Count (open), /A. image: /Interpolate, /Decode, /Intent, /Matte. graphics: ExtGState /LC /LJ /ML /D /RI /OP /op /FL. font: FontDescriptor /Flags /ItalicAngle /Ascent /Descent /CapHeight /StemV /FontBBox (spec). form: /OC, /StructParent, /Group. shading: /AntiAlias /Background /BBox. layers: OCProperties /D /Name /BaseState (spec). Writer keys unknown | Partial: `graphics` creates ExtGState on demand; `outline` needs set-outline first; `image` needs image; `font`, `form`, `shading` need existing resource | `{object:"0", text:"outline.bold", x:1}` after `set-outline {page:1, x:-1, text:"Chapter"}`. Graphics: `{object:"GS1", text:"graphics.cap", x:1}` | pypdf reader outline / ExtGState lookup | PTESTS:608-620, 712-714 (lines 620-714 not read) |
| set-page-extra | /Thumb (thumbnail), /Metadata (metadata), /StructParents, /Group (isolated, knockout, /CS), /AA (action), page key (entry). Writer unknown | Yes | `{page:1, object:"group", text:"deviceRgb", x:1, y:0}` | pypdf `page["/Group"]` | PTESTS:497-501; NPTESTS:77, 104, 111, 118, 120 |
| set-optional-content | /OCProperties /OCGs [ /Name ] and /D /ON or /OFF (spec). OCG id is a string key, object mapping unknown | Yes | `{object:"OC1", text:"Notes", x:1}` | pymupdf `doc.get_ocgs()`; pypdf trailer /OCProperties | PTESTS:323 |
| set-embedded-file | /Names /EmbeddedFiles name-tree entry keyed by id, /Filespec /F = file name, /EF stream with the UTF-8 bytes (spec) | Yes | `{object:"note.txt", text:"note.txt", extra:"hello"}` | pymupdf `doc.embfile_names()`, `doc.embfile_get()`; pypdf `reader.attachments` | PTESTS:327 |
| remove-embedded-file | removes name-tree entry | No: needs prior embedded file (behaviour when absent unknown) | `{object:"note.txt"}` | embfile_names() | PTESTS:329 |
| set-named-destination | /Names /Dests name-tree entry: name -> [page /Fit] (spec); page is zero-based in model | Yes | `{page:1, object:"intro"}` | pymupdf `doc.resolve_names()`; pypdf `reader.named_destinations` | PTESTS:331 |
| remove-named-destination | removes entry | No: needs prior (behaviour when absent unknown) | `{object:"intro"}` | resolve_names() | PTESTS:347 |
| set-encryption | trailer /Encrypt dict: /V /R /O /U /P, /CF for AES (spec). Permissions = x | Yes | `{object:"owner", text:"aes128", extra:"user"}` (x default -1) | pymupdf `doc.is_encrypted`, `doc.authenticate("user")`; pypdf `reader.is_encrypted`; qpdf --check | PTESTS:342; ETESTS:1401 (helper, not confirmed wired to this action) |
| set-output-intent | catalog /OutputIntents [ /S subtype, /OutputConditionIdentifier, /Info, /DestOutputProfile if profile ] (spec). Profile absent on blank doc | Yes (no profile, so not PDF/A-valid) | `{object:"GTS_PDFA1", text:"sRGB IEC61966-2.1"}` | pypdf `reader.trailer["/Root"]["/OutputIntents"]` | PTESTS:371 |
| set-font-reference | font descriptor embedded program /FontFile, /FontFile2, /FontFile3 (spec), bytes from external artifact | No: needs font and artifact in app store | not runnable on blank doc | pymupdf page.get_fonts() embedded flag | PTESTS:384 |
| set-graphics-state | ExtGState: /ca, /CA, /BM (spec); page /Resources registration unknown | Yes (resource placement unknown) | `{object:"GS1", text:"Multiply", x:0.5, y:0.5}` | pypdf page /Resources /ExtGState (if registered) | PTESTS:394 |
| set-pattern | /Pattern resource: tiling (/PatternType 1, /BBox, /XStep, /YStep, empty content) or shading (/PatternType 2, /Shading) (spec) | Yes (tiling only; shading needs existing shading) | `{object:"P1", x:0, y:0, width:10, height:10}` | pypdf page /Resources /Pattern | PTESTS:398 |
| set-color-space | /ColorSpace resource: name or array (/Separation with fixed tint) (spec) | Yes (placement unknown) | `{object:"CS1", text:"deviceRgb"}` | pypdf page /Resources /ColorSpace | PTESTS:401 |
| set-properties | /Properties resource: name -> dict with key = /value (spec) | Yes | `{object:"MC0", text:"Lang", extra:"en"}` | pypdf page /Resources /Properties | PTESTS:403 |
| set-font-metrics | font dict /FirstChar, /Widths, /Encoding (base name), /FontDescriptor /Ascent (spec) | No on blank doc: needs simple font (insert-text creates one via ensure_font, CONTENT:6-14; font id not verified) | after `insert-text`: `{object:"<fontId>", text:"winAnsi", x:32}` | pymupdf page.get_fonts() | PTESTS:405 |
| set-image-mask | image dict /Mask [ranges] (colorKey), /Mask ref (stencil), /SMask ref (soft) (spec) | Yes after insert-image: id `Im1` (DDE and insert_object `Im{n}`, OBJ:95) | `{object:"Im1", text:"colorKey", extra:"0,0"}` | pymupdf page.get_images(full=True) (smask/mask fields) | PTESTS:417, 856 |
| set-catalog-entry | /Root key = /value (Name) (spec) | Yes | `{object:"Marker", text:"Yes"}` | pymupdf `doc.pdf_catalog()` and xref_get_key; pypdf trailer /Root | PTESTS:426-427, 430-431 |
| set-trailer-entry | trailer key = /value (Name). No validation of standard keys | Yes (with non-standard key only) | `{object:"Foo", text:"Bar"}` | pypdf `reader.trailer` | PTESTS:428-429 |
| set-glyph | Type3 font /CharProcs /<name> stream with rect + fill (spec) | No: no listed action creates a Type3 font | not runnable on blank doc | pymupdf font type Type3 | PTESTS:444-447 |
| set-indirect-object | `N G obj /Name endobj` (spec) | Yes | `{x:7, y:0, text:"Hello"}` | pymupdf `doc.xref_object(7)`; pypdf `reader.get_object(7)` | PTESTS:448-450 |
| set-mesh-reference | shading dict /Decode and stream reference to artifact (spec) | No: needs existing mesh shading | not runnable on blank doc | pypdf shading dict | PTESTS:452-455 |
| set-entry | catalog, trailer, info, viewer key (as in rows 13, 30, 31) | Yes | `{object:"info", extra:"Custom", text:"Kept"}` or `{object:"catalog", text:"Marker=Yes"}` | as rows 13, 30 | ATESTS:522-541 |

## Notes and defects

1. Argument-name inversion in several actions. Anyone writing agent calls must follow the handler signatures, not the names:
   - set-annotation-border: x = line width, y = radius_x, width = radius_y (ACT:49; ANN:34).
   - set-extra-entry and set-entry: extra = KEY, text = VALUE (ACT:52, 197; AUTH:1077).
   - set-properties: text = KEY, extra = VALUE (ACT:63; RES:225).
   - set-encryption: object = OWNER password, extra = USER password, text = algorithm (ACT:56; DOC:265).
   - set-output-intent: object = subtype, text = condition id, extra = info (ACT:57; DOC:279).
   - set-form-field: object = name, text = value, extra = kind (ACT:57; DOC:297).
   - set-embedded-file: object = id, text = file name, extra = contents (ACT:40; DOC:172).
   - set-form-settings: object = setting name or field name, text = value or sub-aspect, extra = secondary value (ACT:51; ANN:76).
   - set-font-reference: text = program kind, extra = artifact id (ACT:59).
   - set-font-metrics: text = encoding, extra = widths, x = FirstChar, y = ascent (ACT:64).
   - set-image-mask: text = kind, extra = mask image id or colorKey ranges (ACT:65).
   - set-color-space: text = kind, extra = separation colorant (ACT:62).
   - set-resource-detail: text = owner.field, extra = value (ACT:55).
   - set-page-extra: text = value, extra = key (for action and entry) (ACT:47).

2. Object-id addressing is inconsistent:
   - The annotation id uses a zero-based page prefix (`p0:a0` is page 1, GEOM:12), while `page` is one-based.
   - set-annotation, set-annotation-markup, set-annotation-kind and set-field-data use strict prefix matching through `annotation_at` (ANN:6-8).
   - set-annotation-style and set-annotation-appearance only parse the suffix after `:a` (ANN:404, 446). A wrong page prefix is silently accepted and the annotation on the given `page` is edited.

3. Destructive or structurally unsafe paths with no guard seen:
   - set-form-field with empty object removes the whole AcroForm (DOC:298-299).
   - set-trailer-entry can overwrite any trailer key. Test PTESTS:428-429 writes /Info as a Name, which is not a valid structure for a trailer /Info dictionary.
   - set-catalog-entry can overwrite any /Root key (ANN:438-443; no protected-key check seen).
   - set-indirect-object with an existing number overwrites that object (RES:33-34).

4. Silent no-ops or lossy behaviour:
   - set-annotation with width or height <= 0 leaves the rect unchanged (OBJ:109-111).
   - create-annotation treats pure black [0,0,0] as "no colour" and uses the default (AUTH:85).
   - set-embedded-file accepts UTF-8 only, no binary (DOC:180).
   - set-properties and set-graphics-state with empty values store an empty name rather than removing (RES:230).
   - set-catalog-entry, set-trailer-entry and set-entry ignore extra (ACT:65; RES:190).

5. Fault-code leakage: `req_text` (ARG:16) and `req_page` (ARG:40) wrap the window-kit error with `fault(error.message)`, so every required-argument failure surfaces as `stdio.pdf.page-edit`, and the window-kit codes (`stdio.window-kit.argument-required`, `-duplicate`, `-type`, `-range`) are never returned to the caller for these actions.

6. Required versus optional asymmetry: `req_text` refuses JSON numbers and bools (WK:99), while `text_arg` coerces them to strings (WK:62-63). `req_page` also refuses JSON strings. For example, `{page:"1"}` fails on every page-required action but `{page:"1"}` works for opt_page users such as `set-page-transition` only if they use opt_page; check the mapping per action.

7. Blank-document feasibility: read-only, no run. Actions that need a pre-existing object (set-font-reference, set-glyph, set-mesh-reference, set-font-metrics unless insert-text runs first, remove-*, set-annotation-*) need a sequencing step. set-glyph and set-mesh-reference have no creation path among the listed actions.

8. Field widgets: `set-form-field` creates fields with `widgets: []` (DOC:312). Editing actions never populate `field.widgets`; only the reader and test fixtures do (grep of editor code). Consequence: fill-form-field on a blank document updates /V but does not regenerate any widget appearance. A visible field needs a widget that the editing actions do not create. This is inferred from the code and needs a runtime check.

9. Page resource registration: for ExtGState, ColorSpace, Properties, Pattern, form XObjects, the writer code that adds them to page /Resources was not read. Whether they become visible or referenced in the output is unknown.

10. UI binding: FC lists the controls per group (pages, fonts, images, forms, acroForm, optionalContent, etc.) and `pageInspectorBindings` (create-annotation: 6, set-info-field: 9, set-entry: 4, and others). Every action in Table 1 except fill-form-field appears in FC. fill-form-field is in FC's `acroForm` list (line 76).

## Rust test coverage

- Every one of the 35 action ids appears in at least one unit test, by string search of PTESTS, ATESTS, NPTESTS, ETESTS (refs in Table 2).
- Coverage is at the snapshot level (`edit_from_action` followed by `apply`). These tests do not write bytes and read them back with a third-party reader.
- The only third-party reader in the editor tests is `qpdf --check` (ETESTS:1401-1410), used through the `history_edit_acceptance_law!` macro (ETESTS:1399, 1425). Whether that macro drives these actions was not verified.
- ETESTS:2754 mentions a pymupdf oracle gated by env var `SEMIO_PDF_REAL_WORLD_DIR`. It was not read.
- Deep argument checks in tests are partial: set-field-data is tested only for Choice; set-resource-detail lines 620-714 were not read; set-page-extra box.* routing was not read.

## Unknowns (not verified)

- The writer (snapshot to PDF bytes) and its key names for annotation fields (/C, /BS, /T, /Subj, /CA), AcroForm keys, OCProperties, Names trees, and page-resource registration.
- `set_page_boundary` (DOC via page-extra `box.*`), the button and signature branches of `fill_form_field` (AUTH:311 onward), and `set_font_metrics` details beyond RES:239-284.
- Whether `text_value`-based "required" checks reject empty strings for every action (WK:96-101 accepts empty strings; some handlers reject them later).
- Object-number collision handling for set-indirect-object.
- The `UnknownField` (`stdio.pdf.unknown-field`) fault usage in the actions above: not seen in the action handlers read.
- Whether a runtime blank-document run of any example produces a valid file. None was executed. Recommend a runtime pass for rows 1, 7, 10, 11, 17, 19, 21, 27, 33 first, since their examples need no pre-existing objects.