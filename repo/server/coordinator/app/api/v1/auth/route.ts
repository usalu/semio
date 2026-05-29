// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Auth API: whoami, key management.
// #endregion 🧲Header

// #region 🔌Adapters
import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";
import { randomBytes } from "crypto";
// #endregion 🔌Adapters

import { createApiKey, revokeApiKey, createDeveloper, getDeveloperByEmail, insertAuditLog } from "@/lib/db";
import { requireAuth, requireAdmin, isAuthError, hashApiKey } from "@/lib/auth";

export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  return NextResponse.json({
    id: auth.developer.id,
    email: auth.developer.email,
    github_login: auth.developer.github_login,
    display_name: auth.developer.display_name,
    role: auth.developer.role,
    trusted: auth.developer.trusted,
  });
}

const CreateKeySchema = z.object({
  action: z.literal("create-key"),
  developer_email: z.string().email(),
  label: z.string().default("cli"),
});

const CreateDeveloperSchema = z.object({
  action: z.literal("create-developer"),
  email: z.string().email(),
  github_login: z.string().default(""),
  display_name: z.string().min(1),
  trusted: z.boolean().default(false),
  role: z.enum(["developer", "admin", "owner"]).default("developer"),
});

const RevokeKeySchema = z.object({
  action: z.literal("revoke-key"),
  key_id: z.string().min(1),
});

export async function POST(request: NextRequest) {
  const auth = await requireAdmin(request);
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
        discord_user_id: null,
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
        label: parsed.data.label,
      });
      // Return the raw key only this once - it cannot be retrieved again
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
