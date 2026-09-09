/** 🪪️ En1990Diff contains document fields and canonical child identities. */
import { type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseEn1990Fields } from "../🟦️.ts";

export interface En1990Diff {
  /** 🗿️ @state artifact */
  gK?: number | null;
  /** 🗿️ @state artifact @child kind=s.stdio.semio */
  qK?: ArtifactChild | null;
  /** 🗿️ @state artifact */
  resistanceKn?: number | null;
  /** 🗿️ @state artifact */
  consequenceClass?: number | null;
  /** 🗿️ @state artifact */
  annex?: "En" | "De" | null;
  /** 🗿️ @state artifact */
  seismicAEdKn?: number | null;
}

/** 📥️ Decodes the exact En1990Diff wire contract. */
export function parseEn1990Diff(value: unknown, at = "$"): En1990Diff {
  return parseEn1990Fields(value, true, at) as En1990Diff;
}
