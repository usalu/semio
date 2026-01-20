import path from "path";
import { fileURLToPath } from "url";
import { defineConfig } from "vite";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, "extension.ts"),
      formats: ["cjs"],
      fileName: () => "extension",
    },
    rollupOptions: {
      external: ["vscode"],
      output: {
        entryFileNames: "extension.js",
        format: "cjs",
        sourcemap: true,
      },
    },
    outDir: "out",
    emptyOutDir: true,
    minify: false,
    sourcemap: true,
    target: "node18",
    ssr: true,
  },
  resolve: {
    alias: {
      "@semio/js": path.resolve(__dirname, "../semio"),
    },
  },
});
