import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const reactDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: reactDir,
  test: {
    environment: "node",
    include: ["index.tsx"],
    passWithNoTests: false,
  },
});
