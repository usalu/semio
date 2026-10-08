import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "🧫️fixtures/🔣️.json"), "utf8"));

test("native Architect sparse collection roles preserve schema and independent SQL order", () => {
  const schema = JSON.parse(readFileSync(resolve(root, "🔣️.json"), "utf8"));
  const ajv = new Ajv({strict:false});
  ajv.addSchema(JSON.parse(readFileSync(resolve("🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json"), "utf8")));
  ajv.addSchema(JSON.parse(readFileSync(resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json"), "utf8")));
  ajv.addSchema(JSON.parse(readFileSync(resolve(root, "../🔣️.json"), "utf8")));
  const deltaSchema = {...schema, $ref:"#/$defs/ProgramStakeholdersDelta"};
  delete deltaSchema.required;
  delete deltaSchema.properties;
  delete deltaSchema.additionalProperties;
  expect(ajv.compile(deltaSchema)(fixture.delta)).toBe(true);
  const db = new Database(":memory:");
  db.run("CREATE TABLE owners(id TEXT PRIMARY KEY, ordinal INTEGER NOT NULL)");
  fixture.ids.forEach((id:string, ordinal:number) => db.run("INSERT INTO owners VALUES (?,?)", id, ordinal));
  fixture.delta.removed.forEach((id:string) => db.run("DELETE FROM owners WHERE id=?", id));
  fixture.delta.reordered.forEach((id:string, ordinal:number) => db.run("UPDATE owners SET ordinal=? WHERE id=?", ordinal, id));
  expect(db.query("SELECT id FROM owners ORDER BY ordinal").all().map((row:any) => row.id)).toEqual(fixture.expected);
  db.close();
  const source = readFileSync(resolve(root, "🦀️.rs"), "utf8");
  for(const name of ["ProgramDiff", "ProgramStakeholdersDelta", "ProgramStakeholdersPatchEntry"]) {
    const prefix = source.slice(0, source.indexOf(`pub struct ${name} {`));
    const derive = prefix.slice(prefix.lastIndexOf("#[derive("));
    expect(derive).toContain("semio_framework_dsl_record_derive::DslRecord");
  }
  console.log("[DEBUG] Architect sparse native roles preserve Ajv delta and SQLite removal/order", fixture.expected);
});
