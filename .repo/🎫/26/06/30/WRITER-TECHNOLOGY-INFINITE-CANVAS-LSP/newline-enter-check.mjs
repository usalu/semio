/** @emoji 🧪 Enter/newline allowance smoke for jack queries. */
import { join } from "node:path";

const { jackNewlineAllowedAt, writerNewlineAllowedAt } = await import(join(process.cwd(), "writer/core/index.ts"));

const query = "MATCH (a:Piece) RETURN a.name";
if (!jackNewlineAllowedAt(query, 5)) throw new Error("expected newline after MATCH");
if (jackNewlineAllowedAt(query, 2)) throw new Error("expected no newline inside MATCH");
if (!jackNewlineAllowedAt(query, query.indexOf("RETURN") + 6)) throw new Error("expected newline after RETURN");
if (jackNewlineAllowedAt(query, query.indexOf("."))) throw new Error("expected no newline before dot");
if (!writerNewlineAllowedAt("a\nb", "plaintext", 1)) throw new Error("plaintext always allows newline");

console.log("[DEBUG] newline-enter-check ok");
