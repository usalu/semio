/** 🧪️ Exercises the production catalog identity and PDF host transfer regressions. */
import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const renderer = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
const base = (await import(resolve(renderer,"🎯️targets/⚛️react/📦️packages/🟦️typescript/🧪️testing/🎚️config/🟦️.ts"))).default;
const identity = process.env.SEMIO_PDF_VITEST_SUITE === "identity";
const selected = identity
  ? resolve("✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🪪️plugin-identity/🟦️.ts")
  : resolve(renderer, "🧪️tests/💾️media-export-and-natural-files/🟦️.ts");
export default defineConfig({
  ...base,
  test: { ...base.test, ...(identity ? { environment: "node" } : {}), maxWorkers: 1, testNamePattern: undefined, include: [selected], includeSource: [] },
});
