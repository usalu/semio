# Final Axis and Closure Read-Only Audit — 2026-10-04

Scope: canonical authored native axis coordinates, label rotation and anchors, polar default frame, and schema-owned guide fonts. No production files edited.

## Actionable Finding

The native CCW label rotation is not projected into the Canvas y-down scene convention. In `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts:62`, positive `labelRotation`/`labelRotate` is assigned unchanged to item.rotation. The canonical TikZ emitter at line 432 negates that rotation. Native `semio-viz-guide.sty:695` emits the positive native rotation. Therefore native +30 produces canonical TikZ -30.

Actual Bun inference using the existing `axis-native-at-bottom` specification and `options.labelRotation=30` returned complete=true, text rotations [30,30], and two `rotate=-30` TikZ occurrences. This is runtime evidence of the canonical behavior, compared with the native source contract; it is not a compiled native rotated PDF comparison.

Recommended fix: negate authored native axis label rotations when constructing scene items, while preserving the global scene-to-TikZ emitter convention. Add a language-neutral asymmetric rotated-label fixture and native/canonical comparison of rotation, not merely tick endpoints.

## Checks and Limits

- Explicit authored pairs are projected as (x,height-y). Radial and angular normals use projected native sine signs. Segment normal is consistent with native reflected normal.
- Explicit labelAlign start/middle/end maps to west/center/east in native code and scene anchor with middle baseline; automatic dominant-normal mapping is consistent after reflection.
- Native polar defaults use full figure midpoint and max(1,min(width,height)/2-8). Canonical default center uses full figure midpoint; canonical outer-radius expression subtracts min(frame.x0,frame.y0). With default margins left=16/top=8 this equals native pad 8. Custom chart margins produce a different canonical radius, but native figure has no matching chart-margin contract; no invented parity defect is asserted for this.
- ChartGuide-owned font defaults are imported into frozen VIZ_AXIS_DEFAULTS / VIZ_LEGEND_DEFAULTS in chart snapshot TypeScript lines 9/12. Axis labels/titles read these defaults and resolve through owned print font family/selectors. This audit did not rerun the existing full font probes.
- Existing paired helper compares native tick endpoints after inverse projection and native/inferred title centers, plus PDF page text. It does not compare rotated label geometry. Its current generated proof.json was not present at inspection, so no paired compiler PASS claim is made here.
- Mutation/inference and command closure were not comprehensively re-executed by this read-only audit; root-owned recorded suite/async evidence remains separate.

## Native Common-Frame Polar Default: Confirmed Mismatch

Follow-up scope uses the native common frame itself: figure 100×80; left/bottom margins 8, top/right margins 2. Cloned existing axis-angular fixture, deleted explicit outerRadius and set that exact frame. Actual Bun owned inference returned complete=true, frame {x0:8,y0:2,x1:98,y1:72}, first tick (88,40)→(90,40), second tick (50,2)→(50,0). Thus inferred radius is 38.

Native guide reset at semio-viz-guide.sty lines 378–379 uses max(1,min(width,height)/2-pad), pad=8 (line26), giving radius32. Independent third-party d3-shape pointRadial(pi/2,32) actually returned [32,0]; adding native center (50,40) then reflecting y gives first tick (82,40)→(84,40). This is the same native/common frame, not hypothetical custom-margin compatibility. The canonical formula subtracts min(frame.x0,frame.y0)=2 instead of native pad8. Root notified to repair schema-owned native polar default and add a neutral omitted-outerRadius vector. No production files changed by this audit.
