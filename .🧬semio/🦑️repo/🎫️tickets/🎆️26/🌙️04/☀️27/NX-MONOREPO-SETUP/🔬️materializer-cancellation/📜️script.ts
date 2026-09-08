import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import ts from "typescript";
const root = process.cwd();
const ticket = resolve(import.meta.dir, "..");
const directory = join(ticket, "🗑️generated", "materializer-cancellation");
mkdirSync(directory, { recursive: true });
const path = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
const declaration = source.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "spawnAsync")!;
const code = ts.transpileModule(declaration.getText(source), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
const execute = new Function("spawn", `${code}; return spawnAsync;`)(spawn);
const script = join(directory, "📜️script.ts"), ready = join(directory, "pid");
writeFileSync(script, 'import { writeFileSync } from "node:fs"; process.on("SIGTERM", () => {}); setInterval(() => {}, 1000); writeFileSync(process.argv[2], String(process.pid));');
for (let iteration = 0; iteration < 12; iteration++) {
  rmSync(ready, { force: true });
  const controller = new AbortController();
  const execution = execute(process.execPath, [script, ready], directory, controller.signal).catch((error: Error) => error);
  const startup = Date.now();
  while (!existsSync(ready)) { assert.ok(Date.now() - startup < 10_000); await Bun.sleep(25); }
  const text = readFileSync(ready, "utf8"), pid = Number(text);
  console.log("[DEBUG] ready", { iteration, text, pid });
  try {
    const started = Date.now();
    controller.abort(new Error("materializer cancelled"));
    assert.match(String(await execution), /materializer cancelled/);
    let alive = false;
    try { process.kill(pid, 0); alive = true; } catch {}
    const oracle = Bun.spawnSync(["ps", "-p", String(pid), "-o", "pid=,ppid=,stat=,command="], { stdout: "pipe", stderr: "pipe" });
    console.log("[DEBUG] cancelled", { iteration, milliseconds: Date.now() - started, alive, oracle: oracle.stdout.toString(), error: oracle.stderr.toString() });
    assert.ok(pid > 0);
    assert.equal(alive, false);
  } finally {
    controller.abort();
    if (pid > 0 && process.platform !== "win32") try { process.kill(-pid, "SIGKILL"); } catch {}
  }
}
console.log("[DEBUG] Materializer native cancellation repeats PASS");
