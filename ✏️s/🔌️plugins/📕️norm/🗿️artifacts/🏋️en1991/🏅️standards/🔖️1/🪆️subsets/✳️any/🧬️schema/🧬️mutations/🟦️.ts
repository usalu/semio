/** 🧺️ `En1991Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ChangeAssumedCraneHorizontal, parseChangeAssumedCraneHorizontal } from "./↔️change-assumed-crane-horizontal/🧬️schema/🟦️.ts";
import { type ChangeBridgeLaneWidth, parseChangeBridgeLaneWidth } from "./↔️change-bridge-lane-width/🧬️schema/🟦️.ts";
import { type ChangeWindFaceZ, parseChangeWindFaceZ } from "./↕️change-wind-face-z/🧬️schema/🟦️.ts";
import { type ChangeHoistingSpeed, parseChangeHoistingSpeed } from "./⏫change-hoisting-speed/🧬️schema/🟦️.ts";
import { type ChangeFireDuration, parseChangeFireDuration } from "./⏱️change-fire-duration/🧬️schema/🟦️.ts";
import { type ChangeAssumedGasTemperature, parseChangeAssumedGasTemperature } from "./♨️change-assumed-gas-temperature/🧬️schema/🟦️.ts";
import { type ChangeSiloKind, parseChangeSiloKind } from "./⚖️change-silo-kind/🧬️schema/🟦️.ts";
import { type ChangeAssumedConstructionQk, parseChangeAssumedConstructionQk } from "./⚙️change-assumed-construction-qk/🧬️schema/🟦️.ts";
import { type ChangeRotatingMachineEccentricity, parseChangeRotatingMachineEccentricity } from "./⚙️change-rotating-machine-eccentricity/🧬️schema/🟦️.ts";
import { type ChangeRotatingMachineFrequency, parseChangeRotatingMachineFrequency } from "./⚙️change-rotating-machine-frequency/🧬️schema/🟦️.ts";
import { type ChangeRotatingMachineMass, parseChangeRotatingMachineMass } from "./⚙️change-rotating-machine-mass/🧬️schema/🟦️.ts";
import { type ChangeSiloK, parseChangeSiloK } from "./⚙️change-silo-k/🧬️schema/🟦️.ts";
import { type InsertRotatingMachine, parseInsertRotatingMachine } from "./⚙️insert-rotating-machine/🧬️schema/🟦️.ts";
import { type RemoveRotatingMachine, parseRemoveRotatingMachine } from "./⚙️remove-rotating-machine/🧬️schema/🟦️.ts";
import { type ChangeFireLoadDensityQf, parseChangeFireLoadDensityQf } from "./⛽change-fire-load-density-qf/🧬️schema/🟦️.ts";
import { type ChangeAltitude, parseChangeAltitude } from "./❄️change-altitude/🧬️schema/🟦️.ts";
import { type ChangeEnSk, parseChangeEnSk } from "./❄️change-en-sk/🧬️schema/🟦️.ts";
import { type ChangeHoistClass, parseChangeHoistClass } from "./➕change-hoist-class/🧬️schema/🟦️.ts";
import { type InsertAccidentalCase, parseInsertAccidentalCase } from "./➕️insert-accidental-case/🧬️schema/🟦️.ts";
import { type InsertCase, parseInsertCase } from "./➕️insert-case/🧬️schema/🟦️.ts";
import { type InsertFloor, parseInsertFloor } from "./➕️insert-floor/🧬️schema/🟦️.ts";
import { type InsertRoof, parseInsertRoof } from "./➕️insert-roof/🧬️schema/🟦️.ts";
import { type InsertSelfWeightElement, parseInsertSelfWeightElement } from "./➕️insert-self-weight-element/🧬️schema/🟦️.ts";
import { type InsertWindFace, parseInsertWindFace } from "./➕️insert-wind-face/🧬️schema/🟦️.ts";
import { type ChangeAssumedCraneWheel, parseChangeAssumedCraneWheel } from "./➖change-assumed-crane-wheel/🧬️schema/🟦️.ts";
import { type RemoveAccidentalCase, parseRemoveAccidentalCase } from "./➖️remove-accidental-case/🧬️schema/🟦️.ts";
import { type RemoveCase, parseRemoveCase } from "./➖️remove-case/🧬️schema/🟦️.ts";
import { type RemoveFloor, parseRemoveFloor } from "./➖️remove-floor/🧬️schema/🟦️.ts";
import { type RemoveRoof, parseRemoveRoof } from "./➖️remove-roof/🧬️schema/🟦️.ts";
import { type RemoveSelfWeightElement, parseRemoveSelfWeightElement } from "./➖️remove-self-weight-element/🧬️schema/🟦️.ts";
import { type RemoveWindFace, parseRemoveWindFace } from "./➖️remove-wind-face/🧬️schema/🟦️.ts";
import { type ChangeFallingAssumedForce, parseChangeFallingAssumedForce } from "./⬇️change-falling-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeFallingDropHeight, parseChangeFallingDropHeight } from "./⬇️change-falling-drop-height/🧬️schema/🟦️.ts";
import { type ChangeFallingMass, parseChangeFallingMass } from "./⬇️change-falling-mass/🧬️schema/🟦️.ts";
import { type ChangeFallingStoppingDistance, parseChangeFallingStoppingDistance } from "./⬇️change-falling-stopping-distance/🧬️schema/🟦️.ts";
import { type ChangeFallingX, parseChangeFallingX } from "./⬇️change-falling-x/🧬️schema/🟦️.ts";
import { type ChangeFallingY, parseChangeFallingY } from "./⬇️change-falling-y/🧬️schema/🟦️.ts";
import { type InsertFalling, parseInsertFalling } from "./⬇️insert-falling/🧬️schema/🟦️.ts";
import { type RemoveFalling, parseRemoveFalling } from "./⬇️remove-falling/🧬️schema/🟦️.ts";
import { type ChangeSiloDiameter, parseChangeSiloDiameter } from "./⭕change-silo-diameter/🧬️schema/🟦️.ts";
import { type ChangeSiloHydraulicRadius, parseChangeSiloHydraulicRadius } from "./⭕change-silo-hydraulic-radius/🧬️schema/🟦️.ts";
import { type ChangeAssumedSiloPressure, parseChangeAssumedSiloPressure } from "./🌀change-assumed-silo-pressure/🧬️schema/🟦️.ts";
import { type ChangeBridgeCurveRadius, parseChangeBridgeCurveRadius } from "./🌀change-bridge-curve-radius/🧬️schema/🟦️.ts";
import { type ChangeDynamicAssumedCscd, parseChangeDynamicAssumedCscd } from "./🌀️change-dynamic-assumed-cscd/🧬️schema/🟦️.ts";
import { type ChangeDynamicElementHeight, parseChangeDynamicElementHeight } from "./🌀️change-dynamic-element-height/🧬️schema/🟦️.ts";
import { type ChangeDynamicElementStrouhal, parseChangeDynamicElementStrouhal } from "./🌀️change-dynamic-element-strouhal/🧬️schema/🟦️.ts";
import { type ChangeDynamicElementWidth, parseChangeDynamicElementWidth } from "./🌀️change-dynamic-element-width/🧬️schema/🟦️.ts";
import { type ChangeDynamicFirstFrequency, parseChangeDynamicFirstFrequency } from "./🌀️change-dynamic-first-frequency/🧬️schema/🟦️.ts";
import { type ChangeDynamicLogDecrement, parseChangeDynamicLogDecrement } from "./🌀️change-dynamic-log-decrement/🧬️schema/🟦️.ts";
import { type ChangeDynamicMassPerLength, parseChangeDynamicMassPerLength } from "./🌀️change-dynamic-mass-per-length/🧬️schema/🟦️.ts";
import { type InsertDynamicElement, parseInsertDynamicElement } from "./🌀️insert-dynamic-element/🧬️schema/🟦️.ts";
import { type RemoveDynamicElement, parseRemoveDynamicElement } from "./🌀️remove-dynamic-element/🧬️schema/🟦️.ts";
import { type ChangeBridgeLane, parseChangeBridgeLane } from "./🌉change-bridge-lane/🧬️schema/🟦️.ts";
import { type ChangeStructureKind, parseChangeStructureKind } from "./🌉change-structure-kind/🧬️schema/🟦️.ts";
import { type ChangeThermalBridgeType, parseChangeThermalBridgeType } from "./🌉change-thermal-bridge-type/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeAssumedForce, parseChangeWindBridgeAssumedForce } from "./🌉️change-wind-bridge-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeDeckDepth, parseChangeWindBridgeDeckDepth } from "./🌉️change-wind-bridge-deck-depth/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeDeckWidth, parseChangeWindBridgeDeckWidth } from "./🌉️change-wind-bridge-deck-width/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeGroundHeight, parseChangeWindBridgeGroundHeight } from "./🌉️change-wind-bridge-ground-height/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeParapetHeight, parseChangeWindBridgeParapetHeight } from "./🌉️change-wind-bridge-parapet-height/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeParapetKind, parseChangeWindBridgeParapetKind } from "./🌉️change-wind-bridge-parapet-kind/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeSpan, parseChangeWindBridgeSpan } from "./🌉️change-wind-bridge-span/🧬️schema/🟦️.ts";
import { type ChangeWindBridgeTrafficKind, parseChangeWindBridgeTrafficKind } from "./🌉️change-wind-bridge-traffic-kind/🧬️schema/🟦️.ts";
import { type InsertWindBridge, parseInsertWindBridge } from "./🌉️insert-wind-bridge/🧬️schema/🟦️.ts";
import { type RemoveWindBridge, parseRemoveWindBridge } from "./🌉️remove-wind-bridge/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍change-annex/🧬️schema/🟦️.ts";
import { type ChangeAssumedDeltaT, parseChangeAssumedDeltaT } from "./🌡️change-assumed-delta-t/🧬️schema/🟦️.ts";
import { type ChangeTMax, parseChangeTMax } from "./🌡️change-t-max/🧬️schema/🟦️.ts";
import { type ChangeThermalAssumedRange, parseChangeThermalAssumedRange } from "./🌡️change-thermal-assumed-range/🧬️schema/🟦️.ts";
import { type ChangeThermalSectionAlphaMicro, parseChangeThermalSectionAlphaMicro } from "./🌡️change-thermal-section-alpha-micro/🧬️schema/🟦️.ts";
import { type ChangeThermalSectionDeckType, parseChangeThermalSectionDeckType } from "./🌡️change-thermal-section-deck-type/🧬️schema/🟦️.ts";
import { type ChangeThermalSectionDepth, parseChangeThermalSectionDepth } from "./🌡️change-thermal-section-depth/🧬️schema/🟦️.ts";
import { type ChangeThermalSectionLength, parseChangeThermalSectionLength } from "./🌡️change-thermal-section-length/🧬️schema/🟦️.ts";
import { type ChangeThermalSectionSurfacing, parseChangeThermalSectionSurfacing } from "./🌡️change-thermal-section-surfacing/🧬️schema/🟦️.ts";
import { type InsertThermalSection, parseInsertThermalSection } from "./🌡️insert-thermal-section/🧬️schema/🟦️.ts";
import { type RemoveThermalSection, parseRemoveThermalSection } from "./🌡️remove-thermal-section/🧬️schema/🟦️.ts";
import { type ChangeRoofAssumedSk, parseChangeRoofAssumedSk } from "./🌨️change-roof-assumed-sk/🧬️schema/🟦️.ts";
import { type ChangeEnVb, parseChangeEnVb } from "./🌬️change-en-vb/🧬️schema/🟦️.ts";
import { type ChangeWindFaceCPe1, parseChangeWindFaceCPe1 } from "./🌬️change-wind-face-c-pe1/🧬️schema/🟦️.ts";
import { type ChangeWindFaceCPe10, parseChangeWindFaceCPe10 } from "./🌬️change-wind-face-c-pe10/🧬️schema/🟦️.ts";
import { type ChangeWindFaceCPi, parseChangeWindFaceCPi } from "./🌬️change-wind-face-c-pi/🧬️schema/🟦️.ts";
import { type ChangeWindFaceCD, parseChangeWindFaceCD } from "./🌬️change-wind-face-cd/🧬️schema/🟦️.ts";
import { type ChangeWindFaceCS, parseChangeWindFaceCS } from "./🌬️change-wind-face-cs/🧬️schema/🟦️.ts";
import { type ChangeWindFaceZone, parseChangeWindFaceZone } from "./🌬️change-wind-face-zone/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeTandem, parseChangeAssumedBridgeTandem } from "./🌾change-assumed-bridge-tandem/🧬️schema/🟦️.ts";
import { type ChangeSiloBulkDensity, parseChangeSiloBulkDensity } from "./🌾change-silo-bulk-density/🧬️schema/🟦️.ts";
import { type ChangeBulkSolidInternalFriction, parseChangeBulkSolidInternalFriction } from "./🌾️change-bulk-solid-internal-friction/🧬️schema/🟦️.ts";
import { type InsertBulkSolid, parseInsertBulkSolid } from "./🌾️insert-bulk-solid/🧬️schema/🟦️.ts";
import { type RemoveBulkSolid, parseRemoveBulkSolid } from "./🌾️remove-bulk-solid/🧬️schema/🟦️.ts";
import { type ChangeSiloEccentricity, parseChangeSiloEccentricity } from "./🎯change-silo-eccentricity/🧬️schema/🟦️.ts";
import { type ChangeNorthGermanLowlandSnow, parseChangeNorthGermanLowlandSnow } from "./🏔️change-north-german-lowland-snow/🧬️schema/🟦️.ts";
import { type ChangeBridgeSpan, parseChangeBridgeSpan } from "./🏗️change-bridge-span/🧬️schema/🟦️.ts";
import { type ChangeConstructionLoadKind, parseChangeConstructionLoadKind } from "./🏗️change-construction-load-kind/🧬️schema/🟦️.ts";
import { type ChangeConstructionLoadMagnitude, parseChangeConstructionLoadMagnitude } from "./🏗️change-construction-load-magnitude/🧬️schema/🟦️.ts";
import { type ChangeConstructionLoadStage, parseChangeConstructionLoadStage } from "./🏗️change-construction-load-stage/🧬️schema/🟦️.ts";
import { type ChangeCraneClaimed, parseChangeCraneClaimed } from "./🏗️change-crane-claimed/🧬️schema/🟦️.ts";
import { type ChangeCraneHoistRatedLoad, parseChangeCraneHoistRatedLoad } from "./🏗️change-crane-hoist-rated-load/🧬️schema/🟦️.ts";
import { type ChangeCraneHoistReleaseDevice, parseChangeCraneHoistReleaseDevice } from "./🏗️change-crane-hoist-release-device/🧬️schema/🟦️.ts";
import { type ChangeCraneHoistReleaseFraction, parseChangeCraneHoistReleaseFraction } from "./🏗️change-crane-hoist-release-fraction/🧬️schema/🟦️.ts";
import { type ChangeCraneX, parseChangeCraneX } from "./🏗️change-crane-x/🧬️schema/🟦️.ts";
import { type ChangeStageActivity, parseChangeStageActivity } from "./🏗️change-stage-activity/🧬️schema/🟦️.ts";
import { type ChangeStageAssumedQk, parseChangeStageAssumedQk } from "./🏗️change-stage-assumed-qk/🧬️schema/🟦️.ts";
import { type ChangeStageDurationDays, parseChangeStageDurationDays } from "./🏗️change-stage-duration-days/🧬️schema/🟦️.ts";
import { type ChangeStageReturnYears, parseChangeStageReturnYears } from "./🏗️change-stage-return-years/🧬️schema/🟦️.ts";
import { type ChangeThermalElementType, parseChangeThermalElementType } from "./🏗️change-thermal-element-type/🧬️schema/🟦️.ts";
import { type InsertConstructionLoad, parseInsertConstructionLoad } from "./🏗️insert-construction-load/🧬️schema/🟦️.ts";
import { type InsertConstructionStage, parseInsertConstructionStage } from "./🏗️insert-construction-stage/🧬️schema/🟦️.ts";
import { type InsertCraneHoist, parseInsertCraneHoist } from "./🏗️insert-crane-hoist/🧬️schema/🟦️.ts";
import { type RemoveConstructionLoad, parseRemoveConstructionLoad } from "./🏗️remove-construction-load/🧬️schema/🟦️.ts";
import { type RemoveConstructionStage, parseRemoveConstructionStage } from "./🏗️remove-construction-stage/🧬️schema/🟦️.ts";
import { type RemoveCraneHoist, parseRemoveCraneHoist } from "./🏗️remove-crane-hoist/🧬️schema/🟦️.ts";
import { type ChangeStoreyCount, parseChangeStoreyCount } from "./🏙️change-storey-count/🧬️schema/🟦️.ts";
import { type ChangeCoastOrIsland, parseChangeCoastOrIsland } from "./🏝️change-coast-or-island/🧬️schema/🟦️.ts";
import { type ChangeTerrainCategory, parseChangeTerrainCategory } from "./🏞️change-terrain-category/🧬️schema/🟦️.ts";
import { type ChangeWidth, parseChangeWidth } from "./🏠change-width/🧬️schema/🟦️.ts";
import { type ChangeRoofCE, parseChangeRoofCE } from "./🏠️change-roof-ce/🧬️schema/🟦️.ts";
import { type ChangeRoofCT, parseChangeRoofCT } from "./🏠️change-roof-ct/🧬️schema/🟦️.ts";
import { type ChangeRoofHasParapet, parseChangeRoofHasParapet } from "./🏠️change-roof-has-parapet/🧬️schema/🟦️.ts";
import { type ChangeRoofMultiSpan, parseChangeRoofMultiSpan } from "./🏠️change-roof-multi-span/🧬️schema/🟦️.ts";
import { type ChangeRoofRoofType, parseChangeRoofRoofType } from "./🏠️change-roof-roof-type/🧬️schema/🟦️.ts";
import { type ChangeRoofSurfaceExtent, parseChangeRoofSurfaceExtent } from "./🏠️change-roof-surface-extent/🧬️schema/🟦️.ts";
import { type ChangeRoofSurfaceFace, parseChangeRoofSurfaceFace } from "./🏠️change-roof-surface-face/🧬️schema/🟦️.ts";
import { type ChangeRoofSurfacePitchDeg, parseChangeRoofSurfacePitchDeg } from "./🏠️change-roof-surface-pitch-deg/🧬️schema/🟦️.ts";
import { type ChangeRoofSurfaceShape, parseChangeRoofSurfaceShape } from "./🏠️change-roof-surface-shape/🧬️schema/🟦️.ts";
import { type InsertRoofSurface, parseInsertRoofSurface } from "./🏠️insert-roof-surface/🧬️schema/🟦️.ts";
import { type RemoveRoofSurface, parseRemoveRoofSurface } from "./🏠️remove-roof-surface/🧬️schema/🟦️.ts";
import { type ChangeAirDensity, parseChangeAirDensity } from "./🏢change-air-density/🧬️schema/🟦️.ts";
import { type ChangeFireOccupancy, parseChangeFireOccupancy } from "./🏢change-fire-occupancy/🧬️schema/🟦️.ts";
import { type ChangeFloorAssumedQk, parseChangeFloorAssumedQk } from "./🏢change-floor-assumed-qk/🧬️schema/🟦️.ts";
import { type ChangeFloorAssumedPartitions, parseChangeFloorAssumedPartitions } from "./🏢️change-floor-assumed-partitions/🧬️schema/🟦️.ts";
import { type ChangeFloorAssumedQkConc, parseChangeFloorAssumedQkConc } from "./🏢️change-floor-assumed-qk-conc/🧬️schema/🟦️.ts";
import { type ChangeFloorCategory, parseChangeFloorCategory } from "./🏢️change-floor-category/🧬️schema/🟦️.ts";
import { type ChangeSiloClaimed, parseChangeSiloClaimed } from "./🏭change-silo-claimed/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelDe, parseChangeCaseLabelDe } from "./🏷️change-case-label-de/🧬️schema/🟦️.ts";
import { type ChangeCaseLabelEn, parseChangeCaseLabelEn } from "./🏷️change-case-label-en/🧬️schema/🟦️.ts";
import { type ChangeSiloHeight, parseChangeSiloHeight } from "./🏷️change-silo-height/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeLm4, parseChangeAssumedBridgeLm4 } from "./👥change-assumed-bridge-lm4/🧬️schema/🟦️.ts";
import { type ChangeCraneClass, parseChangeCraneClass } from "./💥change-crane-class/🧬️schema/🟦️.ts";
import { type ChangeImpactActionAssumedForce, parseChangeImpactActionAssumedForce } from "./💥️change-impact-action-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeImpactActionHeight, parseChangeImpactActionHeight } from "./💥️change-impact-action-height/🧬️schema/🟦️.ts";
import { type ChangeImpactActionKind, parseChangeImpactActionKind } from "./💥️change-impact-action-kind/🧬️schema/🟦️.ts";
import { type ChangeImpactActionMass, parseChangeImpactActionMass } from "./💥️change-impact-action-mass/🧬️schema/🟦️.ts";
import { type ChangeImpactActionSpeed, parseChangeImpactActionSpeed } from "./💥️change-impact-action-speed/🧬️schema/🟦️.ts";
import { type ChangeImpactActionStiffness, parseChangeImpactActionStiffness } from "./💥️change-impact-action-stiffness/🧬️schema/🟦️.ts";
import { type ChangeImpactActionX, parseChangeImpactActionX } from "./💥️change-impact-action-x/🧬️schema/🟦️.ts";
import { type ChangeImpactActionY, parseChangeImpactActionY } from "./💥️change-impact-action-y/🧬️schema/🟦️.ts";
import { type InsertImpactAction, parseInsertImpactAction } from "./💥️insert-impact-action/🧬️schema/🟦️.ts";
import { type RemoveImpactAction, parseRemoveImpactAction } from "./💥️remove-impact-action/🧬️schema/🟦️.ts";
import { type ChangeDepth, parseChangeDepth } from "./💨change-depth/🧬️schema/🟦️.ts";
import { type ChangeCaseEntryId, parseChangeCaseEntryId } from "./📇️change-case-entry-id/🧬️schema/🟦️.ts";
import { type ChangeFireCurve, parseChangeFireCurve } from "./📉change-fire-curve/🧬️schema/🟦️.ts";
import { type ChangeBridgeTandemX, parseChangeBridgeTandemX } from "./📍change-bridge-tandem-x/🧬️schema/🟦️.ts";
import { type ChangePlacementX, parseChangePlacementX } from "./📍️change-placement-x/🧬️schema/🟦️.ts";
import { type ChangePlacementY, parseChangePlacementY } from "./📍️change-placement-y/🧬️schema/🟦️.ts";
import { type ChangePlacementYaw, parseChangePlacementYaw } from "./📍️change-placement-yaw/🧬️schema/🟦️.ts";
import { type ChangePlacementZ, parseChangePlacementZ } from "./📍️change-placement-z/🧬️schema/🟦️.ts";
import { type InsertPlacement, parseInsertPlacement } from "./📍️insert-placement/🧬️schema/🟦️.ts";
import { type RemovePlacement, parseRemovePlacement } from "./📍️remove-placement/🧬️schema/🟦️.ts";
import { type ChangeCraneRunwayLength, parseChangeCraneRunwayLength } from "./📏change-crane-runway-length/🧬️schema/🟦️.ts";
import { type ChangeLinearTemperatureGradient, parseChangeLinearTemperatureGradient } from "./📏change-linear-temperature-gradient/🧬️schema/🟦️.ts";
import { type ChangeMixedTerrainDistance, parseChangeMixedTerrainDistance } from "./📏change-mixed-terrain-distance/🧬️schema/🟦️.ts";
import { type ChangeRoofParapetHeight, parseChangeRoofParapetHeight } from "./📏change-roof-parapet-height/🧬️schema/🟦️.ts";
import { type ChangeLoadDispersalPavingThickness, parseChangeLoadDispersalPavingThickness } from "./📏️change-load-dispersal-paving-thickness/🧬️schema/🟦️.ts";
import { type ChangeLoadDispersalSlabThickness, parseChangeLoadDispersalSlabThickness } from "./📏️change-load-dispersal-slab-thickness/🧬️schema/🟦️.ts";
import { type ChangeLoadDispersalWheelLength, parseChangeLoadDispersalWheelLength } from "./📏️change-load-dispersal-wheel-length/🧬️schema/🟦️.ts";
import { type ChangeLoadDispersalWheelWidth, parseChangeLoadDispersalWheelWidth } from "./📏️change-load-dispersal-wheel-width/🧬️schema/🟦️.ts";
import { type InsertLoadDispersal, parseInsertLoadDispersal } from "./📏️insert-load-dispersal/🧬️schema/🟦️.ts";
import { type RemoveLoadDispersal, parseRemoveLoadDispersal } from "./📏️remove-load-dispersal/🧬️schema/🟦️.ts";
import { type ChangeFireCompartmentHeight, parseChangeFireCompartmentHeight } from "./📐change-fire-compartment-height/🧬️schema/🟦️.ts";
import { type ChangeOrographyFactor, parseChangeOrographyFactor } from "./📐change-orography-factor/🧬️schema/🟦️.ts";
import { type ChangeRoofPitchDeg, parseChangeRoofPitchDeg } from "./📐change-roof-pitch-deg/🧬️schema/🟦️.ts";
import { type ChangeImperfectionAssumedForce, parseChangeImperfectionAssumedForce } from "./📐️change-imperfection-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeImperfectionHeight, parseChangeImperfectionHeight } from "./📐️change-imperfection-height/🧬️schema/🟦️.ts";
import { type ChangeImperfectionTolerance, parseChangeImperfectionTolerance } from "./📐️change-imperfection-tolerance/🧬️schema/🟦️.ts";
import { type ChangeImperfectionVerticalLoad, parseChangeImperfectionVerticalLoad } from "./📐️change-imperfection-vertical-load/🧬️schema/🟦️.ts";
import { type ChangeImperfectionX, parseChangeImperfectionX } from "./📐️change-imperfection-x/🧬️schema/🟦️.ts";
import { type ChangeImperfectionY, parseChangeImperfectionY } from "./📐️change-imperfection-y/🧬️schema/🟦️.ts";
import { type InsertImperfection, parseInsertImperfection } from "./📐️insert-imperfection/🧬️schema/🟦️.ts";
import { type RemoveImperfection, parseRemoveImperfection } from "./📐️remove-imperfection/🧬️schema/🟦️.ts";
import { type ChangeAssumedSiloPatch, parseChangeAssumedSiloPatch } from "./📦change-assumed-silo-patch/🧬️schema/🟦️.ts";
import { type ChangeBridgeLoadGroup, parseChangeBridgeLoadGroup } from "./📦change-bridge-load-group/🧬️schema/🟦️.ts";
import { type ChangeAssumedHNet, parseChangeAssumedHNet } from "./🔆change-assumed-h-net/🧬️schema/🟦️.ts";
import { type ChangeAssumedQfD, parseChangeAssumedQfD } from "./🔋change-assumed-qf-d/🧬️schema/🟦️.ts";
import { type ChangeSiloMu, parseChangeSiloMu } from "./🔎change-silo-mu/🧬️schema/🟦️.ts";
import { type ChangeTieRunDeadLoad, parseChangeTieRunDeadLoad } from "./🔗️change-tie-run-dead-load/🧬️schema/🟦️.ts";
import { type ChangeTieRunImposedLoad, parseChangeTieRunImposedLoad } from "./🔗️change-tie-run-imposed-load/🧬️schema/🟦️.ts";
import { type ChangeTieRunKind, parseChangeTieRunKind } from "./🔗️change-tie-run-kind/🧬️schema/🟦️.ts";
import { type ChangeTieRunProvidedForce, parseChangeTieRunProvidedForce } from "./🔗️change-tie-run-provided-force/🧬️schema/🟦️.ts";
import { type ChangeTieRunPsi, parseChangeTieRunPsi } from "./🔗️change-tie-run-psi/🧬️schema/🟦️.ts";
import { type ChangeTieRunSpacing, parseChangeTieRunSpacing } from "./🔗️change-tie-run-spacing/🧬️schema/🟦️.ts";
import { type ChangeTieRunSpan, parseChangeTieRunSpan } from "./🔗️change-tie-run-span/🧬️schema/🟦️.ts";
import { type ChangeTieRunX, parseChangeTieRunX } from "./🔗️change-tie-run-x/🧬️schema/🟦️.ts";
import { type ChangeTieRunY, parseChangeTieRunY } from "./🔗️change-tie-run-y/🧬️schema/🟦️.ts";
import { type InsertTieRun, parseInsertTieRun } from "./🔗️insert-tie-run/🧬️schema/🟦️.ts";
import { type RemoveTieRun, parseRemoveTieRun } from "./🔗️remove-tie-run/🧬️schema/🟦️.ts";
import { type ChangeCaseNumber, parseChangeCaseNumber } from "./🔢️change-case-number/🧬️schema/🟦️.ts";
import { type ChangeCaseText, parseChangeCaseText } from "./🔤️change-case-text/🧬️schema/🟦️.ts";
import { type ChangeConstructionActivity, parseChangeConstructionActivity } from "./🔥change-construction-activity/🧬️schema/🟦️.ts";
import { type ChangeFireMode, parseChangeFireMode } from "./🔥change-fire-mode/🧬️schema/🟦️.ts";
import { type ChangeWindFaceLoadedArea, parseChangeWindFaceLoadedArea } from "./🔲change-wind-face-loaded-area/🧬️schema/🟦️.ts";
import { type ChangeSiloHopperAngleDeg, parseChangeSiloHopperAngleDeg } from "./🔻change-silo-hopper-angle-deg/🧬️schema/🟦️.ts";
import { type ChangeInitialTemperature, parseChangeInitialTemperature } from "./🕰️change-initial-temperature/🧬️schema/🟦️.ts";
import { type ChangeFireCompartmentArea, parseChangeFireCompartmentArea } from "./🗺️change-fire-compartment-area/🧬️schema/🟦️.ts";
import { type ChangeFloorArea, parseChangeFloorArea } from "./🗺️change-floor-area/🧬️schema/🟦️.ts";
import { type ChangeSnowZone, parseChangeSnowZone } from "./🗺️change-snow-zone/🧬️schema/🟦️.ts";
import { type ChangeHelicopterAssumedForce, parseChangeHelicopterAssumedForce } from "./🚁️change-helicopter-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeHelicopterHeight, parseChangeHelicopterHeight } from "./🚁️change-helicopter-height/🧬️schema/🟦️.ts";
import { type ChangeHelicopterMass, parseChangeHelicopterMass } from "./🚁️change-helicopter-mass/🧬️schema/🟦️.ts";
import { type ChangeHelicopterX, parseChangeHelicopterX } from "./🚁️change-helicopter-x/🧬️schema/🟦️.ts";
import { type ChangeHelicopterY, parseChangeHelicopterY } from "./🚁️change-helicopter-y/🧬️schema/🟦️.ts";
import { type InsertHelicopter, parseInsertHelicopter } from "./🚁️insert-helicopter/🧬️schema/🟦️.ts";
import { type RemoveHelicopter, parseRemoveHelicopter } from "./🚁️remove-helicopter/🧬️schema/🟦️.ts";
import { type ChangeAssumedRailLm71Udl, parseChangeAssumedRailLm71Udl } from "./🚆change-assumed-rail-lm71-udl/🧬️schema/🟦️.ts";
import { type ChangeAssumedRailLm71, parseChangeAssumedRailLm71 } from "./🚆change-assumed-rail-lm71/🧬️schema/🟦️.ts";
import { type ChangeAssumedRailSw0, parseChangeAssumedRailSw0 } from "./🚆change-assumed-rail-sw0/🧬️schema/🟦️.ts";
import { type ChangeAssumedRailSw2, parseChangeAssumedRailSw2 } from "./🚆change-assumed-rail-sw2/🧬️schema/🟦️.ts";
import { type ChangeRailAlpha, parseChangeRailAlpha } from "./🚆change-rail-alpha/🧬️schema/🟦️.ts";
import { type ChangeDerailmentAssumedForce, parseChangeDerailmentAssumedForce } from "./🚆️change-derailment-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeDerailmentDesignForceX, parseChangeDerailmentDesignForceX } from "./🚆️change-derailment-design-force-x/🧬️schema/🟦️.ts";
import { type ChangeDerailmentDesignForceY, parseChangeDerailmentDesignForceY } from "./🚆️change-derailment-design-force-y/🧬️schema/🟦️.ts";
import { type ChangeDerailmentDistance, parseChangeDerailmentDistance } from "./🚆️change-derailment-distance/🧬️schema/🟦️.ts";
import { type ChangeDerailmentX, parseChangeDerailmentX } from "./🚆️change-derailment-x/🧬️schema/🟦️.ts";
import { type ChangeDerailmentY, parseChangeDerailmentY } from "./🚆️change-derailment-y/🧬️schema/🟦️.ts";
import { type InsertDerailment, parseInsertDerailment } from "./🚆️insert-derailment/🧬️schema/🟦️.ts";
import { type RemoveDerailment, parseRemoveDerailment } from "./🚆️remove-derailment/🧬️schema/🟦️.ts";
import { type ChangeAccidentalAssumedForce, parseChangeAccidentalAssumedForce } from "./🚗change-accidental-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeAccidentalAssumedPressure, parseChangeAccidentalAssumedPressure } from "./🚗️change-accidental-assumed-pressure/🧬️schema/🟦️.ts";
import { type ChangeAccidentalExplosionMass, parseChangeAccidentalExplosionMass } from "./🚗️change-accidental-explosion-mass/🧬️schema/🟦️.ts";
import { type ChangeAccidentalStandoff, parseChangeAccidentalStandoff } from "./🚗️change-accidental-standoff/🧬️schema/🟦️.ts";
import { type ChangeAccidentalVehicleMass, parseChangeAccidentalVehicleMass } from "./🚗️change-accidental-vehicle-mass/🧬️schema/🟦️.ts";
import { type ChangeAccidentalVehicleSpeed, parseChangeAccidentalVehicleSpeed } from "./🚗️change-accidental-vehicle-speed/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeLm2, parseChangeAssumedBridgeLm2 } from "./🚛change-assumed-bridge-lm2/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeLm3, parseChangeAssumedBridgeLm3 } from "./🚛change-assumed-bridge-lm3/🧬️schema/🟦️.ts";
import { type ChangeSpecialVehicleDesignation, parseChangeSpecialVehicleDesignation } from "./🚛️change-special-vehicle-designation/🧬️schema/🟦️.ts";
import { type ChangeSpecialVehicleX, parseChangeSpecialVehicleX } from "./🚛️change-special-vehicle-x/🧬️schema/🟦️.ts";
import { type ChangeSpecialVehicleY, parseChangeSpecialVehicleY } from "./🚛️change-special-vehicle-y/🧬️schema/🟦️.ts";
import { type InsertSpecialVehicle, parseInsertSpecialVehicle } from "./🚛️insert-special-vehicle/🧬️schema/🟦️.ts";
import { type RemoveSpecialVehicle, parseRemoveSpecialVehicle } from "./🚛️remove-special-vehicle/🧬️schema/🟦️.ts";
import { type ChangeSelfWeightAssumedGk, parseChangeSelfWeightAssumedGk } from "./🚧change-self-weight-assumed-gk/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeFootway, parseChangeAssumedBridgeFootway } from "./🚶change-assumed-bridge-footway/🧬️schema/🟦️.ts";
import { type ChangeFootbridgeDeckWidth, parseChangeFootbridgeDeckWidth } from "./🚶️change-footbridge-deck-width/🧬️schema/🟦️.ts";
import { type ChangeFootbridgeLoadedLength, parseChangeFootbridgeLoadedLength } from "./🚶️change-footbridge-loaded-length/🧬️schema/🟦️.ts";
import { type ChangeFootbridgeNaturalFrequency, parseChangeFootbridgeNaturalFrequency } from "./🚶️change-footbridge-natural-frequency/🧬️schema/🟦️.ts";
import { type InsertFootbridge, parseInsertFootbridge } from "./🚶️insert-footbridge/🧬️schema/🟦️.ts";
import { type RemoveFootbridge, parseRemoveFootbridge } from "./🚶️remove-footbridge/🧬️schema/🟦️.ts";
import { type ChangeCraneWheelBase, parseChangeCraneWheelBase } from "./🛞change-crane-wheel-base/🧬️schema/🟦️.ts";
import { type ChangeWindFaceAssumedWp, parseChangeWindFaceAssumedWp } from "./🛡️change-wind-face-assumed-wp/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeUdl, parseChangeAssumedBridgeUdl } from "./🛣️change-assumed-bridge-udl/🧬️schema/🟦️.ts";
import { type ChangeLaneLm1Lane, parseChangeLaneLm1Lane } from "./🛣️change-lane-lm1-lane/🧬️schema/🟦️.ts";
import { type ChangeLaneOffset, parseChangeLaneOffset } from "./🛣️change-lane-offset/🧬️schema/🟦️.ts";
import { type ChangeLaneWidth, parseChangeLaneWidth } from "./🛣️change-lane-width/🧬️schema/🟦️.ts";
import { type InsertBridgeLane, parseInsertBridgeLane } from "./🛣️insert-bridge-lane/🧬️schema/🟦️.ts";
import { type RemoveBridgeLane, parseRemoveBridgeLane } from "./🛣️remove-bridge-lane/🧬️schema/🟦️.ts";
import { type ChangeRailDynamicDeterminantLength, parseChangeRailDynamicDeterminantLength } from "./🛤️change-rail-dynamic-determinant-length/🧬️schema/🟦️.ts";
import { type ChangeRailDynamicMaintenance, parseChangeRailDynamicMaintenance } from "./🛤️change-rail-dynamic-maintenance/🧬️schema/🟦️.ts";
import { type InsertRailDynamic, parseInsertRailDynamic } from "./🛤️insert-rail-dynamic/🧬️schema/🟦️.ts";
import { type RemoveRailDynamic, parseRemoveRailDynamic } from "./🛤️remove-rail-dynamic/🧬️schema/🟦️.ts";
import { type ChangeSelfWeightThickness, parseChangeSelfWeightThickness } from "./🥞change-self-weight-thickness/🧬️schema/🟦️.ts";
import { type ChangeTMin, parseChangeTMin } from "./🧊change-t-min/🧬️schema/🟦️.ts";
import { type ChangeExplosionAssumedPressure, parseChangeExplosionAssumedPressure } from "./🧨️change-explosion-assumed-pressure/🧬️schema/🟦️.ts";
import { type ChangeExplosionVentArea, parseChangeExplosionVentArea } from "./🧨️change-explosion-vent-area/🧬️schema/🟦️.ts";
import { type ChangeExplosionVentPressure, parseChangeExplosionVentPressure } from "./🧨️change-explosion-vent-pressure/🧬️schema/🟦️.ts";
import { type ChangeInternalExplosionVolume, parseChangeInternalExplosionVolume } from "./🧨️change-internal-explosion-volume/🧬️schema/🟦️.ts";
import { type InsertInternalExplosion, parseInsertInternalExplosion } from "./🧨️insert-internal-explosion/🧬️schema/🟦️.ts";
import { type RemoveInternalExplosion, parseRemoveInternalExplosion } from "./🧨️remove-internal-explosion/🧬️schema/🟦️.ts";
import { type ChangeMixedTerrainUpwind, parseChangeMixedTerrainUpwind } from "./🧭change-mixed-terrain-upwind/🧬️schema/🟦️.ts";
import { type ChangeFireLoadBasis, parseChangeFireLoadBasis } from "./🧯change-fire-load-basis/🧬️schema/🟦️.ts";
import { type ChangeAssumedSiloWallFriction, parseChangeAssumedSiloWallFriction } from "./🧱change-assumed-silo-wall-friction/🧬️schema/🟦️.ts";
import { type ChangeFireThermalInertia, parseChangeFireThermalInertia } from "./🧱change-fire-thermal-inertia/🧬️schema/🟦️.ts";
import { type ChangeHeight, parseChangeHeight } from "./🧱change-height/🧬️schema/🟦️.ts";
import { type ChangeRoofDriftObstructionHeight, parseChangeRoofDriftObstructionHeight } from "./🧱change-roof-drift-obstruction-height/🧬️schema/🟦️.ts";
import { type ChangeSelfWeightMaterial, parseChangeSelfWeightMaterial } from "./🧱️change-self-weight-material/🧬️schema/🟦️.ts";
import { type ChangeWindZone, parseChangeWindZone } from "./🪁change-wind-zone/🧬️schema/🟦️.ts";
import { type ChangeFireOpeningFactor, parseChangeFireOpeningFactor } from "./🪟change-fire-opening-factor/🧬️schema/🟦️.ts";

export type En1991Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeSnowZone: ChangeSnowZone }
  | { ChangeAltitude: ChangeAltitude }
  | { ChangeEnSk: ChangeEnSk }
  | { ChangeNorthGermanLowlandSnow: ChangeNorthGermanLowlandSnow }
  | { ChangeWindZone: ChangeWindZone }
  | { ChangeEnVb: ChangeEnVb }
  | { ChangeTerrainCategory: ChangeTerrainCategory }
  | { ChangeMixedTerrainUpwind: ChangeMixedTerrainUpwind }
  | { ChangeMixedTerrainDistance: ChangeMixedTerrainDistance }
  | { ChangeOrographyFactor: ChangeOrographyFactor }
  | { ChangeCoastOrIsland: ChangeCoastOrIsland }
  | { ChangeAirDensity: ChangeAirDensity }
  | { ChangeHeight: ChangeHeight }
  | { ChangeWidth: ChangeWidth }
  | { ChangeDepth: ChangeDepth }
  | { ChangeAssumedDeltaT: ChangeAssumedDeltaT }
  | { ChangeConstructionActivity: ChangeConstructionActivity }
  | { ChangeAssumedConstructionQk: ChangeAssumedConstructionQk }
  | { ChangeStructureKind: ChangeStructureKind }
  | { ChangeBridgeLane: ChangeBridgeLane }
  | { ChangeBridgeSpan: ChangeBridgeSpan }
  | { ChangeBridgeTandemX: ChangeBridgeTandemX }
  | { ChangeBridgeCurveRadius: ChangeBridgeCurveRadius }
  | { ChangeRailAlpha: ChangeRailAlpha }
  | { ChangeAssumedRailLm71: ChangeAssumedRailLm71 }
  | { ChangeAssumedRailLm71Udl: ChangeAssumedRailLm71Udl }
  | { ChangeAssumedRailSw0: ChangeAssumedRailSw0 }
  | { ChangeAssumedRailSw2: ChangeAssumedRailSw2 }
  | { ChangeCraneX: ChangeCraneX }
  | { ChangeCraneRunwayLength: ChangeCraneRunwayLength }
  | { ChangeCraneWheelBase: ChangeCraneWheelBase }
  | { ChangeSiloDiameter: ChangeSiloDiameter }
  | { ChangeSiloEccentricity: ChangeSiloEccentricity }
  | { ChangeSiloHopperAngleDeg: ChangeSiloHopperAngleDeg }
  | { ChangeBridgeLaneWidth: ChangeBridgeLaneWidth }
  | { ChangeAssumedBridgeTandem: ChangeAssumedBridgeTandem }
  | { ChangeAssumedBridgeUdl: ChangeAssumedBridgeUdl }
  | { ChangeAssumedBridgeLm2: ChangeAssumedBridgeLm2 }
  | { ChangeAssumedBridgeFootway: ChangeAssumedBridgeFootway }
  | { ChangeStoreyCount: ChangeStoreyCount }
  | { ChangeTMax: ChangeTMax }
  | { ChangeTMin: ChangeTMin }
  | { ChangeInitialTemperature: ChangeInitialTemperature }
  | { ChangeThermalElementType: ChangeThermalElementType }
  | { ChangeThermalBridgeType: ChangeThermalBridgeType }
  | { ChangeLinearTemperatureGradient: ChangeLinearTemperatureGradient }
  | { ChangeFireMode: ChangeFireMode }
  | { ChangeFireLoadBasis: ChangeFireLoadBasis }
  | { ChangeFireCurve: ChangeFireCurve }
  | { ChangeFireDuration: ChangeFireDuration }
  | { ChangeAssumedGasTemperature: ChangeAssumedGasTemperature }
  | { ChangeAssumedHNet: ChangeAssumedHNet }
  | { ChangeFireCompartmentArea: ChangeFireCompartmentArea }
  | { ChangeFireCompartmentHeight: ChangeFireCompartmentHeight }
  | { ChangeFireOpeningFactor: ChangeFireOpeningFactor }
  | { ChangeFireThermalInertia: ChangeFireThermalInertia }
  | { ChangeFireOccupancy: ChangeFireOccupancy }
  | { ChangeFireLoadDensityQf: ChangeFireLoadDensityQf }
  | { ChangeAssumedQfD: ChangeAssumedQfD }
  | { ChangeAssumedBridgeLm3: ChangeAssumedBridgeLm3 }
  | { ChangeAssumedBridgeLm4: ChangeAssumedBridgeLm4 }
  | { ChangeBridgeLoadGroup: ChangeBridgeLoadGroup }
  | { ChangeCraneClaimed: ChangeCraneClaimed }
  | { ChangeCraneClass: ChangeCraneClass }
  | { ChangeHoistClass: ChangeHoistClass }
  | { ChangeHoistingSpeed: ChangeHoistingSpeed }
  | { ChangeAssumedCraneWheel: ChangeAssumedCraneWheel }
  | { ChangeAssumedCraneHorizontal: ChangeAssumedCraneHorizontal }
  | { ChangeSiloClaimed: ChangeSiloClaimed }
  | { ChangeSiloKind: ChangeSiloKind }
  | { ChangeSiloBulkDensity: ChangeSiloBulkDensity }
  | { ChangeSiloHeight: ChangeSiloHeight }
  | { ChangeSiloHydraulicRadius: ChangeSiloHydraulicRadius }
  | { ChangeSiloMu: ChangeSiloMu }
  | { ChangeSiloK: ChangeSiloK }
  | { ChangeAssumedSiloPressure: ChangeAssumedSiloPressure }
  | { ChangeAssumedSiloPatch: ChangeAssumedSiloPatch }
  | { ChangeAssumedSiloWallFriction: ChangeAssumedSiloWallFriction }
  | { ChangeFloorAssumedQk: ChangeFloorAssumedQk }
  | { ChangeSelfWeightAssumedGk: ChangeSelfWeightAssumedGk }
  | { ChangeRoofAssumedSk: ChangeRoofAssumedSk }
  | { ChangeWindFaceAssumedWp: ChangeWindFaceAssumedWp }
  | { ChangeAccidentalAssumedForce: ChangeAccidentalAssumedForce }
  | { InsertFloor: InsertFloor }
  | { RemoveFloor: RemoveFloor }
  | { InsertSelfWeightElement: InsertSelfWeightElement }
  | { RemoveSelfWeightElement: RemoveSelfWeightElement }
  | { InsertRoof: InsertRoof }
  | { RemoveRoof: RemoveRoof }
  | { InsertWindFace: InsertWindFace }
  | { RemoveWindFace: RemoveWindFace }
  | { InsertAccidentalCase: InsertAccidentalCase }
  | { RemoveAccidentalCase: RemoveAccidentalCase }
  | { ChangeRoofPitchDeg: ChangeRoofPitchDeg }
  | { ChangeRoofParapetHeight: ChangeRoofParapetHeight }
  | { ChangeRoofDriftObstructionHeight: ChangeRoofDriftObstructionHeight }
  | { ChangeFloorArea: ChangeFloorArea }
  | { ChangeSelfWeightThickness: ChangeSelfWeightThickness }
  | { ChangeWindFaceZ: ChangeWindFaceZ }
  | { ChangeWindFaceLoadedArea: ChangeWindFaceLoadedArea }
  | { ChangeLaneWidth: ChangeLaneWidth }
  | { ChangeLaneOffset: ChangeLaneOffset }
  | { ChangeLaneLm1Lane: ChangeLaneLm1Lane }
  | { InsertBridgeLane: InsertBridgeLane }
  | { RemoveBridgeLane: RemoveBridgeLane }
  | { ChangeStageActivity: ChangeStageActivity }
  | { ChangeStageDurationDays: ChangeStageDurationDays }
  | { ChangeStageReturnYears: ChangeStageReturnYears }
  | { ChangeStageAssumedQk: ChangeStageAssumedQk }
  | { InsertConstructionStage: InsertConstructionStage }
  | { RemoveConstructionStage: RemoveConstructionStage }
  | { ChangeRoofSurfaceFace: ChangeRoofSurfaceFace }
  | { ChangeRoofSurfaceShape: ChangeRoofSurfaceShape }
  | { ChangeRoofSurfacePitchDeg: ChangeRoofSurfacePitchDeg }
  | { ChangeRoofSurfaceExtent: ChangeRoofSurfaceExtent }
  | { InsertRoofSurface: InsertRoofSurface }
  | { RemoveRoofSurface: RemoveRoofSurface }
  | { ChangePlacementX: ChangePlacementX }
  | { ChangePlacementY: ChangePlacementY }
  | { ChangePlacementZ: ChangePlacementZ }
  | { ChangePlacementYaw: ChangePlacementYaw }
  | { InsertPlacement: InsertPlacement }
  | { RemovePlacement: RemovePlacement }
  | { InsertCase: InsertCase }
  | { RemoveCase: RemoveCase }
  | { ChangeCaseNumber: ChangeCaseNumber }
  | { ChangeCaseText: ChangeCaseText }
  | { ChangeCaseLabelEn: ChangeCaseLabelEn }
  | { ChangeCaseLabelDe: ChangeCaseLabelDe }
  | { ChangeCaseEntryId: ChangeCaseEntryId }
  | { ChangeFloorCategory: ChangeFloorCategory }
  | { ChangeFloorAssumedQkConc: ChangeFloorAssumedQkConc }
  | { ChangeFloorAssumedPartitions: ChangeFloorAssumedPartitions }
  | { ChangeSelfWeightMaterial: ChangeSelfWeightMaterial }
  | { ChangeRoofRoofType: ChangeRoofRoofType }
  | { ChangeRoofCE: ChangeRoofCE }
  | { ChangeRoofCT: ChangeRoofCT }
  | { ChangeRoofHasParapet: ChangeRoofHasParapet }
  | { ChangeRoofMultiSpan: ChangeRoofMultiSpan }
  | { ChangeWindFaceZone: ChangeWindFaceZone }
  | { ChangeWindFaceCPe10: ChangeWindFaceCPe10 }
  | { ChangeWindFaceCPe1: ChangeWindFaceCPe1 }
  | { ChangeWindFaceCPi: ChangeWindFaceCPi }
  | { ChangeWindFaceCS: ChangeWindFaceCS }
  | { ChangeWindFaceCD: ChangeWindFaceCD }
  | { ChangeSpecialVehicleDesignation: ChangeSpecialVehicleDesignation }
  | { ChangeSpecialVehicleX: ChangeSpecialVehicleX }
  | { ChangeSpecialVehicleY: ChangeSpecialVehicleY }
  | { InsertSpecialVehicle: InsertSpecialVehicle }
  | { RemoveSpecialVehicle: RemoveSpecialVehicle }
  | { ChangeWindBridgeDeckWidth: ChangeWindBridgeDeckWidth }
  | { ChangeWindBridgeDeckDepth: ChangeWindBridgeDeckDepth }
  | { ChangeWindBridgeParapetKind: ChangeWindBridgeParapetKind }
  | { ChangeWindBridgeParapetHeight: ChangeWindBridgeParapetHeight }
  | { ChangeWindBridgeTrafficKind: ChangeWindBridgeTrafficKind }
  | { ChangeWindBridgeGroundHeight: ChangeWindBridgeGroundHeight }
  | { ChangeWindBridgeSpan: ChangeWindBridgeSpan }
  | { ChangeWindBridgeAssumedForce: ChangeWindBridgeAssumedForce }
  | { InsertWindBridge: InsertWindBridge }
  | { RemoveWindBridge: RemoveWindBridge }
  | { ChangeDynamicElementHeight: ChangeDynamicElementHeight }
  | { ChangeDynamicElementWidth: ChangeDynamicElementWidth }
  | { ChangeDynamicFirstFrequency: ChangeDynamicFirstFrequency }
  | { ChangeDynamicLogDecrement: ChangeDynamicLogDecrement }
  | { ChangeDynamicMassPerLength: ChangeDynamicMassPerLength }
  | { ChangeDynamicElementStrouhal: ChangeDynamicElementStrouhal }
  | { ChangeDynamicAssumedCscd: ChangeDynamicAssumedCscd }
  | { InsertDynamicElement: InsertDynamicElement }
  | { RemoveDynamicElement: RemoveDynamicElement }
  | { ChangeImpactActionKind: ChangeImpactActionKind }
  | { ChangeImpactActionMass: ChangeImpactActionMass }
  | { ChangeImpactActionSpeed: ChangeImpactActionSpeed }
  | { ChangeImpactActionStiffness: ChangeImpactActionStiffness }
  | { ChangeImpactActionHeight: ChangeImpactActionHeight }
  | { ChangeImpactActionX: ChangeImpactActionX }
  | { ChangeImpactActionY: ChangeImpactActionY }
  | { ChangeImpactActionAssumedForce: ChangeImpactActionAssumedForce }
  | { InsertImpactAction: InsertImpactAction }
  | { RemoveImpactAction: RemoveImpactAction }
  | { ChangeInternalExplosionVolume: ChangeInternalExplosionVolume }
  | { ChangeExplosionVentArea: ChangeExplosionVentArea }
  | { ChangeExplosionVentPressure: ChangeExplosionVentPressure }
  | { ChangeExplosionAssumedPressure: ChangeExplosionAssumedPressure }
  | { InsertInternalExplosion: InsertInternalExplosion }
  | { RemoveInternalExplosion: RemoveInternalExplosion }
  | { ChangeThermalSectionDeckType: ChangeThermalSectionDeckType }
  | { ChangeThermalSectionSurfacing: ChangeThermalSectionSurfacing }
  | { ChangeThermalSectionLength: ChangeThermalSectionLength }
  | { ChangeThermalSectionDepth: ChangeThermalSectionDepth }
  | { ChangeThermalSectionAlphaMicro: ChangeThermalSectionAlphaMicro }
  | { ChangeThermalAssumedRange: ChangeThermalAssumedRange }
  | { InsertThermalSection: InsertThermalSection }
  | { RemoveThermalSection: RemoveThermalSection }
  | { ChangeAccidentalVehicleMass: ChangeAccidentalVehicleMass }
  | { ChangeAccidentalVehicleSpeed: ChangeAccidentalVehicleSpeed }
  | { ChangeAccidentalExplosionMass: ChangeAccidentalExplosionMass }
  | { ChangeAccidentalStandoff: ChangeAccidentalStandoff }
  | { ChangeAccidentalAssumedPressure: ChangeAccidentalAssumedPressure }
  | { ChangeHelicopterMass: ChangeHelicopterMass }
  | { ChangeHelicopterX: ChangeHelicopterX }
  | { ChangeHelicopterY: ChangeHelicopterY }
  | { ChangeHelicopterHeight: ChangeHelicopterHeight }
  | { ChangeHelicopterAssumedForce: ChangeHelicopterAssumedForce }
  | { InsertHelicopter: InsertHelicopter }
  | { RemoveHelicopter: RemoveHelicopter }
  | { ChangeTieRunKind: ChangeTieRunKind }
  | { ChangeTieRunSpacing: ChangeTieRunSpacing }
  | { ChangeTieRunSpan: ChangeTieRunSpan }
  | { ChangeTieRunDeadLoad: ChangeTieRunDeadLoad }
  | { ChangeTieRunImposedLoad: ChangeTieRunImposedLoad }
  | { ChangeTieRunPsi: ChangeTieRunPsi }
  | { ChangeTieRunX: ChangeTieRunX }
  | { ChangeTieRunY: ChangeTieRunY }
  | { ChangeTieRunProvidedForce: ChangeTieRunProvidedForce }
  | { InsertTieRun: InsertTieRun }
  | { RemoveTieRun: RemoveTieRun }
  | { ChangeConstructionLoadStage: ChangeConstructionLoadStage }
  | { ChangeConstructionLoadKind: ChangeConstructionLoadKind }
  | { ChangeConstructionLoadMagnitude: ChangeConstructionLoadMagnitude }
  | { InsertConstructionLoad: InsertConstructionLoad }
  | { RemoveConstructionLoad: RemoveConstructionLoad }
  | { ChangeFallingMass: ChangeFallingMass }
  | { ChangeFallingDropHeight: ChangeFallingDropHeight }
  | { ChangeFallingStoppingDistance: ChangeFallingStoppingDistance }
  | { ChangeFallingX: ChangeFallingX }
  | { ChangeFallingY: ChangeFallingY }
  | { ChangeFallingAssumedForce: ChangeFallingAssumedForce }
  | { InsertFalling: InsertFalling }
  | { RemoveFalling: RemoveFalling }
  | { ChangeImperfectionHeight: ChangeImperfectionHeight }
  | { ChangeImperfectionTolerance: ChangeImperfectionTolerance }
  | { ChangeImperfectionVerticalLoad: ChangeImperfectionVerticalLoad }
  | { ChangeImperfectionX: ChangeImperfectionX }
  | { ChangeImperfectionY: ChangeImperfectionY }
  | { ChangeImperfectionAssumedForce: ChangeImperfectionAssumedForce }
  | { InsertImperfection: InsertImperfection }
  | { RemoveImperfection: RemoveImperfection }
  | { ChangeDerailmentDistance: ChangeDerailmentDistance }
  | { ChangeDerailmentDesignForceX: ChangeDerailmentDesignForceX }
  | { ChangeDerailmentDesignForceY: ChangeDerailmentDesignForceY }
  | { ChangeDerailmentX: ChangeDerailmentX }
  | { ChangeDerailmentY: ChangeDerailmentY }
  | { ChangeDerailmentAssumedForce: ChangeDerailmentAssumedForce }
  | { InsertDerailment: InsertDerailment }
  | { RemoveDerailment: RemoveDerailment }
  | { ChangeCraneHoistRatedLoad: ChangeCraneHoistRatedLoad }
  | { ChangeCraneHoistReleaseFraction: ChangeCraneHoistReleaseFraction }
  | { ChangeCraneHoistReleaseDevice: ChangeCraneHoistReleaseDevice }
  | { InsertCraneHoist: InsertCraneHoist }
  | { RemoveCraneHoist: RemoveCraneHoist }
  | { ChangeFootbridgeLoadedLength: ChangeFootbridgeLoadedLength }
  | { ChangeFootbridgeDeckWidth: ChangeFootbridgeDeckWidth }
  | { ChangeFootbridgeNaturalFrequency: ChangeFootbridgeNaturalFrequency }
  | { InsertFootbridge: InsertFootbridge }
  | { RemoveFootbridge: RemoveFootbridge }
  | { ChangeRailDynamicDeterminantLength: ChangeRailDynamicDeterminantLength }
  | { ChangeRailDynamicMaintenance: ChangeRailDynamicMaintenance }
  | { InsertRailDynamic: InsertRailDynamic }
  | { RemoveRailDynamic: RemoveRailDynamic }
  | { ChangeLoadDispersalWheelLength: ChangeLoadDispersalWheelLength }
  | { ChangeLoadDispersalWheelWidth: ChangeLoadDispersalWheelWidth }
  | { ChangeLoadDispersalPavingThickness: ChangeLoadDispersalPavingThickness }
  | { ChangeLoadDispersalSlabThickness: ChangeLoadDispersalSlabThickness }
  | { InsertLoadDispersal: InsertLoadDispersal }
  | { RemoveLoadDispersal: RemoveLoadDispersal }
  | { ChangeBulkSolidInternalFriction: ChangeBulkSolidInternalFriction }
  | { InsertBulkSolid: InsertBulkSolid }
  | { RemoveBulkSolid: RemoveBulkSolid }
  | { ChangeRotatingMachineMass: ChangeRotatingMachineMass }
  | { ChangeRotatingMachineEccentricity: ChangeRotatingMachineEccentricity }
  | { ChangeRotatingMachineFrequency: ChangeRotatingMachineFrequency }
  | { InsertRotatingMachine: InsertRotatingMachine }
  | { RemoveRotatingMachine: RemoveRotatingMachine };

export const parseEn1991Mutation: NormWireReader<En1991Mutation> = normWireExternal<En1991Mutation>({
  ChangeAnnex: parseChangeAnnex,
  ChangeSnowZone: parseChangeSnowZone,
  ChangeAltitude: parseChangeAltitude,
  ChangeEnSk: parseChangeEnSk,
  ChangeNorthGermanLowlandSnow: parseChangeNorthGermanLowlandSnow,
  ChangeWindZone: parseChangeWindZone,
  ChangeEnVb: parseChangeEnVb,
  ChangeTerrainCategory: parseChangeTerrainCategory,
  ChangeMixedTerrainUpwind: parseChangeMixedTerrainUpwind,
  ChangeMixedTerrainDistance: parseChangeMixedTerrainDistance,
  ChangeOrographyFactor: parseChangeOrographyFactor,
  ChangeCoastOrIsland: parseChangeCoastOrIsland,
  ChangeAirDensity: parseChangeAirDensity,
  ChangeHeight: parseChangeHeight,
  ChangeWidth: parseChangeWidth,
  ChangeDepth: parseChangeDepth,
  ChangeAssumedDeltaT: parseChangeAssumedDeltaT,
  ChangeConstructionActivity: parseChangeConstructionActivity,
  ChangeAssumedConstructionQk: parseChangeAssumedConstructionQk,
  ChangeStructureKind: parseChangeStructureKind,
  ChangeBridgeLane: parseChangeBridgeLane,
  ChangeBridgeSpan: parseChangeBridgeSpan,
  ChangeBridgeTandemX: parseChangeBridgeTandemX,
  ChangeBridgeCurveRadius: parseChangeBridgeCurveRadius,
  ChangeRailAlpha: parseChangeRailAlpha,
  ChangeAssumedRailLm71: parseChangeAssumedRailLm71,
  ChangeAssumedRailLm71Udl: parseChangeAssumedRailLm71Udl,
  ChangeAssumedRailSw0: parseChangeAssumedRailSw0,
  ChangeAssumedRailSw2: parseChangeAssumedRailSw2,
  ChangeCraneX: parseChangeCraneX,
  ChangeCraneRunwayLength: parseChangeCraneRunwayLength,
  ChangeCraneWheelBase: parseChangeCraneWheelBase,
  ChangeSiloDiameter: parseChangeSiloDiameter,
  ChangeSiloEccentricity: parseChangeSiloEccentricity,
  ChangeSiloHopperAngleDeg: parseChangeSiloHopperAngleDeg,
  ChangeBridgeLaneWidth: parseChangeBridgeLaneWidth,
  ChangeAssumedBridgeTandem: parseChangeAssumedBridgeTandem,
  ChangeAssumedBridgeUdl: parseChangeAssumedBridgeUdl,
  ChangeAssumedBridgeLm2: parseChangeAssumedBridgeLm2,
  ChangeAssumedBridgeFootway: parseChangeAssumedBridgeFootway,
  ChangeStoreyCount: parseChangeStoreyCount,
  ChangeTMax: parseChangeTMax,
  ChangeTMin: parseChangeTMin,
  ChangeInitialTemperature: parseChangeInitialTemperature,
  ChangeThermalElementType: parseChangeThermalElementType,
  ChangeThermalBridgeType: parseChangeThermalBridgeType,
  ChangeLinearTemperatureGradient: parseChangeLinearTemperatureGradient,
  ChangeFireMode: parseChangeFireMode,
  ChangeFireLoadBasis: parseChangeFireLoadBasis,
  ChangeFireCurve: parseChangeFireCurve,
  ChangeFireDuration: parseChangeFireDuration,
  ChangeAssumedGasTemperature: parseChangeAssumedGasTemperature,
  ChangeAssumedHNet: parseChangeAssumedHNet,
  ChangeFireCompartmentArea: parseChangeFireCompartmentArea,
  ChangeFireCompartmentHeight: parseChangeFireCompartmentHeight,
  ChangeFireOpeningFactor: parseChangeFireOpeningFactor,
  ChangeFireThermalInertia: parseChangeFireThermalInertia,
  ChangeFireOccupancy: parseChangeFireOccupancy,
  ChangeFireLoadDensityQf: parseChangeFireLoadDensityQf,
  ChangeAssumedQfD: parseChangeAssumedQfD,
  ChangeAssumedBridgeLm3: parseChangeAssumedBridgeLm3,
  ChangeAssumedBridgeLm4: parseChangeAssumedBridgeLm4,
  ChangeBridgeLoadGroup: parseChangeBridgeLoadGroup,
  ChangeCraneClaimed: parseChangeCraneClaimed,
  ChangeCraneClass: parseChangeCraneClass,
  ChangeHoistClass: parseChangeHoistClass,
  ChangeHoistingSpeed: parseChangeHoistingSpeed,
  ChangeAssumedCraneWheel: parseChangeAssumedCraneWheel,
  ChangeAssumedCraneHorizontal: parseChangeAssumedCraneHorizontal,
  ChangeSiloClaimed: parseChangeSiloClaimed,
  ChangeSiloKind: parseChangeSiloKind,
  ChangeSiloBulkDensity: parseChangeSiloBulkDensity,
  ChangeSiloHeight: parseChangeSiloHeight,
  ChangeSiloHydraulicRadius: parseChangeSiloHydraulicRadius,
  ChangeSiloMu: parseChangeSiloMu,
  ChangeSiloK: parseChangeSiloK,
  ChangeAssumedSiloPressure: parseChangeAssumedSiloPressure,
  ChangeAssumedSiloPatch: parseChangeAssumedSiloPatch,
  ChangeAssumedSiloWallFriction: parseChangeAssumedSiloWallFriction,
  ChangeFloorAssumedQk: parseChangeFloorAssumedQk,
  ChangeSelfWeightAssumedGk: parseChangeSelfWeightAssumedGk,
  ChangeRoofAssumedSk: parseChangeRoofAssumedSk,
  ChangeWindFaceAssumedWp: parseChangeWindFaceAssumedWp,
  ChangeAccidentalAssumedForce: parseChangeAccidentalAssumedForce,
  InsertFloor: parseInsertFloor,
  RemoveFloor: parseRemoveFloor,
  InsertSelfWeightElement: parseInsertSelfWeightElement,
  RemoveSelfWeightElement: parseRemoveSelfWeightElement,
  InsertRoof: parseInsertRoof,
  RemoveRoof: parseRemoveRoof,
  InsertWindFace: parseInsertWindFace,
  RemoveWindFace: parseRemoveWindFace,
  InsertAccidentalCase: parseInsertAccidentalCase,
  RemoveAccidentalCase: parseRemoveAccidentalCase,
  ChangeRoofPitchDeg: parseChangeRoofPitchDeg,
  ChangeRoofParapetHeight: parseChangeRoofParapetHeight,
  ChangeRoofDriftObstructionHeight: parseChangeRoofDriftObstructionHeight,
  ChangeFloorArea: parseChangeFloorArea,
  ChangeSelfWeightThickness: parseChangeSelfWeightThickness,
  ChangeWindFaceZ: parseChangeWindFaceZ,
  ChangeWindFaceLoadedArea: parseChangeWindFaceLoadedArea,
  ChangeLaneWidth: parseChangeLaneWidth,
  ChangeLaneOffset: parseChangeLaneOffset,
  ChangeLaneLm1Lane: parseChangeLaneLm1Lane,
  InsertBridgeLane: parseInsertBridgeLane,
  RemoveBridgeLane: parseRemoveBridgeLane,
  ChangeStageActivity: parseChangeStageActivity,
  ChangeStageDurationDays: parseChangeStageDurationDays,
  ChangeStageReturnYears: parseChangeStageReturnYears,
  ChangeStageAssumedQk: parseChangeStageAssumedQk,
  InsertConstructionStage: parseInsertConstructionStage,
  RemoveConstructionStage: parseRemoveConstructionStage,
  ChangeRoofSurfaceFace: parseChangeRoofSurfaceFace,
  ChangeRoofSurfaceShape: parseChangeRoofSurfaceShape,
  ChangeRoofSurfacePitchDeg: parseChangeRoofSurfacePitchDeg,
  ChangeRoofSurfaceExtent: parseChangeRoofSurfaceExtent,
  InsertRoofSurface: parseInsertRoofSurface,
  RemoveRoofSurface: parseRemoveRoofSurface,
  ChangePlacementX: parseChangePlacementX,
  ChangePlacementY: parseChangePlacementY,
  ChangePlacementZ: parseChangePlacementZ,
  ChangePlacementYaw: parseChangePlacementYaw,
  InsertPlacement: parseInsertPlacement,
  RemovePlacement: parseRemovePlacement,
  InsertCase: parseInsertCase,
  RemoveCase: parseRemoveCase,
  ChangeCaseNumber: parseChangeCaseNumber,
  ChangeCaseText: parseChangeCaseText,
  ChangeCaseLabelEn: parseChangeCaseLabelEn,
  ChangeCaseLabelDe: parseChangeCaseLabelDe,
  ChangeCaseEntryId: parseChangeCaseEntryId,
  ChangeFloorCategory: parseChangeFloorCategory,
  ChangeFloorAssumedQkConc: parseChangeFloorAssumedQkConc,
  ChangeFloorAssumedPartitions: parseChangeFloorAssumedPartitions,
  ChangeSelfWeightMaterial: parseChangeSelfWeightMaterial,
  ChangeRoofRoofType: parseChangeRoofRoofType,
  ChangeRoofCE: parseChangeRoofCE,
  ChangeRoofCT: parseChangeRoofCT,
  ChangeRoofHasParapet: parseChangeRoofHasParapet,
  ChangeRoofMultiSpan: parseChangeRoofMultiSpan,
  ChangeWindFaceZone: parseChangeWindFaceZone,
  ChangeWindFaceCPe10: parseChangeWindFaceCPe10,
  ChangeWindFaceCPe1: parseChangeWindFaceCPe1,
  ChangeWindFaceCPi: parseChangeWindFaceCPi,
  ChangeWindFaceCS: parseChangeWindFaceCS,
  ChangeWindFaceCD: parseChangeWindFaceCD,
  ChangeSpecialVehicleDesignation: parseChangeSpecialVehicleDesignation,
  ChangeSpecialVehicleX: parseChangeSpecialVehicleX,
  ChangeSpecialVehicleY: parseChangeSpecialVehicleY,
  InsertSpecialVehicle: parseInsertSpecialVehicle,
  RemoveSpecialVehicle: parseRemoveSpecialVehicle,
  ChangeWindBridgeDeckWidth: parseChangeWindBridgeDeckWidth,
  ChangeWindBridgeDeckDepth: parseChangeWindBridgeDeckDepth,
  ChangeWindBridgeParapetKind: parseChangeWindBridgeParapetKind,
  ChangeWindBridgeParapetHeight: parseChangeWindBridgeParapetHeight,
  ChangeWindBridgeTrafficKind: parseChangeWindBridgeTrafficKind,
  ChangeWindBridgeGroundHeight: parseChangeWindBridgeGroundHeight,
  ChangeWindBridgeSpan: parseChangeWindBridgeSpan,
  ChangeWindBridgeAssumedForce: parseChangeWindBridgeAssumedForce,
  InsertWindBridge: parseInsertWindBridge,
  RemoveWindBridge: parseRemoveWindBridge,
  ChangeDynamicElementHeight: parseChangeDynamicElementHeight,
  ChangeDynamicElementWidth: parseChangeDynamicElementWidth,
  ChangeDynamicFirstFrequency: parseChangeDynamicFirstFrequency,
  ChangeDynamicLogDecrement: parseChangeDynamicLogDecrement,
  ChangeDynamicMassPerLength: parseChangeDynamicMassPerLength,
  ChangeDynamicElementStrouhal: parseChangeDynamicElementStrouhal,
  ChangeDynamicAssumedCscd: parseChangeDynamicAssumedCscd,
  InsertDynamicElement: parseInsertDynamicElement,
  RemoveDynamicElement: parseRemoveDynamicElement,
  ChangeImpactActionKind: parseChangeImpactActionKind,
  ChangeImpactActionMass: parseChangeImpactActionMass,
  ChangeImpactActionSpeed: parseChangeImpactActionSpeed,
  ChangeImpactActionStiffness: parseChangeImpactActionStiffness,
  ChangeImpactActionHeight: parseChangeImpactActionHeight,
  ChangeImpactActionX: parseChangeImpactActionX,
  ChangeImpactActionY: parseChangeImpactActionY,
  ChangeImpactActionAssumedForce: parseChangeImpactActionAssumedForce,
  InsertImpactAction: parseInsertImpactAction,
  RemoveImpactAction: parseRemoveImpactAction,
  ChangeInternalExplosionVolume: parseChangeInternalExplosionVolume,
  ChangeExplosionVentArea: parseChangeExplosionVentArea,
  ChangeExplosionVentPressure: parseChangeExplosionVentPressure,
  ChangeExplosionAssumedPressure: parseChangeExplosionAssumedPressure,
  InsertInternalExplosion: parseInsertInternalExplosion,
  RemoveInternalExplosion: parseRemoveInternalExplosion,
  ChangeThermalSectionDeckType: parseChangeThermalSectionDeckType,
  ChangeThermalSectionSurfacing: parseChangeThermalSectionSurfacing,
  ChangeThermalSectionLength: parseChangeThermalSectionLength,
  ChangeThermalSectionDepth: parseChangeThermalSectionDepth,
  ChangeThermalSectionAlphaMicro: parseChangeThermalSectionAlphaMicro,
  ChangeThermalAssumedRange: parseChangeThermalAssumedRange,
  InsertThermalSection: parseInsertThermalSection,
  RemoveThermalSection: parseRemoveThermalSection,
  ChangeAccidentalVehicleMass: parseChangeAccidentalVehicleMass,
  ChangeAccidentalVehicleSpeed: parseChangeAccidentalVehicleSpeed,
  ChangeAccidentalExplosionMass: parseChangeAccidentalExplosionMass,
  ChangeAccidentalStandoff: parseChangeAccidentalStandoff,
  ChangeAccidentalAssumedPressure: parseChangeAccidentalAssumedPressure,
  ChangeHelicopterMass: parseChangeHelicopterMass,
  ChangeHelicopterX: parseChangeHelicopterX,
  ChangeHelicopterY: parseChangeHelicopterY,
  ChangeHelicopterHeight: parseChangeHelicopterHeight,
  ChangeHelicopterAssumedForce: parseChangeHelicopterAssumedForce,
  InsertHelicopter: parseInsertHelicopter,
  RemoveHelicopter: parseRemoveHelicopter,
  ChangeTieRunKind: parseChangeTieRunKind,
  ChangeTieRunSpacing: parseChangeTieRunSpacing,
  ChangeTieRunSpan: parseChangeTieRunSpan,
  ChangeTieRunDeadLoad: parseChangeTieRunDeadLoad,
  ChangeTieRunImposedLoad: parseChangeTieRunImposedLoad,
  ChangeTieRunPsi: parseChangeTieRunPsi,
  ChangeTieRunX: parseChangeTieRunX,
  ChangeTieRunY: parseChangeTieRunY,
  ChangeTieRunProvidedForce: parseChangeTieRunProvidedForce,
  InsertTieRun: parseInsertTieRun,
  RemoveTieRun: parseRemoveTieRun,
  ChangeConstructionLoadStage: parseChangeConstructionLoadStage,
  ChangeConstructionLoadKind: parseChangeConstructionLoadKind,
  ChangeConstructionLoadMagnitude: parseChangeConstructionLoadMagnitude,
  InsertConstructionLoad: parseInsertConstructionLoad,
  RemoveConstructionLoad: parseRemoveConstructionLoad,
  ChangeFallingMass: parseChangeFallingMass,
  ChangeFallingDropHeight: parseChangeFallingDropHeight,
  ChangeFallingStoppingDistance: parseChangeFallingStoppingDistance,
  ChangeFallingX: parseChangeFallingX,
  ChangeFallingY: parseChangeFallingY,
  ChangeFallingAssumedForce: parseChangeFallingAssumedForce,
  InsertFalling: parseInsertFalling,
  RemoveFalling: parseRemoveFalling,
  ChangeImperfectionHeight: parseChangeImperfectionHeight,
  ChangeImperfectionTolerance: parseChangeImperfectionTolerance,
  ChangeImperfectionVerticalLoad: parseChangeImperfectionVerticalLoad,
  ChangeImperfectionX: parseChangeImperfectionX,
  ChangeImperfectionY: parseChangeImperfectionY,
  ChangeImperfectionAssumedForce: parseChangeImperfectionAssumedForce,
  InsertImperfection: parseInsertImperfection,
  RemoveImperfection: parseRemoveImperfection,
  ChangeDerailmentDistance: parseChangeDerailmentDistance,
  ChangeDerailmentDesignForceX: parseChangeDerailmentDesignForceX,
  ChangeDerailmentDesignForceY: parseChangeDerailmentDesignForceY,
  ChangeDerailmentX: parseChangeDerailmentX,
  ChangeDerailmentY: parseChangeDerailmentY,
  ChangeDerailmentAssumedForce: parseChangeDerailmentAssumedForce,
  InsertDerailment: parseInsertDerailment,
  RemoveDerailment: parseRemoveDerailment,
  ChangeCraneHoistRatedLoad: parseChangeCraneHoistRatedLoad,
  ChangeCraneHoistReleaseFraction: parseChangeCraneHoistReleaseFraction,
  ChangeCraneHoistReleaseDevice: parseChangeCraneHoistReleaseDevice,
  InsertCraneHoist: parseInsertCraneHoist,
  RemoveCraneHoist: parseRemoveCraneHoist,
  ChangeFootbridgeLoadedLength: parseChangeFootbridgeLoadedLength,
  ChangeFootbridgeDeckWidth: parseChangeFootbridgeDeckWidth,
  ChangeFootbridgeNaturalFrequency: parseChangeFootbridgeNaturalFrequency,
  InsertFootbridge: parseInsertFootbridge,
  RemoveFootbridge: parseRemoveFootbridge,
  ChangeRailDynamicDeterminantLength: parseChangeRailDynamicDeterminantLength,
  ChangeRailDynamicMaintenance: parseChangeRailDynamicMaintenance,
  InsertRailDynamic: parseInsertRailDynamic,
  RemoveRailDynamic: parseRemoveRailDynamic,
  ChangeLoadDispersalWheelLength: parseChangeLoadDispersalWheelLength,
  ChangeLoadDispersalWheelWidth: parseChangeLoadDispersalWheelWidth,
  ChangeLoadDispersalPavingThickness: parseChangeLoadDispersalPavingThickness,
  ChangeLoadDispersalSlabThickness: parseChangeLoadDispersalSlabThickness,
  InsertLoadDispersal: parseInsertLoadDispersal,
  RemoveLoadDispersal: parseRemoveLoadDispersal,
  ChangeBulkSolidInternalFriction: parseChangeBulkSolidInternalFriction,
  InsertBulkSolid: parseInsertBulkSolid,
  RemoveBulkSolid: parseRemoveBulkSolid,
  ChangeRotatingMachineMass: parseChangeRotatingMachineMass,
  ChangeRotatingMachineEccentricity: parseChangeRotatingMachineEccentricity,
  ChangeRotatingMachineFrequency: parseChangeRotatingMachineFrequency,
  InsertRotatingMachine: parseInsertRotatingMachine,
  RemoveRotatingMachine: parseRemoveRotatingMachine,
});
