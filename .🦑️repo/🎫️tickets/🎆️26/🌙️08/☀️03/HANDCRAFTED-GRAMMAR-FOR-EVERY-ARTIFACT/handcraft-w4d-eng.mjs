#!/usr/bin/env bun
import { existsSync, writeFileSync, readFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");

const common = `assign = IDENT "=" (TEXT | INT | FLOAT | BOOL | list | map | qty | block)
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

const wire = `node = IDENT {":" IDENT}? {"@" IDENT}?
edge-arrow = ARROW | DASHARROW | EDGEARROW | BACKARROW | "-" edge-label ARROW | "-" edge-label "-"
edge-label = IDENT {":" IDENT}?
wire = node edge-arrow node
chain = node {ARROW node}+ | node {DASHARROW node}+
`;

function protocolPack(id, schema) {
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

function protocolSpr(id, schema) {
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

function opDiff(id, ext, family, startName, opLines, role) {
  const start = role === "op" ? "operation" : "diff";
  const grammarId = `${id}.${role}`;
  const use = family ? `use ${family}\n` : "";
  return `dialect grammar
grammar ${grammarId}
extension ${ext}
${use}start ${start}

${start} = op-line*
op-line = ${opLines}
${common}${family === "family-graph" ? wire : ""}`.replace(/\n+/g, (m, i, s) => m); // keep
}

// shared helpers for cleaner bodies
const sheetFrag = common;
const recipeFrag = common;
const catalogFrag = common;
const geoFrag = common;

const fem2dDocument = `dialect grammar
grammar fem.fem2d
extension fem2d
use family-sheet
start document

document = elements-block? analysis-block? nodes-table? regions-table? materials-table? sections-table? supports-table? load-cases-table? combinations-table?
elements-block = "elements" block
analysis-block = "analysis" block
nodes-table = "nodes" table-schema "{" row* "}"
regions-table = "regions" table-schema "{" row* "}"
materials-table = "materials" table-schema "{" row* "}"
sections-table = "sections" table-schema "{" row* "}"
supports-table = "supports" table-schema "{" row* "}"
load-cases-table = "load-cases" table-schema "{" row* "}"
combinations-table = "combinations" table-schema "{" row* "}"
element-stmt = ("beam" | "bar" | "frame" | "rod") assign*
load-stmt = ("nodal" | "udl" | "area") assign*
${sheetFrag}`;

const fem2dOps = `"set-node" | "remove-node" | "set-element" | "remove-element" | "set-material" | "remove-material" | "set-section" | "remove-section" | "set-support" | "remove-support" | "set-load-case" | "remove-load-case" | "set-region" | "remove-region" | "set-combination" | "remove-combination" | "set-analysis-settings" | "set-document"`;

const fem3dDocument = `dialect grammar
grammar fem.fem3d
extension fem3d
use family-sheet
start document

document = elements-block? analysis-block? nodes-table? materials-table? sections-table? solids-table? supports-table? load-cases-table? combinations-table?
elements-block = "elements" block
analysis-block = "analysis" block
nodes-table = "nodes" table-schema "{" row* "}"
materials-table = "materials" table-schema "{" row* "}"
sections-table = "sections" table-schema "{" row* "}"
solids-table = "solids" table-schema "{" row* "}"
supports-table = "supports" table-schema "{" row* "}"
load-cases-table = "load-cases" table-schema "{" row* "}"
combinations-table = "combinations" table-schema "{" row* "}"
element-stmt = ("beam" | "bar" | "frame" | "rod") assign*
load-stmt = ("nodal" | "udl" | "area") assign*
solid-stmt = "solid" assign*
${sheetFrag}`;

const fem3dOps = `"set-node" | "remove-node" | "set-element" | "remove-element" | "set-material" | "remove-material" | "set-section" | "remove-section" | "set-solid" | "remove-solid" | "set-support" | "remove-support" | "set-load-case" | "remove-load-case" | "set-combination" | "remove-combination" | "set-analysis-settings" | "set-document"`;

const architectTables = [
  "stakeholders","users","activities","functions","elements","quantities","relationships","adjacencies","processes","flows","access-rules","operations","equipment","resources","storage","environmental","human-factors","accessibility","privacy","safety","security","regulatory","site-context","organizational","services","infrastructure","information","communication","wayfinding","schedules","flexibility","growth","sustainability","resilience","costs","delivery","risks","conflicts","requirements","priorities","scenarios","options","decisions","validations","performance","quality","documents","changes","collaboration","analyses","reports","search-filters","status-records","workshops","surveys","issues","audit-events","templates","knowledge","benchmarks","assumptions","constraints","compliance-records","approvals","meetings","traces"
];

const architectDocument = `dialect grammar
grammar architect.program
extension architect
use family-catalog
start document

document = schema-field meta-block? project-block? governance-block? catalog-table*
schema-field = "schema" "=" (TEXT | IDENT)
meta-block = "meta" "="? record*
project-block = "project" "="? record*
governance-block = "governance" "="? record*
catalog-table = (${architectTables.map(t => `"${t}"`).join(" | ")}) table-schema "{" row* "}"
${catalogFrag}`;

const architectCollection = architectTables.flatMap(t => [`"${t}-add"`, `"${t}-remove"`, `"${t}-move"`, `"${t}-patch"`, `"${t}-set"`]);
const architectOps = [...architectCollection, `"update-meta"`, `"update-project"`, `"update-governance"`, `"set-adjacency"`, `"clear-adjacency"`, `"set-program"`].join(" | ");

const processDocument = `dialect grammar
grammar process.process3d
extension process3d
use family-recipe
start document

document = resolved-field? workshop-block stock-block steps-field
resolved-field = "resolved-up-to" "=" INT
workshop-block = "workshop" block
stock-block = "stock" block
steps-field = "steps" "=" list
machine-stmt = IDENT assign* block?
step-stmt = IDENT assign* ("cut" | "drill" | "attach" | "crosscut" | "rip" | "bore" | "dowel")? assign* block?
origin-block = "origin" block
pose-block = "pose" block
solid-stmt = ("box" | "cylinder" | "mesh") assign*
measure-stmt = ("blade-cut" | "disc-cut" | "bore-drill" | "cylinder-attach") assign*
rule-stmt = ("max" | "min") assign*
${recipeFrag}`;

const processOps = `"steps-add" | "steps-remove" | "steps-move" | "steps-patch" | "machines-add" | "machines-remove" | "machines-move" | "machines-patch" | "stock" | "cursor" | "document"`;

const playbookDocument = `dialect grammar
grammar playbook.playbook
extension playbook
use family-recipe
start document

document = header-fields steps-field
header-fields = ("schema" | "id" | "version" | "title") "=" (TEXT | IDENT | INT)
steps-field = "steps" "=" list
step-stmt = IDENT assign* block?
block-stmt = IDENT assign* ("condition" block)?
condition-stmt = ("and" | "or" | "eq" | "truthy" | "var" | "const") assign* block?
${recipeFrag}`;

const playbookOps = `"add-step" | "remove-step" | "move-step" | "add-block" | "remove-block" | "move-block" | "update-block" | "update-step" | "update-playbook"`;

const formsDocument = `dialect grammar
grammar forms.forms
extension forms
use family-recipe
start document

document = header-fields steps-field
header-fields = ("schema" | "id" | "version" | "title") "=" (TEXT | IDENT | INT)
steps-field = "steps" "=" list
step-stmt = IDENT assign* block?
block-stmt = IDENT assign* ("condition" block)?
condition-stmt = ("and" | "or" | "eq" | "truthy" | "var" | "const") assign* block?
${recipeFrag}`;

const formsOps = playbookOps;

const gismapDocument = `dialect grammar
grammar gis.gismap
extension gismap
use family-geo
start document

document = positions-table routes-table regions-table
positions-table = "positions" table-schema "{" feature-row* "}"
routes-table = "routes" table-schema "{" feature-row* "}"
regions-table = "regions" table-schema "{" feature-row* "}"
feature-row = IDENT block
feature-field = ("id" | "icon" | "kind" | "label" | "name" | "lat" | "lon" | "sourceUrl" | "points" | "ring" | "data") "=" value
${geoFrag}`;

const gismapOps = `"add-position" | "remove-position" | "move-position" | "patch-position" | "add-route" | "remove-route" | "move-route" | "patch-route" | "add-region" | "remove-region" | "move-region" | "patch-region" | "set-document"`;

const gisterrainDocument = `dialect grammar
grammar gis.gisterrain
extension gisterrain
use family-geo
start document

document = terrain-stmt origin-stmt? position-stmt*
terrain-stmt = "gisterrain" assign*
origin-stmt = "origin" assign*
position-stmt = "position" assign*
terrain-field = ("exaggeration" | "imported-features-json" | "features-json" | "lon" | "lat" | "id" | "label" | "icon") "=" value
${geoFrag}`;

const gisterrainOps = `"set-exaggeration" | "set-imported-features" | "set-document"`;

const imperativeDocument = `dialect grammar
grammar imperative.document
extension imperative
start document

document = schema-field steps-block seed-block?
schema-field = "schema" "=" TEXT
steps-block = "steps" block
seed-block = "seed" block
step-stmt = "step" TEXT assign* ("params" "=" block)? ("bodies" "=" block)?
step-kind = ("state.set" | "log.print" | "control.if" | "control.while" | "math.add") 
${common}`;

const imperativeOps = `"add" | "remove" | "move" | "patch"`;

const homeDocument = `dialect grammar
grammar space.shome
extension shome
start document

document = schema-field gen-field?
schema-field = "schema" "=" (TEXT | IDENT)
gen-field = "gen" "=" INT
${common}`;

const homeOps = `"no-operation" | "set-catalog-generation"`;

const curateDocument = `dialect grammar
grammar sourcing.curate
extension curate
use family-catalog
start document

document = stock-field curated-table
stock-field = "stock" "=" list
curated-table = "curated" table-schema "{" curated-row* "}"
curated-row = IDENT INT
stock-item = IDENT assign* geometry-stmt
geometry-stmt = ("box" | "frame" | "slab" | "mesh") assign*
stock-field-name = ("id" | "name" | "module-id" | "availability" | "typology-path") "=" value
${catalogFrag}`;

const curateOps = `"set-document" | "document"`;

function grammarOpDiff(id, ext, family, ops, role) {
  const start = role === "op" ? "operation" : "diff";
  const use = family ? `use ${family}\n` : "";
  return `dialect grammar
grammar ${id}.${role}
extension ${ext}
${use}start ${start}

${start} = op-line*
op-line = (${ops}) assign* block?
${common}${family === "family-graph" ? wire : ""}`;
}

const specs = [
  {
    plugin: "🏗️fem", artifact: "◻2d", id: "fem.fem2d", ext: "fem2d",
    schemaDoc: "fem.2d", schemaOp: "fem.2d.operation",
    grammars: {
      "🗣️dsl": fem2dDocument,
      "🔧️op": grammarOpDiff("fem.fem2d", "fem2d", "family-sheet", fem2dOps, "op"),
      "🔺️diff": grammarOpDiff("fem.fem2d", "fem2d", "family-sheet", fem2dOps, "diff"),
    },
    protocols: { pack: "2d", spr: "2d" },
  },
  {
    plugin: "🏗️fem", artifact: "️3d", id: "fem.fem3d", ext: "fem3d",
    schemaDoc: "fem.3d", schemaOp: "fem.3d.operation",
    // artifact folder emoji
    artifactDir: "️3d",
  },
];

// Fix fem3d artifact dir - use correct emoji folder name via listing
import { readdirSync } from "node:fs";
const femArts = readdirSync(join(pluginsRoot, "🏗️fem/🗿️artifacts"));
const fem2dArt = femArts.find((a) => a.includes("2d"));
const fem3dArt = femArts.find((a) => a.includes("3d"));
const processArts = readdirSync(join(pluginsRoot, "🏭️process/🗿️artifacts"));
const processArt = processArts.find((a) => a.includes("process3d"));
const archArt = readdirSync(join(pluginsRoot, "🏛️architect/🗿️artifacts"))[0];
const playbookArt = readdirSync(join(pluginsRoot, "📖️playbook/🗿️artifacts"))[0];
const gisArts = readdirSync(join(pluginsRoot, "🌍️gis/🗿️artifacts"));
const gismapArt = gisArts.find((a) => a.includes("gismap"));
const terrainArt = gisArts.find((a) => a.includes("gisterrain") || a.includes("terrain"));
const formsArt = readdirSync(join(pluginsRoot, "📋️forms/🗿️artifacts"))[0];
const impArt = readdirSync(join(pluginsRoot, "📜️imperative/🗿️artifacts"))[0];
const spaceArt = readdirSync(join(pluginsRoot, "🪐️space/🗿️artifacts"))[0];
const sourcingPlugin = readdirSync(pluginsRoot).find((n) => n.includes("sourcing"));
const curateArt = readdirSync(join(pluginsRoot, sourcingPlugin, "🗿️artifacts"))[0];

const all = [
  {
    plugin: "🏗️fem", artifact: fem2dArt, packId: "2d", sprId: "2d",
    schemaDoc: "fem.2d", schemaOp: "fem.2d.operation",
    grammars: {
      "🗣️dsl": fem2dDocument,
      "🔧️op": grammarOpDiff("fem.fem2d", "fem2d", "family-sheet", fem2dOps, "op"),
      "🔺️diff": grammarOpDiff("fem.fem2d", "fem2d", "family-sheet", fem2dOps, "diff"),
    },
  },
  {
    plugin: "🏗️fem", artifact: fem3dArt, packId: "3d", sprId: "3d",
    schemaDoc: "fem.3d", schemaOp: "fem.3d.operation",
    grammars: {
      "🗣️dsl": fem3dDocument,
      "🔧️op": grammarOpDiff("fem.fem3d", "fem3d", "family-sheet", fem3dOps, "op"),
      "🔺️diff": grammarOpDiff("fem.fem3d", "fem3d", "family-sheet", fem3dOps, "diff"),
    },
  },
  {
    plugin: "🏛️architect", artifact: archArt, packId: "program", sprId: "program",
    schemaDoc: "architect.program", schemaOp: "architect.program.operation",
    grammars: {
      "🗣️dsl": architectDocument,
      "🔧️op": grammarOpDiff("architect.program", "program", "family-catalog", architectOps, "op"),
      "🔺️diff": grammarOpDiff("architect.program", "program", "family-catalog", architectOps, "diff"),
    },
  },
  {
    plugin: "🏭️process", artifact: processArt, packId: "process3d", sprId: "process3d",
    schemaDoc: "process.3d", schemaOp: "process.3d.operation",
    grammars: {
      "🗣️dsl": processDocument,
      "🔧️op": grammarOpDiff("process.process3d", "process3d", "family-recipe", processOps, "op"),
      "🔺️diff": grammarOpDiff("process.process3d", "process3d", "family-recipe", processOps, "diff"),
    },
  },
  {
    plugin: "📖️playbook", artifact: playbookArt, packId: "playbook", sprId: "playbook",
    schemaDoc: "playbook.program", schemaOp: "playbook.program.operation",
    grammars: {
      "🗣️dsl": playbookDocument,
      "🔧️op": grammarOpDiff("playbook.playbook", "playbook", "family-recipe", playbookOps, "op"),
      "🔺️diff": grammarOpDiff("playbook.playbook", "playbook", "family-recipe", playbookOps, "diff"),
    },
  },
  {
    plugin: "🌍️gis", artifact: gismapArt, packId: "gismap", sprId: "gismap",
    schemaDoc: "gis.map", schemaOp: "gis.map.operation",
    grammars: {
      "🗣️dsl": gismapDocument,
      "🔧️op": grammarOpDiff("gis.gismap", "gismap", "family-geo", gismapOps, "op"),
      "🔺️diff": grammarOpDiff("gis.gismap", "gismap", "family-geo", gismapOps, "diff"),
    },
  },
  {
    plugin: "🌍️gis", artifact: terrainArt, packId: "gisterrain", sprId: "gisterrain",
    schemaDoc: "gis.terrain", schemaOp: "gis.terrain.operation",
    grammars: {
      "🗣️dsl": gisterrainDocument,
      "🔧️op": grammarOpDiff("gis.gisterrain", "gisterrain", "family-geo", gisterrainOps, "op"),
      "🔺️diff": grammarOpDiff("gis.gisterrain", "gisterrain", "family-geo", gisterrainOps, "diff"),
    },
  },
  {
    plugin: "📋️forms", artifact: formsArt, packId: "forms", sprId: "forms",
    schemaDoc: "forms.form", schemaOp: "forms.form.operation",
    grammars: {
      "🗣️dsl": formsDocument,
      "🔧️op": grammarOpDiff("forms.forms", "forms", "family-recipe", formsOps, "op"),
      "🔺️diff": grammarOpDiff("forms.forms", "forms", "family-recipe", formsOps, "diff"),
    },
  },
  {
    plugin: "📜️imperative", artifact: impArt, packId: "imperative", sprId: "imperative",
    schemaDoc: "imperative.document", schemaOp: "imperative.document.operation",
    grammars: {
      "🗣️dsl": imperativeDocument,
      "🔧️op": grammarOpDiff("imperative.imperative", "imperative", null, imperativeOps, "op"),
      "🔺️diff": grammarOpDiff("imperative.imperative", "imperative", null, imperativeOps, "diff"),
    },
  },
  {
    plugin: "🪐️space", artifact: spaceArt, packId: "home", sprId: "home",
    schemaDoc: "s.home", schemaOp: "s.home.operation",
    grammars: {
      "🗣️dsl": homeDocument,
      "🔧️op": grammarOpDiff("space.shome", "home", null, homeOps, "op"),
      "🔺️diff": grammarOpDiff("space.shome", "home", null, homeOps, "diff"),
    },
  },
  {
    plugin: sourcingPlugin, artifact: curateArt, packId: "curate", sprId: "curate",
    schemaDoc: "sourcing.curate/v1", schemaOp: "sourcing.curate.operation",
    grammars: {
      "🗣️dsl": curateDocument,
      "🔧️op": grammarOpDiff("sourcing.curate", "curate", "family-catalog", curateOps, "op"),
      "🔺️diff": grammarOpDiff("sourcing.curate", "curate", "family-catalog", curateOps, "diff"),
    },
  },
];

const changed = [];
for (const spec of all) {
  for (const [facet, body] of Object.entries(spec.grammars)) {
    const path = join(pluginsRoot, spec.plugin, "🗿️artifacts", spec.artifact, facet, "📖️component.grammar.semio");
    if (!existsSync(path)) {
      console.log("[DEBUG] missing grammar", path);
      continue;
    }
    const next = body.trimEnd() + "\n";
    const prev = readFileSync(path, "utf8");
    if (prev !== next) {
      writeFileSync(path, next);
      changed.push(path.slice(repoRoot.length + 1));
    }
  }
  const packPath = join(pluginsRoot, spec.plugin, "🗿️artifacts", spec.artifact, "🎒️pack", "📡️component.protocol.semio");
  const sprPath = join(pluginsRoot, spec.plugin, "🗿️artifacts", spec.artifact, "📡️spr", "📡️component.protocol.semio");
  if (existsSync(packPath)) {
    const next = protocolPack(spec.packId, spec.schemaDoc);
    const prev = readFileSync(packPath, "utf8");
    if (prev !== next) {
      writeFileSync(packPath, next);
      changed.push(packPath.slice(repoRoot.length + 1));
    }
  }
  if (existsSync(sprPath)) {
    const next = protocolSpr(spec.sprId, spec.schemaOp);
    const prev = readFileSync(sprPath, "utf8");
    if (prev !== next) {
      writeFileSync(sprPath, next);
      changed.push(sprPath.slice(repoRoot.length + 1));
    }
  }
}

console.log(`[DEBUG] w4d-eng files written=${changed.length}`);
for (const p of changed) console.log(p);
writeFileSync(join(import.meta.dir, "🧪w4d-changed-files.txt"), changed.join("\n") + "\n");
