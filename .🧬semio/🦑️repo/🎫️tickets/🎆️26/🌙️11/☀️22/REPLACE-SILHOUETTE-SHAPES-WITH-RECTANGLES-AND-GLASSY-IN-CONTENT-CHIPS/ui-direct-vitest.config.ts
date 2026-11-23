import { defineConfig } from "vitest/config";
const ui = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements";
export default defineConfig({
  test: { root: "/Users/ueli/Documents/semio", environment: "jsdom", include: [`${ui}/🖱️ContextMenu/🧪️tests/📝️text-selection/🟦️.ts`, `${ui}/🌳️Tree/🧪️tests/🧲️drag-drop.tsx`] },
});
