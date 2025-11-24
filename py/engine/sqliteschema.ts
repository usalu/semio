#!/usr/bin/env tsx
import { execSync } from "child_process";
import { join } from "path";

const dbPath = join(__dirname, "debug", "semio.db");
const outputPath = join(__dirname, "..", "..", "sqlite", "schema.sql");

execSync(`sqlite3 ${dbPath} .schema > ${outputPath}`, {
  cwd: __dirname,
  stdio: "inherit",
});

console.log("✅ SQLite schema exported");
