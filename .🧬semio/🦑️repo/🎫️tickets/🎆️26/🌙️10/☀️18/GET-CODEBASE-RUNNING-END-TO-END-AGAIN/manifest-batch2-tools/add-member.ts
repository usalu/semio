/** ➕ Adds missing workspace member paths to a root manifest with the planner's sorted insertion. Usage: bun add-member.ts <root package.json> <member> [--write] */
import { readFileSync, writeFileSync } from "node:fs";
import { planManifest } from "../cycle-break-tools/apply-plan.ts";
const [file, key] = process.argv.slice(2) as [string, string];
const text = readFileSync(file, "utf8");
const problems: string[] = [];
const next = planManifest(text, file, [{ op: "add", packageJson: file, package: "workspace", field: "workspaces", key, value: "", wave: "manifest-batch2", why: "workspace member" } as never], problems);
if (problems.length) { console.error(problems.join("\n")); process.exit(3); }
console.log(next === text ? "unchanged" : "changed");
if (process.argv.includes("--write")) writeFileSync(file, next);
