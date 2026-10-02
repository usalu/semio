#!/usr/bin/env bun
/** 🏋️ Weighs a built quiz site the way `siteArtifactProblems` does: the scripts and stylesheets the document links, gzip-compressed, every other asset beside them, and the total on disk. Usage: `bun site_bundle_weight.ts <dist>`. */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { gzipSync } from "node:zlib";

const dist = process.argv[2]!;
const walk = (directory: string): string[] => readdirSync(directory, { withFileTypes: true }).flatMap((entry) => (entry.isDirectory() ? walk(join(directory, entry.name)) : [join(directory, entry.name)]));
const files = walk(dist);
const document = files.find((file) => file.endsWith(".html"))!;
const html = readFileSync(document, "utf8");
const linked = new Set([...html.matchAll(/<(?:script|link)\b[^>]*\s(?:src|href)="\/(assets\/[^"]+)"/gu)].map((tag) => tag[1]!));
let total = 0;
for (const file of files) {
  const bytes = readFileSync(file);
  total += statSync(file).size;
  const path = file.slice(dist.length + 1).replaceAll("\\", "/");
  if (/\.(?:js|css)$/u.test(path)) process.stdout.write(`${linked.has(path) ? "linked" : "lazy  "}  ${String(bytes.length).padStart(8)} B  gzip ${String(gzipSync(bytes).length).padStart(7)} B  ${path}\n`);
}
process.stdout.write(`files ${files.length}, total ${total} B\n`);
