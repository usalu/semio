#!/usr/bin/env tsx
import { execSync } from "child_process";
import { existsSync, rmSync } from "fs";
import { join } from "path";

const cwd = __dirname;

// Ensure venv exists
if (!existsSync(join(cwd, ".venv"))) {
  execSync("ux sync", { cwd, stdio: "inherit" });
}

// Activate venv and generate schemas
execSync(".venv/Scripts/activate.ps1 && tsx ./generate-schemas.ts", { cwd, stdio: "inherit", shell: "powershell.exe" });

// Clean build directories
if (existsSync(join(cwd, "build"))) {
  rmSync(join(cwd, "build"), { recursive: true });
}
if (existsSync(join(cwd, "dist"))) {
  rmSync(join(cwd, "dist"), { recursive: true });
}

// Run pyinstaller
const args = [
  "--name", "semio-engine",
  "--windowed",
  "--clean",
  "--noconfirm",
  "--copy-metadata", "graphene",
  "--copy-metadata", "sqlalchemy",
  "--copy-metadata", "loguru",
  "--hidden-import=loguru",
  "--add-data", "../../assets/icons/semio_512x512.png;icons/",
  "--icon", "../../assets/icons/semio.ico",
  "engine.py"
];

execSync(`pyinstaller ${args.join(" ")}`, { cwd, stdio: "inherit" });

// Run post-build unless skipped
if (!process.argv.includes("--skip-post-build")) {
  execSync("tsx ./post-build.ts", { cwd, stdio: "inherit" });
}

console.log("✅ Build complete");
