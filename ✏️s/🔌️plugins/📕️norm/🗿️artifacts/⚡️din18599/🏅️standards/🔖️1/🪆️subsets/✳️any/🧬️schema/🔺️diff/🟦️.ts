/** 🔺️ `Din18599Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din18599Attachment, type Din18599AutomationClass, type Din18599BuildingCategory, type Din18599CalculationMethod, type Din18599UseClass, type EnvelopeElement, parseDin18599Attachment, parseDin18599AutomationClass, parseDin18599BuildingCategory, parseDin18599CalculationMethod, parseDin18599UseClass, parseEnvelopeElement, parseThermalZone, type ThermalZone } from "../📸️snapshot/🟦️.ts";

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
  zones: { values: ThermalZone[]; } | null;
  /** @state artifact */
  elements: { values: EnvelopeElement[]; } | null;
  /** @state artifact */
  heating: { generationEfficiency: number; distributionEfficiency: number; storageEfficiency: number; transferEfficiency: number; energyCarrier: string; } | null;
  /** @state artifact */
  dhw: { specificDemandKwhPersonA: number; storageLossKwhA: number; distributionLossKwhA: number; energyCarrier: string; } | null;
  /** @state artifact */
  ventilation: { airflowM3H: number; heatRecoveryEta: number; fanPowerW: number; } | null;
  /** @state artifact */
  cooling: ({ plant: { eer: number; energyCarrier: string; } | null; }) | null;
  /** @state artifact */
  lighting: { controlFactor: number; } | null;
  /** @state artifact */
  renewables: { pvAreaM2: number; pvEfficiency: number; solarThermalKwhA: number; } | null;
  /** @state artifact */
  climate: { thetaEC: number[]; gHWM2: number[]; } | null;
  /** @state artifact */
  climateTable: { childId: string; target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }; } | null;
}

export const parseDin18599Diff: NormWireReader<Din18599Diff> = normWireObject<Din18599Diff>({ buildingCategory: normWireDefault(normWireNullable(parseDin18599BuildingCategory), () => null), attachment: normWireDefault(normWireNullable(parseDin18599Attachment), () => null), useClass: normWireDefault(normWireNullable(parseDin18599UseClass), () => null), method: normWireDefault(normWireNullable(parseDin18599CalculationMethod), () => null), netFloorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), heatedVolumeM3: normWireDefault(normWireNullable(normWireNumber), () => null), gegQpFactor: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUWbWM2k: normWireDefault(normWireNullable(normWireNumber), () => null), automationClass: normWireDefault(normWireNullable(parseDin18599AutomationClass), () => null), zones: normWireDefault(normWireNullable(normWireObject<{ values: ThermalZone[]; }>({ values: normWireRequired(normWireArray(parseThermalZone)) })), () => null), elements: normWireDefault(normWireNullable(normWireObject<{ values: EnvelopeElement[]; }>({ values: normWireRequired(normWireArray(parseEnvelopeElement)) })), () => null), heating: normWireDefault(normWireNullable(normWireObject<{ generationEfficiency: number; distributionEfficiency: number; storageEfficiency: number; transferEfficiency: number; energyCarrier: string; }>({ generationEfficiency: normWireRequired(normWireNumber), distributionEfficiency: normWireRequired(normWireNumber), storageEfficiency: normWireRequired(normWireNumber), transferEfficiency: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) })), () => null), dhw: normWireDefault(normWireNullable(normWireObject<{ specificDemandKwhPersonA: number; storageLossKwhA: number; distributionLossKwhA: number; energyCarrier: string; }>({ specificDemandKwhPersonA: normWireRequired(normWireNumber), storageLossKwhA: normWireRequired(normWireNumber), distributionLossKwhA: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) })), () => null), ventilation: normWireDefault(normWireNullable(normWireObject<{ airflowM3H: number; heatRecoveryEta: number; fanPowerW: number; }>({ airflowM3H: normWireRequired(normWireNumber), heatRecoveryEta: normWireRequired(normWireNumber), fanPowerW: normWireRequired(normWireNumber) })), () => null), cooling: normWireDefault(normWireNullable(normWireObject<{ plant: { eer: number; energyCarrier: string; } | null; }>({ plant: normWireRequired(normWireNullable(normWireObject<{ eer: number; energyCarrier: string; }>({ eer: normWireRequired(normWireNumber), energyCarrier: normWireRequired(normWireString) }))) })), () => null), lighting: normWireDefault(normWireNullable(normWireObject<{ controlFactor: number; }>({ controlFactor: normWireRequired(normWireNumber) })), () => null), renewables: normWireDefault(normWireNullable(normWireObject<{ pvAreaM2: number; pvEfficiency: number; solarThermalKwhA: number; }>({ pvAreaM2: normWireRequired(normWireNumber), pvEfficiency: normWireRequired(normWireNumber), solarThermalKwhA: normWireRequired(normWireNumber) })), () => null), climate: normWireDefault(normWireNullable(normWireObject<{ thetaEC: number[]; gHWM2: number[]; }>({ thetaEC: normWireRequired(normWireArray(normWireNumber, 12, 12)), gHWM2: normWireRequired(normWireArray(normWireRange(normWireNumber, {"minimum":0}), 12, 12)) })), () => null), climateTable: normWireDefault(normWireNullable(normWireObject<{ childId: string; target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }; }>({ childId: normWireRequired(normWireString), target: normWireRequired(normWireObject<{ artifactId: string; dialect: { artifactKind: string; standard: string; subset: string; }; }>({ artifactId: normWireRequired(normWireString), dialect: normWireRequired(normWireObject<{ artifactKind: string; standard: string; subset: string; }>({ artifactKind: normWireRequired(normWireString), standard: normWireRequired(normWireString), subset: normWireRequired(normWireString) })) })) })), () => null) });
