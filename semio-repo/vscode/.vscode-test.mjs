import { defineConfig } from "@vscode/test-cli";

const launchArgs = [];
if (process.platform === "linux") {
  launchArgs.push("--no-sandbox", "--disable-gpu", "--disable-dev-shm-usage");
}

export default defineConfig({
  files: "out/test/**/*.test.js",
  launchArgs,
});
