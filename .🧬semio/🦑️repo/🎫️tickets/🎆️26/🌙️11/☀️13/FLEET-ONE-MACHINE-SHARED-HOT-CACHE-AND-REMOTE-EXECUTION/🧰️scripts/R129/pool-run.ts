// R129: the real farm (pool + worker process) over a sequence of owners; env R129_OWNERS, R129_MB (child reservation), R129_OVER/R129_PRE for the staged code in the children
import { readFileSync, mkdtempSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { deserialize } from "node:v8";
import { spawnSync } from "node:child_process";
const root = "/Users/ueli/Documents/semio", code = process.env.R129_CODE ?? root;
const B = code + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build", PN = B + "/🕸️graph/📥️inference/📦️project-nodes", FARM = PN + "/🏊️project-farm";
const dir = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R129/slim";
const { libraryBootstrap } = await import(B + "/🕸️graph/📥️inference/🟨️.mjs"); await libraryBootstrap;
const R: any = await import(PN + "/🟨️.mjs"), pool: any = await import(FARM + "/🏊️pool/🟨️.mjs"), service: any = await import(FARM + "/👷️service/🟨️.mjs");
const head = JSON.parse(readFileSync(dir + "/head.json", "utf8")), units: any[] = JSON.parse(readFileSync(dir + "/units.json", "utf8")), session = deserialize(readFileSync(dir + "/session.bin"));
const base = mkdtempSync(join(realpathSync(tmpdir()), "semio-r129p-"));
const full = { ...session, workspaceRoot: root, closureStore: undefined, stagingBase: base, authority: undefined };
await service.open(full, { slot: 9, generation: 1 });
const envr = service.sessionEnvironmentV1();
const names = (process.env.R129_OWNERS ?? "").split(",").filter(Boolean), megabytes = Number(process.env.R129_MB ?? 1536);
const rows: any[] = [];
const pre = process.env.R129_PRE ? ["--preload", process.env.R129_PRE] : [];
const farm = pool.createFarmV1({ entry: FARM + "/👷️worker/🟨️.mjs", service: pathToFileURL(FARM + "/👷️service/🟨️.mjs").href, session: full, workers: 1, megabytes, environment: { ...process.env, BUN_RUNTIME_TRANSPILER_CACHE_PATH: "0", MIMALLOC_PURGE_DELAY: "0" }, commandArguments: pre, accept: () => true, report: (kind: string, fields: any) => { if (kind === "recycled") console.log("[recycled]", JSON.stringify(fields)); } });
const rssOf = (pid: number) => Number(spawnSync("ps", ["-o", "rss=", "-p", String(pid)], { encoding: "utf8" }).stdout.trim() || 0) / 1024;
const t0 = performance.now();
for (const name of names) {
  const unit = units.find(u => u.name === name); if (!unit) continue;
  const job = R.ownerJobV1(envr, head.globals, unit), t = performance.now();
  const answer = await farm.run(job);
  const st = farm.stats();
  console.log(JSON.stringify({ name: name.replace("@semio-tech/", ""), seconds: Math.round((performance.now() - t) / 100) / 10, ok: answer.reply !== undefined, footprintMb: answer.footprintMegabytes, started: st.started, recycled: st.recycled, bootMs: Math.round(st.bootMilliseconds) }));
}
const st = farm.stats();
console.log("TOTAL", JSON.stringify({ owners: names.length, seconds: Math.round((performance.now() - t0) / 100) / 10, started: st.started, recycled: st.recycled, bootMs: Math.round(st.bootMilliseconds), peakRssMb: Math.round(st.peakRssMegabytes), peakFootprintMb: Math.round(st.peakFootprintMegabytes) }));
await farm.close(); rmSync(base, { recursive: true, force: true }); process.exit(0);
