import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";
import Ajv2020 from "ajv/dist/2020";
import cases from "./constrained.cases.json";
import schema from "./constrained.json";
import readySchema from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json";
import { withConstrainedCapacity, runWhenDiskAdmitted, requireConstrainedDisk, constrainedDisk, constrainedLabArguments, constrainedMount, constrainedOptions, constrainedPaths, exportReceipts } from "./constrained.ts";
import { newHostCapacityIdV1, withHostCapacityV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import { readHostCapacityPolicyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import { BUILD_HOST_CAPACITY_POLICY_V1 as hostPolicy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🎛️control/🟦️.ts";

test("closed lab options agree with independent JSON Schema validation", () => {
  const validate = new Ajv2020({ strict: false }).compile(schema);
  for (const example of cases.arguments) {
    if (example.value) {
      expect(validate(example.value)).toBe(true);
      expect(constrainedOptions(example.argv)).toEqual(example.value);
    } else {
      expect(() => constrainedOptions(example.argv)).toThrow();
    }
  }
  expect(validate({ mode: "run", imageGiB: 40, timeoutSeconds: 30 })).toBe(false);
  expect(validate({ mode: "run", imageGiB: 44, timeoutSeconds: 30, skip: "tools" })).toBe(false);
});

test("41 through 48 GiB choices respect physical growth and the host reserve", () => {
  const gib = 1024 ** 3;
  for (const example of cases.diskGiB) {
    const disk = constrainedDisk(example.image, example.allocated * gib, example.reserve * gib, example.free * gib);
    expect(disk.admitted).toBe(example.admitted);
    if(example.admitted)expect(()=>requireConstrainedDisk(disk)).not.toThrow();
    else expect(()=>requireConstrainedDisk(disk)).toThrow(JSON.stringify(disk));
    expect(disk.requiredFreeBytes).toBe((example.reserve + 1 + Math.max(0, example.image + 2 - example.allocated)) * gib);
  }
});

test("frozen paths reserve the actual prefix and count surrogate pairs twice", () => {
  const name = (units: number) => "a/" + "😀".repeat(Math.trunc((units - 2) / 2)) + "a".repeat(units % 2);
  for (const row of cases.pathUnits) {
    const tracked = name(row.tracked);
    expect(Buffer.byteLength(tracked, "utf16le") / 2).toBe(row.tracked);
    const paths = constrainedPaths([tracked, ".🧬semio/🦑️repo/🎫️tickets/" + "a".repeat(255)], row.untracked ? [name(row.untracked)] : []);
    expect(paths).toEqual({ trackedCount: 1, maximumTrackedUnits: row.tracked, maximumUntrackedUnits: row.untracked, maximumCloneUnits: Math.max(row.tracked, row.untracked) + 39, admitted: row.admitted });
  }
  expect(() => constrainedPaths(["a/../b"], [])).toThrow();
});

test("lab launch has its original isolated stage flags and short path", () => {
  const value = constrainedOptions(["run", "--image-gib=44"]);
  expect(constrainedLabArguments("/private/tmp/lv-123abc", value)).toEqual([
    "run", "--machines=1", "--memory=8192", "--cores=4", "--stages=all", "--frozen", "--timeout=10800",
    "--work=/private/tmp/lv-123abc", "--out=/private/tmp/lv-123abc/report", "--keep",
  ]);
  expect(constrainedLabArguments("/private/tmp/lv-123abc", constrainedOptions(["run", "--timeout=14400"]))).toContain("--timeout=14400");
  expect(() => constrainedLabArguments("/private/tmp/" + "a".repeat(50), value)).toThrow();
});

test("detach requires one exact image and mount association", () => {
  const backing = "/ticket/generated/l-123/lab.sparseimage";
  const mount = "/private/tmp/lv-123abc";
  const image = (path: string, where: string | undefined, device = "/dev/disk42s1") => ({
    "image-path": path, "system-entities": [{ "dev-entry": "/dev/disk42" }, { "dev-entry": device, ...(where ? { "mount-point": where } : {}) }],
  });
  const own = image(backing, mount);
  expect(constrainedMount({ images: [own, image("/elsewhere/other.dmg", "/Volumes/Other")] }, backing, mount)).toBe("/dev/disk42s1");
  expect(constrainedMount({ images: [image("/elsewhere/other.dmg", "/Volumes/Other")] }, backing, mount)).toBeUndefined();
  expect(() => constrainedMount({ images: [image(backing, undefined)] }, backing, mount)).toThrow();
  expect(() => constrainedMount({ images: [image(backing, "/Volumes/Other")] }, backing, mount)).toThrow();
  expect(() => constrainedMount({ images: [image("/elsewhere/other.dmg", mount)] }, backing, mount)).toThrow();
  expect(() => constrainedMount({ images: [own, image("/elsewhere/other.dmg", mount)] }, backing, mount)).toThrow();
  expect(() => constrainedMount({ images: [own, own] }, backing, mount)).toThrow();
  expect(() => constrainedMount({ images: [image(backing, mount, "/tmp/foreign")] }, backing, mount)).toThrow();
  expect(() => constrainedMount({ images: [{ "image-path": backing }] }, backing, mount)).toThrow();
});

test("retains the exact private machine receipt and refuses a substituted file", () => {
  const generated = join(import.meta.dir, "🗑️generated");
  mkdirSync(generated, { recursive: true, mode: 0o700 });
  const owned = realpathSync(mkdtempSync(join(generated, "lt-")));
  try {
    const mount = join(owned, "test-volume"), destination = join(owned, "retained");
    const machineDirectory = join(mount, "slw-123abc", "m1");
    mkdirSync(machineDirectory, { recursive: true });
    mkdirSync(destination);
    const source = join(machineDirectory, "result.json");
    const exported = join(destination, "m1.json");
    const bytes = Buffer.from('{\n "ready": false, "failure": {"stage":"tools","code":"wall","exit":1,"tail":[]}, "timings":[]\n}\n');
    writeFileSync(source, bytes);
    const device = lstatSync(mount, { bigint: true }).dev;
    expect(exportReceipts(mount, destination, device)).toEqual({ machine: true, verdict: false, timings: false });
    expect(readFileSync(exported)).toEqual(bytes);
    expect(() => exportReceipts(mount, destination, device)).toThrow();
    expect(() => exportReceipts(mount, destination, device + 1n)).toThrow();
    rmSync(source);
    symlinkSync(exported, source);
    expect(() => exportReceipts(mount, destination, device)).toThrow();
    rmSync(source);
    writeFileSync(source, Buffer.alloc(schema["x-policy"].maximumReceiptBytes + 1));
    expect(() => exportReceipts(mount, destination, device)).toThrow();
    expect(readFileSync(exported)).toEqual(bytes);
  } finally {
    rmSync(owned, { recursive: true, force: true });
  }
});

test("an interrupted lab retains the exact stage timing without claiming a terminal verdict", () => {
  const generated = join(import.meta.dir, "🗑️generated");
  mkdirSync(generated, { recursive: true, mode: 0o700 });
  const owned = realpathSync(mkdtempSync(join(generated, "lt-")));
  try {
    const mount = join(owned, "volume"), destination = join(owned, "export");
    const state = join(mount, "slw-123abc", "m1", "s", "f", "ready");
    mkdirSync(state, { recursive: true });
    mkdirSync(destination);
    const bytes = Buffer.from(JSON.stringify({
      version: 1, origin: "slice-lab-1b0f-m1", updatedMs: 14000,
      runs: [{ stage: "native", startedMs: 1000, endedMs: 14000, seconds: 13, exit: 0, outcome: "ok", peakMegabytes: 192, growthMegabytes: 112, attempts: 1, digest: "0083cde0ef5d", endEvidence: "exit-receipt" }],
    }) + "\n");
    const validator = new Ajv2020({ strict: false });
    expect(validator.validate(readySchema.definitions.Timings, JSON.parse(bytes.toString()))).toBe(true);
    const source = join(state, "ready-timings.json");
    writeFileSync(source, bytes);
    const device = lstatSync(mount, { bigint: true }).dev;
    const receipts = exportReceipts(mount, destination, device);
    expect(validator.validate(schema.$defs.exportedReceipts, receipts)).toBe(true);
    expect(receipts).toEqual({ machine: false, verdict: false, timings: true });
    const retained = join(destination, "m1-timings.json");
    expect(readFileSync(retained)).toEqual(bytes);
    rmSync(source);
    symlinkSync(retained, source);
    expect(() => exportReceipts(mount, destination, device)).toThrow();
    rmSync(source);
    writeFileSync(source, JSON.stringify({ version: 1, origin: "slice-lab-1b0f-m1", updatedMs: 1, runs: [{}] }));
    expect(() => exportReceipts(mount, destination, device)).toThrow();
    expect(readFileSync(retained)).toEqual(bytes);
  } finally {
    rmSync(owned, { recursive: true, force: true });
  }
});

test("the machine deadline starts inside the scheduler grant", () => {
  const source = ts.createSourceFile("constrained.ts", readFileSync(join(import.meta.dir, "constrained.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const run = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "runConstrained");
  expect(run).toBeDefined();
  const scopes: string[] = [];
  function visit(node: ts.Node, scope: string) {
    if (ts.isCallExpression(node)) {
      if (node.expression.getText(source) === "AbortSignal.timeout" && node.arguments[0]?.getText(source) === "maximumMilliseconds") scopes.push(scope);
      if (node.expression.getText(source) === "withConstrainedCapacity") {
        for (const arg of node.arguments) visit(arg, ts.isArrowFunction(arg) ? "granted" : scope);
        return;
      }
    }
    ts.forEachChild(node, child => visit(child, scope));
  }
  visit(run!, "queued");
  expect(scopes).toEqual(cases.runDeadlineScopes);
});

test("disk admission retries outside the memory grant and observes cancellation", async () => {
  const events: string[] = [];
  const fixture = cases.diskAdmission;
  const disk = [...fixture.physical], grants = [...fixture.underGrant];
  const stop = new AbortController(), reported: unknown[] = [];
  const validate = new Ajv2020({ strict: true }).compile(schema.$defs.diskWait);
  await runWhenDiskAdmitted(stop.signal, () => { events.push("disk"); return disk.shift()!; },
    async () => { events.push("grant", "release"); return grants.shift() ?? null; },
    async () => { events.push("wait"); },
    row => { expect(validate(row)).toBe(true); events.push("progress"); reported.push(row); });
  expect(reported).toEqual(fixture.progress);
  expect(events).toEqual(fixture.events);
  let reports = 0;
  await expect(runWhenDiskAdmitted(stop.signal, () => fixture.physical[0]!,
    async () => { throw Error("Unexpected grant"); },
    async () => { stop.abort(Error("Cancelled disk wait")); },
    () => { reports++; })).rejects.toThrow("Cancelled disk wait");
  expect(reports).toBe(1);
});

test("queued disk admission corpus satisfies its neutral schema", () => {
  expect(new Ajv2020({ strict: true }).validate(schema.$defs.queueCases, cases.queueAdmission)).toBe(true);
});

for (const fixture of cases.queueAdmission) test(fixture.id + " in the private capacity ledger", async () => {
  const generated = join(import.meta.dir, "🗑️generated");
  mkdirSync(generated, { recursive: true, mode: 0o700 });
  const directory = mkdtempSync(join(generated, "cq-"));
  const parent = new AbortController(), competitorStop = new AbortController();
  const limit = setTimeout(() => parent.abort(Error("Fixture did not settle before its deadline")), 2300);
  const admitted = Promise.withResolvers<void>(), release = Promise.withResolvers<void>(), resume = Promise.withResolvers<void>();
  const firstWait = Promise.withResolvers<void>(), withdrawal = Promise.withResolvers<unknown>();
  const failure = Error("private " + fixture.action);
  const request = (id: string) => ({ id, operation: "command", priority: "batch", minimumJobs: 1, preferredJobs: 1, fixedMegabytes: 8192, perJobMegabytes: 0, cpuPerJob: 4 } as const);
  const capacity = readHostCapacityPolicyV1({ ...hostPolicy, wait: { ...hostPolicy.wait, pollMilliseconds: 20, progressMilliseconds: 50, sampleMaximumAgeMilliseconds: 0 } });
  const sample = async () => ({ host: { version: 1 as const, atMs: Date.now(), platform: "darwin" as const, cores: 8, totalMegabytes: 16384, availableMegabytes: 14500,
    pressure: "normal" as const, swapUsedMegabytes: 0, swapOutMegabytesPerSecond: 0, load: 1, openFiles: 100, openFilesMaximum: 10000, attachedImages: 0 }, swappedOutMegabytes: 0 });
  const options = { directory, policy: capacity, environment: {}, sample, observe: async () => 100, write: () => undefined };
  const resource = (signal: AbortSignal, publish = () => undefined) => ({ signal, remainingMilliseconds: () => null, publish, yieldContinuation: async () => undefined });
  const competitorId = newHostCapacityIdV1(), labId = newHostCapacityIdV1();
  const competitor = withHostCapacityV1({ ...options, request: request(competitorId), control: resource(competitorStop.signal) }, async () => { admitted.resolve(); await release.promise; });
  const rows = (kind: string) => readdirSync(join(directory, kind)).filter(name => name.endsWith(".json"));
  let changed = false, handedOff = false, free = fixture.initialFree, checked = 0, launched = 0, reports = 0;
  const disk = () => {
    if (changed && fixture.action === "identity-error") throw failure;
    if (changed && fixture.action === "cancel") parent.abort(failure);
    return { admitted: free >= fixture.required, freeBytes: free, requiredFreeBytes: fixture.required };
  };
  let lab: Promise<{kind: string; error?: unknown}> | undefined;
  try {
    using oracle = new Database(":memory:");
    const expected = oracle.query(`SELECT CASE WHEN ?1 IN ('cancel','handoff-cancel') THEN 'cancel' WHEN ?1='identity-error' THEN 'identity-error'
      WHEN ?1='handoff-loss' AND ?2<?3 THEN 'after-grant'
      WHEN ?2<?3 THEN 'retry' ELSE 'grant' END AS verdict, (14500-2048)>=8192 AND (14500-2048)<16384 AS competing`).get(fixture.action, fixture.changedFree, fixture.required) as {verdict: string; competing: number};
    expect(expected).toEqual({ verdict: fixture.verdict, competing: 1 });
    await Promise.race([admitted.promise, competitor]);
    expect(rows("grants")).toEqual([competitorId + ".json"]);
    lab = runWhenDiskAdmitted(parent.signal, disk, () => withConstrainedCapacity({
      ...options, request: request(labId), control: resource(parent.signal, () => { firstWait.resolve(); return undefined; }),
      write: () => {
        if (!handedOff && fixture.action.startsWith("handoff-") && rows("grants").includes(labId + ".json")) {
          handedOff = true;
          free = fixture.changedFree;
          if (fixture.action === "handoff-cancel") parent.abort(failure);
        }
      },
    }, disk, async grant => {
      expect(grant).toMatchObject({ id: labId, inherited: false, adopted: false, reservedMegabytes: 8192, cpu: 4 });
      checked++;
      const current = disk();
      if (!current.admitted) return current;
      launched++;
      return null;
    }), () => resume.promise, row => { reports++; withdrawal.resolve(row); }).then(() => ({ kind: "grant" }), error => ({ kind: "error", error }));
    expect(await Promise.race([firstWait.promise.then(() => "queued"), lab.then(() => "ended")])).toBe("queued");
    expect(rows("queue")).toHaveLength(1);
    expect(launched).toBe(0);
    if (!fixture.action.startsWith("handoff-")) free = fixture.changedFree;
    else release.resolve();
    changed = true;
    const retry = expected.verdict === "retry" || expected.verdict === "after-grant";
    if (retry) {
      const queued = expected.verdict === "retry";
      const report = await Promise.race([withdrawal.promise, lab]);
      expect(report).toEqual({ stage: queued ? "while-queued" : "after-grant", freeBytes: fixture.changedFree, requiredFreeBytes: fixture.required });
      expect(new Ajv2020({ strict: true }).validate(schema.$defs.diskWait, report)).toBe(true);
      expect(rows("queue")).toEqual([]);
      expect(rows("grants")).toEqual(queued ? [competitorId + ".json"] : []);
      expect(checked).toBe(queued ? 0 : 1);
      expect(launched).toBe(0);
      free = fixture.initialFree;
      resume.resolve();
    }
    if (expected.verdict === "grant" || retry) {
      release.resolve();
      expect(await lab).toEqual({ kind: "grant" });
      expect(checked).toBe(expected.verdict === "after-grant" ? 2 : 1);
      expect(launched).toBe(1);
    } else {
      const result = await lab;
      expect(result.kind).toBe("error");
      expect(result.error).toBe(failure);
      expect(checked).toBe(0);
      expect(rows("grants")).toEqual(handedOff ? [] : [competitorId + ".json"]);
    }
    expect(reports).toBe(retry ? 1 : 0);
    expect(rows("queue")).toEqual([]);
    release.resolve();
    await competitor;
    expect(rows("grants")).toEqual([]);
  } finally {
    clearTimeout(limit);
    parent.abort(); competitorStop.abort(); release.resolve(); resume.resolve();
    await Promise.allSettled([competitor, lab]);
    rmSync(directory, { recursive: true, force: true });
  }
}, 6000);
