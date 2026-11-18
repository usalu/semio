/** 🧱 Ops that attach a thin extruded solid (elevation sweep w, front and plan sweep v, +-5 mm) to the flat shapes the validator reports as lateral-ambiguous. bun humansb-A-solids.ts <slug> <out.json> */
import { join, resolve } from "node:path";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
const [slug, out] = process.argv.slice(2) as [string, string];
const file = join("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans", slug, "🔣️.json");
const asset = (await Bun.file(file).json()).asset;
const ops: any[] = [];
const seen = new Set<string>();
for (const issue of validateAsset(asset)) {
  if (issue.code !== "lateral-ambiguous") continue;
  const data = (issue as any).data as { shapes: string[]; bands: string[] };
  for (const band of data.bands) {
    const [, scale, view] = /^1:(\d+) (\w+)$/.exec(band)!;
    for (const id of data.shapes) {
      const shape = asset.representations[scale!].geometry[view!].find((s: any) => s.id === id);
      const key = `${scale}/${view}/${id}`;
      if (shape === undefined || shape.solid !== undefined || seen.has(key)) continue;
      seen.add(key);
      ops.push({ op: "shape-set", scale, view, id, set: { solid: { kind: "extruded", sweep: view === "elevation" ? "w" : "v", z0: -0.005, z1: 0.005 } } });
    }
  }
}
await Bun.write(out, JSON.stringify(ops));
console.log(ops.length, "ops");
