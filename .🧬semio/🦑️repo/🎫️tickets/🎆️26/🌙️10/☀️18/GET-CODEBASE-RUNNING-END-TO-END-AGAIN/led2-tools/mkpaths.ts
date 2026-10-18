import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = process.cwd(), nm = join(process.argv[2]!, "node_modules");
const paths: Record<string, string[]> = { ajv: ["ajv/dist/ajv.js"], "ajv/*": ["ajv/*"], "ajv-formats": ["ajv-formats/dist/index.js"] };
const flat = (exp: unknown): string | undefined => typeof exp === "string" ? exp : exp && typeof exp === "object" ? flat((exp as Record<string, unknown>).import ?? (exp as Record<string, unknown>).default ?? (exp as Record<string, unknown>).bun) : undefined;
for (const rel of readFileSync(process.argv[3]!, "utf8").split("\n").filter(Boolean)) {
  if (!existsSync(join(root, rel))) continue;
  let pkg: any; try { pkg = JSON.parse(readFileSync(join(root, rel), "utf8")); } catch { continue; }
  if (typeof pkg.name !== "string" || !pkg.name.startsWith("@semio-tech/") || pkg.exports === undefined) continue;
  const dir = resolve(root, dirname(rel));
  const exp = typeof pkg.exports === "string" ? { ".": pkg.exports } : pkg.exports;
  for (const [key, value] of Object.entries(exp)) { const target = flat(value); if (target === undefined || key.includes("*")) continue; paths[key === "." ? pkg.name : pkg.name + key.slice(1)] = [resolve(dir, target)]; }
}
console.log(JSON.stringify({ compilerOptions: { baseUrl: nm, paths } }, null, 1));
