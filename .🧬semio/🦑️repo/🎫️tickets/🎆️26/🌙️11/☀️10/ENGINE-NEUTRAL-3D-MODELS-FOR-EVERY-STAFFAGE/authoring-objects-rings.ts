import { loadAsset, resolveAsset, restState, bboxOf } from "./volume-lib.ts";
const [slug, view, ...ids] = process.argv.slice(2);
const st: any = restState(loadAsset(resolveAsset(slug!).path), 50);
for (const e of st[view!]) if (ids.includes(e.shape.id)) { const b = bboxOf(e.rings.flat()); console.log(e.shape.id, [b.minX, b.minY, b.maxX, b.maxY].map((x: number) => x.toFixed(3)).join(" ")); }
