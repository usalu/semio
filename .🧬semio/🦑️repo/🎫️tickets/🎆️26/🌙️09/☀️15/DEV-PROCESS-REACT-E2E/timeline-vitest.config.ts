import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../../../../../../..");
const ui = resolve(root, "🧰️framework/🔨️modules/🖱️ui");

export default {
  root: ui,
  cacheDir: resolve(import.meta.dirname, "🗑️generated/tree-vitest-cache"),
  test: {
    include: ["🧱️elements/🌳️Tree/🧪️tests/🧩️component/🟦️.tsx", "🧱️elements/🌳️Tree/🧪️tests/⌨️roving-tabindex.tsx"],
    environment: "jsdom",
    setupFiles: [resolve(ui, "🧪️tests/🧹️react-environment/🟦️.ts")],
    maxWorkers: 1,
    testTimeout: 30000,
  },
};
