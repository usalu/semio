import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import ts from "typescript";
import type { DEMONSTRATOR_PANES } from "../../🪧️brand.ts";

/** 📚️ Keeps branded defaults aligned with authored, registered artifact examples. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: { panes: typeof DEMONSTRATOR_PANES }): Promise<void> {
  const { describe, expect, it } = vitest;
  const directory = dirname(fileURLToPath(import.meta.url)), workspace = resolve(directory, "../../../..");
  const fixture = JSON.parse(readFileSync(resolve(directory, "🧫️cases.json"), "utf8")) as { cases: { paneId: string; brand: string; exampleId: string; source: string }[]; catalogueRows: { paneId: string; editsKey: string; rowId: string; kind: "dsl-object-id" | "cad-typology"; needle: string; source: string }[] };
  const edits = JSON.parse(readFileSync(resolve(directory, "../🎭️acceptance/🧬️edits/🧫️cases.json"), "utf8")) as Record<string, Record<string, unknown>>;
  const schema = JSON.parse(readFileSync(resolve(directory, "🧬️schema/🔣️.json"), "utf8"));
  const source = ts.createSourceFile("brand.ts", readFileSync(resolve(directory, "../../🪧️brand.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const property = (node: ts.ObjectLiteralExpression, name: string): ts.Expression | undefined => node.properties.find((member): member is ts.PropertyAssignment => ts.isPropertyAssignment(member) && member.name.getText(source) === name)?.initializer;
  const declared = new Map<string, string>();
  const visit = (node: ts.Node): void => {
    if (ts.isVariableDeclaration(node) && node.initializer && ts.isObjectLiteralExpression(node.initializer)) {
      const defaults = property(node.initializer, "defaults");
      const exampleId = defaults && ts.isObjectLiteralExpression(defaults) ? property(defaults, "exampleId") : undefined;
      if (exampleId && ts.isStringLiteral(exampleId)) declared.set(node.name.getText(source), exampleId.text);
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  describe("demonstrator registered example defaults", () => {
    it("validates its language-neutral contract", () => {
      expect(new Ajv().validate(schema, fixture)).toBe(true);
    });
    for (const row of fixture.cases) it(`${row.paneId} opens ${row.exampleId}`, () => {
      const registered = /pub const ID: &str = "([^"]+)";/.exec(readFileSync(resolve(workspace, row.source), "utf8"))?.[1];
      expect(registered).toBe(row.exampleId);
      expect(declared.get(row.brand)).toBe(registered);
      expect(dependencies.panes.find(pane => pane.id === row.paneId)?.brand.defaults?.exampleId).toBe(registered);
    });
    for (const row of fixture.catalogueRows) it(`${row.paneId} catalogue row ${row.rowId} resolves in its authored source`, () => {
      expect(edits[row.paneId]?.[row.editsKey]).toBe(row.rowId);
      expect(readFileSync(resolve(workspace, row.source), "utf8")).toContain(row.needle);
      if (row.kind === "cad-typology") expect(row.rowId).toBe(`cad-play-catalogue.${/typology: "([^"]+)"/.exec(row.needle)?.[1]}`);
    });
  });
}
