// #region 🔖Header
// [🧰repo⌨️server🛅app🛅api🛅v1🛅events💻route](repo://p/i/repo/b/b/server/f/app/api/v1/events/route.ts)
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Event ingestion and listing API.
// #endregion 🔖Header

import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import { listEvents } from "@/lib/db";
import { requireAuth, isAuthError } from "@/lib/auth";
import { publishEvent } from "@/lib/events";

const EventSchema = z.object({
  kind: z.string().min(1),
  source: z.string().default(""),
  payload: z.unknown().default({}),
});

export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const kind = request.nextUrl.searchParams.get("kind") || undefined;
  const limit = parseInt(request.nextUrl.searchParams.get("limit") || "100");
  const events = await listEvents(kind, limit);
  return NextResponse.json(events);
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

  const parsed = EventSchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ error: parsed.error.message }, { status: 400 });
  }

  const event = await publishEvent(
    parsed.data.kind,
    parsed.data.source,
    parsed.data.payload
  );
  return NextResponse.json({ status: "ok", event_id: event.id });
}
