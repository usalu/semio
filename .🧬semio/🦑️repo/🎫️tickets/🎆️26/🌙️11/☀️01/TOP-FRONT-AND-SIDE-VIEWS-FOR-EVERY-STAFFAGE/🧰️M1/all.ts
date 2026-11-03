// One process for all of M1: check (validation, evaluation, suitability), three-view consistency over every pose and clip, previews.
// usage: bun all.ts [--no-prev] [--only id,id] [--prev-only]
import { readFileSync, mkdirSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const root = `${S}/🗂️catalog/🏗️machinery`;
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/M1/prev";
const ALL = ["concrete-mixer-truck", "diesel-generator", "evaporative-humidifier", "excavator", "expansion-valve", "finned-heat-exchanger", "forklift", "heat-pump-outdoor-unit"];
const argv = process.argv.slice(2);
const only = argv.includes("--only") ? argv[argv.indexOf("--only") + 1]!.split(",") : ALL;
const noPrev = argv.includes("--no-prev");
const prevOnly = argv.includes("--prev-only");

// previews: asset -> list of [scale, query]
const PREV: Record<string, [string, string][]> = {
  "expansion-valve": [["50", "pose:open"], ["50", "pose:closed"], ["100", "pose:open"]],
  "evaporative-humidifier": [["50", "pose:running"], ["50", "clip:drip@1.0"], ["200", "pose:dry"]],
  "finned-heat-exchanger": [["50", "pose:hot"], ["200", "pose:hot"]],
  "heat-pump-outdoor-unit": [["50", "pose:heating"], ["50", "pose:serviced"], ["50", "clip:defrost-sequence@28"], ["100", "pose:heating"], ["200", "pose:serviced"], ["500", "pose:serviced"]],
  forklift: [["100", "pose:lowered-forks"], ["100", "pose:lifted-load"], ["100", "pose:tilted-mast"], ["100", "clip:lift-pallet@7"], ["50", "pose:lifted-load"], ["200", "pose:tilted-mast"], ["500", "pose:lifted-load"]],
  "diesel-generator": [["100", "pose:off"], ["100", "pose:rated-load"], ["100", "clip:start-run-stop@12"], ["50", "pose:rated-load"], ["200", "pose:half-load"], ["500", "pose:rated-load"]],
  excavator: [["100", "pose:stowed"], ["100", "pose:digging"], ["100", "pose:raised-boom"], ["100", "clip:dig-cycle@3"], ["50", "pose:raised-boom"], ["200", "pose:digging"], ["500", "pose:raised-boom"]],
  "concrete-mixer-truck": [["200", "pose:transit-agitating"], ["200", "pose:discharging"], ["200", "pose:chute-swing"], ["100", "clip:discharge-sequence@4.5"], ["50", "pose:chute-swing"], ["500", "pose:discharging"]],
};

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
  console.log(`[M1] catalog loaded: ${cat.assets.length} assets`);
} catch (e) {
  console.log(`[M1] catalog load failed, using the files directly: ${String(e).slice(0, 400)}`);
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
  for (const [scale, q] of PREV[id] ?? []) {
    const [kind, rest] = q.split(":");
    const args = ["--file", `${root}/${id}/🔣️.json`, "--scale", scale, "--views", "--view", "dark-line", "--dpi", "300", "--out", OUT];
    if (kind === "pose") args.push("--pose", rest!);
    else args.push("--clip", rest!.split("@")[0]!, "--time", rest!.split("@")[1]!);
    try {
      const w = await runPreview(args);
      console.log(`[prev] ${id} 1:${scale} ${q}: ${w.map((p) => p.split("/").pop()).join(", ")}`);
    } catch (e) {
      console.log(`[prev] FAILED ${id} 1:${scale} ${q}: ${String(e).slice(0, 300)}`);
    }
  }
}
console.log("[M1] done");
