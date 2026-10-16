import ts from "typescript";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const FOLDER = join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const originalText = readFileSync(join(FOLDER, "met-tools/time-travel-original.ts.txt"), "utf8");
const files = readFileSync(join(FOLDER, "met-tools/tt-new-files.txt"), "utf8").split("\n").filter(Boolean);

const namesOf = (statement: ts.Statement): string[] => {
  const out: string[] = [];
  if (ts.isVariableStatement(statement)) {
    for (const declaration of statement.declarationList.declarations) {
      if (ts.isIdentifier(declaration.name)) out.push(declaration.name.text);
      else if (ts.isObjectBindingPattern(declaration.name) || ts.isArrayBindingPattern(declaration.name)) for (const element of declaration.name.elements) if (ts.isBindingElement(element) && ts.isIdentifier(element.name)) out.push(element.name.text);
    }
  } else if ((ts.isFunctionDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name) out.push(statement.name.text);
  return out;
};
const normal = (value: string) => value.replace(/\bexport\s+/gu, "").replace(/\s+/gu, " ").trim();

type Row = { names: string[]; text: string; file: string };
const collect = (text: string, file: string): Row[] => {
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return sf.statements.filter((s) => !ts.isImportDeclaration(s)).map((s) => ({ names: namesOf(s), text: normal(text.slice(s.getStart(sf), s.end)), file }));
};
const original = collect(originalText, "original");
const created: Row[] = [];
for (const file of files) created.push(...collect(readFileSync(join(REPO, file), "utf8"), file));

const byName = new Map<string, Row[]>();
for (const row of created) for (const name of row.names) byName.set(name, [...(byName.get(name) ?? []), row]);

let identical = 0;
let changed = 0;
let missing = 0;
const diffs: string[] = [];
for (const row of original) {
  if (row.names.length === 0) continue;
  const candidates = row.names.flatMap((name) => byName.get(name) ?? []);
  if (candidates.length === 0) { missing += 1; diffs.push(`MISSING ${row.names.join(",")}`); continue; }
  if (candidates.some((candidate) => candidate.text === row.text)) { identical += 1; continue; }
  changed += 1;
  const best = candidates[0]!;
  let i = 0;
  while (row.text[i] === best.text[i]) i += 1;
  diffs.push(`CHANGED ${row.names.join(",")} (${best.file.split("/").slice(-2, -1)[0]})\n   orig: ${row.text.slice(Math.max(0, i - 60), i + 140)}\n   new : ${best.text.slice(Math.max(0, i - 60), i + 140)}`);
}
writeFileSync(join(FOLDER, "🗑️generated/met-tt-proof.txt"), diffs.join("\n"));
console.log(JSON.stringify({ originalDeclarations: original.filter((row) => row.names.length).length, identical, changed, missing }));
