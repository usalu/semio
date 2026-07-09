import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");
const dir = join(root, "repo/server/lib");

const ORDER = ["db.ts", "parsing.ts", "auth.ts", "events.ts"] as const;

function stripFile(src: string): string {
  const lines = src.split(/\r?\n/);
  let i = 0;
  if (lines[0]?.includes("#region") && lines[0]?.includes("Header")) {
    i++;
    while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
    i++;
  }
  const out: string[] = [];
  for (; i < lines.length; i++) {
    const line = lines[i]!;
    if (/^import\s+.*from\s+["']\.\/[^"']+["'];?\s*$/.test(line.trim())) continue;
    if (/^import type \{ Scope \} from ["']\.\/db["'];?\s*$/.test(line.trim())) continue;
    out.push(line);
  }
  return out.join("\n").trim();
}

const header = `// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Repo server library: PostgreSQL, auth, events, parsing (Next.js API routes).
// #endregion 🧲Header

// #region 🔌Adapters
import { Pool, type PoolClient } from "pg";
import { createHash } from "crypto";
import { NextRequest, NextResponse } from "next/server";
import PgBoss from "pg-boss";
// #endregion 🔌Adapters

`;

let body = "";
for (const file of ORDER) {
  let chunk = stripFile(readFileSync(join(dir, file), "utf8"));
  if (file === "parsing.ts") {
    chunk = chunk.replace(/^\/\/ #region ⚙️Types\r?\n/, "// #region ⚙️Types\n");
  }
  body += `\n// #region 🔖${file.replace(".ts", "")}\n${chunk}\n// #endregion 🔖${file.replace(".ts", "")}\n`;
}

const workerRaw = readFileSync(join(dir, "worker.ts"), "utf8");
let worker = stripFile(workerRaw);
worker = worker.replace(/\/\/ #region 🔌Adapters[\s\S]*?\/\/ #endregion 🔌Adapters\r?\n\r?\n/, "");

body += `\n// #region 🔖worker\n${worker}\n// #endregion 🔖worker\n`;

writeFileSync(join(dir, "index.ts"), header + body);

for (const file of [...ORDER, "worker.ts"] as const) {
  unlinkSync(join(dir, file));
}

console.log("merged repo/server/lib into index.ts");
