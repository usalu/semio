// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Warnings listing API.
// #endregion 🧲Header

import { NextRequest, NextResponse } from "next/server";
import { listWarnings } from "@/lib/db";
import { requireAuth, isAuthError } from "@/lib/auth";

export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const ticketId = request.nextUrl.searchParams.get("ticket_id") || undefined;
  const warnings = await listWarnings(ticketId);
  return NextResponse.json(warnings);
}
