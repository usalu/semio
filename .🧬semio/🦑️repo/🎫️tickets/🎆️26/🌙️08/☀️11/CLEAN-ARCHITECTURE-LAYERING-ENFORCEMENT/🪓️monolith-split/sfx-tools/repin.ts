import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
const owner = "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️composition";
const fixturePath = resolve(owner, "🧪️tests/🧫️fixtures/🔣️.json");
let text = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(text) as { suites: { path: string; sha256: string; tree?: boolean }[]; retainedArtifactUnits: { path: string; sha256: string }[] };
const write = process.argv.includes("--write");
let changed = 0;
for (const suite of fixture.suites) {
  const source = suite.tree === true ? readModuleTreeSource(resolve(owner, suite.path)) : readFileSync(resolve(owner, suite.path), "utf8");
  const sha = createHash("sha256").update(source).digest("hex");
  if (sha === suite.sha256) continue;
  const old = `"sha256": "${suite.sha256}"`;
  if (text.split(old).length !== 2) throw Error("old sha not unique: " + suite.sha256);
  text = text.replace(old, `"sha256": "${sha}"`);
  changed++;
  console.log(suite.path.split("/").slice(-4).join("/"), suite.sha256.slice(0, 8), "->", sha.slice(0, 8));
}
for (const unit of fixture.retainedArtifactUnits) {
  const sha = createHash("sha256").update(readFileSync(resolve(owner, "../../..", unit.path), "utf8")).digest("hex");
  if (sha === unit.sha256) continue;
  const old = `"sha256": "${unit.sha256}"`;
  if (text.split(old).length !== 2) throw Error("old sha not unique: " + unit.sha256);
  text = text.replace(old, `"sha256": "${sha}"`);
  changed++;
  console.log("unit", unit.path.split("/").slice(-4).join("/"), unit.sha256.slice(0, 8), "->", sha.slice(0, 8));
}
console.log("re-pinned", changed, write ? "(written)" : "(dry run)");
if (write && changed > 0) writeFileSync(fixturePath, text);
