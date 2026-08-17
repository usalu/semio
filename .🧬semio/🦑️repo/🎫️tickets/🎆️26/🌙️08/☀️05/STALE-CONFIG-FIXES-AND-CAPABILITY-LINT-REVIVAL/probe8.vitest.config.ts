import { defineConfig, loadConfigFromFile } from "vite";
import { globSync } from "node:fs";

const root = process.cwd();
const matches = globSync("**/🧪️vitest.config.ts", {
  cwd: root,
  exclude: (p: string) => p.includes("node_modules") || p.startsWith(".🦑️repo/") || p.startsWith("♻️mit-bestand/") || p === "🧪️vitest.config.ts",
});
console.error("[probe8] matches:", matches.length);

const projects: unknown[] = [];
for (const rel of matches) {
  const abs = root + "/" + rel;
  try {
    const result = await loadConfigFromFile({ command: "serve", mode: "test" }, abs, root);
    if (result) {
      projects.push(result.config);
      console.error("[probe8] OK  ", rel, "-> name:", (result.config as any).test?.name);
    } else {
      console.error("[probe8] NULL", rel);
    }
  } catch (err) {
    console.error("[probe8] FAIL", rel, "->", (err as Error).message.split("\n")[0]);
  }
}
console.error("[probe8] total projects loaded:", projects.length);

export default defineConfig({
  test: { projects: projects as never },
} as never);
