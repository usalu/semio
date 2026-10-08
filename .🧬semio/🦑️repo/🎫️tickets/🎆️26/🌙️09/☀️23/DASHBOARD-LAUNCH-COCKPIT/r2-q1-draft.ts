import { readFileSync, writeFileSync } from "node:fs";

const path = "🎮️registry/🦀️.rs";
let text = readFileSync(path, "utf8");
const fields = ["id", "kind", "verb", "path", "owner", "subject", "qualifier", "long_running", "source", "own", "action"];
const needle = "Entry::new(";
let from = 0;
let changed = 0;
for (;;) {
  const at = text.indexOf(needle, from);
  if (at < 0) break;
  const open = at + needle.length;
  let depth = 1;
  let index = open;
  const parts: string[] = [];
  let start = open;
  let quoted = false;
  for (; index < text.length && depth > 0; index++) {
    const character = text[index]!;
    if (quoted) { if (character === "\\") index++; else if (character === '"') quoted = false; continue; }
    if (character === '"') quoted = true;
    else if ("([{".includes(character)) depth++;
    else if (")]}".includes(character)) { depth--; if (depth === 0) parts.push(text.slice(start, index).trim()); }
    else if (character === "," && depth === 1) { parts.push(text.slice(start, index).trim()); start = index + 1; }
  }
  if (parts.length !== fields.length) { from = open; if (parts.length === 11 || true) console.log("skip", parts.length, text.slice(at, at + 60)); continue; }
  const body = fields.map((field, position) => (parts[position] === field ? field : `${field}: ${parts[position]}`)).join(", ");
  const replacement = `Entry::new(Draft { ${body} })`;
  text = text.slice(0, at) + replacement + text.slice(index);
  from = at + replacement.length;
  changed++;
}
console.log("rewritten", changed);
writeFileSync(path, text);
