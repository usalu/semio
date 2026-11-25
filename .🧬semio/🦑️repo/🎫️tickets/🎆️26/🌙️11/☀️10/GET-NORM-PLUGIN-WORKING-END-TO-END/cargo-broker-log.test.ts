import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import ts from "typescript";
import fixture from "./cargo-broker-log-fixture.json" with { type: "json" };

test("the ordinary Cargo launcher routes unit admission messages to its stderr reporter", () => {
  const source = ts.createSourceFile(fixture.launcher, readFileSync(fixture.launcher, "utf8"), ts.ScriptTarget.Latest, true);
  const reported: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.expression.getText(source) === fixture.broker) {
      const options = node.arguments[0];
      if (options && ts.isObjectLiteralExpression(options)) {
        const logger = options.properties.find(property => property.name?.getText(source) === "log");
        if (logger && ts.isPropertyAssignment(logger)) reported.push(logger.initializer.getText(source));
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  expect(reported).toEqual([fixture.reporter]);
});
