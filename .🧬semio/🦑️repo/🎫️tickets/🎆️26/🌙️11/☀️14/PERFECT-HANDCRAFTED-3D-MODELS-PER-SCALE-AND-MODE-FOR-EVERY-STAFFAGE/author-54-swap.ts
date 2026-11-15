import { openCtx, drawn } from "./kit-lib.ts";
import { meshesAt, meshMask, Mask } from "./volumes-lib.ts";
const [spec, sc, id] = [process.argv[2]!, Number(process.argv[3]), process.argv[4]!];
const c = openCtx(spec, sc);
const orig = c.vol.body.find((x: any) => x.id === id);
const swapped = JSON.parse(JSON.stringify(orig));
swapped.sections = swapped.sections.map((s: any) => ({ ...s, at: [s.at[0], s.at[2], s.at[1]], rx: s.ry, ry: s.rx }));
const box = { minX: -1.5, maxX: 1.5, minY: -1.5, maxY: 1.5 };
for (const [name, sol] of [["orig", orig], ["swap", swapped]] as const) {
  const m = meshesAt(c.m, sc, [sol]);
  const out: string[] = [];
  for (const view of ["elevation", "front", "plan"] as const) {
    const cell = 0.005;
    const a = meshMask(view, m, box, cell);
    const b = new Mask(box, cell);
    for (const d of drawn(c, view)) if (d.id === id) b.fill(d.rings);
    let i = 0, u = 0;
    for (let k = 0; k < a.cells.length; k++) { const x = a.cells[k]!, y = b.cells[k]!; if (x && y) i++; if (x || y) u++; }
    out.push(`${view} ${(i / u).toFixed(3)}`);
  }
  console.log(name, id, out.join("  "));
}
