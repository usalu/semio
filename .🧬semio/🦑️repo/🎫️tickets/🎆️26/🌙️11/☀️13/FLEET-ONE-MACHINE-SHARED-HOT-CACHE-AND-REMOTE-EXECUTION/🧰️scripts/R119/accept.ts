// R119: the reader's accept path (gate/import -> settle closure entries -> verify rows) per owner, cpu per step.
// env: R119_OWNERS=comma names (default: first R119_N of the wave in file order), R119_N, R119_REFS=1
import { readFileSync, readdirSync, statSync, mkdtempSync, realpathSync, rmSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { deserialize } from "node:v8";
const root = process.env.R119_ROOT ?? "/Users/ueli/Documents/semio";
const B = (process.env.R119_CODE ?? root) + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const PN = B + "/🕸️graph/📥️inference/📦️project-nodes";
const { libraryBootstrap } = await import(B + "/🕸️graph/📥️inference/🟨️.mjs"); await libraryBootstrap;
const { remoteCacheFetchV1 } = await import(B + "/🌐️remote-cache/🛰️fleet/🟦️.ts"); const { graphFleetSettingsV1 } = await import(B + "/🪪️graph-state/🧶️nx/🟦️.ts");
const X: any = await import(B + "/🪪️graph-state/🛰️exchange/🟦️.ts"); const R: any = await import(PN + "/🟨️.mjs");
const PR: any = await import(B + "/🪪️graph-state/📦️project-records/🟦️.ts");
const OB: any = await import(B + "/👁️observation/🟨️.mjs"); const SI: any = await import(B + "/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const { resolveClosureRefsV1 } = await import(B + "/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs") as any;
const env: any = { ...process.env }; for (const k of Object.keys(env)) if (k.startsWith("SEMIO_REMOTE_CACHE")) delete env[k];
const client = X.graphFleetClientV1(graphFleetSettingsV1(root, env), { fetch: remoteCacheFetchV1(env, { root }) });
const wavesDir = root + "/.cache/f/owner-pool-waves";
const waves = readdirSync(wavesDir).filter(n => existsSync(join(wavesDir, n, "units.json")) && existsSync(join(wavesDir, n, "session.bin")) && existsSync(join(wavesDir, n, "head.json")))
  .map(n => ({ n, head: JSON.parse(readFileSync(join(wavesDir, n, "head.json"), "utf8")) })).sort((a, b) => b.head.createdMs - a.head.createdMs);
const size = (n: string) => statSync(join(wavesDir, n, "units.json")).size, biggest = Math.max(...waves.map(w => size(w.n)));
const wave = process.env.R119_WAVE ? waves.find(w => w.n === process.env.R119_WAVE)! : waves.find(w => size(w.n) >= 0.8 * biggest)!;
const units: any[] = JSON.parse(readFileSync(join(wavesDir, wave.n, "units.json"), "utf8")), session = deserialize(readFileSync(join(wavesDir, wave.n, "session.bin")));
const base = mkdtempSync(join(realpathSync(tmpdir()), "semio-r119-"));
const service: any = await import(PN + "/🏊️project-farm/👷️service/🟨️.mjs");
await service.open({ ...session, workspaceRoot: root, closureStore: undefined, stagingBase: base, authority: undefined }, { slot: 0, generation: 1 });
const envr = service.sessionEnvironmentV1();
SI.bindClosureStoreV1(root, process.env.R119_STORE ?? join(base, "ce"), true); SI.bindClosureScriptsV1(root, SI.createScriptInputCache({ lateClosures: true }));
const port = PR.createProjectFleetPortV1({ root, environment: env, client });
const names = (process.env.R119_OWNERS ?? "").split(",").filter(Boolean), N = Number(process.env.R119_N ?? 5), skip = Number(process.env.R119_SKIP ?? 0);
const pick = names.length ? names.map(n => units.find(u => u.name === n)).filter(Boolean) : units.slice(skip);
const cpu = () => { const u = process.cpuUsage(); return (u.user + u.system) / 1000; };
const rows: any[] = []; const t0 = performance.now(), c0 = cpu(); let accepted = 0;
for (const unit of pick) {
  if (!names.length && accepted >= N) break;
  const job = R.ownerJobV1(envr, wave.head.globals, unit);
  const keyText = R.ownerRecordKeyTextV1(R.projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs));
  const state = PR.projectRecordStateV1({ root, fleet: job.root, keyText, platform: "darwin-arm64" });
  let got: any; const info = { cachePath: join(base, "x.json"), fleet: job.root, keyText, exportRecord: () => undefined, closures: SI.closureTravelV1, import: async (p: any) => { got = p; return true; } };
  const step: Record<string, number> = {}, tick = async (k: string, f: () => any) => { const a = cpu(), w = performance.now(); try { return await f(); } finally { step[k] = (step[k] ?? 0) + cpu() - a; step[k + "Wall"] = (step[k + "Wall"] ?? 0) + performance.now() - w; } };
  const a0 = cpu(), w0 = performance.now(); let verdict: any = "nogate";
  await tick("gate", async () => { try { await PR.runTakeLookV1(() => SI.runClosureTakeV1(() => port.gate(info))); } catch (e: any) { verdict = "gate-throw:" + e.code; } });
  if (got) {
    const value = JSON.parse(got.image), obs = got.observations; const before = SI.closureEntryCountsV1();
    try {
      await SI.withClosurePassV1(() => OB.observeInferenceSnapshotV1(() => SI.runClosureTakeV1(async () => {
        await tick("settle", () => OB.settleClosureRowsV1(obs, value));
        verdict = await tick("verify", () => OB.projectRowsHoldV1(obs, value, (rest: any) => OB.observationRowsHoldV1(rest)));
        if (process.env.R119_REFS === "1" && verdict === true) await tick("refs", () => resolveClosureRefsV1(value, root, envr.scripts, "hash", undefined));
      })), process.env.R119_VERDICTS ? { verdicts: process.env.R119_VERDICTS } : undefined);
    } catch (e: any) { verdict = "throw:" + (e.code ?? "") + ":" + String(e.message).slice(0, 60); }
    const after = SI.closureEntryCountsV1(); step.closureRows = obs.filter((r: any) => r.kind === "closure").length; step.rows = obs.length;
    step.delta = JSON.stringify(Object.fromEntries(Object.keys(after).map(k => [k, after[k] - before[k]]).filter(([, v]: any) => v)));
  }
  if (verdict === true) accepted++;
  rows.push({ name: unit.name, verdict, cpuMs: Math.round(cpu() - a0), wallMs: Math.round(performance.now() - w0), ...Object.fromEntries(Object.entries(step).map(([k, v]) => [k, typeof v === "number" ? Math.round(v) : v])) });
  console.log(JSON.stringify(rows[rows.length - 1]));
}
const sec = (performance.now() - t0) / 1000;
console.log("TOTAL", JSON.stringify({ owners: rows.length, accepted, seconds: Math.round(sec * 10) / 10, cpuSeconds: Math.round((cpu() - c0) / 100) / 10, acceptedPerMinute: Math.round(accepted / sec * 600) / 10, closure: SI.closureEntryCountsV1(), layers: SI.layerVerdictCountsV1?.() }));
rmSync(base, { recursive: true, force: true }); process.exit(0);
