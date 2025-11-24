#!/usr/bin/env tsx
import { execSync } from "child_process";

execSync('.venv/Scripts/activate.ps1 && python -c "from engine import generateSchemas; generateSchemas()"', {
  cwd: __dirname,
  stdio: "inherit",
  shell: "powershell.exe",
});

console.log("✅ Schemas generated");
