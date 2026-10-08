/** 🔺️ `Din18599Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CoolingPlant, type Din18599Adjacency, type Din18599Attachment, type Din18599AutomationClass, type Din18599BuildingCategory, type Din18599CalculationMethod, type Din18599ElementKind, type Din18599UsageProfile, type Din18599UseClass, type EnvelopeElement, parseCoolingPlant, parseDin18599Adjacency, parseDin18599Attachment, parseDin18599AutomationClass, parseDin18599BuildingCategory, parseDin18599CalculationMethod, parseDin18599ElementKind, parseDin18599UsageProfile, parseDin18599UseClass, parseEnvelopeElement, parseThermalZone, type ThermalZone } from "../📸️snapshot/🟦️.ts";

export interface Din18599Diff {
  /** @state artifact */
  buildingCategory: Din18599BuildingCategory | null;
  /** @state artifact */
  attachment: Din18599Attachment | null;
  /** @state artifact */
  useClass: Din18599UseClass | null;
  /** @state artifact */
  method: Din18599CalculationMethod | null;
  /** @state artifact */
  netFloorAreaM2: number | null;
  /** @state artifact */
  heatedVolumeM3: number | null;
  /** @state artifact */
  gegQpFactor: number | null;
  /** @state artifact */
  deltaUWbWM2k: number | null;
  /** @state artifact */
  automationClass: Din18599AutomationClass | null;
  /** @state artifact */
  zones: Din18599ZonesRows | null;
  /** @state artifact */
  elements: Din18599ElementsRows | null;
  /** @state artifact */
  heating: Din18599HeatingPatch | null;
  /** @state artifact */
  dhw: Din18599DhwPatch | null;
  /** @state artifact */
  ventilation: Din18599VentilationPatch | null;
  /** @state artifact */
  cooling: Din18599CoolingPatch | null;
  /** @state artifact */
  lighting: Din18599LightingPatch | null;
  /** @state artifact */
  renewables: Din18599RenewablesPatch | null;
  /** @state artifact */
  climate: Din18599ClimatePatch | null;
  /** @state artifact */
  climateTable: { childId: string; target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }; } | null;
}

export interface Din18599ClimatePatch {
  thetaEC: number[] | null;
  gHWM2: number[] | null;
}

export interface Din18599CoolingPatch {
  plant: Din18599CoolingPatchPlantValue | null;
}

export interface Din18599CoolingPatchPlantValue {
  value: CoolingPlant | null;
}

export interface Din18599DhwPatch {
  specificDemandKwhPersonA: number | null;
  storageLossKwhA: number | null;
  distributionLossKwhA: number | null;
  energyCarrier: string | null;
}

export interface Din18599ElementsPatch {
  id: string;
  labelEn: string | null;
  labelDe: string | null;
  kind: Din18599ElementKind | null;
  zoneId: string | null;
  areaM2: number | null;
  uValueWM2k: number | null;
  orientationDeg: number | null;
  tiltDeg: number | null;
  gValue: number | null;
  fc: number | null;
  adjacency: Din18599Adjacency | null;
}

export interface Din18599ElementsRows {
  added: EnvelopeElement[];
  removed: string[];
  modified: Din18599ElementsPatch[];
  order: string[] | null;
}

export interface Din18599HeatingPatch {
  generationEfficiency: number | null;
  distributionEfficiency: number | null;
  storageEfficiency: number | null;
  transferEfficiency: number | null;
  energyCarrier: string | null;
}

export interface Din18599LightingPatch {
  controlFactor: number | null;
}

export interface Din18599RenewablesPatch {
  pvAreaM2: number | null;
  pvEfficiency: number | null;
  solarThermalKwhA: number | null;
}

export interface Din18599VentilationPatch {
  airflowM3H: number | null;
  heatRecoveryEta: number | null;
  fanPowerW: number | null;
}

export interface Din18599ZonesPatch {
  id: string;
  labelEn: string | null;
  labelDe: string | null;
  usageProfile: Din18599UsageProfile | null;
  areaM2: number | null;
  volumeM3: number | null;
  thetaIHeatC: number | null;
  thetaICoolC: number | null;
  occupants: number | null;
  internalGainsWM2: number | null;
  lightingPowerWM2: number | null;
}

export interface Din18599ZonesRows {
  added: ThermalZone[];
  removed: string[];
  modified: Din18599ZonesPatch[];
  order: string[] | null;
}

export const parseDin18599Diff: NormWireReader<Din18599Diff> = normWireObject<Din18599Diff>({ buildingCategory: normWireDefault(normWireNullable(parseDin18599BuildingCategory), () => null), attachment: normWireDefault(normWireNullable(parseDin18599Attachment), () => null), useClass: normWireDefault(normWireNullable(parseDin18599UseClass), () => null), method: normWireDefault(normWireNullable(parseDin18599CalculationMethod), () => null), netFloorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), heatedVolumeM3: normWireDefault(normWireNullable(normWireNumber), () => null), gegQpFactor: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUWbWM2k: normWireDefault(normWireNullable(normWireNumber), () => null), automationClass: normWireDefault(normWireNullable(parseDin18599AutomationClass), () => null), zones: normWireDefault(normWireNullable(normWireRef(() => parseDin18599ZonesRows)), () => null), elements: normWireDefault(normWireNullable(normWireRef(() => parseDin18599ElementsRows)), () => null), heating: normWireDefault(normWireNullable(normWireRef(() => parseDin18599HeatingPatch)), () => null), dhw: normWireDefault(normWireNullable(normWireRef(() => parseDin18599DhwPatch)), () => null), ventilation: normWireDefault(normWireNullable(normWireRef(() => parseDin18599VentilationPatch)), () => null), cooling: normWireDefault(normWireNullable(normWireRef(() => parseDin18599CoolingPatch)), () => null), lighting: normWireDefault(normWireNullable(normWireRef(() => parseDin18599LightingPatch)), () => null), renewables: normWireDefault(normWireNullable(normWireRef(() => parseDin18599RenewablesPatch)), () => null), climate: normWireDefault(normWireNullable(normWireRef(() => parseDin18599ClimatePatch)), () => null), climateTable: normWireDefault(normWireNullable(normWireObject<{ childId: string; target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }; }>({ childId: normWireRequired(normWireString), target: normWireRequired(normWireObject<{ artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }>({ artifactId: normWireRequired(normWireString), dialect: normWireRequired(normWireObject<{ artifactKind: string; standard: string; subset: string; }>({ artifactKind: normWireRequired(normWireString), standard: normWireRequired(normWireString), subset: normWireRequired(normWireString) })) })) })), () => null) });
export const parseDin18599ClimatePatch: NormWireReader<Din18599ClimatePatch> = normWireObject<Din18599ClimatePatch>({ thetaEC: normWireRequired(normWireNullable(normWireArray(normWireNumber, 12, 12))), gHWM2: normWireRequired(normWireNullable(normWireArray(normWireRange(normWireNumber, {"minimum":0}), 12, 12))) });
export const parseDin18599CoolingPatch: NormWireReader<Din18599CoolingPatch> = normWireObject<Din18599CoolingPatch>({ plant: normWireRequired(normWireNullable(normWireRef(() => parseDin18599CoolingPatchPlantValue))) });
export const parseDin18599CoolingPatchPlantValue: NormWireReader<Din18599CoolingPatchPlantValue> = normWireObject<Din18599CoolingPatchPlantValue>({ value: normWireRequired(normWireNullable(parseCoolingPlant)) });
export const parseDin18599DhwPatch: NormWireReader<Din18599DhwPatch> = normWireObject<Din18599DhwPatch>({ specificDemandKwhPersonA: normWireRequired(normWireNullable(normWireNumber)), storageLossKwhA: normWireRequired(normWireNullable(normWireNumber)), distributionLossKwhA: normWireRequired(normWireNullable(normWireNumber)), energyCarrier: normWireRequired(normWireNullable(normWireString)) });
export const parseDin18599ElementsPatch: NormWireReader<Din18599ElementsPatch> = normWireObject<Din18599ElementsPatch>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireNullable(normWireString)), labelDe: normWireRequired(normWireNullable(normWireString)), kind: normWireRequired(normWireNullable(parseDin18599ElementKind)), zoneId: normWireRequired(normWireNullable(normWireString)), areaM2: normWireRequired(normWireNullable(normWireNumber)), uValueWM2k: normWireRequired(normWireNullable(normWireNumber)), orientationDeg: normWireRequired(normWireNullable(normWireNumber)), tiltDeg: normWireRequired(normWireNullable(normWireNumber)), gValue: normWireRequired(normWireNullable(normWireNumber)), fc: normWireRequired(normWireNullable(normWireNumber)), adjacency: normWireRequired(normWireNullable(parseDin18599Adjacency)) });
export const parseDin18599ElementsRows: NormWireReader<Din18599ElementsRows> = normWireObject<Din18599ElementsRows>({ added: normWireRequired(normWireArray(parseEnvelopeElement)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseDin18599ElementsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseDin18599HeatingPatch: NormWireReader<Din18599HeatingPatch> = normWireObject<Din18599HeatingPatch>({ generationEfficiency: normWireRequired(normWireNullable(normWireNumber)), distributionEfficiency: normWireRequired(normWireNullable(normWireNumber)), storageEfficiency: normWireRequired(normWireNullable(normWireNumber)), transferEfficiency: normWireRequired(normWireNullable(normWireNumber)), energyCarrier: normWireRequired(normWireNullable(normWireString)) });
export const parseDin18599LightingPatch: NormWireReader<Din18599LightingPatch> = normWireObject<Din18599LightingPatch>({ controlFactor: normWireRequired(normWireNullable(normWireNumber)) });
export const parseDin18599RenewablesPatch: NormWireReader<Din18599RenewablesPatch> = normWireObject<Din18599RenewablesPatch>({ pvAreaM2: normWireRequired(normWireNullable(normWireNumber)), pvEfficiency: normWireRequired(normWireNullable(normWireNumber)), solarThermalKwhA: normWireRequired(normWireNullable(normWireNumber)) });
export const parseDin18599VentilationPatch: NormWireReader<Din18599VentilationPatch> = normWireObject<Din18599VentilationPatch>({ airflowM3H: normWireRequired(normWireNullable(normWireNumber)), heatRecoveryEta: normWireRequired(normWireNullable(normWireNumber)), fanPowerW: normWireRequired(normWireNullable(normWireNumber)) });
export const parseDin18599ZonesPatch: NormWireReader<Din18599ZonesPatch> = normWireObject<Din18599ZonesPatch>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireNullable(normWireString)), labelDe: normWireRequired(normWireNullable(normWireString)), usageProfile: normWireRequired(normWireNullable(parseDin18599UsageProfile)), areaM2: normWireRequired(normWireNullable(normWireNumber)), volumeM3: normWireRequired(normWireNullable(normWireNumber)), thetaIHeatC: normWireRequired(normWireNullable(normWireNumber)), thetaICoolC: normWireRequired(normWireNullable(normWireNumber)), occupants: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), internalGainsWM2: normWireRequired(normWireNullable(normWireNumber)), lightingPowerWM2: normWireRequired(normWireNullable(normWireNumber)) });
export const parseDin18599ZonesRows: NormWireReader<Din18599ZonesRows> = normWireObject<Din18599ZonesRows>({ added: normWireRequired(normWireArray(parseThermalZone)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseDin18599ZonesPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
