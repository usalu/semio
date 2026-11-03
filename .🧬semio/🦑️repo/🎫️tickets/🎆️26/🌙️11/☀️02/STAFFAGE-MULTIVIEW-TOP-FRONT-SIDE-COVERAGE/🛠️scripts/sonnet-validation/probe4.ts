const L = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { loadCatalog, extentOf, sceneBox, drawOptions } = await import(`${L}/🧪️tests/🔭️multiview-coverage/🧰️.ts`);
const { resolveViews } = await import(`${L}/🔨️modules/🔭️multiview/🟦️.ts`);
for (const [id, den] of [["cyclist", 100], ["cyclist", 50], ["bamboo-clump", 100], ["goat", 100], ["goat", 50], ["dining-table", 100]] as const) {
  const a = loadCatalog().find((x: any) => x.id === id)!;
  const mv = resolveViews(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", denominator: den }, "light-line", drawOptions("light-line", den));
  const st = mv.state; if (!st) { console.log(id, den, "no state", (mv.views.elevation as any).code); continue; } const d = mv.views.elevation as any; const k = 1000 / den;
  const ex = extentOf(st.shapes.elevation)!;
  const ids = st.shapes.elevation.filter((s: any) => s.shape.role !== "ground").map((s: any) => `shape:${s.shape.id}`);
  const box = sceneBox(d.scene, new Set(ids))!; const all = sceneBox(d.scene)!;
  const present = new Set(d.scene.layers.flatMap((l: any) => l.items.map((i: any) => i.id)));
  const missing = ids.filter((i: string) => !present.has(i));
  console.log(id, den, "extent mm", ((ex.maxX - ex.minX) * k).toFixed(2), ((ex.maxY - ex.minY) * k).toFixed(2), "named", (box.maxX - box.minX).toFixed(2), (box.maxY - box.minY).toFixed(2), "allfill", (all.maxX - all.minX).toFixed(2), "dims", a.dimensions.widthM * k, "ids", ids.length, "missing", missing.length, missing.slice(0, 5).join(","));
}
