# Paint Fidelity (G12, G13, G14)

Paths are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base` unless noted. Page editor means `✏️editor/🖼️page`.

## G12 Base 14 fonts

Applied the staged URW Base35 route to the live tree. The two stage scripts (`📜️pdf-base14-render-stage.ts --all-locks`, `📜️pdf-base14-symbolic-stage.ts`) were rerun against the current sources, because every guarded target had drifted since the first staging. `📜️pdf-base14-apply.py` then wrote the overlays under hash guards (all 5 source edits, 6 resource crates, workspace manifest and 32 `Cargo.lock` files, 14 CFF programs, `LICENSE`, `COPYING`, `SOURCE.md`).

- Source of the programs: URW Base35 release 20200910, commit `c15105598aa7eb256b1ebfcecd3d078801521e73`, verbatim CFF tables of the upstream OpenType files. Provenance and the upstream license texts sit in `🔨️modules/🔤️fonts/🖼️base14/` (`SOURCE.md`, `LICENSE`, `COPYING`). The repository is AGPL-3.0 and the upstream license is AGPL-3.0 with the document-embedding exception, so no incompatible license enters the tree. Liberation (conflicting license metadata) and Foxit (incomplete coverage, no sans) were not used.
- Codec (`🔨️modules/🔤️fonts/🦀️.rs`): `FontCodec` selects the standard program through `standard_outline_program` for non-embedded Type 1 standard fonts. PDF encodings, Differences, ToUnicode and widths stay the PDF's own (AFM widths from `StandardFontMetrics`), so advances never come from the substitute face. Symbol and ZapfDingbats keep their original single-byte codes (`PdfFixedCodeWidth::One`) and Dingbats text maps through the Adobe glyph names.
- Scene: the text paint already routes raw CFF outlines through `truetype-text`, so the Anta, Kelly Slab and Share Tech Mono fallback no longer fires for any of the 14 standard faces. `🧪️text-paint.json` no longer enshrines "Anta"; the scene test is `canvas_paints_base14_glyph_outlines_at_the_pdf_baseline`.
- Validation: earlier staged runs recorded FontTools and MuPDF 1.26.7 agreeing on 54 + 200 glyph contour boxes (`base14-outline-verification.md`). This pass adds MuPDF raster probes for Helvetica, Times-Roman and Courier-Bold text in `🧪️paint-fidelity.json` (cases `base14-*`).

## G13 Placeholder paint

New and changed modules (Page editor unless noted):

| File | Change |
|---|---|
| `🧮️function-colour/🦀️.rs` (new) | PDF function evaluator for types 0 (multilinear, up to 4 inputs, Encode/Decode), 2, 3 and 1-out arrays, plus colour to sRGB for DeviceGray/RGB/CMYK, CalGray/CalRGB, Lab, ICCBased (alternate or by N), Indexed, Separation, DeviceN and named spaces. PostScript calculators (type 4) are not evaluated. |
| `🌄️shading-preview/🦀️.rs` | Axial now accepts any function and colour space (64 sampled stops, one exact stop pair for the linear RGB/Gray case). New radial (two-circle solution with both Extend flags) and function-based (type 1, Domain and Matrix) shadings are rasterised to an image layer clipped to the BBox, or to the page when there is none. Mesh types 4 to 7 still use the labelled placeholder. |
| `🔆️image-preview/🦀️.rs` and `🔨️modules/🖼️images/🎭️sampled-alpha/🦀️.rs` | Stencil masks (`ImageMask`) paint with the current fill colour. Indexed, CMYK, Lab, Cal*, ICC, Separation and DeviceN images convert through the same colour module (injected as `ConvertColour`, so the images crate stays free of colour management). 4-component DCT is accepted when the Adobe transform is 0. |
| `🗺️scene/🖍️annotation-default/🦀️.rs` (new) | Annotations without an appearance stream paint their subtype default: Highlight (multiply), Underline, Squiggly (as underline), StrikeOut, Square, Circle, Line, Ink, Polygon, PolyLine. Link and Popup paint nothing instead of a rectangle. |
| `🧭️geometry/🦀️.rs` | Radial shading bounds now cover both circles. |

Forms and annotation appearance streams already paint through `paint_form`; the placeholder is now reached only by: mesh shadings, images stored as JPX, CCITT or JBIG2 (no decoder exists in the repo), DCT YCCK or non-8-bit coded images, `SMaskInData` images, and annotation subtypes with no synthesizable default (Text, FreeText, Stamp, Caret, FileAttachment, Widget without appearance).

Tests: `🧪️tests/🔬️unit/🧪️paint-fidelity.json` is the language-agnostic fixture (shading, image, annotation and Base 14 text cases with probe pixels). `📜️pdf-paint-fidelity-oracle.py` renders every case with MuPDF (`write` fills the probe colors, `check` verifies them). `🧪️tests/🔬️unit/🌈️paint-fidelity/🦀️.rs` rebuilds each PDF, runs `canvas_layers`, composites the scene with a small reference compositor and compares each probe to the MuPDF value within the case tolerance.

## G14 Large images

No code change, a status account, because the route needs the reactor, WIT and host work other agents own.

- The picker route is bounded by `MAX_FILE_BYTES = 24_546` (`📄️document/🖼️image-authoring/🧵️decode.rs`) and `WIRE_BYTES = 48 KiB`; the 473,683-byte PNG and the 2.1 MB JPEG are far beyond both. The staged raw picker (`raw-picker-product.md`) carries ordered fixed pages through the reactor and publishes through the paged PNG factory; its remaining mount is the typed retained output slot in `📥️local-image`, the actor poll branch and the registered close.
- Metadata donation: the 521 x 257 PNG has 401,691 RGB samples, above the 262,144 `u32` sample donation limit, and the canonical contiguous Samples frame would need 1,606,764 bytes. The staged `set-image-metadata.md` stage handles metadata field by field under paid grants, but the sample body must move to the paged/durable raster path (`paged-png-placement.md`, `pdf-durable-raster-ownership.md`) before the large case can use `SetImage`.
- The 2.1 MB JPEG needs the coded (`Artifact`) body route (`paged-jpeg-resource.md`) and does not enter `SetImage`. Its canvas preview already works through `dct_samples` (96 MiB working bound); this pass extends it to CMYK.
- The authored mounted acceptance (`image-authoring-browser.ts` with the generated PNG) remains unrun.

## Results

| Check | Outcome |
|---|---|
| `📜️pdf-paint-fidelity-oracle.py write` (MuPDF 1.26.7 via `repo tools exec --light uv -- run --offline --no-project --python 3.12 --with pymupdf==1.26.7`) | Passed: 20 cases rendered, probe colors written into `🧪️paint-fidelity.json`. MuPDF itself synthesizes Square, Circle, Highlight and Ink appearances, so the annotation defaults have a real oracle. |
| Standalone compile of `🧮️function-colour` against stub types (rustc, scratch only) | Compiles. Stitching, sampled 1-in and 2-in, Separation, DeviceCMYK corner, CalGray and Lab reproduce the MuPDF probe values within 1 level. |
| Radial solution checked in Python against the MuPDF probes | Within 7 levels (MuPDF samples pixel corners, the preview samples centres), so the radial cases use tolerance 10. |
| CMYK | MuPDF applies a SWOP-like profile. The PDF.js polynomial was off by up to 61 levels, so the converter interpolates the 16 hypercube corners measured with `📜️pdf-cmyk-corners-oracle.py`; mid-tones differ by up to 22 levels (axial CMYK case tolerance 30). |
| `repo cargo ... test -p semio-s-artifact-stdio-pdf-page-editor-tests --lib` | NOT RUN TO COMPLETION. Four attempts, each stopped by a blocker outside this scope: the `semio-plugin-instance-app` crate (267,689 of 262,144 bytes) and later `semio-os-channel` (4,009 of 4,000 active lines) source boundaries, E0599/E0308 in `semio-plugin-instance-stores` (`action-ingress`), a transient missing `mio` registry file, and repeated `repo` host-capacity policy edits (`/swap/activeMegabytesPerSecond`, `hostUnderPressureV1`). The base14 resource crates did compile in the one run that got that far. The page editor, fonts and images crates have therefore not been compiled with these edits. |

The first run also showed the test crate at 264,465 of 262,144 source bytes after adding the new test, so the fixture was compacted (27 KB to 13 KB). The crate sits near its limit; a later addition there needs a new test crate.

### Rerun when the build is free

```
repo cargo --store=shared --configuration=prototype -- test --offline --manifest-path '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.toml' -p semio-s-artifact-stdio-pdf-page-editor-tests --lib
repo tools exec --light uv -- run --offline --no-project --python 3.12 --with pymupdf==1.26.7 -- python 📜️pdf-paint-fidelity-oracle.py check <🧪️paint-fidelity.json>
```

Expected new tests: `canvas_paint_matches_mupdf_probes` (20 cases) and `canvas_paints_base14_glyph_outlines_at_the_pdf_baseline`. Risks to check first if they fail: the Base 14 text probes use tolerance 140 (partial coverage at stem edges), and radial/function-based shading probes depend on the half-pixel sampling note above.

## Round 2: no placeholders

New crate `semio-s-artifact-stdio-pdf-paint-codecs` (`🔨️modules/🎞️paint-codecs`, std only, no runtime dependencies) mounts domain modules that stay in their taxonomy folders. Every decoder takes a `Control` closure (progress in 0..1, `false` cancels). The scaffold script `📜️paint-codecs-scaffold.py` created the crate, the workspace member, the page-editor dependency and 20 lockfile entries.

| Module | Scope | Validated against |
|---|---|---|
| `🗜️filters/🧾️ccitt` | Group 3 (1D and mixed 2D, EOL, byte align) and Group 4, all run-length and extended codes | libtiff through Pillow, 8 vectors (K = -1 and 0, runs up to 2560); the PDF polarity is rechecked by the MuPDF `codec-ccitt-g4` case |
| `🗜️filters/🧩️jbig2` | Generic regions (templates 0 to 3, TPGDON, custom AT pixels, MMR via the CCITT module), generic refinement (decoder only), arithmetic symbol dictionaries (also one-instance refinement aggregates) and text regions (all reference corners, transposed, strips, refinement) | MuPDF (jbig2dec) rasters of streams produced by a Python MQ encoder: 11 cases, 0 pixel mismatches. Refinement paths have no oracle yet. Huffman tables, multi-instance aggregates and halftone regions return `Unsupported` |
| `🖼️images/🌅️jpx` (+ `🧮️arithmetic`, `🧱️tier1`, `🌊️dwt`) | JP2 boxes and raw codestreams, tier-2 packets (all five progression orders, precincts, layers, tiles, SOP/EPH), tier-1 EBCOT with reset, vertical-causal and segmentation-symbol styles, 5/3 and 9/7 DWT, RCT and ICT | OpenJPEG through Pillow, 11 vectors: lossless exact, lossy within 1 level. Bypass, termination-per-pass, POC, packed headers, ROI and palettes return `Unsupported` |
| `🌅️mesh-shading` | Types 4 and 5 (Gouraud), 6 (Coons), 7 (tensor) with flags and per-vertex padding; 12 x 12 patch subdivision | MuPDF: lattice, Coons and tensor within 4 levels, free-form within 1 level except 8 diagonal edge pixels |
| `🧮️postscript` | Type 4 calculator functions, all operators of the standard, step and stack limits | Analytic cases in `🧪️codecs.json` and the MuPDF `postscript-function` probes |

Page editor wiring: `function_colour` evaluates type 4 functions, `shading-preview` rasterises mesh types 4 to 7, `image-preview` decodes JPX (colour space inferred from the component count when the dictionary has none), CCITT and JBIG2 through `coded_samples`, and `sampled-alpha` accepts 1-bit coded bodies. Text, FreeText and Stamp annotations without an appearance stream now paint a note icon, a framed text box and a stamp frame with its icon name. MuPDF draws no FreeText default and a different Stamp, so those cases only probe outside the shapes; the Text note probe tolerance is 30.

Tests added: the crate's own `🧪️tests/🔬️codecs` (JSON `🧪️codecs.json`, 145 KB, five tests including cancellation of all three raster codecs) and 11 further cases in `🧪️paint-fidelity.json` (4 mesh types, PostScript, CCITT, JBIG2 text, JPX RGB, 3 annotation defaults). The oracle script rewrote all 31 cases against MuPDF 1.26.7.

Still placeholders after this round: JPX with palette, bypass or termination styles, JBIG2 Huffman and halftone, multi-instance symbol aggregates, DCT YCCK, `SMaskInData`.

### G14 status

Not implemented. The raw-picker mount spans the reactor adoption slot, the WIT event and actor poll branch, the browser and native senders, the retirement registration and the paged factory publication (`raw-picker-product.md`, `paged-png-placement.md`). Those are the other agents' staged patches against crates that were failing to compile throughout this session, so a mount could not have been verified. The principled limit is the donation: the bound must be the exact paged `u32` sample grant (width x height x channels) instead of `MAX_FILE_BYTES = 24,546` and `MAX_IMAGE_IMPORT_PIXELS = 4,096`; both constants stay until the paged route owns the sample vector.

### Round 2 verification status

Standalone rustc runs (scratch only, same source files via `#[path]`) passed: CCITT 8 of 8, JPX 11 of 11, JBIG2 11 of 11, mesh 3 of 4 types fully and free-form except diagonal edge pixels, PostScript analytic cases. The governed `repo cargo test -p semio-s-artifact-stdio-pdf-paint-codecs` and the page-editor tests did not run: the PDF workspace currently fails with `cyclic package dependency` (`testkit` -> `schema-4` -> `page-editor-authoring` -> `page-editor` -> `tests-2` -> `testkit`), which comes from other agents' manifest edits and blocks Cargo's lock refresh. The page-editor, sampled-alpha, image-preview, shading-preview and annotation-default edits are therefore uncompiled. Rerun the two commands at the end of "Rerun when the build is free" with `-p semio-s-artifact-stdio-pdf-paint-codecs` added.

## Round 3

- JPX bypass, termination-per-pass, reset, vertical-causal and segmentation-symbol code-block styles are implemented (codeword segments per block in packet headers, raw bit-plane decoding); 17 OpenJPEG vectors (Pillow `codeblock_style` 1, 2, 4, 8, 32, 47) match, lossless exact.
- JBIG2 refinement regions over the page are implemented; MuPDF (jbig2dec) oracle with a Python refinement encoder: template 0 plain and TPGRON, template 1 plain match with 0 mismatches. Template 1 with TPGRON disagrees with jbig2dec (875 pixels) and is excluded from the fixture; the T.88 context for that case is unresolved.
- Not done: JPX palettes, JBIG2 Huffman, halftone and multi-instance aggregates, DCT YCCK (no oracle encoder), `SMaskInData`, and G14 (the raw-picker mount spans reactor, WIT, host and paged-factory crates owned by staged patches that could not be compiled; see G14 status above).
- Governed run: `-p semio-s-artifact-stdio-pdf-paint-codecs` got past the Cargo cycle but stopped at `failed to get semio-os-channel-archive-ingress as a dependency of semio-os-channel` (another agent's missing package). The page-editor-tests run was still queued. Neither result is recorded.

## Round 4

- JBIG2 refinement template 1 with TPGRON resolved: the typical-prediction pseudo-context is `0x0004` in the decoder's bit order (the reference pixel to the right of centre), not `0x0008` (centre). With `0x0004` the MuPDF/jbig2dec raster agrees with the decoder and a Python encoder with 0 mismatches; the case `refinement-t1-tpgr` is back in `🧪️codecs.json` (15 JBIG2 cases). `0x0008`, which PDF.js uses, disagrees with jbig2dec on 875 pixels. I could not read ITU-T T.88 §6.3.5.6 offline, so jbig2dec is the authority here.
- G14 dropped (reassigned to the segmented-IO agent).
- Remaining codec gaps unchanged: JPX palettes, JBIG2 Huffman, halftone and multi-instance aggregates, DCT YCCK, `SMaskInData`. Governed tests still wait on the archive-ingress resolution.
- JPX palettes (`pclr` and `cmap`, 8-bit index images) are implemented and match MuPDF on a JP2 built by `📜️jpx-palette-oracle.py` (`jpx-palette` in `🧪️codecs.json`, 0 mismatches). Palettes over sub-8-bit index precision remain unsupported.

## Round 5

- Governed `-p semio-s-artifact-stdio-pdf-paint-codecs`: 5 of 5 pass (CCITT, JPX 12 vectors including palette and all code-block styles, JBIG2 19 cases, PostScript, cancellation).
- Governed page-editor tests: 55 of 57 passed on the last full run. The two failures were `thumbnails_scale_each_page...` (UI arena registration, outside this scope) and `canvas_paint_matches_mupdf_probes`, whose compositor ignored the closing edge of stroked polygons (test-side bug, fixed; not yet rerun because `InteractionConfigMutation` stopped compiling in another crate). Two older tests asserted "Hello" text content for Helvetica; they now follow the object id because standard fonts paint as outlines. The paint-fidelity fixture is read at runtime to stay under the 262,144-byte crate boundary.
- Fixed `sampled-alpha` (Lab default-decode out of range).
- Added: JBIG2 pattern dictionaries and halftone regions (arithmetic, templates 0 to 3; MuPDF 0 mismatches), JPX sub-8-bit palette indices, JPX in-data alpha for `SMaskInData` (MuPDF case `codec-jpx-smask-in-data`), DCT YCCK conversion (cases `codec-dct-ycck` and `-inverted`, MuPDF probes recorded, unverified until the page-editor run).
- Not done: JBIG2 Huffman coding (standard tables B.1 to B.15 cannot be reproduced reliably without the spec text and no Huffman encoder or reference was available), multi-instance symbol aggregates, MMR halftone planes.

## Round 5

- Governed results: `-p semio-s-artifact-stdio-pdf-paint-codecs` passed 5 of 5 (CCITT, 12 JPX vectors including palette and code-block styles, 19 JBIG2 cases, PostScript, cancellation). `-p semio-s-artifact-stdio-pdf-page-editor-tests` ran 57 tests: two failures remained. `thumbnails_scale_each_page_and_reorder_through_move_page` fails in the UI value arena registration (framework code, not paint). `canvas_paint_matches_mupdf_probes` failed on my test compositor, which skipped the closing edge of a closed stroke; fixed in `🌈️paint-fidelity/🦀️.rs` and not yet re-run, because later builds broke in other crates (`InteractionConfigMutation` trait bound, `original_report_key`) and then in the `repo` runner itself.
- Two assertions that required the UI font text "Hello" or a label (`page_lists_text_vector_and_image`, `text_render_modes_select_fill_stroke_and_invisible_text`) now look layers up by object id, because standard fonts paint outlines.
- New and MuPDF-validated: JBIG2 halftone regions with pattern dictionaries (templates 0 to 3, arithmetic coded; 0 mismatches), JPX in-data alpha (`SMaskInData 1`, case `codec-jpx-smask-in-data`), sub-8-bit palette index decoding, DCT YCCK (cases `codec-dct-ycck` and `-inverted`; both render in MuPDF, the page-editor run will confirm). A Lab image bug (channel index past the white-point range) was fixed.
- Not done: JBIG2 Huffman (the standard tables B.1 to B.15 cannot be reproduced reliably without the specification text), multi-instance symbol aggregates (no encoder oracle), MMR-coded halftone planes.

## Round 6

- JBIG2 Huffman: `🧩️jbig2/🔣️huffman` holds standard tables B.1 to B.15 (values parsed from the public jbig2dec `jbig2_hufftab.c`, kept as `🔣️jbig2-huffman-tables.json`), custom tables (segment type 53) and canonical code assignment. Huffman symbol dictionaries (uncompressed and MMR collective bitmaps, export runs) and Huffman text regions (run-coded symbol code lengths, all selector tables, custom tables) are implemented. `📜️jbig2-oracle.py` has a Huffman encoder built from the same tables; MuPDF's jbig2dec agrees on `huffman-symbols-and-text` and `huffman-custom-table` with 0 mismatches. Huffman refinement and aggregates stay unsupported.
- MMR-coded halftone planes and MMR pattern dictionaries: `ccitt::decode_consumed` reports consumed bytes (after the EOFB); `halftone-mmr` matches MuPDF. The JBIG2 fixture now has 22 cases, all with 0 mismatches.
- Sub-8-bit JPX palette indices: a 3-bit JP2 (SIZ and ihdr patched) matches MuPDF (`jpx-palette-3bit`).
- Base 14 asset check: all 14 CFF programs exist on disk (`NimbusMonoPS-Italic.cff` included, in `🖼️mono-italic`). `🖼️base14/PROVENANCE.tsv` records font, file, byte length, SHA-256, upstream commit and license for each; the fonts crate test `every_base14_program_exists_matches_its_provenance_and_is_selected` checks existence, length, CFF header and that `standard_outline_program` returns the same bytes. A closure that lacks the file must include `🖼️base14/🖼️mono-italic` as a data dependency.
- Governed runs: the page-editor build stopped at `semio-s-artifact-stdio-pdf-inferences` (`unresolved import stream_roles`, another agent's edit). The paint-codecs and fonts test runs were queued behind the compiler image when this was written.
- Sampled-alpha out-of-bounds: the panic in `canvas_paint_matches_mupdf_probes` was `default_decode` indexing a Lab range with channel indices 3 to 7 (probe loop over `MAX_CHANNELS`), fixed with `3.. => (0.0, 1.0)`. Four odd-width cases (`odd-width-4bit-gray`, `-4bit-with-2bit-smask`, `odd-width-1bit-stencil`, `odd-width-1bit-gray`, MuPDF-validated, `📜️odd-width-image-cases.py`) now cover row padding for 1, 2 and 4 bit samples and masks; not yet run in the governed suite.

## Round 7

- Aggregate symbols: arithmetic symbol dictionaries with multi-instance aggregates (a text region decoded inside the dictionary, shared IAID, IARDX, IARDY contexts, trailing OOB consumed) and single-instance refinements are implemented; `aggregate-symbols` (Python encoder, two dictionaries, text region) matches MuPDF with 0 mismatches. The JBIG2 fixture has 23 cases, all matching.
- Huffman text refinement is implemented (RI bit, tables B.14/B.15 or custom, RSIZE, byte-aligned arithmetic refinement) but has no oracle: jbig2dec rejects Huffman text regions with refinement ("cannot decode jbig2 image"), so the Python stream `huffman_stream(..., refine=True)` cannot be checked and is left out of the fixture. Huffman refinement aggregate dictionaries stay unsupported.
- Governed results so far: paint-codecs 5 of 5 passed (before the aggregate change). The combined paint-codecs, fonts and page-editor run was stopped by `E0425 lstat_v1` in `semio-os-kernel-dsl-derive-expand` (another agent's crate), after the page-editor dependency closure had compiled up to it.
