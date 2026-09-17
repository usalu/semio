/** 🔏️ Slice 2F — names WHICH clause of `publication-authority-audit`'s `ownerOracle` diverges for
 * one owner, which the audit itself only reports as "diverged from the fixture".
 *
 *   cd /Users/ueli/Documents/semio && bun '.../🟦️2F-publication-diff.ts' Puzzle2dPlayApp
 */
const ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle";
const reserved2d = new Set(["copy", "cut", "paste", "import-media"]);
const reserved5d = new Set(["setFixtureJson"]);

const quoted = (source: string): string[] => [...source.matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
const variant = (value: string): string =>
  value.split("-").map((word, at) => (at === 0 ? word : word[0]!.toUpperCase() + word.slice(1))).join("").replace(/^./, (c) => c.toUpperCase());
const sorted = (values: string[]): string => JSON.stringify([...values].sort());
const missing = (left: string[], right: string[]): { onlyLeft: string[]; onlyRight: string[] } => ({
  onlyLeft: left.filter((value) => !right.includes(value)),
  onlyRight: right.filter((value) => !left.includes(value)),
});

const owner = Bun.argv[2] ?? "Puzzle2dPlayApp";
const fixture = await Bun.file(`${ROOT}/🧫️fixtures/🔏️publication-authority/🔣️.json`).json();
const row = fixture.owners.find((entry: { owner: string }) => entry.owner === owner)!;
const source = (await Bun.file(`${ROOT}/${row.source}`).text()).split("//#region 🧪️UnitTests")[0]!;

const groups = row.groups.map((group: { routes: string[] }) => ({
  ...group,
  routes: group.routes.filter((route: string) => (owner !== "Puzzle5dPlayApp" || !reserved5d.has(route)) && (owner !== "Puzzle2dPlayApp" || !reserved2d.has(route))),
}));
const appRoutes = groups.flatMap((group: { routes: string[] }) => group.routes);
const migrated = groups.filter((group: { status: string }) => group.status === "migrated").flatMap((group: { routes: string[] }) => group.routes);

const pairs = new Map([...source.matchAll(/\.action_interactive_job\("([^"]+)",\s*(?:semio_framework_plugin::)?InteractiveJobClassification::(Migrated|BatchOnlyPendingRewrite)\)/g)].map((m) => [m[1]!, m[2]!]));
const dimension = owner.slice(6, 8).toUpperCase();
const retained = quoted(source.match(new RegExp(`PUZZLE${dimension}_RETAINED_TOOL_IDS: &\\[&str\\] = &\\[([\\s\\S]*?)\\];`))![1]!);
const factory = `${owner.slice(0, 8)}RetainedCommandJobFactory`;
const factoryBlock = source.split(new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*ArtifactOwnedToolJobFactory for ${factory}\\b`))[1]?.split("//#endregion 🧵️RetainedCommands")[0] ?? "";
const contracts = new Map([...factoryBlock.matchAll(/ArtifactToolPublicationContract\s*\{\s*tool_id:\s*"([^"]+)",\s*lanes:\s*&\[([^\]]*)\]/g)].map((m) => [
  m[1]!,
  [...m[2]!.matchAll(/ArtifactToolPublicationLane::(Artifact|Config|Draft|Presence|Transient|WindowConfig|WindowTransient|Child|Interaction|HostOnly)/g)].map((lane) => lane[1]!),
]));
const proofBlock = source.split("semio_framework_plugin::bounded_first_step_tool_proofs!")[1]?.split("fn register_tool_job_factories")[0] ?? "";
const proofIds = quoted(proofBlock.match(/tools:\s*\[([^\]]*)\]/)?.[1] ?? "");

const say = (clause: string, ok: boolean, detail?: unknown): void => console.log(`${ok ? "✅" : "❌"} ${clause}${ok ? "" : `\n     ${JSON.stringify(detail)}`}`);

say("manifest .action_interactive_job rows == fixture routes", sorted([...pairs.keys()]) === sorted(appRoutes), missing([...pairs.keys()], appRoutes));
const wrongVariant = appRoutes.filter((route: string) => pairs.get(route) !== (groups.find((group: { routes: string[] }) => group.routes.includes(route))!.status === "migrated" ? "Migrated" : "BatchOnlyPendingRewrite"));
say("every route carries its fixture's classification", wrongVariant.length === 0, wrongVariant);
say("RETAINED_TOOL_IDS == migrated routes", sorted(retained) === sorted(migrated), missing(retained, migrated));
say("bounded_first_step_tool_proofs tools == migrated routes", sorted(proofIds) === sorted(migrated), missing(proofIds, migrated));
say("PUBLICATION_CONTRACTS keys == migrated routes", sorted([...contracts.keys()]) === sorted(migrated), missing([...contracts.keys()], migrated));
const laneMismatch = groups
  .filter((group: { status: string }) => group.status === "migrated")
  .flatMap((group: { routes: string[]; lanes: string[] }) => group.routes.map((route: string) => ({ route, want: group.lanes.map(variant), got: contracts.get(route) ?? [] })))
  .filter((entry: { want: string[]; got: string[] }) => sorted(entry.want) !== sorted(entry.got));
say("every contract's lanes == its fixture group's lanes", laneMismatch.length === 0, laneMismatch);

const implementsFor = (trait: string, type: string): boolean => new RegExp(`impl (?:[A-Za-z_][A-Za-z0-9_]*::)*${trait} for ${type}\\b`).test(source);
say("impl ToolJobFactory for factory", implementsFor("ToolJobFactory", factory));
say("impl ArtifactOwnedToolJobFactory for factory", implementsFor("ArtifactOwnedToolJobFactory", factory));
say(`proofs factory: "${factory}"`, proofBlock.includes(`factory: "${factory}"`));
say(`proofs factory_type: ${factory}`, proofBlock.includes(`factory_type: ${factory}`));
say("type Owner = EditorApp<owner>", new RegExp(`type Owner = (?:[A-Za-z_][A-Za-z0-9_]*::)*EditorApp<${owner}>;`).test(source));
say("registry.register(factory::new(&controller))", source.includes(owner === "Puzzle5dPlayApp" ? `registry.register(${factory}::new(&controller_id))` : `registry.register(${factory}::new(&controller))`));
say("build_artifact_store_one_item_preparation_factory", source.includes("fn build_artifact_store_one_item_preparation_factory()"));
say("build_config_store_one_item_preparation_factory", source.includes("fn build_config_store_one_item_preparation_factory()"));
say("no draft store preparation", !source.includes("build_draft_store_one_item_preparation_factory"));
say("no presence store preparation", !source.includes("build_presence_store_one_item_preparation_factory"));
say("no transient store preparation", !source.includes("build_transient_store_one_item_preparation_factory"));
say("locale is OS owned", !/"setLocale"|"setTerminology"|ConfigMutation::Set(?:Locale|Terminology)/.test(source));

const dupes = (label: string, values: string[]): void => {
  const seen = new Set<string>();
  const repeated = values.filter((value) => (seen.has(value) ? true : (seen.add(value), false)));
  say(`${label} has no duplicates`, repeated.length === 0, repeated);
};
dupes("fixture routes", appRoutes);
dupes("manifest rows", [...pairs.keys()]);
dupes("RETAINED_TOOL_IDS", retained);
dupes("proof tools", proofIds);
dupes("PUBLICATION_CONTRACTS", [...contracts.keys()]);
console.log(`counts: fixture=${appRoutes.length} manifest=${pairs.size} retained=${retained.length} proofs=${proofIds.length} contracts=${contracts.size} migrated=${migrated.length}`);

if (owner === "Puzzle2dPlayApp") {
  for (const clause of [
    "struct Puzzle2dConfigStorePreparationFactory",
    "impl store::ArtifactStoreOneItemPreparationFactory<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparationFactory",
    "impl store::ArtifactStoreOneItemPreparation<Puzzle2dConfig, Puzzle2dConfigMutation> for Puzzle2dConfigStorePreparation",
    "Some(std::sync::Arc::new(Puzzle2dConfigStorePreparationFactory))",
    "struct Puzzle2dArtifactStorePreparationFactory",
    "impl store::ArtifactStoreOneItemPreparationFactory<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparationFactory",
    "impl store::ArtifactStoreOneItemPreparation<Puzzle2dPlaySnapshot, Puzzle2dMutation> for Puzzle2dArtifactStorePreparation",
    "Some(std::sync::Arc::new(Puzzle2dArtifactStorePreparationFactory))",
    "PUZZLE2D_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536",
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
    "fn puzzle2d_dispatch_emit(",
    "puzzle2d_dispatch_emit(command, &snapshot.0, config, &window_config, &window_transient, window_kind, self.view_state.as_ref(), puzzle2d_active_utility(self.view_state.as_ref()), &selection, None)?",
    "puzzle2d_dispatch_emit(command, &doc.snapshot.0, config, &window_config, &window_transient, window_kind, view_state, puzzle2d_active_utility(view_state), interaction.selection(PUZZLE2D_INTERACTION_DOMAIN), doc.operation_optional().cloned())",
    "PUZZLE2D_SELECTION_BATCH_LIMIT: usize = 1_024",
    "(addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))",
  ]) say(`2d clause: ${clause.slice(0, 80)}`, source.includes(clause));
}
