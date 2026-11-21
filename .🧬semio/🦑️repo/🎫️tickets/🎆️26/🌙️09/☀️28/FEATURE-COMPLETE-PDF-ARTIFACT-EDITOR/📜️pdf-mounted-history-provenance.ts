/** 🏷️ Matches live PDF history schema digests to the actual mounted WASI compiler artifacts. */
import {createHash} from "node:crypto";
import {existsSync, readFileSync, readdirSync, statSync, writeFileSync} from "node:fs";
import {basename, join, resolve} from "node:path";
const run = resolve(process.argv[2]);
const ticket = resolve(import.meta.dirname);
const reviewedAnnotations = process.argv[3] === "annotations";
const rustCache = resolve(".cache/b/r/m/u");
const {readLookupsV1,storePathsV1} = await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts"));
const artifacts = new Map<string, Set<string>>();
for (const line of readFileSync(join(run, "l/cargo-prototype-00.jsonl"), "utf8").split("\n")) {
  if (!line.includes("pdf_mut") && !line.includes("pdf_conformance_support") && !line.includes("pdf_schema_")) continue;
  try {
    const row = JSON.parse(line);
    if (row.reason !== "compiler-artifact") continue;
    artifacts.set(row.target.name, new Set(row.filenames.map((file: string) => basename(file))));
  } catch {}
}
const reviewed = reviewedAnnotations ? JSON.parse(readFileSync(join(ticket, "🗑️generated/pdf-reviewed-postimages.json"), "utf8")) as Record<string,string> : undefined;
const paths = reviewed
  ? Object.keys(reviewed).filter(path => path.endsWith("/🔣️.json"))
  : readFileSync(join(ticket, "📜️pdf-history-labels.patch"), "utf8").split("\n").filter(line => line.startsWith("+++ b/")).map(line => line.slice(6));
const expected = new Map(paths.map(path => [path, reviewed?.[path] ?? createHash("sha256").update(readFileSync(resolve(path))).digest("hex")]));
const observed = new Map<string, { path: string; digest: string; crate: string; key: string; artifact: string }>();
const index = storePathsV1(rustCache).entries;
for (const row of readLookupsV1(rustCache, statSync(run).birthtimeMs)) {
  const names = artifacts.get(row.crate);
  if (!names) continue;
  const key = row.key, directory = join(index, key.slice(0, 2), key);
  if (!existsSync(directory)) continue;
  for (const file of readdirSync(directory)) {
    const receipt = JSON.parse(readFileSync(join(directory, file), "utf8"));
    const output = receipt.outputs.find((value: {name: string}) => names.has(value.name));
    if (!output) continue;
    for (const entry of receipt.dependencies as {path: string; sha256: string | null}[]) {
      for (const [path, sha] of expected) {
        const suffix = path.split("/").slice(-4).join("/");
        if (entry.sha256 === sha && entry.path.replaceAll("\\", "/").endsWith(suffix)) {
          observed.set(path, {path, digest: sha, crate: row.crate, key, artifact: output.name});
        }
      }
    }
  }
}
for (const row of observed.values()) console.log("[DEBUG]", row.crate, row.path.split("/").slice(-4).join("/"), row.digest);
const missing = paths.filter(path=>!observed.has(path));
console.log("[DEBUG] Observed", observed.size, "of", paths.length, reviewedAnnotations ? "reviewed annotation schema inputs in current WASI artifact receipts" : "history schema inputs in current WASI artifact receipts");
writeFileSync(join(ticket, reviewedAnnotations ? `🗑️generated/pdf-mounted-annotation-provenance-${basename(run)}.json` : "🗑️generated/pdf-mounted-history-provenance.json"), JSON.stringify({run:basename(run), verified:[...observed.values()], missing}, null, 2)+"\n");
if (missing.length) process.exitCode = 1;
