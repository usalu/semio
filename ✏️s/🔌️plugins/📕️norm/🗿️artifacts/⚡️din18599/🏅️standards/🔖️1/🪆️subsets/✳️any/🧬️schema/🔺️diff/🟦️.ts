/** 🔺️ `Din18599Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ArtifactChild, parseArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { type CoolingSystem, type DhwSystem, type EnvelopeElement, type HeatingSystem, type LightingSystem, parseCoolingSystem, parseDhwSystem, parseEnvelopeElement, parseHeatingSystem, parseLightingSystem, parseRenewables, parseThermalZone, parseVentilationSystem, type Renewables, type ThermalZone, type VentilationSystem } from "../📸️snapshot/🟦️.ts";

export interface Din18599Diff {
  /** @state artifact */
  buildingCategory: string | null;
  /** @state artifact */
  attachment: string | null;
  /** @state artifact */
  useClass: string | null;
  /** @state artifact */
  method: string | null;
  /** @state artifact */
  netFloorAreaM2: number | null;
  /** @state artifact */
  heatedVolumeM3: number | null;
  /** @state artifact */
  gegQpFactor: number | null;
  /** @state artifact */
  deltaUWbWM2k: number | null;
  /** @state artifact */
  automationClass: string | null;
  /** @state artifact */
  zones: { values: ThermalZone[]; } | null;
  /** @state artifact */
  elements: { values: EnvelopeElement[]; } | null;
  /** @state artifact */
  heating: HeatingSystem | null;
  /** @state artifact */
  dhw: DhwSystem | null;
  /** @state artifact */
  ventilation: VentilationSystem | null;
  /** @state artifact */
  cooling: CoolingSystem | null;
  /** @state artifact */
  lighting: LightingSystem | null;
  /** @state artifact */
  renewables: Renewables | null;
  /** @state artifact */
  climate: ArtifactChild | null;
}

export const parseDin18599Diff: NormWireReader<Din18599Diff> = normWireObject<Din18599Diff>({ buildingCategory: normWireDefault(normWireNullable(normWireString), () => null), attachment: normWireDefault(normWireNullable(normWireString), () => null), useClass: normWireDefault(normWireNullable(normWireString), () => null), method: normWireDefault(normWireNullable(normWireString), () => null), netFloorAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), heatedVolumeM3: normWireDefault(normWireNullable(normWireNumber), () => null), gegQpFactor: normWireDefault(normWireNullable(normWireNumber), () => null), deltaUWbWM2k: normWireDefault(normWireNullable(normWireNumber), () => null), automationClass: normWireDefault(normWireNullable(normWireString), () => null), zones: normWireDefault(normWireNullable(normWireObject<{ values: ThermalZone[]; }>({ values: normWireRequired(normWireArray(parseThermalZone)) })), () => null), elements: normWireDefault(normWireNullable(normWireObject<{ values: EnvelopeElement[]; }>({ values: normWireRequired(normWireArray(parseEnvelopeElement)) })), () => null), heating: normWireDefault(normWireNullable(parseHeatingSystem), () => null), dhw: normWireDefault(normWireNullable(parseDhwSystem), () => null), ventilation: normWireDefault(normWireNullable(parseVentilationSystem), () => null), cooling: normWireDefault(normWireNullable(parseCoolingSystem), () => null), lighting: normWireDefault(normWireNullable(parseLightingSystem), () => null), renewables: normWireDefault(normWireNullable(parseRenewables), () => null), climate: normWireDefault(normWireNullable(parseArtifactChild), () => null) });
