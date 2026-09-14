// #region 🔌️Adapters
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { semioAssetsVitePlugin, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, playgroundStaticSiteBuildOptions } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";
// #endregion 🔌️Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const bundleRoot = resolve(dir, "../../📦️packages/🟦️typescript");
const repoRoot = resolve(bundleRoot, "../../../../..");

export default defineConfig({
  root: bundleRoot,
  base: "./",
  publicDir: resolve(bundleRoot, "../../🌐️public"),
  plugins: [
    ...semioHostHtmlVitePlugin(repoRoot, {
      title: "33. Projektetage",
      entry: "./🟦️.ts",
      bodyClass: "h-screen w-screen overflow-hidden",
    }),
    semioEmojiIndexHtmlVitePlugin(bundleRoot),
    ...semioAssetsVitePlugin(repoRoot),
    tailwindcss(),
    react(),
  ],
  build: playgroundStaticSiteBuildOptions(),
  define: { "import.meta.vitest": "undefined" },
  server: {
    fs: { allow: [repoRoot] },
  },
  resolve: {
    alias: [
:♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/⚙️vite.config.ts
      },
    ],
  },
});
