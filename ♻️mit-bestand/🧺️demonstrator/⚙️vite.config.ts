import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import {
  playgroundPlayBootHtmlPlugin,
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
    playgroundPlayBootHtmlPlugin(),
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
    proxy: {
      "/generator": { target: "http://127.0.0.1:6027", changeOrigin: true, ws: true, rewrite: (path) => path.replace(/^\/generator/, "") || "/" },
      "/koordinator": { target: "http://127.0.0.1:6028", changeOrigin: true, ws: true, rewrite: (path) => path.replace(/^\/koordinator/, "") || "/" },
      "/aggregator": { target: "http://127.0.0.1:6023", changeOrigin: true, ws: true, rewrite: (path) => path.replace(/^\/aggregator/, "") || "/" },
      "/aussuchen": { target: "http://127.0.0.1:6030", changeOrigin: true, ws: true, rewrite: (path) => path.replace(/^\/aussuchen/, "") || "/" },
      "/bearbeiten": { target: "http://127.0.0.1:6031", changeOrigin: true, ws: true, rewrite: (path) => path.replace(/^\/bearbeiten/, "") || "/" },
      "/verfolgen": { target: "http://127.0.0.1:6032", changeOrigin: true, ws: true, rewrite: (path) => path.replace(/^\/verfolgen/, "") || "/" },
    },
  },
  resolve: {
    alias: [{ find: "@semio-tech/ui-react", replacement: uiReact }],
  },
});
