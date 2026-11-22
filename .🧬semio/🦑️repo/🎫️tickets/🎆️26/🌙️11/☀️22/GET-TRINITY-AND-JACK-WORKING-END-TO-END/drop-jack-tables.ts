import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2]!;
const keep = new Set(["jack_document", "jack_camera", "jack_content_child"]);
const text = readFileSync(path, "utf8");
const lines = text.split("\n");
const out = lines.filter(line => {
  const match = /^\s*"(jack_[a-z_]+)":\s*\d+,?\s*$/.exec(line);
  return !match || keep.has(match[1]!);
});
const joined = out.join("\n").replace(/,(\s*\n\s*\})/g, "$1");
JSON.parse(joined);
writeFileSync(path, joined);
console.log(`removed=${lines.length - out.length}`);
