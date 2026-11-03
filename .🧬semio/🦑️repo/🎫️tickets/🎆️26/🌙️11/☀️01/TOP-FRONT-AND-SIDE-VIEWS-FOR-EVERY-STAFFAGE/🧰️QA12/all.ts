// One process for all of QA12: check (validation, evaluation, suitability), three-view consistency over every pose and clip, previews.
// usage: bun all.ts [--no-prev] [--only id,id] [--prev-only]
import { readFileSync, mkdirSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const root = `${S}/🗂️catalog/🏗️machinery`;
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/QA12/prev";
const ALL = ["concrete-mixer-truck", "diesel-generator", "evaporative-humidifier", "excavator", "expansion-valve", "finned-heat-exchanger", "forklift", "heat-pump-outdoor-unit"];
const argv = process.argv.slice(2);
const only = argv.includes("--only") ? argv[argv.indexOf("--only") + 1]!.split(",") : ALL;
const noPrev = argv.includes("--no-prev");
const prevOnly = argv.includes("--prev-only");

type Ext = { minX: number; maxX: number; minY: number; maxY: number; n: number };
const VIEWS = ["elevation", "front", "plan"] as const;
function ext(shapes: any[]): Ext {
  const e: Ext = { minX: Infinity, maxX: -Infinity, minY: Infinity, maxY: -Infinity, n: 0 };
  for (const s of shapes) {
    if (s.shape.role === "ground") continue;
    for (const ring of s.rings) for (const p of ring) {
      e.n++;
      e.minX = Math.min(e.minX, p.x); e.maxX = Math.max(e.maxX, p.x);
      e.minY = Math.min(e.minY, p.y); e.maxY = Math.max(e.maxY, p.y);
    }
  }
  return e;
}
const f = (v: number) => v.toFixed(3);
let bad = 0;
const assets: Record<string, any> = {};
let known: Set<string> | undefined;
try {
  const cat = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
  known = new Set(cat.assets.map((a) => a.id));
  for (const a of cat.assets) if (ALL.includes(a.id)) assets[a.id] = a;
  console.log(`[QA12] catalog loaded: ${cat.assets.length} assets`);
} catch (e) {
  console.log(`[QA12] catalog load failed, using the files directly: ${String(e).slice(0, 400)}`);
  for (const id of ALL) assets[id] = JSON.parse(readFileSync(`${root}/${id}/🔣️.json`, "utf8")).asset;
  known = new Set(ALL);
}
mkdirSync(OUT, { recursive: true });

if (!prevOnly) for (const id of only) {
  const asset = assets[id];
  const d = asset.dimensions;
  console.log(`\n===== ${id} dims w=${d.widthM} d=${d.depthM} h=${d.heightM}`);
  const rep = checkAssetDetailed(asset, { samples: 9, assets: known });
  console.log(rep.problems.length ? "CHECK PROBLEMS:\n  " + rep.problems.slice(0, 30).join("\n  ") : "check ok");
  for (const l of rep.lines) console.log("  " + l);
  if (rep.problems.length) bad++;
  for (const sc of ["500", "200", "100", "50"]) {
    const r = asset.representations[sc];
    if (r.status !== "authored") continue;
    const sel = { kind: "scale", scale: { denominator: Number(sc) } } as any;
    const tol = Math.max(0.02, 0.03 * Math.max(d.widthM, d.depthM, d.heightM));
    const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel);
    const E = ext(st.shapes.elevation), F = ext(st.shapes.front), P = ext(st.shapes.plan);
    const checks: [string, number, number][] = [
      ["E.x/P.x min", E.minX, P.minX], ["E.x/P.x max", E.maxX, P.maxX], ["E.y/F.y min", E.minY, F.minY], ["E.y/F.y max", E.maxY, F.maxY], ["F.x/P.y min", F.minX, P.minY], ["F.x/P.y max", F.maxX, P.maxY],
    ];
    const probs = checks.filter(([, a, b]) => !(Math.abs(a - b) <= tol)).map(([n, a, b]) => `${n}: ${f(a)} vs ${f(b)}`);
    if (Math.abs(E.maxX - E.minX - d.widthM) > tol) probs.push(`E width ${f(E.maxX - E.minX)} != ${d.widthM}`);
    if (Math.abs(P.maxY - P.minY - d.depthM) > tol) probs.push(`P depth ${f(P.maxY - P.minY)} != ${d.depthM}`);
    if (Math.abs(E.maxY - E.minY - d.heightM) > tol) probs.push(`E height ${f(E.maxY - E.minY)} != ${d.heightM}`);
    console.log(`  1:${sc} tol ${f(tol)} E[${f(E.minX)},${f(E.maxX)}]x[${f(E.minY)},${f(E.maxY)}] F[${f(F.minX)},${f(F.maxX)}]x[${f(F.minY)},${f(F.maxY)}] P[${f(P.minX)},${f(P.maxX)}]x[${f(P.minY)},${f(P.maxY)}] ${probs.length ? "PROBLEMS " + probs.join("; ") : "ok"}`);
    if (probs.length) bad++;
    const queries: any[] = [];
    for (const p of asset.poses) queries.push({ kind: "pose", pose: p.id });
    for (const c of asset.clips) for (let i = 0; i <= 12; i++) queries.push({ kind: "clip", clip: c.id, timeS: (c.durationS * Math.min(i / 12, 0.999)) });
    for (const q of queries) {
      const s = evaluate(asset, q, sel);
      const row: string[] = [];
      for (const v of VIEWS) {
        const e = ext(s.shapes[v]);
        if (e.n === 0 || ![e.minX, e.maxX, e.minY, e.maxY].every(Number.isFinite)) { console.log(`   EMPTY/NONFINITE ${v} ${JSON.stringify(q)}`); bad++; }
        row.push(`${v[0]}[${f(e.minX)},${f(e.maxX)}|${f(e.minY)},${f(e.maxY)}]`);
      }
      if (sc === "100" || sc === "200") console.log(`     ${q.kind === "pose" ? q.pose : q.clip + "@" + q.timeS.toFixed(2)} ${row.join(" ")}`);
    }
  }
}
console.log(bad ? `\nPROBLEMS: ${bad}` : "\nALL OK");

if (!noPrev) for (const id of only) {
  const asset = assets[id];
  const jobs: string[][] = [];
  for (const sc of ["500", "200", "100", "50"]) {
    if (asset.representations[sc].status !== "authored") continue;
    for (const p of asset.poses) jobs.push([sc, "--pose", p.id]);
    for (const c of asset.clips) for (const fr of [0.15, 0.5, 0.9]) jobs.push([sc, "--clip", c.id, "--time", (c.durationS * fr).toFixed(2)]);
  }
  for (const [scale, ...rest] of jobs) {
    const args = ["--file", `${root}/${id}/🔣️.json`, "--scale", scale!, "--views", "--view", "dark-line", "--no-png", "--out", OUT, ...rest];
    try {
      const w = await runPreview(args);
      if (w.some((f) => f.includes("explanation")) || process.exitCode) { console.log(`[prev] NOT PRINTABLE ${id} 1:${scale} ${rest.join(" ")}: ${w.map((p) => p.split("/").pop()).join(", ")}`); bad++; process.exitCode = 0; continue; }
      console.log(`[prev] ${id} 1:${scale} ${rest.join(" ")}: ${w.map((p) => p.split("/").pop()).join(", ")}`);
    } catch (e) {
      bad++;
      console.log(`[prev] FAILED ${id} 1:${scale} ${rest.join(" ")}: ${String(e).slice(0, 300)}`);
    }
  }
}
console.log(bad ? `FINAL PROBLEMS: ${bad}` : "FINAL ALL OK");
console.log("[QA12] done");
