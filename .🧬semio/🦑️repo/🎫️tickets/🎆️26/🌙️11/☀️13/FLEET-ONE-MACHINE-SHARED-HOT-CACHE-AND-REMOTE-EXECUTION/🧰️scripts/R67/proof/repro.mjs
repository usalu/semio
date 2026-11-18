// R67: reproduce the publish of one real owner against a private production-configured cache server (production is only READ for the manifest).
import { mkdtempSync, realpathSync, readFileSync, writeFileSync } from "node:fs"; import { tmpdir } from "node:os"; import { join } from "node:path";
const LIVE = "/Users/ueli/Documents/semio", ws = process.env.R67_WS ?? "/private/tmp/r67/ws";
const want = process.argv[2] ?? "semio-s-artifact-procedural-2d-editor-commands";
const B = ws + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build", I = B + "/🕸️graph/📥️inference", N = I + "/📦️project-nodes";
const { libraryBootstrap } = await import(I + "/🟨️.mjs"); await libraryBootstrap;
const ex = await import(B + "/🪪️graph-state/🛰️exchange/🟦️.ts"), { graphFleetSettingsV1 } = await import(B + "/🪪️graph-state/🧶️nx/🟦️.ts"), { remoteCacheFetchV1 } = await import(B + "/🌐️remote-cache/🛰️fleet/🟦️.ts");
const pool = await import(N + "/🌐️owner-pool/🟦️.ts"), { startRemoteCacheServerV1 } = await import(B + "/🌐️remote-cache/🖥️server/🟦️.ts");
let manifest;
const cache = process.env.R67_MANIFEST;
if (cache) manifest = JSON.parse(readFileSync(cache, "utf8")), manifest.session = Buffer.from(manifest.session, "base64");
else {
  const prod = { ...process.env }; for (const k of Object.keys(prod)) if (k.startsWith("SEMIO_REMOTE_CACHE")) delete prod[k];
  const pclient = ex.graphFleetClientV1(graphFleetSettingsV1(LIVE, prod), { fetch: remoteCacheFetchV1(prod, { root: LIVE }) });
  const pointer = await pool.readOwnerPoolPointerV1(pclient); console.log("pointer", JSON.stringify(pointer)?.slice(0, 300));
  const wave = pointer.waves.at(-1); manifest = await pool.readOwnerPoolV1(pclient, wave.id);
  const unit = manifest.units.find(u => u.name === want); if (!unit) { console.log("no unit"); process.exit(1); }
  writeFileSync(process.env.R67_SAVE ?? "manifest-one.json", JSON.stringify({ head: manifest.head, units: [unit], session: Buffer.from(manifest.session).toString("base64") }));
  manifest = { head: manifest.head, units: [unit], session: manifest.session };
}
const unit = manifest.units.find(u => u.name === want);
console.log("unit", unit.name, unit.kind, JSON.stringify(unit.json).slice(0, 300));
const store = mkdtempSync(join(realpathSync(tmpdir()), "r67-store-")), fleetRoot = mkdtempSync(join(realpathSync(tmpdir()), "r67-fleet-"));
const server = await startRemoteCacheServerV1({ directory: store, host: "127.0.0.1", port: 0, maximumBytes: 2684354560, maximumEntryBytes: 536870912, maximumAgeDays: 30, graceMilliseconds: 600000, defaultInstance: "main", tokens: { read: "r", write: "w" }, validation: "complete", maximumActionResultBytes: 4194304 });
const environment = { ...process.env, SEMIO_REMOTE_CACHE_ENDPOINT: server.url, SEMIO_REMOTE_CACHE_ACCESS: "read-write", SEMIO_REMOTE_CACHE_TOKEN: "w", SEMIO_REMOTE_CACHE_INSTANCE: "main" };
if (graphFleetSettingsV1(fleetRoot, environment).endpoint !== server.url) { console.log("WRONG CACHE"); process.exit(3); }
const { openOwnerPoolInferenceV1 } = await import(N + "/🌐️owner-pool/🔬️inference/🟨️.mjs");
const opening = await openOwnerPoolInferenceV1({ root: ws, fleetRoot, environment, session: manifest.session, globals: manifest.head.globals, say: l => console.log("say", l) });
const began = Date.now();
const claim = await opening.claim(unit); console.log("claim", claim.state);
if (claim.state === "granted") {
  console.log("run", await opening.run(unit), Date.now() - began, "ms");
  const t = Date.now(); const stored = await claim.publish(); console.log("publish", stored, Date.now() - t, "ms", "why:", claim.why(), "deterministic:", claim.deterministic());
}
console.log("server", JSON.stringify(await (await fetch(server.url + "/status", { headers: { authorization: "Bearer r" } })).json()).slice(0, 900));
await server.stop(); process.exit(0);
