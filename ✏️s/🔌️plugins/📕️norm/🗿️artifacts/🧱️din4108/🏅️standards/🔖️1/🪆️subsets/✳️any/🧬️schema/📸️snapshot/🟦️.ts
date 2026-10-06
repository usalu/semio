/** 📸️ `Din4108Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din4108Snapshot {
  climateZone: Din4108ClimateZoneDe;
  usage: string;
  tIntC: number;
  rhInt: number;
  hasMechanicalVentilation: boolean;
  airtightnessN50: number;
  bb2DetailsConform: boolean;
  zones: Din4108ThermalZone[];
  elements: Din4108EnvelopeElement[];
  thermalBridges: Din4108ThermalBridge[];
}

export interface Din4108LayerDocument {
  id: string;
  materialId: string;
  thicknessM: number;
  lambda: number;
  mu: number;
  density: number;
  applicationType: string;
  compressiveClass: string;
  waterClass: string;
  tensileClass: string;
  acousticClass: string;
  segments: Din4108LayerSegment[];
}

export interface Din4108ThermalZone {
  id: string;
  floorAreaM2: number;
  heaviness: string;
  nightVentilation: string;
  windows: Din4108ZoneWindow[];
}

export interface Din4108ThermalBridge {
  id: string;
  psi: number;
  lengthM: number;
  bb2Type: string;
}

export type Din4108ClimateZoneDe = "Zone1" | "Zone2" | "Zone3" | "Zone4";

export interface Din4108EnvelopeElement {
  id: string;
  kind: string;
  zoneId: string;
  orientationDeg: number;
  inclinationDeg: number;
  adjacent: string;
  areaM2: number;
  deltaUG: number;
  deltaUF: number;
  deltaUR: number;
  layers: Din4108LayerDocument[];
}

export interface Din4108ZoneWindow {
  id: string;
  orientation: string;
  inclinationDeg: number;
  areaM2: number;
  gValue: number;
  shadingFc: number;
}

export interface Din4108LayerSegment {
  id: string;
  materialId: string;
  fraction: number;
  lambda: number;
  mu: number;
  density: number;
}

export const parseDin4108Snapshot: NormWireReader<Din4108Snapshot> = normWireObject<Din4108Snapshot>({ climateZone: normWireRequired(normWireRef(() => parseDin4108ClimateZoneDe)), usage: normWireRequired(normWireString), tIntC: normWireRequired(normWireNumber), rhInt: normWireRequired(normWireNumber), hasMechanicalVentilation: normWireRequired(normWireBoolean), airtightnessN50: normWireRequired(normWireNumber), bb2DetailsConform: normWireRequired(normWireBoolean), zones: normWireRequired(normWireArray(normWireRef(() => parseDin4108ThermalZone))), elements: normWireRequired(normWireArray(normWireRef(() => parseDin4108EnvelopeElement))), thermalBridges: normWireRequired(normWireArray(normWireRef(() => parseDin4108ThermalBridge))) });
export const parseDin4108LayerDocument: NormWireReader<Din4108LayerDocument> = normWireObject<Din4108LayerDocument>({ id: normWireRequired(normWireString), materialId: normWireRequired(normWireString), thicknessM: normWireRequired(normWireNumber), lambda: normWireRequired(normWireNumber), mu: normWireRequired(normWireNumber), density: normWireRequired(normWireNumber), applicationType: normWireRequired(normWireString), compressiveClass: normWireRequired(normWireString), waterClass: normWireRequired(normWireString), tensileClass: normWireRequired(normWireString), acousticClass: normWireRequired(normWireString), segments: normWireRequired(normWireArray(normWireRef(() => parseDin4108LayerSegment))) });
export const parseDin4108ThermalZone: NormWireReader<Din4108ThermalZone> = normWireObject<Din4108ThermalZone>({ id: normWireRequired(normWireString), floorAreaM2: normWireRequired(normWireNumber), heaviness: normWireRequired(normWireString), nightVentilation: normWireRequired(normWireString), windows: normWireRequired(normWireArray(normWireRef(() => parseDin4108ZoneWindow))) });
export const parseDin4108ThermalBridge: NormWireReader<Din4108ThermalBridge> = normWireObject<Din4108ThermalBridge>({ id: normWireRequired(normWireString), psi: normWireRequired(normWireNumber), lengthM: normWireRequired(normWireNumber), bb2Type: normWireRequired(normWireString) });
export const parseDin4108ClimateZoneDe: NormWireReader<Din4108ClimateZoneDe> = normWireLiteral("Zone1", "Zone2", "Zone3", "Zone4");
export const parseDin4108EnvelopeElement: NormWireReader<Din4108EnvelopeElement> = normWireObject<Din4108EnvelopeElement>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), zoneId: normWireRequired(normWireString), orientationDeg: normWireRequired(normWireNumber), inclinationDeg: normWireRequired(normWireNumber), adjacent: normWireRequired(normWireString), areaM2: normWireRequired(normWireNumber), deltaUG: normWireRequired(normWireNumber), deltaUF: normWireRequired(normWireNumber), deltaUR: normWireRequired(normWireNumber), layers: normWireRequired(normWireArray(normWireRef(() => parseDin4108LayerDocument))) });
export const parseDin4108ZoneWindow: NormWireReader<Din4108ZoneWindow> = normWireObject<Din4108ZoneWindow>({ id: normWireRequired(normWireString), orientation: normWireRequired(normWireString), inclinationDeg: normWireRequired(normWireNumber), areaM2: normWireRequired(normWireNumber), gValue: normWireRequired(normWireNumber), shadingFc: normWireRequired(normWireNumber) });
export const parseDin4108LayerSegment: NormWireReader<Din4108LayerSegment> = normWireObject<Din4108LayerSegment>({ id: normWireRequired(normWireString), materialId: normWireRequired(normWireString), fraction: normWireRequired(normWireNumber), lambda: normWireRequired(normWireNumber), mu: normWireRequired(normWireNumber), density: normWireRequired(normWireNumber) });
