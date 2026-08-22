// @bun
/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/ticket/route.ts */
import { NextResponse } from "next/server";

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🟦️validation.ts */
class OwnedSchema {
  parseValue;
  constructor(parseValue) {
    this.parseValue = parseValue;
  }
  safeParse(value) {
    const parsed = this.parseValue(value, "value");
    if ("message" in parsed)
      return { success: false, error: { message: parsed.message } };
    return parsed;
  }
  default(defaultValue) {
    return new OwnedSchema((value, path) => value === undefined ? { success: true, data: defaultValue } : this.parseValue(value, path));
  }
  nullable() {
    return new OwnedSchema((value, path) => value === null ? { success: true, data: null } : this.parseValue(value, path));
  }
  parse(value, path) {
    return this.parseValue(value, path);
  }
}

class OwnedStringSchema extends OwnedSchema {
  minimum;
  emailRequired;
  constructor(minimum = 0, emailRequired = false) {
    super((value, path) => {
      if (typeof value !== "string")
        return { success: false, message: `${path}: expected string` };
      if (value.length < minimum)
        return { success: false, message: `${path}: expected at least ${minimum} character(s)` };
      if (emailRequired && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value))
        return { success: false, message: `${path}: invalid email address` };
      return { success: true, data: value };
    });
    this.minimum = minimum;
    this.emailRequired = emailRequired;
  }
  min(minimum) {
    return new OwnedStringSchema(minimum, this.emailRequired);
  }
  email() {
    return new OwnedStringSchema(this.minimum, true);
  }
}
function ownedObject(shape) {
  return new OwnedSchema((value, path) => {
    if (typeof value !== "object" || value === null || Array.isArray(value))
      return { success: false, message: `${path}: expected object` };
    const source = value;
    const data = {};
    for (const [key, schema] of Object.entries(shape)) {
      const parsed = schema.parse(source[key], `${path}.${key}`);
      if ("message" in parsed)
        return { success: false, message: parsed.message };
      data[key] = parsed.data;
    }
    return { success: true, data };
  });
}
var ownedSchema = {
  string: () => new OwnedStringSchema,
  boolean: () => new OwnedSchema((value, path) => typeof value === "boolean" ? { success: true, data: value } : { success: false, message: `${path}: expected boolean` }),
  literal: (expected) => new OwnedSchema((value, path) => value === expected ? { success: true, data: expected } : { success: false, message: `${path}: expected ${JSON.stringify(expected)}` }),
  enum: (values) => new OwnedSchema((value, path) => typeof value === "string" && values.includes(value) ? { success: true, data: value } : { success: false, message: `${path}: expected one of ${values.join(", ")}` }),
  unknown: () => new OwnedSchema((value) => ({ success: true, data: value })),
  array: (item) => new OwnedSchema((value, path) => {
    if (!Array.isArray(value))
      return { success: false, message: `${path}: expected array` };
    const data = [];
    for (let index = 0;index < value.length; index++) {
      const parsed = item.parse(value[index], `${path}.${index}`);
      if ("message" in parsed)
        return { success: false, message: parsed.message };
      data.push(parsed.data);
    }
    return { success: true, data };
  }),
  object: ownedObject
};

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/ticket/route.ts */
import { upsertTicket, getTicket, listTickets, insertTicketFiles } from "@/lib";
import { requireAuth, isAuthError } from "@/lib";
import { publishEvent } from "@/lib";
var TicketOpenSchema = ownedSchema.object({
  action: ownedSchema.literal("open"),
  ticket_id: ownedSchema.string().min(1),
  title: ownedSchema.string().min(1),
  prompt: ownedSchema.string().default(""),
  llm: ownedSchema.string().default(""),
  client: ownedSchema.string().default(""),
  author: ownedSchema.string().default(""),
  github_issue: ownedSchema.string().default(""),
  goal: ownedSchema.string().default(""),
  parent: ownedSchema.string().nullable().default(null)
});
var TicketCloseSchema = ownedSchema.object({
  action: ownedSchema.literal("close"),
  ticket_id: ownedSchema.string().min(1),
  summary: ownedSchema.string().min(1),
  files: ownedSchema.array(ownedSchema.string()).default([])
});
var TicketReopenSchema = ownedSchema.object({
  action: ownedSchema.literal("reopen"),
  ticket_id: ownedSchema.string().min(1),
  prompt: ownedSchema.string().min(1),
  llm: ownedSchema.string().default(""),
  title: ownedSchema.string().default(""),
  client: ownedSchema.string().default("")
});
async function GET(request) {
  const auth = await requireAuth(request);
  if (isAuthError(auth))
    return auth;
  const status = request.nextUrl.searchParams.get("status") || undefined;
  const tickets = await listTickets(status);
  return NextResponse.json(tickets);
}
async function POST(request) {
  const auth = await requireAuth(request);
  if (isAuthError(auth))
    return auth;
  let body;
  try {
    body = await request.json();
  } catch {
    return NextResponse.json({ error: "invalid JSON" }, { status: 400 });
  }
  const actionCheck = ownedSchema.object({ action: ownedSchema.string() }).safeParse(body);
  if (!actionCheck.success) {
    return NextResponse.json({ error: "action required" }, { status: 400 });
  }
  switch (actionCheck.data.action) {
    case "open": {
      const parsed = TicketOpenSchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const data = parsed.data;
      const now = new Date;
      const ticket = {
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
        closed_at: null
      };
      await upsertTicket(ticket);
      await publishEvent("TicketOpened", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }
    case "close": {
      const parsed = TicketCloseSchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const data = parsed.data;
      const ticket = await getTicket(data.ticket_id);
      if (!ticket) {
        return NextResponse.json({ error: "ticket not found" }, { status: 404 });
      }
      const now = new Date;
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
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const data = parsed.data;
      const ticket = await getTicket(data.ticket_id);
      if (!ticket) {
        return NextResponse.json({ error: "ticket not found" }, { status: 404 });
      }
      ticket.status = "open";
      ticket.prompt = data.prompt;
      ticket.llm = data.llm;
      if (data.title)
        ticket.title = data.title;
      if (data.client)
        ticket.client = data.client;
      ticket.closed_at = null;
      await upsertTicket(ticket);
      await publishEvent("TicketReopened", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }
    default:
      return NextResponse.json({ error: "unknown action" }, { status: 400 });
  }
}
export {
  POST,
  GET
};
