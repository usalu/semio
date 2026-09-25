import { readFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { registryStaticImports } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const seen = new Set<string>(), bare = new Set<string>();
const pending = ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts"];
while (pending.length) {
  const path = pending.shift()!;
  if (seen.has(path)) continue;
  seen.add(path);
  if (path.endsWith(".json")) continue;
  for (const specifier of registryStaticImports(readFileSync(join(root, path), "utf8"), path)) {
    if (specifier.startsWith("node:")) continue;
    if (specifier.startsWith(".")) pending.push(normalize(join(dirname(path), specifier)));
    else bare.add(specifier);
  }
}
console.log([...seen].join("\n"));
console.log("bare:", [...bare]);
