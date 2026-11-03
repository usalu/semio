/** [DEBUG] QA1 driver: check + three-views guard + `--views` sheets for the 15 QA1 assets in ONE process.
 * usage: bun all.ts [--assets a,b] [--scales 50,100] [--no-render] [--no-check] [--poses-only] [--clips-only] [--fractions 0.1,0.45,0.8]
 * output: T/🗑️generated/QA1/<asset>/<scale>/*.svg|png, log in T/🗑️generated/QA1/log.txt */
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../..");
const S = join(root, "🎓️teaching/🏛️architecture/🧍️staffage");
const OUT = join(import.meta.dir, "../🗑️generated/QA1");
const ALL = "dairy-cow,domestic-cat,goat,hedgehog,herring-gull,horse,labrador-dog,japanese-maple,lombardy-poplar,london-plane-pollarded,miscanthus-grass,multi-stem-birch,scots-pine,small-leaved-lime,weeping-willow";

const argv = process.argv.slice(2);
const opt = (name: string): string | undefined => { const i = argv.indexOf(`--${name}`); return i < 0 ? undefined : argv[i + 1]; };
const has = (name: string): boolean => argv.includes(`--${name}`);
const ids = (opt("assets") ?? ALL).split(",");
const renderSpec = new Map<string, number[] | undefined>((opt("render") ?? opt("assets") ?? ALL).split(",").map((e): [string, number[] | undefined] => { const [a, sc] = e.split(":"); return [a!, sc === undefined ? undefined : sc.split("/").map(Number)]; }));
const renderIds = [...renderSpec.keys()];
const onlyScales = opt("scales")?.split(",").map(Number);
const fractions = (opt("fractions") ?? "0.1,0.45,0.8").split(",").map(Number);
const poseFilter = opt("poses")?.split(",");
const clipFilter = opt("clips")?.split(",");

const { runPreview } = await import(join(S, "🔨️modules/🎨️depiction/🖨️preview/🟦️.ts"));
const { checkAssetDetailed } = await import(join(S, "🔨️modules/🗂️catalog/🩺️check/🟦️.ts"));
const { catalog, authoredScales, sweep, consistency } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));
const { DEFAULT_PRINT_PROFILE } = await import(join(S, "🔨️modules/🖨️representation/🟦️.ts"));

const assets = (catalog() as any[]).filter((a) => ids.includes(a.id));
mkdirSync(OUT, { recursive: true });
const lines: string[] = [];
const log = (line: string): void => { lines.push(line); console.log(line); };
const known = new Set((catalog() as any[]).map((a) => a.id));

if (!has("no-check")) {
  for (const asset of assets) {
    const report = checkAssetDetailed(asset, { samples: 5, profile: DEFAULT_PRINT_PROFILE, assets: known, physics: false });
    log(`CHECK ${asset.id}: ${report.problems.length === 0 ? "ok" : "PROBLEMS"}`);
    for (const p of report.problems.slice(0, 25)) log(`  ${p}`);
    for (const s of report.scales) if (s.status === "authored") log(`  1:${s.denominator} ${s.suitability?.suitable ? "suitable" : "UNSUITABLE"} stroke ${s.suitability?.minStrokeMm.toFixed(2)} gap ${s.suitability?.minGapMm.toFixed(2)} feature ${s.suitability?.minFeatureMm.toFixed(2)}`);
    for (const scale of authoredScales(asset)) {
      const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
      log(`GUARD ${asset.id} 1:${scale}: ${findings.length === 0 ? "ok" : findings.length + " findings"}`);
      for (const f of findings.slice(0, 12)) log(`  [${f.check}] ${f.message}`);
    }
  }
}

const pending: [string, string][] = [];
const collect = { rasterize(svg: string, png: string): void { pending.push([svg, png]); } };
if (!has("no-render")) {
  for (const asset of assets.filter((a) => renderIds.includes(a.id))) {
    const file = (() => { for (const g of readdirSync(join(S, "🗂️catalog"), { withFileTypes: true })) if (g.isDirectory()) { const f = join(S, "🗂️catalog", g.name, asset.id, "🔣️.json"); if (existsSync(f)) return f; } throw new Error(asset.id); })();
    for (const scale of authoredScales(asset)) {
      if (onlyScales && !onlyScales.includes(scale)) continue;
      const per = renderSpec.get(asset.id);
      if (per && !per.includes(scale)) continue;
      const dir = join(OUT, asset.id, String(scale));
      rmSync(dir, { recursive: true, force: true });
      const jobs: string[][] = [];
      const fr0 = fractions;
      if (!has("clips-only")) for (const p of asset.poses) if (!poseFilter || poseFilter.includes(p.id)) jobs.push(["--pose", p.id]);
      if (!has("poses-only")) for (const c of asset.clips) if (!clipFilter || clipFilter.includes(c.id)) for (const fr of fractions) jobs.push(["--clip", c.id, "--time", (c.durationS * fr).toFixed(3)]);
      for (const job of jobs) {
        try { await runPreview(["--file", file, "--scale", String(scale), "--views", "--dpi", "150", "--out", dir, ...job], collect); } catch (e) { log(`RENDER ${asset.id} 1:${scale} ${job.join(" ")}: ${(e as Error).message}`); }
      }
      log(`RENDER ${asset.id} 1:${scale}: ${jobs.length} sheets`);
    }
  }
  const list = join(OUT, "rasterize.json");
  writeFileSync(list, JSON.stringify(pending));
  const py = join(import.meta.dir, "raster.py");
  execFileSync("python", [py, list, "150"], { stdio: "inherit" });
  rmSync(list);
}
writeFileSync(join(OUT, "log.txt"), lines.join("\n"));
