import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const apply = process.argv.includes("--apply");
const roots = process.argv.slice(2).filter(argument => !argument.startsWith("--"));
const isParent = (value: unknown): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value) && "camera" in value && "content" in value && "schema" in value && "query" in value && "name" in value && !("manifestId" in value);

function fill(value: unknown): number {
  let added = 0;
  if (Array.isArray(value)) for (const item of value) added += fill(item);
  else if (value && typeof value === "object") {
    const record = value as Record<string, unknown>;
    if (isParent(record)) {
      const entries = Object.entries(record), id = record.schema === null ? null : "nakagin";
      for (const key of Object.keys(record)) delete record[key];
      for (const [key, item] of entries) { record[key] = item; if (key === "name") record.manifestId = id; }
      added++;
    }
    for (const item of Object.values(record)) added += fill(item);
  }
  return added;
}

function* walk(directory: string): Generator<string> {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) { if (entry.name !== "node_modules" && entry.name !== "target") yield* walk(path); }
    else if (entry.name.endsWith(".json")) yield path;
  }
}

let files = 0;
for (const root of roots) for (const path of walk(root)) {
  const text = readFileSync(path, "utf8");
  let value: unknown;
  try { value = JSON.parse(text); } catch { continue; }
  const added = fill(value);
  if (added === 0) continue;
  files++;
  console.log(`${added} ${path}`);
  if (apply) writeFileSync(path, JSON.stringify(value, null, 2) + (text.endsWith("\n") ? "\n" : ""));
}
console.log(`files=${files} apply=${apply}`);
