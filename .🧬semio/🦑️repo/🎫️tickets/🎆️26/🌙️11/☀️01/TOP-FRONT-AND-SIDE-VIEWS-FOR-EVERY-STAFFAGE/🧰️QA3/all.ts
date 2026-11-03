import { execFile } from "node:child_process";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, queries } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { runPreview, type Rasterizer } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/QA3";
const CHROME = "C:/Program Files/Google/Chrome/Application/chrome.exe";
const queue: { svgPath: string; pngPath: string; dpi: number }[] = [];
const chrome: Rasterizer = { rasterize(svgPath, pngPath, dpi) { queue.push({ svgPath, pngPath, dpi }); } };
const abs = (p: string) => resolve(p).replaceAll("\\", "/");
function shot(job: { svgPath: string; pngPath: string; dpi: number }, n: number): Promise<void> {
  const svg = readFileSync(job.svgPath, "utf8");
  const w = Number(/width="([\d.]+)mm"/.exec(svg)![1]);
  const h = Number(/height="([\d.]+)mm"/.exec(svg)![1]);
  const pw = Math.max(50, Math.min(3800, Math.round((w / 25.4) * job.dpi)));
  const ph = Math.max(50, Math.round((h * pw) / w));
  const html = job.svgPath.replace(/\.svg$/, ".html");
  writeFileSync(html, `<!doctype html><body style="margin:0;background:#fff"><img src="${encodeURIComponent(basename(job.svgPath))}" width="${pw}" height="${ph}">`);
  return new Promise((done) => execFile(CHROME, ["--headless=new", "--disable-gpu", "--hide-scrollbars", `--user-data-dir=${abs(join(OUT, "profile" + n))}`, "--force-device-scale-factor=1", `--window-size=${pw},${ph}`, `--screenshot=${abs(job.pngPath)}`, `file:///${encodeURI(abs(html))}`], () => done()));
}

const ASSETS = ["agave", "apple-tree", "bamboo-clump", "date-palm", "english-oak", "hornbeam-hedge", "hosta-clump", "ivy-on-trellis"];
const arg = (n: string) => { const i = process.argv.indexOf("--" + n); return i < 0 ? undefined : process.argv[i + 1]; };
const onlyAssets = arg("assets")?.split(",") ?? ASSETS;
const onlyScales = arg("scales")?.split(",").map(Number);
const onlyPoses = arg("poses")?.split(",");
const fractions = (arg("fractions") ?? "0.15,0.5,0.85").split(",").map(Number);
const dpi = arg("dpi") ?? "150";
const noCheck = process.argv.includes("--no-check");
const noRender = process.argv.includes("--no-render");

const log = (line: string) => appendFileSync(join(OUT, "all.log"), line + "\n");
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "all.log"), "");
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const known = new Set(catalog.assets.map((a) => a.id));
for (const id of onlyAssets) {
  const asset: any = catalog.assets.find((a) => a.id === id)!;
  if (!noCheck) {
    const report = checkAssetDetailed(asset, { samples: 5, assets: known, physics: false });
    log(`[DEBUG] CHECK ${id}: ${report.problems.length ? "PROBLEMS" : "ok"}`);
    for (const l of report.lines) log(`[DEBUG]   ${l}`);
    for (const p of report.problems.slice(0, 60)) log(`[DEBUG]   ! ${p}`);
  }
  if (process.argv.includes("--probe")) {
    for (const { name, query } of queries(asset)) {
      const st = evaluate(asset, query, { kind: "scale", scale: { denominator: 50 } } as any);
      const e = extentOf(st.shapes.elevation), f = extentOf(st.shapes.front), p = extentOf(st.shapes.plan);
      const fm = (x: any) => x ? `[${x.minX.toFixed(3)},${x.maxX.toFixed(3)}]x[${x.minY.toFixed(3)},${x.maxY.toFixed(3)}]` : "none";
      log(`[DEBUG] PROBE ${id} ${name} E${fm(e)} F${fm(f)} P${fm(p)}${(e && e.minY < -0.003) || (f && f.minY < -0.003) ? " BELOW-GRADE" : ""}`);
    }
  }
  if (noRender) continue;
  const file = join(DEFAULT_CATALOG_ROOT, catalog.loaded.find((e) => e.asset.id === id)!.categoryFolder, id, "🔣️.json");
  const dir = join(OUT, id);
  mkdirSync(dir, { recursive: true });
  const scales = Object.entries<any>(asset.representations).filter(([, r]) => r.status === "authored").map(([k]) => Number(k)).filter((s) => !onlyScales || onlyScales.includes(s));
  const jobs: string[][] = [];
  for (const p of asset.poses) if (!onlyPoses || onlyPoses.includes(p.id)) jobs.push(["--pose", p.id]);
  for (const c of asset.clips) if (!onlyPoses || onlyPoses.includes(c.id)) {
    const dur = c.durationS ?? c.duration;
    for (const f of fractions) jobs.push(["--clip", c.id, "--time", (Math.round(dur * f * 100) / 100).toString()]);
  }
  for (const scale of scales) for (const j of jobs) {
    const args = ["--file", file, "--scale", String(scale), "--views", "--out", dir, "--dpi", dpi, ...j];
    try {
      for (const w of await runPreview(args, chrome)) if (w.endsWith(".png")) log(`[DEBUG] PNG ${w}`);
    } catch (e) {
      log(`[DEBUG] FAIL ${id} ${scale} ${j.join(" ")} ${String(e).slice(0, 300)}`);
    }
  }
}
log(`[DEBUG] SVG DONE ${queue.length}`);
let next = 0;
await Promise.all(Array.from({ length: 8 }, async (_, n) => { while (next < queue.length) { const j = queue[next++]!; await shot(j, n); } }));
log("[DEBUG] ALL DONE");
