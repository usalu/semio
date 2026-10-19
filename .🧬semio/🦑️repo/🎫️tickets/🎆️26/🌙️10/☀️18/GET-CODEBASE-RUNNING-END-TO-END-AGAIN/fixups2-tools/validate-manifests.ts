import { readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { createRequire } from "node:module";

import { validateJsonSchemaSubset } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
const require = createRequire(import.meta.url);
const Ajv = require("ajv").default;
const root = "/Users/ueli/Documents/semio";
const schema = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"), "utf8"));
const validate = new Ajv({ strict: false }).compile(schema);
const skip = new Set(["node_modules", "target", "dist", ".git", "🗑️generated", ".cache", "bazel-out"]);
const files: string[] = [];
const walk = (dir: string) => {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (skip.has(entry.name) || entry.name.startsWith("bazel-")) continue;
    const full = join(dir, entry.name);
    if (entry.isDirectory()) walk(full);
    else if (entry.name === "📋️project.json" || entry.name === "project.json") files.push(full);
  }
};
for (const top of process.argv.slice(2).length ? process.argv.slice(2) : ["✏️s", "🧰️framework"]) walk(join(root, top));
const problems: string[] = [];
const keys = new Map<string, string>();
for (const file of files.sort()) {
  const rel = relative(root, file);
  let project: any;
  try { project = JSON.parse(readFileSync(file, "utf8")); } catch (error) { problems.push(`${rel}: unparsable ${error}`); continue; }
  const own = validateJsonSchemaSubset(schema, project);
  if (own.length) problems.push(`${rel}: repo-validator ${JSON.stringify(own.slice(0, 3))}`);
  if (!validate(project)) problems.push(`${rel}: schema ${JSON.stringify(validate.errors?.slice(0, 3))}`);
  for (const [target, raw] of Object.entries<any>(project.targets ?? {})) {
    const declaration = raw?.metadata?.semio?.workspaceCommand;
    if (declaration === undefined) continue;
    if (!Array.isArray(declaration) || declaration.length < 2 || declaration.some((word: unknown) => typeof word !== "string" || !/^[a-zA-Z0-9-]+$/.test(word))) problems.push(`${rel}:${target} invalid workspaceCommand ${JSON.stringify(declaration)}`);
    else {
      const key = declaration.join(" ");
      if (keys.has(key)) problems.push(`${rel}:${target} duplicate workspaceCommand '${key}' also ${keys.get(key)}`);
      keys.set(key, `${rel}:${target}`);
    }
  }
}
console.log(`manifests=${files.length} workspaceCommands=${keys.size} problems=${problems.length}`);
for (const [key, where] of keys) console.log(`  cmd '${key}' <- ${where}`);
for (const p of problems) console.log(`PROBLEM ${p}`);
process.exit(problems.length ? 1 : 0);
