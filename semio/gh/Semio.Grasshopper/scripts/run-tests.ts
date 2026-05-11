#!/usr/bin/env bun
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..");
const fx = process.platform === "win32" ? "net48" : "net8.0";
execFileSync(
  "dotnet",
  [
    "test",
    "../Semio.Grasshopper.Tests/Semio.Grasshopper.Tests.csproj",
    "-c",
    "UnitTest",
    "-f",
    fx,
  ],
  { cwd: root, stdio: "inherit" },
);
