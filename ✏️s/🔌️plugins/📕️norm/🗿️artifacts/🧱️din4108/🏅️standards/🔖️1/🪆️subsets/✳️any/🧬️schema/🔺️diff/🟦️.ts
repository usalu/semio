/** 🔺️ `Din4108Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ClimateZoneDe, type Din4108EnvelopeElement, type Din4108LayerDocument, type Din4108LayerSegment, type Din4108ThermalBridge, type Din4108ThermalZone, type Din4108ZoneWindow, parseDin4108ClimateZoneDe, parseDin4108EnvelopeElement, parseDin4108LayerDocument, parseDin4108LayerSegment, parseDin4108ThermalBridge, parseDin4108ThermalZone, parseDin4108ZoneWindow } from "../📸️snapshot/🟦️.ts";

export interface Din4108WindowPatch { orientation: (string) | null; inclinationDeg: (number) | null; areaM2: (number) | null; gValue: (number) | null; shadingFc: (number) | null; }
export interface Din4108WindowAddition { after: string | null; row: Din4108ZoneWindow; }
export interface Din4108WindowModification { key: string; patch: Din4108WindowPatch; }
export interface Din4108WindowDelta { removed: string[]; added: Din4108WindowAddition[]; modified: Din4108WindowModification[]; }
export interface Din4108LayerPatch { materialId: (string) | null; thicknessM: (number) | null; lambda: (number) | null; mu: (number) | null; density: (number) | null; applicationType: (string) | null; compressiveClass: (string) | null; waterClass: (string) | null; tensileClass: (string) | null; acousticClass: (string) | null; segments: (Din4108LayerSegment[]) | null; }
export interface Din4108LayerAddition { after: string | null; row: Din4108LayerDocument; }
export interface Din4108LayerModification { key: string; patch: Din4108LayerPatch; }
export interface Din4108LayerDelta { removed: string[]; added: Din4108LayerAddition[]; modified: Din4108LayerModification[]; }
export interface Din4108ZonePatch { floorAreaM2: (number) | null; heaviness: (string) | null; nightVentilation: (string) | null; windows: Din4108WindowDelta; }
export interface Din4108ZoneAddition { after: string | null; row: Din4108ThermalZone; }
export interface Din4108ZoneModification { key: string; patch: Din4108ZonePatch; }
export interface Din4108ZoneDelta { removed: string[]; added: Din4108ZoneAddition[]; modified: Din4108ZoneModification[]; }
export interface Din4108ElementPatch { kind: (string) | null; zoneId: (string) | null; orientationDeg: (number) | null; inclinationDeg: (number) | null; adjacent: (string) | null; areaM2: (number) | null; deltaUG: (number) | null; deltaUF: (number) | null; deltaUR: (number) | null; layers: Din4108LayerDelta; }
export interface Din4108ElementAddition { after: string | null; row: Din4108EnvelopeElement; }
export interface Din4108ElementModification { key: string; patch: Din4108ElementPatch; }
export interface Din4108ElementDelta { removed: string[]; added: Din4108ElementAddition[]; modified: Din4108ElementModification[]; }
export interface Din4108ThermalBridgePatch { psi: (number) | null; lengthM: (number) | null; bb2Type: (string) | null; }
export interface Din4108ThermalBridgeAddition { after: string | null; row: Din4108ThermalBridge; }
export interface Din4108ThermalBridgeModification { key: string; patch: Din4108ThermalBridgePatch; }
export interface Din4108ThermalBridgeDelta { removed: string[]; added: Din4108ThermalBridgeAddition[]; modified: Din4108ThermalBridgeModification[]; }

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
  zones: Din4108ZoneDelta;
  /** @state artifact */
  elements: Din4108ElementDelta;
  /** @state artifact */
  thermalBridges: Din4108ThermalBridgeDelta;
}

export const parseDin4108WindowPatch: NormWireReader<Din4108WindowPatch> = normWireObject<Din4108WindowPatch>({ orientation: normWireDefault(normWireNullable(normWireString), () => null), inclinationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), areaM2: normWireDefault(normWireNullable(normWireNumber), () => null), gValue: normWireDefault(normWireNullable(normWireNumber), () => null), shadingFc: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseDin4108WindowAddition: NormWireReader<Din4108WindowAddition> = normWireObject<Din4108WindowAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseDin4108ZoneWindow) });
export const parseDin4108WindowModification: NormWireReader<Din4108WindowModification> = normWireObject<Din4108WindowModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseDin4108WindowPatch) });
export const parseDin4108WindowDelta: NormWireReader<Din4108WindowDelta> = normWireObject<Din4108WindowDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseDin4108WindowAddition), () => []), modified: normWireDefault(normWireArray(parseDin4108WindowModification), () => []) });
export const parseDin4108LayerPatch: NormWireReader<Din4108LayerPatch> = normWireObject<Din4108LayerPatch>({ materialId: normWireDefault(normWireNullable(normWireString), () => null), thicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), lambda: normWireDefault(normWireNullable(normWireNumber), () => null), mu: normWireDefault(normWireNullable(normWireNumber), () => null), density: normWireDefault(normWireNullable(normWireNumber), () => null), applicationType: normWireDefault(normWireNullable(normWireString), () => null), compressiveClass: normWireDefault(normWireNullable(normWireString), () => null), waterClass: normWireDefault(normWireNullable(normWireString), () => null), tensileClass: normWireDefault(normWireNullable(normWireString), () => null), acousticClass: normWireDefault(normWireNullable(normWireString), () => null), segments: normWireDefault(normWireNullable(normWireArray(parseDin4108LayerSegment)), () => null) });
export const parseDin4108LayerAddition: NormWireReader<Din4108LayerAddition> = normWireObject<Din4108LayerAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseDin4108LayerDocument) });
export const parseDin4108LayerModification: NormWireReader<Din4108LayerModification> = normWireObject<Din4108LayerModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseDin4108LayerPatch) });
export const parseDin4108LayerDelta: NormWireReader<Din4108LayerDelta> = normWireObject<Din4108LayerDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseDin4108LayerAddition), () => []), modified: normWireDefault(normWireArray(parseDin4108LayerModification), () => []) });
export const parseDin4108ZonePatch: NormWireReader<Din4108ZonePatch> = normWireObject<Din4108ZonePatch>({ floorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), heaviness: normWireDefault(normWireNullable(normWireString), () => null), nightVentilation: normWireDefault(normWireNullable(normWireString), () => null), windows: normWireDefault(normWireRef(() => parseDin4108WindowDelta), () => ({ removed: [], added: [], modified: [] })) });
export const parseDin4108ZoneAddition: NormWireReader<Din4108ZoneAddition> = normWireObject<Din4108ZoneAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseDin4108ThermalZone) });
export const parseDin4108ZoneModification: NormWireReader<Din4108ZoneModification> = normWireObject<Din4108ZoneModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseDin4108ZonePatch) });
export const parseDin4108ZoneDelta: NormWireReader<Din4108ZoneDelta> = normWireObject<Din4108ZoneDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseDin4108ZoneAddition), () => []), modified: normWireDefault(normWireArray(parseDin4108ZoneModification), () => []) });
export const parseDin4108ElementPatch: NormWireReader<Din4108ElementPatch> = normWireObject<Din4108ElementPatch>({ kind: normWireDefault(normWireNullable(normWireString), () => null), zoneId: normWireDefault(normWireNullable(normWireString), () => null), orientationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), inclinationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), adjacent: normWireDefault(normWireNullable(normWireString), () => null), areaM2: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUG: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUF: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUR: normWireDefault(normWireNullable(normWireNumber), () => null), layers: normWireDefault(normWireRef(() => parseDin4108LayerDelta), () => ({ removed: [], added: [], modified: [] })) });
export const parseDin4108ElementAddition: NormWireReader<Din4108ElementAddition> = normWireObject<Din4108ElementAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseDin4108EnvelopeElement) });
export const parseDin4108ElementModification: NormWireReader<Din4108ElementModification> = normWireObject<Din4108ElementModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseDin4108ElementPatch) });
export const parseDin4108ElementDelta: NormWireReader<Din4108ElementDelta> = normWireObject<Din4108ElementDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseDin4108ElementAddition), () => []), modified: normWireDefault(normWireArray(parseDin4108ElementModification), () => []) });
export const parseDin4108ThermalBridgePatch: NormWireReader<Din4108ThermalBridgePatch> = normWireObject<Din4108ThermalBridgePatch>({ psi: normWireDefault(normWireNullable(normWireNumber), () => null), lengthM: normWireDefault(normWireNullable(normWireNumber), () => null), bb2Type: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseDin4108ThermalBridgeAddition: NormWireReader<Din4108ThermalBridgeAddition> = normWireObject<Din4108ThermalBridgeAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseDin4108ThermalBridge) });
export const parseDin4108ThermalBridgeModification: NormWireReader<Din4108ThermalBridgeModification> = normWireObject<Din4108ThermalBridgeModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseDin4108ThermalBridgePatch) });
export const parseDin4108ThermalBridgeDelta: NormWireReader<Din4108ThermalBridgeDelta> = normWireObject<Din4108ThermalBridgeDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseDin4108ThermalBridgeAddition), () => []), modified: normWireDefault(normWireArray(parseDin4108ThermalBridgeModification), () => []) });
export const parseDin4108Diff: NormWireReader<Din4108Diff> = normWireObject<Din4108Diff>({ climateZone: normWireDefault(normWireNullable(parseDin4108ClimateZoneDe), () => null), usage: normWireDefault(normWireNullable(normWireString), () => null), tIntC: normWireDefault(normWireNullable(normWireNumber), () => null), rhInt: normWireDefault(normWireNullable(normWireNumber), () => null), hasMechanicalVentilation: normWireDefault(normWireNullable(normWireBoolean), () => null), airtightnessN50: normWireDefault(normWireNullable(normWireNumber), () => null), bb2DetailsConform: normWireDefault(normWireNullable(normWireBoolean), () => null), zones: normWireDefault(normWireRef(() => parseDin4108ZoneDelta), () => ({ removed: [], added: [], modified: [] })), elements: normWireDefault(normWireRef(() => parseDin4108ElementDelta), () => ({ removed: [], added: [], modified: [] })), thermalBridges: normWireDefault(normWireRef(() => parseDin4108ThermalBridgeDelta), () => ({ removed: [], added: [], modified: [] })) });
