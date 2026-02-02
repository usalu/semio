import path from "path";
import { fileURLToPath } from "url";
import { defineConfig } from "vite";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, "extension.test.ts"),
      formats: ["cjs"],
      fileName: () => "extension.test",
    },
    rollupOptions: {
      external: ["vscode", "jsonc-parser", "assert", "path", "mocha"],
      output: {
        entryFileNames: "extension.test.js",
        format: "cjs",
        sourcemap: true,
      },
    },
    outDir: "out/test",
    emptyOutDir: false,
    minify: false,
    sourcemap: true,
    target: "node18",
    ssr: true,
  },
  resolve: {
    alias: {
      "@semio/js": path.resolve(__dirname, "../../@semio/js"),
      "@semio/assets": path.resolve(__dirname, "../../@semio/assets"),
    },
  },
});
