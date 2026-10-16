import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const owner = "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️composition";
const fixturePath = resolve(owner, "🧪️tests/🧫️fixtures/🔣️.json");
let text = readFileSync(fixturePath, "utf8");
const apply = process.argv.includes("--write");
for (const path of process.argv.slice(2).filter(argument => !argument.startsWith("--"))) {
  const tree = readModuleTreeSource(resolve(owner, path));
  const sha = createHash("sha256").update(tree).digest("hex");
  const key = `      "path": ${JSON.stringify(path)},\n`;
  const at = text.indexOf(key);
  if (at < 0 || text.indexOf(key, at + 1) >= 0) throw Error("suite path not unique: " + path);
  const shaAt = text.indexOf('      "sha256": "', at), shaEnd = text.indexOf("\n", shaAt) + 1;
  const old = text.slice(shaAt, shaEnd);
  const hasTree = text.slice(shaEnd, shaEnd + 40).startsWith('      "tree": true');
  const replacement = `      "sha256": "${sha}",\n` + (hasTree ? "" : '      "tree": true,\n');
  console.log(path.split("/").slice(-3).join("/"), old.trim(), "->", replacement.trim().replace("\n", " "));
  text = text.slice(0, shaAt) + replacement + text.slice(shaEnd);
}
if (apply) writeFileSync(fixturePath, text);
