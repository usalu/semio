import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework/compiler/syntax/module-tree";
const root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly";
const test = readFileSync(resolve(root, "🧪️tests/🔬️typescript-package-commands/🟦️.ts"), "utf8");
const block = test.slice(test.indexOf("const structural = ["), test.indexOf("];", test.indexOf("const structural = [")));
const needles = [...block.matchAll(/^\s*("(?:[^"\\]|\\.)*"),?$/gm)].map((m) => JSON.parse(m[1]!));
const source = readModuleTreeSource(resolve(root, "🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"));
console.log("missing", needles.filter((n) => !source.includes(n)));
