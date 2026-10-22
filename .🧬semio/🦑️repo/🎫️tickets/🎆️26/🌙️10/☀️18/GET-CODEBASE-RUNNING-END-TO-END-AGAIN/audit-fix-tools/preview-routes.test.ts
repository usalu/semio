import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import parseArgv from "yargs-parser";
import { generatorPreviewExecution } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import { contractRoot } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🏗️shared-fixtures/🟦️.ts";
test("preview routes keep exact commands and attach the validator cause", () => {
  const root = join(contractRoot, "../🧫️fixtures/🏭️owned-generator-preview-inventory"), fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  let invalid = 0, valid = 0;
  for (const row of fixture.ownerExecutionRoutes) {
    const contract = { ownership: "owned" as const, ownerPath: row.ownerPath ?? "compiler", target: "@neutral/compiler:generate", previewTarget: "@neutral/compiler:preview-generated", ...(row.previewArguments ? { previewArguments: row.previewArguments } : {}), ...(row.previewScript ? { previewScript: row.previewScript } : {}) } as never;
    const target = { executor: "repo:script", options: { cwd: row.cwd, command: row.command } };
    if (!row.valid) { invalid++; expect(() => generatorPreviewExecution(contract, target)).toThrow("exact owner JSON preview"); }
    else { valid++; const route = generatorPreviewExecution(contract, target); expect([route.command, ...route.args].join(" "), row.id).toBe(row.command); expect(route.cwd).toBe(row.cwd); expect(parseArgv([route.command, ...route.args])).toEqual(parseArgv(row.command)); }
  }
  expect(valid).toBeGreaterThan(0);
  let caught: Error | undefined;
  try { generatorPreviewExecution({ ownership: "owned", ownerPath: "compiler", target: "a:b", previewTarget: "a:preview-generated" } as never, { executor: "repo:script", options: { cwd: "compiler", command: "bun \"unterminated" } }); } catch (error) { caught = error as Error; }
  expect(caught?.message).toContain("exact owner JSON preview");
  expect(caught?.cause).toBeInstanceOf(Error);
  expect(invalid + valid).toBe(fixture.ownerExecutionRoutes.length);
});
