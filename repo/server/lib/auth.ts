// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Authentication and authorization layer for the repo server. Enforces trusted developer access.

// Specs:
// - CLI uses API keys (Bearer token auth).
// - API keys are hashed with SHA-256 before storage.
// - Every request must resolve to a trusted, active developer.
// - Admin routes require role 'admin' or 'owner'.
// - No fallback "allow all" behavior.
// #endregion 🧲Header

// #region 🔌Adapters
import { createHash } from "crypto";
import { NextRequest, NextResponse } from "next/server";
import { getDeveloperByApiKeyHash, type Developer } from "./db";
// #endregion 🔌Adapters

// 🔷#region 🩻Hashing
export function hashApiKey(key: string): string {
  return createHash("sha256").update(key).digest("hex");
}
// #endregion 🩻Hashing

// #region 📎Auth
// Authenticate a request by extracting the Bearer token and resolving to a developer.

export async function authenticateRequest(
  request: NextRequest
): Promise<Developer | null> {
  const authHeader = request.headers.get("Authorization");
  if (!authHeader) return null;
  const parts = authHeader.split(" ");
  if (parts.length !== 2 || parts[0] !== "Bearer") return null;
  const apiKey = parts[1];
  if (!apiKey) return null;
  const keyHash = hashApiKey(apiKey);
  const developer = await getDeveloperByApiKeyHash(keyHash);
  if (!developer) return null;
  if (!developer.active || !developer.trusted) return null;
  return developer;
}

export function unauthorizedResponse(message: string = "unauthorized"): NextResponse {
  return NextResponse.json({ error: message }, { status: 401 });
}

export function forbiddenResponse(message: string = "forbidden"): NextResponse {
  return NextResponse.json({ error: message }, { status: 403 });
}

// 🔐Require authentication and trusted developer status.
export async function requireAuth(
  request: NextRequest
): Promise<{ developer: Developer } | NextResponse> {
  const developer = await authenticateRequest(request);
  if (!developer) return unauthorizedResponse();
  return { developer };
}

// 👑Require admin or owner role.
export async function requireAdmin(
  request: NextRequest
): Promise<{ developer: Developer } | NextResponse> {
  const developer = await authenticateRequest(request);
  if (!developer) return unauthorizedResponse();
  if (developer.role !== "admin" && developer.role !== "owner") {
    return forbiddenResponse("admin access required");
  }
  return { developer };
}

// 📩Type guard to check if auth result is an error response.
export function isAuthError(
  result: { developer: Developer } | NextResponse
): result is NextResponse {
  return result instanceof NextResponse;
}
// #endregion 📎Auth
