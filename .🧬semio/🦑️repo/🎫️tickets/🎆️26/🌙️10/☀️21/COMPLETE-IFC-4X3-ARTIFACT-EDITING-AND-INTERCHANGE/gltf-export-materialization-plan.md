# Cooperative glTF Export Materialization Plan

The independent frontier audit identified two exact synchronous export bursts after bounded geometry extraction: writing every index word without a checkpoint, and merging Uint8Array chunks then Array.from over the entire final binary buffer. Vertex conversion also used a fixed 2048 cadence rather than the caller's admitted batch and had no phase completion checkpoint.

Extend the neutral materialization matrix with original pinned IFC export phases for vertices, indices and final buffer bytes, each at interior and completed boundaries. Add tests before production. Use captured original batch and completion checks for vertices and index words. Construct the artifact's required number[] buffer directly through bounded chunk-byte writes, avoiding a second full contiguous Uint8Array merge and final Array.from. Preserve float32 conversion/min/max, little-endian index layout, buffer/accessor offsets and the exact supported output representation. Output remains local until the final original-context checkpoint succeeds.

JSON native translation/stringify/UTF8 encode and raw JSON decode/parse remain a distinct codec boundary, recorded in gltf-export-json-frontier-audit.md. This work does not label those synchronous primitives cooperative. It adds no native runtime proof or new geometry mapping claim.
