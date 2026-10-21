import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { significantConsoleErrors } from "./🟦️.ts";

const cases = JSON.parse(readFileSync(new URL("./🧫️cases.json", import.meta.url), "utf8")) as {
  readonly cases: readonly { readonly name: string; readonly message: string; readonly significant: boolean }[];
};
const schema = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const reference = new Ajv({ strict: true }).compile({
  type: "string",
  not: { enum: [
    "failed to load resource: the server responded with a status of 404",
    "failed to load resource: the server responded with a status of 404 (not found)",
  ] },
});

describe("acceptance console errors", () => {
  it("validates language-neutral examples with Ajv", () => {
    expect(new Ajv({ strict: true }).validate(schema, cases)).toBe(true);
  });
  it.each(cases.cases)("$name", ({ message, significant }) => {
    expect(reference(message.toLowerCase())).toBe(significant);
    expect(significantConsoleErrors([message])).toEqual(significant ? [message] : []);
  });
});

it("validates the authored curation interaction contract with Ajv", () => {
  const workflow = JSON.parse(readFileSync(new URL("../🧬️curation/🧫️cases.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️curation/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv({ strict: true }).validate(schema, workflow)).toBe(true);
});

it("validates the authored Energy and FEM interaction contract with Ajv", () => {
  const cases = JSON.parse(readFileSync(new URL("../🧬️simulation/🧫️cases.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️simulation/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv({ strict: true }).validate(schema, cases)).toBe(true);
});

it("validates all five authored document-edit contracts with Ajv", () => {
  const cases = JSON.parse(readFileSync(new URL("../🧬️edits/🧫️cases.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../🧬️edits/🔣️.json", import.meta.url), "utf8"));
  expect(new Ajv({ strict: true }).validate(schema, cases)).toBe(true);
});


import ts from "typescript";
import { isNewToolRunGroup } from "../🧬️simulation/🟦️.ts";

it("requires a distinct real run group before cancelling a restarted run", () => {
  const workflow = JSON.parse(readFileSync(new URL("../🧬️simulation/🧫️cases.json", import.meta.url), "utf8"));
  for (const row of workflow.restarts) {
    const reference = new Ajv({ strict: true }).compile({ type: "string", pattern: "^framework\\.toolRun\\.[0-9]+$", not: { const: row.previous } });
    expect(isNewToolRunGroup(row.previous, row.candidate), row.name).toBe(row.accepted);
    expect(reference(row.candidate), row.name).toBe(row.accepted);
  }
});

it("uses the actual read-only automatic finalization policy in the browser workflow", () => {
  const workflow = JSON.parse(readFileSync(new URL("../🧬️simulation/🧫️cases.json", import.meta.url), "utf8"));
  const contract = JSON.parse(readFileSync(new URL("../../../../../✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️simulation-session/🔣️.json", import.meta.url), "utf8"));
  const driver = readFileSync(new URL("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🧵️instance/🎟️job-admission-and-finalize-turn/🦀️.rs", import.meta.url), "utf8");
  const begin = driver.indexOf("fn complete_tool_run_job("), end = driver.indexOf("fn complete_tool_run_revalidation(", begin), completion = driver.slice(begin, end);
  expect(contract["x-semio-toolRun"].mutating).toBe(false);
  expect(workflow.energy.completion).toBe("automatic");
  expect(completion).toMatch(/!entry\.definition\.mutating\s*&&\s*entry\.slot\.state\s*==\s*ToolRunState::Complete/);
  expect(completion.indexOf("ToolRunEvent::Finalize")).toBeGreaterThan(completion.indexOf("ToolRunEvent::JobComplete"));
  const source = readFileSync(new URL("../🟦️.ts", import.meta.url), "utf8"), ast = ts.createSourceFile("acceptance.ts", source, ts.ScriptTarget.Latest, true);
  const statement = ast.statements.find(node => ts.isExpressionStatement(node) && ts.isCallExpression(node.expression) && node.expression.arguments[0] !== undefined && ts.isStringLiteral(node.expression.arguments[0]) && node.expression.arguments[0].text === "energie run-cancel");
  expect(statement).toBeDefined();
  const properties: string[] = [], calls: string[] = [];
  const walk = (node: ts.Node): void => { if (ts.isPropertyAccessExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "scenario") properties.push(node.name.text); if (ts.isCallExpression(node) && ts.isIdentifier(node.expression)) calls.push(node.expression.text); ts.forEachChild(node, walk); };
  walk(statement!);
  expect(properties).toContain("finalized");
  expect(properties).toContain("finalStep");
  expect(properties).toContain("acceptedResult");
  expect(properties).not.toContain("complete");
  expect(statement!.getText(ast)).not.toContain("toolRunFinalize");
  expect(statement!.getText(ast)).toContain("toolRunAbort");
  expect(calls).toContain("isNewToolRunGroup");
});
