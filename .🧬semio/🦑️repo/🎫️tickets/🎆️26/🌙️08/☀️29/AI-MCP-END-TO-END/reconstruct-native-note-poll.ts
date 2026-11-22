import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve, sep } from "node:path";
import { diagnosticPollLimitsObject, diagnosticPollLimitsTuple, type PollLimitsObject } from "./owned-input-poll-limits.ts";

const generated = join(import.meta.dir, "🗑️generated");
const config = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config";
const inputs = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/📊️actor-input.json";
const tuple = process.argv.includes("--tuple-limits");
const arguments_ = process.argv.slice(2).filter(argument => argument !== "--tuple-limits");
const [logName = "native-call-canonical-note-input-witness-sdk.log", output = "native-note-poll", capture] = arguments_;
assert(arguments_.length <= 3 && /^[a-z0-9-]+$/u.test(output));
function localInput(name: string): string {
  const path = resolve(generated, name);
  assert(path.startsWith(generated + sep), "Input must belong to the ticket's generated directory");
  return path;
}

type InstanceOpen = { appId: string; actor: string; capabilities: unknown[] };
type PollFixture = {
  valid: boolean;
  wire: { Poll?: { events: { instanceOpen?: InstanceOpen }[]; limits: PollLimitsObject | number[] } };
};

const policy = JSON.parse(readFileSync(join(config, "🏛️host-driver-policy.json"), "utf8"));
const reserves = JSON.parse(readFileSync(join(config, "💡️host-operation-policy.json"), "utf8"));
const fixtures: PollFixture[] = JSON.parse(readFileSync(inputs, "utf8"));
const wire = fixtures.find(row => {
  const open = row.wire.Poll?.events[0]?.instanceOpen;
  return row.valid && open?.appId === "s.note.note@1/*#editor" && open.capabilities.length === 1;
})?.wire;
const poll = wire?.Poll;
const open = poll?.events[0]?.instanceOpen;
assert(poll && open, "Missing synthetic Note InstanceOpen fixture");

const log = readFileSync(localInput(logName), "utf8");
const witness = log.match(/Core Input witness length=(\d+) fnv1a=([0-9a-f]+) scalar_budget=\[([^\]]+)\]/u);
const context = log.match(/context_resolve pins[^\n]*principal=(\S+) session=(\S+)/u);
assert(witness && context, "Missing the Input grant or the synthetic SDK session");
const scalar = witness[3].split(", ").map(Number);
assert(scalar.length === 19 && scalar.every(Number.isSafeInteger));
open.actor = `agent:${context[1]}#${context[2]}`;

const keys = ["maximumItems", "maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes"] as const;
const admittedGuards = [reserves.cleanup, reserves.transit, reserves.infrastructure];
const limits = diagnosticPollLimitsObject(poll.limits);
for (const key of keys) {
  const deducted = admittedGuards.reduce((sum, guard) => sum + guard.retained[key], 0);
  limits.retainedPolicy[key] = Math.min(policy.operation[key] - deducted, 0xffffffff - reserves.infrastructure.retained[key]);
}
limits.retainedPolicy.maximumDepth = policy.operation.maximumDepth;
const root = policy.root;
Object.assign(limits, {
  workFuel: root.operationFuel - admittedGuards.reduce((sum, guard) => sum + guard.workFuel, 0),
  fuel: root.turnExecutionFuel,
  deadlineUs: scalar[4] - reserves.cleanup.maximumMicroseconds,
  maxEffects: root.maxEffects,
  maxPatchBytes: root.maxPatchBytes,
  maxFrames: root.maxFrames,
});
poll.limits = tuple ? diagnosticPollLimitsTuple(limits) : limits;

const bytes = Buffer.from(JSON.stringify(wire));
const hash = bytes.reduce((hash, byte) => BigInt.asUintN(64, (hash ^ BigInt(byte)) * 0x100000001b3n), 0xcbf29ce484222325n);
const fingerprint = hash.toString(16).padStart(16, "0");
assert.equal(bytes.length, Number(witness[1]), "Reconstructed Note wire extent differs");
assert.equal(fingerprint, witness[2], "Reconstructed Note wire fingerprint differs");
if (capture) assert.deepEqual(bytes, readFileSync(localInput(capture)), "Reconstruction differs from the bounded raw test capture");
writeFileSync(join(generated, output + "-exact.json"), bytes);
writeFileSync(join(generated, output + "-scalar.json"), JSON.stringify(scalar));
console.log("[DEBUG]", { length: bytes.length, fingerprint, rawCaptureMatched: Boolean(capture), limits: poll.limits });
