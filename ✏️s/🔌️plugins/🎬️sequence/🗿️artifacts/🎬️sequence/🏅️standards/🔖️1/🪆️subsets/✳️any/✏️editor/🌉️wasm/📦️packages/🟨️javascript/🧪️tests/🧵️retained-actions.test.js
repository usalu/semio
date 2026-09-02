import Ajv2020 from "ajv/dist/2020.js";
import dagre from "dagre";
import deepEqual from "fast-deep-equal";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

//#region 🧬️LanguageNeutralContract

const editorRoot = new URL("../../../", import.meta.url);
const schema = JSON.parse(readFileSync(fileURLToPath(new URL("🧬️schema/🧵️retained-actions.json", editorRoot)), "utf8"));
const fixture = JSON.parse(readFileSync(fileURLToPath(new URL("🧪️fixtures/🧵️retained-actions.json", editorRoot)), "utf8"));
const source = readFileSync(fileURLToPath(new URL("../🦀️.rs", editorRoot)), "utf8");
const validate = new Ajv2020({ strict: true }).compile(schema);

if (!validate(fixture)) throw new Error(`Sequence retained action fixture rejected: ${JSON.stringify(validate.errors)}`);
const ids = fixture.routes.map(({ id }) => id);
if (new Set(ids).size !== 17) throw new Error("Sequence retained action fixture must own 17 unique live routes");

for (const route of fixture.routes) {
  const declaration = `.action_interactive_job("${route.id}", semio_framework_plugin::InteractiveJobClassification::${route.classification})`;
  if (!source.includes(declaration)) throw new Error(`Sequence source classification drifted for ${route.id}`);
}

const migrated = fixture.routes.filter(({ classification }) => classification === "Migrated");
const pending = fixture.routes.filter(({ classification }) => classification === "BatchOnlyPendingRewrite");
if (migrated.length !== 17 || pending.length !== 0) throw new Error("Sequence retained route disposition counts drifted");
for (const route of migrated) {
  if (!source.includes(`tool_id: "${route.id}", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::${route.publicationLane}]`)) throw new Error(`Sequence publication lane drifted for ${route.id}`);
  const persistent = route.contract.maximumUnits === 66049;
  const maximumUnits = persistent ? "66_049" : "2";
  const outputBytes = route.publicationLane === "Artifact" || route.id === "run" ? "65_536" : "4_096";
  const maximumStepMicros = persistent ? "7_500" : "2_000";
  if (!source.includes(`"${route.id}" => semio_framework::ToolExecutionContract::resumable(4_096, ${maximumUnits}, 1, ${outputBytes}, ${maximumStepMicros}, 1, 1)`)) throw new Error(`Sequence bounded proof drifted for ${route.id}`);
}

for (const law of hostileLaws(fixture)) {
  if (validate(law.value)) throw new Error(`Sequence hostile schema law was accepted: ${law.id}`);
}

for (const token of ["ArtifactCommandWorkStep::Progress", "ArtifactCommandWorkStep::Replay", "fn checkpoint", "fn restore", "fn begin_close", "fn close_step", "fn terminal_is_empty"]) {
  if (!source.includes(token)) throw new Error(`Sequence retained state-machine law is missing ${token}`);
}
for (const token of ["fn build_artifact_store_one_item_preparation_factory()", "SequenceArtifactStorePreparationFactory", "fn build_config_store_one_item_preparation_factory()", "SequenceConfigStorePreparationFactory", "canonical_base_revision: request.canonical_base_revision"]) {
  if (!source.includes(token)) throw new Error(`Sequence retained Store/freshness authority is missing ${token}`);
}
for (const token of ["SequenceReorganizeState", "self.edge += 1", "SequenceNodeGraphStage::FixtureSteps", "self.fixture_steps.pop_front()", "SequenceNodeGraphStage::DeleteSelectionDiscover", "self.delete_scan += 1", "self.operation += 1", "SequenceRunOrderStage", "frame.order.advance", "sequence-run-retire-frame", "sequence-persistent-publication-lane", "maximum_items == 0 || maximum_bytes == 0"]) {
  if (!source.includes(token)) throw new Error(`Sequence persistent cursor law is missing ${token}`);
}
const reorganizeOracle = dagreLayoutOracle(fixture.persistentOracle.reorganize);
if (!deepEqual(reorganizeOracle, fixture.persistentOracle.reorganize.expected)) throw new Error(`Sequence reorganize fixture drifted from dagre: ${JSON.stringify(reorganizeOracle)}`);
const nodeGraphOracle = nodeGraphMutationOracle(fixture.persistentOracle.nodeGraphEdit);
if (!deepEqual(nodeGraphOracle, fixture.persistentOracle.nodeGraphEdit.expectedMutations)) throw new Error(`Sequence node-graph fixture drifted: ${JSON.stringify(nodeGraphOracle)}`);
const runOracle = graphOrderOracle(fixture.persistentOracle.run);
if (!deepEqual(runOracle, fixture.persistentOracle.run.expectedOrder)) throw new Error(`Sequence run order drifted from graphlib: ${JSON.stringify(runOracle)}`);

console.log(JSON.stringify({ oracle: "ajv-2020+dagre-0.8.5+graphlib", routes: ids.length, migrated: migrated.length, pending: pending.length, hostileLaws: 7, persistentScenarios: 3, maximumStepMicros: fixture.runtimeLaws.maximumStepMicros, locales: fixture.locales, accessibility: "bounded-progress-cancel-close", customization: Object.keys(fixture.customization) }));

//#endregion 🧬️LanguageNeutralContract

//#region 🧭️ThirdPartyOracles

function graph(scenario) {
  const value = new dagre.graphlib.Graph({ directed: true }).setGraph({ rankdir: "LR", ranksep: 280, nodesep: 160, marginx: 0, marginy: 0 }).setDefaultEdgeLabel(() => ({}));
  for (const id of scenario.nodes) value.setNode(id, { width: 0, height: 0 });
  for (const [from, to] of scenario.edges) value.setEdge(from, to);
  return value;
}

function dagreLayoutOracle(scenario) {
  const value = graph(scenario);
  dagre.layout(value);
  return scenario.nodes.map((id) => ({ id, x: value.node(id).x, y: value.node(id).y }));
}

function graphOrderOracle(scenario) {
  return dagre.graphlib.alg.topsort(graph(scenario));
}

function nodeGraphMutationOracle(scenario) {
  const deleted = scenario.baseSteps.filter((step) => !scenario.targetSteps.some((target) => target.id === step.id)).map((step) => step.id);
  const recreated = scenario.targetSteps.filter((step) => scenario.baseSteps.some((base) => base.id === step.id && base.kind !== step.kind)).map((step) => step.id);
  const mutations = deleted.map((id) => `DeleteStep:${id}`);
  for (const step of scenario.targetSteps) {
    if (recreated.includes(step.id)) mutations.push(`DeleteStep:${step.id}`, `CreateStep:${step.id}`);
    else if (!scenario.baseSteps.some((base) => base.id === step.id)) mutations.push(`CreateStep:${step.id}`);
  }
  for (const edge of scenario.baseEdges) {
    if (![...deleted, ...recreated].includes(edge.from) && ![...deleted, ...recreated].includes(edge.to) && !scenario.targetEdges.some((target) => target.id === edge.id)) mutations.push(`DisconnectSteps:${edge.id}`);
  }
  for (const edge of scenario.targetEdges) {
    if (![...scenario.baseEdges.map(({ id }) => id)].includes(edge.id) || recreated.includes(edge.from) || recreated.includes(edge.to)) mutations.push(`ConnectSteps:${edge.id}`);
  }
  return mutations;
}

//#endregion 🧭️ThirdPartyOracles

//#region ☣️HostileLaws

function hostileLaws(valid) {
  const unknown = structuredClone(valid);
  unknown.routes[0].identityCache = true;
  const falseMigrated = structuredClone(valid);
  falseMigrated.routes[0].classification = "MigratedWithoutCursor";
  const falseBatch = structuredClone(valid);
  falseBatch.routes[0].classification = "BatchOnlyPendingRewrite";
  const oversized = structuredClone(valid);
  oversized.routes.find(({ classification }) => classification === "Migrated").contract.maximumRawBytes = 4097;
  const excessiveUnits = structuredClone(valid);
  excessiveUnits.routes.find(({ contract }) => contract.maximumUnits === 66049).contract.maximumUnits = 66050;
  const monolingual = structuredClone(valid);
  monolingual.locales = ["en"];
  const slowPoll = structuredClone(valid);
  slowPoll.runtimeLaws.maximumStepMicros = 8001;
  return [
    { id: "unknown-property", value: unknown },
    { id: "false-migrated", value: falseMigrated },
    { id: "false-batch-contract", value: falseBatch },
    { id: "oversized-wire", value: oversized },
    { id: "excessive-units", value: excessiveUnits },
    { id: "monolingual", value: monolingual },
    { id: "slow-poll", value: slowPoll },
  ];
}

//#endregion ☣️HostileLaws
