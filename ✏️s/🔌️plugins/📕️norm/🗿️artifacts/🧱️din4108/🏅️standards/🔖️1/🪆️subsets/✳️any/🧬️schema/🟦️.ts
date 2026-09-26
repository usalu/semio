/** 🧬️ Din4108 snapshot schema — complete envelope subject. */

export interface Din4108LayerSegment {
  id: string;
  materialId: string;
  fraction: number;
  lambda: number;
  mu: number;
  density: number;
}

export interface Din4108LayerDocument {
  id: string;
  materialId: string;
  thicknessM: number;
  lambda: number;
  mu: number;
  density: number;
  segments: Din4108LayerSegment[];
}

export interface Din4108ZoneWindow {
  id: string;
  orientation: string;
  areaM2: number;
  gValue: number;
  shadingFc: number;
}

export interface Din4108ThermalZone {
  id: string;
  floorAreaM2: number;
  heaviness: string;
  nightVentilation: string;
  windows: Din4108ZoneWindow[];
}

export interface Din4108EnvelopeElement {
  id: string;
  kind: string;
  zoneId: string;
  orientationDeg: number;
  inclinationDeg: number;
  adjacent: string;
  areaM2: number;
  layers: Din4108LayerDocument[];
}

export interface Din4108ThermalBridge {
  id: string;
  psi: number;
  lengthM: number;
  bb2Type: string;
}

export interface Din4108Artifact {
  /** @state artifact */
  climateZone: string;
  /** @state artifact */
  usage: string;
  /** @state artifact */
  tIntC: number;
  /** @state artifact */
  rhInt: number;
  /** @state artifact */
  hasMechanicalVentilation: boolean;
  /** @state artifact */
  airtightnessN50: number;
  /** @state artifact */
  bb2DetailsConform: boolean;
  /** @state artifact */
  zones: Din4108ThermalZone[];
  /** @state artifact */
  elements: Din4108EnvelopeElement[];
  /** @state artifact */
  thermalBridges: Din4108ThermalBridge[];
}

export type Din4108Snapshot = Din4108Artifact;
