// @bun
/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/auth/route.ts */
import { NextResponse } from "next/server";
import { randomBytes } from "crypto";

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

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/auth/route.ts */
import { createApiKey, revokeApiKey, createDeveloper, getDeveloperByEmail, insertAuditLog } from "@/lib";
import { requireAuth, requireAdmin, isAuthError, hashApiKey } from "@/lib";
async function GET(request) {
  const auth = await requireAuth(request);
  if (isAuthError(auth))
    return auth;
  return NextResponse.json({
    id: auth.developer.id,
    email: auth.developer.email,
    github_login: auth.developer.github_login,
    display_name: auth.developer.display_name,
    role: auth.developer.role,
    trusted: auth.developer.trusted
  });
}
var CreateKeySchema = ownedSchema.object({
  action: ownedSchema.literal("create-key"),
  developer_email: ownedSchema.string().email(),
  label: ownedSchema.string().default("cli")
});
var CreateDeveloperSchema = ownedSchema.object({
  action: ownedSchema.literal("create-developer"),
  email: ownedSchema.string().email(),
  github_login: ownedSchema.string().default(""),
  display_name: ownedSchema.string().min(1),
  trusted: ownedSchema.boolean().default(false),
  role: ownedSchema.enum(["developer", "admin", "owner"]).default("developer")
});
var RevokeKeySchema = ownedSchema.object({
  action: ownedSchema.literal("revoke-key"),
  key_id: ownedSchema.string().min(1)
});
async function POST(request) {
  const auth = await requireAdmin(request);
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
    case "create-developer": {
      const parsed = CreateDeveloperSchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const dev = await createDeveloper({
        email: parsed.data.email,
        github_login: parsed.data.github_login || null,
        display_name: parsed.data.display_name,
        trusted: parsed.data.trusted,
        active: true,
        role: parsed.data.role,
        discord_user_id: null
      });
      await insertAuditLog(auth.developer.id, "developer.created", dev.id, { email: dev.email });
      return NextResponse.json(dev);
    }
    case "create-key": {
      const parsed = CreateKeySchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const dev = await getDeveloperByEmail(parsed.data.developer_email);
      if (!dev) {
        return NextResponse.json({ error: "developer not found" }, { status: 404 });
      }
      const rawKey = randomBytes(32).toString("hex");
      const keyHash = hashApiKey(rawKey);
      const apiKey = await createApiKey(dev.id, keyHash, parsed.data.label);
      await insertAuditLog(auth.developer.id, "api_key.created", apiKey.id, {
        developer: dev.email,
        label: parsed.data.label
      });
      return NextResponse.json({ key: rawKey, id: apiKey.id, label: apiKey.label });
    }
    case "revoke-key": {
      const parsed = RevokeKeySchema.safeParse(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      await revokeApiKey(parsed.data.key_id);
      await insertAuditLog(auth.developer.id, "api_key.revoked", parsed.data.key_id, {});
      return NextResponse.json({ status: "ok" });
    }
    default:
      return NextResponse.json({ error: "unknown action" }, { status: 400 });
  }
}
export {
  POST,
  GET
};
