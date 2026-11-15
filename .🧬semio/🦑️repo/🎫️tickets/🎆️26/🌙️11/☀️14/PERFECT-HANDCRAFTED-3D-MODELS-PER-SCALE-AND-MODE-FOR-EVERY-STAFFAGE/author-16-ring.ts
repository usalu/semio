import { entryOf, restState } from "./volumes-lib.ts";
const [spec, sc, view, ...ids] = process.argv.slice(2);
const { asset } = entryOf(spec!);
const st = restState(asset, Number(sc));
for (const e of (st as any)[view!]) {
  if (ids.length && !ids.some((i) => e.shape.id === i)) continue;
  console.log(e.shape.id, e.shape.kind, JSON.stringify(e.rings.map((r: any) => r.filter((_: any, i: number) => i % 1 === 0).map((p: any) => [Math.round(p.x * 1000) / 1000, Math.round(p.y * 1000) / 1000]))));
}
