/** 🧺️ `Din18599Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireTagged } from "@semio-tech/norm-js/registry/contract";
import { parseUpdateRenewables, type UpdateRenewables } from "./☀️update-renewables/🧬️schema/🟦️.ts";
import { type ChangeGegQpFactor, parseChangeGegQpFactor } from "./⚖️change-geg-qp-factor/🧬️schema/🟦️.ts";
import { parseUpdateCooling, type UpdateCooling } from "./❄️update-cooling/🧬️schema/🟦️.ts";
import { type InsertZone, parseInsertZone } from "./➕insert-zone/🧬️schema/🟦️.ts";
import { parseRemoveZone, type RemoveZone } from "./➖remove-zone/🧬️schema/🟦️.ts";
import { type ChangeElementAreaM2, parseChangeElementAreaM2 } from "./⬜change-element-area-m2/🧬️schema/🟦️.ts";
import { type ChangeDeltaUWb, parseChangeDeltaUWb } from "./🌉change-delta-u-wb/🧬️schema/🟦️.ts";
import { type ChangeElementU, parseChangeElementU } from "./🌡️change-element-u/🧬️schema/🟦️.ts";
import { parseUpdateClimate, type UpdateClimate } from "./🌦️update-climate/🧬️schema/🟦️.ts";
import { parseUpdateVentilation, type UpdateVentilation } from "./🌬️update-ventilation/🧬️schema/🟦️.ts";
import { type ChangeAutomationClass, parseChangeAutomationClass } from "./🎛️change-automation-class/🧬️schema/🟦️.ts";
import { type ChangeBuildingCategory, parseChangeBuildingCategory } from "./🏠️change-building-category/🧬️schema/🟦️.ts";
import { type ChangeUseClass, parseChangeUseClass } from "./🏷️change-use-class/🧬️schema/🟦️.ts";
import { parseUpdateLighting, type UpdateLighting } from "./💡update-lighting/🧬️schema/🟦️.ts";
import { type ChangeNetFloorAreaM2, parseChangeNetFloorAreaM2 } from "./📐️change-net-floor-area-m2/🧬️schema/🟦️.ts";
import { type ChangeHeatedVolumeM3, parseChangeHeatedVolumeM3 } from "./📦change-heated-volume-m3/🧬️schema/🟦️.ts";
import { parseSpecifyHeatingSystem, type SpecifyHeatingSystem } from "./🔥specify-heating-system/🧬️schema/🟦️.ts";
import { type ChangeZoneThetaIHeatC, parseChangeZoneThetaIHeatC } from "./🔴change-zone-theta-i-heat-c/🧬️schema/🟦️.ts";
import { type ChangeZoneOccupants, parseChangeZoneOccupants } from "./🔵change-zone-occupants/🧬️schema/🟦️.ts";
import { type ChangeElementOrientationDeg, parseChangeElementOrientationDeg } from "./🔶change-element-orientation-deg/🧬️schema/🟦️.ts";
import { type ChangeElementTiltDeg, parseChangeElementTiltDeg } from "./🔷change-element-tilt-deg/🧬️schema/🟦️.ts";
import { type ChangeElementGValue, parseChangeElementGValue } from "./🔸change-element-g-value/🧬️schema/🟦️.ts";
import { type ChangeElementFc, parseChangeElementFc } from "./🔹change-element-fc/🧬️schema/🟦️.ts";
import { type ChangeElementAdjacency, parseChangeElementAdjacency } from "./🔻change-element-adjacency/🧬️schema/🟦️.ts";
import { type ChangeZoneThermalMass, parseChangeZoneThermalMass } from "./🧊change-zone-thermal-mass/🧬️schema/🟦️.ts";
import { type ChangeMethod, parseChangeMethod } from "./🧮change-method/🧬️schema/🟦️.ts";
import { type ChangeAttachment, parseChangeAttachment } from "./🧱change-attachment/🧬️schema/🟦️.ts";
import { parseRemoveElement, type RemoveElement } from "./🚮remove-element/🧬️schema/🟦️.ts";
import { parseSpecifyDhwSystem, type SpecifyDhwSystem } from "./🚿specify-dhw-system/🧬️schema/🟦️.ts";
import { type ChangeZoneLabelEn, parseChangeZoneLabelEn } from "./🟠change-zone-label-en/🧬️schema/🟦️.ts";
import { type ChangeZoneLabelDe, parseChangeZoneLabelDe } from "./🟡change-zone-label-de/🧬️schema/🟦️.ts";
import { type ChangeZoneUsageProfile, parseChangeZoneUsageProfile } from "./🟢change-zone-usage-profile/🧬️schema/🟦️.ts";
import { type ChangeZoneAreaM2, parseChangeZoneAreaM2 } from "./🟣change-zone-area-m2/🧬️schema/🟦️.ts";
import { type ChangeZoneVolumeM3, parseChangeZoneVolumeM3 } from "./🟤change-zone-volume-m3/🧬️schema/🟦️.ts";
import { type ChangeZoneThetaICoolC, parseChangeZoneThetaICoolC } from "./🟥change-zone-theta-i-cool-c/🧬️schema/🟦️.ts";
import { type ChangeElementLabelDe, parseChangeElementLabelDe } from "./🟦change-element-label-de/🧬️schema/🟦️.ts";
import { type ChangeZoneInternalGainsWM2, parseChangeZoneInternalGainsWM2 } from "./🟧change-zone-internal-gains-w-m2/🧬️schema/🟦️.ts";
import { type ChangeZoneLightingPowerWM2, parseChangeZoneLightingPowerWM2 } from "./🟨change-zone-lighting-power-w-m2/🧬️schema/🟦️.ts";
import { type ChangeElementLabelEn, parseChangeElementLabelEn } from "./🟩change-element-label-en/🧬️schema/🟦️.ts";
import { type ChangeElementKind, parseChangeElementKind } from "./🟪change-element-kind/🧬️schema/🟦️.ts";
import { type ChangeElementZoneId, parseChangeElementZoneId } from "./🟫change-element-zone-id/🧬️schema/🟦️.ts";
import { type InsertElement, parseInsertElement } from "./🆕insert-element/🧬️schema/🟦️.ts";

export type Din18599Mutation =
  | ChangeBuildingCategory
  | ChangeAttachment
  | ChangeUseClass
  | ChangeMethod
  | ChangeNetFloorAreaM2
  | ChangeHeatedVolumeM3
  | ChangeGegQpFactor
  | ChangeDeltaUWb
  | ChangeAutomationClass
  | SpecifyHeatingSystem
  | SpecifyDhwSystem
  | UpdateVentilation
  | UpdateCooling
  | UpdateLighting
  | UpdateRenewables
  | InsertZone
  | RemoveZone
  | ChangeZoneLabelEn
  | ChangeZoneLabelDe
  | ChangeZoneUsageProfile
  | ChangeZoneOccupants
  | ChangeZoneAreaM2
  | ChangeZoneVolumeM3
  | ChangeZoneThetaIHeatC
  | ChangeZoneThetaICoolC
  | ChangeZoneInternalGainsWM2
  | ChangeZoneLightingPowerWM2
  | ChangeZoneThermalMass
  | InsertElement
  | RemoveElement
  | ChangeElementLabelEn
  | ChangeElementLabelDe
  | ChangeElementKind
  | ChangeElementZoneId
  | ChangeElementAreaM2
  | ChangeElementOrientationDeg
  | ChangeElementTiltDeg
  | ChangeElementGValue
  | ChangeElementFc
  | ChangeElementAdjacency
  | ChangeElementU
  | UpdateClimate;

export const parseDin18599Mutation: NormWireReader<Din18599Mutation> = normWireTagged<Din18599Mutation, "mutation">("mutation", {
  changeBuildingCategory: parseChangeBuildingCategory,
  changeAttachment: parseChangeAttachment,
  changeUseClass: parseChangeUseClass,
  changeMethod: parseChangeMethod,
  changeNetFloorAreaM2: parseChangeNetFloorAreaM2,
  changeHeatedVolumeM3: parseChangeHeatedVolumeM3,
  changeGegQpFactor: parseChangeGegQpFactor,
  changeDeltaUWb: parseChangeDeltaUWb,
  changeAutomationClass: parseChangeAutomationClass,
  specifyHeatingSystem: parseSpecifyHeatingSystem,
  specifyDhwSystem: parseSpecifyDhwSystem,
  updateVentilation: parseUpdateVentilation,
  updateCooling: parseUpdateCooling,
  updateLighting: parseUpdateLighting,
  updateRenewables: parseUpdateRenewables,
  insertzone: parseInsertZone,
  removezone: parseRemoveZone,
  changezonelabelen: parseChangeZoneLabelEn,
  changezonelabelde: parseChangeZoneLabelDe,
  changezoneusageprofile: parseChangeZoneUsageProfile,
  changezoneoccupants: parseChangeZoneOccupants,
  changezoneaream2: parseChangeZoneAreaM2,
  changezonevolumem3: parseChangeZoneVolumeM3,
  changezonethetaiheatc: parseChangeZoneThetaIHeatC,
  changezonethetaicoolc: parseChangeZoneThetaICoolC,
  changezoneinternalgainswm2: parseChangeZoneInternalGainsWM2,
  changezonelightingpowerwm2: parseChangeZoneLightingPowerWM2,
  changezonethermalmass: parseChangeZoneThermalMass,
  insertelement: parseInsertElement,
  removeelement: parseRemoveElement,
  changeelementlabelen: parseChangeElementLabelEn,
  changeelementlabelde: parseChangeElementLabelDe,
  changeelementkind: parseChangeElementKind,
  changeelementzoneid: parseChangeElementZoneId,
  changeelementaream2: parseChangeElementAreaM2,
  changeelementorientationdeg: parseChangeElementOrientationDeg,
  changeelementtiltdeg: parseChangeElementTiltDeg,
  changeelementgvalue: parseChangeElementGValue,
  changeelementfc: parseChangeElementFc,
  changeelementadjacency: parseChangeElementAdjacency,
  changeElementU: parseChangeElementU,
  updateClimate: parseUpdateClimate,
});
