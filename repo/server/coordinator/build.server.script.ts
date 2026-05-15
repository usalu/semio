#!/usr/bin/env bun
/** 🏗️ Go coordinator server binary (`build` + `server`). */
import { execFileSync } from "node:child_process";

const coordinatorRoot = import.meta.dir;
const ext = process.platform === "win32" ? ".exe" : "";
execFileSync("go", ["build", "-o", `server${ext}`, "."], {
  cwd: coordinatorRoot,
  stdio: "inherit",
});
