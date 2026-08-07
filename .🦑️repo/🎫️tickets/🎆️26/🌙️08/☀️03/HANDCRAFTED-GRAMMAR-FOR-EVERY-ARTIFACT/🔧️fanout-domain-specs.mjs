import {
  readdirSync,
  readFileSync,
  writeFileSync,
  statSync,
  existsSync,
} from "fs";
import { join, dirname, relative } from "path";
import { fileURLToPath } from "url";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = join(ticket, "../../../../../..");
const pluginsRoot = join(root, "✏️s/🔌️plugins");

const PILOTS = new Set([
  "💠️lowpoly/💠️lowpoly",
  "📕️norm/📘️en1992",
  "🕸️dag/🕸️dag",
  "📐️cad/📐️cad",
]);

const SEM_MAGIC = Buffer.from([
  0x89, 0x53, 0x45, 0x4d, 0x0d, 0x0a, 0x1a, 0x0a,
]);

const FAMILY_USE = {
  sheet: "family-sheet",
  graph: "family-graph",
  mesh: "family-scene",
  canvas: "family-scene",
  catalog: "family-catalog",
  geo: "family-geo",
  eng: "family-sheet",
  text: "family-embed",
};

const stats = {
  artifactsSeen: 0,
  artifactsSkippedPilot: 0,
  artifactsProcessed: 0,
  filesWritten: 0,
  filesMissingSkipped: 0,
  examplesPadded: 0,
};

function stripEmoji(s) {
  return s.replace(/[^\x20-\x7E]/g, "").replace(/^[^\w]+/, "");
}

function pathKey(pluginDir, artifactDir) {
  return `${pluginDir}/${artifactDir}`;
}

function familyFor(pluginDir, artifactDir) {
  const p = stripEmoji(pluginDir).toLowerCase();
  const a = stripEmoji(artifactDir).toLowerCase();
  const blob = `${p} ${a}`;
  if (p === "norm" || p.includes("norm")) return "sheet";
  if (/\b(architect|energy)\b/.test(blob) || a === "program" || a === "model")
    return "sheet";
  if (/\b(dag|wires|jack|rewrite|flow|sequence)\b/.test(blob)) return "graph";
  if (
    /\b(lowpoly|procedural|block|puzzle|cad|process|remodel|process3d)\b/.test(
      blob,
    )
  )
    return "mesh";
  if (
    /\b(draw|raster|note|layout|present|shooting|forms|playbook)\b/.test(blob)
  )
    return "canvas";
  if (
    /\b(curate|home|playground|sourcing|space|demonstrator)\b/.test(blob)
  )
    return "catalog";
  if (/\b(gis|gismap|gisterrain)\b/.test(blob)) return "geo";
  if (/\b(fem|vcs|fem2d|fem3d)\b/.test(blob)) return "eng";
  return "text";
}

function fnv1a(str) {
  let h = 0x811c9dc5;
  for (let i = 0; i < str.length; i++) {
    h ^= str.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return h >>> 0;
}

function packMagicHex(pathAscii) {
  const h = fnv1a(pathAscii);
  const b = [
    0x89,
    0x53,
    (h >>> 24) & 0xff,
    (h >>> 16) & 0xff,
    (h >>> 8) & 0xff,
    h & 0xff,
    0x0d,
    0x0a,
  ];
  return "0x" + b.map((x) => x.toString(16).padStart(2, "0")).join("");
}

function readMeta(text, facet) {
  const meta = {};
  for (const line of text.split("\n")) {
    if (facet === "grammar") {
      let m = line.match(/^grammar\s+(\S+)/);
      if (m) meta.grammarId = m[1];
      m = line.match(/^extension\s+(\S+)/);
      if (m) meta.extension = m[1];
      m = line.match(/^start\s+(\S+)/);
      if (m) meta.start = m[1];
    } else {
      let m = line.match(/^protocol\s+(\S+)/);
      if (m) meta.protocolId = m[1];
      m = line.match(/^version\s+(\d+)/);
      if (m) meta.version = m[1];
      m = line.match(/^schema\s+(\S+)/);
      if (m) meta.schema = m[1];
      m = line.match(/^start\s+(\S+)/);
      if (m) meta.start = m[1];
    }
  }
  return meta;
}

function defaultExt(pluginDir, artifactDir) {
  const a = stripEmoji(artifactDir).toLowerCase();
  const p = stripEmoji(pluginDir).toLowerCase();
  if (p === "norm") return a;
  if (p === "block" || p === "puzzle" || p === "fem") return a;
  if (p === "procedural" && a.includes("3d")) return "procedural3d";
  if (p === "procedural") return "procedural2d";
  if (p === "process") return "process3d";
  if (p === "architect") return "program";
  if (p === "animate") return "present";
  if (p === "sourcing") return "curate";
  if (p === "space") return "home";
  if (p === "reasoning") return "wires";
  if (p === "trinity" && a.includes("jack")) return "jack";
  if (p === "trinity") return "rewrite";
  return a || p;
}

function uniqueMark(pluginDir, artifactDir) {
  return `${stripEmoji(pluginDir)}.${stripEmoji(artifactDir)}`;
}

function normAssignFields(stdId) {
  const id = stdId.replace(/[^a-z0-9]/gi, "").toLowerCase();
  return [
    `"${id}-basis"`,
    `"${id}-combination"`,
    `"${id}-partial-factor"`,
    `"${id}-verdict"`,
    `"${id}-annex"`,
    `"${id}-actor"`,
    `"started"`,
    `"actor"`,
  ].join(" | ");
}

function dslBody(family, mark, ext, pluginDir, artifactDir) {
  const use = FAMILY_USE[family];
  const stdId = stripEmoji(artifactDir).toLowerCase();
  const lines = [
    `artifact-mark = "${mark}"`,
    `document = artifact-mark doc-body`,
  ];
  switch (family) {
    case "sheet": {
      const fields =
        stripEmoji(pluginDir).toLowerCase() === "norm"
          ? normAssignFields(stdId)
          : `"${mark}-title" | "${mark}-clause" | "${mark}-quantity" | "started" | "actor"`;
      lines.push(
        `doc-body = header sheet-body`,
        `header = "semio" IDENT "v" INT`,
        `sheet-body = assign*`,
        `assign = (${fields}) "=" value`,
        `value = TEXT | INT | FLOAT | BOOL | QUANTITY | clause-ref`,
        `QUANTITY = FLOAT UNIT | INT UNIT`,
        `UNIT = IDENT`,
        `clause-ref = IDENT "." IDENT`,
        `trace = IDENT "=" expr ARROW FLOAT`,
      );
      break;
    }
    case "graph":
      lines.push(
        `doc-body = schema-line graph-table`,
        `schema-line = "schema" "=" TEXT`,
        `graph-table = "edges" "{" edge* "}"`,
        `edge = wire-endpoint edge-arrow wire-endpoint edge-props?`,
        `wire-endpoint = node`,
        `node = IDENT {":" IDENT}? {"@" IDENT}?`,
        `edge-arrow = ARROW | DASHARROW | EDGEARROW | BACKARROW`,
        `edge-props = "{" edge-field* "}"`,
        `edge-field = "weight" "=" (FLOAT | INT) | "kind" "=" TEXT | "label" "=" TEXT`,
        `chain = wire-endpoint {ARROW wire-endpoint}+`,
      );
      break;
    case "mesh":
      lines.push(
        `doc-body = schema-line mesh-block`,
        `schema-line = "schema" "=" TEXT`,
        `mesh-block = "mesh" "{" layer* "}"`,
        `layer = IDENT "@" FLOAT FLOAT FLOAT? transform-block?`,
        `transform-block = "{" transform-field* "}"`,
        `transform-field = "x" "=" (FLOAT | INT) | "y" "=" (FLOAT | INT) | "z" "=" (FLOAT | INT) | "scale" "=" (FLOAT | INT)`,
        `face-loop = "face" "[" INT+ "]"`,
        `vertex-table = "vertices" table-schema "{" row* "}"`,
        `table-schema = "[" col {"," col}* "]"`,
        `col = IDENT ":" IDENT`,
        `row = field+`,
        `field = IDENT "=" (FLOAT | INT | TEXT)`,
      );
      break;
    case "canvas":
      lines.push(
        `doc-body = schema-line layers-block`,
        `schema-line = "schema" "=" TEXT`,
        `layers-block = "layers" "{" layer* "}"`,
        `layer = shape-layer | path-layer | text-layer`,
        `shape-layer = "shape" "{" canvas-field* "}"`,
        `path-layer = "path" "{" canvas-field* "}"`,
        `text-layer = "text" "{" canvas-field* "}"`,
        `canvas-field = "id" "=" IDENT | "x" "=" FLOAT | "y" "=" FLOAT | "fill" "=" TEXT | "stroke" "=" TEXT | "opacity" "=" FLOAT`,
        `paint-block = "{" paint-field* "}"`,
        `paint-field = "color" "=" TEXT | "width" "=" FLOAT | "blend" "=" IDENT`,
      );
      break;
    case "catalog":
      lines.push(
        `doc-body = stock-field curated-block`,
        `stock-field = "stock" "=" list`,
        `list = "[" stock-line* "]"`,
        `stock-line = slash-path IDENT TEXT`,
        `slash-path = IDENT`,
        `curated-block = "curated" table-schema "{" curated-row* "}"`,
        `curated-row = IDENT INT`,
        `table-schema = "[" col {"," col}* "]"`,
        `col = IDENT ":" IDENT`,
        `compat = IDENT DASHARROW IDENT`,
      );
      break;
    case "geo":
      lines.push(
        `doc-body = crs-line geo-body`,
        `crs-line = "crs" "=" IDENT`,
        `geo-body = "features" "{" geo-feature* "}"`,
        `geo-feature = "point" FLOAT FLOAT FLOAT? | geo-field`,
        `geo-field = "lat" "=" FLOAT | "lon" "=" FLOAT | "alt" "=" FLOAT | "tile" "=" IDENT`,
        `bbox = "bbox" FLOAT FLOAT FLOAT FLOAT`,
        `point = FLOAT FLOAT FLOAT?`,
      );
      break;
    case "eng":
      lines.push(
        `doc-body = eng-header eng-tables`,
        `eng-header = "model" "=" IDENT`,
        `eng-tables = nodes-table? elements-table? loads-table?`,
        `nodes-table = "nodes" table-schema "{" row* "}"`,
        `elements-table = "elements" table-schema "{" row* "}"`,
        `loads-table = "loads" table-schema "{" row* "}"`,
        `table-schema = "[" col {"," col}* "]"`,
        `col = IDENT ":" IDENT`,
        `row = eng-node*`,
        `eng-node = "node" "=" INT | "element" "=" IDENT | "load" "=" IDENT | "support" "=" IDENT | "commit" "=" TEXT`,
        `eng-record = eng-node*`,
      );
      break;
    default:
      lines.push(
        `doc-body = schema-line body-fields`,
        `schema-line = "schema" "=" TEXT`,
        `body-fields = text-field? embed-field?`,
        `text-field = "text" "=" TEXT`,
        `embed-field = "embed" "=" fence`,
        `fence = "\`\`\`" IDENT TEXT "\`\`\`"`,
        `embed-stmt = "embed" "=" fence`,
      );
  }
  return { use, body: lines.join("\n") };
}

function opBody(family, mark, ext, pluginDir, artifactDir) {
  const use = FAMILY_USE[family];
  const stdId = stripEmoji(artifactDir).toLowerCase();
  const lines = [`artifact-mark = "${mark}-op"`];
  switch (family) {
    case "sheet": {
      const fields =
        stripEmoji(pluginDir).toLowerCase() === "norm"
          ? normAssignFields(stdId)
          : `"${mark}-title" | "${mark}-clause" | "${mark}-quantity"`;
      lines.push(
        `operation = artifact-mark op-stmt+`,
        `op-stmt = "set-field" field+ | "patch-field" field+ | "remove-field" field+`,
        `field = (${fields}) "=" value`,
        `value = TEXT | INT | FLOAT | BOOL | QUANTITY`,
        `QUANTITY = FLOAT UNIT | INT UNIT`,
        `UNIT = IDENT`,
      );
      break;
    }
    case "graph":
      lines.push(
        `operation = artifact-mark graph-op+`,
        `graph-op = "add-node" node-field+ | "set-port" port-field+ | "wire-edge" edge-op | "unlink-edge" edge-op | "patch-layout" layout-field+`,
        `node-field = "id" "=" IDENT | "kind" "=" IDENT`,
        `port-field = "node" "=" IDENT | "port" "=" IDENT | "dir" "=" IDENT`,
        `edge-op = "from" "=" IDENT "to" "=" IDENT`,
        `layout-field = "x" "=" FLOAT | "y" "=" FLOAT`,
        `edge = wire-endpoint edge-arrow wire-endpoint`,
        `wire-endpoint = IDENT {":" IDENT}?`,
        `edge-arrow = ARROW | DASHARROW | EDGEARROW`,
      );
      break;
    case "mesh":
      lines.push(
        `operation = artifact-mark mesh-op+`,
        `mesh-op = "add-vertex" vtx-field+ | "set-face" face-field+ | "transform-mesh" xfm-field+ | "merge-solid" merge-field+`,
        `vtx-field = "x" "=" FLOAT | "y" "=" FLOAT | "z" "=" FLOAT`,
        `face-field = "index" "=" INT | "loop" "=" list`,
        `xfm-field = "axis" "=" IDENT | "angle" "=" FLOAT`,
        `merge-field = "target" "=" IDENT | "source" "=" IDENT`,
        `list = "[" INT* "]"`,
        `layer = IDENT "@" FLOAT FLOAT FLOAT?`,
      );
      break;
    case "canvas":
      lines.push(
        `operation = artifact-mark canvas-op+`,
        `canvas-op = "add-layer" layer-field+ | "set-stroke" stroke-field+ | "move-layer" move-field+ | "set-fill" fill-field+`,
        `layer-field = "id" "=" IDENT | "kind" "=" IDENT`,
        `stroke-field = "color" "=" TEXT | "width" "=" FLOAT`,
        `move-field = "id" "=" IDENT | "x" "=" FLOAT | "y" "=" FLOAT`,
        `fill-field = "id" "=" IDENT | "fill" "=" TEXT`,
        `paint-field = "opacity" "=" FLOAT | "blend" "=" IDENT`,
      );
      break;
    case "catalog":
      lines.push(
        `operation = artifact-mark catalog-op+`,
        `catalog-op = "add-stock" stock-op+ | "set-typology" typo-field+ | "curate-item" curate-field+`,
        `stock-op = "path" "=" IDENT | "name" "=" TEXT`,
        `typo-field = "path" "=" IDENT | "parent" "=" IDENT`,
        `curate-field = "id" "=" IDENT | "count" "=" INT`,
        `stock = slash-path IDENT TEXT`,
        `slash-path = IDENT`,
      );
      break;
    case "geo":
      lines.push(
        `operation = artifact-mark geo-op+`,
        `geo-op = "set-crs" crs-field+ | "add-point" point-field+ | "tile-ref" tile-field+`,
        `crs-field = "crs" "=" IDENT`,
        `point-field = "lat" "=" FLOAT | "lon" "=" FLOAT | "alt" "=" FLOAT`,
        `tile-field = "z" "=" INT | "x" "=" INT | "y" "=" INT`,
        `point = FLOAT FLOAT FLOAT?`,
      );
      break;
    case "eng":
      lines.push(
        `operation = artifact-mark eng-op+`,
        `eng-op = "add-node" eng-field+ | "set-load" eng-field+ | "set-support" eng-field+ | "commit-step" eng-field+`,
        `eng-field = "node" "=" INT | "element" "=" IDENT | "load" "=" IDENT | "dof" "=" IDENT | "commit" "=" TEXT`,
        `eng-node = "node" "=" INT | "element" "=" IDENT`,
        `eng-record = eng-node*`,
      );
      break;
    default:
      lines.push(
        `operation = artifact-mark text-op+`,
        `text-op = "insert-text" text-field+ | "replace-range" range-field+ | "set-fence" fence-field+`,
        `text-field = "text" "=" TEXT`,
        `range-field = "start" "=" INT | "end" "=" INT`,
        `fence-field = "lang" "=" IDENT | "body" "=" TEXT`,
        `fence = "\`\`\`" IDENT TEXT "\`\`\`"`,
      );
  }
  return { use, body: lines.join("\n") };
}

function diffBody(family, mark, ext, pluginDir, artifactDir) {
  const use = FAMILY_USE[family];
  const stdId = stripEmoji(artifactDir).toLowerCase();
  const lines = [`artifact-mark = "${mark}-diff"`];
  switch (family) {
    case "sheet": {
      const fields =
        stripEmoji(pluginDir).toLowerCase() === "norm"
          ? normAssignFields(stdId)
          : `"${mark}-title" | "${mark}-clause"`;
      lines.push(
        `diff = artifact-mark change+`,
        `change = "set" | "unset" | "replace"`,
        `change-field = change (${fields}) ("=" value)?`,
        `value = TEXT | INT | FLOAT | BOOL | QUANTITY`,
        `QUANTITY = FLOAT UNIT | INT UNIT`,
        `UNIT = IDENT`,
      );
      break;
    }
    case "graph":
      lines.push(
        `diff = artifact-mark graph-change+`,
        `graph-change = "add-node" IDENT | "remove-node" IDENT | "rewire" "from" "=" IDENT "to" "=" IDENT`,
        `edge = wire-endpoint edge-arrow wire-endpoint`,
        `wire-endpoint = IDENT`,
        `edge-arrow = ARROW | EDGEARROW`,
      );
      break;
    case "mesh":
      lines.push(
        `diff = artifact-mark mesh-change+`,
        `mesh-change = "add-vertex" vtx+ | "drop-face" INT | "transform" xfm+`,
        `vtx = "x" "=" FLOAT | "y" "=" FLOAT | "z" "=" FLOAT`,
        `xfm = "axis" "=" IDENT | "delta" "=" FLOAT`,
        `layer = IDENT "@" FLOAT FLOAT`,
      );
      break;
    case "canvas":
      lines.push(
        `diff = artifact-mark canvas-change+`,
        `canvas-change = "add-layer" IDENT | "remove-layer" IDENT | "move" "id" "=" IDENT "dx" "=" FLOAT "dy" "=" FLOAT`,
        `paint-field = "stroke" "=" TEXT | "fill" "=" TEXT`,
      );
      break;
    case "catalog":
      lines.push(
        `diff = artifact-mark catalog-change+`,
        `catalog-change = "stock-add" IDENT | "stock-remove" IDENT | "curate" IDENT INT`,
        `stock = slash-path IDENT TEXT`,
        `slash-path = IDENT`,
      );
      break;
    case "geo":
      lines.push(
        `diff = artifact-mark geo-change+`,
        `geo-change = "crs" "=" IDENT | "point-add" FLOAT FLOAT | "tile" INT INT INT`,
        `point = FLOAT FLOAT FLOAT?`,
      );
      break;
    case "eng":
      lines.push(
        `diff = artifact-mark eng-change+`,
        `eng-change = "node-add" INT | "load-set" IDENT | "support-set" IDENT | "commit" TEXT`,
        `eng-node = "node" "=" INT | "load" "=" IDENT`,
      );
      break;
    default:
      lines.push(
        `diff = artifact-mark text-change+`,
        `text-change = "insert" INT TEXT | "delete" INT INT | "fence" IDENT TEXT`,
        `fence = "\`\`\`" IDENT TEXT "\`\`\`"`,
      );
  }
  return { use, body: lines.join("\n") };
}

function packBody(family, mark, magic, meta, schema) {
  const seg =
    family === "graph"
      ? "segment graph-node u8\nsegment graph-edge u8"
      : family === "mesh"
        ? "segment mesh-chunk u8\nsegment mesh-xfm u8"
        : family === "canvas"
          ? "segment canvas-layer u8\nsegment canvas-paint u8"
          : family === "catalog"
            ? "segment catalog-stock u8\nsegment catalog-curate u8"
            : family === "geo"
              ? "segment geo-point u8\nsegment geo-tile u8"
              : family === "eng"
                ? "segment eng-node u8\nsegment eng-load u8"
                : family === "sheet"
                  ? "segment sheet-clause u8\nsegment sheet-qty u8"
                  : "segment text-chunk u8\nsegment text-fence u8";
  return `dialect protocol
protocol ${meta.protocolId ?? `${schema}.pack`}
version ${meta.version ?? "1"}
schema ${meta.schema ?? schema}
start ${meta.start ?? "frame"}
framing magic ${magic}
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field domain_tag u32
field header_crc32 u32
${seg}
segment flags u8
segment payload varint bytes
footer fixed 64
field artifact_mark utf8
field body_crc32 u32
`;
}

function sprBody(family, mark, meta, schema, ext) {
  const stdId = ext.replace(/[^a-z0-9]/gi, "").toLowerCase();
  const rec = (name, tag) =>
    `record ${name} tag ${tag}\nfield format u8\nfield ordinal varint\nfield body bytes`;
  let r1;
  let r2;
  let r3;
  switch (family) {
    case "graph":
      r1 = rec(`${mark}-add-node`, 1);
      r2 = rec(`${mark}-wire-edge`, 2);
      r3 = rec(`${mark}-patch-layout`, 3);
      break;
    case "mesh":
      r1 = rec(`${mark}-add-vertex`, 1);
      r2 = rec(`${mark}-set-face`, 2);
      r3 = rec(`${mark}-transform-mesh`, 3);
      break;
    case "canvas":
      r1 = rec(`${mark}-add-layer`, 1);
      r2 = rec(`${mark}-set-stroke`, 2);
      r3 = rec(`${mark}-move-layer`, 3);
      break;
    case "catalog":
      r1 = rec(`${mark}-add-stock`, 1);
      r2 = rec(`${mark}-set-typology`, 2);
      r3 = rec(`${mark}-curate-item`, 3);
      break;
    case "geo":
      r1 = rec(`${mark}-set-crs`, 1);
      r2 = rec(`${mark}-add-point`, 2);
      r3 = rec(`${mark}-tile-ref`, 3);
      break;
    case "eng":
      r1 = rec(`${mark}-add-node`, 1);
      r2 = rec(`${mark}-set-load`, 2);
      r3 = rec(`${mark}-commit-step`, 3);
      break;
    case "sheet":
      r1 = rec(`${stdId}-set-field`, 1);
      r2 = rec(`${stdId}-patch-field`, 2);
      r3 = rec(`${stdId}-remove-field`, 3);
      break;
    default:
      r1 = rec(`${mark}-insert-text`, 1);
      r2 = rec(`${mark}-replace-range`, 2);
      r3 = rec(`${mark}-set-fence`, 3);
  }
  return `dialect protocol
protocol ${meta.protocolId ?? `${schema}.spr`}
version ${meta.version ?? "1"}
schema ${meta.schema ?? `${schema}.operation`}
start ${meta.start ?? "record"}
framing record
${r1}
${r2}
${r3}
chain hash u64
`;
}

function grammarFile(
  facet,
  startName,
  grammarId,
  extension,
  use,
  body,
) {
  return `dialect grammar
grammar ${grammarId}
extension ${extension}
use ${use}
start ${startName}

${body}
`;
}

function writeIfExists(artifactRoot, relPath, content) {
  const p = join(artifactRoot, relPath);
  if (!existsSync(p)) {
    stats.filesMissingSkipped++;
    return false;
  }
  writeFileSync(p, content.endsWith("\n") ? content : content + "\n");
  stats.filesWritten++;
  return true;
}

function padExamples(artifactRoot, pluginDir, artifactDir) {
  const examples = join(artifactRoot, "📚️examples");
  if (!existsSync(examples)) return;
  const plugin = stripEmoji(pluginDir);
  const artifact = stripEmoji(artifactDir);
  const walk = (dir) => {
    for (const e of readdirSync(dir)) {
      const p = join(dir, e);
      if (statSync(p).isDirectory()) {
        walk(p);
        continue;
      }
      if (!e.endsWith(".pack.semio") && !e.endsWith(".spr.semio")) continue;
      const size = statSync(p).size;
      if (size > 64) continue;
      const kind = e.endsWith(".spr.semio") ? "spr" : "pack";
      const token = `${plugin}.${artifact}.${kind} v1`;
      const tokenBuf = Buffer.from(token, "utf8");
      const out = Buffer.alloc(SEM_MAGIC.length + 4 + tokenBuf.length + 128);
      SEM_MAGIC.copy(out, 0);
      out.writeUInt32LE(tokenBuf.length, SEM_MAGIC.length);
      tokenBuf.copy(out, SEM_MAGIC.length + 4);
      writeFileSync(p, out);
      stats.examplesPadded++;
    }
  };
  walk(examples);
}

function processArtifact(pluginDir, artifactDir) {
  const key = pathKey(pluginDir, artifactDir);
  stats.artifactsSeen++;
  if (PILOTS.has(key)) {
    stats.artifactsSkippedPilot++;
    return;
  }
  const artifactRoot = join(
    pluginsRoot,
    pluginDir,
    "🗿️artifacts",
    artifactDir,
  );
  const dslDir = join(artifactRoot, "🗣️dsl");
  if (!existsSync(dslDir)) return;

  const family = familyFor(pluginDir, artifactDir);
  const mark = uniqueMark(pluginDir, artifactDir);
  const ext = defaultExt(pluginDir, artifactDir);
  const pathAscii = relative(root, artifactRoot);
  const magic = packMagicHex(pathAscii);
  const schemaBase =
    stripEmoji(pluginDir).toLowerCase() === "block" ||
    stripEmoji(pluginDir).toLowerCase() === "puzzle" ||
    stripEmoji(pluginDir).toLowerCase() === "fem"
      ? `${stripEmoji(pluginDir).toLowerCase()}.${ext}`
      : ext;

  const dslPath = join(artifactRoot, "🗣️dsl/📖️component.grammar.semio");
  const opPath = join(artifactRoot, "🔧️op/📖️component.grammar.semio");
  const diffPath = join(artifactRoot, "🔺️diff/📖️component.grammar.semio");
  const packPath = join(artifactRoot, "🎒️pack/📡️component.protocol.semio");
  const sprPath = join(artifactRoot, "📡️spr/📡️component.protocol.semio");

  const dslMeta = existsSync(dslPath)
    ? readMeta(readFileSync(dslPath, "utf8"), "grammar")
    : {};
  const opMeta = existsSync(opPath)
    ? readMeta(readFileSync(opPath, "utf8"), "grammar")
    : {};
  const diffMeta = existsSync(diffPath)
    ? readMeta(readFileSync(diffPath, "utf8"), "grammar")
    : {};
  const packMeta = existsSync(packPath)
    ? readMeta(readFileSync(packPath, "utf8"), "protocol")
    : {};
  const sprMeta = existsSync(sprPath)
    ? readMeta(readFileSync(sprPath, "utf8"), "protocol")
    : {};

  const dsl = dslBody(family, mark, ext, pluginDir, artifactDir);
  writeIfExists(
    artifactRoot,
    "🗣️dsl/📖️component.grammar.semio",
    grammarFile(
      "dsl",
      dslMeta.start ?? "document",
      dslMeta.grammarId ?? `${ext}.document`,
      dslMeta.extension ?? ext,
      dsl.use,
      dsl.body,
    ),
  );

  const op = opBody(family, mark, ext, pluginDir, artifactDir);
  writeIfExists(
    artifactRoot,
    "🔧️op/📖️component.grammar.semio",
    grammarFile(
      "op",
      opMeta.start ?? "operation",
      opMeta.grammarId ?? `${ext}.op`,
      opMeta.extension ?? ext,
      op.use,
      op.body,
    ),
  );

  const diff = diffBody(family, mark, ext, pluginDir, artifactDir);
  writeIfExists(
    artifactRoot,
    "🔺️diff/📖️component.grammar.semio",
    grammarFile(
      "diff",
      diffMeta.start ?? "diff",
      diffMeta.grammarId ?? `${ext}.diff`,
      diffMeta.extension ?? ext,
      diff.use,
      diff.body,
    ),
  );

  writeIfExists(
    artifactRoot,
    "🎒️pack/📡️component.protocol.semio",
    packBody(family, mark, magic, packMeta, packMeta.schema ?? schemaBase),
  );

  writeIfExists(
    artifactRoot,
    "📡️spr/📡️component.protocol.semio",
    sprBody(
      family,
      mark,
      sprMeta,
      sprMeta.schema ?? `${schemaBase}.operation`,
      ext,
    ),
  );

  padExamples(artifactRoot, pluginDir, artifactDir);
  stats.artifactsProcessed++;
}

for (const pluginDir of readdirSync(pluginsRoot)) {
  const pluginPath = join(pluginsRoot, pluginDir);
  if (!statSync(pluginPath).isDirectory()) continue;
  const artifactsPath = join(pluginPath, "🗿️artifacts");
  if (!existsSync(artifactsPath)) continue;
  for (const artifactDir of readdirSync(artifactsPath)) {
    const artifactPath = join(artifactsPath, artifactDir);
    if (!statSync(artifactPath).isDirectory()) continue;
    processArtifact(pluginDir, artifactDir);
  }
}

const summary = {
  ...stats,
  ranAt: new Date().toISOString(),
};
console.log(JSON.stringify(summary, null, 2));

const progressPath = join(ticket, "progress-v2.md");
const prev = existsSync(progressPath)
  ? readFileSync(progressPath, "utf8")
  : "# Progress v2\n\n";
const line = `- **W5 fan-out (${summary.ranAt.slice(0, 10)}):** processed=${stats.artifactsProcessed} pilots_skipped=${stats.artifactsSkippedPilot} files_written=${stats.filesWritten} files_missing=${stats.filesMissingSkipped} examples_padded=${stats.examplesPadded}\n`;
writeFileSync(progressPath, prev.trimEnd() + "\n" + line);
