/** 🔺️ `Din4108Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ClimateZoneDe, type Din4108EnvelopeElement, type Din4108LayerDocument, type Din4108ThermalBridge, type Din4108ThermalZone, type Din4108ZoneWindow, parseDin4108ClimateZoneDe, parseDin4108EnvelopeElement, parseDin4108LayerDocument, parseDin4108ThermalBridge, parseDin4108ThermalZone, parseDin4108ZoneWindow } from "../📸️snapshot/🟦️.ts";

export type Din4108RowOp = "Insert" | "Remove" | "Replace" | "Patch";
export interface Din4108ZoneEdit { op: Din4108RowOp; index: number; id: string; value: Din4108ThermalZone | null; patch: Din4108ZonePatch | null; }
export interface Din4108ElementEdit { op: Din4108RowOp; index: number; id: string; value: Din4108EnvelopeElement | null; patch: Din4108ElementPatch | null; }
export interface Din4108ThermalBridgeEdit { op: Din4108RowOp; index: number; id: string; value: Din4108ThermalBridge | null; patch: Din4108ThermalBridgePatch | null; }
export interface Din4108WindowEdit { op: Din4108RowOp; index: number; id: string; value: Din4108ZoneWindow | null; patch: Din4108WindowPatch | null; }
export interface Din4108LayerEdit { op: Din4108RowOp; index: number; id: string; value: Din4108LayerDocument | null; patch: Din4108LayerPatch | null; }
export interface Din4108ZonePatch { floorAreaM2: number | null; heaviness: string | null; nightVentilation: string | null; windows: Din4108WindowEdit[]; }
export interface Din4108ElementPatch { kind: string | null; orientationDeg: number | null; inclinationDeg: number | null; adjacent: string | null; areaM2: number | null; deltaUG: number | null; deltaUF: number | null; deltaUR: number | null; layers: Din4108LayerEdit[]; }
export interface Din4108ThermalBridgePatch { psi: number | null; lengthM: number | null; bb2Type: string | null; }
export interface Din4108WindowPatch { orientation: string | null; inclinationDeg: number | null; areaM2: number | null; gValue: number | null; shadingFc: number | null; }
export interface Din4108LayerPatch { materialId: string | null; thicknessM: number | null; lambda: number | null; mu: number | null; applicationType: string | null; compressiveClass: string | null; }

export interface Din4108Diff {
  /** @state artifact */
  climateZone: Din4108ClimateZoneDe | null;
  /** @state artifact */
  usage: string | null;
  /** @state artifact */
  tIntC: number | null;
  /** @state artifact */
  rhInt: number | null;
  /** @state artifact */
  hasMechanicalVentilation: boolean | null;
  /** @state artifact */
  airtightnessN50: number | null;
  /** @state artifact */
  bb2DetailsConform: boolean | null;
  /** @state artifact */
  zones: Din4108ZoneEdit[];
  /** @state artifact */
  elements: Din4108ElementEdit[];
  /** @state artifact */
  thermalBridges: Din4108ThermalBridgeEdit[];
}

export const parseDin4108RowOp: NormWireReader<Din4108RowOp> = normWireLiteral("Insert", "Remove", "Replace", "Patch");
export const parseDin4108ZoneEdit: NormWireReader<Din4108ZoneEdit> = normWireObject<Din4108ZoneEdit>({ op: normWireRequired(parseDin4108RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseDin4108ThermalZone)), patch: normWireRequired(normWireNullable(parseDin4108ZonePatch)) });
export const parseDin4108ElementEdit: NormWireReader<Din4108ElementEdit> = normWireObject<Din4108ElementEdit>({ op: normWireRequired(parseDin4108RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseDin4108EnvelopeElement)), patch: normWireRequired(normWireNullable(parseDin4108ElementPatch)) });
export const parseDin4108ThermalBridgeEdit: NormWireReader<Din4108ThermalBridgeEdit> = normWireObject<Din4108ThermalBridgeEdit>({ op: normWireRequired(parseDin4108RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseDin4108ThermalBridge)), patch: normWireRequired(normWireNullable(parseDin4108ThermalBridgePatch)) });
export const parseDin4108WindowEdit: NormWireReader<Din4108WindowEdit> = normWireObject<Din4108WindowEdit>({ op: normWireRequired(parseDin4108RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseDin4108ZoneWindow)), patch: normWireRequired(normWireNullable(parseDin4108WindowPatch)) });
export const parseDin4108LayerEdit: NormWireReader<Din4108LayerEdit> = normWireObject<Din4108LayerEdit>({ op: normWireRequired(parseDin4108RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseDin4108LayerDocument)), patch: normWireRequired(normWireNullable(parseDin4108LayerPatch)) });
export const parseDin4108ZonePatch: NormWireReader<Din4108ZonePatch> = normWireObject<Din4108ZonePatch>({ floorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), heaviness: normWireDefault(normWireNullable(normWireString), () => null), nightVentilation: normWireDefault(normWireNullable(normWireString), () => null), windows: normWireDefault(normWireArray(parseDin4108WindowEdit), () => []) });
export const parseDin4108ElementPatch: NormWireReader<Din4108ElementPatch> = normWireObject<Din4108ElementPatch>({ kind: normWireDefault(normWireNullable(normWireString), () => null), orientationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), inclinationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), adjacent: normWireDefault(normWireNullable(normWireString), () => null), areaM2: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUG: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUF: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUR: normWireDefault(normWireNullable(normWireNumber), () => null), layers: normWireDefault(normWireArray(parseDin4108LayerEdit), () => []) });
export const parseDin4108ThermalBridgePatch: NormWireReader<Din4108ThermalBridgePatch> = normWireObject<Din4108ThermalBridgePatch>({ psi: normWireDefault(normWireNullable(normWireNumber), () => null), lengthM: normWireDefault(normWireNullable(normWireNumber), () => null), bb2Type: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseDin4108WindowPatch: NormWireReader<Din4108WindowPatch> = normWireObject<Din4108WindowPatch>({ orientation: normWireDefault(normWireNullable(normWireString), () => null), inclinationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), areaM2: normWireDefault(normWireNullable(normWireNumber), () => null), gValue: normWireDefault(normWireNullable(normWireNumber), () => null), shadingFc: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseDin4108LayerPatch: NormWireReader<Din4108LayerPatch> = normWireObject<Din4108LayerPatch>({ materialId: normWireDefault(normWireNullable(normWireString), () => null), thicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), lambda: normWireDefault(normWireNullable(normWireNumber), () => null), mu: normWireDefault(normWireNullable(normWireNumber), () => null), applicationType: normWireDefault(normWireNullable(normWireString), () => null), compressiveClass: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseDin4108Diff: NormWireReader<Din4108Diff> = normWireObject<Din4108Diff>({ climateZone: normWireDefault(normWireNullable(parseDin4108ClimateZoneDe), () => null), usage: normWireDefault(normWireNullable(normWireString), () => null), tIntC: normWireDefault(normWireNullable(normWireNumber), () => null), rhInt: normWireDefault(normWireNullable(normWireNumber), () => null), hasMechanicalVentilation: normWireDefault(normWireNullable(normWireBoolean), () => null), airtightnessN50: normWireDefault(normWireNullable(normWireNumber), () => null), bb2DetailsConform: normWireDefault(normWireNullable(normWireBoolean), () => null), zones: normWireDefault(normWireArray(parseDin4108ZoneEdit), () => []), elements: normWireDefault(normWireArray(parseDin4108ElementEdit), () => []), thermalBridges: normWireDefault(normWireArray(parseDin4108ThermalBridgeEdit), () => []) });
