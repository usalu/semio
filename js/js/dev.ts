import { spawn } from "child_process";

const isWindows = process.platform === "win32";
const npmCmd = isWindows ? "npm.cmd" : "npm";

// Start Vite dev server
const vite = spawn(npmCmd, ["exec", "vite"], {
  stdio: "inherit",
  shell: true,
});

// Start Storybook dev server
const storybook = spawn(
  npmCmd,
  ["exec", "storybook", "dev", "-p", "6006", "--no-open", "--debug"],
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
