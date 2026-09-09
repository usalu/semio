/** 🔺️ Sparse Equation document delta. */
import { parseDslValue, type DslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface EquationDiff {
  /** @state artifact */ notation?: ArtifactChild | null;
  /** @state artifact */ results?: ArtifactChild | null;
  /** @state artifact */ computed?: ArtifactChild | null;
  /** @state artifact */ equation?: DslValue | null;
}

/** 🪪️ Validates the sparse Equation delta boundary. */
export function parseEquationDiff(value: unknown, at = "$" ): EquationDiff {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: Equation diff must be an object`);
  const row = value as Record<string, unknown>;
  const allowed = new Set(["notation", "results", "computed", "equation"]);
  if (Object.keys(row).some((key) => !allowed.has(key))) throw new Error(`${at}: Equation diff has an unknown field`);
  return {
    ...(Object.hasOwn(row, "notation") ? { notation: row.notation == null ? null : parseArtifactChild(row.notation) } : {}),
    ...(Object.hasOwn(row, "results") ? { results: row.results == null ? null : parseArtifactChild(row.results) } : {}),
    ...(Object.hasOwn(row, "computed") ? { computed: row.computed == null ? null : parseArtifactChild(row.computed) } : {}),
    ...(Object.hasOwn(row, "equation") ? { equation: row.equation == null ? null : parseDslValue(row.equation) } : {}),
  };
}
