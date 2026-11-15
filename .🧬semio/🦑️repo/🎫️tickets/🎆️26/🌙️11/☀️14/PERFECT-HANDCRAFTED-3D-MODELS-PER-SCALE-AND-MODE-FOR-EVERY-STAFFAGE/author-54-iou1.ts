import { openCtx, drawn } from "./kit-lib.ts";
import { meshesAt, meshMask, Mask } from "./volumes-lib.ts";
const [spec, sc, id] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const c = openCtx(spec, sc);
const m = meshesAt(c.m, sc, c.vol.body.filter((x: any) => x.id === id));
const box = { minX: -1.5, maxX: 1.5, minY: -1.5, maxY: 1.5 };
for (const view of ["elevation", "front", "plan"] as const) {
  const cell = 0.005;
  const a = meshMask(view, m, box, cell);
  const b = new Mask(box, cell);
  for (const d of drawn(c, view)) if (d.id === id) b.fill(d.rings);
  let i = 0, u = 0, A = 0, B = 0;
  for (let k = 0; k < a.cells.length; k++) { const x = a.cells[k]!, y = b.cells[k]!; A += x; B += y; if (x && y) i++; if (x || y) u++; }
  console.log(view, "3D", (A * cell * cell).toFixed(4), "2D", (B * cell * cell).toFixed(4), "IoU", (i / u).toFixed(3));
}
