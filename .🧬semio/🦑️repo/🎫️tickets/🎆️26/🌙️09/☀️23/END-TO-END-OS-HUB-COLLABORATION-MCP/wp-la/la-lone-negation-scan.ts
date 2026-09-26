/** 🔎️ LA: scans nx.json + every tracked 📋️project.json for input lists whose `!{workspaceRoot}/…` negations have no direct workspace positive beside them. */
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = "/Users/ueli/Documents/semio";
const lone = (list: unknown[]) => list.some((entry) => typeof entry === "string" && entry.startsWith("!{workspaceRoot}/")) && !list.some((entry) => typeof entry === "string" && entry.startsWith("{workspaceRoot}/"));
const findings: string[] = [];
const scan = (owner: string, json: { namedInputs?: Record<string, unknown[]>; targets?: Record<string, { inputs?: unknown[] }> }) => {
  for (const [name, list] of Object.entries(json.namedInputs ?? {})) if (lone(list)) findings.push(`${owner} namedInputs.${name}`);
  for (const [name, target] of Object.entries(json.targets ?? {})) if (target.inputs && lone(target.inputs)) findings.push(`${owner} targets.${name}.inputs`);
};
scan("nx.json", JSON.parse(readFileSync(join(root, "nx.json"), "utf8")));
const projects = execFileSync("git", ["ls-files", "-z", "*📋️project.json"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 26 }).split("\0").filter(Boolean);
for (const path of projects) scan(path, JSON.parse(readFileSync(join(root, path), "utf8")));
console.log(JSON.stringify({ projects: projects.length, findings }, null, 1));
const { cacheInternals, libraryBootstrap } = await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs");
await libraryBootstrap;
const started = performance.now();
const facts = new Map();
for (const path of projects) {
  const projectRoot = path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : ".";
  const json = JSON.parse(readFileSync(join(root, path), "utf8"));
  try { scan(`derived ${projectRoot}`, { namedInputs: cacheInternals.projectInputs(json, projectRoot, root, facts) }); } catch (error) { findings.push(`derived ${projectRoot} threw ${(error as Error).message.slice(0, 120)}`); }
}
console.log(JSON.stringify({ derivedMs: Math.round(performance.now() - started), findings }, null, 1));
