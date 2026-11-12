import { GENERATED, loadAsset, resolveAsset, restState, simplifyRing } from "./volume-lib.ts";
const [slug, view, tol, ...ids] = process.argv.slice(2);
const asset = loadAsset(resolveAsset(slug!).path);
const st = (restState(asset, 50) as any)[view!];
const r = (x: number) => Math.round(x * 1000) / 1000;
for (const s of st) {
  if (ids.length && !ids.some((i) => s.shape.id === i || s.shape.id.startsWith(i + "-"))) continue;
  s.rings.forEach((ring: any, k: number) => {
    const q = simplifyRing(ring, Number(tol));
    console.log(`${s.shape.id}#${k}${s.closed ? "" : " open"}: ` + q.map((p: any) => `(${r(p.x)},${r(p.y)})`).join(" "));
  });
}
