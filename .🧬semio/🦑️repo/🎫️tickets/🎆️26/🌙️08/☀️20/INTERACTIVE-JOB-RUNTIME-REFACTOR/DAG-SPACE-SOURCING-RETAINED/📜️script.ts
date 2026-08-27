#!/usr/bin/env bun
/** 🧪️ Source-only schema, route, grant, and rejection oracle for the retained cohort. */
import { dirname, resolve } from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";

type Lane = "Config" | "HostOnly";
type App = { name: string; source: string; factory: string; rawBytes: number; retained: { id: string; lane: Lane }[]; batch: string[]; forbidden: string[] };
type Fixture = { schema: string; limits: { configRootBytes: number; configTextBytes: number; metadataTextBytes: number; storeGrantBytes: number; preparationSteps: number; worstCaseEncodedBytes: number }; apps: App[]; laws: Record<string, boolean> };

//#region 🔣️SchemaOracle
const packet = import.meta.dir;
let root = packet;
for (let i = 0; i < 8; i += 1) root = dirname(root);
const fixture = await Bun.file(resolve(packet, "🔣️contract.json")).json() as Fixture;
const schema = await Bun.file(resolve(packet, "🔣️contract.schema.json")).json();
const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
const equal = (left: string[], right: string[]) => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort());
function semanticOracle(value: Fixture): boolean {
  const expected = new Map([["dag", [2, 11, 0]], ["space", [10, 30, 0]], ["sourcing", [5, 9, 1]]]);
  return value.schema === "semio.interactivity.dag-space-sourcing.v1"
    && value.apps.length === 3 && new Set(value.apps.map(app => app.name)).size === 3
    && value.limits.storeGrantBytes === 4096 && value.limits.configRootBytes * 4 + 1024 === 4096
    && value.limits.configTextBytes === 96 && value.limits.metadataTextBytes === 64
    && 6 * (2 * value.limits.configTextBytes + 3 * value.limits.metadataTextBytes) + 1536 === value.limits.worstCaseEncodedBytes
    && value.limits.worstCaseEncodedBytes <= value.limits.storeGrantBytes
    && value.limits.preparationSteps === 2 && Object.values(value.laws).every(Boolean)
    && value.apps.every(app => {
      const ids = [...app.retained.map(route => route.id), ...app.batch, ...app.forbidden];
      const count = expected.get(app.name);
      return !!count && count.every((value, index) => value === [app.retained.length, app.batch.length, app.forbidden.length][index])
        && new Set(ids).size === ids.length
        && app.retained.every(route => route.lane === (app.name === "space" && ["setActiveExample", "importSpacePack", "goHome", "navigateVirtualFileSystemNode"].includes(route.id) ? "HostOnly" : "Config"));
    });
}
if (!validate(fixture) || !semanticOracle(fixture)) throw new Error("cohort fixture rejected: " + JSON.stringify(validate.errors));
let rejected = 0;
for (const mutate of [
  (value: any) => { value.extra = true; },
  (value: any) => { value.limits.storeGrantBytes = 4097; },
  (value: any) => { value.limits.configTextBytes = 97; },
  (value: any) => { value.limits.preparationSteps = 1; },
  (value: any) => { value.apps[0].retained[0].lane = "Artifact"; },
  (value: any) => { value.apps[1].retained[0].lane = "HostOnly"; },
  (value: any) => { value.apps[2].retained[0].extra = true; },
  (value: any) => { value.apps[0].batch.push(value.apps[0].batch[0]); },
]) {
  const mutant = structuredClone(fixture); mutate(mutant);
  if (validate(mutant) && semanticOracle(mutant)) throw new Error("hostile fixture accepted");
  rejected += 1;
}
//#endregion 🔣️SchemaOracle

//#region 🧵️SourceOracle
function sourceOracle(app: App, source: string): boolean {
  const commands = [...source.matchAll(/"([^"]+)"\s+as\s+"[^"]+"\s*=>/g)].map(match => match[1]!);
  const classified = (id: string, status: string) => new RegExp("\\.action_interactive_job\\(\"" + id + "\",\\s*(?:semio_framework_plugin::)?InteractiveJobClassification::" + status + "\\)").test(source);
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)];
  const proofs = source.match(/bounded_first_step_tool_proofs!\s*\{[\s\S]*?tools:\s*\{([\s\S]*?)\n\s*\}/)?.[1] ?? "";
  const proofIds = [...proofs.matchAll(/"([^"]+)"\s*=>/g)].map(match => match[1]!);
  const registered = app.retained.every(route => contracts.some(match => match[1] === route.id && match[2]!.trim().endsWith("ArtifactToolPublicationLane::" + route.lane)) && classified(route.id, "Migrated"));
  return equal(commands, [...app.retained.map(route => route.id), ...app.batch, ...app.forbidden])
    && equal(proofIds, app.retained.map(route => route.id))
    && contracts.length === app.retained.length && registered
    && app.batch.every(id => classified(id, "BatchOnlyPendingRewrite"))
    && app.forbidden.every(id => classified(id, "ForbiddenFromUi"))
    && source.includes("struct " + app.factory)
    && source.includes("build_config_store_one_item_preparation_factory")
    && source.includes("authority.prepare_one_item")
    && source.includes("work_items: 2")
    && source.includes("ArtifactStoreOneItemPreparationStep::Progress")
    && source.includes("ArtifactStoreOneItemCheckpoint { cursor: 1")
    && source.includes("ArtifactStoreOneItemCheckpoint { cursor: 2")
    && source.includes("request.operation != request.authority.operation()")
    && source.includes("request.generation != request.authority.generation()")
    && source.includes("request.base_revision != request.authority.base_revision()")
    && source.includes("grant.maximum_bytes < self.retained_bytes")
    && source.includes("fn cancel(&mut self)")
    && source.includes("fn begin_close(&mut self)")
    && source.includes("base.return_to_registry()")
    && source.includes("fn terminal_is_empty(&self)")
    && !source.includes("SourcingCurateResumableCommandWork")
    && !/thread_local!|OnceLock|LazyLock/.test(source);
}
let checked = 0;
for (const app of fixture.apps) {
  const source = (await Bun.file(resolve(root, app.source)).text()).split("//#region 🧪️Testkit")[0]!;
  if (!sourceOracle(app, source)) throw new Error(app.name + " source oracle rejected");
  for (const token of ["authority.prepare_one_item", "request.operation != request.authority.operation()", "request.generation != request.authority.generation()", "request.base_revision != request.authority.base_revision()", "grant.maximum_bytes < self.retained_bytes", "fn cancel(&mut self)", "base.return_to_registry()"]) {
    if (sourceOracle(app, source.replaceAll(token, "REJECTED_TOKEN"))) throw new Error(app.name + " accepted source mutation " + token);
    rejected += 1;
  }
  checked += app.retained.length + app.batch.length + app.forbidden.length;
}
//#endregion 🧵️SourceOracle

//#region 🧪️ThirdPartySpaceOracle
const space = fixture.apps.find(app => app.name === "space")!;
const spaceRoot = dirname(resolve(root, space.source));
const spaceFixture = await Bun.file(resolve(spaceRoot, "🧪️fixtures/🎯️retained-command-limits.json")).json();
const spaceSchema = await Bun.file(resolve(spaceRoot, "🧪️fixtures/🎯️retained-command-limits.schema.json")).json();
const validateSpace = new Ajv2020({ strict: true, allErrors: true }).compile(spaceSchema);
if (!validateSpace(spaceFixture)) throw new Error("Space Ajv2020 fixture rejected: " + JSON.stringify(validateSpace.errors));
if (!equal(spaceFixture.routes.filter((route: any) => route.status === "migrated").map((route: any) => route.id), space.retained.map(route => route.id))) throw new Error("Space independent fixture disagrees");
const wrongSpace = structuredClone(spaceFixture);
wrongSpace.publicationContracts[0].lanes = ["artifact"];
if (validateSpace(wrongSpace)) throw new Error("Space wrong lane accepted");
rejected += 1;
const controlText = "\u0000".repeat(fixture.limits.configTextBytes);
if (JSON.stringify(controlText).length - 2 !== 6 * fixture.limits.configTextBytes) throw new Error("JSON escaping oracle disagrees");
if (fixture.limits.worstCaseEncodedBytes > fixture.limits.storeGrantBytes) throw new Error("encoding bound exceeds scheduler grant");
//#endregion 🧪️ThirdPartySpaceOracle

console.log("[DEBUG] cohort routes=" + checked + " retained=17 config=13 hostOnly=4 batchOnly=50 forbidden=1 sourceMutations=21 hostileRejections=" + rejected + " grantBytes=4096");
