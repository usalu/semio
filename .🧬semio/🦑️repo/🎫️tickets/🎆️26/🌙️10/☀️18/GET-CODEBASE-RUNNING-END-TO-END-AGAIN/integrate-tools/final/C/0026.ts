import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type FixedFilenameContract, type FixedDirectoryContract, type FixedContractScope, type ConfigurableEntryContract } from "../🔣️taxonomy/📐️kind-specs/🟦️.ts";
import { type PackageGlueGrammar } from "../🔣️taxonomy/🏭️generators-packages/🟦️.ts";
import { splitLeadingEmoji, emojiFold, utf8Compare } from "../🛣️path/🟦️.ts";
import { sha256 } from "../📁️input/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { classifyPackageSource, classifyPackageSourceDisposition } from "../../🔍️discovery/🏷️package-source-classification/🟦️.ts";
import { packageImplementationAdmitted } from "../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";
import { fixedContractFilename } from "../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";
import { implementationLeafBasenameFinding, subsetIdForDirectoryName, targetInsidePackageBoundaryFinding, taxonomyFileKindIsImplementation } from "../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { pathEmojiStatuteFindings, reservedDocumentationBasename } from "@semio-tech/framework/identity/path";
import { basename, dirname } from "node:path";
import { GENERIC_SEMANTIC_STEMS, type TaxonomyInventory, type TaxonomyInventoryEntry, type TaxonomyPackageRole, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { matchDirectoryKind, resolveFileKind, splitLeadingEmojiIdentity, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { packageLocation } from "../📸️repository-capture/🟦️.ts";
import { mutationDomainOwnerLocation, mutationFixtureMirrorOwnerPath } from "../👥️mutation-projection/🟦️.ts";

type FixedContract = FixedFilenameContract | FixedDirectoryContract;

type FixedSpecificity = readonly [literalSegments: number, literalCodePoints: number, negativeWildcardTokens: number, scopeRank: number];

/** 🎯️ Ranks `FixedContractScope` kinds by how narrowly they constrain a match, most permissive
 * first — an unrestricted `path-pattern` scope matches anywhere, while `sibling-fixed-filename-
 * contract` and `fixed-directory-contract` additionally require another already-resolved, more
 * specific contract to independently hold. Two contracts sharing one `pathPattern` (e.g.
 * `node-package-manifest` vs `build-owned-node-package-manifest`, both `**\/package.json`) previously
 * tied at equal specificity whenever both scopes matched, producing `fixed-contract-ambiguous`; this
 * grades every scope kind distinctly so the narrower one wins deterministically instead. */
export type FixedContractScopeKind = FixedContractScope["kind"];

const FIXED_CONTRACT_SCOPE_SPECIFICITY: Readonly<Record<FixedContractScopeKind, number>> = {
  "path-pattern": 0,
  "repository-root": 1,
  "directory-kind": 2,
  "package-root": 3,
  "fixed-directory-contract": 4,
  "fixed-directory-contract-set": 4,
  "sibling-fixed-filename-contract": 5,
  "exact-path": 6,
};

/** 🧪️ Explicit, minimal re-export of the scope-kind specificity ladder for `fixedSpecificity` — kept
 * separate from the internal `FixedContractScope` union type so tests depend on a stable string-enum
 * boundary instead of reaching into the taxonomy engine's own contract shapes. */
export function fixedContractScopeSpecificityRank(scopeKind: FixedContractScopeKind): number {
  return FIXED_CONTRACT_SCOPE_SPECIFICITY[scopeKind];
}

function fixedSpecificity(contract: FixedContract): FixedSpecificity {
  const segments = contract.pathPattern.split("/");
  const tokens = contract.pathPattern.match(/\*\*|\*|\?|\[[^\]]+\]/gu) ?? [];
  const literals = contract.pathPattern.replaceAll("/", "").replace(/\*\*|\*|\?|\[[^\]]+\]/gu, "");
  return [segments.filter((segment) => !/[?*\[]/u.test(segment)).length, [...literals].length, -tokens.length, FIXED_CONTRACT_SCOPE_SPECIFICITY[contract.scope.kind]];
}

function compareFixedSpecificity(left: FixedSpecificity, right: FixedSpecificity): number {
  for (let index = 0; index < left.length; index++) if (left[index] !== right[index]) return requireTaxonomyValue(right[index],"right[index]") - requireTaxonomyValue(left[index],"left[index]");
  return 0;
}

function equalFixedSpecificity(left: FixedSpecificity, right: FixedSpecificity): boolean {
  return left.every((value, index) => value === right[index]);
}

function fixedScopeMatches(contract: FixedContract, path: string, packageInfo: ReturnType<typeof packageLocation>, parentKindId?: string, parentFixedDirectoryContractId?: string, siblingFixedFilenameContractIds: readonly string[] = []): boolean {
  if (contract.scope.kind === "exact-path") return path === contract.scope.path;
  if (contract.scope.kind === "repository-root") return !path.includes("/");
  if (contract.scope.kind === "package-root") return packageInfo?.packageRoot === dirname(path) && packageInfo.ecosystemId === contract.scope.ecosystemId;
  if (contract.scope.kind === "directory-kind") return parentKindId === contract.scope.directoryKindId;
  if (contract.scope.kind === "fixed-directory-contract") return parentFixedDirectoryContractId === contract.scope.fixedDirectoryContractId;
  if (contract.scope.kind === "fixed-directory-contract-set") return parentFixedDirectoryContractId !== undefined && contract.scope.fixedDirectoryContractIds.includes(parentFixedDirectoryContractId);
  if (contract.scope.kind === "sibling-fixed-filename-contract") return siblingFixedFilenameContractIds.includes(contract.scope.fixedFilenameContractId);
  return true;
}

/** 📇️ One fixed-contract table indexed once: its entries in declaration order, the positions of entries whose final pattern segment is a literal keyed by that NFC segment, and the positions of every other entry. */
interface FixedContractLeafIndex<T> {
  readonly entries: readonly (readonly [string, T])[];
  readonly byLeaf: ReadonlyMap<string, readonly number[]>;
  readonly anyLeaf: readonly number[];
}

const fixedContractLeafIndexes = new WeakMap<object, FixedContractLeafIndex<unknown>>();

const fixedContractSpecificities = new WeakMap<object, ReadonlyMap<string, FixedSpecificity>>();

/** 🍃️ The literal final segment every path matching `pattern` must end with under the taxonomy glob grammar, or null when that segment may vary. */
function fixedContractLiteralLeaf(pattern: string): string | null {
  const normalized = pattern.normalize("NFC"), slash = normalized.lastIndexOf("/"), leaf = normalized.slice(slash + 1), prefix = slash < 0 ? "" : normalized.slice(0, slash);
  if (!leaf || /[*?[\]]/u.test(leaf) || normalized.includes("[")) return null;
  return slash < 0 || !prefix.endsWith("**") || prefix === "**" || prefix.endsWith("/**") ? leaf : null;
}

/** 🗃️ Prepares {@link FixedContractLeafIndex} for `contracts` once per parsed taxonomy table. */
function fixedContractLeafIndex<T extends { readonly pathPattern: string }>(contracts: Readonly<Record<string, T>>): FixedContractLeafIndex<T> {
  const cached = fixedContractLeafIndexes.get(contracts) as FixedContractLeafIndex<T> | undefined;
  if (cached) return cached;
  const entries = Object.entries(contracts), byLeaf = new Map<string, number[]>(), anyLeaf: number[] = [];
  entries.forEach(([, contract], index) => {
    const leaf = fixedContractLiteralLeaf(contract.pathPattern);
    if (leaf === null) anyLeaf.push(index);
    else byLeaf.set(leaf, [...(byLeaf.get(leaf) ?? []), index]);
  });
  const index = { entries, byLeaf, anyLeaf };
  fixedContractLeafIndexes.set(contracts, index);
  return index;
}

/** 🪴️ The entries of one table that can match `path` at all — those whose literal final segment equals the path's, plus every entry whose final segment may vary — in declaration order. */
export function fixedContractCandidates<T extends { readonly pathPattern: string }>(contracts: Readonly<Record<string, T>>, path: string): readonly (readonly [string, T])[] {
  const index = fixedContractLeafIndex(contracts), normalizedPath = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
  const leafCandidates = index.byLeaf.get(normalizedPath.slice(normalizedPath.lastIndexOf("/") + 1));
  return (leafCandidates ? [...index.anyLeaf, ...leafCandidates].sort((left, right) => left - right) : index.anyLeaf).map((position) => index.entries[position]!);
}

/** 🎚️ One table's contract specificities by id, computed once. */
function fixedContractSpecificity<T extends FixedContract>(contracts: Readonly<Record<string, T>>, id: string): FixedSpecificity {
  let specificities = fixedContractSpecificities.get(contracts);
  if (!specificities) fixedContractSpecificities.set(contracts, specificities = new Map(Object.entries(contracts).map(([key, contract]) => [key, fixedSpecificity(contract)])));
  return specificities.get(id)!;
}

export function matchingFixedContracts<T extends FixedContract>(path: string, contracts: Readonly<Record<string, T>>, taxonomy: LoadedTaxonomy, packageInfo: ReturnType<typeof packageLocation>, parentKindId?: string, parentFixedDirectoryContractId?: string, siblingFixedFilenameContractIds: readonly string[] = []): { readonly selected: readonly [string, T] | null; readonly ambiguous: readonly string[] } {
  const matches = fixedContractCandidates(contracts, path)
    .filter(([, contract]) => taxonomy.pathMatcher.matches(path, contract.pathPattern) && fixedScopeMatches(contract, path, packageInfo, parentKindId, parentFixedDirectoryContractId, siblingFixedFilenameContractIds))
    .map(([id, contract]) => ({ id, contract, specificity: fixedContractSpecificity(contracts, id) }))
    .sort((left, right) => compareFixedSpecificity(left.specificity, right.specificity) || left.id.localeCompare(right.id));
  if (matches.length === 0) return { selected: null, ambiguous: [] };
  const top = matches.filter((entry) => equalFixedSpecificity(entry.specificity, requireTaxonomyValue(matches[0],"matches[0]").specificity));
  return top.length === 1 ? { selected: [requireTaxonomyValue(top[0],"top[0]").id, requireTaxonomyValue(top[0],"top[0]").contract], ambiguous: [] } : { selected: null, ambiguous: top.map((entry) => entry.id) };
}

function configurableContract(path: string, taxonomy: LoadedTaxonomy, packageInfo: ReturnType<typeof packageLocation>): [string, ConfigurableEntryContract] | null {
  const rows = Object.entries(taxonomy.schema.configurableEntryContracts).filter(([, contract]) => basename(path).normalize("NFC") === contract.filename.normalize("NFC") && packageInfo?.ecosystemId === contract.ecosystemId);
  return rows.length === 1 ? requireTaxonomyValue(rows[0],"rows[0]") : null;
}

/** 🧱️ Whether the schema admits implementation at this package file: it sits in a nested build unit of its package rule. `unitHasFile` answers whether a repo path exists as a file. */
export function packageImplementationAdmittedAt(path: string, kindId: string | null, taxonomy: LoadedTaxonomy, unitHasFile: (path: string) => boolean): boolean {
  const location = packageLocation(path, taxonomy);
  if (!location?.rule) return false;
  return packageImplementationAdmitted(location.rule.implementationAdmission, {
    relativePath: path.slice(location.packageRoot.length + 1),
    kindId,
    directoryHasFile: (name) => unitHasFile(`${dirname(path)}/${name}`),
    manifestFilename: (contractId) => { const contract = taxonomy.schema.fixedFilenameContracts[contractId]; return contract ? fixedContractFilename(contract) : null; },
  });
}

export function classifyPackageRole(path: string, kindId: string | null, fixedId: string | undefined, content: string | null, taxonomy: LoadedTaxonomy, admitted = false): TaxonomyPackageRole {
  const location = packageLocation(path, taxonomy);
  if (!location) return "not-package";
  const configurable = configurableContract(path, taxonomy, location);
  if (!location.rule || !location.ecosystemId) return "unresolved";
  if (kindId && !location.rule.allowedFileKindIds.includes(kindId)) return admitted ? "admitted-implementation" : "implementation";
  const contractId = fixedId ?? configurable?.[0];
  const disposition = contractId ? taxonomy.schema.packageSourceDispositions[contractId] : undefined;
  const sourceKind = kindId ? taxonomy.schema.fileKinds[kindId]?.role === "source" : false;
  if (contractId && !sourceKind) return "configuration";
  if (!content) return contractId ? "unresolved" : "configuration";
  const grammar = requireTaxonomyValue(taxonomy.schema.packageGlueGrammar[disposition?.grammarId ?? location.rule.glueGrammarId],"taxonomy.schema.packageGlueGrammar[disposition?.grammarId ?? location.rule.glueGrammarId]");
  if (contractId && !disposition) return "unresolved";
  const dispositionRole = disposition ? classifyPackageSourceDisposition(content, disposition, grammar) : undefined;
  const role = dispositionRole === "tool-metadata" ? dispositionRole : classifyPackageSource(content, grammar).role;
  if (role === "tool-metadata") return "configuration";
  if (role === "implementation" && admitted) return "admitted-implementation";
  return grammar.allowedRoles.includes(role as PackageGlueGrammar["allowedRoles"][number]) ? role : role === "implementation" ? "implementation" : "unresolved";
}

export function canonicalDirectory(path: string, parentCanonical: string, parentKindId: string | undefined, ancestorKindIds: readonly string[], taxonomy: LoadedTaxonomy, mutationLeaves: ReadonlySet<string> = new Set()): { readonly path: string; readonly kindId: string | null; readonly fixedId?: string; readonly violations: readonly TaxonomyViolation[] } {
  const name = basename(path).normalize("NFC");
  const topologyFinding = targetInsidePackageBoundaryFinding(path, taxonomy.discoverySchema);
  const topologyViolations = topologyFinding && topologyFinding.path === path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "").normalize("NFC")
    ? [violation(topologyFinding.breachId, topologyFinding.path, "A target boundary must own its language packages; it cannot be nested inside one.")]
    : [];
  const owned = mutationFixtureMirrorOwnerPath(path, taxonomy), domains = taxonomy.discoverySchema.mutationDomainOwners[dirname(owned)], domainOwner = mutationDomainOwnerLocation(owned, taxonomy);
  if (domains && Object.hasOwn(domains, name) || domainOwner && owned === `${domainOwner.root}/${domainOwner.relativePath}`) return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: "members-of-schema", violations: [] };
  if (parentKindId === "subsets" && taxonomy.discoverySchema.subsetDirectoryOverrides?.[dirname(path)] && subsetIdForDirectoryName(dirname(path), name, taxonomy.discoverySchema) !== null) return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: "subset", violations: topologyViolations };
  const fixed = matchingFixedContracts(path, taxonomy.schema.fixedDirectoryContracts, taxonomy, packageLocation(path, taxonomy), parentKindId);
  if (fixed.ambiguous.length > 0) return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("fixed-directory-contract-ambiguous", path, `Equal-specificity fixed directory contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected) {
    const context = matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: context.kind?.id ?? null, fixedId: fixed.selected[0], violations: [] };
  }
  if (parentKindId === "packages") {
    const packageKinds = Object.keys(taxonomy.schema.packageBoundaryRules).filter((id) => emojiFold(id) === emojiFold(name));
    if (packageKinds.length > 1) return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("package-language-ambiguous", path, `Package language boundary is ambiguous: ${packageKinds.join(", ")}`)] };
  }
  const fixturesDirName = taxonomy.discoverySchema.testFixturesDirName;
  const leafKindId = basename(dirname(dirname(path))) === fixturesDirName && mutationLeaves.has(dirname(path)) && mutationLeaves.has(path) ? "members-of-members-of-fixtures" : basename(dirname(path)) === fixturesDirName && mutationLeaves.has(path) ? "members-of-fixtures" : mutationLeaves.has(path) || mutationLeaves.has(owned) ? "members-of-schema" : null;
  const leaf = leafKindId ? splitLeadingEmojiIdentity(name) : null;
  const match = leafKindId && leaf ? { kind: { id: leafKindId, emoji: leaf.first }, slug: leaf.rest, ambiguous: [] as readonly string[] } : matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
  if (!match.kind) {
    const message = match.ambiguous.length > 1 ? `Directory semantic kind is ambiguous: ${match.ambiguous.join(", ")}` : "Directory has no registered semantic kind";
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [...topologyViolations, violation(match.ambiguous.length > 1 ? "directory-kind-ambiguous" : "directory-kind-unresolved", path, message)] };
  }
  const identity = splitLeadingEmojiIdentity(name);
  const canonicalName = identity.sequence !== identity.first ? name : `${match.kind.emoji}${match.slug}`.normalize("NFC");
  const violations = [...topologyViolations, ...(identity.first ? pathEmojiStatuteFindings([{ path, nodeKind: "directory" }], []).map((finding) => violation(`path-emoji-${finding.kind}`, path, "Directory emoji must be handpicked, singular, and correctly presented.")) : [])];
  return { path: parentCanonical ? `${parentCanonical}/${canonicalName}` : canonicalName, kindId: match.kind.id, violations };
}

export function canonicalFile(
  path: string,
  parentCanonical: string,
  parentKindId: string | undefined,
  ancestorKindIds: readonly string[],
  directoryKindByPath: ReadonlyMap<string, string>,
  fixedDirectoryContractByPath: ReadonlyMap<string, string>,
  siblingFixedFilenameContractIdsByParent: ReadonlyMap<string, readonly string[]>,
  authorizedCargoBuildScriptPaths: ReadonlySet<string>,
  taxonomy: LoadedTaxonomy,
  contentKindId?: string,
): { readonly path: string; readonly fileKind: string | null; readonly stem: string | null; readonly fixedId?: string; readonly semanticDirectoryName?: string; readonly violations: readonly TaxonomyViolation[] } {
  const packageInfo = packageLocation(path, taxonomy);
  let fixedName = basename(path);
  const parent = dirname(path);
  let fixed = matchingFixedContracts(path, taxonomy.schema.fixedFilenameContracts, taxonomy, packageInfo, directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent), siblingFixedFilenameContractIdsByParent.get(parent));
  if (fixed.selected?.[0] === "cargo-build-script" && !authorizedCargoBuildScriptPaths.has(path)) fixed = { selected: null, ambiguous: [] };
  const decoratedFixedName = splitLeadingEmoji(fixedName);
  if (!fixed.selected && fixed.ambiguous.length === 0 && decoratedFixedName.emoji && decoratedFixedName.rest) {
    const candidatePath = dirname(path) === "." ? decoratedFixedName.rest : `${dirname(path)}/${decoratedFixedName.rest}`;
    const candidate = matchingFixedContracts(candidatePath, taxonomy.schema.fixedFilenameContracts, taxonomy, packageLocation(candidatePath, taxonomy), directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent), siblingFixedFilenameContractIdsByParent.get(parent));
    const authorizedCandidate = candidate.selected?.[0] === "cargo-build-script" && !authorizedCargoBuildScriptPaths.has(candidatePath) ? { selected: null, ambiguous: [] } : candidate;
    if (authorizedCandidate.selected || authorizedCandidate.ambiguous.length > 0) {
      fixed = authorizedCandidate;
      fixedName = decoratedFixedName.rest;
    }
  }
  if (fixed.ambiguous.length > 0) return { path: parentCanonical ? `${parentCanonical}/${basename(path)}` : basename(path), fileKind: null, stem: null, violations: [violation("fixed-contract-ambiguous", path, `Equal-specificity fixed filename contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected) return { path: parentCanonical ? `${parentCanonical}/${fixedName}` : fixedName, fileKind: null, stem: null, fixedId: fixed.selected[0], violations: [] };
  const documentationName = reservedDocumentationBasename(basename(path));
  if (documentationName) return { path: parentCanonical ? `${parentCanonical}/${documentationName}` : documentationName, fileKind: null, stem: null, violations: [] };
  const configurable = configurableContract(path, taxonomy, packageInfo);
  const resolvedKind = resolveFileKind(path, taxonomy, parentKindId, ancestorKindIds, configurable?.[1].fileKindId, contentKindId);
  if (!resolvedKind.kind) {
    const message = resolvedKind.ambiguous.length > 1 ? `File kind is ambiguous: ${resolvedKind.ambiguous.join(", ")}` : "No file kind owns the longest extension chain";
    return { path: parentCanonical ? `${parentCanonical}/${basename(path).normalize("NFC")}` : basename(path).normalize("NFC"), fileKind: null, stem: null, violations: [violation(resolvedKind.ambiguous.length > 1 ? "file-kind-ambiguous" : "file-kind-unresolved", path, message)] };
  }
  const sourceIdentity = splitLeadingEmojiIdentity(resolvedKind.stem);
  if (sourceIdentity.first && !taxonomyFileKindIsImplementation(resolvedKind.kind.id, taxonomy.discoverySchema)) {
    const preserved = basename(path).normalize("NFC");
    const violations = pathEmojiStatuteFindings([{ path, nodeKind: "file" }], []).map((finding) => violation(`path-emoji-${finding.kind}`, path, "File emoji must be handpicked, singular, and correctly presented."));
    return { path: parentCanonical ? `${parentCanonical}/${preserved}` : preserved, fileKind: resolvedKind.kind.id, stem: sourceIdentity.rest || null, violations };
  }
  const leadingSemantic = splitLeadingEmoji(resolvedKind.stem);
  const semanticEvidence = leadingSemantic.emoji || "";
  const sourceStem = semanticEvidence ? leadingSemantic.rest : resolvedKind.stem;
  const testSuffix = sourceStem.endsWith(".test");
  const semanticStem = testSuffix ? sourceStem.slice(0, -".test".length) : sourceStem;
  const kindOnly = `${resolvedKind.kind.emoji}${resolvedKind.extension}`.normalize("NFC");
  const basenameFinding = implementationLeafBasenameFinding(path, taxonomy.discoverySchema);
  const violations = [
    ...(sourceIdentity.first ? pathEmojiStatuteFindings([{ path, nodeKind: "file" }], []).map((finding) => violation(`path-emoji-${finding.kind}`, path, "File emoji must be handpicked, singular, and correctly presented.")) : []),
    ...(basenameFinding ? [violation(basenameFinding.breachId, path, `Implementation leaf ${JSON.stringify(basenameFinding.actualBasename)} must use ${JSON.stringify(basenameFinding.expectedBasename)}.`)] : []),
  ];
  if (!semanticStem || configurable || GENERIC_SEMANTIC_STEMS.has(semanticStem.toLocaleLowerCase("und"))) return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem || null, violations };
  const parentSlug = splitLeadingEmoji(basename(dirname(path))).rest;
  if (parentSlug.normalize("NFC").toLocaleLowerCase("und") === semanticStem.normalize("NFC").toLocaleLowerCase("und")) return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations };
  const roleContext = testSuffix ? "tests" : resolvedKind.kind.role === "asset" ? "assets" : resolvedKind.kind.role === "test" ? "tests" : parentKindId;
  const semantic = matchDirectoryKind(`${semanticEvidence}${semanticStem}`, taxonomy, roleContext);
  if (!semantic.kind) {
    const message = semantic.ambiguous.length > 1 ? `Semantic stem matches multiple directory kinds: ${semantic.ambiguous.join(", ")}` : "Semantic stem has no registered directory kind";
    return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [...violations, violation(semantic.ambiguous.length > 1 ? "semantic-stem-ambiguous" : "semantic-stem-unresolved", path, message)] };
  }
  if (parentKindId === semantic.kind.id && parentSlug === semanticStem) return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations };
  const semanticDirectory = `${semantic.kind.emoji}${semanticStem}`.normalize("NFC");
  return { path: parentCanonical ? `${parentCanonical}/${semanticDirectory}/${kindOnly}` : `${semanticDirectory}/${kindOnly}`, fileKind: resolvedKind.kind.id, stem: semanticStem, semanticDirectoryName: semanticDirectory, violations };
}

export function packageImplementationDestination(
  sourcePath: string,
  canonical: ReturnType<typeof canonicalFile>,
  canonicalDirectoryByPath: ReadonlyMap<string, string>,
  directoryKindByPath: ReadonlyMap<string, string>,
  taxonomy: LoadedTaxonomy,
): string | null {
  const location = packageLocation(sourcePath, taxonomy);
  if (!location || !canonical.fileKind) return null;
  const ownerCanonical = canonicalDirectoryByPath.get(location.owner) ?? location.owner.normalize("NFC");
  const packageCanonical = canonicalDirectoryByPath.get(location.packageRoot) ?? location.packageRoot.normalize("NFC");
  const fileName = basename(canonical.path);
  const relativePath = canonical.path.startsWith(`${packageCanonical}/`) ? canonical.path.slice(packageCanonical.length + 1) : fileName;
  const modulePrefix = relativePath === fileName ? "" : `${relativePath.slice(0, relativePath.length - fileName.length)}`;
  const at = (path: string): string => ownerCanonical ? `${ownerCanonical}/${path}` : path;
  const stem = canonical.stem?.normalize("NFC") ?? "";
  if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und")) || canonical.semanticDirectoryName) return at(relativePath);
  const semantic = matchDirectoryKind(stem, taxonomy, directoryKindByPath.get(location.owner));
  if (!semantic.kind) return null;
  const directoryName = `${semantic.kind.emoji}${stem}`.normalize("NFC");
  return at(`${modulePrefix}${directoryName}/${fileName}`);
}

export function directoryHash(path: string, children: readonly Pick<TaxonomyInventoryEntry, "sourcePath" | "nodeKind" | "mode" | "contentHash">[]): string {
  const prefix = path ? `${path}/` : "";
  const rows = [...children]
    .sort((a, b) => utf8Compare(a.sourcePath, b.sourcePath))
    .map((entry) => `${entry.nodeKind}\u0000${entry.mode ?? ""}\u0000${entry.sourcePath.slice(prefix.length)}\u0000${entry.contentHash}`);
  return sha256(rows.join("\u0000"));
}

export function inventoryDigestOf(inventory: Omit<TaxonomyInventory, "inventoryDigest" | "repoRoot" | "taxonomyPath">): string {
  return sha256(canonicalJson(inventory));
}
