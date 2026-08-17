import { defineConfig, loadConfigFromFile } from "vite";

const root = process.cwd();
const rel = "🧰️framework/⚡️implementations/🟦️typescript/🧪️vitest.config.ts";
const abs = root + "/" + rel;
const result = await loadConfigFromFile({ command: "serve", mode: "test" }, abs, root);
console.error("[probe10] root:", (result?.config as any)?.root);
console.error("[probe10] test.include:", (result?.config as any)?.test?.include);
console.error("[probe10] test.name:", (result?.config as any)?.test?.name);

export default defineConfig({
  test: { include: [] },
} as never);
