/** 🔭️ Vite configuration of the preview of work packages I and C3: the quiz's pets glue in a real browser without a
 * proctor.
 *
 * It serves `index.html` beside this file with the aliases, Tailwind chain and asset plugin of the architecture quiz
 * site, so cards, preferences and the pet layer look and stack as they do on the site. A ticket tool, never a product.
 * Start from the repository root: `bun node_modules/vite/bin/vite.js --config <this file> --port 6257 --strictPort`
 * (I used 6193); `c3_preview_check.mjs` drives it.
 * @see ../📓️report-wp-i.md
 * @see ../📓️report2-c3.md */
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { semioReferencedAssetsVitePlugin } from "../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(here, "../../../../../../../..");

export default defineConfig({
  root: here,
  base: "/",
  publicDir: false,
  cacheDir: resolve(here, "../🗑️generated/c3/preview-cache"),
  plugins: [...semioReferencedAssetsVitePlugin(repoRoot), tailwindcss(), react()],
  server: { fs: { allow: [repoRoot] }, hmr: false, watch: null },
  resolve: {
    alias: [
      { find: /^@semio-tech\/quiz-react$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/quiz$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/pets-react$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/pets$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/ui-react$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: /^@semio-tech\/ui-react\/i18n$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts") },
      { find: /^@semio-tech\/ui-react\/chrome$/, replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome/🟦️.ts") },
      { find: /^@semio-tech\/framework-server$/, replacement: resolve(repoRoot, "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/framework$/, replacement: resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
    ],
    dedupe: ["react", "react-dom"],
  },
});
