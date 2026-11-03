# 🔍️ Editability audit: EN 1990–1999 artifacts (wave 2)

Read-only audit. Ticket: `NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`. Generated 2026-11-02 from source as found at audit time; concurrent wave-2 edits may shift counts.

## Method

- **Leaves**: schema-level scalar leaves of `🧬️schema/📸️snapshot/🦀️.rs`, recursed through `Vec<Row>` and nested structs; paths use serde camelCase and `[]` for every list selector (`members[].span`, `joints[].actions[].x`). Each field is counted once (rows are not multiplied by instance count). Enums and `Vec<scalar>` count as one leaf.
- **Properties (edit rules)**: a leaf is editable when its template equals a `SetFieldRule.path` in `🧬️mutations/🧭️edit-rules/🦀️.rs` (dispatched through `NormEditRules::resolve`, which matches the `[]`-normalised address exactly), or when the artifact's own `resolve_edit`/`resolve` has a generic fallback that covers it (see per-artifact notes).
- **3D handles**: a leaf is editable via handles when a handle's `binding(...)`/`FieldBinding` write path, after `[id=…]`/`[index]` normalisation to `[]`, equals the template. Paths were resolved from literal `format!` strings and helper closures (`let path = \|leaf\| format!(…)`, `member_path`, `anchor_grip`/`length_grip` field arguments). Handle writes are committed as `setFields` through the same resolver, so a handle-only leaf is also subject to the properties rule.
- **Not counted**: check-report bindings (`binding(report, …)` / `input.report`), which are read-only utilisation annotations, not writes; `view.activeCombination` (view state, not a document leaf); diagram markers.

## Summary

| Artifact | Snapshot root | Total leaves | Direct SetFieldRule | Fallback only | Editable via properties | Editable via 3D handles | Handle-only (not props) | Not editable at all | Generic replace_row-style fallback |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|
| ⚖️en1990 | `En1990Snapshot` | 55 | 11 | 0 | 11 | 6 | 6 | 38 | no |
| 🏋️en1991 | `En1991Snapshot` | 102 | 77 | 0 | 77 | 7 | 0 | 25 | no |
| 🏛️en1992 | `En1992Snapshot` | 101 | 23 | 0 | 23 | 14 | 4 | 74 | no |
| 🔩️en1993 | `En1993Snapshot` | 354 | 1 | 353 | 354 | 18 | 0 | 0 | yes |
| 🧩️en1994 | `En1994Snapshot` | 88 | 19 | 69 | 88 | 16 | 0 | 0 | yes |
| 🪵️en1995 | `En1995Snapshot` | 62 | 57 | 0 | 57 | 18 | 0 | 5 | no |
| 🪨️en1996 | `En1996Snapshot` | 54 | 50 | 0 | 50 | 12 | 0 | 4 | no |
| 🌍️en1997 | `En1997Snapshot` | 129 | 14 | 0 | 14 | 16 | 9 | 106 | no |
| 🫨️en1998 | `En1998Snapshot` | 113 | 93 | 0 | 93 | 6 | 0 | 20 | no |
| 🪶️en1999 | `En1999Snapshot` | 130 | 8 | 122 | 130 | 30 | 0 | 0 | yes |

Fallback detail:

- 🔩️en1993: generic `update(collection, row)` fallback: any `<table>[id=…]…` leaf rewrites the whole row (16 tables covered, `resolve` returns before EDIT_RULES except for `annex`)
- 🧩️en1994: generic `replace_row` fallback: remove+insert of the whole row, only for `beams`, `columns`, `slabs` (`resolve_edit` tries EDIT_RULES first, then falls back on error)
- 🪶️en1999: generic `update(collection, list)` fallback: any `<table>[id=…]…` leaf replaces the whole collection (8 tables covered, tried after EDIT_RULES fails)
- All other artifacts (en1990, en1991, en1992, en1995, en1996, en1997, en1998): no generic fallback. Leaves outside their SetFieldRule table fail with `no kind sets '…'`. en1992 and en1997 only use `ID_REMOVALS` for row removal.

## Per artifact

### ⚖️en1990

- Total leaves: 55; editable via properties: 11; editable via 3D handles: 6; not editable: 38.
- Handle leaves: `members[].span`, `members[].precamber`, `permanents[].gk`, `variables[].qk`, `accidentals[].ad`, `seismics[].aEk`.
- Non-editable leaves:
  - `structureKind`
  - `kFiDeclared`
  - `permanents[].id`
  - `permanents[].kind`
  - `variables[].id`
  - `variables[].category`
  - `accidentals[].id`
  - `seismics[].id`
  - `seismics[].importanceClass`
  - `members[].id`
  - `members[].labelEn`
  - `members[].labelDe`
  - `members[].rdStr`
  - `members[].rdGeo`
  - `members[].rdEquStab`
  - `members[].rdEquDestab`
  - `members[].rdFat`
  - `members[].deflectionW1`
  - `members[].deflectionW2`
  - `members[].deflectionW3`
  - `members[].deflectionLimitRatio`
  - `members[].deflectionLimitRatioW3`
  - `members[].slsLimitChar`
  - `members[].slsLimitFreq`
  - `members[].slsLimitQp`
  - `members[].vibrationFrequency`
  - `members[].vibrationFrequencyMin`
  - `bridgeSls[].id`
  - `bridgeSls[].memberId`
  - `bridgeSls[].deckAcceleration`
  - `bridgeSls[].deckAccelerationLimit`
  - `bridgeSls[].deckTwist`
  - `bridgeSls[].deckTwistLimit`
  - `bridgeSls[].bridgeDeflection`
  - `bridgeSls[].bridgeDeflectionLimit`
  - `effects[].memberId`
  - `effects[].actionId`
  - `effects[].influence`

### 🏋️en1991

- Total leaves: 102; editable via properties: 77; editable via 3D handles: 7; not editable: 25.
- Handle leaves: `floors[].area`, `roofs[].driftObstructionHeight`, `roofs[].parapetHeight`, `roofs[].pitchDeg`, `selfWeightElements[].thickness`, `windFaces[].loadedArea`, `windFaces[].z`.
- Non-editable leaves:
  - `floors[].id`
  - `floors[].category`
  - `floors[].assumedQkConcentrated`
  - `floors[].assumedPartitions`
  - `selfWeightElements[].id`
  - `selfWeightElements[].material`
  - `roofs[].id`
  - `roofs[].roofType`
  - `roofs[].cE`
  - `roofs[].cT`
  - `roofs[].hasParapet`
  - `roofs[].multiSpan`
  - `windFaces[].id`
  - `windFaces[].zone`
  - `windFaces[].cPe10`
  - `windFaces[].cPe1`
  - `windFaces[].cPi`
  - `windFaces[].cS`
  - `windFaces[].cD`
  - `accidentalCases[].id`
  - `accidentalCases[].impact[].vehicleMass`
  - `accidentalCases[].impact[].vehicleSpeed`
  - `accidentalCases[].explosion[].explosionMass`
  - `accidentalCases[].explosion[].standoff`
  - `accidentalCases[].explosion[].assumedPressure`

### 🏛️en1992

- Total leaves: 101; editable via properties: 23; editable via 3D handles: 14; not editable: 74.
- Handle leaves: `members[].span`, `members[].width`, `members[].height`, `members[].effectiveDepth`, `members[].cover`, `members[].stirrups.spacing`, `members[].fire.axisDistance`, `members[].punching.columnWidth`, `members[].punching.columnDepth`, `members[].longitudinal[].count`, `members[].longitudinal[].diameter`, `anchors[].hEf`, `anchors[].d`, `anchors[].c1`.
- Non-editable leaves:
  - `concreteGrades[].id`
  - `concreteGrades[].name`
  - `reinforcementGrades[].id`
  - `reinforcementGrades[].name`
  - `reinforcementGrades[].eS`
  - `reinforcementGrades[].ductility`
  - `reinforcementGrades[].k`
  - `reinforcementGrades[].epsUk`
  - `prestressSteels[].id`
  - `prestressSteels[].name`
  - `prestressSteels[].fPk`
  - `prestressSteels[].fP01k`
  - `members[].id`
  - `members[].labelEn`
  - `members[].labelDe`
  - `members[].kind`
  - `members[].concreteGradeId`
  - `members[].reinforcementGradeId`
  - `members[].prestressSteelId`
  - `members[].support`
  - `members[].bucklingLength`
  - `members[].longitudinal[].id`
  - `members[].longitudinal[].position`
  - `members[].longitudinal[].anchorageLength`
  - `members[].longitudinal[].lapLength`
  - `members[].longitudinal[].bondCondition`
  - `members[].longitudinal[].aggregateSize`
  - `members[].stirrups.diameter`
  - `members[].stirrups.legs`
  - `members[].punching.columnPosition`
  - `members[].punching.asw`
  - `members[].prestress.force`
  - `members[].prestress.area`
  - `members[].prestress.eccentricity`
  - `members[].prestress.lossRatio`
  - `members[].fire.columnMethod`
  - `members[].fire.slabSystem`
  - `members[].actions[].id`
  - `members[].actions[].kind`
  - `members[].actions[].category`
  - `members[].actions[].source`
  - `members[].actions[].gKLine`
  - `members[].actions[].qKLine`
  - `members[].actions[].pointForce`
  - `members[].actions[].tK`
  - `members[].actions[].vKPunch`
  - `members[].useFem`
  - `members[].udl`
  - `members[].deflectionSensitive`
  - `members[].tightness`
  - `members[].hdOverH`
  - `members[].liquidSigmaS`
  - `members[].liquidRhoPEff`
  - `members[].liquidFCtEff`
  - `members[].liquidSRMax`
  - `members[].bridgeSigmaC`
  - `members[].bridgeDeltaSigmaS`
  - `anchors[].id`
  - `anchors[].cracked`
  - `anchors[].fUk`
  - `anchors[].fYk`
  - `anchors[].fCk`
  - `anchors[].actions[].id`
  - `anchors[].actions[].kind`
  - `anchors[].actions[].category`
  - `anchors[].actions[].source`
  - `anchors[].actions[].gKLine`
  - `anchors[].actions[].qKLine`
  - `anchors[].actions[].pointForce`
  - `anchors[].actions[].mK`
  - `anchors[].actions[].nK`
  - `anchors[].actions[].vK`
  - `anchors[].actions[].tK`
  - `anchors[].actions[].vKPunch`

### 🔩️en1993

- Total leaves: 354; editable via properties: 354; editable via 3D handles: 18; not editable: 0.
- Handle leaves: `joints[].boltRows`, `joints[].boltsPerRow`, `joints[].edgeDistance`, `joints[].endDistance`, `joints[].gauge`, `joints[].pitch`, `joints[].weldLength`, `joints[].weldThroat`, `members[].length`, `members[].ltbRestraintSpacing`, `members[].sectionId`, `piles[].embeddedLength`, `platedPanels[].a`, `platedPanels[].b`, `platedPanels[].thickness`, `siloShells[].depth`, `siloShells[].radius`, `siloShells[].thickness`.
- Non-editable leaves:
  - none

### 🧩️en1994

- Total leaves: 88; editable via properties: 88; editable via 3D handles: 16; not editable: 0.
- Handle leaves: `beams[].sheeting.heightM`, `beams[].slabThicknessM`, `beams[].spanM`, `beams[].spacingM`, `beams[].steel.designation`, `beams[].studs.countPerRib`, `beams[].studs.spacingM`, `beams[].studs.totalCount`, `beams[].sheeting.ribsParallelToBeam`, `columns[].lengthM`, `columns[].outerSizeM`, `columns[].wallThicknessM`, `slabs[].concreteThicknessM`, `slabs[].sheeting.heightM`, `slabs[].spanM`, `slabs[].sheeting.ribsParallelToBeam`.
- Non-editable leaves:
  - none

### 🪵️en1995

- Total leaves: 62; editable via properties: 57; editable via 3D handles: 18; not editable: 5.
- Handle leaves: `members[].spanM`, `members[].bM`, `members[].hM`, `members[].bearingLengthM`, `members[].lateralRestraintSpacingM`, `members[].notchDepthM`, `members[].notchDistanceM`, `members[].fireDurationS`, `connections[].spacingM`, `connections[].endDistanceM`, `connections[].edgeDistanceM`, `connections[].t1M`, `connections[].t2M`, `connections[].steelPlateThicknessM`, `connections[].diameterM`, `connections[].number`, `connections[].rows`, `connections[].shearPlanes`.
- Non-editable leaves:
  - `members[].id`
  - `members[].mCritNm`
  - `members[].actions[].id`
  - `connections[].id`
  - `connections[].actions[].id`

### 🪨️en1996

- Total leaves: 54; editable via properties: 50; editable via 3D handles: 12; not editable: 4.
- Handle leaves: `walls[].heightM`, `walls[].lengthM`, `walls[].thicknessM`, `walls[].slabBearingDepthM`, `walls[].supportSides`, `walls[].eccentricityTopM`, `walls[].eccentricityBottomM`, `walls[].openings[].widthM`, `walls[].openings[].heightM`, `walls[].openings[].sillHeightM`, `walls[].loadCases[].concentrated[].bearingLengthM`, `walls[].loadCases[].slabSpanM`.
- Non-editable leaves:
  - `walls[].id`
  - `walls[].openings[].id`
  - `walls[].loadCases[].id`
  - `walls[].loadCases[].concentrated[].id`

### 🌍️en1997

- Total leaves: 129; editable via properties: 14; editable via 3D handles: 16; not editable: 106.
- Handle leaves: `footings[].embedment`, `footings[].length`, `footings[].width`, `footings[].loadCases[].horizontalVariable`, `layers[].depthBottom`, `layers[].depthTop`, `piles[].length`, `piles[].diameter`, `piles[].count`, `retainingWalls[].height`, `retainingWalls[].embedment`, `retainingWalls[].baseWidth`, `retainingWalls[].stemThickness`, `slopes[].angleDeg`, `slopes[].height`, `groundwaterLevel`.
- Handle templates in the HANDLES table not matching a snapshot leaf (check): `piles[].embedment`.
- Non-editable leaves:
  - `structureId`
  - `layers[].id`
  - `layers[].soilType`
  - `layers[].gamma`
  - `layers[].gammaPrime`
  - `layers[].cohesionEffective`
  - `layers[].cohesionUndrained`
  - `layers[].poissonRatio`
  - `layers[].cptQc`
  - `layers[].sptN`
  - `footings[].id`
  - `footings[].baseInclinationDeg`
  - `footings[].settlementLimit`
  - `footings[].loadCases[].id`
  - `footings[].loadCases[].designSituation`
  - `footings[].loadCases[].verticalPermanent`
  - `footings[].loadCases[].verticalVariable`
  - `footings[].loadCases[].horizontalPermanent`
  - `footings[].loadCases[].momentPermanent`
  - `footings[].loadCases[].momentVariable`
  - `piles[].id`
  - `piles[].pileType`
  - `piles[].alphaS`
  - `piles[].unitShaftResistance`
  - `piles[].unitBaseResistance`
  - `piles[].compressionPermanent`
  - `piles[].compressionVariable`
  - `piles[].tensionPermanent`
  - `piles[].tensionVariable`
  - `piles[].testProfiles[].id`
  - `piles[].testProfiles[].shaftResistance`
  - `piles[].testProfiles[].baseResistance`
  - `retainingWalls[].id`
  - `retainingWalls[].backfillPhiDeg`
  - `retainingWalls[].backfillGamma`
  - `retainingWalls[].wallFrictionDeg`
  - `retainingWalls[].earthPressureMode`
  - `retainingWalls[].wallMovement`
  - `retainingWalls[].ocr`
  - `retainingWalls[].concreteGamma`
  - `retainingWalls[].surcharge`
  - `retainingWalls[].verticalPermanent`
  - `retainingWalls[].horizontalPermanent`
  - `slopes[].id`
  - `slopes[].length`
  - `slopes[].governingLayerId`
  - `upliftCases[].id`
  - `upliftCases[].permanentStabilizing`
  - `upliftCases[].permanentDestabilizing`
  - `upliftCases[].variableDestabilizing`
  - `upliftCases[].porePressure`
  - `upliftCases[].totalStress`
  - `anchors[].id`
  - `anchors[].wallId`
  - `anchors[].tendon`
  - `anchors[].headDepth`
  - `anchors[].inclinationDeg`
  - `anchors[].freeLength`
  - `anchors[].fixedLength`
  - `anchors[].fixedDiameter`
  - `anchors[].bondStress`
  - `anchors[].designForce`
  - `anchors[].tendonYieldForce`
  - `anchors[].minFreeLength`
  - `anchors[].testFactor`
  - `anchors[].tests[].id`
  - `anchors[].tests[].pullOutResistance`
  - `embeddedWalls[].id`
  - `embeddedWalls[].kind`
  - `embeddedWalls[].retainedHeight`
  - `embeddedWalls[].embedment`
  - `embeddedWalls[].propDepth`
  - `embeddedWalls[].thickness`
  - `embeddedWalls[].backfillGamma`
  - `embeddedWalls[].phiDeg`
  - `embeddedWalls[].deltaActiveDeg`
  - `embeddedWalls[].deltaPassiveDeg`
  - `embeddedWalls[].surcharge`
  - `embeddedWalls[].undrainedStrength`
  - `pileGroups[].id`
  - `pileGroups[].pileId`
  - `pileGroups[].rows`
  - `pileGroups[].columns`
  - `pileGroups[].spacing`
  - `pileGroups[].capVertical`
  - `pileGroups[].capMomentX`
  - `pileGroups[].capMomentY`
  - `pileGroups[].singleResistance`
  - `pileGroups[].shaftCohesionSum`
  - `pileGroups[].baseCohesion`
  - `labTests[].id`
  - `labTests[].layerId`
  - `labTests[].kind`
  - `labTests[].voidRatio`
  - `labTests[].preloadStress`
  - `labTests[].drainageHeight`
  - `labTests[].consolidationTime`
  - `labTests[].points[].id`
  - `labTests[].points[].normalStress`
  - `labTests[].points[].shearStress`
  - `layerWaters[].id`
  - `layerWaters[].layerId`
  - `layerWaters[].piezometricDepth`
  - `layerWaters[].permeability`
  - `layerWaters[].headDifference`
  - `layerWaters[].flowLength`

### 🫨️en1998

- Total leaves: 113; editable via properties: 93; editable via 3D handles: 6; not editable: 20.
- Handle leaves: `buildings[].planWidthM`, `buildings[].planLengthM`, `buildings[].storeys[].centreOfMassXM`, `buildings[].storeys[].centreOfMassYM`, `buildings[].storeys[].centreOfStiffnessXM`, `buildings[].storeys[].centreOfStiffnessYM`.
- Non-editable leaves:
  - `buildings[].id`
  - `buildings[].systems[].id`
  - `buildings[].storeys[].id`
  - `buildings[].storeys[].variables[].id`
  - `buildings[].storeys[].variables[].category`
  - `buildings[].storeys[].variables[].qkN`
  - `buildings[].members[].id`
  - `bridges[].id`
  - `bridges[].variables[].id`
  - `bridges[].variables[].category`
  - `bridges[].variables[].qkN`
  - `assessments[].id`
  - `silos[].id`
  - `tanks[].id`
  - `foundations[].id`
  - `retainingWalls[].id`
  - `towers[].id`
  - `towers[].variables[].id`
  - `towers[].variables[].category`
  - `towers[].variables[].qkN`

### 🪶️en1999

- Total leaves: 130; editable via properties: 130; editable via 3D handles: 30; not editable: 0.
- Handle leaves: `members[].length`, `members[].ltbLength`, `members[].bucklingLengthY`, `members[].bucklingLengthZ`, `sections[].height`, `sections[].width`, `sections[].flangeThickness`, `sections[].webThickness`, `sections[].outerDiameter`, `sections[].elements[].thickness`, `sections[].elements[].width`, `sections[].elements[].weldPosition`, `coldFormed[].span`, `coldFormed[].width`, `coldFormed[].thickness`, `shells[].radius`, `shells[].length`, `shells[].thickness`, `connections[].bolts.boltsPerRow`, `connections[].bolts.diameter`, `connections[].bolts.edgeDistance`, `connections[].bolts.gauge`, `connections[].bolts.pitch`, `connections[].bolts.plateThickness`, `connections[].bolts.rows`, `connections[].welds.hazExtent`, `connections[].welds.length`, `connections[].welds.throat`, `fireScenarios[].durationS`, `fireScenarios[].thetaA`.
- Whole-collection setters (SetFieldRule path is a table, not a leaf; they replace the whole list and are not counted as leaf edits): `coldFormed`, `connections`, `fatigueDetails`, `fireScenarios`, `materials`, `members`, `sections`, `shells`.
- Non-editable leaves:
  - none

## Notes

- en1993: `resolve` returns the fallback before consulting EDIT_RULES for every path except `annex`, so the single SetFieldRule is the only rule that runs. Row rules are unreachable and rows are edited only through the `update` fallback.
- en1999: 8 of its 16 SetFieldRules address whole collections (`members`, `sections`, `materials`, `shells`, …) and never match a leaf template. Only 8 leaf rules apply directly. All other table leaves are editable only through the `update` fallback.
- en1994: `designation_pick` (`beams[].steel.designation` with a numeric value) is routed to `replace_row` and snaps to the catalogue section height.
- Handle coverage is conditional on runtime selection. Member and action handles are generated only for selected or affected entities (for example, en1990 magnitude handles exist only for actions affecting a member).
- Unresolved handle sources: en1995 diagram markers (`members[].{leaf}`, `connections[].{leaf}`) and en1998 tower, silo, tank and wall `h`/`r` grips use runtime field names that were not fully traced. Their coverage is not counted here.
- en1991, en1992 and en1997 handle paths come from helper closures and tuples, so those counts depend on the traced field arguments. Re-verify if the handle files change.
