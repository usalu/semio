/** 🫳️ Vitest runner config for the node-graph TS twins W3-T-FLOWCAD touched (gesture record, design §13.3) — their suite
 * files are `🟦️.ts`, which no default `include` glob matches, so the one way to run them is to name them.
 * Usage: cd /Users/ueli/Documents/semio && ./node_modules/.bin/vitest run --config <this file>
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts
 */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    root: "/Users/ueli/Documents/semio",
    include: ["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts"],
  },
});
