/** 🛰️ Activates one closed browser actor under the real WASI/host ports and invokes `reactor.poll` once. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const [transpiledDir, outDir] = process.argv.slice(2);
if (!transpiledDir || !outDir) throw new Error("usage: jc1-poll-lift-probe <transpiled dir with browser-actor.js + cores> <outDir>");

const importInterfaces = [
  "semio:framework/pure@1.0.0",
  "wasi:cli/environment@0.2.0", "wasi:cli/exit@0.2.0", "wasi:cli/stderr@0.2.0", "wasi:cli/stdin@0.2.0", "wasi:cli/stdout@0.2.0",
  "wasi:cli/terminal-input@0.2.0", "wasi:cli/terminal-output@0.2.0", "wasi:cli/terminal-stderr@0.2.0", "wasi:cli/terminal-stdin@0.2.0", "wasi:cli/terminal-stdout@0.2.0",
  "wasi:clocks/monotonic-clock@0.2.0", "wasi:clocks/wall-clock@0.2.0", "wasi:io/error@0.2.0", "wasi:io/poll@0.2.0", "wasi:io/streams@0.2.0",
  "wasi:random/insecure-seed@0.2.9",
];

const repoRoot = "/Users/ueli/Documents/semio";
let bundlePath: string, bundleBytes: number;
if (transpiledDir.endsWith(".mjs")) {
  bundlePath = transpiledDir;
  bundleBytes = readFileSync(bundlePath).byteLength;
} else {
  const { closedBrowserActorBundle } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts"));
  const source = readFileSync(join(transpiledDir, "browser-actor.js"), "utf8");
  const cores = ["browser-actor.core.wasm", "browser-actor.core2.wasm", "browser-actor.core3.wasm"]
    .map(name => ({ name, bytes: new Uint8Array(readFileSync(join(transpiledDir, name))) }));
  mkdirSync(outDir, { recursive: true });
  bundlePath = join(outDir, "closed-actor.mjs");
  const closed = await closedBrowserActorBundle(source, cores, { importInterfaces });
  writeFileSync(bundlePath, closed, { mode: 0o600 });
  bundleBytes = closed.length;
}

const denied = (): never => { throw new Error("jc1 probe: host effect denied"); };
const stdio: string[] = [];
const wasi = Object.freeze({
  nowNs: () => BigInt(Math.floor(performance.now() * 1_000_000)),
  wallNs: () => BigInt(Date.now()) * 1_000_000n,
  write: (channel: "stdout" | "stderr", bytes: Uint8Array) => { if (bytes.byteLength && stdio.length < 64) stdio.push(channel + ": " + new TextDecoder().decode(bytes).slice(0, 400)); },
  exit: () => { throw new Error("jc1 probe: guest exit"); },
});
const port = Object.freeze({ dispatch: denied, cancelEffect: () => "closed" as const, log: denied, traceSpan: denied, nowMs: () => BigInt(Date.now()), wasi });

const module = await import(bundlePath);
const actor = await module.activate({ actorId: "jc1-poll-lift-probe", activationGeneration: 1n }, port, {});
const report: Record<string, unknown> = { bundlePath, bundleBytes, activated: true };
const attempt = async (name: string, path: string[], args: unknown[]) => {
  const started = Date.now();
  try {
    const value = await actor.invoke(path, args);
    report[name] = { ok: true, ms: Date.now() - started, shape: describe(value) };
  } catch (error) {
    report[name] = { ok: false, ms: Date.now() - started, error: String((error as Error)?.message ?? error), stack: ((error as Error)?.stack ?? "").split("\n").slice(0, 6) };
  }
};
function describe(value: unknown, depth = 0): unknown {
  if (value instanceof Uint8Array) return `Uint8Array(${value.byteLength})`;
  if (Array.isArray(value)) return depth >= 3 ? `Array(${value.length})` : { array: value.length, first: value.length ? describe(value[0], depth + 1) : null };
  if (typeof value === "bigint") return `bigint ${value}`;
  if (value && typeof value === "object") return depth >= 3 ? "{…}" : Object.fromEntries(Object.entries(value).slice(0, 12).map(([key, inner]) => [key, describe(inner, depth + 1)]));
  return value;
}

await attempt("describe", ["describe", "describe"], []);
await attempt("poll", ["reactor", "poll"], [[], { fuel: 1_000_000n, deadlineMs: 5_000, maxEffects: 16, maxPatchBytes: 65_536, maxFrames: 16 }]);
await actor.close();
report.stdio = stdio.slice(0, 8);
console.log(JSON.stringify(report, null, 2));
