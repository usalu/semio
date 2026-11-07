import { defineConfig } from "vitest/config";
import { fileURLToPath } from "node:url";

export default defineConfig({
  root: fileURLToPath(new URL("../../../../../../../", import.meta.url)),
  cacheDir: fileURLToPath(new URL("./🗑️generated/contract-cache", import.meta.url)),
  test: { environment: "node", include: ["🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/segments.test.ts"], maxWorkers: 1, fileParallelism: false },
});
