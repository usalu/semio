import { discoverOwners } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🔭️repository-scan/🟦️.ts";
import { readFileSync } from "node:fs";
const bad = new Set(readFileSync(process.argv[2]!, "utf8").split("\n").map(l => l.split("\t")[1]).filter(Boolean).map(p => p!.split("/📦️packages/")[0]));
const owners = discoverOwners("/Users/ueli/Documents/semio");
const clean = owners.filter(o => !bad.has(o.ownerRel));
console.log(owners.length, clean.length);
for (const o of clean.slice(0, 4000)) console.log(o.ownerRel, o.langs.join(","));
