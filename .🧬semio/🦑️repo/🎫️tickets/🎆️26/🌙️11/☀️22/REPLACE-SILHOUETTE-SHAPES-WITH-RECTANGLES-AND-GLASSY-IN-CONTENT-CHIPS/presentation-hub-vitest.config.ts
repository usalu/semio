import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const root = "/Users/ueli/Documents/semio";
export default defineConfig({
  root,
  test: { root, name: "presentation-frame-body", environment: "node", include: [resolve(root, "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/🪟️frame-body/🟦️.ts")] },
});
