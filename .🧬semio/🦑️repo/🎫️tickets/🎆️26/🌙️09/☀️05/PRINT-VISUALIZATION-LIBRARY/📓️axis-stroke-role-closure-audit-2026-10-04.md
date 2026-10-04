# Native And Canonical Axis Stroke Role Audit

This is a read-only audit while root owns the final schema/inference stroke correction. No production source was modified. Findings describe the source and actual canonical runtime before root releases the role repair; they are not a final green certification.

## Exact Native Roles

`🖋️latex/semio-viz-guide.sty:69` resolves guide roles through the native theme chrome accessor. All stroke roles use hairline (`semio-tokens.sty:54`, currently 0.75 TeX pt). The canonical owned token is `chromeBorderHairline=1`; the generator multiplies by 0.75. In millimetres, this is `1 × 0.75 × 25.4 / 72.27 = 0.2635948526359485 mm`.

| Native Role | Default Paint | Draw Opacity | Width |
| --- | --- | ---: | --- |
| domain, endpoint caps and break marks | border-emphasized | 1 | hairline |
| tick | border-emphasized | 1 | hairline |
| tick-minor | border-normal | 0.6 | hairline |
| grid | border-normal | 0.45 | hairline |
| grid-minor style definition | border-normal | 0.22 | hairline |

Current generated token colours (`semio-tokens.sty:68–87`) are light border-emphasized/foreground `#001117`, dark border-emphasized/foreground `#f7f3e3`, and border-normal `#7b827d` in both appearances. Source inspection finds no invocation of the grid-minor role; the axis minor implementation draws tick-minor marks, including when SemioVizGrid requests minor. This bounded audit makes no claim that minor-grid geometry exists.

## Observed Canonical Counterexamples Before Repair

The actual owned `planVizChart` was executed against the retained neutral mutation chart with three major ticks, two minor ticks, grid enabled, labels disabled, in both appearances, with and without authored stroke/width controls. The command exited 0 and printed every actual line under `[DEBUG]` into ticket-generated `axis-stroke-role-audit.log`.

At this source revision `🧬️schema/💡️inferences/🖼️render/🟦️.ts:44` uses default `#808080` and fallback width 0.1 mm for all domain, cap and tick strokes. Minor marks inherit that same grey rather than border-normal; their opacity 0.6 is correct. The grid helper at line61 uses opaque `#c0c0c0` instead of border-normal at opacity 0.45. Neither default colour responds to appearance. Thus correct axis geometry/font publication alone does not establish native visual equivalence.

The custom case authored `stroke=#ff0000,strokeWidth=0.42` is honoured by domain, major/minor ticks and all widths. Its grid remained `#c0c0c0` because the canonical grid chose only `gridStroke` or its grey fallback. Native style resolution applies the general stroke first and then an optional gridStroke override; without gridStroke, the grid must therefore inherit authored red and retain opacity 0.45. Native authored width is explicitly millimetres and must override the converted token width on every stroke role.

The native labels/title additionally use chrome foreground rather than literal black/white; canonical current default fill at line44 is `#000000`/`#ffffff`. Font families now agree, but this colour-role difference remains visible in the source. Root should evaluate foreground along with the stroke role closure.

## Authored Override Precedence And Scope

`semio_viz_guide_style:nN` at native lines143–160 resolves theme role first. General stroke overrides every nontext role; gridStroke then overrides only grid. General strokeWidth overrides every nontext role in millimetres. Fill overrides label/title text. gridDash applies only grid. Raw style is appended last and can override previous TikZ choices, including opacity; canonical emitter likewise appends its opaque tikzStyle after computed paints. Raw backend style has no numerical drawing-scene interpretation.

There is no explicit native axis opacity key. Role opacities and authored CSS paint alpha in the canonical renderer should be distinguished from a public independent opacity option. Native custom colour admission declares six hexadecimal digits or a named TikZ colour (`semio_viz_guide_color:nN`, lines133–140), whereas canonical `vizParseColor` admits CSS named, 3/4/6/8-digit hex, RGB and HSL. This is a source-level grammar difference; this audit did not run a compiler failure for the additional colour forms and does not promote them to an observed runtime failure.

The schema ChartGuide options map is open to primitive values; its owned specification validator checks that schema and does not independently prove that every arbitrary primitive key is consumed by native guide keys. The current schema-owned axis default descriptor covers geometry and typography but lacks the native stroke role and role opacity descriptor at this pre-repair revision. Root is editing this owner; no duplicate descriptor or implementation was authored here.

## Source Identity And Verification Boundaries

Source read immediately around the actual four-case runtime:

| Owner | SHA-256 |
| --- | --- |
| native semio-viz-guide.sty | `623d4f9ee92c09983f784e484978d01776ec4ff8cf4c4a55712fbe3c93a053ed` |
| canonical render/🟦️.ts | `16f794a9982bf08adbb0741aebf23b37192acc2965a00359b092a175521e75d3` |
| print schema/🔣️.json | `9cba2d91cafda399e4a6b9f194df87326d0d98c0f856a0e8ff1bc0ae504b77b9` |

The native source roles and generated token declarations were read; no new native compilation was performed in this bounded audit. Root/native own the current paired actual compiler and independent PDF.js stroke/appearance gate. The final Bun/Chromium/Node worker replay will run only after root releases the corrected role production sources and will fingerprint its complete runtime closure before and after execution.

Fresh shared family helper inspection confirms root already replaced the SunCalc import with system createRequire and a private owned getPosition shape; native strict build is reported passed by its owner. No duplicate import fix was applied here.
The post-audit read confirmed native guide and canonical renderer hashes remained as recorded, while the main schema changed concurrently under root ownership to `43ae8c008bab069bd82fd56b043e14c222149af07443fc2fff285f5b10c83359`. This confirms the role contract is being edited during the audit; the table above identifies the earlier read and is not a stable final closure fingerprint.
A later read confirms root added schema-owned axis fields strokeRole=hairline, strokeColorRole=border-emphasized, minorStrokeColorRole=border-normal, gridStrokeColorRole=border-normal, textColorRole=foreground, minorOpacity=0.6 and gridOpacity=0.45. Numeric authored gridDash lengths are already millimetres in both native guide_dash:nN and canonical TikZ emission. Renderer release and its independent gates remain under root ownership.

## Root Released Role Closure

Root subsequently released the corrected native-equivalent axis roles and required closed theme chrome output contract, reporting its actual registered 174/174 differential gate green. The final retained three-host worker replay then exited 0 at 2026-10-04T11:30:34.934Z with identical initial/final 29-file runtime fingerprints `929794dfc1adea16993acd23d92ba688a10320ee355e5ecc3f99fb48aa7abada`. Complete source hashes, actual host versions, independent timer/cancellation/progress evidence and closed owned/AJV admission are retained in the final section of `📓️async-inference-audit-2026-10-04.md`. Thus the pre-repair role counterexamples in this audit are historical evidence, superseded by root's released implementation and final replay.
