/** 🩺️ Headless mirror of what `🟨️shard-worker.js` does to staged plugin modules: import each
 * generated `🌉️bridge.js`, `createActorApi(actorId, activationGeneration)`, then drive the same
 * `poll` turns `ShardClient` drives at boot — several modules in ONE process, the way one pooled
 * shard worker hosts several actors. `heartbeatTicks` is a 1 s `setInterval` counter standing in for
 * the worker's own progress ticker: a phase that reports 0 ticks is a phase in which the worker
 * provably could NOT heartbeat, i.e. one the host watchdog counts as silence.
 * Run: `node --experimental-wasm-jspi <this> <turns> <module-dir>…`. */
const [, , turnsText, ...moduleDirs] = process.argv;
if (!moduleDirs.length) throw new Error("usage: probe <turns> <staged-module-dir>…");
const turns = Number(turnsText);
const started = Date.now();
const log = (...parts) => console.log(`[probe ${String(Date.now() - started).padStart(6)}ms]`, ...parts);
process.on("unhandledRejection", (reason) => { log("UNHANDLED REJECTION", reason?.stack ?? reason); process.exit(3); });
process.on("uncaughtException", (error) => { log("UNCAUGHT", error?.stack ?? error); process.exit(4); });

const megabytes = (value) => `${(value / 1048576).toFixed(1)}MB`;
const rss = () => { const usage = process.memoryUsage(); return `rss=${megabytes(usage.rss)} heap=${megabytes(usage.heapUsed)}`; };
let ticks = 0;
const ticker = setInterval(() => { ticks += 1; }, 1000);
ticker.unref?.();
const takeTicks = () => { const taken = ticks; ticks = 0; return taken; };
const budget = { fuel: 80000000, wallMs: 200, memoryBytes: 268435456, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };

let activationGeneration = 0n;
for (const [index, moduleDir] of moduleDirs.entries()) {
  const directory = moduleDir.endsWith("/") ? moduleDir : `${moduleDir}/`;
  const name = directory.split("/").filter(Boolean).pop();
  const bridgeUrl = new URL(`file://${directory}\u{1F309}️bridge.js`);
  activationGeneration += 1n;
  const importedAt = Date.now();
  const bridge = await import(bridgeUrl.href);
  const createdAt = Date.now();
  const api = await bridge.createActorApi(`${name}#${index + 1}`, activationGeneration);
  log(`${name}: import ${createdAt - importedAt}ms createActorApi ${Date.now() - createdAt}ms heartbeatTicks=${takeTicks()}`, rss());
  for (let turn = 1; turn <= turns; turn += 1) {
    const at = Date.now();
    const result = await api.poll([], null, null, budget);
    log(`${name}: poll #${turn} ${Date.now() - at}ms heartbeatTicks=${takeTicks()}`, rss(), `effects=${result.effects?.length ?? "?"} uiPatches=${result.uiPatches?.length ?? "?"}`);
  }
}
clearInterval(ticker);
log("done", rss());
