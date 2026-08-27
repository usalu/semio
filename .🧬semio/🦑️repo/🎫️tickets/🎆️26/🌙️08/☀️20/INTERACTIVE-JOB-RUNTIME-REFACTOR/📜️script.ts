#!/usr/bin/env bun
/** 🧪️ Strict source/Ajv oracle for the six small retained-route closures owned by this ticket packet. */
import { dirname, resolve } from "node:path";
import Ajv from "ajv";

type Lane = "Artifact" | "Config";
type Route = { app: string; source: string; executorSource: string; route: string; lane: Lane; mutation: string; factory: string };
type Fixture = { schema: string; storePreparationSteps: number; routes: Route[]; laws: Record<string, boolean> };

const ticketRoot = import.meta.dir;
let repoRoot = ticketRoot;
for (let depth = 0; depth < 7; depth += 1) repoRoot = dirname(repoRoot);

function exactArray(left: string[], right: string[]): boolean {
  return JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;
}

function fixtureOracle(fixture: Fixture): boolean {
  const expected = [
    "Gis3dPlayApp:setCamera:Config:SetCamera:Gis3dConfigStorePreparationFactory",
    "Gis3dPlayApp:setExaggeration:Artifact:ChangeExaggeration:Gis3dArtifactStorePreparationFactory",
    "Gis3dPlayApp:setLocale:Config:SetLocale:Gis3dConfigStorePreparationFactory",
    "PlaygroundEditor:changeSchema:Artifact:ChangeSchema:PlaygroundStorePreparationFactory",
    "Puzzle3dPlayApp:setLocale:Config:SetLocale:Puzzle3dConfigStorePreparationFactory",
    "Puzzle3dPlayApp:setTerminology:Config:SetTerminology:Puzzle3dConfigStorePreparationFactory",
  ];
  const actual = fixture.routes.map(({ app, route, lane, mutation, factory }) => `${app}:${route}:${lane}:${mutation}:${factory}`);
  return fixture.schema === "semio.interactivity.small-retained-routes.v1"
    && fixture.storePreparationSteps === 2
    && Object.values(fixture.laws).every(Boolean)
    && exactArray(actual, expected)
    && fixture.routes.every(({ source, executorSource }) => source.length > 0 && executorSource.length > 0);
}

function contractLane(source: string, route: string): string[] | undefined {
  const contracts = [...source.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)];
  const contract = contracts.find((match) => match[1] === route);
  return contract ? [...contract[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|Child|HostOnly)/g)].map((match) => match[1]!) : undefined;
}

function routeOracle(route: Route, fullSource: string, executorSource: string): boolean {
  const source = fullSource.split("//#region 🧪️Testkit")[0]!;
  const hook = route.lane === "Artifact" ? "build_artifact_store_one_item_preparation_factory" : "build_config_store_one_item_preparation_factory";
  const classification = new RegExp(`\\.action_interactive_job\\(\"${route.route}\",\\s*(?:semio_framework_plugin::)?InteractiveJobClassification::Migrated\\)`);
  const retained = new RegExp(`RETAINED_TOOL_IDS[^;]*\"${route.route}\"`, "s");
  const proof = new RegExp(`bounded_first_step_tool_proofs![\\s\\S]*?tools:[\\s\\S]*?\"${route.route}\"[\\s\\S]*?\\n\\s*}`);
  const executorOwned = executorSource.includes("pub struct ArtifactRetainedCommandJob")
    ? executorSource.includes("raw_page_cursor: usize") && executorSource.includes("checkpoint_page_cursor: usize")
    : executorSource.includes("pub struct RetainedPuzzleCommandJob") && executorSource.includes("work_cursor: usize") && executorSource.includes("raw_page_cursor: usize");
  const exactEnvelope = route.route === "setExaggeration"
    ? source.includes("payload.new_exaggeration.is_finite()")
    : route.route === "setCamera"
      ? source.includes("camera.is_object()")
      : route.app === "Gis3dPlayApp" && route.route === "setLocale"
        ? source.includes('matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")')
        : route.app === "Puzzle3dPlayApp" && route.route === "setLocale"
          ? source.includes('matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")')
          : route.route === "setTerminology"
            ? source.includes('matches!(value.as_str(), "native" | "reuse")')
            : source.includes("PlaygroundMutation::ChangeSchema(payload)");
  return exactArray(contractLane(source, route.route) ?? [], [route.lane])
    && classification.test(source)
    && retained.test(source)
    && proof.test(source)
    && source.includes(`struct ${route.factory}`)
    && source.includes(hook)
    && source.includes(`Arc::new(${route.factory})`)
    && source.includes(route.mutation)
    && exactEnvelope
    && source.includes("authority.prepare_one_item")
    && source.includes("work_items: 2")
    && source.includes("ArtifactStoreOneItemPreparationStep::Progress")
    && source.includes("ArtifactStoreOneItemCheckpoint { cursor: 1")
    && source.includes("ArtifactStoreOneItemCheckpoint { cursor: 2")
    && source.includes("request.operation != request.authority.operation()")
    && source.includes("request.generation != request.authority.generation()")
    && source.includes("request.base_revision != request.authority.base_revision()")
    && source.includes("fn cancel(&mut self)")
    && source.includes("fn begin_close(&mut self)")
    && source.includes("base.return_to_registry()")
    && source.includes("fn terminal_is_empty(&self)")
    && executorOwned
    && executorSource.includes("cx.is_cancelled()")
    && executorSource.includes("fn begin_close(&mut self)")
    && executorSource.includes("fn terminal_is_empty(&self)");
}

const fixturePath = resolve(ticketRoot, "🔣️codex-small-retained-routes-v1.json");
const schemaPath = resolve(ticketRoot, "🔣️codex-small-retained-routes-v1.schema.json");
const fixture = await Bun.file(fixturePath).json() as Fixture;
const schema = await Bun.file(schemaPath).json();
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
if (!validate(fixture)) throw new Error(`small retained-route fixture failed Ajv: ${JSON.stringify(validate.errors)}`);
if (!fixtureOracle(fixture)) throw new Error("small retained-route fixture failed its independent semantic oracle");

for (const route of fixture.routes) {
  const source = await Bun.file(resolve(repoRoot, route.source)).text();
  const executorSource = await Bun.file(resolve(repoRoot, route.executorSource)).text();
  if (!routeOracle(route, source, executorSource)) throw new Error(`${route.app}:${route.route} diverged from the strict source oracle`);
  const missingContract = source.replace(`ArtifactToolPublicationContract { tool_id: "${route.route}", lanes: &[ArtifactToolPublicationLane::${route.lane}] }`, "");
  const missingFactory = source.replace(`Arc::new(${route.factory})`, "Arc::new(RejectedFactory)");
  const staleGeneration = source.replace(/\n\s*\|\| request\.generation != request\.authority\.generation\(\)/, "");
  const noProgress = source.replaceAll("ArtifactStoreOneItemPreparationStep::Progress", "ArtifactStoreOneItemPreparationStep::Prepared");
  const unboundedStep = source.replaceAll("work_items: 2", "work_items: 1");
  if ([missingContract, missingFactory, staleGeneration, noProgress, unboundedStep].some((hostile) => hostile === source)) {
    throw new Error(`${route.app}:${route.route} hostile source mutation did not apply`);
  }
  if (routeOracle(route, missingContract, executorSource)
    || routeOracle(route, missingFactory, executorSource)
    || routeOracle(route, staleGeneration, executorSource)
    || routeOracle(route, noProgress, executorSource)
    || routeOracle(route, unboundedStep, executorSource)) {
    throw new Error(`${route.app}:${route.route} accepted a hostile contract, preparation, freshness, progress, or bound mutation`);
  }
}

const hostileFixtures: Fixture[] = [
  { ...fixture, storePreparationSteps: 1 },
  { ...fixture, routes: fixture.routes.slice(1) },
  { ...fixture, routes: fixture.routes.map((route, index) => index === 0 ? { ...route, lane: "Artifact" } : route) },
  { ...fixture, laws: { ...fixture.laws, cancellation: false } },
];
if (hostileFixtures.some((hostile) => Boolean(validate(hostile)) && fixtureOracle(hostile))) {
  throw new Error("small retained-route fixture accepted a hostile schema/oracle mutation");
}

console.error(`validated ${fixture.routes.length} small retained routes with Ajv, an independent semantic oracle, and hostile mutations`);
