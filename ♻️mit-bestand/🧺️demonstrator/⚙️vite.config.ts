import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import {
  playgroundStaticSiteBuildOptions,
  semioEmojiIndexHtmlVitePlugin,
  staticDirVitePlugin,
  uiAssetsVitePlugin,
} from "../../🧰️framework/🔨️module/🖱️ui/🎨️styling/⚡️implementation/🦀️rust/🟦️vite-elements-assets.ts";
import { DEMONSTRATOR_ASSETS_DIR } from "./🟦️brand.ts";

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../..");
const uiAssetsRoot = resolve(repoRoot, "./🧰️framework/🔨️module/🖼️asset/⚡️implementation/🟦️typescript");
const uiReact = resolve(repoRoot, "./🧰️framework/🔨️module/🖱️ui/⚛️react/⚡️implementation/🟦️typescript/📦️index.tsx");

export default defineConfig({
  root: dir,
  base: "./",
  publicDir: resolve(dir, "public"),
  build: playgroundStaticSiteBuildOptions({ outDir: "dist" }),
  plugins: [
    semioEmojiIndexHtmlVitePlugin(dir),
    ...uiAssetsVitePlugin(uiAssetsRoot),
    staticDirVitePlugin(repoRoot, { kind: "static-dir", route: `/${DEMONSTRATOR_ASSETS_DIR}`, root: DEMONSTRATOR_ASSETS_DIR }),
    tailwindcss(),
    react(),
  ],
  server: {
    fs: { allow: [repoRoot] },
    port: Number(process.env.MIT_BESTAND_DEMONSTRATOR_PORT ?? 6029),
    strictPort: true,
  },
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: uiReact }],
  },
});
