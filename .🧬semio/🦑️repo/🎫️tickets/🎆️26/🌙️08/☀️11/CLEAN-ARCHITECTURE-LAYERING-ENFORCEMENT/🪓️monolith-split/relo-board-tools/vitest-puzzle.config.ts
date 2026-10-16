/** 🧩️ Slice D vitest scope for the puzzle 2d session factory suite (its own renderer-contract config points at a stale path). */
import { defineConfig } from "vitest/config";
import rendererConfig from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

const root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle";
export default defineConfig({
  ...rendererConfig,
  root,
  cacheDir: `${rendererConfig.cacheDir}-relo-puzzle`,
  test: { ...rendererConfig.test, root, name: "relo-puzzle-session-factory", include: ["🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🪪️session-factory/🟦️.ts"], includeSource: [], passWithNoTests: false, testNamePattern: undefined },
  server: { fs: { strict: false } },
} as never);
