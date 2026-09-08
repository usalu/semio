import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { EventEmitter } from "node:events";
import ts from "typescript";
const root = process.cwd(), library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const source = ts.createSourceFile("root.ts", readFileSync(join(root, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
const declaration = source.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "NxScript")!;
const code = ts.transpileModule(declaration.getText(source).replace(/^export /, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
for (const vector of JSON.parse(readFileSync(join(root, library, "⚡️caching/🧫️fixtures/🛑️cancellation.json"), "utf8")).cases) {
  const runtime = Object.assign(new EventEmitter(), { platform: vector.platform, exitCode: 0, kill: (pid: number, signal: number | string) => { if (!signal) throw new Error("No process"); killed.push(pid); } });
  const child = Object.assign(new EventEmitter(), { pid: 1234 }), killed: number[] = [], calls: string[] = [];
  const NxScript = new Function("Script", "process", "createRequire", "join", "resolveNxInvocation", "devToolingEnv", "orchestratorBudgetOpts", "spawnNxProcess", "stopNxProcessTree", code + "; return NxScript;")(
    class { root = root; }, runtime, () => ({ resolve: (name: string) => name }), join, (args: string[]) => ({ args, env: {} }), (env: unknown) => env, () => ({}), () => child,
    (command: string) => { calls.push(command); if (command === "taskkill") { killed.push(child.pid); return { status: 0 }; } if (vector.throws) throw new Error("Snapshot unavailable"); return { status: 0, stdout: vector.stdout }; });
  const done = new NxScript().run(["run", "fixture:build"]);
  assert.doesNotThrow(() => runtime.emit("SIGTERM"), vector.name);
  child.emit("close", null);
  await done;
  assert.equal(runtime.exitCode, vector.expectedExit, vector.name);
  assert.ok(killed.length > 0, `${vector.name}: owned child must still be stopped`);
  assert.equal(runtime.listenerCount("SIGTERM"), 0);
  console.log(`[DEBUG] Cancellation fallback ${vector.name}: signal exit and owned child stop PASS`);
}
