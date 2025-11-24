#!/usr/bin/env tsx
import { execSync } from "child_process";
import { existsSync, rmSync, mkdirSync, copyFileSync } from "fs";
import { join } from "path";

const cwd = __dirname;
const distDir = join(cwd, "dist");

// Clean dist folder
if (existsSync(join(distDir, "semio_512x512.png"))) {
  rmSync(join(distDir, "semio_512x512.png"));
}
if (existsSync(join(distDir, "manifest.yml"))) {
  rmSync(join(distDir, "manifest.yml"));
}

// Create dist folder if needed
if (!existsSync(distDir)) {
  mkdirSync(distDir);
}

// Copy files
copyFileSync(
  join(cwd, "..", "assets", "icons", "semio_512x512.png"),
  join(distDir, "semio_512x512.png")
);
copyFileSync(
  join(cwd, "manifest.yml"),
  join(distDir, "manifest.yml")
);

// Build with yak
const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" build --platform win`, { cwd: distDir, stdio: "inherit" });

console.log("✅ Yak package built");
