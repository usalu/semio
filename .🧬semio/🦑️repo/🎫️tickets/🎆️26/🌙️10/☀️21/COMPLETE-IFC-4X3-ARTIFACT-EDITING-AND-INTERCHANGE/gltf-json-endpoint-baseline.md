# Actual Old GLB JSON Endpoint Baseline

Before root mounted the new writer, gltf-json-endpoint-baseline.ts imported the then-current actual writeIfcGltfGlb endpoint using native()/JSON.stringify and loaded the genuine checked-in Khronos Box GLB through readIfcGltfGlb. It captured 35 deterministic accepted cases. No production source copy was made.

Generated gltf-json-endpoint-baseline.json stores each case id, exact padded JSON chunk text, complete GLB header/chunk fields, binary SHA256 and full GLB SHA256. Cases cover ASCII/control/Unicode/surrogate fragment edges, ten finite numeric zero/extrema/threshold cases, extras empty/null/false/duplicate/integer-key/prototype/namespace cases, attribute duplicate order, morph targets, explicit empty/defaults, optional undefined and record insertion order. Physical binary content is unchanged and represented by its hash.

The script refuses replacing an existing baseline. After the new endpoint is mounted, run the same input with --compare; it emits generated/gltf-json-endpoint-comparison.json and fails on any complete-row discrepancy. Baseline readiness was sent to root before integration. This is actual TypeScript endpoint evidence, not native Rust execution. Independent Three/Khronos subset verification follows the post-mount comparison.

Post-mount comparison completed: all thirty-five exact output rows match; zero mismatches. The independent eleven-case Three subset also preserves full geometry/index signatures. Detailed source/control/physical boundaries are recorded in gltf-json-writer-independent-audit.md.
