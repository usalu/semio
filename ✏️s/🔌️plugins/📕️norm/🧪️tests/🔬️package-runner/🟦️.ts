/** 🏃️ The package test plan reaches the deployed descriptor and every committed cross-language witness. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import plan from "../🧫️fixtures/🏃️package-runner/🔣️.json" with { type: "json" };
import { normPackageTestPlan } from "../🔬️typescript-package-commands/🟦️.ts";

const plugin = join(import.meta.dir, "../..");
const repoRoot = join(plugin, "../../..");

test("the runner resolves the deployed descriptor and all declared suites", () => {
  const actual = normPackageTestPlan(plugin, repoRoot);
  expect(actual.descriptor).toBe(join(repoRoot, plan.descriptor));
  expect(actual.suites).toEqual(plan.suites.map(path => join(plugin, path)));
  expect(actual.suites.every(existsSync)).toBe(true);
  const descriptor = JSON.parse(readFileSync(actual.descriptor, "utf8"));
  const editors = descriptor.manifest.apps.filter((app: { role: string }) => app.role === "editor");
  expect(editors).toHaveLength(plan.expectedEditors);
  const cohort = JSON.parse(readFileSync(join(plugin, "📇️registry/🧬️contract/🧪️tests/🧫️fixtures/🧫️retained-command-dispositions/🔣️.json"), "utf8"));
  for (const app of cohort.apps) {
    const editor = editors.find((entry: { id: string }) => entry.id === app.controller);
    for (const route of cohort.routes) expect(editor.actions.find((action: { id: string }) => action.id === route.id)?.semantics.execution.interactiveJob).toBe(route.admission);
  }
});
