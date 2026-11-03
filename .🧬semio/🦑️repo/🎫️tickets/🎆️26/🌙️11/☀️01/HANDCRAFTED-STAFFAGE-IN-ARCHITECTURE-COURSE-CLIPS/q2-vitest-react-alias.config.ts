import { resolve } from "node:path";
import { mergeConfig } from "vitest/config";
import base from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

const repository = resolve(import.meta.dirname, "../../../../../../..");

export default mergeConfig(base, {
  resolve: {
    alias: [
      { find: /^@semio-tech\/framework-transient$/u, replacement: resolve(repository, "🧰️framework/🔨️modules/🫧️transient/🟦️.ts") },
      { find: /^d3-format$/u, replacement: resolve(repository, "node_modules/.bun/d3-format@3.1.0/node_modules/d3-format/src/index.js") },
    ],
  },
});
