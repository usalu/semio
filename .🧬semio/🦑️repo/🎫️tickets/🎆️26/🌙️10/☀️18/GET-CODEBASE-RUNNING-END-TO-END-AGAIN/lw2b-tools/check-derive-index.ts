import { existsSync, readFileSync } from "node:fs";
const index = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json", "utf8"));
const entries: Array<[string, string]> = [];
const walk = (value: unknown): void => {
  if (!value || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value)) {
    if (typeof child === "string" && key.startsWith("https://")) entries.push([key, child]);
    else walk(child);
  }
};
walk(index);
let missing = 0, mismatch = 0;
for (const [id, path] of entries) {
  if (!existsSync(path)) { missing += 1; console.log("MISSING", path.slice(-90)); continue; }
  const document = JSON.parse(readFileSync(path, "utf8"));
  if (document.$id !== id) { mismatch += 1; console.log("MISMATCH", id, document.$id); }
}
console.log("entries", entries.length, "missing", missing, "idMismatch", mismatch);
