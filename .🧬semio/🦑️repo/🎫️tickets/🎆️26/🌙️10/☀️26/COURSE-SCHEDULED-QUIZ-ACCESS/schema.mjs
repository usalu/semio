import fs from "node:fs";
const core = "🧰️framework/🛍️products/📚️course";
const edit = (path, change) => { const before = fs.readFileSync(path, "utf8"); const after = change(before); if (after === before) throw new Error(path); fs.writeFileSync(path, after); };
const schema = `${core}/🧬️schema/🔣️.json`;
edit(schema, text => {
  const value = JSON.parse(text);
  value.$defs.Unlocks = { description: "Release times in Unix milliseconds, keyed by topic, quiz or quiz/task. Null awaits scheduling; absent keys add no restriction. All ancestors apply.", type: "object", propertyNames: { pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*(?:/[a-z0-9]+(?:-[a-z0-9]+)*)?$" }, additionalProperties: { oneOf: [{ $ref: "#/$defs/Timestamp" }, { type: "null" }] } };
  for (const kind of ["Course", "CourseView"]) value.$defs[kind].properties.unlocks = { $ref: "#/$defs/Unlocks" };
  return JSON.stringify(value, null, 2) + "\n";
});
edit(`${core}/🧬️schema/🟦️.ts`, text => text.replace("export type Course = {", "/** 🗓️ Release times by topic, quiz or quiz/task; null awaits scheduling, absent keys add no restriction. */\nexport type Unlocks = Readonly<Record<string, Timestamp | null>>;\n\nexport type Course = {\n  readonly unlocks?: Unlocks;").replace("export type CourseView = {", "export type CourseView = { readonly unlocks?: Unlocks;"));
edit(`${core}/🧬️schema/📚️quiz-documents/🦀️.rs`, text => text.replace("pub struct Course {", "pub struct Course {\n    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub unlocks: Option<BTreeMap<String, Option<Timestamp>>>,"));
edit(`${core}/🧬️schema/👁️read-views/🦀️.rs`, text => text.replace("pub struct CourseView {", "pub struct CourseView {\n    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n    pub unlocks: Option<BTreeMap<String, Option<Timestamp>>>,"));
edit(`${core}/🔨️modules/👁️views/🟦️.ts`, text => text.replace("id: course.id,", "id: course.id,\n    ...(course.unlocks === undefined ? {} : { unlocks: course.unlocks }),"));
edit(`${core}/🔨️modules/👁️views/🦀️.rs`, text => text.replace("CourseView {\n", "CourseView {\n").replace("        id: course.id.clone(),", "        unlocks: course.unlocks.clone(),\n        id: course.id.clone(),"));
for (const path of ["🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs", "🧪️tests/🧷️companion-support/🧾️lifecycle/🦀️.rs"]) edit(`${core}/${path}`, text => text.replace("    Course {\n", "    Course {\n        unlocks: None,\n"));
edit(`${core}/📦️packages/🟦️typescript/🟦️.ts`, text => text + 'export * from "../../🔨️modules/🔓️access/🟦️.ts";\n');
edit(`${core}/📦️packages/🦀️rust/🦀️.rs`, text => text + '\n#[path = "../../🔨️modules/🔓️access/🦀️.rs"]\npub mod access;\n');
edit(`${core}/🦀️.rs`, text => text + '\npub use crate::access::*;\n');
edit("🎓️teaching/🏛️architecture/📚️course/🔣️.json", text => {
  const value = JSON.parse(text);
  value.unlocks = { energy: 0, parametric: null, grasshopper: null, excel: null, bim: null, optimization: null };
  return JSON.stringify(value, null, 2) + "\n";
});
