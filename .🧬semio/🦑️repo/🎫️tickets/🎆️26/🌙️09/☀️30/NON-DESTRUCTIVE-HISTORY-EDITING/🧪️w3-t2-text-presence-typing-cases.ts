/** 🧫️ W3-T2-TEXT: adds the `typing` (flag bit 14) vectors to the presence peer codec corpus and its limit, and moves the
 * unknown-flag vector to bit 15. Every byte is built by hand here (LEB128 + length-prefixed UTF-8), never by the codec under test.
 * Run: `bun T/🧪️w3-t2-text-presence-typing-cases.ts` from the repo root. */
import { readFileSync, writeFileSync } from "node:fs";

const root = "🧰️framework/🔨️modules/📡️replication";
const fixturePath = `${root}/🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json`;
const schemaPath = `${root}/🧬️schema/🔣️.json`;

const varint = (value: number): number[] => {
  const out: number[] = [];
  let rest = BigInt(value);
  for (;;) {
    const byte = Number(rest & 0x7fn);
    rest >>= 7n;
    if (rest === 0n) return out.push(byte), out;
    out.push(byte | 0x80);
  }
};
const str = (text: string): number[] => {
  const bytes = [...new TextEncoder().encode(text)];
  return [...varint(bytes.length), ...bytes];
};
const hex = (bytes: readonly number[]) => Buffer.from(bytes).toString("hex");
const head = (flags: number) => [...str("a"), ...varint(flags), ...varint(1)];
const runs = (entries: readonly (readonly [string, string, string])[]) => [...varint(entries.length), ...entries.flatMap(([window, deleted, insert]) => [...str(window), ...str(deleted), ...str(insert)])];

const accepted = (id: string, bytes: number[], expected: object) => ({ id, prefixHex: hex(bytes), repeatHex: "00", repeatCount: 0, suffixHex: "", accepted: true, canonicalHex: hex(bytes), expected });
const denied = (id: string, bytes: number[]) => ({ id, prefixHex: hex(bytes), repeatHex: "00", repeatCount: 0, suffixHex: "", accepted: false });

const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
fixture.limits.maximumTypingRuns = 8;
const typingCases = [
  accepted("typing-two-windows", [...head(1 << 14), ...runs([["writer.main", "", "hello wör"], ["query", "MATCH", "RETURN"]])], { actor: "a", connectedAtMs: 1, views: [], typing: [{ windowId: "writer.main", deleted: "", insert: "hello wör" }, { windowId: "query", deleted: "MATCH", insert: "RETURN" }] }),
  denied("typing-empty-flagged-list", [...head(1 << 14), ...varint(0)]),
  denied("typing-over-run-limit", [...head(1 << 14), ...runs(Array.from({ length: 9 }, () => ["w", "", "x"] as const))]),
  denied("typing-excerpt-over-limit", [...head(1 << 14), ...runs([["w", "", "x".repeat(257)]])]),
  denied("typing-truncated", [...head(1 << 14), ...varint(1), ...str("w")]),
];
fixture.cases = fixture.cases.filter((row: { id: string }) => !row.id.startsWith("typing-")).map((row: { id: string }) => (row.id === "unknown-flag" ? denied("unknown-flag", [...str("a"), ...varint(1 << 15), ...varint(1)]) : row));
const at = fixture.cases.findIndex((row: { id: string }) => row.id === "unknown-flag");
fixture.cases.splice(at, 0, ...typingCases);
writeFileSync(fixturePath, JSON.stringify(fixture, null, 2) + "\n");

const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
const limits = schema.definitions.limits;
if (!limits.required.includes("maximumTypingRuns")) limits.required.push("maximumTypingRuns");
limits.properties.maximumTypingRuns = { const: 8 };
writeFileSync(schemaPath, JSON.stringify(schema, null, 2) + "\n");
console.log(`presence typing cases: ${typingCases.length}; unknown-flag moved to bit 15`);
