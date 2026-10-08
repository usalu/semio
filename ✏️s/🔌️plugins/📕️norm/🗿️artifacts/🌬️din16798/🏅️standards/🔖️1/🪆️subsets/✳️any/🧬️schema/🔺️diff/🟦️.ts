/** 🔺️ `Din16798Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din16798VentSystem, type Din16798Zone, parseDin16798VentSystem, parseDin16798Zone } from "../📸️snapshot/🟦️.ts";

export type Din16798RowOp = "Insert" | "Remove" | "Replace" | "Patch";
export interface Din16798ZoneEdit { op: Din16798RowOp; index: number; id: string; value: Din16798Zone | null; patch: Din16798ZonePatch | null; }
export interface Din16798VentSystemEdit { op: Din16798RowOp; index: number; id: string; value: Din16798VentSystem | null; patch: Din16798VentSystemPatch | null; }
export interface Din16798ZonePatch { usageType: string | null; floorAreaM2: number | null; occupants: number | null; comfortCategory: string | null; pollutionClass: string | null; comfortModel: string | null; tOpWinterC: number | null; tOpSummerC: number | null; airSpeedMS: number | null; clothingClo: number | null; metabolicRateMet: number | null; rhPercent: number | null; outdoorAirSuppliedM3H: number | null; co2Ppm: number | null; illuminanceLx: number | null; noiseDb: number | null; turbulenceIntensityPercent: number | null; ventMethod: string | null; ventSystemId: string | null; }
export interface Din16798VentSystemPatch { systemType: string | null; sfpWM3S: number | null; sfpRequiredClass: number | null; heatRecoveryEta: number | null; odaClass: string | null; filterSupClass: string | null; yearsSinceInspection: number | null; ductClass: string | null; ductLeakageM3SM2: number | null; designAirflowM3H: number | null; }

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
  zones: Din16798ZoneEdit[];
  /** @state artifact */
  ventSystems: Din16798VentSystemEdit[];
}

export const parseDin16798RowOp: NormWireReader<Din16798RowOp> = normWireLiteral("Insert", "Remove", "Replace", "Patch");
export const parseDin16798ZoneEdit: NormWireReader<Din16798ZoneEdit> = normWireObject<Din16798ZoneEdit>({ op: normWireRequired(parseDin16798RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseDin16798Zone)), patch: normWireRequired(normWireNullable(parseDin16798ZonePatch)) });
export const parseDin16798VentSystemEdit: NormWireReader<Din16798VentSystemEdit> = normWireObject<Din16798VentSystemEdit>({ op: normWireRequired(parseDin16798RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseDin16798VentSystem)), patch: normWireRequired(normWireNullable(parseDin16798VentSystemPatch)) });
export const parseDin16798ZonePatch: NormWireReader<Din16798ZonePatch> = normWireObject<Din16798ZonePatch>({ usageType: normWireDefault(normWireNullable(normWireString), () => null), floorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), occupants: normWireDefault(normWireNullable(normWireInteger), () => null), comfortCategory: normWireDefault(normWireNullable(normWireString), () => null), pollutionClass: normWireDefault(normWireNullable(normWireString), () => null), comfortModel: normWireDefault(normWireNullable(normWireString), () => null), tOpWinterC: normWireDefault(normWireNullable(normWireNumber), () => null), tOpSummerC: normWireDefault(normWireNullable(normWireNumber), () => null), airSpeedMS: normWireDefault(normWireNullable(normWireNumber), () => null), clothingClo: normWireDefault(normWireNullable(normWireNumber), () => null), metabolicRateMet: normWireDefault(normWireNullable(normWireNumber), () => null), rhPercent: normWireDefault(normWireNullable(normWireNumber), () => null), outdoorAirSuppliedM3H: normWireDefault(normWireNullable(normWireNumber), () => null), co2Ppm: normWireDefault(normWireNullable(normWireNumber), () => null), illuminanceLx: normWireDefault(normWireNullable(normWireNumber), () => null), noiseDb: normWireDefault(normWireNullable(normWireNumber), () => null), turbulenceIntensityPercent: normWireDefault(normWireNullable(normWireNumber), () => null), ventMethod: normWireDefault(normWireNullable(normWireString), () => null), ventSystemId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseDin16798VentSystemPatch: NormWireReader<Din16798VentSystemPatch> = normWireObject<Din16798VentSystemPatch>({ systemType: normWireDefault(normWireNullable(normWireString), () => null), sfpWM3S: normWireDefault(normWireNullable(normWireNumber), () => null), sfpRequiredClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), heatRecoveryEta: normWireDefault(normWireNullable(normWireNumber), () => null), odaClass: normWireDefault(normWireNullable(normWireString), () => null), filterSupClass: normWireDefault(normWireNullable(normWireString), () => null), yearsSinceInspection: normWireDefault(normWireNullable(normWireInteger), () => null), ductClass: normWireDefault(normWireNullable(normWireString), () => null), ductLeakageM3SM2: normWireDefault(normWireNullable(normWireNumber), () => null), designAirflowM3H: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseDin16798Diff: NormWireReader<Din16798Diff> = normWireObject<Din16798Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), thetaRmC: normWireDefault(normWireNullable(normWireNumber), () => null), outdoorCo2Ppm: normWireDefault(normWireNullable(normWireNumber), () => null), envelopeN50HInv: normWireDefault(normWireNullable(normWireNumber), () => null), envelopeVolumeM3: normWireDefault(normWireNullable(normWireNumber), () => null), cellarAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), cellarVentilationM3H: normWireDefault(normWireNullable(normWireNumber), () => null), nightSetbackK: normWireDefault(normWireNullable(normWireNumber), () => null), zones: normWireDefault(normWireArray(parseDin16798ZoneEdit), () => []), ventSystems: normWireDefault(normWireArray(parseDin16798VentSystemEdit), () => []) });
