/** 🪪️ Din18599Artifact contains document fields and canonical child identities. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface Din18599Artifact {
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
  /** 🗿️ @state artifact */
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

/** 🔎️ Validates the document field vocabulary shared by full snapshots and sparse diffs. */
export function parseDin18599Fields(value: unknown, partial: boolean, at = "$"): Partial<{ [K in keyof Din18599Artifact]: Din18599Artifact[K] | null }> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected document fields`);
  const row = value as Record<string, unknown>;
  const keys = ["useClass", "heatedAreaM2", "occupants", "hT", "hV", "climate", "internalGainsWM2", "solarGainsKwh", "systemLossesKwh", "renewableKwh", "annualLimitKwh", "energyCarrier", "referenceQPKwh"];
  if (Object.keys(row).some((key) => !keys.includes(key)) || (!partial && keys.some((key) => !Object.hasOwn(row, key)))) throw new Error(`${at}: invalid document fields`);
  const output: Record<string, unknown> = {};
  for (const [key, item] of Object.entries(row)) {
    if (partial && item === null) { output[key] = null; continue; }
    if (key === "climate") { output[key] = parseArtifactChild(item); continue; }
    if (key === "useClass") { if (item !== "Residential" && item !== "Office" && item !== "School") throw new Error(`${at}.useClass: invalid use class`); }
    else if (key === "energyCarrier") { if (typeof item !== "string") throw new Error(`${at}.energyCarrier: invalid carrier`); }
    else if (typeof item !== "number" || !Number.isFinite(item) || (key === "occupants" && (!Number.isInteger(item) || item < 0 || item > 4294967295))) throw new Error(`${at}.${key}: invalid numeric field`);
    output[key] = item;
  }
  return output;
}

/** 📥️ Decodes the exact Din18599Artifact wire contract. */
export function parseDin18599Artifact(value: unknown, at = "$"): Din18599Artifact {
  return parseDin18599Fields(value, false, at) as Din18599Artifact;
}
