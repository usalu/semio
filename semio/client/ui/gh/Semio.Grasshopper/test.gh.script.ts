#!/usr/bin/env bun
/** 🧪 Grasshopper .NET unit tests (`test` + `gh`). */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const fx = process.platform === "win32" ? "net48" : "net8.0";
execFileSync(
  "dotnet",
  ["test", join(root, "Semio.Grasshopper.Tests", "Semio.Grasshopper.Tests.csproj"), "-c", "UnitTest", "-f", fx],
  { cwd: root, stdio: "inherit" },
);
