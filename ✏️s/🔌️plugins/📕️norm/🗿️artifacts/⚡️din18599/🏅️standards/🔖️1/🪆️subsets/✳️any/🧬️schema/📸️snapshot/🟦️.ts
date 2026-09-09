/** 🪪️ Din18599Snapshot contains document fields and canonical child identities. */
import { type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseDin18599Artifact } from "../🟦️.ts";

export interface Din18599Snapshot {
  /** 🗿️ @state artifact */
  useClass: "Residential" | "Office" | "School";
  /** 🗿️ @state artifact */
  heatedAreaM2: number;
  /** 🗿️ @state artifact */
  occupants: number;
  /** 🗿️ @state artifact */
  hT: number;
  /** 🗿️ @state artifact */
  hV: number;
  /** 🗿️ @state artifact @child kind=s.stdio.semio */
  climate: ArtifactChild;
  /** 🗿️ @state artifact */
  internalGainsWM2: number;
  /** 🗿️ @state artifact */
  solarGainsKwh: number;
  /** 🗿️ @state artifact */
  systemLossesKwh: number;
  /** 🗿️ @state artifact */
  renewableKwh: number;
  /** 🗿️ @state artifact */
  annualLimitKwh: number;
  /** 🗿️ @state artifact */
  energyCarrier: string;
  /** 🗿️ @state artifact */
  referenceQPKwh: number;
}

/** 📥️ Decodes the exact Din18599Snapshot wire contract. */
export function parseDin18599Snapshot(value: unknown, at = "$"): Din18599Snapshot {
  return parseDin18599Artifact(value, at);
}
