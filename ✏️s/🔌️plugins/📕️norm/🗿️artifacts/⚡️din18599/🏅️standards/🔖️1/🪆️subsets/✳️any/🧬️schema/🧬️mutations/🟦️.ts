/** 🧺️ `Din18599Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseUpdateRenewables, type UpdateRenewables } from "./☀️update-renewables/🧬️schema/🟦️.ts";
import { type ChangeGegQpFactor, parseChangeGegQpFactor } from "./⚖️change-geg-qp-factor/🧬️schema/🟦️.ts";
import { parseUpdateCooling, type UpdateCooling } from "./❄️update-cooling/🧬️schema/🟦️.ts";
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
import { parseReplaceZones, type ReplaceZones } from "./🗺️replace-zones/🧬️schema/🟦️.ts";
import { parseReplaceElements, type ReplaceElements } from "./🧩replace-elements/🧬️schema/🟦️.ts";
import { type ChangeMethod, parseChangeMethod } from "./🧮change-method/🧬️schema/🟦️.ts";
import { type ChangeAttachment, parseChangeAttachment } from "./🧱change-attachment/🧬️schema/🟦️.ts";
import { parseSpecifyDhwSystem, type SpecifyDhwSystem } from "./🚿specify-dhw-system/🧬️schema/🟦️.ts";

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
  | ReplaceZones
  | ReplaceElements
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
  replaceZones: parseReplaceZones,
  replaceElements: parseReplaceElements,
  changeElementU: parseChangeElementU,
  updateClimate: parseUpdateClimate,
});
