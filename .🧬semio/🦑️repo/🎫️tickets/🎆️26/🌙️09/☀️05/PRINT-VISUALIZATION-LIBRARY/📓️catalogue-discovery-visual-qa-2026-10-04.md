# Catalogue Discovery Visual QA

These are pre-correction discovery outputs, not final certification. Available published pairs viz-9, viz-2, viz-14, viz-33, viz-38, viz-43 and viz-56 were copied to ticket-generated ASCII filenames. Poppler rendered page 1 for both themes at 110 DPI, and all fourteen full-page PNGs were actually viewed. Independent pypdf extraction confirmed these are first figure pages rather than covers. Scientific viz-30 was not published and is unavailable for inspection. Root separately inspected viz-64 page 2.

| Document | Pages per theme | Actual first-page visual observation |
| --- | ---: | --- |
| viz-9 | 8 | Sankey/alluvial nodes, bands and themes are visible. Bands show white pinch notches immediately before right-hand nodes; allocation correctness needs native diagnosis. Parallel-sets Renew/Cancel/Buy labels visibly overlap in tiny lower-right segments. |
| viz-2 | 16 | Axes, grouped colored lines, points and highlighted muted series are visible in both themes. The simple-line first figure joins repeated x sequences with a long diagonal connector and displays a nameless legend swatch; this may be data/group mapping rather than a renderer defect and needs diagnosis. |
| viz-14 | 12 | Process arrows and node outlines are visible in both themes. Horizontal process Requestdata text exceeds its narrow rectangle; Complete? and nearby edge labels are cramped. |
| viz-33 | 8 | Definite containment failure: molecule structures sit at bottom-left of each figure. Negative-y wedge and N label extend below the border toward the next figure heading. Reaction arrow annotation is rendered as a tiny missing glyph rather than delta. Both themes reproduce it. |
| viz-38 | 8 | Abstract diagrams sit at bottom-left and use a small part of the declared canvas. The first commutative-square edges have no visible arrowheads. Their intended direction contract needs source diagnosis. Theme strokes/text remain visible. |
| viz-43 | 8 | Pitch geometry, shot points, passing links/player labels and heat cells are present. No new definite containment blocker was identified on this sampled page. |
| viz-56 | 8 | Feynman e-/e+ labels extend beyond the left figure border. On the six-by-six game-board variant, R lies above the board and N outside its right edge. Chess and Go sample marks otherwise render in both themes. |

The strongest new blocker is scientific-family containment. `semio-viz-scientific-chemistry.sty` default molecule atoms include `a6/14/-8/N`, but the family emits those coordinates directly after entering a canvas rooted at zero, explaining the actual negative-y overflow. Default reaction annotation is the Unicode delta character. These diagnoses preserve existing canonical family ownership; no stylesheet changes have been made by this catalogue agent during this inspection.

Generated discovery QA inputs, extraction JSON, PNGs and raster stderr remain under `🗑️generated/catalogue-discovery-qa` for the root/native owners until ticket cleanup. The initial pypdf console dump hit the Windows cp1252 console on a noncharacter; the full extraction JSON had already been written and was then read successfully through PowerShell. Raster commands all completed with exit 0.

## Confirmed Sankey Control-Point Defect

Actual compiled neutral probe values were `[80,32]` for source layout x=12, target x=30, offset=14, scale=2 and curvature=.5. Independent D3 horizontal-link controls are `[56,56]`. The second vector (source18,target42,offset3,scale1.2,curvature.35) produced `[49.8,28.2]`, versus expected `[34.68,43.32]`. The existing `semio-viz-flow-sankey.sty` control macros subtracted an unparenthesized `ox + x*k` expression; its x term consequently gained the wrong sign. Parentheses now preserve the complete mapped coordinate before subtraction. Green compiler and actual flow reraster checks are pending.

## Pinned Physical Math Fonts

The actual reaction/Feynman compiler retry exposed `cmsy5.pfb` missing from the pinned TeX payload despite its metrics/map being present. The existing locked archive acquisition owner now pins all twelve missing physical Computer Modern fonts for the already admitted metrics. No archive-wide download or runtime dependency was introduced. Read-only acquisition inspected bounded archive ranges (433,088 and 1,510,000 bytes); the permanent bundle owner continues exact range, byte-count and SHA256 verification with bounded progress/cancellation.

Archive: `https://data1b.fullyjustified.net/tlextras-2022.0r0.tar`, 2,881,562,112 bytes. Pins are handcrafted in the existing `tectonic-template-compilation/📚️bundle/🔒️dependencies.json` owner.

| Physical Font | Payload Offset | Bytes | SHA256 |
| --- | ---: | ---: | --- |
| cmex10.pfb | 1119651328 | 30251 | `791b31aa1db8608d0144b3a40fc0fe53383a60f6b00d0e8fd9f06ac4a11df8cb` |
| cmmi6.pfb | 1119952384 | 37166 | `c31dbaffb861162eadf3a7210bdf271b0dda577841aed4c0572069b0d52662b6` |
| cmmi9.pfb | 1120069632 | 36094 | `c7d9ebe6b8313e0eeee8f9ed4d4e7069c74cfa9d9b64295f0f487e4662281a62` |
| cmr10.pfb | 1120148480 | 35752 | `fdcede8794018df5f2b58f0905fb20a2b418ed8f67b73ee12445855dfbe5b1be` |
| cmr5.pfb | 1120257536 | 31809 | `84d38aac226b5274baca7a292e2039f5284a35d8a2ae31074a475fad87da310b` |
| cmr6.pfb | 1120292352 | 32734 | `9fe20cb9ef24a0f4c74d38a65d4eee5cff3165f3b8600407ceba396aeb2b7617` |
| cmr8.pfb | 1120363008 | 32726 | `8150cbfac5cfe53040327df171c3059f10730b32cabf2504b01713d639a11feb` |
| cmr9.pfb | 1120398336 | 33993 | `ba5207eea69eaa9b598de2bd3d9aef27c4ff9831524400f70ff6a0dd3e36059b` |
| cmsy5.pfb | 1121054208 | 32915 | `46da57e5a06866efa9a20f3dd350811b5d3279a5ae789af63e530f1f570e3c7f` |
| cmsy6.pfb | 1121089536 | 32587 | `73eed8a83a07d8ef04d240da22bce1fd3646074f46179e3677f4f42a542f20b6` |
| cmsy8.pfb | 1121159168 | 32626 | `2313392f0f4cd974d9da5fb54a52feae126059759edc04a4e47e33ea6027418b` |
| cmsy9.pfb | 1121193984 | 32442 | `8049871d56d74fe3f8f2375afbb1e59b149bf6835027b4a101051cfca1c1a779` |

Registered family green2 prepared and verified all 500 pinned files (19,856,791 bytes), then compiled both actual six-case documents and the invalid-board rejection. Geometry, board D3 placement and process PDF.js width checks passed. The sole failed assertion was reaction Delta: standard Type1 extraction maps the visible math Delta glyph to U+2206; the fixture now uses that actual glyph extraction. Poppler light pages 2–7 were all viewed: molecule atoms/wedges, negative-coordinate molecule, reaction triangle, Feynman mu/gamma, all four board pieces and fitted process captions are present and contained. This is focused intermediate evidence; no full catalogue completion is claimed.


Independent PDF.js inspection of the discovery viz-9 light first page confirmed two real text-rectangle intersections: Paid/Renew and Paid/Cancel. This refines the initial visual diagnosis: competing captions in adjacent columns shared horizontal gutters. The existing native flow owner now bounds measured captions to half the adjacent-column clearance as well as spreading crowded strata vertically. Its neutral fixture explicitly encodes the 97:2:1 Renew/Cancel/Buy target distribution; a second stock multistage fixture covers Paid/Renew/Cancel adjacent-column intersections. These are regression assertions, not yet a final passed gate.


## Final Family Geometry Before Conservative Gutter Clearance

Eight actual cases in each theme compiled in green5. PDF.js consumed12pages per theme; PGF records below show the positive containment bounds in millimetres on80×40frame. All min/max checks passed. The exact third-party text rectangle test still detected a Paid/Cancel border touching by0.0037pt, so the reserved adjacent gutter was increased0.6mm for green6; the verifier was not weakened.

| Case | Minimum | Maximum |
| --- | --- | --- |
| molecule-default | 16,0001, 12,6144 | 45,1833, 26,6079 |
| molecule-negative | 9,8169, 7,6144 | 50,1834, 32,3858 |
| reaction-symbol | 6, 12,6144 | 54, 26,6079 |
| feynman-labels | 1,8091, 10,1411 | 58,6944, 29,8591 |
| board-six | 22, 2 | 58, 38 |
| flow-captions | 14, 3 | 66, 37 |
| flow-adjacent | 14, 3 | 66, 37 |
| process-horizontal | 3, 2,9999 | 77, 37 |


## Family Implementation Ownership

Existing native owners changed for confirmed rendered counterexamples:

- `🖋️latex/semio-viz-scientific-field.sty`: shared affine containment helper for explicit scientific geometry.
- `🖋️latex/semio-viz-scientific-chemistry.sty`: atom/reaction extents and proper math reaction Delta.
- `🖋️latex/semio-viz-scientific-physics.sty`: Feynman extents, proper mu labels and admitted propagator labels.
- `🖋️latex/semio-viz-diagram-process.sty`: all stock pieces lie on six-cell board; explicit authored file/rank rejected outside configured board; geometry probes expose centres.
- `🖋️latex/semio-viz-diagram-flowchart.sty`: native-font text measurements interrupt TikZ nullfont, fit labels within configured node width, report actual width.
- `🖋️latex/semio-viz-flow-sankey.sty`: parenthesized affine subtraction fixes cubic ribbons; node geometry widths/heights agree with mapped extents; measured captions fit reserved adjacent gutters and crowded strata with leaders.
- `🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🔒️dependencies.json`: twelve physical CM fonts pinned for existing logical font metrics/maps.

Neutral fixture/helper live in `🧪️tests/🖼️family-containment/🔣️.json` and `🟦️.ts`. Independent test-only D3 validates cubic controls and board centre mapping; PDF.js validates every actual page, expected glyph extraction, label widths and strict rectangle nonintersection. The existing canonical native runner invokes the helper after transform/layout checks; its existing package script exposes `--families-only`. Root owns final launch grouping; the async lane supplied the one-line optional-label narrowing. No standalone script, new runtime dependency or public visualization option was added. Parent's schema mutation/inference mechanism remains the only public customization mechanism.

The six original positive fixtures were visually inspected in both themes by the read-only visual audit lane. The two flow fixtures were also inspected in both themes in green5; exact oracle caught a microscopic text-border touch and the final green6 increased clearance instead of weakening the assertion. Final source freeze is pending green6 terminal evidence.

## Family Gate Completion And Freeze

Registered canonical `@semio-tech/print:test-native-grammar -- --families-only` green6 completed with exit0 in3m19s. Both themes compiled8positive neutral cases and PDF.js consumed all12pages of each actual PDF. All strict text nonintersection, native containment, glyph extraction, process-width, D3 flow-control and board-centre assertions passed. The invalid authored7/2position on6×6board was rejected by the actual compiler with the expected diagnostic.

| Theme | Consumed Pages | PDF Bytes | SHA256 |
| --- | ---: | ---: | --- |
| light | 12 | 37246 | `56e4a1c40945477ca0cd52ec16e25b7ba84bdfec838025b5eefae53af693da5d` |
| dark | 12 | 37206 | `04fae65b287e2c87a19c191065052881e32fe370d859d30b8ca616c5d9a27547` |

Final flow pages7/8 were rasterized with Poppler and actually viewed in both themes. Captions/leaders remain within their figures; tiny targets retain separated captions; dense stock captions retain separation. Independent PDF.js measured Paid/Cancel horizontal clearance1.6983pt, approximately0.599mm. Final Sankey source SHA256: `b734b8aa5db1165687aecaa38a7acc236b39a579106cd7c47de2437f9ab7d32f`.

Native family styles and pinned bundle are frozen for fresh canonical catalogue compilation. This establishes focused family runtime behaviour; full81-document/162-PDF completion remains pending.

### Evaluation and Riemann Native Compiler Repairs

The neutral evaluation compiler before failed at `regions.tex:11`, `Missing ) inserted for expression` (registered session 88817, exit 1). Existing `semio-viz-charts-statistical.sty` now uses expandable integer comparison for the decision-region palette instead of an invalid ternary inside integer arithmetic, and emits region selections for independent D3 threshold verification. Two neutral cases cover the 8×8 region grid, alternate canvas dimensions and padding in both themes.

The isolated stock Riemann compiler before failed at `sheets.tex:11`, `LaTeX3 Error: Invalid operation (0)/(0)` (registered session 45616, exit 1). The complex family used a shared parameter sampler without defining its sample count, so its initial interval count was zero. Schema-first `sci-complex.samples` now declares an integer count (default 120, minimum 1), described in English and German. Existing `semio-viz-scientific-mathematics.sty` binds and initializes it. New neutral `🧪️tests/🖼️family-containment/🌀️complex.json` covers a fresh stock 3-sheet chart and a custom 2-sheet, 8-interval chart with different domain/range/dimensions. The existing family test helper compares every emitted native path point against independent D3 canvas scales and consumes both actual PDFs with PDF.js. Focused after runs are pending; these are not yet asserted green.

### Evaluation and Riemann Final Focused Runtime Results

Registered no-cache evaluation green3 (session 47655) exited 0 in 4m 38s: each theme passed all 128 actual cell rectangles and classification selections against D3; PDF.js consumed four pages per theme. Registered no-cache complex green2 (session 82125) exited 0 in 4m 58s: each theme passed all 381 actual native sampled points against D3 scales; PDF.js consumed four pages per theme.

The intermediate assertion corrections changed only test expectations: common evaluation accepts `pad`, while stock scientific canvas width is its declared 60 mm default, independently of enclosing figure width. Production palette and sampling corrections remained unchanged. Poppler rendered and actual view_image inspection covered stock/custom Riemann pages 2 and 3 in both themes and final evaluation page 1 in both themes. Curves, axes and region stairs are contained and visible; no material clipping was found. Exact final PDF digests follow.
- native-evaluation-boundary-green3/evaluation/light/🧪️probe-out/regions.pdf: SHA256 `542a00e91e85f3fab9218469e74a6b5003a768f0c636ec2be2cd03eb843e837f`, 14658 bytes.
- native-evaluation-boundary-green3/evaluation/dark/🧪️probe-out/regions.pdf: SHA256 `baf634ce3444c95cb92fc074f3ff80f8fd2819a931193bc583c99836bc25fa94`, 14639 bytes.
- native-complex-surface-green2/complex/light/🧪️probe-out/sheets.pdf: SHA256 `fc10a981851af4373ce5be0b9b2dcd19b4523aafd8e8f955a40f2f79483a23ab`, 19169 bytes.
- native-complex-surface-green2/complex/dark/🧪️probe-out/sheets.pdf: SHA256 `acc6424ae0bdeab07c3d7034ffeb97d7916ac9eaae6194b86624a74797faef77`, 19144 bytes.
`sci-complex.samples` schema and neutral fixture helper were updated in place alongside concurrent encoding and solar additions. Production files are the existing statistical and scientific-mathematics owners; no separate runtime module or script was introduced. Fresh Nx graph `nx-catalogue-final-repaired` completed with exit0 (session70115) and resolves81 document owners includingAPI.

### Broader Mathematical Capacity Counterexample

Quoted registered run-many stock gallery replay `build-viz-24,build-viz-25` (session 12810) progressed beyond Riemann. Actual `viz-25.tex:64` at `bifurcation-diagram` failed `TeX capacity exceeded, sorry [main memory size=5000000]`. The existing family retains its full 121 parameter columns × 60 tail points, with 120 transient iterations per column. Its current implementation invokes a complete TikZ fill statement for each of 7,260 circles. The neutral stock/custom capacity fixture preserves those defaults and checks independent D3 canvas positions, stable logistic tails and actual PDF.js fill operators; chaotic high-parameter tails are bounded and counted rather than equated across distinct decimal/binary floating arithmetic. Before and after compiler results remain pending.

### Bifurcation Capacity Diagnosis

The isolated neutral stock case reproduced the same fixed-engine 5,000,000-word capacity failure (30226, exit1, 1m44s). Low-level PGF circles under one outer scope (66420), per-column flush alone (20450), a test-only precompiled numeric probe matcher (26773), and generic PGF reusable-object fallback (87829, exit1, 2m6s) all retained the failure. The precompiled probe override and temporary column diagnostics were removed; neither is a production change. Diagnostic compiler logs reached parameter column109 before exhaustion, establishing growth while rendering retained marks rather than a blocked dispatch. PGF protocol buffer clearing uses global assignment, so TeX grouping does not restore a discarded buffer.

Current focused registered green4 (69628) batches the same independently filled circle operations into a column-sized protocol special. Cached origin-circle geometry is translated for each mark; every separate fill, radius0.09mm and opacity0.55 is preserved. The neutral fixture checks all7,280 stock/custom point records with D3 mappings and stable logistic arithmetic, then PDF.js counts actual individual fill operators on every page. This is still pending compiler evidence; no sample count, transient count, tail count, opacity or default is weakened.

### Stock Mathematical Gallery First Theme

Unchanged current mathematical owner completed the entire28-figure stock viz25 light document, including the original failing Riemann and bifurcation entries and every later mathematical kind. The actual compiled light PDF has8pages,129998bytes, and was copied only for inspection before transactional pair publication. Poppler120dpi page4 was actually viewed: cobweb staircase, full bifurcation cascade, phase-field arrows and phase-space curves are visible and contained within their figures. Dark compilation remains active; the registered target is not yet terminal and the complete pair is not yet certified.

### Bifurcation Final Light Consumption

Registered focused green5 now reports actual light PASS:7,280 neutral stock/custom numerical points,7,310 independent PDF.js fill operators and four consumed PDFpages. The installed PDF.js folds path paints into constructPath opcode arguments; the corrected assertion counts those individual fills as well as standalone fill operations. Renderer source remains SHA740dd6a211654a74b0b60dd07a0a4086b973d94371feb2bf0258d86731485a5c. All temporary diagnostics and the disproven numeric probe override are absent. Dark focused and stock outputs are still running; paired terminal proof remains required.

### Paired Stock Mathematical Gallery Terminal Proof

Registered no-cache `@semio-tech/print:build-viz-25` (37303) completed exit0 in18m54s. All28 authored stock mathematical kinds compiled in both themes; each actual PDF has8pages. Pair was transactionally published to current dist/documents/viz-25 with owner marker. Poppler120dpi actual views covered page4 in both themes after publication; cobweb, full bifurcation cascade, phase-field vectors and phase-space curves remain visible and contained. This is the broad mathematical production release; final81-owner aggregate still must compile or verify current-source cache for every owner and consume all162PDFs.

| Stock PDF | Bytes | Pages | SHA256 |
| --- | ---: | ---: | --- |
| 📈️viz-25.pdf | 129998 | 8 | `abb5668239d536d2d255e1d1892674d4be069bb0dccdfb9b61ebeaff28634895` |
| 📈️viz-25-dark.pdf | 129923 | 8 | `f40bb2c5b3af61a11919a34ccfd1ccbd787066ebac8a594a5cc09b5be1efb8b2` |

The parent accepted frozen mathematical scheduling after actual light numerical/PDF checks; final canonical81 build session84476 started with fresh validated graph and repeated294input SHA comparison. The remaining focused dark numerical/fill proof (38750) is still active. No source changes followed dispatch.

### Final Bifurcation Numerical, Vector And Visual Gate

Registered no-cache green5 (38750) completed exit0 in15m8s. Both themes independently passed7,280 stock/custom retained point records against D3 coordinate maps and stable logistic arithmetic,7,310 actual PDF.js individual fill operations, all case titles, and all four actual PDFpages. The two variants preserve stock121columns×60retained points and custom5columns×4retained points; transient/default counts are unchanged. Column-sized protocol batches retain each separately painted radius0.09mm circle at opacity0.55. No raster substitution, dropped mark, weakened resolution or compiler memory-limit increase was introduced.

| Theme | PDF Bytes | PDF Pages Consumed | Actual Vector Fills | SHA256 |
| --- | ---: | ---: | ---: | --- |
| light | 41899 | 4 | 7310 | `178fe8371be9fbe2176b68ed42d9645298c43853a14e210a834e502d6f78773f` |
| dark | 41872 | 4 | 7310 | `4f904b479f3913793d815510b3bd5c23bc9456034ef25c549239679503761f8f` |

Poppler120dpi actual views cover stock/custom pages2/3 in both themes. Viewed earlier light raster input is byte-identical to final green5 light PDF, established by SHA comparison. Full stock bifurcation/phase page4 was also actually viewed in both published gallery themes. Production mathematics SHA740dd6a211654a74b0b60dd07a0a4086b973d94371feb2bf0258d86731485a5c and family helper SHA8093c6113d1b76a8a98a8f9ea1eefd48736f93a46971848ae012258e1e3efa8e are frozen for final registered native and complete catalogue gates.
