(async () => {
  const url = performance.getEntriesByType("resource").map((e) => e.name).find((n) => new URL(n).pathname === "/index.ts");
  const m = await import(url);
  const c = m.getSketchpadShellController();
  const kitId = location.pathname.split("/")[2];
  const s = c.getStore("topology:" + kitId + ":kit:wires");
  const snap = s?.getSnapshot?.();
  const flat = snap?.flat ?? snap?.payload?.flat ?? snap;
  const find = (o, key, depth = 0) => { if (!o || typeof o !== "object" || depth > 5) return null; if (key in o) return o[key]; for (const v of Object.values(o)) { const r = find(v, key, depth + 1); if (r) return r; } return null; };
  const nodes = find(snap, "nodes"); const edges = find(snap, "edges");
  return JSON.stringify({ keys: snap ? Object.keys(snap) : null, nodes: nodes?.length, edges: edges?.length, edge0: edges?.[0], node0: nodes?.[0], cache: [...(c.kitWiresReferenceCache?.get(kitId)?.pieceBlueprints ?? [])].length });
})()
