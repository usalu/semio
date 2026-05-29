// #region 🧲Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// PostgreSQL database layer for persistent storage of tickets, scopes, claims, warnings, breaches, events, developers, and artifacts.

// Specs:
// - Use pg Pool for connection pooling.
// - Mirror SQLite interface from Go server with PostgreSQL semantics.
// - Use parameterized queries for all inputs.
// - All timestamps are timestamptz.
// #endregion 🧲Header

// #region 🔌Adapters
import { Pool, type PoolClient } from "pg";
// #endregion 🔌Adapters

// #region ⏱️Config
// 🗄️Database configuration from environment variables.
const DATABASE_URL =
  process.env.DATABASE_URL ||
  "postgresql://semio:semio@localhost:5432/semio_repo";

let pool: Pool | null = null;

export function getPool(): Pool {
  if (!pool) {
    pool = new Pool({ connectionString: DATABASE_URL, max: 20 });
  }
  return pool;
}

export async function closePool(): Promise<void> {
  if (pool) {
    await pool.end();
    pool = null;
  }
}
// #endregion ⏱️Config

// #region ⚙️Types
// Data model types matching the PostgreSQL schema.

export interface Developer {
  id: string;
  email: string;
  github_login: string | null;
  display_name: string;
  trusted: boolean;
  active: boolean;
  role: string;
  discord_user_id: string | null;
  created_at: Date;
  revoked_at: Date | null;
}

export interface DeveloperApiKey {
  id: string;
  developer_id: string;
  key_hash: string;
  label: string;
  created_at: Date;
  last_used_at: Date | null;
  revoked_at: Date | null;
}

export interface Ticket {
  id: string;
  status: string;
  title: string;
  prompt: string;
  summary: string;
  llm: string;
  client: string;
  author: string;
  github_issue: string;
  goal: string;
  parent: string | null;
  created_at: Date;
  closed_at: Date | null;
}

export interface Scope {
  id: string;
  kind: string;
  file_path: string;
  section_path: string;
  definition_name: string;
  start_line: number;
  end_line: number;
  updated_at: Date;
}

export interface Warning {
  id: string;
  kind: string;
  severity: string;
  message: string;
  ticket_id: string;
  scope_id: string;
  created_at: Date;
  acknowledged_at: Date | null;
  ack_by: string;
}

export interface Breach {
  id: string;
  kind: string;
  priority: string;
  scope_id: string;
  file_path: string;
  line: number | null;
  col: number | null;
  summary: string;
  excerpt: string;
  autofixable: boolean;
  detected_at: Date;
  ticket_id: string;
  resolved_at: Date | null;
}

export interface Event {
  id: string;
  kind: string;
  source: string;
  payload_json: unknown;
  created_at: Date;
}

export interface Artifact {
  id: string;
  ticket_id: string;
  file_name: string;
  file_size: number;
  checksum: string;
  mime_kind: string;
  blob_path: string;
  uploaded_at: Date;
}
// #endregion ⚙️Types

// 🔷#region 🎼Helpers
function newId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 11)}`;
}
// #endregion 🎼Helpers

// #region 👓Developers
// Developer CRUD operations.

export async function getDeveloperByApiKeyHash(
  keyHash: string
): Promise<Developer | null> {
  const p = getPool();
  const result = await p.query(
    `SELECT d.* FROM developers d
     JOIN developer_api_keys k ON d.id = k.developer_id
     WHERE k.key_hash = $1 AND k.revoked_at IS NULL AND d.active = true`,
    [keyHash]
  );
  if (result.rows.length === 0) return null;
  return result.rows[0] as Developer;
}

export async function getDeveloperByEmail(
  email: string
): Promise<Developer | null> {
  const p = getPool();
  const result = await p.query(`SELECT * FROM developers WHERE email = $1`, [
    email,
  ]);
  if (result.rows.length === 0) return null;
  return result.rows[0] as Developer;
}

export async function listDevelopers(): Promise<Developer[]> {
  const p = getPool();
  const result = await p.query(
    `SELECT * FROM developers ORDER BY created_at DESC`
  );
  return result.rows as Developer[];
}

export async function createDeveloper(
  dev: Omit<Developer, "id" | "created_at" | "revoked_at">
): Promise<Developer> {
  const p = getPool();
  const id = newId();
  const result = await p.query(
    `INSERT INTO developers (id, email, github_login, display_name, trusted, active, role, discord_user_id)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *`,
    [
      id,
      dev.email,
      dev.github_login,
      dev.display_name,
      dev.trusted,
      dev.active,
      dev.role,
      dev.discord_user_id,
    ]
  );
  return result.rows[0] as Developer;
}

export async function createApiKey(
  developerId: string,
  keyHash: string,
  label: string
): Promise<DeveloperApiKey> {
  const p = getPool();
  const id = newId();
  const result = await p.query(
    `INSERT INTO developer_api_keys (id, developer_id, key_hash, label) VALUES ($1, $2, $3, $4) RETURNING *`,
    [id, developerId, keyHash, label]
  );
  return result.rows[0] as DeveloperApiKey;
}

export async function revokeApiKey(keyId: string): Promise<void> {
  const p = getPool();
  await p.query(
    `UPDATE developer_api_keys SET revoked_at = now() WHERE id = $1`,
    [keyId]
  );
}
// #endregion 👓Developers

// #region 📋Tickets
// Ticket CRUD operations.

export async function upsertTicket(ticket: Ticket): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO tickets (id, status, title, prompt, summary, llm, client, author, github_issue, goal, parent, created_at, closed_at)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
     ON CONFLICT(id) DO UPDATE SET
       status=EXCLUDED.status, title=EXCLUDED.title, prompt=EXCLUDED.prompt,
       summary=EXCLUDED.summary, llm=EXCLUDED.llm, client=EXCLUDED.client,
       author=EXCLUDED.author, github_issue=EXCLUDED.github_issue,
       goal=EXCLUDED.goal, parent=EXCLUDED.parent, closed_at=EXCLUDED.closed_at`,
    [
      ticket.id,
      ticket.status,
      ticket.title,
      ticket.prompt,
      ticket.summary,
      ticket.llm,
      ticket.client,
      ticket.author,
      ticket.github_issue,
      ticket.goal,
      ticket.parent,
      ticket.created_at,
      ticket.closed_at,
    ]
  );
}

export async function getTicket(ticketId: string): Promise<Ticket | null> {
  const p = getPool();
  const result = await p.query(`SELECT * FROM tickets WHERE id = $1`, [
    ticketId,
  ]);
  if (result.rows.length === 0) return null;
  return result.rows[0] as Ticket;
}

export async function listTickets(status?: string): Promise<Ticket[]> {
  const p = getPool();
  if (status) {
    const result = await p.query(
      `SELECT * FROM tickets WHERE status = $1 ORDER BY created_at DESC`,
      [status]
    );
    return result.rows as Ticket[];
  }
  const result = await p.query(
    `SELECT * FROM tickets ORDER BY created_at DESC`
  );
  return result.rows as Ticket[];
}

export async function insertTicketFiles(
  ticketId: string,
  files: string[]
): Promise<void> {
  const p = getPool();
  for (const filePath of files) {
    await p.query(
      `INSERT INTO ticket_files (ticket_id, file_path) VALUES ($1, $2) ON CONFLICT DO NOTHING`,
      [ticketId, filePath]
    );
  }
}
// #endregion 📋Tickets

// #region 🧩Scopes
// Scope indexing and claim operations.

export async function replaceScopes(
  filePath: string,
  scopes: Scope[]
): Promise<void> {
  const p = getPool();
  await p.query(`DELETE FROM scopes WHERE file_path = $1`, [filePath]);
  for (const scope of scopes) {
    await p.query(
      `INSERT INTO scopes (id, kind, file_path, section_path, definition_name, start_line, end_line, updated_at)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
      [
        scope.id,
        scope.kind,
        scope.file_path,
        scope.section_path,
        scope.definition_name,
        scope.start_line,
        scope.end_line,
        scope.updated_at,
      ]
    );
  }
}

export async function listScopesByFile(filePath: string): Promise<Scope[]> {
  const p = getPool();
  const result = await p.query(
    `SELECT * FROM scopes WHERE file_path = $1`,
    [filePath]
  );
  return result.rows as Scope[];
}

export async function upsertClaim(
  ticketId: string,
  scopeId: string,
  claimKind: string
): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO ticket_claims (ticket_id, scope_id, claim_kind, first_seen_at, last_seen_at)
     VALUES ($1, $2, $3, now(), now())
     ON CONFLICT(ticket_id, scope_id) DO UPDATE SET claim_kind=EXCLUDED.claim_kind, last_seen_at=now()`,
    [ticketId, scopeId, claimKind]
  );
}

export async function listClaimsByTicket(ticketId: string): Promise<Scope[]> {
  const p = getPool();
  const result = await p.query(
    `SELECT s.* FROM scopes s
     JOIN ticket_claims tc ON s.id = tc.scope_id
     WHERE tc.ticket_id = $1`,
    [ticketId]
  );
  return result.rows as Scope[];
}

export async function listConflicts(): Promise<
  { scope_id: string; tickets: string[] }[]
> {
  const p = getPool();
  const result = await p.query(
    `SELECT tc.scope_id, array_agg(tc.ticket_id) as tickets
     FROM ticket_claims tc
     JOIN tickets t ON tc.ticket_id = t.id
     WHERE t.status = 'open'
     GROUP BY tc.scope_id
     HAVING COUNT(tc.ticket_id) > 1`
  );
  return result.rows.map((r: { scope_id: string; tickets: string[] }) => ({
    scope_id: r.scope_id,
    tickets: r.tickets,
  }));
}
// #endregion 🧩Scopes

// #region 🎊Warnings
// Warning and breach operations.

export async function replaceWarnings(warnings: Warning[]): Promise<void> {
  const p = getPool();
  await p.query(`DELETE FROM warnings WHERE kind = 'conflict'`);
  for (const w of warnings) {
    await p.query(
      `INSERT INTO warnings (id, kind, severity, message, ticket_id, scope_id, created_at, acknowledged_at, ack_by)
       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
      [
        w.id,
        w.kind,
        w.severity,
        w.message,
        w.ticket_id,
        w.scope_id,
        w.created_at,
        w.acknowledged_at,
        w.ack_by,
      ]
    );
  }
}

export async function listWarnings(ticketId?: string): Promise<Warning[]> {
  const p = getPool();
  if (ticketId) {
    const result = await p.query(
      `SELECT * FROM warnings WHERE ticket_id = $1 ORDER BY created_at DESC`,
      [ticketId]
    );
    return result.rows as Warning[];
  }
  const result = await p.query(
    `SELECT * FROM warnings ORDER BY created_at DESC`
  );
  return result.rows as Warning[];
}

export async function listBreaches(ticketId?: string): Promise<Breach[]> {
  const p = getPool();
  if (ticketId) {
    const result = await p.query(
      `SELECT * FROM breaches WHERE ticket_id = $1 ORDER BY detected_at DESC`,
      [ticketId]
    );
    return result.rows as Breach[];
  }
  const result = await p.query(
    `SELECT * FROM breaches ORDER BY detected_at DESC`
  );
  return result.rows as Breach[];
}
// #endregion 🎊Warnings

// #region ⛅Events
// Event persistence operations.

export async function insertEvent(event: Event): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO events (id, kind, source, payload_json, created_at)
     VALUES ($1, $2, $3, $4, $5)`,
    [
      event.id,
      event.kind,
      event.source,
      JSON.stringify(event.payload_json),
      event.created_at,
    ]
  );
}

export async function listEvents(
  kind?: string,
  limit: number = 100
): Promise<Event[]> {
  const p = getPool();
  if (kind) {
    const result = await p.query(
      `SELECT * FROM events WHERE kind = $1 ORDER BY created_at DESC LIMIT $2`,
      [kind, limit]
    );
    return result.rows as Event[];
  }
  const result = await p.query(
    `SELECT * FROM events ORDER BY created_at DESC LIMIT $1`,
    [limit]
  );
  return result.rows as Event[];
}

export { newId };
// #endregion ⛅Events

// #region 🗂️ContributorWork
// Contributor work tracking for conflict detection.

export async function addContributorWork(
  github: string,
  kind: string,
  itemId: string
): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO contributor_work (github, kind, item_id) VALUES ($1, $2, $3)
     ON CONFLICT DO NOTHING`,
    [github, kind, itemId]
  );
}

export async function removeContributorWork(
  github: string,
  entries: { kind: string; id: string }[]
): Promise<void> {
  const p = getPool();
  for (const e of entries) {
    await p.query(
      `DELETE FROM contributor_work WHERE github = $1 AND kind = $2 AND item_id = $3`,
      [github, e.kind, e.id]
    );
  }
}

export async function listContributorsOnItem(
  kind: string,
  itemId: string
): Promise<string[]> {
  const p = getPool();
  const result = await p.query(
    `SELECT github FROM contributor_work WHERE kind = $1 AND item_id = $2`,
    [kind, itemId]
  );
  return result.rows.map((r: { github: string }) => r.github);
}

export async function removeContributorWorkForCheckpoint(
  github: string,
  files: string[]
): Promise<void> {
  const p = getPool();
  for (const f of files) {
    await p.query(
      `DELETE FROM contributor_work WHERE github = $1 AND kind = 'file' AND item_id = $2`,
      [github, f]
    );
  }
}
// #endregion 🗂️ContributorWork

// #region 📌Artifacts
// Artifact storage operations.

export async function insertArtifact(artifact: Artifact): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO artifacts (id, ticket_id, file_name, file_size, checksum, mime_kind, blob_path, uploaded_at)
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
    [
      artifact.id,
      artifact.ticket_id,
      artifact.file_name,
      artifact.file_size,
      artifact.checksum,
      artifact.mime_kind,
      artifact.blob_path,
      artifact.uploaded_at,
    ]
  );
}

export async function listArtifactsByTicket(
  ticketId: string
): Promise<Artifact[]> {
  const p = getPool();
  const result = await p.query(
    `SELECT * FROM artifacts WHERE ticket_id = $1 ORDER BY uploaded_at DESC`,
    [ticketId]
  );
  return result.rows as Artifact[];
}
// #endregion 📌Artifacts

// 📌#region 📜AuditLog
export async function insertAuditLog(
  actorId: string | null,
  action: string,
  target: string | null,
  detail: unknown
): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO audit_log (actor_id, action, target, detail) VALUES ($1, $2, $3, $4)`,
    [actorId, action, target, detail ? JSON.stringify(detail) : null]
  );
}
// #endregion 📜AuditLog

// 📌#region 🔷Discord
export async function insertDiscordDelivery(
  eventId: string,
  channelId: string
): Promise<void> {
  const p = getPool();
  await p.query(
    `INSERT INTO discord_deliveries (event_id, channel_id) VALUES ($1, $2)`,
    [eventId, channelId]
  );
}

export async function markDiscordDeliverySent(id: string): Promise<void> {
  const p = getPool();
  await p.query(
    `UPDATE discord_deliveries SET status = 'sent', sent_at = now() WHERE id = $1`,
    [id]
  );
}

export async function markDiscordDeliveryFailed(
  id: string,
  error: string,
  attempts: number
): Promise<void> {
  const p = getPool();
  const status = attempts >= 4 ? "dead" : "failed";
  await p.query(
    `UPDATE discord_deliveries SET status = $1, last_error = $2, attempts = $3 WHERE id = $4`,
    [status, error, attempts, id]
  );
}
// #endregion 🔷Discord

// #region 🎞️Schema
// Run schema migration from schema.sql.
import { readFileSync } from "fs";
import { join } from "path";

export async function runSchema(): Promise<void> {
  const p = getPool();
  const schemaPath = join(__dirname, "../../postgres/schema.sql");
  const schema = readFileSync(schemaPath, "utf-8");
  await p.query(schema);
}
// #endregion 🎞️Schema
