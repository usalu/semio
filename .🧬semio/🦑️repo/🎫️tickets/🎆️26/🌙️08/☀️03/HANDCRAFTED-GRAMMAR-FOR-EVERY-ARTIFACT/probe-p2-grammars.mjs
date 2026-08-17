import { readFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const familyRoot = join(
  ticketDir,
  "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family",
);

const families = [
  { label: "graph", rel: "🕸️graph/📖️family-graph.grammar.semio", id: "family-graph" },
  { label: "scene", rel: "🎬️scene/📖️family-scene.grammar.semio", id: "family-scene" },
  { label: "sheet", rel: "📊️sheet/📖️family-sheet.grammar.semio", id: "family-sheet" },
  { label: "catalog", rel: "🗂️catalog/📖️family-catalog.grammar.semio", id: "family-catalog" },
  { label: "recipe", rel: "🧑‍🍳️recipe/📖️family-recipe.grammar.semio", id: "family-recipe" },
  { label: "geo", rel: "🌍️geo/📖️family-geo.grammar.semio", id: "family-geo" },
  { label: "embed", rel: "📎️embed/📖️family-embed.grammar.semio", id: "family-embed" },
];

const forbiddenCatchAll =
  /^\s*prop\s*=\s*IDENT\s*=\s*\(TEXT\s*\|\s*FLOAT\s*\|\s*INT\s*\|\s*BOOL/;

let failed = false;
const lines = [];

for (const { label, rel, id } of families) {
  let familyFailed = false;
  const path = join(familyRoot, rel);
  if (!existsSync(path)) {
    lines.push(`FAIL ${label}: missing ${path}`);
    familyFailed = true;
  } else {
    const text = readFileSync(path, "utf8");
    const body = text.trimEnd();
    if (!body.startsWith("dialect grammar\n")) {
      lines.push(`FAIL ${label}: missing dialect grammar header`);
      familyFailed = true;
    }
    const grammarLine = body.split("\n").find((l) => l.startsWith("grammar "));
    if (!grammarLine || grammarLine.trim() !== `grammar ${id}`) {
      lines.push(`FAIL ${label}: expected grammar ${id}, got ${grammarLine ?? "none"}`);
      familyFailed = true;
    }
    const startLine = body.split("\n").find((l) => l.startsWith("start "));
    if (!startLine) {
      lines.push(`FAIL ${label}: missing start directive`);
      familyFailed = true;
    }
    const productions = body
      .split("\n")
      .filter((l) => /^[A-Za-z][\w-]*\s*=/.test(l) && !l.startsWith("grammar "));
    if (productions.length < 5) {
      lines.push(`FAIL ${label}: only ${productions.length} productions (expected >5)`);
      familyFailed = true;
    }
    const catchAll = body.split("\n").filter((l) => forbiddenCatchAll.test(l));
    if (catchAll.length) {
      lines.push(`FAIL ${label}: generic prop catch-all: ${catchAll[0].trim()}`);
      familyFailed = true;
    }
    if (!familyFailed) {
      lines.push(
        `ok ${label} id=${id} productions=${productions.length} start=${startLine?.trim() ?? "?"}`,
      );
    }
  }
  if (familyFailed) failed = true;
}

const out =
  lines.join("\n") +
  (failed ? "\n\nSTRUCTURAL CHECK FAILED\n" : "\n\nSTRUCTURAL CHECK PASSED\n");
console.log(out);
process.exit(failed ? 1 : 0);
