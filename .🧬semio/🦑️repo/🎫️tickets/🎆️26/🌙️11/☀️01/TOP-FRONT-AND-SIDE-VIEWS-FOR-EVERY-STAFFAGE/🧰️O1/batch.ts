/** 🧰️ O1 scratch: check + orthographic consistency + pose/clip sampling + sharp-rendered view sheets for a list of assets.
 * usage: bun batch.ts check|render <ids comma> [--scales 50,100] [--out dir] [--pose id] [--clip id --time s] [--catalog-less] */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";
import { type Asset, type AssetDocument } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects";
const [mode, idsArg, ...rest] = process.argv.slice(2);
const flag = (name: string): string | undefined => { const i = rest.indexOf(`--${name}`); return i < 0 ? undefined : rest[i + 1]; };
const ids = idsArg!.split(",");
const scalesArg = flag("scales");
const out = flag("out") ?? "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/O1";

type Ext = { minX: number; maxX: number; minY: number; maxY: number };
function ext(asset: Asset, query: any, scale: number, vp: "elevation" | "front" | "plan"): Ext | null {
  const state = evaluate(asset, query, { kind: "scale", scale: { denominator: scale } });
  const pts = state.shapes[vp].filter((s) => s.shape.role !== "ground").flatMap((s) => s.rings.flat());
  if (pts.length === 0) return null;
  const e = { minX: Infinity, maxX: -Infinity, minY: Infinity, maxY: -Infinity };
  for (const p of pts) { e.minX = Math.min(e.minX, p.x); e.maxX = Math.max(e.maxX, p.x); e.minY = Math.min(e.minY, p.y); e.maxY = Math.max(e.maxY, p.y); }
  return Object.values(e).every(Number.isFinite) ? e : null;
}
const f = (v: number): string => v.toFixed(4);

for (const id of ids) {
  const asset = (JSON.parse(readFileSync(`${ROOT}/${id}/🔣️.json`, "utf8")) as AssetDocument).asset;
  const authored = Object.values(asset.representations).filter((r: any) => r.status === "authored").map((r: any) => r.denominator as number);
  const scales = scalesArg ? scalesArg.split(",").map(Number) : authored;
  if (mode === "check" || mode === "all") {
    const report = checkAssetDetailed(asset, { samples: 5, physics: false });
    console.log(`${report.problems.length ? "✗" : "✓"} ${id}`);
    for (const l of report.lines) console.log("  " + l);
    for (const p of report.problems) console.log("  PROBLEM " + p);
    const d = asset.dimensions;
    const tol = Math.max(0.02, 0.03 * Math.max(d.widthM, d.depthM, d.heightM));
    for (const sc of authored) {
      const q = { kind: "pose", pose: asset.presentation.defaultPose };
      const e = ext(asset, q, sc, "elevation"), fr = ext(asset, q, sc, "front"), pl = ext(asset, q, sc, "plan");
      if (!e || !fr || !pl) { console.log(`  ORTHO 1:${sc} EMPTY view`); continue; }
      const bad: string[] = [];
      const cmp = (n: string, a: number, b: number) => { if (Math.abs(a - b) > tol) bad.push(`${n} ${f(a)} vs ${f(b)}`); };
      cmp("elev.x/plan.x min", e.minX, pl.minX); cmp("elev.x/plan.x max", e.maxX, pl.maxX);
      cmp("elev.y/front.y min", e.minY, fr.minY); cmp("elev.y/front.y max", e.maxY, fr.maxY);
      cmp("front.x/plan.y min", fr.minX, pl.minY); cmp("front.x/plan.y max", fr.maxX, pl.maxY);
      cmp("widthM", e.maxX - e.minX, d.widthM); cmp("heightM", e.maxY - e.minY, d.heightM);
      cmp("depthM(front)", fr.maxX - fr.minX, d.depthM); cmp("depthM(plan)", pl.maxY - pl.minY, d.depthM);
      console.log(`  ORTHO 1:${sc} tol=${f(tol)} ${bad.length ? "FAIL " + bad.join("; ") : "ok"}  elev=[${f(e.minX)},${f(e.maxX)}]x[${f(e.minY)},${f(e.maxY)}] front=[${f(fr.minX)},${f(fr.maxX)}]x[${f(fr.minY)},${f(fr.maxY)}] plan=[${f(pl.minX)},${f(pl.maxX)}]x[${f(pl.minY)},${f(pl.maxY)}]`);
    }
    for (const sc of authored) {
      let empty = 0, n = 0;
      const queries: any[] = [...asset.poses.map((p) => ({ kind: "pose", pose: p.id })), ...asset.clips.flatMap((c) => Array.from({ length: 9 }, (_, i) => ({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / 8 })))];
      for (const q of queries) for (const vp of ["elevation", "front", "plan"] as const) { n++; if (!ext(asset, q, sc, vp)) { empty++; console.log(`  EMPTY 1:${sc} ${vp} ${JSON.stringify(q)}`); } }
      console.log(`  SAMPLES 1:${sc} ${n - empty}/${n} non-empty finite`);
    }
  }
  if (mode === "render" || mode === "all") {
    mkdirSync(`${out}/${id}`, { recursive: true });
    const file = `${ROOT}/${id}/🔣️.json`;
    const specs: string[][] = [];
    if (mode === "all") {
      for (const p of asset.poses) specs.push(["--pose", p.id]);
      for (const c of asset.clips) for (const f of [0, 0.5, 1]) specs.push(["--clip", c.id, "--time", String(+(c.durationS * f).toFixed(3))]);
    }
    const clipsArg = flag("clip"), timeArg = flag("time"), poseArg = flag("pose");
    if (mode === "all") {} else if (clipsArg) for (const t of (timeArg ?? "0").split(",")) specs.push(["--clip", clipsArg, "--time", t]);
    else if (poseArg) for (const p of poseArg.split(",")) specs.push(["--pose", p]);
    else specs.push([]);
    if (mode === "all" && !flag("scales")) scales.splice(0, scales.length, ...authored.filter((x) => x === 50 || x === 200 || x === 500));
    for (const sc of scales) for (const spec of specs) {
      const written = await runPreview(["--file", file, "--scale", String(sc), "--views", "--no-png", "--out", `${out}/${id}`, ...spec]);
      for (const svg of written) {
        const png = svg.replace(/\.svg$/, ".png");
        await sharp(svg, { density: Number(flag("dpi") ?? 300) }).flatten({ background: "#ffffff" }).png().toFile(png);
        console.log(png);
      }
    }
  }
}
