/** 🧪️ Narrow vitest config for T12: runs only the story-coordination tests it names, rooted at their plugin. */
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    root: "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📖️stories",
    include: ["🧭️coordination/🧪️tests/**/🟦️.ts"],
  },
});
