/** Per-solid exposed area of a thermal region (owned C1 engine): bun author-27-persolid.ts <asset> --scale N --source file --region id [--all] */
import { readFileSync } from "node:fs";
import { entryOf, featureM, merged, scaleMeshes } from "./volumes-lib.ts";
import { exposedArea, itemsOf, contactPlanes } from "./author-27-rulings.ts";

const a = process.argv.slice(2);
const f = (n: string) => (a.indexOf(n) < 0 ? undefined : a[a.indexOf(n) + 1]);
const { asset } = entryOf(a[0]!);
const src = JSON.parse(readFileSync(f("--source")!, "utf8"));
const m = merged(asset, src);
const scale = Number(f("--scale"));
const volume = src.volumes[String(scale)];
const sm = scaleMeshes(m, scale, volume);
const it = itemsOf(m, scale, sm, volume);
const wo = (f("--without") ?? "").split(",").filter((x) => x.length > 0);
const all = [...it.body, ...it.thermal, ...it.structure].filter((x: any) => !wo.some((w) => x.id === w || x.base === w || (w.startsWith("region:") && x.solid?.region === w.slice(7))));
const planes = contactPlanes(m, scale);
const feature = featureM(scale, m);
const regions = f("--region") === undefined ? (m.thermal.regions as any[]).map((r) => r.id) : [f("--region")!];
for (const rid of regions) {
  const r = (m.thermal.regions as any[]).find((x) => x.id === rid);
  const layers: string[] = (r.layers ?? []).map((l: any) => l.id);
  const rank = (x: any) => (x.solid.layer === undefined ? -1 : layers.indexOf(x.solid.layer));
  const own = it.thermal.filter((x: any) => x.solid.region === rid);
  const top = Math.max(...own.map(rank));
  const outer = own.filter((x: any) => rank(x) === top);
  let sa = 0, st = 0;
  const rows: string[] = [];
  for (const x of outer) {
    const e = exposedArea([x], all, planes, feature);
    sa += e.area; st += e.total;
    rows.push(`    ${x.id.padEnd(22)} exposed ${e.area.toFixed(4)} of ${e.total.toFixed(4)}`);
  }
  console.log(`${rid}: exposed ${sa.toFixed(4)} of ${st.toFixed(4)} declared ${r.areaM2}`);
  if (a.includes("--all") || f("--region") !== undefined) console.log(rows.join("\n"));
}
