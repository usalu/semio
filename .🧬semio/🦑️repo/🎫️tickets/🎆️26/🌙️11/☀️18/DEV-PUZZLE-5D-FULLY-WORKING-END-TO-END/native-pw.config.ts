import { defineConfig } from "../../../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
export default defineConfig({
  testDir: "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧊️storybook-spatial",
  testMatch: "🟦️.ts",
  timeout: 180000,
  workers: 1,
  reporter: "list",
  use: { baseURL: "http://localhost:6010/", launchOptions: { args: ["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"] } },
});
