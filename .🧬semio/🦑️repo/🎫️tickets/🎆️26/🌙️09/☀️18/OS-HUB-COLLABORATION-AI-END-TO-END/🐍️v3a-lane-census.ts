#!/usr/bin/env bun
/** 🔎️ V3a census: for every surface/plugin-root schema-lane owner the registry gate judges, record
 * whether a source of truth exists (a real Rust config/presence struct) and whether the lane is present. */
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const PLUGINS = "✏️s/🔌️plugins";
const ARTIFACTS = "🗿️artifacts", STANDARDS = "🏅️standards", SUBSETS = "🪆️subsets";
const ROLE_DIRS = ["👁️viewer", "✏️editor"];
const CONFIG = "🎚️config", PRESENCE = "👥️presence", SCHEMA = "🧬️schema", LEAF = "🦀️.rs";

function listDirs(abs: string): string[] {
  if (!existsSync(abs)) return [];
  return readdirSync(abs).filter((n) => !n.startsWith(".") && statSync(join(abs, n)).isDirectory()).sort();
}

type Row = {
  owner: string;
  kind: "surface" | "plugin-root";
  plugin: string;
  lane: "config" | "presence";
  laneDirExists: boolean;
  schemaDirExists: boolean;
  ownLeaf: boolean;
  structName: string | null;
  structFields: number | null;
  typeConfigBinding: string | null;
};

function structNamesIn(text: string): { name: string; fields: number }[] {
  const out: { name: string; fields: number }[] = [];
  const re = /\bpub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(text))) {
    let depth = 1, i = re.lastIndex;
    for (; i < text.length; i++) {
      const c = text[i];
      if (c === "{") depth++;
      else if (c === "}" && --depth === 0) break;
    }
    const body = text.slice(re.lastIndex, i);
    out.push({ name: m[1]!, fields: (body.match(/\bpub\s+[a-z][a-z0-9_]*\s*:/g) ?? []).length });
  }
  return out;
}

const rows: Row[] = [];

function inspect(ownerAbs: string, kind: Row["kind"], plugin: string, typeConfigBinding: string | null): void {
  for (const [lane, dirName] of [["config", CONFIG], ["presence", PRESENCE]] as const) {
    const laneAbs = join(ownerAbs, dirName);
    const laneDirExists = existsSync(laneAbs);
    if (kind === "surface" && !existsSync(join(ownerAbs, CONFIG))) continue;
    const leafAbs = join(laneAbs, LEAF);
    const ownLeaf = existsSync(leafAbs);
    let structName: string | null = null, structFields: number | null = null;
    if (ownLeaf) {
      const text = readFileSync(leafAbs, "utf8");
      const wanted = lane === "config" ? typeConfigBinding : typeConfigBinding ? typeConfigBinding.replace(/Config$/, "") + "Presence" : null;
      const structs = structNamesIn(text);
      const hit = wanted ? structs.find((s) => s.name === wanted) : structs[0];
      if (hit) { structName = hit.name; structFields = hit.fields; }
      else if (structs.length) { structName = structs[0]!.name; structFields = structs[0]!.fields; }
    }
    rows.push({
      owner: relative(repoRoot, ownerAbs).replaceAll("\\", "/"),
      kind, plugin, lane, laneDirExists,
      schemaDirExists: existsSync(join(laneAbs, SCHEMA)),
      ownLeaf, structName, structFields, typeConfigBinding,
    });
  }
}

for (const plugin of listDirs(join(repoRoot, PLUGINS))) {
  const pluginAbs = join(repoRoot, PLUGINS, plugin);
  inspect(pluginAbs, "plugin-root", plugin, null);
  const artifactsAbs = join(pluginAbs, ARTIFACTS);
  for (const kind of listDirs(artifactsAbs)) {
    const standardsAbs = join(artifactsAbs, kind, STANDARDS);
    for (const std of listDirs(standardsAbs)) {
      const subsetsAbs = join(standardsAbs, std, SUBSETS);
      for (const sub of listDirs(subsetsAbs)) {
        for (const role of ROLE_DIRS) {
          const surfaceAbs = join(subsetsAbs, sub, role);
          if (!existsSync(surfaceAbs)) continue;
          const componentAbs = join(surfaceAbs, LEAF);
          const binding = existsSync(componentAbs) ? /\btype\s+Config\s*=\s*([A-Za-z_][A-Za-z0-9_]*)\s*;/.exec(readFileSync(componentAbs, "utf8"))?.[1] ?? null : null;
          inspect(surfaceAbs, "surface", plugin, binding);
        }
      }
    }
  }
}

const bucket = (r: Row): string => `${r.kind}/${r.lane} lane=${r.laneDirExists ? "y" : "n"} schema=${r.schemaDirExists ? "y" : "n"} rs=${r.ownLeaf ? "y" : "n"} struct=${r.structName ? "y" : "n"} binding=${r.typeConfigBinding ? "y" : "n"}`;
const counts = new Map<string, number>();
for (const r of rows) counts.set(bucket(r), (counts.get(bucket(r)) ?? 0) + 1);
console.log(`rows=${rows.length}`);
for (const [k, v] of [...counts].sort((a, b) => b[1] - a[1])) console.log(String(v).padStart(5), k);

if (process.argv.includes("--json")) console.log(JSON.stringify(rows, null, 1));
