/** 🪪️ Din18599Diff — the sparse field delta of `Din18599Diff` in `🦀️.rs`: every field optional and nullable, whole lists as `{ values }`. */
import { type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDin18599Fields, type CoolingSystem, type DhwSystem, type EnvelopeElement, type HeatingSystem, type LightingSystem, type Renewables, type ThermalZone, type VentilationSystem } from "../🟦️.ts";

export interface Din18599Diff {
  /** 🗿️ @state artifact */
  buildingCategory?: string | null;
  /** 🗿️ @state artifact */
  attachment?: string | null;
  /** 🗿️ @state artifact */
  useClass?: string | null;
  /** 🗿️ @state artifact */
  method?: string | null;
  /** 🗿️ @state artifact */
  netFloorAreaM2?: number | null;
  /** 🗿️ @state artifact */
  heatedVolumeM3?: number | null;
  /** 🗿️ @state artifact */
  gegQpFactor?: number | null;
  /** 🗿️ @state artifact */
  deltaUWbWM2k?: number | null;
  /** 🗿️ @state artifact */
  automationClass?: string | null;
  /** 🗿️ @state artifact */
  zones?: { values: ThermalZone[] } | null;
  /** 🗿️ @state artifact */
  elements?: { values: EnvelopeElement[] } | null;
  /** 🗿️ @state artifact */
  heating?: HeatingSystem | null;
  /** 🗿️ @state artifact */
  dhw?: DhwSystem | null;
  /** 🗿️ @state artifact */
  ventilation?: VentilationSystem | null;
  /** 🗿️ @state artifact */
  cooling?: CoolingSystem | null;
  /** 🗿️ @state artifact */
  lighting?: LightingSystem | null;
  /** 🗿️ @state artifact */
  renewables?: Renewables | null;
  /** 🗿️ @state artifact @child kind=s.stdio.semio */
  climate?: ArtifactChild | null;
}

/** 📥️ Decodes the exact Din18599Diff wire contract. */
export function parseDin18599Diff(value: unknown, at = "$"): Din18599Diff {
  return parseDin18599Fields(value, true, at) as Din18599Diff;
}
