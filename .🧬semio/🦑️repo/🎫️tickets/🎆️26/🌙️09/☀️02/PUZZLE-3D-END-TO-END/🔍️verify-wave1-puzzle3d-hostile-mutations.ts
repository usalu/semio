const puzzleRoot = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle";
const fixture = await Bun.file(`${puzzleRoot}/🧪️publication-authority/🔣️.json`).json();
const owner = fixture.owners.find((o: any) => o.owner === "Puzzle3dPlayApp");
const source = await Bun.file(`${puzzleRoot}/${owner.source}`).text();

function quotedValues(s: string): string[] { return [...s.matchAll(/"([^"]+)"/g)].map((m) => m[1]!); }
function retainedIds(source: string, ownerName: string): string[] {
  const dimension = ownerName.slice(6, 8).toUpperCase();
  const match = source.match(new RegExp(`PUZZLE${dimension}_RETAINED_TOOL_IDS: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`));
  if (!match) return [];
  return quotedValues(match[1]!);
}
function manifestPairs(source: string): Map<string, string> {
  return new Map([...source.matchAll(/\.action_interactive_job\((?:"([^"]+)"|set_fill_count::STEP_ACTION_ID),\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((m) => [m[1] ?? "setFillCountStep", m[2]!]));
}
function exactArray(l: string[], r: string[]) { return JSON.stringify([...l].sort()) === JSON.stringify([...r].sort()) && new Set(l).size === l.length && new Set(r).size === r.length; }
function publicationContracts(source: string): Map<string, string[]> {
  return new Map([...source.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((m) => [
    m[1]!, [...m[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|Child|HostOnly)/g)].map((l) => l[1]!),
  ]));
}
function exactContracts(actual: Map<string, string[]>, groups: any[]): boolean {
  const expected = new Map(groups.flatMap((g) => g.routes.map((r: string) => [r, g.lanes] as const)));
  return exactArray([...actual.keys()], [...expected.keys()]) && [...expected].every(([r, lanes]) => exactArray(actual.get(r as string) ?? [], lanes as string[]));
}
function ownerOracle(owner: any, source: string): boolean {
  const production = source.split("//#region 🧪️Testkit")[0]!;
  const pairs = manifestPairs(production);
  const appGroups = owner.groups;
  const appRoutes = appGroups.flatMap((g: any) => g.routes);
  const migrated = appGroups.filter((g: any) => g.status === "Migrated").flatMap((g: any) => g.routes);
  const expectedPairs = new Map(appGroups.flatMap((g: any) => g.routes.map((r: string) => [r, g.status])));
  if (!exactArray([...pairs.keys()], appRoutes)) return false;
  if (!appRoutes.every((r: string) => pairs.get(r) === expectedPairs.get(r))) return false;
  if (!exactArray(retainedIds(production, owner.owner), migrated)) return false;
  const factory = "Puzzle3dRetainedCommandJobFactory";
  const factoryBlock = production.split(`impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ${factory}`)[1]?.split("//#endregion 🧵️RetainedCommands")[0] ?? "";
  const contracts = publicationContracts(factoryBlock);
  const proofBlock = production.split("semio_framework_plugin::bounded_first_step_tool_proofs!")[1]?.split("fn register_tool_job_factories")[0] ?? "";
  const proofIds = quotedValues(proofBlock.match(/tools:\s*\[([^\]]*)\]/)?.[1] ?? "");
  const exactFactory = production.includes(`impl semio_framework::ToolJobFactory for ${factory}`)
    && production.includes(`impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ${factory}`)
    && proofBlock.includes(`factory: "${factory}"`)
    && proofBlock.includes(`factory_type: ${factory}`)
    && production.includes(`type Owner = semio_framework_plugin::EditorApp<${owner.owner}>;`)
    && production.includes(`registry.register(${factory}::new(&controller))`)
    && (true)
    && !production.includes("build_draft_store_one_item_preparation_factory")
    && !production.includes("build_presence_store_one_item_preparation_factory")
    && !production.includes("build_transient_store_one_item_preparation_factory");
  if (!exactFactory || !exactContracts(contracts, appGroups.filter((g: any) => g.status === "Migrated")) || !exactArray(proofIds, migrated)) return false;
  return production.includes("struct Puzzle3dConfigStorePreparationFactory")
    && production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparationFactory")
    && production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle3dConfig, Puzzle3dConfigMutation> for Puzzle3dConfigStorePreparation")
    && production.includes("fn build_config_store_one_item_preparation_factory()")
    && production.includes("Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))")
    && production.includes('matches!(value.as_str(), "en" | "en-US" | "de" | "de-DE")')
    && production.includes('matches!(value.as_str(), "native" | "reuse")')
    && production.includes("request.operation != request.authority.operation()")
    && production.includes("request.generation != request.authority.generation()")
    && production.includes("request.base_revision != request.authority.base_revision()")
    && production.includes("ArtifactStoreOneItemPreparationStep::Progress")
    && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1")
    && production.includes("ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2")
    && production.includes("fn cancel(&mut self)")
    && production.includes("fn begin_close(&mut self)")
    && production.includes("base.return_to_registry()")
    && production.includes("fn terminal_is_empty(&self)")
    && production.includes("struct Puzzle3dArtifactStorePreparationFactory")
    && production.includes("impl store::ArtifactStoreOneItemPreparationFactory<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparationFactory")
    && production.includes("impl store::ArtifactStoreOneItemPreparation<Puzzle3dPlaySnapshot, Puzzle3dMutation> for Puzzle3dArtifactStorePreparation")
    && production.includes("fn build_artifact_store_one_item_preparation_factory()")
    && production.includes("Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))");
}

console.log("baseline ownerOracle:", ownerOracle(owner, source));

const missingPreparation = source.replace("Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))", "None");
const missingArtifactPreparation = source.replace("Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))", "None");
const widenedTerminology = source.replace('matches!(value.as_str(), "native" | "reuse")', 'matches!(value.as_str(), "native" | "reuse" | "other")');
const staleAuthority = source.replaceAll("            || request.generation != request.authority.generation()\n", "");
const missingProgress = source.replaceAll("ArtifactStoreOneItemPreparationStep::Progress", "ArtifactStoreOneItemPreparationStep::Prepared");

console.log("missingPreparation (Config) rejected:", !ownerOracle(owner, missingPreparation));
console.log("missingArtifactPreparation rejected:", !ownerOracle(owner, missingArtifactPreparation));
console.log("widenedTerminology rejected:", !ownerOracle(owner, widenedTerminology));
console.log("staleAuthority rejected:", !ownerOracle(owner, staleAuthority));
console.log("missingProgress rejected:", !ownerOracle(owner, missingProgress));

const blocked = owner.groups.find((g: any) => g.status === "BatchOnlyPendingRewrite")?.routes[0];
console.log("first BatchOnly route to hostile-activate:", blocked);
const hostile = source.replace(
  new RegExp(`(\\.action_interactive_job\\("${blocked}",\\s*(?:semio_framework_plugin::)?InteractiveJobClassification::)BatchOnlyPendingRewrite`),
  "$1Migrated",
);
console.log("hostile activation applied (source changed):", hostile !== source);
console.log("hostile activation rejected (oracle still false):", !ownerOracle(owner, hostile));

const missingContract = source.replace(/\s*ArtifactToolPublicationContract \{ tool_id: "(?:openAddObjectDialog|setLocale|canvasPointerDown)", lanes: &\[ArtifactToolPublicationLane::(?:HostOnly|Config)\] \},/, "");
console.log("missingContract applied:", missingContract !== source);
console.log("missingContract rejected:", !ownerOracle(owner, missingContract));
