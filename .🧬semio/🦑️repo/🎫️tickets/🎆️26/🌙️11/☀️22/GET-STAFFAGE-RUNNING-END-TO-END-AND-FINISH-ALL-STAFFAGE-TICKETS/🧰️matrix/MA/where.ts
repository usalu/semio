/** 🗺️ Where the outline of a trial copy differs from its base in the default pose: `bun where.ts <base.json> <trial.json> --scale 50 [--cell 2] [--min 5]` rasters the union of the filled closed shapes of every view (cell in mm), and prints every connected region that only one of the two covers, with its thickness (twice the largest distance to its edge, mm), its box in mm and the shapes of the covering drawing that draw it. Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const engine = await import(join(S, "🧪️tests/🔭️multiview-coverage/🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
const cell = Number(flag("cell") ?? 2) / 1000;
const min = Number(flag("min") ?? 5);
const key = flag("scale") ?? "50";
const states = [args[0]!, args[1]!].map((file) => {
  const asset = readDoc(file).doc.asset;
  return evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key)));
});
for (const view of ["elevation", "front", "plan"]) {
  const all = states.flatMap((s: any) => s.shapes[view].filter((x: any) => x.closed && x.shape.role !== "ground").flatMap((x: any) => x.rings.flat()));
  const [x0, x1, y0, y1] = [Math.min(...all.map((p: any) => p.x)) - 2 * cell, Math.max(...all.map((p: any) => p.x)) + 2 * cell, Math.min(...all.map((p: any) => p.y)) - 2 * cell, Math.max(...all.map((p: any) => p.y)) + 2 * cell];
  const [nx, ny] = [Math.ceil((x1 - x0) / cell), Math.ceil((y1 - y0) / cell)];
  const raster = (rings: any[]): Uint8Array => {
    const on = new Uint8Array(nx * ny);
    for (let j = 0; j < ny; j++) {
      const y = y0 + (j + 0.5) * cell;
      const xs: number[] = [];
      for (const ring of rings) for (let i = 0; i < ring.length; i++) {
        const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
        if (p.y > y !== q.y > y) xs.push(p.x + ((q.x - p.x) * (y - p.y)) / (q.y - p.y));
      }
      xs.sort((a, b) => a - b);
      for (let k = 0; k + 1 < xs.length; k += 2) for (let i = Math.max(0, Math.round((xs[k]! - x0) / cell)); i < Math.min(nx, Math.round((xs[k + 1]! - x0) / cell)); i++) on[j * nx + i] = 1;
    }
    return on;
  };
  const layers = states.map((s: any) => s.shapes[view].filter((x: any) => x.closed && x.shape.filled !== false && x.shape.role !== "ground").map((x: any) => ({ id: `${x.shape.bone}/${x.shape.id}`, on: raster(x.rings) })));
  const unions = layers.map((list) => {
    const u = new Uint8Array(nx * ny);
    for (const layer of list) for (let i = 0; i < u.length; i++) u[i]! |= layer.on[i]!;
    return u;
  });
  for (const side of [0, 1]) {
    const only = new Uint8Array(nx * ny);
    for (let i = 0; i < only.length; i++) only[i] = unions[side]![i]! & ~unions[1 - side]! & 1;
    const depth = new Int32Array(only);
    for (let round = 1, more = true; more; round++) {
      more = false;
      const grow: number[] = [];
      for (let i = 0; i < only.length; i++) if (depth[i] === round && [i - 1, i + 1, i - nx, i + nx].every((n) => n >= 0 && n < only.length && depth[n]! >= round)) grow.push(i);
      for (const i of grow) depth[i] = round + 1;
      more = grow.length > 0;
    }
    const seen = new Uint8Array(only.length);
    for (let start = 0; start < only.length; start++) {
      if (!only[start] || seen[start]) continue;
      const stack = [start];
      seen[start] = 1;
      let [a0, a1, b0, b1, thick] = [nx, 0, ny, 0, 0];
      const cells: number[] = [];
      while (stack.length > 0) {
        const i = stack.pop()!;
        cells.push(i);
        const [cx, cy] = [i % nx, Math.floor(i / nx)];
        [a0, a1, b0, b1, thick] = [Math.min(a0, cx), Math.max(a1, cx), Math.min(b0, cy), Math.max(b1, cy), Math.max(thick, depth[i]!)];
        for (const n of [i - 1, i + 1, i - nx, i + nx]) if (n >= 0 && n < only.length && only[n] && !seen[n]) (seen[n] = 1), stack.push(n);
      }
      const mm = thick * 2 * cell * 1000;
      if (mm < min) continue;
      const owners = layers[side]!.filter((layer: any) => cells.filter((i) => layer.on[i]).length > 0.3 * cells.length).map((layer: any) => layer.id);
      console.log(`1:${key} ${view} only ${side === 0 ? "base" : "trial"} ${mm.toFixed(0)} mm thick, x [${Math.round((x0 + a0 * cell) * 1000)}, ${Math.round((x0 + a1 * cell) * 1000)}] y [${Math.round((y0 + b0 * cell) * 1000)}, ${Math.round((y0 + b1 * cell) * 1000)}]: ${owners.join(", ")}`);
    }
  }
}
