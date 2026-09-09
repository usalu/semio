/** 🔺️ Presentation durable delta with exact child-slot replacement. */
import { parseArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";
import { parsePresentationArtifact, type PresentationArtifact, type ArtifactChild } from "../🟦️.ts";

export interface PresentationDiff {
  /** @state artifact */ artifact: PresentationArtifact | null;
  /** @state artifact */ schema: string | null;
  /** @state artifact @child kind=s.stdio.semio */ presentation: ArtifactChild | null;
}

/** 🧮️ Resolves omitted delta fields to the native unchanged null value. */
export function parsePresentationDiff(value: unknown, at = "$"): PresentationDiff {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: delta must be an object`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !["artifact", "schema", "presentation"].includes(key))) throw new Error(`${at}: fields do not match PresentationDiff`);
  if (row.schema != null && typeof row.schema !== "string") throw new Error(`${at}.schema: value must be a string or null`);
  return {
    artifact: row.artifact == null ? null : parsePresentationArtifact(row.artifact, `${at}.artifact`),
    schema: (row.schema ?? null) as string | null,
    presentation: row.presentation == null ? null : parseArtifactChild(row.presentation),
  };
}
