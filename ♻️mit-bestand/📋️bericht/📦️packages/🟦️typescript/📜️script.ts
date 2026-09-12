#!/usr/bin/env bun
/** 🏚️ Validates report sources, Nx contracts and produced PDF documents. */
import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { basename, dirname, join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { ActorNetworkScript, actorNetworkData, validateActorNetwork, validateActorNetworkAssets, validateActorNetworkRenderSync } from "../../🔨️modules/👥️actor-network/📜️script.ts";
const packageRoot = import.meta.dir, ownerRoot = join(packageRoot, "../..");
const reportCatalog = JSON.parse(readFileSync(join(ownerRoot, "🔨️modules/📄️documents/🔣️.json"), "utf8")) as { documents: { id: string; texPath: string; actorNetwork: boolean }[] };
const DOCUMENTS = Object.fromEntries(reportCatalog.documents.map(document => [document.id, document.texPath]));

/** 🧪️ Verifies the report family and every literal appendix reference against the shared source tree. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const level = segments[0] ?? "quick";
    if (!["quick", "long", "exhaustive"].includes(level)) throw new Error(`unknown report test level: ${level}`);
    const fixture = JSON.parse(readFileSync(join(ownerRoot, "🧫️fixtures/🔣️report-family.json"), "utf8"));
    const require = createRequire(import.meta.url), modulePath = join(ownerRoot, "🔨️modules/📄️documents");
    const schema = JSON.parse(readFileSync(join(modulePath, "🧬️schema/🔣️.json"), "utf8"));
    assert.ok(new (require("ajv").default)({ strict: false }).validate(schema, reportCatalog));
    assert.deepEqual(reportCatalog.documents.map(document => ({ id: document.id, path: document.texPath })), fixture.documents);
    const project = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8"));
    for (const document of reportCatalog.documents) {
      const target = project.targets[`build-${document.id}`];
      assert.deepEqual(target.outputs, [`{projectRoot}/dist/documents/${document.id}`]);
      assert.equal(target.cache, true);
      assert.deepEqual(target.inputs.flatMap((input: any) => input.externalDependencies ?? []), ["pdfjs-dist", "sharp"]);
      assert.equal(target.inputs.includes("{workspaceRoot}/bun.lock"), false);
      assert.ok(target.inputs.includes("{workspaceRoot}/🧰️framework/🛍️products/📓️print/🔨️modules/🕸️graph/📐️.tex"));
      assert.equal(target.options.forwardAllArgs, false);
      for (const prerequisite of ["fonts", "deps-tectonic", "deps-tex", "generate"]) assert.ok(target.dependsOn.includes(`@semio-tech/print:${prerequisite}`));
      assert.equal(target.dependsOn.includes("generate-actor-network"), document.actorNetwork);
      assert.deepEqual(project.targets[`watch-${document.id}`].dependsOn, [`build-${document.id}`]);
    }
    assert.deepEqual(project.targets.build.dependsOn, reportCatalog.documents.map(document => `build-${document.id}`));
    const boundaries = JSON.parse(readFileSync(join(ownerRoot, "🧫️fixtures/🧭️commands.json"), "utf8"));
    for (const entry of boundaries.entries) {
      const bundle = await require("esbuild").build({ entryPoints: [join(ownerRoot, entry)], bundle: true, write: false, metafile: true, platform: "node", format: "esm", packages: "external", logLevel: "silent" });
      const inputs = Object.keys(bundle.metafile.inputs);
      assert.ok(inputs.length <= boundaries.maxSourceFiles, `${entry} imports ${inputs.length} source files`);
      for (const forbidden of boundaries.forbidden) assert.ok(!inputs.some(path => path.includes(forbidden)), `${entry} imports ${forbidden}`);
      const external = [...new Set(Object.values(bundle.metafile.outputs).flatMap((output: any) => output.imports).filter((item: any) => item.external && !item.path.startsWith("node:") && item.path !== "bun").map((item: any) => item.path.startsWith("@") ? item.path.split("/").slice(0, 2).join("/") : item.path.split("/")[0]))].sort();
      assert.deepEqual(external, boundaries.externalDependencies[entry], `${entry}: external imports`);
      console.log(`[DEBUG] Report command ${entry}: ${inputs.length} production source files verified`);
    }
    const contract = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")).generatorContracts["report-actor-network"];
    assert.equal(contract.target, `${project.name}:generate-actor-network`);
    assert.deepEqual(project.targets["generate-actor-network"].inputs.flatMap((input: any) => input.externalDependencies ?? []), ["d3-force"]);
    assert.deepEqual(contract.outputRoots.map((output: { path: string }) => `{workspaceRoot}/${output.path}`).sort(), [...project.targets["generate-actor-network"].outputs].sort());
    console.log("[DEBUG] Report catalog, Nx output ownership and generator contract verified");
    for (const document of fixture.documents) {
      assert.equal(DOCUMENTS[document.id as keyof typeof DOCUMENTS], document.path);
      const root = dirname(join(ownerRoot, document.path));
      const visited = new Set<string>();
      const citations = new Set<string>();
      const verify = (path: string): void => {
        if (visited.has(path)) return;
        visited.add(path);
        const source = readFileSync(path, "utf8");
        for (const match of source.matchAll(/\\[a-zA-Z]*cite[a-zA-Z]*(?:\[[^\]]*\])*\{([^{}]*)\}/g)) {
          for (const key of match[1]!.split(",").map((key) => key.trim())) if (!key.includes("\\")) citations.add(key);
        }
        const inputs = [...source.matchAll(/\\input\{([^}]+)\}/g), ...source.matchAll(/\\appendixinput(?:\[[^\]]*\])?\{[^{}]*\}\{([^{}]+)\}/g)];
        for (const match of inputs) {
          if (match[1]!.includes("\\")) continue;
          const input = resolve(root, match[1]!.endsWith(".tex") ? match[1]! : `${match[1]}.tex`);
          assert.ok(existsSync(input), `missing report input: ${input}`);
          verify(input);
        }
      };
      verify(join(ownerRoot, document.path));
      const bibliography = readFileSync(join(root, "📚️references.bib"), "utf8");
      const keys = new Set([...bibliography.matchAll(/@\w+\s*\{([^,\s]+)/g)].map((match) => match[1]!));
      for (const key of citations) assert.ok(keys.has(key), `missing ${document.id} citation: ${key}`);
      console.log(`[DEBUG] report ${document.id}: ${visited.size} source documents resolved`);
    }
    const data = actorNetworkData();
    assert.deepEqual({ nodes: data.nodes.length, edges: data.edges.length, programs: data.programs.length }, fixture.actorNetwork);
    validateActorNetwork(data);
    validateActorNetworkAssets(data);
    if (level !== "quick") {
      validateActorNetworkRenderSync(data);
      const canvas = require("@napi-rs/canvas");
      (globalThis as { DOMMatrix?: unknown }).DOMMatrix ??= canvas.DOMMatrix;
      const { getDocument } = await import("pdfjs-dist/legacy/build/pdf.mjs");
      const expected = JSON.parse(readFileSync(join(ownerRoot, "🧫️fixtures/📄️pdf.json"), "utf8"));
      for (const vector of expected.documents) {
        const path = join(packageRoot, "dist/documents", vector.id, `${basename(DOCUMENTS[vector.id]!, ".tex")}.pdf`);
        const pdf = await getDocument({ data: new Uint8Array(readFileSync(path)) }).promise;
        try {
          assert.ok(pdf.numPages >= vector.minPages, `${vector.id} has ${pdf.numPages} pages`);
          const text: string[] = [];
          for (let index = 1; index <= pdf.numPages; index++) {
            const page = await pdf.getPage(index);
            text.push((await page.getTextContent()).items.map(item => "str" in item ? item.str : "").join(" "));
            page.cleanup();
          }
          const content = text.join(" ").replace(/\s+/g, " ");
          for (const text of vector.text) assert.ok(content.includes(text), `${vector.id} is missing ${text}`);
          console.log(`[DEBUG] Report ${vector.id}: independent PDF.js read ${pdf.numPages} pages and expected content`);
        } finally { await pdf.destroy(); }
      }
    }
  }
}

/** 🖼️ Routes the owned non-writing preview through the package command boundary. */
class PreviewScript extends ActorNetworkScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("Report preview accepts no arguments");
    await super.run(["preview"]);
  }
}

const router = new ScriptRouter(packageRoot).register("test", TestScript).register("preview-generated", PreviewScript);
if (import.meta.main) await router.run(process.argv.length > 2 ? process.argv.slice(2) : ["test"]);
