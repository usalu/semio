// #region 🔖Header
// [🧰repo⌨️server🛅app🛅api🛅v1🛅scopes💻route](repo://p/i/repo/b/b/server/f/app/api/v1/scopes/route.ts)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Scope listing API.
// #endregion 🔖Header

import { NextRequest, NextResponse } from "next/server";
import { listScopesByFile } from "@/lib/db";
import { requireAuth, isAuthError } from "@/lib/auth";

export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const file = request.nextUrl.searchParams.get("file");
  if (!file) {
    return NextResponse.json({ error: "file query required" }, { status: 400 });
  }
  const scopes = await listScopesByFile(file);
  return NextResponse.json(scopes);
}
