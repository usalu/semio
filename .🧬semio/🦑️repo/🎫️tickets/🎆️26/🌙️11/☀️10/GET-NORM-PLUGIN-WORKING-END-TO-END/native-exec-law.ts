import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";
import { Database } from "bun:sqlite";
import { newHostCapacityIdV1, withHostCapacityV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1 as policy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts";

const ticket = import.meta.dirname;
const scenarios = [
  { name: "fresh", live: true, plan: true, code: 0 },
  { name: "inherited", live: true, plan: true, code: 0 },
  { name: "undersized-browser", live: false, plan: false, code: 1 },
  { name: "stale", live: false, plan: false, code: 1 },
  { name: "native", live: true, plan: true, code: 0 },
] as const;
const mock = `
import { appendFileSync, existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const directory = process.env.SEMIO_HOST_CAPACITY_DIRECTORY;
const id = process.env.SEMIO_HOST_CAPACITY_GRANT;
const file = id && join(directory, "grants", id + ".json");
const grant = file && existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : undefined;
const live = (grant?.reservedMegabytes ?? 0) >= 2048;
const retained = grant?.retainReservation === true;
const plan = !!process.env.SEMIO_HOST_CAPACITY_PLAN;
const planMegabytes = plan ? JSON.parse(process.env.SEMIO_HOST_CAPACITY_PLAN).memoryMegabytes : null;
const { parentCompilerMemoryAllowanceBytesV1 } = await import(process.env.SEMIO_NATIVE_EXEC_LAW_ADMISSION);
const allowance = parentCompilerMemoryAllowanceBytesV1(process.env) / 1048576;
const scenario = process.env.SEMIO_NATIVE_EXEC_LAW_RETRY;
if (scenario) {
  const ledger = join(directory, "attempts.jsonl");
  const previous = existsSync(ledger) ? readFileSync(ledger, "utf8").trim().split("\\n").length : 0;
  const attempt = Number(process.env.SEMIO_STALE_REEXEC ?? "0");
  const envelope = JSON.parse(process.env.SEMIO_SCRIPT_PROCESS_INVOCATION);
  appendFileSync(ledger, JSON.stringify({attempt, pid: process.pid, grant: id, plan, allowance, retained, owner: envelope.policy.owner}) + "\\n");
  if (scenario === "ordinary") process.exit(17);
  if (scenario === "repeated" || previous === 0) {
    const { exitIfStaleImplementationV1 } = await import(process.env.SEMIO_NATIVE_EXEC_LAW_STAMP);
    const { concurrentEditV1, staleImplementationV1 } = await import(process.env.SEMIO_NATIVE_EXEC_LAW_CHURN);
    exitIfStaleImplementationV1(scenario === "repeated"
      ? staleImplementationV1("Test child yielded its loaded implementation")
      : concurrentEditV1("length", join(directory, "Cargo.toml"), "Cargo workspace Source length changed", "build"));
  }
  if (scenario === "cancelled") {
    process.once("SIGTERM", () => { writeFileSync(join(directory, "stopped"), ""); process.exit(0); });
    writeFileSync(join(directory, "started"), "");
    setTimeout(() => process.exit(2), 5000);
  } else {
    process.exit(0);
  }
}
if (!scenario) console.log(JSON.stringify({live, retained, plan, planMegabytes, allowance, grants: readdirSync(join(directory, "grants")).length}));
if (process.env.SEMIO_NATIVE_EXEC_LAW_WAIT) {
  process.once("SIGTERM", () => { writeFileSync(join(directory, "stopped"), ""); process.exit(0); });
  writeFileSync(join(directory, "started"), "");
  setTimeout(() => process.exit(2), 5000);
}
`;

function fixture() {
  const directory = mkdtempSync(join(ticket, "🗑️generated/ne-"));
  const { SEMIO_HOST_CAPACITY_GRANT: _grant, SEMIO_HOST_CAPACITY_PLAN: _plan, SEMIO_HOST_CAPACITY_HANDOVER: _handover, SEMIO_STALE_REEXEC: _retry, ...rest } = process.env;
  const env = {
    ...rest, SEMIO_HOST_CAPACITY_DIRECTORY: directory, NATIVE_EXEC_TARGET: "@semio-tech/norm-js:test", LANG: "en_US.UTF-8",
    SEMIO_NATIVE_EXEC_LAW_ADMISSION: join(ticket, "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts"),
    SEMIO_NATIVE_EXEC_LAW_STAMP: join(ticket, "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs"),
    SEMIO_NATIVE_EXEC_LAW_CHURN: join(ticket, "../../../../../../../🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts"),
  };
  const run = (script: string, extra: NodeJS.ProcessEnv = {}, inherited = false) => {
    const command = ["repo", join(ticket, "native-exec.ts"), directory, script];
    return Bun.spawn(
      inherited ? ["repo", "capacity", "run", "--memory=2048", "--cpu=1", "--", ...command] : command,
      { env: { ...env, ...extra }, stdout: "pipe", stderr: "pipe" },
    );
  };
  return { directory, env, run };
}

for (const row of scenarios) test(row.name + " native-exec admission", async () => {
  const { directory, run } = fixture();
  try {
    const native = join(directory, "📦️packages", "🦀️rust");
    mkdirSync(native, { recursive: true });
    const script = row.name === "native" ? join(native, "📜️script.ts") : join(directory, "mock.ts");
    writeFileSync(script, mock);
    const extra = row.name === "undersized-browser" ? { NATIVE_EXEC_TARGET: "@semio-tech/semio-tech-play:serve-standards" } : row.name === "stale" ? {
      SEMIO_HOST_CAPACITY_GRANT: "missing-grant",
      SEMIO_HOST_CAPACITY_PLAN: JSON.stringify({ version: 1, id: "missing-grant", jobs: 1, cpu: 1, memoryMegabytes: 2048, fixedMegabytes: 2048, floorMegabytes: 2048 }),
    } : {};
    const process = run(script, extra, row.name === "inherited" || row.name === "undersized-browser");
    const [exit, stdout, stderr] = await Promise.all([process.exited, new Response(process.stdout).text(), new Response(process.stderr).text()]);
    expect(exit, stderr).toBe(row.code);
    if (exit === 0) {
      expect(JSON.parse(stdout)).toEqual({ live: row.live, retained: true, plan: row.plan, planMegabytes: row.name === "inherited" ? 2048 : 0, allowance: 2048, grants: 1 });
      expect(readdirSync(join(directory, "grants"))).toEqual([]);
    }
    else expect(stdout).toBe("");
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 30_000);

for (const {name, exit, count} of [
  {name: "transient", exit: 0, count: 2},
  {name: "repeated", exit: 75, count: 7},
  {name: "ordinary", exit: 17, count: 1},
]) test(`native-exec owns ${name} child termination`, async () => {
  const { directory, run } = fixture();
  try {
    const script = join(directory, "mock.ts");
    writeFileSync(script, mock);
    const worker = run(script, { SEMIO_NATIVE_EXEC_LAW_RETRY: name }, name === "transient");
    const [status, stderr] = await Promise.all([worker.exited, new Response(worker.stderr).text(), new Response(worker.stdout).text()]);
    expect(status, stderr).toBe(exit);
    const attempts = readFileSync(join(directory, "attempts.jsonl"), "utf8").trim().split("\n").map(line => JSON.parse(line));
    expect(attempts.map(row => row.attempt)).toEqual(Array.from({ length: count }, (_, index) => index));
    expect(new Set(attempts.map(row => row.pid)).size).toBe(count);
    expect(new Set(attempts.map(row => row.grant)).size).toBe(1);
    expect(attempts.every(row => row.owner === "native-exec" && row.plan && row.retained && row.allowance === 2048)).toBe(true);
    expect(readdirSync(join(directory, "grants"))).toEqual([]);
    expect(stderr.match(/Restarting with the current implementation/g)?.length ?? 0).toBe(count - 1);
    if (name === "transient") expect(stderr).toContain("Waiting 250 ms");
    expect(stderr).not.toContain("(7 of 6)");
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 30_000);

test("publication and browser reentry require a sufficient retained grant", async () => {
  const { directory, env, run } = fixture(), db = new Database(":memory:");
  const publication = join(directory, "publication.ts"), browser = join(directory, "browser.ts");
  const envelope = policy.envelopes.find(row => row.name === "end-to-end")!;
  const control = { signal: AbortSignal.timeout(10000), remainingMilliseconds: () => null, publish: () => undefined, yieldContinuation: async () => undefined };
  try {
    writeFileSync(publication, mock);
    writeFileSync(browser, `
import {guardedPlay} from ${JSON.stringify(join(ticket,"guarded-play.ts"))};
await guardedPlay(import.meta.path, async () => {console.log("guarded fixture entered");});
`);
    for (const retained of [false, true]) {
      const expected = db.query("select ?1 and ?2 >= ?3 and ?4 >= ?5 as admitted").get(Number(retained), envelope.megabytes, envelope.megabytes, envelope.cpu, envelope.cpu) as {admitted:number};
      await withHostCapacityV1({ directory, environment: env, policy, control, write: () => undefined,
        request: {id:newHostCapacityIdV1(),operation:"command",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:envelope.megabytes,minimumMegabytes:envelope.megabytes,perJobMegabytes:0,cpuPerJob:envelope.cpu,...retained?{retainReservation:true}:{}},
        sample: async () => ({host:{version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:30000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10},swappedOutMegabytes:0}),
      }, async handle => {
        const commands = [run(publication,handle.environment), Bun.spawn([process.execPath,browser,"--admitted"],{env:{...env,...handle.environment},stdout:"pipe",stderr:"pipe"})];
        const outcomes = await Promise.all(commands.map(child=>Promise.all([child.exited,new Response(child.stdout).text(),new Response(child.stderr).text()])));
        for (const [exit, out, err] of outcomes) {
          expect(exit,err).toBe(expected.admitted ? 0 : 1);
          if(expected.admitted)expect(out).not.toBe("");
          else {expect(out).toBe("");expect(err).toContain("retained");}
        }
      });
      expect(readdirSync(join(directory,"grants"))).toEqual([]);
    }
  } finally {
    db.close();
    rmSync(directory,{recursive:true,force:true});
  }
}, 15_000);

for (const retry of [false, true]) test.skipIf(process.platform === "win32")("native-exec cancellation reaches the owned " + (retry ? "replacement" : "mock child"), async () => {
  const { directory, run } = fixture();
  const script = join(directory, "mock.ts");
  writeFileSync(script, mock);
  const worker = run(script, retry ? { SEMIO_NATIVE_EXEC_LAW_RETRY: "cancelled" } : { SEMIO_NATIVE_EXEC_LAW_WAIT: "1" });
  try {
    for (let i = 0; i < 250; i++) {
      if (readdirSync(directory).includes("started")) break;
      await sleep(20);
    }
    expect(readdirSync(directory)).toContain("started");
    worker.kill("SIGTERM");
    expect(await worker.exited).toBe(143);
    expect(readFileSync(join(directory, "stopped"), "utf8")).toBe("");
    if (retry) expect(readFileSync(join(directory, "attempts.jsonl"), "utf8").trim().split("\n")).toHaveLength(2);
    expect(readdirSync(join(directory, "grants"))).toEqual([]);
  } finally {
    if (worker.exitCode === null) worker.kill("SIGTERM");
    await worker.exited;
    rmSync(directory, { recursive: true, force: true });
  }
}, 30_000);
