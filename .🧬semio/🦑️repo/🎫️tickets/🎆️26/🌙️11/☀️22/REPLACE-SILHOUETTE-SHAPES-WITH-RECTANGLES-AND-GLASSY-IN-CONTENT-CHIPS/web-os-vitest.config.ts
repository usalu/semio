import { defineConfig } from "vitest/config";
const e = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine";
export default defineConfig({
  test: { root: "/Users/ueli/Documents/semio", environment: "jsdom", include: [`${e}/🧪️tests/🪟️chrome-frame-laws/🟦️.ts`, `${e}/🧱️elements/🏛️ShellHost/🧱️notice-bands/🧪️tests/🧩️component/🟦️.tsx`, `${e}/🧱️elements/👕️canvas-presence/🧪️tests/**/*.ts*`, `${e}/🧱️elements/🌐️World3dHost/🧪️tests/**/*.ts*`, `${e}/🧱️elements/✏️TextEditor/🧪️tests/**/*.ts*`, `${e}/🧪️tests/🆚️diff-view-produced-surface/🟦️.tsx`, `${e}/🧪️tests/🖋️ink-canvas-editing/🟦️.tsx`, `${e}/🧱️elements/🖌️Paint2dHost/🧪️tests/**/*.ts*`, `${e}/🧪️tests/💾️saved-state/🟦️.tsx`] },
});
