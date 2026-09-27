#!/usr/bin/env bun
/** 🎯️ R10 item 3: every goal-plan check and provider names a declared nx target (project:target), or is listed as pending
 * in the window-3 spec. Usage: bun plan-targets-check.ts */
const ROOT = "/Users/ueli/Documents/semio";
const { declaredProjectTargets } = await import(`${ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`);
const plan = await Bun.file(`${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/🎚️config/🔣️.json`).json();
const spec = await Bun.file(`${import.meta.dir}/window3-spec.json`).json();
const declared = new Set((declaredProjectTargets(ROOT) as { project: string; targets: string[] }[]).flatMap((project) => project.targets.map((target) => `${project.project}:${target}`)));
const pending = new Set((spec.targets as { project: string; name: string }[]).map((target) => `${target.project}:${target.name}`));
const rows = [...plan.steps.flatMap((step: { checks: { id: string; project: string; target: string }[] }) => step.checks.map((check) => [check.id, `${check.project}:${check.target}`])), ...Object.entries(plan.providers ?? {}).map(([id, provider]) => [`provider:${id}`, `${(provider as { project: string }).project}:${(provider as { target: string }).target}`])];
const missing = rows.filter(([, pair]) => !declared.has(pair!) && !pending.has(pair!));
const windowThree = rows.filter(([, pair]) => !declared.has(pair!) && pending.has(pair!));
console.log(JSON.stringify({ checks: rows.length, declared: rows.length - missing.length - windowThree.length, windowThree: windowThree.map((row) => row.join(" → ")), missing: missing.map((row) => row.join(" → ")) }, null, 1));
