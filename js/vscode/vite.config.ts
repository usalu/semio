import { defineConfig } from "vite";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, "src/extension.ts"),
      formats: ["cjs"],
      fileName: () => "extension",
    },
    rollupOptions: {
      external: ["vscode", "jsonc-parser"],
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
      "@semio/js": path.resolve(__dirname, "../js"),
    },
  },
});
