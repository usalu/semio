import { defineConfig } from "vite";
import { builtinModules } from "module";
import path from "path";

export default defineConfig({
  build: {
    outDir: ".vite/build",
    lib: {
      entry: "preload.ts",
      formats: ["cjs"],
      fileName: () => "preload.js",
    },
    rollupOptions: {
      external: ["electron", ...builtinModules],
    },
    emptyOutDir: false,
    sourcemap: "inline",
  },
});
