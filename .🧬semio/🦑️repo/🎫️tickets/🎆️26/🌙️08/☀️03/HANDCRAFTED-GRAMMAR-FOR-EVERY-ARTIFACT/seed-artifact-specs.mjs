#!/usr/bin/env bun
import { existsSync, mkdirSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const facets = ["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr"];
const grammarSpec = "📖️component.grammar.semio";
const protocolSpec = "📡️component.protocol.semio";
const tsLeaf = "🟦️component.ts";

function listDirs(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((n) => statSync(join(dir, n)).isDirectory());
}

function artifactFamily(plugin, artifact) {
  const a = artifact.replace(/[^\x00-\x7f]/g, "");
  if (["dag", "flow", "wires", "sequence", "imperative", "mathematical", "trinity", "jack", "rewrite", "puzzle", "block", "space", "architect", "procedural"].some((k) => a.includes(k))) return "graph";
  if (["fem", "norm", "din", "en199", "iso", "vdi", "energy"].some((k) => a.includes(k))) return "sheet";
  if (["draw", "raster", "layout", "note", "shooting", "present", "lowpoly", "remodel", "cad"].some((k) => a.includes(k))) return "scene";
  if (["curate", "forms", "block"].some((k) => a.includes(k))) return "catalog";
  if (["process", "playbook", "home"].some((k) => a.includes(k))) return "recipe";
  if (["gis", "vcs"].some((k) => a.includes(k))) return "geo";
  if (["writer"].some((k) => a.includes(k))) return "embed";
  return "document";
}

function grammarBody(id, family) {
  return `dialect grammar
grammar ${id}
extension ${id}
use family-${family}
start document
document = TEXT*
`;
}

function protocolBody(id) {
  return `dialect protocol
protocol ${id}
version 1
start record
record = varint*
`;
}

function tsBody(kind) {
  if (kind === "grammar") {
    return `/** WASM facade — parse/print delegates to the plugin Rust crate. */\nexport function parseDsl(text: string): unknown {\n  throw new Error("wire to plugin WASM");\n}\nexport function printDsl(value: unknown): string {\n  throw new Error("wire to plugin WASM");\n}\n`;
  }
  return `/** WASM facade — encode/decode delegates to the plugin Rust crate. */\nexport function encode(value: unknown): Uint8Array {\n  throw new Error("wire to plugin WASM");\n}\nexport function decode(bytes: Uint8Array): unknown {\n  throw new Error("wire to plugin WASM");\n}\n`;
}

let created = 0;
for (const plugin of listDirs(pluginsRoot)) {
  const artifactsDir = join(pluginsRoot, plugin, "🗿️artifacts");
  for (const artifact of listDirs(artifactsDir)) {
    const id = artifact.replace(/[^\x00-\x7f]/g, "") || "artifact";
    const family = artifactFamily(plugin, id);
    for (const facet of facets) {
      const facetDir = join(artifactsDir, artifact, facet);
      if (!existsSync(facetDir)) continue;
      const spec = facet === "🎒️pack" || facet === "📡️spr" ? protocolSpec : grammarSpec;
      const specPath = join(facetDir, spec);
      if (!existsSync(specPath)) {
        const body = spec === protocolSpec ? protocolBody(id) : grammarBody(id, family);
        writeFileSync(specPath, body);
        created++;
      }
      const tsPath = join(facetDir, tsLeaf);
      if (!existsSync(tsPath)) {
        writeFileSync(tsPath, tsBody(spec === protocolSpec ? "protocol" : "grammar"));
        created++;
      }
    }
  }
}
console.log(`[DEBUG] seed-artifact-specs created ${created} files`);
