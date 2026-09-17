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
      cnameHost: "33.projektetage.zukunft-bau.mit-bestand.de",
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
      { find: "@semio-tech/animate-presentation-core", replacement: resolve(repoRoot, "✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/animate-js", replacement: resolve(repoRoot, "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/presentation", replacement: resolve(repoRoot, "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/presentation-react", replacement: resolve(repoRoot, "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/framework", replacement: resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/mit-bestand-praesentation-projektetage-spec", replacement: resolve(bundleRoot, "🔖️spec.ts") },
    ],
  },
});
