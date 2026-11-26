# Rust source boundaries for the release roots

Checked 1824 distinct exact target/feature closures from 11 selected Hub release roots. The scanner uses the pinned compiler's platform cfg and the production Rust source resolver and limits. Rows show over-limit, unresolved and within 2% of an applicable limit. Admission still rechecks the actual per-build compiler configuration and source epoch.

| Unit | Platform | Features | Active lines | Files | Bytes | Limit or unresolved edge | Release roots |
| --- | --- | --- | ---: | ---: | ---: | --- | --- |
| semio-framework-3d | wasm32-wasip2 | — | 3234/4000 | 15/32 | 257468/262144 | — | cad, demonstrator, procedural, process, puzzle |
| semio-framework-3d-brep-boolean | wasm32-wasip2 | — | 4000/4000 | 18/32 | 208337/262144 | — | cad, demonstrator, procedural, process |
| semio-framework-3d-brep-construction | wasm32-wasip2 | — | 3928/4000 | 19/32 | 197153/262144 | — | cad, demonstrator, procedural, process |
| semio-framework-3d-brep-engine | wasm32-wasip2 | — | 3981/4000 | 23/32 | 237397/262144 | — | cad, demonstrator, procedural, process |
| semio-framework-3d-brep-native-registry | wasm32-wasip2 | — | 4000/4000 | 25/32 | 221929/262144 | — | cad, demonstrator, procedural, process |
| semio-framework-3d-brep-representation | wasm32-wasip2 | — | 3999/4000 | 27/32 | 193324/262144 | — | cad, demonstrator, energy, flow, playbook, procedural, process, puzzle, sourcing |
| semio-framework-3d-brep-solid-transaction | wasm32-wasip2 | — | 3948/4000 | 29/32 | 206723/262144 | — | cad, demonstrator, procedural, process |
| semio-framework-dsl-record-emission | host | — | 1723/4000 | 8/32 | 261923/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-framework-dsl-record-emission | wasm32-wasip2 | — | 1723/4000 | 8/32 | 261923/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-framework-dsl-record-model | host | — | 2352/4000 | 26/32 | 261507/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-framework-dsl-record-model | wasm32-wasip2 | — | 2352/4000 | 26/32 | 261507/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-framework-job-context | host | — | 1830/4000 | 17/32 | 260791/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-framework-job-context | wasm32-wasip2 | — | 1830/4000 | 17/32 | 260791/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-framework-pack-json | host | — | 3232/4000 | 17/32 | 257972/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-framework-pack-json | wasm32-wasip2 | — | 3232/4000 | 17/32 | 257972/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-os-flow-host-session | wasm32-wasip2 | — | 2928/4000 | 21/32 | 260774/262144 | — | flow, playbook, procedural, puzzle |
| semio-os-vcs | wasm32-wasip2 | — | 3261/4000 | 27/32 | 258833/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-plugin-instance-jobs | wasm32-wasip2 | — | 2490/4000 | 19/32 | 257049/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing |
| semio-s-artifact-energy-model-snapshot | wasm32-wasip2 | — | 2872/4000 | 29/32 | 259535/262144 | — | energy |
| semio-s-artifact-fem-2d-model | wasm32-wasip2 | — | 3704/4000 | 28/32 | 257262/262144 | — | fem |
| semio-s-artifact-procedural-3d-model | wasm32-wasip2 | — | 2330/4000 | 30/32 | 257246/262144 | — | procedural, puzzle |
| semio-s-artifact-puzzle-5d-puzzle5d | wasm32-wasip2 | — | 3975/4000 | 13/32 | 257116/262144 | — | puzzle |
| semio-s-artifact-trinity-jack-language-service | wasm32-wasip2 | — | 2857/4000 | 32/32 | 134777/262144 | — | puzzle |
| semio-s-spatial-kernel-semio-session | wasm32-wasip2 | — | 3494/4000 | 13/32 | 257156/262144 | — | cad, demonstrator, procedural, process |

The same compiled-entry census before the two repairs found BREP Engine at 4,006 active lines and Solid Transaction at 4,074 active lines and 33 files. Both exceeded the original limits. FEM/GIS has no dependency on either repaired closure.
