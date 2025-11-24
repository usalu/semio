#!/usr/bin/env tsx
import { execSync } from "child_process";

execSync("poetry run pytest --cov --cov-config=pyproject.toml --cov-report html", {
  cwd: __dirname,
  stdio: "inherit",
});

console.log("✅ Tests complete");
