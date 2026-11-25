import { defineConfig } from "../../../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
const base = "../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests";
export default defineConfig({
  testDir: base,
  testMatch: ["◻️storybook-2d/🟦️.ts", "🧊️storybook-spatial/🟦️.ts"],
  timeout: 180000,
  workers: 1,
  reporter: "list",
  use: { baseURL: `http://localhost:${process.env.P4_PORT ?? "6010"}/`, launchOptions: { args: ["--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader"] } },
});
