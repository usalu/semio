#!/usr/bin/env bun
/**
 * 🧩 Finish W4b/W4c/W4d gaps: writer artifact-specific grammars + pack/spr schema framing
 * for scene/embed plugins that still lack `schema` / `start frame|record`.
 */
import { existsSync, readdirSync, readFileSync, writeFileSync, statSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
const outLog = join(import.meta.dir, "🧪w4bcd-changed-files.txt");

/** @type {{ plugin: string, artifact: string, id: string, docSchema: string, opSchema: string }[]} */
const protocolTargets = [
  { plugin: "🖨️raster", artifact: "🖨️raster", id: "raster", docSchema: "raster.document", opSchema: "raster.document.operation" },
  { plugin: "🎞️animate", artifact: "🎬️present", id: "present", docSchema: "animate.present.deck", opSchema: "animate.present.deck.operation" },
  { plugin: "💠️lowpoly", artifact: "💠️lowpoly", id: "lowpoly", docSchema: "lowpoly.document", opSchema: "lowpoly.document.operation" },
  { plugin: "🖍️draw", artifact: "🖍️draw", id: "draw", docSchema: "draw.document", opSchema: "draw.document.operation" },
  { plugin: "📏️layout", artifact: "📏️layout", id: "layout", docSchema: "layout.fixture", opSchema: "layout.fixture.operation" },
  { plugin: "🎥️shooting", artifact: "🎥️shooting", id: "shooting", docSchema: "shooting.fixture", opSchema: "shooting.fixture.operation" },
  { plugin: "📸️remodel", artifact: "📸️remodel", id: "remodel", docSchema: "remodel.scene", opSchema: "remodel.scene.operation" },
  { plugin: "🗒️note", artifact: "🗒️note", id: "note", docSchema: "note.document", opSchema: "note.document.operation" },
  { plugin: "✒️writer", artifact: "✒️writer", id: "writer", docSchema: "writer.document", opSchema: "writer.document.operation" },
];

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

const writerDoc = `dialect grammar
grammar writer.document
extension writer
use family-embed
start document

document = field*
field = schema-field | id-field | language-id-field | uri-field | text-field
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
document-block = "{" field* "}"
field = schema-field | id-field | language-id-field | uri-field | text-field
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

diff = field*
field = text-field | document-field
text-field = "text" "=" (TEXT | fence)
document-field = "document" "=" document-block
document-block = "{" doc-field* "}"
doc-field = schema-field | id-field | language-id-field | uri-field | text-field
schema-field = "schema" "=" (TEXT | IDENT)
id-field = "id" "=" (TEXT | IDENT)
language-id-field = "language-id" "=" (TEXT | IDENT)
uri-field = "uri" "=" (TEXT | IDENT)
text-embed = "text" "=" (TEXT | fence)
fence = "\`\`\`" IDENT TEXT "\`\`\`"
`;

// fix text-embed typo - use text-field in doc-field
const writerDiffFixed = writerDiff.replace("text-embed =", "text-field =").replace(
  `doc-field = schema-field | id-field | language-id-field | uri-field | text-field
schema-field = "schema" "=" (TEXT | IDENT)
id-field = "id" "=" (TEXT | IDENT)
language-id-field = "language-id" "=" (TEXT | IDENT)
uri-field = "uri" "=" (TEXT | IDENT)
text-field = "text" "=" (TEXT | fence)
fence = "\`\`\`" IDENT TEXT "\`\`\`"
`,
  `doc-field = schema-field | id-field | language-id-field | uri-field | text-field
schema-field = "schema" "=" (TEXT | IDENT)
id-field = "id" "=" (TEXT | IDENT)
language-id-field = "language-id" "=" (TEXT | IDENT)
uri-field = "uri" "=" (TEXT | IDENT)
`
);

const changed = [];

function writeIfChanged(path, body) {
  const next = body.endsWith("\n") ? body : body + "\n";
  if (existsSync(path) && readFileSync(path, "utf8") === next) return false;
  writeFileSync(path, next);
  changed.push(path.replace(repoRoot + "/", ""));
  return true;
}

// Writer grammars
const writerRoot = join(pluginsRoot, "✒️writer/🗿️artifacts/✒️writer");
writeIfChanged(join(writerRoot, "🗣️dsl/📖️component.grammar.semio"), writerDoc);
writeIfChanged(join(writerRoot, "🔧️op/📖️component.grammar.semio"), writerOp);
writeIfChanged(join(writerRoot, "🔺️diff/📖️component.grammar.semio"), writerDiffFixed);

// Protocols for scene + writer
for (const t of protocolTargets) {
  const artRoot = join(pluginsRoot, t.plugin, "🗿️artifacts", t.artifact);
  writeIfChanged(join(artRoot, "🎒️pack/📡️component.protocol.semio"), protocolPack(t.id, t.docSchema));
  writeIfChanged(join(artRoot, "📡️spr/📡️component.protocol.semio"), protocolSpr(t.id, t.opSchema));
}

writeFileSync(outLog, changed.join("\n") + (changed.length ? "\n" : ""));
console.log(JSON.stringify({ changed: changed.length, files: changed }, null, 2));
