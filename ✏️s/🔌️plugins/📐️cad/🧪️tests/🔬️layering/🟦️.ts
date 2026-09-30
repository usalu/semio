import { readdir, readFile } from "node:fs/promises";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import Ajv from "ajv";
import { build } from "esbuild";
import { merge } from "lodash";
import { core } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { describe, expect, it } from "vitest";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");

async function sources(folder: string): Promise<string[]> {
  const entries = await readdir(folder, { withFileTypes: true });
  return (await Promise.all(entries.map((entry) => entry.isDirectory() ? sources(resolve(folder, entry.name)) : Promise.resolve(entry.name.endsWith(".ts") ? [resolve(folder, entry.name)] : [])))).flat();
}

function imports(source: string): string[] {
  const file = ts.createSourceFile("source.ts", source, ts.ScriptTarget.Latest, true);
  const values: string[] = [];
  const visit = (node: ts.Node): void => {
    if ((ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) && node.moduleSpecifier && ts.isStringLiteral(node.moduleSpecifier)) values.push(node.moduleSpecifier.text);
    if (ts.isCallExpression(node) && node.expression.kind === ts.SyntaxKind.ImportKeyword && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) values.push(node.arguments[0].text);
    ts.forEachChild(node, visit);
  };
  visit(file);
  return values;
}

describe("spatial and CAD ownership", () => {
  it("answers contribution vectors with zero, independent, and removed owners", async () => {
    const fixture = JSON.parse(await readFile(resolve(root, "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧫️fixtures/🔌️contributions/🔣️.json"), "utf8"));
    const schema = JSON.parse(await readFile(resolve(root, "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🧬️schema/🔌️contributions/🔣️.json"), "utf8"));
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const contributions = new Map<string, Record<string, unknown>>();
    const removals = new Map<string, () => void>();
    const events: string[][] = [];
    const stop = core.onModelDefinitionAssetsChanged(() => events.push(core.listModelDefinitionManifests().map((row) => row.id).sort()));
    try {
      for (const step of fixture.steps) {
        if (step.operation === "register") {
          const manifests = { [step.owner]: step.manifest };
          contributions.set(step.owner, manifests);
          removals.set(step.owner, core.registerModelDefinitionAssets({ typologies: {}, actions: {}, interactions: {}, manifests, extensions: {}, attributes: {}, propertyDefinitions: {}, properties: {}, statDefinitions: {}, transformations: {} }));
        } else if (step.operation === "remove") {
          contributions.delete(step.owner);
          removals.get(step.owner)?.();
        }
        const ids = core.listModelDefinitionManifests().map((row) => row.id).sort();
        const oracle = Object.values(merge({}, ...contributions.values())).map((row) => (row as { id: string }).id).sort();
        expect(ids).toEqual(step.ids);
        expect(ids).toEqual(oracle);
      }
      expect(events).toEqual([["shape"], ["energy", "shape"], ["energy"], []]);
      stop();
      const remove = core.registerModelDefinitionAssets({ typologies: {}, actions: {}, interactions: {}, manifests: {}, extensions: {}, attributes: {}, propertyDefinitions: {}, properties: {}, statDefinitions: {}, transformations: {} });
      remove();
      expect(events).toHaveLength(4);
    } finally {
      stop();
      for (const remove of removals.values()) remove();
    }
  });
  it("answers removable command vectors and restores independent owners", async () => {
    const fixture = JSON.parse(await readFile(resolve(root, "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🧫️fixtures/🔌️commands/🔣️.json"), "utf8"));
    const schema = JSON.parse(await readFile(resolve(root, "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🧬️schema/🔌️commands/🔣️.json"), "utf8"));
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    const contributions = new Map<string, { owner: string }>();
    const removals = new Map<string, () => void>();
    const kernel = new core.SemioBrepKernel();
    try {
      for (const step of fixture.steps) {
        if (step.operation === "register") {
          contributions.set(step.owner, { owner: step.owner });
          removals.set(step.owner, core.registerSpatialKernelCommand(fixture.command, async () => ({ diff: { solids: { added: [{ id: core.solidRef(step.owner), shellIds: [] }] } } })));
        } else if (step.operation === "remove") {
          contributions.delete(step.owner);
          removals.get(step.owner)?.();
        }
        const command = core.spatialKernelCommandFor(fixture.command);
        const result = command ? await command(kernel, {}) : null;
        const answer = result?.diff.solids?.added?.[0]?.id ?? null;
        const oracle = merge({}, ...contributions.values()).owner ?? null;
        expect(answer).toEqual(step.expect);
        expect(answer).toEqual(oracle);
      }
    } finally {
      for (const remove of removals.values()) remove();
      await kernel.close();
    }
  });
  it("resolves the CAD package with every artifact and oracle hidden", async () => {
    const result = await build({
      absWorkingDir: root,
      entryPoints: ["✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️.ts"],
      bundle: true,
      write: false,
      format: "esm",
      platform: "browser",
      packages: "external",
      external: ["node:*", "*.wasm"],
      define: { "import.meta.vitest": "undefined" },
      plugins: [{
        name: "hide-specific-owners",
        setup(builder) {
          builder.onLoad({ filter: /./ }, (args) => args.path.includes("🗿️artifacts") || args.path.includes("🧪️tests") ? { errors: [{ text: `Deleted owner: ${args.path}` }] } : undefined);
        },
      }],
    });
    expect(result.outputFiles[0]!.text).toContain("registerModelDefinitionAssets");
  });
  it("keeps every spatial-kernel source independent of deletable plugins", async () => {
    const files = await sources(resolve(root, "✏️s/🔨️modules/🌐️spatial-kernel"));
    const violations = (await Promise.all(files.map(async (file) => imports(await readFile(file, "utf8")).filter((value) => value.includes("🔌️plugins") || value.includes("@semio-tech/cad")).map((value) => `${file}: ${value}`)))).flat();
    expect(violations).toEqual([]);
  });
  it("keeps the CAD engine and package entry independent of artifacts and test oracles", async () => {
    const files = [...await sources(resolve(root, "✏️s/🔌️plugins/📐️cad/⚙️engine")), resolve(root, "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️.ts")];
    const violations = (await Promise.all(files.map(async (file) => imports(await readFile(file, "utf8")).filter((value) => value.includes("🗿️artifacts") || value.includes("🧪️tests")).map((value) => `${file}: ${value}`)))).flat();
    expect(violations).toEqual([]);
  });
});
