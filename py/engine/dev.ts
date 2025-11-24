#!/usr/bin/env tsx
import { runProcess, stopProcessOnPort } from "../../scripts/utils";

stopProcessOnPort(2507); // engine
stopProcessOnPort(5678); // debugger

const proc = runProcess("uv", ["run", "engine.py"], {
  cwd: __dirname,
  onExit: () => {
    console.log("Engine stopped");
    process.exit(0);
  },
});

// Wait for process to finish
proc.on("exit", (code) => {
  process.exit(code ?? 0);
});
