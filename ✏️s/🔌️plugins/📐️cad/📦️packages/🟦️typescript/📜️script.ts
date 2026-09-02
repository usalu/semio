#!/usr/bin/env bun
/** 📐️ `@semio-tech/cad-js` task router: `bun ./📜️script.ts test|generate|fixture [args…]`. Folds the former cad-js-{core,renderer,kernel-brepjs,query,machine-stately,runtime} package scripts into one. */
import { join, resolve } from "node:path";
import Ajv from "ajv";
import type { BundleLinter } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { defineLint } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🔌️Dependency-boundary lint across the folded domain files (former per-package `policyFile` checks merged: renderer + stately each carried their own single-file variant). Scoped to the artifact-engine home of the compute modules; `📺️renderer` moved to the app's own `⚙️engine` as app-surface UI and is out of this compute-boundary lint's scope. */
export const policy = defineLint("@semio-tech/cad-js-modules", (_l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine");
});

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

class FixtureScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    process.env.CAD_GENERATE_STEP_FIXTURES = "1";
    runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

class GenerateScript extends BundleScript {
  async run(extra: string[]): Promise<void> {
    const { bootstrapCadModules } = await import("./🟦️");
    const { defaultModelDefinitionId } = await import("./🟦️");
    const { buildSpatialStatelyMachineCatalogView } = await import("./🟦️");
    bootstrapCadModules();
    let outPath = join(this.root, "../../🗿️artifacts/📐️cad/📚️examples/🔣️machine.json");
    let modelDefinitionId = defaultModelDefinitionId();
    const interactionIds: string[] = [];
    for (let i = 0; i < extra.length; i++) {
      const a = extra[i]!;
      if (a === "--out" && extra[i + 1]) {
        outPath = resolve(this.root, extra[i + 1]!);
        i++;
        continue;
      }
      if (a === "--model-definition" && extra[i + 1]) {
        modelDefinitionId = extra[i + 1]!;
        i++;
        continue;
      }
      if (!a.startsWith("-")) interactionIds.push(a);
    }
    const doc = buildSpatialStatelyMachineCatalogView({
      modelDefinitionId,
      interactionIds: interactionIds.length > 0 ? interactionIds : undefined,
    });
    await Bun.write(outPath, `${JSON.stringify(doc, null, 2)}\n`);
    console.error(`wrote ${outPath} (${doc.machines.length} machine(s))`);
  }
}

type RetainedAuditRoute = {
  id: string;
  disposition: "migrated" | "batchOnlyPendingRewrite";
};

type RetainedAuditFixture = {
  routeCount: number;
  excludedFrameworkRoutes: string[];
  admittedRoutes: string[];
  routes: RetainedAuditRoute[];
  limits: { rawBytes: number; decodedItems: number; workItems: number; outputBytes: number; stepMicros: number; closePageBytes: number };
};

class RetainedAuditScript extends BundleScript {
  async run(): Promise<void> {
    const fixturePath = resolve(this.root, "../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️retained-jobs.json");
    const schemaPath = resolve(this.root, "../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️retained-jobs.schema.json");
    const ownerPath = resolve(this.root, "../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
    const fixture = (await Bun.file(fixturePath).json()) as RetainedAuditFixture;
    const schema = await Bun.file(schemaPath).json();
    const owner = await Bun.file(ownerPath).text();
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    if (!validate(fixture)) throw new Error(`CAD retained audit schema rejected the fixture: ${JSON.stringify(validate.errors)}`);

    const commandBlock = owner.slice(owner.indexOf("semio_framework_plugin::app_commands!"), owner.indexOf("/// 🌉️ Converts"));
    const commandIds = [...commandBlock.matchAll(/^\s*"([^"]+)"\s+as\s+/gm)].map((match) => match[1]!).filter((id) => id !== "setActiveUtility");
    const routeIds = fixture.routes.map((route) => route.id);
    const expectedAdmitted = [
      "addNode", "renameNode", "patchCadPlayReference", "focusModelDefinition",
      "setCamera", "setProjection", "setProjectionParam", "setDislocateOption", "setNodeSelection", "setReferenceSelection", "referenceHover", "engagementInput", "engagementPossibleSelect", "engagementRepeatLast",
      "engagementAbort", "worldPointerMove", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity", "setActiveUtility", "setLocale", "setTerminology", "setContributions", "loadRawRequest",
    ];
    const retainedToolBlock = owner.slice(owner.indexOf("const CAD_RETAINED_TOOL_IDS"), owner.indexOf("const CAD_RETAINED_COMMAND_SCHEMA"));
    const retainedToolIds = [...retainedToolBlock.matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
    const semanticValid = (source: string): boolean => {
      const occurrences = (needle: string): number => source.split(needle).length - 1;
      const annotationPairs = [...source.matchAll(/\.action_interactive_job\("([^"]+)", semio_framework_plugin::InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => `${match[1]}:${match[2]}`);
      const expectedPairs = fixture.routes.map((route) => `${route.id}:${route.disposition === "migrated" ? "Migrated" : "BatchOnlyPendingRewrite"}`).concat("setActiveUtility:Migrated");
      return fixture.routeCount === 40
        && new Set(routeIds).size === 40
        && JSON.stringify(commandIds) === JSON.stringify(routeIds)
        && JSON.stringify(annotationPairs.sort()) === JSON.stringify(expectedPairs.sort())
        && JSON.stringify(fixture.excludedFrameworkRoutes) === JSON.stringify(["setActiveUtility"])
        && JSON.stringify(fixture.admittedRoutes) === JSON.stringify(expectedAdmitted)
        && JSON.stringify(retainedToolIds) === JSON.stringify(expectedAdmitted)
        && fixture.limits.closePageBytes === 16_384
        && source.includes('factory: "CadRetainedCommandJobFactory"')
        && source.includes("impl semio_framework::ToolJobFactory for CadRetainedCommandJobFactory")
        && source.includes("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for CadRetainedCommandJobFactory")
        && source.includes("type Owner = semio_framework_plugin::EditorApp<CadPlayApp>;")
        && source.includes("registry.register(CadRetainedCommandJobFactory::new(&controller))")
        && source.includes("fn build_config_store_one_item_preparation_factory()")
        && source.includes("fn build_artifact_store_one_item_preparation_factory()")
        && source.includes("CadConfigStorePreparationFactory")
        && source.includes("CadArtifactStorePreparationFactory")
        && source.includes("authority.prepare_one_item(edit, std::sync::Arc::new(post))")
        && source.includes("digest: prepared.edit_digest()")
        && source.includes('contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500)')
        && source.includes('tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config]')
        && occurrences('tool_id: "loadRawRequest", lanes: &[ArtifactToolPublicationLane::HostOnly]') === 1
        && !source.includes('factory: "BoundedFirstStepCommandJobFactory"');
    };
    if (!semanticValid(owner)) throw new Error("CAD retained audit fixture and canonical Rust owner diverged");

    const hostileSources = [
      owner.replace('factory: "CadRetainedCommandJobFactory"', 'factory: "BoundedFirstStepCommandJobFactory"'),
      owner.replace('.action_interactive_job("saveCurrent", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)', '.action_interactive_job("saveCurrent", semio_framework_plugin::InteractiveJobClassification::Migrated)'),
      owner.replace("fn build_config_store_one_item_preparation_factory()", "fn removed_config_store_one_item_preparation_factory()"),
      owner.replace('tool_id: "loadRawRequest", lanes: &[ArtifactToolPublicationLane::HostOnly]', 'tool_id: "loadRawRequest", lanes: &[ArtifactToolPublicationLane::Config]'),
      owner.replace('tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config]', 'tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::HostOnly]'),
      owner.replace('bounded_first_step(8_192, 64, 1, 16_384, 7_500)', 'bounded_first_step(8_192, 64, 1, 32_768, 7_500)'),
      owner.replace("impl semio_framework::ToolJobFactory for CadRetainedCommandJobFactory", "impl ToolJobFactory for CadRetainedCommandJobFactory"),
      owner.replace("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for CadRetainedCommandJobFactory", "impl ArtifactOwnedToolJobFactory for CadRetainedCommandJobFactory"),
      owner.replace("type Owner = semio_framework_plugin::EditorApp<CadPlayApp>;", "type Owner = EditorApp<CadPlayApp>;"),
      owner.replace("registry.register(CadRetainedCommandJobFactory::new(&controller))", "registry.register(BoundedFirstStepCommandJobFactory::new(&controller))"),
    ];
    const acceptedHostile = hostileSources.map((source, index) => semanticValid(source) ? index : -1).filter((index) => index >= 0);
    if (acceptedHostile.length > 0) throw new Error(`CAD retained audit accepted hostile source mutations ${acceptedHostile.join(",")}`);
    console.error(`validated ${routeIds.length} CAD routes; admitted=${fixture.admittedRoutes.join(",")}; schema=Ajv`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("fixture", FixtureScript).register("generate", GenerateScript).register("retained-audit", RetainedAuditScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
