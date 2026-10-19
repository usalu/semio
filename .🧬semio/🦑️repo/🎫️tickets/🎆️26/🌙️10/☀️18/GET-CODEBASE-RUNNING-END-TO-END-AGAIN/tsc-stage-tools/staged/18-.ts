import { pathEmojiStatuteFindings } from "@semio-tech/framework/identity/path";
import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";
import { cargoWorkspaceDeclaresMemberV1 } from "../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { basename, dirname, join, posix } from "node:path";
import { type CanonicalWgpuPackageCatalog, type SemanticPackageAdapter, type SemanticPackageBrowserProfile, type SemanticPackageProjectionCase, type SemanticPackageProjectionCatalog, type SemanticPackageProjectionMapping, exactOwnerPath, parseCurrentJcoPackageDestination, semanticPackageAuthoredFragmentProblem, semanticPackageAuthoredFragmentReferences, semanticPackageJoinedPathBindingRecords, semanticPackageJoinedPathReferenceAuthority } from "../📦️package-projection-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { pathBudgetCodes } from "../📏️path-budget/🟦️.ts";
import { classifyPackageSource, classifyPackageSourceDisposition, requiredPackageGlueGrammar } from "../🏷️package-source-classification/🟦️.ts";
import { tomlTableBody } from "../🪧️owner-markers/🟦️.ts";
import { type SemanticProjectionAuthorityNode, projectionByteCompare } from "../🧱️path-projection-contracts/🟦️.ts";
import { exactOwnerRegularFile } from "../✅️owned-file-approvals/🟦️.ts";

/** 📏️ Validates explicit browser input authority without discovering or reading unrelated modules. */
export function parseSemanticPackageBrowserProfile(input: unknown, genericEmojiIdentities: readonly string[]): SemanticPackageBrowserProfile {
  const exact = (value: unknown, keys: readonly string[]): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join("\0") === [...keys].sort().join("\0");
  if (!exact(input, ["schemaVersion", "kind", "inlineTestDefine", "ownerPath", "entries", "workspaceImports", "sourceModulePaths"]) || input.schemaVersion !== 1 || input.kind !== "wgpu-browser-esm-v1" || input.inlineTestDefine !== "undefined" || !exactOwnerPath(input.ownerPath)) throw new Error("Invalid WGPU browser profile");
  const expected = ["frame-worker", "browser-boot", "renderer-boot"].map(id => ({ id, inclusion: "ignored" }));
  if (!Array.isArray(input.entries) || input.entries.length !== expected.length || input.entries.some((entry, index) => !exact(entry, ["id", "sourceRelativePath", "outputRelativePath", "inclusion"]) || entry.id !== expected[index]!.id || entry.inclusion !== expected[index]!.inclusion || !exactOwnerPath(entry.sourceRelativePath) || !exactOwnerPath(entry.outputRelativePath))) throw new Error("WGPU browser entry authority drift");
  const directories = input.entries.map((entry) => ({ path: dirname(entry.sourceRelativePath).replaceAll("\\", "/"), nodeKind: "directory" as const }));
  if (pathEmojiStatuteFindings(directories, genericEmojiIdentities).length || input.entries.some((entry, index) => directories[index]!.path.includes("/") || leadingEmojiIdentity(directories[index]!.path).rest !== entry.id || entry.sourceRelativePath !== directories[index]!.path + "/🟦️.ts" || entry.outputRelativePath !== directories[index]!.path + "/🤖️generated/🟨️.js")) throw new Error("WGPU browser entry requires one explicit semantic source and output owner");
  if (!Array.isArray(input.sourceModulePaths) || input.sourceModulePaths.length < expected.length || input.sourceModulePaths.some((path, index, paths) => !exactOwnerPath(path) || !/\.(?:tsx?|m?js|json)$/u.test(path) || index > 0 && projectionByteCompare(paths[index - 1], path) >= 0)) throw new Error("WGPU browser module paths must be exact, nonopaque, unique and byte ordered");
  if (input.entries.some((entry) => !(input.sourceModulePaths as string[]).includes(input.ownerPath + "/" + entry.sourceRelativePath))) throw new Error("WGPU browser entry is not an exact declared current module");
  const specifiers = input.workspaceImports !== null && typeof input.workspaceImports === "object" && !Array.isArray(input.workspaceImports) ? Object.keys(input.workspaceImports) : [];
  if (["@semio-tech/framework", "@semio-tech/framework-os", "@semio-tech/framework-replication"].some((root) => !specifiers.includes(root)) || specifiers.some((specifier, index) => !/^@semio-tech\/[a-z0-9-]+(?:\/[a-z0-9][a-z0-9.-]*)*$/u.test(specifier) || index > 0 && projectionByteCompare(specifiers[index - 1]!, specifier) >= 0)) throw new Error("WGPU browser workspace import identities drifted");
  for (const binding of Object.values(input.workspaceImports as Record<string, unknown>)) if (!exact(binding, ["manifestPath", "entryPath"]) || !exactOwnerPath(binding.manifestPath) || basename(binding.manifestPath) !== "package.json" || !exactOwnerPath(binding.entryPath) || !input.sourceModulePaths.includes(binding.entryPath)) throw new Error("WGPU browser workspace import binding drifted");
  return input as unknown as SemanticPackageBrowserProfile;
}

/** 🔐️ Validates one digest-bound current catalog without weakening historical source checks. */
export function parseCanonicalWgpuPackageCatalog(bytes: string, digest: string, profile: SemanticPackageBrowserProfile, taxonomy: Taxonomy): CanonicalWgpuPackageCatalog {
  if (createHash("sha256").update(bytes).digest("hex") !== digest) throw new Error("Current WGPU package catalog digest drift");
  const row = JSON.parse(bytes) as CanonicalWgpuPackageCatalog;
  const exact = (value: unknown, keys: readonly string[]): boolean => value !== null && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join("\0") === [...keys].sort().join("\0");
  if (!exact(row, ["$schema", "schemaVersion", "kind", "ownerPath", "packageRelativePath", "nodePackageRelativePath", "identity", "entryPaths", "artifacts"]) || row.$schema !== "https://json.schemas.assets.semio-tech.com/os/renderer/component.json#/$defs/RendererPackageCatalogV1" || row.schemaVersion !== 1 || row.kind !== "canonical-wgpu-package" || row.ownerPath !== profile.ownerPath || row.packageRelativePath !== "📦️packages/🦀️rust" || row.nodePackageRelativePath !== "📦️packages/🟦️typescript") throw new Error("Current WGPU package catalog identity drift");
  if (!exact(row.identity, ["cargoPackageName", "nodePackageName", "projectName"]) || row.identity.cargoPackageName !== "semio-framework-os-renderer-wgpu" || row.identity.nodePackageName !== "@semio-tech/framework-renderer-wgpu" || row.identity.projectName !== row.identity.nodePackageName || !exact(row.entryPaths, ["cargoLibrary", "cargoBinary", "cargoBuild", "nodeLibrary"]) || Object.values(row.entryPaths).some((path) => !exactOwnerPath(path)) || row.entryPaths.cargoBuild !== "build.rs") throw new Error("Current WGPU package manifest authority drift");
  const ids = ["build-adapter", "binary-adapter", "typescript-adapter", "renderer-registration"];
  if (!Array.isArray(row.artifacts) || row.artifacts.length !== ids.length || row.artifacts.some((artifact, index) => !exact(artifact, ["id", "relativePath", "targetRelativePath", "language", "role", "content"]) || artifact.id !== ids[index] || !exactOwnerPath(artifact.relativePath) || artifact.targetRelativePath !== null && !exactOwnerPath(artifact.targetRelativePath) || !["rust", "typescript"].includes(artifact.language) || !["declaration", "implementation"].includes(artifact.role) || typeof artifact.content !== "string" || classifyPackageSource(artifact.content, requiredPackageGlueGrammar(taxonomy, artifact.language)).role !== artifact.role) || new Set(row.artifacts.map((artifact) => artifact.relativePath)).size !== ids.length) throw new Error("Current WGPU package artifact authority drift");
  for (const artifact of row.artifacts) {
    if (artifact.id === "renderer-registration") {
      if (artifact.relativePath !== "🧊️renderer/📇️registry/🦀️.rs" || artifact.targetRelativePath !== null || artifact.role !== "implementation") throw new Error("Current WGPU registration ownership drift");
    } else {
      const packageRelativePath = artifact.language === "typescript" ? row.nodePackageRelativePath : row.packageRelativePath;
      if (!artifact.relativePath.startsWith(packageRelativePath + "/") || !artifact.targetRelativePath || artifact.role !== "declaration") throw new Error("Current WGPU adapter ownership drift");
      const relativeTarget = posix.relative(posix.dirname(artifact.relativePath), artifact.targetRelativePath);
      if (!artifact.content.includes(JSON.stringify(relativeTarget))) throw new Error("Current WGPU adapter target drift");
    }
  }
  return row;
}

/** 🔐️ Loads the exact package catalog without following any parent or leaf symlink. */
export function semanticPackageProjectionCatalog(repoRoot: string, taxonomy: Taxonomy): SemanticPackageProjectionCatalog | null {
  const contract = taxonomy.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
  if (!contract) throw new Error("Nested Cargo package projection contract required");
  const state = exactOwnerRegularFile(repoRoot, contract.authorityCatalogPath);
  if (state === "absent") return null;
  if (state !== "file") throw new Error("Nested Cargo catalog must be a no-follow regular file");
  const bytes = readFileSync(join(repoRoot, contract.authorityCatalogPath));
  if (createHash("sha256").update(bytes).digest("hex") !== contract.authorityCatalogSha256) throw new Error("Nested Cargo catalog digest drift");
  const catalog = JSON.parse(bytes.toString("utf8")) as SemanticPackageProjectionCatalog;
  if (catalog.schemaVersion !== 1 || catalog.contractKind !== contract.contractKind || JSON.stringify(catalog.packages.map((row) => row.id)) !== JSON.stringify(contract.packageIds)) throw new Error("Nested Cargo catalog identity drift");
  const sources = new Set<string>(), destinations = new Set<string>();
  for (const [index, row] of catalog.packages.entries()) {
    if (!exactOwnerPath(row.sourceRoot) || !exactOwnerPath(row.destinationRoot) || !exactOwnerPath(row.semanticOwnerRoot) || row.destinationRoot !== `${row.semanticOwnerRoot}/📦️packages/🦀️rust` || row.mappings.length !== contract.sourceLeafCounts[index]) throw new Error("Nested Cargo catalog boundary drift");
    for (const mapping of row.mappings) {
      const key = mapping.destinationPath.normalize("NFC").toLocaleLowerCase("und").replaceAll("\ufe0f", "");
      if (!exactOwnerPath(mapping.sourcePath) || !exactOwnerPath(mapping.destinationPath) || !mapping.sourcePath.startsWith(row.sourceRoot + "/") || !mapping.destinationPath.startsWith(row.semanticOwnerRoot + "/") || !/^[0-9a-f]{64}$/u.test(mapping.sourceHash) || !Number.isSafeInteger(mapping.sourceSize) || mapping.sourceSize < 0 || sources.has(mapping.sourcePath) || destinations.has(key) || pathBudgetCodes(mapping.destinationPath, taxonomy.collisionPolicy).length > 0) throw new Error("Nested Cargo catalog leaf authority drift");
      sources.add(mapping.sourcePath);
      destinations.add(key);
    }
    if (!Array.isArray(row.joinedPathBindings) || row.joinedPathBindings.length !== contract.joinedPathBindingCounts[index] || new Set(row.joinedPathBindings.map((binding) => binding.id)).size !== row.joinedPathBindings.length) throw new Error("Nested Cargo joined-path binding census drift");
    for (const binding of row.joinedPathBindings) semanticPackageJoinedPathBindingRecords(binding, row);
    if (row.authoredSourceFragments?.length !== contract.authoredFragmentCounts[index] || semanticPackageAuthoredFragmentProblem(row)) throw new Error("Nested Cargo authored fragment census or ownership drift");
    if (!Array.isArray(row.generatedSourceRetirements) || row.generatedSourceRetirements.length !== contract.generatedSourceRetirementCounts[index] || new Set(row.generatedSourceRetirements.map((retirement) => retirement.sourcePath)).size !== row.generatedSourceRetirements.length || row.mappings.filter((mapping) => mapping.disposition === "generated").length !== row.generatedSourceRetirements.length) throw new Error("Nested Cargo generated source retirement census drift");
    for (const retirement of row.generatedSourceRetirements) {
      const mapping = row.mappings.find((mapping) => mapping.sourcePath === retirement.sourcePath);
      if (Object.keys(retirement).sort().join("\0") !== "destinationPath\0generatorContractId\0sourceMode\0sourcePath" || row.id !== "wgpu-renderer" || !mapping || mapping.disposition !== "generated" || mapping.destinationPath !== retirement.destinationPath || retirement.generatorContractId !== "wgpu-frame-worker" || retirement.sourceMode !== 0o644 || taxonomy.generatorContracts[retirement.generatorContractId]?.ownership !== "owned") throw new Error("Nested Cargo generated source retirement authority drift");
    }
    if (!Array.isArray(row.derivedLeaves) || row.derivedLeaves.length !== (row.id === "wgpu-renderer" ? contract.derivedLeafCount : 0)) throw new Error("Nested Cargo derived leaf census drift");
    for (const leaf of row.derivedLeaves) {
      const mapping = row.mappings.find((mapping) => mapping.sourcePath === leaf.originSourcePath);
      const key = leaf.path.normalize("NFC").toLocaleLowerCase("und").replaceAll("\ufe0f", "");
      if (leaf.id !== "wgpu-renderer-registration" || leaf.path !== row.semanticOwnerRoot + "/🧊️renderer/📇️registry/🦀️.rs" || !exactOwnerPath(leaf.path) || !mapping || leaf.originSourceHash !== mapping.sourceHash || pathBudgetCodes(leaf.path, taxonomy.collisionPolicy).length > 0 || destinations.has(key) || !taxonomy.semanticDirectoryKinds.registry || classifyPackageSource(leaf.content, requiredPackageGlueGrammar(taxonomy, "rust")).role !== leaf.expectedRole) throw new Error("Nested Cargo derived registration authority drift");
      destinations.add(key);
    }
    for (const adapter of row.adapters) {
      if (!exactOwnerPath(adapter.path) || !adapter.path.startsWith(row.destinationRoot + "/") || pathBudgetCodes(adapter.path, taxonomy.collisionPolicy).length > 0 || adapter.targetPaths.some((path) => !row.mappings.some((mapping) => mapping.destinationPath === path) && !row.derivedLeaves.some((leaf) => leaf.path === path)) || classifyPackageSource(adapter.content, requiredPackageGlueGrammar(taxonomy, adapter.language)).role !== adapter.expectedRole) throw new Error("Nested Cargo adapter authority drift");
    }
    if (!Array.isArray(row.sourceSplices) || row.sourceSplices.length !== (row.id === "wgpu-renderer" ? 2 : 0)) throw new Error("Nested Cargo source splice census drift");
    for (const splice of row.sourceSplices) if (!row.mappings.some((mapping) => mapping.sourcePath === splice.sourcePath && mapping.destinationPath === splice.destinationPath) || !splice.oldValue || !splice.newValue) throw new Error("Nested Cargo source splice ownership drift");
    if (row.id === "wgpu-renderer") {
      const [library, renderer] = row.sourceSplices, mapping = row.mappings.find((mapping) => mapping.sourcePath === library!.sourcePath)!;
      const registration = library!.oldValue.match(/#\[cfg\(target_arch = "wasm32"\)\][\s\S]*?(?=#\[cfg\(not\(target_os = "wasi"\)\)\]\ninclude!\("🦀️.rs"\);)/u)?.[0];
      const split = registration?.indexOf('#[cfg(not(target_os = "wasi"))]') ?? -1;
      if (library!.id !== "library-adapter-replacement" || renderer!.id !== "renderer-registration-merge" || createHash("sha256").update(library!.oldValue).digest("hex") !== mapping.sourceHash || Buffer.byteLength(library!.oldValue) !== mapping.sourceSize || library!.newValue !== row.adapters.find((adapter) => adapter.id === "wgpu-library")?.content || !registration || split < 0 || renderer!.destinationPath !== row.semanticOwnerRoot + "/🧊️renderer/🦀️.rs" || renderer!.newValue !== registration.slice(0, split) + renderer!.oldValue || row.derivedLeaves[0]!.content !== "//! 📇️ Renderer macro registration.\n\n" + registration.slice(split)) throw new Error("Nested Cargo renderer registration merge authority drift");
    }
  }
  if (catalog.packages.flatMap((row) => row.mappings).filter((row) => row.sourceRole !== null).length !== contract.purityCount || catalog.packages.flatMap((row) => row.adapters).length !== contract.adapterCount) throw new Error("Nested Cargo purity census drift");
  const generator = taxonomy.generatorContracts["jco-package-adapter"], current = parseCurrentJcoPackageDestination(generator?.currentPackageDestination);
  if (!generator || generator.projectionActivation || [current.cargoManifestPath, current.cargoLockPath, current.componentPath, current.witPath].some((path) => !generator.inputPatterns.includes(path)) || JSON.stringify(generator.outputRoots) !== JSON.stringify([{ path: current.adapterPath, inclusion: "tracked" }])) throw new Error("Current JCO generator destination authority drift");
  return catalog;
}

function nestedCargoField(content: string, section: string, key: string): unknown {
  const escaped = (value: string): string => value.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
  content = content.replace(/"(?:\\.|[^"\\])*"|#[^\r\n]*/gu, (token) => token.startsWith("#") ? "" : token);
  if ([...content.matchAll(new RegExp(`^\\s*\\[${escaped(section)}\\]\\s*(?:#.*)?$`, "gmu"))].length !== 1) return undefined;
  const body = tomlTableBody(content, section);
  if (body === undefined) return undefined;
  const values = [...body.matchAll(new RegExp(`^\\s*${escaped(key)}\\s*=\\s*("[^"\\r\\n]*"|false|true|\\[[\\s\\S]*?\\])\\s*(?:#.*)?$`, "gmu"))];
  if (values.length !== 1) return undefined;
  try { return JSON.parse(values[0]![1]!.replace(/,\s*\]/gu, "]")); } catch { return undefined; }
}

/** 🪺️ Derives optional ignored leaves only from exact package entry and generator output authority. */
export function semanticPackageIgnoredGeneratedOutputPaths(row: SemanticPackageProjectionCase, taxonomy: Taxonomy): readonly string[] {
  const generator = taxonomy.generatorContracts["wgpu-frame-worker"];
  if (row.id !== "wgpu-renderer" || generator?.packageGeneration?.kind !== "wgpu-package-artifacts") return [];
  const profile = parseSemanticPackageBrowserProfile(generator.packageGeneration.browserProfile, taxonomy.pathEmojiPolicy.genericEmojiIdentities);
  if (profile.ownerPath !== row.semanticOwnerRoot) return [];
  return profile.entries.filter((entry) => entry.inclusion === "ignored").flatMap((entry) => {
    const path = profile.ownerPath + "/" + entry.outputRelativePath;
    return generator.outputRoots.some((root) => root.path === path && root.inclusion === "ignored") ? [path] : [];
  });
}

/** 🧭️ Validates complete source or canonical package facts; no physical reads occur here. */
export function semanticPackageProjectionAuthority(
  facts: Readonly<{ packageId: string; nodes: readonly SemanticProjectionAuthorityNode[]; layout?: "source" | "destination"; occupiedPaths?: readonly string[]; cargoWorkspaceContent?: string; nodeWorkspaceContent?: string }>,
  catalog: SemanticPackageProjectionCatalog,
  taxonomy: Taxonomy,
): Readonly<{ packageId: string; mappings: readonly SemanticPackageProjectionMapping[]; adapters: readonly SemanticPackageAdapter[]; sourceDigest: string; problems: readonly string[] }> {
  const row = catalog.packages.find((entry) => entry.id === facts.packageId), problems: string[] = [];
  const finish = () => ({ packageId: facts.packageId, mappings: problems.length ? [] : row!.mappings, adapters: problems.length ? [] : row!.adapters, sourceDigest: createHash("sha256").update(JSON.stringify([...facts.nodes].sort((a, b) => projectionByteCompare(a.path, b.path)))).digest("hex"), problems });
  if (!row) { problems.push("Unregistered nested Cargo package identity"); return finish(); }
  const memberId = row.id === "wgpu-renderer" ? "members-of-wgpu-target" : "members-of-jco-guest";
  const ownerKind = row.id === "wgpu-renderer" ? "wgpu-target" : "jco-guest";
  const members = taxonomy.semanticDirectoryMemberKinds[memberId];
  const requiredMembers = row.mappings.filter((mapping) => !mapping.destinationPath.startsWith(row.semanticOwnerRoot + "/📦️packages/")).map((mapping) => mapping.destinationPath.slice(row.semanticOwnerRoot.length + 1).split("/")[0]!);
  if (!taxonomy.semanticDirectoryKinds[ownerKind] || members?.source !== "registry" || !members.ownerKindIds.includes(ownerKind) || requiredMembers.some((member) => !members.memberNames.includes(member))) problems.push("Nested Cargo semantic owner/member registration is missing");
  const boundary = taxonomy.packageBoundaryRules["🦀️rust"];
  const requiredEntries = row.id === "wgpu-renderer" ? ["rust-library-entry", "rust-binary-entry", "rust-build-entry", "vitest-config-entry"] : ["rust-library-entry"];
  if (!boundary || requiredEntries.some((id) => !taxonomy.configurableEntryContracts[id] || !taxonomy.packageSourceDispositions[id]) || (row.id === "wgpu-renderer" ? ["library", "binary", "builder", "tests"] : ["library"]).some((id) => !boundary?.allowedDirectoryKindIds.includes(id)) || row.id === "wgpu-renderer" && !taxonomy.semanticDirectoryKinds["typescript-language"]) problems.push("Nested Cargo package boundary prerequisite is missing");
  const cargoEntries = requiredEntries.filter((id) => id.startsWith("rust-"));
  if (cargoEntries.some((id) => !boundary?.entryContractIds.includes(id) || !taxonomy.ecosystems["🦀️rust"]?.entryContractIds?.includes(id) || row.id === "wgpu-renderer" && !taxonomy.targets["🧊️wgpu"]?.entryContractIds?.includes(id))) problems.push("Nested Cargo configured entry is absent from a package, ecosystem or target registry");
  if (problems.length > 0) return finish();
  const destination = facts.layout === "destination";
  const expected = new Map(row.mappings.map((mapping) => [destination ? mapping.destinationPath : mapping.sourcePath, mapping]));
  const adapters = new Map(row.adapters.map((adapter) => [adapter.path, adapter]));
  const derived = new Map(row.derivedLeaves.map((leaf) => [leaf.path, leaf]));
  const expectedPaths = new Set([...expected.keys(), ...(destination ? [...adapters.keys(), ...derived.keys()] : [])]);
  const ignoredOutputs = new Set<string>();
  if (destination) try { for (const path of semanticPackageIgnoredGeneratedOutputPaths(row, taxonomy)) ignoredOutputs.add(path); }
  catch (error) { problems.push(error instanceof Error ? error.message : String(error)); }
  const admittedPaths = new Set([...expectedPaths, ...ignoredOutputs]);
  const allowedDirectories = new Set<string>();
  for (const path of admittedPaths) for (let parent = dirname(path); parent !== "."; parent = dirname(parent)) allowedDirectories.add(parent);
  const nodes = new Map<string, SemanticProjectionAuthorityNode>();
  for (const node of facts.nodes) {
    if (!exactOwnerPath(node.path) || nodes.has(node.path) || !["file", "directory"].includes(node.nodeKind) || node.nodeKind === "directory" && !allowedDirectories.has(node.path) || node.nodeKind === "file" && (!admittedPaths.has(node.path) || ignoredOutputs.has(node.path) && typeof node.content !== "string")) { problems.push("Unadmitted or duplicate nested Cargo node: " + node.path); continue; }
    nodes.set(node.path, node);
  }
  for (const path of expectedPaths) {
    const node = nodes.get(path), mapping = expected.get(path), adapter = destination ? adapters.get(path) : undefined;
    if (node?.nodeKind !== "file" || typeof node.content !== "string") { problems.push("Missing nested Cargo regular leaf: " + path); continue; }
    if (!destination && mapping && (createHash("sha256").update(node.content).digest("hex") !== mapping.sourceHash || Buffer.byteLength(node.content) !== mapping.sourceSize)) problems.push("Nested Cargo source preimage drift: " + path);
    if (adapter && node.content !== adapter.content) problems.push("Nested Cargo canonical adapter bytes drift: " + path);
    if (destination && derived.has(path) && node.content !== derived.get(path)!.content) problems.push("Nested Cargo canonical registration bytes drift: " + path);
  }
  for (const splice of row.sourceSplices) {
    const content = nodes.get(destination ? splice.destinationPath : splice.sourcePath)?.content ?? "", value = destination ? splice.newValue : splice.oldValue;
    if (!content.startsWith(value) || content.split(value).length - 1 !== 1) problems.push("Nested Cargo registration or adapter splice drift: " + splice.id);
  }
  for (const sourcePath of new Set(row.authoredSourceFragments.map((fragment) => fragment.sourcePath))) {
    const mapping = row.mappings.find((mapping) => mapping.sourcePath === sourcePath)!, path = destination ? mapping.destinationPath : sourcePath;
    problems.push(...semanticPackageAuthoredFragmentReferences({ path, content: nodes.get(path)?.content ?? "", layout: destination ? "destination" : "source" }, row).problems);
  }
  if (problems.length === 0) for (const binding of row.joinedPathBindings) {
    const mapping = row.mappings.find((entry) => entry.sourcePath === row.sourceRoot + "/" + binding.consumerRelativePath)!;
    const path = destination ? mapping.destinationPath : mapping.sourcePath;
    problems.push(...semanticPackageJoinedPathReferenceAuthority({ path, content: nodes.get(path)!.content!, layout: destination ? "destination" : "source" }, binding, row).problems);
  }
  if (!destination) {
    const fold = (path: string): string => path.normalize("NFC").toLocaleLowerCase("und").replaceAll("\ufe0f", "");
    const outputPaths = new Set([...row.mappings.map((mapping) => mapping.destinationPath), ...adapters.keys(), ...derived.keys()]);
    const foldedOutputPaths = [...outputPaths].map(fold);
    for (const path of facts.occupiedPaths ?? []) {
      const foldedPath = fold(path);
      if (foldedOutputPaths.some((output) => foldedPath === output || output.startsWith(foldedPath + "/")) && !allowedDirectories.has(path)) problems.push("Nested Cargo destination collision: " + path);
    }
  }
  const activeRoot = destination ? row.destinationRoot : row.sourceRoot;
  const nodeRoot = destination && row.id === "wgpu-renderer" ? row.semanticOwnerRoot + "/📦️packages/🟦️typescript" : activeRoot;
  const manifest = nodes.get(activeRoot + "/Cargo.toml")?.content ?? "";
  if (nestedCargoField(manifest, "package", "name") !== row.identity.cargoPackageName) problems.push("Nested Cargo package name is not the exact registered identity");
  if (nestedCargoField(manifest, "lib", "path") !== (destination ? "📚️library/🦀️.rs" : row.id === "wgpu-renderer" ? "🦀️lib.rs" : "🦀️.rs")) problems.push("Nested Cargo library entry authority drift");
  if (JSON.stringify(nestedCargoField(manifest, "lib", "crate-type")) !== JSON.stringify(row.id === "wgpu-renderer" ? ["cdylib", "rlib"] : ["cdylib"])) problems.push("Nested Cargo crate-type authority drift");
  if (row.workspaceKind === "standalone") {
    const workspace = tomlTableBody(manifest, "workspace");
    if (workspace === undefined || workspace.replace(/#[^\r\n]*/gu, "").trim() !== "" || [...manifest.matchAll(/^\s*\[workspace\]\s*$/gmu)].length !== 1 || nestedCargoField(manifest, "package", "publish") !== false) problems.push("JCO requires its exact empty standalone workspace and non-publishing package");
    const lock = nodes.get(activeRoot + "/Cargo.lock")?.content ?? "";
    if (!lock.startsWith("# This file is automatically @generated by Cargo.\n") || !/^version = 4$/mu.test(lock) || !lock.includes('name = "semio-jcoprobe-guest"')) problems.push("JCO Cargo lock authority drift");
    const wit = nodes.get(activeRoot + (destination ? "/🧬️schema/📜️world.wit" : "/🧬️schema/🧪️world/📜️.wit"))?.content ?? "";
    if (!wit.includes("package semio:jcoprobe@0.1.0;") || !/\bworld\s+jcoprobe\s*\{/u.test(wit)) problems.push("JCO WIT world identity drift");
    const implementation = nodes.get(destination ? row.semanticOwnerRoot + "/🧩️component/🦀️.rs" : row.sourceRoot + "/🦀️.rs")?.content ?? "";
    if ([...implementation.matchAll(/^wit_bindgen::generate!\(\{\s*path:\s*"🧬️schema\/📜️world\.wit",\s*world:\s*"jcoprobe",\s*async:\s*true,?\s*\}\);/gmu)].length !== 1 || [...implementation.matchAll(/^wit_bindgen::generate!/gmu)].length !== 1) problems.push("JCO WIT binding must retain its exact Cargo-manifest-relative authority");
  } else {
    if (nestedCargoField(manifest, "package.metadata.semio", "role") !== "framework" || nestedCargoField(manifest, "package.metadata.semio", "id") !== "renderer-wgpu" || nestedCargoField(manifest, "package", "build") !== (destination ? "🏗️builder/🦀️.rs" : "build.rs")) problems.push("WGPU Cargo target/role authority drift");
    const binary = manifest.replace(/^[ \t]*\[\[bin\]\][ \t]*$/gmu, "[bin]");
    if (nestedCargoField(binary, "bin", "name") !== "semio-wgpu-native" || nestedCargoField(binary, "bin", "path") !== (destination ? "💾️binary/🦀️.rs" : "📦️bin.rs") || JSON.stringify(nestedCargoField(binary, "bin", "required-features")) !== JSON.stringify(["native-bin"])) problems.push("WGPU Cargo binary entry authority drift");
    const declared = (() => { try { return facts.cargoWorkspaceContent !== undefined && cargoWorkspaceDeclaresMemberV1(facts.cargoWorkspaceContent, activeRoot); } catch { return false; } })();
    if (!declared) problems.push("WGPU is absent from the authored Cargo workspace member patterns");
    try {
      const workspace = JSON.parse(facts.nodeWorkspaceContent ?? "null"), node = JSON.parse(nodes.get(nodeRoot + "/package.json")?.content ?? "null"), nx = JSON.parse(nodes.get(nodeRoot + "/📋️project.json")?.content ?? "null");
      const invalidCwd = Object.values(nx?.targets ?? {}).some((target) => { const cwd = (target as { options?: { cwd?: string } }).options?.cwd; return cwd !== undefined && ![nodeRoot, ".", "{projectRoot}"].includes(cwd); });
      if (!Array.isArray(workspace?.workspaces) || !workspace.workspaces.includes(nodeRoot) || node?.name !== row.identity.nodePackageName || node?.exports?.["."] !== (destination ? "./📚️library/🟦️.ts" : "./🟦️.ts") || nx?.name !== row.identity.projectName || nx?.sourceRoot !== (destination ? row.semanticOwnerRoot : activeRoot) || !nx?.targets || invalidCwd || destination && !nx?.namedInputs?.default?.includes(`{workspaceRoot}/${row.semanticOwnerRoot}/**/*`)) problems.push("WGPU Node/Nx workspace identity drift");
    } catch { problems.push("WGPU requires valid Node and Nx manifest evidence"); }
    const configPath = "vitest.config.ts";
    const config = nodes.get(nodeRoot + "/" + configPath)?.content ?? "", script = nodes.get(nodeRoot + "/📜️script.ts")?.content ?? "";
    if (classifyPackageSourceDisposition(config, taxonomy.packageSourceDispositions["vitest-config-entry"]!, taxonomy.packageGlueGrammar.typescript!) !== "tool-metadata" || script.split(JSON.stringify(configPath)).length !== 4) problems.push("WGPU exact Vitest configuration authority drift");
  }
  for (const mapping of row.mappings.filter((entry) => entry.sourceRole !== null && entry.disposition !== "adapter" && entry.disposition !== "tool-metadata")) {
    const path = destination ? mapping.destinationPath : mapping.sourcePath, content = nodes.get(path)?.content;
    const grammar = requiredPackageGlueGrammar(taxonomy, path.endsWith(".rs") ? "rust" : path.endsWith(".js") ? "javascript" : "typescript");
    if (content !== undefined && classifyPackageSource(content, grammar).role !== mapping.sourceRole) problems.push("Nested Cargo implementation role drift: " + path);
  }
  return finish();
}

export function semanticPackageGenerationAuthority(repoRoot: string, packageId: SemanticPackageProjectionCase["id"], taxonomy: Taxonomy) {
  const catalog = semanticPackageProjectionCatalog(repoRoot, taxonomy);
  const row = catalog?.packages.find((entry) => entry.id === packageId);
  if (!catalog || !row) throw new Error("Package adapter catalog is unavailable");
  const source = exactOwnerRegularFile(repoRoot, row.sourceRoot + "/Cargo.toml"), destination = exactOwnerRegularFile(repoRoot, row.destinationRoot + "/Cargo.toml");
  if (source === "invalid" || destination === "invalid" || Number(source === "file") + Number(destination === "file") !== 1) throw new Error("Package adapter requires exactly one source or canonical package");
  const canonical = destination === "file";
  const paths = [...new Set(row.mappings.map((mapping) => canonical ? mapping.destinationPath : mapping.sourcePath))];
  const nodes: SemanticProjectionAuthorityNode[] = paths.map((path) => {
    if (exactOwnerRegularFile(repoRoot, path) !== "file") throw new Error("Package adapter source is not a no-follow regular file: " + path);
    return { path, nodeKind: "file", content: readFileSync(join(repoRoot, path), "utf8") };
  });
  const generated = [...row.adapters.filter((adapter) => !row.mappings.some((mapping) => mapping.destinationPath === adapter.path)), ...row.derivedLeaves];
  for (const leaf of [...row.adapters, ...row.derivedLeaves]) {
    const state = exactOwnerRegularFile(repoRoot, leaf.path);
    if (state === "invalid" || !canonical && state !== "absent") throw new Error("Package generated destination is occupied or invalid: " + leaf.path);
    if (canonical && !paths.includes(leaf.path)) nodes.push({ path: leaf.path, nodeKind: "file", content: state === "file" ? readFileSync(join(repoRoot, leaf.path), "utf8") : leaf.content });
  }
  const workspace = (path: string): string | undefined => {
    if (row.workspaceKind !== "repository") return undefined;
    if (exactOwnerRegularFile(repoRoot, path) !== "file") throw new Error("Package workspace evidence is not a no-follow regular file: " + path);
    return readFileSync(join(repoRoot, path), "utf8");
  };
  const authority = semanticPackageProjectionAuthority({ packageId, nodes, layout: canonical ? "destination" : "source", cargoWorkspaceContent: workspace("Cargo.toml"), nodeWorkspaceContent: workspace("package.json") }, catalog, taxonomy);
  if (authority.problems.length) throw new Error(authority.problems.join(" | "));
  return { authority, generated };
}

/** 🪪️ Checks current source-layout ownership without authorizing frozen source transformations. */
export function semanticPackageSourceManifestIdentity(facts: Readonly<{ cargoManifestContent: string; nodeManifestContent: string; projectManifestContent: string }>, row: SemanticPackageProjectionCase): boolean {
  if (row.id !== "wgpu-renderer" || row.workspaceKind !== "repository") return false;
  try {
    let cargo = facts.cargoManifestContent;
    for (let index = 0; index < cargo.length; index++) {
      if (cargo[index] === "#") { index = cargo.indexOf("\n", index); if (index < 0) break; }
      const quote = cargo[index];
      if (quote !== "\"" && quote !== "'") continue;
      const delimiter = quote.repeat(3);
      if (cargo.slice(index, index + 3) === delimiter) {
        let end = index + 3;
        for (; end < cargo.length && cargo.slice(end, end + 3) !== delimiter; end++) if (quote === "\"" && cargo[end] === "\\") end++;
        if (cargo.slice(end, end + 3) !== delimiter) return false;
        end += 3;
        for (let extra = 0; extra < 2 && cargo[end] === quote; extra++) end++;
        cargo = cargo.slice(0, index) + cargo.slice(index, end).replace(/[^\r\n]/g, " ") + cargo.slice(end);
        index = end - 1;
        continue;
      }
      for (index++; index < cargo.length && cargo[index] !== quote; index++) {
        if (cargo[index] === "\n" || cargo[index] === "\r") return false;
        if (quote === "\"" && cargo[index] === "\\") {
          if (index + 1 >= cargo.length || cargo[index + 1] === "\n" || cargo[index + 1] === "\r") return false;
          index++;
        }
      }
      if (index >= cargo.length) return false;
    }
    const binary = cargo.replace(/^[ \t]*\[\[bin\]\][ \t]*$/gmu, "[bin]");
    const fields: readonly (readonly [string, string, unknown])[] = [
      ["package", "name", row.identity.cargoPackageName], ["package", "build", "build.rs"],
      ["package.metadata.semio", "role", "framework"], ["package.metadata.semio", "id", "renderer-wgpu"],
      ["lib", "path", "🦀️lib.rs"], ["lib", "crate-type", ["cdylib", "rlib"]],
    ];
    if (fields.some(([section, key, expected]) => JSON.stringify(nestedCargoField(cargo, section, key)) !== JSON.stringify(expected))) return false;
    if (nestedCargoField(binary, "bin", "name") !== "semio-wgpu-native" || nestedCargoField(binary, "bin", "path") !== "📦️bin.rs" || JSON.stringify(nestedCargoField(binary, "bin", "required-features")) !== JSON.stringify(["native-bin"])) return false;
    const record = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);
    const node: unknown = JSON.parse(facts.nodeManifestContent), nx: unknown = JSON.parse(facts.projectManifestContent);
    return record(node) && node.name === row.identity.nodePackageName && record(node.exports) && node.exports["."] === "./🟦️.ts" && record(nx) && nx.name === row.identity.projectName && nx.sourceRoot === row.sourceRoot && record(nx.targets) && Object.keys(nx.targets).length > 0 && Object.values(nx.targets).every((target) => record(target) && record(target.options) && target.options.cwd === row.sourceRoot);
  } catch { return false; }
}

export function exampleAssetPath(path: unknown, taxonomy: Pick<Taxonomy, "exampleAssetsDirName">): path is string {
  return exactOwnerPath(path) && path.split("/").includes(taxonomy.exampleAssetsDirName);
}
