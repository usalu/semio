/** 🪟️ Visual wire twins resolve to their real owner modules, checked with the TypeScript compiler oracle. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";
import vectors from "../🧫️fixtures/🪟️visual-imports/🔣️.json" with { type: "json" };

const plugin = resolve(import.meta.dir, "../..");

test("visual modules resolve every relative import and export", () => {
  const files = vectors.artifacts.flatMap(artifact => Object.keys(vectors.roles).map(role => join("🗿️artifacts", artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any", role, "🟦️.ts"))).concat(vectors.sharedModules);
  let scenes = 0;
  for (const file of files) {
    const source = readFileSync(join(plugin, file), "utf8");
    const references = [...source.matchAll(/(?:from|import)\s*["'](\.[^"']+)["']/gu)].map(match => match[1]);
    const oracle = ts.createSourceFile(file, source, ts.ScriptTarget.Latest).statements.flatMap(statement => (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) && statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) ? [statement.moduleSpecifier.text] : []).filter(path => path.startsWith("."));
    expect(references, file).toEqual(oracle);
    for (const reference of references) {
      const target = resolve(dirname(join(plugin, file)), reference);
      expect([target, `${target}.ts`, join(target, "index.ts")].some(existsSync), `${file}: ${reference}`).toBe(true);
      if (reference.includes("structural-scene")) {
        expect(target).toBe(join(plugin, "🎬️structural-scene/🧬️schema/🟦️.ts"));
        scenes++;
      }
    }
  }
  expect(scenes).toBe(vectors.expectedSceneImports);
});

test("editor and viewer twins publish the current dialect and mode", async () => {
  for (const artifact of vectors.artifacts) {
    const name = artifact.slice(artifact.indexOf("en")).toUpperCase();
    for (const [role, mode] of Object.entries(vectors.roles)) {
      const module = await import(join(plugin, "🗿️artifacts", artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any", role, "🟦️.ts"));
      expect(module[`${name}_${mode === "edit" ? "EDITOR" : "VIEWER"}_DIALECT`]).toEqual({ artifactKind: `s.norm.${name.toLowerCase()}`, standard: "1", subset: "*" });
      expect(module[`${name}_${mode.toUpperCase()}_MODE_ID`]).toBe(mode);
    }
  }
});

test("shared visual modules type check against the structural scene wire twin", () => {
  const program = ts.createProgram(vectors.sharedModules.map(file => join(plugin, file)), { noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, target: ts.ScriptTarget.ESNext, types: [] });
  expect(ts.getPreEmitDiagnostics(program).map(diagnostic => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
});
