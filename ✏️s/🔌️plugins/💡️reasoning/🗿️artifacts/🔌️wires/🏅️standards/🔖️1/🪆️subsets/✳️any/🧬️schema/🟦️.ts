/** 🧬️ WiresArtifact carries only the canonical document fields. */
import {parseWiresValue,type WiresValue}from"./🌱️value/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface WiresArtifact {
  /** @state artifact */
  wiresFixture: WiresValue;
  /** @state artifact @child kind=s.stdio.semio */
  content: ArtifactChild;
  /** @state artifact */
  meta: WiresValue;
}

/** 🪪️ Validates the document boundary against its exact field set. */
export function parseWiresArtifact(value: unknown, at = "$" ): WiresArtifact {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: document must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["wiresFixture", "content", "meta"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: document fields do not match its schema`);
  return { wiresFixture: parseWiresValue(row.wiresFixture), content: parseArtifactChild(row.content), meta: parseWiresValue(row.meta) };
}
