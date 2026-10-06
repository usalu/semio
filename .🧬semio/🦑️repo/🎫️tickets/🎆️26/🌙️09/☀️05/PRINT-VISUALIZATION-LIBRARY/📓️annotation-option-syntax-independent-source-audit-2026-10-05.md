# Annotation Option Syntax Independent Source Audit

Read-only current native source SHA256: 39F9C80D4D58268C299A3DDBB73F02E6A90F040CD2AD185469AE4166EB593C4D.

The annotation namespace declarations at lines 67–92 own at/to/from/offset comma lists, x/y/x2/y2 data coordinates, named xScale/yScale, literal text, width with empty natural-width sentinel, anchor/shape identifiers, polygon points, paint color, style, five numeric controls and three booleans. Reset lines 96–120 preserve the empty width sentinel and explicit defaults. Text-node lines 222–233 protect literal caption tokens with exp_not:V; style is appended as a native option-list fragment at lines 210–211. These consumers justify separating list structure, identifiers, expressions, paint and literal caption payloads.

The native namespace declares no fontFamily, fontWeight or fontStyle keys. Generic-model syntax annotations for those fields require a source-bound lowering or existing font transport; they cannot be treated as directly supported native annotation options. This is a source-scope finding, not an executed compiler failure. Native was notified before annotation candidate release.

Prepared annotation counts (246 families, 5832 descriptors) demonstrate classification coverage only. This audit does not claim executable encoder or physical compiler success; the candidate remains under active semantic and emitted-source validation.
