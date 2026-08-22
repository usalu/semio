// @bun
/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/diff/route.ts */
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

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/diff/route.ts */
import { replaceScopes, upsertClaim, listConflicts, replaceWarnings, newId } from "@/lib";
import { requireAuth, isAuthError } from "@/lib";
import { publishEvent } from "@/lib";
import { parseUnifiedDiff, buildScopesForFile } from "@/lib";
import { readFileSync } from "fs";
import { join } from "path";
var REPO_ROOT = process.env.COMPOSE_SERVER_REPO_ROOT || process.cwd();
var DiffIngestSchema = ownedSchema.object({
  ticket_id: ownedSchema.string().min(1),
  repo_id: ownedSchema.string().default(""),
  patch: ownedSchema.string().min(1),
  snapshots: ownedSchema.array(ownedSchema.object({ path: ownedSchema.string(), content: ownedSchema.string() })).default([])
});
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
  const parsed = DiffIngestSchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ error: parsed.error.message }, { status: 400 });
  }
  const { ticket_id, patch, snapshots } = parsed.data;
  const diffFiles = parseUnifiedDiff(patch);
  const changedFiles = [...new Set(diffFiles.map((f) => f.path).filter(Boolean))];
  await publishEvent("DiffIngested", "repo-cli", {
    ticket_id,
    files: changedFiles
  });
  const contentByFile = {};
  for (const s of snapshots) {
    contentByFile[s.path] = s.content;
  }
  const allScopes = [];
  for (const file of changedFiles) {
    let content = contentByFile[file];
    if (!content) {
      try {
        content = readFileSync(join(REPO_ROOT, file), "utf-8");
      } catch {
        continue;
      }
    }
    const scopes = buildScopesForFile(file, content);
    await replaceScopes(file, scopes);
    allScopes.push(...scopes);
  }
  const claimedIds = [];
  for (const diffFile of diffFiles) {
    if (!diffFile.path)
      continue;
    const fileScopes = allScopes.filter((s) => s.file_path === diffFile.path);
    for (const hunk of diffFile.hunks) {
      if (hunk.newRange.end === 0)
        continue;
      for (const scope of fileScopes) {
        if (scope.start_line === 0 && scope.end_line === 0)
          continue;
        if (hunk.newRange.start <= scope.end_line && scope.start_line <= hunk.newRange.end) {
          if (scope.kind === "definition" || scope.kind === "section") {
            if (!claimedIds.includes(scope.id)) {
              claimedIds.push(scope.id);
            }
            await upsertClaim(ticket_id, scope.id, "touched");
          }
        }
      }
    }
  }
  const conflicts = await listConflicts();
  const warnings = conflicts.map((c) => ({
    id: newId(),
    kind: "conflict",
    severity: "error",
    message: `conflict on ${c.scope_id} across tickets ${c.tickets.join(", ")}`,
    ticket_id: "",
    scope_id: c.scope_id,
    created_at: new Date,
    acknowledged_at: null,
    ack_by: ""
  }));
  await replaceWarnings(warnings);
  const blockers = warnings.filter((w) => w.severity === "error").map((w) => w.message);
  return NextResponse.json({
    changed_files: changedFiles,
    claimed_scopes: claimedIds,
    warnings,
    breachs: [],
    blockers
  });
}
export {
  POST
};
