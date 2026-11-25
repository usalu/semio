import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const ui = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui";
const root = resolve(ui, "🎯️targets/⚛️react/📦️packages/🟦️typescript");
const include = (process.env.WEB_UI_INCLUDE ?? "").split(",").filter(Boolean).map((p) => resolve(ui, p));

export default defineConfig({
  root,
  resolve: {
    alias: [
      { find: /^@semio-tech\/ui-react\/introduction$/u, replacement: resolve(root, "👋️introduction.ts") },
      { find: /^@semio-tech\/ui-react$/, replacement: resolve(root, "🟦️.tsx") },
    ],
  },
  test: { root, name: "web-ui", environment: "jsdom", include, setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")] },
});
