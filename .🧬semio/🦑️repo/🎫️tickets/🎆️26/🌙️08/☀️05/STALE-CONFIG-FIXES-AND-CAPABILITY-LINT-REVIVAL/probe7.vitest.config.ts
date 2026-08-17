import { defineConfig } from "vitest/config";
import { pathToFileURL } from "node:url";

const mod = await import(pathToFileURL("/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/STALE-CONFIG-FIXES-AND-CAPABILITY-LINT-REVIVAL/deep/nested/vitest.config.ts").href);
const project = mod.default;
console.error("[aggregator] loaded project object root=", project.root);

export default defineConfig({
  test: {
    projects: [project],
  },
});
