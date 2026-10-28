// undeclared-g scope-set: the files of the executor-command implementation scope of the Bazel adapter (static roots by bun's import scanner, byte-only leaves, traced modules and reads of every class): what the ACTION KEY's implementation digest covers. Prints one workspace-relative path per line.
import {readFileSync} from "node:fs";
import {staticClosure} from "./static-closure.mjs";
const table = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️scopes/🔣️.json", "utf8"));
export function scopeSet(id = "executor-command") {
  const scope = table.scopes.find((row) => row.id === id), files = new Set(scope.leaves);
  for (const path of staticClosure(scope.roots)) files.add(path);
  for (const row of scope.classes) for (const path of [...row.traced.modules, ...(row.traced.reads ?? []), ...(row.traced.physical ?? [])]) files.add(path);
  return files;
}
if (import.meta.main) for (const path of [...scopeSet()].sort()) console.log(path);
