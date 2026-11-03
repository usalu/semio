/** Ticket-local batch renderer: bun render.ts <asset> <out> [scales=50,100,200] [mode=all|extents|pose:<id>|clip:<id>:<t>] */
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles, loadEnvironments } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { readFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const [assetArg, out, scalesArg = "50,100,200", mode = "all"] = process.argv.slice(process.argv.findIndex((x) => x.endsWith("render.ts")) + 1);
const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances";

if (process.env.P2_CHECK !== "0") {
  const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
  const known = new Set(catalog.assets.map((x: any) => x.id));
  const environments = new Set(((await loadEnvironments(catalog.root))?.presets ?? []).map((x: any) => x.id));
  for (const id of assetArg.split(",")) {
    const as = catalog.assets.find((x: any) => x.id === id);
    const rep = checkAssetDetailed(as as any, { samples: 9, profile: DEFAULT_PRINT_PROFILE, assets: known, environments, physics: true });
    console.log(`CHECK ${rep.problems.length ? "FAIL" : "ok"} ${id}`);
    for (const l of rep.lines) console.log("  " + l);
    for (const q of rep.problems.slice(0, 30)) console.log("  ! " + q);
  }
}
for (const asset of assetArg.split(",")) {
const file = `${root}/${asset}/🔣️.json`;
console.log("##", asset);
const doc = JSON.parse(readFileSync(file, "utf8"));
const a = doc.asset;
const scales = scalesArg.split(",").map(Number);
mkdirSync(out, { recursive: true });
const ext = (state: any, vp: string) => {
  let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity;
  for (const s of state.shapes[vp]) {
    if (s.shape.role === "ground") continue;
    for (const ring of s.rings) for (const p of ring) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  }
  return [x0, x1, y0, y1].map((v) => Math.round(v * 1000) / 1000);
};
const queries: any[] = [];
if (mode === "all" || mode === "views" || mode === "extents") {
  for (const p of a.poses) if (!p.key) queries.push({ kind: "pose", pose: p.id });
  for (const c of a.clips) for (const f of [0, 0.5, 1]) queries.push({ kind: "clip", clip: c.id, timeS: Math.round(c.durationS * f * 100) / 100 });
} else if (mode.startsWith("pose:")) queries.push({ kind: "pose", pose: mode.slice(5) });
else if (mode.startsWith("clip:")) { const [, c, t] = mode.split(":"); queries.push({ kind: "clip", clip: c, timeS: Number(t) }); }
for (const d of scales) {
  const rep = a.representations[String(d)];
  if (!rep || rep.status !== "authored") continue;
  const s = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: d } });
  const e = ext(s, "elevation"), f = ext(s, "front"), p = ext(s, "plan");
  const dim = a.dimensions;
  const tol = Math.max(0.02, 0.03 * Math.max(dim.widthM, dim.depthM, dim.heightM));
  const ok = (u: number, v: number) => Math.abs(u - v) <= tol;
  const checks = [
    ["elev.x=plan.x", ok(e[0], p[0]) && ok(e[1], p[1])],
    ["elev.y=front.y", ok(e[2], f[2]) && ok(e[3], f[3])],
    ["front.x=plan.y", ok(f[0], p[2]) && ok(f[1], p[3])],
    ["w", ok(e[1] - e[0], dim.widthM)], ["d", ok(f[1] - f[0], dim.depthM)], ["h", ok(e[3] - e[2], dim.heightM)],
  ];
  console.log(`1:${d} elevation x[${e[0]},${e[1]}] y[${e[2]},${e[3]}] | front x[${f[0]},${f[1]}] y[${f[2]},${f[3]}] | plan x[${p[0]},${p[1]}] y[${p[2]},${p[3]}] | dim w${dim.widthM} d${dim.depthM} h${dim.heightM} tol${tol.toFixed(3)} => ${checks.map(([n, v]) => `${n}:${v ? "ok" : "FAIL"}`).join(" ")}`);
  if (mode === "extents") { for (const q of queries) { const st = evaluate(a, q, { kind: "scale", scale: { denominator: d } }); for (const vp of ["elevation","front","plan"]) { const x = ext(st, vp); if (!x.every(Number.isFinite)) console.log(`  EMPTY ${vp} ${JSON.stringify(q)} 1:${d}`); } } continue; }
  for (const q of queries) {
    const st = evaluate(a, q, { kind: "scale", scale: { denominator: d } });
    for (const vp of ["elevation", "front", "plan"]) { const x = ext(st, vp); if (!x.every(Number.isFinite)) console.log(`  EMPTY ${vp} at ${JSON.stringify(q)} 1:${d}`); }
    const args = ["--file", file, "--scale", String(d), "--views", "--no-png", "--out", out, ...(q.kind === "pose" ? ["--pose", q.pose] : ["--clip", q.clip, "--time", String(q.timeS)])];
    await runPreview(args);
  }
}
}
console.log("done");
