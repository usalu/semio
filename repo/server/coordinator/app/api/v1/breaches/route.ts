// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Breach listing API.
// #endregion 🧲Header

// #region 🔌Adapters
import { NextRequest, NextResponse } from "next/server";
// #endregion 🔌Adapters

import { listBreaches } from "@/lib/db";
import { requireAuth, isAuthError } from "@/lib/auth";

export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const ticketId = request.nextUrl.searchParams.get("ticket_id") || undefined;
  const breaches = await listBreaches(ticketId);
  return NextResponse.json(breaches);
}
