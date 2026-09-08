#!/usr/bin/env bun
/** 🔬️ Faithful replication of `puzzle-js publication-authority-audit`'s `ownerOracle` for
 * `Puzzle3dPlayApp`, with the two locale/terminology anchors a peer removed from the editor at
 * 2026-09-08 23:25 isolated so wave T's own clauses can be judged on their own. */
const root = "/Users/ueli/Documents/semio";
const puzzleRoot = `${root}/✏️s/🔌️plugins/🧩️puzzle`;

type Group = { status: string; lanes: string[]; routes: string[]; blocker?: string };
type Owner = { owner: string; source: string; groups: Group[] };
const fixture = (await Bun.file(`${puzzleRoot}/🔏️publication-authority/🔣️.json`).json()) as { schema: string; closePageBytes: number; owners: Owner[]; laws: Record<string, boolean> };

const quotedValues = (source: string): string[] => [...source.matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
const exactArray = (left: string[], right: string[]): boolean => JSON.stringify([...left].sort()) === JSON.stringify([...right].sort()) && new Set(left).size === left.length && new Set(right).size === right.length;
const retainedIds = (source: string): string[] => quotedValues(source.match(/PUZZLE3D_RETAINED_TOOL_IDS: &\[&str\] = &\[([\s\S]*?)\];/)![1]!);
const manifestPairs = (source: string): Map<string, string> =>
  new Map([...source.matchAll(/\.action_interactive_job\((?:"([^"]+)"|set_fill_count::STEP_ACTION_ID),\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((match) => [match[1] ?? "setFillCountStep", match[2]!]));
const publicationContracts = (source: string): Map<string, string[]> =>
  new Map([...source.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((match) => [
    match[1]!,
    [...match[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|Child|HostOnly)/g)].map((lane) => lane[1]!),
  ]));
const exactContracts = (actual: Map<string, string[]>, groups: Group[]): boolean => {
  const laneOf = (lane: string) => lane.split("-").map((part) => `${part[0]!.toUpperCase()}${part.slice(1)}`).join("");
  const expected = new Map(groups.flatMap((group) => group.routes.map((route) => [route, group.lanes.map(laneOf)] as const)));
  return exactArray([...actual.keys()], [...expected.keys()]) && [...expected].every(([route, lanes]) => exactArray(actual.get(route) ?? [], lanes));
};

const owner = fixture.owners.find((entry) => entry.owner === "Puzzle3dPlayApp")!;
const source = await Bun.file(`${puzzleRoot}/${owner.source}`).text();
const production = source.split("//#region 🧪️Testkit")[0]!;
const migrated = owner.groups.filter((group) => group.status === "migrated").flatMap((group) => group.routes);
const pairs = manifestPairs(production);
const factoryBlock = production.split(/impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory for Puzzle3dRetainedCommandJobFactory\b/)[1]!.split("//#endregion 🧵️RetainedCommands")[0]!;
const proofBlock = production.split("semio_framework_plugin::bounded_first_step_tool_proofs!")[1]!.split("fn register_tool_job_factories")[0]!;

const localeAnchors = [String.raw`matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")`, String.raw`matches!(value.as_str(), "native" | "reuse")`];
const otherAnchors = [
  "struct Puzzle3dConfigStorePreparationFactory",
  "impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory",
  "impl store::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation",
  "fn build_config_store_one_item_preparation_factory()",
  "Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))",
  "request.operation != request.authority.operation()",
  "request.generation != request.authority.generation()",
  "request.base_revision != request.authority.base_revision()",
  "ArtifactStoreOneItemPreparationStep::Progress",
  "ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1",
  "ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2",
  "fn cancel(&mut self)",
  "fn begin_close(&mut self)",
  "base.return_to_registry()",
  "fn terminal_is_empty(&self)",
  "struct Puzzle3dArtifactStorePreparationFactory",
  "impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparationFactory",
  "impl store::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparation",
  "fn build_artifact_store_one_item_preparation_factory()",
  "Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))",
  "registry.register(Puzzle3dRetainedCommandJobFactory::new(&controller))",
];

const results = {
  declared: pairs.size,
  migratedInSource: [...pairs.values()].filter((status) => status === "Migrated").length,
  batchOnlyInSource: [...pairs].filter(([, status]) => status !== "Migrated").map(([route]) => route),
  fixtureMigrated: migrated.length,
  manifestBijection: exactArray([...pairs.keys()], owner.groups.flatMap((group) => group.routes)),
  statusesAgree: owner.groups.every((group) => group.routes.every((route) => pairs.get(route) === (group.status === "migrated" ? "Migrated" : "BatchOnlyPendingRewrite"))),
  retainedExact: exactArray(retainedIds(production), migrated),
  proofsExact: exactArray(quotedValues(proofBlock.match(/tools:\s*\[([^\]]*)\]/)![1]!), migrated),
  contractsExact: exactContracts(publicationContracts(factoryBlock), owner.groups.filter((group) => group.status === "migrated")),
  hostOnlyExclusive: [...publicationContracts(factoryBlock)].every(([, lanes]) => !lanes.includes("HostOnly") || lanes.length === 1),
  transformLanes: publicationContracts(factoryBlock).get("transformBegin")?.concat(publicationContracts(factoryBlock).get("transformEnd") ?? []),
  missingOtherAnchors: otherAnchors.filter((anchor) => !production.includes(anchor)),
  missingLocaleAnchors: localeAnchors.filter((anchor) => !production.includes(anchor)),
  negativeStoreAnchors: ["build_draft_store_one_item_preparation_factory", "build_presence_store_one_item_preparation_factory", "build_transient_store_one_item_preparation_factory"].filter((anchor) => production.includes(anchor)),
  ownerTypeAlias: /type Owner = (?:[A-Za-z_][A-Za-z0-9_]*::)*EditorApp<Puzzle3dPlayApp>;/.test(production),
  factoryNamedInProofs: proofBlock.includes(`factory: "Puzzle3dRetainedCommandJobFactory"`) && proofBlock.includes("factory_type: Puzzle3dRetainedCommandJobFactory"),
  noBatchOnlyGroupLeft: owner.groups.every((group) => group.status === "migrated"),
};
console.log(JSON.stringify(results, null, 2));
