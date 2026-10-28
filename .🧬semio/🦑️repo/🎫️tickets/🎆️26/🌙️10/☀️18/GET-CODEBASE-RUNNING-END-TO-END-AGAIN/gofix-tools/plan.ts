import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(process.argv[2] ?? "");
const out = resolve(process.argv[3] ?? "");
if (!root || !out) throw new Error("usage: plan.ts <mirror-root> <out-dir>");
mkdirSync(out, { recursive: true });
const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts");
const { canonicalGoPlan } = await import(library);
const goWork = readFileSync(join(root, "go.work"), "utf8");
const inWork = new Set([...goWork.matchAll(/^\t\.\/(.+)$/gm)].map((m) => m[1]!));
const listed = spawnSync("git", ["-c", "core.quotepath=off", "ls-files", "-z", "--", "*go.mod"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 26 }).stdout.split("\0").filter(Boolean);
const rows: unknown[] = [];
for (const file of listed) {
  const module = file.slice(0, -"/go.mod".length);
  if (module.startsWith(".🧬semio/") || !existsSync(join(root, file))) continue;
  const row: Record<string, unknown> = { module, inWork: inWork.has(module) };
  try {
    const plan = canonicalGoPlan(join(root, module));
    const id = rows.length;
    writeFileSync(join(out, `${id}.overlay.json`), JSON.stringify({ Replace: plan.replacements }));
    Object.assign(row, { id, packages: plan.packages });
  } catch (error) {
    row.error = String((error as Error).message);
  }
  rows.push(row);
}
writeFileSync(join(out, "modules.json"), JSON.stringify(rows, null, 1));
console.log(`planned ${rows.length} modules`);
