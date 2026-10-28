# Primary Accent Regression Contract

The portable vectors in `🎨️visuals/🧪️tests/🎨️semio-style/🔣️.cases.json` now distinguish canonical primary pink (#ff344f) and secondary teal (#34d1bf). They describe helper calls, primitive rectangle inputs, addressed result paths and the stroke/fill channel to inspect. The native Python adapter instantiates the public Manim helpers, reads their resulting native object colors and compares RGB output to the independent Pillow ImageColor parser.

Fifteen default accent probes cover shared frame and chip borders, chip text, gauge fill, note text, card borders and readings, card grids, formula/math panels, clock hand, parameter highlighting and everyday watt anchors. Every probe also checks that an explicit secondary override remains teal. Typography vectors independently require primary chapter title chrome and beat subtitles while preserving neutral chapter title text. Existing palette vectors keep P_CYAN, P_TEAL and P_BLUE bound to secondary rather than relabeling teal as primary.

## Failing Baseline

Before production changes, the complete style suite ran 8 tests in 41.490 seconds and failed with 17 assertions and 2 missing-attribute errors. All 15 helper default probes failed, as did title chrome and beat subtitle. P_PRIMARY and P_SECONDARY were absent. Explicit secondary overrides and neutral title text passed. Existing silhouette, square/fused chrome, token spacing and render-cache closure checks passed. This establishes that the new cases detect the reported color hierarchy defect without suppressing legitimate secondary accents.

The permanent ticket runner is `run-accent-tests.py`. It uses the shared teaching Python runtime through `repo capacity run --memory=3072 --cpu=1`. Environment output and captured final test transcript stay under this ticket's `🗑️generated/accent-tests` directory. Production code is owned by the root agent.

## Passing Style Verification

After the shared production defaults changed, all 8 style tests passed in 27.006 seconds with no skips. This includes all 15 primary default probes, their 15 explicit secondary overrides, all three typography probes, canonical palette/font/spacing tokens, native square/fused chrome, independent Skia geometry, and source-closure render-cache fingerprints. The complete transcript is generated under accent-tests/tests.log.

## Typography Regression Verification

All 13 existing font tests pass in 40.405 seconds with no skips after the title frame and beat subtitle accent changes. The suite preserves native Pango rendering, exact source glyph contours against FontTools, .notdef rejection, independent cmap parity, explicit font ranges, default text recovery, German/math glyph geometry, SVG cache identity, and text canvas restoration. The transcript is generated under accent-tests/fonts.log.

Owned permanent changes are the two semio-style test files, accent-tests.md and run-accent-tests.py. No production sources or AGENTS.md were edited by this test agent. No test process remains active. Generated test logs remain for root review and ticket cleanup.

## Final Authored Scene Import Verification

After the authored scene agent signaled its final 27-file accent list settled, validate-accent-imports.py parsed and imported all 27 reported authored scenes plus shared theme, visuals and fonts: 30 successful modules. Every exposed P_PRIMARY binding resolved to canonical #ff344f, and none of the modules had an unresolved primary-accent global reference. The script disables Python bytecode writes and directs environment/cache/media output into this ticket's generated/accent-imports directory. The source list is preserved in accent-scenes.md; detailed per-module results are generated in accent-imports/imports.json.

A broader compiled-bytecode global-name scan additionally observed BRICK_FILL and MORTAR references in heating fundamentals during concurrent material implementation. These are outside the accent change and were reported to the root and material/scene owner; no source was edited by this agent. This is not a claim that every unrelated source path is free of defects. The initial symbol-table prototype also surfaced Python 3.12 inlined-comprehension locals; those were eliminated by inspecting LOAD_GLOBAL/LOAD_NAME bytecode. The compiler-created __annotations__ local is separately recognized by its SETUP_ANNOTATIONS instruction. No rendering or repeated style/font tests were needed for this read-only module import check.

The final additional permanent ticket input is validate-accent-imports.py. All test and validation processes have exited. Root owns final generated-output cleanup and ticket closure.

## Virtual Presentation Compact Tab Follow-up

The new compact left chapter/section tab contract passes all nine semio style tests in 10.047 seconds. The beat subtitle probe now addresses its text child within the square tab. Tests verify left alignment, chapter margins, section spacing, body/label type sizes, primary outlines/emphasis and neutral chapter text. The latest existing font suite passes all 13 tests in 14.699 seconds with no skips after these font API changes. Detailed presentation/runtime/native evidence and exact owned file list are in virtual-ui-tests.md. No test processes remain active.
