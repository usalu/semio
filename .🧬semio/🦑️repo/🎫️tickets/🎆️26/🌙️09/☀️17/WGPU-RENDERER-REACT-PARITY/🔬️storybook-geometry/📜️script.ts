import { defineConfig } from "@playwright/test";
export default defineConfig({
  testDir: "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories",
  testMatch: ["**/🟦️.ts"],
  grep: /Mode reserves|Panel chrome/,
  outputDir: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/storybook-geometry-tests",
  workers: 1,
  retries: 0,
  timeout: 120000,
  reporter: [["list"]],
  use: { baseURL: "http://127.0.0.1:6327/", viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1, screenshot: "only-on-failure" }
});
