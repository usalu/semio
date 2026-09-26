import { defineConfig } from "/Users/ueli/Documents/semio/node_modules/vitest/dist/config.js";

export default defineConfig({
  test: {
    root: process.env.WG8_VITEST_ROOT ?? "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine",
    include: [process.env.WG8_VITEST_FILE ?? "none"],
  },
});
