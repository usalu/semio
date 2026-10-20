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
  const output = demonstratorE2eOutputDirectory(root, "t".repeat(105), fixture.sessionId);
  assert.equal(output, join("t".repeat(105), "🗑️generated", policy.namespace, fixture.sessionDirectory));
  assert.equal(fixture.sessionDirectory, fixture.sessionId.replaceAll("-", ""));
  assert.notEqual(output, demonstratorE2eOutputDirectory(root, "t".repeat(105), fixture.sessionId.replace(/^./, "f")));
  for (const id of fixture.invalidSessionIds) assert.throws(() => demonstratorE2eOutputDirectory(root, "ticket", id), /UUID/);
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
        assert.equal(title.getText(entry), "`${paneCase.paneId} boot`");
        titles.push(...fixture.titles.filter((value: string) => value.endsWith(" boot")));
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
  assert.match(runner, /windowsFilesystemFriendlyLength = 60;/);
  assert.match(runner, /this\.titlePath\.slice\(1\)\.join\(" "\)/);
  const maximumTestDirectory = "x".repeat(60) + "-" + fixture.project + "-retry" + fixture.retries;
  for (const file of fixture.directFiles) assertDemonstratorE2eArtifactPath(join(output, maximumTestDirectory, file));
}
