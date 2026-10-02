/** 🧺️ `Din16798Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeZoneTOpSummer, parseChangeZoneTOpSummer } from "./☀️change-zone-t-op-summer/🧬️schema/🟦️.ts";
import { type ChangeVentHeatRecovery, parseChangeVentHeatRecovery } from "./♻️change-vent-heat-recovery/🧬️schema/🟦️.ts";
import { type ChangeVentSystemType, parseChangeVentSystemType } from "./⚙️change-vent-system-type/🧬️schema/🟦️.ts";
import { type ChangeZoneTOpWinter, parseChangeZoneTOpWinter } from "./❄️change-zone-t-op-winter/🧬️schema/🟦️.ts";
import { type InsertZone, parseInsertZone } from "./➕️insert-zone/🧬️schema/🟦️.ts";
import { parseRemoveZone, type RemoveZone } from "./➖️remove-zone/🧬️schema/🟦️.ts";
import { type ChangeCellarVentilation, parseChangeCellarVentilation } from "./🌀change-cellar-ventilation/🧬️schema/🟦️.ts";
import { type ChangeVentSfp, parseChangeVentSfp } from "./🌀️change-vent-sfp/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeNightSetback, parseChangeNightSetback } from "./🌙️change-night-setback/🧬️schema/🟦️.ts";
import { type ChangeOutdoorCo2, parseChangeOutdoorCo2 } from "./🌫️change-outdoor-co2/🧬️schema/🟦️.ts";
import { type ChangeVentDesignAirflow, parseChangeVentDesignAirflow } from "./🌬️change-vent-design-airflow/🧬️schema/🟦️.ts";
import { type ChangeVentSfpClass, parseChangeVentSfpClass } from "./🎓️change-vent-sfp-class/🧬️schema/🟦️.ts";
import { type ChangeZoneMetabolicRate, parseChangeZoneMetabolicRate } from "./🏃️change-zone-metabolic-rate/🧬️schema/🟦️.ts";
import { type ChangeCellarArea, parseChangeCellarArea } from "./🏚️change-cellar-area/🧬️schema/🟦️.ts";
import { type ChangeVentOdaClass, parseChangeVentOdaClass } from "./🏞️change-vent-oda-class/🧬️schema/🟦️.ts";
import { type ChangeEnvelopeN50, parseChangeEnvelopeN50 } from "./🏠️change-envelope-n50/🧬️schema/🟦️.ts";
import { type ChangeZoneUsageType, parseChangeZoneUsageType } from "./🏢️change-zone-usage-type/🧬️schema/🟦️.ts";
import { type ChangeZonePollutionClass, parseChangeZonePollutionClass } from "./🏭️change-zone-pollution-class/🧬️schema/🟦️.ts";
import { type ChangeZoneClothing, parseChangeZoneClothing } from "./👔change-zone-clothing/🧬️schema/🟦️.ts";
import { type ChangeZoneOccupants, parseChangeZoneOccupants } from "./👥️change-zone-occupants/🧬️schema/🟦️.ts";
import { type ChangeZoneIlluminance, parseChangeZoneIlluminance } from "./💡change-zone-illuminance/🧬️schema/🟦️.ts";
import { type ChangeZoneRh, parseChangeZoneRh } from "./💧️change-zone-rh/🧬️schema/🟦️.ts";
import { type ChangeZoneAirSpeed, parseChangeZoneAirSpeed } from "./💨change-zone-air-speed/🧬️schema/🟦️.ts";
import { type ChangeZoneOutdoorAir, parseChangeZoneOutdoorAir } from "./💨️change-zone-outdoor-air/🧬️schema/🟦️.ts";
import { type ChangeZoneTurbulence, parseChangeZoneTurbulence } from "./💨change-zone-turbulence/🧬️schema/🟦️.ts";
import { type ChangeVentInspection, parseChangeVentInspection } from "./📅️change-vent-inspection/🧬️schema/🟦️.ts";
import { type ChangeZoneFloorArea, parseChangeZoneFloorArea } from "./📐️change-zone-floor-area/🧬️schema/🟦️.ts";
import { type ChangeZoneVentMethod, parseChangeZoneVentMethod } from "./📐️change-zone-vent-method/🧬️schema/🟦️.ts";
import { type ChangeEnvelopeVolume, parseChangeEnvelopeVolume } from "./📦️change-envelope-volume/🧬️schema/🟦️.ts";
import { type ChangeThetaRm, parseChangeThetaRm } from "./🔄️change-theta-rm/🧬️schema/🟦️.ts";
import { type ChangeZoneNoise, parseChangeZoneNoise } from "./🔊️change-zone-noise/🧬️schema/🟦️.ts";
import { type ChangeZoneVentSystemId, parseChangeZoneVentSystemId } from "./🔗change-zone-vent-system-id/🧬️schema/🟦️.ts";
import { type ChangeVentDuctLeakage, parseChangeVentDuctLeakage } from "./🕳️change-vent-duct-leakage/🧬️schema/🟦️.ts";
import { parseRemoveVentSystem, type RemoveVentSystem } from "./🗑️remove-vent-system/🧬️schema/🟦️.ts";
import { type ChangeZoneComfortModel, parseChangeZoneComfortModel } from "./🧭️change-zone-comfort-model/🧬️schema/🟦️.ts";
import { type ChangeVentDuctClass, parseChangeVentDuctClass } from "./🧱change-vent-duct-class/🧬️schema/🟦️.ts";
import { type ChangeVentFilterSup, parseChangeVentFilterSup } from "./🧽change-vent-filter-sup/🧬️schema/🟦️.ts";
import { type ChangeZoneCo2, parseChangeZoneCo2 } from "./🫧change-zone-co2/🧬️schema/🟦️.ts";
import { type ChangeZoneComfortCategory, parseChangeZoneComfortCategory } from "./🛋️change-zone-comfort-category/🧬️schema/🟦️.ts";
import { type InsertVentSystem, parseInsertVentSystem } from "./🆕️insert-vent-system/🧬️schema/🟦️.ts";

export type Din16798Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeThetaRm: ChangeThetaRm }
  | { ChangeOutdoorCo2: ChangeOutdoorCo2 }
  | { ChangeEnvelopeN50: ChangeEnvelopeN50 }
  | { ChangeEnvelopeVolume: ChangeEnvelopeVolume }
  | { ChangeCellarArea: ChangeCellarArea }
  | { ChangeCellarVentilation: ChangeCellarVentilation }
  | { ChangeNightSetback: ChangeNightSetback }
  | { InsertZone: InsertZone }
  | { RemoveZone: RemoveZone }
  | { ChangeZoneUsageType: ChangeZoneUsageType }
  | { ChangeZoneFloorArea: ChangeZoneFloorArea }
  | { ChangeZoneOccupants: ChangeZoneOccupants }
  | { ChangeZoneComfortCategory: ChangeZoneComfortCategory }
  | { ChangeZonePollutionClass: ChangeZonePollutionClass }
  | { ChangeZoneComfortModel: ChangeZoneComfortModel }
  | { ChangeZoneTOpWinter: ChangeZoneTOpWinter }
  | { ChangeZoneTOpSummer: ChangeZoneTOpSummer }
  | { ChangeZoneAirSpeed: ChangeZoneAirSpeed }
  | { ChangeZoneClothing: ChangeZoneClothing }
  | { ChangeZoneMetabolicRate: ChangeZoneMetabolicRate }
  | { ChangeZoneRh: ChangeZoneRh }
  | { ChangeZoneOutdoorAir: ChangeZoneOutdoorAir }
  | { ChangeZoneCo2: ChangeZoneCo2 }
  | { ChangeZoneIlluminance: ChangeZoneIlluminance }
  | { ChangeZoneNoise: ChangeZoneNoise }
  | { ChangeZoneVentSystemId: ChangeZoneVentSystemId }
  | { ChangeZoneTurbulence: ChangeZoneTurbulence }
  | { ChangeZoneVentMethod: ChangeZoneVentMethod }
  | { InsertVentSystem: InsertVentSystem }
  | { RemoveVentSystem: RemoveVentSystem }
  | { ChangeVentSystemType: ChangeVentSystemType }
  | { ChangeVentSfp: ChangeVentSfp }
  | { ChangeVentSfpClass: ChangeVentSfpClass }
  | { ChangeVentHeatRecovery: ChangeVentHeatRecovery }
  | { ChangeVentOdaClass: ChangeVentOdaClass }
  | { ChangeVentFilterSup: ChangeVentFilterSup }
  | { ChangeVentInspection: ChangeVentInspection }
  | { ChangeVentDuctClass: ChangeVentDuctClass }
  | { ChangeVentDuctLeakage: ChangeVentDuctLeakage }
  | { ChangeVentDesignAirflow: ChangeVentDesignAirflow };

export const parseDin16798Mutation: NormWireReader<Din16798Mutation> = normWireExternal<Din16798Mutation>({
  ChangeAnnex: parseChangeAnnex,
  ChangeThetaRm: parseChangeThetaRm,
  ChangeOutdoorCo2: parseChangeOutdoorCo2,
  ChangeEnvelopeN50: parseChangeEnvelopeN50,
  ChangeEnvelopeVolume: parseChangeEnvelopeVolume,
  ChangeCellarArea: parseChangeCellarArea,
  ChangeCellarVentilation: parseChangeCellarVentilation,
  ChangeNightSetback: parseChangeNightSetback,
  InsertZone: parseInsertZone,
  RemoveZone: parseRemoveZone,
  ChangeZoneUsageType: parseChangeZoneUsageType,
  ChangeZoneFloorArea: parseChangeZoneFloorArea,
  ChangeZoneOccupants: parseChangeZoneOccupants,
  ChangeZoneComfortCategory: parseChangeZoneComfortCategory,
  ChangeZonePollutionClass: parseChangeZonePollutionClass,
  ChangeZoneComfortModel: parseChangeZoneComfortModel,
  ChangeZoneTOpWinter: parseChangeZoneTOpWinter,
  ChangeZoneTOpSummer: parseChangeZoneTOpSummer,
  ChangeZoneAirSpeed: parseChangeZoneAirSpeed,
  ChangeZoneClothing: parseChangeZoneClothing,
  ChangeZoneMetabolicRate: parseChangeZoneMetabolicRate,
  ChangeZoneRh: parseChangeZoneRh,
  ChangeZoneOutdoorAir: parseChangeZoneOutdoorAir,
  ChangeZoneCo2: parseChangeZoneCo2,
  ChangeZoneIlluminance: parseChangeZoneIlluminance,
  ChangeZoneNoise: parseChangeZoneNoise,
  ChangeZoneVentSystemId: parseChangeZoneVentSystemId,
  ChangeZoneTurbulence: parseChangeZoneTurbulence,
  ChangeZoneVentMethod: parseChangeZoneVentMethod,
  InsertVentSystem: parseInsertVentSystem,
  RemoveVentSystem: parseRemoveVentSystem,
  ChangeVentSystemType: parseChangeVentSystemType,
  ChangeVentSfp: parseChangeVentSfp,
  ChangeVentSfpClass: parseChangeVentSfpClass,
  ChangeVentHeatRecovery: parseChangeVentHeatRecovery,
  ChangeVentOdaClass: parseChangeVentOdaClass,
  ChangeVentFilterSup: parseChangeVentFilterSup,
  ChangeVentInspection: parseChangeVentInspection,
  ChangeVentDuctClass: parseChangeVentDuctClass,
  ChangeVentDuctLeakage: parseChangeVentDuctLeakage,
  ChangeVentDesignAirflow: parseChangeVentDesignAirflow,
});
