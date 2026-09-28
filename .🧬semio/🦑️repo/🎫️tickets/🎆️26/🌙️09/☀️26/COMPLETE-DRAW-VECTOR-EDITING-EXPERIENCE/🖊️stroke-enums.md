# Typed Stroke Caps and Joins

The browser demo exposes an invalid cap=miter value as an empty Line Caps selection. Caps and joins are currently unconstrained strings in Rust, TypeScript and persisted schema. The intended contract is cap ∈ {butt, round, square}, join ∈ {miter, round, bevel}. A shared schema and 16 neutral cases cover every supported value and invalid cross-domain, casing and type inputs. Ajv independently validates the same cases in TypeScript.

Implementation will use scalar enums in Rust, string unions with checked parsers in TypeScript, typed retained storage accounting, and corrected authored demo assets. No fallback or legacy value support is intended.

## Validation checkpoint

The first TypeScript run failed because the checked cap/join parsers did not yet exist, confirming the added tests exercised missing behavior. After implementation, the full Draw TypeScript target passed: 111 tests, 18,095 assertions, plus the 44-case field-patch Ajv oracle. Rust enum codecs additionally test the same neutral cases through serde and the framework value codec. Rust native validation and rebuilt browser verification are still pending.

Retained stroke storage now treats cap/join as inline enum values rather than owned strings, including cloning, retirement, resource accounting and semantic digesting. The authored demo uses cap=butt. JSON, GraphQL, protobuf and DSL grammar definitions carry the supported enum values; SVG and PDF serialization use exhaustive typed values. The current build handle is 79298, writing build-draw-stroke-enums.txt.

The expanded TypeScript suite also validates enum values nested inside groups while preserving Boolean layer identifier references: 112/112 tests and 18,111 assertions passed. The native DSL tests now exercise each supported cap and reject cap=miter, cap=Round, and join=butt. Both ANTLR and Semio DSL grammar mirrors were updated; their transform production also now includes the already-implemented shear field.

Native run 59565 is waiting on the shared target/debug Cargo lock, with another workspace Cargo process actively compiling. Component build 79298 has active rustc children and is making progress. Neither job was restarted or terminated.

The component-dev task has succeeded within build 79298; descriptor generation and materialization are still running. One subsequent source edit replaced duplicate cap/join string matching in the field-patch admission guard with the typed enum parsers; a completed build should be checked for source freshness before activation. The full native test handle remains 59565 and must be polled to completion, not restarted because it is queued. Renderer 44523 and TypeScript 14796 are terminal success. Stable preview handle 76268 remains live on port 6065. After the build finishes, explicitly activate-draw-react-dev and reload the preview, then verify the demo's Line Caps reads Flat and cap/join edits undo correctly. The earlier materialize-only path does not update an existing activation.

Native run 59565 completed successfully: all 378 Draw tests passed, including the new stroke enum/DSL cases, in 29m44s including shared compilation and queueing. This validates typed retained ownership, serialization, mutations and history. The newly added coordinate-row projection test was authored after that test binary was compiled and has a separate pending run.

## Browser Verification

Activated the completed component with the registered `activate-draw-react-dev` target and `--skip-nx-cache` (95840, successful). The initial cached activation result did not establish receipt publication; the uncached run reported one component and a changed receipt.

In the stable browser at port 6065, clicking Orange Wedge now displays Line Caps **Flat** and Line Joins **Miter**. The caps menu offers Flat/Round/Square; the joins menu offers Miter/Round/Bevel. Selecting Round updates the projected cap to `round`. Selecting Bevel updates the join to `bevel`. Command-Z while the join control has focus restores `miter` while retaining the cap change. A second Command-Z restores `butt` (Flat).

Temporary `[DEBUG]` controlled-value diagnostics confirmed the exact sequence `butt, miter, round, bevel, miter, butt`; the browser reported no errors. The diagnostic source line was removed after verification. Evidence: `🗑️generated/stroke-browser-trace.json` and `🗑️generated/stroke-controls-verified.png`.

This validates menu values, document updates and one-edit undo for both controls. The tested path is closed, so this does not establish cap raster appearance on open paths. Native vector renderer parity, raster/export appearance and the broader appearance workflows remain open.
