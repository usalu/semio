/** 🌳️ c4p4 module tree lister: prints the repo-relative files below a Rust root (tests included, crates not followed). */
import { listModuleTree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { relative } from "node:path";
const root = "/Users/ueli/Documents/semio";
for (const file of listModuleTree(process.argv[2]!, { tests: true, crates: false })) console.log(relative(root, file));
