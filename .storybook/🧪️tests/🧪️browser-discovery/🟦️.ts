/** 🧭️ The real Playwright collector loads every registered suite through one testing-framework identity. */
import { expect, test } from "vitest";
import { execFileSync } from "node:child_process";
import { relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import fixture from "../🧫️fixtures/🌐️browser-discovery/🔣️.json";

test("bounded browser discovery preserves all authored suites", () => {
  const root = fileURLToPath(new URL("../../../", import.meta.url));
  const output = execFileSync("repo", ["tools", "exec", "--light", "bun", "--", "x", "--bun", "playwright", "test", "--config", ".storybook/🧪️tests/🧪️browser-runner/🟦️.ts", "--list", "--reporter=json"], { cwd: root, encoding: "utf8", timeout: 30000 });
  type Suite = { file?: string; specs?: { file: string }[]; suites?: Suite[] };
  const report = JSON.parse(output) as { config: { rootDir: string }; suites: Suite[]; errors: unknown[] };
  const files = new Set<string>();
  let tests = 0;
  const visit = (suite: Suite): void => {
    if (suite.specs?.length) {
      for (const spec of suite.specs) files.add(relative(root, resolve(report.config.rootDir, spec.file)).replaceAll("\\", "/"));
      tests += suite.specs.length;
    }
    suite.suites?.forEach(visit);
  };
  report.suites.forEach(visit);
  expect(report.errors).toEqual([]);
  expect([...files].sort()).toEqual([...fixture.suites].sort());
  expect(tests).toBeGreaterThanOrEqual(fixture.minimumTests);
  expect(relative(root, report.config.rootDir).replaceAll("\\", "/")).toBe(".storybook/🧪️tests/🧪️browser-runner/📚️suites");
});
