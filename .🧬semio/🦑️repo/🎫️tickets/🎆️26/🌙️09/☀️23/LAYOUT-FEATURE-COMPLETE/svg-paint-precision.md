# SVG Paint Precision

Scope: Layout artifact export, not IFC.

The existing exporter rounded finite binary32 RGB channels to 8-bit hexadecimal before writing SVG. The neutral regression shows source0.1 becomes0.10196078568696976 after that conversion. Native SVG publication now delegates to a local paint module and emits percentage RGB plus separate opacity. The portable implementation admits the same binary32 model values and emits plain decimal attributes, including subnormal channels. Unsupported paint slots, wrong channel cardinality, and nonfinite colors refuse.

Actual executed results:8 neutral cases,6 refusals, and2 pixel probes from the unchanged SHA-256-pinned W3C SVG1.1 Second Edition painting-fill-01-t.svg. Independent JSDOM, color-string and Sharp libraries are test-only behind the SvgPaintOracle interface. Color attributes round-trip to the same stored binary32 components; raster comparisons use premultiplied channels so transparent 8-bit reader rounding does not create false precision claims. The official input probes verify its hollow and green rectangle centers; this is narrow paint evidence, not whole-suite SVG compliance.

The new owned verification is mounted in layout-pdf-streams with the repository root passed explicitly. Strict TypeScript check of this runner and the paint laws:512sources,0diagnostics. Native implementation and law parse cleanly; native compilation/execution remains unrun because the repo runner is unavailable.

Sources: [SVG1.1 data types](https://www.w3.org/TR/SVG11/types.html#DataTypeColor), [CSS Color3](https://www.w3.org/TR/css-color-3/#rgb-color), [official W3C test](https://www.w3.org/Graphics/SVG/Test/20110816/svg/painting-fill-01-t.svg). Embedded upstream fixture notices remain unchanged.

The exporter still has separate output admission and temporary formatting work; this change establishes paint fidelity and does not claim those owners are fully funded.

The official paint witness now parses the pinned source through JSDOM/color-string, rewrites rectangle paint attributes using the actual portable Layout writer, and compares both original and rewritten SVG center pixels to the official hollow/green expectations with Sharp. The unchanged upstream file remains SHA-256 pinned; no native full Layout round-trip claim is made. Combined closure now709sources/0diagnostics.
