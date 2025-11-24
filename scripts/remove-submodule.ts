#!/usr/bin/env tsx
import { execSync } from "child_process";
import { existsSync, rmSync } from "fs";
import { join } from "path";

const submodulePath = process.argv[2] || "examples/geometry";
const rootDir = join(__dirname, "..");

// Remove from git
execSync(`git rm ${submodulePath}`, { cwd: rootDir, stdio: "inherit" });

// Remove config sections (ignore errors)
try {
  execSync(`git config --remove-section submodule.${submodulePath}`, { cwd: rootDir });
} catch {
  // Ignore errors
}

try {
  execSync(`git config -f .gitmodules --remove-section submodule.${submodulePath}`, { cwd: rootDir });
} catch {
  // Ignore errors
}

// Remove .git/modules directory
const modulesPath = join(rootDir, ".git", "modules", submodulePath);
if (existsSync(modulesPath)) {
  rmSync(modulesPath, { recursive: true, force: true });
}

console.log(`✅ Removed submodule ${submodulePath}`);
