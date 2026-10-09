#!/usr/bin/env bun
/**
 * 🕰️ Wave W04: one-shot hand-migration of every committed JSON document of the BIM plugin to the `phase` field on curtain walls, columns,
 * beams, slabs, roofs, stairs, railings and spaces (`New`, the phase of a freshly authored element). A record keeps the struct order of the
 * snapshot (phase before `name`, after `usage` for a space); a `Created` diff entry keeps its sorted keys. Raw lexemes survive: only the
 * inserted line differs. Run from the repo root: `bun r10-w04-migrate-phase.ts [--dry]`. Idempotent: records that carry `phase` are left alone.
 */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { parse, print, type Node } from "./r6-z-mutations-rawjson.ts";

const repo = join(import.meta.dir, "../../../../../../..");
const plugin = join(repo, readdirSync(repo).find((n) => n.startsWith("✏") && n.endsWith("s"))!);
const bim = (() => {
  const plugins = join(plugin, readdirSync(plugin).find((n) => n.endsWith("plugins"))!);
  return join(plugins, readdirSync(plugins).find((n) => n.endsWith("bim"))!);
})();
const SKIP = new Set(["target", "node_modules", "🗑️generated", ".git", "dist"]);
const DRY = process.argv.includes("--dry");

const KINDS: { name: string; has: string[]; after?: string }[] = [
  { name: "curtain wall", has: ["storey", "u_spacing", "v_spacing", "mullion"] },
  { name: "column", has: ["storey", "column_type", "position"] },
  { name: "beam", has: ["storey", "beam_type", "start", "end"] },
  { name: "slab", has: ["storey", "slab_type", "boundary"] },
  { name: "roof", has: ["storey", "roof_type", "footprint"] },
  { name: "stair", has: ["storey", "flight", "max_riser"] },
  { name: "railing", has: ["storey", "path", "post_spacing"] },
  { name: "space", has: ["storey", "number", "boundary", "usage"], after: "usage" },
];

const NEW: Node = { k: "raw", v: JSON.stringify("New") };

function migrate(node: Node, stats: Record<string, number>): Node {
  if (node.k === "arr") return { k: "arr", v: node.v.map((item) => migrate(item, stats)) };
  if (node.k !== "obj") return node;
  const entries: [string, Node][] = node.v.map(([key, item]) => [key, migrate(item, stats)]);
  const keys = new Set(entries.map(([key]) => key));
  const entry = entries.find(([key]) => key === "entry")?.[1];
  const patched = entry?.k === "raw" && entry.v === '"Patched"';
  const kind = KINDS.find((candidate) => candidate.has.every((key) => keys.has(key)));
  if (!kind || keys.has("phase") || keys.has("mutation") || patched) return { k: "obj", v: entries };
  stats[kind.name] = (stats[kind.name] ?? 0) + 1;
  if (entry) {
    const at = entries.findIndex(([key]) => key > "phase");
    entries.splice(at < 0 ? entries.length : at, 0, ["phase", NEW]);
  } else if (kind.after) {
    entries.splice(entries.findIndex(([key]) => key === kind.after) + 1, 0, ["phase", NEW]);
  } else {
    const at = entries.findIndex(([key]) => key === "name");
    entries.splice(at < 0 ? entries.length : at, 0, ["phase", NEW]);
  }
  return { k: "obj", v: entries };
}

function* tree(root: string): Generator<string> {
  for (const name of readdirSync(root)) {
    if (SKIP.has(name)) continue;
    const path = join(root, name);
    if (statSync(path).isDirectory()) yield* tree(path);
    else if (name.endsWith(".json")) yield path;
  }
}

const totals: Record<string, number> = {};
let files = 0;
for (const path of tree(bim)) {
  const text = readFileSync(path, "utf8");
  if (text.includes('"$schema"')) continue;
  const crlf = text.includes("\r\n");
  let root: Node;
  try {
    root = parse(text.replace(/\r\n/g, "\n"));
  } catch {
    continue;
  }
  const stats: Record<string, number> = {};
  const out = migrate(root, stats);
  if (Object.keys(stats).length === 0) continue;
  for (const [name, count] of Object.entries(stats)) totals[name] = (totals[name] ?? 0) + count;
  files++;
  if (!DRY) writeFileSync(path, print(out).replace(/\n/g, crlf ? "\r\n" : "\n") + (crlf ? "\r\n" : "\n"));
}
console.log(DRY ? "would change" : "changed", files, "files", JSON.stringify(totals));
