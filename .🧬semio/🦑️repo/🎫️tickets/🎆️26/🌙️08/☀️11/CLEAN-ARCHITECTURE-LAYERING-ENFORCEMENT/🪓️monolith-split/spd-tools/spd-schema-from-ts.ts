/** 🧮️ For every disagreeing parser of Ajv differentials (verify.json files): what the schema must state so that schema, hand-declared TypeScript and parser agree. Prints a proposal as JSON lines `{doc, name, addRequired, makeNullable}`: `bun spd-schema-from-ts.ts <out.json> <verify.json>...`. Reads only; the edit is `spd-schema-apply.py`. */
import { readFileSync, writeFileSync } from "node:fs";
const root = process.cwd();
const base = `${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection`;
const { SourceStore } = await import(`${base}/🗂️documents/🟦️.ts`);
const { declaredOptionalFields, declaredTypes, unionMembers } = await import(`${base}/📖️declared-types/🟦️.ts`);
const { nullableOf } = await import(`${base}/📐️type-render/🟦️.ts`);
const sandbox = "/private/tmp/claude-501/-Users-ueli-Documents-semio/fa8b7223-6eff-4238-a950-5afbbb4d5262/scratchpad/spd-sandbox/";
const out = process.argv[2]!;
type Row = { name: string; leaf: string; stricter: string[]; lenientOther: string[]; lenientExtra: string[] };
const seen = new Set<string>();
const rows: Row[] = [];
for (const file of process.argv.slice(3)) for (const row of JSON.parse(readFileSync(file, "utf8")) as Row[]) {
  const key = `${row.leaf}\u0000${row.name}`;
  if (seen.has(key) || (row.stricter.length === 0 && row.lenientOther.length === 0)) continue;
  seen.add(key);
  rows.push(row);
}
const sources = new SourceStore();
const proposals: unknown[] = [];
for (const row of rows) {
  const leaf = `${sandbox}${row.leaf}`;
  const docPath = `${root}/${row.leaf.replace(/🟦️\.ts$/u, "🔣️.json")}`;
  const document = JSON.parse(readFileSync(docPath, "utf8")) as Record<string, any>;
  const defs = { ...(document.definitions ?? {}), ...(document.$defs ?? {}) } as Record<string, any>;
  const node = row.name in defs ? defs[row.name] : document.title === row.name ? document : undefined;
  if (node === undefined || typeof node.properties !== "object") { proposals.push({ doc: row.leaf, name: row.name, skip: "no object schema" }); continue; }
  const source = sources.declarations(`${root}/${row.leaf}`);
  const optional = declaredOptionalFields(source, row.name);
  const types = declaredTypes(source, row.name);
  if (optional === undefined) { proposals.push({ doc: row.leaf, name: row.name, skip: "no declared interface" }); continue; }
  const required = new Set<string>(node.required ?? []);
  const addRequired: string[] = [];
  const makeNullable: string[] = [];
  const notDeclared: string[] = [];
  for (const [field, shape] of Object.entries(node.properties as Record<string, any>)) {
    if (!optional.has(field)) { notDeclared.push(field); continue; }
    if (optional.get(field) === false && !required.has(field)) addRequired.push(field);
    const type = types.get(field);
    if (type !== undefined && unionMembers(type).includes("null") && nullableOf(shape) === undefined && shape.type !== "null") makeNullable.push(field);
  }
  proposals.push({ doc: row.leaf, name: row.name, addRequired, makeNullable, notDeclared, stricter: row.stricter.length, lenient: row.lenientOther.length });
}
writeFileSync(out, JSON.stringify(proposals, null, 1));
for (const proposal of proposals as any[]) console.log(proposal.name, proposal.doc.split("🗿️artifacts/")[1]?.split("/")[0], proposal.skip ?? `+required ${proposal.addRequired.length} nullable ${proposal.makeNullable.length} undeclared ${proposal.notDeclared.length} (stricter ${proposal.stricter}, lenient ${proposal.lenient})`);
