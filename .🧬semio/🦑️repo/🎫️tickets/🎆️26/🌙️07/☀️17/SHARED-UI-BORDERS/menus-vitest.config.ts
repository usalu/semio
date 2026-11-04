import { defineConfig } from "vitest/config";
import { fileURLToPath } from "node:url";
export default defineConfig({
  cacheDir: fileURLToPath(new URL("./🗑️generated/menus-vite", import.meta.url)),
  esbuild: { jsx: "automatic" },
  test: {
    environment: "jsdom",
    globals: true,
    include: ["🧰️framework/🔨️modules/🖱️ui/🧱️elements/{🖱️ContextMenu,🗨️Popover}/**/🧪️tests/**/*.tsx"],
    exclude: ["**/🔶️owns-uncontrolled-default-controlled/**", "**/🔸️routes-sibling-escape-focus/**"],
    setupFiles: [fileURLToPath(new URL("./menus-setup.ts", import.meta.url))],
  },
});
