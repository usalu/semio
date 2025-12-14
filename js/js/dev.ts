import { spawn } from "child_process";

const isWindows = process.platform === "win32";
const npmCmd = isWindows ? "npm.cmd" : "npm";

const vite = spawn(npmCmd, ["run", "dev:sketchpad"], {
  stdio: "inherit",
  shell: true,
});

const storybook = spawn(
  npmCmd,
  ["run", "dev:storybook"],
  {
    stdio: "inherit",
    shell: true,
  }
);

// Handle process termination
process.on("SIGINT", () => {
  vite.kill();
  storybook.kill();
  process.exit();
});

process.on("SIGTERM", () => {
  vite.kill();
  storybook.kill();
  process.exit();
});
