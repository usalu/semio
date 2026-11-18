// R66 proof: N real owners pulled through a private cache by (mode "new") the child-process pool or (mode "live") the live in-process puller.
import { existsSync, mkdtempSync, readFileSync, realpathSync, rmSync } from "node:fs";
import { tmpdir, setPriority } from "node:os";
import { join } from "node:path";
import { serialize } from "node:v8";
const [lib, mode, childrenText, countText, startText = "0"] = process.argv.slice(2), children = Number(childrenText), count = Number(countText), start = Number(startText);
const root = lib, B = lib + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build", I = B + "/🕸️graph/📥️inference", P = I + "/📦️project-nodes", OP = P + "/🌐️owner-pool";
try { setPriority(process.pid, 10); } catch {}
const { libraryBootstrap } = await import(I + "/🟨️.mjs"); await libraryBootstrap;
const pn = await import(P + "/🟨️.mjs"), obs = await import(B + "/👁️observation/🟨️.mjs"), pool = await import(OP + "/🟦️.ts");
const { startRemoteCacheServerV1 } = await import(B + "/🌐️remote-cache/🖥️server/🟦️.ts");
const { graphFleetClientV1 } = await import(B + "/🪪️graph-state/🛰️exchange/🟦️.ts"), { graphFleetSettingsV1 } = await import(B + "/🪪️graph-state/🧶️nx/🟦️.ts"), { remoteCacheFetchV1 } = await import(B + "/🌐️remote-cache/🛰️fleet/🟦️.ts");
const { createProjectFleetPortV1 } = await import(B + "/🪪️graph-state/📦️project-records/🟦️.ts");
const owners = readFileSync(new URL("owners2.txt", import.meta.url), "utf8").split("\n").filter(line => (process.env.R66_KINDS ?? "L").split("").some(kind => line.startsWith(kind + " "))).map(line => line.slice(2)).filter(r => existsSync(join(root, r, "📋️project.json"))).slice(start, start + count);
const fleetRoot = mkdtempSync(join(realpathSync(tmpdir()), "r66-fleet-")), store = mkdtempSync(join(realpathSync(tmpdir()), "r66-store-"));
const server = await startRemoteCacheServerV1({ directory: store, host: "127.0.0.1", port: 0, maximumBytes: 2 ** 31, maximumEntryBytes: 2 ** 29, maximumAgeDays: 30, defaultInstance: "main", tokens: { read: "r66-read", write: "r66-write" }, validation: "off" });
const environment = { ...process.env, SEMIO_REMOTE_CACHE_ENDPOINT: server.url, SEMIO_REMOTE_CACHE_ACCESS: "read-write", SEMIO_REMOTE_CACHE_TOKEN: "r66-write", SEMIO_REMOTE_CACHE_INSTANCE: "main" };
const settings = graphFleetSettingsV1(fleetRoot, environment);
if (settings.endpoint !== server.url) { console.log("WRONG CACHE", settings.endpoint); process.exit(3); }
const contracts = await obs.observeInferenceSnapshotV1(() => pn.projectGeneratorContractsV1(root));
const session = { version: 3, workspaceRoot: root, closureStore: undefined, registry: null, contracts, commandInputs: null, lateClosures: false, projectsByRoot: [], componentRoots: [] };
const globals = { projectAuthority: "r66", registryDigest: "r66", commandRevision: "r66", late: false };
const units = owners.map(r => { const json = { ...JSON.parse(readFileSync(join(root, r, "📋️project.json"), "utf8")), root: r }; return { name: json.name, kind: "project", expectedMilliseconds: 30000, json }; });
const partial = { version: 1, identity: obs.inferenceRecordIdentityV1(), globals, publisher: "r66:" + process.pid, ...(mode === "live" ? { peaks: {} } : {}) };
const head = { ...partial, id: pool.ownerPoolIdV1(partial, units), createdMs: Date.now(), revision: obs.projectRecordKeyTextV1 ? obs.projectRecordKeyTextV1(null) : undefined };
const client = graphFleetClientV1(settings, { fetch: remoteCacheFetchV1(environment, { root: fleetRoot }) });
console.log("publish", await pool.publishOwnerPoolV1({ client, owner: head.publisher, manifest: { head, session: serialize(session), units } }), "owners", units.length, "mode", mode, "children", children);
const runner = await import(OP + "/🏃️runner/🟨️.mjs");
const began = Date.now(), lines = [], dones = [];
let peakRss = 0;
const sampler = setInterval(() => { peakRss = Math.max(peakRss, process.memoryUsage().rss); }, 500);
const cpu0 = process.cpuUsage();
if (mode === "new") {
  await runner.runOwnerPoolV1({ root, fleetRoot, environment, signal: new AbortController().signal, say: line => lines.push(line), capacity: async () => ({ children, reason: null }), once: true, pollMilliseconds: 1000, targetPollMilliseconds: 500, observe: (id, line, at) => { if (line.t === "done") dones.push({ id, at: at - began, ...line }); if (line.t === "fail") lines.push(`fail ${line.name} ${line.message}`); if (line.t === "ready") lines.push(`child${id} ready ${at - began} ms open=${line.openMilliseconds} rss=${line.rss}`); if (line.t === "end") lines.push(`child${id} end ${line.reason} at ${at - began} ${JSON.stringify(line.stats)} rss=${line.rss}`); } });
} else {
  await runner.runOwnerPoolV1({ root, fleetRoot, environment, signal: new AbortController().signal, say: line => lines.push(`${Date.now() - began} ${line}`), pollMilliseconds: Number(process.env.R66_POLL ?? 15000), once: true, cores: children + 1 });
}
clearInterval(sampler);
const wall = (Date.now() - began) / 1000, used = process.cpuUsage(cpu0);
console.log(lines.join("\n"));
const ok = dones.filter(d => d.ok);
const result = { mode, children, owners: units.length, wallSeconds: wall, ownersPerMinuteWall: Math.round(units.length / wall * 600) / 10, parentCpuSeconds: Math.round((used.user + used.system) / 1e5) / 10, parentPeakRssMb: Math.round(peakRss / 1048576) };
if (mode === "new" && ok.length) {
  const first = Math.min(...ok.map(d => d.at)), last = Math.max(...ok.map(d => d.at));
  const cpus = ok.map(d => d.cpuMilliseconds).sort((a, b) => a - b), peaks = ok.map(d => d.peak).sort((a, b) => a - b), ms = ok.map(d => d.milliseconds).sort((a, b) => a - b);
  Object.assign(result, { done: ok.length, failed: dones.length - ok.length, firstDoneMs: first, lastDoneMs: last, ownersPerMinuteSteady: Math.round(ok.length / ((last - first) / 60000) * 10) / 10, cpuMsMedian: cpus[Math.floor(cpus.length / 2)], cpuMsP90: cpus[Math.floor(cpus.length * 0.9)], cpuMsTotal: cpus.reduce((a, b) => a + b, 0), wallMsMedian: ms[Math.floor(ms.length / 2)], wallMsP90: ms[Math.floor(ms.length * 0.9)], childPeakMbMedian: peaks[Math.floor(peaks.length / 2)], childPeakMbMax: peaks.at(-1) });
}
console.log("RESULT " + JSON.stringify(result));
const claims = server.claims.summary?.();
console.log("claims", JSON.stringify(claims));
await server.stop(); rmSync(store, { recursive: true, force: true }); rmSync(fleetRoot, { recursive: true, force: true });
process.exit(0);
