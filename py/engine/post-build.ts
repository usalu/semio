#!/usr/bin/env tsx
import { existsSync, renameSync, rmSync } from "fs";
import { join } from "path";

const cwd = __dirname;
const exePath = join(cwd, "dist", "semio-engine", "semio-engine.exe");
const internalPath = join(cwd, "dist", "semio-engine", "_internal");
const grasshopperBinPath = join(cwd, "..", "..", "net", "Semio.Grasshopper", "bin", "Debug", "net48");
const grasshopperExePath = join(grasshopperBinPath, "semio-engine.exe");
const grasshopperInternalPath = join(grasshopperBinPath, "_internal");

if (existsSync(grasshopperExePath)) {
  rmSync(grasshopperExePath);
}
if (existsSync(grasshopperInternalPath)) {
  rmSync(grasshopperInternalPath, { recursive: true });
}

renameSync(exePath, grasshopperExePath);
renameSync(internalPath, grasshopperInternalPath);

console.log("✅ Post-build complete");
