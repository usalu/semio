#!/usr/bin/env tsx
import { execSync } from "child_process";

const msbuild = "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\MSBuild\\Current\\Bin\\MSBuild.exe";

execSync(`"${msbuild}" Semio.csproj /p:Configuration=Debug`, {
  cwd: __dirname,
  stdio: "inherit",
});

console.log("✅ Semio.NET build complete");
