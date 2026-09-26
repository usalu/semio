#!/usr/bin/env bun
/** 🧮️ R9 item 1: inventory of every declared nx target (📋️project.json) against `.vscode/launch.json`: verb, dependsOn
 * in-degree (targets only other targets run), registration. Usage: `bun launch-inventory.ts <out.json>`. */
import { lstatSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const SKIP = new Set(["node_modules", ".git", "dist", "temp", "storybook-static", "🤖️generated", "🗑️generated", "🎫️tickets", ".🧬semio", "target"]);
type Target = { command?: string; executor?: string; dependsOn?: unknown[]; options?: { command?: string; commands?: unknown[] }; metadata?: Record<string, unknown> };
const projects: { name: string; path: string; targets: Record<string, Target> }[] = [];
function walk(directory: string): void {
  let entries: string[];
  try { entries = readdirSync(directory); } catch { return; }
  if (entries.includes("📋️project.json")) {
    try {
      const parsed = JSON.parse(readFileSync(join(directory, "📋️project.json"), "utf8"));
      if (parsed.name) projects.push({ name: parsed.name, path: relative(ROOT, directory), targets: parsed.targets ?? {} });
    } catch {}
  }
  for (const entry of entries) {
    if (SKIP.has(entry) || entry.startsWith(".")) continue;
    try { if (lstatSync(join(directory, entry)).isDirectory()) walk(join(directory, entry)); } catch {}
  }
}
walk(ROOT);
const launch = Bun.JSONC.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")) as { configurations: { name: string; command?: string }[] };
const registered = new Set<string>();
for (const row of launch.configurations) for (const m of String(row.command ?? "").matchAll(/nx run ([^\s]+):([A-Za-z0-9_.-]+)/g)) registered.add(`${m[1]}:${m[2]}`);
const inDegree = new Map<string, number>();
for (const project of projects) for (const [, target] of Object.entries(project.targets)) for (const dep of target.dependsOn ?? []) {
  const spec = typeof dep === "string" ? dep : (dep as { target?: string; projects?: unknown }).target;
  if (!spec) continue;
  const bare = spec.replace(/^\^/, "");
  const [p, t] = bare.includes(":") ? bare.split(":") : [project.name, bare];
  const key = `${p}:${t}`;
  inDegree.set(key, (inDegree.get(key) ?? 0) + 1);
}
const rows = projects.flatMap((project) => Object.entries(project.targets).map(([target, spec]) => ({
  project: project.name, path: project.path, target, verb: target.split(/[-:]/)[0], registered: registered.has(`${project.name}:${target}`),
  dependedOn: inDegree.get(`${project.name}:${target}`) ?? 0, command: spec.command ?? spec.options?.command ?? (spec.executor ?? ""), metadata: spec.metadata ?? null,
})));
const verbs = new Map<string, { total: number; registered: number; dependedOn: number }>();
for (const row of rows) { const v = verbs.get(row.verb) ?? { total: 0, registered: 0, dependedOn: 0 }; v.total++; if (row.registered) v.registered++; if (row.dependedOn) v.dependedOn++; verbs.set(row.verb, v); }
console.log(`projects=${projects.length} targets=${rows.length} registered=${rows.filter((r) => r.registered).length} dependedOn=${rows.filter((r) => r.dependedOn).length} withMetadata=${rows.filter((r) => r.metadata).length}`);
for (const [verb, v] of [...verbs].sort((a, b) => b[1].total - a[1].total).slice(0, 45)) console.log(`${verb.padEnd(28)} total=${v.total} registered=${v.registered} dependedOn=${v.dependedOn}`);
writeFileSync(process.argv[2]!, JSON.stringify(rows, null, 1));
