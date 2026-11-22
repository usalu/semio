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
  maximumElapsedUs: 300_000_000n,
};
let epoch = 0n;
const clock = () => BigInt(Math.floor(performance.now() * 1000));
const actor = await createActorApi("procedural#1", 1n, admission, clock, {
  receipt(receipt) { epoch = receipt.input.epoch; },
  hold(failure) { console.error("[DEBUG] idle poll source retained failure", failure); },
});
const total = Number(count);
const budget = { fuel: 50_000_000, maxEffects: 64, maxPatchBytes: 1 << 20, maxFrames: 8 };
for (let turn = 0; turn < total; turn += 1) {
  const key = setTimeout(() => {
    console.error("[DEBUG] idle poll unanswered", { turn, epoch });
    process.exit(2);
  }, 5000);
  try {
    const reply = await actor.poll([], undefined, undefined, budget);
    if (turn % 1024 === 0 || epoch >= 32760n && epoch <= 32772n) {
      console.info("[DEBUG] idle poll answered", { turn, epoch, status: reply.status?.tag });
    }
  } finally {
    clearTimeout(key);
  }
}
console.info("[DEBUG] idle poll completed", { total, epoch, remaining: actor.originalSource.retainedRemaining });
hooks.deregister();
