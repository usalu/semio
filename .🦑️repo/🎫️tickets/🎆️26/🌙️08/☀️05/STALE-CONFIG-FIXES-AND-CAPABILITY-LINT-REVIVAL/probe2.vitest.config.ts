import { defineConfig } from "vitest/config";
import { pathToFileURL } from "node:url";
import { resolve } from "node:path";

const root = process.cwd();
console.error("[probe] typeof Bun:", typeof Bun);
const glob = new Bun.Glob("**/🧪️vitest.config.ts");
const matches: string[] = [];
for await (const f of glob.scan({ cwd: root, absolute: true })) {
  if (f.includes("/node_modules/")) continue;
  matches.push(f);
}
console.error("[probe] matches found:", matches.length);
console.error(matches.slice(0, 5));

const projects = await Promise.all(
  matches.slice(0, 1).map(async (p) => {
    const mod = await import(pathToFileURL(p).href);
    return mod.default;
  })
);
console.error("[probe] loaded project configs:", projects.length, projects[0]?.test?.name);

export default defineConfig({
  test: { projects },
});
