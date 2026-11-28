import { defineConfig, devices } from "../../../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
import { browserLaunchOptionsV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
const base = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests";
export default defineConfig({
  testDir: base,
  testMatch: ["◻️storybook-2d/🟦️.ts", "🧊️storybook-spatial/🟦️.ts"],
  timeout: 180000,
  workers: 1,
  reporter: "list",
  use: { ...devices["Desktop Chrome"], baseURL: `http://127.0.0.1:${process.env.P4_PORT ?? "6020"}/`, channel: "chromium", launchOptions: { args: browserLaunchOptionsV1().args } },
});
