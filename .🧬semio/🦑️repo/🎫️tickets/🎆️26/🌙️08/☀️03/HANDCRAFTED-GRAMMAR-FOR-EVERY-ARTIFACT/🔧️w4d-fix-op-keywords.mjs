import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "/Users/ueli/Documents/semio";
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

function grammarOpDiff(id, ext, family, ops, role) {
  const start = role === "op" ? "operation" : "diff";
  const use = family ? `use ${family}\n` : "";
  return `dialect grammar
grammar ${id}.${role}
extension ${ext}
${use}start ${start}

${start} = op-line*
op-line = (${ops}) assign* block?
${common}`;
}

function architectOpDiff(role) {
  const start = role === "op" ? "operation" : "diff";
  return `dialect grammar
grammar architect.program.${role}
extension program
use family-catalog
start ${start}

${start} = json-op*
json-op = map
${common}`;
}

const gismapOps = `"add-position" | "remove-position" | "move-position" | "patch-position" | "add-route" | "remove-route" | "move-route" | "patch-route" | "add-region" | "remove-region" | "move-region" | "patch-region" | "set-document"`;
const imperativeOps = `"add" | "remove" | "move" | "patch"`;

const updates = [
  ["✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔧️op/📖️component.grammar.semio", grammarOpDiff("gis.gismap", "gismap", "family-geo", gismapOps, "op")],
  ["✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔺️diff/📖️component.grammar.semio", grammarOpDiff("gis.gismap", "gismap", "family-geo", gismapOps, "diff")],
  ["✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🔧️op/📖️component.grammar.semio", grammarOpDiff("imperative.imperative", "imperative", null, imperativeOps, "op")],
  ["✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🔺️diff/📖️component.grammar.semio", grammarOpDiff("imperative.imperative", "imperative", null, imperativeOps, "diff")],
  ["✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🔧️op/📖️component.grammar.semio", architectOpDiff("op")],
  ["✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🔺️diff/📖️component.grammar.semio", architectOpDiff("diff")],
];

let n = 0;
for (const [rel, body] of updates) {
  const path = join(root, rel);
  const next = body.trimEnd() + "\n";
  if (readFileSync(path, "utf8") !== next) {
    writeFileSync(path, next);
    n++;
    console.log("[DEBUG] fixed", rel);
  }
}
console.log("[DEBUG] keyword fixes", n);
