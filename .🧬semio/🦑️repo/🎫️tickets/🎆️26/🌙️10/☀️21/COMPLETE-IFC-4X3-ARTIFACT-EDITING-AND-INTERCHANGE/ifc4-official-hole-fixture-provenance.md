# Official IFC4 Polygonal Hole Fixture Provenance

Retrieved 2026-10-21 directly from buildingSMART International's official Standards-builds repository. This supersedes the earlier holes audit's failed direct standards-host retrieval. No source fixture or implementation was changed.

## Unchanged Input

Ticket input: [official-polygonal-face-tessellation.ifc](official-polygonal-face-tessellation.ifc).

- Official repository: https://github.com/buildingSMART/Standards-builds
- Pinned repository commit: a8664407b63d42651e48a78a252c76d09dd6274c
- Original path: IFC/RELEASE/IFC4/ADD2_TC1/HTML/annex/annex-e/polygonal-face-tessellation.ifc
- [Pinned raw bytes](https://raw.githubusercontent.com/buildingSMART/Standards-builds/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/annex/annex-e/polygonal-face-tessellation.ifc)
- [Pinned browser source](https://github.com/buildingSMART/Standards-builds/blob/a8664407b63d42651e48a78a252c76d09dd6274c/IFC/RELEASE/IFC4/ADD2_TC1/HTML/annex/annex-e/polygonal-face-tessellation.ifc)
- [GitHub content metadata](https://api.github.com/repos/buildingSMART/Standards-builds/contents/IFC/RELEASE/IFC4/ADD2_TC1/HTML/annex/annex-e/polygonal-face-tessellation.ifc?ref=a8664407b63d42651e48a78a252c76d09dd6274c)
- HTTP status: 200
- Length: 4,203 bytes
- Git blob SHA-1: 52e03e942125e136b69ceb7f67fd5e9a4631e508
- SHA-256: a76afccde57b9c5e21d797352beb309106037fcaed261ce61fbee413c9460bea
- Saved via Buffer without decoding, line ending normalization, patching, header changes, schema replacement or attribute fixes; read-back byte equality verified.
- Schema declaration: FILE_SCHEMA (('IFC4')).

The repository README identifies this as published web-content release storage and says web content is available at standards.buildingsmart.org. The release path directly corresponds to the intended original official sample URL; it is not a community corrected example.

## Geometry Anchors

Original #21 is IFCINDEXEDPOLYGONALFACEWITHVOIDS with outer indices (6,7,8,5) and inner (14,16,15,13). #27 polygonal face set references #15 3D point list and 11 indexed faces (#16–#26). #28 is Body Tessellation; #30 is the product proxy. The model is a 20,000-unit cube with a 10,000-square recess, recess bottom z=-5,000, top z=10,000. Project uses millimetres; geometry engine currently projects source length units. Expected unscaled solid volume is 6.5e12 source units cubed (8e12 minus 1.5e12); geometric total surface area is 3e9 source units squared. These are analytic expectations inferred from unchanged coordinate data; independent mesh oracle should confirm them. Expected triangulation with four-vertex outer/hole face is 28 triangles total (10 ordinary quad faces * 2 plus holed top * 8). Original coordinate list has 16 vertices. Mesh dedup may omit unrelated metadata points but must preserve these coordinate values.

## Why Earlier Retrieval Was Misleading

The standards host raw/markup URLs now return 403 through direct tools, though earlier web indexed markup was available. HTML rendering can omit literal asterisks in STEP derived values (the fetched indexed markup showed adjacent empty positions). Do not rebuild SPF from web-rendered text. The raw official GitHub byte asset includes the original derived markers and positive exact header.

Current buildingSMART/IFC4.x-specification-models main (commit ef22440ec501551ccc9689309560bbc7e26aa9d7) contains the corresponding model at models/tessellated-shape/polygonal-face-tessellation/polygonal-face-tessellation.ifc, but blob 4d005b5a7e166eac0cfb488102da5d25d307ac3e declares IFC4X3_ADD2 and has redacted headers. It was inspected and rejected, not saved as this IFC4 input. Current Sample-Test-Files main at 80d976a9b193a26a8e928c3e79bff67af1de68a8 has no matching polygonal sample in its complete tree. The Standards-builds overall recursive tree is truncated, so raw exact-path lookup was necessary.

## Open Review Findings Remain

The ring source review's boundary simplification/T-junction and nonzero near-collinear flattening findings remain in ifc4-rings-review.md. Obtaining this official fixture does not resolve those defects. Profile placement validation/numeric observations remain in ifc4-profile-review.md. Main owns implementation and source fixture registration.

License recorded from pinned official README: Final IFC is licensed under Creative Commons Attribution-NoDerivatives 4.0 International (CC BY-ND 4.0); attribution buildingSMART International Ltd. Retain the unchanged original sample and this provenance; do not modify the file and present it as the official fixture. Original producer attribution remains inside FILE_NAME.
