/** 🧬️ WiresSnapshot carries only the canonical document fields. */
import { parseDslValue, type DslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface WiresSnapshot {
  /** @state artifact */
  wiresFixture: DslValue;
  /** @state artifact */
  content: ArtifactChild;
  /** @state artifact */
  meta: DslValue;
}

/** 🪪️ Validates the document boundary against its exact field set. */
export function parseWiresSnapshot(value: unknown, at = "$" ): WiresSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: document must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["wiresFixture", "content", "meta"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: document fields do not match its schema`);
  return { wiresFixture: parseDslValue(row.wiresFixture), content: parseArtifactChild(row.content), meta: parseDslValue(row.meta) };
}

export interface WiresStringList { values: string[] }

/** 📜️ Validates an owned list of strings. */
export function parseWiresStringList(value: unknown, at = "$"): WiresStringList {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: string list must be an object`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 1 || !Array.isArray(row.values) || row.values.some((entry) => typeof entry !== "string")) throw new Error(`${at}: string list requires exactly a values array`);
  return { values: row.values as string[] };
}
