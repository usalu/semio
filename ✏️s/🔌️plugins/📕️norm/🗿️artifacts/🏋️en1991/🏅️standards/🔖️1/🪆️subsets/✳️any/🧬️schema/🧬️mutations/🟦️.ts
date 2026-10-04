/** 🧺️ `En1991Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeAssumedCraneHorizontal, parseChangeAssumedCraneHorizontal } from "./↔️change-assumed-crane-horizontal/🧬️schema/🟦️.ts";
import { type ChangeBridgeLaneWidth, parseChangeBridgeLaneWidth } from "./↔️change-bridge-lane-width/🧬️schema/🟦️.ts";
import { type ChangeHoistingSpeed, parseChangeHoistingSpeed } from "./⏫change-hoisting-speed/🧬️schema/🟦️.ts";
import { type ChangeFireDuration, parseChangeFireDuration } from "./⏱️change-fire-duration/🧬️schema/🟦️.ts";
import { type ChangeAssumedGasTemperature, parseChangeAssumedGasTemperature } from "./♨️change-assumed-gas-temperature/🧬️schema/🟦️.ts";
import { type ChangeSiloKind, parseChangeSiloKind } from "./⚖️change-silo-kind/🧬️schema/🟦️.ts";
import { type ChangeAssumedConstructionQk, parseChangeAssumedConstructionQk } from "./⚙️change-assumed-construction-qk/🧬️schema/🟦️.ts";
import { type ChangeSiloK, parseChangeSiloK } from "./⚙️change-silo-k/🧬️schema/🟦️.ts";
import { type ChangeFireLoadDensityQf, parseChangeFireLoadDensityQf } from "./⛽change-fire-load-density-qf/🧬️schema/🟦️.ts";
import { type ChangeAltitude, parseChangeAltitude } from "./❄️change-altitude/🧬️schema/🟦️.ts";
import { type ChangeEnSk, parseChangeEnSk } from "./❄️change-en-sk/🧬️schema/🟦️.ts";
import { type ChangeHoistClass, parseChangeHoistClass } from "./➕change-hoist-class/🧬️schema/🟦️.ts";
import { type InsertAccidentalCases, parseInsertAccidentalCases } from "./➕️insert-accidental-cases/🧬️schema/🟦️.ts";
import { type InsertFloors, parseInsertFloors } from "./➕️insert-floors/🧬️schema/🟦️.ts";
import { type InsertRoofs, parseInsertRoofs } from "./➕️insert-roofs/🧬️schema/🟦️.ts";
import { type InsertSelfWeightElements, parseInsertSelfWeightElements } from "./➕️insert-self-weight-elements/🧬️schema/🟦️.ts";
import { type InsertWindFaces, parseInsertWindFaces } from "./➕️insert-wind-faces/🧬️schema/🟦️.ts";
import { type ChangeAssumedCraneWheel, parseChangeAssumedCraneWheel } from "./➖change-assumed-crane-wheel/🧬️schema/🟦️.ts";
import { parseRemoveAccidentalCases, type RemoveAccidentalCases } from "./➖️remove-accidental-cases/🧬️schema/🟦️.ts";
import { parseRemoveFloors, type RemoveFloors } from "./➖️remove-floors/🧬️schema/🟦️.ts";
import { parseRemoveRoofs, type RemoveRoofs } from "./➖️remove-roofs/🧬️schema/🟦️.ts";
import { parseRemoveSelfWeightElements, type RemoveSelfWeightElements } from "./➖️remove-self-weight-elements/🧬️schema/🟦️.ts";
import { parseRemoveWindFaces, type RemoveWindFaces } from "./➖️remove-wind-faces/🧬️schema/🟦️.ts";
import { type ChangeSiloHydraulicRadius, parseChangeSiloHydraulicRadius } from "./⭕change-silo-hydraulic-radius/🧬️schema/🟦️.ts";
import { type ChangeAssumedSiloPressure, parseChangeAssumedSiloPressure } from "./🌀change-assumed-silo-pressure/🧬️schema/🟦️.ts";
import { type ChangeBridgeLane, parseChangeBridgeLane } from "./🌉change-bridge-lane/🧬️schema/🟦️.ts";
import { type ChangeStructureKind, parseChangeStructureKind } from "./🌉change-structure-kind/🧬️schema/🟦️.ts";
import { type ChangeThermalBridgeType, parseChangeThermalBridgeType } from "./🌉change-thermal-bridge-type/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍change-annex/🧬️schema/🟦️.ts";
import { type ChangeAssumedDeltaT, parseChangeAssumedDeltaT } from "./🌡️change-assumed-delta-t/🧬️schema/🟦️.ts";
import { type ChangeTMax, parseChangeTMax } from "./🌡️change-t-max/🧬️schema/🟦️.ts";
import { type ChangeRoofAssumedSk, parseChangeRoofAssumedSk } from "./🌨️change-roof-assumed-sk/🧬️schema/🟦️.ts";
import { type ChangeEnVb, parseChangeEnVb } from "./🌬️change-en-vb/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeTandem, parseChangeAssumedBridgeTandem } from "./🌾change-assumed-bridge-tandem/🧬️schema/🟦️.ts";
import { type ChangeSiloBulkDensity, parseChangeSiloBulkDensity } from "./🌾change-silo-bulk-density/🧬️schema/🟦️.ts";
import { type ChangeNorthGermanLowlandSnow, parseChangeNorthGermanLowlandSnow } from "./🏔️change-north-german-lowland-snow/🧬️schema/🟦️.ts";
import { type ChangeBridgeSpan, parseChangeBridgeSpan } from "./🏗️change-bridge-span/🧬️schema/🟦️.ts";
import { type ChangeCraneClaimed, parseChangeCraneClaimed } from "./🏗️change-crane-claimed/🧬️schema/🟦️.ts";
import { type ChangeThermalElementType, parseChangeThermalElementType } from "./🏗️change-thermal-element-type/🧬️schema/🟦️.ts";
import { type ChangeStoreyCount, parseChangeStoreyCount } from "./🏙️change-storey-count/🧬️schema/🟦️.ts";
import { type ChangeCoastOrIsland, parseChangeCoastOrIsland } from "./🏝️change-coast-or-island/🧬️schema/🟦️.ts";
import { type ChangeTerrainCategory, parseChangeTerrainCategory } from "./🏞️change-terrain-category/🧬️schema/🟦️.ts";
import { type ChangeWidth, parseChangeWidth } from "./🏠change-width/🧬️schema/🟦️.ts";
import { type ChangeAirDensity, parseChangeAirDensity } from "./🏢change-air-density/🧬️schema/🟦️.ts";
import { type ChangeFireOccupancy, parseChangeFireOccupancy } from "./🏢change-fire-occupancy/🧬️schema/🟦️.ts";
import { type ChangeFloorAssumedQk, parseChangeFloorAssumedQk } from "./🏢change-floor-assumed-qk/🧬️schema/🟦️.ts";
import { type ChangeSiloClaimed, parseChangeSiloClaimed } from "./🏭change-silo-claimed/🧬️schema/🟦️.ts";
import { type ChangeSiloHeight, parseChangeSiloHeight } from "./🏷️change-silo-height/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeLm4, parseChangeAssumedBridgeLm4 } from "./👥change-assumed-bridge-lm4/🧬️schema/🟦️.ts";
import { type ChangeCraneClass, parseChangeCraneClass } from "./💥change-crane-class/🧬️schema/🟦️.ts";
import { type ChangeDepth, parseChangeDepth } from "./💨change-depth/🧬️schema/🟦️.ts";
import { type ChangeFireCurve, parseChangeFireCurve } from "./📉change-fire-curve/🧬️schema/🟦️.ts";
import { type ChangeLinearTemperatureGradient, parseChangeLinearTemperatureGradient } from "./📏change-linear-temperature-gradient/🧬️schema/🟦️.ts";
import { type ChangeMixedTerrainDistance, parseChangeMixedTerrainDistance } from "./📏change-mixed-terrain-distance/🧬️schema/🟦️.ts";
import { type ChangeFireCompartmentHeight, parseChangeFireCompartmentHeight } from "./📐change-fire-compartment-height/🧬️schema/🟦️.ts";
import { type ChangeOrographyFactor, parseChangeOrographyFactor } from "./📐change-orography-factor/🧬️schema/🟦️.ts";
import { type ChangeAssumedSiloPatch, parseChangeAssumedSiloPatch } from "./📦change-assumed-silo-patch/🧬️schema/🟦️.ts";
import { type ChangeBridgeLoadGroup, parseChangeBridgeLoadGroup } from "./📦change-bridge-load-group/🧬️schema/🟦️.ts";
import { type ChangeAssumedHNet, parseChangeAssumedHNet } from "./🔆change-assumed-h-net/🧬️schema/🟦️.ts";
import { type ChangeAssumedQfD, parseChangeAssumedQfD } from "./🔋change-assumed-qf-d/🧬️schema/🟦️.ts";
import { type ChangeSiloMu, parseChangeSiloMu } from "./🔎change-silo-mu/🧬️schema/🟦️.ts";
import { type ChangeConstructionActivity, parseChangeConstructionActivity } from "./🔥change-construction-activity/🧬️schema/🟦️.ts";
import { type ChangeFireMode, parseChangeFireMode } from "./🔥change-fire-mode/🧬️schema/🟦️.ts";
import { type ChangeInitialTemperature, parseChangeInitialTemperature } from "./🕰️change-initial-temperature/🧬️schema/🟦️.ts";
import { type ChangeFireCompartmentArea, parseChangeFireCompartmentArea } from "./🗺️change-fire-compartment-area/🧬️schema/🟦️.ts";
import { type ChangeSnowZone, parseChangeSnowZone } from "./🗺️change-snow-zone/🧬️schema/🟦️.ts";
import { type ChangeTMin, parseChangeTMin } from "./🧊change-t-min/🧬️schema/🟦️.ts";
import { type ChangeMixedTerrainUpwind, parseChangeMixedTerrainUpwind } from "./🧭change-mixed-terrain-upwind/🧬️schema/🟦️.ts";
import { type ChangeAssumedSiloWallFriction, parseChangeAssumedSiloWallFriction } from "./🧱change-assumed-silo-wall-friction/🧬️schema/🟦️.ts";
import { type ChangeFireThermalInertia, parseChangeFireThermalInertia } from "./🧱change-fire-thermal-inertia/🧬️schema/🟦️.ts";
import { type ChangeHeight, parseChangeHeight } from "./🧱change-height/🧬️schema/🟦️.ts";
import { type ChangeWindZone, parseChangeWindZone } from "./🪁change-wind-zone/🧬️schema/🟦️.ts";
import { type ChangeFireOpeningFactor, parseChangeFireOpeningFactor } from "./🪟change-fire-opening-factor/🧬️schema/🟦️.ts";
import { type ChangeAccidentalAssumedForce, parseChangeAccidentalAssumedForce } from "./🚗change-accidental-assumed-force/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeLm2, parseChangeAssumedBridgeLm2 } from "./🚛change-assumed-bridge-lm2/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeLm3, parseChangeAssumedBridgeLm3 } from "./🚛change-assumed-bridge-lm3/🧬️schema/🟦️.ts";
import { type ChangeSelfWeightAssumedGk, parseChangeSelfWeightAssumedGk } from "./🚧change-self-weight-assumed-gk/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeFootway, parseChangeAssumedBridgeFootway } from "./🚶change-assumed-bridge-footway/🧬️schema/🟦️.ts";
import { type ChangeWindFaceAssumedWp, parseChangeWindFaceAssumedWp } from "./🛡️change-wind-face-assumed-wp/🧬️schema/🟦️.ts";
import { type ChangeAssumedBridgeUdl, parseChangeAssumedBridgeUdl } from "./🛣️change-assumed-bridge-udl/🧬️schema/🟦️.ts";

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
  | { InsertFloors: InsertFloors }
  | { RemoveFloors: RemoveFloors }
  | { InsertSelfWeightElements: InsertSelfWeightElements }
  | { RemoveSelfWeightElements: RemoveSelfWeightElements }
  | { InsertRoofs: InsertRoofs }
  | { RemoveRoofs: RemoveRoofs }
  | { InsertWindFaces: InsertWindFaces }
  | { RemoveWindFaces: RemoveWindFaces }
  | { InsertAccidentalCases: InsertAccidentalCases }
  | { RemoveAccidentalCases: RemoveAccidentalCases };

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
  InsertFloors: parseInsertFloors,
  RemoveFloors: parseRemoveFloors,
  InsertSelfWeightElements: parseInsertSelfWeightElements,
  RemoveSelfWeightElements: parseRemoveSelfWeightElements,
  InsertRoofs: parseInsertRoofs,
  RemoveRoofs: parseRemoveRoofs,
  InsertWindFaces: parseInsertWindFaces,
  RemoveWindFaces: parseRemoveWindFaces,
  InsertAccidentalCases: parseInsertAccidentalCases,
  RemoveAccidentalCases: parseRemoveAccidentalCases,
});
