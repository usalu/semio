// @bun
/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/event/route.ts */
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

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/event/route.ts */
import { listEvents } from "@/lib";
import { requireAuth, isAuthError } from "@/lib";
import { publishEvent } from "@/lib";
var EventSchema = ownedSchema.object({
  kind: ownedSchema.string().min(1),
  source: ownedSchema.string().default(""),
  payload: ownedSchema.unknown().default({})
});
async function GET(request) {
  const auth = await requireAuth(request);
  if (isAuthError(auth))
    return auth;
  const kind = request.nextUrl.searchParams.get("kind") || undefined;
  const limit = parseInt(request.nextUrl.searchParams.get("limit") || "100");
  const events = await listEvents(kind, limit);
  return NextResponse.json(events);
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
  const parsed = EventSchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ error: parsed.error.message }, { status: 400 });
  }
  const event = await publishEvent(parsed.data.kind, parsed.data.source, parsed.data.payload);
  return NextResponse.json({ status: "ok", event_id: event.id });
}
export {
  POST,
  GET
};
