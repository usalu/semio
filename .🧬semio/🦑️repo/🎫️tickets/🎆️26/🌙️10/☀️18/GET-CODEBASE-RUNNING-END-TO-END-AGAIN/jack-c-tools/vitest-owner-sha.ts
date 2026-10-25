import { createHash } from "node:crypto";
import { relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = "/Users/ueli/Documents/semio";
const [ownerRelative, rootRelative] = process.argv.slice(2) as [string, string];

function portable(value: unknown): unknown {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([left], [right]) => left.localeCompare(right)).map(([key, child]) => [key, portable(child)]));
  return value;
}
const config = ((await import(pathToFileURL(resolve(repoRoot, ownerRelative)).href)) as { default: Record<string, any> }).default;
const projection = { aliases: config.resolve?.alias ?? null, cacheDir: config.cacheDir ?? null, coverageInclude: config.test?.coverage?.include ?? null, root: rootRelative, exclude: config.test?.exclude ?? null, include: config.test?.include ?? null, includeSource: config.test?.includeSource ?? null, name: config.test?.name ?? null, passWithNoTests: config.test?.passWithNoTests ?? null, projects: config.test?.projects ?? null, setupFiles: config.test?.setupFiles ?? null };
const serialized = JSON.stringify(portable(projection));
console.log(JSON.stringify({ serialized, sha: createHash("sha256").update(serialized).digest("hex"), root: config.root, testRoot: config.test?.root, expectedRoot: resolve(repoRoot, rootRelative) }, null, 1));
