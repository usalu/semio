import { defineConfig } from "vitest/config";
const e = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
export default defineConfig({
  test: { root: "/Users/ueli/Documents/semio", environment: "jsdom", include: [`${e}/🧪️tests/⏱️hop-trace/🟦️.ts`, `${e}/🧱️elements/📐️Canvas2dHost/🧪️tests/🎨️paint/🟦️.ts`, "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏱️trace/🧪️tests/**/*.ts"] },
});
