#!/usr/bin/env bun
/**
 * E2E placement + dialect sweep for plugin artifact facet specs.
 * Writes 🧪e2e-dialect-sweep.json (counts, manifest paths, failures). Exit 1 on any failure.
 */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "fs";
import { dirname, join, relative } from "path";
import { fileURLToPath } from "url";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(ticketDir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const outPath = join(ticketDir, "🧪e2e-dialect-sweep.json");

const textFacets = ["🗣️dsl", "🔧️op", "🔺️diff"];
const binFacets = ["🎒️pack", "📡️spr"];

function listDirs(d) {
  if (!existsSync(d)) return [];
  return readdirSync(d).filter((n) => statSync(join(d, n)).isDirectory());
}

function dialectOf(body) {
  const m = body.match(/^dialect\s+(\S+)/m);
  return m?.[1] ?? null;
}

function looksBinaryGrammar(body) {
  return /framing\s+magic|start\s+frame/.test(body) && !/^dialect\s+protocol/m.test(body);
}

function looksTextProtocol(body) {
  return /EDGEARROW|family-graph/.test(body) && !/framing|start\s+(frame|record)/.test(body);
}

const failures = [];
const manifest = { grammars: [], protocols: [] };

for (const plugin of listDirs(pluginsRoot)) {
  const artifactsRoot = join(pluginsRoot, plugin, "🗿️artifacts");
  if (!existsSync(artifactsRoot)) continue;
  for (const artifact of listDirs(artifactsRoot)) {
    const artifactPath = join(artifactsRoot, artifact);
    for (const facet of textFacets) {
      const dir = join(artifactPath, facet);
      if (!existsSync(dir)) continue;
      const grammarFile = join(dir, "📖️component.grammar.semio");
      const protocolFile = join(dir, "📡️component.protocol.semio");
      if (existsSync(protocolFile)) {
        failures.push({ kind: "wrongFile", path: relative(repoRoot, protocolFile), detail: "grammar facet must not host protocol file" });
      }
      if (!existsSync(grammarFile)) continue;
      const rel = relative(repoRoot, grammarFile);
      manifest.grammars.push(rel);
      const body = readFileSync(grammarFile, "utf8");
      const d = dialectOf(body);
      if (d !== "grammar") failures.push({ kind: "wrongDialect", path: rel, expect: "grammar", got: d });
      if (looksBinaryGrammar(body)) failures.push({ kind: "grammarLooksBinary", path: rel });
    }
    for (const facet of binFacets) {
      const dir = join(artifactPath, facet);
      if (!existsSync(dir)) continue;
      const protocolFile = join(dir, "📡️component.protocol.semio");
      const grammarFile = join(dir, "📖️component.grammar.semio");
      if (existsSync(grammarFile)) {
        failures.push({ kind: "wrongFile", path: relative(repoRoot, grammarFile), detail: "binary facet must not host grammar file" });
      }
      if (!existsSync(protocolFile)) continue;
      const rel = relative(repoRoot, protocolFile);
      manifest.protocols.push(rel);
      const body = readFileSync(protocolFile, "utf8");
      const d = dialectOf(body);
      if (d !== "protocol") failures.push({ kind: "wrongDialect", path: rel, expect: "protocol", got: d });
      if (looksTextProtocol(body)) failures.push({ kind: "protocolLooksText", path: rel });
    }
  }
}

manifest.grammars.sort();
manifest.protocols.sort();

const report = {
  generatedAt: new Date().toISOString(),
  ok: failures.length === 0,
  counts: {
    grammarOnText: manifest.grammars.length,
    protocolOnBinary: manifest.protocols.length,
    failures: failures.length,
  },
  failures,
  manifest,
};

writeFileSync(outPath, `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify({ ok: report.ok, counts: report.counts }, null, 2));
if (!report.ok) {
  for (const f of failures) console.error(f);
  process.exit(1);
}
