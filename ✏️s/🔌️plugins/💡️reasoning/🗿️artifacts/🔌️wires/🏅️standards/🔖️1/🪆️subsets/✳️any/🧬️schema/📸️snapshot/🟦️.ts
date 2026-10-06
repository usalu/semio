/** 🧬️ WiresSnapshot carries only the canonical document fields. */
import {parseWiresValue,type WiresValue}from"../🌱️value/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface WiresSnapshot {
  /** @state artifact */
  wiresSnapshot: WiresValue;
  /** @state artifact @child kind=s.stdio.semio */
  content: ArtifactChild;
  /** @state artifact */
  meta: WiresValue;
}

/** 🪪️ Validates the document boundary against its exact field set. */
export function parseWiresSnapshot(value: unknown, at = "$" ): WiresSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: document must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["wiresSnapshot", "content", "meta"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: document fields do not match its schema`);
  return { wiresSnapshot: parseWiresValue(row.wiresSnapshot), content: parseArtifactChild(row.content), meta: parseWiresValue(row.meta) };
}

export interface WiresStringList { values: string[] }

/** 📜️ Validates an owned list of strings. */
export function parseWiresStringList(value: unknown, at = "$"): WiresStringList {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: string list must be an object`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 1 || !Array.isArray(row.values) || row.values.some((entry) => typeof entry !== "string")) throw new Error(`${at}: string list requires exactly a values array`);
  return { values: row.values as string[] };
}
