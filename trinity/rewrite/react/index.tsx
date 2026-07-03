// #region 🧲Header
/** @emoji ♻️ Trinity rewrite directed-port-graph adapters — visual LHS/RHS rule graphs to Jack rule JSON. */
// #endregion 🧲Header

import { mergeManifestCatalogBundles, rewrite_lhsManifestCatalogBundle, rewrite_rhsManifestCatalogBundle } from "@semio-tech/graph-manifest";
import {
  parsePuzzle2dFixture,
  type KindCatalogBundle,
  type Puzzle2dFixtureCircleNode,
  type Puzzle2dFixtureHandle,
  type Puzzle2dFixture,
} from "@semio-tech/puzzle-2d-react";
import type { RuleParameter } from "@semio-tech/trinity-react";

export const REWRITE_NODE_MATCH = "rewrite.match" as const;
export const REWRITE_NODE_WHERE = "rewrite.where" as const;
export const REWRITE_NODE_SET = "rewrite.set" as const;
export const REWRITE_NODE_PARAMETER = "rewrite.parameter" as const;
export const REWRITE_NODE_CREATE = "rewrite.create" as const;
export const REWRITE_NODE_DELETE = "rewrite.delete" as const;
export const REWRITE_NODE_MERGE = "rewrite.merge" as const;
export const REWRITE_EDGE_FLOW = "edge.flow" as const;
export const REWRITE_EDGE_PATTERN = "edge.pattern" as const;
export const REWRITE_PORT_HANDLE = "port" as const;

export const REWRITE_LHS_MANIFEST_ID = "rewrite-lhs" as const;
export const REWRITE_RHS_MANIFEST_ID = "rewrite-rhs" as const;

type RewritePortKind = "in" | "out";

function rewritePortHandle(nodeId: string, port: RewritePortKind): Puzzle2dFixtureHandle {
  return { id: `${nodeId}:${port}`, handleKind: REWRITE_PORT_HANDLE, angle: port === "out" ? 0 : Math.PI };
}

function rewriteGraphNode(
  id: string,
  nodeKind: string,
  text: string,
  x: number,
  y: number,
  ports: readonly RewritePortKind[],
): Puzzle2dFixtureCircleNode {
  return { id, nodeKind, text, x, y, radius: 44, shape: "circle", handles: ports.map((port) => rewritePortHandle(id, port)) };
}

function parseHandleEndpoint(ref: string): string {
  const idx = ref.indexOf(":");
  return idx >= 0 ? ref.slice(0, idx) : ref;
}

function formatMatchLabel(parsed: { var: string; kind: string }): string {
  return `${parsed.var} : ${parsed.kind}`;
}

function parseWhereVars(text: string): readonly string[] {
  const vars = new Set<string>();
  for (const match of text.matchAll(/\b([a-zA-Z_$][\w$]*)\./g)) {
    vars.add(match[1]!);
  }
  return [...vars];
}

function rewriteNodeVar(node: Puzzle2dFixtureCircleNode): string | null {
  if (
    node.nodeKind === REWRITE_NODE_MATCH ||
    node.nodeKind === REWRITE_NODE_CREATE ||
    node.nodeKind === REWRITE_NODE_DELETE ||
    node.nodeKind === REWRITE_NODE_MERGE
  ) {
    return parseMatchLabel(node.text ?? "").var;
  }
  if (node.nodeKind === REWRITE_NODE_SET) {
    return parseSetLabel(node.text ?? "")?.var ?? null;
  }
  if (node.nodeKind === REWRITE_NODE_WHERE) {
    return parseWhereVars(node.text ?? "")[0] ?? null;
  }
  return null;
}

/** @emoji 🏷️ Resolve the Jack variable name bound to a rewrite rule-graph node id. */
export function rewriteVarForNodeId(fixture: Puzzle2dFixture, nodeId: string): string | null {
  const node = fixture.nodes.find((entry) => entry.id === nodeId);
  if (!node) return null;
  return rewriteNodeVar(node);
}

/** @emoji 🔗 Resolve rewrite rule-graph node ids that reference a Jack variable name. */
export function rewriteNodeIdsForVar(fixture: Puzzle2dFixture, varName: string): readonly string[] {
  const ids: string[] = [];
  for (const node of fixture.nodes) {
    const bound = rewriteNodeVar(node);
    if (bound === varName) {
      ids.push(node.id);
    }
    if (node.nodeKind === REWRITE_NODE_WHERE && parseWhereVars(node.text ?? "").includes(varName)) {
      ids.push(node.id);
    }
  }
  return [...new Set(ids)];
}

function lhsPatternToMatchClause(pattern: {
  leftVar: string;
  leftKind: string;
  edgeVar?: string;
  edgeKind?: string;
  rightVar?: string;
  rightKind?: string;
}): string {
  if (pattern.rightVar && pattern.rightKind) {
    const edgeVar = pattern.edgeVar ?? "e";
    const edgeKind = pattern.edgeKind ?? "Connection";
    return `(${pattern.leftVar}:${pattern.leftKind})-[${edgeVar}:${edgeKind}]->(${pattern.rightVar}:${pattern.rightKind})`;
  }
  return `(${pattern.leftVar}:${pattern.leftKind})`;
}

/** @emoji 🧵 Build a Jack MATCH query for the LHS pattern returning one variable binding. */
export function rewriteLhsMatchQuery(lhsJson: string, returnVar: string): string {
  const lhs = JSON.parse(lhsJson) as {
    pattern?: {
      leftVar?: string;
      leftKind?: string;
      edgeVar?: string;
      edgeKind?: string;
      rightVar?: string;
      rightKind?: string;
    };
    whereClause?: string;
  };
  const pattern = {
    leftVar: lhs.pattern?.leftVar ?? "a",
    leftKind: lhs.pattern?.leftKind ?? "Piece",
    edgeVar: lhs.pattern?.edgeVar,
    edgeKind: lhs.pattern?.edgeKind,
    rightVar: lhs.pattern?.rightVar,
    rightKind: lhs.pattern?.rightKind,
  };
  let query = `MATCH ${lhsPatternToMatchClause(pattern)}`;
  const where = lhs.whereClause?.trim();
  if (where) {
    query += ` WHERE ${where}`;
  }
  query += ` RETURN ${returnVar}`;
  return query;
}

function parseMatchLabel(text: string): { var: string; kind: string } {
  const trimmed = text.trim();
  const idx = trimmed.indexOf(":");
  if (idx < 0) {
    return { var: trimmed || "a", kind: "Node" };
  }
  return { var: trimmed.slice(0, idx).trim() || "a", kind: trimmed.slice(idx + 1).trim() || "Node" };
}

function parseSetLabel(text: string): { var: string; prop: string; value: string } | null {
  const match = text.trim().match(/^([\w$]+)\.(\w+)\s*=\s*(.+)$/);
  if (!match) return null;
  return { var: match[1], prop: match[2], value: match[3].trim() };
}

function parseParameterLabel(text: string): RuleParameter | null {
  const match = text.trim().match(/^([\w$]+):(string|number|boolean)=(.*)$/);
  if (!match) return null;
  const name = match[1];
  const kind = match[2] as RuleParameter["kind"];
  const raw = match[3].trim();
  if (kind === "number") {
    const value = Number(raw);
    return { name, kind, default: Number.isFinite(value) ? value : 0 };
  }
  if (kind === "boolean") {
    return { name, kind, default: raw === "true" };
  }
  const unquoted = raw.startsWith("'") && raw.endsWith("'") ? raw.slice(1, -1) : raw;
  return { name, kind, default: unquoted };
}

function formatParameterDefault(param: RuleParameter): string {
  if (param.kind === "string") return String(param.default ?? "");
  if (param.kind === "number") return String(param.default ?? 0);
  return param.default === true ? "true" : "false";
}

export function rewriteLhsKindCatalogs(): KindCatalogBundle {
  return mergeManifestCatalogBundles(rewrite_lhsManifestCatalogBundle());
}

export function rewriteRhsKindCatalogs(): KindCatalogBundle {
  return mergeManifestCatalogBundles(rewrite_rhsManifestCatalogBundle());
}

export const REWRITE_DEFAULT_LHS_FIXTURE: Puzzle2dFixture = {
  schema: "puzzle.2d.fixture",
  camera: { x: 0, y: 0, zoom: 1 },
  meta: { manifestId: REWRITE_LHS_MANIFEST_ID },
  nodes: [
    rewriteGraphNode("match-a", REWRITE_NODE_MATCH, formatMatchLabel({ var: "a", kind: "Piece" }), -140, 0, ["out"]),
    rewriteGraphNode("where-b", REWRITE_NODE_WHERE, "a.name = 'b'", 140, 0, ["in"]),
  ],
  edges: [{ id: "lhs-flow", source: "match-a:out", target: "where-b:in", edgeKind: REWRITE_EDGE_FLOW }],
};

export const REWRITE_DEFAULT_RHS_FIXTURE: Puzzle2dFixture = {
  schema: "puzzle.2d.fixture",
  camera: { x: 0, y: 0, zoom: 1 },
  meta: { manifestId: REWRITE_RHS_MANIFEST_ID },
  nodes: [
    rewriteGraphNode("param-label", REWRITE_NODE_PARAMETER, "label:string=nakagin-core", -260, 0, ["out"]),
    rewriteGraphNode("set-label", REWRITE_NODE_SET, "a.label = $label", 40, 0, ["in", "out"]),
  ],
  edges: [{ id: "rhs-flow", source: "param-label:out", target: "set-label:in", edgeKind: REWRITE_EDGE_FLOW }],
};

export const REWRITE_DEFAULT_LHS_FIXTURE_JSON = JSON.stringify(REWRITE_DEFAULT_LHS_FIXTURE);
export const REWRITE_DEFAULT_RHS_FIXTURE_JSON = JSON.stringify(REWRITE_DEFAULT_RHS_FIXTURE);

export function parseRewriteGraphFixtureJson(json: string): Puzzle2dFixture | null {
  try {
    return parsePuzzle2dFixture(JSON.parse(json));
  } catch {
    return null;
  }
}

export function rewriteLhsGraphToJson(fixture: Puzzle2dFixture): string {
  const matchNodes = fixture.nodes.filter((node) => node.nodeKind === REWRITE_NODE_MATCH);
  const whereNodes = fixture.nodes.filter((node) => node.nodeKind === REWRITE_NODE_WHERE);
  let pattern: {
    leftVar: string;
    leftKind: string;
    edgeVar?: string;
    edgeKind?: string;
    rightVar?: string;
    rightKind?: string;
  };

  const patternEdge = fixture.edges.find((edge) => {
    const leftId = parseHandleEndpoint(edge.source);
    const rightId = parseHandleEndpoint(edge.target);
    const left = matchNodes.find((node) => node.id === leftId);
    const right = matchNodes.find((node) => node.id === rightId);
    return left != null && right != null;
  });

  if (patternEdge && matchNodes.length >= 2) {
    const leftId = parseHandleEndpoint(patternEdge.source);
    const rightId = parseHandleEndpoint(patternEdge.target);
    const leftNode = matchNodes.find((node) => node.id === leftId) ?? matchNodes[0];
    const rightNode = matchNodes.find((node) => node.id === rightId) ?? matchNodes[1];
    const left = parseMatchLabel(leftNode.text ?? "");
    const right = parseMatchLabel(rightNode.text ?? "");
    const edgeLabel = patternEdge.edgeKind === REWRITE_EDGE_PATTERN ? "Connection" : undefined;
    pattern = {
      leftVar: left.var,
      leftKind: left.kind,
      edgeVar: "e",
      edgeKind: edgeLabel,
      rightVar: right.var,
      rightKind: right.kind,
    };
  } else if (matchNodes[0]) {
    const left = parseMatchLabel(matchNodes[0].text ?? "");
    pattern = { leftVar: left.var, leftKind: left.kind };
  } else {
    pattern = { leftVar: "a", leftKind: "Piece" };
  }

  const whereClause = whereNodes.map((node) => node.text?.trim()).find((row) => row && row.length > 0);
  const lhs: { pattern: typeof pattern; whereClause?: string } = { pattern };
  if (whereClause) lhs.whereClause = whereClause;
  return JSON.stringify(lhs, null, 2);
}

export function rewriteRhsGraphToJson(fixture: Puzzle2dFixture): string {
  const setRows = fixture.nodes
    .filter((node) => node.nodeKind === REWRITE_NODE_SET)
    .map((node) => parseSetLabel(node.text ?? ""))
    .filter((row): row is NonNullable<typeof row> => row != null)
    .map((row) => ({ var: row.var, prop: row.prop, value: row.value }));

  const parameters = fixture.nodes
    .filter((node) => node.nodeKind === REWRITE_NODE_PARAMETER)
    .map((node) => parseParameterLabel(node.text ?? ""))
    .filter((row): row is RuleParameter => row != null);

  const create = fixture.nodes
    .filter((node) => node.nodeKind === REWRITE_NODE_CREATE)
    .map((node) => {
      const parsed = parseMatchLabel(node.text ?? "");
      return { leftVar: parsed.var, leftKind: parsed.kind };
    });

  const del = fixture.nodes
    .filter((node) => node.nodeKind === REWRITE_NODE_DELETE)
    .map((node) => {
      const parsed = parseMatchLabel(node.text ?? "");
      return { leftVar: parsed.var, leftKind: parsed.kind };
    });

  const merge = fixture.nodes
    .filter((node) => node.nodeKind === REWRITE_NODE_MERGE)
    .map((node) => {
      const parsed = parseMatchLabel(node.text ?? "");
      return { leftVar: parsed.var, leftKind: parsed.kind };
    });

  return JSON.stringify({ create, delete: del, set: setRows, merge, parameters }, null, 2);
}

export function rewriteLhsJsonToGraph(lhsJson: string): Puzzle2dFixture {
  try {
    const lhs = JSON.parse(lhsJson) as {
      pattern?: {
        leftVar?: string;
        leftKind?: string;
        edgeVar?: string;
        edgeKind?: string;
        rightVar?: string;
        rightKind?: string;
      };
      whereClause?: string;
    };
    const pattern = lhs.pattern ?? { leftVar: "a", leftKind: "Piece" };
    const nodes: Puzzle2dFixtureCircleNode[] = [];
    const edges: Puzzle2dFixture["edges"] = [];
    if (pattern.rightVar && pattern.rightKind) {
      nodes.push(
        rewriteGraphNode("match-left", REWRITE_NODE_MATCH, formatMatchLabel({ var: pattern.leftVar!, kind: pattern.leftKind! }), -220, 0, ["out"]),
        rewriteGraphNode("match-right", REWRITE_NODE_MATCH, formatMatchLabel({ var: pattern.rightVar!, kind: pattern.rightKind! }), 0, 0, ["in", "out"]),
      );
      edges.push({
        id: "pattern-edge",
        source: "match-left:out",
        target: "match-right:in",
        edgeKind: REWRITE_EDGE_PATTERN,
      });
      if (lhs.whereClause) {
        nodes.push(rewriteGraphNode("where", REWRITE_NODE_WHERE, lhs.whereClause, 220, 0, ["in"]));
        edges.push({ id: "where-flow", source: "match-right:out", target: "where:in", edgeKind: REWRITE_EDGE_FLOW });
      }
    } else {
      nodes.push(rewriteGraphNode("match-a", REWRITE_NODE_MATCH, formatMatchLabel({ var: pattern.leftVar!, kind: pattern.leftKind! }), -140, 0, ["out"]));
      if (lhs.whereClause) {
        nodes.push(rewriteGraphNode("where", REWRITE_NODE_WHERE, lhs.whereClause, 140, 0, ["in"]));
        edges.push({ id: "where-flow", source: "match-a:out", target: "where:in", edgeKind: REWRITE_EDGE_FLOW });
      }
    }
    return { ...REWRITE_DEFAULT_LHS_FIXTURE, nodes, edges };
  } catch {
    return REWRITE_DEFAULT_LHS_FIXTURE;
  }
}

export function rewriteRhsJsonToGraph(rhsJson: string): Puzzle2dFixture {
  try {
    const rhs = JSON.parse(rhsJson) as {
      set?: readonly { var?: string; prop?: string; value?: string }[];
      parameters?: readonly RuleParameter[];
      create?: readonly { leftVar?: string; leftKind?: string }[];
      delete?: readonly { leftVar?: string; leftKind?: string }[];
      merge?: readonly { leftVar?: string; leftKind?: string }[];
    };
    const nodes: Puzzle2dFixtureCircleNode[] = [];
    const edges: Puzzle2dFixture["edges"] = [];
    let y = -120;
    const bumpY = () => {
      y += 80;
      return y;
    };
    for (const param of rhs.parameters ?? []) {
      const id = `param-${param.name}`;
      nodes.push(
        rewriteGraphNode(id, REWRITE_NODE_PARAMETER, `${param.name}:${param.kind}=${formatParameterDefault(param)}`, -260, bumpY(), ["out"]),
      );
    }
    for (const row of rhs.set ?? []) {
      if (!row.var || !row.prop || row.value == null) continue;
      const id = `set-${row.var}-${row.prop}`;
      const node = rewriteGraphNode(id, REWRITE_NODE_SET, `${row.var}.${row.prop} = ${row.value}`, 60, bumpY(), ["in", "out"]);
      nodes.push(node);
      const paramName = String(row.value).startsWith("$") ? String(row.value).slice(1) : null;
      if (paramName) {
        const paramNode = nodes.find((entry) => entry.nodeKind === REWRITE_NODE_PARAMETER && entry.text?.startsWith(`${paramName}:`));
        if (paramNode) {
          edges.push({ id: `flow-${paramNode.id}-${id}`, source: `${paramNode.id}:out`, target: `${id}:in`, edgeKind: REWRITE_EDGE_FLOW });
        }
      }
    }
    for (const row of rhs.create ?? []) {
      if (!row.leftVar || !row.leftKind) continue;
      nodes.push(rewriteGraphNode(`create-${row.leftVar}`, REWRITE_NODE_CREATE, formatMatchLabel({ var: row.leftVar, kind: row.leftKind }), 60, bumpY(), ["out"]));
    }
    for (const row of rhs.delete ?? []) {
      if (!row.leftVar || !row.leftKind) continue;
      nodes.push(rewriteGraphNode(`delete-${row.leftVar}`, REWRITE_NODE_DELETE, formatMatchLabel({ var: row.leftVar, kind: row.leftKind }), 60, bumpY(), ["out"]));
    }
    for (const row of rhs.merge ?? []) {
      if (!row.leftVar || !row.leftKind) continue;
      nodes.push(rewriteGraphNode(`merge-${row.leftVar}`, REWRITE_NODE_MERGE, formatMatchLabel({ var: row.leftVar, kind: row.leftKind }), 60, bumpY(), ["out"]));
    }
    if (!nodes.length) return REWRITE_DEFAULT_RHS_FIXTURE;
    return { ...REWRITE_DEFAULT_RHS_FIXTURE, nodes, edges };
  } catch {
    return REWRITE_DEFAULT_RHS_FIXTURE;
  }
}

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("rewrite rule graphs", () => {
    it("default lhs graph compiles to match + where", () => {
      const json = rewriteLhsGraphToJson(REWRITE_DEFAULT_LHS_FIXTURE);
      const lhs = JSON.parse(json) as { pattern: { leftVar: string; leftKind: string }; whereClause?: string };
      expect(lhs.pattern.leftVar).toBe("a");
      expect(lhs.pattern.leftKind).toBe("Piece");
      expect(lhs.whereClause).toBe("a.name = 'b'");
    });

    it("default rhs graph compiles to set + parameter", () => {
      const json = rewriteRhsGraphToJson(REWRITE_DEFAULT_RHS_FIXTURE);
      const rhs = JSON.parse(json) as { set: { var: string; prop: string; value: string }[]; parameters: RuleParameter[] };
      expect(rhs.set[0]).toEqual({ var: "a", prop: "label", value: "$label" });
      expect(rhs.parameters[0]?.name).toBe("label");
      expect(rhs.parameters[0]?.default).toBe("nakagin-core");
    });

    it("var mapping resolves match and where nodes", () => {
      expect(rewriteVarForNodeId(REWRITE_DEFAULT_LHS_FIXTURE, "match-a")).toBe("a");
      expect(rewriteVarForNodeId(REWRITE_DEFAULT_LHS_FIXTURE, "where-b")).toBe("a");
      expect(rewriteNodeIdsForVar(REWRITE_DEFAULT_LHS_FIXTURE, "a")).toEqual(["match-a", "where-b"]);
    });

    it("rewriteLhsMatchQuery builds RETURN query", () => {
      const lhsJson = rewriteLhsGraphToJson(REWRITE_DEFAULT_LHS_FIXTURE);
      const query = rewriteLhsMatchQuery(lhsJson, "a");
      expect(query).toContain("MATCH (a:Piece)");
      expect(query).toContain("WHERE a.name = 'b'");
      expect(query).toContain("RETURN a");
    });

    it("lhs json round-trips through graph adapter", () => {
      const json = rewriteLhsGraphToJson(REWRITE_DEFAULT_LHS_FIXTURE);
      const graph = rewriteLhsJsonToGraph(json);
      expect(rewriteLhsGraphToJson(graph)).toContain('"leftVar": "a"');
    });

    it("default lhs graph uses node-qualified handle ids for edge sync", () => {
      expect(REWRITE_DEFAULT_LHS_FIXTURE.nodes[0]?.handles[0]?.id).toBe("match-a:out");
      expect(REWRITE_DEFAULT_LHS_FIXTURE.edges[0]?.source).toBe("match-a:out");
      expect(REWRITE_DEFAULT_LHS_FIXTURE.edges[0]?.target).toBe("where-b:in");
    });
  });
}
// #endregion 🧪Tests
