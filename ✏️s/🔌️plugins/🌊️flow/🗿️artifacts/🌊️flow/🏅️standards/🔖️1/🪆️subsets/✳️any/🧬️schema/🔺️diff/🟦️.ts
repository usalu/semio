/** 🔺️ Sparse Flow document delta. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface FlowArtifact { schema: string; content: ArtifactChild }
export interface FlowDiff {
  /** @state artifact */ artifact?: FlowArtifact;
  /** @state artifact */ schema?: string;
  /** @state artifact @child kind=s.stdio.semio standard=v1 subset=flow */ content?: ArtifactChild;
}

const object = (value: unknown, at: string): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value is not an object`);
  return value as Record<string, unknown>;
};

export function parseFlowArtifact(value: unknown, at = "$"): FlowArtifact {
  const row = object(value, at);
  if (Object.keys(row).length !== 2 || !Object.hasOwn(row, "schema") || !Object.hasOwn(row, "content") || typeof row.schema !== "string") throw new Error(`${at}: FlowArtifact requires exactly schema and content`);
  return { schema: row.schema, content: parseArtifactChild(row.content) };
}

export function parseFlowDiff(value: unknown, at = "$"): FlowDiff {
  const row = object(value, at);
  if (Object.keys(row).some((key) => !["artifact", "schema", "content"].includes(key))) throw new Error(`${at}: unexpected field`);
  if (row.schema !== undefined && typeof row.schema !== "string") throw new Error(`${at}.schema: value is not a string`);
  return {
    artifact: row.artifact === undefined ? undefined : parseFlowArtifact(row.artifact, `${at}.artifact`),
    schema: row.schema as string | undefined,
    content: row.content === undefined ? undefined : parseArtifactChild(row.content),
  };
}
