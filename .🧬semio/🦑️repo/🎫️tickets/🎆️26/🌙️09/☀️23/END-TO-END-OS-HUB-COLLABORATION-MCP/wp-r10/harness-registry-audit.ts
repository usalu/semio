#!/usr/bin/env bun
/**
 * 🧾️ R10 session 15: for every goal-plan check and every relayed permanent harness (rule 17), is it registered end to end —
 * the nx target declared in its `📋️project.json`, a `.vscode/launch.json` row that runs `nx run <project>:<target>`, and a
 * goal-plan check that names it? Read-only. Extra harnesses (relayed, not in the plan yet) come from `EXTRA` below.
 * Usage: bun harness-registry-audit.ts [--json <out>]
 */
import { readFileSync, writeFileSync } from "node:fs";

const ROOT = "/Users/ueli/Documents/semio";
const { declaredProjectTargets } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const plan = JSON.parse(readFileSync(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🎚️config/🔣️.json`, "utf8"));
const launch = JSON.parse(readFileSync(`${ROOT}/.vscode/launch.json`, "utf8").replace(/^\s*\/\/.*$/gmu, "")) as { configurations: { name: string; command?: string }[] };
const EXTRA: { owner: string; project: string; target: string; note: string }[] = [
  { owner: "AV2", project: "workspace", target: "video-render-export", note: "wp-av2 §R10: bun ./📜️script.ts verify video-render-export (group dev)" },
  { owner: "AV2", project: "workspace", target: "video-render-export-native", note: "wp-av2 §R10: … verify video-render-export native (group gate)" },
  { owner: "F3", project: "@semio-tech/framework-os-dev", target: "interaction-latency", note: "F3 verify latency (React commits/input + puzzle3d hover)" },
  { owner: "G12", project: "@semio-tech/framework-os-dev", target: "channel-version-check", note: "G11/G12 relay" },
  { owner: "G12", project: "@semio-tech/framework-os-dev", target: "channel-version-generate", note: "G11/G12 relay" },
  { owner: "SH2", project: "@semio-tech/framework-os-dev", target: "verify-home", note: "SH2 relay" },
  { owner: "S18", project: "@semio-tech/framework-os-dev", target: "serve-hold", note: "goal-gate serve provider" },
  { owner: "S18", project: "@semio-tech/framework-os-dev", target: "local-hub", note: "goal-gate hub provider" },
];
const declared = new Set((declaredProjectTargets(ROOT) as { project: string; targets: string[] }[]).flatMap((project) => project.targets.map((target) => `${project.project}:${target}`)));
const rowsFor = (pair: string) => launch.configurations.filter((row) => row.command !== undefined && new RegExp(`nx run ${pair.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}(\\s|$)`, "u").test(row.command)).map((row) => row.name);
const planChecks = (plan.steps as { id: string; checks: { id: string; project: string; target: string }[] }[]).flatMap((step) => step.checks.map((check) => ({ step: step.id, id: check.id, pair: `${check.project}:${check.target}` })));
const planPairs = new Set(planChecks.map((check) => check.pair));
const report = {
  plan: planChecks.map((check) => ({ ...check, declared: declared.has(check.pair), launchRows: rowsFor(check.pair).length })),
  extra: EXTRA.map((extra) => { const pair = `${extra.project}:${extra.target}`; return { ...extra, declared: declared.has(pair), launchRows: rowsFor(pair), inPlan: planPairs.has(pair) }; }),
};
const gaps = [...report.plan.filter((row) => !row.declared || row.launchRows === 0).map((row) => `plan ${row.step}/${row.id} → ${row.pair}: declared=${row.declared} rows=${row.launchRows}`), ...report.extra.filter((row) => !row.declared || row.launchRows.length === 0 || !row.inPlan).map((row) => `extra ${row.owner} ${row.project}:${row.target}: declared=${row.declared} rows=${row.launchRows.length} inPlan=${row.inPlan}`)];
console.log(JSON.stringify({ planChecks: report.plan.length, planDeclared: report.plan.filter((row) => row.declared).length, planWithRow: report.plan.filter((row) => row.launchRows > 0).length, gaps }, null, 1));
const jsonAt = process.argv.indexOf("--json");
if (jsonAt >= 0) writeFileSync(process.argv[jsonAt + 1]!, JSON.stringify(report, null, 1));
