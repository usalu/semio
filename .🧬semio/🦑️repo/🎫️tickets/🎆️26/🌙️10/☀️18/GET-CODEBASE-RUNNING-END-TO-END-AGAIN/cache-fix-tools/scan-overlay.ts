/** 🔎 Runs the cache-correctness law over the overlay workspace with the staged target-policy overlay. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { cacheCorrectnessRepositoryFindingsV1, repositoryCacheRecordsV1 } from "./law/🟦️.ts";
import { load } from "./overlay.ts";
const root = join(import.meta.dir, "../🗑️generated/cachefix-ovroot");
const staged = await load("staged");
const tracked = readFileSync(join(import.meta.dir, "../🗑️generated/cachefix-x/pj-list.txt"), "utf8").split("\n").filter(Boolean);
const gate = process.argv.includes("--gate");
const records = repositoryCacheRecordsV1(root, { targetPolicy: (name, target) => staged.module.targetPolicy(name, target, staged.POLICY) }, tracked);
const findings = cacheCorrectnessRepositoryFindingsV1(records);
const by: Record<string, number> = {}; for (const f of findings) by[f.rule] = (by[f.rule] ?? 0) + 1;
console.log(records.length, "records", findings.length, "findings", by);
if (process.argv.includes("--list")) for (const f of findings) console.log(f.rule, f.id, f.detail.slice(0, 200));
