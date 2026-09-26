/** 🧬️ Din18599Artifact schema — artifact-lane fields matching Rust camelCase wire names. */

import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface ThermalZone {
  /** @state artifact */
  id: string;
  /** @state artifact */
  labelEn: string;
  /** @state artifact */
  labelDe: string;
  /** @state artifact */
  usageProfile: "WFH" | "Office" | "School";
  /** @state artifact */
  areaM2: number;
  /** @state artifact */
  volumeM3: number;
  /** @state artifact */
  thetaIHeatC: number;
  /** @state artifact */
  thetaICoolC: number;
  /** @state artifact */
  occupants: number;
  /** @state artifact */
  internalGainsWM2: number;
  /** @state artifact */
  lightingPowerWM2: number;
}

export interface EnvelopeElement {
  /** @state artifact */
  id: string;
  /** @state artifact */
  labelEn: string;
  /** @state artifact */
  labelDe: string;
  /** @state artifact */
  kind: string;
  /** @state artifact */
  zoneId: string;
  /** @state artifact */
  areaM2: number;
  /** @state artifact */
  uValueWM2k: number;
  /** @state artifact */
  orientationDeg: number;
  /** @state artifact */
  tiltDeg: number;
  /** @state artifact */
  gValue: number;
  /** @state artifact */
  fc: number;
  /** @state artifact */
  adjacency: string;
}

export interface HeatingSystem {
  /** @state artifact */
  generationEfficiency: number;
  /** @state artifact */
  distributionEfficiency: number;
  /** @state artifact */
  storageEfficiency: number;
  /** @state artifact */
  transferEfficiency: number;
  /** @state artifact */
  energyCarrier: string;
}

export interface DhwSystem {
  /** @state artifact */
  specificDemandKwhPersonA: number;
  /** @state artifact */
  storageLossKwhA: number;
  /** @state artifact */
  distributionLossKwhA: number;
  /** @state artifact */
  energyCarrier: string;
}

export interface VentilationSystem {
  /** @state artifact */
  airflowM3H: number;
  /** @state artifact */
  heatRecoveryEta: number;
  /** @state artifact */
  fanPowerW: number;
}

export interface CoolingPlant {
  /** @state artifact */
  eer: number;
  /** @state artifact */
  energyCarrier: string;
}
export interface CoolingSystem {
  /** @state artifact */
  plant: CoolingPlant | null;
}; eer: number; energyCarrier: string };

export interface LightingSystem {
  /** @state artifact */
  controlFactor: number;
}

export interface Renewables {
  /** @state artifact */
  pvAreaM2: number;
  /** @state artifact */
  pvEfficiency: number;
  /** @state artifact */
  solarThermalKwhA: number;
}

export interface Din18599Artifact {
  /** @state artifact */
  buildingCategory: string;
  /** @state artifact */
  attachment: string;
  /** @state artifact */
  useClass: string;
  /** @state artifact */
  method: string;
  /** @state artifact */
  netFloorAreaM2: number;
  /** @state artifact */
  heatedVolumeM3: number;
  /** @state artifact */
  gegQpFactor: number;
  /** @state artifact */
  deltaUWbWM2k: number;
  /** @state artifact */
  automationClass: string;
  /** @state artifact */
  zones: ThermalZone[];
  /** @state artifact */
  elements: EnvelopeElement[];
  /** @state artifact */
  heating: HeatingSystem;
  /** @state artifact */
  dhw: DhwSystem;
  /** @state artifact */
  ventilation: VentilationSystem;
  /** @state artifact */
  cooling: CoolingSystem;
  /** @state artifact */
  lighting: LightingSystem;
  /** @state artifact */
  renewables: Renewables;
  /** @state artifact @child kind=s.stdio.semio */
  climate: ArtifactChild;
}

/** 📥️ Decodes Din18599 artifact JSON (camelCase). */
export function parseDin18599Artifact(value: unknown, _at = "$"): Din18599Artifact {
  return value as Din18599Artifact;
}

export { parseArtifactChild };
