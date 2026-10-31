import { readFileSync, writeFileSync, readdirSync, unlinkSync, existsSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const JOBS = process.argv[2]!;
mkdirSync(JOBS, { recursive: true });
const rep = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const chk = await import(`${S}/🔨️modules/🗂️catalog/🩺️check/🟦️.ts`);
const prev = await import(`${S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts`);
const dirs: Record<string, string> = {};
for (const [cat, list] of [["🛋️furniture", "park-bench cafe-chair office-swivel-chair height-adjustable-desk"], ["🪑️objects", "street-lamp market-parasol waste-bin playground-swing"]] as const) for (const i of list.split(" ")) dirs[i] = `${S}/🗂️catalog/${cat}/${i}/🔣️.json`;
const load = (id: string) => JSON.parse(readFileSync(dirs[id]!, "utf8")).asset;
console.log("[DEBUG] worker ready", JOBS);
for (;;) {
  const files = readdirSync(JOBS).filter((f) => f.endsWith(".job.json"));
  for (const f of files) {
    const p = `${JOBS}/${f}`;
    let job: any;
    try { job = JSON.parse(readFileSync(p, "utf8")); } catch { continue; }
    unlinkSync(p);
    const out: string[] = [];
    const log = (s: string) => out.push(s);
    const t0 = Date.now();
    try {
      for (const id of job.ids ?? []) {
        const asset = load(id);
        if (job.cmd === "probe") {
          for (const d of job.scales ?? [500, 200, 100, 50]) {
            const r = asset.representations[String(d)];
            if (r.status !== "authored") { log(`${id} 1:${d} unavailable`); continue; }
            const s = rep.measureSuitability(asset, r, d, rep.DEFAULT_PRINT_PROFILE);
            log(`${id} 1:${d} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${s.minGapMm.toFixed(3)} feature=${s.minFeatureMm.toFixed(3)} violations=${s.violations.length}`);
            for (const v of s.violations.slice(0, job.limit ?? 14)) log(`   ${v.check} ${v.shapes.join(" + ")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}${v.at ? ` at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}` : ""}`);
          }
        } else if (job.cmd === "check") {
          const report = chk.checkAssetDetailed(asset, { samples: job.samples ?? 4, profile: rep.DEFAULT_PRINT_PROFILE, assets: new Set(Object.keys(dirs)), physics: true });
          log(`${report.problems.length ? "✗" : "✓"} ${id}${report.lines.map((l: string) => `\n  ${l}`).join("")}${report.problems.length ? `\n  ${report.problems.slice(0, 40).join("\n  ")}` : ""}`);
        } else if (job.cmd === "script") {
          const m = await import(`${job.path}?t=${Date.now()}`);
          log(String(await m.default({ id, asset, S, load, job, modules: { rep, chk, prev } })));
        } else if (job.cmd === "preview") {
          const argv = ["--file", dirs[id]!, ...job.args];
          process.exitCode = 0;
          const w = await prev.runPreview(argv);
          log(w.join("\n") + (process.exitCode ? `\nexit ${process.exitCode}` : ""));
          process.exitCode = 0;
        }
      }
    } catch (e: any) { log("ERROR " + (e?.stack ?? e)); }
    log(`[${Date.now() - t0} ms]`);
    writeFileSync(`${JOBS}/${f.replace(".job.json", ".out")}`, out.join("\n") + "\n");
  }
  await new Promise((r) => setTimeout(r, 300));
}
