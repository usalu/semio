/** Dumps the flattened rings of every filled path / closed poly of the catalog: bun author-72-dump.ts > out.json */
import { readdirSync, readFileSync } from "node:fs";
import { flattenPath } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const root = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const out: unknown[] = [];
for (const cat of readdirSync(root)) {
  let slugs: string[] = [];
  try { slugs = readdirSync(`${root}/${cat}`); } catch { continue; }
  for (const slug of slugs) {
    let a: any;
    try { a = JSON.parse(readFileSync(`${root}/${cat}/${slug}/🔣️.json`, "utf8")).asset; } catch { continue; }
    for (const [sc, rep] of Object.entries<any>(a.representations ?? {})) {
      if (!rep.geometry) continue;
      for (const [view, list] of Object.entries<any>(rep.geometry)) for (const s of list as any[]) {
        if (s.filled === false) continue;
        let rings: { x: number; y: number }[][] | undefined;
        if (s.kind === "path") rings = flattenPath(s.d).rings.map((r) => [...r]);
        else if (s.kind === "poly" && s.closed) rings = [s.points.map(([x, y]: number[]) => ({ x, y }))];
        if (!rings) continue;
        out.push({ cat, slug, scale: sc, view, id: s.id, kind: s.kind, bone: s.bone, rings: rings.map((r) => r.map((p) => [p.x, p.y])) });
      }
    }
  }
}
console.log(JSON.stringify(out));
