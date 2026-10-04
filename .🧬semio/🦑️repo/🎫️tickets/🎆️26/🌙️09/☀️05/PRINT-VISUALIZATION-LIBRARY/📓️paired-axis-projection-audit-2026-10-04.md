# Paired Axis Projection Audit — 2026-10-04

Actual native and canonical inferred TikZ were compiled together for top, bottom, and angular guides using the tracked print toolchain. The helper is `🗑️generated/axis-visual-before/📜️script.ts`. The compiler produced `🗑️generated/axis-visual-before/🧪️probe-out/before.pdf`; the wrapper then exited 1 because this raw visual fixture produced no probe stream. This is a compiled visual counterexample, not a passing numerical gate.

Poppler page 1 confirms native and inferred Cartesian top/bottom ticks point outward. Page 2 shows native angular guide on the upper semicircle with its title above, while the inferred angular guide occupies the lower semicircle with its title below. Images are `🗑️generated/legend-visual-qa/axis-1.png` and `axis-2.png`. Blank trailing pages were excluded from evidence.

Native guide coordinates use a positive upward y axis. Angular geometry is `center + radius * (cos(angle), sin(angle))`, with positive counterclockwise angle. Radial geometry uses the same point and normal `(sin(angle), -cos(angle))`. Segment normal is `(dy/length, -dx/length)`. Explicit `at`, `from`, `to`, and `center` coordinates must be projected into the downward y axis of the canonical render plan through `height - y`; Cartesian frame placement already implements outward top/bottom direction.

Root owns the correction and its independent neutral oracle. Native style font selection is owned by this lane. The inferred axis title currently selects the default Anta family; native title selects SemioMono (Share Tech Mono).
## Actual After Render

After root's schema-first axis correction, the preserved paired helper was replayed in `🗑️generated/axis-visual-after/📜️script.ts` with geometry tracing enabled. Session 95422 terminated 0 and compiled all three native/canonical pairs. The current PDF is `🗑️generated/axis-visual-after/🧪️probe-out/after.pdf`; log is `🗑️generated/axis-visual-after.log`.

Poppler page 2, `🗑️generated/legend-visual-qa/axis-after-2.png`, confirms both angular axes now occupy the upper semicircle with titles above, and both titles select the same Mono role. This is actual visual/backend evidence; independent numerical axis checks remain owned by root's neutral fixture and D3 oracle.

The same actual image exposed a remaining styling difference: canonical domain/ticks were thin gray, while native domain/ticks use emphasized border color and the hairline stroke token. Inspection confirmed the canonical helper's literal gray/gridMinor defaults diverge from native theme roles. Native roles are border-emphasized/hairline for domain and major ticks; border-normal/hairline with opacity 0.6 for minor ticks, 0.45 for grid, and 0.22 for minor grid. Root owns schema-first role-default closure in the axis renderer; no native change is required. The after projection/font image is not a complete axis paint-parity claim.
## Current Canonical Closure Gates

The registered no-cache canonical exhaustive route terminated 0 with 630 / 630 checks, including 171 render checks, in 31.7 seconds. The registered strict build terminated 0 in 1 minute 26 seconds, checking both canonical browser/worker inference and the separate native grammar runner/helpers and publishing 250 symbols. Exact temporary logs are `🗑️generated/canonical-current-final-exhaustive.log` and `🗑️generated/axis-role-current-strict.log`.

The actual PDF paint baseline reproduced 9 canonical tick failures out of 18 native/canonical ticks: gray `#808080` at 0.28346 PDF points versus the native emphasized `#001117` at 0.74721 PDF points. This is separate from the successful earlier geometry/font projection proof. Neutral paint inputs are retained in `🔣️axis-style-proof-2026-10-04.json`; the actual paired compiler/PDF.js operator oracle is `🗑️generated/axis-style-proof/📜️script.ts`. The final 5-case actual PDF paint gate is running; no passing result is claimed until terminal.

## Actual Axis Paint Closure

Session 91020 terminated 0 after all 5 authored paired cases. Actual PDF.js path painting passed 30 major ticks (3 native and 3 canonical per case), 24 minor ticks and 12 grid lines across the light/dark override cases. Default strokes are 0.75 TeX points, measured as 0.74721 PDF points; authored 0.6 mm strokes measure 1.7008 PDF points. Light default emphasized paint is `#001117`, dark is `#f7f3e3`. Overrides paint major/minor ticks red, grids blue, with minor opacity 0.6 and grid opacity 0.45. Opaque authored `style=opacity=0.25` paints both implementations at 0.25. Path lengths are independently measured from PDF coordinates in millimetres and compared at 0.001 mm tolerance; stroke width tolerance is 0.0001 PDF points and opacity tolerance is 0.00001.

The oracle consumes the actual PDF.js graphics state and stroked paths, with D3 color normalization. It does not inspect the renderer's expected style objects. The prior actual PDF red gate found 9 mismatches, while the final compiler/backend gate is green. Source ownership remains the root-owned existing axis renderer/schema/theme; this lane added only retained neutral authored inputs and ticket-local compiler/PDF oracles.

Poppler rendered and this lane inspected page 1 of both `light-overrides` and `dark-overrides` PDFs. Native and canonical red axis stroke, minor ticks and blue grids agree visibly. Images: `🗑️generated/legend-visual-qa/axis-style-light.png` and `axis-style-dark.png`. The framed native figure includes its authored caption/frame; the canonical emitted plan is rendered below. Blank trailing pages are excluded from evidence. Exact log: `🗑️generated/axis-style-pdf-after.log`; compiled PDFs reside in `🗑️generated/axis-style-proof/<case>/🧪️probe-out/<case>.pdf`.

## Strengthened Actual Label Paint Counterexample

A separate PDF.js graphics-state/showText oracle validates the actual numeric tick labels rather than only paths. It passed all 24 default/light/dark/override label colors against the neutral schema-owned theme roles. In the authored opaque-style case, the native labels carry fill opacity 0.25 while canonical labels remain 1. The first 5-case path gate remains a passing stroke proof, but it does not certify opaque style on labels. This strengthened gate terminated 1 with 3 real canonical label opacity mismatches. Log: `🗑️generated/axis-label-paint-after.log`; temporary oracle: `🗑️generated/axis-text-paint/📜️script.ts`. Root owns the existing axis renderer correction before the final canonical/full-native replay. No native stylesheet change is needed.

Latest source inspection confirms `axisGuideItems` already forwards the opaque style to text and the existing TikZ emitter appends it. The mismatch comes from that emitter's unconditional explicit `text opacity=1`, which overrides inherited authored opacity in PGF node text. The correction belongs to the existing shared text emitter's native option precedence, rather than parsing arbitrary native style syntax or adding another guide implementation.

The strengthened label-opacity counterexample was also rasterized and viewed: `🗑️generated/legend-visual-qa/axis-opacity-before.png`. The native labels fade with the axis, while canonical labels remain fully dark. A focused single-case replay is now available in the retained compiler helper by passing `light-opaque-style`; the current source correction and subsequent green result remain pending.

## Actual Text Opacity After Correction

The root-owned existing shared text emitter now omits the forced unit text opacity while preserving nonunit computed paint/authored numeric opacity. Focused compiler session 86781 terminated 0, then the independent actual PDF label oracle terminated 0 for all 30 labels across the retained 5 cases. The opaque-style case now measures 0.25 fill opacity on all 6 native/canonical labels. Logs are `🗑️generated/axis-opacity-fixed-compiler.log` and `🗑️generated/axis-label-paint-fixed.log`. The unchanged 4 other actual PDFs remain valid source-scope proofs of their recorded default/override colors. Current registered strict/exhaustive closure is 250 symbols and 633 / 633 checks, terminal 0.

Poppler after-image `🗑️generated/legend-visual-qa/axis-opacity-after.png` was viewed and confirms the canonical labels now fade with the line just as the native labels do. This supplies actual visual evidence alongside the independent graphics-state assertion.
