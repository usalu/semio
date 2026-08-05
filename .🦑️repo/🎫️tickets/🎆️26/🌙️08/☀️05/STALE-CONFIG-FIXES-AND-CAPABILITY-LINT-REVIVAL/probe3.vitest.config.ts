import { defineConfig } from "vitest/config";
import { globSync } from "node:fs";
import { pathToFileURL } from "node:url";

const matches = globSync("**/🧪️vitest.config.ts", {
  cwd: process.cwd(),
  exclude: (p: string) => p.includes("node_modules") || p.startsWith(".🦑️repo/") || p.startsWith("♻️mit-bestand/"),
});
console.error("[probe] matches found:", matches.length);
console.error(matches.slice(0, 10));

const projects = await Promise.all(
  matches.map(async (p) => {
    const mod = await import(pathToFileURL(process.cwd() + "/" + p).href);
    return mod.default;
  })
);
console.error("[probe] loaded project configs:", projects.length);
console.error(projects.map((p) => p?.test?.name ?? "(unnamed)"));

export default defineConfig({
  test: { projects },
});
