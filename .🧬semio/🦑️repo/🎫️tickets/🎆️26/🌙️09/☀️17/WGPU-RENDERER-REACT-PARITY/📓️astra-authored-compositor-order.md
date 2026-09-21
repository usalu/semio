# Authored Compositor Order

The checkpoint 19 browser journey shows earlier General content overprinting later menus and World pixels covering opaque Actions and window options. The prepared renderer has two independent ordering defects: it visits every UI layer before every scene pass, then composites all glass before replaying all foreground content.

The glass regression uses the actual `PreparedRenderJob` packet and a language-neutral fixture for partial overlap, nested glass, and parent content after a child closes. An independent tiny-skia raster agrees with the fixture's authored pixel samples. UI13 ran one test and failed with `base, panel-text, popup-text, parent-after, panel, popup`; the required order is `base, panel, panel-text, popup, popup-text, parent-after`.

The repair establishes charged layer boundaries at glass and scene insertion. The presenter will walk one command stream into the retained composite. Each glass command snapshots the accumulated composite, advances its blur one mip per opportunity, and composites that region before continuing. Window surface acquisition and presentation remain inside one terminal opportunity.

UI14 initially failed to compile because its new test addressed an absent ScenePass3d field. The corrected scene fixture identifies its actual pass by viewport. Its real RED run failed the intended ordering assertion: one test run, one failure, 665 filtered tests. UI15 then ran 664 tests with one old layer-count expectation failing. The fixture now explicitly accounts for the charged glass boundary.

UI16 passes all 664 tests with no skips in 1.777 seconds (Nx 17 seconds). Both actual prepared-command regressions pass. The presenter now encodes every scalar into the accumulated composite and snapshots that composite at each authored glass command. The old foreground replay and containment heuristic are removed. The watchdog retains the phase, command, and blur-mip cursors. The isolated P5d source and hostile-mutation gate also passes through Nx. Its outdated deadline units and external test-source loading were repaired while retaining the same bounded-work checks.

UI17 additionally failed the intended scoped overlay law. The repair now visits a layer's normal content, its ScenePass, its overlay content, then the following layer. The normal-raster terminal follows the same scene boundary. UI18 passes all 665 tests with no skips in 2.643 seconds (Nx 25.5 seconds). The browser-worker suite passes all 156 tests across 11 files (Vitest 4.52 seconds; Nx 9.8 seconds).

Native136 is compiling again after correcting both Find type references and the watchdog fixture's old five-counter tuple. The browser bundle build is in progress. This document records no new browser runtime acceptance yet.

Generated evidence: `🗑️generated/astra-runtime/ui13-glass-stacking-red/run.log` and `🗑️generated/astra-runtime/ui14-scene-stacking-red/run.log`.
