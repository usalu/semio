/** 🪪️ Din18599Diff contains document fields and canonical child identities. */
import { type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDin18599Fields } from "../🟦️.ts";

export interface Din18599Diff {
  /** 🗿️ @state artifact */
  useClass?: "Residential" | "Office" | "School" | null;
  /** 🗿️ @state artifact */
  heatedAreaM2?: number | null;
  /** 🗿️ @state artifact */
  occupants?: number | null;
  /** 🗿️ @state artifact */
  hT?: number | null;
  /** 🗿️ @state artifact */
  hV?: number | null;
  /** 🗿️ @state artifact @child kind=s.stdio.semio */
  climate?: ArtifactChild | null;
  /** 🗿️ @state artifact */
  internalGainsWM2?: number | null;
  /** 🗿️ @state artifact */
  solarGainsKwh?: number | null;
  /** 🗿️ @state artifact */
  systemLossesKwh?: number | null;
  /** 🗿️ @state artifact */
  renewableKwh?: number | null;
  /** 🗿️ @state artifact */
  annualLimitKwh?: number | null;
  /** 🗿️ @state artifact */
  energyCarrier?: string | null;
  /** 🗿️ @state artifact */
  referenceQPKwh?: number | null;
}

/** 📥️ Decodes the exact Din18599Diff wire contract. */
export function parseDin18599Diff(value: unknown, at = "$"): Din18599Diff {
  return parseDin18599Fields(value, true, at) as Din18599Diff;
}
