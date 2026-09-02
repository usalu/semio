#!/usr/bin/env bun
/** 🏭️ Process TypeScript package source verifier. */
import { resolve } from "node:path";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

//#region 🔖️RetainedRouteAudit
//#region 🔖️Model
type Lane = "Artifact" | "Config" | "Draft" | "Presence" | "Transient" | "Child" | "HostOnly";
type Group = { status: "Migrated"; execution: "bounded" | "resumable"; lanes: Lane[]; routes: string[] };
type Fixture = {
  schema: string;
  source: string;
  routeCount: number;
  migratedCount: number;
  limits: { rawBytes: number; workItems: number; chunkBytes: number; stepMicros: number; storeBytes: number; documentGrantBytes: number; documentMaximumBytes: number };
  groups: Group[];
  oracleCases: { bytes: number; expectedExtent: number | null }[];
  laws: Record<string, boolean | unknown[]>;
};
//#endregion 🔖️Model

//#region 🔖️Oracles
function exact(left: string[], right: string[]): boolean {
  return new Set(left).size === left.length && new Set(right).size === right.length && JSON.stringify([...left].sort()) === JSON.stringify([...right].sort());
}

function constantRoutes(source: string, symbol: string): string[] {
  const start = source.indexOf(`const ${symbol}`);
  const end = start < 0 ? -1 : source.indexOf("];", start);
  return start < 0 || end < start ? [] : [...source.slice(start, end + 2).matchAll(/"([A-Za-z][A-Za-z0-9]+)"/g)].map((match) => match[1]!);
}

function commandRows(source: string): string[] {
  return [...source.matchAll(/^\s*"([^"]+)" as "[^"]+" =>/gm)].map((match) => match[1]!);
}

function manifestRows(source: string): Map<string, string> {
  return new Map([...source.matchAll(/\.action_interactive_job\("([^"]+)",\s*InteractiveJobClassification::([A-Za-z]+)\)/g)].map((match) => [match[1]!, match[2]!]));
}

function publicationRows(source: string): { route: string; lanes: Lane[] }[] {
  return [...source.matchAll(/ArtifactToolPublicationContract \{ tool_id: "([^"]+)", lanes: &\[([^\]]*)\] \}/g)].map((match) => ({
    route: match[1]!,
    lanes: [...match[2]!.matchAll(/ArtifactToolPublicationLane::([A-Za-z]+)/g)].map((lane) => lane[1]! as Lane),
  }));
}

/** 🧾️ Every `bounded_first_step_tool_proofs!` catalog in the file, not just the first — process3d
 * carries one per execution shape (`Process3dBoundedProofs` and `Process3dResumableProofs`). */
function proofRows(source: string): string[] {
  const rows: string[] = [];
  for (let start = source.indexOf("semio_framework_plugin::bounded_first_step_tool_proofs!"); start >= 0; start = source.indexOf("semio_framework_plugin::bounded_first_step_tool_proofs!", start + 1)) {
    const end = source.indexOf("\n    }", start);
    if (end < start) continue;
    rows.push(...[...source.slice(start, end).matchAll(/^\s*"([^"]+)" =>/gm)].map((match) => match[1]!));
  }
  return rows;
}

function fixtureOracle(fixture: Fixture): boolean {
  const migrated = fixture.groups.flatMap((group) => group.routes);
  const expectedExtent = (bytes: number) => bytes > fixture.limits.rawBytes ? null : Math.max(1, Math.ceil(bytes / fixture.limits.chunkBytes));
  return migrated.length === fixture.migratedCount
    && fixture.migratedCount === fixture.routeCount
    && new Set(migrated).size === fixture.routeCount
    && fixture.groups.every((group) => group.status === "Migrated" && group.routes.length > 0 && group.lanes.length > 0 && new Set(group.lanes).size === group.lanes.length && (!group.lanes.includes("HostOnly") || group.lanes.length === 1))
    && fixture.oracleCases.every((test) => expectedExtent(test.bytes) === test.expectedExtent)
    && Object.entries(fixture.laws).every(([law, value]) => law === "scanThenMonolithRoutes" ? Array.isArray(value) && value.length === 0 : value === true);
}

function sourceOracle(fixture: Fixture, source: string): boolean {
  const migrated = fixture.groups.flatMap((group) => group.routes);
  const bounded = fixture.groups.filter((group) => group.execution === "bounded").flatMap((group) => group.routes);
  const resumable = fixture.groups.filter((group) => group.execution === "resumable").flatMap((group) => group.routes);
  const classifications = manifestRows(source);
  const publications = publicationRows(source);
  const publicationMap = new Map(publications.map((row) => [row.route, row.lanes]));
  return exact(commandRows(source), migrated)
    && source.includes(`const PROCESS3D_RETAINED_RAW_BYTES: usize = ${fixture.limits.rawBytes.toLocaleString("en-US").replace(",", "_")};`)
    && source.includes(`const PROCESS3D_RETAINED_WORK_ITEMS: usize = ${fixture.limits.workItems};`)
    && source.includes(`const PROCESS3D_SCAN_BYTES: usize = ${fixture.limits.chunkBytes};`)
    && source.includes(`const PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES: usize = ${fixture.limits.storeBytes.toLocaleString("en-US").replace(",", "_")};`)
    && source.includes(`const PROCESS3D_DOCUMENT_GRANT_BYTES: usize = ${fixture.limits.documentGrantBytes.toLocaleString("en-US").replace(",", "_")};`)
    && source.includes(`const PROCESS3D_DOCUMENT_MAXIMUM_BYTES: usize = ${fixture.limits.documentMaximumBytes / 1_024} * 1_024;`)
    && source.includes(`ToolExecutionContract::resumable(PROCESS3D_RETAINED_RAW_BYTES, 64, 1, 16_384, ${fixture.limits.stepMicros.toLocaleString("en-US").replace(",", "_")}, 1, 1)`)
    && classifications.size === fixture.routeCount
    && migrated.every((route) => classifications.get(route) === "Migrated")
    && exact(constantRoutes(source, "PROCESS3D_BOUNDED_TOOL_IDS"), bounded)
    && exact(constantRoutes(source, "PROCESS3D_RESUMABLE_TOOL_IDS"), resumable)
    && exact(proofRows(source), migrated)
    && exact(publications.map((row) => row.route), migrated)
    && fixture.groups.every((group) => group.routes.every((route) => exact(publicationMap.get(route) ?? [], group.lanes)))
    && source.includes("struct Process3dResumableCommandWork")
    && source.includes("stage: \"process3d-config-prepare\"")
    && source.includes("fn checkpoint(&self")
    && source.includes("process3d-retained-checkpoint-tool-mismatch")
    && source.includes("fn begin_close(&mut self)")
    && source.includes("fn terminal_is_empty(&self)")
    && source.includes("fn build_config_store_one_item_preparation_factory()")
    && source.includes("fn build_artifact_store_one_item_preparation_factory()")
    && source.includes("struct Process3dArtifactPreparationFactory")
    && source.includes("grant.maximum_bytes < PROCESS3D_DOCUMENT_GRANT_BYTES")
    && source.includes("authority.prepare_one_item(edit")
    && source.includes("request.base_revision != request.authority.base_revision()")
    && source.includes("ToolCancellationPolicy::PerOperation")
    && !source.includes("BatchOnlyPendingRewrite")
    && !source.includes("PROCESS3D_BATCH_ONLY_TOOL_IDS")
    && !source.includes("process3d-command-scan")
    && !source.includes("process3d_payload_chunk");
}
//#endregion 🔖️Oracles

//#region 🔖️Command
class TestScript extends BundleScript {
  async run(): Promise<void> {
    const pluginRoot = resolve(this.root, "../..");
    const fixture = await Bun.file(resolve(pluginRoot, "🗿️artifacts/🧊️process3d/🧪️tests/🔣️retained-route-laws.json")).json() as Fixture;
    const schema = await Bun.file(resolve(pluginRoot, "🗿️artifacts/🧊️process3d/🧪️tests/🔣️retained-route-schema.json")).json();
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    if (!validate(fixture)) throw new Error(`Process3d retained route fixture failed Ajv: ${JSON.stringify(validate.errors)}`);
    if (!fixtureOracle(fixture)) throw new Error("Process3d retained route fixture failed its independent extent/partition oracle");
    const source = await Bun.file(resolve(pluginRoot, fixture.source)).text();
    if (!sourceOracle(fixture, source)) throw new Error("Process3d retained route source diverged from the strict fixture");
    const firstArtifact = fixture.groups.find((group) => group.lanes.includes("Artifact"))!.routes[0]!;
    const hostileDeactivation = source.replace(
      new RegExp(`(\\.action_interactive_job\\("${firstArtifact}",\\s*InteractiveJobClassification::)Migrated`),
      "$1BatchOnlyPendingRewrite",
    );
    const hostileProof = source.replace(new RegExp(`^\\s*"${firstArtifact}" =>.*$`, "m"), "");
    const hostilePublication = `${source}\nArtifactToolPublicationContract { tool_id: "${firstArtifact}", lanes: &[ArtifactToolPublicationLane::HostOnly] }`;
    const hostileGrant = source.replaceAll("grant.maximum_bytes < PROCESS3D_DOCUMENT_GRANT_BYTES", "grant.maximum_bytes < process3d_document_bytes(base)?");
    const hostileFixture = { ...fixture, migratedCount: fixture.migratedCount - 1 };
    if (
      hostileDeactivation === source
      || hostileProof === source
      || hostileGrant === source
      || sourceOracle(fixture, hostileDeactivation)
      || sourceOracle(fixture, hostileProof)
      || sourceOracle(fixture, hostilePublication)
      || sourceOracle(fixture, hostileGrant)
      || fixtureOracle(hostileFixture)
    ) {
      throw new Error("Process3d retained route audit accepted hostile deactivation, missing proof, publication, measured-grant, or count drift");
    }
    console.error(`validated Process3d retained routes; routes=${fixture.routeCount}; migrated=${fixture.migratedCount}; bounded=${constantRoutes(source, "PROCESS3D_BOUNDED_TOOL_IDS").length}; resumable=${constantRoutes(source, "PROCESS3D_RESUMABLE_TOOL_IDS").length}; batchOnly=0; scanThenMonolith=0; schema=Ajv; oracle=independent`);
  }
}
//#endregion 🔖️Command
//#endregion 🔖️RetainedRouteAudit

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
