import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

/** 🧵️ Bounds diagnostics under the ticket while preserving every service and test identity. */
export async function testDemonstratorE2eArtifactPaths(workspace: string): Promise<void> {
  const root = join(workspace, "♻️mit-bestand/🧺️demonstrator"), directory = join(root, "🔨️modules/🧪️e2e/📁️artifacts"), require = createRequire(import.meta.url);
  const schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8")), policy = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8")), fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8"));
  const validator = new (require("ajv").default)();
  validator.addSchema(schema);
  assert.ok(validator.validate(schema.$id, policy), JSON.stringify(validator.errors));
  assert.ok(validator.validate({ $ref: schema.$id + "#/$defs/Cases" }, fixture), JSON.stringify(validator.errors));
  const { demonstratorE2eOutputDirectory, assertDemonstratorE2eArtifactPath } = await import(pathToFileURL(join(directory, "🟦️.ts")).href);
  for (const platform of fixture.platformRoots) {
    const platformOutput = demonstratorE2eOutputDirectory(root, "t".repeat(platform.ticketRootChars), fixture.sessionId);
    const maximumWorkerIndex = policy.maxCases * fixture.projects.length * (fixture.retries + 1) - 1;
    const workerSuffix = `.playwright-artifacts-${maximumWorkerIndex}/traces/${"0".repeat(20)}-${"0".repeat(20)}-retry${fixture.retries}-recording${maximumWorkerIndex}.network`;
    assert.equal(platformOutput.length, platform.outputRootChars);
    assert.equal(workerSuffix, fixture.workerFiles[0]);
    assert.equal(workerSuffix.length + 1, policy.reservedSuffixChars);
    assertDemonstratorE2eArtifactPath(join(platformOutput, workerSuffix));
    for (const file of fixture.workerFiles) assertDemonstratorE2eArtifactPath(join(platformOutput, file));
    const defaultOutput = demonstratorE2eOutputDirectory("r".repeat(platform.ticketRootChars), undefined, fixture.sessionId);
    assert.equal(defaultOutput, join("r".repeat(platform.ticketRootChars), "dist/reports", fixture.sessionDirectory));
    assert.equal(defaultOutput.length, platform.outputRootChars);
    assertDemonstratorE2eArtifactPath(join(defaultOutput, workerSuffix));
  }
  const output = demonstratorE2eOutputDirectory(root, "t".repeat(105), fixture.sessionId);
  assert.equal(output, join("t".repeat(105), "🗑️generated", fixture.sessionDirectory));
  assert.equal(fixture.sessionDirectory, fixture.sessionId.replaceAll("-", ""));
  assert.notEqual(output, demonstratorE2eOutputDirectory(root, "t".repeat(105), fixture.sessionId.replace(/^./, "f")));
  for (const id of fixture.invalidSessionIds) assert.throws(() => demonstratorE2eOutputDirectory(root, "ticket", id), /UUID/);
  assert.throws(() => demonstratorE2eOutputDirectory(root, "x".repeat(108), fixture.sessionId), /artifact root/);
  assert.throws(() => demonstratorE2eOutputDirectory("x".repeat(108), undefined, fixture.sessionId), /artifact root/);
  assert.throws(() => demonstratorE2eOutputDirectory(root, "x".repeat(256), fixture.sessionId), /artifact root/);
  assertDemonstratorE2eArtifactPath("x".repeat(policy.maxPathChars));
  assert.throws(() => assertDemonstratorE2eArtifactPath("x".repeat(policy.maxPathChars + 1)), /artifact path/);
  for (const suffix of ["x".repeat(policy.reservedSuffixChars - 1), ...fixture.directFiles, ...fixture.workerFiles]) assertDemonstratorE2eArtifactPath(join(output, suffix));
  const typescript = require("typescript"), source = readFileSync(join(root, "🧪️tests/🎭️acceptance/🟦️.ts"), "utf8"), entry = typescript.createSourceFile("🟦️.ts", source, typescript.ScriptTarget.Latest, true, typescript.ScriptKind.TS);
  const titles: string[] = [];
  const visit = (node: any): void => {
    if (typescript.isCallExpression(node) && node.expression.getText(entry) === "test") {
      const title = node.arguments[0];
      if (typescript.isStringLiteral(title)) titles.push(title.text);
      else {
        assert.ok(typescript.isTemplateExpression(title) && title.templateSpans.length === 1);
        assert.ok(["language", "paneCase.paneId"].includes(title.templateSpans[0].expression.getText(entry)));
        const head = title.head.text, tail = title.templateSpans[0].literal.text;
        titles.push(...fixture.titles.filter((value: string) => value.length > head.length + tail.length && value.startsWith(head) && value.endsWith(tail)));
      }
    }
    typescript.forEachChild(node, visit);
  };
  visit(entry);
  assert.deepEqual(titles.sort(), [...fixture.titles].sort());
  assert.ok(titles.every(title => title.length <= policy.maxTitleChars));
  assert.ok(titles.length <= policy.maxCases);
  const config = readFileSync(join(root, "🔨️modules/🧪️e2e/🎚️config/🟦️.ts"), "utf8");
  assert.match(config, /retries: process\.env\.CI \? 2 : 0/);
  assert.match(config, /workers: 1/);
  assert.equal(config.includes("repeatEach:"), false);
  const runner = readFileSync(join(workspace, "node_modules/playwright/lib/worker/workerProcessEntry.js"), "utf8");
  const runnerIndex = readFileSync(join(workspace, "node_modules/playwright/lib/runner/index.js"), "utf8"), common = readFileSync(join(workspace, "node_modules/playwright/lib/common/index.js"), "utf8");
  assert.ok(runnerIndex.includes("return `.playwright-artifacts-${workerIndex}`;"));
  assert.ok(runner.includes("const ordinalSuffix = traceOrdinal ? `-recording${traceOrdinal}` : \"\";"));
  assert.ok(runner.includes("++traceOrdinal;"));
  assert.ok(runner.includes("return `${this._testInfo.testId}${retrySuffix}${ordinalSuffix}`;"));
  const core = readFileSync(join(workspace, "node_modules/playwright-core/lib/coreBundle.js"), "utf8");
  assert.ok(core.includes("networkFile: import_path15.default.join(tracesDir, traceName + \".network\")"));
  assert.ok(common.includes("const testId = fileId + \"-\" + calculateSha13(testIdExpression).slice(0, 20);"));
  assert.equal(/\.newContext\s*\(/.test(source), false);
  assert.match(runner, /windowsFilesystemFriendlyLength = 60;/);
  assert.match(runner, /this\.titlePath\.slice\(1\)\.join\(" "\)/);
  assert.deepEqual(fixture.projects.map(({ name, locale }: { name: string; locale: string }) => [name, locale]), [["en", "en-US"], ["de", "de-DE"]]);
  const configEntry = typescript.createSourceFile("config.ts", config, typescript.ScriptTarget.Latest, true, typescript.ScriptKind.TS);
  let projectsExpression = "";
  const visitConfig = (node: any): void => {
    if (typescript.isPropertyAssignment(node) && node.name.getText(configEntry) === "projects") projectsExpression = node.initializer.getText(configEntry);
    typescript.forEachChild(node, visitConfig);
  };
  visitConfig(configEntry);
  assert.ok(projectsExpression);
  const playwright = require("@playwright/test");
  const projects = new Function("languageCases", "devices", "browserLaunchArgs", "return " + projectsExpression)(fixture, playwright.devices, () => []);
  const officialConfig = playwright.defineConfig({ projects });
  assert.deepEqual(officialConfig.projects.map((project: any) => [project.name, project.use.locale, project.use.defaultBrowserType]), [["en", "en-US", "chromium"], ["de", "de-DE", "chromium"]]);
  assert.equal(config.includes("locale: project.locale"), true);
  assert.equal(config.includes("name: project.name"), true);
  assert.equal(source.includes("test.use({ locale: LANGUAGE_CASES[0].browserLocale })"), false);
  for (const project of fixture.projects) {
  const maximumTestDirectory = "x".repeat(60) + "-" + project.name + "-retry" + fixture.retries;
  for (const file of fixture.directFiles) assertDemonstratorE2eArtifactPath(join(output, maximumTestDirectory, file));
  }
}
