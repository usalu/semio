// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Health check endpoint. No auth required.
// #endregion 🧲Header

import { NextResponse } from "next/server";

export async function GET() {
  return new NextResponse("ok", { status: 200 });
}
