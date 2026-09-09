# Canonical Artifact Field Parity

The permanent audit checks the 104 artifact/diff roster plus 52 snapshot facets across Rust, TypeScript, GraphQL, and Protobuf against each facet’s canonical JSON field names. Explicit native `value(rename)` attributes are honored, and state annotations survive intervening attributes. A neutral oracle first reproduced incorrect renamed fields, then passed against independent TypeScript AST extraction and Ajv exact-record validation.

The first live report found **84 mismatching representations**. This is a diagnostic count, not a claim that the JSON side is correct: canonical document runtime fixtures determine which side needs correction. Empty or stale projections, incorrectly named fields, and missing exact declarations must be resolved in their owners. A second neutral regression reproduced missing TypeScript quoted numeric keys; the extractor correction passed against the independent AST oracle. The second live report contained 80 mismatching representations after that correction and concurrent family edits. The table below retains the first report as a timestamped diagnostic; refresh the permanent report for current findings.

Report 4 passed its diagnostic run in 15.3 seconds and counted 69 mismatching representations after Forms contract correction. Playbook's subsequent twelve-facet correction passed independent fixture/schema/parser checks; report 5 is the next live refresh. These counts cover field names, not nested type correctness. The newly shared framework Schema document-contract testkit additionally validates actual committed native inputs against Ajv and production parsers, currently adopted by Forms and Playbook.

Run `workspace:artifact-field-parity-test`, `workspace:artifact-field-parity-report`, or `workspace:artifact-field-parity-enforce` through the matching launch configurations. The enforcement command is expected to fail until the recorded mismatches are corrected. This focused audit does not change the old field-exclusion gate’s native-law prerequisite.

| Representation | Missing From Representation | Extra In Representation |
| --- | --- | --- |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | text | document |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | text | document |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto |  | cameraX,cameraY,cameraZoom,editorSelection,editorSettings,engagementInput,formatSignal,lintSignal,revision |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | text | document |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | geometry,graph | computed,equation,notation,results |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | artifact,geometry,graph | computed,equation,notation,results |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | geometry,graph | computed,equation,notation,results |
| ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | layout,synapses,widgets | content |
| ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | layout,synapses,widgets | content |
| ✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | layout,synapses,widgets | content |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs |  | mesh |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs |  | mesh |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs |  | drawing,image,value |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs |  | image |
| ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | source,tiles | animation,presentation |
| ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | source,tiles | presentation |
| ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | source,tiles | animation,presentation |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs |  | emblem |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs |  | emblem |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs |  | emblem |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | edges,steps | content |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | edges,steps | content |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | edges,steps | content |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | documents | artifacts |
| ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | documents | artifacts |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql | value | id,results,structure,title,version |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | value | id,results,structure,title,version |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts | value | id,results,structure,title,version |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql | value | id,results,structure,title,version |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | value | id,results,structure,title,version |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts | value | id,results,structure,title,version |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | steps | results,structure |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql | value | backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,roles,spreads,stories |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | value | backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts | value | backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql | value | artifact,backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,roles,spreads,stories |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | value | artifact,backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts | value | artifact,backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs |  | backgroundDrawing,referencedModel |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | buildingGeometry,buildingObjects,energyGeometry,energyObjects,objects,shapeGeometry,structureClassicGeometry,structureClassicObjects | buildingModel,drawings,energyModel,shapeModel,structureClassicModel |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | buildingGeometry,buildingObjects,energyGeometry,energyObjects,objects,shapeGeometry,structureClassicGeometry,structureClassicObjects | buildingModel,drawings,energyModel,shapeModel,structureClassicModel |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | buildingGeometry,buildingObjects,energyGeometry,energyObjects,objects,shapeGeometry,structureClassicGeometry,structureClassicObjects | buildingModel,drawings,energyModel,shapeModel,structureClassicModel |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | artifact |  |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts | artifact |  |
| ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | artifact |  |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql | value | document,flow,id,title,version |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | value | document,flow,id,title,version |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts | value | document,flow,id,title,version |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql | value | artifact,document,flow,id,title,version |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | value | artifact,document,flow,id,title,version |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts | value | artifact,document,flow,id,title,version |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | steps | document,flow |
| ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | path,seed | flow,text |
| ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | path,seed | flow,text |
| ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | path,seed | flow,text |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql | artifact |  |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | artifact |  |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | edges,nodes | content |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql | artifact |  |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | artifact,edges,nodes | content |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | edges,nodes | content |
| ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | edges,nodes | content |
| ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | artifact,edges,nodes,setEdges,setNodes | content |
| ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs | edges,nodes | content |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql | value | artboard,assets,id,layers,title |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | value | artboard,assets,id,layers,title |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts | value | artboard,assets,id,layers,title |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql | value | artboard,artifact,assets,id,layers,title |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | value | artboard,artifact,assets,id,layers,title |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts | value | artboard,artifact,assets,id,layers,title |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql | value | assets,id,layers,title |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | value | assets,id,layers,title |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts | value | assets,id,layers,title |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql | value | artifact,assets,id,layers,title |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs | value | artifact,assets,id,layers,title |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts | value | artifact,assets,id,layers,title |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs |  | linkedArtifact |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs |  | linkedArtifact |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto |  | cameraX,cameraY,cameraZoom,engagementInput,selectedBlockIds |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs |  | linkedArtifact |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs |  | kindCatalogsExtra |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs |  | kindCatalogsExtra |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs | part2d,part3d | 2d,3d |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts | part2d,part3d |  |

Latest refresh: `artifact-field-parity-report-5.log` completed successfully in 13.3 seconds and reports 62 mismatching schema representations after the Playbook correction. Report success is inventory generation, not enforcement success.

Equation projections are now corrected by the execution agent. Its `artifact-field-parity-report-equation-6.log` refresh reports 59 remaining mismatching representations and zero Equation findings. The dedicated Equation native lifecycle rerun is separate and still pending.

The Drawing/Raster execution-agent refresh (`artifact-field-parity-drawing-raster-1.log`) reports 41 remaining representation mismatches and no Drawing/Raster rows. This includes concurrent Equation/Shooting/Sequence corrections, so the total reduction is not attributed solely to Drawing/Raster. Focused Drawing/Raster TypeScript checks passed with skipLibCheck; full ambient type checking remains affected by unrelated MDX JSX declarations.
