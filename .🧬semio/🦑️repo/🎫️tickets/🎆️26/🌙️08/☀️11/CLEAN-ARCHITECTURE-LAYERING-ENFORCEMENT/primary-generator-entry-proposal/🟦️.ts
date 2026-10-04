import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { getWorkspaceRoot } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import fixture from "../../🧫️fixtures/🏭️generate/🔣️.json";
import schema from "../../🧬️schema/🏭️generate/🔣️.json";

test("primary generator uses the existing taxonomy-owned aggregate and its declared executable target", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const root = getWorkspaceRoot(), taxonomy = JSON.parse(readFileSync(join(root, fixture.taxonomy), "utf8")), contract = taxonomy.generatorContracts[fixture.generatorContract];
  expect(contract.target).toBe(fixture.target);
  expect(contract.outputRoots.some((row: { path: string }) => row.path === fixture.output)).toBe(true);
  const project = JSON.parse(readFileSync(join(root, contract.ownerPath, "📋️project.json"), "utf8")), target = fixture.target.slice(fixture.target.lastIndexOf(":") + 1);
  expect(`${project.name}:${target}`).toBe(fixture.target);
  expect(project.targets[target].options.command).toBe(fixture.implementation);
  expect(project.targets[target].options.cwd).toBe(contract.ownerPath);
  console.log(`[DEBUG] primary generator aggregate authority: ${fixture.target}; existing declared permanent script; owned output ${fixture.output}`);
});

test("authored and rendered primary generator launcher preserve identity and select the canonical executable target", async () => {
  const root = getWorkspaceRoot(), source = readFileSync(join(root, fixture.seed), "utf8"), independent = ts.parseConfigFileTextToJson(fixture.seed, source);
  expect(independent.error).toBeUndefined();
  expect(Bun.JSONC.parse(source)).toEqual(independent.config);
  const { generateLaunchJson, declaredProjectTargets } = await import("../../🟦️.ts"), { generatePlaygroundRegistry } = await import("../../../🎮️playground/🔎️discovery/🟦️.ts");
  const rendered = generateLaunchJson(root, generatePlaygroundRegistry(root), declaredProjectTargets(root)), projected = ts.parseConfigFileTextToJson(fixture.output, rendered);
  expect(projected.error).toBeUndefined();
  expect(Bun.JSONC.parse(rendered)).toEqual(projected.config);
  for (const [label, document] of [["authored", independent.config], ["rendered", projected.config]] as const) {
    const selected = document.configurations.filter((row: { name?: string }) => row && typeof row === "object" && row.name === fixture.name);
    expect(selected.length, label).toBe(1);
    expect(selected[0], label).toMatchObject({ name: fixture.name, type: "node-terminal", request: "launch", command: `bun nx run ${fixture.target}`, cwd: "${workspaceFolder}", presentation: { group: fixture.group, order: fixture.order } });
  }
  console.log(`[DEBUG] authored and rendered primary generator select ${fixture.target}; preserved name/group/order; independent TypeScript JSONC admission`);
});
