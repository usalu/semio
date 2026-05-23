// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Ticket detail and claims endpoint.
// #endregion 🧲Header

// #region 🔌Adapters
import { NextRequest, NextResponse } from "next/server";
// #endregion 🔌Adapters

import { getTicket, listClaimsByTicket, requireAuth, isAuthError } from "@/lib";

export async function GET(
  request: NextRequest,
  { params }: { params: Promise<{ id: string }> }
) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const { id } = await params;
  const ticketId = decodeURIComponent(id);
  const ticket = await getTicket(ticketId);
  if (!ticket) {
    return NextResponse.json({ error: "ticket not found" }, { status: 404 });
  }

  const claims = request.nextUrl.searchParams.get("claims");
  if (claims === "true") {
    const scopeClaims = await listClaimsByTicket(ticketId);
    return NextResponse.json({ ticket, claims: scopeClaims });
  }

  return NextResponse.json(ticket);
}
