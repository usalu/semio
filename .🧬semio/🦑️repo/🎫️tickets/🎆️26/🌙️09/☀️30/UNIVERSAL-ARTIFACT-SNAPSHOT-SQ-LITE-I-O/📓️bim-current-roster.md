# Current BIM Defining Role Roster

Read-only October 9 census: current ModelSnapshot has 49 fields: schema, project, and **47 persisted maps**, rather than the historical 24. The machine input records native source origins/lines, declaration-order field IDs, 115 native struct/enum definitions, collection aliases, current JSON definitions when locally present, and per-case literal fixture map coverage. No production edits or runtime qualification.

[Current roster](📥️inputs/bim-current-roster.json)

CurtainWall now persists storey, curtain_wall_type, axis, base_offset, top, optional u_grid/v_grid, phase, name (field IDs 0–8). CurtainWallType owns two CurtainGrid variants, two distinct Profile roles (interior_mullion/border_mullion), CurtainPanel, panel_material and mullion_material. The original curtain-wall fixture must contain the required phase and type reference; the removed inline mullion shape cannot stand in for these roles. ClassificationSet is a BTreeMap<String,String>, requiring a distinct owned parent per element and named ordered system-key/value children, including empty sets.

Old 44-table/192-row forecasts are historical and cannot qualify this expanded domain. Derive exact current row/byte costs from all nested roles in the input. Domain-reference strings may remain unresolved; persisted IDs and ordered ownership must still be validated.
