#!/usr/bin/env bun
/** 🗣️ Extracts the Go `mcpDescriptionTable` literal from the component snapshot into the schema-owned JSON table. */
import { readFileSync, writeFileSync } from "node:fs";

const [source, target] = process.argv.slice(2);
const text = readFileSync(source, "utf8");
const start = text.indexOf("var mcpDescriptionTable = map[string]map[McpClientKind]string{");
if (start < 0) throw new Error("mcpDescriptionTable not found");
const end = text.indexOf("\n}\n", start);
const literal = text.slice(text.indexOf("{", start) + 1, end);

const KIND: Record<string, string> = {
  McpClientGeneric: "generic",
  McpClientCursor: "cursor",
  McpClientKiro: "kiro",
  McpClientCopilot: "copilot",
  McpClientClaude: "claude",
  McpClientCodex: "codex",
};

const descriptions: Record<string, Record<string, string>> = {};
let key = "";
for (const raw of literal.split("\n")) {
  const line = raw.trim();
  const opened = line.match(/^"([a-z_]+)":\s*\{$/);
  if (opened) {
    key = opened[1];
    descriptions[key] = {};
    continue;
  }
  const entry = line.match(/^(McpClient\w+):\s*"((?:[^"\\]|\\.)*)",$/);
  if (entry && key) descriptions[key][KIND[entry[1]]] = JSON.parse(`"${entry[2]}"`);
}

/** 🎯️ Goal tool descriptions are new surface: the Go CLI owns the verbs, the MCP table never carried them. */
const GOALS: Record<string, Record<string, string>> = {
  tool_goal_open: {
    generic: "Call when a new goal must be recorded before any ticket can reference it.",
    cursor: "Call when the Cursor agent must record a new goal before any ticket references it.",
    kiro: "Call when the Kiro agent must record a new goal before any ticket references it.",
    copilot: "Call when the Copilot agent must record a new goal before any ticket references it.",
    claude: "Call when Claude Code must record a new goal before any ticket references it.",
    codex: "Call when Codex must record a new goal before any ticket references it.",
  },
  tool_goal_close: {
    generic: "Call when a goal is fulfilled and must be closed with an evidence summary.",
    cursor: "Call when the Cursor agent has fulfilled a goal and must close it with an evidence summary.",
    kiro: "Call when the Kiro agent has fulfilled a goal and must close it with an evidence summary.",
    copilot: "Call when the Copilot agent has fulfilled a goal and must close it with an evidence summary.",
    claude: "Call when Claude Code has fulfilled a goal and must close it with an evidence summary.",
    codex: "Call when Codex has fulfilled a goal and must close it with an evidence summary.",
  },
  tool_goal_reopen: {
    generic: "Call when a closed goal must be reopened because its outcome did not hold.",
    cursor: "Call when the Cursor agent must reopen a closed goal because its outcome did not hold.",
    kiro: "Call when the Kiro agent must reopen a closed goal because its outcome did not hold.",
    copilot: "Call when the Copilot agent must reopen a closed goal because its outcome did not hold.",
    claude: "Call when Claude Code must reopen a closed goal because its outcome did not hold.",
    codex: "Call when Codex must reopen a closed goal because its outcome did not hold.",
  },
};

const merged = { ...descriptions, ...GOALS };
const ordered = Object.fromEntries(
  Object.keys(merged)
    .sort()
    .map((name) => [name, Object.fromEntries(Object.keys(KIND).map((k) => KIND[k]).filter((k) => merged[name][k]).map((k) => [k, merged[name][k]]))]),
);

writeFileSync(
  target,
  JSON.stringify({ schema: "semio.repo.mcp.descriptions/1", kinds: Object.values(KIND), descriptions: ordered }, null, 2) + "\n",
);
console.log(`[descriptions] ${Object.keys(ordered).length} keys -> ${target}`);
