import { resolve } from "node:path";
import { defineConfig } from "vitest/config";

const component = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧩️component";
export default defineConfig({
  cacheDir: resolve(import.meta.dirname, "🗑️generated/grouped-hover-vitest"),
  esbuild: { jsx: "automatic" },
  test: {
    environment: "jsdom",
    include: [".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/image-source-body.test.tsx", component + "/hover-publication-order.test.ts", component + "/🟦️.tsx", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🖱️hover/async-pointer.test.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🖱️hover/🟦️.test.ts", ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/preview-detail-react.test.tsx"],
  },
});
