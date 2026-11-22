import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

type Decision = number | "ambiguous" | "none";
type Row = { name: string; candidates: string[]; mostGeneral: Decision };

const root = process.cwd();
const generated = join(import.meta.dir, "🗑️generated", "dialect-selection");
mkdirSync(generated, { recursive: true });
const selection = resolve(root, "🧰️framework/🔨️modules/🗿️artifact-reference/🌳️selection/🦀️.rs");
const fixture = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🧫️fixtures/🌳️most-general-dialect/🔣️.json"), "utf8")) as { cases: Row[] };
const rows = fixture.cases.map(row => ({
  ...row,
  dialects: row.candidates.map((coordinate, i) => {
    const match = /^([^@]+)@([^/]+)\/(.+)$/.exec(coordinate);
    assert.ok(match, coordinate);
    return { index: i, kind: match[1], standard: match[2], subset: match[3] };
  }),
}));

const database = new Database(":memory:");
database.exec("CREATE TABLE dialect (ordinal INTEGER, kind TEXT, standard TEXT, subset TEXT)");
const insert = database.query("INSERT INTO dialect VALUES (?, ?, ?, ?)");
const uncovered = database.query<{ ordinal: number }, []>(`
  SELECT d.ordinal FROM dialect d
  WHERE NOT EXISTS (
    SELECT 1 FROM dialect w
    WHERE w.kind = d.kind AND w.standard = d.standard AND w.subset = '*' AND d.subset <> '*'
  )
`);
for (const row of rows) {
  for (const order of [row.dialects, [...row.dialects].reverse()]) {
    database.exec("DELETE FROM dialect");
    for (const d of order) insert.run(d.index, d.kind, d.standard, d.subset);
    const decision = uncovered.all();
    assert.equal(decision.length === 1 ? decision[0].ordinal : decision.length > 1 ? "ambiguous" : "none", row.mostGeneral, row.name);
  }
}
database.close();

const cases = rows.map(row => {
  const dialects = row.dialects.map(d => `(${d.index}, ${JSON.stringify(d.kind)}, ${JSON.stringify(d.standard)}, ${JSON.stringify(d.subset)})`).join(", ");
  const answer = typeof row.mostGeneral === "number" ? `MostGeneralDialect::One(${row.mostGeneral})`
    : row.mostGeneral === "ambiguous" ? "MostGeneralDialect::Ambiguous" : "MostGeneralDialect::Empty";
  return `(${JSON.stringify(row.name)}, &[${dialects}], ${answer})`;
}).join(",\n");
const source = join(generated, "verify.rs");
const executable = join(generated, process.platform === "win32" ? "verify.exe" : "verify");
writeFileSync(source, `
#[path = ${JSON.stringify(selection)}] mod selection;
pub struct SubsetId(pub &'static str);
impl SubsetId { pub const ANY: Self = Self("*"); }
use selection::{MostGeneralDialect, most_general_dialect};
type Dialect = (usize, &'static str, &'static str, &'static str);
fn main() {
    let rows: &[(&str, &[Dialect], MostGeneralDialect<usize>)] = &[\n${cases}\n];
    for (name, candidates, expected) in rows {
        let mut input = candidates.to_vec();
        for _ in [false, true] {
            let found = most_general_dialect(input.clone(), |(_, kind, standard, subset)| (kind, standard, subset));
            let ordinal = found.one().map(|(index, _, _, _)| index);
            let actual = match found {
                MostGeneralDialect::One(_) => MostGeneralDialect::One(ordinal.unwrap()),
                MostGeneralDialect::Ambiguous => MostGeneralDialect::Ambiguous,
                MostGeneralDialect::Empty => MostGeneralDialect::Empty,
            };
            assert_eq!(&actual, expected, "{name}");
            input.reverse();
        }
    }
}
`);
const compile = [
  "repo", "tools", "exec", "--light", "rustup", "--",
  "run", "nightly-2026-07-20", "rustc", "--edition", "2024",
  "--crate-name", "verify_dialect_selection", source, "-o", executable,
];
assert.equal(Bun.spawnSync(compile, { stdout: "inherit", stderr: "inherit" }).exitCode, 0);
assert.equal(Bun.spawnSync([executable], { stdout: "inherit", stderr: "inherit" }).exitCode, 0);
console.log(`Actual Rust selector and independent SQLite selected identical outcomes for ${rows.length} fixture cases in both declaration orders.`);
