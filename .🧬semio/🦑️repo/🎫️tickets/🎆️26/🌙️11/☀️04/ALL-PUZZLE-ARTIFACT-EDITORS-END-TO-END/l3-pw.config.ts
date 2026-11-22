import { defineConfig } from "../../../../../../../.storybook/🧪️tests/🎭️playwright/🟦️.ts";
const dir = process.env.L3_TEST_DIR!;
export default defineConfig({
  testDir: `../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🧪️tests/${dir}`,
  testMatch: "🟦️.ts",
  timeout: 180000,
  workers: 1,
  expect: { timeout: 60000 },
  reporter: "list",
  use: { baseURL: "http://localhost:6010/", launchOptions: { args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--use-angle=swiftshader", "--use-vulkan=swiftshader", "--use-webgpu-adapter=swiftshader", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] } },
});
