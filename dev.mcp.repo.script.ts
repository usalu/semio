#!/usr/bin/env bun
/**
 * 🧩 Repo MCP stdio (`dev` → `mcp` → `repo`): `go run` for the IDE-specific entry under `repo/client/mcp/*`.
 * Positional: `client` | `codex` | `copilot` | `cursor` | `kiro` | `claude` (default `client`). Extra args pass through to `go run`.
 */
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const slug = (process.argv[2] ?? "client").trim().toLowerCase();
const extra = process.argv.slice(3);

const packages: Record<string, string> = {
  client: "./repo/client/mcp",
  codex: "./repo/client/mcp/codex",
  copilot: "./repo/client/mcp/copilot",
  cursor: "./repo/client/mcp/cursor",
  kiro: "./repo/client/mcp/kiro",
  claude: "./repo/client/mcp/claude",
};

const pkg = packages[slug];
if (!pkg) {
  console.error(`[dev.mcp.repo] unknown profile ${JSON.stringify(slug)}; expected one of: ${Object.keys(packages).join(", ")}`);
  process.exit(1);
}

const result = spawnSync("go", ["run", pkg, ...extra], {
  cwd: root,
  stdio: "inherit",
  env: { ...process.env, GOWORK: join(root, "go.work") },
});

process.exit(result.status ?? 1);
