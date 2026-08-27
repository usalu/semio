#!/usr/bin/env bun
/** 🌊️ `@semio-tech/flow-js` router: `bun ./📜️script.ts test`. */
import { resolve } from "node:path";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "🧪️vitest.config.ts");
  }
}

type Lane = "Artifact" | "Config" | "Draft" | "Presence" | "Transient" | "Child" | "HostOnly";
type Group = { status: "Migrated" | "BatchOnlyPendingRewrite"; lanes: Lane[]; routes: string[]; blocker?: string };
type Fixture = {
  schema: string;
  owner: "FlowPlayApp" | "NotePlayApp";
  source: string;
  routeCount: number;
  retainedRoutes: string[];
  frameworkOwnedRoutes: string[];
  groups: Group[];
  globals: { symbol: string; scope: "framework-process" | "plugin-process"; blocker: string }[];
  scanThenMonolithRoutes: string[];
  blockedSeams?: string[];
  materialization?: Record<string, boolean | number>;
  laws: Record<string, boolean>;
};

function exact(left: string[], right: string[]): boolean {
  return new Set(left).size === left.length && new Set(right).size === right.length && JSON.stringify([...left].sort()) === JSON.stringify([...right].sort());
}

function commandRows(source: string): string[] {
  return [...source.matchAll(/^\s*"([^"]+)" as "[^"]+" =>/gm)].map((match) => match[1]!);
}

function manifestRows(source: string): Map<string, string> {
  return new Map([...source.matchAll(/\.action_interactive_job\("([^"]+)",\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => [match[1]!, match[2]!]));
}

function publicationRows(source: string): string[] {
  return [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes:/g)].map((match) => match[1]!);
}

function exactPublication(source: string, route: string, lanes: Lane[]): boolean {
  const declared = lanes.map((lane) => `(?:semio_framework_plugin::)?ArtifactToolPublicationLane::${lane}`).join(",\\s*");
  return new RegExp(`ArtifactToolPublicationContract \\{ tool_id: "${route}", lanes: &\\[${declared}\\] \\}`).test(source);
}

function fixtureOracle(fixture: Fixture): boolean {
  const classified = fixture.groups.flatMap((group) => group.routes);
  const retained = fixture.groups.filter((group) => group.status === "Migrated").flatMap((group) => group.routes);
  return ["semio.flow-note.action-cohort.v1", "semio.note.action-cohort.v1"].includes(fixture.schema)
    && exact(fixture.retainedRoutes, retained)
    && classified.length === fixture.routeCount - fixture.frameworkOwnedRoutes.length
    && new Set(classified).size === classified.length
    && new Set(fixture.frameworkOwnedRoutes).size === fixture.frameworkOwnedRoutes.length
    && fixture.scanThenMonolithRoutes.length === 0
    && Object.values(fixture.laws).every(Boolean)
    && fixture.groups.every((group) => group.routes.length > 0
      && group.lanes.length > 0
      && new Set(group.lanes).size === group.lanes.length
      && (group.status === "Migrated" ? group.blocker === undefined : Boolean(group.blocker?.length))
      && (!group.lanes.includes("HostOnly") || group.lanes.length === 1));
}

function sourceOracle(fixture: Fixture, source: string, retainedSource = ""): boolean {
  const classified = fixture.groups.flatMap((group) => group.routes);
  const pairs = manifestRows(source);
  const directStart = source.indexOf("//#region 🧵️DirectStoreLaneRoutes");
  const directEnd = source.indexOf("//#endregion 🧵️DirectStoreLaneRoutes");
  const directStore = directStart >= 0 && directEnd > directStart ? source.slice(directStart, directEnd) : "";
  const retainedContractsStart = retainedSource.indexOf("pub const NOTE_RETAINED_PUBLICATION_CONTRACTS");
  const retainedContractsEnd = retainedContractsStart < 0 ? -1 : retainedSource.indexOf("];", retainedContractsStart);
  const retainedContracts = retainedContractsStart >= 0 && retainedContractsEnd > retainedContractsStart ? retainedSource.slice(retainedContractsStart, retainedContractsEnd + 2) : "";
  const exactRetainedSource = fixture.owner === "FlowPlayApp"
    ? source.includes("impl semio_framework::ToolJobFactory for FlowHostEffectJobFactory")
      && source.includes("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowHostEffectJobFactory")
      && source.includes("type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;")
      && source.includes("registry.register(FlowHostEffectJobFactory::new(&controller))")
      && (!fixture.groups.some((group) => group.status === "Migrated" && group.lanes.some((lane) => lane !== "HostOnly"))
        || source.includes("impl semio_framework::ToolJobFactory for FlowDirectStoreJobFactory")
          && source.includes("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowDirectStoreJobFactory")
          && source.includes("registry.register(FlowDirectStoreJobFactory::new(&controller))")
          && source.includes("fn build_artifact_store_one_item_preparation_factory()")
          && source.includes("fn build_config_store_one_item_preparation_factory()")
          && source.includes("authority.prepare_one_item(edit, std::sync::Arc::new(post))")
          && source.includes("prepared.edit_digest()")
          && !source.includes("flow_store_edit_digest")
          && directStore.includes("scan_cursor")
          && directStore.includes("fn checkpoint(&self")
          && directStore.includes("fn restore(&mut self")
          && directStore.includes("fn close_step(&mut self")
          && directStore.includes("fn terminal_is_empty(&self")
          && !directStore.includes("scene.widgets.iter().any")
          && !directStore.includes("scene.synapses.iter().any")
          && !directStore.includes("preview_off.contains")
          && !directStore.includes("preview_off.retain"))
      && exact(publicationRows(source), fixture.retainedRoutes)
      && fixture.groups.filter((group) => group.status === "Migrated").every((group) => group.routes.every((route) => exactPublication(source, route, group.lanes)))
    : retainedSource.includes("impl semio_framework::ToolJobFactory for NoteCommandJobFactory")
      && retainedSource.includes("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for NoteCommandJobFactory")
      && retainedSource.includes("type Owner = EditorApp<NotePlayApp>;")
      && retainedSource.includes("registry.register(NoteCommandJobFactory::new(&controller))")
      && source.includes("fn build_artifact_store_one_item_preparation_factory()")
      && source.includes("fn build_config_store_one_item_preparation_factory()")
      && exact(publicationRows(retainedContracts), fixture.retainedRoutes)
      && fixture.groups.filter((group) => group.status === "Migrated").every((group) => group.routes.every((route) => exactPublication(retainedContracts, route, group.lanes)));
  return exact(commandRows(source), [...classified, ...fixture.frameworkOwnedRoutes])
    && exact([...pairs.keys()], classified)
    && fixture.groups.every((group) => group.routes.every((route) => pairs.get(route) === group.status))
    && (fixture.retainedRoutes.length === 0
      ? !source.includes("impl semio_framework::ToolJobFactory")
        && !source.includes("impl semio_framework_plugin::ArtifactOwnedToolJobFactory")
        && !source.includes("PUBLICATION_CONTRACTS")
        && publicationRows(source).length === 0
        && !source.includes("registry.register(")
      : source.includes("semio_framework_plugin::bounded_first_step_tool_proofs!") && exactRetainedSource);
}

async function globalOracle(fixture: Fixture, source: string, pluginRoot: string): Promise<boolean> {
  if (fixture.owner === "FlowPlayApp") {
    const duplicate = await Bun.file(resolve(pluginRoot, "🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️component.rs")).text();
    return fixture.globals.length === 0
      && !source.includes("with_process_flow_eval_session")
      && !source.includes("PROCESS_FLOW_EVAL_SESSION")
      && !duplicate.includes("NEXT_DUPLICATE_WIDGET_REQUEST")
      && !duplicate.includes("AtomicU64");
  }
  const schema = await Bun.file(resolve(pluginRoot, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs")).text();
  return fixture.globals.length === 0 && !schema.includes("static NEXT: AtomicU64");
}

class ActionCohortAuditScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const flowRoot = resolve(this.root, "../..");
    const noteRoot = resolve(flowRoot, "../🗒️note");
    const schema = await Bun.file(resolve(flowRoot, "🧪️action-cohort/🔣️schema.json")).json();
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    const fixtures = [
      await Bun.file(resolve(flowRoot, "🧪️action-cohort/🔣️component.json")).json() as Fixture,
      await Bun.file(resolve(noteRoot, "🧪️action-cohort/🔣️component.json")).json() as Fixture,
    ];
    const scope = segments[0] ?? "all";
    if (!["all", "flow", "note"].includes(scope)) throw new Error(`unknown action-cohort scope ${scope}`);
    const selected = fixtures.filter((fixture) => scope === "all" || fixture.owner === (scope === "flow" ? "FlowPlayApp" : "NotePlayApp"));
    for (const fixture of selected) {
      if (!validate(fixture)) throw new Error(`${fixture.owner} failed Ajv: ${JSON.stringify(validate.errors)}`);
      if (!fixtureOracle(fixture)) throw new Error(`${fixture.owner} failed the independent fixture oracle`);
      const pluginRoot = fixture.owner === "FlowPlayApp" ? flowRoot : noteRoot;
      const source = await Bun.file(resolve(pluginRoot, fixture.source)).text();
      const retainedSource = fixture.owner === "NotePlayApp"
        ? await Bun.file(resolve(pluginRoot, "🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️component.rs")).text()
        : "";
      if (!sourceOracle(fixture, source, retainedSource) || !(await globalOracle(fixture, source, pluginRoot))) throw new Error(`${fixture.owner} diverged from source truth`);
      const first = fixture.groups.find((group) => group.status === "BatchOnlyPendingRewrite")!.routes[0]!;
      const hostileActivation = source.replace(
        new RegExp(`(\\.action_interactive_job\\("${first}",\\s*(?:semio_framework_plugin::)?InteractiveJobClassification::)BatchOnlyPendingRewrite`),
        "$1Migrated",
      );
      const forgedContract = `${source}\nArtifactToolPublicationContract { tool_id: "${first}", lanes: &[ArtifactToolPublicationLane::HostOnly] }`;
      const forgedRetainedSource = fixture.owner === "NotePlayApp"
        ? retainedSource.replace("pub const NOTE_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[", `pub const NOTE_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[\n    ArtifactToolPublicationContract { tool_id: "${first}", lanes: &[ArtifactToolPublicationLane::HostOnly] },`)
        : retainedSource;
      if (hostileActivation === source || sourceOracle(fixture, hostileActivation, retainedSource) || sourceOracle(fixture, forgedContract, forgedRetainedSource)) throw new Error(`${fixture.owner} accepted hostile activation or forged publication source`);
    }
    const hostileFixtures: Fixture[] = [
      { ...fixtures[0]!, retainedRoutes: [...fixtures[0]!.retainedRoutes, "addWidget"] },
      { ...fixtures[0]!, routeCount: 36 },
      { ...fixtures[1]!, groups: fixtures[1]!.groups.map((group, index) => index === 3 ? { ...group, lanes: ["HostOnly", "Artifact"] } : group) },
    ];
    if (hostileFixtures.some((fixture) => Boolean(validate(fixture)) && fixtureOracle(fixture))) throw new Error("Flow/Note hostile fixture mutation passed both oracles");
    const total = selected.reduce((sum, fixture) => sum + fixture.routeCount, 0);
    const failclosed = selected.reduce((sum, fixture) => sum + fixture.groups.flatMap((group) => group.routes).length, 0);
    const globals = selected.reduce((sum, fixture) => sum + fixture.globals.length, 0);
    const retained = selected.reduce((sum, fixture) => sum + fixture.retainedRoutes.length, 0);
    console.error(`validated ${scope === "all" ? "Flow/Note" : scope} action cohort; routes=${total}; retained=${retained}; failclosed=${failclosed - retained}; frameworkDelegated=${scope === "all" ? 1 : 0}; globals=${globals}; scanThenMonolith=0; schema=Ajv; oracle=independent`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("action-cohort-audit", ActionCohortAuditScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
