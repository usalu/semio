import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const ui = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui";
const root = resolve(ui, "🎯️targets/⚛️react/📦️packages/🟦️typescript");
const chrome = resolve(ui, "🧱️elements/🗂️WindowChrome");

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: /^@semio-tech\/ui-react\/introduction$/u, replacement: resolve(root, "👋️introduction.ts") },
      { find: /^@semio-tech\/ui-react$/, replacement: resolve(root, "🟦️.tsx") },
    ],
  },
  test: {
    root,
    name: "compositor-lane",
    environment: "jsdom",
    include: [
      resolve(chrome, "*.test.ts"),
      resolve(chrome, "*.test.tsx"),
    ],
    setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")],
  },
});
