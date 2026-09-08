#!/usr/bin/env bun
/** 🔬️ Runs the real `publication-authority-audit` with its `ownerOracle` instrumented: the audit's
 * own source is copied out, `production` is wrapped in a logging proxy and every `return false` is
 * numbered, so the exact failing clause is named instead of the opaque "diverged from the fixture".
 * Diagnostic only — the copy is written to a scratch directory and never imported by production. */
import { mkdirSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const auditDir = resolve(repoRoot, "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript");
const auditFile = resolve(auditDir, "📜️script.ts");
const outDir = process.env["SEMIO_TRACE_DIR"] ?? resolve(repoRoot, "🗑️trace-publication-authority");
let audit = await Bun.file(auditFile).text();

audit = audit.replace(/from "(\.\.\/[^"]+)"/g, (_match, path: string) => `from ${JSON.stringify(resolve(auditDir, path))}`);
audit = audit.replace("new ScriptRouter(import.meta.dir)", `new ScriptRouter(${JSON.stringify(auditDir)})`);
audit = audit.replace(
  'const production = source.split("//#region 🧪️Testkit")[0]!;',
  `const rawProduction = source.split("//#region 🧪️Testkit")[0]!;
  const production = new Proxy(Object(rawProduction) as unknown as string, {
    get(target: object, property: string | symbol) {
      if (property === "includes") {
        return (needle: string): boolean => {
          const held = rawProduction.includes(needle);
          if (!held) console.error(\`[DEBUG] \${owner.owner} missing anchor \${JSON.stringify(needle)}\`);
          return held;
        };
      }
      const value = (target as Record<string | symbol, unknown>)[property];
      return typeof value === "function" ? (value as (...args: unknown[]) => unknown).bind(rawProduction) : value;
    },
  }) as unknown as string;`,
);
let clause = 0;
audit = audit.replace(/return false;/g, () => {
  clause += 1;
  return `return dbgFalse(${clause});`;
});
audit = audit.replace(
  "function ownerOracle(",
  `function dbgFalse(clause: number): false {
  console.error(\`[DEBUG] ownerOracle structural clause #\${clause} failed\`);
  return false;
}

function ownerOracle(`,
);
audit = audit.replace("if (!validate(hostile)", "if (!validate(hostile)");
mkdirSync(outDir, { recursive: true });
const tracedFile = resolve(outDir, "traced-audit.ts");
await Bun.write(tracedFile, audit);
const owners = process.argv.slice(2);
for (const owner of owners.length > 0 ? owners : ["Puzzle2dPlayApp", "Puzzle3dPlayApp", "Puzzle5dPlayApp"]) {
  console.error(`===== ${owner}`);
  const proc = Bun.spawnSync([process.execPath, tracedFile, "publication-authority-audit", owner], { cwd: repoRoot, stderr: "pipe", stdout: "pipe" });
  console.error(new TextDecoder().decode(proc.stderr).split("\n").filter((line) => line.startsWith("[DEBUG]") || line.startsWith("error:") || line.startsWith("validated")).join("\n"));
}
