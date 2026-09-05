// 🔬️ Ad-hoc ajv validation of every recorded lowpoly mutation-family fixture against the
// lowpoly json-schema representations (mutations dispatch envelope, snapshot, diff). Run with:
//   bun 🔬️validate-lowpoly-fixtures.ts
// Ticket: 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS — re-establishing (not inheriting)
// the "ajv validates 17/17 recorded fixtures" claim from a prior session.

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, dirname } from "node:path";
import Ajv from "ajv";

const REPO = "/Users/ueli/Documents/semio";
const SCHEMA_ROOT = join(
  REPO,
  "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
);
const MUT_ROOT = join(SCHEMA_ROOT, "🧬️mutations");

function readJson(path: string) {
  return JSON.parse(readFileSync(path, "utf8"));
}

function listDirs(path: string): string[] {
  return readdirSync(path).filter((name) => statSync(join(path, name)).isDirectory());
}

// 🌱️ Discover every mutation directory (excludes the shared 💾️binary / 📝️text triad-language dirs).
const mutationDirs = listDirs(MUT_ROOT).filter((name) => !name.includes("binary") && !name.endsWith("text") && name !== "💾️binary" && name !== "📝️text");
// the above emoji-substring filter is unreliable across renames; filter by presence of 🧬️.schema.json instead
const realMutationDirs = listDirs(MUT_ROOT).filter((name) => {
  try {
    statSync(join(MUT_ROOT, name, "🧬️.schema.json"));
    return true;
  } catch {
    return false;
  }
});

console.log(`Discovered ${realMutationDirs.length} mutation directories with a 🧬️.schema.json payload leaf.`);

// 🧬️ Build the mutation-dispatch ajv instance: the root mutations/🔣️.json references each sibling
// mutation's 🧬️.schema.json by relative path — register each under that exact ref string.
// Ajv percent-encodes non-ASCII $ref path segments through its URI resolver, which cannot then be
// matched back to a plain-string addSchema() key — sidestep URI resolution entirely by manually
// dereferencing each sibling-file $ref into an inline copy before handing the schema to ajv.
const mutationSchema = readJson(join(MUT_ROOT, "🔣️.json"));
delete mutationSchema.$id;
for (const branch of mutationSchema.oneOf ?? []) {
  for (const key of Object.keys(branch.properties ?? {})) {
    const ref: string | undefined = branch.properties[key].$ref;
    if (ref) {
      const payload = readJson(join(MUT_ROOT, ref));
      delete payload.$schema;
      branch.properties[key] = payload;
    }
  }
}
const ajvMutations = new Ajv({ strict: false });
const validateMutation = ajvMutations.compile(mutationSchema);

const snapshotSchema = readJson(join(SCHEMA_ROOT, "📸️snapshot", "🔣️.json"));
const ajvSnapshot = new Ajv({ strict: false });
const validateSnapshot = ajvSnapshot.compile(snapshotSchema);

const diffSchema = readJson(join(SCHEMA_ROOT, "🔺️diff", "🔣️.json"));
const ajvDiff = new Ajv({ strict: false });
const validateDiff = ajvDiff.compile(diffSchema);

type Result = { fixture: string; schema: string; ok: boolean; errors?: string };

const results: Result[] = [];

function validate(label: string, schemaName: string, validator: any, data: unknown) {
  const ok = validator(data) as boolean;
  results.push({ fixture: label, schema: schemaName, ok, errors: ok ? undefined : JSON.stringify(validator.errors) });
}

for (const dir of realMutationDirs) {
  const testsRoot = join(MUT_ROOT, dir, "🧪️tests");
  let cases: string[] = [];
  try {
    cases = listDirs(testsRoot);
  } catch {
    console.log(`  ! ${dir} has no 🧪️tests directory`);
    continue;
  }
  for (const c of cases) {
    const base = join(testsRoot, c);
    const mutationFixture = join(base, "🦠️mutation", "🔣️.json");
    const snapshotBefore = join(base, "📸️snapshot", "⬅️before", "🔣️.json");
    const snapshotAfter = join(base, "📸️snapshot", "➡️after", "🔣️.json");
    const diffFixture = join(base, "🔺️diff", "🔣️.json");
    const outcomeFixture = join(base, "🎯️outcome", "🔣️.json");

    try {
      validate(`${dir}/${c}/🦠️mutation`, "mutations/🔣️.json", validateMutation, readJson(mutationFixture));
    } catch (e) {
      results.push({ fixture: `${dir}/${c}/🦠️mutation`, schema: "mutations/🔣️.json", ok: false, errors: String(e) });
    }
    try {
      validate(`${dir}/${c}/📸️snapshot/⬅️before`, "snapshot/🔣️.json", validateSnapshot, readJson(snapshotBefore));
    } catch (e) {
      results.push({ fixture: `${dir}/${c}/📸️snapshot/⬅️before`, schema: "snapshot/🔣️.json", ok: false, errors: String(e) });
    }
    try {
      validate(`${dir}/${c}/📸️snapshot/➡️after`, "snapshot/🔣️.json", validateSnapshot, readJson(snapshotAfter));
    } catch (e) {
      results.push({ fixture: `${dir}/${c}/📸️snapshot/➡️after`, schema: "snapshot/🔣️.json", ok: false, errors: String(e) });
    }
    try {
      validate(`${dir}/${c}/🔺️diff`, "diff/🔣️.json", validateDiff, readJson(diffFixture));
    } catch (e) {
      results.push({ fixture: `${dir}/${c}/🔺️diff`, schema: "diff/🔣️.json", ok: false, errors: String(e) });
    }
    try {
      readJson(outcomeFixture);
      results.push({ fixture: `${dir}/${c}/🎯️outcome`, schema: "(no lowpoly json-schema — protocol-level MutationOutcome shape)", ok: true });
    } catch (e) {
      results.push({ fixture: `${dir}/${c}/🎯️outcome`, schema: "(none)", ok: false, errors: String(e) });
    }
  }
}

const passed = results.filter((r) => r.ok).length;
console.log(`\n${passed}/${results.length} fixture validations passed.\n`);
for (const r of results) {
  console.log(`${r.ok ? "PASS" : "FAIL"}  ${r.fixture}  [${r.schema}]${r.errors ? `\n      ${r.errors}` : ""}`);
}

const mutationOnly = results.filter((r) => r.schema === "mutations/🔣️.json");
console.log(`\nMutation-envelope-only tally: ${mutationOnly.filter((r) => r.ok).length}/${mutationOnly.length}`);

if (passed !== results.length) process.exit(1);
