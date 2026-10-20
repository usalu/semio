// ⏱️ Cold and warm resolution of 400 real crates: graph-time receipt refresh (plain reads, writer), hash-time without receipts (live batch reader) and hash-time over the receipts.
import {readFileSync, rmSync, mkdirSync, readdirSync, statSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
const SB = "/private/tmp/gk/sb", L = SB + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", REAL = "/Users/ueli/Documents/semio", STORE = "/private/tmp/gk/rr-store";
const read: any = await import(L + "/🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs");
const engine: any = await import(L + "/🏗️build/📥️inputs/🦀️resolution/🟦️.ts");
const receipt: any = await import(L + "/🏗️build/📥️inputs/🦀️resolution/💾️receipt/🟨️.mjs");
const routing: any = await import(SB + "/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
const caller: any = await import(L + "/🏗️build/📥️caller/🟦️.ts");
const policy = engine.readCargoResolutionPolicyV1(JSON.parse(readFileSync(L + "/🏗️build/📥️inputs/🦀️resolution/🔣️.json", "utf8")));
const caps = caller.createNativeBuildCallerCapabilitiesV1(SB, ["run", "fixture:build"], "bazel", {SEMIO_BUILD_MODE: "prototype"});
const run = <T>(fn: (c: any) => Promise<T>) => routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version: 1, owner: "gk:measure", maximumElapsedMilliseconds: 900000}, caps, Date.now()), (o: any) => fn(routing.createIssuedResourceControlV1(o, "source:measure", 900000)));
const all = JSON.parse(readFileSync("/private/tmp/gk/authored.json", "utf8"));
const keys = Object.keys(all).filter((k) => { if (!k || k === ".") return false; try { readFileSync(REAL + "/" + k + "/Cargo.toml"); return true; } catch { return false; } }).slice(0, Number(process.env.N ?? 400)).map((k) => k + "/Cargo.toml");
const sha = (b: Uint8Array | string) => createHash("sha256").update(b).digest("hex");
const tomlParse = (await import("/Users/ueli/Documents/semio/node_modules/@iarna/toml/toml.js" as string).catch(() => null))?.parse ?? ((t: string) => Bun.TOML.parse(t));
const rawHost = () => { const bytes = (p: string) => { try { return readFileSync(join(REAL, p)); } catch { return null; } }; return {read: async (p: string) => { const b = bytes(p); return b === null ? null : {value: tomlParse(b.toString("utf8")), sha: sha(b)}; }, digest: async (p: string) => { const b = bytes(p); return b === null ? null : sha(b); }}; };
const summary = (c: any) => ({derived: c.counts.derived, reused: c.counts.reused, parsedFiles: c.counts.parsed, digestedFiles: c.counts.digested});
const out: Record<string, unknown> = {crates: keys.length};
rmSync(STORE, {recursive: true, force: true}); mkdirSync(STORE, {recursive: true});
const publish = (_p: string, _plan: unknown, write: () => void) => { write(); return true; };
const writer = receipt.createCargoReceiptWriterV1(STORE, publish);
// 1. graph build, cold: no receipts yet
let t = performance.now(); let c = engine.createCargoResolutionContextV1(rawHost(), policy, writer);
const graphIds: string[] = [];
for (const m of keys) { const p = await engine.cargoResolutionPartsV1(c, m, {tolerant: true}); graphIds.push(engine.composeCargoResolutionV1(p)); }
const written = await engine.flushCargoReceiptsV1(c);
out.graphCold = {seconds: +((performance.now() - t) / 1000).toFixed(1), ...summary(c), receiptsWritten: written};
let bytes = 0, files = 0; for (const f of readdirSync(join(STORE, "rr"))) { bytes += statSync(join(STORE, "rr", f)).size; files++; }
out.receiptStore = {leaves: files, bytes, bytesPerCrate: Math.round(bytes / files)};
// 2. graph build, warm
t = performance.now(); c = engine.createCargoResolutionContextV1(rawHost(), policy, receipt.createCargoReceiptWriterV1(STORE, publish));
const warmIds: string[] = [];
for (const m of keys) warmIds.push(engine.composeCargoResolutionV1(await engine.cargoResolutionPartsV1(c, m, {tolerant: true})));
out.graphWarm = {seconds: +((performance.now() - t) / 1000).toFixed(1), ...summary(c), written: await engine.flushCargoReceiptsV1(c), equalToCold: JSON.stringify(warmIds) === JSON.stringify(graphIds)};
// 3. hash time through the live batch reader: no receipts, then receipts
const batchContext = (batch: any, store?: any) => engine.createCargoResolutionContextV1({read: async (p: string) => { const f = await read.readExactSourceTomlV1(batch, p); return f ? {value: f.value, sha: f.sha256} : null; }, digest: async (p: string) => (await read.readExactSourceFileV1(batch, p))?.sha256 ?? null}, policy, store);
for (const [label, store] of [["hashCold", undefined], ["hashWarm", receipt.createCargoReceiptReaderV1(STORE)]] as const) {
  const ids: string[] = []; let info: any;
  const start = performance.now();
  await run((control) => read.withExactSourceReadBatchV1(REAL, control, async (batch: any) => { const ctx = batchContext(batch, store); for (const m of keys) ids.push(engine.composeCargoResolutionV1(await engine.cargoResolutionPartsV1(ctx, m, {tolerant: true}))); info = summary(ctx); }));
  out[label] = {seconds: +((performance.now() - start) / 1000).toFixed(1), ...info, equalToGraph: JSON.stringify(ids) === JSON.stringify(graphIds)};
}
console.log(JSON.stringify(out, null, 1));
