# Guide Control Parity Audit

The numerical renderer now honors authored `guide.minor`, explicit `options.minor`, `minorTicks`, `minorSize`, and both `domain` and `domainLine` axis controls. Explicit minor options override the typed flag. Both domain aliases remain current public options; `domainLine` wins if both are authored, matching sorted native option emission. Axes draw the domain by default, grids omit it by default, and an explicit grid domain option overrides that default.

Minor ticks divide the segment between already mapped adjacent major positions. Four subdivisions create three interior ticks by default, with a 0.7 mm length, 0.6 opacity and no labels. This interpolation also holds for logarithmic scales and reversed positions. Domain lines use the first and last actual scale range endpoints, including narrower or reversed ranges. Major ticks remain first in the resolved output; minor ticks and the domain follow in native drawing order.

The existing conversion between point-sized text and millimetre scene geometry remains in place. Existing scene fixtures were handcrafted to include the newly honored default domains: demo line count 12 → 14, emitted statements 32 → 34, and band-axis primitive count 4 → 5.

## Test-Driven Evidence

Seventeen language-neutral guide vectors were added to the existing customization fixture and its existing registered scenario outline. They cover every orientation, default and custom subdivision/length, explicit typed-flag override, a single major position, logarithmic interpolation, both domain aliases and precedence, narrower/reversed scale ranges, and explicit grid domain controls.

Before changing the renderer, the fourteen initial guide vectors were admitted into the existing differential harness and compared with `d3-scale`. The registered command `bun x nx run '@semio-tech/print-viz-inference:test' --args='quick render'` reported **51/63 passed, 12 mismatches, zero errors**. Failures exposed the missing domains, ignored minor ticks and ignored `domainLine` precedence. The original implementation produced two lines where default minor ticks plus a domain required six.

After implementing the controls, the same fourteen-vector registered command passed **63/63**. After adding the three scale-range/grid-override vectors, registered exhaustive inference passed **425/425**, including **66/66 render checks**, with zero mismatches and errors.

Guide fixture subjects now also run through the actual asynchronous canonical `inferVizChart` worker. Completed plans therefore pass closed output admission before their coordinates and labels are compared with the independent D3 oracle. The final registered exhaustive replay of that worker integration passed **425/425**, including all seventeen guide vectors, with zero mismatches and errors (58.8 seconds).

The registered strict inference build passed after all production changes and exported 248 symbols. Its command was `bun x nx run '@semio-tech/print-viz-inference:build'`, with the worker included as an explicit type-check entry, and its runtime was 37.0 seconds.

## Native Semantics

The current `semio-viz-guide.sty` accepts both `domain` and `domainLine` as current boolean options. Its reset procedure sets subdivisions to four, minor size to 0.7, minor disabled and domain enabled. Its minor draw procedure uses `majorA + division/subdivisions * (majorB-majorA)` for interior divisions, without labeling them. The native range-bounds procedure reads the first and last scale range values. Native guide option emission writes explicit `options` after typed fields, establishing their override order.

The native owner is independently running the committed minor-tick fixture against its registered D3 numerical oracle. The parent ticket audit records that owner's final evidence; this report does not infer a native test pass from source inspection.

## Changed Files

- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️customization.json`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🥒️.feature`
- This retained ticket audit.

Native owner subsequently confirmed the measured guide minor positions [12.5,25,37.5,62.5,75,87.5] in the focused TeX probe. Evidence is retained in the native final verification report. The later complete TypeScript exhaustive gate (494/494) preserves all guide cases and the physical text unit conversion.
