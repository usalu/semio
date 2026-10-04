/** 🔺️ Sparse Equation document delta. */
import {parseEquationExprSnapshot,parseEquationGeometry,parseEquationGraph,type EquationExprSnapshot,type EquationGeometry,type EquationGraph} from "../🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface EquationDiff {
  /** @state artifact */ graph?: EquationGraph | null;
  /** @state artifact */ geometry?: EquationGeometry | null;
  /** @state artifact */ notation?: ArtifactChild | null;
  /** @state artifact */ results?: ArtifactChild | null;
  /** @state artifact */ computed?: ArtifactChild | null;
  /** @state artifact */ equation?: EquationExprSnapshot | null;
}

/** 🪪️ Validates the sparse Equation delta boundary. */
export function parseEquationDiff(value: unknown, at = "$" ): EquationDiff {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: Equation diff must be an object`);
  const row = value as Record<string, unknown>;
  const allowed = new Set(["graph", "geometry", "notation", "results", "computed", "equation"]);
  if (Object.keys(row).some((key) => !allowed.has(key))) throw new Error(`${at}: Equation diff has an unknown field`);
  return {
    ...(Object.hasOwn(row, "graph") ? { graph: row.graph == null ? null : parseEquationGraph(row.graph) } : {}),
    ...(Object.hasOwn(row, "geometry") ? { geometry: row.geometry == null ? null : parseEquationGeometry(row.geometry) } : {}),
    ...(Object.hasOwn(row, "notation") ? { notation: row.notation == null ? null : parseArtifactChild(row.notation) } : {}),
    ...(Object.hasOwn(row, "results") ? { results: row.results == null ? null : parseArtifactChild(row.results) } : {}),
    ...(Object.hasOwn(row, "computed") ? { computed: row.computed == null ? null : parseArtifactChild(row.computed) } : {}),
    ...(Object.hasOwn(row, "equation") ? { equation: row.equation == null ? null : parseEquationExprSnapshot(row.equation) } : {}),
  };
}
