#!/usr/bin/env bun
/** ➗️ Mathematical source, schema, and publication-authority laws. */
import { resolve } from "node:path";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
type Lane = "Artifact" | "Config";
type Fixture = { schema: string; owner: "MathematicalPlayApp"; source: string; routes: { id: string; lane: Lane }[]; laws: Record<string, boolean>; ui: { locales: ["en", "de"]; accessibleLabels: boolean; customizableUi: boolean } };

const exact = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;

function oracle(fixture: Fixture, source: string): boolean {
  const ids = [...source.match(/MATHEMATICAL_TOOL_IDS: &\[&str\] = &\[([^\]]*)\]/s)?.[1]?.matchAll(/"([^"]+)"/g) ?? []].map((match) => match[1]!);
  const contracts = new Map([...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[ArtifactToolPublicationLane::(Artifact|Config)\] \}/g)].map((match) => [match[1]!, match[2]! as Lane]));
  const classifications = [...source.matchAll(/\.action_interactive_job\("([^"]+)", InteractiveJobClassification::Migrated\)/g)].map((match) => match[1]!);
  const expected = fixture.routes.map(({ id }) => id);
  return fixture.schema === "semio.app.publication-authority.v1" && Object.values(fixture.laws).every(Boolean)
    && fixture.ui.locales.join(",") === "en,de" && fixture.ui.accessibleLabels && fixture.ui.customizableUi
    && exact(ids, expected) && exact(classifications, expected) && exact([...contracts.keys()], expected)
    && fixture.routes.every(({ id, lane }) => contracts.get(id) === lane)
    && ["ToolExecutionContract::resumable", "semio_framework_plugin::bounded_first_step_tool_proofs!", "build_artifact_store_one_item_preparation_factory", "build_config_store_one_item_preparation_factory", "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()", "authority.prepare_one_item", "fn cancel(&mut self)", "fn begin_close(&mut self)", "base.return_to_registry()", "fn terminal_is_empty(&self)", "LocalizedLabel::native", ".default_layout(edit::layout())"].every((anchor) => source.includes(anchor));
}

class TestScript extends BundleScript {
  async run(): Promise<void> {
    const manifest = await Bun.file(resolve(import.meta.dir, "package.json")).json() as Record<string, unknown>;
    const scripts = manifest.scripts as Record<string, unknown>;
    const dependencies = manifest.dependencies as Record<string, unknown>;
    if (typeof manifest.description !== "string" || !manifest.description.includes("Mathematical plugin TS") || manifest.description.includes("CAD plugin")) throw new Error("Mathematical package description is not domain-scoped");
    if (JSON.stringify(scripts) !== JSON.stringify({ test: "bun nx run @semio-tech/mathematical-js:test" })) throw new Error("Mathematical package scripts do not match its Nx targets");
    if (JSON.stringify(dependencies) !== JSON.stringify({ ajv: "^8.20.0" })) throw new Error("Mathematical package dependencies are not source-scoped");
    const plugin = resolve(this.root, "../..");
    const authority = resolve(plugin, "🧪️publication-authority");
    const fixture = await Bun.file(resolve(authority, "🔣️.json")).json() as Fixture;
    const schema = await Bun.file(resolve(authority, "🔣️.schema.json")).json();
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    if (!validate(fixture)) throw new Error(`Mathematical fixture failed strict Ajv: ${JSON.stringify(validate.errors)}`);
    const source = await Bun.file(resolve(plugin, fixture.source)).text();
    if (!oracle(fixture, source)) throw new Error("Mathematical publication-authority oracle rejected production");
    const hostileSource = [source.replace('ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },', ""), source.replace("            || request.generation != request.authority.generation()\n", ""), source.replace('.action_interactive_job("setPoints", InteractiveJobClassification::Migrated)', "")];
    if (hostileSource.some((candidate) => oracle(fixture, candidate))) throw new Error("Mathematical oracle accepted a hostile source mutation");
    if (validate({ ...fixture, extra: true })) throw new Error("Mathematical strict schema accepted an extra property");
    console.error(`validated Mathematical publication authority; routes=${fixture.routes.length}; schema=Ajv; oracle=owned; hostile=3`);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
