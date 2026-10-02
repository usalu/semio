/** 📸️ `Din16798Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din16798Snapshot {
  annex: "En" | "De";
  thetaRmC: number;
  outdoorCo2Ppm: number;
  zones: Din16798Zone[];
  ventSystems: Din16798VentSystem[];
  envelopeN50HInv: number;
  envelopeVolumeM3: number;
  cellarAreaM2: number;
  cellarVentilationM3H: number;
  nightSetbackK: number;
}

export interface Din16798Zone {
  id: string;
  name: string;
  usageType: string;
  floorAreaM2: number;
  occupants: number;
  comfortCategory: string;
  pollutionClass: string;
  comfortModel: string;
  tOpWinterC: number;
  tOpSummerC: number;
  airSpeedMS: number;
  clothingClo: number;
  metabolicRateMet: number;
  rhPercent: number;
  outdoorAirSuppliedM3H: number;
  co2Ppm: number;
  illuminanceLx: number;
  noiseDb: number;
  turbulenceIntensityPercent: number;
  ventMethod: string;
  ventSystemId: string;
}

export interface Din16798VentSystem {
  id: string;
  name: string;
  systemType: string;
  sfpWM3S: number;
  sfpRequiredClass: number;
  heatRecoveryEta: number;
  odaClass: string;
  filterSupClass: string;
  yearsSinceInspection: number;
  humidificationRequiredKgH: number;
  humidificationProvidedKgH: number;
  fanQVM3S: number;
  fanTRunH: number;
  ductClass: string;
  ductTestPressurePa: number;
  ductLeakageM3SM2: number;
  designAirflowM3H: number;
}

export type Din16798AnnexChoice = "En" | "De";

export const parseDin16798Snapshot: NormWireReader<Din16798Snapshot> = normWireObject<Din16798Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), thetaRmC: normWireRequired(normWireNumber), outdoorCo2Ppm: normWireRequired(normWireNumber), zones: normWireRequired(normWireArray(normWireRef(() => parseDin16798Zone))), ventSystems: normWireRequired(normWireArray(normWireRef(() => parseDin16798VentSystem))), envelopeN50HInv: normWireRequired(normWireNumber), envelopeVolumeM3: normWireRequired(normWireNumber), cellarAreaM2: normWireRequired(normWireNumber), cellarVentilationM3H: normWireRequired(normWireNumber), nightSetbackK: normWireRequired(normWireNumber) });
export const parseDin16798Zone: NormWireReader<Din16798Zone> = normWireObject<Din16798Zone>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), usageType: normWireRequired(normWireString), floorAreaM2: normWireRequired(normWireNumber), occupants: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), comfortCategory: normWireRequired(normWireString), pollutionClass: normWireRequired(normWireString), comfortModel: normWireRequired(normWireString), tOpWinterC: normWireRequired(normWireNumber), tOpSummerC: normWireRequired(normWireNumber), airSpeedMS: normWireRequired(normWireNumber), clothingClo: normWireRequired(normWireNumber), metabolicRateMet: normWireRequired(normWireNumber), rhPercent: normWireRequired(normWireNumber), outdoorAirSuppliedM3H: normWireRequired(normWireNumber), co2Ppm: normWireRequired(normWireNumber), illuminanceLx: normWireRequired(normWireNumber), noiseDb: normWireRequired(normWireNumber), turbulenceIntensityPercent: normWireRequired(normWireNumber), ventMethod: normWireRequired(normWireString), ventSystemId: normWireRequired(normWireString) });
export const parseDin16798VentSystem: NormWireReader<Din16798VentSystem> = normWireObject<Din16798VentSystem>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), systemType: normWireRequired(normWireString), sfpWM3S: normWireRequired(normWireNumber), sfpRequiredClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), heatRecoveryEta: normWireRequired(normWireNumber), odaClass: normWireRequired(normWireString), filterSupClass: normWireRequired(normWireString), yearsSinceInspection: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), humidificationRequiredKgH: normWireRequired(normWireNumber), humidificationProvidedKgH: normWireRequired(normWireNumber), fanQVM3S: normWireRequired(normWireNumber), fanTRunH: normWireRequired(normWireNumber), ductClass: normWireRequired(normWireString), ductTestPressurePa: normWireRequired(normWireNumber), ductLeakageM3SM2: normWireRequired(normWireNumber), designAirflowM3H: normWireRequired(normWireNumber) });
export const parseDin16798AnnexChoice: NormWireReader<Din16798AnnexChoice> = normWireLiteral("En", "De");

export * from "./🪶️sqlite/🟦️.ts";
