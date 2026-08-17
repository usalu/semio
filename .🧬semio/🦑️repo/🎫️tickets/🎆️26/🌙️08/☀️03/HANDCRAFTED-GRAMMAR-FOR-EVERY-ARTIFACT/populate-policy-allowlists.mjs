#!/usr/bin/env bun
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const facets = ["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr"];
const grammarSpec = "📖️component.grammar.semio";
const protocolSpec = "📡️component.protocol.semio";
const tsLeaf = "🟦️component.ts";
const stubTs = 'throw new Error("wire to plugin WASM")';
const stubGrammar = "document = TEXT*";

const grammar = [];
const protocol = [];
const tsFacade = [];

function listDirs(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((n) => statSync(join(dir, n)).isDirectory());
}

for (const plugin of listDirs(pluginsRoot)) {
  const artifactsDir = join(pluginsRoot, plugin, "🗿️artifacts");
  for (const artifact of listDirs(artifactsDir)) {
    for (const facet of facets) {
      const facetDir = join(artifactsDir, artifact, facet);
      if (!existsSync(facetDir)) continue;
      const rel = join("✏️s/🔌️plugins", plugin, "🗿️artifacts", artifact, facet).replaceAll("\\", "/");
      const spec = facet === "🎒️pack" || facet === "📡️spr" ? protocolSpec : grammarSpec;
      const specPath = join(facetDir, spec);
      if (existsSync(specPath)) {
        const body = readFileSync(specPath, "utf8");
        if (body.includes(stubGrammar) || body.includes("use family-")) {
          (spec === protocolSpec ? protocol : grammar).push(`${rel}/${spec}`);
        }
      }
      const tsPath = join(facetDir, tsLeaf);
      if (existsSync(tsPath) && readFileSync(tsPath, "utf8").includes(stubTs)) {
        tsFacade.push(`${rel}/${tsLeaf}`);
      }
    }
  }
}

const out = join(import.meta.dir, "policy-allowlist-snippet.txt");
writeFileSync(
  out,
  `// paste into 📜️script.ts POLICY_* sets\nPOLICY_GRAMMAR count ${grammar.length}\nPOLICY_PROTOCOL count ${protocol.length}\nPOLICY_TS count ${tsFacade.length}\n`,
);
console.log(`[DEBUG] wrote ${out} grammar=${grammar.length} protocol=${protocol.length} ts=${tsFacade.length}`);
