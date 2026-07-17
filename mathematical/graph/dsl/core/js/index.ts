/** 🃏 Jack query execution for board-style graph fixtures in TypeScript. */

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

function jackSymbolAtOffset(query: string, offset: number): { readonly kind: "variable" | "other"; readonly name: string } | null {
  const pattern = /\b([A-Za-z_][A-Za-z0-9_]*)\b/g;
  for (const match of query.matchAll(pattern)) {
    if (match.index == null) continue;
    const start = match.index;
    const end = start + match[0].length;
    if (offset < start || offset >= end) continue;
    const name = match[1]!;
    const before = query.slice(Math.max(0, start - 24), start);
    const isVariable = /:\s*$/.test(before) || /\(\s*$/.test(before) || /\bAS\s+$/i.test(before);
    return { kind: isVariable ? "variable" : "other", name };
  }
  return null;
}

function jackVariableOccurrences(query: string, varName: string): readonly { readonly start: number; readonly end: number }[] {
  const pattern = new RegExp(`\\b${varName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\b`, "g");
  const occurrences: { start: number; end: number }[] = [];
  for (const match of query.matchAll(pattern)) {
    if (match.index == null) continue;
    occurrences.push({ start: match.index, end: match.index + match[0].length });
  }
  return occurrences;
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

export function runJackOnMediaGraph(graph: { readonly nodes: readonly { readonly id: string; readonly instanceId: string }[] }, instances: readonly { readonly id: string; readonly programId: string }[], query: string): JackRunResult {
  const instanceById = new Map(instances.map((row) => [row.id, row]));
  const fixtureJson = JSON.stringify({
    nodes: graph.nodes.map((node) => {
      const instance = instanceById.get(node.instanceId);
      const kind = instance?.programId ?? "app";
      return { id: node.id, kind, nodeKind: kind, text: instance?.programId ?? node.id };
    }),
    edges: [],
  });
  return runJackOnBoardFixture(fixtureJson, query);
}

export function mediaGraphFixtureJson(graph: { readonly nodes: readonly { readonly id: string; readonly instanceId: string }[] }, instances: readonly { readonly id: string; readonly programId: string }[]): string {
  const instanceById = new Map(instances.map((row) => [row.id, row]));
  return JSON.stringify({
    nodes: graph.nodes.map((node) => {
      const instance = instanceById.get(node.instanceId);
      const kind = instance?.programId ?? "app";
      return { id: node.id, kind, nodeKind: kind, text: instance?.programId ?? node.id };
    }),
    edges: [],
  });
}

export function runJackOnPuzzle3dFixture(fixtureJson: string, query: string): JackRunResult {
  const raw = JSON.parse(fixtureJson) as Record<string, unknown>;
  if (!Array.isArray(raw.nodes) && Array.isArray(raw.objects)) {
    const objects = raw.objects as Record<string, unknown>[];
    const nodes = objects
      .map((obj) => {
        const id = typeof obj.id === "string" ? obj.id : null;
        if (!id) return null;
        const kind = (typeof obj.objectKind === "string" ? obj.objectKind : typeof obj.kind === "string" ? obj.kind : "Object") as string;
        const name = (typeof obj.name === "string" ? obj.name : typeof obj.label === "string" ? obj.label : id) as string;
        return { id, nodeKind: kind, text: name };
      })
      .filter((row): row is { id: string; nodeKind: string; text: string } => Boolean(row));
    return runJackOnBoardFixture(JSON.stringify({ ...raw, nodes }), query);
  }
  return runJackOnBoardFixture(fixtureJson, query);
}

function formatWirePropertyValue(value: unknown): string {
  if (value == null) return "null";
  if (typeof value === "string") return `'${value.replace(/'/g, "\\'")}'`;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  if (Array.isArray(value)) return `[${value.map(formatWirePropertyValue).join(", ")}]`;
  if (typeof value === "object") {
    const entries = Object.entries(value as Record<string, unknown>).map(([key, row]) => `${key}: ${formatWirePropertyValue(row)}`);
    return `{${entries.join(", ")}}`;
  }
  return "null";
}

function formatWireProperties(properties: Readonly<Record<string, unknown>> | undefined): string {
  if (!properties || Object.keys(properties).length === 0) return "";
  const inner = Object.entries(properties)
    .map(([key, value]) => `${key}: ${formatWirePropertyValue(value)}`)
    .join(", ");
  return `{${inner}}`;
}

function splitWireEndpoint(endpoint: string): { readonly node: string; readonly port: string } {
  if (endpoint.includes(":")) {
    const [node, port] = endpoint.split(":");
    return { node: node ?? endpoint, port: port ?? "out" };
  }
  if (endpoint.includes(".")) {
    const [node, port] = endpoint.split(".");
    return { node: node ?? endpoint, port: port ?? "out" };
  }
  if (endpoint.includes("@")) {
    const [node, port] = endpoint.split("@");
    return { node: node ?? endpoint, port: port ?? "out" };
  }
  return { node: endpoint, port: "out" };
}

function dagNodeKind(row: Record<string, unknown>): string {
  return readString(row.operatorKind) ?? readString(row.operator_kind) ?? readString(row.programId) ?? readString(row.nodeKind) ?? readString(row.kind) ?? "node";
}

/** @emoji 🔌 Render a DAG fixture JSON document as wire-literal compiled text. */
export function wireLiteralFromDagFixtureJson(fixtureJson: string): string {
  const raw = JSON.parse(fixtureJson) as Record<string, unknown>;
  const nodeRows = Array.isArray(raw.nodes) ? raw.nodes : [];
  const edgeRows = Array.isArray(raw.edges) ? raw.edges : [];
  const nodes = nodeRows
    .filter((row): row is Record<string, unknown> => Boolean(row && typeof row === "object"))
    .map((row) => ({
      id: readString(row.id) ?? "",
      kind: dagNodeKind(row),
      properties: (row.properties && typeof row.properties === "object" && !Array.isArray(row.properties) ? row.properties : {}) as Record<string, unknown>,
    }))
    .filter((row) => row.id);
  const lines: string[] = [];
  for (const node of nodes) {
    const props = formatWireProperties(node.properties);
    lines.push(props ? `${node.id}:${node.kind}${props}` : `${node.id}:${node.kind}`);
  }
  for (const row of edgeRows) {
    if (!row || typeof row !== "object") continue;
    const source = readString((row as Record<string, unknown>).source);
    const target = readString((row as Record<string, unknown>).target);
    if (!source || !target) continue;
    const from = splitWireEndpoint(source);
    const to = splitWireEndpoint(target);
    const fromKind = nodes.find((node) => node.id === from.node)?.kind ?? "node";
    const toKind = nodes.find((node) => node.id === to.node)?.kind ?? "node";
    const props = formatWireProperties((row as Record<string, unknown>).properties && typeof (row as Record<string, unknown>).properties === "object" ? ((row as Record<string, unknown>).properties as Record<string, unknown>) : undefined);
    lines.push(`${from.node}:${fromKind}@${from.port}->${to.node}:${toKind}@${to.port}${props}`);
  }
  return lines.join("\n");
}

function escapeWireRegex(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

/** @emoji 📍 Highlight spans for a node id across wire-literal node declarations and edge endpoints. */
export function wireNodeOccurrences(text: string, nodeId: string): readonly { readonly start: number; readonly end: number }[] {
  if (!nodeId) return [];
  const escaped = escapeWireRegex(nodeId);
  const re = new RegExp(`(?:^|[\\n\\r]|->|-)(${escaped})(?=:)`, "gm");
  const out: { start: number; end: number }[] = [];
  let match: RegExpExecArray | null;
  while ((match = re.exec(text))) {
    const start = match.index + match[0].length - match[1]!.length;
    out.push({ start, end: start + match[1]!.length });
  }
  return out;
}

/** @emoji 🎯 Resolve the wire node id at a text offset (cursor must sit on the id token before `:`). */
export function wireNodeIdAtOffset(text: string, offset: number | null): string | null {
  if (offset == null || offset < 0 || offset > text.length) return null;
  const clamped = Math.min(offset, Math.max(0, text.length - 1));
  let start = clamped;
  while (start > 0 && /[A-Za-z0-9_-]/.test(text[start - 1]!)) start -= 1;
  let end = start;
  while (end < text.length && /[A-Za-z0-9_-]/.test(text[end]!)) end += 1;
  const id = text.slice(start, end);
  if (!id || text[end] !== ":") return null;
  const before = start > 0 ? text[start - 1] : undefined;
  if (before !== undefined && before !== "\n" && before !== "\r" && before !== ">" && before !== "-") return null;
  return id;
}

export type WireEndpointRef = {
  readonly widgetId: string;
  readonly port: string;
  readonly direction: "in" | "out";
};

export type WireHoverTarget = { readonly kind: "node"; readonly nodeId: string } | { readonly kind: "endpoint"; readonly widgetId: string; readonly port: string; readonly direction: "in" | "out" };

function wireLineBounds(text: string, offset: number): { readonly start: number; readonly end: number; readonly line: string; readonly lineOffset: number } {
  const start = text.lastIndexOf("\n", offset - 1) + 1;
  const nextBreak = text.indexOf("\n", offset);
  const end = nextBreak === -1 ? text.length : nextBreak;
  return { start, end, line: text.slice(start, end), lineOffset: offset - start };
}

function parseWireEndpointSegment(segment: string): { readonly widgetId: string; readonly port: string } | null {
  const match = segment.trim().match(/^([A-Za-z0-9_-]+):[^@]+@([A-Za-z0-9_-]+)/);
  if (!match) return null;
  return { widgetId: match[1]!, port: match[2]! };
}

/** @emoji 📍 Highlight spans for a port token on wire edge endpoints. */
export function wireEndpointOccurrences(text: string, widgetId: string, port: string, direction: "in" | "out"): readonly { readonly start: number; readonly end: number }[] {
  if (!widgetId || !port) return [];
  const escapedId = escapeWireRegex(widgetId);
  const escapedPort = escapeWireRegex(port);
  const pattern = direction === "out" ? `(?:^|[\\n\\r])${escapedId}:[^@\\n\\r]+@(${escapedPort})(?=->)` : `->${escapedId}:[^@\\n\\r]+@(${escapedPort})(?=[{\\s]|$)`;
  const re = new RegExp(pattern, "gm");
  const out: { start: number; end: number }[] = [];
  let match: RegExpExecArray | null;
  while ((match = re.exec(text))) {
    const portStart = match.index + match[0].length - match[1]!.length;
    const atStart = portStart - 1;
    if (atStart < 0 || text[atStart] !== "@") continue;
    out.push({ start: atStart, end: portStart + match[1]!.length });
  }
  return out;
}

/** @emoji 📍 Highlight spans for a wire hover target (node id or channel port). */
export function wireTargetOccurrences(text: string, target: WireHoverTarget | null): readonly { readonly start: number; readonly end: number }[] {
  if (!target) return [];
  if (target.kind === "node") return wireNodeOccurrences(text, target.nodeId);
  return wireEndpointOccurrences(text, target.widgetId, target.port, target.direction);
}

/** @emoji 🎯 Resolve node or port endpoint at a wire text offset. */
export function wireTargetAtOffset(text: string, offset: number | null): WireHoverTarget | null {
  if (offset == null || offset < 0 || offset > text.length) return null;
  const { line, lineOffset } = wireLineBounds(text, offset);
  const arrowIdx = line.indexOf("->");
  const atIdx = line.lastIndexOf("@", lineOffset);
  if (atIdx >= 0 && arrowIdx >= 0) {
    const portStart = atIdx + 1;
    let portEnd = portStart;
    while (portEnd < line.length && /[A-Za-z0-9_-]/.test(line[portEnd]!)) portEnd += 1;
    if (lineOffset >= portStart && lineOffset < portEnd) {
      const segment = lineOffset < arrowIdx ? line.slice(0, arrowIdx) : line.slice(arrowIdx + 2);
      const parsed = parseWireEndpointSegment(segment);
      if (parsed) {
        return {
          kind: "endpoint",
          widgetId: parsed.widgetId,
          port: parsed.port,
          direction: lineOffset < arrowIdx ? "out" : "in",
        };
      }
    }
  }
  const nodeId = wireNodeIdAtOffset(text, offset);
  if (nodeId) return { kind: "node", nodeId };
  return null;
}

export function wireEndpointRefsEqual(left: WireEndpointRef | null | undefined, right: WireEndpointRef | null | undefined): boolean {
  if (left === right) return true;
  if (!left || !right) return false;
  return left.widgetId === right.widgetId && left.port === right.port && left.direction === right.direction;
}

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("wire node hover helpers", () => {
    const sample = ["slider:core.number{value: 3}", "add:math.add", "slider:core.number@number->add:math.add@a"].join("\n");

    it("collects all node id occurrences in wire text", () => {
      expect(wireNodeOccurrences(sample, "slider")).toEqual([
        { start: 0, end: 6 },
        { start: 42, end: 48 },
      ]);
      expect(wireNodeOccurrences(sample, "add")).toEqual([
        { start: 29, end: 32 },
        { start: 69, end: 72 },
      ]);
    });

    it("resolves node id at offset only on id tokens", () => {
      expect(wireNodeIdAtOffset(sample, 2)).toBe("slider");
      expect(wireNodeIdAtOffset(sample, 45)).toBe("slider");
      expect(wireNodeIdAtOffset(sample, 20)).toBeNull();
    });

    it("highlights channel port tokens on edge lines", () => {
      expect(wireEndpointOccurrences(sample, "slider", "number", "out")).toEqual([{ start: 60, end: 67 }]);
      expect(wireEndpointOccurrences(sample, "add", "a", "in")).toEqual([{ start: 81, end: 83 }]);
    });

    it("resolves endpoint targets at port offsets", () => {
      expect(wireTargetAtOffset(sample, 62)).toEqual({ kind: "endpoint", widgetId: "slider", port: "number", direction: "out" });
      expect(wireTargetAtOffset(sample, 82)).toEqual({ kind: "endpoint", widgetId: "add", port: "a", direction: "in" });
    });
  });
}
