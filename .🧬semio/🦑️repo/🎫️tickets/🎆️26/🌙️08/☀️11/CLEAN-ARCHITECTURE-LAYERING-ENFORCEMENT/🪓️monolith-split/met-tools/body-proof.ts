import ts from "typescript";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const FOLDER = join(REPO, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const ORIGINAL = join(FOLDER, "met-tools/gis-original.ts.txt");
const ORIGINAL_DIR = join(REPO, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests");
const OS_FILE = resolve(ORIGINAL_DIR, "../../../../../../🧰️framework/🛍️products/💻️os/🟦️.ts");
const OS_DIR = dirname(OS_FILE);
const FE = "️";
const FIXTURES_TESTS = `\u{1F9EA}${FE}tests/\u{1F9EB}${FE}fixtures`;
const FIXTURES = `\u{1F9EB}${FE}fixtures`;
const ADMIN_OLD = `\u{1F9EC}${FE}schema/\u{1F3DB}${FE}administration/`;
const ADMIN_NEW = `\u{1F3DB}${FE}administration/`;
const NEUTRAL: readonly (readonly [string, string])[] = [
  ["s.gis.gismap", "s.demo.demo"], ['"GIS Map"', '"Demo Map"'], ['"GIS-Karte"', '"Demo-Karte"'], ["gis.map.operation", "demo.map.operation"], ["s.cad.cad", "s.demo.demo"], ['pluginId: "cad"', 'pluginId: "demo"'], ["s.note.note@1", "s.demo.demo@1"], ["noteShellCommand", "demoShellCommand"], ['"semio:gis"', '"semio:demo"'], ['profile: "wasm-release"', 'profile: "release"'],
];

type Test = { readonly key: string; readonly body: string };

const collect = (file: string, text: string): Test[] => {
  const sf = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const fileDir = dirname(file);
  const out: Test[] = [];
  const normalizeCallback = (node: ts.Node): string => {
    const start = node.getStart(sf);
    const replacements: { start: number; end: number; value: string }[] = [];
    const absolute = (literal: ts.StringLiteralLike, base: string) => {
      const value = literal.text;
      if (!value.startsWith(".")) return;
      replacements.push({ start: literal.getStart(sf) + 1, end: literal.end - 1, value: "«" + relative(REPO, resolve(base, value)) + "»" });
    };
    const walk = (current: ts.Node) => {
      if (ts.isCallExpression(current) && current.expression.kind === ts.SyntaxKind.ImportKeyword && current.arguments[0] && ts.isStringLiteralLike(current.arguments[0])) absolute(current.arguments[0], fileDir);
      if (ts.isImportTypeNode(current) && ts.isLiteralTypeNode(current.argument) && ts.isStringLiteralLike(current.argument.literal)) absolute(current.argument.literal, fileDir);
      if (ts.isNewExpression(current) && current.expression.getText(sf) === "URL" && current.arguments?.[0] && ts.isStringLiteralLike(current.arguments[0])) {
        const second = current.arguments[1]?.getText(sf);
        if (second === "import.meta.url") absolute(current.arguments[0], fileDir);
        if (second === "source.url") absolute(current.arguments[0], OS_DIR);
      }
      current.forEachChild(walk);
    };
    walk(node);
    replacements.sort((a, b) => b.start - a.start);
    let body = text.slice(start, node.end);
    for (const replacement of replacements) body = body.slice(0, replacement.start - start) + replacement.value + body.slice(replacement.end - start);
    for (const [from, to] of NEUTRAL) body = body.split(from).join(to);
    body = body.split(FIXTURES_TESTS).join(FIXTURES).split(ADMIN_OLD).join(ADMIN_NEW);
    body = body.replace(/\n\s*resetWorkerLaw\(\);/gu, "");
    return body.replace(/\s+/gu, " ");
  };
  const visit = (node: ts.Node, stack: readonly string[]) => {
    if (ts.isCallExpression(node)) {
      const head = node.expression.getText(sf);
      const first = node.arguments[0];
      const title = first && ts.isStringLiteralLike(first) ? first.text : undefined;
      if (title !== undefined && /^(vitest\.)?(describe)$/u.test(head)) {
        node.forEachChild((child) => visit(child, [...stack, title]));
        return;
      }
      if (title !== undefined && /^(vitest\.)?(it|test)$/u.test(head)) {
        out.push({ key: [...stack, title].join(" > "), body: normalizeCallback(node.arguments[node.arguments.length - 1]!) });
        return;
      }
    }
    node.forEachChild((child) => visit(child, stack));
  };
  visit(sf, []);
  return out;
};

const originalTests = collect(join(ORIGINAL_DIR, "🟦️.ts"), readFileSync(ORIGINAL, "utf8"));
const files = readFileSync(join(FOLDER, "met-tools/gis-new-files.txt"), "utf8").split("\n").filter(Boolean);
const newTests: Test[] = [];
for (const rel of files) newTests.push(...collect(join(REPO, rel), readFileSync(join(REPO, rel), "utf8")));

const count = (rows: Test[]) => rows.reduce((map, row) => map.set(row.key, [...(map.get(row.key) ?? []), row.body]), new Map<string, string[]>());
const a = count(originalTests);
const b = count(newTests);
let mismatched = 0;
const titles: string[] = [];
for (const [key, bodies] of a) {
  const other = b.get(key);
  if (!other || other.length !== bodies.length) { console.log("TITLE MISSING/COUNT", key, bodies.length, other?.length); mismatched += 1; continue; }
  bodies.forEach((body, index) => { if (body !== other[index]) { mismatched += 1; console.log("BODY DIFF", key); let i = 0; while (body[i] === other[index]![i]) i += 1; console.log("  orig:", body.slice(Math.max(0, i - 80), i + 120)); console.log("  new :", other[index]!.slice(Math.max(0, i - 80), i + 120)); } });
  titles.push(key);
}
for (const key of b.keys()) if (!a.has(key)) { console.log("EXTRA TITLE", key); mismatched += 1; }
console.log(JSON.stringify({ originalTests: originalTests.length, newTests: newTests.length, distinctTitles: a.size, mismatched }));
