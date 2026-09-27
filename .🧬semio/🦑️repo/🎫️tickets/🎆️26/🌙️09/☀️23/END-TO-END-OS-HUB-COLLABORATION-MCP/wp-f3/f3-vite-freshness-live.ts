/** 🔬️ F3 — the Vite activation plugin's receipt-change freshness pass, live: a real receipt directory + watcher, a staged module
 * without a stat index over a 3000-file source tree, then two receipt changes back to back. Records the longest event-loop
 * gap while the pass runs (5 ms ticker) and the `[stale]` lines it prints; the first pass must be superseded (aborted) by the
 * second. usage: bun f3-vite-freshness-live.ts */
import { createServer } from "node:http";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const DEV = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { semioActivationVitePlugin } = await import(`${DEV}/🔌️vite-plugins/🟦️.ts`);
const act = await import(`${DEV}/♻️activation/🟦️.ts`);
const sandbox = mkdtempSync(join(tmpdir(), "f3-vite-fresh-"));
const warnings: string[] = [];
const originalWarn = console.warn;
console.warn = (...parts: unknown[]) => { warnings.push(`${Math.round(performance.now())} ${parts.join(" ")}`); };
const server = createServer();
try {
  const receiptDirectory = join(sandbox, "activation"), moduleRoot = join(sandbox, "modules"), sourceRoot = join(sandbox, "source");
  mkdirSync(receiptDirectory, { recursive: true });
  mkdirSync(join(moduleRoot, "probe"), { recursive: true });
  writeFileSync(join(moduleRoot, "probe", "🌉️bridge.js"), "export {}");
  for (let index = 0; index < 3000; index += 1) {
    mkdirSync(join(sourceRoot, `m${index % 40}`), { recursive: true });
    writeFileSync(join(sourceRoot, `m${index % 40}`, `f${index}.rs`), `fn f${index}() {}`);
  }
  const receipt = (n: number) => act.nextActivationReceipt("s", "dev", [{ pluginId: "probe", artifactSha256: String(n).padStart(64, "0"), sourceContentSha256: "a".repeat(64) }], undefined, 1_000 + n);
  act.publishActivationReceipt(receiptDirectory, receipt(1));
  await new Promise<void>((resolveListen) => server.listen(0, "127.0.0.1", resolveListen));
  const plugin = semioActivationVitePlugin({ receiptDirectory, moduleRoot, installRoot: join(sandbox, "extensions"), components: [{ pluginId: "probe", directoryName: "probe", role: "plugin", sourceRoot }] });
  plugin.configureServer({ middlewares: { use: () => undefined }, httpServer: server as never, ws: { send: () => undefined } });
  let last = performance.now(), maxGap = 0;
  const ticker = setInterval(() => { const now = performance.now(); maxGap = Math.max(maxGap, now - last); last = now; }, 5);
  const t0 = performance.now();
  act.publishActivationReceipt(receiptDirectory, receipt(2));
  await new Promise((resolveWait) => setTimeout(resolveWait, 30));
  act.publishActivationReceipt(receiptDirectory, receipt(3));
  const deadline = Date.now() + 120_000;
  while (!warnings.some((line) => line.includes("[stale] probe")) && Date.now() < deadline) await new Promise((resolveWait) => setTimeout(resolveWait, 20));
  clearInterval(ticker);
  console.log(JSON.stringify({ passMs: Math.round(performance.now() - t0), maxEventLoopGapMs: Math.round(maxGap), staleLines: warnings.filter((line) => line.includes("[stale]")).length, warnings: warnings.map((line) => line.slice(0, 200)) }, null, 1));
} finally {
  console.warn = originalWarn;
  server.close();
  rmSync(sandbox, { recursive: true, force: true });
  process.exit(0);
}
