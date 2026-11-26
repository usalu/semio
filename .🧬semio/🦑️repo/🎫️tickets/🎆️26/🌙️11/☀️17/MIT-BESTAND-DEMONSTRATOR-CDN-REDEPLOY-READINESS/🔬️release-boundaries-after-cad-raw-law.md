# Rust source boundaries for the release roots

Checked 2043 distinct exact target/feature closures from 12 selected Hub release roots. The scanner uses the pinned compiler's platform cfg and the production Rust source resolver and limits. Rows show over-limit, unresolved and within 2% of an applicable limit. Admission still rechecks the actual per-build compiler configuration and source epoch.

| Unit | Platform | Features | Active lines | Files | Bytes | Limit or unresolved edge | Release roots |
| --- | --- | --- | ---: | ---: | ---: | --- | --- |
| semio-s-artifact-puzzle-5d-puzzle5d | wasm32-wasip2 | — | 4006/4000 | 13/32 | 259143/262144 | lines | puzzle |
| semio-framework-3d | wasm32-wasip2 | — | 3234/4000 | 15/32 | 257468/262144 | — | cad, demonstrator, procedural, process, puzzle, stdio |
| semio-framework-3d-brep-boolean | wasm32-wasip2 | — | 4000/4000 | 18/32 | 208337/262144 | — | cad, demonstrator, procedural, process, stdio |
| semio-framework-3d-brep-construction | wasm32-wasip2 | — | 3928/4000 | 19/32 | 197153/262144 | — | cad, demonstrator, procedural, process, stdio |
| semio-framework-3d-brep-engine | wasm32-wasip2 | — | 3981/4000 | 23/32 | 237397/262144 | — | cad, demonstrator, procedural, process, stdio |
| semio-framework-3d-brep-native-registry | wasm32-wasip2 | — | 3995/4000 | 25/32 | 222734/262144 | — | cad, demonstrator, procedural, process, stdio |
| semio-framework-3d-brep-representation | wasm32-wasip2 | — | 3999/4000 | 27/32 | 193324/262144 | — | cad, demonstrator, energy, flow, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-3d-brep-solid-transaction | wasm32-wasip2 | — | 3970/4000 | 29/32 | 209216/262144 | — | cad, demonstrator, procedural, process, stdio |
| semio-framework-dsl-record-emission | host | — | 1723/4000 | 8/32 | 261923/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-dsl-record-emission | wasm32-wasip2 | — | 1723/4000 | 8/32 | 261923/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-dsl-record-model | host | — | 2359/4000 | 26/32 | 262007/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-dsl-record-model | wasm32-wasip2 | — | 2359/4000 | 26/32 | 262007/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-job-context | host | — | 1845/4000 | 17/32 | 262138/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-job-context | wasm32-wasip2 | testkit | 1938/4000 | 17/32 | 262138/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-pack-json | host | — | 3242/4000 | 17/32 | 258173/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-framework-pack-json | wasm32-wasip2 | — | 3242/4000 | 17/32 | 258173/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-os-flow-host-session | wasm32-wasip2 | — | 2928/4000 | 21/32 | 260774/262144 | — | flow, playbook, procedural, puzzle |
| semio-os-vcs | wasm32-wasip2 | — | 3261/4000 | 27/32 | 258833/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-plugin-instance-docs | wasm32-wasip2 | — | 2654/4000 | 16/32 | 257356/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-plugin-instance-jobs | wasm32-wasip2 | — | 2489/4000 | 19/32 | 256921/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-plugin-retained-command | wasm32-wasip2 | — | 2462/4000 | 15/32 | 258390/262144 | — | cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing, stdio |
| semio-s-artifact-energy-model-snapshot | wasm32-wasip2 | — | 2872/4000 | 29/32 | 259535/262144 | — | energy |
| semio-s-artifact-fem-2d-model | wasm32-wasip2 | — | 3704/4000 | 28/32 | 257262/262144 | — | fem, stdio |
| semio-s-artifact-procedural-3d-model | wasm32-wasip2 | — | 2330/4000 | 30/32 | 257246/262144 | — | procedural, puzzle |
| semio-s-artifact-stdio-bmp-mutations | wasm32-wasip2 | — | 1364/4000 | 32/32 | 90061/262144 | — | stdio |
| semio-s-artifact-stdio-ifc2x3-catalog-entities-00 | wasm32-wasip2 | — | 113/4000 | 1/32 | 261338/262144 | — | stdio |
| semio-s-artifact-stdio-ifc2x3-catalog-entities-01 | wasm32-wasip2 | — | 97/4000 | 1/32 | 261550/262144 | — | stdio |
| semio-s-artifact-stdio-ifc2x3-catalog-entities-02 | wasm32-wasip2 | — | 102/4000 | 1/32 | 261421/262144 | — | stdio |
| semio-s-artifact-stdio-ifc2x3-catalog-entities-03 | wasm32-wasip2 | — | 116/4000 | 1/32 | 261555/262144 | — | stdio |
| semio-s-artifact-stdio-ifc2x3-catalog-entities-04 | wasm32-wasip2 | — | 97/4000 | 1/32 | 259686/262144 | — | stdio |
| semio-s-artifact-stdio-ifc2x3-catalog-entities-05 | wasm32-wasip2 | — | 91/4000 | 1/32 | 260487/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-00 | wasm32-wasip2 | — | 91/4000 | 1/32 | 259724/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-01 | wasm32-wasip2 | — | 99/4000 | 1/32 | 261667/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-02 | wasm32-wasip2 | — | 92/4000 | 1/32 | 259490/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-03 | wasm32-wasip2 | — | 95/4000 | 1/32 | 258512/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-04 | wasm32-wasip2 | — | 110/4000 | 1/32 | 261285/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-05 | wasm32-wasip2 | — | 87/4000 | 1/32 | 261968/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-06 | wasm32-wasip2 | — | 94/4000 | 1/32 | 260596/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4-catalog-entities-07 | wasm32-wasip2 | — | 94/4000 | 1/32 | 260208/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-00 | wasm32-wasip2 | — | 89/4000 | 1/32 | 259955/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-01 | wasm32-wasip2 | — | 97/4000 | 1/32 | 261838/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-02 | wasm32-wasip2 | — | 91/4000 | 1/32 | 261610/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-03 | wasm32-wasip2 | — | 99/4000 | 1/32 | 261581/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-04 | wasm32-wasip2 | — | 99/4000 | 1/32 | 260280/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-05 | wasm32-wasip2 | — | 108/4000 | 1/32 | 259529/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-06 | wasm32-wasip2 | — | 83/4000 | 1/32 | 260688/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-07 | wasm32-wasip2 | — | 95/4000 | 1/32 | 261649/262144 | — | stdio |
| semio-s-artifact-stdio-ifc4x3-add2-catalog-entities-08 | wasm32-wasip2 | — | 95/4000 | 1/32 | 260250/262144 | — | stdio |
| semio-s-artifact-trinity-jack-language-service | wasm32-wasip2 | — | 2857/4000 | 32/32 | 134777/262144 | — | puzzle |
| semio-s-spatial-kernel-semio-session | wasm32-wasip2 | — | 3494/4000 | 13/32 | 257156/262144 | — | cad, demonstrator, procedural, process, stdio |

The same compiled-entry census before the two repairs found BREP Engine at 4,006 active lines and Solid Transaction at 4,074 active lines and 33 files. Both exceeded the original limits. FEM/GIS has no dependency on either repaired closure.
