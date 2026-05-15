#!/usr/bin/env bun
/**
 * 💻 Root `dev` entry: default `@semio/desktop` via Nx; `mcp`, `mcp repo`, `mcp engine` delegate to bundle-local MCP scripts.
 */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const argv = process.argv.slice(2);

if (argv[0] === "storybook") {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env.STORYBOOK_PORT ?? "6010";
  const extra = argv.slice(1);
  execFileSync("bunx", ["storybook", "dev", "-c", ".storybook", "-p", port, "--exact-port", "--host", host, "--no-open", "--debug", ...extra], {
    cwd: root,
    stdio: "inherit",
    env: {
      ...process.env,
      WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
      CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    },
  });
  process.exit(0);
}

if (argv[0] === "mcp") {
  const mode = argv[1];
  if (mode === "engine") {
    execFileSync("bun", [join(root, "semio", "client", "bin", "engine", "dev.mcp.script.ts")], {
      cwd: root,
      stdio: "inherit",
    });
  } else if (mode === "repo") {
    execFileSync("bun", [join(root, "dev.mcp.inspector.script.ts"), "repo"], { cwd: root, stdio: "inherit" });
  } else {
    execFileSync("bun", [join(root, "dev.mcp.inspector.script.ts")], { cwd: root, stdio: "inherit" });
  }
  process.exit(0);
}

execFileSync("bun", ["nx", "run", "@semio/desktop:dev"], { cwd: root, stdio: "inherit" });
