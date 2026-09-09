# Current Artifact Field Parity

The previous full report completed through Bun/Nx (`artifact-field-parity-note-11.log`, exit 0, 36.6 seconds). Its independent TypeScript AST/Ajv/fast-glob oracle passed and inspected 192 standard/subset owners. Reporting succeeds while findings remain; this is not a passing enforcement result.

That report found 25 mismatching representations, down from the last completed 36-finding report. Note, Jack and DAG have no field-set mismatch in this completed report. Remaining families are Flow, GIS Map/Terrain, Presentation, Architect Program, Layout, CAD and Procedure.

## Exact Findings

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing=layout,synapses,widgets extra=content`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs: missing=layout,synapses,widgets extra=content`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing=layout,synapses,widgets extra=content`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing= extra=mesh`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing= extra=mesh`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing= extra=drawing,image,value`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing= extra=image`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing=source,tiles extra=animation,presentation`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs: missing=source,tiles extra=presentation`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing=source,tiles extra=animation,presentation`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing=documents extra=artifacts`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing=documents extra=artifacts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql: missing=value extra=backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,roles,spreads,stories`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing=value extra=backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts: missing=value extra=backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql: missing=value extra=artifact,backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,roles,spreads,stories`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs: missing=value extra=artifact,backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts: missing=value extra=artifact,backgroundDrawing,characterStyles,dataFieldsJson,grid,links,name,pages,paragraphStyles,parentPages,printTarget,referencedModel,spreads,stories`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing= extra=backgroundDrawing,referencedModel`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing=buildingGeometry,buildingObjects,energyGeometry,energyObjects,objects,shapeGeometry,structureClassicGeometry,structureClassicObjects extra=buildingModel,drawings,energyModel,shapeModel,structureClassicModel`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs: missing=buildingGeometry,buildingObjects,energyGeometry,energyObjects,objects,shapeGeometry,structureClassicGeometry,structureClassicObjects extra=buildingModel,drawings,energyModel,shapeModel,structureClassicModel`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing=buildingGeometry,buildingObjects,energyGeometry,energyObjects,objects,shapeGeometry,structureClassicGeometry,structureClassicObjects extra=buildingModel,drawings,energyModel,shapeModel,structureClassicModel`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs: missing=path,seed extra=flow,text`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs: missing=path,seed extra=flow,text`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs: missing=path,seed extra=flow,text`

## Next Ownership Decisions

Flow's native snapshot explicitly keeps camera in the artifact lane while its own documentation calls it editor viewport state. Merely aligning non-Rust facets to that native field would preserve an ownership violation. It needs a real WindowConfig migration together with its composed-content contract repair.

GIS Map's snapshot has drawing/value children beyond its artifact DTO; those children are derived from the same positions/routes/regions fields. GIS Terrain describes its mesh child as always re-derived from other persisted fields. Before adding these missing fields mechanically to the other representations, audit whether these are canonical composed data, derived projections, or an inconsistent artifact/snapshot split. The optional map image is independently authored data and must survive conversion.

## OS Declaration Gate

The separate OS ownership enforcement target completed (`os-owner-enforce-current-1.log`, exit 0, 5.1 seconds): 11 ownership vectors, four nested-schema vectors, three command-source vectors, and 104 artifact contract facets were checked, with zero misplaced declarations under that policy. This confirms the policy's OS preference/host-control checks. It does not resolve the 25 field mismatches above or prove that every native `[state(artifact)]` annotation is semantically correct; Flow camera is the concrete counterexample still requiring migration.

## Procedure Checkpoint

The newer `artifact-field-parity-procedure-1.log` completed with exit 0 in report mode (34.6 seconds): 192 owners inspected and 22 remaining mismatching representations. All three Procedure discrepancies are resolved; the earlier 25-finding list above is historical for those Procedure entries. Flow, GIS Map/Terrain, Presentation, Architect Program, Layout and CAD remain. This report is not a zero-findings enforcement result.
