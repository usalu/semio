import { registerE2eSourceScopeTests } from "./🎭️e2e/🟦️.ts";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { fileURLToPath } from "node:url";

/** 🎪️ Ensures the demonstrator wasm link only pulls pane artifact crates, not full multi-artifact plugins. */
export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>): Promise<void> {
  const { describe, expect, it } = vitest;
  const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "../../../..");
  await registerE2eSourceScopeTests(vitest, repoRoot);
  const cargoPath = join(repoRoot, "🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml");
  const cargo = readFileSync(cargoPath, "utf8");

  describe("demonstratorCompileClosure", () => {
    it("does not link whole puzzle, procedural, or gis plugin composition crates", () => {
      expect(cargo).not.toMatch(/semio-hub-puzzle/);
      expect(cargo).not.toMatch(/semio-hub-procedural/);
      expect(cargo).not.toMatch(/semio-s-plugin-gis/);
    });

    it("links only the artifact crates used by the eight demonstrator panes", () => {
      expect(cargo).toContain("semio-s-artifact-puzzle-3d");
      expect(cargo).toContain("semio-s-artifact-procedural-generation3d");
      expect(cargo).toContain("semio-s-artifact-gis-gismap");
      expect(cargo).not.toContain("semio-s-artifact-puzzle-2d");
      expect(cargo).not.toContain("semio-s-artifact-puzzle-5d");
      expect(cargo).not.toContain("semio-s-artifact-procedural-generation2d");
      expect(cargo).not.toContain("semio-s-artifact-procedural-assembly");
      expect(cargo).not.toContain("semio-s-artifact-gis-gisterrain");
    });
  });
  describe("demonstrator unit source scope", () => {
    it("retains every configured and guarded test entry without promoting all app sources", () => {
      const root = resolve(repoRoot, "♻️mit-bestand/🧺️demonstrator"), directory = dirname(fileURLToPath(import.meta.url));
      const fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8")) as { entries: string[]; freshnessDataInputs: string[]; dataResources: string[]; cases: { target: string; executionVersion: number; executionKind: string; observation: string; command: string }[]; controls: { target: string; executionKind: null; observation: null }[] };
      const schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8"));
      expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
      const manifest = JSON.parse(readFileSync(join(root, "📋️project.json"), "utf8"));
      const executionSchema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🧬️schema/🔣️.json"), "utf8"));
      const validateExecution = new Ajv({ strict: true }).compile({ ...executionSchema, $ref: "#/definitions/Source" });
      expect(validateExecution({ kind: "source" })).toBe(false);
      const configPath = join(root, "🧪️tests/🎚️config/🟦️.ts"), configured = new Set([configPath]);
      const parse = (path: string) => ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
      const visitConfig = (node: ts.Node): void => {
        if (ts.isPropertyAssignment(node) && ["include", "includeSource"].includes(node.name.getText().replace(/^["']|["']$/g, ""))) {
          const collect = (child: ts.Node): void => { if (ts.isStringLiteral(child) && child.text.startsWith("./") && child.text.endsWith(".ts")) configured.add(resolve(root, child.text)); ts.forEachChild(child, collect); };
          collect(node.initializer);
        }
        ts.forEachChild(node, visitConfig);
      };
      visitConfig(parse(configPath));
      const pending = [...configured], visited = new Set<string>();
      while (pending.length) {
        const path = pending.pop()!; if (visited.has(path)) continue; visited.add(path);
        const visit = (node: ts.Node): void => {
          if (ts.isIfStatement(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "vitest" && ts.isMetaProperty(node.expression.expression)) {
            const collect = (child: ts.Node): void => {
              if (ts.isCallExpression(child) && child.expression.kind === ts.SyntaxKind.ImportKeyword && child.arguments.length === 1 && ts.isStringLiteral(child.arguments[0]) && child.arguments[0].text.startsWith(".")) { const entry = resolve(dirname(path), child.arguments[0].text); if (!configured.has(entry)) { configured.add(entry); pending.push(entry); } }
              ts.forEachChild(child, collect);
            };
            collect(node.thenStatement);
          }
          ts.forEachChild(node, visit);
        };
        visit(parse(path));
      }
      const expected = [...fixture.entries].sort(), actual = [...configured].map(path => "{projectRoot}/" + relative(root, path).replaceAll("\\", "/")).sort();
      expect(actual).toEqual(expected);
      const oracleReads=(path:string,projectTypeScript=false):string[]=>{const files=new Set<string>();const visit=(node:ts.Node):void=>{const args=ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="mapTileFunctionV1"?node.arguments:ts.isNewExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="URL"?node.arguments:undefined;if(args?.[0]&&ts.isStringLiteral(args[0])&&args[0].text.startsWith(".")){const resource=resolve(dirname(path),args[0].text);if(relative(root,resource).startsWith(".."))files.add("{workspaceRoot}/"+relative(repoRoot,resource).replaceAll("\\","/"));else if(projectTypeScript&&resource.endsWith(".ts"))files.add("{projectRoot}/"+relative(root,resource).replaceAll("\\","/"));}ts.forEachChild(node,visit);};visit(parse(path));return [...files].sort();};
      expect(oracleReads(join(root,"🧪️tests/🧪️demonstratormaptiles/🟦️.ts"))).toEqual([...fixture.freshnessDataInputs].sort());
      expect(oracleReads(join(root,"🧪️tests/🎭️acceptance/🧬️console/🧪️.ts"),true)).toEqual([...fixture.dataResources].sort());
      expect(fixture.cases.map(row => row.target).sort()).toEqual(["canonical-architecture", "test"]);
      for (const row of fixture.cases) {
        const target = manifest.targets[row.target], semio = target.metadata?.semio;
        expect(validateExecution(semio?.execution), row.target).toBe(true);
        expect(semio?.execution?.version, row.target).toBe(row.executionVersion);
        expect(semio?.execution?.kind, row.target).toBe(row.executionKind);
        expect(semio?.sourceInputObservation, row.target).toBe(row.observation);
        expect([...semio.runtimeSourceEntries].sort(), row.target).toEqual(expected);
        expect(target.inputs,row.target).toEqual(expect.arrayContaining(["default","^default",...fixture.freshnessDataInputs]));
        expect(semio.closure?.resources,row.target).toEqual(expect.arrayContaining(fixture.dataResources));
        expect(target.options.command, row.target).toBe(row.command);
        expect(target.options.forwardAllArgs, row.target).toBe(true);
      }
      for (const row of fixture.controls) {
        const semio = manifest.targets[row.target].metadata?.semio;
        expect(semio?.execution?.kind ?? null, row.target).toBe(row.executionKind);
        expect(semio?.sourceInputObservation ?? null, row.target).toBe(row.observation);
      }
    });
  });

}
