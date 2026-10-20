// 📋 Lists the test files that reference a lifted script in a non-import position (spawn, path data, source read) and are not yet in a batch. usage: bun other-tests.ts
import {existsSync, readFileSync} from "node:fs";
import {refsOf} from "./refs-lib.ts";
import {C, LIB, REPO} from "./vfs.ts";
import {liftList} from "./plans/lifts.ts";
const seen = new Set<string>();
for (const name of ["bazel", "caching", "rest", "tc1", "tc2", "tc3"]) { try { for (const l of readFileSync(`${C}/🗑️generated/tax-stage/tests-${name}.txt`, "utf8").split("\n")) if (l) seen.add(l); } catch { /* absent */ } }
const out = new Set<string>();
for (const target of liftList()) for (const r of refsOf(target)) if (/\/🟦️\.ts$/u.test(r.file) && r.file.includes("🧪️tests") && !seen.has(r.file) && existsSync(`${REPO}/${r.file}`)) out.add(r.file);
console.log([...out].join("\n"));
