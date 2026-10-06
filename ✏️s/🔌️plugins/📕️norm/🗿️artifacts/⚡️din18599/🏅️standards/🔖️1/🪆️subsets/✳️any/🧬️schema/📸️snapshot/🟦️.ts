/** 📸️ `Din18599Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Din18599Snapshot {
  buildingCategory: Din18599BuildingCategory;
  attachment: Din18599Attachment;
  useClass: Din18599UseClass;
  method: Din18599CalculationMethod;
  netFloorAreaM2: number;
  heatedVolumeM3: number;
  gegQpFactor: number;
  deltaUWbWM2k: number;
  automationClass: Din18599AutomationClass;
  zones: ThermalZone[];
  elements: EnvelopeElement[];
  heating: { generationEfficiency: number; distributionEfficiency: number; storageEfficiency: number; transferEfficiency: number; energyCarrier: string; };
  dhw: { specificDemandKwhPersonA: number; storageLossKwhA: number; distributionLossKwhA: number; energyCarrier: string; };
  ventilation: { airflowM3H: number; heatRecoveryEta: number; fanPowerW: number; };
  cooling: { plant: { eer: number; energyCarrier: string; } | null; };
  lighting: { controlFactor: number; };
  renewables: { pvAreaM2: number; pvEfficiency: number; solarThermalKwhA: number; };
  climate: { thetaEC: number[]; gHWM2: number[]; };
  climateTable: { childId: string; target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }; };
}

export interface Renewables {
  pvAreaM2: number;
  pvEfficiency: number;
  solarThermalKwhA: number;
}

export interface CoolingSystem {
  plant: CoolingPlant | null;
}

export interface Din18599MonthlyClimate {
  thetaEC: number[];
  gHWM2: number[];
}

export interface VentilationSystem {
  airflowM3H: number;
  heatRecoveryEta: number;
  fanPowerW: number;
}

export type Din18599AutomationClass = "A" | "B" | "C" | "D";

export type Din18599BuildingCategory = "Residential" | "NonResidential";

export type Din18599UseClass = "Residential" | "Office" | "School";

export interface LightingSystem {
  controlFactor: number;
}

export interface HeatingSystem {
  generationEfficiency: number;
  distributionEfficiency: number;
  storageEfficiency: number;
  transferEfficiency: number;
  energyCarrier: string;
}

export interface ThermalZone {
  id: string;
  labelEn: string;
  labelDe: string;
  usageProfile: Din18599UsageProfile;
  areaM2: number;
  volumeM3: number;
  thetaIHeatC: number;
  thetaICoolC: number;
  occupants: number;
  internalGainsWM2: number;
  lightingPowerWM2: number;
}

export interface DhwSystem {
  specificDemandKwhPersonA: number;
  storageLossKwhA: number;
  distributionLossKwhA: number;
  energyCarrier: string;
}

export interface EnvelopeElement {
  id: string;
  labelEn: string;
  labelDe: string;
  kind: Din18599ElementKind;
  zoneId: string;
  areaM2: number;
  uValueWM2k: number;
  orientationDeg: number;
  tiltDeg: number;
  gValue: number;
  fc: number;
  adjacency: Din18599Adjacency;
}

export type Din18599CalculationMethod = "DetailedMonthly" | "Tabular";

export type Din18599Attachment = "Detached" | "SemiDetached" | "EndTerrace" | "MidTerrace";

export interface CoolingPlant {
  eer: number;
  energyCarrier: string;
}

export type Din18599UsageProfile = "WFH" | "Office" | "School";

export type Din18599ElementKind = "Wall" | "Roof" | "Floor" | "Door" | "Window";

export type Din18599Adjacency = "Outdoor" | "Ground" | "Unheated" | "Heated";

export const parseDin18599Snapshot: NormWireReader<Din18599Snapshot> = normWireObject<Din18599Snapshot>({ buildingCategory: normWireRequired(normWireRef(() => parseDin18599BuildingCategory)), attachment: normWireRequired(normWireRef(() => parseDin18599Attachment)), useClass: normWireRequired(normWireRef(() => parseDin18599UseClass)), method: normWireRequired(normWireRef(() => parseDin18599CalculationMethod)), netFloorAreaM2: normWireRequired(normWireNumber), heatedVolumeM3: normWireRequired(normWireNumber), gegQpFactor: normWireRequired(normWireNumber), deltaUWbWM2k: normWireRequired(normWireNumber), automationClass: normWireRequired(normWireRef(() => parseDin18599AutomationClass)), zones: normWireRequired(normWireArray(normWireRef(() => parseThermalZone))), elements: normWireRequired(normWireArray(normWireRef(() => parseEnvelopeElement))), heating: normWireRequired(normWireObject<{ generationEfficiency: number; distributionEfficiency: number; storageEfficiency: number; transferEfficiency: number; energyCarrier: string; }>({ generationEfficiency: normWireRequired(normWireNumber), distributionEfficiency: normWireRequired(normWireNumber), storageEfficiency: normWireRequired(normWireNumber), transferEfficiency: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) })), dhw: normWireRequired(normWireObject<{ specificDemandKwhPersonA: number; storageLossKwhA: number; distributionLossKwhA: number; energyCarrier: string; }>({ specificDemandKwhPersonA: normWireRequired(normWireNumber), storageLossKwhA: normWireRequired(normWireNumber), distributionLossKwhA: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) })), ventilation: normWireRequired(normWireObject<{ airflowM3H: number; heatRecoveryEta: number; fanPowerW: number; }>({ airflowM3H: normWireRequired(normWireNumber), heatRecoveryEta: normWireRequired(normWireNumber), fanPowerW: normWireRequired(normWireNumber) })), cooling: normWireRequired(normWireObject<{ plant: { eer: number; energyCarrier: string; } | null; }>({ plant: normWireRequired(normWireNullable(normWireObject<{ eer: number; energyCarrier: string; }>({ eer: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) }))) })), lighting: normWireRequired(normWireObject<{ controlFactor: number; }>({ controlFactor: normWireRequired(normWireNumber) })), renewables: normWireRequired(normWireObject<{ pvAreaM2: number; pvEfficiency: number; solarThermalKwhA: number; }>({ pvAreaM2: normWireRequired(normWireNumber), pvEfficiency: normWireRequired(normWireNumber), solarThermalKwhA: normWireRequired(normWireNumber) })), climate: normWireRequired(normWireObject<{ thetaEC: number[]; gHWM2: number[]; }>({ thetaEC: normWireRequired(normWireArray(normWireNumber, 12, 12)), gHWM2: normWireRequired(normWireArray(normWireRange(normWireNumber, {"minimum":0}), 12, 12)) })), climateTable: normWireRequired(normWireObject<{ childId: string; target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }; }>({ childId: normWireRequired(normWireString), target: normWireRequired(normWireObject<{ artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }>({ artifactId: normWireRequired(normWireString), dialect: normWireRequired(normWireObject<{ artifactKind: string; standard: string; subset: string; }>({ artifactKind: normWireRequired(normWireString), standard: normWireRequired(normWireString), subset: normWireRequired(normWireString) })) })) })) });
export const parseRenewables: NormWireReader<Renewables> = normWireObject<Renewables>({ pvAreaM2: normWireRequired(normWireNumber), pvEfficiency: normWireRequired(normWireNumber), solarThermalKwhA: normWireRequired(normWireNumber) });
export const parseCoolingSystem: NormWireReader<CoolingSystem> = normWireObject<CoolingSystem>({ plant: normWireDefault(normWireNullable(normWireRef(() => parseCoolingPlant)), () => null) });
export const parseDin18599MonthlyClimate: NormWireReader<Din18599MonthlyClimate> = normWireObject<Din18599MonthlyClimate>({ thetaEC: normWireRequired(normWireArray(normWireNumber, 12, 12)), gHWM2: normWireRequired(normWireArray(normWireRange(normWireNumber, {"minimum":0}), 12, 12)) });
export const parseVentilationSystem: NormWireReader<VentilationSystem> = normWireObject<VentilationSystem>({ airflowM3H: normWireRequired(normWireNumber), heatRecoveryEta: normWireRequired(normWireNumber), fanPowerW: normWireRequired(normWireNumber) });
export const parseDin18599AutomationClass: NormWireReader<Din18599AutomationClass> = normWireLiteral("A", "B", "C", "D");
export const parseDin18599BuildingCategory: NormWireReader<Din18599BuildingCategory> = normWireLiteral("Residential", "NonResidential");
export const parseDin18599UseClass: NormWireReader<Din18599UseClass> = normWireLiteral("Residential", "Office", "School");
export const parseLightingSystem: NormWireReader<LightingSystem> = normWireObject<LightingSystem>({ controlFactor: normWireRequired(normWireNumber) });
export const parseHeatingSystem: NormWireReader<HeatingSystem> = normWireObject<HeatingSystem>({ generationEfficiency: normWireRequired(normWireNumber), distributionEfficiency: normWireRequired(normWireNumber), storageEfficiency: normWireRequired(normWireNumber), transferEfficiency: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) });
export const parseThermalZone: NormWireReader<ThermalZone> = normWireObject<ThermalZone>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), usageProfile: normWireRequired(normWireRef(() => parseDin18599UsageProfile)), areaM2: normWireRequired(normWireNumber), volumeM3: normWireRequired(normWireNumber), thetaIHeatC: normWireRequired(normWireNumber), thetaICoolC: normWireRequired(normWireNumber), occupants: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), internalGainsWM2: normWireRequired(normWireNumber), lightingPowerWM2: normWireRequired(normWireNumber) });
export const parseDhwSystem: NormWireReader<DhwSystem> = normWireObject<DhwSystem>({ specificDemandKwhPersonA: normWireRequired(normWireNumber), storageLossKwhA: normWireRequired(normWireNumber), distributionLossKwhA: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) });
export const parseEnvelopeElement: NormWireReader<EnvelopeElement> = normWireObject<EnvelopeElement>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseDin18599ElementKind)), zoneId: normWireRequired(normWireString), areaM2: normWireRequired(normWireNumber), uValueWM2k: normWireRequired(normWireNumber), orientationDeg: normWireRequired(normWireNumber), tiltDeg: normWireRequired(normWireNumber), gValue: normWireRequired(normWireNumber), fc: normWireRequired(normWireNumber), adjacency: normWireRequired(normWireRef(() => parseDin18599Adjacency)) });
export const parseDin18599CalculationMethod: NormWireReader<Din18599CalculationMethod> = normWireLiteral("DetailedMonthly", "Tabular");
export const parseDin18599Attachment: NormWireReader<Din18599Attachment> = normWireLiteral("Detached", "SemiDetached", "EndTerrace", "MidTerrace");
export const parseCoolingPlant: NormWireReader<CoolingPlant> = normWireObject<CoolingPlant>({ eer: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) });
export const parseDin18599UsageProfile: NormWireReader<Din18599UsageProfile> = normWireLiteral("WFH", "Office", "School");
export const parseDin18599ElementKind: NormWireReader<Din18599ElementKind> = normWireLiteral("Wall", "Roof", "Floor", "Door", "Window");
export const parseDin18599Adjacency: NormWireReader<Din18599Adjacency> = normWireLiteral("Outdoor", "Ground", "Unheated", "Heated");
