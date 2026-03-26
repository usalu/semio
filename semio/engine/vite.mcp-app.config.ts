// #region 🔖Header
// [👤semio📚engine💻vitemcpappconfig](repo://p/u/semio/b/l/engine/f/vite.mcp-app.config.ts)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Vite config for building the MCP App design viewer as a single HTML file.
// Summary: Vite build config bundling the MCP App React UI into a single inlined HTML file.

// #endregion 🔖Header

import { defineConfig } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";
import path from "path";

export default defineConfig({
  root: __dirname,
  build: {
    outDir: path.resolve(__dirname, "dist"),
    emptyOutDir: true,
    rollupOptions: {
      input: path.resolve(__dirname, "mcp-app.html"),
    },
  },
  plugins: [viteSingleFile()],
  resolve: {
    alias: {
      "@semio/ui": path.resolve(__dirname, "../ui/index.tsx"),
      "@semio/js": path.resolve(__dirname, "../js/index.ts"),
      "@elements/ui/elements": path.resolve(__dirname, "../../elements/ui/index.tsx"),
      "@elements/ui": path.resolve(__dirname, "../../elements/ui/index.tsx"),
      "@semio/assets/icons": path.resolve(__dirname, "../assets/icons/index.tsx"),
      "@semio/assets/lists/adjectives.json": path.resolve(__dirname, "../assets/lists/adjectives.json"),
      "@semio/assets/lists/animals.json": path.resolve(__dirname, "../assets/lists/animals.json"),
    },
  },
  esbuild: {
    jsx: "automatic",
  },
});
