import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const repoRoot = "/Users/ueli/Documents/semio";
const fixturePath = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const portable = (value: unknown): unknown => {
  if (typeof value === "string") return value === repoRoot ? "." : value.startsWith(`${repoRoot}/`) ? `./${relative(repoRoot, value).replaceAll("\\", "/")}` : value;
  if (value instanceof RegExp) return { flags: value.flags, regex: value.source };
  if (typeof value === "function") return { function: value.name || "anonymous" };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([key]) => key !== "plugins").sort(([l], [r]) => l.localeCompare(r)).map(([k, c]) => [k, portable(c)]));
  return value;
};
const projection = (config: any, root: string) => ({ aliases: config.resolve?.alias ?? null, cacheDir: config.cacheDir ?? null, coverageInclude: config.test?.coverage?.include ?? null, root, exclude: config.test?.exclude ?? null, include: config.test?.include ?? null, includeSource: config.test?.includeSource ?? null, name: config.test?.name ?? null, passWithNoTests: config.test?.passWithNoTests ?? null, projects: config.test?.projects ?? null, setupFiles: config.test?.setupFiles ?? null });
for (const owner of fixture.owners) {
  const module = await import(pathToFileURL(resolve(repoRoot, owner.ownerPath)).href);
  const exported = module.default;
  const config = typeof exported === "function" ? await exported({ command: "serve", mode: "test", isSsrBuild: false, isPreview: false }) : exported;
  const hash = createHash("sha256").update(JSON.stringify(portable(projection(config, owner.configurationRoot)))).digest("hex");
  if (hash !== owner.projectionSha256) console.log(`${owner.ownerPath}\t${owner.projectionSha256}\t${hash}\t${JSON.stringify(portable(projection(config, owner.configurationRoot)))}`);
}
