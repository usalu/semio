/** ⏱️ Times every taxonomy-verify phase; argv: <repoRoot> <deadlineSeconds> [scope|-] [reportOut]; env TAXONOMY_MODULE overrides the normalization module (old-baseline mirror), TAXONOMY_EXIT_AT=<operation/phase> exits on entering that phase. Exits from the progress callback at the deadline so --cpu-prof flushes. */
import { writeFileSync } from "node:fs";
const [repoRoot, deadline, scope, out] = process.argv.slice(2);
const { verifyTaxonomy } = await import(process.env.TAXONOMY_MODULE ?? `${repoRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`) as typeof import("../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
const shared = new SharedArrayBuffer(8192), sequence = new Int32Array(shared, 0, 2), pathBytes = new Uint8Array(shared, 8);
const watchdog = new Worker(URL.createObjectURL(new Blob([`self.onmessage = ({ data }) => { const seq = new Int32Array(data, 0, 2), bytes = new Uint8Array(data, 8); let last = -1, since = Date.now(); setInterval(() => { const now = Atomics.load(seq, 0); if (now !== last) { last = now; since = Date.now(); return; } if (Date.now() - since > 20000) { console.error("[DEBUG] stalled " + Math.round((Date.now() - since) / 1000) + "s after " + new TextDecoder().decode(bytes.slice(0, Atomics.load(seq, 1)))); since = Date.now(); } }, 1000); };`], { type: "application/javascript" })));
watchdog.postMessage(shared);
watchdog.unref();
const start = performance.now(), limit = start + Number(deadline) * 1000, spent = new Map<string, number>(), order: string[] = [];
let phase = "", phaseAt = start, lastPrint = start, stepAt = start, stepPath = "";
const summary = (): string => order.map((key) => `${key}=${(spent.get(key)! / 1000).toFixed(2)}s`).join(" ");
const verification = verifyTaxonomy({
  repoRoot: repoRoot!,
  ...(scope && scope !== "-" ? { scope } : {}),
  progress: (p) => {
    const key = `${p.operation}/${p.phase}`, now = performance.now();
    if (now - stepAt > 2000) console.error(`[DEBUG] slow step ${((now - stepAt) / 1000).toFixed(1)}s before ${key} ${p.current}/${p.total} after ${stepPath} now ${p.path ?? ""}`);
    stepAt = now;
    stepPath = `${key} ${p.current}/${p.total} ${p.path ?? ""}`;
    const encoded = new TextEncoder().encode(stepPath).slice(0, pathBytes.length);
    pathBytes.set(encoded);
    Atomics.store(sequence, 1, encoded.length);
    Atomics.add(sequence, 0, 1);
    if (!spent.has(key)) { order.push(key); spent.set(key, 0); console.error(`[DEBUG] ${((now - start) / 1000).toFixed(2)}s enter=${key} total=${p.total} rss=${(process.memoryUsage().rss / 2 ** 20).toFixed(0)}MiB`); }
    spent.set(phase || key, (spent.get(phase || key) ?? 0) + now - phaseAt);
    phase = key;
    phaseAt = now;
    if (now - lastPrint > 30000) { lastPrint = now; console.error(`[DEBUG] ${((now - start) / 1000).toFixed(0)}s ${key} ${p.current}/${p.total} ${summary()}`); }
    if (key === process.env.TAXONOMY_EXIT_AT) { console.error(`[DEBUG] exit at ${key} ${((now - start) / 1000).toFixed(2)}s ${summary()}`); process.exit(4); }
    if (now > limit) { console.error(`[DEBUG] deadline ${key} ${p.current}/${p.total} ${summary()}`); process.exit(3); }
  },
});
const lines = [`clean=${verification.clean} violations=${verification.violations.length}`, ...verification.violations.map((v) => `${v.severity} ${v.code} ${v.path}: ${v.message}`)];
if (out) writeFileSync(out, `${lines.join("\n")}\n`);
console.error(`[DEBUG] done ${((performance.now() - start) / 1000).toFixed(2)}s ${lines[0]} ${summary()}`);
watchdog.terminate();
