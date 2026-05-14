#!/usr/bin/env bun
/** 🏗️ Builds the Go repo MCP client binary from this bundle. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..");
execFileSync("bun", ["nx", "run", "@repo/client:build"], { cwd: root, stdio: "inherit" });
