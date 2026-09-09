/** 🪪️ En1990Artifact contains document fields and canonical child identities. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface En1990Artifact {
  /** 🗿️ @state artifact */
  gK: number;
  /** 🗿️ @state artifact @child kind=s.stdio.semio */
  qK: ArtifactChild;
  /** 🗿️ @state artifact */
  resistanceKn: number;
  /** 🗿️ @state artifact */
  consequenceClass: number;
  /** 🗿️ @state artifact */
  annex: "En" | "De";
  /** 🗿️ @state artifact */
  seismicAEdKn: number;
}

/** 🔎️ Validates the document field vocabulary shared by full snapshots and sparse diffs. */
export function parseEn1990Fields(value: unknown, partial: boolean, at = "$"): Partial<{ [K in keyof En1990Artifact]: En1990Artifact[K] | null }> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected document fields`);
  const row = value as Record<string, unknown>;
  const keys = ["gK", "qK", "resistanceKn", "consequenceClass", "annex", "seismicAEdKn"];
  if (Object.keys(row).some((key) => !keys.includes(key)) || (!partial && keys.some((key) => !Object.hasOwn(row, key)))) throw new Error(`${at}: invalid document fields`);
  const output: Record<string, unknown> = {};
  for (const [key, item] of Object.entries(row)) {
    if (partial && item === null) { output[key] = null; continue; }
    if (key === "qK") { output[key] = parseArtifactChild(item); continue; }
    if (key === "annex") { if (item !== "En" && item !== "De") throw new Error(`${at}.annex: invalid annex`); }
    else if (typeof item !== "number" || !Number.isFinite(item) || (key === "consequenceClass" && (!Number.isInteger(item) || item < 0 || item > 255))) throw new Error(`${at}.${key}: invalid numeric field`);
    output[key] = item;
  }
  return output;
}

/** 📥️ Decodes the exact En1990Artifact wire contract. */
export function parseEn1990Artifact(value: unknown, at = "$"): En1990Artifact {
  return parseEn1990Fields(value, false, at) as En1990Artifact;
}
