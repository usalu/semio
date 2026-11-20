#!/usr/bin/env repo
import {readFileSync, statSync} from "node:fs";
const p = "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any";
const paths = process.argv.slice(2);
const modified = paths[0] === "--modified";
if (modified) paths.shift();
for (const path of paths) {
  const source = p + "/" + path;
  if (modified) console.log("[DEBUG]", path, statSync(source).mtime.toISOString());
  else console.log(path, readFileSync(source, "utf8"));
}
