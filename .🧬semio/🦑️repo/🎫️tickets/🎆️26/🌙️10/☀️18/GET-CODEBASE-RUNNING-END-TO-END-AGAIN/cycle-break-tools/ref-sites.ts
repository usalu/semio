/** 🔎 Every string in the non-source reference files that names a moved, deleted or re-keyed path or specifier, with the resolved replacement. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { ROOT } from "./lib.ts";
import { deleted, dirMap, entryMap, keyMap, net, exists } from "./refs-map.ts";
export const FILES = [
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/📥️physical/🧬️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🚫️resolved-boundary-classification/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/📦️workspace-exports/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧱️manifestless-source-closure/🔣️.json",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json",
];
export type Site = { file: string; line: number; value: string; kind: "entry" | "net" | "dir" | "key" | "spec" | "deleted"; to?: string };
export function sites(): Site[] {
  const out: Site[] = [];
  const dirKeys = [...dirMap.keys()].sort((a, b) => b.length - a.length);
  for (const file of FILES) {
    const lines = readFileSync(resolve(ROOT, file), "utf8").split("\n");
    lines.forEach((text, i) => {
      for (const m of text.matchAll(/"((?:[^"\\]|\\.)*)"/gu)) {
        let v = m[1]!; const raw = v; v = v.replace(/^\{workspaceRoot\}\//u, "");
        const at = (kind: Site["kind"], to?: string) => out.push({ file, line: i + 1, value: raw, kind, to });
        if (entryMap.has(v)) { at("entry", entryMap.get(v)); continue; }
        if (net.has(v)) { at("net", net.get(v)); continue; }
        const dk = dirKeys.find(k => v === k || v.startsWith(k + "/"));
        if (dk) { at("dir", dirMap.get(dk)! + v.slice(dk.length)); continue; }
        if (keyMap.has(v)) { at("key", keyMap.get(v)); continue; }
        if (deleted.has(v) && !exists(v)) { at("deleted"); continue; }
      }
    });
  }
  return out;
}
if (import.meta.main) for (const s of sites()) console.log(s.file.slice(-48), s.line, s.kind, s.value.replace("🧰️framework/", "FW/").slice(0, 110), "=>", (s.to ?? "-").replace("🧰️framework/", "FW/").slice(0, 100));
