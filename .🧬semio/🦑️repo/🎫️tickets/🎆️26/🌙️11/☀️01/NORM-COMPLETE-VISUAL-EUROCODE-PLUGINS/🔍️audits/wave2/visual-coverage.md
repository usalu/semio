# 🔍️ Visual Coverage Audit, Wave 2 (NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS)

Read-only audit by a Haiku auditor. No source edits, builds or git writes. Scripts and raw JSON: `TICKET/🗑️generated/visual-coverage/` (`collections.py`, `rows.py`, `registry.py`, `matrix.py`, `matrix2.py`, `nested.py`, `nested_eval.py`, `checks.py`, `checkbind.py`, `gen.py`; data `rows.json`, `matrix2.json`, `nested_eval.json`, `checkbind.json`).

## 🧭️ Method and columns

Schema source: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<code>/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`. Top-level arrays are entity collections; arrays inside `$defs` are nested rows. Column meanings:

- **a 3D**: the collection is an entity kind in the artifact's `ENTITY_KINDS` registry (`🎬️scene*/🧰️kit/🦀️.rs`, which emits `EntityKind::*`), or its key is read by scene code. The registry is the strong signal; the key-read fallback is weaker.
- **b grip**: the collection path (`"key"`, `key[id=…]`, `path_of("key")`) appears in a `🕹️handles` source. The grip field list is heuristic and omitted here where noisy.
- **c diagram**: the key or item type is referenced in a `📈️diagrams` source. Heuristic; a hit means some diagram reads the collection, not that every field is charted.
- **d insert/outliner**: `insert` = in the insert palette (`insert_specs()` from `ENTITY_KINDS`, `case` excluded by design). The outliner lists every top-level array generically (`render_outliner_document`), so every top-level collection is in the outliner; nested rows are not.
- **e showcase**: the kebab-case key starts a row in a showcase `🗣️.dsl.semio` under `🖼️assets` or `📚️examples`.

Static evidence only: nothing was run. Literal matches on short words (`members`, `cases`) can over-count, so treat ✓ as "wired" and confirm before relying on it.

## 🗺️ Matrix per code

### ⚖️en1990 (EN 1990)

Top-level collections: 14. Nested rows: 2. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 8.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `permanents` | PermanentAction | ✓ | ✓ | ✓ | ✓ registry | ✓ | a_ek |
| `variables` | VariableAction | ✓ | ✓ | ✓ | ✓ registry | ✓ | a_ek |
| `accidentals` | AccidentalAction | ✓ | ✓ | ✓ | ✓ registry | ✓ | a_ek |
| `seismics` | SeismicAction | ✓ | ✗ | ✓ | ✓ registry | ✓ |  |
| `members` | Member | ✓ | ✓ | ✓ | ✓ registry | ✓ | label_de, label_en, precamber, span |
| `bridgeSls` | BridgeSls | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `effects` | MemberEffect | ✓ | ✗ | ✗ | outliner only | ✓ |  |
| `fatigueSpectra` | FatigueSpectrum | ✓ | ✓ | ✗ | ✓ registry | ✓ | collection |
| `reliabilityAnalyses` | ReliabilityAnalysis | ✓ | ✓ | ✓ | ✓ registry | ✓ | owner, per_metre |
| `testSeries` | TestSeries | ✓ | ✗ | ✓ | ✓ registry | ✓ |  |
| `placements` | Placement | ✓ | ✗ | ✗ | outliner only | ✓ |  |
| `cases` | CaseRow | ✓ | ✓ | ✓ | ✗ (excluded: case) | ✓ |  |
| `factorOverrides` | FactorOverride | ✓ | ✓ | ✗ | ✓ registry | ✓ | collection |
| `trafficGroups` | TrafficGroup | ✓ | ✗ | ✗ | ✓ registry | ✗ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `CaseRow.numbers` | ✗ | ✗ | ✗ | ✗ |
| `CaseRow.texts` | ✗ | ✗ | ✗ | ✗ |

### ⚡️din18599 (DIN 18599)

Top-level collections: 2. Nested rows: 2. Registry kit: `none`. Showcase docs: 5.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `zones` | ThermalZone | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `elements` | EnvelopeElement | ✗ | ✗ | ✗ | outliner only | ✓ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `MonthlyClimate.thetaEC` | ✗ | ✗ | ✗ | ✗ |
| `MonthlyClimate.gHWM2` | ✗ | ✗ | ✗ | ✗ |

Note: no `🎬️scene` folder exists in this artifact, so no scene crate and no 3D projection.

### 🌍️en1997 (EN 1997)

Top-level collections: 13. Nested rows: 6. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 3.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `layers` | SoilLayer | ✓ | ✓ | ✓ | ✓ registry | ✓ |  |
| `footings` | SpreadFoundation | ✓ | ✓ | ✓ | ✓ registry | ✓ | depth |
| `piles` | Pile | ✓ | ✓ | ✓ | ✓ registry | ✓ | length |
| `retainingWalls` | RetainingWall | ✓ | ✓ | ✓ | ✓ registry | ✓ | embedment, height, reinforcement_spacing |
| `slopes` | Slope | ✓ | ✓ | ✗ | ✓ registry | ✓ | height, length |
| `upliftCases` | UpliftCase | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `anchors` | Anchor | ✓ | ✓ | ✗ | ✓ registry | ✓ | fixed_length, free_length |
| `embeddedWalls` | EmbeddedWall | ✓ | ✓ | ✓ | ✓ registry | ✓ | embedment, prop_depth, retained_height |
| `pileGroups` | PileGroup | ✓ | ✓ | ✗ | ✓ registry | ✓ | columns, rows |
| `labTests` | LabTest | ✓ | ✗ | ✓ | ✓ registry | ✓ |  |
| `layerWaters` | LayerWater | ✓ | ✓ | ✗ | ✓ registry | ✓ | depth, piezometric_depth, well_water_depth |
| `placements` | Placement | ✓ | ✓ | ✓ | outliner only | ✓ |  |
| `cases` | CaseRow | ✓ | ✓ | ✓ | ✗ (excluded: case) | ✓ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `SpreadFoundation.loadCases` | ✓ | ✓ | ✗ | ✗ |
| `Pile.testProfiles` | ✗ | ✗ | ✗ | ✗ |
| `Anchor.tests` | ✓ | ✗ | ✗ | ✗ |
| `LabTest.points` | ✓ | ✗ | ✓ | ✗ |
| `CaseRow.numbers` | ✗ | ✗ | ✗ | ✗ |
| `CaseRow.texts` | ✗ | ✗ | ✗ | ✗ |

### 🌬️din16798 (DIN 16798)

Top-level collections: 2. Nested rows: 0. Registry kit: `none`. Showcase docs: 4.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `zones` | ZoneDocument | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `ventSystems` | VentSystemDocument | ✗ | ✗ | ✗ | outliner only | ✓ |  |


Note: no `🎬️scene` folder exists in this artifact, so no scene crate and no 3D projection.

### 🏋️en1991 (EN 1991)

Top-level collections: 22. Nested rows: 4. Registry kit: `🎬️scene-members/🧰️kit/🦀️.rs`. Showcase docs: 2.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `floors` | FloorArea | ✓ | ✓ | ✓ | ✓ registry | ✓ | area, thickness |
| `selfWeightElements` | SelfWeightElement | ✓ | ✓ | ✗ | ✓ registry | ✓ | area, thickness |
| `roofs` | RoofArea | ✓ | ✓ | ✓ | ✓ registry | ✓ | area, crane, obstruction, parapet, pitch, silo |
| `windFaces` | WindFace | ✓ | ✓ | ✗ | ✓ registry | ✓ | area |
| `accidentalCases` | AccidentalCase | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `bridgeLanes` | BridgeLane | ✓ | ✓ | ✗ | ✓ registry | ✓ | number, offset, width |
| `constructionStages` | ConstructionStage | ✓ | ✓ | ✗ | ✓ registry | ✓ | duration, load |
| `roofSurfaces` | RoofSurface | ✓ | ✓ | ✗ | ✓ registry | ✓ | extent, pitch |
| `placements` | Placement | ✓ | ✓ | ✗ | outliner only | ✓ | bridge, building, crane, silo |
| `specialVehicles` | SpecialVehicle | ✓ | ✓ | ✗ | ✓ registry | ✓ |  |
| `windBridges` | WindBridge | ✓ | ✓ | ✗ | ✓ registry | ✓ | height |
| `dynamicElements` | DynamicElement | ✓ | ✓ | ✓ | ✓ registry | ✓ | height |
| `impactActions` | ImpactAction | ✓ | ✓ | ✗ | ✓ registry | ✓ | height |
| `internalExplosions` | InternalExplosion | ✓ | ✓ | ✓ | ✓ registry | ✓ | depth |
| `thermalSections` | ThermalSection | ✓ | ✓ | ✗ | ✓ registry | ✓ | depth |
| `cases` | CaseRow | ✓ | ✓ | ✓ | ✗ (excluded: case) | ✓ |  |
| `helicopterLandings` | HelicopterLanding | ✓ | ✓ | ✗ | ✓ registry | ✗ | height |
| `tieRuns` | TieRun | ✓ | ✓ | ✗ | ✓ registry | ✗ |  |
| `constructionLoads` | ConstructionLoad | ✓ | ✓ | ✗ | ✓ registry | ✗ | drop_height |
| `fallingLoads` | FallingLoad | ✓ | ✓ | ✗ | ✓ registry | ✗ | drop_height, height |
| `executionImperfections` | ExecutionImperfection | ✓ | ✓ | ✗ | ✓ registry | ✗ | height |
| `derailmentCases` | DerailmentCase | ✓ | ✓ | ✗ | ✓ registry | ✗ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `AccidentalCase.impact` | ✓ | ✗ | ✓ | ✓ |
| `AccidentalCase.explosion` | ✓ | ✗ | ✗ | ✗ |
| `CaseRow.numbers` | ✗ | ✗ | ✗ | ✗ |
| `CaseRow.texts` | ✗ | ✗ | ✗ | ✗ |

### 🏛️en1992 (EN 1992)

Top-level collections: 7. Nested rows: 9. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 5.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `concreteGrades` | ConcreteGrade | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `reinforcementGrades` | ReinforcementGrade | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `prestressSteels` | PrestressSteel | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `members` | RcMember | ✓ | ✓ | ✓ | ✓ registry | ✓ |  |
| `anchors` | Anchor | ✓ | ✓ | ✗ | ✓ registry | ✓ |  |
| `placements` | Placement | ✓ | ✗ | ✗ | outliner only | ✓ |  |
| `cases` | CaseRow | ✓ | ✗ | ✗ | ✗ (excluded: case) | ✓ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `RcMember.longitudinal` | ✓ | ✓ | ✓ | ✗ |
| `RcMember.actions` | ✓ | ✗ | ✗ | ✗ |
| `RcMember.sectionPoints` | ✗ | ✗ | ✗ | ✗ |
| `RcMember.bars` | ✓ | ✗ | ✓ | ✗ |
| `RcMember.tendonPoints` | ✓ | ✗ | ✗ | ✗ |
| `RcMember.studRings` | ✓ | ✓ | ✗ | ✗ |
| `Anchor.actions` | ✓ | ✗ | ✗ | ✗ |
| `CaseRow.numbers` | ✗ | ✗ | ✗ | ✗ |
| `CaseRow.texts` | ✓ | ✗ | ✗ | ✗ |

### 🔩️en1993 (EN 1993)

Top-level collections: 26. Nested rows: 8. Registry kit: `🎬️scene-members/🧰️kit/🦀️.rs`. Showcase docs: 2.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `materials` | SteelMaterial | ✓ | ✗ | ✓ | outliner only | ✓ |  |
| `sections` | SteelSection | ✓ | ✗ | ✓ | outliner only | ✓ |  |
| `members` | SteelMember | ✓ | ✓ | ✓ | ✓ registry | ✓ | bolt_diameter, label, length, ltb_restraint_spacing, member_id, plate_thickness |
| `loadCases` | LoadCase | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `memberActions` | MemberAction | ✓ | ✗ | ✗ | outliner only | ✓ |  |
| `joints` | SteelJoint | ✓ | ✓ | ✗ | ✓ registry | ✓ |  |
| `fatigueDetails` | FatigueDetail | ✓ | ✗ | ✓ | ✓ registry | ✓ |  |
| `fireExposures` | FireExposure | ✓ | ✗ | ✓ | ✓ registry | ✓ |  |
| `coldFormedMembers` | ColdFormedMember | ✓ | ✓ | ✗ | ✓ registry | ✓ | length, lip_length, section_id, web_height |
| `platedPanels` | PlatedPanel | ✓ | ✓ | ✗ | ✓ registry | ✓ |  |
| `siloShells` | SiloShell | ✓ | ✓ | ✗ | ✓ registry | ✓ | deck_thickness, depth, embedded, embedded_length, radius |
| `tensionComponents` | TensionComponent | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `bridgeFatigue` | BridgeFatigue | ✓ | ✓ | ✗ | ✓ registry | ✓ | deck_thickness, ring_spacing, web_height |
| `towerLegs` | TowerLeg | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `piles` | SteelPile | ✓ | ✓ | ✗ | ✓ registry | ✓ | embedded, embedded_length |
| `craneRunways` | CraneRunway | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `storeys` | Storey | ✓ | ✓ | ✗ | ✓ registry | ✓ | height, member_id |
| `restraints` | Restraint | ✓ | ✓ | ✓ | ✓ registry | ✓ | hinge_to_restraint, lateral_spring_position, length, load_height, section_id |
| `builtUpMembers` | BuiltUpMember | ✓ | ✓ | ✗ | ✓ registry | ✓ | chain_spacing, length, panel_spacing, throat |
| `weldGroups` | WeldGroup | ✓ | ✓ | ✗ | ✓ registry | ✓ | eccentricity, throat |
| `parametricSections` | ParametricSection | ✓ | ✓ | ✗ | ✓ registry | ✓ | flange_thickness, section_id |
| `hollowJoints` | HollowJoint | ✓ | ✓ | ✗ | ✓ registry | ✓ |  |
| `jointRows` | JointRow | ✓ | ✓ | ✗ | ✓ registry | ✓ | bolt_to_web, edge_distance, plate_thickness, row-bolts, section_id |
| `frameNodes` | FrameNode | ✓ | ✓ | ✗ | ✓ registry | ✓ | ring_spacing |
| `frameBars` | FrameBar | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `placements` | Placement | ✓ | ✗ | ✗ | outliner only | ✓ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `SteelJoint.actions` | ✓ | ✗ | ✗ | ✗ |
| `FatigueDetail.spectrum` | ✓ | ✗ | ✓ | ✗ |
| `ColdFormedMember.actions` | ✓ | ✗ | ✗ | ✗ |
| `PlatedPanel.actions` | ✓ | ✗ | ✗ | ✗ |
| `TensionComponent.actions` | ✓ | ✗ | ✗ | ✗ |
| `TowerLeg.actions` | ✓ | ✗ | ✗ | ✗ |
| `SteelPile.actions` | ✓ | ✗ | ✗ | ✗ |
| `CraneRunway.actions` | ✓ | ✗ | ✗ | ✗ |

### 🧩️en1994 (EN 1994)

Top-level collections: 4. Nested rows: 3. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 3.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `beams` | CompositeBeam | ✓ | ✓ | ✓ | ✓ registry | ✓ |  |
| `columns` | CompositeColumn | ✓ | ✓ | ✓ | ✓ registry | ✓ |  |
| `slabs` | CompositeSlab | ✓ | ✓ | ✓ | ✓ registry | ✓ | concrete_thickness_m, sheeting-height, span, thickness |
| `placements` | Placement | ✓ | ✗ | ✗ | outliner only | ✓ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `CompositeBeam.actions` | ✓ | ✓ | ✓ | ✗ |
| `CompositeColumn.actions` | ✓ | ✓ | ✓ | ✗ |
| `CompositeSlab.actions` | ✓ | ✓ | ✓ | ✗ |

### 🧱️din4108 (DIN 4108)

Top-level collections: 3. Nested rows: 3. Registry kit: `none`. Showcase docs: 2.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `zones` | ThermalZone | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `elements` | EnvelopeElement | ✗ | ✗ | ✗ | outliner only | ✓ |  |
| `thermalBridges` | ThermalBridge | ✗ | ✗ | ✗ | outliner only | ✓ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `LayerDocument.segments` | ✗ | ✗ | ✗ | ✓ |
| `ThermalZone.windows` | ✗ | ✗ | ✗ | ✗ |
| `EnvelopeElement.layers` | ✗ | ✗ | ✗ | ✗ |

Note: no `🎬️scene` folder exists in this artifact, so no scene crate and no 3D projection.

### 🪨️en1996 (EN 1996)

Top-level collections: 5. Nested rows: 12. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 2.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `walls` | MasonryWall | ✓ | ✓ | ✓ | ✓ registry | ✓ | bearing-depth, bearing-length, eccentricity, height |
| `placements` | Placement | ✓ | ✗ | ✗ | outliner only | ✓ |  |
| `cases` | CaseRow | ✓ | ✓ | ✓ | ✗ (excluded: case) | ✓ |  |
| `chases` | WallChase | ✓ | ✓ | ✗ | ✓ registry | ✓ | depth, length, position-x, position-y |
| `movementJoints` | MovementJoint | ✓ | ✓ | ✗ | ✓ registry | ✓ | position, spacing |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `WallLoadCase.concentrated` | ✓ | ✓ | ✗ | ✓ |
| `MasonryWall.openings` | ✓ | ✓ | ✗ | ✗ |
| `MasonryWall.loadCases` | ✓ | ✓ | ✗ | ✗ |
| `MasonryWall.piers` | ✓ | ✗ | ✗ | ✗ |
| `MasonryWall.lintels` | ✓ | ✗ | ✗ | ✗ |
| `MasonryWall.ties` | ✓ | ✗ | ✗ | ✗ |
| `MasonryWall.prestress` | ✓ | ✗ | ✓ | ✗ |
| `MasonryWall.confined` | ✓ | ✗ | ✗ | ✗ |
| `MasonryWall.flanges` | ✓ | ✗ | ✗ | ✗ |
| `MasonryWall.bars` | ✓ | ✗ | ✗ | ✗ |
| `CaseRow.numbers` | ✗ | ✗ | ✗ | ✗ |
| `CaseRow.texts` | ✓ | ✗ | ✗ | ✗ |

### 🪵️en1995 (EN 1995)

Top-level collections: 13. Nested rows: 6. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 4.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `members` | TimberMember | ✓ | ✓ | ✓ | ✓ registry | ✓ | label_de, label_en, length, span |
| `connections` | TimberConnection | ✓ | ✓ | ✓ | ✓ registry | ✓ | diameter, edge-distance, end-distance, number, rows |
| `cases` | CaseRow | ✓ | ✓ | ✓ | ✗ (excluded: case) | ✓ |  |
| `placements` | Placement | ✓ | ✓ | ✗ | outliner only | ✓ |  |
| `tapers` | Taper | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `cutouts` | Cutout | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `fasteners` | Fastener | ✓ | ✗ | ✓ | ✓ registry | ✓ |  |
| `panels` | Panel | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `floors` | Floor | ✓ | ✗ | ✓ | ✓ registry | ✓ |  |
| `diaphragms` | Diaphragm | ✓ | ✗ | ✗ | ✓ registry | ✗ |  |
| `trusses` | Truss | ✓ | ✗ | ✗ | ✓ registry | ✗ |  |
| `bracings` | Bracing | ✓ | ✗ | ✗ | ✓ registry | ✗ |  |
| `decks` | Deck | ✓ | ✗ | ✗ | ✓ registry | ✗ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `TimberConnection.actions` | ✓ | ✗ | ✓ | ✗ |
| `Fastener.positions` | ✗ | ✗ | ✗ | ✗ |
| `Panel.layers` | ✓ | ✗ | ✓ | ✗ |
| `TimberMember.actions` | ✓ | ✗ | ✓ | ✗ |
| `CaseRow.numbers` | ✓ | ✓ | ✓ | ✗ |
| `CaseRow.texts` | ✓ | ✗ | ✓ | ✗ |

### 🪶️en1999 (EN 1999)

Top-level collections: 15. Nested rows: 5. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 2.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `materials` | AluminiumMaterial | ✓ | ✗ | ✓ | outliner only | ✓ |  |
| `sections` | AluminiumSection | ✓ | ✓ | ✓ | outliner only | ✓ | depth, diameter |
| `members` | AluminiumMember | ✓ | ✓ | ✓ | ✓ registry | ✓ | depth, diameter, flange-thickness |
| `connections` | AluminiumConnection | ✓ | ✓ | ✗ | ✓ registry | ✓ | bolt-diameter, edge-distance, edge_distance, gauge |
| `fireScenarios` | FireScenario | ✓ | ✓ | ✓ | ✓ registry | ✓ | duration, section_id, start-temperature |
| `fatigueDetails` | FatigueDetail | ✓ | ✓ | ✓ | ✓ registry | ✓ | cycles, delta-sigma, section_id |
| `coldFormed` | ColdFormedSheet | ✓ | ✓ | ✗ | ✓ registry | ✓ | span, thickness, width |
| `shells` | AluminiumShell | ✓ | ✓ | ✓ | ✓ registry | ✓ | length, radius, section_id, thickness |
| `placements` | Placement | ✓ | ✓ | ✗ | outliner only | ✓ |  |
| `weldRuns` | WeldRun | ✓ | ✓ | ✓ | ✓ registry | ✓ | length, section_id, start |
| `outlineVertices` | SectionVertex | ✓ | ✓ | ✗ | outliner only | ✓ | section_id |
| `fatigueCategories` | FatigueCategory | ✓ | ✓ | ✗ | ✓ registry | ✓ | bearing, bearing_length, length, location, position, section_id |
| `patchLoads` | PatchLoad | ✓ | ✓ | ✗ | ✓ registry | ✓ | bearing, bearing_length, force, length, position |
| `ringStiffeners` | RingStiffener | ✓ | ✓ | ✗ | ✓ registry | ✓ | depth, length, position, section_id |
| `transverseStiffeners` | TransverseStiffener | ✓ | ✓ | ✗ | ✓ registry | ✗ | length, position, section_id, thickness, width |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `ColdFormedSheet.actions` | ✓ | ✗ | ✗ | ✗ |
| `AluminiumMember.actions` | ✓ | ✗ | ✗ | ✗ |
| `AluminiumSection.elements` | ✓ | ✓ | ✓ | ✗ |
| `AluminiumConnection.actions` | ✓ | ✗ | ✗ | ✗ |
| `AluminiumShell.actions` | ✓ | ✗ | ✗ | ✗ |

### 🫨️en1998 (EN 1998)

Top-level collections: 14. Nested rows: 6. Registry kit: `🎬️scene/🧰️kit/🦀️.rs`. Showcase docs: 4.

| Collection | Item type | (a) 3D | (b) grip | (c) diagram | (d) insert | (e) showcase | Grip fields (heuristic) |
|---|---|---|---|---|---|---|---|
| `buildings` | En1998Building | ✓ | ✓ | ✓ | ✓ registry | ✓ | name |
| `bridges` | En1998Bridge | ✓ | ✓ | ✗ | ✓ registry | ✓ | pier_height_m |
| `assessments` | En1998Assessment | ✓ | ✓ | ✗ | ✓ registry | ✓ | name |
| `silos` | En1998Silo | ✓ | ✓ | ✗ | ✓ registry | ✓ | radius_m |
| `tanks` | En1998Tank | ✓ | ✓ | ✗ | ✓ registry | ✓ | height_m, radius_m |
| `foundations` | En1998Foundation | ✓ | ✓ | ✗ | ✓ registry | ✓ | pile_interface_depth_m, pile_length_m |
| `retainingWalls` | En1998RetainingWall | ✓ | ✓ | ✗ | ✓ registry | ✓ | height_m |
| `towers` | En1998Tower | ✓ | ✓ | ✗ | ✓ registry | ✓ | height_m, radius_m |
| `soilLayers` | En1998SoilLayer | ✓ | ✓ | ✗ | ✓ registry | ✓ | height_m, radius_m |
| `diaphragms` | En1998Diaphragm | ✓ | ✓ | ✗ | ✓ registry | ✓ |  |
| `accelerograms` | En1998Accelerogram | ✓ | ✗ | ✗ | ✓ registry | ✓ |  |
| `piers` | En1998Pier | ✓ | ✓ | ✗ | ✓ registry | ✓ |  |
| `bearings` | En1998Bearing | ✓ | ✓ | ✗ | ✓ registry | ✓ | pile_length_m |
| `placements` | En1998Placement | ✓ | ✗ | ✗ | outliner only | ✓ |  |

Nested rows (a, b, c, e; d not applicable):

| Nested row | (a) 3D | (b) grip | (c) diagram | (e) showcase |
|---|---|---|---|---|
| `En1998Storey.variables` | ✗ | ✗ | ✗ | ✓ |
| `En1998Building.systems` | ✓ | ✗ | ✓ | ✗ |
| `En1998Building.storeys` | ✓ | ✓ | ✓ | ✗ |
| `En1998Building.members` | ✓ | ✗ | ✗ | ✗ |
| `En1998Bridge.variables` | ✗ | ✗ | ✗ | ✓ |
| `En1998Tower.variables` | ✗ | ✗ | ✗ | ✓ |

## 📊 Totals (top-level collections only)

Top-level collections: 140. (a) 3D: 129. (b) grip: 94. (c) diagram: 46. (d) insert: 106. (e) showcase: 128. Nested rows listed: 66.

## 🚫 Collections lacking 3D or grips

### Lacking 3D (a)

- ⚡️din18599: `zones`, `elements`
- 🌬️din16798: `zones`, `ventSystems`
- 🏛️en1992: `concreteGrades`, `reinforcementGrades`, `prestressSteels`
- 🔩️en1993: `loadCases`
- 🧱️din4108: `zones`, `elements`, `thermalBridges`
- Whole artifacts with no scene at all: ⚡️din18599 (2 collections), 🌬️din16798 (2), 🧱️din4108 (3), 🏭️vdi3805 (0 top-level; 20 nested rows), 📇️iso16757 (0 top-level; 36 nested rows).

### Lacking grips (b)

- ⚖️en1990 (6 of 14): `seismics`, `bridgeSls`, `effects`, `testSeries`, `placements`, `trafficGroups`
- ⚡️din18599 (2 of 2): `zones`, `elements`
- 🌍️en1997 (2 of 13): `upliftCases`, `labTests`
- 🌬️din16798 (2 of 2): `zones`, `ventSystems`
- 🏋️en1991 (1 of 22): `accidentalCases`
- 🏛️en1992 (5 of 7): `concreteGrades`, `reinforcementGrades`, `prestressSteels`, `placements`, `cases`
- 🔩️en1993 (11 of 26): `materials`, `sections`, `loadCases`, `memberActions`, `fatigueDetails`, `fireExposures`, `tensionComponents`, `towerLegs`, `craneRunways`, `frameBars`, `placements`
- 🧩️en1994 (1 of 4): `placements`
- 🧱️din4108 (3 of 3): `zones`, `elements`, `thermalBridges`
- 🪨️en1996 (1 of 5): `placements`
- 🪵️en1995 (9 of 13): `tapers`, `cutouts`, `fasteners`, `panels`, `floors`, `diaphragms`, `trusses`, `bracings`, `decks`
- 🪶️en1999 (1 of 15): `materials`
- 🫨️en1998 (2 of 14): `accelerograms`, `placements`

Nested rows lacking grips: ⚖️en1990 2 of 2; ⚡️din18599 2 of 2; 🌍️en1997 5 of 6; 🏋️en1991 4 of 4; 🏛️en1992 7 of 9; 🏭️vdi3805 20 of 20; 📇️iso16757 36 of 36; 🔩️en1993 8 of 8; 🧩️en1994 0 of 3; 🧱️din4108 3 of 3; 🪨️en1996 9 of 12; 🪵️en1995 5 of 6; 🪶️en1999 4 of 5; 🫨️en1998 5 of 6.

## 🧭️ Check families and 3D highlighting (step 3)

Mechanism: a check colours its entity only when its `subject.entity_id` matches a scene entity (`binding_of_subject`, `UtilisationBinding`). Check rows without a subject are filed under an aggregate entity; `report_owner` in `🎬️scene-members/🧰️kit` does that for EN 1991 only. Static proxy: a check id counts as bound if its literal id (or its family prefix) appears in the artifact's scene code.

| Code | Check ids (spec-trace) | Literal-bound in scene | Families | Families with no bound id | Failure/glyph refs | Unbound families (sample) |
|---|---|---|---|---|---|---|
| ⚖️en1990 | 33 | 17 | 17 | 8 | 32 | `en1990.6`, `en1990.e10`, `en1990.e11`, `en1990.e12` |
| 🌍️en1997 | 43 | 9 | 33 | 26 | 23 | `en1997.10`, `en1997.anc-01`, `en1997.anc-02`, `en1997.anc-03` |
| 🏋️en1991 | 33 | 19 | 7 | 0 | 43 |  |
| 🏛️en1992 | 57 | 16 | 36 | 26 | 24 | `en1992-2.6`, `en1992-2.7`, `en1992-4.edge`, `en1992-4.interaction` |
| 🔩️en1993 | 97 | 24 | 21 | 12 | 33 | `en1993.1-10`, `en1993.1-11`, `en1993.1-3`, `en1993.1-4` |
| 🧩️en1994 | 48 | 23 | 7 | 1 | 16 | `en1994.3` |
| 🪨️en1996 | 39 | 18 | 29 | 15 | 19 | `en1996.2`, `en1996.3`, `m-01.`, `m-02.` |
| 🪵️en1995 | 26 | 7 | 17 | 13 | 21 | `en1995.1-2`, `en1995.2`, `en1995.6`, `en1995.material` |
| 🪶️en1999 | 41 | 15 | 7 | 2 | 30 | `en1999.1-4`, `en1999.8` |
| 🫨️en1998 | 50 | 23 | 48 | 25 | 30 | `.analysis`, `.componentCombination`, `.displacement`, `.elevationRegularity` |

Failing checks are highlighted in 3D only where bound. EN 1991 is fully bound by family (7 families, all bound, `report_owner` routes unsubjected checks). Other artifacts have families with no literal scene binding, so their failures stay in the checks table and are not highlighted in the scene.

## 🎯 Top 15 visual gaps (ranked by user impact)

1. **Building-physics artifacts have no 3D at all.** ⚡️din18599 (zones, elements), 🌬️din16798 (zones, ventSystems), 🧱️din4108 (zones, elements, thermalBridges): no `🎬️scene`, no grips, no insert palette. Users edit thermal zones and envelope faces in properties only. Next action: Scaffold `🎬️scene` per artifact from the 🧩️en1994 kit: zone volumes and envelope faces as 3D entities, grips on areaM2 and orientation, register them in `ENTITY_KINDS`, add one golden scene test and a `🥒️.feature`.

2. **Catalogue standards have no 3D for product geometry.** 📇️iso16757 (36 nested rows, incl. GeometryObject ports/surfaces, PortDefinition position/direction, BoundingBox) and 🏭️vdi3805 (20 nested rows, ParametricGeometry connections, ConnectionPoint): zero scene, despite geometry data in the snapshot. Next action: Add a catalogue scene per artifact: bounding box wireframe, port arrows and connection points as entities; grips on BoundingBox min/max and PortDefinition position.

3. **Timber has grips on only 4 of 13 collections.** 🪵️en1995: tapers, cutouts, fasteners, panels, floors, diaphragms, trusses, bracings and decks have no grip; diaphragms, trusses, bracings and decks also have no diagram and no showcase row. Next action: Add ScalarAlong handles for span, width, depth and spacing in `🕹️handles` for each of the nine; add a diagram for diaphragm shear and truss forces; add showcase rows for the four missing collections.

4. **Steel sections, materials and load cases are not editable in 3D.** 🔩️en1993: sections and materials have no grip and no insert entry; loadCases has no 3D, no grip, no insert and no diagram. Section profile (h, b, tw, tf) is not gripped on the member it drives. Next action: Put section-profile handles (h, b, tw, tf) on the member that references the section; add loadCases as a legend-anchored entity; add sections and materials to ENTITY_KINDS for insert.

5. **Check families do not highlight their entity in 3D.** Literal scene binding is missing for many families: 🌍️en1997 26 of 33, 🏛️en1992 26 of 36, 🫨️en1998 25 of 48, 🪨️en1996 15 of 29, 🔩️en1993 12 of 21, 🪵️en1995 13 of 17, ⚖️en1990 8 of 17 (about 125 families). Only 🏋️en1991 routes unsubjected checks via `report_owner`. Next action: Generalise the EN 1991 `report_owner` pattern into the shared kit (per artifact map of check-id prefix to entity or aggregate); add a test that every spec-trace check id resolves to a scene entity or an aggregate, and that a failing check colours that entity.

6. **EN 1998 has almost no diagrams.** 🫨️en1998: 13 of 14 collections have no diagram (only buildings). Seismic results (bridges, silos, tanks, towers, foundations, bearings, piers, accelerograms) are only visible in the checks table. Next action: Add diagrams for storey drift and base shear (buildings, towers), capacity curves (piers, bridges), pressure and fill-level charts (silos, tanks), and accelerogram response for accelerograms.

7. **EN 1991 diagrams and showcase cover a minority of collections.** 🏋️en1991: 17 of 22 collections have no diagram (windFaces, roofSurfaces, bridgeLanes, constructionStages, specialVehicles, windBridges, impactActions, thermalSections and others). Six collections are missing from the showcase DSL: helicopterLandings, tieRuns, constructionLoads, fallingLoads, executionImperfections, derailmentCases. Next action: Add a wind-pressure plan diagram (windFaces, roofSurfaces), a lane-position diagram (bridgeLanes, specialVehicles), and a time-line diagram (constructionStages); add the six missing rows to the `de-office-compliant` showcase; add a grip for accidentalCases.

8. **Placements have no grips in six artifacts.** Placement drag in plan is missing in 🏛️en1992, 🔩️en1993, 🧩️en1994, 🪨️en1996, ⚖️en1990 and 🫨️en1998 (placements listed as no-grip). 🏋️en1991, 🌍️en1997 and 🪵️en1995 do grip them. Next action: Port the 🏋️en1991 placement handles (x, y, yaw) to the other six kits; use the shared `layout` module so the same drag works everywhere.

9. **Nested action and layer rows are never gripped.** Across artifacts, nested rows such as `actions`, `spectrum`, `impact`, `explosion`, `variables`, `testProfiles` and `points` have no grip in most artifacts: 🏋️en1991 4 of 4, 🔩️en1993 8 of 8, 🌍️en1997 5 of 6, 🪶️en1999 4 of 5, 🫨️en1998 5 of 6. Their values are editable only in properties. Next action: Give each nested row with a geometric or quantity field a handle on its parent entity (for example, opening width and height on the wall, a layer thickness stack on the element), or a stacked-layer diagram.

10. **EN 1990 has six collections without grips and no trafficGroups showcase.** ⚖️en1990: seismics, bridgeSls, effects, testSeries, placements and trafficGroups have no grip; effects and placements are not in the insert palette; bridgeSls and effects have no diagram; trafficGroups is not in any showcase. Next action: Add grips for seismic aEk and importance class, bridge SLS limits and test-series mean and standard deviation; add a trafficGroups row to a showcase; add a bridge SLS diagram.

11. **EN 1997 foundation and retaining collections are thin.** 🌍️en1997: upliftCases and labTests have no grip; upliftCases, slopes, anchors, pileGroups and layerWaters have no diagram; placements and cases are not in the insert palette. Next action: Add a grip for uplift destabilising load and lab-test stress; add a slope circle-slip diagram (slopes) and a groundwater head diagram (layerWaters); add pile-group and anchor-layout diagrams.

12. **EN 1999 has gaps in materials, outlines and showcase.** 🪶️en1999: materials has no grip; materials, sections, placements and outlineVertices are not in the insert palette; outlineVertices has no diagram; transverseStiffeners is not in the showcase. Next action: Put an outline-shape handle on sections (height, width, thickness) and an outline-vertex diagram; add transverseStiffeners to the showcase.

13. **EN 1996 walls: placements, chases and movement joints are thin.** 🪨️en1996: placements has no grip; chases and movementJoints have no diagram; cases is not in the insert palette. Wall geometry itself is gripped and diagrammed. Next action: Add wall-plan drag for placements; add a chase depth and position diagram and a movement-joint spacing diagram.

14. **EN 1992 material grades are invisible in 3D.** 🏛️en1992: concreteGrades, reinforcementGrades and prestressSteels have no 3D, no grip, no insert, no diagram. Members carry their grade, but no colour or legend links the grade to the member in 3D. Next action: Colour RC members by grade (legend from the grade table), add a grade legend diagram (fck, fyk) and show the grade swatch in each member's outliner entry.

15. **EN 1995 and EN 1992 showcase gaps.** Showcase rows are missing for diaphragms, trusses, bracings and decks in 🪵️en1995 and for trafficGroups in ⚖️en1990. Nested row RcMember.sectionPoints in 🏛️en1992 has no grip. Next action: Add the missing showcase rows in the same change as the grips (one showcase per family); add a grip for RcMember.sectionPoints (cross-section outline points) on the member.

## ✅ Not gaps (checked)

- 🏋️en1991 and 🌍️en1997 have 3D for every top-level collection; 🏋️en1991 binds all 7 check families.
- 🏭️vdi3805 and 📇️iso16757 showcase inclusion is high (22 and 2 DSL docs), so the gap there is 3D, not showcase.
- Insert palette for `case` is excluded on purpose (design cases are not placed in 3D).

## 🛑 Caveats

- Static, literal-match analysis. No builds, no runs, no screenshots.
- Grip fields are inferred from literals and can contain noise (for example numeric tokens); they are dropped from the gap text.
- Check-to-entity binding is a literal proxy; a runtime test is the real confirmation.
- The scene-kit file used for the registry per code: en1990, en1992, en1994, en1995, en1996, en1997, en1998, en1999 use `🎬️scene/🧰️kit`; en1991 and en1993 use `🎬️scene-members/🧰️kit`.
