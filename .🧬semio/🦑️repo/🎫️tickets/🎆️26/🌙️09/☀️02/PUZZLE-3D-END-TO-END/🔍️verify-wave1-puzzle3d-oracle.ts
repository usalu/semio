const puzzleRoot = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle";
const fixture = await Bun.file(`${puzzleRoot}/🧪️publication-authority/🔣️.json`).json();
const owner = fixture.owners.find((o: any) => o.owner === "Puzzle3dPlayApp");
const source = await Bun.file(`${puzzleRoot}/${owner.source}`).text();

function quotedValues(s: string): string[] { return [...s.matchAll(/"([^"]+)"/g)].map((m) => m[1]!); }
function retainedIds(source: string, ownerName: string): string[] {
  const dimension = ownerName.slice(6, 8).toUpperCase();
  const match = source.match(new RegExp(`PUZZLE${dimension}_RETAINED_TOOL_IDS: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`));
  if (!match) { console.log("NO retainedIds MATCH"); return []; }
  return quotedValues(match[1]!);
}
function manifestPairs(source: string): Map<string, string> {
  return new Map([...source.matchAll(/\.action_interactive_job\((?:"([^"]+)"|set_fill_count::STEP_ACTION_ID),\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((m) => [m[1] ?? "setFillCountStep", m[2]!]));
}
function exactArray(l: string[], r: string[]) { return JSON.stringify([...l].sort()) === JSON.stringify([...r].sort()) && new Set(l).size === l.length && new Set(r).size === r.length; }
function publicationContracts(source: string): Map<string, string[]> {
  return new Map([...source.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((m) => [
    m[1]!,
    [...m[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|Child|HostOnly)/g)].map((l) => l[1]!),
  ]));
}
function exactContracts(actual: Map<string, string[]>, groups: any[]): boolean {
  const expected = new Map(groups.flatMap((g) => g.routes.map((r: string) => [r, g.lanes] as const)));
  return exactArray([...actual.keys()], [...expected.keys()]) && [...expected].every(([r, lanes]) => exactArray(actual.get(r as string) ?? [], lanes as string[]));
}

const production = source.split("//#region 🧪️Testkit")[0]!;
const pairs = manifestPairs(production);
const appGroups = owner.groups;
const appRoutes = appGroups.flatMap((g: any) => g.routes);
const migrated = appGroups.filter((g: any) => g.status === "Migrated").flatMap((g: any) => g.routes);
const expectedPairs = new Map(appGroups.flatMap((g: any) => g.routes.map((r: string) => [r, g.status])));

console.log("appRoutes.length:", appRoutes.length, "pairs.size:", pairs.size);
console.log("exactArray(pairs.keys, appRoutes):", exactArray([...pairs.keys()], appRoutes));
for (const r of appRoutes) { if (pairs.get(r) !== expectedPairs.get(r)) console.log("MISMATCH", r, "actual=", pairs.get(r), "expected=", expectedPairs.get(r)); }
console.log("retainedIds:", retainedIds(production, owner.owner));
console.log("migrated (fixture):", migrated);
console.log("exactArray(retainedIds, migrated):", exactArray(retainedIds(production, owner.owner), migrated));

const factory = "Puzzle3dRetainedCommandJobFactory";
const factoryBlock = production.split(`impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ${factory}`)[1]?.split("//#endregion 🧵️RetainedCommands")[0] ?? "";
const contracts = publicationContracts(factoryBlock);
const proofBlock = production.split("semio_framework_plugin::bounded_first_step_tool_proofs!")[1]?.split("fn register_tool_job_factories")[0] ?? "";
const proofIds = quotedValues(proofBlock.match(/tools:\s*\[([^\]]*)\]/)?.[1] ?? "");
console.log("proofIds:", proofIds);
console.log("exactArray(proofIds, migrated):", exactArray(proofIds, migrated));
console.log("contracts map:", [...contracts.entries()]);
console.log("exactContracts:", exactContracts(contracts, appGroups.filter((g: any) => g.status === "Migrated")));

const exactFactory = production.includes(`impl semio_framework::ToolJobFactory for ${factory}`)
  && production.includes(`impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ${factory}`)
  && proofBlock.includes(`factory: "${factory}"`)
  && proofBlock.includes(`factory_type: ${factory}`)
  && production.includes(`type Owner = semio_framework_plugin::EditorApp<${owner.owner}>;`)
  && production.includes(`registry.register(${factory}::new(&controller))`)
  && (owner.owner === "Puzzle5dPlayApp" || owner.owner === "Puzzle3dPlayApp" || !production.includes("build_artifact_store_one_item_preparation_factory"))
  && !production.includes("build_draft_store_one_item_preparation_factory")
  && !production.includes("build_presence_store_one_item_preparation_factory")
  && !production.includes("build_transient_store_one_item_preparation_factory");
console.log("exactFactory:", exactFactory);

const puzzle3dChecks = [
  ["struct Puzzle3dConfigStorePreparationFactory", production.includes("struct Puzzle3dConfigStorePreparationFactory")],
  ["impl ...Config prep factory", production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory")],
  ["impl ...Config prep", production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation")],
  ["build_config_store_one_item_preparation_factory", production.includes("fn build_config_store_one_item_preparation_factory()")],
  ["Some(...Puzzle3dConfigStorePreparationFactory)", production.includes("Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))")],
  ["en-US de-DE match", production.includes('matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")')],
  ["native reuse match", production.includes('matches!(value.as_str(), "native" | "reuse")')],
  ["operation !=", production.includes("request.operation != request.authority.operation()")],
  ["generation !=", production.includes("request.generation != request.authority.generation()")],
  ["base_revision !=", production.includes("request.base_revision != request.authority.base_revision()")],
  ["Progress", production.includes("ArtifactStoreOneItemPreparationStep::Progress")],
  ["cursor:1", production.includes("ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1")],
  ["cursor:2", production.includes("ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2")],
  ["fn cancel", production.includes("fn cancel(&mut self)")],
  ["fn begin_close", production.includes("fn begin_close(&mut self)")],
  ["return_to_registry", production.includes("base.return_to_registry()")],
  ["fn terminal_is_empty", production.includes("fn terminal_is_empty(&self)")],
  ["struct Puzzle3dArtifactStorePreparationFactory", production.includes("struct Puzzle3dArtifactStorePreparationFactory")],
  ["impl ...Artifact prep factory", production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparationFactory")],
  ["impl ...Artifact prep", production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparation")],
  ["build_artifact_store_one_item_preparation_factory", production.includes("fn build_artifact_store_one_item_preparation_factory()")],
  ["Some(...Puzzle3dArtifactStorePreparationFactory)", production.includes("Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))")],
];
for (const [name, ok] of puzzle3dChecks) console.log(ok ? "OK  " : "FAIL", name);
console.log("all puzzle3d checks pass:", puzzle3dChecks.every(([, ok]) => ok));
