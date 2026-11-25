import { expect, test } from "bun:test";
import { once } from "node:events";
import { join } from "node:path";
import { play, runSweep, startSweepCommand, type PlayPlan } from "./guarded-play.ts";
import { companionPlan } from "./probe-norm-base-companions.ts";
import { din4108Plan } from "./verify-din4108-scoped-play.ts";
import { standardsPlan } from "./verify-standards-publication.ts";
import fixture from "./sweep-capacity-fixture.json";

for (const row of fixture.cases) test(`scope progression after ${row.outcome}`, async () => {
  const controller = new AbortController();
  const started: string[] = [];
  const pass = row.outcome === "pass";
  const work = runSweep(fixture.scopes, controller.signal, async scope => {
    started.push(scope);
    const source = row.outcome === "cancel" ? "import time; print('ready', flush=True); time.sleep(60)" :
      row.outcome === "child-exit" ? "raise SystemExit(137)" : "pass";
    const task = startSweepCommand(["repo", "tools", "exec", "--light", "python", "--", "-c", source],
      controller.signal, { stdio: ["ignore", "pipe", "ignore"], windowsHide: true });
    try {
      if (row.outcome === "cancel") {
        await once(task.child.stdout!, "data", { signal: AbortSignal.timeout(5000) });
        controller.abort(Error("mock resource cancellation"));
      }
      const code = await task.exit;
      if (!pass) expect(code).not.toBe(0);
      controller.signal.throwIfAborted();
      if (code) throw Error(`mock child status ${code}`);
    } finally {
      task.stop();
      await task.exit;
    }
  });
  if (pass) await work;
  else await expect(work).rejects.toThrow(row.outcome === "cancel" ? "mock resource cancellation" : "mock child status 137");
  expect(started).toEqual(row.started);
}, 10000);

const plans = [
  din4108Plan({}),
  din4108Plan({ PLAY_TEST_GUEST: "core.wasm", PLAY_TEST_GUEST_JS: "bindings.js", SEMIO_TECH_PLAY_PORT: "6054" }),
  standardsPlan,
  companionPlan("en1990", { VERIFY_SIGNED_MODULES: "1", PLAY_TEST_PICKER_ROUNDTRIP: "1", PLAY_TEST_OBSERVE_ARCHIVES: "1" }),
] satisfies PlayPlan[];

test("the guarded workflows retain their lanes, probe arguments and picker settings", () => {
  expect(plans.length).toBe(fixture.diagnostics.length);
  for (const [index, row] of fixture.diagnostics.entries()) {
    const plan = plans[index]!;
    expect({ scope: plan.scope, port: plan.port, lanes: plan.lanes }, row.name).toEqual({ scope: row.scope, port: row.port, lanes: row.lanes });
    expect(plan.probes.map(probe => [probe.script, probe.args ?? [], probe.cwd === play, probe.environment?.VERIFY_EXAMPLE_WORKFLOW === "1"]), row.name).toEqual(row.probes);
  }
  expect(() => din4108Plan({ PLAY_TEST_GUEST: "core.wasm" })).toThrow("needs both");
  expect(() => din4108Plan({ PLAY_TEST_GUEST_JS: "bindings.js" })).toThrow("needs both");
  expect(standardsPlan.environment?.PLAY_ROUTE_CONCURRENCY).toBe("1");
});

test("unguarded direct re-entry refuses all three workflows before starting any command", async () => {
  for (const script of ["probe-norm-base-companions.ts", "verify-din4108-scoped-play.ts", "verify-standards-publication.ts"]) {
    const task = startSweepCommand(["repo", join(import.meta.dirname, script), "--admitted"], new AbortController().signal, {
      env: { ...process.env, SEMIO_HOST_CAPACITY_GRANT: undefined, SEMIO_HOST_CAPACITY_PLAN: undefined },
      stdio: "ignore",
    });
    expect(await task.exit, script).toBe(1);
  }
});
