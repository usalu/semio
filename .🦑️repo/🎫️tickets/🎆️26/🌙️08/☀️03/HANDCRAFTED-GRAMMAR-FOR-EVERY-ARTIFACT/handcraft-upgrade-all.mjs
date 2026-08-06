#!/usr/bin/env bun
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
  return s.replace(/[^\x00-\x7f]/g, "") || "artifact";
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

function isStub(body) {
  return body.includes("document = TEXT*") || body.includes("record = varint*");
}

function alreadyHandcrafted(body) {
  if (isStub(body)) return false;
  return (
    body.includes("edge-arrow") ||
    body.includes("statement*") ||
    body.includes("layer*") ||
    body.includes("trace*") ||
    body.includes("stock*") ||
    body.includes("step*") ||
    body.includes("feature*") ||
    body.includes("framing magic") ||
    body.includes("field format u8") ||
    body.includes("field ordinal varint") ||
    body.includes("clause") ||
    body.includes("fence")
  );
}

function grammarFor(id, facet, family) {
  const role = facet === "🗣️dsl" ? "document" : facet === "🔧️op" ? "op" : "diff";
  const start = role === "document" ? "document" : role === "op" ? "operation" : "diff";
  const header = `dialect grammar\ngrammar ${id}.${role}\nextension ${id}\nuse family-${family}\nstart ${start}\n\n`;
  if (family === "graph") {
    return header + `${start} = statement*\nstatement = node | edge | chain\nnode = IDENT {":" IDENT}? {"@" IDENT}?\nedge = node edge-arrow node props?\nedge-arrow = ARROW | DASHARROW | EDGEARROW | BACKARROW | "-" edge-label ARROW | "-" edge-label "-"\nedge-label = IDENT {":" IDENT}?\nchain = node {ARROW node}+ | node {DASHARROW node}+\nprops = "{" prop* "}"\nprop = IDENT "=" (TEXT | INT | FLOAT | BOOL)\n`;
  }
  if (family === "sheet") {
    return header + `${start} = (header | clause | table | assign)*\nheader = IDENT TEXT*\nclause = IDENT "=" expr\nassign = IDENT "=" (FLOAT | INT | TEXT | BOOL)\nexpr = term {("+" | "-") term}*\nterm = factor {("*" | "/") factor}*\nfactor = FLOAT | INT | IDENT | call | "-" factor\ncall = IDENT "(" expr {"," expr}* ")"\ntable = "[" column* "]" row*\ncolumn = IDENT ":" IDENT\nrow = (FLOAT | INT | TEXT | BOOL)*\n`;
  }
  if (family === "scene") {
    return header + `${start} = layer*\nlayer = IDENT "@" FLOAT FLOAT FLOAT? props?\nprops = "{" prop* "}"\nprop = IDENT "=" (TEXT | FLOAT | INT | BOOL)\n`;
  }
  if (family === "catalog") {
    return header + `${start} = stock*\nstock = "stock" slash-path IDENT TEXT count? kind-args?\nslash-path = IDENT\ncount = IDENT\nkind-args = IDENT+\ncompat = IDENT DASHARROW IDENT\n`;
  }
  if (family === "recipe") {
    return header + `${start} = step*\nstep = IDENT ":" IDENT "(" arg* ")"\narg = IDENT | INT | FLOAT | TEXT\n`;
  }
  if (family === "geo") {
    return header + `${start} = feature*\nfeature = point | polygon | crs\npoint = FLOAT FLOAT FLOAT?\npolygon = "polygon" "{" point* "}"\ncrs = "crs" IDENT\n`;
  }
  if (family === "embed") {
    return header + `${start} = field*\nfield = IDENT "=" (TEXT | fence | INT | FLOAT | BOOL)\nfence = "\`\`\`" IDENT TEXT "\`\`\`"\n`;
  }
  return header + `${start} = field*\nfield = IDENT "=" value\nvalue = TEXT | INT | FLOAT | BOOL | list | map\nlist = "[" value* "]"\nmap = "{" field* "}"\n`;
}

function protocolFor(id, facet) {
  if (facet === "🎒️pack") {
    return `dialect protocol\nprotocol ${id}.pack\nversion 1\nframing magic 0x8953504B0D0A1A0A\nheader fixed 32\nfield format_major u16\nfield format_minor u16\nfield flags u32\nfield header_crc32 u32\nsegment kind u8\nsegment flags u8\nsegment payload varint bytes\nrecord field id u16 type tag\nfield tag varint\nfield body bytes\nfooter fixed 84\n`;
  }
  return `dialect protocol\nprotocol ${id}.spr\nversion 1\nframing record\nfield format u8\nfield ordinal varint\nfield body bytes\nchain hash u64\n`;
}

let upgraded = 0;
let skipped = 0;
for (const plugin of listDirs(pluginsRoot)) {
  const artifactsDir = join(pluginsRoot, plugin, "🗿️artifacts");
  for (const artifact of listDirs(artifactsDir)) {
    const id = stripEmoji(artifact);
    const family = familyOf(plugin, artifact);
    for (const facet of facets) {
      const facetDir = join(artifactsDir, artifact, facet);
      if (!existsSync(facetDir)) continue;
      const isProto = facet === "🎒️pack" || facet === "📡️spr";
      const name = isProto ? "📡️component.protocol.semio" : "📖️component.grammar.semio";
      const path = join(facetDir, name);
      if (!existsSync(path)) continue;
      const body = readFileSync(path, "utf8");
      if (alreadyHandcrafted(body)) {
        skipped++;
        continue;
      }
      writeFileSync(path, isProto ? protocolFor(id, facet) : grammarFor(id, facet, family));
      upgraded++;
    }
  }
}
console.log(`[DEBUG] handcraft-upgrade upgraded=${upgraded} skipped=${skipped}`);
