/** 📸️ Flow document snapshot: durable schema and composed content identity. */
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface FlowSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact @child kind=s.stdio.semio standard=v1 subset=flow */ content: ArtifactChild;
}

export function parseFlowSnapshot(value: unknown, at = "$"): FlowSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: FlowSnapshot must be an object`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 2 || !Object.hasOwn(row, "schema") || !Object.hasOwn(row, "content")) throw new Error(`${at}: FlowSnapshot requires exactly schema and content`);
  if (typeof row.schema !== "string") throw new Error(`${at}.schema: value is not a string`);
  return { schema: row.schema, content: parseArtifactChild(row.content) };
}
