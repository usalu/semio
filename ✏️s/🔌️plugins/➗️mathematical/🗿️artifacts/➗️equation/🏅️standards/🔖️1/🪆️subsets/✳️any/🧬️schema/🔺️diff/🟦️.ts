/** 🔺️ Sparse Equation document delta: graph scalars, id-keyed node and edge row deltas, positional point edits and label-addressed expression-node kind patches. */
import {parseEquationExprSnapshot,parseEquationGeometry,parseEquationGraph,type EquationEdge,type EquationNode,type EquationNodeKind,type EquationPoint} from "../🟦️.ts";
import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface EquationOptionalSeed { value: string | null }
export interface RowDelta<Row, Patch> { added: Row[]; removed: string[]; patched: Patch[]; reordered: string[] | null }
export interface EquationNodePatch { id: string; label?: string; x?: number; y?: number }
export interface EquationEdgePatch { id: string; source?: string; target?: string }
export type EquationPointEdit = { op: "insert"; at: number; point: EquationPoint } | { op: "remove"; at: number } | { op: "set"; at: number; point: EquationPoint };
export interface EquationPointsDelta { edits: EquationPointEdit[] }
export interface EquationKindPatch { label: bigint; kind: EquationNodeKind }
export interface EquationExprDiff { kinds: EquationKindPatch[]; nextLabel: bigint | null }
export type EquationNodesDelta = RowDelta<EquationNode, EquationNodePatch>;
export type EquationEdgesDelta = RowDelta<EquationEdge, EquationEdgePatch>;

export interface EquationDiff {
  /** @state artifact */ directed?: boolean;
  /** @state artifact */ algorithm?: string;
  /** @state artifact */ algorithmSeed?: EquationOptionalSeed;
  /** @state artifact */ nodes?: EquationNodesDelta;
  /** @state artifact */ edges?: EquationEdgesDelta;
  /** @state artifact */ points?: EquationPointsDelta;
  /** @state artifact */ notation?: ArtifactChild;
  /** @state artifact */ results?: ArtifactChild;
  /** @state artifact */ computed?: ArtifactChild;
  /** @state artifact */ equation?: EquationExprDiff;
}

const record = (value: unknown, at: string, allowed: readonly string[]): Record<string, unknown> => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: Equation diff value must be an object`);
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => !allowed.includes(key))) throw new Error(`${at}: Equation diff has an unknown field`);
  return row;
};
const list = (value: unknown, at: string): unknown[] => {
  if (!Array.isArray(value)) throw new Error(`${at}: Equation diff list required`);
  return value;
};
const text = (value: unknown, at: string): string => {
  if (typeof value !== "string") throw new Error(`${at}: Equation diff text required`);
  return value;
};
const finite = (value: unknown, at: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) throw new Error(`${at}: Equation diff finite number required`);
  return value;
};
const index = (value: unknown, at: string): number => {
  if (!Number.isSafeInteger(value) || (value as number) < 0) throw new Error(`${at}: Equation diff index required`);
  return value as number;
};
const present = <K extends string, T>(row: Record<string, unknown>, key: K, parse: (value: unknown) => T): Partial<Record<K, T>> => (Object.hasOwn(row, key) ? ({ [key]: parse(row[key]) } as Record<K, T>) : {});

const delta = <Row, Patch>(value: unknown, at: string, row: (value: unknown, at: string) => Row, patch: (value: unknown, at: string) => Patch): RowDelta<Row, Patch> => {
  const fields = record(value, at, ["added", "removed", "patched", "reordered"]);
  return {
    added: list(fields.added, `${at}.added`).map((item, position) => row(item, `${at}.added[${position}]`)),
    removed: list(fields.removed, `${at}.removed`).map((id, position) => text(id, `${at}.removed[${position}]`)),
    patched: list(fields.patched, `${at}.patched`).map((item, position) => patch(item, `${at}.patched[${position}]`)),
    reordered: fields.reordered == null ? null : list(fields.reordered, `${at}.reordered`).map((id, position) => text(id, `${at}.reordered[${position}]`)),
  };
};

export function parseEquationNodePatch(value: unknown, at = "$"): EquationNodePatch {
  const row = record(value, at, ["id", "label", "x", "y"]);
  return { id: text(row.id, `${at}.id`), ...present(row, "label", (label) => text(label, `${at}.label`)), ...present(row, "x", (x) => finite(x, `${at}.x`)), ...present(row, "y", (y) => finite(y, `${at}.y`)) };
}

export function parseEquationEdgePatch(value: unknown, at = "$"): EquationEdgePatch {
  const row = record(value, at, ["id", "source", "target"]);
  return { id: text(row.id, `${at}.id`), ...present(row, "source", (source) => text(source, `${at}.source`)), ...present(row, "target", (target) => text(target, `${at}.target`)) };
}

export function parseEquationPointEdit(value: unknown, at = "$"): EquationPointEdit {
  const row = record(value, at, ["op", "at", "point"]);
  const position = index(row.at, `${at}.at`);
  switch (row.op) {
    case "insert":
      return { op: "insert", at: position, point: parseEquationGeometry({ points: [row.point] }).points[0]! };
    case "set":
      return { op: "set", at: position, point: parseEquationGeometry({ points: [row.point] }).points[0]! };
    case "remove":
      if (Object.hasOwn(row, "point")) throw new Error(`${at}: a remove edit carries no point`);
      return { op: "remove", at: position };
    default:
      throw new Error(`${at}.op: Equation point edit differs`);
  }
}

export function parseEquationExprDiff(value: unknown, at = "$"): EquationExprDiff {
  const row = record(value, at, ["kinds", "nextLabel"]);
  const kinds = list(row.kinds, `${at}.kinds`).map((item, position) => {
    const patch = record(item, `${at}.kinds[${position}]`, ["label", "kind"]);
    const checked = parseEquationExprSnapshot({ expr: { label: patch.label, kind: patch.kind }, nextLabel: patch.label });
    return { label: checked.expr.label, kind: checked.expr.kind };
  });
  const nextLabel = row.nextLabel == null ? null : parseEquationExprSnapshot({ expr: { label: row.nextLabel, kind: { kind: "integer", lexeme: "0" } }, nextLabel: row.nextLabel }).nextLabel;
  return { kinds, nextLabel };
}

/** 🪪️ Validates the sparse Equation delta boundary. */
export function parseEquationDiff(value: unknown, at = "$"): EquationDiff {
  const row = record(value, at, ["directed", "algorithm", "algorithmSeed", "nodes", "edges", "points", "notation", "results", "computed", "equation"]);
  const graph = (nodes: unknown[], edges: unknown[]) => parseEquationGraph({ directed: true, nodes, edges, algorithm: "x" });
  return {
    ...present(row, "directed", (directed) => {
      if (typeof directed !== "boolean") throw new Error(`${at}.directed: boolean required`);
      return directed;
    }),
    ...present(row, "algorithm", (algorithm) => text(algorithm, `${at}.algorithm`)),
    ...present(row, "algorithmSeed", (seed) => {
      const slot = record(seed, `${at}.algorithmSeed`, ["value"]);
      return { value: slot.value == null ? null : text(slot.value, `${at}.algorithmSeed.value`) };
    }),
    ...present(row, "nodes", (nodes) => delta(nodes, `${at}.nodes`, (item) => graph([item], []).nodes[0]!, parseEquationNodePatch)),
    ...present(row, "edges", (edges) => delta(edges, `${at}.edges`, (item) => graph([], [item]).edges[0]!, parseEquationEdgePatch)),
    ...present(row, "points", (points) => ({ edits: list(record(points, `${at}.points`, ["edits"]).edits, `${at}.points.edits`).map((edit, position) => parseEquationPointEdit(edit, `${at}.points.edits[${position}]`)) })),
    ...present(row, "notation", (child) => parseArtifactChild(child)),
    ...present(row, "results", (child) => parseArtifactChild(child)),
    ...present(row, "computed", (child) => parseArtifactChild(child)),
    ...present(row, "equation", (expr) => parseEquationExprDiff(expr, `${at}.equation`)),
  };
}
