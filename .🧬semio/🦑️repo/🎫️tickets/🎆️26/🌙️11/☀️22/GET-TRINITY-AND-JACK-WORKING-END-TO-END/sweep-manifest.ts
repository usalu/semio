import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const apply = process.argv.includes("--apply");
const roots = process.argv.slice(2).filter(argument => !argument.startsWith("--"));
const isParent = (value: unknown): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value) && "camera" in value && "content" in value && "manifest" in value && "schema" in value && "query" in value;

function strip(value: unknown): number {
  let removed = 0;
  if (Array.isArray(value)) for (const item of value) removed += strip(item);
  else if (value && typeof value === "object") {
    const record = value as Record<string, unknown>;
    if (isParent(record)) { delete record.manifest; removed++; }
    for (const item of Object.values(record)) removed += strip(item);
  }
  return removed;
}

function* walk(directory: string): Generator<string> {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) { if (entry.name !== "node_modules" && entry.name !== "target") yield* walk(path); }
    else if (entry.name.endsWith(".json")) yield path;
  }
}

let files = 0, parents = 0, reformat = 0;
for (const root of roots) for (const path of walk(root)) {
  const text = readFileSync(path, "utf8");
  let value: unknown;
  try { value = JSON.parse(text); } catch { continue; }
  const roundtrip = JSON.stringify(value, null, 2) === text.replace(/\r\n/g, "\n").replace(/\n$/, "");
  const removed = strip(value);
  if (removed === 0) continue;
  files++; parents += removed;
  if (!roundtrip) reformat++;
  console.log(`${roundtrip ? "same-format" : "REFORMAT"} ${removed} ${path}`);
  if (apply) writeFileSync(path, JSON.stringify(value, null, 2) + (text.endsWith("\n") ? "\n" : ""));
}
console.log(`files=${files} parents=${parents} reformatted=${reformat} apply=${apply}`);
