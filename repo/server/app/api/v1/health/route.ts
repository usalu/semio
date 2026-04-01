// #region 🔖Header
// [🧰repo⌨️server🛅app🛅api🛅v1🛅health💻route](repo://p/i/repo/b/b/server/f/app/api/v1/health/route.ts)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Health check endpoint. No auth required.
// #endregion 🔖Header

import { NextResponse } from "next/server";

export async function GET() {
  return new NextResponse("ok", { status: 200 });
}
