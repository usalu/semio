#!/usr/bin/env bun
/**
 * 🧪 Score grammar vs protocol facet specs; rewrite cross-contaminated or weak templates.
 */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const facets = ["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr"];

function listDirs(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((n) => statSync(join(dir, n)).isDirectory());
}

function stripEmoji(s) {
  const a = s.replace(/[^\x00-\x7f]/g, "");
  return a || "artifact";
}

function familyOf(plugin, artifact) {
  const a = stripEmoji(artifact);
  const p = stripEmoji(plugin);
  if (["dag", "flow", "wires", "sequence", "imperative", "mathematical", "trinity", "jack", "rewrite", "puzzle", "block", "space", "architect", "procedural"].some((k) => a.includes(k) || p.includes(k))) return "graph";
  if (["fem", "norm", "din", "en199", "iso", "vdi", "energy"].some((k) => a.includes(k) || p.includes(k))) return "sheet";
  if (["draw", "raster", "layout", "note", "shooting", "present", "lowpoly", "remodel", "cad", "animate"].some((k) => a.includes(k) || p.includes(k))) return "scene";
  if (["curate", "forms"].some((k) => a.includes(k) || p.includes(k))) return "catalog";
  if (["process", "playbook", "home"].some((k) => a.includes(k) || p.includes(k))) return "recipe";
  if (["gis", "vcs"].some((k) => a.includes(k) || p.includes(k))) return "geo";
  if (["writer"].some((k) => a.includes(k) || p.includes(k))) return "embed";
  return "document";
}

const COMMON_TABLE = `
assign = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map | qty | block)
list = "[" value* "]"
map = "{" assign* "}"
value = TEXT | INT | FLOAT | BOOL | IDENT | qty | list | map | block
qty = (FLOAT | INT) IDENT?
block = "{" record* "}"
record = IDENT assign* block?
table = IDENT table-schema "{" row* "}"
table-schema = "[" col {"," col}* "]"
col = IDENT ":" IDENT
row = field+
field = IDENT "=" value | value
props = "{" prop* "}"
prop = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map | qty)
`;

function camelToKebab(s) {
  return s.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
}

function extractOpKeywords(opRsPath) {
  if (!existsSync(opRsPath)) return [];
  const text = readFileSync(opRsPath, "utf8");
  const keys = [];
  const kwPat = /dsl\(keyword\s*=\s*"([^"]+)"/g;
  let m;
  while ((m = kwPat.exec(text))) keys.push(m[1]);
  if (keys.length) return keys;
  const enumStart = text.search(/pub enum \w*Operation\w*/);
  if (enumStart < 0) return [];
  const brace = text.indexOf("{", enumStart);
  if (brace < 0) return [];
  let depth = 0;
  let end = brace;
  for (let i = brace; i < text.length; i++) {
    if (text[i] === "{") depth++;
    else if (text[i] === "}") {
      depth--;
      if (depth === 0) {
        end = i;
        break;
      }
    }
  }
  const body = text.slice(brace + 1, end);
  for (const line of body.split("\n")) {
    const tuple = line.match(/^\s*([A-Z][A-Za-z0-9]*)\s*\(/);
    if (tuple) {
      keys.push(camelToKebab(tuple[1]));
      continue;
    }
    const v = line.match(/^\s*([A-Z][A-Za-z0-9]*)\s*\{/);
    if (v) keys.push(camelToKebab(v[1]));
  }
  return keys;
}

function inferDocSchema(artifactDir, artifactName) {
  const artRs = join(artifactDir, "🦀️component.rs");
  if (existsSync(artRs)) {
    const m = readFileSync(artRs, "utf8").match(/SCHEMA:\s*&str\s*=\s*"([^"]+)"/);
    if (m) return m[1];
    const m2 = readFileSync(artRs, "utf8").match(/(\w+_SCHEMA):\s*&str\s*=\s*"([^"]+)"/);
    if (m2) return m2[2];
  }
  const dslGram = join(artifactDir, "🗣️dsl", "📖️component.grammar.semio");
  if (existsSync(dslGram)) {
    const m = readFileSync(dslGram, "utf8").match(/^grammar\s+(.+)/m);
    if (m) {
      const g = m[1].trim();
      if (g.endsWith(".document")) return g.slice(0, -".document".length);
      return g;
    }
  }
  return stripEmoji(artifactName);
}

function inferProtocolId(artifactDir, artifactName, facet) {
  const path = join(artifactDir, facet, "📡️component.protocol.semio");
  if (existsSync(path)) {
    const m = readFileSync(path, "utf8").match(/^protocol\s+(.+)/m);
    if (m) return m[1].trim();
  }
  const id = stripEmoji(artifactName);
  return facet === "🎒️pack" ? `${id}.pack` : `${id}.spr`;
}

function scoreGrammar(body) {
  const issues = [];
  if (!body.includes("dialect grammar")) issues.push("no-dialect-grammar");
  if (!body.match(/start (document|operation|diff)/)) issues.push("bad-start");
  if (!body.includes("IDENT") && !body.includes("TEXT")) issues.push("no-tokens");
  if (body.includes("framing magic") || body.includes("segment kind") || body.includes("footer fixed")) issues.push("pack-leak");
  if (body.includes("dialect protocol")) issues.push("wrong-dialect");
  if (body.match(/\n(document|operation|diff) = (field\*|TEXT\*|statement\*|layer\*|step\*|feature\*|stock\*)\s*$/m)) issues.push("generic-star");
  const pass = issues.length === 0;
  return { pass, issues, score: pass ? 10 : Math.max(0, 10 - issues.length * 2) };
}

function scoreProtocol(body) {
  const issues = [];
  if (!body.includes("dialect protocol")) issues.push("no-dialect-protocol");
  if (!body.match(/protocol [\w.-]+\.(pack|spr)/)) issues.push("no-protocol-id");
  if (!body.includes("schema ")) issues.push("no-schema");
  if (!body.match(/start (frame|record)/)) issues.push("bad-start");
  const bin = /u8|u16|u32|varint|bytes|leb128/.test(body);
  if (!bin) issues.push("no-binary-fields");
  if (body.includes("EDGEARROW") || body.includes('header = "semio"') || body.includes('section = ("nodes"')) issues.push("text-ebnf");
  if (body.includes("dialect grammar")) issues.push("wrong-dialect");
  if (body.includes("start frame") && body.includes("start record")) issues.push("mixed-start");
  const pass = issues.length === 0;
  return { pass, issues, score: pass ? 10 : Math.max(0, 10 - issues.length * 2) };
}

function protocolPack(id, schema) {
  return `dialect protocol
protocol ${id}
version 1
schema ${schema}
start frame
framing magic 0x8953504B0D0A1A0A
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
record field id u16 type tag
field tag varint
field body bytes
footer fixed 84
`;
}

function protocolSpr(id, schema) {
  return `dialect protocol
protocol ${id}
version 1
schema ${schema}
start record
framing record
field format u8
field ordinal varint
field body bytes
chain hash u64
`;
}

function writerGrammars() {
  const writerDoc = `dialect grammar
grammar writer.document
extension writer
use family-embed
start document

document = schema-field id-field? language-id-field? uri-field? text-field?
schema-field = "schema" "=" (TEXT | IDENT)
id-field = "id" "=" (TEXT | IDENT)
language-id-field = "language-id" "=" (TEXT | IDENT)
uri-field = "uri" "=" (TEXT | IDENT)
text-field = "text" "=" (TEXT | fence)
fence = "\`\`\`" IDENT TEXT "\`\`\`"
`;
  const writerOp = `dialect grammar
grammar writer.op
extension writer
use family-embed
start operation

operation = set-text | set-document
set-text = "set-text" "text" "=" (TEXT | fence)
set-document = "set-document" "document" "=" document-block
document-block = "{" doc-field* "}"
doc-field = schema-field | id-field | language-id-field | uri-field | text-field
schema-field = "schema" "=" (TEXT | IDENT)
id-field = "id" "=" (TEXT | IDENT)
language-id-field = "language-id" "=" (TEXT | IDENT)
uri-field = "uri" "=" (TEXT | IDENT)
text-field = "text" "=" (TEXT | fence)
fence = "\`\`\`" IDENT TEXT "\`\`\`"
`;
  const writerDiff = `dialect grammar
grammar writer.diff
extension writer
use family-embed
start diff

diff = text-field | document-field
text-field = "text" "=" (TEXT | fence)
document-field = "document" "=" document-block
document-block = "{" doc-field* "}"
doc-field = schema-field | id-field | language-id-field | uri-field | text-field
schema-field = "schema" "=" (TEXT | IDENT)
id-field = "id" "=" (TEXT | IDENT)
language-id-field = "language-id" "=" (TEXT | IDENT)
uri-field = "uri" "=" (TEXT | IDENT)
text-field = "text" "=" (TEXT | fence)
fence = "\`\`\`" IDENT TEXT "\`\`\`"
`;
  return { writerDoc, writerOp, writerDiff };
}

function rewriteDslGrammar(id, ext, family, artifactDir) {
  const artAscii = stripEmoji(ext);
  if (artAscii === "rewrite") {
    return `dialect grammar
grammar rewrite.document
extension rewrite
use family-embed
start document

document = before-field lhs-field rhs-field bindings-field layout-field
before-field = "before-fixture-json" "=" TEXT
lhs-field = "lhs-json" "=" (TEXT | fence)
rhs-field = "rhs-json" "=" (TEXT | fence)
bindings-field = "parameter-bindings" "=" map
layout-field = "rule-layout" "=" map
fence = "\`\`\`" "json" TEXT "\`\`\`"
map = "{" map-entry* "}"
map-entry = IDENT "=" (TEXT | INT | FLOAT | BOOL)
`;
  }
  const dslRs = join(artifactDir, "🗣️dsl", "🦀️component.rs");
  const dslKeys = [];
  if (existsSync(dslRs)) {
    const t = readFileSync(dslRs, "utf8");
    let m;
    const pat = /dsl\(keyword\s*=\s*"([^"]+)"/g;
    while ((m = pat.exec(t))) dslKeys.push(m[1]);
  }
  const start = "document";
  const header = `dialect grammar\ngrammar ${id}.document\nextension ${ext}\nuse family-${family}\nstart ${start}\n\n`;
  if (family === "graph") {
    const kw = dslKeys.length ? `keyword-stmt = (${dslKeys.map((k) => `"${k}"`).join(" | ")}) field*\nfield = IDENT "=" (TEXT | INT | FLOAT | BOOL)\n` : "";
    return (
      header +
      `document = ${dslKeys.length ? "keyword-stmt | " : ""}schema-field camera-block? widgets-block? layout-field? synapses-table?\nschema-field = "schema" "=" TEXT\ncamera-block = "camera" block\nwidgets-block = "widgets" block\nlayout-field = "layout" "=" map\nsynapses-table = "synapses" table\nnode = IDENT {":" IDENT}? {"@" IDENT}?\nedge-arrow = ARROW | DASHARROW | EDGEARROW | BACKARROW | "-" edge-label ARROW | "-" edge-label "-"\nedge-label = IDENT {":" IDENT}?\nwire = node edge-arrow node props?\nchain = node {ARROW node}+ | node {DASHARROW node}+\n${kw}` +
      COMMON_TABLE
    );
  }
  if (family === "sheet") {
    return (
      header +
      `document = header assign*\nheader = "semio" IDENT "v" INT\nassign = IDENT "=" (TEXT | INT | FLOAT | BOOL | UNITNUM)\nUNITNUM = FLOAT UNIT | INT UNIT\nUNIT = IDENT\n`
    );
  }
  if (family === "scene") {
    return (
      header +
      `document = schema-line layers-block assets-map?\nschema-line = "schema" "=" IDENT\nlayers-block = "layers" "{" layer* "}"\nlayer = IDENT "@" FLOAT FLOAT FLOAT? props?\nprops = "{" prop* "}"\nprop = IDENT "=" (TEXT | FLOAT | INT | BOOL)\n`
    );
  }
  if (family === "embed") {
    return (
      header +
      `document = schema-field id-field? language-id-field? uri-field? text-field?\nschema-field = "schema" "=" (TEXT | IDENT)\nid-field = "id" "=" (TEXT | IDENT)\nlanguage-id-field = "language-id" "=" (TEXT | IDENT)\nuri-field = "uri" "=" (TEXT | IDENT)\ntext-field = "text" "=" (TEXT | fence)\nfence = "\`\`\`" IDENT TEXT "\`\`\`"\n`
    );
  }
  return header + `document = schema-field field*\nschema-field = "schema" "=" (TEXT | IDENT)\nfield = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map)\nlist = "[" value* "]"\nmap = "{" assign* "}"\nassign = IDENT "=" value\nvalue = TEXT | INT | FLOAT | BOOL | list | map\n`;
}

function rewriteOpGrammar(id, ext, family, opKeys, artifactDir) {
  const start = "operation";
  const header = `dialect grammar\ngrammar ${id}.op\nextension ${ext}\nuse family-${family}\nstart ${start}\n\n`;
  if (opKeys.length === 1 && opKeys[0] === "set-state") {
    return header + `operation = "set-state" block\n${COMMON_TABLE}`;
  }
  if (opKeys.length >= 2) {
    const alts = opKeys.map((k) => k.replace(/-/g, "_") + "-op").join(" | ");
    const lines = opKeys.map((k) => {
      const alt = k.replace(/-/g, "_") + "-op";
      return `${alt} = "${k}" assign* block?`;
    });
    return header + `operation = ${alts}\n${lines.join("\n")}\n${COMMON_TABLE}`;
  }
  const dslBody = existsSync(join(artifactDir, "🗣️dsl", "📖️component.grammar.semio"))
    ? readFileSync(join(artifactDir, "🗣️dsl", "📖️component.grammar.semio"), "utf8")
    : "";
  if (family === "graph" && dslBody.includes("EDGEARROW")) {
    return header + `operation = graph-op*\ngraph-op = wire | chain | assign\nwire = node edge-arrow node props?\nnode = IDENT {":" IDENT}? {"@" IDENT}?\nedge-arrow = ARROW | DASHARROW | EDGEARROW | BACKARROW | "-" edge-label ARROW | "-" edge-label "-"\nedge-label = IDENT {":" IDENT}?\nchain = node {ARROW node}+ | node {DASHARROW node}+\n${COMMON_TABLE}`;
  }
  return header + `operation = op-line*\nop-line = IDENT assign* block?\n${COMMON_TABLE}`;
}

function rewriteDiffGrammar(id, ext, family, opKeys) {
  const start = "diff";
  const header = `dialect grammar\ngrammar ${id}.diff\nextension ${ext}\nuse family-${family}\nstart ${start}\n\n`;
  if (opKeys.length >= 2) {
    const ops = opKeys.map((k) => `"${k}"`).join(" | ");
    return header + `diff = op-line*\nop-line = (${ops}) assign* block?\n${COMMON_TABLE}`;
  }
  return header + `diff = field*\nfield = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map)\nlist = "[" value* "]"\nmap = "{" assign* "}"\nassign = IDENT "=" value\nvalue = TEXT | INT | FLOAT | BOOL | list | map\n`;
}

function stripGrammarHeader(body) {
  return body.replace(/^dialect grammar[\s\S]*?start \w+\n\n/, "");
}

function auditAll() {
  const entries = [];
  for (const plugin of listDirs(pluginsRoot)) {
    const artifactsDir = join(pluginsRoot, plugin, "🗿️artifacts");
    for (const artifact of listDirs(artifactsDir)) {
      const artifactDir = join(artifactsDir, artifact);
      const family = familyOf(plugin, artifact);
      const docSchema = inferDocSchema(artifactDir, artifact);
      const opKeys = extractOpKeywords(join(artifactDir, "🔧️op", "🦀️component.rs"));
      for (const facet of facets) {
        const facetDir = join(artifactDir, facet);
        if (!existsSync(facetDir)) continue;
        const isProto = facet === "🎒️pack" || facet === "📡️spr";
        const name = isProto ? "📡️component.protocol.semio" : "📖️component.grammar.semio";
        const path = join(facetDir, name);
        if (!existsSync(path)) continue;
        const body = readFileSync(path, "utf8");
        const rel = path.replace(repoRoot + "/", "");
        const score = isProto ? scoreProtocol(body) : scoreGrammar(body);
        const meta = { rel, path, facet, plugin, artifact, family, docSchema, opKeys, body, ...score };
        if (!isProto && facet === "🔧️op" && existsSync(join(artifactDir, "🗣️dsl", "📖️component.grammar.semio"))) {
          const dslBody = readFileSync(join(artifactDir, "🗣️dsl", "📖️component.grammar.semio"), "utf8");
          if (stripGrammarHeader(body) === stripGrammarHeader(dslBody) && opKeys.length >= 2) {
            meta.issues = [...(meta.issues || []), "op-clones-dsl"];
            meta.pass = false;
          }
        }
        entries.push(meta);
      }
    }
  }
  return entries;
}

function rewriteOffenders(entries) {
  const changed = [];
  const writers = writerGrammars();
  for (const e of entries) {
    if (e.pass) continue;
    const artifactDir = join(pluginsRoot, e.plugin, "🗿️artifacts", e.artifact);
    const ext = stripEmoji(e.artifact);
    const gramId = e.docSchema.includes(".") ? e.docSchema : `${stripEmoji(e.plugin)}.${ext}`;
    const packId = inferProtocolId(artifactDir, e.artifact, "🎒️pack");
    const sprId = inferProtocolId(artifactDir, e.artifact, "📡️spr");
    const opSchema = existsSync(join(artifactDir, "🔧️op", "🦀️component.rs")) ? `${e.docSchema}.operation` : e.docSchema;

    let next = null;
    if (e.facet === "🎒️pack") next = protocolPack(packId, e.docSchema);
    else if (e.facet === "📡️spr") next = protocolSpr(sprId, opSchema);
    else if (e.facet === "🗣️dsl") {
      if (stripEmoji(e.artifact) === "writer") next = writers.writerDoc;
      else if (stripEmoji(e.artifact) === "rewrite") {
        next = rewriteDslGrammar("rewrite", "rewrite", "embed", artifactDir);
      } else next = rewriteDslGrammar(gramId, ext, e.family, artifactDir);
    } else if (e.facet === "🔧️op") {
      if (ext === "writer" || gramId.includes("writer")) next = writers.writerOp;
      else next = rewriteOpGrammar(gramId, ext, e.family, e.opKeys, artifactDir);
    } else if (e.facet === "🔺️diff") {
      if (ext === "writer" || gramId.includes("writer")) next = writers.writerDiff;
      else next = rewriteDiffGrammar(gramId, ext, e.family, e.opKeys);
    }

    if (!next) continue;
    const normalized = next.endsWith("\n") ? next : next + "\n";
    if (readFileSync(e.path, "utf8") === normalized) continue;
    writeFileSync(e.path, normalized);
    changed.push(e.rel);
  }
  return changed;
}

function summarize(entries) {
  const grammars = entries.filter((e) => e.facet !== "🎒️pack" && e.facet !== "📡️spr");
  const protocols = entries.filter((e) => e.facet === "🎒️pack" || e.facet === "📡️spr");
  return {
    grammarTotal: grammars.length,
    grammarPass: grammars.filter((e) => e.pass).length,
    grammarFail: grammars.filter((e) => !e.pass).length,
    protocolTotal: protocols.length,
    protocolPass: protocols.filter((e) => e.pass).length,
    protocolFail: protocols.filter((e) => !e.pass).length,
    weak: entries.filter((e) => !e.pass).map((e) => ({ rel: e.rel, issues: e.issues })),
  };
}

const before = summarize(auditAll());
let totalRewritten = 0;
const rewriteLog = [];
for (let round = 0; round < 5; round++) {
  const entries = auditAll();
  const offenders = entries.filter((e) => !e.pass);
  if (offenders.length === 0) break;
  const changed = rewriteOffenders(offenders);
  totalRewritten += changed.length;
  rewriteLog.push({ round, changedCount: changed.length, files: changed });
  if (changed.length === 0) break;
}
const after = summarize(auditAll());

const report = {
  before,
  after,
  rounds: rewriteLog,
  totalFilesRewritten: totalRewritten,
  timestamp: new Date().toISOString(),
};

const ticketDir = import.meta.dir;
writeFileSync(join(ticketDir, "🧪e2e-differentiate-report.json"), JSON.stringify(report, null, 2));

const md = `# E2E differentiate specs

## Before
- Grammars: ${before.grammarPass}/${before.grammarTotal} pass (${before.grammarFail} weak)
- Protocols: ${before.protocolPass}/${before.protocolTotal} pass (${before.protocolFail} weak)

## After
- Grammars: ${after.grammarPass}/${after.grammarTotal} pass (${after.grammarFail} weak)
- Protocols: ${after.protocolPass}/${after.protocolTotal} pass (${after.protocolFail} weak)

## Files rewritten (unique passes): ${totalRewritten}

## Rounds
${rewriteLog.map((r) => `- Round ${r.round}: ${r.changedCount} files`).join("\n")}

## Still weak
${after.weak.length === 0 ? "None." : after.weak.map((w) => `- ${w.rel}: ${w.issues.join(", ")}`).join("\n")}
`;
writeFileSync(join(ticketDir, "🧪e2e-differentiate-report.md"), md);

console.log("[DEBUG] e2e-differentiate before", before.grammarPass, before.grammarTotal, before.protocolPass, before.protocolTotal);
console.log("[DEBUG] e2e-differentiate after", after.grammarPass, after.grammarTotal, after.protocolPass, after.protocolTotal);
console.log("[DEBUG] rewritten", totalRewritten);
if (after.weak.length) console.log("[DEBUG] still weak", after.weak.slice(0, 10));
