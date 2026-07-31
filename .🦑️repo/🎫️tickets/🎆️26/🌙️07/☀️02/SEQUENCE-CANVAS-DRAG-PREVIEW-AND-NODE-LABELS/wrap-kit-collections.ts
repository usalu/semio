#!/usr/bin/env bun
import fs from "node:fs";
import path from "node:path";

const HASH = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
const KIT_COLLECTION_KEYS = new Set([
  "typologies",
  "types",
  "designs",
  "families",
  "files",
  "folders",
  "pieces",
  "connections",
  "ports",
  "connectors",
  "representations",
  "compatiblePorts",
  "qualities",
  "authors",
  "concepts",
  "tags",
  "attributes",
  "layers",
  "groups",
  "stats",
  "benchmarks",
  "props",
]);

function wrapCollection(value: unknown): unknown {
  if (Array.isArray(value)) {
    return { hash: HASH, items: value.map((entry) => annotateValue(entry)) };
  }
  return annotateValue(value);
}

function annotateValue(value: unknown): unknown {
  if (value == null || typeof value !== "object") return value;
  if (Array.isArray(value)) return wrapCollection(value);
  const row = { ...(value as Record<string, unknown>) };
  if (typeof row.id === "string" && row.hash === undefined) row.hash = HASH;
  for (const [key, child] of Object.entries(row)) {
    if (KIT_COLLECTION_KEYS.has(key) && Array.isArray(child)) {
      row[key] = wrapCollection(child);
    } else if (child != null && typeof child === "object" && !Array.isArray(child) && KIT_COLLECTION_KEYS.has(key)) {
      row[key] = annotateValue(child);
    } else if (child != null && typeof child === "object") {
      row[key] = annotateValue(child);
    }
  }
  return row;
}

function convertFile(filePath: string): void {
  const raw = JSON.parse(fs.readFileSync(filePath, "utf8")) as unknown;
  const next = annotateValue(raw);
  fs.writeFileSync(filePath, `${JSON.stringify(next, null, 2)}\n`);
  console.log(`[DEBUG] wrapped collections in ${filePath}`);
}

const root = path.resolve(import.meta.dir, "../../../../../../");
const targets = [
  "compose/fixture/metabolism.shallow.kit.compose.json",
  "compose/fixture/nakagin-capsule-tower.filtered.kit.compose.json",
  "compose/fixture/synthetic-find-replaceable.kit.compose.json",
  "compose/fixture/validate-kit-diff.cases.compose.json",
  "compose/fixture/metabolism.kit.diffed.compose.json",
];

for (const rel of targets) {
  convertFile(path.join(root, rel));
}
