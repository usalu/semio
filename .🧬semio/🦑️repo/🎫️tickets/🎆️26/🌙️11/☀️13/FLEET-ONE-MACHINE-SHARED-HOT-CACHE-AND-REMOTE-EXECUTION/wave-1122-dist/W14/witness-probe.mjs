/** 🔬️ W14 probe of the closure-entry stamp verification a look runs (`entryStampHoldsV1` -> `observationWitnessHoldsV1`): S stored stamps of a selection store, verified by N owners, once with fresh memos per verification (a read without a validation round: the selected read today) and once with the memos of one pass round; prints synchronous fs calls, distinct paths, verdicts. Env: STAMPS (directory `ce/s`), S, N. */
import { readdirSync, readFileSync } from "node:fs"; import { brotliDecompressSync } from "node:zlib";
const B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference = await import(B + "🕸️graph/📥️inference/🟨️.mjs"); await inference.libraryBootstrap;
const obs = await import(B + "👁️observation/🟨️.mjs"), C = globalThis.__w14, dir = process.env.STAMPS, S = Number(process.env.S ?? 30), N = Number(process.env.N ?? 20);
const names = readdirSync(dir).sort(), stamps = [];
for (const name of names) { if (stamps.length >= S) break; try { const w = JSON.parse(brotliDecompressSync(readFileSync(dir + "/" + name)).toString()).witness; if (w) stamps.push(w); } catch {} }
const step = () => new Promise(r => setImmediate(r)), reset = () => { C.calls = {}; C.callers = {}; C.paths = {}; };
for (const mode of ["owner", "pass"]) {
  reset(); const shared = { tuples: new Map(), verdicts: new Map() }; let holds = 0; const t = performance.now();
  for (let owner = 0; owner < N; owner++) for (const w of stamps) if (await obs.observationWitnessHoldsV1(w, { ...(mode === "pass" ? shared : { tuples: new Map(), verdicts: new Map() }), step })) holds++;
  console.log(JSON.stringify({ mode, stamps: stamps.length, owners: N, paths: stamps.reduce((s, w) => s + (w.paths?.length ?? 0), 0), holds, wallMs: Math.round(performance.now() - t), calls: C.calls, distinct: Object.fromEntries(Object.entries(C.paths).map(([k, v]) => [k, v.size])), top: Object.entries(C.callers).sort((a, b) => b[1] - a[1]).slice(0, 6) }));
}
