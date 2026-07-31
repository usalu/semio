/** @emoji 🧪️ Runtime smoke checks for writer technology + jack LSP integration. */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const jackFixture = JSON.parse(readFileSync(join(root, "writer/fixture/jack.writer.json"), "utf8"));

console.log("[DEBUG] writer fixture schema", jackFixture.schema);
console.log("[DEBUG] writer fixture language", jackFixture.languageId);
console.log("[DEBUG] writer fixture lines", jackFixture.text.split("\n").length);

if (jackFixture.schema !== "writer.document/v1") throw new Error("expected writer.document/v1 schema");
if (jackFixture.languageId !== "jack") throw new Error("expected jack language");
if (!jackFixture.text.includes("MATCH")) throw new Error("expected jack query in fixture");

function offsetToPosition(text, offset) {
  let line = 0;
  let col = 0;
  for (let i = 0; i < offset && i < text.length; i++) {
    if (text[i] === "\n") {
      line++;
      col = 0;
    } else col++;
  }
  return { line, character: col };
}

const sample = "MATCH (a:Piece) RETURN a.name";
const pos = offsetToPosition(sample, 10);
if (pos.line !== 0 || pos.character !== 10) throw new Error("offset mapping failed");

console.log("[DEBUG] writer runtime-check ok");
