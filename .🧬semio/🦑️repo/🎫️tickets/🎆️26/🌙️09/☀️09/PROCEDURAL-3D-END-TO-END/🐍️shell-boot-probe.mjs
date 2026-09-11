/** 🐚️ Headless mirror of the wgpu `shell-boot` phase: what `🐚️plugin-bridge.ts` actually drives against
 * one staged plugin closure when `ShellState::boot` calls `create_app` and then `render_with_document`.
 * Drives the same three legs in order — `createActorApi`, the `instance-open` lifecycle turn (with the
 * `instance-lifecycle-ack` the bridge's `settleInstanceLifecycle` sends), then a `surface-visible` turn
 * pumped with empty turns while the guest answers `more-work` — and prints one line per turn with its
 * status, effects, patches, wall cost and the heartbeat ticks a host watchdog would have observed.
 * A phase that stops advancing here is the phase the live boot is silent inside.
 * Run: `node --experimental-wasm-jspi <this> <payload.json> <turns> <moduleDir> <appId> <surfaceId> <bodyKey>`. */
const [, , payloadPath, turnsText, moduleDir, appId, surfaceId, bodyKey] = process.argv;
if (!bodyKey) throw new Error("usage: probe <payload.json> <turns> <moduleDir> <appId> <surfaceId> <bodyKey>");
const { readFileSync } = await import("node:fs");
const payload = JSON.parse(readFileSync(payloadPath, "utf8"));
const turns = Number(turnsText);
const started = Date.now();
const log = (...parts) => console.log(`[shell-boot ${String(Date.now() - started).padStart(7)}ms]`, ...parts);
process.on("unhandledRejection", (reason) => { log("UNHANDLED REJECTION", reason?.stack ?? reason); process.exit(3); });
process.on("uncaughtException", (error) => { log("UNCAUGHT", error?.stack ?? error); process.exit(4); });
const megabytes = (value) => `${(value / 1048576).toFixed(1)}MB`;
const rss = () => `rss=${megabytes(process.memoryUsage().rss)}`;
let ticks = 0;
const ticker = setInterval(() => { ticks += 1; }, 1000);
ticker.unref?.();
const takeTicks = () => { const taken = ticks; ticks = 0; return taken; };
const budget = { fuel: 80000000, wallMs: 200, memoryBytes: 268435456, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152 };
const bytes = (base64) => Array.from(Buffer.from(base64, "base64"));

const directory = moduleDir.endsWith("/") ? moduleDir : `${moduleDir}/`;
const name = directory.split("/").filter(Boolean).pop();
const bridge = await import(new URL(`file://${directory}🌉️bridge.js`).href);
const activationGeneration = 1n;
const instance = 1;
const api = await bridge.createActorApi(`${name}#${instance}`, activationGeneration);
log(`${name}: createActorApi done heartbeatTicks=${takeTicks()}`, rss());

const describe = (result) => {
  const status = typeof result.status === "string" ? result.status : result.status?.tag ?? JSON.stringify(result.status);
  return `status=${status} effects=${result.effects?.length ?? "?"} uiPatches=${result.uiPatches?.length ?? "?"} receipt=${result.lifecycleReceipt ? "yes" : "no"} ingress=${result.commandIngress?.tag ?? "?"} nextWake=${result.nextWake ?? "-"}`;
};

const drive = async (label, events) => {
  const at = Date.now();
  const result = await api.poll(events, null, null, budget);
  log(`${label} ${Date.now() - at}ms heartbeatTicks=${takeTicks()} ${describe(result)}`, rss());
  return result;
};

/** 🔓️ Decodes the `captured` receipt `🌉️bridge.js` hands back as LEB128 bytes — tag, activation
 * generation, instance, guest lifetime, request sequence — so the ack turn can be rebuilt without
 * `ShardClient`. */
const decodeReceipt = (encoded) => {
  let index = 0;
  const take = () => { let shift = 0n, value = 0n, byte; do { byte = encoded[index++]; value |= BigInt(byte & 127) << shift; shift += 7n; } while (byte & 128); return value; };
  const tag = encoded[index++];
  const activation = take(), instanceId = Number(take()), guestLifetime = take(), requestSequence = Number(take());
  return { tag, lifetime: { activationGeneration: activation, instanceId, guestLifetime }, requestSequence };
};

let opened = await drive("open#1", [{ kind: "instance-open", payload: { instance, activationGeneration, requestSequence: 1, appId, actor: "local", config: [], assets: [], capabilities: [], quotas: bytes(payload.quotas) } }]);
for (let turn = 2; turn <= turns && !opened.lifecycleReceipt; turn += 1) opened = await drive(`open#${turn}`, []);
if (!opened.lifecycleReceipt) { log("STALL: instance-open produced no lifecycle receipt"); process.exit(1); }
const captured = decodeReceipt(opened.lifecycleReceipt);
log(`captured lifecycle receipt tag=${captured.tag} guestLifetime=${captured.lifetime.guestLifetime} seq=${captured.requestSequence}`);
let acked = await drive("ack#1", [{ kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt: { kind: "captured", lifetime: captured.lifetime, requestSequence: captured.requestSequence } } }]);
for (let turn = 2; turn <= turns; turn += 1) {
  const status = typeof acked.status === "string" ? acked.status : acked.status?.tag ?? "";
  if (status.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase() !== "more-work") break;
  acked = await drive(`ack#${turn}`, []);
}

let current = await drive("render#1", [{ kind: "surface-visible", payload: { surface: { instance, surface: surfaceId }, bodyKey, viewState: bytes(payload[process.env.SEMIO_PROBE_VIEW_STATE ?? "viewStateRecord"]) } }]);
let published = current.uiPatches?.length ? 1 : 0;
for (let turn = 2; turn <= turns; turn += 1) {
  const status = typeof current.status === "string" ? current.status : current.status?.tag ?? "";
  if (status.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase() !== "more-work" && published) break;
  current = await drive(`render#${turn}`, []);
  published += current.uiPatches?.length ? 1 : 0;
}
log(`done patches=${published}`, rss());
clearInterval(ticker);
