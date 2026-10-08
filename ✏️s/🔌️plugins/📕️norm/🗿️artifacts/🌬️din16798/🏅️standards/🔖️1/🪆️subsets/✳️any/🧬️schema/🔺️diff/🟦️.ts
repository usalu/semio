/** 🔺️ `Din16798Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din16798VentSystem, type Din16798Zone, parseDin16798VentSystem, parseDin16798Zone } from "../📸️snapshot/🟦️.ts";

export interface Din16798ZonePatch { name: (string) | null; usageType: (string) | null; floorAreaM2: (number) | null; occupants: (number) | null; comfortCategory: (string) | null; pollutionClass: (string) | null; comfortModel: (string) | null; tOpWinterC: (number) | null; tOpSummerC: (number) | null; airSpeedMS: (number) | null; clothingClo: (number) | null; metabolicRateMet: (number) | null; rhPercent: (number) | null; outdoorAirSuppliedM3H: (number) | null; co2Ppm: (number) | null; illuminanceLx: (number) | null; noiseDb: (number) | null; turbulenceIntensityPercent: (number) | null; ventMethod: (string) | null; ventSystemId: (string) | null; }
export interface Din16798ZoneAddition { after: string | null; row: Din16798Zone; }
export interface Din16798ZoneModification { key: string; patch: Din16798ZonePatch; }
export interface Din16798ZoneDelta { removed: string[]; added: Din16798ZoneAddition[]; modified: Din16798ZoneModification[]; }
export interface Din16798VentSystemPatch { name: (string) | null; systemType: (string) | null; sfpWM3S: (number) | null; sfpRequiredClass: (number) | null; heatRecoveryEta: (number) | null; odaClass: (string) | null; filterSupClass: (string) | null; yearsSinceInspection: (number) | null; humidificationRequiredKgH: (number) | null; humidificationProvidedKgH: (number) | null; fanQVM3S: (number) | null; fanTRunH: (number) | null; ductClass: (string) | null; ductTestPressurePa: (number) | null; ductLeakageM3SM2: (number) | null; designAirflowM3H: (number) | null; }
export interface Din16798VentSystemAddition { after: string | null; row: Din16798VentSystem; }
export interface Din16798VentSystemModification { key: string; patch: Din16798VentSystemPatch; }
export interface Din16798VentSystemDelta { removed: string[]; added: Din16798VentSystemAddition[]; modified: Din16798VentSystemModification[]; }

export interface Din16798Diff {
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  thetaRmC: number | null;
  /** @state artifact */
  outdoorCo2Ppm: number | null;
  /** @state artifact */
  envelopeN50HInv: number | null;
  /** @state artifact */
  envelopeVolumeM3: number | null;
  /** @state artifact */
  cellarAreaM2: number | null;
  /** @state artifact */
  cellarVentilationM3H: number | null;
  /** @state artifact */
  nightSetbackK: number | null;
  /** @state artifact */
  zones: Din16798ZoneDelta;
  /** @state artifact */
  ventSystems: Din16798VentSystemDelta;
}

export const parseDin16798ZonePatch: NormWireReader<Din16798ZonePatch> = normWireObject<Din16798ZonePatch>({ name: normWireDefault(normWireNullable(normWireString), () => null), usageType: normWireDefault(normWireNullable(normWireString), () => null), floorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), occupants: normWireDefault(normWireNullable(normWireInteger), () => null), comfortCategory: normWireDefault(normWireNullable(normWireString), () => null), pollutionClass: normWireDefault(normWireNullable(normWireString), () => null), comfortModel: normWireDefault(normWireNullable(normWireString), () => null), tOpWinterC: normWireDefault(normWireNullable(normWireNumber), () => null), tOpSummerC: normWireDefault(normWireNullable(normWireNumber), () => null), airSpeedMS: normWireDefault(normWireNullable(normWireNumber), () => null), clothingClo: normWireDefault(normWireNullable(normWireNumber), () => null), metabolicRateMet: normWireDefault(normWireNullable(normWireNumber), () => null), rhPercent: normWireDefault(normWireNullable(normWireNumber), () => null), outdoorAirSuppliedM3H: normWireDefault(normWireNullable(normWireNumber), () => null), co2Ppm: normWireDefault(normWireNullable(normWireNumber), () => null), illuminanceLx: normWireDefault(normWireNullable(normWireNumber), () => null), noiseDb: normWireDefault(normWireNullable(normWireNumber), () => null), turbulenceIntensityPercent: normWireDefault(normWireNullable(normWireNumber), () => null), ventMethod: normWireDefault(normWireNullable(normWireString), () => null), ventSystemId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseDin16798ZoneAddition: NormWireReader<Din16798ZoneAddition> = normWireObject<Din16798ZoneAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseDin16798Zone) });
export const parseDin16798ZoneModification: NormWireReader<Din16798ZoneModification> = normWireObject<Din16798ZoneModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseDin16798ZonePatch) });
export const parseDin16798ZoneDelta: NormWireReader<Din16798ZoneDelta> = normWireObject<Din16798ZoneDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseDin16798ZoneAddition), () => []), modified: normWireDefault(normWireArray(parseDin16798ZoneModification), () => []) });
export const parseDin16798VentSystemPatch: NormWireReader<Din16798VentSystemPatch> = normWireObject<Din16798VentSystemPatch>({ name: normWireDefault(normWireNullable(normWireString), () => null), systemType: normWireDefault(normWireNullable(normWireString), () => null), sfpWM3S: normWireDefault(normWireNullable(normWireNumber), () => null), sfpRequiredClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), heatRecoveryEta: normWireDefault(normWireNullable(normWireNumber), () => null), odaClass: normWireDefault(normWireNullable(normWireString), () => null), filterSupClass: normWireDefault(normWireNullable(normWireString), () => null), yearsSinceInspection: normWireDefault(normWireNullable(normWireInteger), () => null), humidificationRequiredKgH: normWireDefault(normWireNullable(normWireNumber), () => null), humidificationProvidedKgH: normWireDefault(normWireNullable(normWireNumber), () => null), fanQVM3S: normWireDefault(normWireNullable(normWireNumber), () => null), fanTRunH: normWireDefault(normWireNullable(normWireNumber), () => null), ductClass: normWireDefault(normWireNullable(normWireString), () => null), ductTestPressurePa: normWireDefault(normWireNullable(normWireNumber), () => null), ductLeakageM3SM2: normWireDefault(normWireNullable(normWireNumber), () => null), designAirflowM3H: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseDin16798VentSystemAddition: NormWireReader<Din16798VentSystemAddition> = normWireObject<Din16798VentSystemAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseDin16798VentSystem) });
export const parseDin16798VentSystemModification: NormWireReader<Din16798VentSystemModification> = normWireObject<Din16798VentSystemModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseDin16798VentSystemPatch) });
export const parseDin16798VentSystemDelta: NormWireReader<Din16798VentSystemDelta> = normWireObject<Din16798VentSystemDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseDin16798VentSystemAddition), () => []), modified: normWireDefault(normWireArray(parseDin16798VentSystemModification), () => []) });
export const parseDin16798Diff: NormWireReader<Din16798Diff> = normWireObject<Din16798Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), thetaRmC: normWireDefault(normWireNullable(normWireNumber), () => null), outdoorCo2Ppm: normWireDefault(normWireNullable(normWireNumber), () => null), envelopeN50HInv: normWireDefault(normWireNullable(normWireNumber), () => null), envelopeVolumeM3: normWireDefault(normWireNullable(normWireNumber), () => null), cellarAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), cellarVentilationM3H: normWireDefault(normWireNullable(normWireNumber), () => null), nightSetbackK: normWireDefault(normWireNullable(normWireNumber), () => null), zones: normWireDefault(normWireRef(() => parseDin16798ZoneDelta), () => ({ removed: [], added: [], modified: [] })), ventSystems: normWireDefault(normWireRef(() => parseDin16798VentSystemDelta), () => ({ removed: [], added: [], modified: [] })) });
