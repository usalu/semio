// #region 🔌Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { uiAssetsVitePlugin, playgroundStaticSiteBuildOptions } from "../../../../../🧰/🔨/ui/⚡️/🦀/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const bundleRoot = resolve(dir, "..");
const repoRoot = resolve(bundleRoot, "../../../..");
const uiAssetsRoot = resolve(repoRoot, "framework/ui/asset");
const uiReact = resolve(repoRoot, "framework/ui/js/react/index.tsx");
const presentationCore = resolve(repoRoot, "animate/present/core/js/index.ts");
const presentationRenderer = resolve(repoRoot, "animate/present/renderer/react/index.tsx");
const frameworkCore = resolve(repoRoot, "framework/core/js/index.ts");

export default defineConfig({
  root: bundleRoot,
  base: "./",
  publicDir: resolve(bundleRoot, "public"),
  plugins: [...uiAssetsVitePlugin(uiAssetsRoot), tailwindcss(), react()],
  build: playgroundStaticSiteBuildOptions(),
  server: {
    fs: { allow: [repoRoot] },
  },
  resolve: {
    alias: [
      { find: "@semio-tech/ui-react", replacement: uiReact },
      { find: "@semio-tech/animate-present-core", replacement: presentationCore },
      { find: "@semio-tech/animate-present-renderer-react", replacement: presentationRenderer },
      { find: "@semio-tech/framework-core", replacement: frameworkCore },
      {
        find: "@semio-tech/mit-bestand-praesentation-projektetage-spec",
        replacement: resolve(dir, "index.ts"),
      },
    ],
  },
});
