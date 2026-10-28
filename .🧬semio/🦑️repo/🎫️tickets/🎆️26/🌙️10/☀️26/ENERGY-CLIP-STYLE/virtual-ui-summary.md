# Continuous Virtual UI Completion

The institute intro and centered title placement have been replaced by a shared virtual semio application. Its persistent navbar and footer surround a canvas with five lesson windows in curriculum order: physical understanding, ventilation, heating, cooling and energy balance. Each lesson enlarges its own window, presents the authored content, and returns to the same overview. The window silhouette, primary-accent border, fused title chip, spacing and bundled typefaces use the existing semio interfaces. Chapter and section context appear as compact left-aligned content tabs instead of the former centered title block.

The presentation has an explicit English/German locale and a language-neutral JSON/schema layout contract. Full authored frames are contained without cropping, preserving citations, formulae and captions. Source narration is delayed for the incoming zoom and preserved with silent transition padding. The concurrent speaker/spectator implementation is preserved and participates in cache settings; it was not replaced or simplified.

## Implementation

The canonical curriculum/schema and render plan no longer contain intro records. The obsolete institute intro source, its speech generator and the old physics section composition entry were removed. All five lesson entry points and selected-chapter outputs use the same presentation envelope after canonical source/chapter assembly. Presentation identity includes the explicit locale and the complete chrome/font/profile source closure. Candidates pass both presentation and nested authored-input validation before publication, and a failed certificate write cannot retain stale certification. Previous movies remain available when an input change rejects a candidate. Expensive operations expose phase/native-frame progress and owned process cancellation. Temporary native frame work is cleaned on every exit. No runtime dependency, AGENTS.md, shared Python environment or Git state was changed.

Root coordinated integration with two GPT 6.1 Sol High execution agents and two GPT 6.1 Sol Low audits, using all four concurrent slots where independent work was available. Other agents' audience/scientific geometry changes were preserved.

## Verification

The component verification totals 63 test methods: 57 passed and six documented environment/opt-in skips. The new presentation tests themselves use real native geometry, FFmpeg, PyAV, Pillow and NumPy with no skips. All five native narrated test movies contain exactly 24 frames / six seconds, preserve every one of eight original source frames in order, retain the 440 Hz narration tone/amplitude and exact six-second audio timeline with silent pads. All ten decoded first/last boundary frames are byte-identical. Portable layout and localization cases, source-cache fingerprints, cancellation, candidate rejection and certificate I/O failure regressions pass. All 13 font tests and nine compact-style tests pass. See virtual-ui-tests.md for accurate per-suite counts and independent oracle details.

Independent source/visual audits found no remaining correctness concerns after fixes; see virtual-ui-audit.md and virtual-ui-continuity-audit.md.

## Review Film

virtual-ui.mp4 is a 16 second, 960×540, six fps review film showing the physical-to-ventilation handoff using two native authored clause snapshots per lesson. It is a focused UI demonstration, not a regenerated full curriculum. Both eight-second components contain 48 frames; all 96 joined frames decode successfully. The fixed navbar/footer pixels remain identical throughout. The physical last frame is exactly the ventilation first frame, and the joined frame count equals the sum of both components. FFmpeg independently decoded the film without errors. It was opened in the native player. virtual-ui.png shows the overview and virtual-ui-focused.png shows the focused physical window. See virtual-ui-render.md for media hashes and exact measurements.

All generated logs, failed attempts, diagnostic media and native PNG sequences under this ticket's generated folder are removed at close. The three review media files, permanent reports and reproducible input scripts remain. The older physical.mp4 and primary-accent draft artifacts were not overwritten.
