#!/usr/bin/env tsx
// #region Header

// update.ts

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

import { execFileSync } from "child_process";
import { existsSync, readFileSync, writeFileSync } from "fs";
import { dirname, join, resolve } from "path";
import { fileURLToPath } from "url";

//#region Types
interface UpdateConfig {
  exclude: Record<string, string[]>;
  preserveLocalVersions: {
    npm: {
      pattern: string;
      autoDetectWorkspaces?: boolean;
    };
  };
  paths: {
    npm: { root: string; workspaces: boolean };
    python: string[];
    rust: string[];
    go: string[];
    dotnet: string[];
  };
}

interface PackageJson {
  name?: string;
  version?: string;
  workspaces?: string[];
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
  [key: string]: unknown;
}

type UpdateTarget = "all" | "npm" | "python" | "rust" | "go" | "dotnet";
//#endregion Types

//#region Cli
function parseArgs(argv: string[]): { target: UpdateTarget; dryRun: boolean } {
  let target: UpdateTarget = "all";
  let dryRun = false;

  for (let i = 2; i < argv.length; i++) {
    const arg = argv[i] ?? "";
    if (arg === "--help" || arg === "-h") {
      console.log("Usage: npx tsx update.ts [target] [--dry-run]");
      console.log("Targets: all, npm, python, rust, go, dotnet");
      console.log("Options:");
      console.log("  --dry-run  Show what would be updated without making changes");
      process.exit(0);
    }
    if (arg === "--dry-run") {
      dryRun = true;
      continue;
    }
    if (["all", "npm", "python", "rust", "go", "dotnet"].includes(arg)) {
      target = arg as UpdateTarget;
    }
  }

  return { target, dryRun };
}
//#endregion Cli

//#region Exec
const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = resolve(__dirname);

function run(command: string, args: string[] = [], cwd: string = rootDir): void {
  console.log(`  Running: ${command} ${args.join(" ")} in ${cwd}`);
  execFileSync(command, args, { stdio: "inherit", cwd, shell: true });
}

function runQuiet(command: string, args: string[] = [], cwd: string = rootDir): string {
  try {
    return execFileSync(command, args, { cwd, shell: true, encoding: "utf-8" });
  } catch {
    return "";
  }
}

function loadConfig(): UpdateConfig {
  const configPath = join(rootDir, "update.config.json");
  if (!existsSync(configPath)) {
    throw new Error("update.config.json not found");
  }
  return JSON.parse(readFileSync(configPath, "utf-8"));
}

function loadPackageJson(path: string): PackageJson {
  return JSON.parse(readFileSync(path, "utf-8"));
}

function savePackageJson(path: string, pkg: PackageJson): void {
  writeFileSync(path, JSON.stringify(pkg, null, 2) + "\n");
}
//#endregion Exec

//#region NPM
function getWorkspacePackageNames(): Set<string> {
  const names = new Set<string>();
  const rootPkg = loadPackageJson(join(rootDir, "package.json"));
  const workspaces = rootPkg.workspaces ?? [];

  for (const workspace of workspaces) {
    const wsPath = join(rootDir, workspace, "package.json");
    if (existsSync(wsPath)) {
      const pkg = loadPackageJson(wsPath);
      if (pkg.name) {
        names.add(pkg.name);
      }
    }
  }

  return names;
}

function findPackageJsonFiles(): string[] {
  const results: string[] = [];
  const rootPkg = loadPackageJson(join(rootDir, "package.json"));
  const workspaces = rootPkg.workspaces ?? [];

  // Root package.json
  results.push(join(rootDir, "package.json"));

  // Workspace package.json files
  for (const workspace of workspaces) {
    const wsPath = join(rootDir, workspace, "package.json");
    if (existsSync(wsPath)) {
      results.push(wsPath);
    }
  }

  return results;
}

function preserveLocalVersions(config: UpdateConfig, packageJsonPaths: string[], workspacePackages: Set<string>): Map<string, Map<string, string>> {
  const preserved = new Map<string, Map<string, string>>();
  const pattern = config.preserveLocalVersions.npm.pattern;

  for (const pkgPath of packageJsonPaths) {
    const pkg = loadPackageJson(pkgPath);
    const versions = new Map<string, string>();

    // Check dependencies
    if (pkg.dependencies) {
      for (const [name, version] of Object.entries(pkg.dependencies)) {
        if (workspacePackages.has(name) && version === pattern) {
          versions.set(`dependencies.${name}`, version);
        }
      }
    }

    // Check devDependencies
    if (pkg.devDependencies) {
      for (const [name, version] of Object.entries(pkg.devDependencies)) {
        if (workspacePackages.has(name) && version === pattern) {
          versions.set(`devDependencies.${name}`, version);
        }
      }
    }

    if (versions.size > 0) {
      preserved.set(pkgPath, versions);
    }
  }

  return preserved;
}

function restoreLocalVersions(preserved: Map<string, Map<string, string>>): void {
  for (const [pkgPath, versions] of preserved) {
    const pkg = loadPackageJson(pkgPath);
    let modified = false;

    for (const [key, value] of versions) {
      const [section, name] = key.split(".");
      if (section && name) {
        const deps = pkg[section] as Record<string, string> | undefined;
        if (deps && deps[name] !== value) {
          console.log(`  Restoring ${name}: "${value}" in ${pkgPath}`);
          deps[name] = value;
          modified = true;
        }
      }
    }

    if (modified) {
      savePackageJson(pkgPath, pkg);
    }
  }
}

function updateNpm(config: UpdateConfig, dryRun: boolean): void {
  console.log("\n[NPM] Updating npm packages...");

  const packageJsonPaths = findPackageJsonFiles();
  const workspacePackages = getWorkspacePackageNames();

  console.log(`  Detected ${workspacePackages.size} workspace packages: ${[...workspacePackages].join(", ")}`);

  // Preserve local versions before update
  const preserved = preserveLocalVersions(config, packageJsonPaths, workspacePackages);
  if (preserved.size > 0) {
    console.log("  Will preserve local package versions:");
    for (const [path, versions] of preserved) {
      for (const [key, value] of versions) {
        console.log(`    ${path}: ${key} = "${value}"`);
      }
    }
  }

  if (dryRun) {
    console.log("  [DRY RUN] Would run: npm update -S");
    console.log("[NPM] Done.");
    return;
  }

  // Run npm update -S to update package.json versions
  run("npm", ["update", "-S"]);

  // Restore local versions
  restoreLocalVersions(preserved);

  console.log("[NPM] Done.");
}
//#endregion NPM

//#region Python
interface PyProjectToml {
  content: string;
  dependencies: { name: string; spec: string; line: number }[];
}

function parsePyProjectToml(content: string): PyProjectToml {
  const dependencies: { name: string; spec: string; line: number }[] = [];
  const lines = content.split("\n");
  let inDependencies = false;
  let inDevDependencies = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i] ?? "";
    const trimmed = line.trim();

    // Detect section headers
    if (trimmed === "[project]" || trimmed.startsWith("dependencies = [")) {
      inDependencies = true;
      continue;
    }
    if (trimmed.startsWith("[dependency-groups]") || trimmed.startsWith("[tool.")) {
      inDependencies = false;
      inDevDependencies = trimmed.includes("dev") || trimmed.includes("test") || trimmed.includes("build");
      continue;
    }
    if (trimmed.startsWith("[") && !trimmed.startsWith("[dependency-groups]")) {
      inDependencies = false;
      inDevDependencies = false;
      continue;
    }

    // Parse dependency lines like: "package>=1.0.0",
    if (inDependencies || inDevDependencies) {
      const match = trimmed.match(/^"([a-zA-Z0-9_-]+)(\[.*?\])?([><=!~]+.+)?",?$/);
      if (match) {
        const name = match[1]!;
        const spec = match[3] ?? "";
        dependencies.push({ name, spec, line: i });
      }
    }
  }

  return { content, dependencies };
}

async function getLatestPyPIVersion(packageName: string): Promise<string | null> {
  try {
    const response = await fetch(`https://pypi.org/pypi/${packageName}/json`);
    if (!response.ok) return null;
    const data = await response.json();
    return data.info?.version ?? null;
  } catch {
    return null;
  }
}

async function updatePython(config: UpdateConfig, dryRun: boolean): Promise<void> {
  console.log("\n[Python] Updating Python packages...");

  for (const pyPath of config.paths.python) {
    const fullPath = join(rootDir, pyPath);
    const tomlPath = join(fullPath, "pyproject.toml");
    if (!existsSync(tomlPath)) {
      console.log(`  Skipping ${pyPath}: no pyproject.toml found`);
      continue;
    }

    console.log(`  Updating ${pyPath}...`);

    const content = readFileSync(tomlPath, "utf-8");
    const parsed = parsePyProjectToml(content);
    const lines = content.split("\n");
    let modified = false;

    for (const dep of parsed.dependencies) {
      const latestVersion = await getLatestPyPIVersion(dep.name);
      if (!latestVersion) {
        console.log(`    Skipping ${dep.name}: could not fetch latest version`);
        continue;
      }

      // Extract current minimum version from spec like ">=1.0.0" or ">=1.0.0,<2"
      const currentMatch = dep.spec.match(/>=([0-9.]+)/);
      const currentVersion = currentMatch ? currentMatch[1] : null;

      if (currentVersion && currentVersion !== latestVersion) {
        console.log(`    ${dep.name}: ${currentVersion} -> ${latestVersion}`);

        if (!dryRun) {
          // Update the line with new version
          const line = lines[dep.line] ?? "";
          const newLine = line.replace(
            new RegExp(`(${dep.name}(?:\\[.*?\\])?)>=([0-9.]+)`),
            `$1>=${latestVersion}`
          );
          lines[dep.line] = newLine;
          modified = true;
        }
      }
    }

    if (modified) {
      // Backup original content
      const originalContent = content;
      const newContent = lines.join("\n");
      writeFileSync(tomlPath, newContent);

      // Try to update the lock file
      try {
        run("uv", ["lock"], fullPath);
        console.log("    Successfully updated and locked.");
      } catch (err) {
        // Rollback on failure
        console.log("    Lock failed! Rolling back pyproject.toml...");
        writeFileSync(tomlPath, originalContent);
        console.log("    Rolled back to original versions.");
      }
    } else if (dryRun) {
      console.log("  [DRY RUN] Would update pyproject.toml versions");
    }
  }

  console.log("[Python] Done.");
}
//#endregion Python

//#region Rust
interface CargoToml {
  content: string;
  dependencies: { name: string; version: string; line: number; isTable: boolean }[];
}

function parseCargoToml(content: string): CargoToml {
  const dependencies: { name: string; version: string; line: number; isTable: boolean }[] = [];
  const lines = content.split("\n");
  let inDependencies = false;
  let inDevDependencies = false;
  let inTargetDeps = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i] ?? "";
    const trimmed = line.trim();

    // Detect section headers
    if (trimmed === "[dependencies]") {
      inDependencies = true;
      inDevDependencies = false;
      inTargetDeps = false;
      continue;
    }
    if (trimmed === "[dev-dependencies]") {
      inDevDependencies = true;
      inDependencies = false;
      inTargetDeps = false;
      continue;
    }
    if (trimmed.startsWith("[target.") && trimmed.includes("dependencies]")) {
      inTargetDeps = true;
      inDependencies = false;
      inDevDependencies = false;
      continue;
    }
    if (trimmed.startsWith("[") && !trimmed.includes("dependencies")) {
      inDependencies = false;
      inDevDependencies = false;
      inTargetDeps = false;
      continue;
    }

    if (inDependencies || inDevDependencies || inTargetDeps) {
      // Match simple format: package = "1.0"
      const simpleMatch = trimmed.match(/^([a-zA-Z0-9_-]+)\s*=\s*"([^"]+)"$/);
      if (simpleMatch) {
        dependencies.push({
          name: simpleMatch[1]!,
          version: simpleMatch[2]!,
          line: i,
          isTable: false
        });
        continue;
      }

      // Match table format: package = { version = "1.0", features = [...] }
      const tableMatch = trimmed.match(/^([a-zA-Z0-9_-]+)\s*=\s*\{.*version\s*=\s*"([^"]+)".*\}$/);
      if (tableMatch) {
        dependencies.push({
          name: tableMatch[1]!,
          version: tableMatch[2]!,
          line: i,
          isTable: true
        });
      }
    }
  }

  return { content, dependencies };
}

async function getLatestCratesVersion(packageName: string): Promise<string | null> {
  try {
    const response = await fetch(`https://crates.io/api/v1/crates/${packageName}`, {
      headers: {
        "User-Agent": "semio-update-script (https://github.com/usalu/semio)"
      }
    });
    if (!response.ok) return null;
    const data = await response.json();
    return data.crate?.max_stable_version ?? data.crate?.max_version ?? null;
  } catch {
    return null;
  }
}

async function updateRust(config: UpdateConfig, dryRun: boolean): Promise<void> {
  console.log("\n[Rust] Updating Rust packages...");

  for (const rsPath of config.paths.rust) {
    const fullPath = join(rootDir, rsPath);
    const tomlPath = join(fullPath, "Cargo.toml");
    if (!existsSync(tomlPath)) {
      console.log(`  Skipping ${rsPath}: no Cargo.toml found`);
      continue;
    }

    console.log(`  Updating ${rsPath}...`);

    const content = readFileSync(tomlPath, "utf-8");
    const parsed = parseCargoToml(content);
    const lines = content.split("\n");
    let modified = false;

    for (const dep of parsed.dependencies) {
      const latestVersion = await getLatestCratesVersion(dep.name);
      if (!latestVersion) {
        console.log(`    Skipping ${dep.name}: could not fetch latest version`);
        continue;
      }

      if (dep.version !== latestVersion) {
        console.log(`    ${dep.name}: ${dep.version} -> ${latestVersion}`);

        if (!dryRun) {
          const line = lines[dep.line] ?? "";
          if (dep.isTable) {
            // Replace version in table format
            const newLine = line.replace(
              /version\s*=\s*"[^"]+"/,
              `version = "${latestVersion}"`
            );
            lines[dep.line] = newLine;
          } else {
            // Replace version in simple format
            const newLine = line.replace(
              /=\s*"[^"]+"/,
              `= "${latestVersion}"`
            );
            lines[dep.line] = newLine;
          }
          modified = true;
        }
      }
    }

    if (modified) {
      // Backup original content
      const originalContent = content;
      const newContent = lines.join("\n");
      writeFileSync(tomlPath, newContent);

      // Try to update the lock file
      try {
        run("cargo", ["update"], fullPath);
        console.log("    Successfully updated and locked.");
      } catch (err) {
        // Rollback on failure
        console.log("    Cargo update failed! Rolling back Cargo.toml...");
        writeFileSync(tomlPath, originalContent);
        console.log("    Rolled back to original versions.");
      }
    } else if (dryRun) {
      console.log("  [DRY RUN] Would update Cargo.toml versions");
    }
  }

  console.log("[Rust] Done.");
}
//#endregion Rust

//#region Go
function updateGo(config: UpdateConfig, dryRun: boolean): void {
  console.log("\n[Go] Updating Go modules...");

  for (const goPath of config.paths.go) {
    const fullPath = join(rootDir, goPath);
    if (!existsSync(join(fullPath, "go.mod"))) {
      console.log(`  Skipping ${goPath}: no go.mod found`);
      continue;
    }

    console.log(`  Updating ${goPath}...`);

    if (dryRun) {
      console.log(`  [DRY RUN] Would run: go get -u ./... && go mod tidy in ${goPath}`);
      continue;
    }

    run("go", ["get", "-u", "./..."], fullPath);
    run("go", ["mod", "tidy"], fullPath);
  }

  console.log("[Go] Done.");
}
//#endregion Go

//#region DotNet
function updateCsprojPackage(content: string, packageName: string, newVersion: string): string {
  const regex = new RegExp(
    `(<PackageReference\\s+Include="${packageName}"\\s+Version=")([^"]+)(")`,
    "g"
  );
  return content.replace(regex, `$1${newVersion}$3`);
}

function updateDotNet(config: UpdateConfig, dryRun: boolean): void {
  console.log("\n[.NET] Updating .NET packages...");

  for (const csprojPath of config.paths.dotnet) {
    const fullPath = join(rootDir, csprojPath);
    if (!existsSync(fullPath)) {
      console.log(`  Skipping ${csprojPath}: file not found`);
      continue;
    }

    const excludedPackages = config.exclude[csprojPath] ?? [];
    console.log(`  Updating ${csprojPath}...`);
    if (excludedPackages.length > 0) {
      console.log(`    Excluding: ${excludedPackages.join(", ")}`);
    }

    if (dryRun) {
      console.log(`  [DRY RUN] Would check for package updates`);
      continue;
    }

    const csprojDir = dirname(fullPath);

    // Get list of outdated packages
    const outdatedOutput = runQuiet("dotnet", ["list", fullPath, "package", "--outdated"], csprojDir);

    // Parse outdated packages
    const lines = outdatedOutput.split("\n");
    const updates: { name: string; current: string; latest: string }[] = [];

    for (const line of lines) {
      // Match lines like: > PackageName    1.0.0    1.0.0    2.0.0
      const match = line.match(/>\s+(\S+)\s+\S+\s+(\S+)\s+(\S+)/);
      if (match) {
        const [, name, current, latest] = match;
        if (name && current && latest && !excludedPackages.includes(name)) {
          updates.push({ name, current, latest });
        }
      }
    }

    if (updates.length === 0) {
      console.log("    All packages are up to date (or excluded).");
      continue;
    }

    // Read and update csproj file
    let content = readFileSync(fullPath, "utf-8");
    for (const update of updates) {
      console.log(`    Updating ${update.name}: ${update.current} -> ${update.latest}`);
      content = updateCsprojPackage(content, update.name, update.latest);
    }
    writeFileSync(fullPath, content);
  }

  console.log("[.NET] Done.");
}
//#endregion DotNet

//#region Main
async function main() {
  const { target, dryRun } = parseArgs(process.argv);
  const config = loadConfig();

  console.log("=== Dependency Update Script ===");
  if (dryRun) {
    console.log("Running in DRY RUN mode - no changes will be made.");
  }
  console.log(`Target: ${target}`);

  if (target === "all" || target === "npm") {
    updateNpm(config, dryRun);
  }

  if (target === "all" || target === "python") {
    await updatePython(config, dryRun);
  }

  if (target === "all" || target === "rust") {
    await updateRust(config, dryRun);
  }

  if (target === "all" || target === "go") {
    updateGo(config, dryRun);
  }

  if (target === "all" || target === "dotnet") {
    updateDotNet(config, dryRun);
  }

  console.log("\n=== Update Complete ===");
  console.log("Run 'git diff' to review changes before committing.");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
//#endregion Main
