import { defineConfig, loadConfigFromFile } from "vite";
import { globSync } from "node:fs";

const root = process.cwd();
const allMatches = globSync("**/🧪️vitest.config.ts", { cwd: root });
const matches = allMatches.filter(
  (p) => !p.includes("node_modules") && !p.startsWith(".🦑️repo/") && !p.startsWith("♻️mit-bestand/") && p !== "🧪️vitest.config.ts",
);
console.error("[probe9] matches:", matches.length);

const projects: unknown[] = [];
for (const rel of matches) {
  const abs = root + "/" + rel;
  try {
    const result = await loadConfigFromFile({ command: "serve", mode: "test" }, abs, root);
    if (result) {
      projects.push(result.config);
      console.error("[probe9] OK  ", rel);
    } else {
      console.error("[probe9] NULL", rel);
    }
  } catch (err) {
    console.error("[probe9] FAIL", rel, "->", (err as Error).message.split("\n")[0]);
  }
}
console.error("[probe9] total projects loaded:", projects.length);

export default defineConfig({
  test: { projects: projects as never },
} as never);
