#!/usr/bin/env bun
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");

const wireFragment = `node = IDENT {":" IDENT}? {"@" IDENT}?
edge-arrow = ARROW | DASHARROW | EDGEARROW | BACKARROW | "-" edge-label ARROW | "-" edge-label "-"
edge-label = IDENT {":" IDENT}?
wire = node edge-arrow node
props = "{" prop* "}"
prop = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map)
assign = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map)
list = "[" value* "]"
map = "{" assign* "}"
value = TEXT | INT | FLOAT | BOOL | list | map
table-schema = "[" col {"," col}* "]"
col = IDENT ":" IDENT
table = table-schema "{" row* "}"
row = field+
field = IDENT "=" value
block = "{" record* "}"
record = IDENT assign* block?
`;

const flowDocument = `dialect grammar
grammar flow.document
extension flow
use family-graph
start document

document = schema-field camera-block widgets-block layout-field synapses-table
schema-field = "schema" "=" TEXT
camera-block = "camera" block
widgets-block = "widgets" block
layout-field = "layout" "=" map
synapses-table = "synapses" table
synapse-row = IDENT wire props?
${wireFragment}
widget-stmt = IDENT assign*
`;

const flowFlowOp = `dialect grammar
grammar flow.op
extension flow
use family-graph
start operation

operation = op-stmt*
op-stmt = flow-op
flow-op = IDENT assign* block?
${wireFragment}
`;

const flowFlowDiff = flowFlowOp.replace("grammar flow.op", "grammar flow.diff").replace("start operation", "start diff").replace("operation =", "diff =");

const proceduralDocument = (id, ext) => `dialect grammar
grammar ${id}.document
extension ${ext}
use family-graph
start document

document = schema-field camera-block widgets-block layout-field synapses-table generation-fields
schema-field = "schema" "=" TEXT
camera-block = "camera" block
widgets-block = "widgets" block
layout-field = "layout" "=" map
synapses-table = "synapses" table
synapse-row = IDENT wire props?
generation-fields = gen-field*
gen-field = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map | table)
generations-table = "generations" table
${wireFragment}
`;

const proceduralOp = (id, ext) => `dialect grammar
grammar ${id}.op
extension ${ext}
use family-graph
start operation

operation = op-stmt*
op-stmt = IDENT assign* block?
${wireFragment}
`;

const proceduralDiff = (id, ext) => proceduralOp(id, ext).replace("grammar " + id + ".op", "grammar " + id + ".diff").replace("start operation", "start diff").replace("operation =", "diff =");

const puzzleDocument = (id, ext) => `dialect grammar
grammar ${id}
extension ${ext}
start document

document = schema-field header-field* camera-block? meta-block? table-section*
schema-field = "schema" "=" TEXT
header-field = IDENT "=" value
camera-block = "camera" block
meta-block = "meta" block
table-section = IDENT table
${wireFragment.replace("wire = node edge-arrow node\n", "")}
`;

const puzzleOp = (id, ext) => `dialect grammar
grammar ${id}.op
extension ${ext}
start operation

operation = op-stmt*
op-stmt = IDENT assign* block?
${wireFragment.replace("wire = node edge-arrow node\n", "")}
`;

const puzzleDiff = (id, ext) => puzzleOp(id, ext).replace(".op", ".diff").replace("start operation", "start diff").replace("operation =", "diff =");

const blockDocument = (id, ext) => `dialect grammar
grammar ${id}
extension ${ext}
use family-catalog
start document

document = schema-field kind-block presentation-block? camera-block? meta-block? catalog-table*
schema-field = "schema" "=" TEXT
kind-block = IDENT block
presentation-block = "presentation" block
camera-block = IDENT block
meta-block = "meta" block
catalog-table = IDENT table
compat-row = IDENT IDENT IDENT BOOL
${wireFragment.replace("wire = node edge-arrow node\n", "")}
`;

const blockOp = (id, ext) => `dialect grammar
grammar ${id}.op
extension ${ext}
start operation

operation = op-stmt*
op-stmt = IDENT assign* block?
${wireFragment.replace("wire = node edge-arrow node\n", "")}
`;

const blockDiff = (id, ext) => blockOp(id, ext).replace(".op", ".diff").replace("start operation", "start diff").replace("operation =", "diff =");

const cadDocument = `dialect grammar
grammar cad.document
extension cad
use family-scene
start document

document = header-field* geometry-block* object-table* nodes-table
header-field = IDENT "=" value
geometry-block = IDENT "-geometry" block
object-table = IDENT table
nodes-table = "nodes" table
layer = IDENT "@" FLOAT FLOAT FLOAT? props?
${wireFragment.replace("wire = node edge-arrow node\n", "")}
`;

const cadOpDiff = (role) => `dialect grammar
grammar cad.${role}
extension cad
use family-scene
start ${role === "op" ? "operation" : "diff"}

${role} = cad-edit*
cad-edit = IDENT assign* block?
layer = IDENT "@" FLOAT FLOAT FLOAT? props?
props = "{" prop* "}"
prop = IDENT "=" (TEXT | FLOAT | INT | BOOL)
assign = IDENT "=" (TEXT | FLOAT | INT | BOOL | list | map)
list = "[" value* "]"
map = "{" assign* "}"
value = TEXT | FLOAT | INT | BOOL | list | map
block = "{" record* "}"
record = IDENT assign* block?
`;

const vcsDocument = `dialect grammar
grammar vcs.document
extension vcsdemo
start document

document = schema-field title-field counter-field notes-field status-field tags-field
schema-field = "schema" "=" TEXT
title-field = "title" "=" TEXT
counter-field = "counter" "=" INT
notes-field = "notes" "=" TEXT
status-field = "status" "=" TEXT
tags-field = "tags" "=" list
list = "[" TEXT* "]"
`;

const vcsOpDiff = (role) => `dialect grammar
grammar vcs.${role}
extension vcsdemo
start ${role === "op" ? "operation" : "diff"}

${role} = vcs-edit*
vcs-edit = "set-counter" assign | "set-title" assign | "set-notes" assign | "set-status" assign | "add-tag" assign | "remove-tag" assign
assign = IDENT "=" (TEXT | INT)
`;

const specs = [
  { plugin: "🌊️flow", artifact: "🌊️flow", grammars: { "🗣️dsl": flowDocument, "🔧️op": flowFlowOp, "🔺️diff": flowFlowDiff } },
  { plugin: "🌀️procedural", artifact: "🌀️procedural2d", grammars: { "🗣️dsl": proceduralDocument("procedural.procedural2d", "procedural2d"), "🔧️op": proceduralOp("procedural.procedural2d", "procedural2d"), "🔺️diff": proceduralDiff("procedural.procedural2d", "procedural2d") } },
  { plugin: "🌀️procedural", artifact: "🧊️procedural3d", grammars: { "🗣️dsl": proceduralDocument("procedural.procedural3d", "procedural3d"), "🔧️op": proceduralOp("procedural.procedural3d", "procedural3d"), "🔺️diff": proceduralDiff("procedural.procedural3d", "procedural3d") } },
  { plugin: "🧩️puzzle", artifact: "◻2d", grammars: { "🗣️dsl": puzzleDocument("puzzle.puzzle2d", "puzzle2d"), "🔧️op": puzzleOp("puzzle.puzzle2d", "puzzle2d"), "🔺️diff": puzzleDiff("puzzle.puzzle2d", "puzzle2d") } },
  { plugin: "🧩️puzzle", artifact: "🧊️3d", grammars: { "🗣️dsl": puzzleDocument("puzzle.puzzle3d", "puzzle3d"), "🔧️op": puzzleOp("puzzle.puzzle3d", "puzzle3d"), "🔺️diff": puzzleDiff("puzzle.puzzle3d", "puzzle3d") } },
  { plugin: "🧩️puzzle", artifact: "🖐️5d", grammars: { "🗣️dsl": puzzleDocument("puzzle.puzzle5d", "puzzle5d"), "🔧️op": puzzleOp("puzzle.puzzle5d", "puzzle5d"), "🔺️diff": puzzleDiff("puzzle.puzzle5d", "puzzle5d") } },
  { plugin: "🧱️block", artifact: "◻2d", grammars: { "🗣️dsl": blockDocument("block.block2d", "block2d"), "🔧️op": blockOp("block.block2d", "block2d"), "🔺️diff": blockDiff("block.block2d", "block2d") } },
  { plugin: "🧱️block", artifact: "🧊️3d", grammars: { "🗣️dsl": blockDocument("block.block3d", "block3d"), "🔧️op": blockOp("block.block3d", "block3d"), "🔺️diff": blockDiff("block.block3d", "block3d") } },
  { plugin: "🧱️block", artifact: "🖐️5d", grammars: { "🗣️dsl": blockDocument("block.block5d", "block5d"), "🔧️op": blockOp("block.block5d", "block5d"), "🔺️diff": blockDiff("block.block5d", "block5d") } },
  { plugin: "📐️cad", artifact: "📐️cad", grammars: { "🗣️dsl": cadDocument, "🔧️op": cadOpDiff("op"), "🔺️diff": cadOpDiff("diff") } },
  { plugin: "🌿️vcs", artifact: "🌿️vcs", grammars: { "🗣️dsl": vcsDocument, "🔧️op": vcsOpDiff("op"), "🔺️diff": vcsOpDiff("diff") } },
];

const changed = [];
for (const { plugin, artifact, grammars } of specs) {
  for (const [facet, body] of Object.entries(grammars)) {
    const path = join(pluginsRoot, plugin, "🗿️artifacts", artifact, facet, "📖️component.grammar.semio");
    if (!existsSync(path)) continue;
    const prev = readFileSync(path, "utf8");
    const next = body.trimEnd() + "\n";
    if (prev !== next) {
      writeFileSync(path, next);
      changed.push(path.slice(repoRoot.length + 1));
    }
  }
}
console.log(`[DEBUG] w4e-hot grammars written=${changed.length}`);
for (const p of changed) console.log(p);
