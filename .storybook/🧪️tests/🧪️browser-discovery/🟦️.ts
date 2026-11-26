/** 🧭️ The real Playwright collector loads every registered suite through one testing-framework identity. */
import { expect, test } from "vitest";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import fixture from "../🧫️fixtures/🌐️browser-discovery/🔣️.json";

test("bounded browser discovery preserves all authored suites", () => {
  const root = fileURLToPath(new URL("../../../", import.meta.url));
  type Suite = { file?: string; specs?: { file: string }[]; suites?: Suite[] };
  type Report = { config: { rootDir: string }; suites: Suite[]; errors: unknown[] };
  const directory = mkdtempSync(join(tmpdir(), "storybook-discovery-"));
  let report: Report;
  try {
    const file = join(directory, "suites.json");
    execFileSync("repo", ["tools", "exec", "--light", "bun", "--", "x", "--bun", "playwright", "test", "--config", ".storybook/🧪️tests/🧪️browser-runner/🟦️.ts", "--list", "--reporter=json"], {
      cwd: root, stdio: "inherit", timeout: 30000, env: { ...process.env, PLAYWRIGHT_JSON_OUTPUT_FILE: file },
    });
    report = JSON.parse(readFileSync(file, "utf8")) as Report;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
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
