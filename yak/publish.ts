#!/usr/bin/env tsx
import { execSync } from "child_process";
import { readFileSync } from "fs";
import { join } from "path";

const cwd = join(__dirname, "dist");


const manifestContent = readFileSync(join(cwd, "manifest.yml"), "utf-8");
const versionMatch = manifestContent.match(/version:\s*(.+)/);
if (!versionMatch) {
  throw new Error("Could not find version in manifest.yml");
}
const version = versionMatch[1].trim();
const buildName = `semio-${version}-rh8_10-win.yak`;

const yak = "C:\\Program Files\\Rhino 8\\System\\Yak.exe";
execSync(`"${yak}" push ${buildName}`, { cwd, stdio: "inherit" });

console.log("✅ Yak package published");
