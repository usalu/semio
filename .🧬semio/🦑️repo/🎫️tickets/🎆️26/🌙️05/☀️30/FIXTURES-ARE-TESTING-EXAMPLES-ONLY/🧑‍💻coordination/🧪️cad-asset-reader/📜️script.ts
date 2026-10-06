import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const source = join(root, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx");
const test = join(root, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx");
assert(readFileSync(source,"utf8").includes("{ directory: import.meta.dir, url: import.meta.url }"));
const path = readFileSync(test,"utf8").match(/resolve\(source\.directory, "([^"]+)"\)/)?.[1];
assert(path);
const target = resolve(dirname(source),path);
assert(statSync(target).isFile());
const nodeBytes = readFileSync(target);
const bunJson = await Bun.file(target).json();
assert.deepEqual(bunJson, JSON.parse(nodeBytes.toString("utf8")));
const result = { path:relative(root,target), bytes:nodeBytes.length, sha256:createHash("sha256").update(nodeBytes).digest("hex"), rootKeys:Object.keys(bunJson), reader:"actual TestSource.directory argument and current test literal", boundary:"filesystem and JSON read only; renderer suite failed during unrelated artifact initialization before tests" };
writeFileSync(join(ticket,"🗑️generated/coordinator/cad-asset-reader.json"),JSON.stringify(result,null,2)+"\n");
console.log("[DEBUG] "+JSON.stringify(result));

