/** 🔬️ Exercises the exact retained component's idle poll source through the observed epoch boundary. */
import { registerHooks } from "node:module";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const [guestPath, vendorPath, count = "16416"] = process.argv.slice(2);
const guest = pathToFileURL(resolve(guestPath));
const vendor = resolve(vendorPath);
const vendorMarker = "/🪞️vendor/";
const hooks = registerHooks({
  resolve(specifier, context, next) {
    if (context.parentURL?.startsWith("file:") && specifier.startsWith("../🪞️vendor/")) {
      const attempted = fileURLToPath(new URL(specifier, context.parentURL));
      const path = attempted.slice(attempted.lastIndexOf(vendorMarker) + vendorMarker.length);
      return next(pathToFileURL(join(vendor, path)).href, context);
    }
    return next(specifier, context);
  },
});

globalThis.self = globalThis;
globalThis.postMessage = (message) => {
  throw new Error(`idle guest unexpectedly requested a host operation ${message.kind}`);
};

const { createActorApi } = await import(guest.href);
const admission = {
  operation: 1n,
  generation: 1n,
  grant: { maximumItems: 8388608, maximumCopyBytes: 2147483648, maximumCapacityBytes: 536870912, maximumReleaseBytes: 536870912, maximumDepth: 64 },
  fuel: 50_000_000,
  maximumElapsedUs: 3_600_000_000n,
};
let epoch = 0n;
const clock = () => BigInt(Math.floor(performance.now() * 1000));
const actor = await createActorApi("demonstrator#1", 1n, admission, clock, {
  receipt(receipt) { epoch = receipt.input.epoch; },
  hold(failure) { console.error("[DEBUG] source retained failure", failure); },
});
const { readFileSync } = await import("node:fs");
const lines = readFileSync(process.argv[4], "utf8").split("\n");
const revive = (_, value) => value && typeof value === "object" && Object.keys(value).length === 1 && "bigint" in value ? BigInt(value.bigint) : value && typeof value === "object" && Array.isArray(value.bytes) && value.length === value.bytes.length ? Uint8Array.from(value.bytes) : value;
const requests = lines.filter(line => line.includes("[DEBUG] authenticated-request ")).map(line => JSON.parse(line.split("[DEBUG] authenticated-request ")[1], revive)).filter(row => row.actorId === "demonstrator#1" && row.kind === "turn");
const budget = requests[0].budget;
for (const request of requests.filter(row => row.events?.length && row.events[0].kind !== "message")) {
 let reply = await actor.poll(request.events, undefined, undefined, budget);
 if (request.events[0].kind === "instance-open") for (let n = 0; !reply.lifecycleReceipt && n < 5000; n++) reply = await actor.poll(request.events, undefined, undefined, budget);
 console.info("[DEBUG] event", request.events[0].kind, String(epoch), !!reply.lifecycleReceipt);
}
const seen = new Set();
for (const request of requests.filter(row => row.commandPage)) {
 const page = request.commandPage, key = `${page.cursor.seq}/${page.cursor.pageIndex}`;
 if (seen.has(key)) continue;
 seen.add(key);
 let reply, attempt = 0;
 do { reply = await actor.poll([], page, undefined, budget); attempt++; } while (reply.commandIngress?.tag === "backpressure" && attempt < 10000);
 console.info("[DEBUG] page", key, attempt, String(epoch), reply.commandIngress?.tag);
 if (reply.commandIngress?.tag === "fault") throw Error("page fault");
 if (page.cursor.seq === 2n) {
   for (let n = 0; n < 10000 && reply.commandIngress?.tag !== "command-complete"; n++) reply = await actor.poll([], undefined, undefined, budget);
   console.info("[DEBUG] boot", String(epoch), reply.commandIngress?.tag);
 }
}
const { Session } = await import("node:inspector");
const session = new Session(); session.connect();
const inspect = (method, params = {}) => new Promise((resolve, reject) => session.post(method, params, (error, result) => error ? reject(error) : resolve(result)));
await inspect("Profiler.enable");
for (let turn = 0; turn < 10000; turn++) {
 if (turn === 5000) { await inspect("Profiler.setSamplingInterval", { interval: 100 }); await inspect("Profiler.start"); }
 if (turn === 9000) { const { profile } = await inspect("Profiler.stop"); const { writeFileSync } = await import("node:fs"); writeFileSync(process.argv[5], JSON.stringify(profile)); }
 const laterMessage = requests.find(row => row.events?.[0]?.kind === "message");
 const reply = await actor.poll(turn === 4500 && laterMessage ? laterMessage.events : [], undefined, undefined, budget);
 if (turn % 1024 === 0 || reply.commandIngress?.tag === "fault" || reply.commandIngress?.tag === "command-complete") console.info("[DEBUG] replay", turn, String(epoch), reply.commandIngress?.tag);
 if (["fault", "command-complete"].includes(reply.commandIngress?.tag)) break;
}
session.disconnect();
hooks.deregister();
