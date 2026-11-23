import { defineConfig } from "vitest/config";

/** 🧪 Runs the catalog readback file, which is not named `*.test.ts`. */
export default defineConfig({
  test: {
    dir: "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗃️catalog-acknowledgment",
    include: ["*readback.ts"],
  },
});
