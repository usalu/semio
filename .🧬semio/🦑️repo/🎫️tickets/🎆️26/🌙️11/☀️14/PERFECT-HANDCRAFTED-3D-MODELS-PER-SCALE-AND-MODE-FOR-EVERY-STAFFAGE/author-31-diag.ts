import { entryOf, loadSource, merged, scaleMeshes, meshMask, Mask, type Json } from "./volumes-lib.ts";
const [spec, S, list = "structure"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const source = loadSource(entry)!;
const m = merged(asset, source);
const vol = (m.volumes ?? source.volumes)[S!];
const sm = scaleMeshes(m, Number(S), vol);
const cell = Number(process.env.CELL ?? 0.02);
const box = { minX: -8, maxX: 8, minY: -8, maxY: 14 };
for (const view of ["elevation", "front", "plan"] as const) {
  const body = meshMask(view as any, sm.body, box, cell);
  const rows: string[] = [];
  sm[list as "structure"].forEach((mesh, i) => {
    const mk = meshMask(view as any, [mesh], box, cell);
    let out = 0, tot = 0;
    for (let j = 0; j < mk.cells.length; j++) if (mk.cells[j]) { tot++; if (!body.cells[j]) out++; }
    if (out) rows.push(`${sm.solids[list as "structure"][i]!.id} out ${(out * cell * cell).toFixed(3)} m2 of ${(tot * cell * cell).toFixed(3)}`);
  });
  console.log(view, rows.join(" | "));
}
