# SVG Export Fidelity

## Finding

Draw SVG export converted its scene to SemioDrawingSnapshot and then dispatched the stdio bridge. That intermediate style contract dropped gradient paint, blend modes, fill rules, stroke caps/joins/dashes, image opacity and font size; its matrix decomposition also cannot retain arbitrary shear. The stdio SVG schema already has a typed element model that represents these properties directly.

## Change

The Draw SVG serializer now constructs that first-party typed SVG model and uses its XML writer. The public Drawing export helper routes directly to this serializer; it no longer registers and invokes the reduced semio-drawing bridge for SVG. The other semio-drawing conversion remains available for its own declared uses.

The Rust and TypeScript scene serializers preserve full affine matrices, all path commands including arcs, linear/radial user-space gradients with stop alpha, solid fill/stroke alpha, dash/cap/join/fill-rule, layer opacity/blending, image data and multiline Unicode text with its size. Authored absence of paint is explicit `none`. Text escaping is tested against a third-party XML parser. Gradient IDs and layer metadata occupy separate namespaces. Without an authored artboard, SVG viewBox uses the visible artwork bounds including negative coordinates; authored artboard dimensions are retained instead of rounded in the markup.

Scene projection now retains authored arc segments. Boolean/trace algorithms keep their own flattening where their computation needs it. A native regression checks that scene projection preserves original arcs and quadratics.

## Verification

- Registered Draw TypeScript regression reproduced the old bridge routing: **1 failed / 50 passed**.
- The corrected registered TypeScript suite passed **53 tests / 1,191 assertions**. Tests parse the neutral scene's real SVG with @xmldom/xmldom and check gradients, line geometry, full matrix coefficients, text escaping and baselines, image namespace/opacity and explicit disabled paint. Invalid dimensions and transform coefficients are refused.
- Native fixture, document SVG integration and scene curve tests were updated/added but have not run successfully against the current source yet.
- Existing native test handle 30810 and component build 43904 remain live. The first-party XML dependency was added after their initial Cargo graph construction; their eventual result must be inspected, and a new run may be needed after terminal completion. They were not restarted solely because of elapsed time.
- Scoped git diff whitespace validation passed.

## Limits Still Open

This is a scene export. It does not establish editable SVG round-trip fidelity or fully solve group isolation/opacity in the scene projection. System fonts remain external to the SVG file; PDF Unicode font embedding is separate. Native canvas arc rendering needs its own runtime verification. Large export progress/cancellation and actual rebuilt browser download remain unverified. IoFidelity remains Lossy while those representation limits remain.


## Numeric Output Validation

The public scene export boundary checked viewBox and matrix values but still accepted nonfinite opacity, geometry, paint, text size and image dimensions. Such values can arise from failed geometry calculations and produce unusable SVG values. A shared six-case fixture now drives Rust/TypeScript refusal regressions. The initial registered TypeScript regression reproduced the failure: **53 passed / 1 failed**, handle 65187 exited 1. Both serializers now refuse visible layers containing nonfinite numeric values before generating markup and include the layer ID in the diagnostic. The TypeScript gradient attribute map also has an explicit shared scalar type. The registered corrected suite passed **54 tests / 1,197 assertions**, handle 27100 exited 0. Native twin tests are authored but unverified. The Cargo lockfile already contains Draw’s direct first-party XML dependency; no lockfile edit was necessary in this continuation.


The existing native suite and component build remained live on handles 30810 and 43904 when checked after this change. Browser tab 4 is retained for the next workflow check. The shared preview briefly recovered after reload, then its DOM became empty again during ongoing HMR; no successful action or pixel assertion is claimed for this continuation.
