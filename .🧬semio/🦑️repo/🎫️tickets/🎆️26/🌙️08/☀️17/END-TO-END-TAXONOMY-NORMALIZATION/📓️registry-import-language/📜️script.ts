import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const fingerprint = (path: string) => {
  let absolute = repoRoot;
  const parts = path.split("/");
  for (let index = 0; index < parts.length; index++) {
    absolute = join(absolute, parts[index]!);
    const state = lstatSync(absolute);
    if (state.isSymbolicLink() || (index === parts.length - 1 ? !state.isFile() : !state.isDirectory())) throw new Error("Unexpected authority node: " + path);
  }
  const bytes = readFileSync(absolute);
  return { path, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex") };
};
if (process.argv[2] !== "closure") throw new Error("Usage: 📜️script.ts closure");
const authorities = ["📜️script.ts", library + "/🔣️taxonomy.json", library + "/🔍️discovery/🟦️component.ts"];
const before = authorities.map(fingerprint), start = performance.now();
const { loadTaxonomy, registryCatalogInputPaths, registryCatalogInputView } = await import(join(repoRoot, library, "🔍️discovery/🟦️component.ts"));
const taxonomy = loadTaxonomy(), view = registryCatalogInputView(repoRoot, taxonomy);
let reads = 0;
console.log("[DEBUG] actual registry closure started", new Date().toISOString());
try {
  const paths = registryCatalogInputPaths(repoRoot, taxonomy, { ...view, readText(path: string) { reads++; if (reads % 500 === 0) console.log("[DEBUG] registry catalog content reads", reads, path); return view.readText(path); } });
  console.log(JSON.stringify({ event: "registry-catalog-closure", milliseconds: performance.now() - start, paths: paths.length, contentReads: reads, pathsSha256: createHash("sha256").update(JSON.stringify(paths)).digest("hex"), opaqueAdmission: paths.some((path: string) => path === "compose" || path.startsWith("compose/") || path === "temp/compose" || path.startsWith("temp/compose/")) }));
} finally {
  const after = authorities.map(fingerprint);
  console.log(JSON.stringify({ before, after, stable: JSON.stringify(before) === JSON.stringify(after) }));
}
