import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, readFileSync, writeFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";

const ticket = import.meta.dir;
const generated = join(ticket, "🗑️generated");
const [componentArgument, wireArgument, budgetArgument] = process.argv.slice(2);
const replay = join(generated, "cold-job-jit.ts");
let source = readFileSync(join(ticket, "owned-input-jit.ts"), "utf8");

function replace(before: string, after: string): void {
  assert.equal(source.split(before).length, 2, "Shared owned-input harness changed around " + before.slice(0, 90));
  source = source.replace(before, after);
}

replace('from "./owned-input-poll-limits.ts";', 'from "../owned-input-poll-limits.ts";');
replace('  const exact = live ? JSON.parse(readFileSync(liveBudget, "utf8")).map(BigInt) as bigint[] : undefined;',
  `  const suppliedBudget = process.env.SEMIO_COLD_JOB_BUDGET;
  const exact = suppliedBudget ? JSON.parse(readFileSync(suppliedBudget, "utf8")).map(BigInt) as bigint[] : undefined;`);
const start = source.indexOf("  const fixtures = JSON.parse(");
const end = source.indexOf("\n  let epoch = 0n;", start);
assert(start > 0 && end > start);
replace(source.slice(start, end), `  const wirePath = process.env.SEMIO_COLD_JOB_WIRE;
  assert(wirePath);
  const wire = readFileSync(wirePath);
  assert(wire.length >= 40 && wire.subarray(0, 4).toString("ascii") === "SJS1");
  const flags = wire[4];
  const counts = [16, 24, 32].map(offset => wire.readBigUInt64LE(offset));
  assert(flags <= 3 && wire.subarray(5, 8).every(byte => byte === 0));
  assert((flags & 1) !== 0 || counts[1] === 0n);
  assert((flags & 2) !== 0 || counts[2] === 0n);
  assert.equal(counts.reduce((total, count) => total + count, 40n), BigInt(wire.length));
  log({ stage: "cold-job-wire", wireSha256: createHash("sha256").update(wire).digest("hex"),
    job: wire.readBigUInt64LE(8).toString(), suppliedBudget: !!exact, flags, lengths: counts.map(Number) });`);
replace('    const ready = (flags & 1) !== 0;', `    const outputClose = name === "close_actor_output";
    const contextClose = name === "close_actor_context";
    const ready = contextClose ? kind === 6 : (flags & 1) !== 0;`);
replace(' : kind === 1 && flags <= 1, "Unexpected receipt kind/flags "',
  ' : outputClose ? kind === 2 && flags <= 2 : contextClose ? (kind === 2 && flags <= 2) || (kind === 6 && flags === 0) : kind === 1 && flags <= 1, "Unexpected receipt kind/flags "');
replace('  assert(inputReady && inputReturnCount === 1, "Input did not reach ready and return its wire");', `  assert(inputReady && inputReturnCount === 1, "Input did not reach ready and return its wire");
  let outputClosed = false;
  for (let attempt = 0; attempt < 64 && !outputClosed; attempt++) outputClosed = readReceipt("close_actor_output");
  assert(outputClosed, "The retained typed input did not close");
  let quiescent = false;
  for (let attempt = 0; attempt < 64 && !quiescent; attempt++) quiescent = readReceipt("close_actor_context");
  assert(quiescent, "The actual guest context did not reach Core quiescence");`);
writeFileSync(replay, source);

if (componentArgument === "--prepare-only") {
  console.log("[DEBUG] Prepared Cold Job replay source", replay);
} else {
  assert(componentArgument && wireArgument, "Pass a rebuilt WFC component, its exact SJS1 donor, and optionally the nineteen original loan words");
  const component = resolve(componentArgument);
  const wire = resolve(wireArgument);
  const budget = budgetArgument === undefined ? undefined : resolve(budgetArgument);
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(component)) hash.update(chunk);
  const digest = hash.digest("hex");
  const label = basename(wire).replace(/[^a-zA-Z0-9_-]/gu, "-");
  const transcript = join(generated, `cold-job-${label}-${digest.slice(0, 12)}.trace`);
  const environment: NodeJS.ProcessEnv = { ...process.env, SEMIO_COLD_JOB_WIRE: wire, SEMIO_OWNED_INPUT_TRANSCRIPT: transcript };
  if (budget) environment.SEMIO_COLD_JOB_BUDGET = budget;
  else delete environment.SEMIO_COLD_JOB_BUDGET;
  const child = Bun.spawn(["repo", "tools", "exec", "bun", "--", replay, component, digest, "wfc"], {
    env: environment, stdout: "inherit", stderr: "inherit",
  });
  assert.equal(await child.exited, 0);
  console.log("[DEBUG] Cold Job replay transcript", transcript);
}
