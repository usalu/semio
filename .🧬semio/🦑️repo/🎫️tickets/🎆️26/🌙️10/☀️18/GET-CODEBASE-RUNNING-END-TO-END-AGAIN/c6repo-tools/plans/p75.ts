import type { Plan } from "../split.ts";
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts";
export const plan: Plan = {
  source: `${base}/🟦️.ts`,
  baseDir: base,
  modules: [
    { dir: "🧾️|command-inputs", ranges: [["testCommandInputs", "testNxDaemonRetention"]] },
    { dir: ".", ranges: [["proveCachePolicy", "proveCachePolicy"], ["createCachePolicyTests", "createCachePolicyTests"]] },
  ],
};
