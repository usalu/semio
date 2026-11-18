import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

const repo = resolve(import.meta.dirname, "../../../../../../..");
const product = resolve(repo, "🧰️framework/🛍️products/📚️course");

/** 🧪️ Ticket-local course test config: jsdom with the repo's React and locale setup, bypassing the broken runner. */
export default defineConfig({
  root: product,
  test: {
    include: ["🧪️tests/*/🟦️.tsx", "🎯️targets/⚛️react/🧪️tests/📚️course-shell/🟦️.tsx"],
    exclude: ["**/node_modules/**", "**/*-kit/**"],
    environment: "jsdom",
    css: { include: [/🎨️\.css(?:\?|$)/u] },
    setupFiles: [resolve(repo, "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧹️react-environment/🟦️.ts"), resolve(product, "🧪️tests/🌐️preloaded-locales/🟦️.ts")],
    testTimeout: 120000,
  },
});
