# Native Terrain Brush Strokes

The existing World3d shell captures the pointer on the registered Terrain editor. The interaction authority picks the authored elevation grid through the shared terrain query. It retains ordered grid cells locally while the primary button is held, samples at a spacing derived from the brush radius, and sends one scoped `editElevation` on release. The captured drag sends neither intermediate mutations nor a camera action. The action uses the app’s registered command, so the reducer and Store own the mutation and its History entry.

Escape, capture loss, a missed terrain hit, a terrain or brush revision change, and a stroke that exceeds the 64-stamp budget discard the buffered points. The native host also cancels on window leave, since some window systems stop reporting releases outside the window. The path and budget are transient. The viewport updates on release when Store publishes the new terrain.

The neutral journal fixture covers taps, an ordered turn, duplicate hits, capture loss, blur, source and brush changes, a missing terrain hit, and the expanded stroke budget. The regular top-down camera maps viewport pixels to terrain samples so the fixture can be checked independently using GDAL and SciPy.

The patch passed `repo tools exec git --light -- apply --check` in the shared checkout. No production input was modified while the active compilers were reading the workspace. Use the existing `world_host_journal_sequences` target with filter `native_terrain_brush` to run the fixture after application.

The independent GDAL and SciPy run passed. It placed the four bend samples at `[[16,16],[18,16],[18,19],[16,19]]`, confirmed the duplicate samples stay in cell `[16,16]`, found the missed ray beyond the elevation bounds, and counted 91 requested stamps in the rejected over-budget path.

The repository’s pinned Rust compiler parsed each revised Rust source with `-Zparse-crate-root-only`. This check did not resolve cross-crate types or run the journal test.

After application, the shell capture-loss path was switched to the existing shell diagnostic on queue overflow. The shell pointer crate does not depend directly on the bounded-action package. The focused native accumulator separately passed two pinned Rust tests covering an ordered turn, an exact release below the brush spacing, and rejecting a 91-stamp path.

The journal build first waited at capacity admission while machine swap exceeded its ceiling. It then reached source admission and reported 4,032 active lines in the World3d interaction crate against its limit of 4,000. The stroke accumulator moved into the existing world model, and the action encoder moved into the world cursor publisher. The canonical source counter measured 3,970 active interaction lines with `testkit` after the change. The model (3,538), cursors (3,096), frame-actions (1,767), and shell-pointer (3,485) also fit their 4,000-line limits. The scoped `world_host_journal_sequences` run passed its `native_terrain_brush` regression. One test passed with 17 filtered out. It ran all nine neutral fixture cases through the real World3d host authority, including input reservations and action publication. Running the same native journal binary without its filter passed all 18 camera, selection, hover, pointer ownership, and terrain stroke tests.
