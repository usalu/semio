# Energy Clip Style Test Contract

The language-neutral contract is `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🔣️.cases.json`. It records canonical WindowSilhouette vertices for plain, fused-title, separated, merged, clamped, and top/bottom chip outlines. The same regions are independently unioned through Skia PathOps, already installed by Manim, and compared by symmetric-difference area. This checks a continuous filled silhouette without an internal title seam.

The Python adapter also verifies semio palette roles, Anta UI type, spacing, chrome stroke/fill tokens, square chip corners, and readable fused stat cards. `🏗️authoring/📜️script.ts` registers the adapter alongside the existing clip and font suites.

The tests were authored before the theme and geometry implementation. Test environment provisioning uses the existing authoring Python project and test dependency group through `repo tools exec uv`. Environment, text cache, and media output are confined to this ticket's `🗑️generated` folder. Execution results will be recorded below when available.

The initial stdlib structural check failed with `Missing canonical fused silhouette implementation` before implementation began. The independent architecture contract appeared concurrently at the initially selected test path; those files were preserved and the semio contract moved to its own subfolder. The architecture baseline had five failures and one error in room material, radiator scale, people outlines, and open-window geometry; those concern separate concurrent work.

The first executed semio suite ran five tests. Four passed: all six portable silhouette vectors, independent Skia symmetric-difference verification, exact fused polygon frame geometry, and square chip/stat-card containment. The remaining expected failure was `BODY_FONT: Georgia != Anta` before the root agent's font migration. Manim 0.22.0 and its native PathOps dependencies provisioned successfully through the existing authoring dependency lock.

The updated semio suite passes all six tests, including canonical type scale and a regression proving every theme token, font catalog, and registered outline is fingerprinted by segment render state. All 95 energy Python source files also parse successfully.

The bundled-font contract replaces host-specific Georgia assertions with Anta, Kelly Slab and Share Tech Mono source-family/path vectors. FontTools now checks every registered family's complete cmap union, including Greek/math symbols, German text, emoji and an astral mathematical character, against the production stdlib SFNT parser. That independent test passes on this Windows host in 1.924 seconds without host-font skips.

Early full text runs exposed expensive Windows font enumeration in Manim/Pango. The root agent replaced host discovery with deterministic bundled coverage, added the existing Libertinus Math outline for missing mathematical glyphs, registered only requested native families, and disabled unnecessary Manim missing-font enumeration. A fresh run then completed all six semio style checks and proceeded through the font regression suite.

The clip runtime suite completed 27 tests in 12.846 seconds: 22 passed and five skipped. Four skips require the real-render integration flag, and one requires unavailable Windows symbolic-link privileges. All 86 clip source modules imported successfully in the provisioned environment.

The synthetic font contract exercises direct format-4 delta mapping from U+0000, indirect missing glyphs, delta wrap to zero, nonzero indirect mappings, and format-12 astral range boundaries. Handcrafted minimal SFNT bytes are independently decoded by FontTools and compared with production membership output; this test passes in 1.445 seconds. Exact bundled FontTools source coverage replaces fallback-aware Cairo ToyFontFace assertions, whose host resolution disagreed for subscript two, double implication, and contour integral despite valid checked-in glyphs. Actual Pango SVG/vector comparisons remain intact. The glyph suite batches related symbols into single renders to preserve geometry and coverage assertions while avoiding repeated Windows font-map setup.

## Initial Verification

The combined semio style and font suite passes all **17 tests** in **231.787 seconds**. This includes public `Text.set_default()` recovery, exact bundled glyph coverage, explicit font ranges, independent synthetic and real-font FontTools parity, Pango vector equality, text cache identity, low-resolution text canvas geometry, theme source fingerprinting, canonical type scales, square chips, and fused cards. No tests were skipped in this combined suite.

The separate clip runtime suite passes **22 tests** with **five documented skips**, and **86 clip modules** import successfully. After the final shared style edits, **96 energy Python sources** parse successfully. Modified test-source paths are at most **105 characters**.

Test ownership changes are the two files in `🎨️visuals/🧪️tests/🎨️semio-style`, the two files in `🔤️fonts/🧪️tests`, and the authoring `📜️script.ts` test registration. Concurrent architecture test files were preserved. The provisioned environment remains in `🗑️generated/py` for the root agent's visual QA and must be removed with generated artifacts when the ticket is complete.

## Native Glyph Outline Follow-up

Visual QA exposed missing German and punctuation glyphs despite the initial green suite. Checking font cmaps and comparing two Pango renders did not prove which same-family face native Pango actually selected. The Anta symbols subset contains ASCII letters while omitting German letters and punctuation; its math subset also contains ASCII letters while omitting German letters. Registering all these outlines under the same family allowed the native renderer to select a partial face.

The root agent narrowed native registration to one complete teaching face per family: Latin for Anta, Kelly Slab and Share Tech Mono, complete for Noto Emoji, and the independent Libertinus Math font. The neutral glyph contract now includes actual German/punctuation source outlines, mathematical symbols, and the reported full German caption context. FontTools draws the expected checked-in outlines directly through SVGPathPen. Actual native glyph boundaries are uniformly normalized and compared with these independent contours using symmetric dense-sampled Hausdorff distance. The threshold is 0.04 at normalized height 2, allowing native grid fitting while rejecting different glyph shapes. A `.notdef` control verifies that the same normalization rejects a missing-glyph box.

The standalone independent native source-outline test passes in 22.433 seconds after the registration correction. The first attempted polygon-area comparison was unsuitable for compound native glyph contours and was replaced by the boundary comparison before inclusion in the final suite. `font-collision.py` preserves a child-process-only reproduction of the former conflicting registration policy to exercise the new oracle without changing source files.

## Final Verification After Visual QA

The updated combined style/font suite passes all **19 tests** with no skips in **144.292 seconds**, including the independent FontTools-to-native contour oracle and the `.notdef` rejection control. The additional reported full-caption context passes its targeted native outline check in **6.712 seconds**. The boundary comparison samples **257 points per cubic curve** and keeps the actual glyph's aspect ratio.

The former all-Anta-subsets registration policy was restored only inside the `font-collision.py` child process. The new contour oracle correctly rejects **14 faulty glyph subcases**, including German letters and the colon, smart quotes, and period in the reported caption. Their normalized distances range from **0.176 to 1.251**, well above the **0.04** accepted threshold. This reproduction proves the new regression catches the rendering defect missed by the initial cmap/Pango-reference checks. The reproduction process completed and leaves production registration unchanged.

All **97 current energy Python sources** parse after the final physics chrome changes. The test agent has no remaining processes. All generated environment and media files remain available for the root agent's final visual QA and cleanup. The added permanent ticket input is `font-collision.py`.

## Final Energy Balance Native Preview

Ran the ticket preview script scoped to `energy_balance` through `repo capacity run --memory=6144 --cpu=1`, with its own video text cache. The process captured two settled clauses from `Beat1_Bilanzgrenze`, exited successfully, and released its resources.

Visually inspected both `🗑️generated/o/qa/energy_balance-0.png` and `energy_balance-1.png`. The Anta title and subtitles render correctly, including `Gebäudes`, `Hülle`, and `übers`. Colons and periods render as glyphs, with no tofu boxes. The two-line second caption fits its square frame and stays separate from the diagram. Semio dark background, light foreground, secondary teal, shared spacing, and square chrome remain consistent. No visual defect required a source edit. The generated environment and previews remain for the root agent's final QA and cleanup.

## Final Calculation and Ventilation Audit

After the final math type-scale and chart-column normalization, the affected heating calculation and ventilation heat-flow modules both parse and import successfully. The scoped `validate-final.py` check creates the shared chapter title `Die Hauptgleichung des Heizwärmebedarfs (DIN V 18599)` at 1280×720, verifies all title bounds remain inside the viewport with the shared margins, and verifies its text remains inside its square chrome. The final run completes successfully in 4.221 seconds using the already generated text cache.

Visually inspected `🗑️generated/o/qa/final-title/calculation-title.png`: the full heading fits on one line, has consistent padding and shared type size, and renders the umlaut correctly. No production code change or heavy font-suite rerun was needed. The new ticket input `validate-final.py` preserves this scoped check. All test-agent processes have exited; generated assets remain for root cleanup.
