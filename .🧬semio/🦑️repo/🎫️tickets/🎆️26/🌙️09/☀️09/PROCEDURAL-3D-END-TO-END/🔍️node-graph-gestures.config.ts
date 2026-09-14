/** 🫳️ Vitest runner config for the node-graph gesture TS twin — the suite file is `🟦️.ts`, which no
 * default `include` glob matches, so the one way to run it is to name it.
 * Usage: cd /Users/ueli/Documents/semio && ./node_modules/.bin/vitest run --config <this file>
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts
 */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: { root: "/Users/ueli/Documents/semio", include: ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts"] },
});
