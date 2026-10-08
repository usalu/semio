/** 🔺️ `Din4108Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ClimateZoneDe, type Din4108EnvelopeElement, type Din4108LayerDocument, type Din4108LayerSegment, type Din4108ThermalBridge, type Din4108ThermalZone, type Din4108ZoneWindow, parseDin4108ClimateZoneDe, parseDin4108EnvelopeElement, parseDin4108LayerDocument, parseDin4108LayerSegment, parseDin4108ThermalBridge, parseDin4108ThermalZone, parseDin4108ZoneWindow } from "../📸️snapshot/🟦️.ts";

export interface Din4108WindowPatch { orientation: (string) | null; inclinationDeg: (number) | null; areaM2: (number) | null; gValue: (number) | null; shadingFc: (number) | null; }
export interface Din4108WindowRemoval { id: string; index: number; }
export interface Din4108WindowInsertion { index: number; row: Din4108ZoneWindow; }
export interface Din4108WindowRelocation { id: string; from: number; to: number; }
export interface Din4108WindowModification { id: string; patch: Din4108WindowPatch; }
export interface Din4108WindowDelta { removed: Din4108WindowRemoval[]; inserted: Din4108WindowInsertion[]; moved: Din4108WindowRelocation[]; modified: Din4108WindowModification[]; }
export interface Din4108LayerPatch { materialId: (string) | null; thicknessM: (number) | null; lambda: (number) | null; mu: (number) | null; density: (number) | null; applicationType: (string) | null; compressiveClass: (string) | null; waterClass: (string) | null; tensileClass: (string) | null; acousticClass: (string) | null; segments: (Din4108LayerSegment[]) | null; }
export interface Din4108LayerRemoval { id: string; index: number; }
export interface Din4108LayerInsertion { index: number; row: Din4108LayerDocument; }
export interface Din4108LayerRelocation { id: string; from: number; to: number; }
export interface Din4108LayerModification { id: string; patch: Din4108LayerPatch; }
export interface Din4108LayerDelta { removed: Din4108LayerRemoval[]; inserted: Din4108LayerInsertion[]; moved: Din4108LayerRelocation[]; modified: Din4108LayerModification[]; }
export interface Din4108ZonePatch { floorAreaM2: (number) | null; heaviness: (string) | null; nightVentilation: (string) | null; windows: Din4108WindowDelta; }
export interface Din4108ZoneRemoval { id: string; index: number; }
export interface Din4108ZoneInsertion { index: number; row: Din4108ThermalZone; }
export interface Din4108ZoneRelocation { id: string; from: number; to: number; }
export interface Din4108ZoneModification { id: string; patch: Din4108ZonePatch; }
export interface Din4108ZoneDelta { removed: Din4108ZoneRemoval[]; inserted: Din4108ZoneInsertion[]; moved: Din4108ZoneRelocation[]; modified: Din4108ZoneModification[]; }
export interface Din4108ElementPatch { kind: (string) | null; zoneId: (string) | null; orientationDeg: (number) | null; inclinationDeg: (number) | null; adjacent: (string) | null; areaM2: (number) | null; deltaUG: (number) | null; deltaUF: (number) | null; deltaUR: (number) | null; layers: Din4108LayerDelta; }
export interface Din4108ElementRemoval { id: string; index: number; }
export interface Din4108ElementInsertion { index: number; row: Din4108EnvelopeElement; }
export interface Din4108ElementRelocation { id: string; from: number; to: number; }
export interface Din4108ElementModification { id: string; patch: Din4108ElementPatch; }
export interface Din4108ElementDelta { removed: Din4108ElementRemoval[]; inserted: Din4108ElementInsertion[]; moved: Din4108ElementRelocation[]; modified: Din4108ElementModification[]; }
export interface Din4108ThermalBridgePatch { psi: (number) | null; lengthM: (number) | null; bb2Type: (string) | null; }
export interface Din4108ThermalBridgeRemoval { id: string; index: number; }
export interface Din4108ThermalBridgeInsertion { index: number; row: Din4108ThermalBridge; }
export interface Din4108ThermalBridgeRelocation { id: string; from: number; to: number; }
export interface Din4108ThermalBridgeModification { id: string; patch: Din4108ThermalBridgePatch; }
export interface Din4108ThermalBridgeDelta { removed: Din4108ThermalBridgeRemoval[]; inserted: Din4108ThermalBridgeInsertion[]; moved: Din4108ThermalBridgeRelocation[]; modified: Din4108ThermalBridgeModification[]; }

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
export const parseDin4108WindowRemoval: NormWireReader<Din4108WindowRemoval> = normWireObject<Din4108WindowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) });
export const parseDin4108WindowInsertion: NormWireReader<Din4108WindowInsertion> = normWireObject<Din4108WindowInsertion>({ index: normWireRequired(normWireInteger), row: normWireRequired(parseDin4108ZoneWindow) });
export const parseDin4108WindowRelocation: NormWireReader<Din4108WindowRelocation> = normWireObject<Din4108WindowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) });
export const parseDin4108WindowModification: NormWireReader<Din4108WindowModification> = normWireObject<Din4108WindowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(parseDin4108WindowPatch) });
export const parseDin4108WindowDelta: NormWireReader<Din4108WindowDelta> = normWireObject<Din4108WindowDelta>({ removed: normWireDefault(normWireArray(parseDin4108WindowRemoval), () => []), inserted: normWireDefault(normWireArray(parseDin4108WindowInsertion), () => []), moved: normWireDefault(normWireArray(parseDin4108WindowRelocation), () => []), modified: normWireDefault(normWireArray(parseDin4108WindowModification), () => []) });
export const parseDin4108LayerPatch: NormWireReader<Din4108LayerPatch> = normWireObject<Din4108LayerPatch>({ materialId: normWireDefault(normWireNullable(normWireString), () => null), thicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), lambda: normWireDefault(normWireNullable(normWireNumber), () => null), mu: normWireDefault(normWireNullable(normWireNumber), () => null), density: normWireDefault(normWireNullable(normWireNumber), () => null), applicationType: normWireDefault(normWireNullable(normWireString), () => null), compressiveClass: normWireDefault(normWireNullable(normWireString), () => null), waterClass: normWireDefault(normWireNullable(normWireString), () => null), tensileClass: normWireDefault(normWireNullable(normWireString), () => null), acousticClass: normWireDefault(normWireNullable(normWireString), () => null), segments: normWireDefault(normWireNullable(normWireArray(parseDin4108LayerSegment)), () => null) });
export const parseDin4108LayerRemoval: NormWireReader<Din4108LayerRemoval> = normWireObject<Din4108LayerRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) });
export const parseDin4108LayerInsertion: NormWireReader<Din4108LayerInsertion> = normWireObject<Din4108LayerInsertion>({ index: normWireRequired(normWireInteger), row: normWireRequired(parseDin4108LayerDocument) });
export const parseDin4108LayerRelocation: NormWireReader<Din4108LayerRelocation> = normWireObject<Din4108LayerRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) });
export const parseDin4108LayerModification: NormWireReader<Din4108LayerModification> = normWireObject<Din4108LayerModification>({ id: normWireRequired(normWireString), patch: normWireRequired(parseDin4108LayerPatch) });
export const parseDin4108LayerDelta: NormWireReader<Din4108LayerDelta> = normWireObject<Din4108LayerDelta>({ removed: normWireDefault(normWireArray(parseDin4108LayerRemoval), () => []), inserted: normWireDefault(normWireArray(parseDin4108LayerInsertion), () => []), moved: normWireDefault(normWireArray(parseDin4108LayerRelocation), () => []), modified: normWireDefault(normWireArray(parseDin4108LayerModification), () => []) });
export const parseDin4108ZonePatch: NormWireReader<Din4108ZonePatch> = normWireObject<Din4108ZonePatch>({ floorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), heaviness: normWireDefault(normWireNullable(normWireString), () => null), nightVentilation: normWireDefault(normWireNullable(normWireString), () => null), windows: normWireDefault(normWireRef(() => parseDin4108WindowDelta), () => ({ removed: [], inserted: [], moved: [], modified: [] })) });
export const parseDin4108ZoneRemoval: NormWireReader<Din4108ZoneRemoval> = normWireObject<Din4108ZoneRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) });
export const parseDin4108ZoneInsertion: NormWireReader<Din4108ZoneInsertion> = normWireObject<Din4108ZoneInsertion>({ index: normWireRequired(normWireInteger), row: normWireRequired(parseDin4108ThermalZone) });
export const parseDin4108ZoneRelocation: NormWireReader<Din4108ZoneRelocation> = normWireObject<Din4108ZoneRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) });
export const parseDin4108ZoneModification: NormWireReader<Din4108ZoneModification> = normWireObject<Din4108ZoneModification>({ id: normWireRequired(normWireString), patch: normWireRequired(parseDin4108ZonePatch) });
export const parseDin4108ZoneDelta: NormWireReader<Din4108ZoneDelta> = normWireObject<Din4108ZoneDelta>({ removed: normWireDefault(normWireArray(parseDin4108ZoneRemoval), () => []), inserted: normWireDefault(normWireArray(parseDin4108ZoneInsertion), () => []), moved: normWireDefault(normWireArray(parseDin4108ZoneRelocation), () => []), modified: normWireDefault(normWireArray(parseDin4108ZoneModification), () => []) });
export const parseDin4108ElementPatch: NormWireReader<Din4108ElementPatch> = normWireObject<Din4108ElementPatch>({ kind: normWireDefault(normWireNullable(normWireString), () => null), zoneId: normWireDefault(normWireNullable(normWireString), () => null), orientationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), inclinationDeg: normWireDefault(normWireNullable(normWireNumber), () => null), adjacent: normWireDefault(normWireNullable(normWireString), () => null), areaM2: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUG: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUF: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUR: normWireDefault(normWireNullable(normWireNumber), () => null), layers: normWireDefault(normWireRef(() => parseDin4108LayerDelta), () => ({ removed: [], inserted: [], moved: [], modified: [] })) });
export const parseDin4108ElementRemoval: NormWireReader<Din4108ElementRemoval> = normWireObject<Din4108ElementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) });
export const parseDin4108ElementInsertion: NormWireReader<Din4108ElementInsertion> = normWireObject<Din4108ElementInsertion>({ index: normWireRequired(normWireInteger), row: normWireRequired(parseDin4108EnvelopeElement) });
export const parseDin4108ElementRelocation: NormWireReader<Din4108ElementRelocation> = normWireObject<Din4108ElementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) });
export const parseDin4108ElementModification: NormWireReader<Din4108ElementModification> = normWireObject<Din4108ElementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(parseDin4108ElementPatch) });
export const parseDin4108ElementDelta: NormWireReader<Din4108ElementDelta> = normWireObject<Din4108ElementDelta>({ removed: normWireDefault(normWireArray(parseDin4108ElementRemoval), () => []), inserted: normWireDefault(normWireArray(parseDin4108ElementInsertion), () => []), moved: normWireDefault(normWireArray(parseDin4108ElementRelocation), () => []), modified: normWireDefault(normWireArray(parseDin4108ElementModification), () => []) });
export const parseDin4108ThermalBridgePatch: NormWireReader<Din4108ThermalBridgePatch> = normWireObject<Din4108ThermalBridgePatch>({ psi: normWireDefault(normWireNullable(normWireNumber), () => null), lengthM: normWireDefault(normWireNullable(normWireNumber), () => null), bb2Type: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseDin4108ThermalBridgeRemoval: NormWireReader<Din4108ThermalBridgeRemoval> = normWireObject<Din4108ThermalBridgeRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireInteger) });
export const parseDin4108ThermalBridgeInsertion: NormWireReader<Din4108ThermalBridgeInsertion> = normWireObject<Din4108ThermalBridgeInsertion>({ index: normWireRequired(normWireInteger), row: normWireRequired(parseDin4108ThermalBridge) });
export const parseDin4108ThermalBridgeRelocation: NormWireReader<Din4108ThermalBridgeRelocation> = normWireObject<Din4108ThermalBridgeRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireInteger), to: normWireRequired(normWireInteger) });
export const parseDin4108ThermalBridgeModification: NormWireReader<Din4108ThermalBridgeModification> = normWireObject<Din4108ThermalBridgeModification>({ id: normWireRequired(normWireString), patch: normWireRequired(parseDin4108ThermalBridgePatch) });
export const parseDin4108ThermalBridgeDelta: NormWireReader<Din4108ThermalBridgeDelta> = normWireObject<Din4108ThermalBridgeDelta>({ removed: normWireDefault(normWireArray(parseDin4108ThermalBridgeRemoval), () => []), inserted: normWireDefault(normWireArray(parseDin4108ThermalBridgeInsertion), () => []), moved: normWireDefault(normWireArray(parseDin4108ThermalBridgeRelocation), () => []), modified: normWireDefault(normWireArray(parseDin4108ThermalBridgeModification), () => []) });
export const parseDin4108Diff: NormWireReader<Din4108Diff> = normWireObject<Din4108Diff>({ climateZone: normWireDefault(normWireNullable(parseDin4108ClimateZoneDe), () => null), usage: normWireDefault(normWireNullable(normWireString), () => null), tIntC: normWireDefault(normWireNullable(normWireNumber), () => null), rhInt: normWireDefault(normWireNullable(normWireNumber), () => null), hasMechanicalVentilation: normWireDefault(normWireNullable(normWireBoolean), () => null), airtightnessN50: normWireDefault(normWireNullable(normWireNumber), () => null), bb2DetailsConform: normWireDefault(normWireNullable(normWireBoolean), () => null), zones: normWireDefault(normWireRef(() => parseDin4108ZoneDelta), () => ({ removed: [], inserted: [], moved: [], modified: [] })), elements: normWireDefault(normWireRef(() => parseDin4108ElementDelta), () => ({ removed: [], inserted: [], moved: [], modified: [] })), thermalBridges: normWireDefault(normWireRef(() => parseDin4108ThermalBridgeDelta), () => ({ removed: [], inserted: [], moved: [], modified: [] })) });
