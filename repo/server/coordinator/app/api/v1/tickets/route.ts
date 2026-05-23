// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Ticket lifecycle API: open, close, reopen, list, detail.

// Specs:
// - POST /api/v1/tickets with action field dispatches open/close/reopen.
// - GET /api/v1/tickets lists tickets, optionally filtered by status.
// - All mutating operations require authenticated trusted developer.
// - Events are published for all lifecycle transitions.
// #endregion 🧲Header

// #region 🔌Adapters
import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import {
  upsertTicket,
  getTicket,
  listTickets,
  insertTicketFiles,
  listClaimsByTicket,
  type Ticket,
} from "@/lib";
import { requireAuth, isAuthError } from "@/lib";
import { publishEvent } from "@/lib";
// #endregion 🔌Adapters

// 🎫#region 🎄Schemas
const TicketOpenSchema = z.object({
  action: z.literal("open"),
  ticket_id: z.string().min(1),
  title: z.string().min(1),
  prompt: z.string().default(""),
  llm: z.string().default(""),
  client: z.string().default(""),
  author: z.string().default(""),
  github_issue: z.string().default(""),
  goal: z.string().default(""),
  parent: z.string().nullable().default(null),
});

const TicketCloseSchema = z.object({
  action: z.literal("close"),
  ticket_id: z.string().min(1),
  summary: z.string().min(1),
  files: z.array(z.string()).default([]),
});

const TicketReopenSchema = z.object({
  action: z.literal("reopen"),
  ticket_id: z.string().min(1),
  prompt: z.string().min(1),
  llm: z.string().default(""),
  title: z.string().default(""),
  client: z.string().default(""),
});
// #endregion 🎄Schemas

// 🎯#region 🪄Handlers
export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const status = request.nextUrl.searchParams.get("status") || undefined;
  const tickets = await listTickets(status);
  return NextResponse.json(tickets);
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
    case "open": {
      const parsed = TicketOpenSchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json(
          { error: parsed.error.message },
          { status: 400 }
        );
      }
      const data = parsed.data;
      const now = new Date();
      const ticket: Ticket = {
        id: data.ticket_id,
        status: "open",
        title: data.title,
        prompt: data.prompt,
        summary: "",
        llm: data.llm,
        client: data.client,
        author: data.author,
        github_issue: data.github_issue,
        goal: data.goal,
        parent: data.parent,
        created_at: now,
        closed_at: null,
      };
      await upsertTicket(ticket);
      await publishEvent("TicketOpened", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }

    case "close": {
      const parsed = TicketCloseSchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json(
          { error: parsed.error.message },
          { status: 400 }
        );
      }
      const data = parsed.data;
      const ticket = await getTicket(data.ticket_id);
      if (!ticket) {
        return NextResponse.json(
          { error: "ticket not found" },
          { status: 404 }
        );
      }
      const now = new Date();
      ticket.status = "closed";
      ticket.summary = data.summary;
      ticket.closed_at = now;
      await upsertTicket(ticket);
      if (data.files.length > 0) {
        await insertTicketFiles(data.ticket_id, data.files);
      }
      await publishEvent("TicketClosed", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }

    case "reopen": {
      const parsed = TicketReopenSchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json(
          { error: parsed.error.message },
          { status: 400 }
        );
      }
      const data = parsed.data;
      const ticket = await getTicket(data.ticket_id);
      if (!ticket) {
        return NextResponse.json(
          { error: "ticket not found" },
          { status: 404 }
        );
      }
      ticket.status = "open";
      ticket.prompt = data.prompt;
      ticket.llm = data.llm;
      if (data.title) ticket.title = data.title;
      if (data.client) ticket.client = data.client;
      ticket.closed_at = null;
      await upsertTicket(ticket);
      await publishEvent("TicketReopened", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }

    default:
      return NextResponse.json({ error: "unknown action" }, { status: 400 });
  }
}
// #endregion 🪄Handlers
