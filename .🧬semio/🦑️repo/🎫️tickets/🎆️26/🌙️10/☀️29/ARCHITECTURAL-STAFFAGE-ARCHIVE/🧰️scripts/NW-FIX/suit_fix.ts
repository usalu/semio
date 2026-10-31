/** [DEBUG] Close office-laser gaps by thickening pens into joins, and redraw thin fills as strokes. */
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { featureWidthMm, gapsBetween, whiteSliver } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { outlineRuns, prepare, sourceOf } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const cats = ["vegetation", "furniture", "objects", "vehicles", "machinery", "components"];
const only = new Set((process.argv[3] ?? "").split(",").filter(Boolean));

type Hit = { a: string; b: string; mm: number; kind: "gap" | "sliver" | "feature" | "stroke" };

function files(): string[] {
  const out: string[] = [];
  for (const cat of readdirSync(root)) {
    if (!cats.some((c) => cat.includes(c))) continue;
    for (const folder of readdirSync(`${root}/${cat}`, { withFileTypes: true })) {
      if (!folder.isDirectory()) continue;
      if (only.size && !only.has(folder.name)) continue;
      const doc = readdirSync(`${root}/${cat}/${folder.name}`).find((f) => f.endsWith(".json"));
      if (doc) out.push(`${root}/${cat}/${folder.name}/${doc}`);
    }
  }
  return out;
}

function pen(rep: any, shape: any): number {
  return shape.penMm ?? rep.pens[shape.role] ?? 0.13;
}

function boost(rep: any, id: string, add: number): void {
  const shape = rep.geometry.elevation.find((s: any) => s.id === id);
  if (!shape || shape.role === "ground") return;
  const next = Math.min(1.8, Math.max(pen(rep, shape) + add, 0.13));
  shape.penMm = Math.round(next * 1000) / 1000;
}

function hitsOf(asset: any, rep: any, d: number): Hit[] {
  const hits: Hit[] = [];
  const k = 1000 / d;
  for (const pose of asset.poses.filter((p: any) => !p.key)) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: d } });
    const prepared = prepare(state, "elevation", sourceOf(state), k);
    for (const p of prepared) {
      if (p.shape.role === "ground" || !p.closed || !p.shape.filled) continue;
      const w = featureWidthMm(p.rings, 0.2);
      if (w < 0.2 - 1e-9) hits.push({ a: p.shape.id, b: "", mm: w, kind: "feature" });
    }
    const paper: any[] = [];
    for (const { p, run } of outlineRuns(prepared, true)) if (p.shape.role !== "ground") paper.push({ id: p.shape.id, rings: [run.points], closed: run.closed, penMm: p.weightMm });
    const sl = whiteSliver(prepared.filter((p: any) => p.closed && p.shape.filled && p.shape.role !== "ground").map((p: any) => p.rings), paper.map((s) => ({ points: s.rings[0], closed: s.closed, penMm: s.penMm })), 0.15);
    if (sl) {
      const at = [sl.at.x, sl.at.y];
      const near = paper.map((s) => ({ id: s.id, d: Math.min(...s.rings[0].map((q: any) => Math.hypot(q.x - at[0], q.y - at[1]))) })).sort((x, y) => x.d - y.d);
      hits.push({ a: near[0]?.id ?? "", b: near[1]?.id ?? "", mm: sl.widthMm, kind: "sliver" });
    }
    for (const g of gapsBetween(paper, 1)) if (g.gapMm > 0 && g.gapMm < 0.45 - 1e-9) hits.push({ a: g.a, b: g.b, mm: g.gapMm, kind: "gap" });
  }
  for (const shape of rep.geometry.elevation) {
    const weight = pen(rep, shape);
    if (shape.role !== "ground" && weight < 0.1 - 1e-9) hits.push({ a: shape.id, b: "", mm: weight, kind: "stroke" });
  }
  return hits;
}

let changedFiles = 0;
for (const path of files()) {
  const doc = JSON.parse(readFileSync(path, "utf8"));
  const asset = doc.asset;
  let changed = false;
  for (const d of [500, 200, 100, 50]) {
    const rep = asset.representations[String(d)];
    if (!rep || rep.status !== "authored") continue;
    for (const hatch of rep.hatches ?? []) if (hatch.spacingMm < 0.4) { hatch.spacingMm = 0.4; changed = true; }
    for (const role of Object.keys(rep.pens)) if (typeof rep.pens[role] === "number" && rep.pens[role] < 0.1) { rep.pens[role] = 0.13; changed = true; }
    for (let it = 0; it < 5; it++) {
      const hits = hitsOf(asset, rep, d);
      const worst = new Map<string, Hit>();
      for (const h of hits) {
        const key = h.kind + ":" + [h.a, h.b].sort().join("+");
        const prev = worst.get(key);
        if (!prev || h.mm < prev.mm) worst.set(key, h);
      }
      if (worst.size === 0) break;
      console.log(`[DEBUG] ${asset.id} 1:${d} pass ${it} ${worst.size}`);
      for (const h of worst.values()) {
        if (h.kind === "feature") {
          const shape = rep.geometry.elevation.find((s: any) => s.id === h.a);
          if (!shape) continue;
          shape.filled = false;
          shape.penMm = Math.min(1.8, Math.max(pen(rep, shape), 0.22));
          changed = true;
        } else if (h.kind === "stroke") {
          boost(rep, h.a, 0.13 - h.mm);
          changed = true;
        } else {
          const add = (Math.min(h.mm, 0.39) + 0.04) / 2;
          boost(rep, h.a, add);
          if (h.b) boost(rep, h.b, add);
          changed = true;
        }
      }
    }
  }
  if (changed) {
    writeFileSync(path, JSON.stringify(doc, null, 1) + "\n");
    changedFiles++;
  }
}
console.log(`[DEBUG] files changed ${changedFiles}`);
