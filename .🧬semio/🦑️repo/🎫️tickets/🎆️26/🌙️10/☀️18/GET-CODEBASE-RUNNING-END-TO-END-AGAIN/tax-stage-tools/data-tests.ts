// 📋 Lists the test files whose fixture JSON references a lifted script (fixture directory name = test directory name). usage: bun data-tests.ts
import {existsSync} from "node:fs";
import {dirname, join} from "node:path";
import {refsOf} from "./refs-lib.ts";
import {liftList} from "./plans/lifts.ts";
import {REPO} from "./vfs.ts";
const out = new Set<string>();
for (const t of liftList()) for (const r of refsOf(t)) {
  if (!r.file.endsWith("🔣️.json") || !r.file.includes("🧫️fixtures/")) continue;
  const i = r.file.indexOf("/🧫️fixtures/"), base = r.file.slice(0, i), name = dirname(r.file.slice(i + "/🧫️fixtures/".length));
  const test = `${base}/${name}/🟦️.ts`;
  if (existsSync(join(REPO, test))) out.add(test);
}
console.log([...out].join("\n"));
