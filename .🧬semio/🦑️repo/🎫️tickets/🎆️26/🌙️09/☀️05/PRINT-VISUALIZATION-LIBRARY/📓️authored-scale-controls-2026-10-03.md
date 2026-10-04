# Authored Scale and Paint Controls

The initial seven neutral authored scale vectors exposed missing sequential/diverging dispatch, numeric coercion of color ranges, descending diverging-domain arithmetic and ISO temporal-domain conversion. Further vectors failed for named-scheme reversal, declared Lab/HCL/OKLab and rounded numeric interpolation, CSS named/functional paints, exact fractional alpha and transparent endpoints, ISO timestamp values, authored tick values/count/format and UTC interval selection. Every failing behavior was observed before its production correction.

## Current Runtime Evidence

Direct Bun runtime comparisons now cover32 authored checks using independent d3-scale, d3-interpolate, d3-color and d3-time. The checks include148 CSS named aliases (one neutral schema-owned table),18 valid/invalid paint parsing vectors, piecewise and descending domains, clamp/reverse, Lab/HCL, OKLab knots, numeric round interpolation, fractional-alpha and transparent interpolation, ISO temporal positions, explicit ticks/formats and UTC day/week/month/year intervals and reversed auto-year domains. Native color/temporal counterparts remain under actual Tectonic verification by the execution agent. The registered exhaustive and strict TypeScript build passed again after the29th calendar check. Later CSS preset and explicit week/year vectors match their direct D3 oracles; final registered replay follows the shared result-contract edits.

The owned chart admission validator and independent AJV agree on all14 neutral validity vectors, including unknown scale keys, incorrect boolean controls, negative padding and valid authored controls. ChartScaleOptions is now a closed, typed schema-first vocabulary rather than silently ignored arbitrary keys.

## Exact Source Inventory

- print/🧬️schema/🔣️.json: closed ChartScaleOptions.
- print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts: matching scale control types.
- print/🧬️schema/📸️snapshot/📊️chart/🎨️color/🔣️.json: handcrafted shared CSS color aliases.
- print/🧬️schema/💡️inferences/📐scale/🟦️.ts: complete authored dispatch, multi-stop ramps, palette reversal, interpolation-space choice, precise time parsing and UTC tick/format controls.
- print/🧬️schema/💡️inferences/🎨theme/🟦️.ts: owned CSS parsing, CIE Lab/HCL interpolation and reuse of first-party OKLab mixing; precise alpha.
- print/🧬️schema/💡️inferences/🖼️render/🟦️.ts: guide consumption of scale ticks/formats.
- print/🧪️tests/🎨️authored-color-scales/{🔣️.json,🥒️.feature,🟦️.ts}: neutral vectors and independent differential adapters.
- print/🧪️tests/📜️chart-specification/🧫️fixtures/🔣️.json: additional scale admission vectors.
- print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts: registered authored checks.

All paint/scale derivation remains beneath schema inference ownership, with no external runtime dependency. Authored timestamp parsing uses UTC for zone-free ISO chart values. Raw numerical local-time helpers continue to represent their own explicitly documented D3 numerical contract.

## Catalogue Paint Derivation

The neutral preset paint batch observed missing named/function paint aliases before implementation, then observed every fractional-alpha preset as opaque. The catalogue inference now uses the same owned CSS parser and emits the shared native `\SemioVizPaintAlpha{name}{fraction}` registry. All ten preset paint vectors match independent d3-color RGB and exact alpha, including transparent and four/eight-digit hex. Identical RGB with different alpha receives distinct deterministic aliases; ordinary authored text is never rewritten as paint.

Additional inventory: print/🧬️schema/💡️inferences/📚️catalogue/🟦️.ts. The native grammar target now declares deps-tectonic, deps-tex and generate prerequisites in the existing print project configuration; fresh Nx discovery confirmed this ordering.
