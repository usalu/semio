# Energy Clip Style

The energy teaching clip sources now use semio's canonical typography, color roles, compact spacing, square borders and continuous WindowSilhouette outlines with fused title chips. A shared Python theme interface reads the existing framework styling contract; it does not duplicate a separate palette or add runtime dependencies.

Anta body text loads from the checked-in Latin outline on every host. Native font registration uses one outline per family to avoid same-name subset collisions that produced missing German characters and punctuation on Windows. Mathematical glyphs use the existing bundled Libertinus Math outline. A standard-library OpenType cmap reader selects fallback glyphs from actual source coverage. Text SVG identities and rendered-segment provenance include the typography implementation and bundled source inputs.

Shared headers, captions, formulas, chips, quantity cards and scene-local callouts now share semio chrome. Institution intros and heating calculation headings use the same hierarchy. Energy certificate A+–H colors remain exact domain data. Physical diagram geometry is outside this styling task.

## Validation

- 19 scoped styling/font tests passed, including six language-neutral silhouette cases validated against independent Skia path unions, canonical tokens, typography, source fingerprints, native Pango vectors and exact FontTools source outlines.
- The source-outline oracle rejected 14 cases under a child-process reproduction of the former colliding-font policy, demonstrating that it catches the observed defect.
- The reported German paragraph context passed an additional native source-outline comparison.
- 27 rendering-runtime tests: 22 passed, 5 skipped (four opt-in integration checks and one host symlink privilege check).
- 86 source modules imported successfully; latest source syntax and final callsite checks are recorded in tests.md.
- Ten settled clause frames across all five video series, a complete shared institution intro frame, and a dedicated German/math glyph sheet were rendered and visually reviewed. These are diagnostic frames, not full-duration exports of all videos.
- The final calculation and ventilation modules passed syntax/import checks; the long calculation chapter heading passed a native viewport/chrome containment check.
- Scoped whitespace validation passed. No AGENTS.md edits, modifying Git commands, worktrees or IDEs were used.

## Coordination and Infrastructure

The maximum four-slot fleet used GPT 6.1 Sol high execution agents and low read-only scouts/auditors. The repo ticket catalog does not accept the current model name, so ticket metadata uses its generic codex entry while the agent model choices match the request.

The shell lacked repo on PATH. The existing native launcher was located and its full native repo installation restored through repo run repo-mcp:install. The tracked native-repo-path now selects this host's installed command. Repo MCP supplied goal discovery and ticket lifecycle operations. Other agents' concurrent logical changes were preserved.

## Evidence

See tests.md, style-implementation.md, visual-review.md, font-review.md and the supporting audit reports in this ticket. Diagnostic input scripts remain reproducible; generated environments, screenshots, logs and compiler/test artifacts are removed before closure.

## Closure

Generated environments, SVG/PNG diagnostics, raw logs and test artifacts were removed. Permanent reports and all reproducible diagnostic input scripts remain in the ticket.

## Complete Physical Video Preview

The requested complete physical fundamentals video was rendered at 640 × 360 and 6 fps, including the institute intro and all eight sections. The retained deliverable is [physical.mp4](./physical.mp4): 17:04.821, H.264, 12,219,804 bytes, with no audio stream. It was opened in the native player.

All 6,149 frames decoded successfully with PyAV and FFmpeg error checking; their count and duration exactly match the sum of all nine parts. Midpoint visual samples were reviewed for every part. Concurrent physical-source edits invalidated the first attempt; the affected sources were refreshed, and all final provenance matched the stable source revision. No production sources or shared environments were changed for this render follow-up.

Temporary caches, posters and raw logs were removed. The MP4, reports and reproducible render/verification input scripts remain.

## Primary Accent Follow-Up

Generic clip chrome, titles, notes, cards, chips, formula panels and annotations now use canonical primary pink. Scientific secondary roles remain deliberate. All 21 style/font tests and 30 module imports pass; all five series passed ten-frame visual review. See accent-summary.md for details and the retained primary-accent.mp4 review draft. The earlier full physical.mp4 remains unchanged; a separate concurrent section-assembly animation error prevents fresh canonical video certification.

## Continuous Virtual UI

The intro/title presentation has been replaced by a continuous five-window virtual UI with fixed navbar/footer, native window zooms, preserved narration and byte-identical shared lesson boundaries. The production renderer and all lesson wrappers use it. Component verification covers 63 methods (57 passed / six documented skips), and virtual-ui.mp4 is the verified 16 second physical-to-ventilation UI demonstration. See virtual-ui-summary.md and virtual-ui-render.md.

## Cursor Driven Interaction

The virtual UI now uses canonical cursor assets and event-driven hover/press/click states with complete parent emphasis. Fullscreen and restore require tab-control clicks before zooming; authored physics equipment activates through real committed clicks. Cursor activity travels through source/excerpt/chapter composition and participates in guarded cache publication. The retained cursor-ui.mp4 review film is 12.375 seconds with 99 frames and exact shared overview endpoints. Component verification covers 60 methods:54 passed and six documented skips. See cursor-summary.md, cursor-tests.md and cursor-render.md.

## Exact UI Component Rendering

The production teaching chrome now renders actual semio React WindowChrome, Navbar, Footer, Popover and Tree components with source CSS/fonts/icons and computed cursors. It uses canonical compact geometry and full canvas tiling, exports through device scale, and preserves click-before-zoom behavior and identical overview boundaries. The verified12.375second exact-ui.mp4 review demonstrates the corrected layout and interactions. Independent actual-UI pixel/geometry checks, all five encoded lesson envelopes, native authored controls and zero-touch browser provisioning pass. See exact-ui-summary.md and exact-ui-tests.md.
