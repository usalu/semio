/** 🧬️ Sequence diff schema — sparse field delta. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface SequenceDiff {
  /** @state artifact */ schema?: string;
  /** @state artifact @child kind=s.stdio.semio */ content?: ArtifactChild;
}

const object = (value: unknown, at: string): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value is not an object`);
  return value as Record<string, unknown>;
};

export function parseSequenceDiff(value: unknown, at = "$"): SequenceDiff {
  const row = object(value, at);
  if (Object.keys(row).some((key) => !["schema", "content"].includes(key))) throw new Error(`${at}: unexpected field`);
  if (row.schema !== undefined && typeof row.schema !== "string") throw new Error(`${at}.schema: value is not a string`);
  return {
    schema: row.schema as string | undefined,
    content: row.content === undefined ? undefined : parseArtifactChild(row.content),
  };
}
