import { join } from "node:path";

const [mode] = Bun.argv.slice(2);
if (mode !== "dry-run" && mode !== "enforce") throw Error("Expected dry-run or enforce");
const workspace = process.cwd();
const library = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching");
const { retireStaleUnitsV1 } = await import(join(library, "🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts"));
const source = await Bun.file(join(library, "🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts")).text();
const importOf = (name: string): string => {
  const row = source.split("\n").find(line => line.startsWith("import") && line.includes(name) && !line.startsWith("import type"));
  const from = row?.match(/from\s*["']([^"']+)["']/)?.[1];
  if (!from) throw Error("Missing import of " + name);
  return join(library, "🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession", from);
};
const { createCasStoreV1 } = await import(importOf("createCasStoreV1"));
const { casRootV1 } = await import(importOf("casRootV1"));
const { removedKeysList, examples, ...result } = await retireStaleUnitsV1(join(workspace, ".cache/b/r/m/u"), { workspace, mode, host: createCasStoreV1({ root: casRootV1() }) });
console.log("[DEBUG] retire", JSON.stringify(result));
