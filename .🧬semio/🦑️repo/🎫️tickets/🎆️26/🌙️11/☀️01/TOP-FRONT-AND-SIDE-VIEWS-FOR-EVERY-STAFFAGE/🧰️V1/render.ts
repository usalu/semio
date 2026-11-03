import { execFileSync } from "node:child_process";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { runPreview, type Rasterizer } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";

const CHROME = "C:/Program Files/Google/Chrome/Application/chrome.exe";
const chromeRasterizer: Rasterizer = {
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

const [asset, scales, jobs, out, dpi, view] = process.argv.slice(2);
const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const dir = readdirSync(root).map((d) => join(root, d)).find((d) => readdirSync(d).includes(asset!))!;
const file = join(dir, asset!, "🔣️.json");
for (const scale of scales!.split(",")) {
  for (const job of jobs!.split(",")) {
    const args = ["--file", file, "--scale", scale, "--views", "--out", out!, "--dpi", dpi ?? "300"];
    if (view) args.push("--view", view);
    if (job.startsWith("clip:")) {
      const [id, t] = job.slice(5).split("@");
      args.push("--clip", id!, "--time", t ?? "0");
    } else args.push("--pose", job.replace(/^pose:/, ""));
    try {
      for (const p of await runPreview(args, chromeRasterizer)) if (p.endsWith(".png")) console.log("[DEBUG]", p);
    } catch (e) {
      console.log("[DEBUG] FAIL", scale, job, String(e).slice(0, 300));
    }
  }
}
