/** 🃏 Jack query execution for board-style graph fixtures in TypeScript. */
import { jackSymbolAtOffset, jackVariableOccurrences } from "@semio-tech/writer-core";

export type JackResultKind = "table" | "graph";

export interface JackRunResult {
  readonly kind: JackResultKind;
  readonly columns: readonly string[];
  readonly rows: readonly (readonly unknown[])[];
  readonly graphFixtureJson?: string;
}

interface BoardNode {
  readonly id: string;
  readonly kind: string;
  readonly name: string;
  readonly properties: Readonly<Record<string, unknown>>;
}

interface BoardEdge {
  readonly id: string;
  readonly kind: string;
  readonly sourceNodeId: string;
  readonly targetNodeId: string;
}

function readString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

function boardGraphFromFixture(fixtureJson: string): { nodes: BoardNode[]; edges: BoardEdge[]; raw: unknown } {
  const raw = JSON.parse(fixtureJson) as Record<string, unknown>;
  const handleToNode = new Map<string, string>();
  const nodes: BoardNode[] = [];
  const nodeRows = Array.isArray(raw.nodes) ? raw.nodes : [];
  for (const row of nodeRows) {
    if (!row || typeof row !== "object") continue;
    const obj = row as Record<string, unknown>;
    const id = readString(obj.id);
    if (!id) continue;
    const kind = readString(obj.nodeKind) ?? readString(obj.node_kind) ?? readString(obj.kind) ?? "";
    const name = readString(obj.text) ?? readString(obj.name) ?? readString(obj.label) ?? id;
    const properties: Record<string, unknown> = {};
    const userData = obj.userData ?? obj.user_data;
    if (userData && typeof userData === "object" && !Array.isArray(userData)) {
      Object.assign(properties, userData as Record<string, unknown>);
    }
    for (const [key, value] of Object.entries(obj)) {
      if (["id", "nodeKind", "node_kind", "kind", "text", "name", "label", "handles", "x", "y", "shape", "radius", "width", "height", "userData", "user_data"].includes(key)) continue;
      properties[key] = value;
    }
    nodes.push({ id, kind, name, properties });
    const handles = Array.isArray(obj.handles) ? obj.handles : [];
    for (const handle of handles) {
      if (!handle || typeof handle !== "object") continue;
      const hid = readString((handle as Record<string, unknown>).id);
      if (hid) handleToNode.set(hid, id);
    }
  }
  const edges: BoardEdge[] = [];
  const edgeRows = Array.isArray(raw.edges) ? raw.edges : [];
  for (const row of edgeRows) {
    if (!row || typeof row !== "object") continue;
    const obj = row as Record<string, unknown>;
    const id = readString(obj.id);
    const source = readString(obj.source);
    const target = readString(obj.target);
    if (!id || !source || !target) continue;
    const kind = readString(obj.edgeKind) ?? readString(obj.edge_kind) ?? readString(obj.kind) ?? "";
    const sourceNodeId = handleToNode.get(source) ?? source.split(":")[0] ?? source;
    const targetNodeId = handleToNode.get(target) ?? target.split(":")[0] ?? target;
    edges.push({ id, kind, sourceNodeId, targetNodeId });
  }
  return { nodes, edges, raw };
}

function nodeProperty(node: BoardNode, prop: string): unknown {
  if (prop === "id") return node.id;
  if (prop === "name" || prop === "label" || prop === "text") return node.name;
  if (prop === "kind") return node.kind;
  return node.properties[prop] ?? null;
}

function parseMatchReturn(query: string): { nodeKind: string; returnProp: string } | null {
  const match = query.match(/MATCH\s*\(\s*\w+\s*:\s*([A-Za-z0-9_.-]+)\s*\)\s*RETURN\s*\w+\.(\w+)/i);
  if (!match) return null;
  return { nodeKind: match[1]!, returnProp: match[2]! };
}

/** ▶️ Run a simple Jack query against a board fixture JSON document. */
export function runJackOnBoardFixture(fixtureJson: string, query: string): JackRunResult {
  const parsed = parseMatchReturn(query.trim());
  const { nodes, edges, raw } = boardGraphFromFixture(fixtureJson);
  if (!parsed) {
    return { kind: "table", columns: [], rows: [] };
  }
  const matched = nodes.filter((node) => node.kind === parsed.nodeKind);
  const columns = [`${parsed.returnProp}`];
  const rows = matched.map((node) => [nodeProperty(node, parsed.returnProp)]);
  if (/RETURN\s+\w+\s*$/i.test(query.trim())) {
    const subset = {
      ...(typeof raw === "object" && raw ? raw : {}),
      nodes: matched,
      edges: edges.filter((edge) => matched.some((node) => node.id === edge.sourceNodeId || node.id === edge.targetNodeId)),
    };
    return { kind: "graph", columns: ["graph"], rows: [], graphFixtureJson: JSON.stringify(subset) };
  }
  return { kind: "table", columns, rows };
}

export function jackHoverOccurrencesForQuery(query: string, activeVar: string | null): readonly { readonly start: number; readonly end: number }[] {
  if (!activeVar) return [];
  return jackVariableOccurrences(query, activeVar);
}

export function jackVarAtOffset(query: string, offset: number | null): string | null {
  if (offset == null) return null;
  const symbol = jackSymbolAtOffset(query, offset);
  return symbol?.kind === "variable" ? symbol.name : null;
}

export function nodeIdsForJackVar(fixtureJson: string, query: string, varName: string): readonly string[] {
  const match = query.match(new RegExp(`MATCH\\s*\\(\\s*${varName}\\s*:\\s*([A-Za-z0-9_.-]+)`, "i"));
  if (!match) return [];
  const kind = match[1]!;
  const { nodes } = boardGraphFromFixture(fixtureJson);
  return nodes.filter((node) => node.kind === kind).map((node) => node.id);
}

export function jackVarForBoardNodeId(fixtureJson: string, query: string, nodeId: string): string | null {
  const { nodes } = boardGraphFromFixture(fixtureJson);
  const node = nodes.find((row) => row.id === nodeId);
  if (!node) return null;
  const re = /MATCH\s*\(\s*(\w+)\s*:\s*([A-Za-z0-9_.-]+)\s*\)/gi;
  let match: RegExpExecArray | null;
  while ((match = re.exec(query))) {
    if (match[2] === node.kind) return match[1]!;
  }
  return null;
}

export function runJackOnMediaGraph(graph: { readonly nodes: readonly { readonly id: string; readonly kind: string; readonly label?: string }[] }, query: string): JackRunResult {
  const fixtureJson = JSON.stringify({ nodes: graph.nodes.map((node) => ({ id: node.id, kind: node.kind, text: node.label ?? node.id, nodeKind: node.kind })), edges: [] });
  return runJackOnBoardFixture(fixtureJson, query);
}

export function runJackOnPuzzle3dFixture(fixtureJson: string, query: string): JackRunResult {
  return runJackOnBoardFixture(fixtureJson, query);
}
