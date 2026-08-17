import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
console.error("[nested-config] import.meta.url =", import.meta.url);
console.error("[nested-config] computed root =", root);

export default defineConfig({
  root,
  test: {
    name: "nested-probe",
    include: ["probe.test.ts"],
  },
});
