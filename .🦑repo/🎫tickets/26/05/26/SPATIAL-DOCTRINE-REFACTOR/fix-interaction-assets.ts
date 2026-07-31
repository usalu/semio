/** One-off: drop legacy surface/part accept and surface.resolveFaces from extension interaction JSON. */
import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";

function walk(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    const p = path.join(dir, name);
    if (statSync(p).isDirectory()) walk(p, out);
    else if (name.endsWith(".json")) out.push(p);
  }
  return out;
}

const root = "c:/git/compose/spatial/assets/extension";
const interactionRoot = path.join(root, "builtin", "interaction");

for (const file of walk(interactionRoot)) {
  let text = readFileSync(file, "utf8");
  const before = text;
  text = text.replace(/^\s*"surface",?\r?\n/gm, "");
  text = text.replace(/^\s*"part",?\r?\n/gm, "");
  text = text.replace(/^\s*"volume",?\r?\n/gm, "");
  text = text.replace(/"surface\.resolveFaces"/g, '"face.resolveIds"');
  text = text.replace(/"kind": "surface\.resolveFaces"/g, '"kind": "face.resolveIds"');
  text = text.replace(/"geometryEntityKind": "surface"/g, '"geometryEntityKind": "face"');
  text = text.replace(/,(\s*[\r\n]+\s*])/g, "$1");
  if (text !== before) writeFileSync(file, text);
}

for (const file of walk(path.join(root, "builtin", "attribute"))) {
  const text = readFileSync(file, "utf8");
  const next = text.replace(/"surface"/g, '"object"');
  if (next !== text) writeFileSync(file, next);
}

console.log("[DEBUG] fix-interaction-assets done");
