// Ticket-scoped verification: exercises the exact bun:sqlite schema/query pattern used by
// semioBlobVitePlugin() in framework/product/os/dev/script.ts (INSERT OR IGNORE dedupe + BLOB column
// round-trip via a Buffer param), against an in-memory database — deliberately not touching the real
// repoRoot/.semio/blobs.db path (that's the production side effect location, not a ticket temp file).
import { Database } from "bun:sqlite";

const db = new Database(":memory:");
db.run("CREATE TABLE IF NOT EXISTS blob (hash TEXT PRIMARY KEY, media_type TEXT NOT NULL, size INTEGER NOT NULL, bytes BLOB NOT NULL)");

const bytes = Buffer.from("hello content-addressed world");
const hash = "deadbeef";
db.run("INSERT OR IGNORE INTO blob (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)", [hash, "text/plain", bytes.length, bytes]);
// Re-put identical hash with different bytes to confirm dedupe (INSERT OR IGNORE keeps the first row).
db.run("INSERT OR IGNORE INTO blob (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)", [hash, "text/plain", 999, Buffer.from("different")]);

const row = db.query("SELECT media_type, bytes FROM blob WHERE hash = ?1").get(hash) as { media_type?: string; bytes?: Uint8Array } | null;
if (!row) throw new Error("row missing after insert");
const roundTripped = Buffer.from(row.bytes ?? new Uint8Array()).toString();
console.log("roundTripped:", roundTripped);
console.log("mediaType:", row.media_type);
if (roundTripped !== bytes.toString()) throw new Error(`byte round-trip mismatch: got ${roundTripped}`);
if (row.media_type !== "text/plain") throw new Error(`media_type mismatch: got ${row.media_type}`);

const count = db.query("SELECT COUNT(*) as c FROM blob").get() as { c: number };
if (count.c !== 1) throw new Error(`dedupe failed: expected 1 row, got ${count.c}`);

console.log("[DEBUG] blob sqlite schema/dedupe verification passed");
