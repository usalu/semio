import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const mod: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
const [spec, a, b, list = "body"] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
for (const sc of [a, b]) {
  const s = mod.solidsOf ? m.volumes[sc!][list] : m.volumes[sc!][list];
  const rows = s.map((x: any) => [x.id, mod.fixedTriangleCount([x])]);
  rows.sort((p: any, q: any) => q[1] - p[1]);
  console.log(sc, list, "total", mod.fixedTriangleCount(s), rows.slice(0, 8).map((r: any) => `${r[0]} ${r[1]}`).join(", "));
}
