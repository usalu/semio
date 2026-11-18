import { existsSync, mkdtempSync, readFileSync, realpathSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createHash } from "node:crypto";
import { serialize } from "node:v8";
const [rootArg, countText] = process.argv.slice(2), root = rootArg, count = Number(countText);
const B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build", I = B + "/🕸️graph/📥️inference", P = I + "/📦️project-nodes", OP = P + "/🌐️owner-pool";
const { libraryBootstrap } = await import(I + "/🟨️.mjs"); await libraryBootstrap;
const pn = await import(P + "/🟨️.mjs"), obs = await import(B + "/👁️observation/🟨️.mjs"), pool = await import(OP + "/🟦️.ts");
const { startRemoteCacheServerV1 } = await import(B + "/🌐️remote-cache/🖥️server/🟦️.ts");
const { graphFleetClientV1 } = await import(B + "/🪪️graph-state/🛰️exchange/🟦️.ts"), { graphFleetSettingsV1 } = await import(B + "/🪪️graph-state/🧶️nx/🟦️.ts"), { remoteCacheFetchV1 } = await import(B + "/🌐️remote-cache/🛰️fleet/🟦️.ts");
const owners = readFileSync("/private/tmp/r68/owners2.txt", "utf8").split("\n").filter(l => l.startsWith("L ")).map(l => l.slice(2)).filter(r => existsSync(join(root, r, "📋️project.json"))).slice(0, count);
const store = mkdtempSync(join(realpathSync(tmpdir()), "r68-store-")), fleetRoot = mkdtempSync(join(realpathSync(tmpdir()), "r68-fleet-"));
const server = await startRemoteCacheServerV1({ directory: store, host: "127.0.0.1", port: 0, maximumBytes: 2 ** 31, maximumEntryBytes: 2 ** 29, maximumAgeDays: 30, defaultInstance: "main", tokens: { read: "r68-read", write: "r68-write" }, validation: "off" });
const fleet = { SEMIO_REMOTE_CACHE_ENDPOINT: server.url, SEMIO_REMOTE_CACHE_ACCESS: "read-write", SEMIO_REMOTE_CACHE_TOKEN: "r68-write", SEMIO_REMOTE_CACHE_INSTANCE: "main", SEMIO_LANGUAGE: "en" };
Object.assign(process.env, fleet);
const environment = { ...process.env };
const mods = {}; for (const [key, dir] of Object.entries({ ownerDiscovery: "🔎️owner-discovery", coordinates: "📍️coordinates", scriptInputs: "🧭️script-inputs", sourceAnalyses: "🧬️source-analyses", playgroundTargets: "🎮️playground-targets", nativeManifests: "🦀️native-manifests", componentTargets: "🧩️component-targets", projectAssembly: "🧱️project-assembly", bootstrap: "🔁️bootstrap", bunLock: "🧶️bun-lock", stamps: "🔖️stamps" })) mods[key] = await import(I + "/" + dir + "/🟨️.mjs");
const dataRoot = join(root, ".cache", "r68-data");rmSync(dataRoot, { recursive: true, force: true });
pn.wire({ ...mods, coordinates: { ...mods.coordinates, workspaceDataDirectory: () => dataRoot } });
const sha = text => createHash("sha256").update(text).digest("hex");
const contracts = await obs.observeInferenceSnapshotV1(() => pn.projectGeneratorContractsV1(root));
const globals = { projectAuthority: sha(JSON.stringify([contracts])), registryDigest: sha(JSON.stringify(null)), commandRevision: sha(JSON.stringify(null)), late: false };
const settings = graphFleetSettingsV1(fleetRoot, environment), client = graphFleetClientV1(settings, { fetch: remoteCacheFetchV1(environment, { root: fleetRoot }) });
const unitsOf = list => list.map(r => { const json = { ...JSON.parse(readFileSync(join(root, r, "📋️project.json"), "utf8")), root: r }; return { name: json.name, kind: "project", expectedMilliseconds: 30000, json }; });
const session = { version: 3, workspaceRoot: root, closureStore: undefined, registry: null, contracts, commandInputs: null, lateClosures: false, projectsByRoot: [], componentRoots: [] };
const wanted = Math.floor(owners.length * 0.9), units = unitsOf(owners.slice(0, wanted));
const partial = { version: 1, identity: obs.inferenceRecordIdentityV1(), globals, publisher: "r68:" + process.pid };
const head = { ...partial, id: pool.ownerPoolIdV1(partial, units), createdMs: Date.now(), revision: obs.projectRecordKeyTextV1(null) };
const out = line => process.stdout.write(line + "\n");
out("globals "+JSON.stringify(globals)+" units "+units.map(u=>u.name+":"+JSON.stringify(u.json).length).join(","));out("publish wave " + (await pool.publishOwnerPoolV1({ client, owner: head.publisher, manifest: { head, session: serialize(session), units } })) + " units " + units.length + " of " + owners.length);
const runner = await import(OP + "/🏃️runner/🟨️.mjs"), began = Date.now();
await runner.runOwnerPoolV1({ root, fleetRoot, environment, signal: new AbortController().signal, say: () => {}, capacity: async () => ({ children: 4, reason: null }), once: true, pollMilliseconds: 1000, targetPollMilliseconds: 500 });
import("node:child_process").then(()=>{});const acCount=(await import("node:child_process")).execSync("find "+store+" -path '*ac*' -type f | wc -l").toString().trim();out("pool done in " + Math.round((Date.now() - began) / 1000) + " s; entries " + JSON.stringify(server.claims?.summary?.()));
const configFiles = owners.map(r => r + "/📋️project.json"), lines = [], write = process.stderr.write.bind(process.stderr);
process.stderr.write = chunk => { lines.push(String(chunk)); return true; };
async function read(label, list) {
  lines.length = 0; let inferred = -1, failure; const began = Date.now(), observed = [];
  try { await obs.settleObservedComputationV1(() => pn.emojiProjectJsonNodes(configFiles, { analyzeLockfile: false, catalogProviders: [], snapshot: false, ownerRoots: new Set(list), onInferred: c => { inferred = c; }, onOwnerObservations: name => observed.push(name) }, { workspaceRoot: root, registry: null })); }
  catch (error) { failure = String(error?.message ?? error).slice(0, 120); }
  const text = lines.join(""); process.stderr.write = write;
  out(`== ${label}: owners ${list.length}, inferred here ${inferred}, taken/observed ${observed.length}, ${Date.now() - began} ms${failure ? ", ended with: " + failure : ""}`);
  out(text.split("\n").filter(l => l.startsWith("[owners]") || l.startsWith("[fleet]") || l.startsWith("[DEBUGx]")).slice(-6).join("\n"));
  process.stderr.write = chunk => { lines.push(String(chunk)); return true; };
}
await read("D other machine, empty local store: the 9 stored owners only", owners.slice(0, wanted));
rmSync(dataRoot, { recursive: true, force: true });
await server.stop(); rmSync(store, { recursive: true, force: true }); rmSync(fleetRoot, { recursive: true, force: true }); rmSync(dataRoot, { recursive: true, force: true });
process.exit(0);
