import { execFileSync } from "node:child_process";
import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { checkAssetDetailed } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { DEFAULT_CATALOG_ROOT, loadCatalogFromFiles } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { runPreview, type Rasterizer } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/V1";
const CHROME = "C:/Program Files/Google/Chrome/Application/chrome.exe";
const chrome: Rasterizer = {
  rasterize(svgPath, pngPath, dpi) {
    const svg = readFileSync(svgPath, "utf8");
    const w = Number(/width="([\d.]+)mm"/.exec(svg)![1]);
    const h = Number(/height="([\d.]+)mm"/.exec(svg)![1]);
    const pw = Math.max(50, Math.min(3800, Math.round((w / 25.4) * dpi)));
    const ph = Math.max(50, Math.round((h * pw) / w));
    const html = svgPath.replace(/\.svg$/, ".html");
    const abs = (p: string) => resolve(p).replace(/\\/g, "/");
    writeFileSync(html, `<!doctype html><body style="margin:0;background:#fff"><img src="${encodeURIComponent(basename(svgPath))}" width="${pw}" height="${ph}">`);
    execFileSync(CHROME, ["--headless=new", "--disable-gpu", "--hide-scrollbars", "--force-device-scale-factor=1", `--window-size=${pw},${ph}`, `--screenshot=${abs(pngPath)}`, `file:///${encodeURI(abs(html))}`], { stdio: "ignore" });
  },
};

type Job = { asset: string; scales: number[]; poses: string[]; dpi: number };
const JOBS: Job[] = [
  { asset: "agave", scales: [500, 200], poses: ["summer-turgid"], dpi: 350 },
  { asset: "agave", scales: [100, 50], poses: ["summer-turgid", "drought-infolded", "frost-sag", "clip:wind-sway@1.5"], dpi: 500 },
  { asset: "apple-tree", scales: [500, 200, 100, 50], poses: ["summer-fruit-set"], dpi: 300 },
  { asset: "apple-tree", scales: [100], poses: ["gale-windfall", "clip:breeze-sway@1.5", "clip:gust@1"], dpi: 300 },
  { asset: "english-oak", scales: [500, 200, 100, 50], poses: ["calm"], dpi: 300 },
  { asset: "english-oak", scales: [100], poses: ["storm", "clip:wind-sway@1.5"], dpi: 300 },
  { asset: "bamboo-clump", scales: [500, 200, 100, 50], poses: ["summer-calm"], dpi: 300 },
  { asset: "bamboo-clump", scales: [100], poses: ["storm-bent", "clip:wind-sway@1.5"], dpi: 350 },
  { asset: "date-palm", scales: [500, 200, 100, 50], poses: ["calm"], dpi: 300 },
  { asset: "date-palm", scales: [100], poses: ["storm", "clip:wind-sway@1.5"], dpi: 300 },
  { asset: "hornbeam-hedge", scales: [500, 200, 100, 50], poses: ["calm"], dpi: 350 },
  { asset: "hornbeam-hedge", scales: [100], poses: ["snow", "clip:wind-sway@1.5"], dpi: 350 },
  { asset: "hosta-clump", scales: [200, 100, 50], poses: ["summer-full"], dpi: 600 },
  { asset: "hosta-clump", scales: [50], poses: ["autumn-collapse", "clip:wind-flutter@1.5"], dpi: 600 },
  { asset: "ivy-on-trellis", scales: [500, 200, 100, 50], poses: ["summer-calm"], dpi: 350 },
  { asset: "ivy-on-trellis", scales: [100], poses: ["storm", "clip:wind-sway@1.5"], dpi: 350 },
];

const log = (line: string) => appendFileSync(join(OUT, "all.log"), line + "\n");
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "all.log"), "");
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const known = new Set(catalog.assets.map((a) => a.id));
const ids = [...new Set(JOBS.map((j) => j.asset))];
for (const id of ids) {
  const asset = catalog.assets.find((a) => a.id === id)!;
  const report = checkAssetDetailed(asset, { samples: 5, assets: known, physics: false });
  log(`[DEBUG] CHECK ${id}: ${report.problems.length ? "PROBLEMS" : "ok"}`);
  for (const l of report.lines) log(`[DEBUG]   ${l}`);
  for (const p of report.problems.slice(0, 40)) log(`[DEBUG]   ! ${p}`);
}
const only = process.argv[2];
for (const job of JOBS) {
  if (only && job.asset !== only) continue;
  const file = join(DEFAULT_CATALOG_ROOT, catalog.loaded.find((e) => e.asset.id === job.asset)!.categoryFolder, job.asset, "🔣️.json");
  const dir = join(OUT, job.asset);
  mkdirSync(dir, { recursive: true });
  for (const scale of job.scales) {
    for (const p of job.poses) {
      const args = ["--file", file, "--scale", String(scale), "--views", "--out", dir, "--dpi", String(job.dpi)];
      if (p.startsWith("clip:")) {
        const [cid, t] = p.slice(5).split("@");
        args.push("--clip", cid!, "--time", t ?? "0");
      } else args.push("--pose", p);
      try {
        for (const w of await runPreview(args, chrome)) if (w.endsWith(".png")) log(`[DEBUG] PNG ${w}`);
      } catch (e) {
        log(`[DEBUG] FAIL ${job.asset} ${scale} ${p} ${String(e).slice(0, 200)}`);
      }
    }
  }
}
log("[DEBUG] ALL DONE");
