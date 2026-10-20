import { expect, test } from "bun:test";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import ts from "typescript";
import { planPlain } from "../tax2-tools/plain-lift.ts";
import { planRestructure } from "../tax2-tools/restructure.ts";
import { planLift } from "../tax-stage-tools/lift.ts";
import { Vfs } from "../tax-stage-tools/vfs.ts";
import { accepts, C, REPO } from "../tax2-tools/lib.ts";
import fixture from "./shape.fixture.json" with { type: "json" };

const root = join(C, "🗑️generated/lift-fix");
const smoke = join(C, "lift-fix-tools/smoke.sh");

/** 🔬️ Independent syntax oracle: the TypeScript compiler parses every emitted file without diagnostics. */
const parses = (text: string): boolean => ts.transpileModule(text, { reportDiagnostics: true, compilerOptions: { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext } }).diagnostics?.length === 0;

for (const row of fixture.cases) {
  test(row.id, () => {
    const directory = join(root, row.directory), scriptPath = join(directory, "📜️script.ts");
    const plan = planPlain(scriptPath.slice(REPO.length + 1), row.source);
    if (!plan.ok) throw Error(plan.reason);
    expect(parses(plan.script)).toBe(true);
    expect(parses(plan.module)).toBe(true);
    expect(accepts(plan.script)).toBe(true);
    expect(plan.script.includes("receiveScriptProcessInvocation")).toBe(row.form === "router");
    if (row.form === "router") expect(plan.script).toContain(row.terminal!);
    rmSync(root + "/synth", { recursive: true, force: true });
    mkdirSync(directory, { recursive: true });
    writeFileSync(scriptPath, plan.script);
    writeFileSync(join(directory, "🟦️.ts"), plan.module);
    const out = join(directory, "out.txt");
    const args = row.arguments.map((argument) => (argument === "OUT" ? out : argument));
    const run = row.form === "bare" ? Bun.spawnSync([process.execPath, scriptPath, ...args], { stderr: "pipe" }) : Bun.spawnSync(["sh", smoke, scriptPath, ...args], { stderr: "pipe" });
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    expect(readFileSync(out, "utf8")).toBe(row.written);
    expect(existsSync(dirname(out))).toBe(true);
    rmSync(root + "/synth", { recursive: true, force: true });
  });
}

for (const row of fixture.cases) {
  test(`${row.id} through the overlay lift`, () => {
    const scriptPath = join(root, row.directory, "📜️script.ts").slice(REPO.length + 1);
    const plan = planLift(scriptPath, row.source, new Vfs());
    if (!plan.ok) throw Error(plan.reason);
    expect(parses(plan.script)).toBe(true);
    expect(parses(plan.module)).toBe(true);
    expect(accepts(plan.script)).toBe(true);
    expect(plan.script.includes("receiveScriptProcessInvocation")).toBe(row.form === "router");
    if (row.form === "router") expect(plan.script).toContain(row.terminal!);
  });
}

for (const row of fixture.cases) {
  test(`${row.id} through the restructure lift`, () => {
    const scriptPath = join(root, row.directory, "📜️script.ts").slice(REPO.length + 1);
    const plan = planRestructure(scriptPath, row.source);
    if (!plan.ok) throw Error(plan.reason);
    expect(parses(plan.script)).toBe(true);
    expect(parses(plan.module)).toBe(true);
    expect(accepts(plan.script)).toBe(true);
    expect(plan.script.includes("receiveScriptProcessInvocation")).toBe(row.form === "router");
    if (row.form === "router") expect(plan.script).toContain(row.terminal!);
  });
}
