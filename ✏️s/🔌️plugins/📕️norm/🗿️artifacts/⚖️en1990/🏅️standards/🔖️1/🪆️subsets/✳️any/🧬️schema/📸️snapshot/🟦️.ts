/** 🪪️ En1990Snapshot contains document fields and canonical child identities. */
import { type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parseEn1990Artifact } from "../🟦️.ts";

export interface En1990Snapshot {
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

/** 📥️ Decodes the exact En1990Snapshot wire contract. */
export function parseEn1990Snapshot(value: unknown, at = "$"): En1990Snapshot {
  return parseEn1990Artifact(value, at);
}
