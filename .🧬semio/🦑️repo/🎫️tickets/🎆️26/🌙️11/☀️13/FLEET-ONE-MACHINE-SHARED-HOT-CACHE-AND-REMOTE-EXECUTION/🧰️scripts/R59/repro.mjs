// R49: reproduce a pool failure of one real owner of the live wave in a private session against a private cache (production is read-only here).
import { mkdtempSync, realpathSync } from "node:fs"; import { tmpdir } from "node:os"; import { join } from "node:path";
const root = "/Users/ueli/Documents/semio", B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build", N = B + "/🕸️graph/📥️inference/📦️project-nodes";
const want = process.argv[2] ?? "semio-framework-geometry";
const ex = await import(B + "/🪪️graph-state/🛰️exchange/🟦️.ts"), { graphFleetSettingsV1 } = await import(B + "/🪪️graph-state/🧶️nx/🟦️.ts"), { remoteCacheFetchV1 } = await import(B + "/🌐️remote-cache/🛰️fleet/🟦️.ts");
const pool = await import(N + "/🌐️owner-pool/🟦️.ts"), { startRemoteCacheServerV1 } = await import(B + "/🌐️remote-cache/🖥️server/🟦️.ts");
const prod = { ...process.env }; for (const k of Object.keys(prod)) if (k.startsWith("SEMIO_REMOTE_CACHE")) delete prod[k];
const pclient = ex.graphFleetClientV1(graphFleetSettingsV1(root, prod), { fetch: remoteCacheFetchV1(prod, { root }) });
const pointer = await pool.readOwnerPoolPointerV1(pclient); console.log("pointer", pointer?.id.slice(0, 12), pointer?.units);
const manifest = await pool.readOwnerPoolV1(pclient, pointer.id);
const unit = manifest.units.find(u => u.name === want); if (!unit) { console.log("no such unit; have", manifest.units.map(u => u.name).slice(0, 100000).join(",")); process.exit(1); }
const store = mkdtempSync(join(realpathSync(tmpdir()), "r49-repro-store-")), fleetRoot = mkdtempSync(join(realpathSync(tmpdir()), "r49-repro-root-"));
const server = await startRemoteCacheServerV1({ directory: store, host: "127.0.0.1", port: 0, maximumBytes: 2 ** 31, maximumEntryBytes: 2 ** 29, maximumAgeDays: 30, defaultInstance: "main", tokens: { read: "r", write: "w" }, validation: "off" });
const environment = { ...process.env, SEMIO_REMOTE_CACHE_ENDPOINT: server.url, SEMIO_REMOTE_CACHE_ACCESS: "read-write", SEMIO_REMOTE_CACHE_TOKEN: "w", SEMIO_REMOTE_CACHE_INSTANCE: "main" };
const client = ex.graphFleetClientV1(graphFleetSettingsV1(fleetRoot, environment), { fetch: remoteCacheFetchV1(environment, { root: fleetRoot }) });
if (graphFleetSettingsV1(fleetRoot, environment).endpoint !== server.url) { console.log("WRONG CACHE"); process.exit(3); }
const partial = { version: 1, identity: manifest.head.identity, revision: manifest.head.revision, globals: manifest.head.globals, peaks: manifest.head.peaks, publisher: "repro" };
const units = [unit], head = { ...partial, id: pool.ownerPoolIdV1(partial, units), createdMs: Date.now() };
console.log("publish", await pool.publishOwnerPoolV1({ client, owner: "repro", manifest: { head, session: manifest.session, units } }));
const { runOwnerPoolV1 } = await import(N + "/🌐️owner-pool/🏃️runner/🟨️.mjs"), { createOwnerPoolStatusV1 } = await import(N + "/🌐️owner-pool/📊️status/🟦️.ts");
const sroot = mkdtempSync(join(realpathSync(tmpdir()), "r49-repro-status-")), status = createOwnerPoolStatusV1({ store: sroot, origin: "repro", role: "puller" });
console.log("STORE", store, fleetRoot); const started = Date.now();
await runOwnerPoolV1({ root, environment, signal: new AbortController().signal, say: l => console.log("say", l.slice(0, 200)), pollMilliseconds: 500, once: true, cores: 3, fleetRoot, status });
console.log("seconds", Math.round((Date.now() - started) / 1000), JSON.stringify(status.current(), (k, v) => k === "reasonText" ? undefined : v));
process.exit(0);
