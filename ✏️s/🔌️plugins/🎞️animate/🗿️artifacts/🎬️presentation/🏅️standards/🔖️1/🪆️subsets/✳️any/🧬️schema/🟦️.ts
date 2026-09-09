/** 🧬️ Presentation document with independently owned presentation and animation children. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
export type { ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface PresentationArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact @child kind=s.stdio.semio */ presentation: ArtifactChild;
  /** @state artifact @child kind=s.stdio.semio */ animation: ArtifactChild;
}

/** 🪪️ Parses the exact document identity boundary shared by all Presentation views. */
export function parsePresentationArtifact(value: unknown, at = "$"): PresentationArtifact {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: document must be an object`);
  const row = value as Record<string, unknown>, keys = ["schema", "presentation", "animation"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: fields do not match PresentationArtifact`);
  if (typeof row.schema !== "string") throw new Error(`${at}.schema: value must be a string`);
  return { schema: row.schema, presentation: parseArtifactChild(row.presentation), animation: parseArtifactChild(row.animation) };
}
