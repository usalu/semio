# FEM 2D canvas text

The React Canvas2d paint path accepts the catalog font families and requires a fill or stroke. The FEM 2D results window supplied neither for analysis status, reactions, mode captions, animation, or the stress legend. Its model window also published text without those fields in the retained visual packet and live scene. The renderer throws while painting such a layer, stopping the frame.

Results and model scenes now share the FEM 2D text layer style. The retained model packet writes the same style directly to its fixed pages. The mounted results window emits one status layer and uses the static-displacement preview only in static mode.

The mounted static results scene paints the Tx and Ty reactions already recovered by the retained numerical job, filtered to the support's constrained directions. Its label layout is shared with the standalone results renderer, which now reports moments in N·m.

`🧪️canvas-text-paint.ts` uses the real Canvas2d renderer in Chromium via Playwright. The previous layer was refused for its missing catalog font. The corrected status painted 693 pixels on a transparent canvas. The Rust label and mounted packet regressions assert that the producer supplies both fields and projects only constrained reactions. Rustfmt parsed all six edited source files. `🧪️browser-smoke.mjs` exercises boot, both canvases, the first static analysis, an inspector edit, and recomputed deformation, and verifies the demo's positive 12000 N reaction in Live Load.

The first navigation to the 02:56 component returned a Vite dependency 504 and an empty shell. A second and third navigation loaded the shell, then refused `input #1 setActiveExample` with `original-turn.native-return-refused: semantic` before the model or results surface existed. The FEM 3D ticket records the same failure on that shared component publication.

The browser probe's optional `--capture-native-refusal` records the first WIT exception using a transient bridge response in that test profile. On the stale component, the original stack was JCO `finishCompletion` → `reactor.poll` → component bridge. The payload contained the semantic tag and numerical resource accounting, with no detailed Rust fault available from that publication.

The standalone results tests had decoded the Canvas2d spine without its externally stored layer carrier. The fixture now reassembles the real built children through the scene lane merger, so the existing geometry, caption and playback assertions exercise the host-visible payload. Static rendering resolves a missing source before the solver, preserving the localized empty-document placeholder.
