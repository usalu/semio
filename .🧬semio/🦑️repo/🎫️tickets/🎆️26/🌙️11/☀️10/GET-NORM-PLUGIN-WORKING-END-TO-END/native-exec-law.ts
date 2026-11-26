import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";

const ticket = import.meta.dirname;
const scenarios = [
  { name: "fresh", live: true, plan: true, code: 0 },
  { name: "inherited", live: true, plan: true, code: 0 },
  { name: "undersized-browser", live: false, plan: false, code: 1 },
  { name: "stale", live: false, plan: false, code: 1 },
  { name: "native", live: true, plan: true, code: 0 },
] as const;
const mock = `
import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const directory = process.env.SEMIO_HOST_CAPACITY_DIRECTORY;
const id = process.env.SEMIO_HOST_CAPACITY_GRANT;
const file = id && join(directory, "grants", id + ".json");
const live = !!file && existsSync(file) && JSON.parse(readFileSync(file, "utf8")).reservedMegabytes >= 2048;
const plan = !!process.env.SEMIO_HOST_CAPACITY_PLAN;
const planMegabytes = plan ? JSON.parse(process.env.SEMIO_HOST_CAPACITY_PLAN).memoryMegabytes : null;
const { parentCompilerMemoryAllowanceBytesV1 } = await import(process.env.SEMIO_NATIVE_EXEC_LAW_ADMISSION);
const allowance = parentCompilerMemoryAllowanceBytesV1(process.env) / 1048576;
console.log(JSON.stringify({live, plan, planMegabytes, allowance, grants: readdirSync(join(directory, "grants")).length}));
if (process.env.SEMIO_NATIVE_EXEC_LAW_WAIT) {
  process.once("SIGTERM", () => { writeFileSync(join(directory, "stopped"), ""); process.exit(0); });
  writeFileSync(join(directory, "started"), "");
  setTimeout(() => process.exit(2), 5000);
}
`;

function fixture() {
  const directory = mkdtempSync(join(ticket, "🗑️generated/ne-"));
  const { SEMIO_HOST_CAPACITY_GRANT: _grant, SEMIO_HOST_CAPACITY_PLAN: _plan, SEMIO_HOST_CAPACITY_HANDOVER: _handover, ...rest } = process.env;
  const env = { ...rest, SEMIO_HOST_CAPACITY_DIRECTORY: directory, NATIVE_EXEC_TARGET: "@semio-tech/norm-js:test", LANG: "en_US.UTF-8", SEMIO_NATIVE_EXEC_LAW_ADMISSION: join(ticket, "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts") };
  const run = (script: string, extra: NodeJS.ProcessEnv = {}, inherited = false) => {
    const command = ["repo", join(ticket, "native-exec.ts"), directory, script];
    return Bun.spawn(
      inherited ? ["repo", "capacity", "run", "--memory=2048", "--cpu=1", "--", ...command] : command,
      { env: { ...env, ...extra }, stdout: "pipe", stderr: "pipe" },
    );
  };
  return { directory, run };
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
      expect(JSON.parse(stdout)).toEqual({ live: row.live, plan: row.plan, planMegabytes: row.name === "inherited" ? 2048 : 0, allowance: 2048, grants: 1 });
      expect(readdirSync(join(directory, "grants"))).toEqual([]);
    }
    else expect(stdout).toBe("");
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 30_000);

test.skipIf(process.platform === "win32")("native-exec cancellation reaches the owned mock child", async () => {
  const { directory, run } = fixture();
  const script = join(directory, "mock.ts");
  writeFileSync(script, mock);
  const worker = run(script, { SEMIO_NATIVE_EXEC_LAW_WAIT: "1" });
  try {
    for (let i = 0; i < 100; i++) {
      if (readdirSync(directory).includes("started")) break;
      await sleep(20);
    }
    expect(readdirSync(directory)).toContain("started");
    worker.kill("SIGTERM");
    expect(await worker.exited).toBe(143);
    expect(readFileSync(join(directory, "stopped"), "utf8")).toBe("");
    expect(readdirSync(join(directory, "grants"))).toEqual([]);
  } finally {
    if (worker.exitCode === null) worker.kill("SIGTERM");
    await worker.exited;
    rmSync(directory, { recursive: true, force: true });
  }
}, 30_000);
