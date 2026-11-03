import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const out = process.argv[2]!;
const files: string[] = [];
const walk = (d: string): void => {
  for (const n of readdirSync(d)) {
    const p = join(d, n);
    if (statSync(p).isDirectory()) walk(p);
    else if (n === "🔣️.json" && p.split(/[\\/]/).length > root.split(/[\\/]/).length + 2) files.push(p);
  }
};
walk(root);
const rows: any[] = [];
for (const f of files) {
  const doc = JSON.parse(readFileSync(f, "utf8"));
  const a = doc.asset;
  if (a === undefined) continue;
  const t = a.thermal;
  const resp = t.respiration;
  const anchorBone = (id: string) => a.anchors?.find((x: any) => x.id === id)?.bone;
  const em = (t.emitters ?? []).map((e: any) => ({ id: e.id, kind: e.kind, bone: anchorBone(e.anchor), key: e.operatingKey, region: e.region }));
  rows.push({
    id: a.id,
    category: a.category,
    body: t.body?.species ?? null,
    patterns: resp?.patterns.map((p: any) => `${p.id}:${p.rateBpm}`) ?? null,
    nose: resp?.noseEmitter ?? null,
    mouth: resp?.mouthEmitter ?? null,
    emitters: em,
    operating: (t.operating ?? []).map((o: any) => o.id),
    operatingRegions: t.regions.filter((r: any) => r.operating || r.hydronic).map((r: any) => `${r.id}:${r.operating?.key ?? r.hydronic?.key}`),
    leaf: t.regions.filter((r: any) => r.model === "leaf").length,
    emissivityMissing: (a.materials ?? []).filter((m: any) => typeof m.emissivity !== "number").map((m: any) => m.id),
    baseMet: t.baseMet ?? null,
    poseMets: (a.poses ?? []).map((p: any) => p.thermal?.activityMet).filter((x: any) => x !== undefined),
    cloLayers: t.regions.flatMap((r: any) => r.layers.filter((l: any) => l.kind === "clothing").map((l: any) => `${r.id}/${l.id}:${l.resistanceM2KW}`)).slice(0, 3),
  });
}
writeFileSync(out, JSON.stringify(rows, null, 1));
console.log(rows.length);
