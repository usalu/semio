import { Database } from "bun:sqlite";

const [sourcePath, fixturePath] = process.argv.slice(2);
const source = await Bun.file(sourcePath!).text();
const { issuedArchiveClose: fixture } = await Bun.file(fixturePath!).json();
const decisions = [...source.matchAll(/if !semio_framework_value::retained_clone::retained_clone_grant_admits\(cx\.(original_\w+)\(\),demand\)/g)];
if (decisions.length !== 2) throw new Error("Both retained close admission checks must be present");

const db = new Database(":memory:");
const decide = db.query<{ disposition: string }, [number, number, number]>(
  "SELECT CASE WHEN ?1 >= ?3 THEN 'admitted' WHEN ?2 >= ?3 THEN 'pending' ELSE 'refused' END AS disposition",
);
const required = fixture.copyBytesRequired;
const narrow = fixture.copyBytesAvailable;

for (const [, ceiling] of decisions) {
  const cases = [
    { issued: narrow, source: required, expected: fixture.expectedWhenSourceCanFund },
    { issued: narrow, source: narrow, expected: fixture.expectedWhenSourceCannotFund },
    { issued: required, source: required, expected: "admitted" },
  ];
  for (const row of cases) {
    const permanent = ceiling === "original_source_total_grant" ? row.source : row.issued;
    const { disposition } = decide.get(row.issued, permanent, required)!;
    if (disposition !== row.expected) throw new Error(`Measured close demand expected ${row.expected}, got ${disposition}`);
  }
}
db.close();
console.log("Passed six close admission decisions");
