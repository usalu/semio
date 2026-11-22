import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../..");
const interpreter = resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter");
const ui = resolve(root, "🧰️framework/🔨️modules/🖱️ui");

export default {
  root: interpreter,
  cacheDir: resolve(import.meta.dirname, "🗑️generated/interpreter-vitest-cache"),
  server: { fs: { strict: false } },
  test: {
    include: [],
    includeSource: ["🟦️.tsx"],
    environment: "jsdom",
    setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")],
    maxWorkers: 1,
    testTimeout: 30000,
  },
};
