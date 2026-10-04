/** 🧺️ `Din4108Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeElementAdjacent, parseChangeElementAdjacent } from "./↔️change-element-adjacent/🧬️schema/🟦️.ts";
import { type ChangeThermalBridgeLength, parseChangeThermalBridgeLength } from "./↔️change-thermal-bridge-length/🧬️schema/🟦️.ts";
import { type ChangeZoneWindowGValue, parseChangeZoneWindowGValue } from "./☀️change-zone-window-g-value/🧬️schema/🟦️.ts";
import { type ChangeZoneWindowShadingFc, parseChangeZoneWindowShadingFc } from "./⛱️change-zone-window-shading-fc/🧬️schema/🟦️.ts";
import { type ChangeBb2DetailsConform, parseChangeBb2DetailsConform } from "./✅️change-bb2-details-conform/🧬️schema/🟦️.ts";
import { type InsertLayer, parseInsertLayer } from "./➕️insert-layer/🧬️schema/🟦️.ts";
import { type InsertZone, parseInsertZone } from "./➕️insert-zone/🧬️schema/🟦️.ts";
import { parseRemoveLayer, type RemoveLayer } from "./➖️remove-layer/🧬️schema/🟦️.ts";
import { parseRemoveZone, type RemoveZone } from "./➖️remove-zone/🧬️schema/🟦️.ts";
import { type InsertThermalBridge, parseInsertThermalBridge } from "./🌉️insert-thermal-bridge/🧬️schema/🟦️.ts";
import { type ChangeZoneNightVentilation, parseChangeZoneNightVentilation } from "./🌙change-zone-night-ventilation/🧬️schema/🟦️.ts";
import { type ChangeLayerLambda, parseChangeLayerLambda } from "./🌡️change-layer-lambda/🧬️schema/🟦️.ts";
import { type ChangeTIntC, parseChangeTIntC } from "./🌡️change-t-int-c/🧬️schema/🟦️.ts";
import { type ChangeClimateZone, parseChangeClimateZone } from "./🌦️change-climate-zone/🧬️schema/🟦️.ts";
import { type InsertElement, parseInsertElement } from "./🏠️insert-element/🧬️schema/🟦️.ts";
import { type ChangeElementKind, parseChangeElementKind } from "./🏷️change-element-kind/🧬️schema/🟦️.ts";
import { type ChangeLayerApplicationType, parseChangeLayerApplicationType } from "./🏷️change-layer-application-type/🧬️schema/🟦️.ts";
import { type ChangeLayerCompressiveClass, parseChangeLayerCompressiveClass } from "./🏷️change-layer-compressive-class/🧬️schema/🟦️.ts";
import { type ChangeThermalBridgeBb2Type, parseChangeThermalBridgeBb2Type } from "./🏷️change-thermal-bridge-bb2-type/🧬️schema/🟦️.ts";
import { type ChangeLayerMu, parseChangeLayerMu } from "./💧change-layer-mu/🧬️schema/🟦️.ts";
import { type ChangeRhInt, parseChangeRhInt } from "./💧️change-rh-int/🧬️schema/🟦️.ts";
import { type ChangeAirtightnessN50, parseChangeAirtightnessN50 } from "./💨️change-airtightness-n50/🧬️schema/🟦️.ts";
import { type ChangeHasMechanicalVentilation, parseChangeHasMechanicalVentilation } from "./💨change-has-mechanical-ventilation/🧬️schema/🟦️.ts";
import { type ChangeElementDeltaUf, parseChangeElementDeltaUf } from "./📈️change-element-delta-uf/🧬️schema/🟦️.ts";
import { type ChangeElementDeltaUg, parseChangeElementDeltaUg } from "./📈️change-element-delta-ug/🧬️schema/🟦️.ts";
import { type ChangeElementDeltaUr, parseChangeElementDeltaUr } from "./📈️change-element-delta-ur/🧬️schema/🟦️.ts";
import { type ChangeLayerThickness, parseChangeLayerThickness } from "./📏️change-layer-thickness/🧬️schema/🟦️.ts";
import { type ChangeZoneWindowArea, parseChangeZoneWindowArea } from "./📏change-zone-window-area/🧬️schema/🟦️.ts";
import { type ChangeElementArea, parseChangeElementArea } from "./📐️change-element-area/🧬️schema/🟦️.ts";
import { type ChangeElementInclinationDeg, parseChangeElementInclinationDeg } from "./📐change-element-inclination-deg/🧬️schema/🟦️.ts";
import { type ChangeZoneFloorArea, parseChangeZoneFloorArea } from "./📐️change-zone-floor-area/🧬️schema/🟦️.ts";
import { type ChangeZoneWindowInclinationDeg, parseChangeZoneWindowInclinationDeg } from "./📐change-zone-window-inclination-deg/🧬️schema/🟦️.ts";
import { parseReorderLayers, type ReorderLayers } from "./🔀️reorder-layers/🧬️schema/🟦️.ts";
import { type ChangeThermalBridgePsi, parseChangeThermalBridgePsi } from "./🔘change-thermal-bridge-psi/🧬️schema/🟦️.ts";
import { type ChangeUsage, parseChangeUsage } from "./🗂️change-usage/🧬️schema/🟦️.ts";
import { parseRemoveThermalBridge, type RemoveThermalBridge } from "./🧊remove-thermal-bridge/🧬️schema/🟦️.ts";
import { type ChangeElementOrientationDeg, parseChangeElementOrientationDeg } from "./🧭change-element-orientation-deg/🧬️schema/🟦️.ts";
import { type ChangeZoneWindowOrientation, parseChangeZoneWindowOrientation } from "./🧭change-zone-window-orientation/🧬️schema/🟦️.ts";
import { type ChangeZoneHeaviness, parseChangeZoneHeaviness } from "./🧱change-zone-heaviness/🧬️schema/🟦️.ts";
import { type ChangeLayerMaterialId, parseChangeLayerMaterialId } from "./🧽️change-layer-material-id/🧬️schema/🟦️.ts";
import { type InsertZoneWindow, parseInsertZoneWindow } from "./🪟insert-zone-window/🧬️schema/🟦️.ts";
import { parseRemoveElement, type RemoveElement } from "./🚫️remove-element/🧬️schema/🟦️.ts";
import { parseRemoveZoneWindow, type RemoveZoneWindow } from "./🚫️remove-zone-window/🧬️schema/🟦️.ts";

export type Din4108Mutation =
  | { ChangeClimateZone: ChangeClimateZone }
  | { ChangeUsage: ChangeUsage }
  | { ChangeTIntC: ChangeTIntC }
  | { ChangeRhInt: ChangeRhInt }
  | { ChangeAirtightnessN50: ChangeAirtightnessN50 }
  | { ChangeHasMechanicalVentilation: ChangeHasMechanicalVentilation }
  | { ChangeBb2DetailsConform: ChangeBb2DetailsConform }
  | { InsertZone: InsertZone }
  | { RemoveZone: RemoveZone }
  | { ChangeZoneFloorArea: ChangeZoneFloorArea }
  | { ChangeZoneHeaviness: ChangeZoneHeaviness }
  | { ChangeZoneNightVentilation: ChangeZoneNightVentilation }
  | { InsertZoneWindow: InsertZoneWindow }
  | { RemoveZoneWindow: RemoveZoneWindow }
  | { ChangeZoneWindowArea: ChangeZoneWindowArea }
  | { ChangeZoneWindowGValue: ChangeZoneWindowGValue }
  | { ChangeZoneWindowShadingFc: ChangeZoneWindowShadingFc }
  | { InsertElement: InsertElement }
  | { RemoveElement: RemoveElement }
  | { ChangeElementArea: ChangeElementArea }
  | { ChangeElementAdjacent: ChangeElementAdjacent }
  | { ChangeElementKind: ChangeElementKind }
  | { InsertLayer: InsertLayer }
  | { RemoveLayer: RemoveLayer }
  | { ReorderLayers: ReorderLayers }
  | { ChangeLayerThickness: ChangeLayerThickness }
  | { ChangeLayerLambda: ChangeLayerLambda }
  | { ChangeLayerMu: ChangeLayerMu }
  | { ChangeLayerMaterialId: ChangeLayerMaterialId }
  | { InsertThermalBridge: InsertThermalBridge }
  | { RemoveThermalBridge: RemoveThermalBridge }
  | { ChangeThermalBridgePsi: ChangeThermalBridgePsi }
  | { ChangeThermalBridgeLength: ChangeThermalBridgeLength }
  | { ChangeElementOrientationDeg: ChangeElementOrientationDeg }
  | { ChangeElementInclinationDeg: ChangeElementInclinationDeg }
  | { ChangeElementDeltaUg: ChangeElementDeltaUg }
  | { ChangeElementDeltaUf: ChangeElementDeltaUf }
  | { ChangeElementDeltaUr: ChangeElementDeltaUr }
  | { ChangeThermalBridgeBb2Type: ChangeThermalBridgeBb2Type }
  | { ChangeZoneWindowOrientation: ChangeZoneWindowOrientation }
  | { ChangeZoneWindowInclinationDeg: ChangeZoneWindowInclinationDeg }
  | { ChangeLayerApplicationType: ChangeLayerApplicationType }
  | { ChangeLayerCompressiveClass: ChangeLayerCompressiveClass };

export const parseDin4108Mutation: NormWireReader<Din4108Mutation> = normWireExternal<Din4108Mutation>({
  ChangeClimateZone: parseChangeClimateZone,
  ChangeUsage: parseChangeUsage,
  ChangeTIntC: parseChangeTIntC,
  ChangeRhInt: parseChangeRhInt,
  ChangeAirtightnessN50: parseChangeAirtightnessN50,
  ChangeHasMechanicalVentilation: parseChangeHasMechanicalVentilation,
  ChangeBb2DetailsConform: parseChangeBb2DetailsConform,
  InsertZone: parseInsertZone,
  RemoveZone: parseRemoveZone,
  ChangeZoneFloorArea: parseChangeZoneFloorArea,
  ChangeZoneHeaviness: parseChangeZoneHeaviness,
  ChangeZoneNightVentilation: parseChangeZoneNightVentilation,
  InsertZoneWindow: parseInsertZoneWindow,
  RemoveZoneWindow: parseRemoveZoneWindow,
  ChangeZoneWindowArea: parseChangeZoneWindowArea,
  ChangeZoneWindowGValue: parseChangeZoneWindowGValue,
  ChangeZoneWindowShadingFc: parseChangeZoneWindowShadingFc,
  InsertElement: parseInsertElement,
  RemoveElement: parseRemoveElement,
  ChangeElementArea: parseChangeElementArea,
  ChangeElementAdjacent: parseChangeElementAdjacent,
  ChangeElementKind: parseChangeElementKind,
  InsertLayer: parseInsertLayer,
  RemoveLayer: parseRemoveLayer,
  ReorderLayers: parseReorderLayers,
  ChangeLayerThickness: parseChangeLayerThickness,
  ChangeLayerLambda: parseChangeLayerLambda,
  ChangeLayerMu: parseChangeLayerMu,
  ChangeLayerMaterialId: parseChangeLayerMaterialId,
  InsertThermalBridge: parseInsertThermalBridge,
  RemoveThermalBridge: parseRemoveThermalBridge,
  ChangeThermalBridgePsi: parseChangeThermalBridgePsi,
  ChangeThermalBridgeLength: parseChangeThermalBridgeLength,
  ChangeElementOrientationDeg: parseChangeElementOrientationDeg,
  ChangeElementInclinationDeg: parseChangeElementInclinationDeg,
  ChangeElementDeltaUg: parseChangeElementDeltaUg,
  ChangeElementDeltaUf: parseChangeElementDeltaUf,
  ChangeElementDeltaUr: parseChangeElementDeltaUr,
  ChangeThermalBridgeBb2Type: parseChangeThermalBridgeBb2Type,
  ChangeZoneWindowOrientation: parseChangeZoneWindowOrientation,
  ChangeZoneWindowInclinationDeg: parseChangeZoneWindowInclinationDeg,
  ChangeLayerApplicationType: parseChangeLayerApplicationType,
  ChangeLayerCompressiveClass: parseChangeLayerCompressiveClass,
});
