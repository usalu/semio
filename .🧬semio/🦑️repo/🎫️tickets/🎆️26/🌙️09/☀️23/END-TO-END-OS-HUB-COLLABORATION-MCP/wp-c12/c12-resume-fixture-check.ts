/** 🔎️ C12 14c: parses every case of `⏯️execution-target-resume-v1.json` with the live lease parser and prints parse errors + relations. */
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio/";
const { parseDocumentExecutionTargetLeaseFieldsV1, sameExecutionTargetV1 } = await import(root + "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts");
const fixture = JSON.parse(readFileSync(root + "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/⏯️execution-target-resume-v1.json", "utf8"));
const manifest = JSON.parse(readFileSync(root + fixture.current.fixture, "utf8")).manifest;
const merge = (target: any, patch: any): any => {
  if (patch === null || typeof patch !== "object" || Array.isArray(patch)) return patch;
  const merged: any = target !== null && typeof target === "object" && !Array.isArray(target) ? { ...target } : {};
  for (const [key, value] of Object.entries(patch)) value === null ? delete merged[key] : (merged[key] = merge(merged[key], value));
  return merged;
};
const current = parseDocumentExecutionTargetLeaseFieldsV1(manifest);
for (const row of fixture.cases) {
  try {
    const next = parseDocumentExecutionTargetLeaseFieldsV1(merge(structuredClone(manifest), row.patch));
    const got = sameExecutionTargetV1(current, next);
    console.log(`${got === row.resumes ? "ok " : "BAD"} ${row.case}: resumes=${got} expected=${row.resumes}`);
  } catch (error) {
    console.log(`THROW ${row.case}: ${(error as Error).message}`);
  }
}
