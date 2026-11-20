import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { JSDOM } from "jsdom";
import ts from "typescript";

const ticket = dirname(fileURLToPath(import.meta.url));
const workspace = resolve(ticket, "../../../../../../..");
const expected = JSON.parse(readFileSync(resolve(ticket, "host-launch-cases.json"), "utf8"));
const path = resolve(workspace, expected.browserJourneyDir, expected.journey);
const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
const route = source.statements.flatMap(statement => ts.isVariableStatement(statement) ? [...statement.declarationList.declarations] : []).find(declaration => declaration.name.getText(source) === "url")?.initializer;
assert.ok(route && ts.isBinaryExpression(route) && route.operatorToken.kind === ts.SyntaxKind.QuestionQuestionToken && ts.isStringLiteral(route.right));
const descriptor = JSON.parse(readFileSync(resolve(workspace, expected.componentDescriptor), "utf8"));
assert.equal(descriptor.manifest.pluginId, expected.pluginId);
assert.ok(descriptor.manifest.examples.some(example => example.id === expected.exampleId));
const example = readFileSync(resolve(workspace, expected.exampleSource), "utf8");
assert.equal(example.match(/pub const ID: &str = "([^"]+)";/)?.[1], expected.exampleId);
const actual = new JSDOM("", { url: route.right.text }).window.location;
assert.equal(actual.port, String(expected.port));
assert.equal(new URL(actual.href).searchParams.get("plugin"), expected.pluginId);
assert.equal(new URL(actual.href).searchParams.get("example"), expected.exampleId);

let launch;
function visit(node) {
  if (ts.isCallExpression(node) && node.expression.getText(source) === "chromium.launch") launch = node.arguments[0];
  ts.forEachChild(node, visit);
}
visit(source);
assert.ok(launch && ts.isObjectLiteralExpression(launch));
const channel = launch.properties.find(property => property.name?.getText(source) === "channel");
assert.ok(channel && ts.isPropertyAssignment(channel) && ts.isStringLiteral(channel.initializer));
assert.equal(channel.initializer.text, expected.browserChannel);
