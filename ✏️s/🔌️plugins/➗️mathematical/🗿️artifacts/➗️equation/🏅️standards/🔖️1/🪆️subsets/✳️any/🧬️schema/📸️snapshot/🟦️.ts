/** 📸️ Persisted Equation document snapshot. */
import { parseDslValue, type DslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface EquationSnapshot {
  /** @state artifact */ notation: ArtifactChild;
  /** @state artifact */ results: ArtifactChild;
  /** @state artifact */ computed: ArtifactChild;
  /** @state artifact */ equation: DslValue;
}

/** 🪪️ Validates the exact persisted Equation snapshot boundary. */
export function parseEquationSnapshot(value: unknown, at = "$" ): EquationSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: Equation snapshot must be an object`);
  const row = value as Record<string, unknown>;
  const keys = ["notation", "results", "computed", "equation"];
  if (Object.keys(row).length !== keys.length || keys.some((key) => !Object.hasOwn(row, key))) throw new Error(`${at}: Equation snapshot fields do not match its schema`);
  return { notation: parseArtifactChild(row.notation), results: parseArtifactChild(row.results), computed: parseArtifactChild(row.computed), equation: parseDslValue(row.equation) };
}
