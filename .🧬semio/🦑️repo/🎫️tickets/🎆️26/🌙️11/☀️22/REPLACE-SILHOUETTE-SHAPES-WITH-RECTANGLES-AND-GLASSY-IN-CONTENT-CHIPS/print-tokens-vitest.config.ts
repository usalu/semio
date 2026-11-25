import { defineConfig } from "vitest/config";
const root = "/Users/ueli/Documents/semio";
const print = `${root}/🧰️framework/🛍️products/📓️print`;
const ticket = `${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/REPLACE-SILHOUETTE-SHAPES-WITH-RECTANGLES-AND-GLASSY-IN-CONTENT-CHIPS`;
export default defineConfig({
  resolve: { alias: { "bun:test": "vitest" } },
  plugins: [{ name: "bun-import-meta-dir", enforce: "pre", transform: (code) => (/import\.meta\.dir\b(?!name)/.test(code) ? code.replace(/import\.meta\.dir\b(?!name)/g, "import.meta.dirname") : null) }],
  test: { include: [`${ticket}/print-tokens-verify.test.ts`, `${print}/🔨️modules/🎨print-design-token-paints/🧪️tests/🟦️.ts`, `${print}/🧪️tests/📝️assignment/🟦️.ts`], root, testTimeout: 120000 },
});
