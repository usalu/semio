import { writeFileSync } from "node:fs";

const d3 = (await Bun.file("puzzle/3d/fixture/nakagin-capsule-tower.3d.json").json()) as {
  meta: { kindCatalogs: { vortices: { id: string; name?: string; label?: string; color?: string }[]; objects: { id: string; name?: string; label?: string }[] }; kindCompatibility: { source: string; target: string; specificity?: string; bidirectional?: boolean }[] };
};
const d2 = (await Bun.file("puzzle/2d/fixture/nakagin-capsule-tower.2d.json").json()) as {
  meta: { kindCatalogs: { handles: { id: string; name: string }[]; nodes: { id: string; name: string; handles?: { handleKind: string; angle: number; radius?: number }[] }[] }; kindCompatibility: unknown[] };
};

const handleIdMap = new Map<string, string>();
for (const row of d2.meta.kindCatalogs.handles ?? []) {
  handleIdMap.set(row.id, row.name);
}

const nodeIdMap = new Map<string, string>();
const nodeHandlesByName = new Map<string, { handleKind: string; angle: number; radius?: number }[]>();
for (const row of d2.meta.kindCatalogs.nodes ?? []) {
  const name = row.name?.trim() ?? "";
  if (name.startsWith("semio.")) continue;
  nodeIdMap.set(row.id, name);
  if (row.handles?.length) {
    const existing = nodeHandlesByName.get(name) ?? [];
    nodeHandlesByName.set(name, existing.length >= row.handles.length ? existing : row.handles);
  }
}

function remapHandleKind(hk: string): string {
  return handleIdMap.get(hk) ?? hk;
}

function remapNodeTemplates(handles: { handleKind: string; angle: number; radius?: number }[] | undefined) {
  if (!handles?.length) return undefined;
  return handles.map((h) => ({ ...h, handleKind: remapHandleKind(h.handleKind) }));
}

const kc3 = d3.meta.kindCatalogs;
const kindCatalogs = {
  handles: (kc3.vortices ?? []).map((v) => ({
    id: v.id,
    name: v.name ?? v.label ?? v.id,
    color: v.color,
    defaultWireKind: "wire.link",
  })),
  nodes: (kc3.objects ?? []).map((o) => {
    const templates = remapNodeTemplates(nodeHandlesByName.get(o.id));
    const row: { id: string; name: string; handles?: { handleKind: string; angle: number; radius?: number }[] } = {
      id: o.id,
      name: o.name ?? o.label ?? o.id,
    };
    if (templates?.length) row.handles = templates;
    return row;
  }),
  wires: [{ id: "wire.link", name: "Link", defaultEdgeKind: "edge.link" }],
  edges: [{ id: "edge.link", name: "Link" }],
};

const specificityMap: Record<string, string> = {
  vortex: "handle",
  cable: "wire",
  attraction: "edge",
  object: "node",
  general: "general",
};

const kindCompatibility = (d3.meta.kindCompatibility ?? []).map((e) => ({
  ...e,
  source: handleIdMap.get(e.source) ?? e.source,
  target: handleIdMap.get(e.target) ?? e.target,
  specificity: e.specificity ? (specificityMap[e.specificity] ?? e.specificity) : undefined,
}));

function remapValue(v: unknown): unknown {
  if (typeof v === "string") {
    if (nodeIdMap.has(v)) return nodeIdMap.get(v)!;
    if (handleIdMap.has(v)) return handleIdMap.get(v)!;
    return v;
  }
  if (Array.isArray(v)) return v.map(remapValue);
  if (v && typeof v === "object") {
    const out: Record<string, unknown> = {};
    for (const [k, val] of Object.entries(v as Record<string, unknown>)) {
      out[k] = remapValue(val);
    }
    return out;
  }
  return v;
}

const out = remapValue({ ...d2, meta: { ...d2.meta, kindCatalogs, kindCompatibility } });
const path = "puzzle/2d/fixture/nakagin-capsule-tower.2d.json";
writeFileSync(path, `${JSON.stringify(out, null, 2)}\n`);

const left = JSON.stringify(out).match(/semio\.metabolism/g)?.length ?? 0;
console.log(`written ${path}; remaining semio.metabolism: ${left}`);
