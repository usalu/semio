import { load, set, edit } from "./lib.mjs";

const B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
export const P = {
  precisionData: B + "📥️inputs/🎯️precision/🔣️.json",
  precisionSchema: B + "📥️inputs/🎯️precision/🧬️schema/🔣️.json",
  precisionFixture: B + "📥️inputs/🎯️precision/🧪️tests/🔣️.json",
  precisionFixtureSchema: B + "📥️inputs/🎯️precision/🧪️tests/🧬️schema/🔣️.json",
  precisionTests: B + "📥️inputs/🎯️precision/🧪️tests/🟦️.ts",
  hermeticData: B + "🏃️execution/🔒️hermetic/🔣️.json",
  hermeticSchema: B + "🏃️execution/🧬️schema/🔒️hermetic-classes.json",
  hermeticTests: B + "🏃️execution/🔒️hermetic/🧪️tests/🟦️.ts",
};

function pointer(path, target) {
  const text = load(path);
  const m = /^\{\n( +)/u.exec(text);
  if (!m) throw Error("unexpected JSON head in " + path);
  set(path, text.replace(/^\{\n/u, `{\n${m[1]}"$schema": ${JSON.stringify(target)},\n`));
}

export function build() {
  pointer(P.precisionData, "./🧬️schema/🔣️.json");
  pointer(P.precisionFixture, "./🧬️schema/🔣️.json");
  pointer(P.hermeticData, "../🧬️schema/🔒️hermetic-classes.json");
  edit(P.precisionSchema, `  "properties": {
    "version": {
      "const": 1
    },`, `  "properties": {
    "$schema": {
      "type": "string"
    },
    "version": {
      "const": 1
    },`);
  edit(P.precisionFixtureSchema, `  "properties": {
    "globs":`, `  "properties": {
    "$schema": { "type": "string" },
    "globs":`);
  edit(P.hermeticSchema, `  "properties": {
    "version": {
      "const": 1
    },`, `  "properties": {
    "$schema": {
      "type": "string"
    },
    "version": {
      "const": 1
    },`);
  set(P.precisionTests, load(P.precisionTests).replace(/\n*$/, "\n") + `
test("the rules and the fixture point at their sibling schemas through $schema and validate against the pointed schema", () => {
  const pointed = (base: string, data: {readonly $schema: string}) => JSON.parse(readFileSync(join(base, data.$schema), "utf8"));
  expect(pointed(join(here, ".."), rules)).toEqual(schema);
  expect(pointed(here, fixture)).toEqual(fixtureSchema);
  const validate = new Ajv({strict: false});
  expect(validate.validate(pointed(join(here, ".."), rules), rules), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(pointed(here, fixture), fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(pointed(join(here, ".."), rules), {...rules, $schema: 1})).toBe(false);
});
`);
  edit(P.hermeticTests, `import {mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync} from "node:fs";`, `import {mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";`);
  set(P.hermeticTests, load(P.hermeticTests).replace(/\n*$/, "\n") + `
test("the class table points at its sibling schema through $schema and validates against the pointed schema",()=>{
 const pointed=JSON.parse(readFileSync(join(import.meta.dir,"..",HERMETIC_TABLE_V1.$schema),"utf8"));
 expect(pointed).toEqual(schema);
 const validate=new Ajv({strict:true}).compile(pointed);
 expect(validate(HERMETIC_TABLE_V1),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...HERMETIC_TABLE_V1,$schema:1})).toBe(false);
});
`);
}
