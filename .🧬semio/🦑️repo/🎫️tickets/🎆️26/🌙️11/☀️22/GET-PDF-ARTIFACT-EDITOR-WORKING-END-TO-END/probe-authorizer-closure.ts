import { dirname, join, normalize, relative } from "node:path";
import { readFileSync } from "node:fs";

/** 🔬️ Captures the authorizer compiler closure exactly as the nx authorizer build does, then lists every relative JSON import of a captured module whose target is not captured.
 * Usage: `bun probe-authorizer-closure.ts` from the workspace root. */

const root = process.cwd();
const tools = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools");
const { captureNxAuthorizerSourcesV1 } = await import(join(tools, "🔏️authorizers/🟦️.ts"));
const entry = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📜️script.ts";
const sources = await captureNxAuthorizerSourcesV1(root, entry, new AbortController().signal);
const manifest: any = sources.manifest;
const files: string[] = Object.keys((manifest.sources ?? {}) as Record<string, unknown>);
console.log(`captured ${files.length} files; manifest keys: ${Object.keys(manifest).join(",")}`);
const captured = new Set(files.map((file) => normalize(file)));
const rx = /(?:from|import)\s*\(?\s*["'](\.{1,2}\/[^"']+\.json)["']/gu;
let missing = 0;
for (const file of captured) {
  if (!/\.(ts|mjs|js)$/u.test(file)) continue;
  let text: string;
  try { text = readFileSync(join(root, file), "utf8"); } catch { continue; }
  for (const match of text.matchAll(rx)) {
    const target = normalize(join(dirname(file), match[1]!));
    if (!captured.has(target)) { missing++; console.log(`MISSING ${target}\n   imported by ${file}`); }
  }
}
console.log(`${missing} uncaptured JSON imports`);
