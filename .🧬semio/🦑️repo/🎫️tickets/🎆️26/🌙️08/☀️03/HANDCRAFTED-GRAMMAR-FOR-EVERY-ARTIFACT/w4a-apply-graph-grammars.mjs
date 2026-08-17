#!/usr/bin/env bun
import { writeFileSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const repo = join(import.meta.dir, "../../../../../..");
const plugins = join(repo, "✏️s/🔌️plugins");

const wire = `wire-endpoint = IDENT {":" IDENT}? {"@" IDENT}?
edge-arrow = ARROW | DASHARROW | EDGEARROW | BACKARROW | "-" edge-label ARROW | "-" edge-label "-"
edge-label = IDENT {":" IDENT}?
wire = wire-endpoint edge-arrow wire-endpoint props?
chain = wire-endpoint {ARROW wire-endpoint}+ | wire-endpoint {DASHARROW wire-endpoint}+
props = "{" prop* "}"
prop = IDENT "=" (TEXT | INT | FLOAT | BOOL)
table = IDENT columns "{" row* "}"
columns = "[" column+ "]"
column = IDENT ":" IDENT
row = cell+
cell = TEXT | INT | FLOAT | BOOL | IDENT | wire | MAP | BLOCK
block = "{" field* "}"
field = IDENT "=" value
value = TEXT | INT | FLOAT | BOOL | wire | MAP | block | table
`;

const specs = {
  dag: {
    ext: "dag",
    schema: "dag.fixture",
    document: `document = TEXT schema-line node-body edges-table
schema-line = "schema=" IDENT
node-body = node-entry*
node-entry = "id=" IDENT field* node-kind
node-kind = "slider" field* | "select" field* | "computation" comp-meta io-table io-table | "screen" field* | "note" field* | "image" field* | "preview" field* | "action" field* | "export" field* | "cluster" comp-meta io-table io-table | "app-instance" field* io-table io-table
comp-meta = "variadic-inputs=" BOOL "variadic-outputs=" BOOL
io-table = "inputs" table | "outputs" table
edges-table = "edges" table
`,
    operation: `operation = op-line+
op-line = "nodes-add" field* | "nodes-remove" field* | "nodes-move" field* | "nodes-patch" field* | "edges-add" field* | "edges-remove" field* | "edges-move" field* | "edges-patch" field* | "set-nodes" block | "set-edges" block | "set-document" block
`,
    diff: `diff = diff-line*
diff-line = "document" block | "nodes" block | "edges" block | "set-nodes" block | "set-edges" block
`,
  },
  wires: {
    ext: "wires",
    schema: "reasoning.wires.fixture",
    document: `document = TEXT wires-block board-block
wires-block = "wires" block
board-block = "board" block
`,
    operation: `operation = op-line+
op-line = "add-node" field* | "remove-node" field* | "patch-node" field* | "add-relationship" field* | "remove-edge" field* | "replace-document" field*
`,
    diff: `diff = diff-line*
diff-line = "wires" block | "board" block
`,
  },
  sequence: {
    ext: "sequence",
    schema: "sequence.fixture",
    document: `document = TEXT schema-line steps-table edges-table
schema-line = "schema=" IDENT
steps-table = "steps" table
edges-table = "edges" table
`,
    operation: `operation = op-line+
op-line = "steps-add" field* | "steps-remove" field* | "steps-move" field* | "steps-patch" field* | "edges-add" field* | "edges-remove" field* | "edges-move" field* | "edges-patch" field*
`,
    diff: `diff = diff-line*
diff-line = "steps" block | "edges" block
`,
  },
  mathematical: {
    ext: "mathematical",
    schema: "semio.mathematical/v1",
    document: `document = TEXT graph-block geometry-block
graph-block = "graph" block
geometry-block = "geometry" block
`,
    operation: `operation = "set-graph" block | "set-geometry" block
`,
    diff: `diff = diff-field*
diff-field = "graph" block | "geometry" block
`,
  },
  jack: {
    ext: "trinity",
    schema: "trinity.graph",
    document: `document = TEXT header camera-block nodes-table edges-table
header = field*
camera-block = "camera" block
nodes-table = "nodes" table
edges-table = "edges" table
`,
    operation: `operation = op-line+
op-line = "create-node" field* | "delete-node" field* | "create-edge" field* | "delete-edge" field* | "rename" field* | "reposition" field* | "set-data-property" field* | "clear-data-property" field* | "set-fixture" block
`,
    diff: `diff = diff-line*
diff-line = "nodes" block | "edges" block | "properties" block | "geometry" block | "fixture" block
`,
  },
  rewrite: {
    ext: "rewrite",
    schema: "trinity.rewrite.rule",
    document: `document = field*
field = "before-fixture-json" "=" TEXT | "lhs-json" "=" TEXT | "rhs-json" "=" fence | "parameter-bindings" map | "rule-layout" map
fence = "\`\`\`" "json" TEXT "\`\`\`"
map = "{" map-entry* "}"
map-entry = IDENT "=" (TEXT | INT | FLOAT | BOOL)
`,
    operation: `operation = "set-state" block
`,
    diff: `diff = "next" block
`,
  },
};

const pluginArtifacts = [
  ["🕸️dag", "🕸️dag", "dag"],
  ["💡️reasoning", "🔌️wires", "wires"],
  ["🎬️sequence", "🎬️sequence", "sequence"],
  ["➗️mathematical", "➗️mathematical", "mathematical"],
  ["🔱️trinity", "🔌️jack", "jack"],
  ["🔱️trinity", "♻️rewrite", "rewrite"],
];

function grammar(id, role, ext, body) {
  const start = role === "document" ? "document" : role === "op" ? "operation" : "diff";
  const use = id === "rewrite" && role === "document" ? "family-embed" : "family-graph";
  const graphWire = id === "rewrite" && role === "document" ? "" : wire;
  return `dialect grammar
grammar ${id}.${role}
extension ${ext}
use ${use}
start ${start}

${body}
${graphWire}`.replace(/\n\n$/, "\n");
}

function packProtocol(id, schema) {
  return `dialect protocol
protocol ${id}.pack
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

function sprProtocol(id, schema) {
  return `dialect protocol
protocol ${id}.spr
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

let changed = 0;
for (const [plugin, artifact, key] of pluginArtifacts) {
  const spec = specs[key];
  const base = join(plugins, plugin, "🗿️artifacts", artifact);
  const facets = [
    ["🗣️dsl", "document", spec.document],
    ["🔧️op", "op", spec.operation],
    ["🔺️diff", "diff", spec.diff],
  ];
  for (const [facet, role, body] of facets) {
    const path = join(base, facet, "📖️component.grammar.semio");
    const next = grammar(key, role, spec.ext, body);
    const prev = existsSync(path) ? readFileSync(path, "utf8") : "";
    if (prev !== next) {
      writeFileSync(path, next);
      changed++;
    }
  }
  for (const [facet, kind, schemaSuffix] of [
    ["🎒️pack", "pack", spec.schema],
    ["📡️spr", "spr", `${key}.operation`],
  ]) {
    const path = join(base, facet, "📡️component.protocol.semio");
    const next = kind === "pack" ? packProtocol(key, schemaSuffix) : sprProtocol(key, schemaSuffix);
    const prev = existsSync(path) ? readFileSync(path, "utf8") : "";
    if (prev !== next) {
      writeFileSync(path, next);
      changed++;
    }
  }
  const tsFacets = ["🗣️dsl", "🔧️op", "🔺️diff", "🎒️pack", "📡️spr"];
  for (const facet of tsFacets) {
    const path = join(base, facet, "🟦️component.ts");
    if (!existsSync(path)) continue;
    const isPack = facet === "🎒️pack" || facet === "📡️spr";
    const facetLabel = { "🗣️dsl": "dsl", "🔧️op": "op", "🔺️diff": "diff", "🎒️pack": "pack", "📡️spr": "spr" }[facet];
    const body = isPack
      ? `/** ${key} ${facetLabel} — thin WASM encode/decode facade. */
export function encode(value: unknown): Uint8Array {
  throw new Error("wire to plugin WASM");
}
export function decode(bytes: Uint8Array): unknown {
  throw new Error("wire to plugin WASM");
}
`
      : facet === "🗣️dsl"
        ? `/** ${key} ${facetLabel} — thin WASM parse/print facade. */
export function parseDsl(text: string): unknown {
  throw new Error("wire to plugin WASM");
}
export function printDsl(value: unknown): string {
  throw new Error("wire to plugin WASM");
}
`
        : facet === "🔧️op"
          ? `/** ${key} ${facetLabel} — thin WASM op-line facade. */
export function parseOp(line: string): unknown {
  throw new Error("wire to plugin WASM");
}
export function printOp(value: unknown): string {
  throw new Error("wire to plugin WASM");
}
`
          : `/** ${key} ${facetLabel} — thin WASM diff facade. */
export function parseDiff(text: string): unknown {
  throw new Error("wire to plugin WASM");
}
export function printDiff(value: unknown): string {
  throw new Error("wire to plugin WASM");
}
`;
    const prev = readFileSync(path, "utf8");
    if (prev !== body) {
      writeFileSync(path, body);
      changed++;
    }
  }
}

console.log(`[DEBUG] w4a-apply changed=${changed}`);
