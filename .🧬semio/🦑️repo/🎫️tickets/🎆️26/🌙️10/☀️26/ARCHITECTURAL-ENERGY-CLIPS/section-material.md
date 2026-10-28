# Section Material

Shared architectural cut construction uses the theme's `light` color, opacity 1, stroke width 0, and stroke opacity 0. Glazing and sash lines keep their distinct semantic styling. House ground hatches and the extra datum outline were removed.

The portable material contract/schema and JSON cases were written first. Baseline run of six methods failed nine cases plus the missing public style interface; the old 0.7 fills caused maximum raster channel error 74. Applying opaque unstroked styling alone still failed three house raster scales, with 46–120 dark joint pixels and maximum error 62.

`section_assembly(*parts)` retains original material objects and adds one combined fill of normalized closed subpaths. Its updater caches live geometry, opacity and color; unchanged sources do not reconstruct contours. Fully opaque base material fills in one Cairo pass. Faded, moved, replaced and removed sources update the fill; semantic window objects remain separate. Shared house exposes `material`, while room exposes the assembled `shell`. Whole-group transformations retain original geometric references.

## File Inventory

- Updated `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py`.
- Updated adjacent `🔣️.json`.
- Updated adjacent `🧬️schema/🔣️.schema.json`.
- Created adjacent `🧪️tests/🐍️sections.py`.
- Created adjacent `🧪️tests/🔣️sections.json`.
- Created this ticket report, `section-material.md`.

Validation passed: nine methods in 30.209 seconds. After moving lifecycle and transformation parameters into the portable JSON fixture, the three affected methods passed again in 35.005 seconds. Coverage: actual Manim styles, raster equality to independent NumPy polygon-union masks and a Manim solid-color oracle, SciPy connected components, three house scales, four room aperture variants, nonpixel transforms, mixed path winding, partial fades, moves, replacement/removal, copying, Create and FadeIn.

The composite is a cached single Cairo fill of normalized closed subpaths, not a permanent static underlay. Supported live operations are tested through the retained base references and removal from the assembly. Existing geometric anchors and floor/wall/roof contours remain unchanged. Original child ordering is retained and the composite is appended. Shared window sash/glazing lines remain independent. No external dependencies were added.

Parent owns integrated checks and representative shared-house/room/plant previews; no full curriculum rerender was performed. Existing geometry tests must validate closure per subpath for a compound fill. Concurrent unrelated palette edits in the shared module were preserved.
