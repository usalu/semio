/** 🔏️ Lists composition law suites whose pinned sha256 drifted from the authored source or whose named laws vanished; pass --write to repin hashes. */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

const root = process.cwd();
const path = resolve(root, "🧪️tests/🧫️fixtures/🔣️.json");
const text = readFileSync(path, "utf8");
const fixture = JSON.parse(text);
const hash = (s: string) => createHash("sha256").update(s).digest("hex");
let next = text;
for (const suite of fixture.suites) {
  const actual = hash(suite.tree === true ? readModuleTreeSource(resolve(root, suite.path)) : readFileSync(resolve(root, suite.path), "utf8"));
  if (actual !== suite.sha256) { console.log(`suite ${suite.target} ${suite.path}`); next = next.replace(suite.sha256, actual); }
  const source = suite.tree === true ? readModuleTreeSource(resolve(root, suite.path)) : readFileSync(resolve(root, suite.path), "utf8");
  for (const law of suite.laws) if (!source.includes(`fn ${law.split("::").at(-1)}(`)) console.log(`missing ${suite.target} ${law}`);
}
for (const unit of fixture.retainedArtifactUnits) {
  const actual = hash(readFileSync(resolve(root, "../../..", unit.path), "utf8"));
  if (actual !== unit.sha256) { console.log(`unit ${unit.target.name} ${unit.path}`); next = next.replace(unit.sha256, actual); }
  const source = readFileSync(resolve(root, "../../..", unit.path), "utf8");
  for (const law of unit.laws) if (!source.includes(`fn ${law.split("::").at(-1)}(`)) console.log(`missing ${unit.target.name} ${law}`);
}
if (process.argv.includes("--write") && next !== text) writeFileSync(path, next);
