#!/usr/bin/env bun
/** Builds the native `repo` (module 💻️client/🔌️mcp) through the canonical Go plan with a pinned `go`, exactly the `go build -overlay=<plan> -o <out> .` of `repo-mcp:build` without the capacity queue. usage: bun build-native.ts <out> */
import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const [out] = process.argv.slice(2);
const root = process.cwd(), module = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp");
const { canonicalGoPlan } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts"));
const plan = canonicalGoPlan(module);
const dir = mkdtempSync(join(tmpdir(), "zt-go-")), overlay = join(dir, "overlay.json");
writeFileSync(overlay, JSON.stringify({ Replace: plan.replacements }));
const result = spawnSync("go", ["build", "-trimpath", "-ldflags=-s -w", `-overlay=${overlay}`, "-o", out!, "."], { cwd: module, stdio: "inherit", env: { ...process.env, GOWORK: join(root, "go.work"), GOPROXY: "off", GOFLAGS: "", GOTOOLCHAIN: "local", CGO_ENABLED: "0" } });
rmSync(dir, { recursive: true, force: true });
process.exit(result.status ?? 1);
