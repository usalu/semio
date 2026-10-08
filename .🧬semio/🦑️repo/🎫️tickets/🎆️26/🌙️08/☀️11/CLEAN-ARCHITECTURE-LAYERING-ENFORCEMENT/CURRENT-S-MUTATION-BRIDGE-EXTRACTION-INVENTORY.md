# Current S Mutation Bridge Inventory

Read-only authored bridge inventory on 2026-10-08. Includes the five newly authored WFC child TypeScript routes before native extraction; the retained audit-12 baseline was 34 actual routes. Counts classify actual owner depth, without assuming 33 parent bridges. No extraction closure is claimed beyond the WFC work in progress.

Current routes: 39; direct plugin parents: 33; nested children: 6.

| Owner | Scope | Specific Native Dependencies | Aggregates | Coordinates | Native Dependency Identities |
| --- | --- | ---: | ---: | ---: | --- |
| `✏️s/🔌️plugins/✒️writer` | plugin-parent | 1 | 3 | 3 | `semio-s-artifact-writer-writer` |
| `✏️s/🔌️plugins/➗️mathematical` | plugin-parent | 1 | 2 | 4 | `semio-s-artifact-mathematical-equation` |
| `✏️s/🔌️plugins/🀄️wfc` | plugin-parent | 5 | 10 | 10 | `semio-s-artifact-wfc-2d`, `semio-s-artifact-wfc-3d`, `semio-s-artifact-wfc-bitmap`, `semio-s-artifact-wfc-grid2d`, `semio-s-artifact-wfc-grid3d` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d` | artifact-child | 0 | 0 | 0 |  |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d` | artifact-child | 0 | 0 | 0 |  |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap` | artifact-child | 0 | 0 | 0 |  |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d` | artifact-child | 0 | 0 | 0 |  |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d` | artifact-child | 0 | 0 | 0 |  |
| `✏️s/🔌️plugins/🌀️procedural` | plugin-parent | 2 | 8 | 8 | `semio-s-artifact-procedural-generation2d`, `semio-s-artifact-procedural-generation3d` |
| `✏️s/🔌️plugins/🌊️flow` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-flow-flow` |
| `✏️s/🔌️plugins/🌍️gis` | plugin-parent | 2 | 5 | 5 | `semio-s-artifact-gis-gismap`, `semio-s-artifact-gis-gisterrain` |
| `✏️s/🔌️plugins/🌿️vcs` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-vcs-vcs` |
| `✏️s/🔌️plugins/🎞️animate` | plugin-parent | 1 | 3 | 3 | `semio-s-artifact-animate-presentation` |
| `✏️s/🔌️plugins/🎥️shooting` | plugin-parent | 1 | 3 | 3 | `semio-s-artifact-shooting-shooting` |
| `✏️s/🔌️plugins/🎪️demonstrator` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-demonstrator-playground` |
| `✏️s/🔌️plugins/🎬️sequence` | plugin-parent | 1 | 1 | 2 | `semio-s-artifact-sequence-sequence` |
| `✏️s/🔌️plugins/🏗️fem` | plugin-parent | 2 | 3 | 3 | `semio-s-artifact-fem-2d`, `semio-s-artifact-fem-3d` |
| `✏️s/🔌️plugins/🏛️architect` | plugin-parent | 1 | 3 | 3 | `semio-s-artifact-architect-program` |
| `✏️s/🔌️plugins/🏭️process` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-process-process3d` |
| `✏️s/🔌️plugins/💠️lowpoly` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-lowpoly-lowpoly` |
| `✏️s/🔌️plugins/💡️reasoning` | plugin-parent | 1 | 3 | 3 | `semio-s-artifact-reasoning-wires` |
| `✏️s/🔌️plugins/📋️forms` | plugin-parent | 1 | 2 | 2 | `semio-s-artifact-forms-forms` |
| `✏️s/🔌️plugins/📏️layout` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-layout-layout` |
| `✏️s/🔌️plugins/📐️cad` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-cad-cad` |
| `✏️s/🔌️plugins/📕️norm` | plugin-parent | 15 | 15 | 15 | `semio-s-artifact-norm-din16798`, `semio-s-artifact-norm-din18599`, `semio-s-artifact-norm-din4108`, `semio-s-artifact-norm-en1990`, `semio-s-artifact-norm-en1991`, `semio-s-artifact-norm-en1992`, `semio-s-artifact-norm-en1993`, `semio-s-artifact-norm-en1994`, `semio-s-artifact-norm-en1995`, `semio-s-artifact-norm-en1996`, `semio-s-artifact-norm-en1997`, `semio-s-artifact-norm-en1998`, `semio-s-artifact-norm-en1999`, `semio-s-artifact-norm-iso16757`, `semio-s-artifact-norm-vdi3805` |
| `✏️s/🔌️plugins/📖️playbook` | plugin-parent | 1 | 2 | 2 | `semio-s-artifact-playbook-playbook` |
| `✏️s/🔌️plugins/📜️imperative` | plugin-parent | 1 | 2 | 2 | `semio-s-artifact-imperative-procedure` |
| `✏️s/🔌️plugins/📸️remodel` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-remodel-remodeling` |
| `✏️s/🔌️plugins/🔋️energy` | plugin-parent | 1 | 4 | 4 | `semio-s-artifact-energy-model` |
| `✏️s/🔌️plugins/🔱️trinity` | plugin-parent | 2 | 6 | 5 | `semio-s-artifact-trinity-jack`, `semio-s-artifact-trinity-rewriting` |
| `✏️s/🔌️plugins/🕸️dag` | plugin-parent | 1 | 3 | 3 | `semio-s-artifact-dag-dag` |
| `✏️s/🔌️plugins/🖍️draw` | plugin-parent | 1 | 1 | 4 | `semio-s-artifact-draw-drawing` |
| `✏️s/🔌️plugins/🖨️raster` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-raster-raster` |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base` | artifact-child | 1 | 0 | 0 | `semio-s-artifact-stdio-semio` |
| `✏️s/🔌️plugins/🗒️note` | plugin-parent | 1 | 2 | 9 | `semio-s-artifact-note-note` |
| `✏️s/🔌️plugins/🧩️puzzle` | plugin-parent | 3 | 3 | 3 | `semio-s-artifact-puzzle-2d`, `semio-s-artifact-puzzle-3d`, `semio-s-artifact-puzzle-5d` |
| `✏️s/🔌️plugins/🧱️block` | plugin-parent | 3 | 4 | 4 | `semio-s-artifact-block-2d`, `semio-s-artifact-block-3d`, `semio-s-artifact-block-5d` |
| `✏️s/🔌️plugins/🪐️space` | plugin-parent | 2 | 2 | 2 | `semio-s-artifact-space-home`, `semio-s-artifact-space-space` |
| `✏️s/🔌️plugins/🪵️sourcing` | plugin-parent | 1 | 1 | 1 | `semio-s-artifact-sourcing-curation` |

All remaining parent owners require separate descriptor/coordinate retention, owner-local declarations, runtime inventories and physical missing-child compilation proof. New WFC child routes have zero standalone manifests; their defining artifact packages will own native binaries and locks.
