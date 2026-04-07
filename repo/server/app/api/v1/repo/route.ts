// #region 🔖Header
// [🧰repo⌨️server🛅app🛅api🛅v1🛅repo💻route](repo://p/i/repo/b/b/server/f/app/api/v1/repo/route.ts)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Repo indexing API: reindex all files, index single file.
// #endregion 🔖Header

import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import { replaceScopes } from "@/lib/db";
import { requireAuth, isAuthError } from "@/lib/auth";
import { publishEvent } from "@/lib/events";
import { buildScopesForFile } from "@/lib/parsing";
import { readFileSync, readdirSync, statSync } from "fs";
import { join, relative } from "path";

const REPO_ROOT = process.env.SEMIO_SERVER_REPO_ROOT || process.cwd();

const IndexFileSchema = z.object({
  action: z.literal("index-file"),
  file_path: z.string().min(1),
  content: z.string().default(""),
});

const ReindexSchema = z.object({
  action: z.literal("reindex"),
});

function walkFiles(dir: string): string[] {
  const files: string[] = [];
  try {
    const entries = readdirSync(dir, { withFileTypes: true });
    for (const entry of entries) {
      if (entry.name.startsWith(".")) continue;
      const fullPath = join(dir, entry.name);
      if (entry.isDirectory()) {
        files.push(...walkFiles(fullPath));
      } else {
        const rel = relative(REPO_ROOT, fullPath).replace(/\\/g, "/");
        files.push(rel);
      }
    }
  } catch {
    // Skip inaccessible directories
  }
  return files;
}

export async function POST(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  let body: unknown;
  try {
    body = await request.json();
  } catch {
    return NextResponse.json({ error: "invalid JSON" }, { status: 400 });
  }

  const actionCheck = z.object({ action: z.string() }).safeParse(body);
  if (!actionCheck.success) {
    return NextResponse.json({ error: "action required" }, { status: 400 });
  }

  switch (actionCheck.data.action) {
    case "index-file": {
      const parsed = IndexFileSchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const { file_path, content } = parsed.data;
      let fileContent = content;
      if (!fileContent) {
        try {
          fileContent = readFileSync(join(REPO_ROOT, file_path), "utf-8");
        } catch {
          return NextResponse.json({ error: "file not found" }, { status: 404 });
        }
      }
      const scopes = buildScopesForFile(file_path, fileContent);
      await replaceScopes(file_path, scopes);
      await publishEvent("IndexUpdated", "server", { file: file_path });
      return NextResponse.json({ status: "ok" });
    }

    case "reindex": {
      const files = walkFiles(REPO_ROOT);
      let indexed = 0;
      for (const file of files) {
        try {
          const content = readFileSync(join(REPO_ROOT, file), "utf-8");
          const scopes = buildScopesForFile(file, content);
          await replaceScopes(file, scopes);
          indexed++;
        } catch {
          continue;
        }
      }
      return NextResponse.json({ files: indexed });
    }

    default:
      return NextResponse.json({ error: "unknown action" }, { status: 400 });
  }
}
