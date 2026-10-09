# 🔎️ R11 Audit — w06-ceilings (WP-06) and w07-zones (WP-07)

Status: w06 **PARTIAL**, w07 **NEARLY DONE**. Newest logs show errors only in peer areas.

## WP-06 present
7 leaves (`🎑️🎐️🎏️` ceiling type, `🏞️🌄️🌇️🌆️` ceiling) with fixtures; enum, KINDS, aggregate JSON, binary tags 6000–6006,
grammar, root mounts, checks, mutate feature; snapshot `CeilingType`, `Ceiling`; solids `🧊️element-solids/🔲️ceilings` (7 tests);
spaces clear height = min(slab soffit, hung ceiling); quantities (area, volume, mass); diagnostics (validity, references,
clashes `ClashBeamCeiling`, messages); model graph wired; editor entity, area tool (`i`, `shift+i`), commands, en+de labels;
IFC export `IfcCovering CEILING`; shapely coverage in solids-rest; fixture `🔲️ceilings-holes-slope`.

## WP-06 missing
1. `🚪️io/📤️export/🏗️ifc/🔲️ceilings/🧪️tests/🔬️unit/🦀️.rs` (owned by r11-baseline; extend afterwards).
2. IfcCovering import (remove from `report_unsupported`, `📥️import/🏗️ifc/🏛️spatial/🦀️.rs:165`) + round trip.
3. three.js ceiling-volume oracle case (`🎲️infer-bim-1-solids-three`); missing `T/r10-w06-ceilings-oracle.py` reference.
4. Spaces gating test (ceiling edit changes clear height) + spaces feature scenario.
5. Dedicated ceiling feature + oracle (`🧪️tests/<emoji>infer-bim-1-ceilings`).
6. Model-graph incremental test for ceiling edits.
7. Ceiling tool gesture test; ceilings in the house example.

## WP-07 present
Zone (`🗾️🪄️🧯️`) and area-scheme (`🗃️🗳️🧻️`) leaves; space zone + finish fields in create/set-space; delete-material refuses
when used as a finish; inferences `🏘️zones` (zone/scheme totals, 11 tests) and `🎨️finishes` (19 tests) in model graph;
quantities consume finishes; editor zoning entities, outliner, en+de labels; IFC export (IfcZone, Pset_ZoneCommon,
IfcRelAssignsToGroup, IfcGroup for schemes, Pset_SpaceCoveringRequirements) and import; features + shapely oracles for
zones and finishes.

## WP-07 missing
1. IFC zoning export/import unit tests.
2. Finish ceiling area from authored hung ceilings (`🎨️finishes/🦀️.rs:125`) + oracle + fixture.
3. Model-graph zone/scheme incremental tests.
4. Room finish schedule → assigned to the w13 schedules finisher (preset).
5. Zones, schemes, finishes in the house example.

## Coordinator rulings
- Clear height = min(slab soffit, hung ceiling): accepted (a hung ceiling only lowers the clear height).
- `delete-zone` refuses with `mutation.in-use` while an area scheme counts the zone: accepted (consistent with library
  deletes; scheme membership is the scheme's authored data, edited through `set-area-scheme`).
