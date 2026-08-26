// @bun
/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts */
import { execFileSync, spawnSync } from "child_process";
import { createHash as createHash2, randomUUID } from "crypto";
import {
  chmodSync,
  closeSync,
  existsSync as existsSync2,
  fsyncSync,
  linkSync,
  lstatSync,
  mkdirSync,
  mkdtempSync,
  openSync,
  readFileSync as readFileSync2,
  readdirSync as readdirSync2,
  readlinkSync,
  renameSync,
  rmdirSync,
  rmSync,
  symlinkSync,
  writeFileSync
} from "fs";
import { tmpdir } from "os";
import { basename as basename2, dirname as dirname2, isAbsolute, join as join2, posix, relative as relative2, resolve as resolve2, sep } from "path";

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts */
import { ephemeralMap, ephemeralBox } from "@semio-tech/framework";
import { createHash } from "crypto";
import { existsSync, readFileSync, readdirSync, realpathSync, statSync } from "fs";
import { basename, dirname, extname, join, relative, resolve } from "path";
import { fileURLToPath } from "url";
var __dirname2 = dirname(fileURLToPath(import.meta.url));
var cachedTaxonomy = ephemeralBox("framework.products.repo.modules.lib.discovery.component.ts.cachedTaxonomy", undefined);
function readTaxonomyUnchecked() {
  return JSON.parse(readFileSync(join(__dirname2, "../\uD83D\uDD23\uFE0Ftaxonomy.json"), "utf8"));
}
function taxonomyWorkspaceRoot() {
  for (let candidate = __dirname2;dirname(candidate) !== candidate; candidate = dirname(candidate)) {
    if (existsSync(join(candidate, "nx.json")) && existsSync(join(candidate, "\uD83D\uDCCB\uFE0Fproject.json")))
      return candidate;
  }
  return null;
}
function loadTaxonomy() {
  if (cachedTaxonomy.current)
    return cachedTaxonomy.current;
  const taxonomy = readTaxonomyUnchecked();
  const workspaceRoot = taxonomyWorkspaceRoot();
  const problems = [...validateTaxonomy(taxonomy), ...workspaceRoot ? validateGeneratorContractsAgainstWorkspace(workspaceRoot, taxonomy) : ["generatorContracts workspace root could not be resolved."]];
  if (problems.length > 0)
    throw new Error(`Invalid taxonomy schema:
${problems.map((problem) => `- ${problem}`).join(`
`)}`);
  cachedTaxonomy.current = taxonomy;
  return cachedTaxonomy.current;
}
function canonicalFilenamesForKind(kindId, taxonomy = loadTaxonomy()) {
  const kind = taxonomy.fileKinds[kindId];
  return kind ? kind.extensionChains.map((extension) => `${kind.emoji}${extension}`) : [];
}
function canonicalFilenameForKind(kindId, taxonomy = loadTaxonomy()) {
  const names = canonicalFilenamesForKind(kindId, taxonomy);
  if (names.length !== 1)
    throw new Error(`File kind ${JSON.stringify(kindId)} must have exactly one extension chain, got ${names.length}.`);
  return names[0];
}
function fileKindIdForSourcePath(path, taxonomy = loadTaxonomy()) {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const filename = normalized.slice(normalized.lastIndexOf("/") + 1).toLowerCase();
  const terminalCandidates = Object.entries(taxonomy.fileKinds).flatMap(([kindId, kind]) => kind.extensionChains.filter((extension) => filename.endsWith(extension)).map((extension) => ({ kindId, extension })));
  const longest = Math.max(0, ...terminalCandidates.map((candidate) => candidate.extension.length));
  const longestKindIds = [...new Set(terminalCandidates.filter((candidate) => candidate.extension.length === longest).map((candidate) => candidate.kindId))];
  return longestKindIds.length === 1 ? longestKindIds[0] : null;
}
function scopedFileKindIdForSourcePath(path, taxonomy = loadTaxonomy(), context = {}) {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const filename = normalized.slice(normalized.lastIndexOf("/") + 1);
  const matches = Object.entries(taxonomy.scopedFileKinds).filter(([, spec]) => taxonomyPathPatternMatches(normalized, spec.pathPattern) && (spec.parentDirectoryKindId === undefined || spec.parentDirectoryKindId === context.parentDirectoryKindId) && new RegExp(spec.sourceFilenamePattern, "u").test(filename) && spec.extensionChains.some((extension) => filename.endsWith(extension)));
  return matches.length === 1 ? matches[0][0] : null;
}
function taxonomyCliAttemptPreparationsProblems(facts, taxonomy = loadTaxonomy()) {
  const problems = [];
  const names = new Set;
  const ordinals = new Set;
  for (const preparation of [...facts].sort((left, right) => projectionByteCompare(left.directoryName, right.directoryName))) {
    const kindId = semanticDirectoryKindId(preparation.directoryName, taxonomy, { parentKindId: preparation.parentKindId });
    if (kindId !== "transaction-attempt-preparation")
      problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} must resolve below transaction-attempts.`);
    if (names.has(preparation.directoryName))
      problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} is duplicated.`);
    names.add(preparation.directoryName);
    const ordinal = preparation.directoryName.match(/^\uD83D\uDEA7\uFE0Fprepare-([0-9]{6})-/u)?.[1];
    if (!ordinal)
      problems.push(`Attempt preparation ${JSON.stringify(preparation.directoryName)} must carry one six-digit ordinal.`);
    else if (ordinals.has(ordinal))
      problems.push(`Attempt preparation ordinal ${ordinal} is duplicated.`);
    else
      ordinals.add(ordinal);
    const childNames = new Set;
    for (const child of [...preparation.children].sort((left, right) => projectionByteCompare(`${left.name}\x00${left.nodeKind}`, `${right.name}\x00${right.nodeKind}`))) {
      if (childNames.has(child.name))
        problems.push(`Attempt preparation child ${JSON.stringify(child.name)} is duplicated.`);
      childNames.add(child.name);
      const expectedKind = child.name === "\uD83D\uDEA7\uFE0Fstage" ? "transaction-stage" : child.name === "\uD83D\uDCBE\uFE0Fbackup" ? "transaction-backup" : child.name === "\uD83D\uDD12\uFE0Flease" ? "transaction-lease" : null;
      if (expectedKind) {
        if (child.nodeKind !== "directory" || semanticDirectoryKindId(child.name, taxonomy, { parentKindId: kindId ?? undefined }) !== expectedKind)
          problems.push(`Attempt preparation child ${JSON.stringify(child.name)} must be its exact no-follow directory kind.`);
      } else if (child.name === canonicalFilenameForKind("json", taxonomy)) {
        if (child.nodeKind !== "file")
          problems.push("Attempt preparation journal must be an exact regular file.");
      } else
        problems.push(`Attempt preparation child ${JSON.stringify(child.name)} is not admitted.`);
    }
  }
  return problems;
}
function taxonomyCliJsonWritePreparationProblems(facts, taxonomy = loadTaxonomy()) {
  const problems = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-json-write-preparation")
    problems.push("JSON write preparation must resolve below transaction-journal-write, transaction-lease-preparation, or transaction-lease.");
  const allowed = new Set(["\uD83D\uDD23\uFE0F.json", "\u23EE\uFE0F.json"]);
  if (new Set(facts.leafNames).size !== facts.leafNames.length)
    problems.push("JSON write preparation leaves must be unique.");
  for (const leaf of facts.leafNames) {
    if (!allowed.has(leaf))
      problems.push(`JSON write preparation leaf ${JSON.stringify(leaf)} is not admitted.`);
    if (leaf === "\u23EE\uFE0F.json" && scopedFileKindIdForSourcePath(`\uD83D\uDEA7\uFE0Fjournal/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined }) !== "transaction-json-previous")
      problems.push("JSON write displaced previous leaf has no scoped exchange authority.");
  }
  return problems;
}
function taxonomyCliLeaseDirectoryProblems(facts, taxonomy = loadTaxonomy()) {
  const problems = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-lease" && kindId !== "transaction-lease-preparation")
    problems.push("Lease directory must resolve as a canonical lease or token-bound preparation below its exact parent.");
  if (new Set(facts.leafNames).size !== facts.leafNames.length)
    problems.push("Lease leaves must be unique.");
  for (const leaf of facts.leafNames)
    if (leaf !== canonicalFilenameForKind("json", taxonomy))
      problems.push(`Lease leaf ${JSON.stringify(leaf)} is not the canonical JSON record.`);
  if (facts.writePreparations.length > 1)
    problems.push("Lease directory may contain at most one JSON-write preparation.");
  for (const writer of facts.writePreparations)
    problems.push(...taxonomyCliJsonWritePreparationProblems({ parentKindId: kindId ?? "", directoryName: writer.directoryName, leafNames: writer.leafNames }, taxonomy));
  const publishedState = kindId === "transaction-lease" || facts.directoryName.endsWith("-stale");
  const hasCanonical = facts.leafNames.includes(canonicalFilenameForKind("json", taxonomy));
  const exchangeWithoutCanonical = facts.writePreparations.length === 1 && [...facts.writePreparations[0].leafNames].sort(projectionByteCompare).join("\x00") === ["\u23EE\uFE0F.json", "\uD83D\uDD23\uFE0F.json"].sort(projectionByteCompare).join("\x00");
  if (publishedState && !hasCanonical && !exchangeWithoutCanonical)
    problems.push("Published or stale lease must retain canonical JSON or the exact displaced-previous exchange state.");
  return problems;
}
function generatorContractIdsForOutputPath(path, taxonomy = loadTaxonomy()) {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  return Object.entries(taxonomy.generatorContracts).filter(([, contract]) => contract.outputRoots.some((root) => normalized === root.path || normalized.startsWith(`${root.path}/`))).map(([id]) => id);
}
function canonicalSemanticDirectoryName(name, taxonomy) {
  const normalized = name.normalize(taxonomy.unicodeNormalization.form);
  const leading = normalized.match(/^(\p{Extended_Pictographic}\uFE0F?(?:\u200D\p{Extended_Pictographic}\uFE0F?)*)/u)?.[1];
  if (!leading)
    return normalized;
  const canonicalEmoji = leading.replace(/(\p{Extended_Pictographic})(?!\uFE0F)/gu, "$1\uFE0F");
  return `${canonicalEmoji}${normalized.slice(leading.length)}`;
}
function semanticDirectoryKindId(name, taxonomy = loadTaxonomy(), context = {}) {
  const normalized = canonicalSemanticDirectoryName(name, taxonomy);
  const parentKindId = context.parentKindId;
  const matches = Object.entries(taxonomy.semanticDirectoryKinds).filter(([, spec]) => {
    if (spec.parentKindIds && !parentKindId)
      return false;
    if (spec.parentKindIds && !spec.parentKindIds.includes(parentKindId))
      return false;
    if (!normalized.startsWith(spec.emoji))
      return false;
    const slug = normalized.slice(spec.emoji.length);
    return slug.length === 0 && spec.allowEmojiOnly || slug.length > 0 && new RegExp(spec.slugPattern, "u").test(slug);
  });
  const contextual = parentKindId ? matches.filter(([, spec]) => spec.parentKindIds?.includes(parentKindId)) : [];
  const resolved = contextual.length > 0 ? contextual : matches.filter(([, spec]) => !spec.parentKindIds);
  if (resolved.length === 1)
    return resolved[0][0];
  if (resolved.length > 1)
    return null;
  const owners = [context.parentKindId, ...context.ancestorKindIds ?? []].filter((id) => Boolean(id));
  for (const ownerKindId of owners) {
    const memberMatches = Object.entries(taxonomy.semanticDirectoryMemberKinds).filter(([, spec]) => spec.ownerKindIds.includes(ownerKindId) && spec.memberNames.includes(normalized));
    if (memberMatches.length > 0)
      return memberMatches.length === 1 ? memberMatches[0][0] : null;
  }
  return null;
}
function renderSemanticProjectionProfile(contractId, identity, taxonomy = loadTaxonomy()) {
  const contract = taxonomy.semanticPathProjectionContracts[contractId];
  const renderer = contract && taxonomy.semanticPathProjectionProfileRenderers[contract.profileRendererId];
  if (!contract || !renderer || !identity.artifactId)
    throw new Error(`Unknown or incomplete semantic projection ${JSON.stringify(contractId)}.`);
  if (!new RegExp(taxonomy.semanticDirectoryKinds.standard.slugPattern, "u").test(identity.standardVersion) || !new RegExp(taxonomy.semanticDirectoryKinds.subset.slugPattern, "u").test(identity.subsetId))
    throw new Error("Projection profile captures do not satisfy standard/subset slug contracts.");
  const rendered = renderer.template.replace("{standardVersion}", identity.standardVersion).replace("{subsetId}", identity.subsetId);
  const profileIndex = contract.destinationSegments.findIndex((segment) => ("render" in segment) && segment.render === "profile");
  const parent = profileIndex > 0 ? contract.destinationSegments[profileIndex - 1] : undefined;
  const parentKindId = parent && "kindId" in parent ? parent.kindId : undefined;
  if (semanticDirectoryKindId(rendered, taxonomy, { parentKindId }) !== renderer.directoryKindId)
    throw new Error(`Projection profile renderer produced invalid directory ${JSON.stringify(rendered)}.`);
  return rendered;
}
function semanticPathProjectionReferenceConsumers(projectionContractId, sourcePath, adapter, form, taxonomy = loadTaxonomy()) {
  return Object.entries(taxonomy.semanticPathProjectionReferenceConsumerContracts).filter(([, contract]) => contract.projectionContractId === projectionContractId && contract.sourcePathIdentities.includes(sourcePath) && contract.adapters.includes(adapter) && contract.supportedForms.includes(form) && new RegExp(contract.sourcePathPattern, "u").test(sourcePath)).map(([id, contract]) => ({ id, contract })).sort((left, right) => projectionByteCompare(left.id, right.id));
}
function projectionByteCompare(left, right) {
  return Buffer.from(left).compare(Buffer.from(right));
}
function projectionDirectories(root, paths) {
  const directories = new Set([root]);
  for (const path of paths) {
    const segments = path.slice(root.length + 1).split("/");
    segments.pop();
    let current = root;
    for (const segment of segments) {
      current = `${current}/${segment}`;
      directories.add(current);
    }
  }
  return [...directories].sort(projectionByteCompare);
}
function projectionCanonicalKey(path, comparison) {
  const normalized = path.normalize("NFC");
  if (comparison === "nfc")
    return normalized;
  if (comparison === "case-fold")
    return normalized.toLocaleLowerCase("und");
  return normalized.replaceAll("\uFE0F", "");
}
function artifactProjectionPathProblems(paths, occupiedPaths, taxonomy) {
  const problems = [];
  for (const comparison of ["nfc", "case-fold", "vs16-fold"]) {
    const seen = new Map;
    for (const entry of paths) {
      const key = `${entry.nodeKind}\x00${projectionCanonicalKey(entry.path, comparison)}`;
      const prior = seen.get(key);
      if (prior && prior !== entry.path)
        problems.push(`${comparison} collision between ${JSON.stringify(prior)} and ${JSON.stringify(entry.path)}.`);
      seen.set(key, entry.path);
    }
  }
  const occupied = new Set(occupiedPaths.flatMap((path) => [path, projectionCanonicalKey(path, "nfc"), projectionCanonicalKey(path, "case-fold"), projectionCanonicalKey(path, "vs16-fold")]));
  for (const { path } of paths) {
    if ([path, projectionCanonicalKey(path, "nfc"), projectionCanonicalKey(path, "case-fold"), projectionCanonicalKey(path, "vs16-fold")].some((key) => occupied.has(key)))
      problems.push(`Projected destination ${JSON.stringify(path)} is occupied.`);
    if (new TextEncoder().encode(path).length > taxonomy.collisionPolicy.maxPathBytes)
      problems.push(`Projected destination ${JSON.stringify(path)} exceeds maxPathBytes ${taxonomy.collisionPolicy.maxPathBytes}.`);
  }
  return problems;
}
function renderArtifactPathProjectionRoot(options, taxonomy = loadTaxonomy()) {
  const problems = [];
  const contract = taxonomy.semanticPathProjectionContracts[options.contractId];
  if (!contract || !contract.sourceArtifactMemberName)
    return { captures: {}, destinationRoot: "", problems: [`Unknown artifact projection contract ${JSON.stringify(options.contractId)}.`] };
  const sourceOwner = taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
  const artifactDirectoryName = basename(options.artifactRoot);
  if (artifactDirectoryName !== contract.sourceArtifactMemberName || !sourceOwner?.memberNames.includes(artifactDirectoryName))
    problems.push(`Artifact root does not match sourceArtifactMemberName ${JSON.stringify(contract.sourceArtifactMemberName)}.`);
  if (options.sourceRoot !== options.sourceRoot.normalize("NFC") || options.artifactRoot !== options.artifactRoot.normalize("NFC") || /\uFE0E/u.test(`${options.artifactRoot}\x00${options.sourceRoot}`))
    problems.push("Projection roots must be NFC and must not contain VS15.");
  const prefix = `${options.artifactRoot}/`;
  const sourceNames = options.sourceRoot.startsWith(prefix) ? options.sourceRoot.slice(prefix.length).split("/") : [];
  if (sourceNames.length !== contract.sourceSegments.length)
    problems.push("Source root does not have the exact projection grammar length.");
  const captures = {};
  let parentKindId;
  for (const [index, segment] of contract.sourceSegments.entries()) {
    const name = sourceNames[index] ?? "";
    if ("memberKindId" in segment) {
      const member = taxonomy.semanticDirectoryMemberKinds[segment.memberKindId];
      if (name !== segment.literal || !member?.ownerKindIds.includes(parentKindId ?? "") || !member.memberNames.includes(canonicalSemanticDirectoryName(name, taxonomy)))
        problems.push(`Source segment ${index} does not match exact member registry ${JSON.stringify(segment.memberKindId)}.`);
      parentKindId = segment.memberKindId;
      continue;
    }
    if ("projectedMemberKindId" in segment) {
      const projected = taxonomy.semanticProjectedMemberKinds[segment.projectedMemberKindId];
      const source = projected && taxonomy.semanticDirectoryMemberKinds[projected.sourceMemberKindId];
      const canonical = canonicalSemanticDirectoryName(name, taxonomy);
      if (!projected || projected.projectionContractId !== options.contractId || !projected.ownerKindIds.includes(parentKindId ?? "") || !source?.memberNames.includes(canonical) || canonical !== name)
        problems.push(`Source segment ${index} does not match projected member ${JSON.stringify(segment.projectedMemberKindId)}.`);
      captures[segment.capture] = name;
      parentKindId = segment.projectedMemberKindId;
      continue;
    }
    if ("literal" in segment) {
      if (name !== segment.literal || semanticDirectoryKindId(name, taxonomy, { parentKindId }) !== segment.kindId)
        problems.push(`Source segment ${index} does not match kind ${JSON.stringify(segment.kindId)}.`);
    } else {
      const kind = taxonomy.semanticDirectoryKinds[segment.kindId];
      if (!kind || semanticDirectoryKindId(name, taxonomy, { parentKindId }) !== segment.kindId || !name.startsWith(kind.emoji))
        problems.push(`Source capture ${JSON.stringify(segment.capture)} does not match kind ${JSON.stringify(segment.kindId)}.`);
      else
        captures[segment.capture] = name.slice(kind.emoji.length);
    }
    parentKindId = segment.kindId;
  }
  const standardVersion = captures.standardVersion ?? "";
  const subsetId = captures.subsetId ?? "";
  let destinationParentKindId;
  const destinationNames = [];
  for (const segment of contract.destinationSegments) {
    if ("projectedMemberKindId" in segment) {
      const value = captures[segment.copy];
      if (!value)
        problems.push(`Destination copy ${JSON.stringify(segment.copy)} is missing.`);
      else
        destinationNames.push(value);
      destinationParentKindId = segment.projectedMemberKindId;
      continue;
    }
    if ("literal" in segment)
      destinationNames.push(segment.literal);
    else if ("render" in segment) {
      try {
        destinationNames.push(renderSemanticProjectionProfile(options.contractId, { artifactId: artifactDirectoryName, standardVersion, subsetId }, taxonomy));
      } catch (error) {
        problems.push(error instanceof Error ? error.message : String(error));
      }
    } else {
      const value = captures[segment.copy];
      if (!value)
        problems.push(`Destination copy ${JSON.stringify(segment.copy)} is missing.`);
      else
        destinationNames.push(value);
    }
    destinationParentKindId = segment.kindId;
  }
  return { captures, destinationRoot: problems.length === 0 ? `${options.artifactRoot}/${destinationNames.join("/")}` : "", problems };
}
function projectionJsonManifest(node, scope, problems) {
  if (node.nodeKind !== "file" || typeof node.content !== "string") {
    problems.push(`${scope} must be a readable JSON file.`);
    return null;
  }
  try {
    const value = JSON.parse(node.content);
    if (!value || typeof value !== "object" || Array.isArray(value))
      throw new Error("not an object");
    return value;
  } catch {
    problems.push(`${scope} must contain one JSON object.`);
    return null;
  }
}
function projectionFileMappingDigest(mappings) {
  return createHash("sha256").update(mappings.map(({ sourcePath, destinationPath }) => `${sourcePath}\x00${destinationPath}`).join(`
`)).digest("hex");
}
function projectionRelativePath(fromDirectory, toPath) {
  const from = fromDirectory.split("/");
  const to = toPath.split("/");
  let shared = 0;
  while (shared < from.length && shared < to.length && from[shared] === to[shared])
    shared++;
  return [...from.slice(shared).map(() => ".."), ...to.slice(shared)].join("/");
}
function projectionStructuredValue(content, adapter, structuredLocation) {
  if (adapter === "json") {
    let value = JSON.parse(content);
    for (const segment of structuredLocation.split("."))
      value = typeof value === "object" && value !== null ? value[segment] : undefined;
    return value;
  }
  const separator = structuredLocation.lastIndexOf(".");
  if (separator < 1 || separator === structuredLocation.length - 1)
    return;
  const body = tomlTableBody(content, structuredLocation.slice(0, separator));
  return body === undefined ? undefined : tomlTableValues(body)[structuredLocation.slice(separator + 1)];
}
function semanticPathProjectionAuthority(options, taxonomy = loadTaxonomy()) {
  const root = renderArtifactPathProjectionRoot(options, taxonomy);
  const problems = [...root.problems];
  const contract = taxonomy.semanticPathProjectionContracts[options.contractId];
  const pathOwners = new Map;
  for (const node of options.nodes) {
    if (node.path !== node.path.normalize("NFC") || /\uFE0E/u.test(node.path))
      problems.push(`Projection node ${JSON.stringify(node.path)} must be NFC and must not contain VS15.`);
    if (!(node.path === options.sourceRoot || node.path.startsWith(`${options.sourceRoot}/`)))
      problems.push(`Projection node ${JSON.stringify(node.path)} is outside sourceRoot.`);
    if (pathOwners.has(node.path))
      problems.push(`Projection node ${JSON.stringify(node.path)} is duplicated.`);
    pathOwners.set(node.path, node);
    if (node.nodeKind === "symlink")
      problems.push(`Projection source contains forbidden symlink ${JSON.stringify(node.path)}.`);
  }
  if (pathOwners.get(options.sourceRoot)?.nodeKind !== "directory")
    problems.push("Projection sourceRoot must be present as a directory node.");
  const actualFiles = options.nodes.filter((node) => node.nodeKind === "file").map((node) => node.path).sort(projectionByteCompare);
  const expectedSourceDirectories = projectionDirectories(options.sourceRoot, actualFiles);
  const actualSourceDirectories = options.nodes.filter((node) => node.nodeKind === "directory").map((node) => node.path).sort(projectionByteCompare);
  if (expectedSourceDirectories.join("\x00") !== actualSourceDirectories.join("\x00"))
    problems.push("Projection source directories must be exactly those owned by source files.");
  const candidateMappings = [];
  const configurableEntries = [];
  if (contract && root.destinationRoot) {
    const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
    if (catalog && "contractKind" in catalog && catalog.contractKind === "distributed-json-manifest-catalog") {
      const modelIds = new Set;
      const memberIds = new Set;
      const modelDirectories = actualSourceDirectories.filter((path) => dirname(path) === options.sourceRoot).map((path) => basename(path));
      for (const modelDirectoryName of modelDirectories) {
        const manifestPath = `${options.sourceRoot}/${modelDirectoryName}/${catalog.modelManifestSourceFilename}`;
        const manifestNode = pathOwners.get(manifestPath);
        const manifest = manifestNode ? projectionJsonManifest(manifestNode, `Model manifest ${JSON.stringify(manifestPath)}`, problems) : null;
        if (!manifestNode)
          problems.push(`Model manifest is missing for ${JSON.stringify(modelDirectoryName)}.`);
        const leading = modelDirectoryName.match(/^(\p{Extended_Pictographic}\uFE0F?(?:\u200D\p{Extended_Pictographic}\uFE0F?)*)/u)?.[1] ?? "";
        const semanticStem = modelDirectoryName.slice(leading.length);
        if (!leading || canonicalSemanticDirectoryName(modelDirectoryName, taxonomy) !== modelDirectoryName || manifest?.schema !== catalog.modelManifestSchema || manifest?.[catalog.memberVersionField] !== catalog.requiredMemberVersion || manifest?.[catalog.modelIdentityField] !== semanticStem)
          problems.push(`Model manifest ${JSON.stringify(manifestPath)} must declare the canonical directory id, schema, and version.`);
        if (typeof manifest?.[catalog.modelIdentityField] === "string") {
          const id = manifest[catalog.modelIdentityField];
          if (modelIds.has(id))
            problems.push(`Model manifest identity ${JSON.stringify(id)} is duplicated.`);
          modelIds.add(id);
        }
      }
      for (const sourcePath of actualFiles) {
        const relativePath = sourcePath.slice(options.sourceRoot.length + 1);
        const segments = relativePath.split("/");
        const modelDirectoryName = segments[0] ?? "";
        let destinationRelativePath = "";
        let expectedSchema = catalog.modelManifestSchema;
        if (segments.length === 2 && segments[1] === catalog.modelManifestSourceFilename)
          destinationRelativePath = `${modelDirectoryName}/${canonicalFilenameForKind("json", taxonomy)}`;
        else {
          const rule = catalog.categoryRules.find((candidate) => candidate.sourceDirectoryName === segments[1]);
          if (!rule)
            problems.push(`Unknown CAD catalog category in ${JSON.stringify(sourcePath)}.`);
          else if (rule.sourceShape === "direct-semantic-json" && segments.length === 3) {
            const match = segments[2].match(/^\uD83D\uDD23\uFE0F(.+)\.json$/u);
            if (!match)
              problems.push(`CAD direct category file ${JSON.stringify(sourcePath)} does not have the exact semantic JSON shape.`);
            else
              destinationRelativePath = `${modelDirectoryName}/${rule.sourceDirectoryName}/${rule.memberDirectoryEmoji}${match[1]}/${canonicalFilenameForKind("json", taxonomy)}`;
            expectedSchema = rule.manifestSchema;
          } else if (rule.sourceShape === "nested-fixed-json" && segments.length === 4 && segments[3] === rule.fixedSourceFilename && canonicalSemanticDirectoryName(segments[2], taxonomy) === segments[2]) {
            destinationRelativePath = `${modelDirectoryName}/${rule.sourceDirectoryName}/${segments[2]}/${canonicalFilenameForKind("json", taxonomy)}`;
            expectedSchema = rule.manifestSchema;
          } else
            problems.push(`CAD category member ${JSON.stringify(sourcePath)} does not match its exact category rule.`);
        }
        const node = pathOwners.get(sourcePath);
        const manifest = projectionJsonManifest(node, `Catalog manifest ${JSON.stringify(sourcePath)}`, problems);
        if (manifest && (manifest.schema !== expectedSchema || manifest[catalog.memberVersionField] !== catalog.requiredMemberVersion || typeof manifest[catalog.memberIdentityField] !== "string" || manifest[catalog.memberIdentityField] === ""))
          problems.push(`Catalog manifest ${JSON.stringify(sourcePath)} has an invalid manifest schema, identity, or version.`);
        if (manifest && typeof manifest[catalog.memberIdentityField] === "string") {
          const key = `${modelDirectoryName}\x00${segments[1] ?? ""}\x00${manifest[catalog.memberIdentityField]}`;
          if (memberIds.has(key))
            problems.push(`Distributed catalog member identity ${JSON.stringify(key)} is duplicated.`);
          memberIds.add(key);
        }
        if (destinationRelativePath)
          candidateMappings.push({ sourcePath, destinationPath: `${root.destinationRoot}/${destinationRelativePath}` });
      }
    } else if (catalog && "contractKind" in catalog && catalog.contractKind === "exact-owner-vectors") {
      const capture = root.captures.commandDirectoryName;
      const vectors = catalog.vectors.filter((vector) => vector.artifactId === basename(options.artifactRoot) && vector.standardVersion === root.captures.standardVersion && vector.subsetId === root.captures.subsetId && vector.commandDirectoryName === capture);
      if (vectors.length !== 1)
        problems.push("Draw command source must match exactly one owner vector.");
      const descendant = taxonomy.semanticDescendantContracts[contract.descendantContractId];
      if (!descendant || "contractKind" in descendant)
        problems.push("Draw projection must reference one exact descendant bundle.");
      else {
        const expectedSourceNodes = new Set;
        for (const node of descendant.requiredNodes) {
          const sourceParent = ("configurableEntry" in node ? node.sourcePathSegments : node.pathSegments).map((segment) => segment.literal);
          const destinationRelativePath = semanticDescendantNodeRelativePath(node, taxonomy);
          let sourceRelativePath = sourceParent.join("/");
          if (node.nodeType === "file") {
            if ("kindId" in node) {
              const kind = taxonomy.fileKinds[node.kindId];
              sourceRelativePath = [...sourceParent, node.sourceFilename ?? canonicalFilenameForKind(node.kindId, taxonomy)].join("/");
            } else if ("fixedFilenameContractId" in node)
              sourceRelativePath = [...sourceParent, fixedContractFilename(taxonomy.fixedFilenameContracts[node.fixedFilenameContractId])].join("/");
            else
              sourceRelativePath = [...sourceParent, node.configurableEntry.sourceFilename].join("/");
            const mapping = { sourcePath: `${options.sourceRoot}/${sourceRelativePath}`, destinationPath: `${root.destinationRoot}/${destinationRelativePath}` };
            candidateMappings.push(mapping);
            if ("configurableEntry" in node)
              configurableEntries.push({ ...mapping, configurationReferences: node.configurableEntry.configurationReferences });
          }
          expectedSourceNodes.add(sourceRelativePath ? `${options.sourceRoot}/${sourceRelativePath}` : options.sourceRoot);
        }
        const actualSourceNodes = new Set(options.nodes.filter((node) => node.nodeKind !== "symlink").map((node) => node.path));
        if (expectedSourceNodes.size !== actualSourceNodes.size || [...expectedSourceNodes].some((path) => !actualSourceNodes.has(path)))
          problems.push("Draw source does not contain the exact command bundle.");
      }
    } else
      problems.push(`Projection catalog ${JSON.stringify(contract.catalogContractId)} has the wrong authority kind.`);
  }
  candidateMappings.sort((left, right) => projectionByteCompare(left.sourcePath, right.sourcePath));
  const candidateReferenceEdits = [];
  for (const entry of configurableEntries)
    for (const reference of entry.configurationReferences) {
      const fixedContract = taxonomy.fixedFilenameContracts[reference.fixedFilenameContractId];
      const manifestFilename = fixedContract && fixedContractFilename(fixedContract);
      const sourceManifestPath = manifestFilename ? `${entry.sourcePath.slice(0, entry.sourcePath.lastIndexOf("/"))}/${manifestFilename}` : "";
      const manifestMapping = candidateMappings.find(({ sourcePath }) => sourcePath === sourceManifestPath);
      const manifestNode = pathOwners.get(sourceManifestPath);
      if (!fixedContract || !manifestMapping || manifestNode?.nodeKind !== "file" || manifestNode.content === undefined) {
        problems.push(`Configurable entry ${JSON.stringify(entry.sourcePath)} is missing its exact configuration manifest mapping.`);
        continue;
      }
      const oldValue = projectionRelativePath(sourceManifestPath.slice(0, sourceManifestPath.lastIndexOf("/")), entry.sourcePath);
      const newValue = projectionRelativePath(manifestMapping.destinationPath.slice(0, manifestMapping.destinationPath.lastIndexOf("/")), entry.destinationPath);
      let actualValue;
      try {
        actualValue = projectionStructuredValue(manifestNode.content, reference.adapter, reference.structuredLocation);
      } catch {
        actualValue = undefined;
      }
      if (actualValue !== oldValue) {
        problems.push(`Configuration reference ${JSON.stringify(`${sourceManifestPath}:${reference.structuredLocation}`)} must resolve exactly to ${JSON.stringify(oldValue)}.`);
        continue;
      }
      candidateReferenceEdits.push({
        path: manifestMapping.destinationPath,
        adapter: reference.adapter,
        structuredLocation: reference.structuredLocation,
        oldValue,
        newValue,
        preimageHash: createHash("sha256").update(manifestNode.content).digest("hex")
      });
    }
  candidateReferenceEdits.sort((left, right) => projectionByteCompare(`${left.path}\x00${left.structuredLocation}`, `${right.path}\x00${right.structuredLocation}`));
  const destinationDirectories = root.destinationRoot ? projectionDirectories(root.destinationRoot, candidateMappings.map(({ destinationPath }) => destinationPath)) : [];
  const destinationNodes = [...destinationDirectories.map((path) => ({ path, nodeKind: "directory" })), ...candidateMappings.map(({ destinationPath: path }) => ({ path, nodeKind: "file" }))];
  problems.push(...artifactProjectionPathProblems(destinationNodes, options.occupiedPaths ?? [], taxonomy));
  const accepted = problems.length === 0;
  return {
    contractId: options.contractId,
    sourceRoot: options.sourceRoot,
    destinationRoot: root.destinationRoot,
    mappings: accepted ? candidateMappings : [],
    referenceEdits: accepted ? candidateReferenceEdits : [],
    destinationDirectoryCount: accepted ? destinationDirectories.length : 0,
    destinationNodeCount: accepted ? destinationNodes.length : 0,
    mappingDigest: accepted ? projectionFileMappingDigest(candidateMappings) : "",
    maxPathBytes: accepted ? Math.max(0, ...destinationNodes.map(({ path }) => new TextEncoder().encode(path).length)) : 0,
    problems
  };
}
function semanticDescendantNodeRelativePath(node, taxonomy = loadTaxonomy()) {
  const parent = ("configurableEntry" in node ? node.destinationPathSegments : node.pathSegments).map((segment) => segment.literal);
  if (node.nodeType === "directory")
    return parent.join("/");
  if ("kindId" in node)
    return [...parent, canonicalFilenameForKind(node.kindId, taxonomy)].join("/");
  if ("fixedFilenameContractId" in node) {
    const contract = taxonomy.fixedFilenameContracts[node.fixedFilenameContractId];
    if (!contract)
      throw new Error(`Unknown fixed filename contract ${JSON.stringify(node.fixedFilenameContractId)}.`);
    return [...parent, fixedContractFilename(contract)].join("/");
  }
  const entry = taxonomy.configurableEntryContracts[node.configurableEntry.contractId];
  if (!entry)
    throw new Error(`Unknown configurable entry contract ${JSON.stringify(node.configurableEntry.contractId)}.`);
  return [...parent, entry.filename].join("/");
}
function taxonomyPatternExpression(pattern) {
  let expression = "^";
  for (let index = 0;index < pattern.length; ) {
    if (pattern.slice(index, index + 3) === "**/") {
      expression += "(?:[^/]+/)*";
      index += 3;
      continue;
    }
    const character = pattern[index];
    if (character === "*" && pattern[index + 1] === "*") {
      expression += ".*";
      index += 2;
      continue;
    }
    if (character === "*")
      expression += "[^/]*";
    else if (character === "?")
      expression += "[^/]";
    else if (character === "[") {
      const end = pattern.indexOf("]", index + 1);
      if (end < 0)
        throw new Error(`Invalid taxonomy path pattern ${JSON.stringify(pattern)}.`);
      expression += pattern.slice(index, end + 1);
      index = end;
    } else
      expression += character.replace(/[\\^$.*+?()[\]{}|]/gu, "\\$&");
    index += 1;
  }
  return new RegExp(`${expression}$`, "u");
}
function taxonomyPathPatternMatches(path, pattern) {
  const normalizedPath = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
  return taxonomyPatternExpression(pattern.normalize("NFC")).test(normalizedPath);
}
function fixedContractFilename(contract) {
  return contract.pathPattern.slice(contract.pathPattern.lastIndexOf("/") + 1);
}
function validateTaxonomy(taxonomy = readTaxonomyUnchecked()) {
  const problems = [];
  const document = taxonomy;
  const removedKeys = [
    "semanticManifestFilename",
    "subsetsManifestFilename",
    "packagingFileNames",
    "packagingFileSuffixes",
    "packagingDirNames",
    "surfaceSchemaSpecFilenames",
    "textSpecFilenames",
    "binarySpecFilenames",
    "artifactSchemaSpecFilenames",
    "exampleLeafFilenames",
    "exampleTestLeafFilenames",
    "semioDataLeafPrefix",
    "semioFileExtension",
    "artifactSpecFilenames",
    "windowEmptyFacetFilename",
    "taxonomyLeafFilenames",
    "entryFilenames",
    "storyLeafFilename",
    "requireEmojiPrefixWithVs16",
    "rootDataFileNames",
    "rootDocFileNames",
    "areaStates",
    "pluginTaxonomyStates",
    "repoWideFiles",
    "testFeatureFilename",
    "testAdapterFilenames",
    "testContributionFilename",
    "testOutputMarkerFilename",
    "testExcludedPathPrefixes",
    "testOracleRegistryPath",
    "testSchemaPath",
    "layeringGeneratedInventories",
    "semanticProjectionContracts",
    "projectedMemberKinds",
    "projectionContracts",
    "profileRenderers",
    "descendantContracts",
    "mutationCatalogProjectionContractId",
    "mutationCatalogKindBijection"
  ];
  for (const key of removedKeys)
    if (key in document)
      problems.push(`${key} was removed by schema version 7; use kind IDs or exact contracts.`);
  if (taxonomy.schemaVersion !== 7)
    problems.push(`schemaVersion must be exactly 7, got ${JSON.stringify(taxonomy.schemaVersion)}.`);
  const record = (value, key) => {
    const valid = typeof value === "object" && value !== null && !Array.isArray(value);
    if (!valid || Object.keys(value).length === 0)
      problems.push(`${key} must be a non-empty object.`);
    return valid;
  };
  const ids = (values, registry, key) => {
    if (!Array.isArray(values)) {
      problems.push(`${key} must be an array.`);
      return;
    }
    const seen = new Set;
    for (const id of values) {
      if (seen.has(id))
        problems.push(`${key} contains duplicate id ${JSON.stringify(id)}.`);
      seen.add(id);
      if (!(id in registry))
        problems.push(`${key} references missing id ${JSON.stringify(id)}.`);
    }
  };
  const pattern = (value, key) => {
    try {
      new RegExp(value, "u");
    } catch {
      problems.push(`${key} is not a valid Unicode regular expression.`);
    }
  };
  const fullPattern = (value, key) => {
    if (typeof value !== "string") {
      problems.push(`${key} must be a string.`);
      return;
    }
    pattern(value, key);
    if (!value.startsWith("^") || !value.endsWith("$"))
      problems.push(`${key} must be an anchored full-match pattern.`);
  };
  const pathPattern = (value, key) => {
    if (typeof value !== "string" || !value || value !== value.normalize("NFC") || value.startsWith("/") || value.includes("\\") || /[{}!]/u.test(value) || /(^|\/)\*\*[^/]|[^/]\*\*(\/|$)/u.test(value)) {
      problems.push(`${key} must be one NFC workspace-relative v7 path pattern.`);
      return;
    }
    try {
      taxonomyPatternExpression(value);
    } catch {
      problems.push(`${key} is not a valid v7 path pattern.`);
    }
  };
  const workspacePath = (value, key) => {
    const valid = typeof value === "string" && value.length > 0 && value === value.normalize("NFC") && !value.startsWith("/") && !value.endsWith("/") && !value.includes("\\") && !/[*?\[\]{}!]/u.test(value) && value.split("/").every((segment) => segment.length > 0 && segment !== "." && segment !== "..");
    if (!valid)
      problems.push(`${key} must be one exact NFC workspace-relative path.`);
    return valid;
  };
  const extensionChain = (value, key) => {
    if (typeof value !== "string" || !/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(value))
      problems.push(`${key} must be one lowercase dot-prefixed extension chain.`);
  };
  const exactKeys = (value, allowed, key) => {
    const actual = Object.keys(value).sort();
    const expected = [...allowed].sort();
    if (actual.join("\x00") !== expected.join("\x00"))
      problems.push(`${key} must contain exactly ${expected.join(", ")}.`);
  };
  const kebabId = (value, key) => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(value))
      problems.push(`${key} must be kebab-case.`);
  };
  if (record(taxonomy.fileKinds, "fileKinds")) {
    const canonical = new Map;
    const extensionOwners = new Map;
    for (const [id, spec] of Object.entries(taxonomy.fileKinds)) {
      if (!/^\p{Extended_Pictographic}\uFE0F$/u.test(spec.emoji) || spec.emoji !== spec.emoji.normalize("NFC"))
        problems.push(`fileKinds[${JSON.stringify(id)}].emoji must be one NFC emoji plus U+FE0F.`);
      if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0)
        problems.push(`fileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
      for (const extension of spec.extensionChains ?? []) {
        if (!/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(extension))
          problems.push(`fileKinds[${JSON.stringify(id)}] has invalid extension chain ${JSON.stringify(extension)}.`);
        const extensionOwner = extensionOwners.get(extension);
        if (extensionOwner)
          problems.push(`fileKinds ${JSON.stringify(extensionOwner)} and ${JSON.stringify(id)} both own physical extension ${JSON.stringify(extension)}.`);
        extensionOwners.set(extension, id);
        const filename = `${spec.emoji}${extension}`;
        const prior = canonical.get(filename);
        if (prior)
          problems.push(`fileKinds ${JSON.stringify(prior)} and ${JSON.stringify(id)} collide at ${JSON.stringify(filename)}.`);
        canonical.set(filename, id);
      }
    }
    for (const [id, extension] of [["png", ".png"], ["bmp", ".bmp"]]) {
      const kind = taxonomy.fileKinds[id];
      if (!kind || kind.role !== "asset" || kind.emoji !== "\uD83D\uDDBC\uFE0F" || kind.extensionChains.join("\x00") !== extension)
        problems.push(`fileKinds.${id} must be the canonical \uD83D\uDDBC\uFE0F${extension} asset kind.`);
    }
  }
  if (record(taxonomy.fileKindResolutionRules, "fileKindResolutionRules"))
    for (const [id, rule] of Object.entries(taxonomy.fileKindResolutionRules)) {
      extensionChain(rule.extensionChain, `fileKindResolutionRules[${JSON.stringify(id)}].extensionChain`);
      const kind = taxonomy.fileKinds[rule.fileKindId];
      if (!kind || !kind.extensionChains.includes(rule.extensionChain))
        problems.push(`fileKindResolutionRules[${JSON.stringify(id)}] must reference a kind owning its extension chain.`);
      if (rule.priority !== 0)
        problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].priority must be zero for physical resolution.`);
      for (const removed of ["filenamePattern", "pathPattern", "parentKindIds", "ancestorKindIds"])
        if (removed in rule)
          problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].${removed} is forbidden; directories own semantics.`);
    }
  for (const [fileKindId, kind] of Object.entries(taxonomy.fileKinds))
    for (const extension of kind.extensionChains) {
      const rules = Object.values(taxonomy.fileKindResolutionRules).filter((rule) => rule.extensionChain === extension && rule.fileKindId === fileKindId);
      if (rules.length !== 1)
        problems.push(`fileKindResolutionRules must own ${JSON.stringify(extension)} exactly once for ${JSON.stringify(fileKindId)}.`);
    }
  if (!(typeof taxonomy.scopedFileKinds === "object" && taxonomy.scopedFileKinds !== null && !Array.isArray(taxonomy.scopedFileKinds)))
    problems.push("scopedFileKinds must be an object.");
  else
    for (const [id, spec] of Object.entries(taxonomy.scopedFileKinds)) {
      pathPattern(spec.pathPattern, `scopedFileKinds[${JSON.stringify(id)}].pathPattern`);
      if (spec.parentDirectoryKindId !== undefined && !taxonomy.semanticDirectoryKinds[spec.parentDirectoryKindId])
        problems.push(`scopedFileKinds[${JSON.stringify(id)}].parentDirectoryKindId must reference a semantic directory kind.`);
      if (typeof spec.emoji !== "string" || !/^\p{Extended_Pictographic}\uFE0F$/u.test(spec.emoji) || spec.emoji !== spec.emoji.normalize("NFC"))
        problems.push(`scopedFileKinds[${JSON.stringify(id)}].emoji must be one NFC emoji plus U+FE0F.`);
      if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0)
        problems.push(`scopedFileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
      for (const extension of spec.extensionChains ?? [])
        extensionChain(extension, `scopedFileKinds[${JSON.stringify(id)}].extensionChains`);
      if (spec.role !== "evidence")
        problems.push(`scopedFileKinds[${JSON.stringify(id)}].role must be evidence.`);
      fullPattern(spec.sourceFilenamePattern, `scopedFileKinds[${JSON.stringify(id)}].sourceFilenamePattern`);
      if (!spec.authority || !spec.reason || !spec.verification)
        problems.push(`scopedFileKinds[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
      if (!(spec.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(spec.expires)))
        problems.push(`scopedFileKinds[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
    }
  if (record(taxonomy.semanticDirectoryKinds, "semanticDirectoryKinds"))
    for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryKinds)) {
      if (!/^\p{Extended_Pictographic}\uFE0F(?:\u200D\p{Extended_Pictographic}\uFE0F)*$/u.test(spec.emoji) || spec.emoji !== spec.emoji.normalize("NFC"))
        problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence with U+FE0F.`);
      if (typeof spec.slugPattern !== "string")
        problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern must be a string.`);
      else
        pattern(spec.slugPattern, `semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern`);
      if (typeof spec.allowEmojiOnly !== "boolean")
        problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].allowEmojiOnly must be boolean.`);
      if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean")
        problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].inferWithoutEmoji must be boolean when present.`);
      if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean")
        problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].projectionOnly must be boolean when present.`);
      ids(spec.parentKindIds ?? [], { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds }, `semanticDirectoryKinds[${JSON.stringify(id)}].parentKindIds`);
    }
  const taxonomyCliArtifactDirectoryKinds = {
    "taxonomy-transaction": { name: "\uD83E\uDDFE\uFE0Ftaxonomy-transaction", emoji: "\uD83E\uDDFE\uFE0F", slugPattern: "^taxonomy-transaction$" },
    "transaction-digest": { name: `\uD83D\uDD16\uFE0F${"0".repeat(64)}`, emoji: "\uD83D\uDD16\uFE0F", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-transaction"] },
    "transaction-attempts": { name: "\uD83D\uDD02\uFE0Fattempts", emoji: "\uD83D\uDD02\uFE0F", slugPattern: "^attempts$", parentKindIds: ["transaction-digest"] },
    "transaction-attempt": { name: "\uD83D\uDD22\uFE0F000001", emoji: "\uD83D\uDD22\uFE0F", slugPattern: "^[0-9]{6}$", parentKindIds: ["transaction-attempts"] },
    "transaction-stage": { name: "\uD83D\uDEA7\uFE0Fstage", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^stage$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-backup": { name: "\uD83D\uDCBE\uFE0Fbackup", emoji: "\uD83D\uDCBE\uFE0F", slugPattern: "^backup$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-lease": { name: "\uD83D\uDD12\uFE0Flease", emoji: "\uD83D\uDD12\uFE0F", slugPattern: "^lease$", parentKindIds: ["transaction-attempt", "transaction-attempt-preparation"] },
    "transaction-attempt-preparation": { name: "\uD83D\uDEA7\uFE0Fprepare-000001-42-123e4567-e89b-42d3-a456-426614174000", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^prepare-[0-9]{6}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-attempts"] },
    "transaction-edit-preparation": { name: "\uD83D\uDEA7\uFE0Fedit-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^edit-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-stage"] },
    "transaction-edit-write-preparation": { name: "\uD83D\uDEA7\uFE0Fwrite-42-123e4567-e89b-42d3-a456-426614174000", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-edit-preparation"] },
    "transaction-journal-write": { name: "\uD83D\uDEA7\uFE0Fjournal", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^journal$", parentKindIds: ["transaction-stage"] },
    "transaction-json-write-preparation": { name: "\uD83D\uDEA7\uFE0Fwrite-42-123e4567-e89b-42d3-a456-426614174000", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-journal-write", "transaction-lease-preparation", "transaction-lease"] },
    "transaction-lease-preparation": { name: "\uD83D\uDEA7\uFE0Flease-42-123e4567-e89b-42d3-a456-426614174000-preparing", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^lease-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}-(preparing|stale)$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-preparation": { name: "\uD83D\uDEA7\uFE0Fbackup-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^backup-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "transaction-backup-write-preparation": { name: "\uD83D\uDEA7\uFE0Fwrite-42-123e4567-e89b-42d3-a456-426614174000", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^write-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup-preparation"] },
    "transaction-restore-preparation": { name: "\uD83D\uDEA7\uFE0Frestore-0123456789abcdef01234567-42-123e4567-e89b-42d3-a456-426614174000", emoji: "\uD83D\uDEA7\uFE0F", slugPattern: "^restore-[0-9a-f]{24}-[1-9][0-9]*-[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$", parentKindIds: ["transaction-backup"] },
    "taxonomy-inventory-data": { name: "\uD83D\uDCCA\uFE0Ftaxonomy-inventory", emoji: "\uD83D\uDCCA\uFE0F", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-data": { name: "\uD83D\uDCCA\uFE0Ftaxonomy-plan", emoji: "\uD83D\uDCCA\uFE0F", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-data": { name: "\uD83D\uDCCA\uFE0Ftaxonomy-apply", emoji: "\uD83D\uDCCA\uFE0F", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-data": { name: "\uD83D\uDCCA\uFE0Ftaxonomy-verification", emoji: "\uD83D\uDCCA\uFE0F", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-summary": { name: "\uD83D\uDCD3\uFE0Ftaxonomy-inventory", emoji: "\uD83D\uDCD3\uFE0F", slugPattern: "^taxonomy-inventory$" },
    "taxonomy-plan-summary": { name: "\uD83D\uDCD3\uFE0Ftaxonomy-plan", emoji: "\uD83D\uDCD3\uFE0F", slugPattern: "^taxonomy-plan$" },
    "taxonomy-apply-summary": { name: "\uD83D\uDCD3\uFE0Ftaxonomy-apply", emoji: "\uD83D\uDCD3\uFE0F", slugPattern: "^taxonomy-apply$" },
    "taxonomy-verification-summary": { name: "\uD83D\uDCD3\uFE0Ftaxonomy-verification", emoji: "\uD83D\uDCD3\uFE0F", slugPattern: "^taxonomy-verification$" },
    "taxonomy-inventory-shards": { name: "\uD83D\uDCCA\uFE0Fshards", emoji: "\uD83D\uDCCA\uFE0F", slugPattern: "^shards$", parentKindIds: ["taxonomy-inventory-data"] },
    "taxonomy-inventory-shard-digest": { name: `\uD83D\uDD16\uFE0F${"0".repeat(64)}`, emoji: "\uD83D\uDD16\uFE0F", slugPattern: "^[a-f0-9]{64}$", parentKindIds: ["taxonomy-inventory-shards"] }
  };
  for (const [id, expected] of Object.entries(taxonomyCliArtifactDirectoryKinds)) {
    const spec = taxonomy.semanticDirectoryKinds[id];
    if (!spec) {
      problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must declare the permanent taxonomy CLI artifact directory.`);
      continue;
    }
    exactKeys(spec, expected.parentKindIds ? ["emoji", "slugPattern", "allowEmojiOnly", "parentKindIds"] : ["emoji", "slugPattern", "allowEmojiOnly"], `semanticDirectoryKinds[${JSON.stringify(id)}]`);
    if (spec.emoji !== expected.emoji || spec.slugPattern !== expected.slugPattern || spec.allowEmojiOnly || JSON.stringify(spec.parentKindIds) !== JSON.stringify(expected.parentKindIds))
      problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] must remain the exact taxonomy CLI artifact contract.`);
    if (semanticDirectoryKindId(expected.name, taxonomy, { parentKindId: expected.parentKindIds?.[0] }) !== id)
      problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}] does not resolve its canonical taxonomy CLI artifact directory uniquely.`);
  }
  const editCandidate = taxonomy.scopedFileKinds["transaction-edit-candidate"];
  if (!editCandidate || editCandidate.pathPattern !== "**/\uD83D\uDEA7\uFE0Fstage/\uD83D\uDEA7\uFE0Fedit-*/*.edit" || editCandidate.parentDirectoryKindId !== "transaction-edit-preparation" || editCandidate.extensionChains.join("\x00") !== ".edit" || editCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.edit$")
    problems.push("scopedFileKinds.transaction-edit-candidate must remain the exact transaction edit candidate authority.");
  const editPreimage = taxonomy.scopedFileKinds["transaction-edit-preimage"];
  if (!editPreimage || editPreimage.pathPattern !== "**/\uD83D\uDEA7\uFE0Fstage/\uD83D\uDEA7\uFE0Fedit-*/*.pre" || editPreimage.parentDirectoryKindId !== "transaction-edit-preparation" || editPreimage.extensionChains.join("\x00") !== ".pre" || editPreimage.sourceFilenamePattern !== "^[0-9a-f]{24}\\.pre$")
    problems.push("scopedFileKinds.transaction-edit-preimage must remain the exact transaction edit displaced-preimage authority.");
  const editWriteCandidate = taxonomy.scopedFileKinds["transaction-edit-write-candidate"];
  if (!editWriteCandidate || editWriteCandidate.pathPattern !== "**/\uD83D\uDEA7\uFE0Fstage/\uD83D\uDEA7\uFE0Fedit-*/\uD83D\uDEA7\uFE0Fwrite-*/\uD83D\uDEA7\uFE0F.edit" || editWriteCandidate.parentDirectoryKindId !== "transaction-edit-write-preparation" || editWriteCandidate.extensionChains.join("\x00") !== ".edit" || editWriteCandidate.sourceFilenamePattern !== "^\uD83D\uDEA7\uFE0F\\.edit$")
    problems.push("scopedFileKinds.transaction-edit-write-candidate must remain the exact unpublished edit-writer authority.");
  const backupWriteCandidate = taxonomy.scopedFileKinds["transaction-backup-write-candidate"];
  if (!backupWriteCandidate || backupWriteCandidate.pathPattern !== "**/\uD83D\uDCBE\uFE0Fbackup/\uD83D\uDEA7\uFE0Fbackup-*/\uD83D\uDEA7\uFE0Fwrite-*/\uD83D\uDEA7\uFE0F.backup" || backupWriteCandidate.parentDirectoryKindId !== "transaction-backup-write-preparation" || backupWriteCandidate.extensionChains.join("\x00") !== ".backup" || backupWriteCandidate.sourceFilenamePattern !== "^\uD83D\uDEA7\uFE0F\\.backup$")
    problems.push("scopedFileKinds.transaction-backup-write-candidate must remain the exact unpublished backup-writer authority.");
  const jsonPrevious = taxonomy.scopedFileKinds["transaction-json-previous"];
  if (!jsonPrevious || jsonPrevious.pathPattern !== "**/\uD83D\uDEA7\uFE0Fwrite-*/\u23EE\uFE0F.json" || jsonPrevious.parentDirectoryKindId !== "transaction-json-write-preparation" || jsonPrevious.extensionChains.join("\x00") !== ".json" || jsonPrevious.sourceFilenamePattern !== "^\u23EE\uFE0F\\.json$")
    problems.push("scopedFileKinds.transaction-json-previous must remain the exact displaced canonical JSON authority.");
  const backupCandidate = taxonomy.scopedFileKinds["transaction-backup-candidate"];
  if (!backupCandidate || backupCandidate.pathPattern !== "**/\uD83D\uDCBE\uFE0Fbackup/\uD83D\uDEA7\uFE0Frestore-*/*.backup" || backupCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || backupCandidate.extensionChains.join("\x00") !== ".backup" || backupCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.backup$")
    problems.push("scopedFileKinds.transaction-backup-candidate must remain the exact transaction restore preimage authority.");
  const postimageCandidate = taxonomy.scopedFileKinds["transaction-postimage-candidate"];
  if (!postimageCandidate || postimageCandidate.pathPattern !== "**/\uD83D\uDCBE\uFE0Fbackup/\uD83D\uDEA7\uFE0Frestore-*/*.post" || postimageCandidate.parentDirectoryKindId !== "transaction-restore-preparation" || postimageCandidate.extensionChains.join("\x00") !== ".post" || postimageCandidate.sourceFilenamePattern !== "^[0-9a-f]{24}\\.post$")
    problems.push("scopedFileKinds.transaction-postimage-candidate must remain the exact transaction restore postimage authority.");
  if (semanticDirectoryKindId("\uD83D\uDEA7\uFE0Fwrite-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease-preparation" }) !== "transaction-json-write-preparation" || semanticDirectoryKindId("\uD83D\uDEA7\uFE0Fwrite-42-123e4567-e89b-42d3-a456-426614174000", taxonomy, { parentKindId: "transaction-lease" }) !== "transaction-json-write-preparation")
    problems.push("semanticDirectoryKinds.transaction-json-write-preparation must resolve below all three exact parent kinds.");
  for (const [name, expectedKindId] of [["\uD83D\uDEA7\uFE0Fstage", "transaction-stage"], ["\uD83D\uDCBE\uFE0Fbackup", "transaction-backup"], ["\uD83D\uDD12\uFE0Flease", "transaction-lease"]])
    if (semanticDirectoryKindId(name, taxonomy, { parentKindId: "transaction-attempt-preparation" }) !== expectedKindId)
      problems.push(`semanticDirectoryKinds.${expectedKindId} must resolve below transaction-attempt-preparation.`);
  const attemptPreparation = { parentKindId: "transaction-attempts", directoryName: "\uD83D\uDEA7\uFE0Fprepare-000001-42-123e4567-e89b-42d3-a456-426614174000", children: [{ name: "\uD83D\uDEA7\uFE0Fstage", nodeKind: "directory" }, { name: "\uD83D\uDCBE\uFE0Fbackup", nodeKind: "directory" }, { name: "\uD83D\uDD12\uFE0Flease", nodeKind: "directory" }, { name: "\uD83D\uDD23\uFE0F.json", nodeKind: "file" }] };
  if (taxonomyCliAttemptPreparationsProblems([attemptPreparation], taxonomy).length > 0 || taxonomyCliAttemptPreparationsProblems([attemptPreparation, attemptPreparation], taxonomy).length === 0)
    problems.push("transaction-attempt-preparation must retain its validate-all exact child and duplicate-sibling authority.");
  const leasePreparation = { parentKindId: "transaction-backup", directoryName: "\uD83D\uDEA7\uFE0Flease-42-123e4567-e89b-42d3-a456-426614174000-preparing", leafNames: ["\uD83D\uDD23\uFE0F.json"], writePreparations: [] };
  if (taxonomyCliLeaseDirectoryProblems(leasePreparation, taxonomy).length > 0)
    problems.push("transaction-lease-preparation must admit one complete canonical JSON lease before publication.");
  if (record(taxonomy.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds")) {
    const allDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
    const ownerMembers = new Set;
    for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryMemberKinds)) {
      ids(spec.ownerKindIds, allDirectoryKinds, `semanticDirectoryMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      if (!Array.isArray(spec.memberNames) || spec.memberNames.length === 0)
        problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].memberNames must be non-empty.`);
      for (const name of spec.memberNames ?? []) {
        if (typeof name !== "string" || name !== name.normalize("NFC") || /[\\/]/u.test(name) || !/^\p{Extended_Pictographic}\uFE0F/u.test(name))
          problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}] has invalid exact member ${JSON.stringify(name)}.`);
        for (const owner of spec.ownerKindIds ?? []) {
          const key = `${owner}\x00${name}`;
          if (ownerMembers.has(key))
            problems.push(`semanticDirectoryMemberKinds collide for owner ${JSON.stringify(owner)} and member ${JSON.stringify(name)}.`);
          ownerMembers.add(key);
        }
      }
      if (spec.source !== "registry")
        problems.push(`semanticDirectoryMemberKinds[${JSON.stringify(id)}].source must be registry.`);
    }
  }
  const projectionDirectoryKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds };
  if (record(taxonomy.semanticProjectedMemberKinds, "semanticProjectedMemberKinds")) {
    for (const [id, spec] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
      kebabId(id, `semanticProjectedMemberKinds id ${JSON.stringify(id)}`);
      exactKeys(spec, ["ownerKindIds", "projectionContractId", "sourceMemberKindId", "identityField"], `semanticProjectedMemberKinds[${JSON.stringify(id)}]`);
      ids(spec.ownerKindIds, projectionDirectoryKinds, `semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds`);
      if (spec.ownerKindIds.length === 0)
        problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].ownerKindIds must be non-empty.`);
      if (!taxonomy.semanticPathProjectionContracts[spec.projectionContractId])
        problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].projectionContractId is missing.`);
      if (!taxonomy.semanticDirectoryMemberKinds[spec.sourceMemberKindId])
        problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].sourceMemberKindId is missing.`);
      const expectedIdentityField = spec.projectionContractId === "artifact-mutation-tests-v1" ? "mutationDirectoryName" : spec.projectionContractId === "artifact-editor-command-bundle-v1" ? "commandDirectoryName" : null;
      if (spec.identityField !== expectedIdentityField)
        problems.push(`semanticProjectedMemberKinds[${JSON.stringify(id)}].identityField does not match its projection contract.`);
    }
    const visiting = new Set;
    const visited = new Set;
    const visit = (id) => {
      if (visiting.has(id)) {
        problems.push(`semanticProjectedMemberKinds contains an owner cycle at ${JSON.stringify(id)}.`);
        return;
      }
      if (visited.has(id))
        return;
      visiting.add(id);
      for (const owner of taxonomy.semanticProjectedMemberKinds[id]?.ownerKindIds ?? [])
        if (taxonomy.semanticProjectedMemberKinds[owner])
          visit(owner);
      visiting.delete(id);
      visited.add(id);
    };
    for (const id of Object.keys(taxonomy.semanticProjectedMemberKinds))
      visit(id);
  }
  if (record(taxonomy.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers"))
    for (const [id, renderer] of Object.entries(taxonomy.semanticPathProjectionProfileRenderers)) {
      kebabId(id, `semanticPathProjectionProfileRenderers id ${JSON.stringify(id)}`);
      exactKeys(renderer, ["direction", "captureFields", "directoryKindId", "template", "tupleCollisionFields"], `semanticPathProjectionProfileRenderers[${JSON.stringify(id)}]`);
      if (renderer.direction !== "forward-only")
        problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].direction must be forward-only.`);
      if (renderer.captureFields.join("\x00") !== "standardVersion\x00subsetId")
        problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].captureFields must be exactly standardVersion, subsetId.`);
      if (!taxonomy.semanticDirectoryKinds[renderer.directoryKindId])
        problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].directoryKindId is missing.`);
      if (renderer.template !== "\uD83E\uDE86\uFE0F{standardVersion}-{subsetId}")
        problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].template must be the exact forward profile renderer.`);
      if (renderer.tupleCollisionFields.join("\x00") !== "artifactId\x00standardVersion\x00subsetId")
        problems.push(`semanticPathProjectionProfileRenderers[${JSON.stringify(id)}].tupleCollisionFields must be exactly artifactId, standardVersion, subsetId.`);
    }
  const validateDescendantNode = (node, key, rootKindId) => {
    const configurable = "configurableEntry" in node;
    const authorityKey = node.nodeType === "file" ? "kindId" in node ? "kindId" : ("fixedFilenameContractId" in node) ? "fixedFilenameContractId" : "configurableEntry" : "kindId";
    exactKeys(node, configurable ? ["sourcePathSegments", "destinationPathSegments", "nodeType", authorityKey] : ["pathSegments", "nodeType", authorityKey, ..."sourceFilename" in node ? ["sourceFilename"] : []], key);
    const validateSegments = (segments, field) => {
      if (!Array.isArray(segments)) {
        problems.push(`${key}.${field} must be an array.`);
        return;
      }
      let parentKindId = rootKindId;
      for (const [index, segment] of segments.entries()) {
        exactKeys(segment, ["kindId", "literal"], `${key}.${field}[${index}]`);
        if (!taxonomy.semanticDirectoryKinds[segment.kindId])
          problems.push(`${key}.${field}[${index}].kindId is missing.`);
        else if (semanticDirectoryKindId(segment.literal, taxonomy, { parentKindId }) !== segment.kindId)
          problems.push(`${key}.${field}[${index}].literal does not resolve uniquely to its kind.`);
        parentKindId = segment.kindId;
      }
    };
    if (configurable) {
      validateSegments(node.sourcePathSegments, "sourcePathSegments");
      validateSegments(node.destinationPathSegments, "destinationPathSegments");
    } else
      validateSegments(node.pathSegments, "pathSegments");
    if (node.nodeType === "directory") {
      if (!projectionDirectoryKinds[node.kindId])
        problems.push(`${key}.kindId is not a directory kind.`);
      const expected = node.pathSegments.at(-1)?.kindId ?? rootKindId;
      if (node.kindId !== expected)
        problems.push(`${key}.kindId must equal its realized directory kind ${JSON.stringify(expected)}.`);
    } else if (node.nodeType === "file") {
      if ("kindId" in node && !taxonomy.fileKinds[node.kindId])
        problems.push(`${key}.kindId is not a file kind.`);
      else if ("kindId" in node && node.sourceFilename !== undefined && (node.sourceFilename !== node.sourceFilename.normalize("NFC") || /[\\/]/u.test(node.sourceFilename) || !node.sourceFilename.startsWith(taxonomy.fileKinds[node.kindId].emoji) || fileKindIdForSourcePath(node.sourceFilename, taxonomy) !== node.kindId))
        problems.push(`${key}.sourceFilename must be one NFC basename resolving to kindId.`);
      else if ("fixedFilenameContractId" in node && !taxonomy.fixedFilenameContracts[node.fixedFilenameContractId])
        problems.push(`${key}.fixedFilenameContractId is missing.`);
      else if (configurable) {
        exactKeys(node.configurableEntry, ["contractId", "sourceFilename", "configurationReferences"], `${key}.configurableEntry`);
        const entry = taxonomy.configurableEntryContracts[node.configurableEntry.contractId];
        if (!entry)
          problems.push(`${key}.configurableEntry.contractId is missing.`);
        else if (node.configurableEntry.sourceFilename !== node.configurableEntry.sourceFilename.normalize("NFC") || /[\\/]/u.test(node.configurableEntry.sourceFilename) || fileKindIdForSourcePath(node.configurableEntry.sourceFilename, taxonomy) !== entry.fileKindId)
          problems.push(`${key}.configurableEntry.sourceFilename must be one NFC basename resolving to the entry file kind.`);
        if (!Array.isArray(node.configurableEntry.configurationReferences) || node.configurableEntry.configurationReferences.length === 0)
          problems.push(`${key}.configurableEntry.configurationReferences must be non-empty.`);
        const actualConfigurationSources = [];
        for (const [index, reference] of (node.configurableEntry.configurationReferences ?? []).entries()) {
          const scope = `${key}.configurableEntry.configurationReferences[${index}]`;
          exactKeys(reference, ["fixedFilenameContractId", "adapter", "structuredLocation"], scope);
          const fixed = taxonomy.fixedFilenameContracts[reference.fixedFilenameContractId];
          const filename = fixed && fixedContractFilename(fixed);
          if (!fixed || fixed.scope.kind !== "package-root" || fixed.scope.ecosystemId !== entry?.ecosystemId)
            problems.push(`${scope}.fixedFilenameContractId must own the same package ecosystem.`);
          if ((reference.adapter === "toml" ? !filename?.endsWith(".toml") : reference.adapter === "json" ? !filename?.endsWith(".json") : true) || !/^[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)+$/u.test(reference.structuredLocation))
            problems.push(`${scope} must declare an exact structured adapter location.`);
          if (filename)
            actualConfigurationSources.push(`${filename}:${reference.structuredLocation}`);
        }
        if (entry && actualConfigurationSources.sort().join("\x00") !== [...entry.configurationSources].sort().join("\x00"))
          problems.push(`${key}.configurableEntry.configurationReferences must cover every declared configuration source exactly once.`);
      }
    } else
      problems.push(`${key}.nodeType must be directory or file.`);
    try {
      return `${node.nodeType}:${semanticDescendantNodeRelativePath(node, taxonomy)}`;
    } catch {
      return null;
    }
  };
  if (record(taxonomy.semanticDescendantContracts, "semanticDescendantContracts"))
    for (const [id, contract] of Object.entries(taxonomy.semanticDescendantContracts)) {
      kebabId(id, `semanticDescendantContracts id ${JSON.stringify(id)}`);
      if ("contractKind" in contract) {
        exactKeys(contract, ["contractKind", "rootDirectoryKindId", "catalogContractId", "leafFileKindId", "rendering", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
        if (contract.contractKind !== "catalog")
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].contractKind must be catalog.`);
        if (!taxonomy.semanticDirectoryKinds[contract.rootDirectoryKindId])
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
        const catalog = taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId];
        if (!catalog || !("contractKind" in catalog) || catalog.contractKind !== "distributed-json-manifest-catalog")
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].catalogContractId must reference one distributed JSON manifest catalog.`);
        if (!taxonomy.fileKinds[contract.leafFileKindId])
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].leafFileKindId is missing.`);
        if (contract.rendering !== "semantic-member-directory-and-physical-kind-leaf")
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rendering is invalid.`);
        exactKeys(contract.pathBudgetReserve, ["derivation", "bytes"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
        if (contract.pathBudgetReserve.derivation !== "longest-rendered-catalog-descendant-suffix")
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
        if (!Number.isSafeInteger(contract.pathBudgetReserve.bytes) || contract.pathBudgetReserve.bytes <= 0 || contract.pathBudgetReserve.bytes >= taxonomy.collisionPolicy.maxPathBytes)
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.bytes must be a positive safe integer below maxPathBytes.`);
        continue;
      }
      exactKeys(contract, ["rootDirectoryKindId", "requiredNodes", "exclusiveAlternatives", "realizedNodeCount", "pathBudgetReserve"], `semanticDescendantContracts[${JSON.stringify(id)}]`);
      if (!projectionDirectoryKinds[contract.rootDirectoryKindId])
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].rootDirectoryKindId is missing.`);
      if (!Array.isArray(contract.requiredNodes) || contract.requiredNodes.length === 0)
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be non-empty.`);
      const requiredKeys = (contract.requiredNodes ?? []).map((node, index) => validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes[${index}]`, contract.rootDirectoryKindId)).filter((key) => key !== null);
      if (new Set(requiredKeys).size !== requiredKeys.length)
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].requiredNodes must be unique.`);
      if (!Array.isArray(contract.exclusiveAlternatives))
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives must be an array.`);
      const alternativeIds = new Set;
      const alternativeKeys = [];
      for (const [groupIndex, alternative] of (contract.exclusiveAlternatives ?? []).entries()) {
        exactKeys(alternative, ["id", "mode", "nodes"], `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}]`);
        kebabId(alternative.id, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].id`);
        if (alternativeIds.has(alternative.id))
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] has duplicate alternative id ${JSON.stringify(alternative.id)}.`);
        alternativeIds.add(alternative.id);
        if (alternative.mode !== "exactly-one")
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].mode must be exactly-one.`);
        if (!Array.isArray(alternative.nodes) || alternative.nodes.length < 2)
          problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes must contain at least two nodes.`);
        for (const [nodeIndex, node] of (alternative.nodes ?? []).entries()) {
          const nodeKey = validateDescendantNode(node, `semanticDescendantContracts[${JSON.stringify(id)}].exclusiveAlternatives[${groupIndex}].nodes[${nodeIndex}]`, contract.rootDirectoryKindId);
          if (nodeKey)
            alternativeKeys.push(nodeKey);
        }
      }
      const allKeys = [...requiredKeys, ...alternativeKeys];
      if (new Set(allKeys).size !== allKeys.length)
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}] descendant nodes must not overlap.`);
      const requiredRealizedKeys = new Set(requiredKeys);
      for (const key of requiredKeys)
        if (key.startsWith("file:")) {
          const segments = key.slice(5).split("/");
          segments.pop();
          let directory = "";
          for (const segment of segments) {
            directory = directory ? `${directory}/${segment}` : segment;
            requiredRealizedKeys.add(`directory:${directory}`);
          }
        }
      if (contract.realizedNodeCount !== requiredRealizedKeys.size + (contract.exclusiveAlternatives?.length ?? 0))
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].realizedNodeCount must equal realized required destination nodes plus one node per exclusive group.`);
      exactKeys(contract.pathBudgetReserve, ["derivation", "bytes"], `semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve`);
      if (contract.pathBudgetReserve.derivation !== "longest-canonical-descendant-suffix")
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.derivation is invalid.`);
      const derivedBytes = Math.max(0, ...allKeys.map((key) => new TextEncoder().encode(`/${key.slice(key.indexOf(":") + 1)}`).length));
      if (contract.pathBudgetReserve.bytes !== derivedBytes)
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve.bytes must equal derived longest suffix ${derivedBytes}.`);
      if (contract.pathBudgetReserve.bytes >= taxonomy.collisionPolicy.maxPathBytes)
        problems.push(`semanticDescendantContracts[${JSON.stringify(id)}].pathBudgetReserve must be below maxPathBytes.`);
      if (id === "mutation-scenario-bundle-v1") {
        const expectedRequired = ["directory:", "file:\uD83E\uDD80\uFE0F.rs", "directory:\uD83E\uDDA0\uFE0Fmutation", "file:\uD83E\uDDA0\uFE0Fmutation/\uD83D\uDD23\uFE0F.json", "directory:\uD83D\uDCF8\uFE0Fsnapshot", "directory:\uD83D\uDCF8\uFE0Fsnapshot/\u2B05\uFE0Fbefore", "file:\uD83D\uDCF8\uFE0Fsnapshot/\u2B05\uFE0Fbefore/\uD83D\uDD23\uFE0F.json", "directory:\uD83D\uDCF8\uFE0Fsnapshot/\u27A1\uFE0Fafter", "file:\uD83D\uDCF8\uFE0Fsnapshot/\u27A1\uFE0Fafter/\uD83D\uDD23\uFE0F.json", "directory:\uD83D\uDD3A\uFE0Fdiff", "directory:\uD83C\uDFAF\uFE0Foutcome", "file:\uD83C\uDFAF\uFE0Foutcome/\uD83D\uDD23\uFE0F.json"].sort();
        const expectedAlternatives = ["file:\uD83D\uDD3A\uFE0Fdiff/\uD83D\uDD23\uFE0F.json", "file:\uD83D\uDD3A\uFE0Fdiff/\uD83D\uDEAB\uFE0F.absent"].sort();
        if ([...requiredKeys].sort().join("\x00") !== expectedRequired.join("\x00") || [...alternativeKeys].sort().join("\x00") !== expectedAlternatives.join("\x00"))
          problems.push("semanticDescendantContracts.mutation-scenario-bundle-v1 must encode the exact 13-node physical bundle and exclusive diff alternatives.");
      }
      if (id === "draw-editor-command-bundle-v1") {
        const expectedRequired = ["directory:", "file:\uD83E\uDD80\uFE0F.rs", "directory:\uD83D\uDD04\uFE0Ffsm", "file:\uD83D\uDD04\uFE0Ffsm/\uD83E\uDD80\uFE0F.rs", "directory:\uD83D\uDD04\uFE0Ffsm/\uD83D\uDCE6\uFE0Fpackages", "directory:\uD83D\uDD04\uFE0Ffsm/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust", "file:\uD83D\uDD04\uFE0Ffsm/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/Cargo.toml", "file:\uD83D\uDD04\uFE0Ffsm/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/\uD83D\uDCCB\uFE0Fproject.json", "file:\uD83D\uDD04\uFE0Ffsm/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/\uD83D\uDCDC\uFE0Fscript.ts", "file:\uD83D\uDD04\uFE0Ffsm/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/\uD83D\uDCDA\uFE0Flibrary/\uD83E\uDD80\uFE0F.rs", "directory:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros", "file:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros/\uD83E\uDD80\uFE0F.rs", "directory:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros/\uD83D\uDCE6\uFE0Fpackages", "directory:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust", "file:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/Cargo.toml", "file:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/\uD83D\uDCCB\uFE0Fproject.json", "file:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/\uD83D\uDCDC\uFE0Fscript.ts", "file:\uD83D\uDD04\uFE0Ffsm/\u2728\uFE0Fmacros/\uD83D\uDCE6\uFE0Fpackages/\uD83E\uDD80\uFE0Frust/\uD83D\uDCDA\uFE0Flibrary/\uD83E\uDD80\uFE0F.rs"].sort();
        if ([...requiredKeys].sort().join("\x00") !== expectedRequired.join("\x00") || alternativeKeys.length !== 0 || contract.realizedNodeCount !== 20 || contract.pathBudgetReserve.bytes !== 78)
          problems.push("semanticDescendantContracts.draw-editor-command-bundle-v1 must encode the exact 20-node configurable-entry bundle and 78-byte reserve.");
      }
    }
  if (record(taxonomy.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts"))
    for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionCatalogContracts)) {
      kebabId(id, `semanticPathProjectionCatalogContracts id ${JSON.stringify(id)}`);
      if ("contractKind" in contract && contract.contractKind === "distributed-json-manifest-catalog") {
        exactKeys(contract, ["contractKind", "ownerArtifactMemberName", "modelManifestSchema", "modelManifestSourceFilename", "modelIdentityField", "memberIdentityField", "memberVersionField", "requiredMemberVersion", "requiredModelManifest", "categoryRules", "coverage", "unknownCategoryPolicy", "unownedModelPolicy"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
        if (!contract.ownerArtifactMemberName || !contract.modelManifestSchema || !contract.modelManifestSourceFilename || contract.modelIdentityField !== "id" || contract.memberIdentityField !== "id" || contract.memberVersionField !== "version" || !contract.requiredMemberVersion || contract.requiredModelManifest !== true)
          problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must declare exact non-empty manifest authority fields.`);
        if (!Array.isArray(contract.categoryRules) || contract.categoryRules.length === 0)
          problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules must be non-empty.`);
        const categoryNames = new Set;
        for (const [index, rule] of (contract.categoryRules ?? []).entries()) {
          const scope = `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules[${index}]`;
          exactKeys(rule, rule.sourceShape === "direct-semantic-json" ? ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "memberDirectoryEmoji"] : ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "fixedSourceFilename"], scope);
          if (categoryNames.has(rule.sourceDirectoryName))
            problems.push(`${scope}.sourceDirectoryName is duplicated.`);
          categoryNames.add(rule.sourceDirectoryName);
          if (!taxonomy.semanticDirectoryKinds[rule.directoryKindId])
            problems.push(`${scope}.directoryKindId is missing.`);
          if (!rule.manifestSchema)
            problems.push(`${scope}.manifestSchema must be non-empty.`);
          if (rule.sourceShape === "direct-semantic-json") {
            if (!/^\p{Extended_Pictographic}\uFE0F$/u.test(rule.memberDirectoryEmoji))
              problems.push(`${scope}.memberDirectoryEmoji must be one emoji plus U+FE0F.`);
          } else if (rule.sourceShape === "nested-fixed-json") {
            if (!rule.fixedSourceFilename)
              problems.push(`${scope}.fixedSourceFilename must be non-empty.`);
          } else
            problems.push(`${scope}.sourceShape is invalid.`);
        }
        if (contract.coverage !== "every-source-file-and-destination-node-exactly-once" || contract.unknownCategoryPolicy !== "problem" || contract.unownedModelPolicy !== "problem")
          problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must fail closed over exact source and destination ownership.`);
        continue;
      }
      if ("contractKind" in contract && contract.contractKind === "exact-owner-vectors") {
        exactKeys(contract, ["contractKind", "required", "allowEmpty", "identityFields", "coverage", "vectors"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
        if (contract.required !== true || contract.allowEmpty !== false || contract.identityFields.join("\x00") !== "artifactId\x00standardVersion\x00subsetId\x00commandDirectoryName" || contract.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(contract.vectors) || contract.vectors.length === 0)
          problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact non-empty command owner-vector contract.`);
        const owners = new Set;
        for (const [index, vector] of (contract.vectors ?? []).entries()) {
          exactKeys(vector, ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}]`);
          const owner = [vector.artifactId, vector.standardVersion, vector.subsetId, vector.commandDirectoryName].join("\x00");
          if (!vector.artifactId || !vector.standardVersion || !vector.subsetId || !vector.commandDirectoryName || owners.has(owner))
            problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}] must be one unique non-empty owner tuple.`);
          owners.add(owner);
        }
        continue;
      }
      exactKeys(contract, ["registryField", "required", "allowEmpty", "runtimeKindsField", "runtimeKindsRelation", "mutationIdField", "sourceMutationDirectoryNameField", "mutationDirectoryNameField", "scenariosField", "scenarioIdField", "scenarioDirectoryNameField", "sourceBundleUniquenessFields", "canonicalBundleUniquenessFields", "coverage"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      const expected = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
      if (JSON.stringify(contract) !== JSON.stringify(expected))
        problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact independent physical vectors contract.`);
    }
  if (record(taxonomy.semanticPathProjectionContracts, "semanticPathProjectionContracts"))
    for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionContracts)) {
      kebabId(id, `semanticPathProjectionContracts id ${JSON.stringify(id)}`);
      exactKeys(contract, ["sourceOwnerKindId", ...contract.sourceArtifactMemberName === undefined ? [] : ["sourceArtifactMemberName"], "sourceSegments", "profileRendererId", "destinationOwnerKindId", "destinationSegments", "descendantContractId", "catalogContractId", "rationaleRule"], `semanticPathProjectionContracts[${JSON.stringify(id)}]`);
      const artifactProjection = contract.rationaleRule !== "artifact-mutation-test-projection-v1";
      if (artifactProjection !== (typeof contract.sourceArtifactMemberName === "string") || artifactProjection && !taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId]?.memberNames.includes(contract.sourceArtifactMemberName))
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceArtifactMemberName must be one exact source-owner member only for artifact projections.`);
      if (!taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId])
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceOwnerKindId is missing.`);
      if (!taxonomy.semanticDirectoryMemberKinds[contract.destinationOwnerKindId])
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationOwnerKindId is missing.`);
      if (!taxonomy.semanticPathProjectionProfileRenderers[contract.profileRendererId])
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].profileRendererId is missing.`);
      if (!taxonomy.semanticDescendantContracts[contract.descendantContractId])
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].descendantContractId is missing.`);
      if (!taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId])
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].catalogContractId is missing.`);
      if (!["artifact-mutation-test-projection-v1", "artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(contract.rationaleRule))
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].rationaleRule is invalid.`);
      const captures = new Set;
      let sourceParentKindId;
      for (const [index, segment] of contract.sourceSegments.entries()) {
        const value = segment;
        const hasProjected = typeof value.projectedMemberKindId === "string";
        const hasMember = typeof value.memberKindId === "string";
        exactKeys(value, hasProjected ? ["projectedMemberKindId", "capture"] : hasMember ? ["memberKindId", "literal"] : ["kindId", "literal" in value ? "literal" : "capture"], `semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}]`);
        const kindId = hasProjected ? value.projectedMemberKindId : hasMember ? value.memberKindId : value.kindId;
        const kind = hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : hasMember ? taxonomy.semanticDirectoryMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId];
        if (!kind)
          problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}] references a missing kind.`);
        if (typeof value.literal === "string") {
          const validLiteral = hasMember ? Boolean(taxonomy.semanticDirectoryMemberKinds[kindId]?.ownerKindIds.includes(sourceParentKindId ?? "") && taxonomy.semanticDirectoryMemberKinds[kindId]?.memberNames.includes(value.literal)) : semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: sourceParentKindId }) === kindId;
          if (!validLiteral)
            problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].literal does not resolve to its kind.`);
        }
        if (typeof value.capture === "string") {
          if (!["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"].includes(value.capture))
            problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].capture is invalid.`);
          if (captures.has(value.capture))
            problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] captures ${JSON.stringify(value.capture)} more than once.`);
          captures.add(value.capture);
        }
        sourceParentKindId = kindId;
      }
      const requiredCaptures = contract.rationaleRule === "artifact-mutation-test-projection-v1" ? ["standardVersion", "subsetId", "mutationId", "scenarioId"] : contract.rationaleRule === "artifact-example-model-catalog-projection-v1" ? ["standardVersion", "subsetId"] : ["standardVersion", "subsetId", "commandDirectoryName"];
      if (captures.size !== requiredCaptures.length || requiredCaptures.some((field) => !captures.has(field)))
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must capture exactly ${requiredCaptures.join(", ")}.`);
      let renderedProfiles = 0;
      let destinationParentKindId;
      for (const [index, segment] of contract.destinationSegments.entries()) {
        const value = segment;
        const hasProjected = typeof value.projectedMemberKindId === "string";
        const operation = "literal" in value ? "literal" : ("render" in value) ? "render" : "copy";
        exactKeys(value, [hasProjected ? "projectedMemberKindId" : "kindId", operation], `semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}]`);
        const kindId = hasProjected ? value.projectedMemberKindId : value.kindId;
        if (!(hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId]))
          problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}] references a missing kind.`);
        if (typeof value.literal === "string" && semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: destinationParentKindId }) !== kindId)
          problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].literal does not resolve to its kind.`);
        if (value.render !== undefined) {
          if (value.render !== "profile")
            problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].render must be profile.`);
          renderedProfiles += 1;
        }
        if (typeof value.copy === "string" && !captures.has(value.copy))
          problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].copy must reference a source capture.`);
        destinationParentKindId = kindId;
      }
      if (renderedProfiles !== 1)
        problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must render exactly one profile segment.`);
      const projectedReferences = [...contract.sourceSegments, ...contract.destinationSegments].flatMap((segment) => ("projectedMemberKindId" in segment) ? [segment.projectedMemberKindId] : []);
      for (const [projectedId, projected] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
        const references = projectedReferences.filter((candidate) => candidate === projectedId).length;
        if (projected.projectionContractId === id && references !== 2)
          problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must reference projected member ${JSON.stringify(projectedId)} exactly once in source and destination.`);
      }
    }
  if (record(taxonomy.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts")) {
    const ownerTuples = new Set;
    for (const [id, contract] of Object.entries(taxonomy.semanticOwnedFileProjectionContracts)) {
      const scope = `semanticOwnedFileProjectionContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      if (!contract || typeof contract !== "object" || Array.isArray(contract)) {
        problems.push(`${scope} must be an object.`);
        continue;
      }
      exactKeys(contract, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], scope);
      if (contract.contractKind !== "owner-sibling-manifest-file" || contract.manifestAdapter !== "json" || contract.manifestStatusLocation !== "status" || contract.emptyContentRule !== "zero-byte" || contract.rationaleRule !== "ticket-important-markdown-projection-v1")
        problems.push(`${scope} must use the exact ticket important owner-file grammar.`);
      if (!Array.isArray(contract.allowedStatuses) || contract.allowedStatuses.join("\x00") !== "closed\x00open")
        problems.push(`${scope}.allowedStatuses must be exactly closed, open.`);
      if (contract.statusDispositions && typeof contract.statusDispositions === "object" && !Array.isArray(contract.statusDispositions))
        exactKeys(contract.statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${scope}.statusDispositions`);
      else
        problems.push(`${scope}.statusDispositions must be an object.`);
      if (JSON.stringify(contract.statusDispositions) !== JSON.stringify({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }))
        problems.push(`${scope}.statusDispositions must be the exact lifecycle mapping.`);
      if (!taxonomy.fixedDirectoryContracts[contract.ownerFixedDirectoryContractId])
        problems.push(`${scope}.ownerFixedDirectoryContractId is missing.`);
      if (!taxonomy.fixedFilenameContracts[contract.requiredSiblingFixedFilenameContractId])
        problems.push(`${scope}.requiredSiblingFixedFilenameContractId is missing.`);
      if (typeof contract.sourceFilename !== "string" || !taxonomy.fileKinds[contract.sourceFileKindId] || fileKindIdForSourcePath(contract.sourceFilename, taxonomy) !== contract.sourceFileKindId)
        problems.push(`${scope}.sourceFilename must resolve to sourceFileKindId.`);
      const destinationKind = taxonomy.semanticDirectoryKinds[contract.destinationDirectoryKindId];
      if (!destinationKind || destinationKind.projectionOnly !== true || typeof contract.destinationDirectoryName !== "string" || semanticDirectoryKindId(contract.destinationDirectoryName, taxonomy) !== contract.destinationDirectoryKindId)
        problems.push(`${scope}.destination directory must resolve to one projectionOnly kind.`);
      if (contract.destinationFilename !== "\uD83D\uDCDD\uFE0F.md" || typeof contract.destinationFilename !== "string" || fileKindIdForSourcePath(contract.destinationFilename, taxonomy) !== contract.sourceFileKindId)
        problems.push(`${scope}.destinationFilename must be the exact Markdown physical leaf.`);
      for (const [field, value] of [["sourceFilename", contract.sourceFilename], ["destinationDirectoryName", contract.destinationDirectoryName], ["destinationFilename", contract.destinationFilename]])
        if (typeof value !== "string" || !value || value !== value.normalize("NFC") || /[\\/]/u.test(value))
          problems.push(`${scope}.${field} must be one non-empty NFC name.`);
      const tuple = [contract.ownerFixedDirectoryContractId, contract.requiredSiblingFixedFilenameContractId, contract.sourceFilename].join("\x00");
      if (ownerTuples.has(tuple))
        problems.push(`${scope} overlaps another owned-file projection contract.`);
      ownerTuples.add(tuple);
    }
    if (Object.keys(taxonomy.semanticOwnedFileProjectionContracts).join("\x00") !== "ticket-important-markdown-v1")
      problems.push("semanticOwnedFileProjectionContracts must contain only ticket-important-markdown-v1.");
  }
  if (record(taxonomy.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts")) {
    const identities = new Set;
    const patterns = new Set;
    const adapters = ["rust", "typescript", "json", "toml"];
    const forms = ["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:interaction-glob", "artifact-catalog-prose:catalog-grammar"];
    for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionReferenceConsumerContracts)) {
      const scope = `semanticPathProjectionReferenceConsumerContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      exactKeys(contract, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], scope);
      const projection = taxonomy.semanticPathProjectionContracts[contract.projectionContractId];
      if (!projection || !["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(projection.rationaleRule))
        problems.push(`${scope}.projectionContractId must reference one CAD or Draw artifact projection.`);
      if (contract.consumerIdentity !== id || identities.has(contract.consumerIdentity))
        problems.push(`${scope}.consumerIdentity must equal its unique registry id.`);
      identities.add(contract.consumerIdentity);
      if (contract.ownership !== "external")
        problems.push(`${scope}.ownership must be external.`);
      if (!contract.sourcePathPattern.startsWith("^") || !contract.sourcePathPattern.endsWith("$") || contract.sourcePathPattern !== contract.sourcePathPattern.normalize("NFC") || /\uFE0E/u.test(contract.sourcePathPattern))
        problems.push(`${scope}.sourcePathPattern must be one anchored NFC regex without VS15.`);
      try {
        const expression = new RegExp(contract.sourcePathPattern, "u");
        if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0 || contract.sourcePathIdentities.some((path) => !path || path !== path.normalize("NFC") || /\uFE0E/u.test(path) || !expression.test(path)) || new Set(contract.sourcePathIdentities).size !== contract.sourcePathIdentities.length)
          problems.push(`${scope}.sourcePathIdentities must be unique exact NFC paths admitted by sourcePathPattern.`);
      } catch {
        problems.push(`${scope}.sourcePathPattern must compile with Unicode semantics.`);
      }
      const patternKey = `${contract.projectionContractId}\x00${contract.sourcePathPattern}`;
      if (patterns.has(patternKey))
        problems.push(`${scope}.sourcePathPattern is duplicated for its projection.`);
      patterns.add(patternKey);
      if (!Array.isArray(contract.adapters) || contract.adapters.length === 0 || contract.adapters.some((adapter) => !adapters.includes(adapter)) || new Set(contract.adapters).size !== contract.adapters.length)
        problems.push(`${scope}.adapters must be unique supported reference adapters.`);
      if (!Array.isArray(contract.supportedForms) || contract.supportedForms.length === 0 || contract.supportedForms.some((form) => !forms.includes(form)) || new Set(contract.supportedForms).size !== contract.supportedForms.length)
        problems.push(`${scope}.supportedForms must be unique supported structural forms.`);
      if (!Array.isArray(contract.staleMarkers) || contract.staleMarkers.length === 0 || contract.staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC") || /\uFE0E/u.test(marker)) || new Set(contract.staleMarkers).size !== contract.staleMarkers.length)
        problems.push(`${scope}.staleMarkers must be unique non-empty NFC markers without VS15.`);
      if (contract.supportedForms.some((form) => form.startsWith("artifact-catalog-")) && (projection?.rationaleRule !== "artifact-example-model-catalog-projection-v1" || !contract.adapters.some((adapter) => adapter === "rust" || adapter === "typescript")))
        problems.push(`${scope} artifact-catalog forms require the CAD projection and a Rust or TypeScript adapter.`);
    }
    const required = ["cad-spatial-kernel-geometry", "draw-dependency-registry", "draw-workspace-cargo", "draw-workspace-script"];
    if ([...identities].sort().join("\x00") !== required.join("\x00"))
      problems.push("semanticPathProjectionReferenceConsumerContracts must encode the four exact current external CAD/Draw consumers.");
    const rows = Object.values(taxonomy.semanticPathProjectionReferenceConsumerContracts);
    for (const [index, left] of rows.entries())
      for (const right of rows.slice(index + 1)) {
        if (left.projectionContractId !== right.projectionContractId || !left.supportedForms.some((form) => right.supportedForms.includes(form)))
          continue;
        const leftPattern = new RegExp(left.sourcePathPattern, "u");
        const rightPattern = new RegExp(right.sourcePathPattern, "u");
        if (left.sourcePathIdentities.some((path) => rightPattern.test(path)) || right.sourcePathIdentities.some((path) => leftPattern.test(path)))
          problems.push(`semanticPathProjectionReferenceConsumerContracts ${JSON.stringify(left.consumerIdentity)} and ${JSON.stringify(right.consumerIdentity)} overlap for one supported form.`);
      }
  }
  const projectionIds = taxonomy.mutationCatalogProjection;
  if (!projectionIds || typeof projectionIds !== "object")
    problems.push("mutationCatalogProjection must be an object.");
  else {
    exactKeys(projectionIds, ["projectionContractId", "projectedMemberKindId", "descendantContractId", "catalogContractId"], "mutationCatalogProjection");
    const projected = taxonomy.semanticProjectedMemberKinds[projectionIds.projectedMemberKindId];
    const projection = taxonomy.semanticPathProjectionContracts[projectionIds.projectionContractId];
    if (!projected || projected.projectionContractId !== projectionIds.projectionContractId)
      problems.push("mutationCatalogProjection projected member and projection IDs do not agree.");
    if (!projection || projection.descendantContractId !== projectionIds.descendantContractId || projection.catalogContractId !== projectionIds.catalogContractId)
      problems.push("mutationCatalogProjection contract IDs do not agree with its projection.");
  }
  const exactProjection = taxonomy.semanticPathProjectionContracts["artifact-mutation-tests-v1"];
  const exactSource = [{ kindId: "standards", literal: "\uD83C\uDFC5\uFE0Fstandards" }, { kindId: "standard", capture: "standardVersion" }, { kindId: "subsets", literal: "\uD83E\uDE86\uFE0Fsubsets" }, { kindId: "subset", capture: "subsetId" }, { kindId: "schema", literal: "\uD83E\uDDEC\uFE0Fschema" }, { kindId: "schema", literal: "\uD83E\uDDEC\uFE0Fmutations" }, { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" }, { kindId: "tests", literal: "\uD83E\uDDEA\uFE0Ftests" }, { kindId: "test-case", capture: "scenarioId" }];
  const exactDestination = [{ kindId: "tests", literal: "\uD83E\uDDEA\uFE0Ftests" }, { kindId: "mutation-test-profile", render: "profile" }, { projectedMemberKindId: "mutation-test-subject", copy: "mutationId" }, { kindId: "test-case", copy: "scenarioId" }];
  if (!exactProjection || JSON.stringify(exactProjection.sourceSegments) !== JSON.stringify(exactSource) || JSON.stringify(exactProjection.destinationSegments) !== JSON.stringify(exactDestination))
    problems.push("semanticPathProjectionContracts.artifact-mutation-tests-v1 must encode the exact source and destination path grammar.");
  const exactBundle = taxonomy.semanticDescendantContracts["mutation-scenario-bundle-v1"];
  if (!exactBundle || "contractKind" in exactBundle || exactBundle.realizedNodeCount !== 13 || exactBundle.exclusiveAlternatives.length !== 1 || exactBundle.exclusiveAlternatives[0]?.id !== "diff-leaf" || exactBundle.pathBudgetReserve.bytes !== 42)
    problems.push("semanticDescendantContracts.mutation-scenario-bundle-v1 must encode 13 nodes, one diff alternative, and the derived 42-byte reserve.");
  const fixedScope = (scope, contractPathPattern, key, filenameContract) => {
    if (!(typeof scope === "object" && scope !== null && !Array.isArray(scope)) || typeof scope.kind !== "string") {
      problems.push(`${key} must be a tagged fixed-contract scope.`);
      return;
    }
    if (scope.kind === "exact-path") {
      exactKeys(scope, ["kind", "path"], key);
      pathPattern(scope.path, `${key}.path`);
      if (scope.path !== contractPathPattern || /[*?\[]/u.test(scope.path))
        problems.push(`${key}.path must equal the exact wildcard-free contract path.`);
    } else if (scope.kind === "repository-root") {
      exactKeys(scope, ["kind"], key);
      if (contractPathPattern.includes("/") || /[*?\[]/u.test(contractPathPattern))
        problems.push(`${key} repository-root contract must be one exact basename.`);
    } else if (scope.kind === "package-root") {
      exactKeys(scope, ["kind", "ecosystemId"], key);
      if (!filenameContract)
        problems.push(`${key} cannot use package-root scope.`);
      if (!scope.ecosystemId || !taxonomy.ecosystems[scope.ecosystemId])
        problems.push(`${key}.ecosystemId must reference an ecosystem.`);
    } else if (scope.kind === "directory-kind") {
      exactKeys(scope, ["kind", "directoryKindId"], key);
      if (!scope.directoryKindId || !taxonomy.semanticDirectoryKinds[scope.directoryKindId])
        problems.push(`${key}.directoryKindId must reference a semantic directory kind.`);
    } else if (scope.kind === "fixed-directory-contract") {
      exactKeys(scope, ["kind", "fixedDirectoryContractId"], key);
      if (!filenameContract)
        problems.push(`${key} cannot use fixed-directory-contract scope.`);
      if (!scope.fixedDirectoryContractId || !taxonomy.fixedDirectoryContracts[scope.fixedDirectoryContractId])
        problems.push(`${key}.fixedDirectoryContractId must reference a fixed directory contract.`);
    } else if (scope.kind === "sibling-fixed-filename-contract") {
      exactKeys(scope, ["kind", "fixedFilenameContractId"], key);
      if (!filenameContract)
        problems.push(`${key} cannot use sibling-fixed-filename-contract scope.`);
      if (!scope.fixedFilenameContractId || !taxonomy.fixedFilenameContracts[scope.fixedFilenameContractId])
        problems.push(`${key}.fixedFilenameContractId must reference a fixed filename contract.`);
    } else if (scope.kind === "path-pattern")
      exactKeys(scope, ["kind"], key);
    else
      problems.push(`${key}.kind is invalid.`);
  };
  if (record(taxonomy.fixedFilenameContracts, "fixedFilenameContracts"))
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
      pathPattern(contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].pathPattern`);
      if (typeof contract.pathPattern === "string" && /[*?\[]/u.test(fixedContractFilename(contract)))
        problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].pathPattern must end in one exact literal basename.`);
      if (!contract.authority || !contract.reason || !contract.verification)
        problems.push(`fixedFilenameContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
      if (contract.configurability !== "unconfigurable")
        problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
      fixedScope(contract.scope, contract.pathPattern, `fixedFilenameContracts[${JSON.stringify(id)}].scope`, true);
      if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires)))
        problems.push(`fixedFilenameContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
    }
  const cargoTargetEvidence = taxonomy.semanticDirectoryKinds["ticket-cargo-target-evidence"];
  if (!cargoTargetEvidence || cargoTargetEvidence.emoji !== "\uD83E\uDDEA\uFE0F" || cargoTargetEvidence.slugPattern !== "^target-[a-z0-9]+(?:-[a-z0-9]+)*$" || cargoTargetEvidence.allowEmojiOnly || cargoTargetEvidence.parentKindIds !== undefined)
    problems.push("semanticDirectoryKinds.ticket-cargo-target-evidence must remain the exact ticket-local Cargo target authority.");
  const cargoCacheTag = taxonomy.fixedFilenameContracts["cargo-cache-tag"];
  if (!cargoCacheTag || cargoCacheTag.pathPattern !== "**/.\uD83E\uDDECsemio/\uD83E\uDD91\uFE0Frepo/\uD83C\uDFAB\uFE0Ftickets/\uD83C\uDF86\uFE0F[0-9][0-9]/\uD83C\uDF19\uFE0F[0-9][0-9]/\u2600\uFE0F[0-9][0-9]/*/**/CACHEDIR.TAG" || cargoCacheTag.authority !== "Cargo" || cargoCacheTag.scope.kind !== "directory-kind" || cargoCacheTag.scope.directoryKindId !== "ticket-cargo-target-evidence")
    problems.push("fixedFilenameContracts.cargo-cache-tag must remain conjunctively scoped to a governed ticket path and the ticket-cargo-target-evidence directory kind.");
  const ticketCargoPattern = "**/.\uD83E\uDDECsemio/\uD83E\uDD91\uFE0Frepo/\uD83C\uDFAB\uFE0Ftickets/\uD83C\uDF86\uFE0F[0-9][0-9]/\uD83C\uDF19\uFE0F[0-9][0-9]/\u2600\uFE0F[0-9][0-9]/*/**/";
  for (const triple of ["wasm32-unknown-unknown", "wasm32-wasip2"]) {
    const directoryId = `cargo-target-triple-${triple}`;
    const directory = taxonomy.fixedDirectoryContracts[directoryId];
    if (!directory || directory.pathPattern !== `${ticketCargoPattern}${triple}` || directory.authority !== "Cargo" || directory.scope.kind !== "directory-kind" || directory.scope.directoryKindId !== "ticket-cargo-target-evidence")
      problems.push(`fixedDirectoryContracts.${directoryId} must remain the exact governed ticket Cargo target-triple authority.`);
    const cacheId = `cargo-cache-tag-${triple}`;
    const cache = taxonomy.fixedFilenameContracts[cacheId];
    if (!cache || cache.pathPattern !== `${ticketCargoPattern}${triple}/CACHEDIR.TAG` || cache.authority !== "Cargo" || cache.scope.kind !== "fixed-directory-contract" || cache.scope.fixedDirectoryContractId !== directoryId)
      problems.push(`fixedFilenameContracts.${cacheId} must remain conjunctively scoped through its exact Cargo target-triple contract.`);
  }
  const nxManifestScopes = [
    ["nx-owned-node-package-manifest", "**/package.json", "Nx and Node.js"],
    ["nx-owned-typescript-config", "**/tsconfig.json", "Nx and TypeScript"]
  ];
  for (const [id, pattern2, authority] of nxManifestScopes) {
    const contract = taxonomy.fixedFilenameContracts[id];
    if (!contract || contract.pathPattern !== pattern2 || contract.authority !== authority || contract.scope.kind !== "sibling-fixed-filename-contract" || contract.scope.fixedFilenameContractId !== "nx-project-manifest")
      problems.push(`fixedFilenameContracts.${id} must remain conjunctively scoped through an adjacent exact Nx project manifest.`);
  }
  const ticketCargoManifest = taxonomy.fixedFilenameContracts["ticket-cargo-manifest"];
  if (!ticketCargoManifest || ticketCargoManifest.pathPattern !== `${ticketCargoPattern}Cargo.toml` || ticketCargoManifest.authority !== "Cargo" || ticketCargoManifest.scope.kind !== "path-pattern")
    problems.push("fixedFilenameContracts.ticket-cargo-manifest must remain scoped to governed canonical or embedded ticket paths.");
  const ticketCargoLock = taxonomy.fixedFilenameContracts["ticket-cargo-lock"];
  if (!ticketCargoLock || ticketCargoLock.pathPattern !== `${ticketCargoPattern}Cargo.lock` || ticketCargoLock.authority !== "Cargo" || ticketCargoLock.scope.kind !== "path-pattern")
    problems.push("fixedFilenameContracts.ticket-cargo-lock must remain scoped to governed canonical or embedded ticket paths.");
  const rootCargoLock = taxonomy.fixedFilenameContracts["root-cargo-lock"];
  if (!rootCargoLock || rootCargoLock.pathPattern !== "Cargo.lock" || rootCargoLock.authority !== "Cargo" || rootCargoLock.scope.kind !== "repository-root")
    problems.push("fixedFilenameContracts.root-cargo-lock must remain the exact repository-root Cargo lock authority.");
  if (record(taxonomy.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts")) {
    const identities = new Map;
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameRejectionContracts)) {
      exactKeys(contract, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts[${JSON.stringify(id)}]`);
      if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0)
        problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities must be non-empty.`);
      for (const identity of contract.sourcePathIdentities ?? []) {
        pathPattern(identity, `fixedFilenameRejectionContracts[${JSON.stringify(id)}].sourcePathIdentities`);
        if (/[*?\[]/u.test(identity))
          problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}] identities must be exact paths.`);
        if (identities.has(identity))
          problems.push(`Fixed filename rejection identity ${JSON.stringify(identity)} is duplicated by ${JSON.stringify(identities.get(identity))} and ${JSON.stringify(id)}.`);
        identities.set(identity, id);
      }
      if (!["normalize", "relocate"].includes(contract.disposition))
        problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].disposition is invalid.`);
      if (!contract.reason)
        problems.push(`fixedFilenameRejectionContracts[${JSON.stringify(id)}].reason must be non-empty.`);
    }
  }
  if (record(taxonomy.fixedDirectoryContracts, "fixedDirectoryContracts"))
    for (const [id, contract] of Object.entries(taxonomy.fixedDirectoryContracts)) {
      pathPattern(contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].pathPattern`);
      if (!contract.authority || !contract.reason || !contract.verification)
        problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
      if (contract.configurability !== "unconfigurable")
        problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].configurability must be unconfigurable.`);
      fixedScope(contract.scope, contract.pathPattern, `fixedDirectoryContracts[${JSON.stringify(id)}].scope`, false);
      if (!(contract.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(contract.expires)))
        problems.push(`fixedDirectoryContracts[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
    }
  if (record(taxonomy.configurableEntryContracts, "configurableEntryContracts"))
    for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts)) {
      if (!taxonomy.fileKinds[contract.fileKindId])
        problems.push(`configurableEntryContracts[${JSON.stringify(id)}].fileKindId is missing.`);
      else if (!canonicalFilenamesForKind(contract.fileKindId, taxonomy).includes(contract.filename))
        problems.push(`configurableEntryContracts[${JSON.stringify(id)}].filename is not canonical for its file kind.`);
      if (!taxonomy.ecosystems[contract.ecosystemId])
        problems.push(`configurableEntryContracts[${JSON.stringify(id)}].ecosystemId is missing.`);
      if (!Array.isArray(contract.configurationSources) || contract.configurationSources.length === 0)
        problems.push(`configurableEntryContracts[${JSON.stringify(id)}].configurationSources must be non-empty.`);
    }
  if (record(taxonomy.packageGlueGrammar, "packageGlueGrammar"))
    for (const [id, grammar] of Object.entries(taxonomy.packageGlueGrammar)) {
      if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp"].includes(grammar.analyzer))
        problems.push(`packageGlueGrammar[${JSON.stringify(id)}].analyzer is invalid.`);
      if (!Number.isSafeInteger(grammar.maxDelegationStatements) || grammar.maxDelegationStatements < 0)
        problems.push(`packageGlueGrammar[${JSON.stringify(id)}].maxDelegationStatements is invalid.`);
      if (!Array.isArray(grammar.allowedRoles) || grammar.allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role)))
        problems.push(`packageGlueGrammar[${JSON.stringify(id)}].allowedRoles is invalid.`);
    }
  if (record(taxonomy.packageBoundaryRules, "packageBoundaryRules"))
    for (const [ecosystemId, rule] of Object.entries(taxonomy.packageBoundaryRules)) {
      if (!taxonomy.ecosystems[ecosystemId])
        problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] has no ecosystem.`);
      if (rule.manifestContractId && !taxonomy.fixedFilenameContracts[rule.manifestContractId])
        problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].manifestContractId is missing.`);
      ids(rule.entryContractIds, taxonomy.configurableEntryContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].entryContractIds`);
      ids(rule.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFixedContractIds`);
      ids(rule.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFileKindIds`);
      ids(rule.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedDirectoryKindIds`);
      if (!taxonomy.packageGlueGrammar[rule.glueGrammarId])
        problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].glueGrammarId is missing.`);
      if (rule.recursive !== true || rule.uncertainRole !== "problem" || rule.implementationRole !== "problem")
        problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] must recursively block uncertain and implementation roles.`);
    }
  if (record(taxonomy.packageBoundaryProfiles, "packageBoundaryProfiles"))
    for (const [id, profile] of Object.entries(taxonomy.packageBoundaryProfiles)) {
      exactKeys(profile, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles[${JSON.stringify(id)}]`);
      if (profile.admission !== "blocked-until-language-directory-registered")
        problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].admission is invalid.`);
      ids(profile.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFileKindIds`);
      ids(profile.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedDirectoryKindIds`);
      ids(profile.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFixedContractIds`);
      if (!taxonomy.packageGlueGrammar[profile.glueGrammarId])
        problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].glueGrammarId is missing.`);
      if (profile.recursive !== true || profile.uncertainRole !== "problem" || profile.implementationRole !== "problem" || !profile.reason)
        problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}] must fail closed with a reason.`);
    }
  if (record(taxonomy.packageSourceDispositions, "packageSourceDispositions")) {
    const expected = new Map;
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
      const kindId = fileKindIdForSourcePath(fixedContractFilename(contract), taxonomy);
      if (kindId && taxonomy.fileKinds[kindId]?.role === "source")
        expected.set(id, "fixed");
    }
    for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts))
      if (taxonomy.fileKinds[contract.fileKindId]?.role === "source")
        expected.set(id, "configurable");
    for (const missing of [...expected.keys()].filter((id) => !taxonomy.packageSourceDispositions[id]))
      problems.push(`packageSourceDispositions is missing source-format contract ${JSON.stringify(missing)}.`);
    for (const [id, disposition] of Object.entries(taxonomy.packageSourceDispositions)) {
      exactKeys(disposition, ["contractKind", "disposition", "validator", "authority", "verification"], `packageSourceDispositions[${JSON.stringify(id)}]`);
      if (!expected.has(id))
        problems.push(`packageSourceDispositions[${JSON.stringify(id)}] does not name a source-format fixed/configurable contract.`);
      else if (expected.get(id) !== disposition.contractKind)
        problems.push(`packageSourceDispositions[${JSON.stringify(id)}].contractKind does not match its registry.`);
      if (!["adapter-source", "tool-metadata"].includes(disposition.disposition))
        problems.push(`packageSourceDispositions[${JSON.stringify(id)}].disposition is invalid.`);
      if (!["package-glue", "command-router"].includes(disposition.validator) || disposition.disposition === "adapter-source" !== (disposition.validator === "package-glue"))
        problems.push(`packageSourceDispositions[${JSON.stringify(id)}] disposition/validator pair is invalid.`);
      if (!disposition.authority || !disposition.verification)
        problems.push(`packageSourceDispositions[${JSON.stringify(id)}] must declare authority and verification.`);
    }
  }
  if (record(taxonomy.generatorContracts, "generatorContracts")) {
    const contractIds = Object.keys(taxonomy.generatorContracts);
    if (contractIds.join("\x00") !== [...contractIds].sort().join("\x00"))
      problems.push("generatorContracts ids must be lexically ordered.");
    const opaqueRoots = Object.values(taxonomy.pathExclusions ?? {}).map((entry) => entry.path.replace(/\/$/u, ""));
    const exactTouchesOpaque = (value) => opaqueRoots.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
    const patternTouchesOpaque = (value) => {
      const first = value.split("/")[0];
      return /[*?\[]/u.test(first) || opaqueRoots.some((opaque) => opaque.split("/")[0] === first);
    };
    const nxTarget = (value, key) => {
      const valid = typeof value === "string" && /^(?:@[a-z0-9][a-z0-9._-]*\/)?[a-z0-9][a-z0-9._-]*:[a-z0-9][a-z0-9._-]*$/u.test(value);
      if (!valid)
        problems.push(`${key} must be one exact Nx project:target identity.`);
      return valid;
    };
    const outputOwners = [];
    const targets = new Map;
    for (const [id, contract] of Object.entries(taxonomy.generatorContracts)) {
      if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id))
        problems.push(`generatorContracts id ${JSON.stringify(id)} must be kebab-case.`);
      const allowedKeys = new Set(["ownership", "ownerPath", "target", "previewTarget", "checkTarget", "inputPatterns", "outputRoots", "reason"]);
      for (const key of Object.keys(contract))
        if (!allowedKeys.has(key))
          problems.push(`generatorContracts[${JSON.stringify(id)}].${key} is forbidden.`);
      const ownership = contract.ownership;
      if (!["owned", "external"].includes(ownership))
        problems.push(`generatorContracts[${JSON.stringify(id)}].ownership must be owned or external.`);
      if (!contract.reason)
        problems.push(`generatorContracts[${JSON.stringify(id)}].reason must be non-empty.`);
      const runnable = ownership === "owned";
      const targetKnown = runnable;
      if (targetKnown) {
        if (workspacePath(contract.ownerPath, `generatorContracts[${JSON.stringify(id)}].ownerPath`) && exactTouchesOpaque(contract.ownerPath))
          problems.push(`generatorContracts[${JSON.stringify(id)}].ownerPath crosses an opaque boundary.`);
        if (nxTarget(contract.target, `generatorContracts[${JSON.stringify(id)}].target`)) {
          const prior = targets.get(contract.target);
          if (prior)
            problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.target)}.`);
          targets.set(contract.target, id);
        }
      } else if (contract.ownerPath !== null || contract.target !== null)
        problems.push(`generatorContracts[${JSON.stringify(id)}] unowned/external classifications must have null ownerPath and target.`);
      if (runnable) {
        if (contract.previewTarget === undefined)
          problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is required for owned contracts.`);
        else if (nxTarget(contract.previewTarget, `generatorContracts[${JSON.stringify(id)}].previewTarget`) && typeof contract.target === "string") {
          const expected = `${contract.target.slice(0, contract.target.lastIndexOf(":"))}:preview-generated`;
          if (contract.previewTarget !== expected)
            problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget must be the exact owner preview-generated target ${JSON.stringify(expected)}.`);
          const prior = targets.get(contract.previewTarget);
          if (prior)
            problems.push(`generatorContracts ${JSON.stringify(prior)} and ${JSON.stringify(id)} duplicate target ${JSON.stringify(contract.previewTarget)}.`);
          targets.set(contract.previewTarget, id);
        }
      } else if (contract.previewTarget !== undefined)
        problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget is forbidden for external contracts.`);
      if (contract.checkTarget !== undefined) {
        if (!targetKnown || !nxTarget(contract.checkTarget, `generatorContracts[${JSON.stringify(id)}].checkTarget`))
          problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget requires a known target.`);
        else if (typeof contract.target === "string" && contract.checkTarget.slice(0, contract.checkTarget.lastIndexOf(":")) !== contract.target.slice(0, contract.target.lastIndexOf(":")))
          problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget must belong to the target project.`);
      }
      if (!Array.isArray(contract.inputPatterns) || runnable && contract.inputPatterns.length === 0 || !runnable && contract.inputPatterns.length > 0)
        problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be non-empty only for owned contracts.`);
      if (Array.isArray(contract.inputPatterns)) {
        if (contract.inputPatterns.join("\x00") !== [...contract.inputPatterns].sort().join("\x00") || new Set(contract.inputPatterns).size !== contract.inputPatterns.length)
          problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns must be unique and lexically ordered.`);
        for (const [index, input] of contract.inputPatterns.entries()) {
          pathPattern(input, `generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}]`);
          if (typeof input === "string" && patternTouchesOpaque(input))
            problems.push(`generatorContracts[${JSON.stringify(id)}].inputPatterns[${index}] can cross an opaque boundary.`);
        }
      }
      if (!Array.isArray(contract.outputRoots) || contract.outputRoots.length === 0)
        problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be non-empty.`);
      if (Array.isArray(contract.outputRoots)) {
        const outputPaths = contract.outputRoots.map((output) => output.path);
        if (outputPaths.join("\x00") !== [...outputPaths].sort().join("\x00") || new Set(outputPaths).size !== outputPaths.length)
          problems.push(`generatorContracts[${JSON.stringify(id)}].outputRoots must be unique and lexically ordered.`);
        for (const [index, output] of contract.outputRoots.entries()) {
          const key = `generatorContracts[${JSON.stringify(id)}].outputRoots[${index}]`;
          if (workspacePath(output.path, `${key}.path`) && exactTouchesOpaque(output.path))
            problems.push(`${key}.path crosses an opaque boundary.`);
          if (!["tracked", "ignored"].includes(output.inclusion))
            problems.push(`${key}.inclusion must be tracked or ignored.`);
          if (runnable && contract.inputPatterns.some((input) => taxonomyPathPatternMatches(output.path, input)))
            problems.push(`${key}.path is also declared as an input.`);
          outputOwners.push({ id, path: output.path });
        }
      }
    }
    for (let left = 0;left < outputOwners.length; left += 1)
      for (let right = left + 1;right < outputOwners.length; right += 1) {
        const a = outputOwners[left];
        const b = outputOwners[right];
        if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`))
          problems.push(`generatorContracts ${JSON.stringify(a.id)} and ${JSON.stringify(b.id)} have overlapping output roots ${JSON.stringify(a.path)} and ${JSON.stringify(b.path)}.`);
      }
    const unsettled = Object.entries(taxonomy.generatorContracts).filter(([, contract]) => !["owned", "external"].includes(contract.ownership)).map(([id]) => id);
    if (unsettled.length > 0)
      problems.push(`generatorContracts must contain zero unknown or unsafe contracts; found ${unsettled.join(", ")}.`);
    for (const removed of ["ownerless-ui-icons", "root-layering-declarations"])
      if (taxonomy.generatorContracts[removed])
        problems.push(`generatorContracts.${removed} is false ownership and must remain absent.`);
    const ralphTrackedPaths = [
      ".ralph-tui/config.toml",
      ".ralph-tui/prd/kit-store-architecture-contracts-first-multi-backbone-pointer-based-rs-core/prd.json",
      ".ralph-tui/prd/kit-store-architecture-contracts-first-multi-backbone-pointer-based-rs-core/prd.md",
      ".ralph-tui/progress.md",
      ".ralph-tui/ralph.lock",
      ".ralph-tui/session-meta.json",
      ".ralph-tui/session.json"
    ];
    const setup = taxonomy.generatorContracts["setup-wizard-config"];
    if (!setup || setup.ownership !== "external" || setup.ownerPath !== null || setup.target !== null || setup.inputPatterns.length !== 0 || setup.outputRoots.some((output) => output.inclusion !== "tracked") || setup.outputRoots.map((output) => output.path).join("\x00") !== ralphTrackedPaths.join("\x00"))
      problems.push("generatorContracts.setup-wizard-config must externally own exactly the seven tracked Ralph files.");
    const ralphFileContracts = {
      "ralph-config": ".ralph-tui/config.toml",
      "ralph-lock": ".ralph-tui/ralph.lock",
      "ralph-prd-json": ".ralph-tui/prd/*/prd.json",
      "ralph-prd-markdown": ".ralph-tui/prd/*/prd.md",
      "ralph-progress": ".ralph-tui/progress.md",
      "ralph-session-meta": ".ralph-tui/session-meta.json",
      "ralph-session": ".ralph-tui/session.json"
    };
    for (const [id, expected] of Object.entries(ralphFileContracts)) {
      const contract = taxonomy.fixedFilenameContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI" || contract.scope.kind !== "path-pattern")
        problems.push(`fixedFilenameContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    const ralphDirectoryContracts = { "ralph-metadata": ".ralph-tui", "ralph-prd-root": ".ralph-tui/prd", "ralph-prd-identifier": ".ralph-tui/prd/*" };
    for (const [id, expected] of Object.entries(ralphDirectoryContracts)) {
      const contract = taxonomy.fixedDirectoryContracts[id];
      if (!contract || contract.pathPattern !== expected || contract.authority !== "Ralph TUI")
        problems.push(`fixedDirectoryContracts.${id} must be the exact Ralph-owned path contract ${JSON.stringify(expected)}.`);
    }
    for (const [id, contract] of [...Object.entries(taxonomy.fixedFilenameContracts), ...Object.entries(taxonomy.fixedDirectoryContracts)])
      if (contract.pathPattern.startsWith(".ralph-tui/") && contract.pathPattern.includes("**"))
        problems.push(`Ralph contract ${JSON.stringify(id)} must not use a recursive wildcard.`);
    for (const output of outputOwners.filter((output2) => output2.path === ".ralph-tui" || output2.path.startsWith(".ralph-tui/")))
      if (output.id !== "setup-wizard-config")
        problems.push(`Ralph path ${JSON.stringify(output.path)} must be owned only by setup-wizard-config.`);
    const fixedRootManifests = { "root-package": "package.json", "root-cargo": "Cargo.toml", "root-go-work": "go.work" };
    for (const [id, expected] of Object.entries(fixedRootManifests))
      if (taxonomy.fixedFilenameContracts[id]?.pathPattern !== expected)
        problems.push(`fixedFilenameContracts.${id} must remain the authored root manifest contract ${JSON.stringify(expected)}.`);
    const generatedRootManifests = outputOwners.filter((output) => ["package.json", "Cargo.toml", "go.work"].includes(output.path));
    if (generatedRootManifests.length > 0)
      problems.push("Root Bun, Cargo, and Go manifests are authored fixed contracts, not generator outputs.");
  }
  if (record(taxonomy.pathExclusions, "pathExclusions")) {
    const entries = Object.entries(taxonomy.pathExclusions);
    if (entries.map(([id]) => id).join("\x00") !== "compose\x00temp-compose")
      problems.push('pathExclusions must contain exactly ordered "compose" and "temp-compose" contracts.');
    const compose = taxonomy.pathExclusions.compose;
    if (!compose || compose.path !== "compose/" || compose.mode !== "opaque" || !compose.reason)
      problems.push('pathExclusions.compose must be the exact opaque "compose/" contract.');
    const tempCompose = taxonomy.pathExclusions["temp-compose"];
    if (!tempCompose || tempCompose.path !== "temp/compose/" || tempCompose.mode !== "opaque" || !tempCompose.reason)
      problems.push('pathExclusions.temp-compose must be the exact opaque "temp/compose/" contract.');
  }
  if (taxonomy.unicodeNormalization?.form !== "NFC" || taxonomy.unicodeNormalization?.caseFold !== "lower" || taxonomy.unicodeNormalization?.locale !== "und")
    problems.push("unicodeNormalization must be NFC/lower/und.");
  if (taxonomy.variationSelectorPolicy?.selector !== "\uFE0F" || taxonomy.variationSelectorPolicy?.requiredAfterEmoji !== true || taxonomy.variationSelectorPolicy?.comparison !== "ignore-selector")
    problems.push("variationSelectorPolicy is invalid.");
  const comparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (taxonomy.collisionPolicy?.comparisons?.join("\x00") !== comparisons.join("\x00"))
    problems.push(`collisionPolicy.comparisons must be exactly ${comparisons.join(", ")}.`);
  if (taxonomy.collisionPolicy?.maxPathBytes !== 240 || taxonomy.collisionPolicy?.rejectWindowsReservedNames !== true || taxonomy.collisionPolicy?.rejectTrailingDotsAndSpaces !== true)
    problems.push("collisionPolicy platform constraints must retain maxPathBytes 240 and reject reserved/trailing names.");
  if (taxonomy.areaEnforcement?.requiredState !== "clean" || taxonomy.areaEnforcement?.undeclaredAreas !== "enforce")
    problems.push("areaEnforcement must enforce clean declared and undeclared areas.");
  ids(taxonomy.areaEnforcement?.opaquePathExclusionIds, taxonomy.pathExclusions, "areaEnforcement.opaquePathExclusionIds");
  if (taxonomy.areaEnforcement?.opaquePathExclusionIds?.join("\x00") !== "compose\x00temp-compose")
    problems.push('areaEnforcement.opaquePathExclusionIds must be exactly ["compose", "temp-compose"].');
  if (record(taxonomy.areas, "areas"))
    for (const [area, state] of Object.entries(taxonomy.areas)) {
      if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/"))
        problems.push("Opaque compose prefixes must exist only in pathExclusions.");
      if (state !== "clean")
        problems.push(`areas[${JSON.stringify(area)}] must be "clean".`);
    }
  if (record(taxonomy.areaLayers, "areaLayers"))
    for (const [area, layer] of Object.entries(taxonomy.areaLayers)) {
      if (area === "compose" || area.startsWith("compose/") || area === "temp/compose" || area.startsWith("temp/compose/"))
        problems.push("Opaque compose prefixes must not appear in areaLayers.");
      if (!["framework", "implementation", "repo-wide"].includes(layer))
        problems.push(`areaLayers[${JSON.stringify(area)}] is invalid.`);
    }
  for (const [lang, ecosystem] of Object.entries(taxonomy.ecosystems ?? {})) {
    const oldShape = ecosystem;
    for (const key of ["manifestFilename", "moduleRootFilename", "entryFilenames", "leafFilename", "sourceExtension", "packagingDirNames"])
      if (key in oldShape)
        problems.push(`ecosystems[${JSON.stringify(lang)}].${key} was removed.`);
    if (!["manifest", "boundary-only"].includes(ecosystem.packageIdentity))
      problems.push(`ecosystems[${JSON.stringify(lang)}].packageIdentity is invalid.`);
    if (ecosystem.packageIdentity === "manifest" && (!ecosystem.manifestContractId || !ecosystem.marker))
      problems.push(`ecosystems[${JSON.stringify(lang)}] manifest identity requires a manifest contract and marker.`);
    if (ecosystem.packageIdentity === "boundary-only" && (ecosystem.manifestContractId !== null || ecosystem.marker !== null))
      problems.push(`ecosystems[${JSON.stringify(lang)}] boundary-only identity cannot declare a manifest or marker.`);
    if (ecosystem.manifestContractId && !taxonomy.fixedFilenameContracts[ecosystem.manifestContractId])
      problems.push(`ecosystems[${JSON.stringify(lang)}].manifestContractId is missing.`);
    if (ecosystem.moduleRootContractId && !taxonomy.fixedFilenameContracts[ecosystem.moduleRootContractId])
      problems.push(`ecosystems[${JSON.stringify(lang)}].moduleRootContractId is missing.`);
    if (!taxonomy.fileKinds[ecosystem.componentFileKindId])
      problems.push(`ecosystems[${JSON.stringify(lang)}].componentFileKindId is missing.`);
    ids(ecosystem.sourceFileKindIds, taxonomy.fileKinds, `ecosystems[${JSON.stringify(lang)}].sourceFileKindIds`);
    ids(ecosystem.entryContractIds, taxonomy.configurableEntryContracts, `ecosystems[${JSON.stringify(lang)}].entryContractIds`);
    ids(ecosystem.packagingDirectoryKindIds ?? [], taxonomy.semanticDirectoryKinds, `ecosystems[${JSON.stringify(lang)}].packagingDirectoryKindIds`);
    if (!taxonomy.packageBoundaryRules[lang])
      problems.push(`ecosystems[${JSON.stringify(lang)}] has no packageBoundaryRules entry.`);
  }
  for (const [target, spec] of Object.entries(taxonomy.targets ?? {})) {
    const oldShape = spec;
    for (const key of ["leafFilename", "entryFilenames"])
      if (key in oldShape)
        problems.push(`targets[${JSON.stringify(target)}].${key} was removed.`);
    if (!taxonomy.ecosystems[spec.lang])
      problems.push(`targets[${JSON.stringify(target)}].lang is missing.`);
    if (!taxonomy.fileKinds[spec.componentFileKindId])
      problems.push(`targets[${JSON.stringify(target)}].componentFileKindId is missing.`);
    ids(spec.entryContractIds, taxonomy.configurableEntryContracts, `targets[${JSON.stringify(target)}].entryContractIds`);
  }
  const mappings = [
    ["componentFileKinds", taxonomy.componentFileKinds],
    ["exampleFileKinds", taxonomy.exampleFileKinds],
    ["exampleTestFileKinds", taxonomy.exampleTestFileKinds],
    ["testAdapterFileKinds", taxonomy.testAdapterFileKinds],
    ["artifactSpecFileKinds", taxonomy.artifactSpecFileKinds],
    ["artifactSchemaSpecFileKinds", taxonomy.artifactSchemaSpecFileKinds],
    ["surfaceSchemaSpecFileKinds", taxonomy.surfaceSchemaSpecFileKinds]
  ];
  for (const [key, mapping] of mappings)
    for (const [owner, kindId] of Object.entries(mapping ?? {}))
      if (!taxonomy.fileKinds[kindId])
        problems.push(`${key}[${JSON.stringify(owner)}] references missing kind ${JSON.stringify(kindId)}.`);
  for (const [key, kindId] of [
    ["semanticManifestFileKindId", taxonomy.semanticManifestFileKindId],
    ["subsetsManifestFileKindId", taxonomy.subsetsManifestFileKindId],
    ["storyFileKindId", taxonomy.storyFileKindId],
    ["testFeatureFileKindId", taxonomy.testFeatureFileKindId],
    ["testContributionFileKindId", taxonomy.testContributionFileKindId],
    ["testOutputMarkerFileKindId", taxonomy.testOutputMarkerFileKindId],
    ["windowEmptyFacetFileKindId", taxonomy.windowEmptyFacetFileKindId],
    ["testOracleRegistryLocation.fileKindId", taxonomy.testOracleRegistryLocation?.fileKindId],
    ["testSchemaLocation.fileKindId", taxonomy.testSchemaLocation?.fileKindId]
  ])
    if (!kindId || !taxonomy.fileKinds[kindId])
      problems.push(`${key} references a missing file kind.`);
  ids(taxonomy.textSpecFileKinds, taxonomy.fileKinds, "textSpecFileKinds");
  ids(taxonomy.binarySpecFileKinds, taxonomy.fileKinds, "binarySpecFileKinds");
  ids(taxonomy.rootDataContractIds, taxonomy.fixedFilenameContracts, "rootDataContractIds");
  ids(taxonomy.rootDocumentContractIds, taxonomy.fixedFilenameContracts, "rootDocumentContractIds");
  ids(taxonomy.repoWideContractIds, taxonomy.fixedFilenameContracts, "repoWideContractIds");
  ids(taxonomy.layeringGeneratedContractIds, taxonomy.fixedFilenameContracts, "layeringGeneratedContractIds");
  if (taxonomy.layeringGeneratedContractIds.length !== 0)
    problems.push("layeringGeneratedContractIds must be empty until an exact deterministic writer exists.");
  for (const [formatId, format] of Object.entries(taxonomy.schemaFormats ?? {})) {
    if ("leafFilename" in format || "extension" in format)
      problems.push(`schemaFormats[${JSON.stringify(formatId)}] contains removed filename fields.`);
    if (!taxonomy.fileKinds[format.fileKindId])
      problems.push(`schemaFormats[${JSON.stringify(formatId)}].fileKindId is missing.`);
    if (!["snake", "camel", "kebab"].includes(format.fieldCasing))
      problems.push(`schemaFormats[${JSON.stringify(formatId)}].fieldCasing is invalid.`);
  }
  for (const [kindId, kind] of Object.entries(taxonomy.schemaFacetKinds ?? {})) {
    if (!kind.formats.includes(kind.normativeFormat))
      problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}].formats must include its normative format.`);
    for (const formatId of kind.formats)
      if (!taxonomy.schemaFormats[formatId])
        problems.push(`schemaFacetKinds[${JSON.stringify(kindId)}] references missing format ${JSON.stringify(formatId)}.`);
  }
  const directoryValues = [
    taxonomy.packagesDirName,
    taxonomy.targetsDirName,
    taxonomy.elementsDirName,
    taxonomy.artifactsDirName,
    taxonomy.modesDirName,
    taxonomy.windowsDirName,
    taxonomy.standardsDirName,
    taxonomy.subsetsDirName,
    taxonomy.viewerDirName,
    taxonomy.editorDirName,
    taxonomy.exampleAssetsDirName,
    taxonomy.exampleTestsDirName,
    ...taxonomy.artifactChildDirs,
    ...taxonomy.newArtifactChildDirs,
    ...taxonomy.standardChildDirs,
    ...taxonomy.subsetChildDirs,
    ...taxonomy.surfaceChildDirs,
    ...taxonomy.modeChildDirs,
    ...taxonomy.windowChildDirs,
    ...taxonomy.taxonomyLeafParentDirs,
    ...taxonomy.pluginChildDirs,
    ...taxonomy.osChildDirs,
    ...taxonomy.rootDataDirNames,
    ...taxonomy.schemaChildDirs,
    ...taxonomy.representationDirs,
    ...taxonomy.ioDirectionDirs,
    ...taxonomy.ioSemanticCollectionDirNames,
    ...Object.values(taxonomy.ioDirectionChildDirs)
  ];
  for (const directory of new Set(directoryValues))
    if (!semanticDirectoryKindId(directory, taxonomy))
      problems.push(`Semantic directory ${JSON.stringify(directory)} is not uniquely registered.`);
  for (const dir of taxonomy.artifactComponentDirs)
    if (!taxonomy.artifactChildDirs.includes(dir))
      problems.push(`artifactChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.windowRequiredChildDirs)
    if (!taxonomy.windowChildDirs.includes(dir))
      problems.push(`windowChildDirs must include ${JSON.stringify(dir)}.`);
  for (const dir of taxonomy.surfaceRequiredChildDirs)
    if (!taxonomy.surfaceChildDirs.includes(dir))
      problems.push(`surfaceChildDirs must include ${JSON.stringify(dir)}.`);
  return problems;
}
function validateGeneratorContractsAgainstWorkspace(repoRoot, taxonomy = readTaxonomyUnchecked()) {
  const problems = [];
  const root = resolve(repoRoot);
  for (const [id, contract] of Object.entries(taxonomy.generatorContracts ?? {})) {
    if (contract.target) {
      if (!contract.ownerPath) {
        problems.push(`generatorContracts[${JSON.stringify(id)}] has a target without an ownerPath.`);
      } else {
        const manifestPath = join(root, contract.ownerPath, "\uD83D\uDCCB\uFE0Fproject.json");
        if (!existsSync(manifestPath)) {
          problems.push(`generatorContracts[${JSON.stringify(id)}] owner project is missing at ${JSON.stringify(relative(root, manifestPath))}.`);
        } else {
          try {
            const project = JSON.parse(readFileSync(manifestPath, "utf8"));
            const separator = contract.target.lastIndexOf(":");
            const projectName = contract.target.slice(0, separator);
            const targetName = contract.target.slice(separator + 1);
            if (project.name !== projectName || !project.targets?.[targetName])
              problems.push(`generatorContracts[${JSON.stringify(id)}].target ${JSON.stringify(contract.target)} is absent from its owner project.`);
            if (contract.previewTarget) {
              const previewSeparator = contract.previewTarget.lastIndexOf(":");
              const previewProject = contract.previewTarget.slice(0, previewSeparator);
              const previewName = contract.previewTarget.slice(previewSeparator + 1);
              const preview = project.targets?.[previewName];
              if (project.name !== previewProject || !preview)
                problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget ${JSON.stringify(contract.previewTarget)} is absent from its owner project.`);
              else if (preview.executor !== "nx:run-commands" || preview.options?.cwd !== contract.ownerPath || preview.options?.command !== "bun ./\uD83D\uDCDC\uFE0Fscript.ts preview-generated")
                problems.push(`generatorContracts[${JSON.stringify(id)}].previewTarget must route exactly to bun ./\uD83D\uDCDC\uFE0Fscript.ts preview-generated in its owner project.`);
            }
            if (contract.checkTarget) {
              const checkSeparator = contract.checkTarget.lastIndexOf(":");
              const checkProject = contract.checkTarget.slice(0, checkSeparator);
              const checkName = contract.checkTarget.slice(checkSeparator + 1);
              if (project.name !== checkProject || !project.targets?.[checkName])
                problems.push(`generatorContracts[${JSON.stringify(id)}].checkTarget ${JSON.stringify(contract.checkTarget)} is absent from its owner project.`);
            }
          } catch {
            problems.push(`generatorContracts[${JSON.stringify(id)}] owner project is not valid JSON.`);
          }
        }
      }
    }
    for (const output of contract.outputRoots ?? []) {
      const owners = generatorContractIdsForOutputPath(output.path, taxonomy);
      if (owners.length !== 1 || owners[0] !== id)
        problems.push(`generatorContracts[${JSON.stringify(id)}] output ${JSON.stringify(output.path)} does not have exactly one owner.`);
      if (output.inclusion === "tracked" && !existsSync(join(root, output.path)))
        problems.push(`generatorContracts[${JSON.stringify(id)}] tracked output ${JSON.stringify(output.path)} is missing.`);
    }
  }
  return problems;
}
function tomlTableBody(text, table) {
  const header = `[${table}]`;
  const lines = text.split(`
`);
  const start = lines.findIndex((line) => line.trim() === header);
  if (start === -1)
    return;
  const body = [];
  for (let i = start + 1;i < lines.length; i++) {
    if (lines[i].trim().startsWith("["))
      break;
    body.push(lines[i]);
  }
  return body.join(`
`);
}
function tomlTableValues(body) {
  const result = {};
  for (const rawLine of body.split(`
`)) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#"))
      continue;
    const arrayMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\[([^\]]*)\]\s*$/);
    if (arrayMatch) {
      result[arrayMatch[1]] = [...arrayMatch[2].matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((m) => m[1]);
      continue;
    }
    const scalarMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$/);
    if (scalarMatch)
      result[scalarMatch[1]] = scalarMatch[2];
  }
  return result;
}
var DISCOVERY_SKIP_DIRS = new Set(["node_modules", "target", "dist", "\uD83D\uDCE4\uFE0Fdist", ".git", ".\uD83E\uDDECsemio", "\uD83E\uDD16\uFE0Fgenerated", "\uD83D\uDD0C\uFE0Fplugin-modules", "pkg", "storybook-static", "temp", ".venv", "coverage", "__pycache__", "client", "client_bin"]);
var scanCache = ephemeralMap("framework.products.repo.modules.lib.discovery.component.ts.scanCache");
var SEMANTIC_SKIP_DIRS = new Set(["node_modules", "target", "dist", ".git", ".nx", ".cache", "vendor", "pkg", "storybook-static", "temp"]);
var SEMANTIC_NON_PRODUCTION_SEGMENTS = new Set(["\uD83E\uDDEA\uFE0Ftests", "tests", "test", "__tests__", "\uD83D\uDCDA\uFE0Fexamples", "\uD83E\uDDEA\uFE0Fexamples", "examples", "fixtures", "\uD83E\uDDEA\uFE0Ffixtures", "\uD83E\uDD16\uFE0Fgenerated"]);

/* 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts */
var TAXONOMY_RELATIVE_PATH = "\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83D\uDD23\uFE0Ftaxonomy.json";
var TRANSACTION_DISPOSITIONS_FIXTURE_PATH = "\uD83E\uDDF0\uFE0Fframework/\uD83D\uDECD\uFE0Fproducts/\uD83E\uDD91\uFE0Frepo/\uD83D\uDD28\uFE0Fmodules/\uD83D\uDCDA\uFE0Flibrary/\uD83D\uDCE6\uFE0Fpackages/\uD83D\uDFE6\uFE0Ftypescript/\uD83E\uDDEB\uFE0Ffixtures/\uD83E\uDDEA\uFE0Ftransaction-dispositions/\uD83D\uDD23\uFE0F.json";
var LEXICAL_OPAQUE_ROOTS = ["compose", "temp/compose"];
var GENERIC_SEMANTIC_STEMS = new Set(["asset", "assets", "component", "components", "empty", "glue", "test", "tests", "implementation", "impl", "index"]);
var WINDOWS_RESERVED = /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i;
var SEGMENTER = new Intl.Segmenter("und", { granularity: "grapheme" });
function record(value, name) {
  if (!value || typeof value !== "object" || Array.isArray(value))
    throw new Error(`Taxonomy v7 field ${name} must be an object`);
  return value;
}
function stringArray(value, name) {
  if (!Array.isArray(value) || value.some((item) => typeof item !== "string"))
    throw new Error(`Taxonomy v7 field ${name} must be a string array`);
  return value;
}
function requiredString(value, name) {
  if (typeof value !== "string" || value.length === 0)
    throw new Error(`Taxonomy v7 field ${name} must be a non-empty string`);
  return value;
}
function requireExactKeys(value, keys, name) {
  const actual = Object.keys(value).sort();
  const expected = [...keys].sort();
  if (canonicalJson(actual) !== canonicalJson(expected))
    throw new Error(`Taxonomy v7 field ${name} must contain exactly ${expected.join(", ")}`);
}
function validatedContractPattern(value, name, exactBasename) {
  const pattern = requiredString(value, name);
  if (pattern !== pattern.normalize("NFC") || pattern.startsWith("/") || pattern.endsWith("/") || pattern.includes("\\") || pattern.includes("//") || pattern.includes("\x00"))
    throw new Error(`Taxonomy v7 ${name} must be one NFC workspace-relative POSIX pattern`);
  if (/[{}]/u.test(pattern) || /^!/u.test(pattern) || /[!@+?*]\(/u.test(pattern))
    throw new Error(`Taxonomy v7 ${name} uses unsupported glob syntax`);
  for (const segment of pattern.split("/")) {
    if (segment.includes("**") && segment !== "**")
      throw new Error(`Taxonomy v7 ${name} may use ** only as a whole segment`);
    for (const match of segment.matchAll(/\[([^\]]*)\]/gu))
      if (!/^[A-Za-z0-9-]+$/u.test(match[1]) || /^[!^]/u.test(match[1]))
        throw new Error(`Taxonomy v7 ${name} has an invalid character class`);
    if ((segment.match(/\[/gu)?.length ?? 0) !== (segment.match(/\]/gu)?.length ?? 0))
      throw new Error(`Taxonomy v7 ${name} has an unclosed character class`);
  }
  const filename = pattern.slice(pattern.lastIndexOf("/") + 1);
  if (exactBasename && /[*?\[\]]/u.test(filename))
    throw new Error(`Taxonomy v7 ${name} must end in one exact literal basename`);
  taxonomyPathPatternMatches("", pattern);
  return pattern;
}
function fixedExpiry(value, name) {
  if (value === null)
    return null;
  const expires = requiredString(value, name);
  if (!/^\d{4}-\d{2}-\d{2}$/u.test(expires))
    throw new Error(`Taxonomy v7 ${name} must be null or YYYY-MM-DD`);
  return expires;
}
function parseTaxonomy(raw, path) {
  const root = record(raw, "root");
  if (root.schemaVersion !== 7)
    throw new Error(`Taxonomy schemaVersion must be 7 at ${path}`);
  const discoveryProblems = validateTaxonomy(root);
  if (discoveryProblems.length > 0)
    throw new Error(`Taxonomy v7 discovery contract validation failed at ${path}: ${discoveryProblems.join(" | ")}`);
  const fileKindRows = record(root.fileKinds, "fileKinds");
  const directoryKindRows = record(root.semanticDirectoryKinds, "semanticDirectoryKinds");
  const fixedRows = record(root.fixedFilenameContracts, "fixedFilenameContracts");
  const fixedRejectionRows = record(root.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts");
  const fixedDirectoryRows = record(root.fixedDirectoryContracts, "fixedDirectoryContracts");
  const configurableRows = record(root.configurableEntryContracts, "configurableEntryContracts");
  const fileResolutionRows = record(root.fileKindResolutionRules, "fileKindResolutionRules");
  const scopedFileRows = record(root.scopedFileKinds, "scopedFileKinds");
  const directoryMemberRows = record(root.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds");
  const projectedMemberRows = record(root.semanticProjectedMemberKinds, "semanticProjectedMemberKinds");
  const projectionRendererRows = record(root.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers");
  const descendantContractRows = record(root.semanticDescendantContracts, "semanticDescendantContracts");
  const projectionCatalogRows = record(root.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts");
  const projectionRows = record(root.semanticPathProjectionContracts, "semanticPathProjectionContracts");
  const ownedFileProjectionRows = record(root.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts");
  const projectionConsumerRows = record(root.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts");
  const mutationCatalogProjectionRow = record(root.mutationCatalogProjection, "mutationCatalogProjection");
  const generatorRows = record(root.generatorContracts, "generatorContracts");
  const ecosystemRows = record(root.ecosystems, "ecosystems");
  const boundaryRows = record(root.packageBoundaryRules, "packageBoundaryRules");
  const boundaryProfileRows = record(root.packageBoundaryProfiles, "packageBoundaryProfiles");
  const grammarRows = record(root.packageGlueGrammar, "packageGlueGrammar");
  const sourceDispositionRows = record(root.packageSourceDispositions, "packageSourceDispositions");
  const exclusionRows = record(root.pathExclusions, "pathExclusions");
  const unicode = record(root.unicodeNormalization, "unicodeNormalization");
  const selector = record(root.variationSelectorPolicy, "variationSelectorPolicy");
  const collision = record(root.collisionPolicy, "collisionPolicy");
  const enforcement = record(root.areaEnforcement, "areaEnforcement");
  if (unicode.form !== "NFC" || unicode.caseFold !== "lower" || unicode.locale !== "und")
    throw new Error("Taxonomy v7 unicodeNormalization must select NFC/lower/und");
  if (selector.selector !== "\uFE0F" || selector.requiredAfterEmoji !== true || selector.comparison !== "ignore-selector")
    throw new Error("Taxonomy v7 variationSelectorPolicy is not canonical");
  const requiredComparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (canonicalJson(collision.comparisons) !== canonicalJson(requiredComparisons) || !Number.isSafeInteger(collision.maxPathBytes) || collision.maxPathBytes < 1 || collision.rejectWindowsReservedNames !== true || collision.rejectTrailingDotsAndSpaces !== true)
    throw new Error("Taxonomy v7 collisionPolicy is incomplete");
  if (enforcement.requiredState !== "clean" || enforcement.undeclaredAreas !== "enforce")
    throw new Error("Taxonomy v7 areaEnforcement must enforce clean undeclared areas");
  const fileKinds = {};
  for (const [id, value] of Object.entries(fileKindRows)) {
    const spec = record(value, `fileKinds.${id}`);
    const emoji = requiredString(spec.emoji, `fileKinds.${id}.emoji`).normalize("NFC");
    const extensionChains = stringArray(spec.extensionChains, `fileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith(".")))
      throw new Error(`Taxonomy v7 fileKinds.${id}.extensionChains must contain dotted chains`);
    fileKinds[id] = { emoji, extensionChains: [...new Set(extensionChains)].sort((a, b) => b.length - a.length || a.localeCompare(b)), role: requiredString(spec.role, `fileKinds.${id}.role`) };
  }
  if (Object.keys(fileKinds).length === 0)
    throw new Error("Taxonomy v7 fileKinds must not be empty");
  const semanticDirectoryKinds = {};
  for (const [id, value] of Object.entries(directoryKindRows)) {
    const spec = record(value, `semanticDirectoryKinds.${id}`);
    const emoji = requiredString(spec.emoji, `semanticDirectoryKinds.${id}.emoji`).normalize("NFC");
    const slugPattern = requiredString(spec.slugPattern, `semanticDirectoryKinds.${id}.slugPattern`);
    new RegExp(slugPattern, "u");
    if (typeof spec.allowEmojiOnly !== "boolean")
      throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.allowEmojiOnly must be boolean`);
    if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean")
      throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.inferWithoutEmoji must be boolean when present`);
    if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean")
      throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.projectionOnly must be boolean when present`);
    semanticDirectoryKinds[id] = { emoji, slugPattern, allowEmojiOnly: spec.allowEmojiOnly, ...spec.inferWithoutEmoji === undefined ? {} : { inferWithoutEmoji: spec.inferWithoutEmoji }, ...spec.projectionOnly === undefined ? {} : { projectionOnly: spec.projectionOnly }, ...spec.parentKindIds === undefined ? {} : { parentKindIds: stringArray(spec.parentKindIds, `semanticDirectoryKinds.${id}.parentKindIds`) } };
  }
  if (Object.keys(semanticDirectoryKinds).length === 0)
    throw new Error("Taxonomy v7 semanticDirectoryKinds must not be empty");
  const fixedFilenameContracts = {};
  for (const [id, value] of Object.entries(fixedRows)) {
    const spec = record(value, `fixedFilenameContracts.${id}`);
    if (spec.configurability !== "unconfigurable")
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.configurability must be unconfigurable`);
    const scopeRow = record(spec.scope, `fixedFilenameContracts.${id}.scope`);
    const scopeKind = requiredString(scopeRow.kind, `fixedFilenameContracts.${id}.scope.kind`);
    if (!["exact-path", "repository-root", "package-root", "directory-kind", "fixed-directory-contract", "sibling-fixed-filename-contract", "path-pattern"].includes(scopeKind))
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.kind is invalid`);
    const scope = scopeKind === "exact-path" ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedFilenameContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requiredString(scopeRow.path, `fixedFilenameContracts.${id}.scope.path`)) }) : scopeKind === "package-root" ? (requireExactKeys(scopeRow, ["kind", "ecosystemId"], `fixedFilenameContracts.${id}.scope`), { kind: "package-root", ecosystemId: requiredString(scopeRow.ecosystemId, `fixedFilenameContracts.${id}.scope.ecosystemId`) }) : scopeKind === "directory-kind" ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedFilenameContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requiredString(scopeRow.directoryKindId, `fixedFilenameContracts.${id}.scope.directoryKindId`) }) : scopeKind === "fixed-directory-contract" ? (requireExactKeys(scopeRow, ["kind", "fixedDirectoryContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "fixed-directory-contract", fixedDirectoryContractId: requiredString(scopeRow.fixedDirectoryContractId, `fixedFilenameContracts.${id}.scope.fixedDirectoryContractId`) }) : scopeKind === "sibling-fixed-filename-contract" ? (requireExactKeys(scopeRow, ["kind", "fixedFilenameContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "sibling-fixed-filename-contract", fixedFilenameContractId: requiredString(scopeRow.fixedFilenameContractId, `fixedFilenameContracts.${id}.scope.fixedFilenameContractId`) }) : (requireExactKeys(scopeRow, ["kind"], `fixedFilenameContracts.${id}.scope`), { kind: scopeKind });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId])
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.directoryKindId is invalid`);
    fixedFilenameContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedFilenameContracts.${id}.pathPattern`, true),
      authority: requiredString(spec.authority, `fixedFilenameContracts.${id}.authority`),
      reason: requiredString(spec.reason, `fixedFilenameContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requiredString(spec.verification, `fixedFilenameContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedFilenameContracts.${id}.expires`)
    };
  }
  const fixedDirectoryContracts = {};
  for (const [id, value] of Object.entries(fixedDirectoryRows)) {
    const spec = record(value, `fixedDirectoryContracts.${id}`);
    if (spec.configurability !== "unconfigurable")
      throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.configurability must be unconfigurable`);
    const scopeRow = record(spec.scope, `fixedDirectoryContracts.${id}.scope`);
    const scopeKind = requiredString(scopeRow.kind, `fixedDirectoryContracts.${id}.scope.kind`);
    if (!["exact-path", "repository-root", "directory-kind", "path-pattern"].includes(scopeKind))
      throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.kind is invalid`);
    const scope = scopeKind === "exact-path" ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedDirectoryContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requiredString(scopeRow.path, `fixedDirectoryContracts.${id}.scope.path`)) }) : scopeKind === "directory-kind" ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedDirectoryContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requiredString(scopeRow.directoryKindId, `fixedDirectoryContracts.${id}.scope.directoryKindId`) }) : (requireExactKeys(scopeRow, ["kind"], `fixedDirectoryContracts.${id}.scope`), { kind: scopeKind });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId])
      throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.directoryKindId is invalid`);
    fixedDirectoryContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedDirectoryContracts.${id}.pathPattern`, false),
      authority: requiredString(spec.authority, `fixedDirectoryContracts.${id}.authority`),
      reason: requiredString(spec.reason, `fixedDirectoryContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requiredString(spec.verification, `fixedDirectoryContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedDirectoryContracts.${id}.expires`)
    };
  }
  if (Object.keys(fixedDirectoryContracts).length === 0)
    throw new Error("Taxonomy v7 fixedDirectoryContracts must not be empty");
  for (const [id, contract] of Object.entries(fixedFilenameContracts)) {
    if (contract.scope.kind === "fixed-directory-contract" && !fixedDirectoryContracts[contract.scope.fixedDirectoryContractId])
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.fixedDirectoryContractId is invalid`);
    if (contract.scope.kind === "sibling-fixed-filename-contract" && !fixedFilenameContracts[contract.scope.fixedFilenameContractId])
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.fixedFilenameContractId is invalid`);
  }
  const fixedFilenameRejectionContracts = {};
  const rejectedFixedPaths = new Set;
  for (const [id, value] of Object.entries(fixedRejectionRows)) {
    const spec = record(value, `fixedFilenameRejectionContracts.${id}`);
    requireExactKeys(spec, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts.${id}`);
    if (spec.disposition !== "normalize" && spec.disposition !== "relocate")
      throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.disposition is invalid`);
    const sourcePathIdentities = stringArray(spec.sourcePathIdentities, `fixedFilenameRejectionContracts.${id}.sourcePathIdentities`).map(normalizeRelative);
    if (sourcePathIdentities.length === 0 || sourcePathIdentities.some((path2) => rejectedFixedPaths.has(path2)))
      throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.sourcePathIdentities are empty or duplicated`);
    for (const path2 of sourcePathIdentities)
      rejectedFixedPaths.add(path2);
    fixedFilenameRejectionContracts[id] = { sourcePathIdentities, disposition: spec.disposition, reason: requiredString(spec.reason, `fixedFilenameRejectionContracts.${id}.reason`) };
  }
  if (Object.keys(fixedFilenameRejectionContracts).length === 0)
    throw new Error("Taxonomy v7 fixedFilenameRejectionContracts must not be empty");
  const configurableEntryContracts = {};
  for (const [id, value] of Object.entries(configurableRows)) {
    const spec = record(value, `configurableEntryContracts.${id}`);
    const fileKindId = requiredString(spec.fileKindId, `configurableEntryContracts.${id}.fileKindId`);
    if (!fileKinds[fileKindId])
      throw new Error(`Taxonomy v7 configurableEntryContracts.${id} references unknown file kind ${fileKindId}`);
    configurableEntryContracts[id] = {
      filename: requiredString(spec.filename, `configurableEntryContracts.${id}.filename`),
      fileKindId,
      ecosystemId: requiredString(spec.ecosystemId, `configurableEntryContracts.${id}.ecosystemId`),
      role: requiredString(spec.role, `configurableEntryContracts.${id}.role`),
      configurationSources: stringArray(spec.configurationSources, `configurableEntryContracts.${id}.configurationSources`)
    };
  }
  const fileKindResolutionRules = {};
  for (const [id, value] of Object.entries(fileResolutionRows)) {
    const spec = record(value, `fileKindResolutionRules.${id}`);
    const extensionChain = requiredString(spec.extensionChain, `fileKindResolutionRules.${id}.extensionChain`);
    const fileKindId = requiredString(spec.fileKindId, `fileKindResolutionRules.${id}.fileKindId`);
    if (!fileKinds[fileKindId]?.extensionChains.includes(extensionChain))
      throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} does not reference an owned extension chain`);
    if (!Number.isSafeInteger(spec.priority))
      throw new Error(`Taxonomy v7 fileKindResolutionRules.${id}.priority must be an integer`);
    const filenamePattern = typeof spec.filenamePattern === "string" ? spec.filenamePattern : undefined;
    const pathPattern = typeof spec.pathPattern === "string" ? validatedContractPattern(spec.pathPattern, `fileKindResolutionRules.${id}.pathPattern`, false) : undefined;
    if (filenamePattern)
      new RegExp(filenamePattern, "u");
    const parentKindIds = spec.parentKindIds === undefined ? undefined : stringArray(spec.parentKindIds, `fileKindResolutionRules.${id}.parentKindIds`);
    const ancestorKindIds = spec.ancestorKindIds === undefined ? undefined : stringArray(spec.ancestorKindIds, `fileKindResolutionRules.${id}.ancestorKindIds`);
    for (const kindId of [...parentKindIds ?? [], ...ancestorKindIds ?? []])
      if (!semanticDirectoryKinds[kindId])
        throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} references unknown directory kind ${kindId}`);
    fileKindResolutionRules[id] = { extensionChain, fileKindId, priority: spec.priority, filenamePattern, pathPattern, parentKindIds, ancestorKindIds };
  }
  if (Object.keys(fileKindResolutionRules).length === 0)
    throw new Error("Taxonomy v7 fileKindResolutionRules must not be empty");
  const scopedFileKinds = {};
  for (const [id, value] of Object.entries(scopedFileRows)) {
    const spec = record(value, `scopedFileKinds.${id}`);
    const extensionChains = stringArray(spec.extensionChains, `scopedFileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith(".")))
      throw new Error(`Taxonomy v7 scopedFileKinds.${id}.extensionChains must contain dotted chains`);
    const sourceFilenamePattern = requiredString(spec.sourceFilenamePattern, `scopedFileKinds.${id}.sourceFilenamePattern`);
    new RegExp(sourceFilenamePattern, "u");
    if (spec.role !== "evidence")
      throw new Error(`Taxonomy v7 scopedFileKinds.${id}.role must be evidence`);
    const parentDirectoryKindId = requiredString(spec.parentDirectoryKindId, `scopedFileKinds.${id}.parentDirectoryKindId`);
    if (!semanticDirectoryKinds[parentDirectoryKindId])
      throw new Error(`Taxonomy v7 scopedFileKinds.${id} references unknown parent directory kind ${parentDirectoryKindId}`);
    scopedFileKinds[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `scopedFileKinds.${id}.pathPattern`, false),
      parentDirectoryKindId,
      emoji: requiredString(spec.emoji, `scopedFileKinds.${id}.emoji`).normalize("NFC"),
      extensionChains: [...new Set(extensionChains)].sort((left, right) => right.length - left.length || left.localeCompare(right)),
      role: "evidence",
      sourceFilenamePattern,
      authority: requiredString(spec.authority, `scopedFileKinds.${id}.authority`),
      reason: requiredString(spec.reason, `scopedFileKinds.${id}.reason`),
      verification: requiredString(spec.verification, `scopedFileKinds.${id}.verification`),
      expires: fixedExpiry(spec.expires, `scopedFileKinds.${id}.expires`)
    };
  }
  const semanticDirectoryMemberKinds = {};
  for (const [id, value] of Object.entries(directoryMemberRows)) {
    const spec = record(value, `semanticDirectoryMemberKinds.${id}`);
    if (spec.source !== "registry")
      throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id}.source must be registry`);
    const ownerKindIds = stringArray(spec.ownerKindIds, `semanticDirectoryMemberKinds.${id}.ownerKindIds`);
    const memberNames = stringArray(spec.memberNames, `semanticDirectoryMemberKinds.${id}.memberNames`);
    if (ownerKindIds.length === 0 || memberNames.length === 0)
      throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} must declare owners and members`);
    if (memberNames.some((name) => name !== name.normalize("NFC") || !splitLeadingEmoji(name).emoji))
      throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} member names must be NFC emoji-leading evidence`);
    semanticDirectoryMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), memberNames: [...new Set(memberNames)].sort(), source: "registry" };
  }
  const directoryContextIds = new Set([...Object.keys(semanticDirectoryKinds), ...Object.keys(semanticDirectoryMemberKinds)]);
  for (const [id, spec] of Object.entries(semanticDirectoryMemberKinds))
    for (const ownerId of spec.ownerKindIds)
      if (!directoryContextIds.has(ownerId))
        throw new Error(`Taxonomy v7 semanticDirectoryMemberKinds.${id} references unknown owner kind ${ownerId}`);
  const semanticProjectedMemberKinds = {};
  for (const [id, value] of Object.entries(projectedMemberRows)) {
    const spec = record(value, `semanticProjectedMemberKinds.${id}`);
    if (spec.identityField !== "mutationDirectoryName" && spec.identityField !== "commandDirectoryName")
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.identityField is invalid`);
    const ownerKindIds = stringArray(spec.ownerKindIds, `semanticProjectedMemberKinds.${id}.ownerKindIds`);
    if (ownerKindIds.length === 0)
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id}.ownerKindIds must not be empty`);
    semanticProjectedMemberKinds[id] = { ownerKindIds: [...new Set(ownerKindIds)].sort(), projectionContractId: requiredString(spec.projectionContractId, `semanticProjectedMemberKinds.${id}.projectionContractId`), sourceMemberKindId: requiredString(spec.sourceMemberKindId, `semanticProjectedMemberKinds.${id}.sourceMemberKindId`), identityField: spec.identityField };
  }
  if (Object.keys(semanticProjectedMemberKinds).length === 0)
    throw new Error("Taxonomy v7 semanticProjectedMemberKinds must not be empty");
  const allDirectoryContextIds = new Set([...directoryContextIds, ...Object.keys(semanticProjectedMemberKinds)]);
  for (const [id, spec] of Object.entries(semanticDirectoryKinds))
    for (const parentId of spec.parentKindIds ?? [])
      if (!allDirectoryContextIds.has(parentId))
        throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id} references unknown parent kind ${parentId}`);
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds)) {
    if (!semanticDirectoryMemberKinds[spec.sourceMemberKindId])
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown source member kind ${spec.sourceMemberKindId}`);
    for (const ownerId of spec.ownerKindIds)
      if (!allDirectoryContextIds.has(ownerId))
        throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown owner kind ${ownerId}`);
  }
  const semanticPathProjectionProfileRenderers = {};
  for (const [id, value] of Object.entries(projectionRendererRows)) {
    const spec = record(value, `semanticPathProjectionProfileRenderers.${id}`);
    if (spec.direction !== "forward-only" || canonicalJson(spec.captureFields) !== canonicalJson(["standardVersion", "subsetId"]) || spec.template !== "\uD83E\uDE86\uFE0F{standardVersion}-{subsetId}" || canonicalJson(spec.tupleCollisionFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId"]))
      throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} is not the forward-only standard/subset contract`);
    const directoryKindId = requiredString(spec.directoryKindId, `semanticPathProjectionProfileRenderers.${id}.directoryKindId`);
    if (!semanticDirectoryKinds[directoryKindId])
      throw new Error(`Taxonomy v7 semanticPathProjectionProfileRenderers.${id} references unknown directory kind ${directoryKindId}`);
    semanticPathProjectionProfileRenderers[id] = { direction: "forward-only", captureFields: ["standardVersion", "subsetId"], directoryKindId, template: "\uD83E\uDE86\uFE0F{standardVersion}-{subsetId}", tupleCollisionFields: ["artifactId", "standardVersion", "subsetId"] };
  }
  if (Object.keys(semanticPathProjectionProfileRenderers).length === 0)
    throw new Error("Taxonomy v7 semanticPathProjectionProfileRenderers must not be empty");
  const parseDescendantNode = (value, name) => {
    const spec = record(value, name);
    if (spec.nodeType !== "directory" && spec.nodeType !== "file")
      throw new Error(`Taxonomy v7 ${name}.nodeType is invalid`);
    const parseSegments = (value2, key) => {
      if (!Array.isArray(value2))
        throw new Error(`Taxonomy v7 ${key} must be an array`);
      return value2.map((value3, index) => {
        const segment = record(value3, `${key}[${index}]`);
        const kindId = requiredString(segment.kindId, `${key}[${index}].kindId`);
        const literal = requiredString(segment.literal, `${key}[${index}].literal`).normalize("NFC");
        const kind = semanticDirectoryKinds[kindId];
        const leading = splitLeadingEmoji(literal);
        if (!kind || emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest))
          throw new Error(`Taxonomy v7 ${key} has an invalid semantic path segment ${literal}`);
        return { kindId, literal };
      });
    };
    if (spec.nodeType === "file" && spec.configurableEntry !== undefined) {
      const sourcePathSegments = parseSegments(spec.sourcePathSegments, `${name}.sourcePathSegments`);
      const destinationPathSegments = parseSegments(spec.destinationPathSegments, `${name}.destinationPathSegments`);
      const configurable = record(spec.configurableEntry, `${name}.configurableEntry`);
      const contractId = requiredString(configurable.contractId, `${name}.configurableEntry.contractId`);
      const contract = configurableEntryContracts[contractId];
      const sourceFilename = requiredString(configurable.sourceFilename, `${name}.configurableEntry.sourceFilename`).normalize("NFC");
      if (!contract || /[\\/]/u.test(sourceFilename) || sourceFilename !== basename2(sourceFilename))
        throw new Error(`Taxonomy v7 ${name}.configurableEntry is not a registered source basename`);
      if (!Array.isArray(configurable.configurationReferences) || configurable.configurationReferences.length === 0)
        throw new Error(`Taxonomy v7 ${name}.configurableEntry.configurationReferences must not be empty`);
      const configurationReferences = configurable.configurationReferences.map((value2, index) => {
        const reference = record(value2, `${name}.configurableEntry.configurationReferences[${index}]`);
        const fixedFilenameContractId = requiredString(reference.fixedFilenameContractId, `${name}.configurableEntry.configurationReferences[${index}].fixedFilenameContractId`);
        if (!fixedFilenameContracts[fixedFilenameContractId] || reference.adapter !== "json" && reference.adapter !== "toml")
          throw new Error(`Taxonomy v7 ${name}.configurableEntry.configurationReferences[${index}] is invalid`);
        return { fixedFilenameContractId, adapter: reference.adapter, structuredLocation: requiredString(reference.structuredLocation, `${name}.configurableEntry.configurationReferences[${index}].structuredLocation`) };
      });
      return { sourcePathSegments, destinationPathSegments, nodeType: "file", configurableEntry: { contractId, sourceFilename, configurationReferences } };
    }
    const pathSegments = parseSegments(spec.pathSegments, `${name}.pathSegments`);
    if (spec.nodeType === "directory") {
      const kindId = requiredString(spec.kindId, `${name}.kindId`);
      if (!allDirectoryContextIds.has(kindId) || spec.sourceFilename !== undefined || spec.fixedFilenameContractId !== undefined || spec.packageGlue !== undefined)
        throw new Error(`Taxonomy v7 ${name} references an invalid directory kind ${kindId}`);
      return { pathSegments, nodeType: "directory", kindId };
    }
    const authorities = [spec.kindId !== undefined, spec.fixedFilenameContractId !== undefined].filter(Boolean).length;
    if (authorities !== 1)
      throw new Error(`Taxonomy v7 ${name} must declare exactly one file authority`);
    if (spec.kindId !== undefined) {
      const kindId = requiredString(spec.kindId, `${name}.kindId`);
      if (!fileKinds[kindId])
        throw new Error(`Taxonomy v7 ${name} references unknown file kind ${kindId}`);
      const sourceFilename = spec.sourceFilename === undefined ? undefined : requiredString(spec.sourceFilename, `${name}.sourceFilename`).normalize("NFC");
      if (sourceFilename !== undefined && (kindId !== "rust-source" || sourceFilename !== "\uD83E\uDD80\uFE0Fcomponent.rs"))
        throw new Error(`Taxonomy v7 ${name}.sourceFilename is not the frozen Draw Rust source leaf`);
      return { pathSegments, nodeType: "file", kindId, ...sourceFilename ? { sourceFilename } : {} };
    }
    if (spec.fixedFilenameContractId !== undefined) {
      const fixedFilenameContractId = requiredString(spec.fixedFilenameContractId, `${name}.fixedFilenameContractId`);
      if (!fixedFilenameContracts[fixedFilenameContractId])
        throw new Error(`Taxonomy v7 ${name} references unknown fixed filename contract ${fixedFilenameContractId}`);
      return { pathSegments, nodeType: "file", fixedFilenameContractId };
    }
    throw new Error(`Taxonomy v7 ${name} has no file authority`);
  };
  const semanticDescendantContracts = {};
  for (const [id, value] of Object.entries(descendantContractRows)) {
    const spec = record(value, `semanticDescendantContracts.${id}`);
    const rootDirectoryKindId = requiredString(spec.rootDirectoryKindId, `semanticDescendantContracts.${id}.rootDirectoryKindId`);
    if (!allDirectoryContextIds.has(rootDirectoryKindId))
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references unknown root directory kind ${rootDirectoryKindId}`);
    if (spec.contractKind === "catalog") {
      const catalogContractId = requiredString(spec.catalogContractId, `semanticDescendantContracts.${id}.catalogContractId`);
      const leafFileKindId = requiredString(spec.leafFileKindId, `semanticDescendantContracts.${id}.leafFileKindId`);
      const reserve2 = record(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
      if (!fileKinds[leafFileKindId] || spec.rendering !== "semantic-member-directory-and-physical-kind-leaf" || reserve2.derivation !== "longest-rendered-catalog-descendant-suffix" || !Number.isSafeInteger(reserve2.bytes) || reserve2.bytes <= 0)
        throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} is not a valid catalog descendant contract`);
      semanticDescendantContracts[id] = { contractKind: "catalog", rootDirectoryKindId, catalogContractId, leafFileKindId, rendering: "semantic-member-directory-and-physical-kind-leaf", pathBudgetReserve: { derivation: "longest-rendered-catalog-descendant-suffix", bytes: reserve2.bytes } };
      continue;
    }
    if (!Array.isArray(spec.requiredNodes) || !Array.isArray(spec.exclusiveAlternatives))
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} node lists must be arrays`);
    const requiredNodes = spec.requiredNodes.map((node, index) => parseDescendantNode(node, `semanticDescendantContracts.${id}.requiredNodes[${index}]`));
    const exclusiveAlternatives = spec.exclusiveAlternatives.map((value2, index) => {
      const alternative = record(value2, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}]`);
      if (alternative.mode !== "exactly-one" || !Array.isArray(alternative.nodes) || alternative.nodes.length < 2)
        throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} alternative must contain exactly-one candidates`);
      return { id: requiredString(alternative.id, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].id`), mode: "exactly-one", nodes: alternative.nodes.map((node, nodeIndex) => parseDescendantNode(node, `semanticDescendantContracts.${id}.exclusiveAlternatives[${index}].nodes[${nodeIndex}]`)) };
    });
    const realizedRequiredCount = requiredNodes.length + requiredNodes.filter((node) => ("configurableEntry" in node)).length;
    if (!Number.isSafeInteger(spec.realizedNodeCount) || spec.realizedNodeCount !== realizedRequiredCount + exclusiveAlternatives.length)
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.realizedNodeCount is invalid`);
    const reserve = record(spec.pathBudgetReserve, `semanticDescendantContracts.${id}.pathBudgetReserve`);
    const suffix = (node) => {
      const segments = ("configurableEntry" in node ? node.destinationPathSegments : node.pathSegments).map((segment) => segment.literal);
      if (node.nodeType === "file") {
        if ("configurableEntry" in node)
          segments.push(configurableEntryContracts[node.configurableEntry.contractId].filename);
        else if ("kindId" in node) {
          const kind = fileKinds[node.kindId];
          if (kind.extensionChains.length !== 1)
            throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} file kind ${node.kindId} must have one physical extension chain`);
          segments.push(`${kind.emoji}${kind.extensionChains[0]}`);
        } else if ("fixedFilenameContractId" in node)
          segments.push(posix.basename(fixedFilenameContracts[node.fixedFilenameContractId].pathPattern));
        else
          throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} file authority is invalid`);
      }
      return segments.length === 0 ? "" : `/${segments.join("/")}`;
    };
    const reserveBytes = Math.max(...[...requiredNodes, ...exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].map((node) => Buffer.byteLength(suffix(node), "utf8")));
    if (reserve.derivation !== "longest-canonical-descendant-suffix" || reserve.bytes !== reserveBytes)
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id}.pathBudgetReserve is not derived from its longest suffix`);
    semanticDescendantContracts[id] = { rootDirectoryKindId, requiredNodes, exclusiveAlternatives, realizedNodeCount: spec.realizedNodeCount, pathBudgetReserve: { derivation: "longest-canonical-descendant-suffix", bytes: reserveBytes } };
  }
  if (Object.keys(semanticDescendantContracts).length === 0)
    throw new Error("Taxonomy v7 semanticDescendantContracts must not be empty");
  const semanticPathProjectionCatalogContracts = {};
  const expectedCatalogContract = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
  for (const [id, value] of Object.entries(projectionCatalogRows)) {
    const spec = record(value, `semanticPathProjectionCatalogContracts.${id}`);
    if (spec.contractKind === undefined) {
      if (canonicalJson(value) !== canonicalJson(expectedCatalogContract))
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not the independent required vector registry contract`);
      semanticPathProjectionCatalogContracts[id] = expectedCatalogContract;
      continue;
    }
    if (spec.contractKind === "distributed-json-manifest-catalog") {
      if (spec.modelIdentityField !== "id" || spec.memberIdentityField !== "id" || spec.memberVersionField !== "version" || spec.requiredModelManifest !== true || spec.coverage !== "every-source-file-and-destination-node-exactly-once" || spec.unknownCategoryPolicy !== "problem" || spec.unownedModelPolicy !== "problem" || !Array.isArray(spec.categoryRules) || spec.categoryRules.length === 0)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict distributed manifest catalog`);
      const categoryRules = spec.categoryRules.map((value2, index) => {
        const rule = record(value2, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}]`);
        const sourceDirectoryName = requiredString(rule.sourceDirectoryName, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceDirectoryName`).normalize("NFC");
        const directoryKindId = requiredString(rule.directoryKindId, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].directoryKindId`);
        const manifestSchema = requiredString(rule.manifestSchema, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].manifestSchema`);
        if (!semanticDirectoryKinds[directoryKindId])
          throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}] references an unknown directory kind`);
        if (rule.sourceShape === "direct-semantic-json")
          return { sourceDirectoryName, directoryKindId, sourceShape: "direct-semantic-json", manifestSchema, memberDirectoryEmoji: requiredString(rule.memberDirectoryEmoji, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].memberDirectoryEmoji`).normalize("NFC") };
        if (rule.sourceShape === "nested-fixed-json")
          return { sourceDirectoryName, directoryKindId, sourceShape: "nested-fixed-json", manifestSchema, fixedSourceFilename: requiredString(rule.fixedSourceFilename, `semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].fixedSourceFilename`).normalize("NFC") };
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.categoryRules[${index}].sourceShape is invalid`);
      });
      if (new Set(categoryRules.map((rule) => rule.sourceDirectoryName)).size !== categoryRules.length)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats a catalog category`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "distributed-json-manifest-catalog", ownerArtifactMemberName: requiredString(spec.ownerArtifactMemberName, `semanticPathProjectionCatalogContracts.${id}.ownerArtifactMemberName`).normalize("NFC"), modelManifestSchema: requiredString(spec.modelManifestSchema, `semanticPathProjectionCatalogContracts.${id}.modelManifestSchema`), modelManifestSourceFilename: requiredString(spec.modelManifestSourceFilename, `semanticPathProjectionCatalogContracts.${id}.modelManifestSourceFilename`).normalize("NFC"), modelIdentityField: "id", memberIdentityField: "id", memberVersionField: "version", requiredMemberVersion: requiredString(spec.requiredMemberVersion, `semanticPathProjectionCatalogContracts.${id}.requiredMemberVersion`), requiredModelManifest: true, categoryRules, coverage: "every-source-file-and-destination-node-exactly-once", unknownCategoryPolicy: "problem", unownedModelPolicy: "problem" };
      continue;
    }
    if (spec.contractKind === "exact-owner-vectors") {
      if (spec.required !== true || spec.allowEmpty !== false || canonicalJson(spec.identityFields) !== canonicalJson(["artifactId", "standardVersion", "subsetId", "commandDirectoryName"]) || spec.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(spec.vectors) || spec.vectors.length === 0)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} is not a strict exact-owner vector registry`);
      const vectors = spec.vectors.map((value2, index) => {
        const vector = record(value2, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}]`);
        return { artifactId: requiredString(vector.artifactId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].artifactId`).normalize("NFC"), standardVersion: requiredString(vector.standardVersion, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].standardVersion`), subsetId: requiredString(vector.subsetId, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].subsetId`), commandDirectoryName: requiredString(vector.commandDirectoryName, `semanticPathProjectionCatalogContracts.${id}.vectors[${index}].commandDirectoryName`).normalize("NFC") };
      });
      if (new Set(vectors.map((vector) => canonicalJson(vector))).size !== vectors.length)
        throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id} repeats an owner vector`);
      semanticPathProjectionCatalogContracts[id] = { contractKind: "exact-owner-vectors", required: true, allowEmpty: false, identityFields: ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], coverage: "every-physical-command-bundle-exactly-once", vectors };
      continue;
    }
    throw new Error(`Taxonomy v7 semanticPathProjectionCatalogContracts.${id}.contractKind is invalid`);
  }
  if (Object.keys(semanticPathProjectionCatalogContracts).length === 0)
    throw new Error("Taxonomy v7 semanticPathProjectionCatalogContracts must not be empty");
  for (const [id, contract] of Object.entries(semanticDescendantContracts))
    if ("contractKind" in contract && !semanticPathProjectionCatalogContracts[contract.catalogContractId])
      throw new Error(`Taxonomy v7 semanticDescendantContracts.${id} references an unknown catalog contract`);
  const captureFields = new Set(["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"]);
  const parseProjectionSegment = (value, name, destination) => {
    const spec = record(value, name);
    const kindId = typeof spec.kindId === "string" ? spec.kindId : undefined;
    const memberKindId = typeof spec.memberKindId === "string" ? spec.memberKindId : undefined;
    const projectedMemberKindId = typeof spec.projectedMemberKindId === "string" ? spec.projectedMemberKindId : undefined;
    if ((kindId ? 1 : 0) + (memberKindId ? 1 : 0) + (projectedMemberKindId ? 1 : 0) !== 1)
      throw new Error(`Taxonomy v7 ${name} must identify exactly one kind`);
    if (kindId && !allDirectoryContextIds.has(kindId))
      throw new Error(`Taxonomy v7 ${name} references unknown directory kind ${kindId}`);
    if (memberKindId && !semanticDirectoryMemberKinds[memberKindId])
      throw new Error(`Taxonomy v7 ${name} references unknown semantic member kind ${memberKindId}`);
    if (projectedMemberKindId && !semanticProjectedMemberKinds[projectedMemberKindId])
      throw new Error(`Taxonomy v7 ${name} references unknown projected member kind ${projectedMemberKindId}`);
    if (destination) {
      if (memberKindId)
        throw new Error(`Taxonomy v7 ${name} cannot render a source member kind`);
      if (spec.literal !== undefined && kindId)
        return { kindId, literal: requiredString(spec.literal, `${name}.literal`) };
      if (spec.render === "profile" && kindId)
        return { kindId, render: "profile" };
      if (typeof spec.copy === "string" && captureFields.has(spec.copy))
        return projectedMemberKindId ? { projectedMemberKindId, copy: spec.copy } : { kindId, copy: spec.copy };
    } else {
      if (spec.literal !== undefined && kindId)
        return { kindId, literal: requiredString(spec.literal, `${name}.literal`) };
      if (spec.literal !== undefined && memberKindId) {
        const literal = requiredString(spec.literal, `${name}.literal`).normalize("NFC");
        if (!semanticDirectoryMemberKinds[memberKindId].memberNames.includes(literal))
          throw new Error(`Taxonomy v7 ${name}.literal is not registered by ${memberKindId}`);
        return { memberKindId, literal };
      }
      if (typeof spec.capture === "string" && captureFields.has(spec.capture))
        return projectedMemberKindId ? { projectedMemberKindId, capture: spec.capture } : { kindId, capture: spec.capture };
    }
    throw new Error(`Taxonomy v7 ${name} has an invalid ${destination ? "destination" : "source"} operation`);
  };
  const semanticPathProjectionContracts = {};
  for (const [id, value] of Object.entries(projectionRows)) {
    const spec = record(value, `semanticPathProjectionContracts.${id}`);
    if (!Array.isArray(spec.sourceSegments) || !Array.isArray(spec.destinationSegments) || !["artifact-mutation-test-projection-v1", "artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(String(spec.rationaleRule)))
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} is invalid`);
    const sourceOwnerKindId = requiredString(spec.sourceOwnerKindId, `semanticPathProjectionContracts.${id}.sourceOwnerKindId`);
    const destinationOwnerKindId = requiredString(spec.destinationOwnerKindId, `semanticPathProjectionContracts.${id}.destinationOwnerKindId`);
    if (!semanticDirectoryMemberKinds[sourceOwnerKindId] || !semanticDirectoryMemberKinds[destinationOwnerKindId])
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} owner kind is invalid`);
    const profileRendererId = requiredString(spec.profileRendererId, `semanticPathProjectionContracts.${id}.profileRendererId`);
    const descendantContractId = requiredString(spec.descendantContractId, `semanticPathProjectionContracts.${id}.descendantContractId`);
    const catalogContractId = requiredString(spec.catalogContractId, `semanticPathProjectionContracts.${id}.catalogContractId`);
    if (!semanticPathProjectionProfileRenderers[profileRendererId] || !semanticDescendantContracts[descendantContractId] || !semanticPathProjectionCatalogContracts[catalogContractId])
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references an unknown registry`);
    const rationaleRule = spec.rationaleRule;
    const sourceArtifactMemberName = spec.sourceArtifactMemberName === undefined ? undefined : requiredString(spec.sourceArtifactMemberName, `semanticPathProjectionContracts.${id}.sourceArtifactMemberName`).normalize("NFC");
    const expectedArtifact = rationaleRule === "artifact-example-model-catalog-projection-v1" ? "\uD83D\uDCD0\uFE0Fcad" : rationaleRule === "artifact-editor-command-projection-v1" ? "\uD83D\uDD8D\uFE0Fdraw" : undefined;
    if (sourceArtifactMemberName !== expectedArtifact)
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id}.sourceArtifactMemberName does not match its rationale`);
    const sourceSegments = spec.sourceSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.sourceSegments[${index}]`, false));
    const destinationSegments = spec.destinationSegments.map((segment, index) => parseProjectionSegment(segment, `semanticPathProjectionContracts.${id}.destinationSegments[${index}]`, true));
    const captures = sourceSegments.flatMap((segment) => ("capture" in segment) ? [segment.capture] : []);
    const expectedCaptures = rationaleRule === "artifact-mutation-test-projection-v1" ? ["standardVersion", "subsetId", "mutationId", "scenarioId"] : rationaleRule === "artifact-editor-command-projection-v1" ? ["standardVersion", "subsetId", "commandDirectoryName"] : ["standardVersion", "subsetId"];
    if (canonicalJson(captures) !== canonicalJson(expectedCaptures))
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid captures for ${rationaleRule}`);
    const descendant = semanticDescendantContracts[descendantContractId];
    const catalog = semanticPathProjectionCatalogContracts[catalogContractId];
    if (rationaleRule === "artifact-mutation-test-projection-v1" ? "contractKind" in descendant || "contractKind" in catalog : rationaleRule === "artifact-example-model-catalog-projection-v1" ? !(("contractKind" in descendant) && descendant.contractKind === "catalog" && ("contractKind" in catalog) && catalog.contractKind === "distributed-json-manifest-catalog") : ("contractKind" in descendant) || !(("contractKind" in catalog) && catalog.contractKind === "exact-owner-vectors"))
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} references incompatible descendant/catalog authorities`);
    const descendantNodes = "contractKind" in descendant ? [] : [...descendant.requiredNodes, ...descendant.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)];
    const sourceNamedNodes = descendantNodes.filter((node) => ("kindId" in node) && node.sourceFilename !== undefined);
    if (rationaleRule === "artifact-editor-command-projection-v1" ? sourceNamedNodes.length !== 3 || descendantNodes.filter((node) => ("kindId" in node) && node.nodeType === "file" && node.kindId === "rust-source").length !== 3 : sourceNamedNodes.length !== 0)
      throw new Error(`Taxonomy v7 semanticPathProjectionContracts.${id} has invalid source-filename descendant authority`);
    semanticPathProjectionContracts[id] = { sourceOwnerKindId, ...sourceArtifactMemberName ? { sourceArtifactMemberName } : {}, sourceSegments, profileRendererId, destinationOwnerKindId, destinationSegments, descendantContractId, catalogContractId, rationaleRule };
  }
  if (Object.keys(semanticPathProjectionContracts).length === 0)
    throw new Error("Taxonomy v7 semanticPathProjectionContracts must not be empty");
  for (const [id, spec] of Object.entries(semanticProjectedMemberKinds))
    if (!semanticPathProjectionContracts[spec.projectionContractId])
      throw new Error(`Taxonomy v7 semanticProjectedMemberKinds.${id} references unknown projection contract ${spec.projectionContractId}`);
  const semanticOwnedFileProjectionContracts = {};
  for (const [id, value] of Object.entries(ownedFileProjectionRows)) {
    const name = `semanticOwnedFileProjectionContracts.${id}`;
    const spec = record(value, name);
    requireExactKeys(spec, ["contractKind", "ownerFixedDirectoryContractId", "requiredSiblingFixedFilenameContractId", "manifestAdapter", "manifestStatusLocation", "allowedStatuses", "sourceFileKindId", "sourceFilename", "destinationDirectoryKindId", "destinationDirectoryName", "destinationFilename", "emptyContentRule", "statusDispositions", "rationaleRule"], name);
    const allowedStatuses = stringArray(spec.allowedStatuses, `${name}.allowedStatuses`);
    const statusDispositions = record(spec.statusDispositions, `${name}.statusDispositions`);
    requireExactKeys(statusDispositions, ["open", "closed-empty", "closed-nonempty", "invalid"], `${name}.statusDispositions`);
    const ownerFixedDirectoryContractId = requiredString(spec.ownerFixedDirectoryContractId, `${name}.ownerFixedDirectoryContractId`);
    const requiredSiblingFixedFilenameContractId = requiredString(spec.requiredSiblingFixedFilenameContractId, `${name}.requiredSiblingFixedFilenameContractId`);
    const sourceFileKindId = requiredString(spec.sourceFileKindId, `${name}.sourceFileKindId`);
    const destinationDirectoryKindId = requiredString(spec.destinationDirectoryKindId, `${name}.destinationDirectoryKindId`);
    if (spec.contractKind !== "owner-sibling-manifest-file" || spec.manifestAdapter !== "json" || spec.manifestStatusLocation !== "status" || canonicalJson(allowedStatuses) !== canonicalJson(["closed", "open"]) || spec.emptyContentRule !== "zero-byte" || spec.rationaleRule !== "ticket-important-markdown-projection-v1" || canonicalJson(statusDispositions) !== canonicalJson({ open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }))
      throw new Error(`Taxonomy v7 ${name} does not use the exact owner-file projection grammar`);
    if (!fixedDirectoryRows[ownerFixedDirectoryContractId] || !fixedFilenameContracts[requiredSiblingFixedFilenameContractId] || !fileKinds[sourceFileKindId] || semanticDirectoryKinds[destinationDirectoryKindId]?.projectionOnly !== true)
      throw new Error(`Taxonomy v7 ${name} references unknown or non-projection authority`);
    semanticOwnedFileProjectionContracts[id] = { contractKind: "owner-sibling-manifest-file", ownerFixedDirectoryContractId, requiredSiblingFixedFilenameContractId, manifestAdapter: "json", manifestStatusLocation: "status", allowedStatuses: ["closed", "open"], sourceFileKindId, sourceFilename: requiredString(spec.sourceFilename, `${name}.sourceFilename`), destinationDirectoryKindId, destinationDirectoryName: requiredString(spec.destinationDirectoryName, `${name}.destinationDirectoryName`), destinationFilename: requiredString(spec.destinationFilename, `${name}.destinationFilename`), emptyContentRule: "zero-byte", statusDispositions: { open: "project", "closed-empty": "remove", "closed-nonempty": "problem", invalid: "problem" }, rationaleRule: "ticket-important-markdown-projection-v1" };
  }
  if (canonicalJson(Object.keys(semanticOwnedFileProjectionContracts)) !== canonicalJson(["ticket-important-markdown-v1"]))
    throw new Error("Taxonomy v7 semanticOwnedFileProjectionContracts must contain exactly ticket-important-markdown-v1");
  const semanticPathProjectionReferenceConsumerContracts = {};
  const referenceConsumerForms = new Set(["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:interaction-glob", "artifact-catalog-prose:catalog-grammar"]);
  const referenceConsumerAdapters = new Set(["rust", "typescript", "json", "toml"]);
  const referenceConsumerIdentities = new Set;
  for (const [id, value] of Object.entries(projectionConsumerRows)) {
    const spec = record(value, `semanticPathProjectionReferenceConsumerContracts.${id}`);
    requireExactKeys(spec, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], `semanticPathProjectionReferenceConsumerContracts.${id}`);
    const projectionContractId = requiredString(spec.projectionContractId, `semanticPathProjectionReferenceConsumerContracts.${id}.projectionContractId`);
    if (!semanticPathProjectionContracts[projectionContractId])
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} references an unknown projection contract`);
    if (spec.ownership !== "external")
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.ownership must be external`);
    const consumerIdentity = requiredString(spec.consumerIdentity, `semanticPathProjectionReferenceConsumerContracts.${id}.consumerIdentity`);
    if (referenceConsumerIdentities.has(consumerIdentity))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts repeats consumer identity ${consumerIdentity}`);
    referenceConsumerIdentities.add(consumerIdentity);
    const sourcePathPattern = requiredString(spec.sourcePathPattern, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern`);
    if (!sourcePathPattern.startsWith("^") || !sourcePathPattern.endsWith("$"))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathPattern must be a full-match expression`);
    const sourcePathRegex = new RegExp(sourcePathPattern, "u");
    const sourcePathIdentities = stringArray(spec.sourcePathIdentities, `semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities`);
    const adapters = stringArray(spec.adapters, `semanticPathProjectionReferenceConsumerContracts.${id}.adapters`);
    const supportedForms = stringArray(spec.supportedForms, `semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms`);
    const staleMarkers = stringArray(spec.staleMarkers, `semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers`);
    if (sourcePathIdentities.length === 0 || adapters.length === 0 || supportedForms.length === 0 || staleMarkers.length === 0)
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} must be nonempty`);
    if (new Set(sourcePathIdentities).size !== sourcePathIdentities.length || sourcePathIdentities.some((path2) => path2 !== normalizeRelative(path2) || !sourcePathRegex.test(path2)))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.sourcePathIdentities are invalid`);
    if (new Set(adapters).size !== adapters.length || adapters.some((adapter) => !referenceConsumerAdapters.has(adapter)))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.adapters are invalid`);
    if (new Set(supportedForms).size !== supportedForms.length || supportedForms.some((form) => !referenceConsumerForms.has(form)))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.supportedForms are invalid`);
    if (new Set(staleMarkers).size !== staleMarkers.length || staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC")))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id}.staleMarkers are invalid`);
    semanticPathProjectionReferenceConsumerContracts[id] = { projectionContractId, consumerIdentity, ownership: "external", sourcePathPattern, sourcePathIdentities: [...sourcePathIdentities], adapters: [...adapters], supportedForms: [...supportedForms], staleMarkers: [...staleMarkers] };
  }
  if (Object.keys(semanticPathProjectionReferenceConsumerContracts).length === 0)
    throw new Error("Taxonomy v7 semanticPathProjectionReferenceConsumerContracts must not be empty");
  const mutationCatalogProjection = {
    projectionContractId: requiredString(mutationCatalogProjectionRow.projectionContractId, "mutationCatalogProjection.projectionContractId"),
    projectedMemberKindId: requiredString(mutationCatalogProjectionRow.projectedMemberKindId, "mutationCatalogProjection.projectedMemberKindId"),
    descendantContractId: requiredString(mutationCatalogProjectionRow.descendantContractId, "mutationCatalogProjection.descendantContractId"),
    catalogContractId: requiredString(mutationCatalogProjectionRow.catalogContractId, "mutationCatalogProjection.catalogContractId")
  };
  if (!semanticPathProjectionContracts[mutationCatalogProjection.projectionContractId] || !semanticProjectedMemberKinds[mutationCatalogProjection.projectedMemberKindId] || !semanticDescendantContracts[mutationCatalogProjection.descendantContractId] || !semanticPathProjectionCatalogContracts[mutationCatalogProjection.catalogContractId])
    throw new Error("Taxonomy v7 mutationCatalogProjection references unknown projection registries");
  const generatorContracts = {};
  const generatorRoots = [];
  for (const [id, value] of Object.entries(generatorRows)) {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id))
      throw new Error(`Taxonomy v7 generatorContracts.${id} has an invalid identifier`);
    const spec = record(value, `generatorContracts.${id}`);
    if (spec.ownership !== "owned" && spec.ownership !== "external")
      throw new Error(`Taxonomy v7 generatorContracts.${id}.ownership is invalid`);
    const ownership = spec.ownership;
    const ownerPath = spec.ownerPath === null ? null : normalizeRelative(requiredString(spec.ownerPath, `generatorContracts.${id}.ownerPath`));
    const target = spec.target === null ? null : requiredString(spec.target, `generatorContracts.${id}.target`);
    const previewTarget = spec.previewTarget === undefined ? undefined : requiredString(spec.previewTarget, `generatorContracts.${id}.previewTarget`);
    const checkTarget = spec.checkTarget === undefined ? undefined : requiredString(spec.checkTarget, `generatorContracts.${id}.checkTarget`);
    if (ownership === "owned" !== (ownerPath !== null && target !== null))
      throw new Error(`Taxonomy v7 generatorContracts.${id} owner and target do not match ownership`);
    if (target && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(target))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.target must be one exact Nx target`);
    if (ownership === "owned" ? !previewTarget : previewTarget !== undefined)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget does not match ownership`);
    if (previewTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(previewTarget))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget must be one exact Nx target`);
    if (target && previewTarget !== `${target.slice(0, target.lastIndexOf(":"))}:preview-generated`)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget must be the exact owner preview-generated target`);
    if (checkTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(checkTarget))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.checkTarget must be one exact Nx target`);
    const inputPatterns = stringArray(spec.inputPatterns, `generatorContracts.${id}.inputPatterns`).map((pattern, index) => validatedContractPattern(pattern, `generatorContracts.${id}.inputPatterns[${index}]`, false));
    if (ownership === "owned" ? inputPatterns.length === 0 : inputPatterns.length !== 0)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.inputPatterns do not match ownership`);
    const outputRows = spec.outputRoots;
    if (!Array.isArray(outputRows) || outputRows.length === 0)
      throw new Error(`Taxonomy v7 generatorContracts.${id}.outputRoots must not be empty`);
    const outputRoots = outputRows.map((value2, index) => {
      const output = record(value2, `generatorContracts.${id}.outputRoots[${index}]`);
      const outputPath = requiredString(output.path, `generatorContracts.${id}.outputRoots[${index}].path`);
      if (outputPath !== normalizeRelative(outputPath) || /[*?\[\]]/u.test(outputPath))
        throw new Error(`Taxonomy v7 generatorContracts.${id} output path must be one literal NFC repository path`);
      if (output.inclusion !== "tracked" && output.inclusion !== "ignored")
        throw new Error(`Taxonomy v7 generatorContracts.${id} output inclusion is invalid`);
      generatorRoots.push({ id, path: outputPath });
      return { path: outputPath, inclusion: output.inclusion };
    }).sort((left, right) => left.path.localeCompare(right.path));
    if (new Set(outputRoots.map((output) => output.path)).size !== outputRoots.length)
      throw new Error(`Taxonomy v7 generatorContracts.${id} repeats an output root`);
    generatorContracts[id] = { ownership, ownerPath, target, previewTarget, checkTarget, inputPatterns: [...new Set(inputPatterns)].sort(), outputRoots, reason: requiredString(spec.reason, `generatorContracts.${id}.reason`) };
  }
  if (Object.keys(generatorContracts).length === 0)
    throw new Error("Taxonomy v7 generatorContracts must not be empty");
  for (let left = 0;left < generatorRoots.length; left++)
    for (let right = left + 1;right < generatorRoots.length; right++) {
      const a = generatorRoots[left];
      const b = generatorRoots[right];
      if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`))
        throw new Error(`Taxonomy v7 generator output roots overlap: ${a.id}:${a.path} and ${b.id}:${b.path}`);
    }
  const ecosystems = {};
  for (const [id, value] of Object.entries(ecosystemRows)) {
    const spec = record(value, `ecosystems.${id}`);
    if (spec.packageIdentity !== "manifest" && spec.packageIdentity !== "boundary-only")
      throw new Error(`Taxonomy v7 ecosystems.${id}.packageIdentity is invalid`);
    const manifestContractId = spec.manifestContractId === null ? null : requiredString(spec.manifestContractId, `ecosystems.${id}.manifestContractId`);
    if (spec.packageIdentity === "manifest" !== (manifestContractId !== null))
      throw new Error(`Taxonomy v7 ecosystems.${id} manifest identity is incomplete`);
    ecosystems[id] = { packageIdentity: spec.packageIdentity, manifestContractId };
  }
  if (Object.keys(ecosystems).length === 0)
    throw new Error("Taxonomy v7 ecosystems must not be empty");
  const packageGlueGrammar = {};
  for (const [id, value] of Object.entries(grammarRows)) {
    const spec = record(value, `packageGlueGrammar.${id}`);
    if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp"].includes(String(spec.analyzer)))
      throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.analyzer is invalid`);
    const allowedRoles = stringArray(spec.allowedRoles, `packageGlueGrammar.${id}.allowedRoles`);
    if (allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role)) || new Set(allowedRoles).size !== allowedRoles.length)
      throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.allowedRoles is invalid`);
    if (!Number.isSafeInteger(spec.maxDelegationStatements) || spec.maxDelegationStatements < 0)
      throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.maxDelegationStatements is invalid`);
    packageGlueGrammar[id] = { analyzer: spec.analyzer, allowedRoles, maxDelegationStatements: spec.maxDelegationStatements };
  }
  const packageBoundaryRules = {};
  for (const [id, value] of Object.entries(boundaryRows)) {
    const spec = record(value, `packageBoundaryRules.${id}`);
    const glueGrammarId = requiredString(spec.glueGrammarId, `packageBoundaryRules.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId])
      throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown grammar ${glueGrammarId}`);
    if (spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem")
      throw new Error(`Taxonomy v7 packageBoundaryRules.${id} must be recursive and fail closed`);
    packageBoundaryRules[id] = {
      manifestContractId: spec.manifestContractId === null ? null : requiredString(spec.manifestContractId, `packageBoundaryRules.${id}.manifestContractId`),
      entryContractIds: stringArray(spec.entryContractIds, `packageBoundaryRules.${id}.entryContractIds`),
      allowedFixedContractIds: stringArray(spec.allowedFixedContractIds, `packageBoundaryRules.${id}.allowedFixedContractIds`),
      allowedFileKindIds: stringArray(spec.allowedFileKindIds, `packageBoundaryRules.${id}.allowedFileKindIds`),
      allowedDirectoryKindIds: stringArray(spec.allowedDirectoryKindIds, `packageBoundaryRules.${id}.allowedDirectoryKindIds`),
      glueGrammarId,
      recursive: true,
      uncertainRole: "problem",
      implementationRole: "problem"
    };
    const rule = packageBoundaryRules[id];
    if (rule.manifestContractId && !fixedFilenameContracts[rule.manifestContractId])
      throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown manifest contract ${rule.manifestContractId}`);
    for (const contractId of rule.entryContractIds)
      if (!configurableEntryContracts[contractId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown entry contract ${contractId}`);
    for (const contractId of rule.allowedFixedContractIds)
      if (!fixedFilenameContracts[contractId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown fixed contract ${contractId}`);
    for (const kindId of rule.allowedFileKindIds)
      if (!fileKinds[kindId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown file kind ${kindId}`);
    for (const kindId of rule.allowedDirectoryKindIds)
      if (!semanticDirectoryKinds[kindId])
        throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown directory kind ${kindId}`);
  }
  const packageBoundaryProfiles = {};
  for (const [id, value] of Object.entries(boundaryProfileRows)) {
    const spec = record(value, `packageBoundaryProfiles.${id}`);
    requireExactKeys(spec, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles.${id}`);
    if (spec.admission !== "blocked-until-language-directory-registered" || spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem")
      throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} must remain fail-closed`);
    const glueGrammarId = requiredString(spec.glueGrammarId, `packageBoundaryProfiles.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId])
      throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} references unknown grammar`);
    packageBoundaryProfiles[id] = { admission: "blocked-until-language-directory-registered", allowedFileKindIds: stringArray(spec.allowedFileKindIds, `packageBoundaryProfiles.${id}.allowedFileKindIds`), allowedDirectoryKindIds: stringArray(spec.allowedDirectoryKindIds, `packageBoundaryProfiles.${id}.allowedDirectoryKindIds`), allowedFixedContractIds: stringArray(spec.allowedFixedContractIds, `packageBoundaryProfiles.${id}.allowedFixedContractIds`), glueGrammarId, recursive: true, uncertainRole: "problem", implementationRole: "problem", reason: requiredString(spec.reason, `packageBoundaryProfiles.${id}.reason`) };
  }
  if (Object.keys(packageBoundaryProfiles).length === 0)
    throw new Error("Taxonomy v7 packageBoundaryProfiles must not be empty");
  const packageSourceDispositions = {};
  for (const [id, value] of Object.entries(sourceDispositionRows)) {
    const spec = record(value, `packageSourceDispositions.${id}`);
    requireExactKeys(spec, ["contractKind", "disposition", "validator", "authority", "verification"], `packageSourceDispositions.${id}`);
    if (spec.contractKind !== "fixed" && spec.contractKind !== "configurable" || spec.disposition !== "adapter-source" && spec.disposition !== "tool-metadata" || spec.validator !== "package-glue" && spec.validator !== "command-router")
      throw new Error(`Taxonomy v7 packageSourceDispositions.${id} is invalid`);
    packageSourceDispositions[id] = { contractKind: spec.contractKind, disposition: spec.disposition, validator: spec.validator, authority: requiredString(spec.authority, `packageSourceDispositions.${id}.authority`), verification: requiredString(spec.verification, `packageSourceDispositions.${id}.verification`) };
  }
  if (Object.keys(packageSourceDispositions).length === 0)
    throw new Error("Taxonomy v7 packageSourceDispositions must not be empty");
  for (const [id, contract] of Object.entries(fixedFilenameContracts))
    if (contract.scope.kind === "package-root" && !packageBoundaryRules[contract.scope.ecosystemId])
      throw new Error(`Taxonomy v7 fixedFilenameContracts.${id} references unknown ecosystem ${contract.scope.ecosystemId}`);
  const pathExclusions = {};
  const exclusions = [];
  for (const [id, value] of Object.entries(exclusionRows)) {
    const spec = record(value, `pathExclusions.${id}`);
    if (spec.mode !== "opaque")
      throw new Error(`Taxonomy v7 pathExclusions.${id}.mode must be opaque`);
    const excludedPath = normalizeRelative(requiredString(spec.path, `pathExclusions.${id}.path`));
    pathExclusions[id] = { path: excludedPath, mode: "opaque", reason: requiredString(spec.reason, `pathExclusions.${id}.reason`) };
    exclusions.push({ id, path: excludedPath });
  }
  if (canonicalJson(Object.entries(pathExclusions).map(([id, spec]) => [id, spec.path])) !== canonicalJson([["compose", "compose"], ["temp-compose", "temp/compose"]]))
    throw new Error("Taxonomy v7 pathExclusions must contain exactly opaque compose and temp/compose");
  for (const id of stringArray(enforcement.opaquePathExclusionIds, "areaEnforcement.opaquePathExclusionIds")) {
    if (!pathExclusions[id])
      throw new Error(`Taxonomy v7 areaEnforcement references unknown opaque exclusion ${id}`);
  }
  if (canonicalJson(enforcement.opaquePathExclusionIds) !== canonicalJson(["compose", "temp-compose"]))
    throw new Error("Taxonomy v7 areaEnforcement must require compose and temp-compose in order");
  const opaquePaths = Object.values(pathExclusions).map((entry) => entry.path);
  const crossesOpaque = (value) => opaquePaths.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
  for (const [id, contract] of Object.entries(semanticPathProjectionReferenceConsumerContracts)) {
    if (contract.sourcePathIdentities.some(crossesOpaque))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} crosses an opaque path`);
    const pattern = new RegExp(contract.sourcePathPattern, "u");
    if (opaquePaths.some((opaque) => pattern.test(opaque) || pattern.test(`${opaque}/probe`)))
      throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} admits an opaque path`);
  }
  for (const [id, contract] of Object.entries(generatorContracts)) {
    if (contract.ownerPath && crossesOpaque(contract.ownerPath))
      throw new Error(`Taxonomy v7 generatorContracts.${id}.ownerPath crosses an opaque path`);
    for (const pattern of contract.inputPatterns)
      if (opaquePaths.some((opaque) => taxonomyPathPatternMatches(opaque, pattern) || taxonomyPathPatternMatches(`${opaque}/probe`, pattern)))
        throw new Error(`Taxonomy v7 generatorContracts.${id} input pattern admits an opaque path`);
    for (const output of contract.outputRoots)
      if (crossesOpaque(output.path))
        throw new Error(`Taxonomy v7 generatorContracts.${id} output root crosses an opaque path`);
  }
  const schema = {
    schemaVersion: 7,
    fileKinds,
    semanticDirectoryKinds,
    fixedFilenameContracts,
    fixedFilenameRejectionContracts,
    fixedDirectoryContracts,
    configurableEntryContracts,
    fileKindResolutionRules,
    scopedFileKinds,
    semanticDirectoryMemberKinds,
    semanticProjectedMemberKinds,
    semanticPathProjectionProfileRenderers,
    semanticDescendantContracts,
    semanticPathProjectionCatalogContracts,
    semanticPathProjectionContracts,
    semanticOwnedFileProjectionContracts,
    semanticPathProjectionReferenceConsumerContracts,
    mutationCatalogProjection,
    generatorContracts,
    ecosystems,
    packageBoundaryRules,
    packageBoundaryProfiles,
    packageGlueGrammar,
    packageSourceDispositions,
    pathExclusions,
    unicodeNormalization: { form: "NFC", caseFold: "lower", locale: "und" },
    variationSelectorPolicy: { selector: "\uFE0F", requiredAfterEmoji: true, comparison: "ignore-selector" },
    collisionPolicy: {
      comparisons: collision.comparisons,
      maxPathBytes: collision.maxPathBytes,
      rejectWindowsReservedNames: collision.rejectWindowsReservedNames === true,
      rejectTrailingDotsAndSpaces: collision.rejectTrailingDotsAndSpaces === true
    },
    areaEnforcement: { requiredState: "clean", undeclaredAreas: "enforce", opaquePathExclusionIds: [...enforcement.opaquePathExclusionIds] }
  };
  return {
    path,
    schema,
    exclusions: exclusions.sort((a, b) => a.path.localeCompare(b.path)),
    fileKinds: Object.entries(fileKinds).map(([id, spec]) => ({ id, ...spec })).sort((a, b) => a.id.localeCompare(b.id)),
    directoryKinds: Object.entries(semanticDirectoryKinds).map(([id, spec]) => ({ id, ...spec, slugRegex: new RegExp(`^(?:${spec.slugPattern})$`, "u") })).sort((a, b) => a.id.localeCompare(b.id))
  };
}
function loadTaxonomy2(options) {
  const path = assertLexicalInputOutsideOpaque(options.repoRoot, options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath", true);
  return parseTaxonomy(JSON.parse(readFileSync2(path, "utf8")), path);
}
function sha256(value) {
  return createHash2("sha256").update(value).digest("hex");
}
function canonicalArrayKey(value) {
  if (!value || typeof value !== "object" || Array.isArray(value))
    return null;
  const row = value;
  const keys = ["operationId", "sourcePath", "path", "id", "destinationPath", "code", "relativeRoot", "structuredLocation"];
  const parts = keys.filter((key) => typeof row[key] === "string").map((key) => `${key}:${row[key]}`);
  return parts.length > 0 ? parts.join("\x00") : null;
}
function canonicalValue(value) {
  if (Array.isArray(value)) {
    const rows = value.map(canonicalValue);
    if (rows.every((row) => canonicalArrayKey(row) !== null))
      return [...rows].sort((a, b) => Buffer.from(canonicalArrayKey(a)).compare(Buffer.from(canonicalArrayKey(b))));
    return rows;
  }
  if (!value || typeof value !== "object")
    return value;
  const source = value;
  const target = {};
  for (const key of Object.keys(source).sort()) {
    if (source[key] !== undefined)
      target[key] = canonicalValue(source[key]);
  }
  return target;
}
function canonicalJson(value) {
  return JSON.stringify(canonicalValue(value));
}
var PLAN_HASH = /^[a-f0-9]{64}$/u;
var PLAN_OPERATION_ID = /^[a-f0-9]{24}$/u;
var PLAN_COMMIT_ID = /^[a-f0-9]{40}$/u;
function planRecord(value, name, requiredKeys, optionalKeys = []) {
  if (!value || typeof value !== "object" || Array.isArray(value))
    throw new Error(`${name} must be an object`);
  const row = value;
  const allowed = new Set([...requiredKeys, ...optionalKeys]);
  const keys = Object.keys(row);
  if (requiredKeys.some((key) => !(key in row)) || keys.some((key) => !allowed.has(key)))
    throw new Error(`${name} has missing or unknown keys`);
  return row;
}
function planString(value, name, pattern) {
  if (typeof value !== "string" || pattern && !pattern.test(value))
    throw new Error(`${name} is invalid`);
  return value;
}
function planPath(value, name) {
  const path = planString(value, name);
  if (path === "" || path !== normalizeRelative(path) || path !== path.normalize("NFC"))
    throw new Error(`${name} is not a canonical repository-relative path`);
  return path;
}
function planInteger(value, name, maximum = Number.MAX_SAFE_INTEGER) {
  if (!Number.isSafeInteger(value) || value < 0 || value > maximum)
    throw new Error(`${name} is invalid`);
  return value;
}
function planStringArray(value, name, pattern) {
  if (!Array.isArray(value))
    throw new Error(`${name} must be an array`);
  return value.map((entry, index) => planString(entry, `${name}[${index}]`, pattern));
}
function parseLeafPreimage(value, name) {
  const base = planRecord(value, name, ["nodeKind", "contentHash", "mode", "size"], ["target"]);
  const row = base.nodeKind === "symlink" ? planRecord(value, name, ["nodeKind", "contentHash", "mode", "size", "target"]) : planRecord(value, name, ["nodeKind", "contentHash", "mode", "size"]);
  if (row.nodeKind !== "file" && row.nodeKind !== "symlink")
    throw new Error(`${name}.nodeKind is invalid`);
  const contentHash = planString(row.contentHash, `${name}.contentHash`, PLAN_HASH);
  const mode = planInteger(row.mode, `${name}.mode`, 4095);
  const size = planInteger(row.size, `${name}.size`);
  if (row.nodeKind === "file")
    return { nodeKind: "file", contentHash, mode, size };
  const target = planString(row.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size)
    throw new Error(`${name} symlink target does not match its hash and size`);
  return { nodeKind: "symlink", contentHash, mode, size, target };
}
function parsePathPreimage(value, name) {
  const base = planRecord(value, name, ["state"], ["contentHash", "mode", "size", "target"]);
  if (!["absent", "directory", "file", "symlink"].includes(String(base.state)))
    throw new Error(`${name}.state is invalid`);
  if (base.state === "absent" || base.state === "directory") {
    if (Object.keys(base).length !== 1)
      throw new Error(`${name} absent preimage cannot have payload`);
    return { state: base.state };
  }
  if (typeof base.contentHash !== "string" || !PLAN_HASH.test(base.contentHash) || !Number.isSafeInteger(base.mode) || !Number.isSafeInteger(base.size))
    throw new Error(`${name} present preimage requires hash, mode and size`);
  const contentHash = base.contentHash, mode = planInteger(base.mode, `${name}.mode`, 4095), size = planInteger(base.size, `${name}.size`);
  if (base.state === "file") {
    if (base.target !== undefined)
      throw new Error(`${name} file preimage cannot have a symlink target`);
    return { state: "file", contentHash, mode, size };
  }
  const target = planString(base.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size)
    throw new Error(`${name} symlink target does not match its hash and size`);
  return { state: "symlink", contentHash, mode, size, target };
}
function parseNoFollowTreeDigest(value, name) {
  const row = planRecord(value, name, ["algorithm", "digest", "files", "directories", "symlinks", "others"]);
  if (row.algorithm !== "sha256-no-follow-merkle-v1")
    throw new Error(`${name}.algorithm is invalid`);
  return { algorithm: row.algorithm, digest: planString(row.digest, `${name}.digest`, PLAN_HASH), files: planInteger(row.files, `${name}.files`), directories: planInteger(row.directories, `${name}.directories`), symlinks: planInteger(row.symlinks, `${name}.symlinks`), others: planInteger(row.others, `${name}.others`) };
}
function dispositionOperationId(domain, value) {
  return sha256(`${domain}\x00${canonicalJson(value)}`).slice(0, 24);
}
function parseEvidenceMember(value, name) {
  const row = planRecord(value, name, ["sourcePath", "finalPath", "disposition", "preimage"]);
  if (!["remove", "retain", "relocate"].includes(String(row.disposition)))
    throw new Error(`${name}.disposition is invalid`);
  return { sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), finalPath: planPath(row.finalPath, `${name}.finalPath`), disposition: row.disposition, preimage: parseLeafPreimage(row.preimage, `${name}.preimage`) };
}
function parseRemovalAuthority(value, name) {
  const candidate = planRecord(value, name, ["kind"], ["evidenceSetDigest", "retainedFinalPath", "members", "fixturePath", "fixtureContentHash", "caseId", "serializedInputPath", "expectedViolationCode", "authorityDigest"]);
  if (candidate.kind === "byte-and-mode-identical") {
    const row = planRecord(value, name, ["kind", "evidenceSetDigest", "retainedFinalPath", "members"]);
    if (!Array.isArray(row.members) || row.members.length < 2)
      throw new Error(`${name}.members must contain complete retained evidence`);
    const members = row.members.map((entry, index) => parseEvidenceMember(entry, `${name}.members[${index}]`));
    const keys = members.map((entry) => Buffer.from(entry.sourcePath).toString("hex"));
    if (keys.some((key, index) => index > 0 && keys[index - 1] >= key))
      throw new Error(`${name}.members are not unique and bytewise path sorted`);
    const identity = canonicalJson(members[0].preimage);
    if (members.some((entry) => canonicalJson(entry.preimage) !== identity))
      throw new Error(`${name}.members are not byte, kind, mode and size identical`);
    const retainedFinalPath = planPath(row.retainedFinalPath, `${name}.retainedFinalPath`);
    if (!members.some((entry) => entry.disposition !== "remove" && entry.finalPath === retainedFinalPath))
      throw new Error(`${name}.retainedFinalPath has no retained member`);
    const digestible = { algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath };
    const evidenceSetDigest = planString(row.evidenceSetDigest, `${name}.evidenceSetDigest`, PLAN_HASH);
    if (evidenceSetDigest !== sha256(canonicalJson(digestible)))
      throw new Error(`${name}.evidenceSetDigest does not match its members`);
    return { kind: "byte-and-mode-identical", evidenceSetDigest, retainedFinalPath, members };
  }
  if (candidate.kind === "serialized-path-sentinel") {
    const row = planRecord(value, name, ["kind", "fixturePath", "fixtureContentHash", "caseId", "serializedInputPath", "expectedViolationCode", "authorityDigest"]);
    if (row.expectedViolationCode !== "windows-reserved-name" && row.expectedViolationCode !== "trailing-dot-or-space")
      throw new Error(`${name}.expectedViolationCode is invalid`);
    const result = { kind: "serialized-path-sentinel", fixturePath: planPath(row.fixturePath, `${name}.fixturePath`), fixtureContentHash: planString(row.fixtureContentHash, `${name}.fixtureContentHash`, PLAN_HASH), caseId: planString(row.caseId, `${name}.caseId`), serializedInputPath: planString(row.serializedInputPath, `${name}.serializedInputPath`), expectedViolationCode: row.expectedViolationCode, authorityDigest: planString(row.authorityDigest, `${name}.authorityDigest`, PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible)))
      throw new Error(`${name}.authorityDigest does not match its authority`);
    return result;
  }
  throw new Error(`${name}.kind is invalid`);
}
function parseReferenceEdit(value, name) {
  const row = planRecord(value, name, ["path", "adapter", "structuredLocation", "oldValue", "newValue", "preimage"]);
  const adapters = ["rust", "typescript", "go", "python", "dotnet", "native", "json", "jsonc", "toml", "yaml", "xml", "markdown", "gherkin"];
  if (!adapters.includes(row.adapter))
    throw new Error(`${name}.adapter is invalid`);
  const preimage = parseLeafPreimage(row.preimage, `${name}.preimage`);
  if (preimage.nodeKind !== "file")
    throw new Error(`${name}.preimage must be a regular file`);
  return { path: planPath(row.path, `${name}.path`), adapter: row.adapter, structuredLocation: planString(row.structuredLocation, `${name}.structuredLocation`), oldValue: planString(row.oldValue, `${name}.oldValue`), newValue: planString(row.newValue, `${name}.newValue`), preimage };
}
function parseMove(value, name) {
  const row = planRecord(value, name, ["operationId", "sourcePath", "destinationPath", "sourcePreimage", "rationaleRule", "ownerId", "referenceEdits"], ["collisionGroup"]);
  if (!Array.isArray(row.referenceEdits))
    throw new Error(`${name}.referenceEdits must be an array`);
  const result = { operationId: planString(row.operationId, `${name}.operationId`, PLAN_OPERATION_ID), sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), destinationPath: planPath(row.destinationPath, `${name}.destinationPath`), sourcePreimage: parseLeafPreimage(row.sourcePreimage, `${name}.sourcePreimage`), rationaleRule: planString(row.rationaleRule, `${name}.rationaleRule`), ownerId: planString(row.ownerId, `${name}.ownerId`), collisionGroup: row.collisionGroup === undefined ? undefined : planString(row.collisionGroup, `${name}.collisionGroup`), referenceEdits: row.referenceEdits.map((entry, index) => parseReferenceEdit(entry, `${name}.referenceEdits[${index}]`)) };
  if (result.operationId !== dispositionOperationId("move-v2", { sourcePath: result.sourcePath, destinationPath: result.destinationPath, sourcePreimage: result.sourcePreimage }))
    throw new Error(`${name}.operationId does not match its fields`);
  return result;
}
function parseGeneratorNodeRecord(value, name) {
  const base = planRecord(value, name, ["path", "nodeKind", "contentHash", "mode"], ["size", "target"]);
  const path = planPath(base.path, `${name}.path`), contentHash = planString(base.contentHash, `${name}.contentHash`, PLAN_HASH), mode = planInteger(base.mode, `${name}.mode`, 4095);
  if (base.nodeKind === "directory") {
    if (base.size !== undefined || base.target !== undefined)
      throw new Error(`${name} directory cannot carry leaf evidence`);
    return { path, nodeKind: "directory", contentHash, mode };
  }
  const size = planInteger(base.size, `${name}.size`);
  if (base.nodeKind === "file") {
    if (base.target !== undefined)
      throw new Error(`${name} file cannot carry a symlink target`);
    return { path, nodeKind: "file", contentHash, mode, size };
  }
  if (base.nodeKind !== "symlink")
    throw new Error(`${name}.nodeKind is invalid`);
  const target = planString(base.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size)
    throw new Error(`${name} symlink target does not match its hash and size`);
  return { path, nodeKind: "symlink", contentHash, mode, size, target };
}
function parseRegeneration(value, name) {
  const row = planRecord(value, name, ["id", "contractId", "cwd", "command", "outputRoots", "inputs", "preOutputs", "outputs", "preview", "previewManifestDigest", "staleRemovals"], ["verifyCommand"]);
  const command = row.command;
  if (!Array.isArray(command) || command.length !== 4 || command[0] !== "bun" || command[1] !== "nx" || command[2] !== "run" || typeof command[3] !== "string")
    throw new Error(`${name}.command is invalid`);
  const verifyCommand = row.verifyCommand;
  if (verifyCommand !== undefined && (!Array.isArray(verifyCommand) || verifyCommand.length !== 4 || verifyCommand[0] !== "bun" || verifyCommand[1] !== "nx" || verifyCommand[2] !== "run" || typeof verifyCommand[3] !== "string"))
    throw new Error(`${name}.verifyCommand is invalid`);
  if (!["outputRoots", "inputs", "preOutputs", "outputs", "staleRemovals"].every((key) => Array.isArray(row[key])))
    throw new Error(`${name} array fields are invalid`);
  const preview = planRecord(row.preview, `${name}.preview`, ["contractId", "nodes", "schemaVersion", "staleRemovals"]);
  if (preview.schemaVersion !== 1 || preview.contractId !== row.contractId || !Array.isArray(preview.nodes) || !Array.isArray(preview.staleRemovals))
    throw new Error(`${name}.preview is invalid`);
  const previewNodes = preview.nodes.map((value2, index) => {
    const node = planRecord(value2, `${name}.preview.nodes[${index}]`, ["bytesBase64", "mode", "nodeKind", "path"]);
    if (node.nodeKind !== "directory" && node.nodeKind !== "file")
      throw new Error(`${name}.preview.nodes[${index}].nodeKind is invalid`);
    return { bytesBase64: planString(node.bytesBase64, `${name}.preview.nodes[${index}].bytesBase64`), mode: planInteger(node.mode, `${name}.preview.nodes[${index}].mode`, 4095), nodeKind: node.nodeKind, path: planPath(node.path, `${name}.preview.nodes[${index}].path`) };
  });
  const result = { id: planString(row.id, `${name}.id`, PLAN_OPERATION_ID), contractId: planString(row.contractId, `${name}.contractId`), cwd: planPath(row.cwd, `${name}.cwd`), command, verifyCommand, outputRoots: row.outputRoots.map((entry, index) => planPath(entry, `${name}.outputRoots[${index}]`)), inputs: row.inputs.map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.inputs[${index}]`)), preOutputs: row.preOutputs.map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.preOutputs[${index}]`)), outputs: row.outputs.map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.outputs[${index}]`)), preview: { contractId: preview.contractId, nodes: previewNodes, schemaVersion: 1, staleRemovals: preview.staleRemovals.map((entry, index) => planPath(entry, `${name}.preview.staleRemovals[${index}]`)) }, previewManifestDigest: planString(row.previewManifestDigest, `${name}.previewManifestDigest`, PLAN_HASH), staleRemovals: row.staleRemovals.map((entry, index) => planPath(entry, `${name}.staleRemovals[${index}]`)) };
  const provisional = { contractId: result.contractId, cwd: result.cwd, command: result.command, verifyCommand: result.verifyCommand, outputRoots: result.outputRoots, inputs: result.inputs, preOutputs: result.preOutputs, outputs: result.outputs, preview: result.preview, previewManifestDigest: result.previewManifestDigest, staleRemovals: result.staleRemovals };
  if (result.id !== sha256(canonicalJson(provisional)).slice(0, 24))
    throw new Error(`${name}.id does not match its fields`);
  return result;
}
function parseOpaqueDigest(value, name) {
  const row = planRecord(value, name, ["algorithm", "relativeRoot", "digest", "files", "directories", "symlinks", "others"]);
  if (row.algorithm !== "sha256-merkle-v1")
    throw new Error(`${name}.algorithm is invalid`);
  return { algorithm: row.algorithm, relativeRoot: planPath(row.relativeRoot, `${name}.relativeRoot`), digest: planString(row.digest, `${name}.digest`, PLAN_HASH), files: planInteger(row.files, `${name}.files`), directories: planInteger(row.directories, `${name}.directories`), symlinks: planInteger(row.symlinks, `${name}.symlinks`), others: planInteger(row.others, `${name}.others`) };
}
function parsePlanViolation(value, name) {
  const row = planRecord(value, name, ["code", "severity", "path", "message"]);
  if (row.severity !== "warning" && row.severity !== "error")
    throw new Error(`${name}.severity is invalid`);
  return { code: planString(row.code, `${name}.code`), severity: row.severity, path: planPath(row.path, `${name}.path`), message: planString(row.message, `${name}.message`) };
}
function parseTaxonomyPlan(value) {
  const row = planRecord(value, "taxonomy plan", ["schemaVersion", "taxonomySchemaVersion", "baselineCommit", "sourceTreeDigest", "excludedTreeDigests", "moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "destinationAncestorPreimages", "edits", "regenerations", "unresolved", "expectedAffectedPreStateDigest", "expectedPostStateDigest", "planDigest"], ["scope"]);
  if (row.schemaVersion !== 2 || row.taxonomySchemaVersion !== 7)
    throw new Error("Taxonomy plan must use schemaVersion 2 and taxonomySchemaVersion 7");
  if (!["excludedTreeDigests", "moves", "embeddedTicketRoots", "embeddedTicketRootRelocations", "symlinkTargetEdits", "evidenceRemovals", "destinationAncestorPreimages", "edits", "regenerations", "unresolved"].every((key) => Array.isArray(row[key])))
    throw new Error("Taxonomy plan operation and evidence fields must be arrays");
  const destinationAncestorPreimages = row.destinationAncestorPreimages.map((value2, index) => {
    const name = `taxonomy plan destinationAncestorPreimages[${index}]`;
    const entry = planRecord(value2, name, ["path", "state"]);
    if (entry.state !== "absent" && entry.state !== "directory")
      throw new Error(`${name}.state is invalid`);
    return { path: planPath(entry.path, `${name}.path`), state: entry.state };
  });
  if (destinationAncestorPreimages.some((entry, index) => index > 0 && generatorPathCompare(destinationAncestorPreimages[index - 1].path, entry.path) >= 0))
    throw new Error("Taxonomy plan destinationAncestorPreimages must be unique and bytewise sorted");
  const parseOperationId = (entry, name) => planString(entry.operationId, `${name}.operationId`, PLAN_OPERATION_ID);
  const embeddedTicketRootRelocations = row.embeddedTicketRootRelocations.map((value2, index) => {
    const name = `taxonomy plan embeddedTicketRootRelocations[${index}]`;
    const entry = planRecord(value2, name, ["operationId", "embeddedTicketRootId", "sourcePath", "destinationPath", "relativeEvidencePath", "preimage", "ownerId", "rationaleRule"], ["fixedContractId"]);
    if (entry.rationaleRule !== "embedded-ticket-root-relocation-v1")
      throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result2 = { operationId, embeddedTicketRootId: planString(entry.embeddedTicketRootId, `${name}.embeddedTicketRootId`, PLAN_OPERATION_ID), sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), destinationPath: planPath(entry.destinationPath, `${name}.destinationPath`), relativeEvidencePath: planPath(entry.relativeEvidencePath, `${name}.relativeEvidencePath`), preimage: parseLeafPreimage(entry.preimage, `${name}.preimage`), fixedContractId: entry.fixedContractId === undefined ? undefined : planString(entry.fixedContractId, `${name}.fixedContractId`), ownerId: planString(entry.ownerId, `${name}.ownerId`), rationaleRule: entry.rationaleRule };
    const { operationId: _id, ...digestible } = result2;
    if (operationId !== dispositionOperationId("embedded-ticket-root-relocation", digestible))
      throw new Error(`${name}.operationId does not match its fields`);
    return result2;
  });
  const evidenceRemovals = row.evidenceRemovals.map((value2, index) => {
    const name = `taxonomy plan evidenceRemovals[${index}]`;
    const entry = planRecord(value2, name, ["operationId", "sourcePath", "preimage", "authority", "rationaleRule", "ownerId"], ["embeddedTicketRootId"]);
    if (entry.rationaleRule !== "redundant-ticket-evidence-v1" && entry.rationaleRule !== "serialized-platform-sentinel-v1")
      throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result2 = { operationId, sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), preimage: parseLeafPreimage(entry.preimage, `${name}.preimage`), authority: parseRemovalAuthority(entry.authority, `${name}.authority`), embeddedTicketRootId: entry.embeddedTicketRootId === undefined ? undefined : planString(entry.embeddedTicketRootId, `${name}.embeddedTicketRootId`, PLAN_OPERATION_ID), rationaleRule: entry.rationaleRule, ownerId: planString(entry.ownerId, `${name}.ownerId`) };
    if (result2.authority.kind === "byte-and-mode-identical") {
      const removals = result2.authority.members.filter((member) => member.disposition === "remove" && member.sourcePath === result2.sourcePath && canonicalJson(member.preimage) === canonicalJson(result2.preimage));
      if (result2.rationaleRule !== "redundant-ticket-evidence-v1" || removals.length !== 1)
        throw new Error(`${name} is not bound to exactly one redundant evidence member`);
    } else if (result2.rationaleRule !== "serialized-platform-sentinel-v1" || result2.embeddedTicketRootId !== undefined)
      throw new Error(`${name} serialized sentinel authority has an invalid rationale or embedded-root binding`);
    const { operationId: _id, ...digestible } = result2;
    if (operationId !== dispositionOperationId("evidence-removal", digestible))
      throw new Error(`${name}.operationId does not match its fields`);
    return result2;
  });
  const symlinkTargetEdits = row.symlinkTargetEdits.map((value2, index) => {
    const name = `taxonomy plan symlinkTargetEdits[${index}]`;
    const entry = planRecord(value2, name, ["operationId", "sourcePath", "finalPath", "oldTarget", "newTarget", "oldTargetHash", "newTargetHash", "logicalTargetSourcePath", "logicalTargetFinalPath", "logicalTargetPreimage", "windowsLinkType", "sourceTargetDigest", "rationaleRule", "ownerId"]);
    if (entry.rationaleRule !== "repository-local-absolute-symlink-target-v1" || entry.windowsLinkType !== "file" && entry.windowsLinkType !== "dir")
      throw new Error(`${name} has invalid literals`);
    const operationId = parseOperationId(entry, name);
    const result2 = { operationId, sourcePath: planPath(entry.sourcePath, `${name}.sourcePath`), finalPath: planPath(entry.finalPath, `${name}.finalPath`), oldTarget: planString(entry.oldTarget, `${name}.oldTarget`), newTarget: planString(entry.newTarget, `${name}.newTarget`), oldTargetHash: planString(entry.oldTargetHash, `${name}.oldTargetHash`, PLAN_HASH), newTargetHash: planString(entry.newTargetHash, `${name}.newTargetHash`, PLAN_HASH), logicalTargetSourcePath: planPath(entry.logicalTargetSourcePath, `${name}.logicalTargetSourcePath`), logicalTargetFinalPath: planPath(entry.logicalTargetFinalPath, `${name}.logicalTargetFinalPath`), logicalTargetPreimage: parsePathPreimage(entry.logicalTargetPreimage, `${name}.logicalTargetPreimage`), windowsLinkType: entry.windowsLinkType, sourceTargetDigest: planString(entry.sourceTargetDigest, `${name}.sourceTargetDigest`, PLAN_HASH), rationaleRule: entry.rationaleRule, ownerId: planString(entry.ownerId, `${name}.ownerId`) };
    const absoluteTarget = result2.oldTarget.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(result2.oldTarget) || /^(?:\\\\|\/\/)[^\\/]+[\\/][^\\/]+/u.test(result2.oldTarget);
    if (!absoluteTarget || result2.oldTarget.includes("\x00") || result2.newTarget === "" || result2.newTarget.includes("\\") || result2.newTarget.startsWith("/") || /^[A-Za-z]:/u.test(result2.newTarget))
      throw new Error(`${name} target syntax is invalid`);
    const resolvedNewTarget = posix.normalize(posix.join(posix.dirname(result2.finalPath), result2.newTarget));
    if (resolvedNewTarget !== result2.logicalTargetFinalPath)
      throw new Error(`${name}.newTarget does not resolve to logicalTargetFinalPath`);
    const targetDigestible = { sourcePath: result2.sourcePath, finalPath: result2.finalPath, oldTarget: result2.oldTarget, newTarget: result2.newTarget, logicalTargetSourcePath: result2.logicalTargetSourcePath, logicalTargetFinalPath: result2.logicalTargetFinalPath, logicalTargetPreimage: result2.logicalTargetPreimage };
    if (result2.oldTargetHash !== sha256(result2.oldTarget) || result2.newTargetHash !== sha256(result2.newTarget) || result2.sourceTargetDigest !== sha256(canonicalJson(targetDigestible)))
      throw new Error(`${name} target hashes do not match raw targets`);
    const { operationId: _id, ...digestible } = result2;
    if (operationId !== dispositionOperationId("symlink-target-edit", digestible))
      throw new Error(`${name}.operationId does not match its fields`);
    return result2;
  });
  const embeddedTicketRoots = row.embeddedTicketRoots.map((value2, index) => {
    const name = `taxonomy plan embeddedTicketRoots[${index}]`;
    const entry = planRecord(value2, name, ["operationId", "sourceMetadataRoot", "sourceTicketRoot", "canonicalTicketRoot", "ticketId", "sourceTreeDigest", "residualTreeDigest", "incomingReferenceDigest", "relocationOperationIds", "removalOperationIds", "rationaleRule"]);
    if (entry.rationaleRule !== "embedded-ticket-root-relocation-v1")
      throw new Error(`${name}.rationaleRule is invalid`);
    const operationId = parseOperationId(entry, name);
    const result2 = { operationId, sourceMetadataRoot: planPath(entry.sourceMetadataRoot, `${name}.sourceMetadataRoot`), sourceTicketRoot: planPath(entry.sourceTicketRoot, `${name}.sourceTicketRoot`), canonicalTicketRoot: planPath(entry.canonicalTicketRoot, `${name}.canonicalTicketRoot`), ticketId: planString(entry.ticketId, `${name}.ticketId`), sourceTreeDigest: parseNoFollowTreeDigest(entry.sourceTreeDigest, `${name}.sourceTreeDigest`), residualTreeDigest: parseNoFollowTreeDigest(entry.residualTreeDigest, `${name}.residualTreeDigest`), incomingReferenceDigest: planString(entry.incomingReferenceDigest, `${name}.incomingReferenceDigest`, PLAN_HASH), relocationOperationIds: planStringArray(entry.relocationOperationIds, `${name}.relocationOperationIds`, PLAN_OPERATION_ID), removalOperationIds: planStringArray(entry.removalOperationIds, `${name}.removalOperationIds`, PLAN_OPERATION_ID), rationaleRule: entry.rationaleRule };
    const { operationId: _id, relocationOperationIds: _relocations, removalOperationIds: _removals, ...digestible } = result2;
    if (operationId !== dispositionOperationId("embedded-ticket-root", digestible))
      throw new Error(`${name}.operationId does not match its fields`);
    return result2;
  });
  const allOperationIds = [...row.moves.map((entry, index) => planString(entry.operationId, `taxonomy plan moves[${index}].operationId`, PLAN_OPERATION_ID)), ...embeddedTicketRoots.map((entry) => entry.operationId), ...embeddedTicketRootRelocations.map((entry) => entry.operationId), ...symlinkTargetEdits.map((entry) => entry.operationId), ...evidenceRemovals.map((entry) => entry.operationId), ...row.regenerations.map((entry, index) => planString(entry.id, `taxonomy plan regenerations[${index}].id`, PLAN_OPERATION_ID))];
  if (new Set(allOperationIds).size !== allOperationIds.length)
    throw new Error("Taxonomy plan operation IDs are not globally unique");
  const relocationIds = new Set(embeddedTicketRootRelocations.map((entry) => entry.operationId));
  const removalIds = new Set(evidenceRemovals.map((entry) => entry.operationId));
  const rootIds = new Set(embeddedTicketRoots.map((entry) => entry.operationId));
  if (embeddedTicketRootRelocations.some((entry) => !rootIds.has(entry.embeddedTicketRootId)) || evidenceRemovals.some((entry) => entry.embeddedTicketRootId !== undefined && !rootIds.has(entry.embeddedTicketRootId)))
    throw new Error("Embedded ticket disposition references an unknown root");
  for (const root of embeddedTicketRoots) {
    if (!root.sourceTicketRoot.startsWith(`${root.sourceMetadataRoot}/`))
      throw new Error(`Embedded ticket root ${root.operationId} source ticket root escapes its metadata root`);
    const ticketSegments = root.canonicalTicketRoot.split("/").slice(-4);
    const expectedTicketId = `${splitLeadingEmoji(ticketSegments[0] ?? "").rest}/${splitLeadingEmoji(ticketSegments[1] ?? "").rest}/${splitLeadingEmoji(ticketSegments[2] ?? "").rest}/${ticketSegments[3] ?? ""}`;
    if (root.ticketId !== expectedTicketId || !/^[0-9]{2}\/[0-9]{2}\/[0-9]{2}\/.+/u.test(root.ticketId))
      throw new Error(`Embedded ticket root ${root.operationId} ticketId does not match its canonical root`);
    const orderedChildren = [...root.relocationOperationIds, ...root.removalOperationIds];
    if ([root.relocationOperationIds, root.removalOperationIds].some((ids) => ids.some((id, index) => index > 0 && Buffer.from(ids[index - 1]).compare(Buffer.from(id)) >= 0)))
      throw new Error(`Embedded ticket root ${root.operationId} child IDs are not unique and bytewise sorted`);
    if (root.relocationOperationIds.some((id) => !relocationIds.has(id)) || root.removalOperationIds.some((id) => !removalIds.has(id)))
      throw new Error(`Embedded ticket root ${root.operationId} has dangling disposition IDs`);
    const actualChildren = [...embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId), ...evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId)];
    if (new Set(orderedChildren).size !== orderedChildren.length || canonicalJson(orderedChildren.sort(generatorPathCompare)) !== canonicalJson(actualChildren.sort(generatorPathCompare)))
      throw new Error(`Embedded ticket root ${root.operationId} does not exhaust its child dispositions`);
    if (actualChildren.length !== root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks || root.sourceTreeDigest.others !== 0 || root.residualTreeDigest.files !== 0 || root.residualTreeDigest.symlinks !== 0 || root.residualTreeDigest.others !== 0)
      throw new Error(`Embedded ticket root ${root.operationId} tree closure does not equal its child dispositions`);
    for (const relocation of embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId))
      if (!relocation.sourcePath.startsWith(`${root.sourceTicketRoot}/`) || relocation.destinationPath !== `${root.canonicalTicketRoot}/${relocation.relativeEvidencePath}` || relocation.relativeEvidencePath !== relocation.sourcePath.slice(root.sourceTicketRoot.length + 1))
        throw new Error(`Embedded ticket root ${root.operationId} relocation escapes its frozen roots`);
    for (const removal of evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId))
      if (!removal.sourcePath.startsWith(`${root.sourceTicketRoot}/`))
        throw new Error(`Embedded ticket root ${root.operationId} removal escapes its source ticket root`);
  }
  if (new Set(embeddedTicketRoots.map((entry) => entry.sourceMetadataRoot)).size !== embeddedTicketRoots.length || new Set(embeddedTicketRoots.map((entry) => entry.sourceTicketRoot)).size !== embeddedTicketRoots.length)
    throw new Error("Embedded ticket root source roots are not unique");
  const excludedTreeDigests = row.excludedTreeDigests.map((entry, index) => parseOpaqueDigest(entry, `taxonomy plan excludedTreeDigests[${index}]`));
  const moves = row.moves.map((entry, index) => parseMove(entry, `taxonomy plan moves[${index}]`));
  for (const edit of symlinkTargetEdits) {
    const owningMoves = moves.filter((move) => move.sourcePath === edit.sourcePath && move.destinationPath === edit.finalPath);
    if (edit.sourcePath !== edit.finalPath && owningMoves.length !== 1)
      throw new Error(`Symlink target edit ${edit.operationId} finalPath is not its exact move destination`);
  }
  const edits = row.edits.map((entry, index) => parseReferenceEdit(entry, `taxonomy plan edits[${index}]`));
  const regenerations = row.regenerations.map((entry, index) => parseRegeneration(entry, `taxonomy plan regenerations[${index}]`));
  const unresolved = row.unresolved.map((entry, index) => parsePlanViolation(entry, `taxonomy plan unresolved[${index}]`));
  const result = { schemaVersion: 2, taxonomySchemaVersion: 7, baselineCommit: planString(row.baselineCommit, "taxonomy plan baselineCommit", PLAN_COMMIT_ID), scope: row.scope === undefined ? undefined : planPath(row.scope, "taxonomy plan scope"), sourceTreeDigest: planString(row.sourceTreeDigest, "taxonomy plan sourceTreeDigest", PLAN_HASH), excludedTreeDigests, moves, embeddedTicketRoots, embeddedTicketRootRelocations, symlinkTargetEdits, evidenceRemovals, destinationAncestorPreimages, edits, regenerations, unresolved, expectedAffectedPreStateDigest: planString(row.expectedAffectedPreStateDigest, "taxonomy plan expectedAffectedPreStateDigest", PLAN_HASH), expectedPostStateDigest: planString(row.expectedPostStateDigest, "taxonomy plan expectedPostStateDigest", PLAN_HASH), planDigest: planString(row.planDigest, "taxonomy plan planDigest", PLAN_HASH) };
  const requiredAncestorPaths = new Set;
  for (const destination of [...moves.map((entry) => entry.destinationPath), ...embeddedTicketRootRelocations.map((entry) => entry.destinationPath), ...symlinkTargetEdits.map((entry) => entry.finalPath), ...regenerations.flatMap((entry) => entry.outputRoots)])
    for (let path = posix.dirname(destination);path !== "." && path !== ""; path = posix.dirname(path))
      requiredAncestorPaths.add(path);
  if (canonicalJson(destinationAncestorPreimages.map((entry) => entry.path)) !== canonicalJson([...requiredAncestorPaths].sort(generatorPathCompare)))
    throw new Error("Taxonomy plan destinationAncestorPreimages do not exhaust mutation destination parents");
  planString(result.sourceTreeDigest, "taxonomy plan sourceTreeDigest", PLAN_HASH);
  planString(result.expectedAffectedPreStateDigest, "taxonomy plan expectedAffectedPreStateDigest", PLAN_HASH);
  planString(result.expectedPostStateDigest, "taxonomy plan expectedPostStateDigest", PLAN_HASH);
  planString(result.planDigest, "taxonomy plan planDigest", PLAN_HASH);
  if (result.scope !== undefined)
    planPath(result.scope, "taxonomy plan scope");
  if (taxonomyPlanDigest(result) !== result.planDigest)
    throw new Error("Taxonomy plan digest does not match canonical plan bytes");
  return result;
}
function generatorPathCompare(left, right) {
  return Buffer.from(left).compare(Buffer.from(right));
}
function generatorPreviewJson(manifest) {
  return JSON.stringify({
    contractId: manifest.contractId,
    nodes: manifest.nodes.map((node) => ({ bytesBase64: node.bytesBase64, mode: node.mode, nodeKind: node.nodeKind, path: node.path })),
    schemaVersion: manifest.schemaVersion,
    staleRemovals: manifest.staleRemovals
  });
}
function parseGeneratorPreviewManifest(content, expectedContractId, outputRoots, excludedRoots = []) {
  let value;
  try {
    value = JSON.parse(content);
  } catch {
    throw new Error(`Generator preview stdout is not one canonical JSON document: bytes=${Buffer.byteLength(content)}, sha256=${sha256(content)}`);
  }
  const root = record(value, "generator preview");
  if (Object.keys(root).join("\x00") !== "contractId\x00nodes\x00schemaVersion\x00staleRemovals")
    throw new Error("Generator preview has noncanonical top-level keys or order");
  if (root.schemaVersion !== 1)
    throw new Error("Generator preview schemaVersion must be 1");
  if (root.contractId !== expectedContractId)
    throw new Error(`Generator preview contractId does not match ${expectedContractId}`);
  if (!Array.isArray(root.nodes) || !Array.isArray(root.staleRemovals))
    throw new Error("Generator preview nodes and staleRemovals must be arrays");
  const roots = [...new Set(outputRoots.map((path) => normalizeRelative(path)))].sort(generatorPathCompare);
  if (roots.length !== outputRoots.length || roots.some((path, index) => path !== outputRoots[index]))
    throw new Error("Generator preview output roots must be unique, NFC, repository-relative, and byte-sorted");
  const exclusions = excludedRoots.map(normalizeRelative);
  const withinRoot = (path) => roots.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const excluded = (path) => exclusions.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const nodes = root.nodes.map((value2, index) => {
    const node = record(value2, `generator preview nodes[${index}]`);
    if (Object.keys(node).join("\x00") !== "bytesBase64\x00mode\x00nodeKind\x00path")
      throw new Error(`Generator preview node ${index} has noncanonical keys or order`);
    const path = requiredString(node.path, `generator preview nodes[${index}].path`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path))
      throw new Error(`Generator preview node path is unsafe or outside registered roots: ${path}`);
    if (node.nodeKind !== "directory" && node.nodeKind !== "file")
      throw new Error(`Generator preview nodeKind is invalid at ${path}`);
    if (!Number.isSafeInteger(node.mode) || node.mode < 0 || node.mode > 4095)
      throw new Error(`Generator preview mode is invalid at ${path}`);
    if (typeof node.bytesBase64 !== "string" || !/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/u.test(node.bytesBase64))
      throw new Error(`Generator preview base64 is invalid at ${path}`);
    const decoded = Buffer.from(node.bytesBase64, "base64");
    if (decoded.toString("base64") !== node.bytesBase64 || node.nodeKind === "directory" && node.bytesBase64 !== "")
      throw new Error(`Generator preview base64 is noncanonical at ${path}`);
    return { bytesBase64: node.bytesBase64, mode: node.mode, nodeKind: node.nodeKind, path };
  });
  const nodeByPath = new Map;
  for (let index = 0;index < nodes.length; index++) {
    const node = nodes[index];
    if (nodeByPath.has(node.path) || index > 0 && generatorPathCompare(nodes[index - 1].path, node.path) >= 0)
      throw new Error(`Generator preview nodes repeat or are not byte-sorted at ${node.path}`);
    nodeByPath.set(node.path, node);
  }
  for (const registeredRoot of roots)
    if (!nodeByPath.has(registeredRoot))
      throw new Error(`Generator preview omits registered output root ${registeredRoot}`);
  for (const node of nodes) {
    let parent = posix.dirname(node.path);
    const registeredRoot = roots.filter((candidate) => node.path === candidate || node.path.startsWith(`${candidate}/`)).sort((left, right) => right.length - left.length)[0];
    while (registeredRoot && parent !== posix.dirname(registeredRoot)) {
      const parentNode = nodeByPath.get(parent);
      if (!parentNode || parentNode.nodeKind !== "directory")
        throw new Error(`Generator preview omits directory node ${parent}`);
      if (parent === registeredRoot)
        break;
      parent = posix.dirname(parent);
    }
    if (node.nodeKind === "file" && nodes.some((candidate) => candidate.path.startsWith(`${node.path}/`)))
      throw new Error(`Generator preview file has descendants at ${node.path}`);
  }
  const staleRemovals = root.staleRemovals.map((value2, index) => {
    const path = requiredString(value2, `generator preview staleRemovals[${index}]`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path))
      throw new Error(`Generator preview stale removal is unsafe or outside registered roots: ${path}`);
    if (nodeByPath.has(path) || nodes.some((node) => node.path.startsWith(`${path}/`)))
      throw new Error(`Generator preview stale removal overlaps expected output ${path}`);
    return path;
  });
  for (let index = 0;index < staleRemovals.length; index++)
    if (index > 0 && generatorPathCompare(staleRemovals[index - 1], staleRemovals[index]) >= 0 || staleRemovals.some((path, candidate) => candidate !== index && path.startsWith(`${staleRemovals[index]}/`)))
      throw new Error(`Generator preview stale removals repeat, overlap, or are not byte-sorted at ${staleRemovals[index]}`);
  const manifest = { contractId: expectedContractId, nodes, schemaVersion: 1, staleRemovals };
  if (content !== `${generatorPreviewJson(manifest)}
`)
    throw new Error("Generator preview stdout is noisy or not byte-canonical JSON");
  return manifest;
}
function normalizeRelative(value) {
  return sourceRelative(value).normalize("NFC");
}
function sourceRelative(value) {
  const slash = value.replaceAll("\\", "/").replace(/^\.\//, "");
  const normalized = posix.normalize(slash);
  if (normalized === ".")
    return "";
  if (normalized === ".." || normalized.startsWith("../") || normalized.startsWith("/") || normalized.includes("\x00"))
    throw new Error(`Path escapes repository scope: ${value}`);
  return normalized.replace(/\/$/, "");
}
function absolutePath(repoRoot, path) {
  const root = resolve2(repoRoot);
  const result = resolve2(root, ...sourceRelative(path).split("/").filter(Boolean));
  const rel = relative2(root, result);
  if (rel === ".." || rel.startsWith(`..${sep}`) || rel.startsWith("../") || rel.startsWith("..\\") || isAbsolute(rel))
    throw new Error(`Path escapes repository root: ${path}`);
  return result;
}
function assertNoFollowAncestors(repoRoot, target, label, rejectLeafSymlink = false) {
  const root = resolve2(repoRoot);
  const relativeTarget = relative2(root, target);
  const segments = relativeTarget.split(sep).filter(Boolean);
  let current = root;
  const end = segments.length - (rejectLeafSymlink ? 0 : 1);
  for (let index = 0;index < end; index++) {
    current = join2(current, segments[index]);
    const stat = lstatOrNull(current);
    const leaf = rejectLeafSymlink && index === segments.length - 1;
    if (stat?.isSymbolicLink() || !leaf && stat && !stat.isDirectory())
      throw new Error(`${label} has a non-directory or symlink ancestor: ${segments.slice(0, index + 1).join("/")}`);
  }
}
function assertLexicalInputOutsideOpaque(repoRoot, path, label, rejectLeafSymlink = false) {
  const root = resolve2(repoRoot);
  const target = isAbsolute(path) ? resolve2(path) : resolve2(root, path);
  const nativeRelative = relative2(root, target);
  if (nativeRelative === ".." || nativeRelative.startsWith(`..${sep}`) || nativeRelative.startsWith("../") || nativeRelative.startsWith("..\\") || isAbsolute(nativeRelative))
    throw new Error(`${label} must be repository-local`);
  const repositoryRelative = posix.normalize(nativeRelative.replaceAll("\\", "/"));
  if (LEXICAL_OPAQUE_ROOTS.some((opaque) => repositoryRelative === opaque || repositoryRelative.startsWith(`${opaque}/`)))
    throw new Error(`${label} is inside an opaque path: ${repositoryRelative}`);
  assertNoFollowAncestors(root, target, label, rejectLeafSymlink);
  return target;
}
function isExcluded(path, taxonomy) {
  const normalized = normalizeRelative(path);
  return taxonomy.exclusions.some((entry) => normalized === entry.path || normalized.startsWith(`${entry.path}/`));
}
function inScope(path, scope) {
  if (!scope)
    return true;
  const normalizedScope = normalizeRelative(scope);
  const normalizedPath = normalizeRelative(path);
  return normalizedPath === normalizedScope || normalizedPath.startsWith(`${normalizedScope}/`) || normalizedScope.startsWith(`${normalizedPath}/`);
}
function emojiFold(value) {
  return value.normalize("NFC").replaceAll("\uFE0F", "");
}
function graphemes(value) {
  return [...SEGMENTER.segment(value)].map((entry) => entry.segment);
}
function isEmojiGrapheme(value) {
  return /[\p{Extended_Pictographic}\p{Emoji_Presentation}\uFE0F\u20E3]/u.test(value);
}
function splitLeadingEmoji(value) {
  const segments = graphemes(value);
  if (segments.length === 0 || !isEmojiGrapheme(segments[0]))
    return { emoji: "", rest: value };
  return { emoji: segments[0], rest: segments.slice(1).join("") };
}
function matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds = []) {
  const normalized = name.normalize("NFC");
  const leading = splitLeadingEmoji(normalized);
  const contextAllows = (kind) => (kind.parentKindIds?.length ?? 0) === 0 || parentKindId !== undefined && kind.parentKindIds?.includes(parentKindId) === true;
  if (leading.emoji) {
    const global = taxonomy.directoryKinds.filter((kind) => emojiFold(kind.emoji) === emojiFold(leading.emoji) && (leading.rest.length === 0 && kind.allowEmojiOnly || kind.slugRegex.test(leading.rest)));
    const exact2 = global.filter((kind) => contextAllows(kind) && kind.id.normalize("NFC").toLocaleLowerCase("und") === leading.rest.toLocaleLowerCase("und"));
    if (exact2.length === 1)
      return { kind: exact2[0], slug: leading.rest, ambiguous: [] };
    if (exact2.length > 1)
      return { kind: null, slug: leading.rest, ambiguous: exact2.map((entry) => entry.id) };
    const contextual2 = parentKindId === undefined ? [] : global.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
    const ordinary = contextual2.length > 0 ? contextual2 : global.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
    if (ordinary.length === 1)
      return { kind: ordinary[0], slug: leading.rest, ambiguous: [] };
    const contexts = [parentKindId, ...ancestorKindIds].filter((kindId, index, rows) => Boolean(kindId) && rows.indexOf(kindId) === index);
    const overlays = Object.entries(taxonomy.schema.semanticDirectoryMemberKinds).filter(([, spec]) => spec.memberNames.some((memberName) => emojiFold(memberName) === emojiFold(normalized))).map(([id, spec]) => ({ id, distance: contexts.findIndex((kindId) => spec.ownerKindIds.includes(kindId)) })).filter((entry) => entry.distance >= 0).sort((left, right) => left.distance - right.distance || left.id.localeCompare(right.id));
    if (overlays.length > 0) {
      const nearest = overlays.filter((entry) => entry.distance === overlays[0].distance);
      if (nearest.length === 1)
        return { kind: { id: nearest[0].id, emoji: leading.emoji }, slug: leading.rest, ambiguous: [] };
      return { kind: null, slug: leading.rest, ambiguous: nearest.map((entry) => entry.id) };
    }
    return { kind: null, slug: leading.rest, ambiguous: ordinary.length > 0 ? ordinary.map((entry) => entry.id) : global.map((entry) => entry.id) };
  }
  const exact = taxonomy.directoryKinds.filter((kind) => contextAllows(kind) && kind.inferWithoutEmoji !== false && kind.id.normalize("NFC").toLocaleLowerCase("und") === normalized.toLocaleLowerCase("und"));
  if (exact.length === 1)
    return { kind: exact[0], slug: normalized, ambiguous: [] };
  if (exact.length > 1)
    return { kind: null, slug: normalized, ambiguous: exact.map((entry) => entry.id) };
  const matching = taxonomy.directoryKinds.filter((kind) => kind.inferWithoutEmoji !== false && kind.slugRegex.test(normalized));
  const contextual = parentKindId === undefined ? [] : matching.filter((kind) => kind.parentKindIds?.includes(parentKindId) === true);
  const matches = contextual.length > 0 ? contextual : matching.filter((kind) => (kind.parentKindIds?.length ?? 0) === 0);
  return { kind: matches.length === 1 ? matches[0] : null, slug: normalized, ambiguous: matches.map((entry) => entry.id) };
}
function resolveFileKind(path, taxonomy, parentKindId, ancestorKindIds, forcedId, contentKindId) {
  const name = basename2(path);
  const normalized = name.normalize("NFC");
  const folded = normalized.toLocaleLowerCase("und");
  const scoped = Object.entries(taxonomy.schema.scopedFileKinds).flatMap(([id, spec]) => {
    if (!taxonomyPathPatternMatches(path, spec.pathPattern) || !new RegExp(spec.sourceFilenamePattern, "u").test(normalized))
      return [];
    const extensions = spec.extensionChains.filter((chain) => folded.endsWith(chain.toLocaleLowerCase("und"))).sort((left, right) => right.length - left.length || left.localeCompare(right));
    return extensions.length > 0 ? [{ id, spec, extension: extensions[0] }] : [];
  }).sort((left, right) => left.id.localeCompare(right.id));
  if (scoped.length > 1)
    return { kind: null, extension: "", stem: normalized, ambiguous: scoped.map(({ id }) => `scoped:${id}`) };
  if (scoped.length === 1) {
    const selected2 = scoped[0];
    const kind = { id: `scoped:${selected2.id}`, emoji: selected2.spec.emoji, extensionChains: selected2.spec.extensionChains, role: selected2.spec.role };
    const withoutExtension2 = normalized.slice(0, -selected2.extension.length);
    const leading2 = splitLeadingEmoji(withoutExtension2);
    return { kind, extension: selected2.extension, stem: leading2.emoji && emojiFold(leading2.emoji) === emojiFold(kind.emoji) ? leading2.rest : withoutExtension2, ambiguous: [] };
  }
  const forced = forcedId ? taxonomy.fileKinds.find((kind) => kind.id === forcedId) : undefined;
  if (forced) {
    const extensions = forced.extensionChains.filter((chain) => normalized.endsWith(chain)).sort((left, right) => right.length - left.length || left.localeCompare(right));
    if (extensions.length > 0) {
      const extension2 = extensions[0];
      const withoutExtension2 = normalized.slice(0, -extension2.length);
      const leading2 = splitLeadingEmoji(withoutExtension2);
      return { kind: forced, extension: extension2, stem: leading2.emoji && emojiFold(leading2.emoji) === emojiFold(forced.emoji) ? leading2.rest : withoutExtension2, ambiguous: [] };
    }
  }
  const extensionRows = Object.entries(taxonomy.schema.fileKindResolutionRules).filter(([, rule]) => normalized.endsWith(rule.extensionChain)).sort((left, right) => right[1].extensionChain.length - left[1].extensionChain.length || left[0].localeCompare(right[0]));
  const longest = extensionRows[0]?.[1].extensionChain.length ?? 0;
  const candidates = extensionRows.filter(([, rule]) => rule.extensionChain.length === longest).filter(([, rule]) => !rule.filenamePattern || new RegExp(rule.filenamePattern, "u").test(normalized)).filter(([, rule]) => !rule.pathPattern || taxonomyPathPatternMatches(path, rule.pathPattern)).filter(([, rule]) => !rule.parentKindIds || parentKindId !== undefined && rule.parentKindIds.includes(parentKindId)).filter(([, rule]) => !rule.ancestorKindIds || rule.ancestorKindIds.some((kindId) => ancestorKindIds.includes(kindId))).map(([id, rule]) => ({ id, rule, predicates: Number(Boolean(rule.filenamePattern)) + Number(Boolean(rule.pathPattern)) + Number(Boolean(rule.parentKindIds)) + Number(Boolean(rule.ancestorKindIds)) })).sort((left, right) => right.rule.priority - left.rule.priority || right.predicates - left.predicates || left.id.localeCompare(right.id));
  if (candidates.length === 0) {
    const contentKind = contentKindId ? taxonomy.fileKinds.find((kind) => kind.id === contentKindId) : undefined;
    if (!contentKind)
      return { kind: null, extension: "", stem: normalized, ambiguous: [] };
    const extension2 = [...contentKind.extensionChains].sort((left, right) => left.length - right.length || left.localeCompare(right))[0];
    const leading2 = splitLeadingEmoji(normalized);
    const stem2 = (leading2.emoji && emojiFold(leading2.emoji) === emojiFold(contentKind.emoji) ? leading2.rest : normalized).trim().replace(/[. ]+$/u, "");
    return { kind: contentKind, extension: extension2, stem: stem2, ambiguous: [] };
  }
  const top = candidates.filter((entry) => entry.rule.priority === candidates[0].rule.priority && entry.predicates === candidates[0].predicates);
  const kindIds = [...new Set(top.map((entry) => entry.rule.fileKindId))];
  const extension = top[0].rule.extensionChain;
  const withoutExtension = normalized.slice(0, normalized.length - extension.length);
  if (kindIds.length !== 1)
    return { kind: null, extension, stem: withoutExtension, ambiguous: top.map((entry) => `${entry.id}:${entry.rule.fileKindId}`) };
  const selected = taxonomy.fileKinds.find((kind) => kind.id === kindIds[0]);
  if (!selected)
    return { kind: null, extension, stem: withoutExtension, ambiguous: kindIds };
  const leading = splitLeadingEmoji(withoutExtension);
  const stem = leading.emoji && emojiFold(leading.emoji) === emojiFold(selected.emoji) ? leading.rest : withoutExtension;
  return { kind: selected, extension, stem, ambiguous: [] };
}
function shebangCommand(line) {
  const raw = line.startsWith("#!") ? line.slice(2).trim() : "";
  if (!raw)
    return null;
  const tokens = raw.split(/\s+/u).filter(Boolean);
  let command = tokens.shift() ?? "";
  if (basename2(command).toLocaleLowerCase("und") === "env") {
    while (tokens[0]?.startsWith("-") || /^[A-Za-z_][A-Za-z0-9_]*=/u.test(tokens[0] ?? ""))
      tokens.shift();
    command = tokens.shift() ?? "";
  }
  return command ? basename2(command).replace(/\.exe$/iu, "").toLocaleLowerCase("und") : null;
}
function typescriptSyntax(text) {
  return /\b(?:interface|namespace|enum)\s+[A-Za-z_$]|\btype\s+[A-Za-z_$][\w$]*\s*=|\b(?:const|let|var)\s+[A-Za-z_$][\w$]*\s*:\s*[^=]|\b(?:satisfies|as\s+const)\b/u.test(text);
}
function extensionlessContentKind(path, bytes, taxonomy) {
  const name = basename2(path);
  if (name.includes(".") || !bytes)
    return { kindId: null };
  if (bytes.includes(0))
    return { kindId: "binary" };
  let text;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    return { kindId: "binary" };
  }
  if (/[\u0001-\u0008\u000B\u000C\u000E-\u001F]/u.test(text))
    return { kindId: null, violation: violation("content-kind-ambiguous", path, "Extensionless content contains non-text control bytes without a binary signature") };
  if (text.startsWith("#!")) {
    const command = shebangCommand(text.split(/\r?\n/u, 1)[0] ?? "");
    const kindId = command && /^(?:ba|da|z|k|fi)?sh$/u.test(command) ? "shell" : command && /^python(?:\d+(?:\.\d+)*)?$/u.test(command) ? "python-source" : command && /^(?:pwsh|powershell)$/u.test(command) ? "powershell" : command && /^(?:node|nodejs)$/u.test(command) ? typescriptSyntax(text) ? "typescript-source" : "javascript-source" : command && /^(?:bun|deno|tsx|ts-node)$/u.test(command) ? typescriptSyntax(text) ? "typescript-source" : "javascript-source" : null;
    if (!kindId)
      return { kindId: null, violation: violation("shebang-kind-unresolved", path, `Extensionless shebang interpreter is unknown or contradictory: ${command ?? "missing"}`) };
    if (!taxonomy.schema.fileKinds[kindId])
      return { kindId: null, violation: violation("shebang-kind-unregistered", path, `Shebang resolved to unregistered file kind ${kindId}`) };
    return { kindId };
  }
  if (!taxonomy.schema.fileKinds["plain-text"])
    return { kindId: null, violation: violation("text-kind-unregistered", path, "Extensionless UTF-8 content requires registered plain-text kind") };
  return { kindId: "plain-text" };
}
function ownerId(path) {
  const parts = path.split("/");
  if (parts[0] === ".\uD83E\uDDECsemio" && parts[1] === "\uD83E\uDD91\uFE0Frepo" && parts[2] === "\uD83C\uDFAB\uFE0Ftickets" && parts.length >= 7)
    return parts.slice(0, 7).join("/");
  if (parts[0] === "\u270F\uFE0Fs" && (parts[1] === "\uD83D\uDD0C\uFE0Fplugins" || parts[1] === "\uD83D\uDD28\uFE0Fmodules") && parts[2])
    return parts.slice(0, 3).join("/");
  if (parts[0] === "\uD83E\uDDF0\uFE0Fframework" && (parts[1] === "\uD83D\uDECD\uFE0Fproducts" || parts[1] === "\uD83D\uDD28\uFE0Fmodules") && parts[2])
    return parts.slice(0, 3).join("/");
  if ((parts[0] === "\uD83C\uDF0E\uFE0Fhub" || parts[0] === "\u267B\uFE0Fmit-bestand") && parts[1])
    return parts.slice(0, 2).join("/");
  return parts[0] ?? "";
}
function areaId(path) {
  const first = path.split("/")[0] ?? "";
  if (first === "\u270F\uFE0Fs")
    return path.split("/").slice(0, 2).join("/");
  return first;
}
function violation(code, path, message, severity = "error") {
  return { code, severity, path, message };
}
function stableViolations(rows) {
  return [...new Map(rows.map((entry) => [`${entry.path}\x00${entry.code}\x00${entry.severity}\x00${entry.message}`, entry])).values()].sort((a, b) => a.path.localeCompare(b.path) || a.code.localeCompare(b.code) || a.message.localeCompare(b.message));
}
function report(progress, operation, phase, current, total, path) {
  progress?.({ operation, phase, current, total, path });
}

class TaxonomyCancellationError extends Error {
  constructor() {
    super("Taxonomy operation cancelled");
  }
}
function checkCancellation(repoRoot, cancelFile) {
  if (!cancelFile)
    return;
  const path = assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true);
  if (existsSync2(path))
    throw new TaxonomyCancellationError;
}
function cancellationRequested(repoRoot, cancelFile) {
  if (!cancelFile)
    return false;
  return existsSync2(assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true));
}
function gitRows(repoRoot, taxonomy) {
  const exclusions = taxonomy.exclusions.map((entry) => `:(exclude,top,literal)${entry.path}`);
  const stdout = execFileSync("git", ["ls-files", "--stage", "-z", "--", ".", ...exclusions], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return stdout.toString("utf8").split("\x00").filter(Boolean).map((row) => {
    const tab = row.indexOf("\t");
    const [mode, objectId, stage] = row.slice(0, tab).split(" ");
    return { path: sourceRelative(row.slice(tab + 1)), mode, objectId, stage };
  }).filter((row) => row.stage === "0").map(({ path, mode, objectId }) => ({ path, mode, objectId }));
}
function untrackedGitPaths(repoRoot, taxonomy) {
  const exclusions = taxonomy.exclusions.map((entry) => `:(exclude,top,literal)${entry.path}`);
  const stdout = execFileSync("git", ["ls-files", "--others", "--exclude-standard", "-z", "--", ".", ...exclusions], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return stdout.toString("utf8").split("\x00").filter(Boolean).map(sourceRelative).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
}
function worktreeCandidate(repoRoot, path) {
  const stat = lstatOrNull(absolutePath(repoRoot, path));
  if (!stat)
    return null;
  if (stat.isSymbolicLink())
    return { path, mode: "120000" };
  if (stat.isDirectory())
    return { path, mode: "040000", explicitDirectory: true };
  return { path, mode: (stat.mode & 73) !== 0 ? "100755" : "100644" };
}
function explicitTicketRows(repoRoot, ticketDir, taxonomy) {
  if (!ticketDir)
    return [];
  const rel = sourceRelative(isAbsolute(ticketDir) ? relative2(resolve2(repoRoot), resolve2(ticketDir)) : ticketDir);
  if (isExcluded(rel, taxonomy))
    return [];
  const root = absolutePath(repoRoot, rel);
  if (!existsSync2(root))
    return [];
  const rows = [];
  const walk = (currentRel) => {
    if (isExcluded(currentRel, taxonomy))
      return;
    const currentAbs = absolutePath(repoRoot, currentRel);
    const stat = lstatSync(currentAbs);
    if (stat.isSymbolicLink()) {
      rows.push({ path: currentRel, mode: "120000" });
      return;
    }
    if (!stat.isDirectory()) {
      rows.push({ path: currentRel, mode: (stat.mode & 73) !== 0 ? "100755" : "100644" });
      return;
    }
    rows.push({ path: currentRel, mode: "040000", explicitDirectory: true });
    const nestedGit = taxonomy.schema.fixedDirectoryContracts["nested-git-metadata"];
    if (nestedGit && basename2(currentRel) === ".git" && taxonomyPathPatternMatches(currentRel, nestedGit.pathPattern))
      return;
    const children = readdirSync2(currentAbs).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
    for (const child of children) {
      const childRel = sourceRelative(`${currentRel}/${child}`);
      if (isExcluded(childRel, taxonomy))
        continue;
      walk(childRel);
    }
  };
  walk(rel);
  return rows;
}
function generatorContractsForOutputPath(path, taxonomy) {
  const normalized = normalizeRelative(path);
  return Object.entries(taxonomy.schema.generatorContracts).filter(([, contract]) => contract.outputRoots.some((root) => normalized === root.path || normalized.startsWith(`${root.path}/`))).map(([id, contract]) => ({ id, contract })).sort((left, right) => left.id.localeCompare(right.id));
}
function ignoredGeneratorRows(repoRoot, taxonomy) {
  const rows = new Map;
  const walk = (path) => {
    if (isExcluded(path, taxonomy))
      return;
    const stat = lstatOrNull(absolutePath(repoRoot, path));
    if (!stat)
      return;
    if (stat.isSymbolicLink()) {
      rows.set(path, { path, mode: "120000" });
      return;
    }
    if (!stat.isDirectory()) {
      rows.set(path, { path, mode: (stat.mode & 73) !== 0 ? "100755" : "100644" });
      return;
    }
    rows.set(path, { path, mode: "040000", explicitDirectory: true });
    for (const child of readdirSync2(absolutePath(repoRoot, path)).sort((a, b) => Buffer.from(a).compare(Buffer.from(b))))
      walk(sourceRelative(`${path}/${child}`));
  };
  for (const contract of Object.values(taxonomy.schema.generatorContracts))
    for (const root of contract.outputRoots)
      if (root.inclusion === "ignored")
        walk(root.path);
  return [...rows.values()].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
}
function contentOf(repoRoot, row) {
  if (row.mode === "040000")
    return { kind: "directory", hash: "", mode: 0, size: 0 };
  const path = absolutePath(repoRoot, row.path);
  if (!existsSync2(path) && row.mode !== "120000")
    return { kind: "file", hash: row.objectId ?? sha256(""), mode: 0, size: 0, violation: violation("tracked-path-missing", row.path, "Tracked path is missing from the worktree") };
  try {
    const stat = lstatSync(path);
    if (stat.isSymbolicLink() || row.mode === "120000") {
      const target = readlinkSync(path);
      return { kind: "symlink", hash: sha256(target), mode: stat.mode & 4095, size: Buffer.byteLength(target), symlinkTarget: target };
    }
    if (stat.isDirectory())
      return { kind: "directory", hash: "", mode: stat.mode & 4095, size: 0 };
    const bytes = readFileSync2(path);
    return { kind: "file", hash: sha256(bytes), mode: stat.mode & 4095, size: bytes.byteLength, bytes };
  } catch (error) {
    return { kind: row.mode === "120000" ? "symlink" : "file", hash: row.objectId ?? sha256(""), mode: 0, size: 0, violation: violation("path-read-failed", row.path, error instanceof Error ? error.message : String(error)) };
  }
}
function packageLocation(path, taxonomy) {
  const parts = path.split("/");
  const packageIndex = parts.findIndex((part) => {
    const leading = splitLeadingEmoji(part);
    return leading.rest === "packages" && taxonomy.directoryKinds.some((kind) => kind.id === "packages" && emojiFold(kind.emoji) === emojiFold(leading.emoji));
  });
  if (packageIndex < 0)
    return null;
  const owner = parts.slice(0, packageIndex).join("/");
  const ecosystemSegment = parts[packageIndex + 1] ?? "";
  const ecosystemIds = Object.keys(taxonomy.schema.ecosystems).filter((id) => emojiFold(id) === emojiFold(ecosystemSegment));
  const selected = ecosystemIds.length === 1 && taxonomy.schema.packageBoundaryRules[ecosystemIds[0]] ? [ecosystemIds[0], taxonomy.schema.packageBoundaryRules[ecosystemIds[0]]] : null;
  return { owner, packageRoot: parts.slice(0, packageIndex + 2).join("/"), ecosystemId: selected?.[0] ?? null, rule: selected?.[1] ?? null };
}
function fixedSpecificity(contract) {
  const segments = contract.pathPattern.split("/");
  const tokens = contract.pathPattern.match(/\*\*|\*|\?|\[[^\]]+\]/gu) ?? [];
  const literals = contract.pathPattern.replaceAll("/", "").replace(/\*\*|\*|\?|\[[^\]]+\]/gu, "");
  return [segments.filter((segment) => !/[?*\[]/u.test(segment)).length, [...literals].length, -tokens.length, contract.scope.kind === "path-pattern" ? 0 : 1];
}
function compareFixedSpecificity(left, right) {
  for (let index = 0;index < left.length; index++)
    if (left[index] !== right[index])
      return right[index] - left[index];
  return 0;
}
function equalFixedSpecificity(left, right) {
  return left.every((value, index) => value === right[index]);
}
function fixedScopeMatches(contract, path, packageInfo, parentKindId, parentFixedDirectoryContractId, siblingFixedFilenameContractIds = []) {
  if (contract.scope.kind === "exact-path")
    return path === contract.scope.path;
  if (contract.scope.kind === "repository-root")
    return !path.includes("/");
  if (contract.scope.kind === "package-root")
    return packageInfo?.packageRoot === dirname2(path) && packageInfo.ecosystemId === contract.scope.ecosystemId;
  if (contract.scope.kind === "directory-kind")
    return parentKindId === contract.scope.directoryKindId;
  if (contract.scope.kind === "fixed-directory-contract")
    return parentFixedDirectoryContractId === contract.scope.fixedDirectoryContractId;
  if (contract.scope.kind === "sibling-fixed-filename-contract")
    return siblingFixedFilenameContractIds.includes(contract.scope.fixedFilenameContractId);
  return true;
}
function matchingFixedContracts(path, contracts, packageInfo, parentKindId, parentFixedDirectoryContractId, siblingFixedFilenameContractIds = []) {
  const matches = Object.entries(contracts).filter(([, contract]) => taxonomyPathPatternMatches(path, contract.pathPattern) && fixedScopeMatches(contract, path, packageInfo, parentKindId, parentFixedDirectoryContractId, siblingFixedFilenameContractIds)).map(([id, contract]) => ({ id, contract, specificity: fixedSpecificity(contract) })).sort((left, right) => compareFixedSpecificity(left.specificity, right.specificity) || left.id.localeCompare(right.id));
  if (matches.length === 0)
    return { selected: null, ambiguous: [] };
  const top = matches.filter((entry) => equalFixedSpecificity(entry.specificity, matches[0].specificity));
  return top.length === 1 ? { selected: [top[0].id, top[0].contract], ambiguous: [] } : { selected: null, ambiguous: top.map((entry) => entry.id) };
}
function configurableContract(path, taxonomy, packageInfo) {
  const rows = Object.entries(taxonomy.schema.configurableEntryContracts).filter(([, contract]) => basename2(path).normalize("NFC") === contract.filename.normalize("NFC") && packageInfo?.ecosystemId === contract.ecosystemId);
  return rows.length === 1 ? rows[0] : null;
}
function classifyGlue(analyzer, content, maxStatements) {
  const normalized = content.replace(/\/\*[\s\S]*?\*\//g, "").replace(/^\s*\/\/.*$/gm, "").replace(/^\s*#.*$/gm, "").trim();
  if (normalized.length === 0)
    return "declaration";
  if (analyzer === "rust") {
    if (/\b(?:struct|enum|trait|union|impl)\b/.test(normalized))
      return "implementation";
    const bodies = [...normalized.matchAll(/\bfn\s+\w+[^\{]*\{([\s\S]*?)\}/g)].map((match) => match[1].split(";").map((part) => part.trim()).filter(Boolean).length);
    if (bodies.some((count) => count > maxStatements))
      return "implementation";
    if (/\bfn\s+(?:main|start|bootstrap)\b/.test(normalized))
      return "bootstrap";
    if (/\b(?:register|provide|bind)\w*\s*\(/i.test(normalized))
      return "registration";
    if (/^(?:\s*(?:pub\s+)?(?:mod|use)\b[^;]*;|\s*(?:pub\s+)?extern\s+crate\b[^;]*;|\s*#!?\[[^\]]+\]\s*)+$/s.test(normalized))
      return "declaration";
    return bodies.length > 0 ? "thin-delegation" : "unresolved";
  }
  if (analyzer === "typescript" || analyzer === "javascript") {
    if (/\b(?:class|namespace)\b/.test(normalized))
      return "implementation";
    if (/^(?:\s*(?:import\b[^;]*;?|export\s+(?:\*|\{[^}]*\}|type\b[^;]*|interface\b[^{]*\{[^}]*\}|enum\b[^{]*\{[^}]*\})[^;]*;?)\s*)+$/s.test(normalized))
      return "declaration";
    if (/\b(?:register|provide|bind)\w*\s*\(/i.test(normalized))
      return "registration";
    const functionBodies = [...normalized.matchAll(/(?:function\s+([\w$]+)[^{]*|(?:const|let)\s+([\w$]+)\s*=\s*(?:async\s*)?\([^)]*\)\s*=>)\{([\s\S]*?)\}/g)];
    if (functionBodies.length > 0) {
      const thin = functionBodies.every((match) => {
        const name = match[1] ?? match[2] ?? "";
        const statements = match[3].split(";").map((part) => part.trim()).filter(Boolean);
        return /^(?:main|start|bootstrap|run)$/i.test(name) && statements.length <= maxStatements && statements.every((statement) => /^(?:return\s+)?(?:await\s+)?[\w$.]+\([^;]*\)$/.test(statement));
      });
      return thin ? "thin-delegation" : "implementation";
    }
    if (/=>|\bfunction\b|\.(?:reduce|map|filter|flatMap|sort)\s*\(/.test(normalized))
      return "implementation";
    return "implementation";
  }
  if (analyzer === "go") {
    if (/\btype\s+\w+\s+(?:struct|interface)\b/.test(normalized))
      return "implementation";
    const bodies = [...normalized.matchAll(/\bfunc\s+(?:main|init)\s*\([^)]*\)\s*\{([\s\S]*?)\}/g)];
    if (bodies.length > 0 && bodies.every((match) => match[1].split(`
`).map((line) => line.trim()).filter(Boolean).length <= maxStatements))
      return "bootstrap";
    if (/^package\s+\w+\s+(?:import\s*(?:\([^)]*\)|"[^"]+")\s*)?$/s.test(normalized))
      return "declaration";
    return "implementation";
  }
  if (analyzer === "python") {
    if (/^\s*(?:class|def)\s+/m.test(normalized))
      return "implementation";
    if (/^(?:\s*(?:from\s+\S+\s+import|import\s+|__all__\s*=)[^\n]*\n?)+$/s.test(normalized))
      return "declaration";
    const statements = normalized.split(`
`).map((line) => line.trim()).filter(Boolean).length;
    if (statements <= maxStatements && /if\s+__name__\s*==\s*["']__main__["']/.test(normalized))
      return "bootstrap";
    return "implementation";
  }
  if (analyzer === "c-cpp") {
    if (/\b(?:class|struct|union|enum)\b|\w+\s*\([^;{}]*\)\s*\{/u.test(normalized))
      return "implementation";
    if (/^(?:\s*(?:#\s*(?:include|define|pragma)\b[^\n]*|(?:using|typedef|extern)\b[^;]*;)\s*)+$/su.test(normalized))
      return "declaration";
    return "unresolved";
  }
  if (/\b(?:class|struct|interface|record|enum)\b/.test(normalized))
    return "implementation";
  if (/\b(?:AddSingleton|AddScoped|AddTransient|Register)\b/.test(normalized))
    return "registration";
  if (/^(?:\s*(?:using|global\s+using|\[assembly:)[^;\n]*(?:;|\])\s*)+$/s.test(normalized))
    return "declaration";
  return "unresolved";
}
function classifyPackageRole(path, kindId, fixedId, content, taxonomy) {
  const location = packageLocation(path, taxonomy);
  if (!location)
    return "not-package";
  if (fixedId || configurableContract(path, taxonomy, location))
    return "configuration";
  if (!location.rule || !location.ecosystemId)
    return "unresolved";
  if (kindId && !location.rule.allowedFileKindIds.includes(kindId))
    return "implementation";
  if (!content)
    return "configuration";
  const grammar = taxonomy.schema.packageGlueGrammar[location.rule.glueGrammarId];
  const role = classifyGlue(grammar.analyzer, content, grammar.maxDelegationStatements);
  return grammar.allowedRoles.includes(role) ? role : role === "implementation" ? "implementation" : "unresolved";
}
function canonicalDirectory(path, parentCanonical, parentKindId, ancestorKindIds, taxonomy) {
  const name = basename2(path).normalize("NFC");
  const fixed = matchingFixedContracts(path, taxonomy.schema.fixedDirectoryContracts, packageLocation(path, taxonomy), parentKindId);
  if (fixed.ambiguous.length > 0)
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("fixed-directory-contract-ambiguous", path, `Equal-specificity fixed directory contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected) {
    const context = matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: context.kind?.id ?? null, fixedId: fixed.selected[0], violations: [] };
  }
  if (parentKindId === "packages") {
    const packageKinds = Object.keys(taxonomy.schema.packageBoundaryRules).filter((id) => emojiFold(id) === emojiFold(name));
    if (packageKinds.length === 1)
      return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: `package-language:${packageKinds[0]}`, violations: [] };
    if (packageKinds.length > 1)
      return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation("package-language-ambiguous", path, `Package language boundary is ambiguous: ${packageKinds.join(", ")}`)] };
  }
  const match = matchDirectoryKind(name, taxonomy, parentKindId, ancestorKindIds);
  if (!match.kind) {
    const message = match.ambiguous.length > 1 ? `Directory semantic kind is ambiguous: ${match.ambiguous.join(", ")}` : "Directory has no registered semantic kind";
    return { path: parentCanonical ? `${parentCanonical}/${name}` : name, kindId: null, violations: [violation(match.ambiguous.length > 1 ? "directory-kind-ambiguous" : "directory-kind-unresolved", path, message)] };
  }
  const canonicalName = `${match.kind.emoji}${match.slug}`.normalize("NFC");
  return { path: parentCanonical ? `${parentCanonical}/${canonicalName}` : canonicalName, kindId: match.kind.id, violations: [] };
}
function canonicalFile(path, parentCanonical, parentKindId, ancestorKindIds, directoryKindByPath, fixedDirectoryContractByPath, siblingFixedFilenameContractIdsByParent, taxonomy, contentKindId) {
  const packageInfo = packageLocation(path, taxonomy);
  let fixedName = basename2(path);
  const parent = dirname2(path);
  let fixed = matchingFixedContracts(path, taxonomy.schema.fixedFilenameContracts, packageInfo, directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent), siblingFixedFilenameContractIdsByParent.get(parent));
  const decoratedFixedName = splitLeadingEmoji(fixedName);
  if (!fixed.selected && fixed.ambiguous.length === 0 && decoratedFixedName.emoji && decoratedFixedName.rest) {
    const candidatePath = dirname2(path) === "." ? decoratedFixedName.rest : `${dirname2(path)}/${decoratedFixedName.rest}`;
    const candidate = matchingFixedContracts(candidatePath, taxonomy.schema.fixedFilenameContracts, packageLocation(candidatePath, taxonomy), directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent), siblingFixedFilenameContractIdsByParent.get(parent));
    if (candidate.selected || candidate.ambiguous.length > 0) {
      fixed = candidate;
      fixedName = decoratedFixedName.rest;
    }
  }
  if (fixed.ambiguous.length > 0)
    return { path: parentCanonical ? `${parentCanonical}/${basename2(path)}` : basename2(path), fileKind: null, stem: null, violations: [violation("fixed-contract-ambiguous", path, `Equal-specificity fixed filename contracts match: ${fixed.ambiguous.join(", ")}`)] };
  if (fixed.selected)
    return { path: parentCanonical ? `${parentCanonical}/${fixedName}` : fixedName, fileKind: null, stem: null, fixedId: fixed.selected[0], violations: [] };
  const configurable = configurableContract(path, taxonomy, packageInfo);
  const resolvedKind = resolveFileKind(path, taxonomy, parentKindId, ancestorKindIds, configurable?.[1].fileKindId, contentKindId);
  if (!resolvedKind.kind) {
    const message = resolvedKind.ambiguous.length > 1 ? `File kind is ambiguous: ${resolvedKind.ambiguous.join(", ")}` : "No file kind owns the longest extension chain";
    return { path: parentCanonical ? `${parentCanonical}/${basename2(path).normalize("NFC")}` : basename2(path).normalize("NFC"), fileKind: null, stem: null, violations: [violation(resolvedKind.ambiguous.length > 1 ? "file-kind-ambiguous" : "file-kind-unresolved", path, message)] };
  }
  const leadingSemantic = splitLeadingEmoji(resolvedKind.stem);
  const semanticEvidence = leadingSemantic.emoji || "";
  const sourceStem = semanticEvidence ? leadingSemantic.rest : resolvedKind.stem;
  const testSuffix = sourceStem.endsWith(".test");
  const semanticStem = testSuffix ? sourceStem.slice(0, -".test".length) : sourceStem;
  const kindOnly = `${resolvedKind.kind.emoji}${resolvedKind.extension}`.normalize("NFC");
  if (!semanticStem || configurable || GENERIC_SEMANTIC_STEMS.has(semanticStem.toLocaleLowerCase("und")))
    return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem || null, violations: [] };
  const parentSlug = splitLeadingEmoji(basename2(dirname2(path))).rest;
  if (parentSlug.normalize("NFC").toLocaleLowerCase("und") === semanticStem.normalize("NFC").toLocaleLowerCase("und"))
    return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [] };
  const roleContext = testSuffix ? "tests" : resolvedKind.kind.role === "asset" ? "assets" : resolvedKind.kind.role === "test" ? "tests" : parentKindId;
  const semantic = matchDirectoryKind(`${semanticEvidence}${semanticStem}`, taxonomy, roleContext);
  if (!semantic.kind) {
    const message = semantic.ambiguous.length > 1 ? `Semantic stem matches multiple directory kinds: ${semantic.ambiguous.join(", ")}` : "Semantic stem has no registered directory kind";
    return { path: parentCanonical ? `${parentCanonical}/${basename2(path).normalize("NFC")}` : basename2(path).normalize("NFC"), fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [violation(semantic.ambiguous.length > 1 ? "semantic-stem-ambiguous" : "semantic-stem-unresolved", path, message)] };
  }
  if (parentKindId === semantic.kind.id && parentSlug === semanticStem)
    return { path: parentCanonical ? `${parentCanonical}/${kindOnly}` : kindOnly, fileKind: resolvedKind.kind.id, stem: semanticStem, violations: [] };
  const semanticDirectory = `${semantic.kind.emoji}${semanticStem}`.normalize("NFC");
  return { path: parentCanonical ? `${parentCanonical}/${semanticDirectory}/${kindOnly}` : `${semanticDirectory}/${kindOnly}`, fileKind: resolvedKind.kind.id, stem: semanticStem, semanticDirectoryName: semanticDirectory, violations: [] };
}
function packageImplementationDestination(sourcePath, canonical, canonicalDirectoryByPath, directoryKindByPath, taxonomy) {
  const location = packageLocation(sourcePath, taxonomy);
  if (!location || !canonical.fileKind)
    return null;
  const ownerCanonical = canonicalDirectoryByPath.get(location.owner) ?? location.owner.normalize("NFC");
  const fileName = basename2(canonical.path);
  const stem = canonical.stem?.normalize("NFC") ?? "";
  if (!stem || GENERIC_SEMANTIC_STEMS.has(stem.toLocaleLowerCase("und")))
    return ownerCanonical ? `${ownerCanonical}/${fileName}` : fileName;
  if (canonical.semanticDirectoryName)
    return ownerCanonical ? `${ownerCanonical}/${canonical.semanticDirectoryName}/${fileName}` : `${canonical.semanticDirectoryName}/${fileName}`;
  const semantic = matchDirectoryKind(stem, taxonomy, directoryKindByPath.get(location.owner));
  if (!semantic.kind)
    return null;
  const directoryName = `${semantic.kind.emoji}${stem}`.normalize("NFC");
  return ownerCanonical ? `${ownerCanonical}/${directoryName}/${fileName}` : `${directoryName}/${fileName}`;
}
function directoryHash(path, children) {
  const prefix = path ? `${path}/` : "";
  const rows = [...children].sort((a, b) => Buffer.from(a.sourcePath).compare(Buffer.from(b.sourcePath))).map((entry) => `${entry.nodeKind}\x00${entry.mode ?? ""}\x00${entry.sourcePath.slice(prefix.length)}\x00${entry.contentHash}`);
  return sha256(rows.join("\x00"));
}
function inventoryDigestOf(inventory) {
  return sha256(canonicalJson(inventory));
}
var indexedLineContent = "";
var indexedLineStarts = [0];
function lineLocation(content, start, label) {
  if (indexedLineContent !== content) {
    const starts = [0];
    for (let index = content.indexOf(`
`);index >= 0; index = content.indexOf(`
`, index + 1))
      starts.push(index + 1);
    indexedLineContent = content;
    indexedLineStarts = starts;
  }
  let low = 0;
  let high = indexedLineStarts.length;
  while (low < high) {
    const middle = low + high >>> 1;
    if (indexedLineStarts[middle] <= start)
      low = middle + 1;
    else
      high = middle;
  }
  const line = Math.max(1, low);
  const column = start - indexedLineStarts[line - 1] + 1;
  return `${label}:${line}:${column}@${start}`;
}
function regexTokens(content, adapter, label, patterns) {
  const rows = [];
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      const value = match[1];
      if (typeof value !== "string" || match.index === undefined)
        continue;
      const relativeIndex = match[0].indexOf(value);
      const start = match.index + relativeIndex;
      rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
    }
  }
  return rows;
}
function argumentTokens(content, fragment, fragmentStart, adapter, label) {
  const rows = [];
  for (const match of fragment.matchAll(/"([^"]+)"|'([^']+)'|([^\s()[\],;]+)/gu)) {
    if (match.index === undefined)
      continue;
    const value = match[1] ?? match[2] ?? match[3];
    if (!value || /^(?:=>|PUBLIC|PRIVATE|INTERFACE|EXCLUDE_FROM_ALL)$/u.test(value))
      continue;
    const inner = match[0].indexOf(value);
    const start = fragmentStart + match.index + inner;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
  }
  return rows;
}
function embeddedArgumentTokens(content, value, valueStart, adapter, label) {
  if (!/\s|(?:^|\s)--?[\w-]+=|\$\{(?:workspaceFolder|workspaceRoot)\}/u.test(value))
    return [];
  const rows = [];
  for (const match of value.matchAll(/[^\s"'`]+/gu)) {
    if (match.index === undefined)
      continue;
    let candidate = match[0].replace(/^[[(]+|[\]),;]+$/gu, "");
    let offset = match[0].indexOf(candidate);
    const assignment = candidate.match(/^--?[\w-]+=(.+)$/u);
    if (assignment) {
      offset += candidate.indexOf(assignment[1]);
      candidate = assignment[1];
    }
    const workspace = candidate.match(/^\$\{(?:workspaceFolder|workspaceRoot)\}\/(.+)$/u);
    if (workspace) {
      offset += candidate.indexOf(workspace[1]);
      candidate = workspace[1];
    }
    if (!candidate || /^(?:bun|node|python|python3|go|cargo|nx|run|test|build)$/u.test(candidate))
      continue;
    const start = valueStart + match.index + offset;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + candidate.length, value: candidate });
  }
  for (const match of value.matchAll(/(?:\.\.?\/|\/)[^\s\\"'`()\],;]+/gu)) {
    if (match.index === undefined)
      continue;
    const start = valueStart + match.index;
    rows.push({ adapter, structuredLocation: lineLocation(content, start, label), start, end: start + match[0].length, value: match[0] });
  }
  return [...new Map(rows.map((entry) => [`${entry.start}\x00${entry.end}\x00${entry.value}`, entry])).values()].sort((left, right) => left.start - right.start || left.value.localeCompare(right.value));
}
var OLD_MUTATION_TEST_PREFIX_SOURCE = "\uD83C\uDFC5\uFE0Fstandards/\uD83D\uDD16\uFE0F([^/\\s\"'`|]+)\\/\uD83E\uDE86\uFE0Fsubsets/\u2733\uFE0F([^/\\s\"'`|]+)\\/\uD83E\uDDEC\uFE0Fschema/\uD83E\uDDEC\uFE0Fmutations\\/([^/\\s\"'`|]+)\\/\uD83E\uDDEA\uFE0Ftests\\/";
var OLD_MUTATION_STRUCTURE_SOURCE = `${OLD_MUTATION_TEST_PREFIX_SOURCE}([^/\\s"'\`|]+)(\\/[^\\s"'\`|)>}\\]]+)?`;
function artifactRootForPath(path) {
  const segments = normalizeRelative(path).split("/");
  const index = segments.findIndex((segment) => emojiFold(segment) === emojiFold("\uD83D\uDDFF\uFE0Fartifacts"));
  if (index >= 0 && index + 1 < segments.length)
    return segments.slice(0, index + 2).join("/");
  const standards = segments.findIndex((segment) => emojiFold(segment) === emojiFold("\uD83C\uDFC5\uFE0Fstandards"));
  return standards > 0 ? segments.slice(0, standards).join("/") : null;
}
function mutationStructuralPaths(content, fragmentStart = 0) {
  const rows = [];
  const pattern = new RegExp(OLD_MUTATION_STRUCTURE_SOURCE, "gu");
  for (const match of content.matchAll(pattern)) {
    if (match.index === undefined)
      continue;
    rows.push({ value: match[0], start: fragmentStart + match.index, standard: match[1], subset: match[2], mutation: match[3], scenario: match[4], suffix: match[5] ?? "" });
  }
  return rows;
}
function canonicalProjectionSuffix(suffix) {
  const segments = suffix.split("/");
  const name = segments.at(-1) ?? "";
  const leading = splitLeadingEmoji(name);
  if (leading.emoji && /^component\.[a-z0-9.]+$/u.test(leading.rest))
    segments[segments.length - 1] = `${leading.emoji}.${leading.rest.slice("component.".length)}`;
  return segments.join("/");
}
function projectionKey(artifactRoot, standard, subset) {
  return `${artifactRoot}\x00${standard}\x00${subset}`;
}
function projectedStructuralValue(row) {
  const scenario = splitLeadingEmoji(row.scenario).emoji ? row.scenario : `\uD83E\uDDEA\uFE0F${row.scenario}`;
  return `\uD83E\uDDEA\uFE0Ftests/\uD83E\uDE86\uFE0F${row.standard}-${row.subset}/${row.mutation}/${scenario}${canonicalProjectionSuffix(row.suffix)}`.normalize("NFC");
}
function structuralProjectionToken(content, row, adapter, label, artifactRoot, prefix = "") {
  const value = `${prefix}${row.value}`;
  const start = row.start - prefix.length;
  const target = artifactRoot && !/[<>]/u.test(row.value) ? `${artifactRoot}/${row.value}` : undefined;
  return {
    adapter,
    structuredLocation: label.startsWith("/") ? `${label}@${start}` : lineLocation(content, start, label),
    start,
    end: start + value.length,
    value,
    targetValues: target ? [target] : undefined,
    rewriteKind: prefix === "asset://" ? "artifact-uri" : "projection-prose",
    rewriteData: {
      newValue: `${prefix}${projectedStructuralValue(row)}`,
      projectionKey: artifactRoot ? projectionKey(artifactRoot, row.standard, row.subset) : "",
      projectionProfile: `${row.standard}\x00${row.subset}`,
      artifactRoot: artifactRoot ?? ""
    }
  };
}
function structuralTokensInFragment(content, fragment, fragmentStart, adapter, label, artifactRoot) {
  const rows = [];
  for (const structural of mutationStructuralPaths(fragment, fragmentStart)) {
    const localStart = structural.start - fragmentStart;
    const before = fragment.slice(0, localStart);
    const prefix = before.endsWith("asset://") ? "asset://" : before.match(/(?:(?:\.\.\/|\.\/)+)$/u)?.[0] ?? "";
    rows.push(structuralProjectionToken(content, structural, adapter, prefix === "asset://" && adapter === "gherkin" ? "gherkin" : label, artifactRoot, prefix));
  }
  return rows;
}
function jsonTokens(path, content, adapter) {
  const rows = [];
  let ordinal = 0;
  const embeddedArgv = /(?:^|\/)(?:launch(?:\.seed)?\.jsonc?|tasks\.json|project\.json|package\.json)$/iu.test(path);
  for (const match of content.matchAll(/"((?:\\.|[^"\\])*)"/g)) {
    if (match.index === undefined)
      continue;
    const tail = content.slice(match.index + match[0].length).match(/^\s*/)?.[0].length ?? 0;
    const key = content[match.index + match[0].length + tail] === ":";
    let value;
    try {
      value = JSON.parse(match[0]);
    } catch {
      continue;
    }
    const raw = match[1];
    const start = match.index + 1;
    if (raw === value)
      rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${ordinal++}]@${start}`, start, end: start + raw.length, value });
    const workspaceGlob = !key && raw === value ? value.match(/^\{workspaceRoot\}\/(.+?)(\/\*\*\/\*[^/]*)$/u) : null;
    if (workspaceGlob)
      rows.push({ adapter, structuredLocation: `${key ? "/@key" : "/@value"}[${Math.max(0, ordinal - 1)}]/workspace-glob@${start}`, start, end: start + raw.length, value, targetValues: [workspaceGlob[1]], rewriteKind: "path-prefix", rewriteData: { prefix: "{workspaceRoot}/", suffix: workspaceGlob[2] } });
    if (!key && raw !== value && /^\{workspaceRoot\}\/.+\/\*\*/u.test(value))
      rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/workspace-glob@${start}`, start, end: start + raw.length, value: raw, rewriteKind: "path-prefix", unsupportedReason: "Escaped workspace projection glob has no proven decoded-to-raw offset map" });
    if (!key && raw === value)
      rows.push(...structuralTokensInFragment(content, raw, start, adapter, `/@value[${Math.max(0, ordinal - 1)}]/prose`, artifactRootForPath(path)));
    if (!key && raw !== value && mutationStructuralPaths(value).length > 0)
      rows.push({ adapter, structuredLocation: `/@value[${Math.max(0, ordinal - 1)}]/prose@${start}`, start, end: start + raw.length, value: raw, unsupportedReason: "Escaped JSON projection prose has no proven decoded-to-raw offset map" });
    if (!key && embeddedArgv)
      rows.push(...embeddedArgumentTokens(content, raw, start, adapter, "embedded-argv"));
  }
  return rows;
}
function tomlTokens(path, content) {
  const adapter = "toml";
  const rows = [];
  for (const match of content.matchAll(/"([^"\r\n]+)"|'([^'\r\n]+)'/gu)) {
    if (match.index === undefined)
      continue;
    const value = match[1] ?? match[2];
    const start = match.index + match[0].indexOf(value);
    const entrypoint = value.match(/^([A-Za-z_]\w*(?:\.[A-Za-z_]\w*)+):([A-Za-z_]\w*)$/u);
    const prefix = content.slice(0, start);
    const section = [...prefix.matchAll(/^\s*\[([^\]]+)\]\s*$/gmu)].at(-1)?.[1];
    const lineStart = prefix.lastIndexOf(`
`) + 1;
    const key = content.slice(lineStart, start).match(/^\s*([A-Za-z0-9_.-]+)\s*=\s*["']/u)?.[1];
    const label = section && key ? `${section}.${key}` : "toml-string";
    rows.push(entrypoint ? { adapter, structuredLocation: lineLocation(content, start, "python-entrypoint"), start, end: start + value.length, value, targetValues: [entrypoint[1]], rewriteKind: "python-entrypoint", rewriteData: { suffix: `:${entrypoint[2]}` } } : { adapter, structuredLocation: lineLocation(content, start, label), start, end: start + value.length, value });
  }
  return rows;
}
function rustTokens(path, content) {
  const rows = regexTokens(content, "rust", "rust-string-path", [/#\s*\[\s*path\s*=\s*"([^"]+)"/gu, /\b(?:include|include_str|include_bytes)!\s*\(\s*"([^"]+)"/gu, /\.join\(\s*"([^"]+)"\s*\)/gu]);
  for (const match of content.matchAll(/\.join\(\s*"([^"]*\uD83D\uDDBC\uFE0Fassets\/\uD83C\uDFD7\uFE0FmodelDefinitions)"\s*\)/gu)) {
    if (match.index === undefined)
      continue;
    const start = match.index + match[0].indexOf(match[1]);
    rows.push({ adapter: "rust", structuredLocation: lineLocation(content, start, "artifact-catalog-root-join"), start, end: start + match[1].length, value: match[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "relative-root" } });
  }
  for (const match of content.matchAll(/^([ \t]*)((?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;)/gmu)) {
    if (match.index === undefined)
      continue;
    const statement = match[2];
    const start = match.index + match[1].length;
    const name = match[3];
    rows.push({
      adapter: "rust",
      structuredLocation: lineLocation(content, start, "rust-mod"),
      start,
      end: start + statement.length,
      value: statement,
      targetValues: [`./${name}.rs`, `./${name}/mod.rs`],
      rewriteKind: "rust-mod",
      rewriteData: { indentation: match[1], declaration: statement }
    });
  }
  for (const match of content.matchAll(/(?:^|\n)[ \t]*(?:(?:\/\/\/)|(?:\/\/!)|(?:\/\/))([^\r\n]*)/gu)) {
    if (match.index === undefined)
      continue;
    const fragment = match[1];
    const start = match.index + match[0].indexOf(fragment);
    rows.push(...structuralTokensInFragment(content, fragment, start, "rust", "rust-comment", artifactRootForPath(path)));
    for (const quoted of fragment.matchAll(/`([^`]+)`/gu)) {
      if (quoted.index === undefined || !/[/.]/u.test(quoted[1]))
        continue;
      const tokenStart = start + quoted.index + quoted[0].indexOf(quoted[1]);
      rows.push({ adapter: "rust", structuredLocation: lineLocation(content, tokenStart, "rust-comment-path"), start: tokenStart, end: tokenStart + quoted[1].length, value: quoted[1] });
    }
    const catalog = fragment.match(/(\uD83D\uDDBC\uFE0Fassets\/\uD83C\uDFD7\uFE0FmodelDefinitions\/\*\/\uD83C\uDFAC\uFE0Finteractions\/\*\.json)/u);
    if (catalog) {
      const tokenStart = start + fragment.indexOf(catalog[1]);
      rows.push({ adapter: "rust", structuredLocation: lineLocation(content, tokenStart, "artifact-catalog-comment"), start: tokenStart, end: tokenStart + catalog[1].length, value: catalog[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "interaction-glob" } });
    }
  }
  return rows;
}
function pythonTokens(path, content) {
  const rows = regexTokens(content, "python", "python-reference", [/^\s*from\s+([\w.]+)\s+import\s+/gmu, /^\s*import\s+([\w.]+)(?:\s+as\s+\w+)?\s*$/gmu, /\b(?:open|Path|joinpath|files|read_text|read_bytes)\s*\(\s*["']([^"']+)["']/gu, /__file__[^\r\n]*?\/\s*["']([^"']+)["']/gu]);
  for (const match of content.matchAll(/^\s*([A-Z][A-Z0-9_]*VECTOR_ROOT|VECTOR_ROOT)\s*=\s*["'](asset:\/\/\uD83C\uDFC5\uFE0Fstandards\/\uD83D\uDD16\uFE0F([^/"']+)\/\uD83E\uDE86\uFE0Fsubsets\/\u2733\uFE0F([^/"']+)\/\uD83E\uDDEC\uFE0Fschema\/\uD83E\uDDEC\uFE0Fmutations)["']/gmu)) {
    if (match.index === undefined)
      continue;
    const value = match[2];
    const start = match.index + match[0].indexOf(value);
    rows.push({ adapter: "python", structuredLocation: lineLocation(content, start, `python-string:${match[1]}`), start, end: start + value.length, value, rewriteKind: "structural-projection", rewriteData: { newValue: `asset://\uD83E\uDDEA\uFE0Ftests/\uD83E\uDE86\uFE0F${match[3]}-${match[4]}`, projectionKey: "", projectionProfile: `${match[3]}\x00${match[4]}`, artifactRoot: artifactRootForPath(path) ?? "" } });
  }
  for (const match of content.matchAll(/^\s*(stem)\s*=\s*["'](%s\/%s\/\uD83E\uDDEA\uFE0Ftests\/%s)["']\s*%/gmu)) {
    if (match.index === undefined)
      continue;
    const value = match[2];
    const start = match.index + match[0].indexOf(value);
    rows.push({ adapter: "python", structuredLocation: lineLocation(content, start, `python-format:${match[1]}`), start, end: start + value.length, value, rewriteKind: "structural-projection", rewriteData: { newValue: "%s/%s/\uD83E\uDDEA\uFE0F%s", projectionKey: "", projectionProfile: "*", artifactRoot: artifactRootForPath(path) ?? "" } });
  }
  return rows;
}
function gherkinTokens(path, content) {
  return structuralTokensInFragment(content, content, 0, "gherkin", "gherkin-description", artifactRootForPath(path));
}
function typescriptTokens(path, content) {
  const rows = regexTokens(content, "typescript", "typescript-path", [
    /(?:\bfrom\s*|\bimport\s*\(|\brequire\s*\(|\bimport\s+)["'\s]*([^"'\s)]+)["']/gu,
    /\b(?:worker|url)\s*\(\s*["']([^"']+)["']/giu,
    /\b(?:[A-Za-z_$][\w$]*(?:Path|File|Filename|Root|Schema|Taxonomy|Config|Entry|Target|Source|Output|Input)[\w$]*|(?:path|file|filename|root|schema|taxonomy|config|entry|target|source|output|input))\s*(?:=|:)\s*["']([^"']+)["']/giu,
    /\b(?:resolve|join|readFileSync|writeFileSync|existsSync|openSync|Bun\.file)\s*\([^;\r\n]*?["']([^"']+)["']/giu
  ]);
  for (const match of content.matchAll(/^([^\r\n]*\bimport\.meta\.glob\s*\([^\r\n]+(?:\r?\n|$))/gmu)) {
    if (match.index === undefined)
      continue;
    const selectors = [...match[1].matchAll(/["']([^"']*modelDefinitions[^"']*)["']/gu)].map((row) => row[1]);
    if (selectors.length === 0)
      continue;
    const start = match.index;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, start, "artifact-catalog-glob"), start, end: start + match[1].length, value: match[1], rewriteKind: "artifact-catalog-glob", rewriteData: { selectors: JSON.stringify(selectors) } });
  }
  for (const match of content.matchAll(/(\uD83D\uDDBC\uFE0Fassets\/\uD83C\uDFD7\uFE0FmodelDefinitions\/<modelDefinition>\/\{[\s\S]*?\})/gu)) {
    if (match.index === undefined)
      continue;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, match.index, "artifact-catalog-comment"), start: match.index, end: match.index + match[1].length, value: match[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "catalog-grammar" } });
  }
  for (const match of content.matchAll(/(\uD83D\uDDBC\uFE0Fassets\/\uD83C\uDFD7\uFE0FmodelDefinitions\/)/gu)) {
    if (match.index === undefined || rows.some((token) => token.start <= match.index && token.end >= match.index + match[1].length))
      continue;
    rows.push({ adapter: "typescript", structuredLocation: lineLocation(content, match.index, "artifact-catalog-marker"), start: match.index, end: match.index + match[1].length, value: match[1], rewriteKind: "artifact-catalog-prose", rewriteData: { form: "root-marker" } });
  }
  for (const declaration of content.matchAll(/\bconst\s+([A-Za-z_$][\w$]*(?:Sources|Paths|Files))\s*=\s*\[([\s\S]*?)\]\s*(?:\.map\b|;)/gu)) {
    if (declaration.index === undefined || !new RegExp(`\\b${declaration[1]}\\b[\\s\\S]*?\\.map\\([\\s\\S]*?\\b(?:policyReadFileSafe|readFileSync|Bun\\.file)\\b`, "u").test(content.slice(declaration.index)))
      continue;
    const fragmentStart = declaration.index + declaration[0].indexOf(declaration[2]);
    for (const token of regexTokens(declaration[2], "typescript", "path-collection", [/["']([^"']+)["']/gu]))
      rows.push({ ...token, start: fragmentStart + token.start, end: fragmentStart + token.end, structuredLocation: lineLocation(content, fragmentStart + token.start, "path-collection") });
  }
  return rows;
}
function goTokens(path, content) {
  const rows = [];
  if (path.toLowerCase().endsWith(".go")) {
    rows.push(...regexTokens(content, "go", "go-import", [/^\s*(?:[\w.]+\s+)?"([^"]+)"\s*$/gmu]));
    for (const match of content.matchAll(/^\s*\/\/go:(?:embed|generate)\s+([^\r\n]+)$/gmu))
      if (match.index !== undefined)
        rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-directive"));
    return rows;
  }
  for (const match of content.matchAll(/\buse\s*\(([\s\S]*?)\)/gu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*use\s+([^\r\n(][^\r\n]*)$/gmu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-work-use"));
  for (const match of content.matchAll(/^\s*replace\s+[^\r\n=]+=>\s*([^\s]+).*$/gmu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "go", "go-mod-replace"));
  return rows;
}
function cmakeTokens(content) {
  const rows = [];
  for (const match of content.matchAll(/\b(?:add_subdirectory|add_executable|add_library|target_sources|include|configure_file|set)\s*\(([\s\S]*?)\)/giu))
    if (match.index !== undefined)
      rows.push(...argumentTokens(content, match[1], match.index + match[0].indexOf(match[1]), "native", "cmake-argument"));
  return rows;
}
function htmlTokens(content, adapter) {
  return regexTokens(content, adapter, "html-attribute", [/<(?:a|img|script|link|source|video|audio|form)\b[^>]*\b(?:href|src|srcset|poster|data|action)\s*=\s*["']([^"']+)["'][^>]*>/giu]);
}
function referenceTokens(path, content) {
  const lower = path.toLowerCase();
  if (lower.endsWith(".rs"))
    return rustTokens(path, content);
  if (lower.endsWith(".feature"))
    return gherkinTokens(path, content);
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower))
    return typescriptTokens(path, content);
  if (/\.(?:go|mod|work)$/u.test(lower) || /(?:^|\/)go\.(?:mod|work)$/u.test(lower))
    return goTokens(path, content);
  if (lower.endsWith(".py"))
    return pythonTokens(path, content);
  if (/\.(?:csproj|fsproj|vbproj|sln|props|targets|cs|fs|vb)$/u.test(lower))
    return regexTokens(content, "dotnet", "dotnet-reference", [/(?:Include|Update|Remove|Link|HintPath)\s*=\s*["']([^"']+)["']/giu, /^Project\([^\r\n]+?=\s*[^,]+,\s*"([^"]+)"/gmu, /\b(?:GetManifestResourceStream|ReadAllText|ReadAllBytes)\s*\(\s*["']([^"']+)["']/gu]);
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename2(path) === "CMakeLists.txt")
    return [...regexTokens(content, "native", "native-path", [/^\s*#\s*include\s*[<"]([^>"]+)[>"]/gmu, /["']([^"']+\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx))["']/gu]), ...cmakeTokens(content)];
  if (lower.endsWith(".json"))
    return jsonTokens(path, content, "json");
  if (lower.endsWith(".jsonc"))
    return jsonTokens(path, content, "jsonc");
  if (lower.endsWith(".toml"))
    return tomlTokens(path, content);
  if (/\.ya?ml$/u.test(lower)) {
    const direct = regexTokens(content, "yaml", "yaml-value", [/^\s*(?:-\s*)?[\w.-]+\s*:\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu, /^\s*-\s*["']?([^"'\s][^\r\n#]*?)["']?\s*(?:#.*)?$/gmu]);
    const embeddedArgv = /(?:workflow|action|launch|task|project|(?:^|\/)ci(?:\/|$))/iu.test(path);
    return embeddedArgv ? [...direct, ...direct.flatMap((token) => embeddedArgumentTokens(content, token.value, token.start, "yaml", "embedded-argv"))] : direct;
  }
  if (/\.(?:xml|html|htm)$/u.test(lower))
    return [...regexTokens(content, "xml", "xml-attribute", [/(?:href|src|path|include|file|link|hintpath)\s*=\s*["']([^"']+)["']/giu]), ...htmlTokens(content, "xml")];
  if (/\.(?:md|mdx)$/u.test(lower))
    return [...regexTokens(content, "markdown", "markdown-link", [/!?(?:\[[^\]]*\])\(([^)\s]+)(?:\s+"[^"]*")?\)/gu, /^\s*\[[^\]]+\]:\s*(\S+)/gmu]), ...htmlTokens(content, "markdown")];
  return [];
}
function textualPath(path) {
  return /(?:\.rs|\.tsx?|\.jsx?|\.mjs|\.cjs|\.mts|\.cts|\.go|\.mod|\.work|\.py|\.cs|\.fs|\.vb|\.csproj|\.fsproj|\.vbproj|\.sln|\.props|\.targets|\.c|\.cc|\.cpp|\.cxx|\.h|\.hh|\.hpp|\.hxx|\.cmake|\.jsonc?|\.toml|\.ya?ml|\.xml|\.html?|\.mdx?|\.feature)$/iu.test(path) || basename2(path) === "CMakeLists.txt";
}
function splitTokenSuffix(value) {
  const index = value.search(/[?#]/);
  return index < 0 ? { path: value, suffix: "" } : { path: value.slice(0, index), suffix: value.slice(index) };
}
function addUniqueIndex(index, key, value) {
  if (!key)
    return;
  const existing = index.get(key);
  if (existing === undefined)
    index.set(key, value);
  else if (existing !== value)
    index.set(key, null);
}
function referencePathIndex(paths) {
  const exact = new Set;
  const nfc = new Map;
  const extensionless = new Map;
  const pythonModule = new Map;
  for (const path of paths) {
    exact.add(path);
    const normalized = path.normalize("NFC");
    addUniqueIndex(nfc, normalized, path);
    addUniqueIndex(extensionless, normalized.replace(/\.[^/.]+(?:\.[^/.]+)*$/u, ""), path);
    if (!normalized.endsWith(".py"))
      continue;
    const moduleSegments = (normalized.endsWith("/__init__.py") ? dirname2(normalized) : normalized.slice(0, -3)).split("/").filter(Boolean);
    for (let index = 0;index < moduleSegments.length; index++)
      addUniqueIndex(pythonModule, moduleSegments.slice(index).join("."), path);
  }
  return { exact, nfc, extensionless, pythonModule };
}
function resolveReferencePath(referencePath, token, index) {
  const split = splitTokenSuffix(token);
  if (!split.path || /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/i.test(split.path) || /[*{}]/.test(split.path))
    return null;
  const candidates = [];
  try {
    candidates.push(normalizeRelative(split.path.replace(/^\//, "")));
  } catch {}
  try {
    candidates.push(normalizeRelative(posix.join(dirname2(referencePath), split.path)));
  } catch {}
  for (const candidate of candidates) {
    if (index.exact.has(candidate))
      return candidate;
    const comparison = candidate.normalize("NFC");
    const nfc = index.nfc.get(comparison);
    if (nfc)
      return nfc;
    const extensionless = index.extensionless.get(comparison);
    if (extensionless)
      return extensionless;
  }
  if (/^[\w.]+$/.test(split.path)) {
    const python = index.pythonModule.get(split.path.normalize("NFC"));
    if (python)
      return python;
  }
  return null;
}
function resolveReferenceTokenPath(referencePath, token, index) {
  const matches = [...new Set((token.targetValues ?? [token.value]).map((value) => resolveReferencePath(referencePath, value, index)).filter((value) => value !== null))];
  return matches.length === 1 ? matches[0] : null;
}
function lexicalOpaqueReferenceTarget(referencePath, token, taxonomy) {
  for (const value of token.targetValues ?? [token.value]) {
    const path = splitTokenSuffix(value).path;
    if (!path || /^(?:[a-z][a-z0-9+.-]*:|#|@|\$|\{)/iu.test(path) || /[*{}]/u.test(path))
      continue;
    const candidates = [path.replace(/^\//u, ""), posix.join(dirname2(referencePath), path)];
    for (const candidate of candidates) {
      try {
        const normalized = normalizeRelative(candidate);
        if (isExcluded(normalized, taxonomy))
          return normalized;
      } catch {}
    }
  }
  return null;
}
function rewriteReferenceValue(referencePath, oldValue, oldTarget, newTarget, sourceReferencePath = referencePath) {
  const split = splitTokenSuffix(oldValue);
  if (/^[\w.]+$/.test(split.path) && oldTarget.endsWith(".py")) {
    const modulePath = newTarget.replace(/(?:\/__init__)?\.py$/, "").replaceAll("/", ".");
    return `${modulePath}${split.suffix}`;
  }
  const absoluteStyle = split.path.startsWith("/");
  const relativeStyle = split.path.startsWith("./") || split.path.startsWith("../");
  let localBareStyle = false;
  if (!absoluteStyle && !relativeStyle) {
    try {
      localBareStyle = normalizeRelative(posix.join(dirname2(sourceReferencePath), split.path)) === oldTarget;
    } catch {}
  }
  const omittedExtension = !posix.extname(split.path);
  let value = absoluteStyle ? `/${newTarget}` : relativeStyle || localBareStyle ? posix.relative(dirname2(referencePath), newTarget) : newTarget;
  if (relativeStyle && !value.startsWith("."))
    value = `./${value}`;
  if (omittedExtension) {
    const finalName = basename2(newTarget);
    const extensionStart = finalName.indexOf(".");
    const extensionChain = extensionStart < 0 ? "" : finalName.slice(extensionStart);
    if (extensionChain && value.endsWith(extensionChain))
      value = value.slice(0, -extensionChain.length);
  }
  if (oldValue.includes("\\"))
    value = value.replaceAll("/", "\\");
  return `${value}${split.suffix}`;
}
function rewriteReferenceToken(referencePath, sourceReferencePath, token, oldTarget, newTarget) {
  if (token.rewriteKind === "rust-mod") {
    let relativeTarget = posix.relative(dirname2(referencePath), newTarget);
    if (!relativeTarget.startsWith("."))
      relativeTarget = `./${relativeTarget}`;
    const indentation = token.rewriteData?.indentation ?? "";
    const declaration = token.rewriteData?.declaration ?? token.value;
    return `#[path = ${JSON.stringify(relativeTarget)}]
${indentation}${declaration}`;
  }
  if (token.rewriteKind === "python-entrypoint") {
    const targetValue = token.targetValues?.[0] ?? token.value;
    return `${rewriteReferenceValue(referencePath, targetValue, oldTarget, newTarget, sourceReferencePath)}${token.rewriteData?.suffix ?? ""}`;
  }
  if (token.rewriteKind === "artifact-uri") {
    const artifactRoot = token.rewriteData?.artifactRoot;
    if (!artifactRoot || !newTarget.startsWith(`${artifactRoot}/`))
      throw new Error(`Artifact URI target escapes its captured owner: ${newTarget}`);
    return `asset://${newTarget.slice(artifactRoot.length + 1)}`;
  }
  if (token.rewriteKind === "path-prefix")
    return `${token.rewriteData?.prefix ?? ""}${newTarget}${token.rewriteData?.suffix ?? ""}`;
  if (token.rewriteKind === "projection-prose" && token.value.startsWith("\uD83C\uDFC5\uFE0Fstandards/")) {
    const artifactRoot = token.rewriteData?.artifactRoot;
    if (!artifactRoot || !newTarget.startsWith(`${artifactRoot}/`))
      throw new Error(`Projection prose target escapes its captured owner: ${newTarget}`);
    return newTarget.slice(artifactRoot.length + 1);
  }
  return rewriteReferenceValue(referencePath, token.value, oldTarget, newTarget, sourceReferencePath);
}
function unsupportedReferenceTokens(content, adapter) {
  const rows = [];
  const patterns = [/"([^"\r\n]+)"|'([^'\r\n]+)'|`([^`\r\n]+)`/gu, /(?:^|[\s(=,:])((?:\.\.?\/|\/)?[^\s"'`()\],;]+\/[^\s"'`()\],;]+|[A-Za-z0-9_.@-]+\.[A-Za-z0-9.]+)(?=$|[\s),;\]])/gmu];
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      if (match.index === undefined)
        continue;
      const value = match[1] ?? match[2] ?? match[3];
      if (!value || /^\/\//u.test(value) || /^[\\/]+$/u.test(value) || /^(?:\/{2,}|\.{1,2}\/?|\*+)$/u.test(value) || !/[\\/]/u.test(value) && !/^\.{1,2}$/u.test(value) && !/\.[A-Za-z0-9][A-Za-z0-9.-]*$/u.test(value))
        continue;
      const start = match.index + match[0].indexOf(value);
      rows.push({ adapter, structuredLocation: lineLocation(content, start, "unsupported-path-syntax"), start, end: start + value.length, value });
    }
  }
  return rows;
}
function referenceAdapter(path) {
  const lower = path.toLocaleLowerCase("und");
  if (lower.endsWith(".feature"))
    return "gherkin";
  if (lower.endsWith(".rs"))
    return "rust";
  if (/\.(?:ts|tsx|js|jsx|mjs|cjs|mts|cts)$/u.test(lower))
    return "typescript";
  if (/\.(?:go|mod|work)$/u.test(lower))
    return "go";
  if (lower.endsWith(".py"))
    return "python";
  if (/\.(?:cs|fs|vb|csproj|fsproj|vbproj|sln|props|targets)$/u.test(lower))
    return "dotnet";
  if (/\.(?:c|cc|cpp|cxx|h|hh|hpp|hxx|cmake)$/u.test(lower) || basename2(path) === "CMakeLists.txt")
    return "native";
  if (lower.endsWith(".jsonc"))
    return "jsonc";
  if (lower.endsWith(".json"))
    return "json";
  if (lower.endsWith(".toml"))
    return "toml";
  if (/\.ya?ml$/u.test(lower))
    return "yaml";
  if (/\.(?:xml|html|htm)$/u.test(lower))
    return "xml";
  return "markdown";
}
function applyEditsToContent(content, edits) {
  let result = content;
  const offset = (edit) => {
    const value = edit.structuredLocation.match(/@(\d+)$/)?.[1];
    if (value === undefined)
      throw new Error(`Reference edit lacks a structured offset at ${edit.path}:${edit.structuredLocation}`);
    return Number.parseInt(value, 10);
  };
  const sorted = [...edits].sort((a, b) => offset(b) - offset(a) || b.structuredLocation.localeCompare(a.structuredLocation));
  for (const edit of sorted) {
    const start = offset(edit);
    const end = start + edit.oldValue.length;
    if (result.slice(start, end) !== edit.oldValue)
      throw new Error(`Reference edit preimage mismatch at ${edit.path}:${edit.structuredLocation}`);
    result = `${result.slice(0, start)}${edit.newValue}${result.slice(end)}`;
  }
  return result;
}
function referenceGraph(repoRoot, entries, taxonomy, progress, cancelFile) {
  const known = referencePathIndex(entries.keys());
  const files = [...entries.values()].filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && (entry.size ?? 0) <= 16 * 1024 * 1024);
  for (let index = 0;index < files.length; index++) {
    checkCancellation(repoRoot, cancelFile);
    const entry = files[index];
    if (isExcluded(entry.sourcePath, taxonomy))
      continue;
    let content;
    try {
      content = readFileSync2(absolutePath(repoRoot, entry.sourcePath), "utf8");
    } catch {
      continue;
    }
    for (const token of referenceTokens(entry.sourcePath, content)) {
      const target = resolveReferenceTokenPath(entry.sourcePath, token, known);
      if (!target || !entries.has(target)) {
        const opaque = lexicalOpaqueReferenceTarget(entry.sourcePath, token, taxonomy);
        if (opaque)
          entry.violations.push(violation("opaque-reference-target", entry.sourcePath, `${token.adapter} ${token.structuredLocation} lexically targets excluded ${opaque}`, "warning"));
        continue;
      }
      entry.referencesOut.push(target);
      entries.get(target)?.referencesIn.push(entry.sourcePath);
    }
    report(progress, "inventory", "references", index + 1, files.length, entry.sourcePath);
  }
  for (const entry of entries.values()) {
    entry.referencesIn = [...new Set(entry.referencesIn)].sort();
    entry.referencesOut = [...new Set(entry.referencesOut)].sort();
  }
}
function referenceEditIdentity(edit) {
  return `${edit.path}\x00${edit.structuredLocation}\x00${edit.oldValue}\x00${edit.newValue}`;
}
function artifactReferenceProjections(inventory, moves, taxonomy) {
  const rows = [];
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const catalog = taxonomy.schema.semanticPathProjectionCatalogContracts[contract.catalogContractId];
    if (!("contractKind" in catalog))
      continue;
    for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "directory")) {
      const location = artifactProjectionSourceLocation(entry.sourcePath, contract, taxonomy);
      if (!location || location.sourceRoot !== entry.sourcePath)
        continue;
      const mappings = moves.filter((move) => move.rationaleRule === contract.rationaleRule && move.sourcePath.startsWith(`${location.sourceRoot}/`)).map(({ sourcePath, destinationPath }) => ({ sourcePath, destinationPath }));
      if (mappings.length === 0)
        continue;
      const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot }, taxonomy.schema);
      const authority = semanticPathProjectionAuthority({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot, nodes: artifactProjectionAuthorityNodes(inventory.repoRoot, location.sourceRoot, entries, taxonomy) }, taxonomy.schema);
      const orderedMappings = mappings.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
      const mappingProblems = canonicalJson(orderedMappings) === canonicalJson(authority.mappings) ? [] : ["Planned artifact mappings do not equal the schema projection authority"];
      rows.push({ id, artifactRoot: location.artifactRoot, sourceRoot: location.sourceRoot, destinationRoot: rendered.destinationRoot, rationaleRule: contract.rationaleRule, catalog, mappings: orderedMappings, authorityReferenceEdits: authority.referenceEdits, authorityProblems: [...rendered.problems, ...authority.problems, ...mappingProblems] });
    }
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourceRoot, right.sourceRoot) || left.id.localeCompare(right.id));
}
function artifactProjectionTail(path) {
  const marker = "\uD83D\uDDFF\uFE0Fartifacts/";
  const index = path.indexOf(marker);
  return index < 0 ? path : path.slice(index);
}
function artifactReferenceForm(token) {
  if (token.rewriteKind === "artifact-catalog-glob")
    return "artifact-catalog-glob";
  if (token.rewriteKind !== "artifact-catalog-prose")
    return null;
  const form = token.rewriteData?.form;
  return form === "root-marker" || form === "relative-root" || form === "interaction-glob" || form === "catalog-grammar" ? `artifact-catalog-prose:${form}` : null;
}
function registeredArtifactConsumers(context, referencePath, token, taxonomy) {
  const form = artifactReferenceForm(token);
  if (!form || !["rust", "typescript", "json", "toml"].includes(token.adapter))
    return [];
  return semanticPathProjectionReferenceConsumers(context.id, referencePath, token.adapter, form, taxonomy.schema).map((row) => row.id);
}
function catalogProjectionForToken(referencePath, token, contexts, taxonomy) {
  const selectors = token.rewriteData?.selectors ? JSON.parse(token.rewriteData.selectors) : [];
  const matches = contexts.filter((context) => {
    if (context.rationaleRule !== "artifact-example-model-catalog-projection-v1")
      return false;
    const authorized = referencePath === context.artifactRoot || referencePath.startsWith(`${context.artifactRoot}/`) || registeredArtifactConsumers(context, referencePath, token, taxonomy).length === 1;
    const selectorMatches = selectors.length === 0 || selectors.some((selector) => selector.includes(artifactProjectionTail(context.sourceRoot)));
    return authorized && selectorMatches;
  });
  if (matches.length === 1)
    return matches[0];
  if (matches.length > 1)
    return `Reference form matches multiple artifact projection owners: ${matches.map((row) => row.id).join(", ")}`;
  const cad = contexts.filter((context) => context.rationaleRule === "artifact-example-model-catalog-projection-v1");
  if (cad.length > 0)
    return selectors.length > 0 ? "Artifact selector or reference file does not match a registered source owner" : "Artifact catalog prose occurs outside an authorized owner or consumer location";
  return null;
}
function renderCatalogGlob(referencePath, token, context) {
  if (context.catalog.contractKind !== "distributed-json-manifest-catalog")
    return { problem: `${context.id} has no distributed catalog grammar` };
  const selectors = JSON.parse(token.rewriteData?.selectors ?? "[]");
  if (selectors.length === 0 || selectors.some((selector) => typeof selector !== "string"))
    return { problem: "Artifact catalog glob has no exact literal selectors" };
  const sourceTail = artifactProjectionTail(context.sourceRoot);
  const baseRelative = posix.relative(dirname2(referencePath), context.destinationRoot);
  const base = baseRelative.startsWith(".") ? baseRelative : `./${baseRelative}`;
  const zeroSource = [/\/\*\*\/\uD83D\uDD23\uFE0Fextension\.json$/u, /\/\*\*\/\uD83C\uDFF7\uFE0Fproperties\/\*\.json$/u, /\/\*\*\/\uD83D\uDD27\uFE0Fproperties\/\*\.json$/u];
  const rendered = [];
  for (const selector of selectors) {
    const tailIndex = selector.indexOf(sourceTail);
    if (tailIndex < 0)
      return { problem: `Artifact selector does not contain its registered source owner: ${selector}` };
    const suffix = selector.slice(tailIndex + sourceTail.length);
    const sourcePattern = `${context.sourceRoot}${suffix}`;
    const admitted = context.mappings.filter((mapping) => taxonomyPathPatternMatches(mapping.sourcePath, sourcePattern));
    if (admitted.length === 0) {
      if (zeroSource.some((pattern) => pattern.test(suffix)))
        continue;
      return { problem: `Nonempty artifact selector has no exact authority mapping: ${selector}` };
    }
    if (suffix.endsWith(`/${context.catalog.modelManifestSourceFilename}`)) {
      rendered.push(`${base}/*/\uD83D\uDD23\uFE0F.json`);
      continue;
    }
    const rules = context.catalog.categoryRules.filter((rule) => suffix.includes(`/${rule.sourceDirectoryName}/`));
    if (rules.length !== 1)
      return { problem: `Artifact selector has no unique registered category: ${selector}` };
    rendered.push(`${base}/**/${rules[0].sourceDirectoryName}/${rules[0].sourceShape === "nested-fixed-json" ? "**" : "*"}/\uD83D\uDD23\uFE0F.json`);
  }
  if (rendered.length === 0) {
    if (!/^\s*[A-Za-z_$][\w$]*\s*:\s*import\.meta\.glob\([^\r\n]+\),?\s*(?:as\s+[^;]+)?;?\s*(?:\r?\n)?$/u.test(token.value))
      return { problem: "Zero-source artifact selectors cannot be removed without owning their complete object member" };
    return "";
  }
  if (rendered.length !== selectors.length)
    return { problem: "Artifact selector list mixes registered and zero-source selectors" };
  let result = token.value;
  for (let index = 0;index < selectors.length; index++) {
    const quoted = JSON.stringify(selectors[index]);
    if (!result.includes(quoted))
      return { problem: "Artifact selector raw literal is not canonical JSON-compatible syntax" };
    result = result.replace(quoted, JSON.stringify(rendered[index]));
  }
  return result;
}
function artifactStructuralReferenceRewrite(referencePath, token, contexts, taxonomy) {
  if (token.rewriteKind !== "artifact-catalog-glob" && token.rewriteKind !== "artifact-catalog-prose")
    return null;
  const selected = catalogProjectionForToken(referencePath, token, contexts, taxonomy);
  if (typeof selected === "string")
    return { problem: selected };
  if (!selected)
    return null;
  if (token.rewriteKind === "artifact-catalog-glob") {
    const rendered = renderCatalogGlob(referencePath, token, selected);
    return typeof rendered === "string" ? { newValue: rendered } : rendered;
  }
  if (selected.catalog.contractKind !== "distributed-json-manifest-catalog")
    return { problem: `${selected.id} cannot render catalog prose` };
  const root = posix.relative(selected.artifactRoot, selected.destinationRoot);
  if (token.rewriteData?.form === "root-marker")
    return { newValue: `${root}/` };
  if (token.rewriteData?.form === "relative-root") {
    const value = posix.relative(dirname2(referencePath), selected.destinationRoot);
    return { newValue: value.startsWith(".") ? value : `./${value}` };
  }
  if (token.rewriteData?.form === "interaction-glob")
    return { newValue: `${root}/*/\uD83C\uDFAC\uFE0Finteractions/*/\uD83D\uDD23\uFE0F.json` };
  if (token.rewriteData?.form === "catalog-grammar") {
    const members = selected.catalog.categoryRules.map((rule) => `${rule.sourceDirectoryName}/<member>/\uD83D\uDD23\uFE0F.json`).sort(generatorPathCompare);
    return { newValue: `${root}/<model>/{${members.join(",")},\uD83D\uDD23\uFE0F.json}` };
  }
  return { problem: `Unknown artifact catalog prose form ${token.rewriteData?.form ?? ""}` };
}
function buildReferenceEdits(inventory, moves, taxonomy, options, known) {
  const moveBySource = new Map(moves.map((move) => [move.sourcePath, move]));
  const destinationBySource = new Map(inventory.entries.filter((entry) => entry.sourcePath !== entry.normalizedPath && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).map((entry) => [entry.sourcePath, entry.normalizedPath]));
  const edits = [];
  const editTargets = new Map;
  const unresolved = [];
  const resultHashes = new Map;
  const resultSizes = new Map;
  const accountedIncoming = new Set;
  const artifactContexts = artifactReferenceProjections(inventory, moves, taxonomy);
  for (const context of artifactContexts)
    for (const problem of context.authorityProblems)
      unresolved.push(violation("projection-reference-authority-invalid", context.sourceRoot, `${context.id}: ${problem}`));
  const activeProjectionKeys = new Set;
  const activeProjectionProfiles = new Set;
  for (const move of moves.filter((entry) => entry.rationaleRule === "artifact-mutation-test-projection-v1")) {
    const structural = mutationStructuralPaths(move.sourcePath)[0];
    const artifactRoot = artifactRootForPath(move.sourcePath);
    if (!structural || !artifactRoot)
      continue;
    activeProjectionKeys.add(projectionKey(artifactRoot, structural.standard, structural.subset));
    activeProjectionProfiles.add(`${structural.standard}\x00${structural.subset}`);
  }
  const candidates = inventory.entries.filter((entry) => entry.nodeKind === "file" && textualPath(entry.sourcePath) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0);
  for (let index = 0;index < candidates.length; index++) {
    checkCancellation(inventory.repoRoot, options.cancelFile);
    const entry = candidates[index];
    if (isExcluded(entry.sourcePath, taxonomy))
      continue;
    let content;
    try {
      content = readFileSync2(absolutePath(inventory.repoRoot, entry.sourcePath), "utf8");
    } catch (error) {
      unresolved.push(violation("reference-preimage-unreadable", entry.sourcePath, error instanceof Error ? error.message : String(error)));
      continue;
    }
    const finalReferencePath = moveBySource.get(entry.sourcePath)?.destinationPath ?? entry.normalizedPath;
    const fileEdits = [];
    const fileTargets = new Map;
    const tokens = [...new Map(referenceTokens(entry.sourcePath, content).map((token) => [`${token.start}\x00${token.end}\x00${token.value}\x00${(token.targetValues ?? []).join("\x00")}`, token])).values()].sort((left, right) => left.start - right.start || left.end - right.end || left.structuredLocation.localeCompare(right.structuredLocation));
    const supported = tokens.map((token) => ({ token, target: resolveReferenceTokenPath(entry.sourcePath, token, known) }));
    for (const { token, target: oldTarget } of supported) {
      const destination = oldTarget ? destinationBySource.get(oldTarget) : undefined;
      const artifactRewrite = artifactStructuralReferenceRewrite(finalReferencePath, token, artifactContexts, taxonomy);
      if (artifactRewrite?.problem) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${artifactRewrite.problem}`));
        continue;
      }
      const projectionProfile = token.rewriteData?.projectionProfile;
      const projectionActive = activeProjectionKeys.has(token.rewriteData?.projectionKey ?? "") || projectionProfile === "*" && activeProjectionProfiles.size > 0 || projectionProfile !== undefined && activeProjectionProfiles.has(projectionProfile);
      const artifactProjectionActive = artifactContexts.length > 0 && (token.rewriteKind === "path-prefix" || token.rewriteKind === "artifact-catalog-glob" || token.rewriteKind === "artifact-catalog-prose");
      if (token.unsupportedReason && (projectionActive || artifactProjectionActive)) {
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${token.structuredLocation}: ${token.unsupportedReason}`));
        continue;
      }
      if ((!oldTarget || !destination) && !(projectionActive && token.rewriteData?.newValue) && artifactRewrite?.newValue === undefined)
        continue;
      const newValue = artifactRewrite?.newValue ?? (oldTarget && destination ? rewriteReferenceToken(finalReferencePath, entry.sourcePath, token, oldTarget, destination) : token.rewriteData.newValue);
      if (newValue === token.value)
        continue;
      if (oldTarget)
        accountedIncoming.add(`${oldTarget}\x00${entry.sourcePath}`);
      const edit = {
        path: finalReferencePath,
        adapter: token.adapter,
        structuredLocation: token.structuredLocation,
        oldValue: token.value,
        newValue,
        preimage: { nodeKind: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size }
      };
      fileEdits.push(edit);
      if (oldTarget)
        fileTargets.set(referenceEditIdentity(edit), oldTarget);
    }
    for (const candidate of unsupportedReferenceTokens(content, referenceAdapter(entry.sourcePath))) {
      const oldTarget = resolveReferenceTokenPath(entry.sourcePath, candidate, known);
      if (!oldTarget || !destinationBySource.has(oldTarget))
        continue;
      const covered = supported.some(({ token, target }) => target === oldTarget && token.start <= candidate.start && token.end >= candidate.end);
      const destination = destinationBySource.get(oldTarget);
      const unchanged = rewriteReferenceValue(finalReferencePath, candidate.value, oldTarget, destination, entry.sourcePath) === candidate.value;
      if (!covered && !unchanged)
        unresolved.push(violation("reference-syntax-unsupported", entry.sourcePath, `${candidate.adapter} ${candidate.structuredLocation} contains unsupported path-bearing token ${JSON.stringify(candidate.value)} targeting ${oldTarget}`));
    }
    if (fileEdits.length > 0) {
      const deduplicated = [...new Map(fileEdits.map((edit) => [`${edit.structuredLocation}:${edit.newValue}`, edit])).values()].sort(referenceEditCompare);
      edits.push(...deduplicated);
      for (const edit of deduplicated) {
        const target = fileTargets.get(referenceEditIdentity(edit));
        if (target)
          editTargets.set(referenceEditIdentity(edit), target);
      }
      const rendered = applyEditsToContent(content, deduplicated);
      resultHashes.set(finalReferencePath, sha256(rendered));
      resultSizes.set(finalReferencePath, Buffer.byteLength(rendered));
    }
    report(options.progress, "plan", "references", index + 1, candidates.length, entry.sourcePath);
  }
  for (const move of moves) {
    const entry = inventory.entries.find((candidate) => candidate.sourcePath === move.sourcePath);
    if (entry?.fileKind) {
      const role = taxonomy.schema.fileKinds[entry.fileKind]?.role;
      const unaccounted = entry.referencesIn.filter((source) => !accountedIncoming.has(`${entry.sourcePath}\x00${source}`));
      if (role === "binary" && unaccounted.length > 0)
        unresolved.push(violation("opaque-reference-rewrite-unresolved", entry.sourcePath, `Binary target has unsupported incoming references from ${unaccounted.join(", ")}`));
      if (role === "generated" && entry.referencesIn.length > 0)
        unresolved.push(violation("generated-reference-rewrite-unresolved", entry.sourcePath, "Generated target requires an explicit regeneration contract before its incoming references can move"));
    }
  }
  const semanticLocationMatches = (actual, expected) => actual === expected || actual.startsWith(`${expected}:`) || actual.startsWith(`${expected}@`);
  for (const context of artifactContexts) {
    const requirements = context.authorityReferenceEdits;
    const concrete = edits.filter((edit) => requirements.some((required) => edit.path === required.path && edit.adapter === required.adapter && semanticLocationMatches(edit.structuredLocation, required.structuredLocation)));
    for (const required of requirements) {
      const matches = concrete.filter((edit) => edit.path === required.path && edit.adapter === required.adapter && semanticLocationMatches(edit.structuredLocation, required.structuredLocation) && edit.oldValue === required.oldValue && edit.newValue === required.newValue && edit.preimage.contentHash === required.preimageHash);
      if (matches.length !== 1)
        unresolved.push(violation("projection-reference-authority-invalid", required.path, `${context.id} requires exactly one ${required.adapter} ${required.structuredLocation} edit with its declared values and preimage; found ${matches.length}`));
    }
    if (concrete.length !== requirements.length)
      unresolved.push(violation("projection-reference-authority-invalid", context.sourceRoot, `${context.id} declares ${requirements.length} configuration reference edits but planning produced ${concrete.length}`));
  }
  return { edits: edits.sort(referenceEditCompare), editTargets, resultHashes, resultSizes, unresolved: stableViolations(unresolved) };
}
function referenceEditCompare(a, b) {
  return a.path.localeCompare(b.path) || a.structuredLocation.localeCompare(b.structuredLocation) || a.oldValue.localeCompare(b.oldValue) || a.newValue.localeCompare(b.newValue);
}
function taxonomyPlatformPathViolationCodes(path, maxPathBytes = 240) {
  const rows = [];
  if (Buffer.byteLength(path, "utf8") > maxPathBytes)
    rows.push("path-too-long");
  for (const segment of path.replaceAll("\\", "/").split("/")) {
    if (WINDOWS_RESERVED.test(segment))
      rows.push("windows-reserved-name");
    if (/[. ]$/u.test(segment))
      rows.push("trailing-dot-or-space");
  }
  return [...new Set(rows)];
}
function pathPolicyViolations(path, taxonomy) {
  const rows = [];
  if (Buffer.byteLength(path, "utf8") > taxonomy.schema.collisionPolicy.maxPathBytes)
    rows.push(violation("path-too-long", path, `Path exceeds ${taxonomy.schema.collisionPolicy.maxPathBytes} UTF-8 bytes`));
  for (const segment of path.split("/")) {
    if (taxonomy.schema.collisionPolicy.rejectWindowsReservedNames && WINDOWS_RESERVED.test(segment))
      rows.push(violation("windows-reserved-name", path, `Path segment is Windows-reserved: ${segment}`));
    if (taxonomy.schema.collisionPolicy.rejectTrailingDotsAndSpaces && /[. ]$/.test(segment))
      rows.push(violation("trailing-dot-or-space", path, `Path segment ends with a dot or space: ${segment}`));
  }
  return rows;
}
function sourceTreeDigest(entries) {
  return sha256(canonicalJson(entries.map((entry) => ({ sourcePath: entry.sourcePath, nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, symlinkTarget: entry.symlinkTarget }))));
}
function inventoryWithoutTransactionEvidence(inventory, transactionRoot, exactPlanArtifactPath) {
  const suppressed = (path) => path === transactionRoot || path.startsWith(`${transactionRoot}/`) || path === exactPlanArtifactPath;
  const entries = inventory.entries.filter((entry) => !suppressed(entry.sourcePath)).map((entry) => ({ ...entry, referencesIn: entry.referencesIn.filter((path) => !suppressed(path)), referencesOut: entry.referencesOut.filter((path) => !suppressed(path)) }));
  return { ...inventory, entries, violations: inventory.violations.filter((entry) => !suppressed(entry.path)), sourceTreeDigest: sourceTreeDigest(entries) };
}
function ancestorDirectoryKindIds(path, kinds) {
  const rows = [];
  let current = dirname2(path);
  while (current && current !== ".") {
    const kindId = kinds.get(current);
    if (kindId)
      rows.push(kindId);
    current = dirname2(current);
  }
  return rows;
}
function projectionDirectorySlug(name, kindId, taxonomy) {
  const kind = taxonomy.schema.semanticDirectoryKinds[kindId];
  if (!kind)
    return null;
  const leading = splitLeadingEmoji(name.normalize("NFC"));
  if (emojiFold(leading.emoji) !== emojiFold(kind.emoji) || !new RegExp(kind.slugPattern, "u").test(leading.rest))
    return null;
  return leading.rest;
}
function projectionSourceAt(path, scope, entries, kinds, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const contract = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const segments = path.split("/");
  if (segments.length <= contract.sourceSegments.length)
    return null;
  const start = segments.length - contract.sourceSegments.length;
  const artifactRoot = segments.slice(0, start).join("/");
  const ownerRegistry = taxonomy.schema.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
  const ownerMatches = ownerRegistry.memberNames.filter((name) => emojiFold(name) === emojiFold(basename2(artifactRoot)));
  if (ownerMatches.length !== 1 && !(scope && (artifactRoot === scope || artifactRoot.startsWith(`${scope}/`))))
    return null;
  const captures = new Map;
  for (let index = 0;index < contract.sourceSegments.length; index++) {
    const segment = contract.sourceSegments[index];
    const currentPath = segments.slice(0, start + index + 1).join("/");
    const current = entries.get(currentPath);
    if (!current || current.nodeKind !== "directory")
      return null;
    const canonicalName = basename2(current.normalizedPath);
    if ("literal" in segment) {
      if (canonicalName !== segment.literal || kinds.get(currentPath) !== segment.kindId)
        return null;
      continue;
    }
    if ("projectedMemberKindId" in segment) {
      if (segment.projectedMemberKindId !== ids.projectedMemberKindId)
        return null;
      const sourceName2 = basename2(current.sourcePath).normalize("NFC");
      const slug2 = splitLeadingEmoji(sourceName2).rest;
      if (!slug2)
        return null;
      captures.set(segment.capture, slug2);
      continue;
    }
    const sourceName = basename2(current.sourcePath).normalize("NFC");
    const contextualUnprefixed = segment.capture === "scenarioId" && !splitLeadingEmoji(sourceName).emoji && new RegExp(taxonomy.schema.semanticDirectoryKinds[segment.kindId].slugPattern, "u").test(sourceName);
    if (kinds.get(currentPath) !== segment.kindId && !contextualUnprefixed)
      return null;
    const slug = contextualUnprefixed ? sourceName : projectionDirectorySlug(canonicalName, segment.kindId, taxonomy);
    if (!slug)
      return null;
    captures.set(segment.capture, slug);
  }
  const standardVersion = captures.get("standardVersion");
  const subsetId = captures.get("subsetId");
  const mutationId = captures.get("mutationId");
  const sourceScenarioId = captures.get("scenarioId");
  if (!standardVersion || !subsetId || !mutationId || !sourceScenarioId)
    return null;
  const source = contract.sourceSegments.map((_segment, index) => segments.slice(0, start + index + 1).join("/"));
  return {
    artifactRoot,
    artifactId: splitLeadingEmoji(basename2(artifactRoot)).rest || basename2(artifactRoot),
    standardVersion,
    standardDirectoryName: basename2(source[1]),
    subsetId,
    subsetDirectoryName: basename2(source[3]),
    mutationId,
    mutationDirectoryName: basename2(source[6]),
    sourceScenarioId,
    sourceScenarioDirectoryName: basename2(source[8]),
    subsetRoot: source[3],
    mutationRoot: source[6],
    scenarioRoot: source[8]
  };
}
function projectionCatalogVectors(path, source) {
  let root;
  try {
    root = record(JSON.parse(readFileSync2(path, "utf8")), "mutation projection catalog");
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  if (!Array.isArray(root.mutationCatalogs))
    return { vectors: [], error: "mutationCatalogs must be an array" };
  const vectors = [];
  const seenSource = new Set;
  const seenCanonical = new Set;
  try {
    for (let catalogIndex = 0;catalogIndex < root.mutationCatalogs.length; catalogIndex++) {
      const catalog = record(root.mutationCatalogs[catalogIndex], `mutationCatalogs[${catalogIndex}]`);
      requiredString(catalog.id, `mutationCatalogs[${catalogIndex}].id`);
      requiredString(catalog.capability, `mutationCatalogs[${catalogIndex}].capability`);
      if (requiredString(catalog.standardDirectoryName, `mutationCatalogs[${catalogIndex}].standardDirectoryName`) !== source.standardDirectoryName || requiredString(catalog.subsetDirectoryName, `mutationCatalogs[${catalogIndex}].subsetDirectoryName`) !== source.subsetDirectoryName)
        throw new Error(`mutationCatalogs[${catalogIndex}] owner identity does not match its physical standard/subset`);
      stringArray(catalog.kinds, `mutationCatalogs[${catalogIndex}].kinds`);
      if (!Array.isArray(catalog.vectors))
        throw new Error(`mutationCatalogs[${catalogIndex}].vectors must be an array`);
      for (let vectorIndex = 0;vectorIndex < catalog.vectors.length; vectorIndex++) {
        const vector = record(catalog.vectors[vectorIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}]`);
        const mutationId = requiredString(vector.mutationId, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationId`);
        const sourceMutationDirectoryName = requiredString(vector.sourceMutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName`);
        if (sourceMutationDirectoryName !== sourceMutationDirectoryName.normalize("NFC") || sourceMutationDirectoryName.includes("/"))
          throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].sourceMutationDirectoryName is not one exact NFC basename`);
        const mutationDirectoryName = requiredString(vector.mutationDirectoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].mutationDirectoryName`).normalize("NFC");
        if (!Array.isArray(vector.scenarios))
          throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}] has an invalid physical mutation identity`);
        for (let scenarioIndex = 0;scenarioIndex < vector.scenarios.length; scenarioIndex++) {
          const scenario = record(vector.scenarios[scenarioIndex], `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}]`);
          const scenarioId = requiredString(scenario.id, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].id`);
          const scenarioDirectoryName = requiredString(scenario.directoryName, `mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}].directoryName`).normalize("NFC");
          if (splitLeadingEmoji(scenarioDirectoryName).rest !== scenarioId)
            throw new Error(`mutationCatalogs[${catalogIndex}].vectors[${vectorIndex}].scenarios[${scenarioIndex}] has an invalid physical scenario identity`);
          const sourceKey = `${mutationId}\x00${sourceMutationDirectoryName}\x00${scenarioId}`;
          const canonicalKey = `${mutationId}\x00${mutationDirectoryName}\x00${scenarioId}`;
          if (seenSource.has(sourceKey) || seenCanonical.has(canonicalKey))
            throw new Error(`Duplicate physical vector identity ${sourceKey.replaceAll("\x00", "/")}`);
          seenSource.add(sourceKey);
          seenCanonical.add(canonicalKey);
          vectors.push({ mutationId, sourceMutationDirectoryName, mutationDirectoryName, scenarioId, scenarioDirectoryName });
        }
      }
    }
  } catch (error) {
    return { vectors: [], error: error instanceof Error ? error.message : String(error) };
  }
  return { vectors: vectors.sort((left, right) => left.sourceMutationDirectoryName.localeCompare(right.sourceMutationDirectoryName) || left.scenarioDirectoryName.localeCompare(right.scenarioDirectoryName)) };
}
function projectionCatalogEntryForSubset(entries, subsetRoot) {
  const oracleRoot = `${subsetRoot}/\uD83E\uDDEA\uFE0Foracle`;
  const candidates = [...entries.values()].filter((entry) => entry.nodeKind === "file" && entry.fileKind === "json" && dirname2(entry.sourcePath) === oracleRoot && basename2(entry.normalizedPath) === "\uD83D\uDD23\uFE0F.json");
  return candidates.length === 1 ? candidates[0] : null;
}
function mutationDescendantContract(taxonomy) {
  const contract = taxonomy.schema.semanticDescendantContracts[taxonomy.schema.mutationCatalogProjection.descendantContractId];
  if (!contract || "contractKind" in contract || [...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].some((node) => !("kindId" in node)))
    throw new Error("Mutation projection must reference one physical-kind exact descendant contract");
  return contract;
}
function projectionDescendantPath(node, taxonomy) {
  const segments = node.pathSegments.map((segment) => segment.literal);
  if (node.nodeType === "file") {
    const kind = taxonomy.schema.fileKinds[node.kindId];
    if (!kind || kind.extensionChains.length !== 1)
      throw new Error(`Projection descendant kind ${node.kindId} is not a single physical leaf`);
    segments.push(`${kind.emoji}${kind.extensionChains[0]}`.normalize("NFC"));
  }
  return segments.join("/");
}
function canonicalProjectedMemberName(name, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const sourceKind = taxonomy.schema.semanticProjectedMemberKinds[ids.projectedMemberKindId].sourceMemberKindId;
  const matches = taxonomy.schema.semanticDirectoryMemberKinds[sourceKind].memberNames.filter((candidate) => emojiFold(candidate) === emojiFold(name.normalize("NFC")));
  return matches.length === 1 ? matches[0] : null;
}
function projectionBundleProblem(source, entries, kinds, contract, taxonomy) {
  const root = entries.get(source.scenarioRoot);
  if (!root)
    return "scenario root is absent";
  const actual = [...entries.values()].filter((entry) => entry.sourcePath === source.scenarioRoot || entry.sourcePath.startsWith(`${source.scenarioRoot}/`));
  if (actual.length !== contract.realizedNodeCount)
    return `bundle has ${actual.length} nodes, expected ${contract.realizedNodeCount}`;
  if (actual.some((entry) => entry.nodeKind === "symlink"))
    return "bundle contains a symlink";
  const byKey = new Map;
  for (const entry of actual) {
    const relativePath = entry.normalizedPath === root.normalizedPath ? "" : entry.normalizedPath.startsWith(`${root.normalizedPath}/`) ? entry.normalizedPath.slice(root.normalizedPath.length + 1) : null;
    if (relativePath === null)
      return `bundle node normalizes outside its scenario: ${entry.sourcePath}`;
    const key = `${entry.nodeKind}\x00${relativePath}`;
    if (byKey.has(key))
      return `bundle normalization duplicates ${relativePath}`;
    byKey.set(key, entry);
  }
  const matches = (node) => {
    const entry = byKey.get(`${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`);
    if (!entry)
      return false;
    return node.nodeType === "file" ? entry.fileKind === node.kindId : node.pathSegments.length === 0 && entry.sourcePath === source.scenarioRoot && node.kindId === contract.rootDirectoryKindId || kinds.get(entry.sourcePath) === node.kindId;
  };
  const missing = contract.requiredNodes.filter((node) => !matches(node));
  if (missing.length > 0)
    return `bundle is missing ${projectionDescendantPath(missing[0], taxonomy) || "scenario root"}`;
  for (const alternative of contract.exclusiveAlternatives)
    if (alternative.nodes.filter(matches).length !== 1)
      return `bundle must realize exactly one ${alternative.id} alternative`;
  const allowed = new Set([...contract.requiredNodes, ...contract.exclusiveAlternatives.flatMap((alternative) => alternative.nodes)].filter(matches).map((node) => `${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`));
  const extra = [...byKey.keys()].find((key) => !allowed.has(key));
  return extra ? `bundle contains unregistered node ${extra.slice(extra.indexOf("\x00") + 1)}` : null;
}
function setProjectedPath(entry, destination, taxonomy) {
  entry.normalizedPath = destination.normalize("NFC");
  const superseded = new Set(["directory-kind-ambiguous", "directory-kind-unresolved", "file-kind-ambiguous", "file-kind-unresolved", "semantic-stem-ambiguous", "semantic-stem-unresolved", "path-too-long", "windows-reserved-name", "trailing-dot-or-space"]);
  entry.violations = [...entry.violations.filter((row) => !superseded.has(row.code)), ...pathPolicyViolations(entry.normalizedPath, taxonomy)];
}
function mutationProjectionRationale(sourcePath, destinationPath, taxonomy) {
  const structural = mutationStructuralPaths(sourcePath)[0];
  const artifactRoot = artifactRootForPath(sourcePath);
  if (!artifactRoot)
    return null;
  const relativeDestination = destinationPath.startsWith(`${artifactRoot}/`) ? destinationPath.slice(artifactRoot.length + 1).split("/") : [];
  if (structural && relativeDestination[0] === "\uD83E\uDDEA\uFE0Ftests" && relativeDestination[1] === `\uD83E\uDE86\uFE0F${structural.standard}-${structural.subset}` && canonicalProjectedMemberName(relativeDestination[2] ?? "", taxonomy) === relativeDestination[2] && emojiFold(splitLeadingEmoji(relativeDestination[3] ?? "").emoji) === emojiFold(taxonomy.schema.semanticDirectoryKinds[taxonomy.schema.semanticDescendantContracts[taxonomy.schema.mutationCatalogProjection.descendantContractId].rootDirectoryKindId].emoji))
    return "artifact-mutation-test-projection-v1";
  const relativeSource = sourcePath.slice(artifactRoot.length + 1).split("/");
  const prefix = ["\uD83C\uDFC5\uFE0Fstandards", relativeSource[1], "\uD83E\uDE86\uFE0Fsubsets", relativeSource[3], "\uD83E\uDDEC\uFE0Fschema", "\uD83E\uDDEC\uFE0Fmutations"];
  if (relativeSource.length > 7 && prefix.every((segment, index) => relativeSource[index] === segment) && prefix.every((segment, index) => relativeDestination[index] === segment) && relativeSource[6] !== relativeDestination[6] && canonicalProjectedMemberName(relativeDestination[6] ?? "", taxonomy) === relativeDestination[6])
    return "artifact-mutation-source-canonicalization-v1";
  return null;
}
function projectMutationTestBundles(repoRoot, scope, entries, kinds, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const projection = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const descendant = mutationDescendantContract(taxonomy);
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const sources = [...entries.values()].filter((entry) => entry.nodeKind === "directory").map((entry) => projectionSourceAt(entry.sourcePath, scope, entries, kinds, taxonomy)).filter((entry) => entry !== null).sort((left, right) => left.scenarioRoot.localeCompare(right.scenarioRoot));
  const bySubset = new Map;
  for (const source of sources)
    bySubset.set(source.subsetRoot, [...bySubset.get(source.subsetRoot) ?? [], source]);
  const profileOwners = new Map;
  for (const source of sources) {
    const profile = renderer.template.replace("{standardVersion}", source.standardVersion).replace("{subsetId}", source.subsetId);
    const key = `${source.artifactRoot}\x00${emojiFold(profile).toLocaleLowerCase("und")}`;
    const owners = profileOwners.get(key) ?? new Set;
    owners.add(`${source.artifactId}\x00${source.standardVersion}\x00${source.subsetId}`);
    profileOwners.set(key, owners);
  }
  for (const [subsetRoot, subsetSources] of [...bySubset.entries()].sort(([left], [right]) => left.localeCompare(right))) {
    const catalogEntry = projectionCatalogEntryForSubset(entries, subsetRoot);
    const catalogPath = catalogEntry?.sourcePath ?? `${subsetRoot}/\uD83E\uDDEA\uFE0Foracle/\uD83D\uDD23\uFE0F.json`;
    const catalog = catalogEntry?.nodeKind === "file" ? projectionCatalogVectors(absolutePath(repoRoot, catalogPath), subsetSources[0]) : { vectors: [], error: `catalog is missing at ${catalogPath}` };
    if (catalog.error) {
      for (const source of subsetSources)
        entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-invalid", catalogPath, catalog.error));
      continue;
    }
    const vectorsByMutation = new Map;
    for (const vector of catalog.vectors) {
      const key = vector.sourceMutationDirectoryName;
      vectorsByMutation.set(key, [...vectorsByMutation.get(key) ?? [], vector]);
    }
    const sourcesByMutation = new Map;
    for (const source of subsetSources) {
      const key = source.mutationDirectoryName.normalize("NFC");
      sourcesByMutation.set(key, [...sourcesByMutation.get(key) ?? [], source]);
    }
    const consumed = new Set;
    const canonicalizedMutationRoots = new Set;
    for (const [mutationKey, mutationSources] of sourcesByMutation) {
      const vectors = vectorsByMutation.get(mutationKey) ?? [];
      const canonicalNames = [...new Set(vectors.map((vector) => canonicalProjectedMemberName(vector.mutationDirectoryName, taxonomy)).filter((name) => name !== null))];
      const mutationName = canonicalNames.length === 1 ? canonicalNames[0] : null;
      if (!mutationName || vectors.some((vector) => vector.sourceMutationDirectoryName !== mutationKey || vector.mutationDirectoryName !== mutationName)) {
        for (const source of mutationSources)
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-member-unresolved", source.mutationRoot, `Mutation member ${source.mutationDirectoryName} has no unique canonical registry identity`));
        continue;
      }
      const exact = new Map(vectors.map((vector) => [vector.scenarioDirectoryName, vector]));
      const assignments = new Map;
      for (const source of mutationSources) {
        const canonicalSourceName = `${taxonomy.schema.semanticDirectoryKinds[descendant.rootDirectoryKindId].emoji}${source.sourceScenarioId}`.normalize("NFC");
        const vector = exact.get(canonicalSourceName);
        if (vector)
          assignments.set(source, vector);
      }
      const unmatchedSources = mutationSources.filter((source) => !assignments.has(source));
      const matchedVectors = new Set(assignments.values());
      const unmatchedVectors = vectors.filter((vector) => !matchedVectors.has(vector));
      if (unmatchedSources.length === 1 && unmatchedVectors.length === 1)
        assignments.set(unmatchedSources[0], unmatchedVectors[0]);
      if (assignments.size !== mutationSources.length || assignments.size !== vectors.length) {
        for (const source of mutationSources)
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-coverage", source.scenarioRoot, `Physical mutation ${mutationName} does not have an exact one-to-one vector registry`));
        continue;
      }
      for (const [source, vector] of assignments) {
        const vectorKey = `${vector.mutationId}\x00${vector.sourceMutationDirectoryName}\x00${vector.scenarioId}`;
        if (consumed.has(vectorKey)) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-catalog-duplicate", source.scenarioRoot, `Vector ${vectorKey.replaceAll("\x00", "/")} owns more than one physical bundle`));
          continue;
        }
        const problem = projectionBundleProblem(source, entries, kinds, descendant, taxonomy);
        if (problem) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-bundle-invalid", source.scenarioRoot, problem));
          continue;
        }
        const profile = renderer.template.replace("{standardVersion}", source.standardVersion).replace("{subsetId}", source.subsetId).normalize("NFC");
        const profileKey = `${source.artifactRoot}\x00${emojiFold(profile).toLocaleLowerCase("und")}`;
        if ((profileOwners.get(profileKey)?.size ?? 0) !== 1) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-profile-collision", source.scenarioRoot, `Profile ${profile} is not a unique standard/subset rendering`));
          continue;
        }
        const destinationSegments = projection.destinationSegments.map((segment) => {
          if ("literal" in segment)
            return segment.literal;
          if ("render" in segment)
            return profile;
          if ("projectedMemberKindId" in segment)
            return mutationName;
          return vector.scenarioDirectoryName;
        });
        const destinationRoot = `${source.artifactRoot}/${destinationSegments.join("/")}`.normalize("NFC");
        if (Buffer.byteLength(destinationRoot, "utf8") + descendant.pathBudgetReserve.bytes > taxonomy.schema.collisionPolicy.maxPathBytes) {
          entries.get(source.scenarioRoot)?.violations.push(violation("projection-path-budget", source.scenarioRoot, `Projected scenario plus reserved descendant suffix exceeds ${taxonomy.schema.collisionPolicy.maxPathBytes} bytes`));
          continue;
        }
        consumed.add(vectorKey);
        const root = entries.get(source.scenarioRoot);
        root.violations = root.violations.filter((row) => row.code !== "directory-kind-unresolved");
        const initialRoot = root.normalizedPath;
        for (const entry of entries.values()) {
          if (entry.sourcePath !== source.scenarioRoot && !entry.sourcePath.startsWith(`${source.scenarioRoot}/`))
            continue;
          const suffix = entry.normalizedPath === initialRoot ? "" : entry.normalizedPath.slice(initialRoot.length + 1);
          setProjectedPath(entry, suffix ? `${destinationRoot}/${suffix}` : destinationRoot, taxonomy);
        }
        if (!canonicalizedMutationRoots.has(source.mutationRoot)) {
          const mutation = entries.get(source.mutationRoot);
          const testsRoot = dirname2(source.scenarioRoot);
          if (mutation) {
            const initialMutationRoot = mutation.normalizedPath;
            const canonicalMutationRoot = `${dirname2(source.mutationRoot)}/${mutationName}`.normalize("NFC");
            const mutationEntries = [...entries.values()].filter((entry) => entry.sourcePath === source.mutationRoot || entry.sourcePath.startsWith(`${source.mutationRoot}/`)).filter((entry) => entry.sourcePath !== testsRoot && !entry.sourcePath.startsWith(`${testsRoot}/`));
            mutation.violations = mutation.violations.filter((row) => row.code !== "directory-kind-unresolved");
            for (const entry of mutationEntries) {
              const suffix = entry.normalizedPath === initialMutationRoot ? "" : entry.normalizedPath.startsWith(`${initialMutationRoot}/`) ? entry.normalizedPath.slice(initialMutationRoot.length + 1) : entry.sourcePath.slice(source.mutationRoot.length + 1);
              setProjectedPath(entry, suffix ? `${canonicalMutationRoot}/${suffix}` : canonicalMutationRoot, taxonomy);
            }
          }
          canonicalizedMutationRoots.add(source.mutationRoot);
        }
      }
    }
    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\x00${vector.sourceMutationDirectoryName}\x00${vector.scenarioId}`;
      if (!consumed.has(key))
        catalogEntry?.violations.push(violation("projection-catalog-unrealized", catalogPath, `Registered vector ${key.replaceAll("\x00", "/")} has no physical bundle`));
    }
  }
}
function validateProjectedMutationTestBundles(repoRoot, scope, entries, kinds, taxonomy) {
  const ids = taxonomy.schema.mutationCatalogProjection;
  const projection = taxonomy.schema.semanticPathProjectionContracts[ids.projectionContractId];
  const descendant = mutationDescendantContract(taxonomy);
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const expected = new Set;
  const catalogs = [...entries.values()].filter((entry) => entry.nodeKind === "file" && entry.fileKind === "json" && basename2(dirname2(entry.sourcePath)) === "\uD83E\uDDEA\uFE0Foracle" && basename2(entry.normalizedPath) === "\uD83D\uDD23\uFE0F.json").sort((left, right) => left.sourcePath.localeCompare(right.sourcePath));
  for (const catalogEntry of catalogs) {
    const subsetRoot = dirname2(dirname2(catalogEntry.sourcePath));
    const segments = subsetRoot.split("/");
    if (segments.length < 5)
      continue;
    const artifactRoot = segments.slice(0, -4).join("/");
    const [standardsName, standardDirectoryName, subsetsName, subsetDirectoryName] = segments.slice(-4);
    if (standardsName !== "\uD83C\uDFC5\uFE0Fstandards" || subsetsName !== "\uD83E\uDE86\uFE0Fsubsets")
      continue;
    const ownerRegistry = taxonomy.schema.semanticDirectoryMemberKinds[projection.sourceOwnerKindId];
    const ownerMatches = ownerRegistry.memberNames.filter((name) => emojiFold(name) === emojiFold(basename2(artifactRoot)));
    if (ownerMatches.length !== 1 && !(scope && (artifactRoot === scope || artifactRoot.startsWith(`${scope}/`))))
      continue;
    const standardVersion = projectionDirectorySlug(standardDirectoryName, "standard", taxonomy);
    const subsetId = projectionDirectorySlug(subsetDirectoryName, "subset", taxonomy);
    if (!standardVersion || !subsetId)
      continue;
    const catalog = projectionCatalogVectors(absolutePath(repoRoot, catalogEntry.sourcePath), { standardDirectoryName, subsetDirectoryName });
    if (catalog.error) {
      catalogEntry.violations.push(violation("projection-catalog-invalid", catalogEntry.sourcePath, catalog.error));
      continue;
    }
    const profile = renderer.template.replace("{standardVersion}", standardVersion).replace("{subsetId}", subsetId).normalize("NFC");
    for (const vector of catalog.vectors) {
      const mutationDirectoryName = canonicalProjectedMemberName(vector.mutationDirectoryName, taxonomy);
      if (!mutationDirectoryName) {
        catalogEntry.violations.push(violation("projection-member-unresolved", catalogEntry.sourcePath, `Mutation member ${vector.mutationDirectoryName} has no unique canonical registry identity`));
        continue;
      }
      const mutationRoot = `${artifactRoot}/\uD83E\uDDEA\uFE0Ftests/${profile}/${mutationDirectoryName}`;
      const scenarioRoot = `${mutationRoot}/${vector.scenarioDirectoryName}`;
      expected.add(scenarioRoot);
      const root = entries.get(scenarioRoot);
      if (!root)
        continue;
      const source = { artifactRoot, artifactId: splitLeadingEmoji(basename2(artifactRoot)).rest || basename2(artifactRoot), standardVersion, standardDirectoryName, subsetId, subsetDirectoryName, mutationId: vector.mutationId, mutationDirectoryName, sourceScenarioId: vector.scenarioId, sourceScenarioDirectoryName: vector.scenarioDirectoryName, subsetRoot, mutationRoot, scenarioRoot };
      const problem = projectionBundleProblem(source, entries, kinds, descendant, taxonomy);
      if (problem) {
        root.violations.push(violation("projection-bundle-invalid", scenarioRoot, problem));
        continue;
      }
      root.violations = root.violations.filter((row) => row.code !== "directory-kind-unresolved");
      const initialRoot = root.normalizedPath;
      for (const entry of entries.values()) {
        if (entry.sourcePath !== scenarioRoot && !entry.sourcePath.startsWith(`${scenarioRoot}/`))
          continue;
        const suffix = entry.normalizedPath === initialRoot ? "" : entry.normalizedPath.slice(initialRoot.length + 1);
        setProjectedPath(entry, suffix ? `${scenarioRoot}/${suffix}` : scenarioRoot, taxonomy);
      }
      const mutation = entries.get(mutationRoot);
      if (mutation) {
        mutation.violations = mutation.violations.filter((row) => row.code !== "directory-kind-unresolved");
        setProjectedPath(mutation, mutationRoot, taxonomy);
      }
    }
  }
  for (const entry of entries.values()) {
    if (entry.nodeKind !== "directory" || expected.has(entry.sourcePath))
      continue;
    const segments = entry.sourcePath.split("/");
    if (segments.length < 4)
      continue;
    const profilePath = segments.slice(0, -2).join("/");
    const testsPath = dirname2(profilePath);
    if (kinds.get(profilePath) === renderer.directoryKindId && basename2(testsPath) === "\uD83E\uDDEA\uFE0Ftests")
      entry.violations.push(violation("projection-destination-unregistered", entry.sourcePath, "Projected scenario has no exact catalog vector identity"));
  }
}
function artifactProjectionContracts(taxonomy) {
  return Object.entries(taxonomy.schema.semanticPathProjectionContracts).filter((entry) => entry[1].sourceArtifactMemberName !== undefined && (entry[1].rationaleRule === "artifact-example-model-catalog-projection-v1" || entry[1].rationaleRule === "artifact-editor-command-projection-v1")).map(([id, contract]) => ({ id, contract })).sort((left, right) => left.id.localeCompare(right.id));
}
function artifactProjectionSourceLocation(path, contract, taxonomy) {
  const segments = path.split("/");
  for (let artifactIndex = 0;artifactIndex < segments.length; artifactIndex++) {
    if (segments[artifactIndex] !== contract.sourceArtifactMemberName)
      continue;
    const owner = taxonomy.schema.semanticDirectoryMemberKinds[contract.sourceOwnerKindId];
    if (!owner?.memberNames.includes(segments[artifactIndex]) || artifactIndex === 0 || segments[artifactIndex - 1] !== canonicalDirectoryName(taxonomy, "artifacts", "artifacts"))
      continue;
    const sourceNames = segments.slice(artifactIndex + 1, artifactIndex + 1 + contract.sourceSegments.length);
    if (sourceNames.length !== contract.sourceSegments.length)
      continue;
    const matches = contract.sourceSegments.every((segment, index) => ("literal" in segment) ? sourceNames[index] === segment.literal : sourceNames[index] !== "");
    if (!matches)
      continue;
    return { artifactRoot: segments.slice(0, artifactIndex + 1).join("/"), sourceRoot: segments.slice(0, artifactIndex + 1 + contract.sourceSegments.length).join("/") };
  }
  return null;
}
function artifactProjectionAuthorityNodes(repoRoot, sourceRoot, entries, taxonomy) {
  const nodes = [];
  for (const entry of [...entries.values()].filter((candidate) => candidate.sourcePath === sourceRoot || candidate.sourcePath.startsWith(`${sourceRoot}/`)).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath))) {
    if (isExcluded(entry.sourcePath, taxonomy))
      throw new Error(`Artifact projection crosses opaque path ${entry.sourcePath}`);
    if (entry.nodeKind !== "file") {
      nodes.push({ path: entry.sourcePath, nodeKind: entry.nodeKind });
      continue;
    }
    let content;
    try {
      content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync2(absolutePath(repoRoot, entry.sourcePath)));
    } catch {
      throw new Error(`Artifact projection source is not readable UTF-8: ${entry.sourcePath}`);
    }
    nodes.push({ path: entry.sourcePath, nodeKind: "file", content });
  }
  return nodes;
}
function commonProjectionDirectory(paths, floor) {
  if (paths.length === 0)
    return floor;
  const parts = paths.map((path) => path.split("/"));
  const common = [];
  for (let index = 0;index < Math.min(...parts.map((row) => row.length)); index++) {
    const segment = parts[0][index];
    if (!parts.every((row) => row[index] === segment))
      break;
    common.push(segment);
  }
  const result = common.join("/");
  return result === floor || result.startsWith(`${floor}/`) ? result : floor;
}
function artifactProjectionDirectoryMappings(sourceRoot, destinationRoot, mappings, entries) {
  const result = new Map([[sourceRoot, destinationRoot]]);
  const directories = [...entries.values()].filter((entry) => entry.nodeKind === "directory" && entry.sourcePath.startsWith(`${sourceRoot}/`)).sort((left, right) => left.sourcePath.split("/").length - right.sourcePath.split("/").length || generatorPathCompare(left.sourcePath, right.sourcePath));
  for (const entry of directories) {
    const descendants = mappings.filter((mapping) => mapping.sourcePath.startsWith(`${entry.sourcePath}/`)).map((mapping) => dirname2(mapping.destinationPath));
    if (descendants.length === 0)
      continue;
    const parentDestination = result.get(dirname2(entry.sourcePath)) ?? destinationRoot;
    const common = commonProjectionDirectory(descendants, parentDestination);
    const commonSegments = common.split("/");
    const parentSegments = parentDestination.split("/");
    let destination = common;
    for (let index = commonSegments.length - 1;index >= parentSegments.length; index--)
      if (commonSegments[index] === basename2(entry.sourcePath)) {
        destination = commonSegments.slice(0, index + 1).join("/");
        break;
      }
    result.set(entry.sourcePath, destination);
  }
  return result;
}
function applyArtifactProjectionPath(entry, destinationPath, taxonomy) {
  entry.normalizedPath = destinationPath.normalize("NFC");
  const superseded = new Set(["directory-kind-ambiguous", "directory-kind-unresolved", "file-kind-ambiguous", "file-kind-unresolved", "semantic-stem-ambiguous", "semantic-stem-unresolved", "path-too-long", "trailing-dot-or-space", "windows-reserved-name"]);
  entry.violations = [...entry.violations.filter((row) => !superseded.has(row.code)), ...pathPolicyViolations(entry.normalizedPath, taxonomy)];
}
function projectArtifactCatalogs(repoRoot, entries, taxonomy) {
  const projectedSources = new Map;
  const projectedDestinations = new Map;
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const candidates = [...entries.values()].filter((entry) => entry.nodeKind === "directory").map((entry) => ({ entry, location: artifactProjectionSourceLocation(entry.sourcePath, contract, taxonomy) })).filter((row) => row.location !== null && row.entry.sourcePath === row.location.sourceRoot).sort((left, right) => generatorPathCompare(left.entry.sourcePath, right.entry.sourcePath));
    for (const { entry: root, location } of candidates) {
      const nodes = artifactProjectionAuthorityNodes(repoRoot, location.sourceRoot, entries, taxonomy);
      const occupiedPaths = [...entries.keys()].filter((path) => path !== location.sourceRoot && !path.startsWith(`${location.sourceRoot}/`)).sort(generatorPathCompare);
      const authority = semanticPathProjectionAuthority({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot, nodes, occupiedPaths }, taxonomy.schema);
      if (authority.problems.length > 0) {
        root.violations.push(violation("projection-authority-invalid", root.sourcePath, `${id}: ${authority.problems.join(" | ")}`));
        continue;
      }
      const accepted = authority.mappings.map((mapping) => ({ sourcePath: normalizeRelative(mapping.sourcePath), destinationPath: normalizeRelative(mapping.destinationPath) }));
      for (const mapping of accepted) {
        const source = entries.get(mapping.sourcePath);
        const priorSource = projectedSources.get(mapping.sourcePath);
        const priorDestination = projectedDestinations.get(mapping.destinationPath);
        if (!source || source.nodeKind !== "file" || priorSource && priorSource !== mapping.destinationPath || priorDestination && priorDestination !== mapping.sourcePath) {
          root.violations.push(violation("projection-mapping-collision", mapping.sourcePath, `${id} does not own one unique admitted source/destination pair`));
          continue;
        }
        projectedSources.set(mapping.sourcePath, mapping.destinationPath);
        projectedDestinations.set(mapping.destinationPath, mapping.sourcePath);
        applyArtifactProjectionPath(source, mapping.destinationPath, taxonomy);
      }
      for (const [sourcePath, destinationPath] of artifactProjectionDirectoryMappings(location.sourceRoot, authority.destinationRoot, accepted, entries)) {
        const source = entries.get(sourcePath);
        if (source)
          applyArtifactProjectionPath(source, destinationPath, taxonomy);
      }
    }
  }
}
function artifactCatalogProjectionRationale(sourcePath, destinationPath, taxonomy) {
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const location = artifactProjectionSourceLocation(sourcePath, contract, taxonomy);
    if (!location)
      continue;
    const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot: location.sourceRoot }, taxonomy.schema);
    if (rendered.problems.length === 0 && (destinationPath === rendered.destinationRoot || destinationPath.startsWith(`${rendered.destinationRoot}/`)))
      return contract.rationaleRule;
  }
  return null;
}
function inventoryTaxonomy(options) {
  const repoRoot = resolve2(options.repoRoot);
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1))
    throw new Error("workers must be a positive integer");
  const taxonomy = loadTaxonomy2({ repoRoot, taxonomyPath: options.taxonomyPath });
  const scope = options.scope === undefined ? undefined : normalizeRelative(options.scope);
  if (scope && isExcluded(scope, taxonomy))
    throw new Error(`Inventory scope is opaque: ${scope}`);
  checkCancellation(repoRoot, options.cancelFile);
  const admitted = new Map;
  const trackedRows = gitRows(repoRoot, taxonomy);
  const activeExclusions = [];
  for (const row of trackedRows) {
    if (isExcluded(row.path, taxonomy) || !inScope(row.path, scope))
      continue;
    if (lstatOrNull(absolutePath(repoRoot, row.path)))
      admitted.set(row.path, row);
  }
  for (const path of untrackedGitPaths(repoRoot, taxonomy)) {
    if (isExcluded(path, taxonomy) || !inScope(path, scope) || admitted.has(path))
      continue;
    const row = worktreeCandidate(repoRoot, path);
    if (row)
      admitted.set(path, row);
  }
  for (const row of ignoredGeneratorRows(repoRoot, taxonomy)) {
    if (isExcluded(row.path, taxonomy) || !inScope(row.path, scope))
      continue;
    if (!admitted.has(row.path) || row.explicitDirectory)
      admitted.set(row.path, row);
  }
  for (const row of explicitTicketRows(repoRoot, options.ticketDir, taxonomy)) {
    if (isExcluded(row.path, taxonomy) || !inScope(row.path, scope))
      continue;
    if (!admitted.has(row.path) || row.explicitDirectory)
      admitted.set(row.path, row);
  }
  const directoryPaths = new Set;
  for (const row of admitted.values()) {
    if (row.explicitDirectory || row.mode === "040000")
      directoryPaths.add(row.path);
    let parent = dirname2(row.path);
    while (parent && parent !== ".") {
      if (inScope(parent, scope))
        directoryPaths.add(parent);
      parent = dirname2(parent);
    }
  }
  const entries = new Map;
  const canonicalDirectoryByPath = new Map;
  const directoryKindByPath = new Map;
  const fixedDirectoryContractByPath = new Map;
  const directories = [...directoryPaths].sort((a, b) => a.split("/").length - b.split("/").length || Buffer.from(a).compare(Buffer.from(b)));
  for (let index = 0;index < directories.length; index++) {
    checkCancellation(repoRoot, options.cancelFile);
    const path = directories[index];
    const parentCanonical = canonicalDirectoryByPath.get(dirname2(path)) ?? "";
    const canonical = canonicalDirectory(path, parentCanonical, directoryKindByPath.get(dirname2(path)), ancestorDirectoryKindIds(path, directoryKindByPath), taxonomy);
    canonicalDirectoryByPath.set(path, canonical.path);
    if (canonical.kindId)
      directoryKindByPath.set(path, canonical.kindId);
    if (canonical.fixedId)
      fixedDirectoryContractByPath.set(path, canonical.fixedId);
    entries.set(path, {
      sourcePath: path,
      normalizedPath: canonical.path,
      nodeKind: "directory",
      ownerId: ownerId(path),
      areaId: areaId(path),
      fileKind: null,
      semanticStem: splitLeadingEmoji(basename2(path)).rest || null,
      fixedContractId: canonical.fixedId,
      contentHash: "",
      referencesIn: [],
      referencesOut: [],
      violations: [...canonical.violations, ...pathPolicyViolations(canonical.path, taxonomy)],
      mode: (lstatOrNull(absolutePath(repoRoot, path))?.mode ?? 0) & 4095,
      size: 0
    });
    report(options.progress, "inventory", "directories", index + 1, directories.length, path);
  }
  const leaves = [...admitted.values()].filter((row) => row.mode !== "040000" && !row.explicitDirectory).sort((a, b) => Buffer.from(a.path).compare(Buffer.from(b.path)));
  const siblingFixedFilenameContractIdsByParent = new Map;
  const siblingIds = new Map;
  for (const row of leaves) {
    const parent = dirname2(row.path);
    const fixed = matchingFixedContracts(row.path, taxonomy.schema.fixedFilenameContracts, packageLocation(row.path, taxonomy), directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent));
    if (!fixed.selected)
      continue;
    const ids = siblingIds.get(parent) ?? new Set;
    ids.add(fixed.selected[0]);
    siblingIds.set(parent, ids);
  }
  for (const [parent, ids] of siblingIds)
    siblingFixedFilenameContractIdsByParent.set(parent, [...ids].sort(generatorPathCompare));
  for (let index = 0;index < leaves.length; index++) {
    checkCancellation(repoRoot, options.cancelFile);
    const row = leaves[index];
    const content = contentOf(repoRoot, row);
    const parent = dirname2(row.path) === "." ? "" : dirname2(row.path);
    const contentKind = content.kind === "file" ? extensionlessContentKind(row.path, content.bytes, taxonomy) : { kindId: null };
    const canonical = canonicalFile(row.path, canonicalDirectoryByPath.get(parent) ?? "", directoryKindByPath.get(parent), ancestorDirectoryKindIds(row.path, directoryKindByPath), directoryKindByPath, fixedDirectoryContractByPath, siblingFixedFilenameContractIdsByParent, taxonomy, contentKind.kindId ?? undefined);
    const violations2 = [...canonical.violations];
    if (content.violation)
      violations2.push(content.violation);
    if (contentKind.violation && !canonical.fixedId)
      violations2.push(contentKind.violation);
    let text = null;
    if (content.kind === "file" && content.size <= 16 * 1024 * 1024 && (textualPath(row.path) || contentKind.kindId !== null && contentKind.kindId !== "binary")) {
      try {
        text = new TextDecoder("utf-8", { fatal: true }).decode(content.bytes);
      } catch {
        text = null;
      }
    }
    const role = classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, text, taxonomy);
    let normalizedPath = canonical.path;
    if (role === "implementation") {
      const extracted = packageImplementationDestination(row.path, canonical, canonicalDirectoryByPath, directoryKindByPath, taxonomy);
      if (extracted) {
        normalizedPath = extracted;
        violations2.push(violation("package-implementation-file", row.path, `Package implementation must move to ${extracted}`, "warning"));
      } else
        violations2.push(violation("package-implementation-destination-unresolved", row.path, "Package implementation has no deterministic semantic owner"));
    }
    if (role === "unresolved")
      violations2.push(violation("package-role-unresolved", row.path, "Package role cannot be proven by the configured glue grammar"));
    violations2.push(...pathPolicyViolations(normalizedPath, taxonomy));
    if (content.kind === "symlink") {
      try {
        const target = readlinkSync(absolutePath(repoRoot, row.path));
        if (isAbsolute(target))
          violations2.push(violation("symlink-absolute-target", row.path, "Absolute symlink target cannot be proven repository-local"));
        else {
          const lexicalTarget = normalizeRelative(posix.join(dirname2(row.path), target.replaceAll("\\", "/")));
          if (isExcluded(lexicalTarget, taxonomy))
            violations2.push(violation("symlink-opaque-boundary", row.path, `Symlink lexically targets opaque path ${lexicalTarget}`));
        }
      } catch (error) {
        violations2.push(violation("symlink-target-unreadable", row.path, error instanceof Error ? error.message : String(error)));
      }
    }
    entries.set(row.path, {
      sourcePath: row.path,
      normalizedPath,
      nodeKind: content.kind,
      ownerId: ownerId(row.path),
      areaId: areaId(row.path),
      fileKind: canonical.fileKind,
      semanticStem: canonical.stem,
      fixedContractId: canonical.fixedId,
      packageRole: role,
      contentHash: content.hash,
      referencesIn: [],
      referencesOut: [],
      violations: violations2,
      mode: content.mode,
      size: content.size,
      symlinkTarget: content.symlinkTarget
    });
    report(options.progress, "inventory", "files", index + 1, leaves.length, row.path);
  }
  projectMutationTestBundles(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  validateProjectedMutationTestBundles(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  projectArtifactCatalogs(repoRoot, entries, taxonomy);
  const childrenByParent = new Map;
  for (const entry of entries.values()) {
    const parent = dirname2(entry.sourcePath);
    const children = childrenByParent.get(parent) ?? [];
    children.push(entry);
    childrenByParent.set(parent, children);
  }
  for (const path of [...directoryPaths].sort((a, b) => b.split("/").length - a.split("/").length || b.localeCompare(a))) {
    const entry = entries.get(path);
    if (entry)
      entry.contentHash = directoryHash(path, childrenByParent.get(path) ?? []);
  }
  referenceGraph(repoRoot, entries, taxonomy, options.progress, options.cancelFile);
  const frozenEntries = [...entries.values()].sort((a, b) => Buffer.from(a.sourcePath).compare(Buffer.from(b.sourcePath))).map((entry) => ({
    sourcePath: entry.sourcePath,
    normalizedPath: entry.normalizedPath,
    nodeKind: entry.nodeKind,
    ownerId: entry.ownerId,
    areaId: entry.areaId,
    fileKind: entry.fileKind,
    semanticStem: entry.semanticStem,
    fixedContractId: entry.fixedContractId,
    packageRole: entry.packageRole,
    contentHash: entry.contentHash,
    mode: entry.mode,
    size: entry.size,
    symlinkTarget: entry.symlinkTarget,
    referencesIn: [...entry.referencesIn],
    referencesOut: [...entry.referencesOut],
    violations: stableViolations(entry.violations)
  }));
  const violations = stableViolations(frozenEntries.flatMap((entry) => entry.violations));
  const sourceDigest = sourceTreeDigest(frozenEntries);
  const partial = {
    schemaVersion: 1,
    taxonomySchemaVersion: 7,
    scope,
    pathExclusions: taxonomy.exclusions.map((entry) => entry.path),
    activePathExclusions: activeExclusions,
    entries: frozenEntries,
    violations,
    sourceTreeDigest: sourceDigest
  };
  const inventory = {
    ...partial,
    repoRoot,
    taxonomyPath: taxonomy.path,
    inventoryDigest: inventoryDigestOf(partial)
  };
  report(options.progress, "inventory", "complete", frozenEntries.length, frozenEntries.length);
  return inventory;
}
function collisionKey(path, comparison) {
  if (comparison === "byte" || comparison === "same-kind")
    return path;
  if (comparison === "nfc")
    return path.normalize("NFC");
  if (comparison === "case-fold")
    return path.normalize("NFC").toLocaleLowerCase("und");
  return emojiFold(path).toLocaleLowerCase("und");
}
function collisionGroups(entries, taxonomy) {
  const groups = [];
  for (const comparison of taxonomy.schema.collisionPolicy.comparisons) {
    const buckets = new Map;
    for (const entry of entries) {
      const key = comparison === "same-kind" ? `${entry.nodeKind}\x00${entry.fileKind ?? "fixed"}\x00${collisionKey(entry.normalizedPath, comparison)}` : collisionKey(entry.normalizedPath, comparison);
      const rows = buckets.get(key) ?? [];
      rows.push(entry);
      buckets.set(key, rows);
    }
    for (const [key, rows] of buckets) {
      if (rows.length < 2)
        continue;
      const sources = rows.map((entry) => entry.sourcePath).sort();
      groups.push({ id: sha256(`${comparison}\x00${key}\x00${sources.join("\x00")}`).slice(0, 24), comparison, paths: [...new Set(rows.map((entry) => entry.normalizedPath))].sort(), sources });
    }
  }
  return groups.sort((a, b) => a.comparison.localeCompare(b.comparison) || a.id.localeCompare(b.id));
}
function generatorNodeRecord(repoRoot, path, taxonomy) {
  if (isExcluded(path, taxonomy))
    throw new Error(`Generator node is opaque: ${path}`);
  const absolute = absolutePath(repoRoot, path);
  const stat = lstatSync(absolute);
  const nodeKind = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file";
  const normalized = normalizeRelative(path), mode = stat.mode & 4095;
  if (nodeKind === "directory")
    return { path: normalized, nodeKind, contentHash: sha256("directory"), mode };
  if (nodeKind === "symlink") {
    const target = readlinkSync(absolute);
    return { path: normalized, nodeKind, contentHash: sha256(target), mode, size: Buffer.byteLength(target), target };
  }
  const bytes = readFileSync2(absolute);
  return { path: normalized, nodeKind, contentHash: sha256(bytes), mode, size: bytes.byteLength };
}
function generatorTreeInventory(repoRoot, roots, taxonomy) {
  const rows = new Map;
  const walk = (path) => {
    if (isExcluded(path, taxonomy))
      throw new Error(`Generator output root is opaque: ${path}`);
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat)
      return;
    rows.set(path, generatorNodeRecord(repoRoot, path, taxonomy));
    if (!stat.isDirectory() || stat.isSymbolicLink())
      return;
    for (const child of readdirSync2(absolute).sort((a, b) => Buffer.from(a).compare(Buffer.from(b))))
      walk(sourceRelative(`${path}/${child}`));
  };
  for (const root of [...new Set(roots.map(normalizeRelative))].sort(generatorPathCompare))
    walk(root);
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.path, right.path));
}
function generatorInputInventory(inventory, contract, taxonomy) {
  return inventory.entries.filter((entry) => contract.inputPatterns.some((pattern) => taxonomyPathPatternMatches(entry.sourcePath, pattern)) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).map((entry) => {
    const stat = lstatOrNull(absolutePath(inventory.repoRoot, entry.sourcePath));
    if (entry.nodeKind === "directory")
      return { path: entry.sourcePath, nodeKind: "directory", contentHash: entry.contentHash || sha256("directory"), mode: stat ? stat.mode & 4095 : 0 };
    if (entry.nodeKind === "symlink") {
      if (entry.symlinkTarget === undefined)
        throw new Error(`Generator input symlink lacks raw target authority: ${entry.sourcePath}`);
      return { path: entry.sourcePath, nodeKind: "symlink", contentHash: entry.contentHash, mode: entry.mode, size: entry.size, target: entry.symlinkTarget };
    }
    return { path: entry.sourcePath, nodeKind: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size };
  }).sort((left, right) => generatorPathCompare(left.path, right.path));
}
function previewNodeRecords(manifest) {
  return manifest.nodes.map((node) => {
    if (node.nodeKind === "directory")
      return { path: node.path, nodeKind: "directory", contentHash: sha256("directory"), mode: node.mode };
    const bytes = Buffer.from(node.bytesBase64, "base64");
    return { path: node.path, nodeKind: "file", contentHash: sha256(bytes), mode: node.mode, size: bytes.byteLength };
  });
}
function validatePreviewPreState(manifest, preOutputs) {
  const expected = new Set(manifest.nodes.map((node) => node.path));
  const prePaths = new Set(preOutputs.map((node) => node.path));
  for (const stale of manifest.staleRemovals)
    if (![...prePaths].some((path) => path === stale || path.startsWith(`${stale}/`)))
      throw new Error(`Generator preview stale removal does not exist in the output pre-state: ${stale}`);
  for (const path of prePaths)
    if (!expected.has(path) && !manifest.staleRemovals.some((stale) => path === stale || path.startsWith(`${stale}/`)))
      throw new Error(`Generator preview omits stale output from staleRemovals: ${path}`);
}
function invokeGeneratorPreview(inventory, id, contract, taxonomy) {
  if (!contract.ownerPath || !contract.previewTarget)
    throw new Error(`Owned generator ${id} has no preview target`);
  assertGeneratorPreviewTarget(inventory.repoRoot, contract.ownerPath, contract.previewTarget);
  const capture = mkdtempSync(join2(tmpdir(), "semio-generator-preview-"));
  const stdoutPath = join2(capture, "stdout.json");
  const stderrPath = join2(capture, "stderr.txt");
  let exitCode = -1;
  let success = false;
  try {
    const wrapper = 'const [stdoutPath,stderrPath]=process.argv.slice(1);const result=Bun.spawnSync(["bun","./\uD83D\uDCDC\uFE0Fscript.ts","preview-generated"],{stderr:"pipe",stdout:"pipe"});await Bun.write(stdoutPath,result.stdout);await Bun.write(stderrPath,result.stderr);process.exit(result.exitCode);';
    const result = spawnSync("bun", ["-e", wrapper, stdoutPath, stderrPath], { cwd: absolutePath(inventory.repoRoot, contract.ownerPath), encoding: "utf8", maxBuffer: 1024 * 1024 });
    exitCode = result.status ?? -1;
    success = !result.error && result.status === 0 && result.signal === null && result.stdout === "" && result.stderr === "";
    const stdout = readFileSync2(stdoutPath, "utf8");
    const stderr = readFileSync2(stderrPath, "utf8");
    if (!success || stderr !== "")
      throw new Error(`Generator preview command failed for ${id}: status=${exitCode}, stdout=${sha256(stdout)}, stderr=${sha256(stderr)}`);
    const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
    const manifest = parseGeneratorPreviewManifest(stdout, id, roots, taxonomy.exclusions.map((entry) => entry.path));
    return { manifest, digest: sha256(stdout) };
  } finally {
    rmSync(capture, { recursive: true, force: true });
  }
}
function projectedPath(path, entries) {
  const mappings = entries.filter((entry) => entry.sourcePath !== entry.normalizedPath && (path === entry.sourcePath || path.startsWith(`${entry.sourcePath}/`))).sort((left, right) => right.sourcePath.length - left.sourcePath.length || generatorPathCompare(left.sourcePath, right.sourcePath));
  if (mappings.length === 0)
    return path;
  const longest = mappings[0].sourcePath.length;
  const destinations = new Set(mappings.filter((entry) => entry.sourcePath.length === longest).map((entry) => `${entry.normalizedPath}${path.slice(entry.sourcePath.length)}`));
  if (destinations.size !== 1)
    throw new Error(`Path projection is ambiguous for ${path}`);
  return [...destinations][0];
}
function repositoryLocalSymlinkTargetPath(repoRoot, target) {
  if (repoRoot.includes("\x00") || target.includes("\x00"))
    return null;
  const slash = (value) => value.replaceAll("\\", "/").replace(/\/+$/u, "");
  const root = slash(repoRoot);
  const candidate = slash(target);
  const drive = /^([A-Za-z]):\/(.*)$/u;
  const rootDrive = drive.exec(root);
  const targetDrive = drive.exec(candidate);
  const unc = /^\/\/([^/]+)\/([^/]+)(?:\/(.*))?$/u;
  const rootUnc = unc.exec(root);
  const targetUnc = unc.exec(candidate);
  let suffix;
  if (rootDrive || targetDrive) {
    if (!rootDrive || !targetDrive || rootDrive[1].toLowerCase() !== targetDrive[1].toLowerCase())
      return null;
    const rootTail = rootDrive[2].replace(/\/+$/u, "");
    const targetTail = targetDrive[2];
    if (targetTail.toLowerCase() !== rootTail.toLowerCase() && !targetTail.toLowerCase().startsWith(`${rootTail.toLowerCase()}/`))
      return null;
    suffix = targetTail.slice(rootTail.length).replace(/^\//u, "");
  } else if (rootUnc || targetUnc) {
    if (!rootUnc || !targetUnc || rootUnc[1].toLowerCase() !== targetUnc[1].toLowerCase() || rootUnc[2].toLowerCase() !== targetUnc[2].toLowerCase())
      return null;
    const rootTail = (rootUnc[3] ?? "").replace(/\/+$/u, "");
    const targetTail = targetUnc[3] ?? "";
    if (targetTail.toLowerCase() !== rootTail.toLowerCase() && !targetTail.toLowerCase().startsWith(`${rootTail.toLowerCase()}/`))
      return null;
    suffix = targetTail.slice(rootTail.length).replace(/^\//u, "");
  } else {
    if (!root.startsWith("/") || !candidate.startsWith("/") || candidate !== root && !candidate.startsWith(`${root}/`))
      return null;
    suffix = candidate.slice(root.length).replace(/^\//u, "");
  }
  if (!suffix || suffix.split("/").some((segment) => segment === "" || segment === "." || segment === ".."))
    return null;
  try {
    return normalizeRelative(suffix);
  } catch {
    return null;
  }
}
function planSymlinkTargetEdits(inventory, taxonomy) {
  const edits = [];
  const violations = [];
  const bySource = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "symlink" && candidate.symlinkTarget !== undefined)) {
    const oldTarget = entry.symlinkTarget;
    const absoluteSyntax = oldTarget.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(oldTarget) || /^(?:\\\\|\/\/)/u.test(oldTarget);
    if (!absoluteSyntax)
      continue;
    const logicalTargetSourcePath = repositoryLocalSymlinkTargetPath(inventory.repoRoot, oldTarget);
    if (logicalTargetSourcePath === null || isExcluded(logicalTargetSourcePath, taxonomy)) {
      violations.push(violation("symlink-absolute-target-authority-invalid", entry.sourcePath, "Absolute symlink target is external, escaping, or opaque"));
      continue;
    }
    const finalPath = projectedPath(entry.sourcePath, inventory.entries);
    const logicalTargetFinalPath = projectedPath(logicalTargetSourcePath, inventory.entries);
    const targetEntry = bySource.get(logicalTargetSourcePath);
    if (targetEntry?.nodeKind === "directory") {
      violations.push(violation("symlink-target-directory-authority-unresolved", entry.sourcePath, `Directory target requires a recursive no-follow authority: ${logicalTargetSourcePath}`));
      continue;
    }
    const logicalTargetPreimage = !targetEntry ? { state: "absent" } : targetEntry.nodeKind === "directory" ? { state: "directory" } : targetEntry.nodeKind === "symlink" ? { state: "symlink", contentHash: targetEntry.contentHash, mode: targetEntry.mode, size: targetEntry.size, target: targetEntry.symlinkTarget } : { state: "file", contentHash: targetEntry.contentHash, mode: targetEntry.mode, size: targetEntry.size };
    const extension = resolveFileKind(logicalTargetSourcePath, taxonomy, [], []).kind;
    if (!targetEntry && !extension) {
      violations.push(violation("symlink-target-kind-unresolved", entry.sourcePath, `Broken target kind cannot be proven: ${logicalTargetSourcePath}`));
      continue;
    }
    const newTarget = posix.relative(posix.dirname(finalPath), logicalTargetFinalPath);
    if (!newTarget || newTarget.startsWith("/") || isExcluded(posix.normalize(posix.join(posix.dirname(finalPath), newTarget)), taxonomy)) {
      violations.push(violation("symlink-target-render-invalid", entry.sourcePath, "Relative target rendering is empty, absolute, or opaque"));
      continue;
    }
    const targetDigestible = { sourcePath: entry.sourcePath, finalPath, oldTarget, newTarget, logicalTargetSourcePath, logicalTargetFinalPath, logicalTargetPreimage };
    const provisional = { sourcePath: entry.sourcePath, finalPath, oldTarget, newTarget, oldTargetHash: sha256(oldTarget), newTargetHash: sha256(newTarget), logicalTargetSourcePath, logicalTargetFinalPath, logicalTargetPreimage, windowsLinkType: targetEntry?.nodeKind === "directory" ? "dir" : "file", sourceTargetDigest: sha256(canonicalJson(targetDigestible)), rationaleRule: "repository-local-absolute-symlink-target-v1", ownerId: entry.ownerId };
    edits.push({ operationId: dispositionOperationId("symlink-target-edit", provisional), ...provisional });
  }
  return { edits: edits.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}
function incomingEmbeddedReferences(inventory, root) {
  const rows = new Set;
  for (const entry of inventory.entries.filter((candidate) => candidate.sourcePath === root || candidate.sourcePath.startsWith(`${root}/`)))
    for (const source of entry.referencesIn)
      if (source !== root && !source.startsWith(`${root}/`))
        rows.add(`text\x00${source}\x00${entry.sourcePath}`);
  for (const link of inventory.entries.filter((candidate) => candidate.nodeKind === "symlink" && candidate.symlinkTarget !== undefined && candidate.sourcePath !== root && !candidate.sourcePath.startsWith(`${root}/`))) {
    let target = null;
    if (link.symlinkTarget.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(link.symlinkTarget) || /^(?:\\\\|\/\/)/u.test(link.symlinkTarget))
      target = repositoryLocalSymlinkTargetPath(inventory.repoRoot, link.symlinkTarget);
    else
      try {
        target = normalizeRelative(posix.join(posix.dirname(link.sourcePath), link.symlinkTarget.replaceAll("\\", "/")));
      } catch {
        target = null;
      }
    if (target && (target === root || target.startsWith(`${root}/`)))
      rows.add(`symlink\x00${link.sourcePath}\x00${target}`);
  }
  return [...rows].sort(generatorPathCompare);
}
function lexicalTargetIncomingReferences(repoRoot, targetPaths, ignoredSourceRoots, taxonomy, ticketDir, planAuthority, transactionRoot) {
  const candidates = new Set([...gitRows(repoRoot, taxonomy).map((entry) => entry.path), ...untrackedGitPaths(repoRoot, taxonomy)]);
  if (ticketDir)
    for (const row of explicitTicketRows(repoRoot, ticketDir, taxonomy))
      candidates.add(row.path);
  const targetIndex = referencePathIndex(targetPaths);
  const rows = [];
  for (const path of [...candidates].sort(generatorPathCompare)) {
    if (isExcluded(path, taxonomy) || ignoredSourceRoots.some((root) => path === root || path.startsWith(`${root}/`)) || transactionRoot && (path === transactionRoot || path.startsWith(`${transactionRoot}/`)))
      continue;
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat || stat.isDirectory())
      continue;
    if (stat.isSymbolicLink()) {
      const raw = readlinkSync(absolute);
      let target = null;
      if (raw.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(raw) || /^(?:\\\\|\/\/)/u.test(raw))
        target = repositoryLocalSymlinkTargetPath(repoRoot, raw);
      else
        try {
          target = normalizeRelative(posix.join(posix.dirname(path), raw.replaceAll("\\", "/")));
        } catch {
          target = null;
        }
      if (target && targetPaths.has(target))
        rows.push(`symlink\x00${path}\x00${target}`);
      continue;
    }
    if (!stat.isFile() || !textualPath(path))
      continue;
    const bytes = readFileSync2(absolute);
    if (planAuthority?.path === path && bytes.equals(planAuthority.bytes))
      continue;
    const content = bytes.toString("utf8");
    for (const token of referenceTokens(path, content)) {
      const target = resolveReferenceTokenPath(path, token, targetIndex);
      if (target && targetPaths.has(target))
        rows.push(`text\x00${path}\x00${target}`);
    }
  }
  return [...new Set(rows)].sort(generatorPathCompare);
}
function embeddedTargetPaths(plan, root) {
  const targetPaths = new Set([root.sourceMetadataRoot, root.sourceTicketRoot]);
  for (const id of [...root.relocationOperationIds, ...root.removalOperationIds]) {
    const leaf = plan.embeddedTicketRootRelocations.find((entry) => entry.operationId === id)?.sourcePath ?? plan.evidenceRemovals.find((entry) => entry.operationId === id)?.sourcePath;
    if (!leaf)
      continue;
    targetPaths.add(leaf);
    for (let parent = posix.dirname(leaf);parent === root.sourceMetadataRoot || parent.startsWith(`${root.sourceMetadataRoot}/`); parent = posix.dirname(parent)) {
      targetPaths.add(parent);
      if (parent === root.sourceMetadataRoot)
        break;
    }
  }
  return targetPaths;
}
function lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, ticketDir, planAuthority, transactionRoot) {
  return lexicalTargetIncomingReferences(repoRoot, embeddedTargetPaths(plan, root), [root.sourceMetadataRoot], taxonomy, ticketDir, planAuthority, transactionRoot);
}
function planEmbeddedTicketRoots(inventory, taxonomy) {
  const violations = [];
  const roots = [];
  const relocations = [];
  const removals = [];
  const nested = new Map;
  for (const entry of inventory.entries) {
    const parts = entry.sourcePath.split("/");
    for (let index = 1;index + 7 < parts.length; index++) {
      if (parts[index] !== ".\uD83E\uDDECsemio" || parts[index + 1] !== "\uD83E\uDD91\uFE0Frepo" || parts[index + 2] !== "\uD83C\uDFAB\uFE0Ftickets")
        continue;
      const sourceMetadataRoot = parts.slice(0, index + 1).join("/");
      const suffix = parts.slice(index + 3, index + 7);
      if (!/^\uD83C\uDF86\uFE0F[0-9]{2}$/u.test(suffix[0] ?? "") || !/^\uD83C\uDF19\uFE0F[0-9]{2}$/u.test(suffix[1] ?? "") || !/^\u2600\uFE0F[0-9]{2}$/u.test(suffix[2] ?? "") || !suffix[3]) {
        violations.push(violation("embedded-ticket-root-identity-invalid", entry.sourcePath, "Nested metadata path has no exact ticket identity"));
        continue;
      }
      const canonicalTicketRoot = [".\uD83E\uDDECsemio", "\uD83E\uDD91\uFE0Frepo", "\uD83C\uDFAB\uFE0Ftickets", ...suffix].join("/");
      const rootContract = matchingFixedContracts(canonicalTicketRoot, taxonomy.schema.fixedDirectoryContracts, null).selected;
      const manifestPath = `${canonicalTicketRoot}/\uD83C\uDFAB\uFE0Fticket.json`;
      const manifestContract = matchingFixedContracts(manifestPath, taxonomy.schema.fixedFilenameContracts, null).selected;
      const manifestStat = lstatOrNull(absolutePath(inventory.repoRoot, manifestPath));
      if (rootContract?.[0] !== "ticket-slug" || manifestContract?.[0] !== "ticket-manifest" || !manifestStat?.isFile() || manifestStat.isSymbolicLink()) {
        violations.push(violation("embedded-ticket-root-authority-missing", entry.sourcePath, "Canonical ticket root and exact ticket manifest authority must exist"));
        continue;
      }
      nested.set(sourceMetadataRoot, { sourceTicketRoot: parts.slice(0, index + 7).join("/"), canonicalTicketRoot, ticketId: `${splitLeadingEmoji(suffix[0]).rest}/${splitLeadingEmoji(suffix[1]).rest}/${splitLeadingEmoji(suffix[2]).rest}/${suffix[3]}` });
    }
  }
  const occupancy = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const candidates = [];
  for (const [sourceMetadataRoot, identity] of [...nested].sort(([left], [right]) => generatorPathCompare(left, right))) {
    const allLeaves = inventory.entries.filter((entry) => entry.nodeKind !== "directory" && entry.sourcePath.startsWith(`${sourceMetadataRoot}/`));
    const leaves = allLeaves.filter((entry) => entry.sourcePath.startsWith(`${identity.sourceTicketRoot}/`));
    if (allLeaves.length !== leaves.length) {
      violations.push(violation("embedded-ticket-root-residual-leaf", sourceMetadataRoot, "Nested metadata root contains a leaf outside its exact ticket root"));
      continue;
    }
    if (leaves.some((entry) => entry.nodeKind === "symlink" || !entry.fixedContractId)) {
      violations.push(violation("embedded-ticket-root-leaf-unresolved", sourceMetadataRoot, "Nested metadata root contains a symlink or a leaf without exact fixed-contract authority"));
      continue;
    }
    const sourceTreeDigest2 = noFollowTreeDigest(inventory.repoRoot, sourceMetadataRoot);
    const residualTreeDigest = noFollowTreeDigestExcluding(inventory.repoRoot, sourceMetadataRoot, leaves.map((entry) => entry.sourcePath));
    const incoming = incomingEmbeddedReferences(inventory, sourceMetadataRoot);
    const incomingReferenceDigest = sha256(`sha256-taxonomy-reference-set-v1\x00${canonicalJson(incoming)}`);
    if (incoming.length > 0) {
      violations.push(violation("embedded-ticket-root-incoming-reference", sourceMetadataRoot, `Nested metadata root has ${incoming.length} incoming reference(s)`));
      continue;
    }
    const authority = { sourceMetadataRoot, sourceTicketRoot: identity.sourceTicketRoot, canonicalTicketRoot: identity.canonicalTicketRoot, ticketId: identity.ticketId, sourceTreeDigest: sourceTreeDigest2, residualTreeDigest, incomingReferenceDigest, rationaleRule: "embedded-ticket-root-relocation-v1" };
    const rootId = dispositionOperationId("embedded-ticket-root", authority);
    for (const entry of leaves) {
      const destinationPath = `${identity.canonicalTicketRoot}/${entry.sourcePath.slice(identity.sourceTicketRoot.length + 1)}`;
      if (destinationPath !== normalizeRelative(destinationPath) || pathPolicyViolations(destinationPath, taxonomy).length > 0 || !entry.fixedContractId) {
        violations.push(violation("embedded-ticket-root-destination-invalid", destinationPath, "Embedded evidence destination is noncanonical, over budget, or lacks exact fixed authority"));
        continue;
      }
      candidates.push({ root: sourceMetadataRoot, rootId, sourcePath: entry.sourcePath, destinationPath, entry });
    }
    roots.push({ operationId: rootId, ...authority, relocationOperationIds: [], removalOperationIds: [] });
  }
  const byDestination = new Map;
  for (const candidate of candidates)
    byDestination.set(candidate.destinationPath, [...byDestination.get(candidate.destinationPath) ?? [], candidate]);
  for (const [destinationPath, group] of [...byDestination].sort(([left], [right]) => generatorPathCompare(left, right))) {
    const sorted = [...group].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    const occupied = occupancy.get(destinationPath);
    const evidenceIdentity = (entry) => canonicalJson({ nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, ownerId: entry.ownerId, fixedContractId: entry.fixedContractId, packageRole: entry.packageRole });
    const identity = evidenceIdentity(sorted[0].entry);
    if (sorted.some((entry) => evidenceIdentity(entry.entry) !== identity) || occupied && evidenceIdentity(occupied) !== identity) {
      violations.push(violation("embedded-ticket-root-destination-conflict", destinationPath, "Many-to-one ticket evidence is not byte, mode, kind, size, owner, role and contract identical"));
      continue;
    }
    const installer = occupied ? null : sorted[0];
    if (installer) {
      const provisional = { embeddedTicketRootId: installer.rootId, sourcePath: installer.sourcePath, destinationPath, relativeEvidencePath: installer.sourcePath.slice(nested.get(installer.root).sourceTicketRoot.length + 1), preimage: inventoryLeafPreimage(installer.entry), fixedContractId: installer.entry.fixedContractId, ownerId: installer.entry.ownerId, rationaleRule: "embedded-ticket-root-relocation-v1" };
      relocations.push({ operationId: dispositionOperationId("embedded-ticket-root-relocation", provisional), ...provisional });
    }
    for (const candidate of sorted.filter((entry) => entry !== installer)) {
      const members = [
        ...sorted.map((entry) => ({ sourcePath: entry.sourcePath, finalPath: destinationPath, disposition: entry === installer ? "relocate" : "remove", preimage: inventoryLeafPreimage(entry.entry) })),
        ...occupied ? [{ sourcePath: destinationPath, finalPath: destinationPath, disposition: "retain", preimage: inventoryLeafPreimage(occupied) }] : []
      ].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
      const retainedFinalPath = destinationPath;
      const evidenceSetDigest = sha256(canonicalJson({ algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath }));
      const authority = { kind: "byte-and-mode-identical", evidenceSetDigest, retainedFinalPath, members };
      const provisional = { sourcePath: candidate.sourcePath, preimage: inventoryLeafPreimage(candidate.entry), authority, embeddedTicketRootId: candidate.rootId, rationaleRule: "redundant-ticket-evidence-v1", ownerId: candidate.entry.ownerId };
      removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
    }
  }
  for (const root of roots) {
    const children = relocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).length + removals.filter((entry) => entry.embeddedTicketRootId === root.operationId).length;
    if (children !== root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks)
      violations.push(violation("embedded-ticket-root-closure-incomplete", root.sourceMetadataRoot, `Frozen tree has ${root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks} leaves but ${children} dispositions`));
  }
  if (violations.length > 0)
    return { roots: [], relocations: [], removals: [], violations: stableViolations(violations) };
  const finalizedRoots = roots.map((root) => ({ ...root, relocationOperationIds: relocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId).sort(generatorPathCompare), removalOperationIds: removals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId).sort(generatorPathCompare) }));
  return { roots: finalizedRoots, relocations: relocations.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), removals: removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}
function planTrailingEvidenceRemovals(inventory) {
  const rows = [];
  for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind !== "directory" && candidate.sourcePath.startsWith(".\uD83E\uDDECsemio/\uD83E\uDD91\uFE0Frepo/\uD83C\uDFAB\uFE0Ftickets/") && /^[. ]+$/u.test(basename2(candidate.sourcePath)))) {
    const parent = posix.dirname(entry.sourcePath);
    const identical = inventory.entries.filter((candidate) => candidate.nodeKind === entry.nodeKind && posix.dirname(candidate.sourcePath) === parent && candidate.contentHash === entry.contentHash && candidate.mode === entry.mode && candidate.size === entry.size && candidate.ownerId === entry.ownerId && candidate.fixedContractId === entry.fixedContractId && candidate.packageRole === entry.packageRole && candidate.sourcePath !== entry.sourcePath && !/^[. ]+$/u.test(basename2(candidate.sourcePath))).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    if (identical.length === 0 || entry.referencesIn.length > 0)
      continue;
    const retainedFinalPath = identical[0].normalizedPath;
    const members = [{ sourcePath: entry.sourcePath, finalPath: retainedFinalPath, disposition: "remove", preimage: inventoryLeafPreimage(entry) }, ...identical.map((candidate) => ({ sourcePath: candidate.sourcePath, finalPath: candidate.normalizedPath, disposition: "retain", preimage: inventoryLeafPreimage(candidate) }))].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    const evidenceSetDigest = sha256(canonicalJson({ algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath }));
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority: { kind: "byte-and-mode-identical", evidenceSetDigest, retainedFinalPath, members }, rationaleRule: "redundant-ticket-evidence-v1", ownerId: entry.ownerId };
    rows.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
}
function serializedSentinelCases(repoRoot) {
  const absolute = absolutePath(repoRoot, TRANSACTION_DISPOSITIONS_FIXTURE_PATH);
  const stat = lstatOrNull(absolute);
  if (!stat)
    return null;
  if (!stat.isFile() || stat.isSymbolicLink())
    throw new Error("Transaction disposition authority fixture must be a regular no-follow file");
  const bytes = readFileSync2(absolute);
  const value = record(JSON.parse(bytes.toString("utf8")), "transaction disposition fixture");
  requireExactKeys(value, ["schemaVersion", "virtualPathPolicyCases", "symlinkFlavorCases"], "transaction disposition fixture");
  if (value.schemaVersion !== 1 || !Array.isArray(value.virtualPathPolicyCases) || !Array.isArray(value.symlinkFlavorCases))
    throw new Error("Transaction disposition fixture has an invalid schema");
  const cases = value.virtualPathPolicyCases.map((item, index) => {
    const row = record(item, `transaction disposition fixture.virtualPathPolicyCases[${index}]`);
    requireExactKeys(row, ["id", "inputPath", "physicalSourcePath", "expectedViolationCode", "sourceContentHash"], `transaction disposition fixture.virtualPathPolicyCases[${index}]`);
    if (row.expectedViolationCode !== "windows-reserved-name" && row.expectedViolationCode !== "trailing-dot-or-space")
      throw new Error("Transaction disposition fixture has an invalid violation code");
    if (row.physicalSourcePath !== null && typeof row.physicalSourcePath !== "string")
      throw new Error("Transaction disposition fixture has an invalid physical source path");
    return { id: planString(row.id, "sentinel case id"), inputPath: planPath(row.inputPath, "sentinel input path"), physicalSourcePath: row.physicalSourcePath === null ? null : planPath(row.physicalSourcePath, "sentinel physical source path"), expectedViolationCode: row.expectedViolationCode, sourceContentHash: planString(row.sourceContentHash, "sentinel content hash", PLAN_HASH) };
  }).sort((left, right) => generatorPathCompare(left.id, right.id));
  if (new Set(cases.map((entry) => entry.id)).size !== cases.length || new Set(cases.map((entry) => entry.inputPath)).size !== cases.length)
    throw new Error("Transaction disposition sentinel cases must have unique IDs and input paths");
  return { fixtureContentHash: sha256(bytes), cases };
}
function planSerializedEvidenceRemovals(inventory) {
  const fixtureEntry = inventory.entries.find((entry) => entry.sourcePath === TRANSACTION_DISPOSITIONS_FIXTURE_PATH);
  if (!fixtureEntry)
    return { removals: [], violations: [] };
  const authority = serializedSentinelCases(inventory.repoRoot);
  if (!authority || fixtureEntry.nodeKind !== "file" || fixtureEntry.contentHash !== authority.fixtureContentHash)
    return { removals: [], violations: [violation("serialized-sentinel-authority-invalid", TRANSACTION_DISPOSITIONS_FIXTURE_PATH, "Serialized sentinel fixture bytes are not frozen by inventory")] };
  const removals = [];
  const violations = [];
  for (const sentinel of authority.cases) {
    if (sentinel.physicalSourcePath === null)
      continue;
    const entry = inventory.entries.find((candidate) => candidate.sourcePath === sentinel.physicalSourcePath);
    if (!entry)
      continue;
    if (entry.nodeKind !== "file" || entry.contentHash !== sentinel.sourceContentHash || entry.referencesIn.length > 0 || !entry.violations.some((row) => row.code === sentinel.expectedViolationCode)) {
      violations.push(violation("serialized-sentinel-source-invalid", sentinel.inputPath, `Physical sentinel does not match serialized case ${sentinel.id}`));
      continue;
    }
    const removalAuthority = { kind: "serialized-path-sentinel", fixturePath: TRANSACTION_DISPOSITIONS_FIXTURE_PATH, fixtureContentHash: authority.fixtureContentHash, caseId: sentinel.id, serializedInputPath: sentinel.inputPath, expectedViolationCode: sentinel.expectedViolationCode, authorityDigest: "" };
    const { authorityDigest: _authorityDigest, ...digestible } = removalAuthority;
    const frozenAuthority = { ...removalAuthority, authorityDigest: sha256(canonicalJson(digestible)) };
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority: frozenAuthority, rationaleRule: "serialized-platform-sentinel-v1", ownerId: entry.ownerId };
    removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return { removals: removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}
function planMoveReferenceAuthority(inventory, taxonomy, options, embeddedRoots, evidenceRemovals) {
  const embeddedPrefixes = embeddedRoots.map((root) => root.sourceMetadataRoot);
  const removalSources = new Set(evidenceRemovals.map((entry) => entry.sourcePath));
  const isEmbedded = (path) => embeddedPrefixes.some((root) => path === root || path.startsWith(`${root}/`));
  const groups = collisionGroups(inventory.entries.filter((entry) => !removalSources.has(entry.sourcePath)), taxonomy);
  const groupBySource = new Map;
  for (const group of groups)
    for (const source of group.sources)
      if (!groupBySource.has(source))
        groupBySource.set(source, group.id);
  const preliminaryMoves = inventory.entries.filter((entry) => entry.nodeKind !== "directory" && entry.sourcePath !== entry.normalizedPath && !groupBySource.has(entry.sourcePath) && !isEmbedded(entry.sourcePath) && !removalSources.has(entry.sourcePath) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0).map((entry) => {
    const sourcePreimage = inventoryLeafPreimage(entry);
    return {
      operationId: dispositionOperationId("move-v2", { sourcePath: entry.sourcePath, destinationPath: entry.normalizedPath, sourcePreimage }),
      sourcePath: entry.sourcePath,
      destinationPath: entry.normalizedPath,
      sourcePreimage,
      rationaleRule: artifactCatalogProjectionRationale(entry.sourcePath, entry.normalizedPath, taxonomy) ?? mutationProjectionRationale(entry.sourcePath, entry.normalizedPath, taxonomy) ?? (entry.semanticStem ? "semantic-stem-resolution" : entry.fixedContractId ? "fixed-contract-preservation" : "canonical-kind-name"),
      ownerId: entry.ownerId,
      collisionGroup: groupBySource.get(entry.sourcePath),
      referenceEdits: []
    };
  }).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath) || generatorPathCompare(left.destinationPath, right.destinationPath));
  const references = buildReferenceEdits(inventory, preliminaryMoves, taxonomy, options, referencePathIndex(inventory.entries.map((entry) => entry.sourcePath)));
  return {
    moves: preliminaryMoves.map((move) => ({ ...move, referenceEdits: references.edits.filter((edit) => references.editTargets.get(referenceEditIdentity(edit)) === move.sourcePath) })),
    edits: references.edits,
    editTargets: references.editTargets,
    resultHashes: references.resultHashes,
    resultSizes: references.resultSizes,
    unresolved: references.unresolved,
    collisionGroups: groups
  };
}
function generatorPlanning(inventory, moves, edits, taxonomy, options) {
  const mutations = new Set;
  for (const move of moves) {
    mutations.add(move.sourcePath);
    mutations.add(move.destinationPath);
  }
  for (const edit of edits) {
    mutations.add(edit.path);
    const source = inventory.entries.find((entry) => entry.normalizedPath === edit.path)?.sourcePath;
    if (source)
      mutations.add(source);
  }
  const rows = [];
  const regenerations = [];
  const contracts = Object.entries(taxonomy.schema.generatorContracts).sort(([left], [right]) => left.localeCompare(right));
  for (let index = 0;index < contracts.length; index++) {
    const [id, contract] = contracts[index];
    const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
    const outputEntries = inventory.entries.filter((entry) => roots.some((root) => entry.sourcePath === root || entry.sourcePath.startsWith(`${root}/`)));
    const outputProblem = outputEntries.some((entry) => !roots.includes(entry.sourcePath) && (entry.sourcePath !== entry.normalizedPath || entry.violations.some((entry2) => entry2.severity === "error")));
    const outputMutation = [...mutations].some((path2) => roots.some((root) => pathsOverlap(path2, root)));
    const inputMutation = [...mutations].some((path2) => contract.inputPatterns.some((pattern) => taxonomyPathPatternMatches(path2, pattern)));
    if (!outputProblem && !outputMutation && !inputMutation)
      continue;
    const inputs = generatorInputInventory(inventory, contract, taxonomy);
    const preOutputs = generatorTreeInventory(inventory.repoRoot, roots, taxonomy);
    const inputDigest = sha256(canonicalJson(inputs));
    const preOutputDigest = sha256(canonicalJson(preOutputs));
    const path = roots[0];
    if (contract.ownership !== "owned") {
      rows.push(violation(`generator-ownership-${contract.ownership}`, path, `Generator contract ${id} is ${contract.ownership}; ${contract.reason}; input ${inputDigest}, output ${preOutputDigest}`));
      continue;
    }
    try {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const preview = invokeGeneratorPreview(inventory, id, contract, taxonomy);
      checkCancellation(inventory.repoRoot, options.cancelFile);
      validatePreviewPreState(preview.manifest, preOutputs);
      const outputs = previewNodeRecords(preview.manifest);
      const changed = canonicalJson(preOutputs) !== canonicalJson(outputs) || preview.manifest.staleRemovals.length > 0;
      if (inputMutation || outputMutation || changed) {
        const command = ["bun", "nx", "run", contract.target];
        const verifyCommand = contract.checkTarget ? ["bun", "nx", "run", contract.checkTarget] : undefined;
        const provisional = { contractId: id, cwd: contract.ownerPath, command, verifyCommand, outputRoots: roots, inputs, preOutputs, outputs, preview: preview.manifest, previewManifestDigest: preview.digest, staleRemovals: preview.manifest.staleRemovals };
        regenerations.push({ id: sha256(canonicalJson(provisional)).slice(0, 24), ...provisional });
      }
      report(options.progress, "plan", "generator-preview", index + 1, contracts.length, id);
    } catch (error) {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const message = error instanceof Error ? error.message.replaceAll(resolve2(inventory.repoRoot), "<repo>") : String(error);
      rows.push(violation("generator-preview-invalid", path, `Generator ${id} preview was rejected: ${message}`));
    }
  }
  return { regenerations: regenerations.sort((left, right) => left.contractId.localeCompare(right.contractId) || left.id.localeCompare(right.id)), violations: stableViolations(rows) };
}
function affectedStateDigest(rows) {
  const sorted = [...rows].sort((left, right) => generatorPathCompare(left.path, right.path));
  const unique = new Map;
  for (const row of sorted) {
    const prior = unique.get(row.path);
    if (prior && canonicalJson(prior) !== canonicalJson(row))
      throw new Error(`Conflicting affected path-state rows at ${row.path}`);
    unique.set(row.path, row);
  }
  return sha256(`sha256-affected-path-state-v2\x00${canonicalJson([...unique.values()])}`);
}
function entryStateRow(path, entry) {
  if (!entry)
    return { path, state: "absent" };
  if (entry.nodeKind === "symlink")
    return { path, state: "symlink", targetHash: entry.contentHash, targetSize: entry.size };
  if (entry.nodeKind === "file")
    return { path, state: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size };
  throw new Error(`Affected directory requires an explicit no-follow tree digest: ${path}`);
}
function pathPreimageRow(path, preimage) {
  if (preimage.state === "absent")
    return { path, state: "absent" };
  if (preimage.state === "directory")
    throw new Error(`Directory logical target requires recursive tree authority: ${path}`);
  return preimage.state === "symlink" ? { path, state: "symlink", targetHash: preimage.contentHash, targetSize: preimage.size } : { path, state: "file", contentHash: preimage.contentHash, mode: preimage.mode, size: preimage.size };
}
function destinationAncestorPreimages(repoRoot, destinations) {
  const rows = new Map;
  for (const destination of destinations) {
    for (let path = posix.dirname(destination);path !== "." && path !== ""; path = posix.dirname(path)) {
      const stat = lstatOrNull(absolutePath(repoRoot, path));
      if (stat?.isSymbolicLink() || stat && !stat.isDirectory())
        throw new Error(`Mutation destination ancestor is not a no-follow directory: ${path}`);
      rows.set(path, { path, state: stat ? "directory" : "absent" });
    }
  }
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.path, right.path));
}
function plannedAffectedStateDigests(inventory, plan, resultHashes, resultSizes) {
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const pre = [];
  const post = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    pre.push(ancestor);
    post.push({ path: ancestor.path, state: "directory" });
  }
  for (const move of plan.moves) {
    const source = entries.get(move.sourcePath);
    const targetEdit = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath);
    const postSource = source && targetEdit ? { ...source, sourcePath: move.destinationPath, contentHash: targetEdit.newTargetHash, size: Buffer.byteLength(targetEdit.newTarget), symlinkTarget: targetEdit.newTarget } : source ? { ...source, sourcePath: move.destinationPath, contentHash: resultHashes.get(move.destinationPath) ?? source.contentHash, size: resultSizes.get(move.destinationPath) ?? source.size } : undefined;
    pre.push(entryStateRow(move.sourcePath, source), entryStateRow(move.destinationPath, entries.get(move.destinationPath)));
    post.push({ path: move.sourcePath, state: "absent" }, entryStateRow(move.destinationPath, postSource));
  }
  for (const relocation of plan.embeddedTicketRootRelocations) {
    pre.push(entryStateRow(relocation.sourcePath, entries.get(relocation.sourcePath)), entryStateRow(relocation.destinationPath, entries.get(relocation.destinationPath)));
    post.push({ path: relocation.sourcePath, state: "absent" }, relocation.preimage.nodeKind === "symlink" ? { path: relocation.destinationPath, state: "symlink", targetHash: relocation.preimage.contentHash, targetSize: relocation.preimage.size } : { path: relocation.destinationPath, state: "file", contentHash: relocation.preimage.contentHash, mode: relocation.preimage.mode, size: relocation.preimage.size });
  }
  for (const removal of plan.evidenceRemovals) {
    pre.push(entryStateRow(removal.sourcePath, entries.get(removal.sourcePath)));
    post.push({ path: removal.sourcePath, state: "absent" });
    if (removal.authority.kind === "byte-and-mode-identical")
      for (const member of removal.authority.members.filter((member2) => member2.disposition !== "remove")) {
        pre.push(entryStateRow(member.sourcePath, entries.get(member.sourcePath)));
        post.push(member.preimage.nodeKind === "symlink" ? { path: member.finalPath, state: "symlink", targetHash: member.preimage.contentHash, targetSize: member.preimage.size } : { path: member.finalPath, state: "file", contentHash: member.preimage.contentHash, mode: member.preimage.mode, size: member.preimage.size });
      }
    if (removal.authority.kind === "serialized-path-sentinel") {
      const fixture = entries.get(removal.authority.fixturePath);
      pre.push(entryStateRow(removal.authority.fixturePath, fixture));
      post.push(entryStateRow(removal.authority.fixturePath, fixture));
    }
  }
  for (const root of plan.embeddedTicketRoots) {
    pre.push({ path: root.sourceMetadataRoot, state: "directory-tree", tree: root.sourceTreeDigest });
    post.push({ path: root.sourceMetadataRoot, state: "absent" });
  }
  for (const edit of plan.symlinkTargetEdits) {
    pre.push({ path: edit.sourcePath, state: "symlink", targetHash: edit.oldTargetHash, targetSize: Buffer.byteLength(edit.oldTarget) });
    pre.push(pathPreimageRow(edit.logicalTargetSourcePath, edit.logicalTargetPreimage));
    post.push({ path: edit.finalPath, state: "symlink", targetHash: edit.newTargetHash, targetSize: Buffer.byteLength(edit.newTarget) });
    const logicalPost = pathPreimageRow(edit.logicalTargetFinalPath, edit.logicalTargetPreimage);
    const logicalTargetEdit = plan.symlinkTargetEdits.find((candidate) => candidate.sourcePath === edit.logicalTargetSourcePath && candidate.finalPath === edit.logicalTargetFinalPath);
    post.push(logicalPost.state === "symlink" && logicalTargetEdit ? { ...logicalPost, targetHash: logicalTargetEdit.newTargetHash, targetSize: Buffer.byteLength(logicalTargetEdit.newTarget) } : logicalPost.state === "file" && resultHashes.has(edit.logicalTargetFinalPath) ? { ...logicalPost, contentHash: resultHashes.get(edit.logicalTargetFinalPath), size: resultSizes.get(edit.logicalTargetFinalPath) ?? logicalPost.size } : logicalPost);
  }
  for (const [path, hash] of new Map(plan.edits.map((edit) => [edit.path, resultHashes.get(edit.path) ?? edit.preimage.contentHash]))) {
    const entry = entries.get(path) ?? inventory.entries.find((candidate) => candidate.normalizedPath === path);
    if (entry?.nodeKind === "file") {
      pre.push(entryStateRow(entry.sourcePath, entry));
      post.push({ path, state: "file", contentHash: hash, mode: entry.mode, size: resultSizes.get(path) ?? entry.size });
    }
  }
  for (const regeneration of plan.regenerations) {
    pre.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.preOutputs)) });
    post.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.outputs)) });
  }
  return { pre: affectedStateDigest(pre), post: affectedStateDigest(post) };
}
function taxonomyPlanDigest(plan) {
  const { planDigest: _planDigest, ...digestible } = plan;
  return sha256(canonicalJson(digestible));
}
function planTaxonomy(inventory, options) {
  if (inventory.taxonomySchemaVersion !== 7)
    throw new Error("Inventory taxonomy schemaVersion must be 7");
  if (inventory.sourceTreeDigest !== sourceTreeDigest(inventory.entries))
    throw new Error("Inventory sourceTreeDigest does not match inventory entries");
  const taxonomy = loadTaxonomy2({ repoRoot: inventory.repoRoot, taxonomyPath: inventory.taxonomyPath });
  const baselineCommit = options.baselineCommit.trim();
  if (!PLAN_COMMIT_ID.test(baselineCommit))
    throw new Error("baselineCommit must be a full lowercase SHA-1 commit ID");
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const embedded = planEmbeddedTicketRoots(inventory, taxonomy);
  const trailingRemovals = planTrailingEvidenceRemovals(inventory);
  const serializedRemovals = planSerializedEvidenceRemovals(inventory);
  const serializedSources = new Set(serializedRemovals.removals.map((entry) => entry.sourcePath));
  const evidenceRemovals = [...embedded.removals, ...trailingRemovals.filter((entry) => !serializedSources.has(entry.sourcePath)), ...serializedRemovals.removals].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
  const ownedRemovals = new Set(evidenceRemovals.map((entry) => entry.sourcePath));
  const embeddedPrefixes = embedded.roots.map((root) => root.sourceMetadataRoot);
  const isEmbedded = (path) => embeddedPrefixes.some((root) => path === root || path.startsWith(`${root}/`));
  const references = planMoveReferenceAuthority(inventory, taxonomy, options, embedded.roots, evidenceRemovals);
  const moves = references.moves;
  const generators = generatorPlanning(inventory, moves, references.edits, taxonomy, options);
  const symlinks = planSymlinkTargetEdits(inventory, taxonomy);
  const destinationAncestors = destinationAncestorPreimages(inventory.repoRoot, [...moves.map((entry) => entry.destinationPath), ...embedded.relocations.map((entry) => entry.destinationPath), ...symlinks.edits.map((entry) => entry.finalPath), ...generators.regenerations.flatMap((entry) => entry.outputRoots)]);
  const ownedSymlinks = new Set(symlinks.edits.map((entry) => entry.sourcePath));
  const unresolved = [
    ...inventory.violations.filter((entry) => entry.severity === "error" && !isEmbedded(entry.path) && generatorContractsForOutputPath(entry.path, taxonomy).length === 0 && !(entry.code === "symlink-absolute-target" && ownedSymlinks.has(entry.path)) && !(entry.code === "trailing-dot-or-space" && ownedRemovals.has(entry.path))),
    ...references.unresolved,
    ...generators.violations,
    ...symlinks.violations,
    ...embedded.violations,
    ...serializedRemovals.violations
  ];
  for (const group of references.collisionGroups)
    if (group.sources.some((source) => !isEmbedded(source)))
      unresolved.push(violation(`collision-${group.comparison}`, group.paths[0] ?? group.sources[0], `Normalization collision ${group.id}: ${group.sources.join(", ")}`));
  for (const digest of options.excludedTreeDigests) {
    if (digest.algorithm !== "sha256-merkle-v1")
      unresolved.push(violation("opaque-digest-algorithm", digest.relativeRoot, `Unsupported opaque digest algorithm ${digest.algorithm}`));
    if (!inventory.pathExclusions.includes(normalizeRelative(digest.relativeRoot)))
      unresolved.push(violation("opaque-digest-unregistered", digest.relativeRoot, "Opaque digest is not registered by taxonomy pathExclusions"));
  }
  const affected = plannedAffectedStateDigests(inventory, { moves, embeddedTicketRoots: embedded.roots, embeddedTicketRootRelocations: embedded.relocations, symlinkTargetEdits: symlinks.edits, evidenceRemovals, destinationAncestorPreimages: destinationAncestors, edits: references.edits, regenerations: generators.regenerations }, references.resultHashes, references.resultSizes);
  const provisionalBase = {
    schemaVersion: 2,
    taxonomySchemaVersion: 7,
    baselineCommit,
    scope: inventory.scope,
    sourceTreeDigest: inventory.sourceTreeDigest,
    excludedTreeDigests: [...options.excludedTreeDigests].sort((a, b) => a.relativeRoot.localeCompare(b.relativeRoot)),
    moves,
    embeddedTicketRoots: embedded.roots,
    embeddedTicketRootRelocations: embedded.relocations,
    symlinkTargetEdits: symlinks.edits,
    evidenceRemovals,
    destinationAncestorPreimages: destinationAncestors,
    edits: [...references.edits].sort(referenceEditCompare),
    regenerations: generators.regenerations,
    unresolved: stableViolations(unresolved),
    expectedAffectedPreStateDigest: affected.pre,
    expectedPostStateDigest: affected.post,
    planDigest: ""
  };
  const provisional = { ...provisionalBase, unresolved: stableViolations([...provisionalBase.unresolved, ...projectionStaleViolations(inventory.repoRoot, provisionalBase, taxonomy, inventory)]) };
  const plan = { ...provisional, planDigest: taxonomyPlanDigest(provisional) };
  const plannedOperations = moves.length + embedded.roots.length + embedded.relocations.length + symlinks.edits.length + evidenceRemovals.length + references.edits.length + generators.regenerations.length;
  report(options.progress, "plan", "complete", plannedOperations, plannedOperations);
  return plan;
}
function noFollowMerkleNode(path, counts, format, excluded = new Set) {
  const stat = lstatSync(path);
  const numericMode = stat.mode & 4095;
  const mode = numericMode.toString(8);
  if (stat.isSymbolicLink()) {
    counts.symlinks++;
    const target = readlinkSync(path);
    return format === "opaque-v1" ? sha256(`symlink\x00${mode}\x00${target}`) : sha256(canonicalJson({ kind: "symlink", mode: numericMode, target }));
  }
  if (stat.isFile()) {
    counts.files++;
    return sha256(Buffer.concat([Buffer.from(format === "opaque-v1" ? `file\x00${mode}\x00` : `file\x00${numericMode}\x00${stat.size}\x00`), readFileSync2(path)]));
  }
  if (stat.isDirectory()) {
    counts.directories++;
    const children = readdirSync2(path).sort((left, right) => Buffer.from(left).compare(Buffer.from(right))).filter((name) => !excluded.has(join2(path, name))).map((name) => ({ name: Buffer.from(name).toString("hex"), digest: noFollowMerkleNode(join2(path, name), counts, format, excluded) }));
    return format === "opaque-v1" ? sha256(`directory\x00${mode}\x00${children.map((child) => `${child.name}\x00${child.digest}`).join("\x00")}`) : sha256(canonicalJson({ kind: "directory", mode: numericMode, children }));
  }
  counts.others++;
  return format === "opaque-v1" ? sha256(`other\x00${mode}\x00${stat.size}`) : sha256(canonicalJson({ kind: "other", mode: numericMode, size: stat.size }));
}
function opaqueNodeDigest(path, counts) {
  return noFollowMerkleNode(path, counts, "opaque-v1");
}
function noFollowNodeDigest(path, counts, excluded = new Set) {
  return noFollowMerkleNode(path, counts, "path-state-v1", excluded);
}
function noFollowTreeDigestExcluding(root, relativeRoot, excludedPaths) {
  const path = absolutePath(root, normalizeRelative(relativeRoot));
  const excluded = new Set(excludedPaths.map((entry) => absolutePath(root, entry)));
  const counts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = noFollowNodeDigest(path, counts, excluded);
  return { algorithm: "sha256-no-follow-merkle-v1", digest, ...counts };
}
function noFollowTreeDigest(root, relativeRoot) {
  const path = absolutePath(root, normalizeRelative(relativeRoot));
  const counts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = noFollowNodeDigest(path, counts);
  return { algorithm: "sha256-no-follow-merkle-v1", digest, ...counts };
}
function opaqueTreeDigest(root, relativeRoot) {
  const normalized = normalizeRelative(relativeRoot);
  const path = absolutePath(root, normalized);
  const counts = { files: 0, directories: 0, symlinks: 0, others: 0 };
  const digest = opaqueNodeDigest(path, counts);
  return { algorithm: "sha256-merkle-v1", relativeRoot: normalized, digest, ...counts };
}
function repositoryHead(repoRoot) {
  return execFileSync("git", ["rev-parse", "HEAD"], { cwd: repoRoot, encoding: "utf8" }).trim();
}
function verifyTaxonomy(options) {
  const inventory = inventoryTaxonomy(options);
  const plan = planTaxonomy(inventory, {
    baselineCommit: options.baselineCommit ?? repositoryHead(inventory.repoRoot),
    excludedTreeDigests: options.excludedTreeDigests ?? [],
    cancelFile: options.cancelFile,
    progress: options.progress
  });
  const violations = [...plan.unresolved];
  for (const move of plan.moves)
    violations.push(violation("normalization-move-required", move.sourcePath, `Path must move to ${move.destinationPath}`));
  for (const relocation of plan.embeddedTicketRootRelocations)
    violations.push(violation("embedded-ticket-root-relocation-required", relocation.sourcePath, `Embedded ticket evidence must relocate to ${relocation.destinationPath}`));
  for (const edit of plan.symlinkTargetEdits)
    violations.push(violation("symlink-target-edit-required", edit.sourcePath, `Absolute repository-local symlink target must become ${edit.newTarget}`));
  for (const removal of plan.evidenceRemovals)
    violations.push(violation("evidence-removal-required", removal.sourcePath, "Redundant evidence must be disposition-staged"));
  for (const edit of plan.edits)
    violations.push(violation("reference-edit-required", edit.path, `Structured reference must change at ${edit.structuredLocation}`));
  const stable = stableViolations(violations);
  const clean = stable.every((entry) => entry.severity !== "error");
  report(options.progress, "verify", "complete", stable.length, stable.length);
  return { inventory, plan, violations: stable, clean };
}
function fsyncDirectory(path) {
  try {
    const directory = openSync(path, "r");
    try {
      fsyncSync(directory);
    } finally {
      closeSync(directory);
    }
  } catch (error) {
    if (!["EINVAL", "ENOTSUP", "EISDIR"].includes(String(error.code)))
      throw error;
  }
}
function fsyncFile(path) {
  const file = openSync(path, "r");
  try {
    fsyncSync(file);
  } finally {
    closeSync(file);
  }
}
function durableRename(source, destination) {
  renameSync(source, destination);
  fsyncDirectory(dirname2(source));
  if (dirname2(destination) !== dirname2(source))
    fsyncDirectory(dirname2(destination));
}
function durableSymlink(target, path, type) {
  symlinkSync(target, path, type);
  fsyncDirectory(dirname2(path));
}
function durableRemove(path, recursive = false) {
  rmSync(path, { recursive, force: true });
  fsyncDirectory(dirname2(path));
}
function durablySyncGeneratorRecords(repoRoot, records) {
  for (const record2 of [...records].sort((left, right) => right.path.split("/").length - left.path.split("/").length || generatorPathCompare(left.path, right.path))) {
    const path = absolutePath(repoRoot, record2.path);
    if (record2.nodeKind === "file")
      fsyncFile(path);
    else if (record2.nodeKind === "directory")
      fsyncDirectory(path);
    else
      fsyncDirectory(dirname2(path));
  }
}
function canonicalJsonFile(path) {
  try {
    const bytes = readFileSync2(path, "utf8");
    return bytes === `${canonicalJson(JSON.parse(bytes))}
`;
  } catch {
    return false;
  }
}
function publishCanonicalJsonCandidate(container, finalName, previousName, value, preparationName) {
  mkdirSync(container, { recursive: true });
  const root = join2(container, preparationName(process.pid, randomUUID()));
  const leaf = join2(root, finalName);
  mkdirSync(root);
  fsyncDirectory(container);
  const descriptor = openSync(leaf, "wx", 384);
  try {
    writeFileSync(descriptor, `${canonicalJson(value)}
`, "utf8");
    fsyncSync(descriptor);
  } finally {
    closeSync(descriptor);
  }
  fsyncDirectory(root);
  const final = join2(container, finalName);
  const previous = join2(root, previousName), finalStat = lstatOrNull(final);
  if (finalStat && (!finalStat.isFile() || finalStat.isSymbolicLink()))
    throw new Error(`Canonical JSON destination is occupied: ${final}`);
  if (finalStat && readFileSync2(final).equals(readFileSync2(leaf)))
    durableRemove(leaf);
  else {
    if (finalStat)
      durableRename(final, previous);
    durableRename(leaf, final);
    if (lstatOrNull(previous))
      durableRemove(previous);
  }
  durableRemove(root, true);
}
function recoverCanonicalJsonCandidates(container, finalName, previousName, preparationName, validate, serialized, validateOnly = false) {
  const stat = lstatOrNull(container);
  if (!stat)
    return;
  if (!stat.isDirectory() || stat.isSymbolicLink())
    throw new Error(`Canonical JSON container must be a no-follow directory: ${container}`);
  const actions = [];
  const final = join2(container, finalName);
  let prospective = lstatOrNull(final) ? final : undefined;
  for (const name of readdirSync2(container).sort(generatorPathCompare)) {
    if (name === finalName)
      continue;
    const match = /^write-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[2]) || name !== preparationName(Number.parseInt(match[1], 10), match[2]))
      throw new Error(`Canonical JSON write preparation name is invalid: ${name}`);
    const root = join2(container, name), rootStat = lstatOrNull(root);
    if (!rootStat?.isDirectory() || rootStat.isSymbolicLink())
      throw new Error(`Canonical JSON write preparation must be a no-follow directory: ${name}`);
    const children = readdirSync2(root).sort(generatorPathCompare);
    if (children.length > 2 || children.some((child) => child !== finalName && child !== previousName))
      throw new Error(`Canonical JSON write preparation contains unexpected evidence: ${name}`);
    const previous = children.includes(previousName) ? join2(root, previousName) : undefined;
    if (previous) {
      const previousStat = lstatOrNull(previous);
      if (!previousStat?.isFile() || previousStat.isSymbolicLink() || !canonicalJsonFile(previous))
        throw new Error(`Canonical JSON previous evidence is invalid: ${previous}`);
      validate(previous);
    }
    if (children.length === 0) {
      if (!serialized && transactionLeaseProcessIsAlive(Number.parseInt(match[1], 10)))
        throw new Error(`Canonical JSON write preparation is active for pid ${match[1]}`);
      actions.push({ root, exchange: false, publish: false });
      continue;
    }
    const leaf = children.includes(finalName) ? join2(root, finalName) : undefined, leafStat = leaf ? lstatOrNull(leaf) : undefined;
    if (leaf && (!leafStat?.isFile() || leafStat.isSymbolicLink()))
      throw new Error(`Canonical JSON write candidate must be a regular no-follow file: ${leaf}`);
    if (leaf && !canonicalJsonFile(leaf)) {
      if (previous)
        throw new Error(`Canonical JSON exchanged candidate is invalid: ${leaf}`);
      if (!serialized && transactionLeaseProcessIsAlive(Number.parseInt(match[1], 10)))
        throw new Error(`Canonical JSON write preparation is active for pid ${match[1]}`);
      actions.push({ root, exchange: false, publish: false });
      continue;
    }
    if (leaf)
      validate(leaf);
    const finalStat = lstatOrNull(final);
    if (finalStat && (!finalStat.isFile() || finalStat.isSymbolicLink() || !canonicalJsonFile(final)))
      throw new Error(`Canonical JSON destination is invalid: ${final}`);
    if (leaf && previous && finalStat)
      throw new Error(`Canonical JSON exchange has simultaneous previous and durable destinations: ${root}`);
    if (!leaf && previous && !finalStat)
      throw new Error(`Canonical JSON previous-only state has no durable destination: ${root}`);
    if (!leaf && previous)
      validate(final);
    const equal = Boolean(leaf && finalStat && readFileSync2(final).equals(readFileSync2(leaf)));
    const exchange = Boolean(leaf && finalStat && !equal && !previous);
    const publish = Boolean(leaf && !finalStat);
    prospective = leaf && (exchange || publish) ? leaf : finalStat ? final : prospective;
    actions.push({ root, candidate: leaf, previous, exchange, publish });
  }
  if (actions.length > 1)
    throw new Error(`Canonical JSON container has duplicate write preparations: ${container}`);
  if (!validateOnly)
    for (const action of actions) {
      const previous = join2(action.root, previousName);
      if (action.exchange)
        durableRename(final, previous);
      if (action.publish || action.exchange)
        durableRename(action.candidate, final);
      if (lstatOrNull(previous))
        durableRemove(previous);
      durableRemove(action.root, true);
    }
  return validateOnly ? prospective : lstatOrNull(final) ? final : undefined;
}
var TRANSACTION_LEASE_TOKEN = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u;
function transactionLeaseSnapshot(record2) {
  return { schemaVersion: 1, planDigest: record2.planDigest, attemptOrdinal: record2.attemptOrdinal, token: record2.token, pid: record2.pid };
}
function parseTransactionLease(path, planDigest, attemptOrdinal, expectedToken) {
  const file = lstatOrNull(path);
  if (!file?.isFile() || file.isSymbolicLink())
    throw new Error(`Transaction lease metadata must be a regular no-follow file: ${path}`);
  const bytes = readFileSync2(path, "utf8");
  const value = planRecord(JSON.parse(bytes), "transaction lease", ["schemaVersion", "planDigest", "attemptOrdinal", "token", "pid"]);
  if (value.schemaVersion !== 1 || value.planDigest !== planDigest || value.attemptOrdinal !== attemptOrdinal || typeof value.token !== "string" || !TRANSACTION_LEASE_TOKEN.test(value.token) || expectedToken !== undefined && value.token !== expectedToken || !Number.isSafeInteger(value.pid) || value.pid < 1)
    throw new Error(`Transaction lease identity is invalid: ${path}`);
  const record2 = transactionLeaseSnapshot(value);
  if (bytes !== `${canonicalJson(record2)}
`)
    throw new Error(`Transaction lease is not canonical JSON: ${path}`);
  return record2;
}
function readTransactionLease(root, filename, planDigest, attemptOrdinal, expectedToken) {
  const stat = lstatOrNull(root);
  if (!stat?.isDirectory() || stat.isSymbolicLink())
    throw new Error(`Transaction lease must be a direct no-follow directory: ${root}`);
  if (canonicalJson(readdirSync2(root).sort(generatorPathCompare)) !== canonicalJson([filename]))
    throw new Error(`Transaction lease contains unexpected evidence: ${root}`);
  return parseTransactionLease(join2(root, filename), planDigest, attemptOrdinal, expectedToken);
}
function transactionLeaseProcessIsAlive(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    const code = String(error.code);
    if (code === "EPERM")
      return true;
    if (code === "ESRCH")
      return false;
    throw error;
  }
}
function acquireTransactionLease(repoRoot, attemptRelative, backupRelative, leaseDirectory, leasePreparationName, jsonWritePreparationName, filename, previousName, planDigest, attemptOrdinal, beforePublish) {
  const attemptRoot = absolutePath(repoRoot, attemptRelative);
  const backupRoot = absolutePath(repoRoot, backupRelative);
  const leaseRoot = join2(attemptRoot, leaseDirectory);
  const backup = lstatOrNull(backupRoot);
  if (!backup?.isDirectory() || backup.isSymbolicLink())
    throw new Error(`Transaction backup authority is unavailable for lease acquisition: ${backupRelative}`);
  const leasePreparations = () => {
    const rows = [];
    for (const name of readdirSync2(backupRoot).sort(generatorPathCompare)) {
      const match = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(splitLeadingEmoji(name).rest);
      if (!match)
        continue;
      const root = join2(backupRoot, name), pid = Number.parseInt(match[1], 10), token = match[2], state = match[3];
      if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== leasePreparationName(pid, token, state))
        throw new Error(`Transaction lease preparation name is invalid: ${name}`);
      const stat = lstatOrNull(root);
      if (!stat?.isDirectory() || stat.isSymbolicLink())
        throw new Error(`Transaction lease preparation must be a no-follow directory: ${name}`);
      recoverCanonicalJsonCandidates(root, filename, previousName, jsonWritePreparationName, (path) => {
        const candidate = parseTransactionLease(path, planDigest, attemptOrdinal, token);
        if (candidate.pid !== pid)
          throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
      }, false, true);
      const canonical = join2(root, filename);
      const record3 = lstatOrNull(canonical) ? parseTransactionLease(canonical, planDigest, attemptOrdinal, token) : undefined;
      if (record3 && record3.pid !== pid)
        throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
      if (transactionLeaseProcessIsAlive(pid))
        throw new Error(`Transaction attempt lease preparation is active for pid ${pid}`);
      rows.push({ root, pid, token, state, record: record3 });
    }
    return rows;
  };
  leasePreparations();
  const current = lstatOrNull(leaseRoot);
  if (current) {
    const record3 = readTransactionLease(leaseRoot, filename, planDigest, attemptOrdinal);
    if (transactionLeaseProcessIsAlive(record3.pid))
      throw new Error(`Transaction attempt is leased by active pid ${record3.pid}`);
    const stale = join2(backupRoot, leasePreparationName(record3.pid, record3.token, "stale"));
    if (lstatOrNull(stale))
      throw new Error(`Transaction attempt contains duplicate stale lease evidence: ${record3.token}`);
    durableRename(leaseRoot, stale);
  }
  const record2 = { schemaVersion: 1, planDigest, attemptOrdinal, token: randomUUID(), pid: process.pid };
  const preparing = join2(backupRoot, leasePreparationName(record2.pid, record2.token, "preparing"));
  if (lstatOrNull(preparing))
    throw new Error(`Transaction lease token collision: ${record2.token}`);
  mkdirSync(preparing);
  fsyncDirectory(backupRoot);
  try {
    beforePublish?.();
    publishCanonicalJsonCandidate(preparing, filename, previousName, record2, jsonWritePreparationName);
    if (lstatOrNull(leaseRoot))
      throw new Error("Transaction lease acquisition fence found concurrent canonical lease evidence");
    readTransactionLease(preparing, filename, planDigest, attemptOrdinal, record2.token);
    beforePublish?.();
    durableRename(preparing, leaseRoot);
    beforePublish?.();
    for (const preparation of leasePreparations()) {
      recoverCanonicalJsonCandidates(preparation.root, filename, previousName, jsonWritePreparationName, (path) => {
        const candidate = parseTransactionLease(path, planDigest, attemptOrdinal, preparation.token);
        if (candidate.pid !== preparation.pid)
          throw new Error(`Transaction lease preparation pid is invalid: ${basename2(preparation.root)}`);
      }, true);
      const canonical = join2(preparation.root, filename);
      if (lstatOrNull(canonical)) {
        const stale = parseTransactionLease(canonical, planDigest, attemptOrdinal, preparation.token);
        if (stale.pid !== preparation.pid)
          throw new Error(`Transaction lease preparation pid is invalid: ${basename2(preparation.root)}`);
      }
      durableRemove(preparation.root, true);
    }
  } catch (error) {
    if (lstatOrNull(leaseRoot)) {
      const owned = readTransactionLease(leaseRoot, filename, planDigest, attemptOrdinal, record2.token);
      if (owned.pid !== record2.pid)
        throw new Error("Transaction lease ownership changed during failed acquisition");
      durableRemove(leaseRoot, true);
    }
    if (lstatOrNull(preparing)) {
      const stat = lstatOrNull(preparing);
      if (!stat?.isDirectory() || stat.isSymbolicLink())
        throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      recoverCanonicalJsonCandidates(preparing, filename, previousName, jsonWritePreparationName, (path) => {
        const owned = parseTransactionLease(path, planDigest, attemptOrdinal, record2.token);
        if (owned.pid !== record2.pid)
          throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      }, true, true);
      const canonical = join2(preparing, filename);
      if (lstatOrNull(canonical)) {
        const owned = parseTransactionLease(canonical, planDigest, attemptOrdinal, record2.token);
        if (owned.pid !== record2.pid)
          throw new Error("Transaction lease preparation ownership changed during failed acquisition");
      }
      durableRemove(preparing, true);
    }
    throw new Error(`Transaction attempt lease acquisition failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  return { root: leaseRoot, filename, record: record2 };
}
function releaseTransactionLease(handle) {
  const current = readTransactionLease(handle.root, handle.filename, handle.record.planDigest, handle.record.attemptOrdinal, handle.record.token);
  if (current.pid !== handle.record.pid)
    throw new Error("Transaction lease ownership changed before release");
  durableRemove(handle.root, true);
}
function journalSnapshot(journal) {
  return {
    schemaVersion: 2,
    revision: journal.revision,
    planDigest: journal.planDigest,
    attemptOrdinal: journal.attemptOrdinal,
    state: journal.state,
    stagingRoot: journal.stagingRoot,
    backupRoot: journal.backupRoot,
    preparedMoveIds: [...journal.preparedMoveIds].sort(generatorPathCompare),
    stagedMoveIds: [...journal.stagedMoveIds].sort(generatorPathCompare),
    installedMoveIds: [...journal.installedMoveIds].sort(generatorPathCompare),
    preparedEmbeddedRelocationIds: [...journal.preparedEmbeddedRelocationIds].sort(generatorPathCompare),
    stagedEmbeddedRelocationIds: [...journal.stagedEmbeddedRelocationIds].sort(generatorPathCompare),
    installedEmbeddedRelocationIds: [...journal.installedEmbeddedRelocationIds].sort(generatorPathCompare),
    preparedEvidenceRemovalIds: [...journal.preparedEvidenceRemovalIds].sort(generatorPathCompare),
    stagedEvidenceRemovalIds: [...journal.stagedEvidenceRemovalIds].sort(generatorPathCompare),
    preparedEmbeddedRootIds: [...journal.preparedEmbeddedRootIds].sort(generatorPathCompare),
    stagedEmbeddedRootIds: [...journal.stagedEmbeddedRootIds].sort(generatorPathCompare),
    preparedSymlinkTargetEditIds: [...journal.preparedSymlinkTargetEditIds].sort(generatorPathCompare),
    stagedSymlinkTargetEditIds: [...journal.stagedSymlinkTargetEditIds].sort(generatorPathCompare),
    installedSymlinkTargetEditIds: [...journal.installedSymlinkTargetEditIds].sort(generatorPathCompare),
    appliedEditPaths: [...journal.appliedEditPaths].sort(generatorPathCompare),
    startedRegenerationIds: [...journal.startedRegenerationIds].sort(generatorPathCompare),
    completedRegenerationIds: [...journal.completedRegenerationIds].sort(generatorPathCompare),
    backups: Object.fromEntries(Object.entries(journal.backups).sort(([a], [b]) => generatorPathCompare(a, b))),
    error: journal.error
  };
}
function persistJournal(repoRoot, path, journal) {
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const walRoot = join2(stageRoot, journal.journalWriteDirectory);
  const walPath = join2(walRoot, basename2(path));
  const walStat = lstatOrNull(walRoot);
  if (walStat && (!walStat.isDirectory() || walStat.isSymbolicLink() || readdirSync2(walRoot).length > 0))
    throw new Error(`Taxonomy journal WAL is occupied: ${walRoot}`);
  mkdirSync(walRoot, { recursive: true });
  const next = { ...journal, revision: journal.revision + 1 };
  publishCanonicalJsonCandidate(walRoot, basename2(walPath), journal.jsonPreviousName, journalSnapshot(next), journal.jsonWritePreparationName);
  durableRename(walPath, path);
  journal.revision = next.revision;
  durableRemove(walRoot, true);
}
function readJournal(path, journalWriteDirectory, jsonWritePreparationName, jsonPreviousName) {
  const bytes = readFileSync2(path, "utf8");
  const parsed = JSON.parse(bytes);
  const value = planRecord(parsed, "taxonomy journal", ["schemaVersion", "revision", "planDigest", "attemptOrdinal", "state", "stagingRoot", "backupRoot", "preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds", "backups"], ["error"]);
  const arrays = ["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds"];
  const states = ["prepared", "staging", "disposing", "installing", "retargeting", "editing", "regenerating", "verifying", "committed", "rolling-back", "rolled-back"];
  if (value.schemaVersion !== 2 || !Number.isSafeInteger(value.revision) || value.revision < 0 || typeof value.planDigest !== "string" || !PLAN_HASH.test(value.planDigest) || typeof value.attemptOrdinal !== "string" || !/^[0-9]{6}$/u.test(value.attemptOrdinal) || !states.includes(value.state) || typeof value.stagingRoot !== "string" || typeof value.backupRoot !== "string" || !arrays.every((key) => Array.isArray(value[key])) || !value.backups || typeof value.backups !== "object" || value.error !== undefined && typeof value.error !== "string")
    throw new Error(`Invalid taxonomy journal at ${path}`);
  planPath(value.stagingRoot, "taxonomy journal stagingRoot");
  planPath(value.backupRoot, "taxonomy journal backupRoot");
  for (const key of arrays) {
    const ids = value[key];
    const pattern = key === "appliedEditPaths" ? undefined : PLAN_OPERATION_ID;
    const parsedIds = ids.map((entry, index) => pattern ? planString(entry, `taxonomy journal ${key}[${index}]`, pattern) : planPath(entry, `taxonomy journal ${key}[${index}]`));
    if (new Set(parsedIds).size !== parsedIds.length || parsedIds.some((entry, index) => index > 0 && Buffer.from(parsedIds[index - 1]).compare(Buffer.from(entry)) >= 0))
      throw new Error(`Taxonomy journal ${key} must be unique and bytewise sorted`);
  }
  for (const [logicalPath, backup] of Object.entries(value.backups)) {
    planPath(logicalPath, "taxonomy journal backup path");
    const candidate = planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind"], ["backupPath", "contentHash", "mode", "size", "target", "targetHash"]);
    if (candidate.kind === "absent")
      planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind"]);
    else if (candidate.kind === "file") {
      planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind", "backupPath", "contentHash", "mode", "size"]);
      planPath(candidate.backupPath, `taxonomy journal backup ${logicalPath}.backupPath`);
      planString(candidate.contentHash, `taxonomy journal backup ${logicalPath}.contentHash`, PLAN_HASH);
      planInteger(candidate.mode, `taxonomy journal backup ${logicalPath}.mode`, 4095);
      planInteger(candidate.size, `taxonomy journal backup ${logicalPath}.size`);
    } else if (candidate.kind === "symlink") {
      planRecord(backup, `taxonomy journal backup ${logicalPath}`, ["kind", "target", "targetHash", "mode", "size"]);
      const target = planString(candidate.target, `taxonomy journal backup ${logicalPath}.target`);
      if (planString(candidate.targetHash, `taxonomy journal backup ${logicalPath}.targetHash`, PLAN_HASH) !== sha256(target) || planInteger(candidate.mode, `taxonomy journal backup ${logicalPath}.mode`, 4095) < 0 || planInteger(candidate.size, `taxonomy journal backup ${logicalPath}.size`) !== Buffer.byteLength(target))
        throw new Error(`Taxonomy journal backup ${logicalPath} symlink preimage changed`);
    } else
      throw new Error(`Taxonomy journal backup ${logicalPath}.kind is invalid`);
  }
  const journal = {
    schemaVersion: 2,
    revision: value.revision,
    planDigest: value.planDigest,
    attemptOrdinal: value.attemptOrdinal,
    state: value.state,
    stagingRoot: value.stagingRoot,
    backupRoot: value.backupRoot,
    journalWriteDirectory,
    jsonWritePreparationName,
    jsonPreviousName,
    preparedMoveIds: [...value.preparedMoveIds],
    stagedMoveIds: [...value.stagedMoveIds],
    installedMoveIds: [...value.installedMoveIds],
    preparedEmbeddedRelocationIds: [...value.preparedEmbeddedRelocationIds],
    stagedEmbeddedRelocationIds: [...value.stagedEmbeddedRelocationIds],
    installedEmbeddedRelocationIds: [...value.installedEmbeddedRelocationIds],
    preparedEvidenceRemovalIds: [...value.preparedEvidenceRemovalIds],
    stagedEvidenceRemovalIds: [...value.stagedEvidenceRemovalIds],
    preparedEmbeddedRootIds: [...value.preparedEmbeddedRootIds],
    stagedEmbeddedRootIds: [...value.stagedEmbeddedRootIds],
    preparedSymlinkTargetEditIds: [...value.preparedSymlinkTargetEditIds],
    stagedSymlinkTargetEditIds: [...value.stagedSymlinkTargetEditIds],
    installedSymlinkTargetEditIds: [...value.installedSymlinkTargetEditIds],
    appliedEditPaths: [...value.appliedEditPaths],
    startedRegenerationIds: [...value.startedRegenerationIds],
    completedRegenerationIds: [...value.completedRegenerationIds],
    backups: { ...value.backups },
    error: value.error
  };
  if (bytes !== `${canonicalJson(journalSnapshot(journal))}
`)
    throw new Error(`Taxonomy journal at ${path} is not canonical JSON`);
  const empty = (keys) => keys.every((key) => Array.isArray(journal[key]) && journal[key].length === 0);
  const moveFuture = ["installedMoveIds"];
  const disposalFuture = ["preparedEmbeddedRootIds", "stagedEmbeddedRootIds"];
  const relocationFuture = ["installedEmbeddedRelocationIds"];
  const linkFuture = ["preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds"];
  const editFuture = ["appliedEditPaths"];
  const regenerationFuture = ["startedRegenerationIds", "completedRegenerationIds"];
  if (journal.state === "prepared" && (!empty(["preparedMoveIds", "stagedMoveIds", ...moveFuture, "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", ...relocationFuture, "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", ...disposalFuture, ...linkFuture, ...editFuture, ...regenerationFuture]) || Object.keys(journal.backups).length > 0) || journal.state === "staging" && !empty([...moveFuture, ...disposalFuture, ...relocationFuture, ...linkFuture, ...editFuture, ...regenerationFuture]) || journal.state === "disposing" && !empty([...moveFuture, ...relocationFuture, ...linkFuture, ...editFuture, ...regenerationFuture]) || journal.state === "installing" && !empty([...linkFuture, ...editFuture, ...regenerationFuture]) || journal.state === "retargeting" && !empty([...editFuture, ...regenerationFuture]) || journal.state === "editing" && !empty(regenerationFuture) || journal.state === "rolled-back" && !empty(["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds"]))
    throw new Error(`Taxonomy journal state ${journal.state} contains operations from an impossible phase`);
  return journal;
}
var JOURNAL_OPERATION_ARRAYS = ["preparedMoveIds", "stagedMoveIds", "installedMoveIds", "preparedEmbeddedRelocationIds", "stagedEmbeddedRelocationIds", "installedEmbeddedRelocationIds", "preparedEvidenceRemovalIds", "stagedEvidenceRemovalIds", "preparedEmbeddedRootIds", "stagedEmbeddedRootIds", "preparedSymlinkTargetEditIds", "stagedSymlinkTargetEditIds", "installedSymlinkTargetEditIds", "appliedEditPaths", "startedRegenerationIds", "completedRegenerationIds"];
function assertJournalTransition(current, next) {
  const order = ["prepared", "staging", "disposing", "installing", "retargeting", "editing", "regenerating", "verifying", "committed"];
  const currentRank = order.indexOf(current.state), nextRank = order.indexOf(next.state);
  const transition = current.state === "committed" || current.state === "rolled-back" ? next.state === current.state : current.state === "rolling-back" ? next.state === "rolling-back" || next.state === "rolled-back" : next.state === current.state || nextRank === currentRank + 1 || next.state === "rolling-back";
  if (!transition)
    throw new Error(`Taxonomy journal WAL has an invalid ${current.state} -> ${next.state} transition`);
  const contains = (parent, child) => child.every((entry) => parent.includes(entry));
  const currentTerminal = current.state === "committed" || current.state === "rolled-back";
  const rollingBack = current.state === "rolling-back";
  for (const key of JOURNAL_OPERATION_ARRAYS) {
    const valid = currentTerminal ? canonicalJson(current[key]) === canonicalJson(next[key]) : rollingBack ? contains(current[key], next[key]) : contains(next[key], current[key]);
    if (!valid)
      throw new Error(`Taxonomy journal WAL ${key} is not a legal monotonic transition`);
  }
  const currentBackupKeys = Object.keys(current.backups);
  const backupsValid = currentTerminal ? canonicalJson(current.backups) === canonicalJson(next.backups) : currentBackupKeys.every((key) => canonicalJson(current.backups[key]) === canonicalJson(next.backups[key]));
  if (!backupsValid)
    throw new Error("Taxonomy journal WAL backups are not a legal monotonic transition");
  if (currentTerminal && next.error !== current.error)
    throw new Error("Taxonomy journal WAL cannot alter terminal error evidence");
}
function reconcileJournalWal(repoRoot, path, current, plan, taxonomy, validateOnly = false) {
  const walRoot = join2(absolutePath(repoRoot, current.stagingRoot), current.journalWriteDirectory);
  const stat = lstatOrNull(walRoot);
  if (!stat)
    return current;
  if (!stat.isDirectory() || stat.isSymbolicLink())
    throw new Error(`Taxonomy journal WAL must be a direct no-follow directory: ${walRoot}`);
  const prospectiveWal = recoverCanonicalJsonCandidates(walRoot, basename2(path), current.jsonPreviousName, current.jsonWritePreparationName, (candidatePath) => {
    const candidate = readJournal(candidatePath, current.journalWriteDirectory, current.jsonWritePreparationName, current.jsonPreviousName);
    if (candidate.revision !== current.revision + 1 || candidate.planDigest !== current.planDigest || candidate.attemptOrdinal !== current.attemptOrdinal || candidate.stagingRoot !== current.stagingRoot || candidate.backupRoot !== current.backupRoot)
      throw new Error("Taxonomy journal WAL candidate identity or revision differs from its durable attempt");
    assertJournalTransition(current, candidate);
    assertJournalPlanMembership(plan, candidate);
    assertJournalPhaseMembership(plan, candidate);
    assertJournalBackupAuthority(plan, candidate);
    assertActiveTransactionEvidence(repoRoot, plan, candidate, false);
    const changed = candidate.state === "rolling-back" || candidate.state === "rolled-back" ? reconcileRollbackTuples(repoRoot, plan, candidate, taxonomy) : validateResumeTuples(repoRoot, plan, candidate, taxonomy);
    if (changed)
      throw new Error("Taxonomy journal WAL candidate does not exactly match its durable filesystem tuples");
  }, true, validateOnly);
  const children = readdirSync2(walRoot).sort(generatorPathCompare);
  if (!prospectiveWal) {
    if (!validateOnly)
      durableRemove(walRoot, true);
    return current;
  }
  if (!validateOnly && canonicalJson(children) !== canonicalJson([basename2(path)]))
    throw new Error(`Taxonomy journal WAL contains unexpected evidence: ${walRoot}`);
  const walPath = validateOnly ? prospectiveWal : join2(walRoot, basename2(path));
  const walStat = lstatOrNull(walPath);
  if (!walStat?.isFile() || walStat.isSymbolicLink())
    throw new Error(`Taxonomy journal WAL snapshot is not a regular no-follow file: ${walPath}`);
  const next = readJournal(walPath, current.journalWriteDirectory, current.jsonWritePreparationName, current.jsonPreviousName);
  if (next.revision !== current.revision + 1 || next.planDigest !== current.planDigest || next.attemptOrdinal !== current.attemptOrdinal || next.stagingRoot !== current.stagingRoot || next.backupRoot !== current.backupRoot)
    throw new Error("Taxonomy journal WAL identity or revision differs from its durable attempt");
  assertJournalTransition(current, next);
  assertJournalPlanMembership(plan, next);
  assertJournalPhaseMembership(plan, next);
  assertJournalBackupAuthority(plan, next);
  assertActiveTransactionEvidence(repoRoot, plan, next, false);
  const tupleChanged = next.state === "rolling-back" || next.state === "rolled-back" ? reconcileRollbackTuples(repoRoot, plan, next, taxonomy) : validateResumeTuples(repoRoot, plan, next, taxonomy);
  if (tupleChanged)
    throw new Error("Taxonomy journal WAL does not exactly match its durable filesystem tuples");
  if (!validateOnly) {
    durableRename(walPath, path);
    durableRemove(walRoot, true);
  }
  return next;
}
function lstatOrNull(path) {
  try {
    return lstatSync(path);
  } catch (error) {
    if (error.code === "ENOENT")
      return null;
    throw error;
  }
}
function hashPath(path) {
  const stat = lstatSync(path);
  if (stat.isSymbolicLink())
    return sha256(readlinkSync(path));
  if (!stat.isFile())
    throw new Error(`Expected file or symlink at ${path}`);
  return sha256(readFileSync2(path));
}
function inventoryLeafPreimage(entry) {
  if (entry.nodeKind === "file")
    return { nodeKind: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size };
  if (entry.nodeKind !== "symlink" || entry.symlinkTarget === undefined || sha256(entry.symlinkTarget) !== entry.contentHash || Buffer.byteLength(entry.symlinkTarget) !== entry.size)
    throw new Error("Inventory leaf lacks exact no-follow file/symlink authority");
  return { nodeKind: "symlink", contentHash: entry.contentHash, mode: entry.mode, size: entry.size, target: entry.symlinkTarget };
}
function leafPreimage(path) {
  const stat = lstatSync(path);
  if (!stat.isFile() && !stat.isSymbolicLink())
    throw new Error(`Expected no-follow leaf at ${path}`);
  const nodeKind = stat.isSymbolicLink() ? "symlink" : "file";
  const bytes = nodeKind === "symlink" ? Buffer.from(readlinkSync(path)) : readFileSync2(path);
  const core = { contentHash: sha256(bytes), mode: stat.mode & 4095, size: bytes.byteLength };
  return nodeKind === "symlink" ? { nodeKind, ...core, target: bytes.toString() } : { nodeKind, ...core };
}
function leafPathPreimage(path) {
  const leaf = leafPreimage(path);
  return leaf.nodeKind === "symlink" ? { state: "symlink", contentHash: leaf.contentHash, mode: leaf.mode, size: leaf.size, target: leaf.target } : { state: "file", contentHash: leaf.contentHash, mode: leaf.mode, size: leaf.size };
}
function assertLeafPreimage(repoRoot, path, expected) {
  const absolute = absolutePath(repoRoot, path);
  if (!lstatOrNull(absolute) || canonicalJson(leafPreimage(absolute)) !== canonicalJson(expected))
    throw new Error(`Disposition preimage changed: ${path}`);
}
function retargetedMovePreimage(move, edit) {
  if (!edit)
    return move.sourcePreimage;
  if (move.sourcePreimage.nodeKind !== "symlink" || edit.oldTarget !== move.sourcePreimage.target || edit.oldTargetHash !== move.sourcePreimage.contentHash)
    throw new Error(`Symlink edit is not bound to move preimage: ${move.sourcePath}`);
  return { nodeKind: "symlink", contentHash: edit.newTargetHash, mode: move.sourcePreimage.mode, size: Buffer.byteLength(edit.newTarget), target: edit.newTarget };
}
function assertDirectoryOnlyTree(path) {
  const stat = lstatSync(path);
  if (!stat.isDirectory() || stat.isSymbolicLink())
    throw new Error(`Embedded root residual node is not a directory: ${path}`);
  for (const name of readdirSync2(path).sort((left, right) => Buffer.from(left).compare(Buffer.from(right))))
    assertDirectoryOnlyTree(join2(path, name));
}
function assertWritableAncestors(repoRoot, logicalPath) {
  const target = absolutePath(repoRoot, logicalPath);
  for (let current = dirname2(target);current !== repoRoot && current !== dirname2(current); current = dirname2(current)) {
    const stat = lstatOrNull(current);
    if (stat?.isSymbolicLink() || stat && !stat.isDirectory())
      throw new Error(`Destination ancestor is not a no-follow directory: ${logicalPath}`);
  }
}
function canonicalDirectoryName(taxonomy, kindId, slug, parentKindId) {
  const kind = taxonomy.directoryKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.slugRegex.test(slug) || (kind.parentKindIds?.length ?? 0) > 0 && !kind.parentKindIds?.includes(parentKindId ?? ""))
    throw new Error(`Taxonomy directory kind ${kindId} cannot own slug ${slug}`);
  return `${kind.emoji}${slug}`.normalize("NFC");
}
function canonicalKindOnlyFilename(taxonomy, kindId, extension) {
  const kind = taxonomy.fileKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.extensionChains.includes(extension))
    throw new Error(`Taxonomy file kind ${kindId} cannot own extension ${extension}`);
  return `${kind.emoji}${extension}`.normalize("NFC");
}
function canonicalScopedKindOnlyFilename(taxonomy, kindId, parentKindId, extension) {
  const kind = taxonomy.schema.scopedFileKinds[kindId];
  const filename = `${kind?.emoji ?? ""}${extension}`.normalize("NFC");
  if (!kind || kind.parentDirectoryKindId !== parentKindId || !kind.extensionChains.includes(extension) || !new RegExp(kind.sourceFilenamePattern, "u").test(filename))
    throw new Error(`Taxonomy scoped file kind ${kindId} cannot own ${filename} below ${parentKindId}`);
  return filename;
}
function pathsOverlap(left, right) {
  const a = normalizeRelative(left);
  const b = normalizeRelative(right);
  return a === b || a === "" || b === "" || a.startsWith(`${b}/`) || b.startsWith(`${a}/`);
}
function assertGeneratorNodeRecords(records, roots, label) {
  const seen = new Set;
  for (const record2 of records) {
    const path = normalizeRelative(record2.path);
    if (path !== record2.path || seen.has(path))
      throw new Error(`${label} contains a duplicate or noncanonical path: ${record2.path}`);
    if (!roots.some((root) => path === root || path.startsWith(`${root}/`)))
      throw new Error(`${label} path is outside registered roots: ${path}`);
    if (!["directory", "file", "symlink"].includes(record2.nodeKind) || !/^[a-f0-9]{64}$/u.test(record2.contentHash) || !Number.isSafeInteger(record2.mode) || record2.mode < 0 || record2.mode > 4095)
      throw new Error(`${label} contains an invalid node record: ${path}`);
    if (record2.nodeKind !== "directory" && (!Number.isSafeInteger(record2.size) || record2.size < 0) || record2.nodeKind === "symlink" && (sha256(record2.target) !== record2.contentHash || Buffer.byteLength(record2.target) !== record2.size))
      throw new Error(`${label} contains incomplete no-follow leaf evidence: ${path}`);
    seen.add(path);
  }
  if (records.some((record2, index) => index > 0 && generatorPathCompare(records[index - 1].path, record2.path) > 0))
    throw new Error(`${label} must be path-sorted`);
}
function nxTargetRecord(repoRoot, ownerPath, target) {
  const manifestPath = absolutePath(repoRoot, `${ownerPath}/\uD83D\uDCCB\uFE0Fproject.json`);
  const manifest = record(JSON.parse(readFileSync2(manifestPath, "utf8")), `Nx manifest ${manifestPath}`);
  const separator = target.lastIndexOf(":");
  const project = target.slice(0, separator);
  const targetName = target.slice(separator + 1);
  const targets = record(manifest.targets, `Nx manifest ${manifestPath}.targets`);
  if (manifest.name !== project || !Object.hasOwn(targets, targetName))
    throw new Error(`Nx manifest ${manifestPath} does not own target ${target}`);
  return record(targets[targetName], `Nx target ${target}`);
}
function assertNxTarget(repoRoot, ownerPath, target) {
  nxTargetRecord(repoRoot, ownerPath, target);
}
function assertGeneratorPreviewTarget(repoRoot, ownerPath, target) {
  const preview = nxTargetRecord(repoRoot, ownerPath, target);
  const options = record(preview.options, `Nx target ${target}.options`);
  if (preview.executor !== "nx:run-commands" || options.cwd !== ownerPath || options.command !== "bun ./\uD83D\uDCDC\uFE0Fscript.ts preview-generated")
    throw new Error(`Nx target ${target} is not the exact owner JSON preview command`);
}
function assertRegenerationContract(regeneration, taxonomy, repoRoot) {
  const contract = taxonomy.schema.generatorContracts[regeneration.contractId];
  if (!contract || contract.ownership !== "owned" || !contract.ownerPath || !contract.target || !contract.previewTarget)
    throw new Error(`Regeneration ${regeneration.id} does not reference an owned generator contract`);
  const roots = contract.outputRoots.map((output) => output.path).sort(generatorPathCompare);
  if (regeneration.cwd !== contract.ownerPath || canonicalJson(regeneration.command) !== canonicalJson(["bun", "nx", "run", contract.target]))
    throw new Error(`Regeneration ${regeneration.id} command is not schema-owned`);
  const expectedVerify = contract.checkTarget ? ["bun", "nx", "run", contract.checkTarget] : undefined;
  if (canonicalJson(regeneration.verifyCommand) !== canonicalJson(expectedVerify))
    throw new Error(`Regeneration ${regeneration.id} verification command is not schema-owned`);
  if (canonicalJson([...regeneration.outputRoots].sort()) !== canonicalJson(roots))
    throw new Error(`Regeneration ${regeneration.id} output roots do not match its contract`);
  assertGeneratorNodeRecords(regeneration.preOutputs, roots, `Regeneration ${regeneration.id} preOutputs`);
  assertGeneratorNodeRecords(regeneration.outputs, roots, `Regeneration ${regeneration.id} outputs`);
  assertGeneratorNodeRecords(regeneration.inputs, regeneration.inputs.map((input) => input.path), `Regeneration ${regeneration.id} inputs`);
  for (const input of regeneration.inputs)
    if (!contract.inputPatterns.some((pattern) => taxonomyPathPatternMatches(input.path, pattern)))
      throw new Error(`Regeneration ${regeneration.id} input is not schema-owned: ${input.path}`);
  const preview = parseGeneratorPreviewManifest(`${generatorPreviewJson(regeneration.preview)}
`, regeneration.contractId, roots, taxonomy.exclusions.map((entry) => entry.path));
  if (regeneration.previewManifestDigest !== sha256(`${generatorPreviewJson(preview)}
`) || canonicalJson(regeneration.staleRemovals) !== canonicalJson(preview.staleRemovals) || canonicalJson(regeneration.outputs) !== canonicalJson(previewNodeRecords(preview)))
    throw new Error(`Regeneration ${regeneration.id} does not match its frozen preview manifest`);
  validatePreviewPreState(preview, regeneration.preOutputs);
  const identity = sha256(canonicalJson({ contractId: regeneration.contractId, cwd: regeneration.cwd, command: regeneration.command, verifyCommand: regeneration.verifyCommand, outputRoots: roots, inputs: regeneration.inputs, preOutputs: regeneration.preOutputs, outputs: regeneration.outputs, preview, previewManifestDigest: regeneration.previewManifestDigest, staleRemovals: regeneration.staleRemovals })).slice(0, 24);
  if (regeneration.id !== identity)
    throw new Error(`Regeneration ${regeneration.id} does not match canonical regeneration bytes`);
  assertNxTarget(repoRoot, contract.ownerPath, contract.target);
  assertGeneratorPreviewTarget(repoRoot, contract.ownerPath, contract.previewTarget);
  if (contract.checkTarget)
    assertNxTarget(repoRoot, contract.ownerPath, contract.checkTarget);
  return contract;
}
function assertPlanOutsideTransaction(plan, transactionRoot, taxonomy, repoRoot) {
  transactionBackupAuthorities(plan);
  const paths = [
    ...plan.scope ? [plan.scope] : [],
    ...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]),
    ...plan.embeddedTicketRoots.flatMap((root) => [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
    ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
    ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...entry.authority.kind === "byte-and-mode-identical" ? entry.authority.members.flatMap((member) => [member.sourcePath, member.finalPath]) : [entry.authority.fixturePath]]),
    ...plan.destinationAncestorPreimages.map((entry) => entry.path),
    ...plan.edits.map((edit) => edit.path),
    ...plan.regenerations.flatMap((regeneration) => [regeneration.cwd, ...regeneration.outputRoots, ...regeneration.inputs.map((input) => input.path), ...regeneration.preOutputs.map((output) => output.path), ...regeneration.outputs.map((output) => output.path), ...regeneration.staleRemovals])
  ];
  const excludedPath = paths.find((path) => isExcluded(path, taxonomy));
  if (excludedPath)
    throw new Error(`Plan path crosses a lexical opaque exclusion: ${excludedPath}`);
  const ancestorAuthority = new Map(plan.destinationAncestorPreimages.map((entry) => [entry.path, entry]));
  const overlap = paths.find((path) => pathsOverlap(path, transactionRoot) && !(ancestorAuthority.get(path)?.state === "directory" && transactionRoot.startsWith(`${path}/`)));
  if (overlap)
    throw new Error(`Plan path overlaps taxonomy transaction root: ${overlap} <> ${transactionRoot}`);
  assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, transactionRoot), "taxonomy transaction root", true);
  for (const path of paths)
    assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, path), `plan path ${path}`);
  const directoryRoles = [
    ...plan.scope ? [plan.scope] : [],
    ...plan.embeddedTicketRoots.flatMap((root) => [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]),
    ...plan.regenerations.flatMap((regeneration) => [regeneration.cwd, ...regeneration.outputRoots])
  ];
  for (const path of directoryRoles)
    assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, path), `plan directory ${path}`, true);
  const sourceRoles = new Map;
  const ownSource = (path, role) => {
    const prior = sourceRoles.get(path);
    if (prior)
      throw new Error(`Plan source has conflicting ${prior} and ${role} operations: ${path}`);
    sourceRoles.set(path, role);
  };
  for (const move of plan.moves)
    ownSource(move.sourcePath, "move");
  for (const relocation of plan.embeddedTicketRootRelocations)
    ownSource(relocation.sourcePath, "embedded relocation");
  for (const removal of plan.evidenceRemovals)
    ownSource(removal.sourcePath, "evidence removal");
  const destinations = [...plan.moves.map((entry) => entry.destinationPath), ...plan.embeddedTicketRootRelocations.map((entry) => entry.destinationPath)];
  if (new Set(destinations).size !== destinations.length || destinations.some((path, index) => destinations.some((candidate, other) => index !== other && pathsOverlap(path, candidate))))
    throw new Error("Plan contains duplicate or overlapping move/relocation destinations");
  const removalSources = new Set(plan.evidenceRemovals.map((entry) => entry.sourcePath));
  if (destinations.some((path) => removalSources.has(path)))
    throw new Error("Plan destination overlaps an evidence-removal source");
  const relocationSources = new Set(plan.embeddedTicketRootRelocations.map((entry) => entry.sourcePath));
  if (plan.moves.some((entry) => relocationSources.has(entry.destinationPath)) || plan.embeddedTicketRootRelocations.some((entry) => sourceRoles.has(entry.destinationPath)))
    throw new Error("Move/relocation destination overlaps another mutable source");
  if (plan.embeddedTicketRoots.some((root, index) => plan.embeddedTicketRoots.some((candidate, other) => index !== other && pathsOverlap(root.sourceMetadataRoot, candidate.sourceMetadataRoot))))
    throw new Error("Embedded metadata roots overlap");
  for (const root of plan.embeddedTicketRoots) {
    const forbidden = [
      ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
      ...plan.edits.map((entry) => entry.path),
      ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals])
    ].find((path) => pathsOverlap(path, root.sourceMetadataRoot));
    if (forbidden)
      throw new Error(`Operation conflicts with embedded metadata-root ownership: ${forbidden} <> ${root.sourceMetadataRoot}`);
  }
  if (new Set(plan.symlinkTargetEdits.map((entry) => entry.sourcePath)).size !== plan.symlinkTargetEdits.length || new Set(plan.symlinkTargetEdits.map((entry) => entry.finalPath)).size !== plan.symlinkTargetEdits.length)
    throw new Error("Symlink target edits do not have unique source/final paths");
  for (const edit of plan.symlinkTargetEdits) {
    const move = plan.moves.filter((candidate) => candidate.sourcePath === edit.sourcePath && candidate.destinationPath === edit.finalPath);
    if (edit.sourcePath !== edit.finalPath && move.length !== 1)
      throw new Error(`Symlink target edit is not bound to its exact move: ${edit.sourcePath}`);
    if (plan.evidenceRemovals.some((entry) => entry.sourcePath === edit.sourcePath) || plan.embeddedTicketRootRelocations.some((entry) => entry.sourcePath === edit.sourcePath) || plan.edits.some((entry) => entry.path === edit.finalPath))
      throw new Error(`Symlink target edit conflicts with another mutation: ${edit.sourcePath}`);
  }
  for (const edit of plan.edits) {
    const sourceMove = plan.moves.find((move) => move.sourcePath === edit.path && move.destinationPath !== edit.path);
    if (sourceMove || plan.evidenceRemovals.some((entry) => entry.sourcePath === edit.path) || plan.embeddedTicketRootRelocations.some((entry) => entry.sourcePath === edit.path || entry.destinationPath === edit.path))
      throw new Error(`Text edit conflicts with a mutable source: ${edit.path}`);
  }
  for (const regeneration of plan.regenerations) {
    assertRegenerationContract(regeneration, taxonomy, repoRoot);
    const conflict = [...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]), ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]), ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]), ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...entry.authority.kind === "byte-and-mode-identical" ? entry.authority.members.flatMap((member) => [member.sourcePath, member.finalPath]) : [entry.authority.fixturePath]]), ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath]), ...plan.edits.map((edit) => edit.path)].find((path) => regeneration.outputRoots.some((root) => pathsOverlap(path, root)));
    if (conflict)
      throw new Error(`Generated output must be regenerated source-first, not moved or edited directly: ${conflict}`);
  }
  const roles = [
    ...plan.moves.flatMap((entry) => [{ path: entry.sourcePath, role: "move-source", owner: entry.operationId }, { path: entry.destinationPath, role: "move-destination", owner: entry.operationId }]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [{ path: entry.sourcePath, role: "relocation-source", owner: entry.operationId }, { path: entry.destinationPath, role: "relocation-destination", owner: entry.operationId }]),
    ...plan.evidenceRemovals.map((entry) => ({ path: entry.sourcePath, role: "removal-source", owner: entry.operationId })),
    ...plan.embeddedTicketRoots.map((entry) => ({ path: entry.sourceMetadataRoot, role: "root-source", owner: entry.operationId })),
    ...plan.symlinkTargetEdits.flatMap((entry) => [{ path: entry.sourcePath, role: "symlink-source", owner: entry.operationId }, { path: entry.finalPath, role: "symlink-final", owner: entry.operationId }]),
    ...[...new Set(plan.edits.map((entry) => entry.path))].map((path) => ({ path, role: "edit", owner: path })),
    ...plan.regenerations.flatMap((entry) => entry.outputRoots.map((path) => ({ path, role: "generator-output", owner: entry.id })))
  ];
  const allowedOverlap = (left, right) => {
    if (left.role === "edit" && right.role === "edit" && left.path === right.path)
      return true;
    if (left.owner === right.owner && left.path === right.path && new Set([left.role, right.role]).has("symlink-source") && new Set([left.role, right.role]).has("symlink-final"))
      return true;
    const pair = new Set([left.role, right.role]);
    if (pair.has("move-source") && pair.has("symlink-source") || pair.has("move-destination") && pair.has("symlink-final")) {
      const move = left.role.startsWith("move-") ? left : right;
      const symlink = left.role.startsWith("symlink-") ? left : right;
      return plan.symlinkTargetEdits.some((entry) => entry.operationId === symlink.owner && plan.moves.some((candidate) => candidate.operationId === move.owner && candidate.sourcePath === entry.sourcePath && candidate.destinationPath === entry.finalPath));
    }
    if (pair.has("move-destination") && pair.has("edit") && left.path === right.path)
      return true;
    if (pair.has("root-source") && (pair.has("relocation-source") || pair.has("removal-source"))) {
      const root = left.role === "root-source" ? left : right, child = left.role === "root-source" ? right : left;
      return child.path.startsWith(`${root.path}/`) && (child.role === "relocation-source" ? plan.embeddedTicketRootRelocations.some((entry) => entry.operationId === child.owner && entry.embeddedTicketRootId === root.owner) : plan.evidenceRemovals.some((entry) => entry.operationId === child.owner && entry.embeddedTicketRootId === root.owner));
    }
    return false;
  };
  for (let leftIndex = 0;leftIndex < roles.length; leftIndex++)
    for (let rightIndex = leftIndex + 1;rightIndex < roles.length; rightIndex++) {
      const left = roles[leftIndex], right = roles[rightIndex];
      if (pathsOverlap(left.path, right.path) && !allowedOverlap(left, right))
        throw new Error(`Plan mutable path roles overlap: ${left.role}:${left.path} <> ${right.role}:${right.path}`);
    }
}
function cleanupCommittedTransaction(repoRoot, journal, plan) {
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  if (!lstatOrNull(stagingRoot) && !lstatOrNull(backupRoot))
    return;
  const stagingStat = lstatOrNull(stagingRoot);
  if (stagingStat) {
    if (!stagingStat.isDirectory() || stagingStat.isSymbolicLink())
      throw new Error("Committed staging root must be a no-follow directory");
    const expectedTop = [
      ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
      ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
      ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`)
    ].sort(generatorPathCompare);
    if (canonicalJson(readdirSync2(stagingRoot).sort(generatorPathCompare)) !== canonicalJson(expectedTop))
      throw new Error("Committed staging root has unexpected or missing operation evidence");
  }
  for (const root of plan.embeddedTicketRoots) {
    const staged = normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`);
    if (lstatOrNull(absolutePath(repoRoot, staged)) && canonicalJson(noFollowTreeDigest(repoRoot, staged)) !== canonicalJson(root.residualTreeDigest))
      throw new Error(`Committed embedded root residual tree changed: ${root.operationId}`);
  }
  for (const removal of plan.evidenceRemovals) {
    const staged = join2(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
    if (lstatOrNull(staged) && canonicalJson(leafPreimage(staged)) !== canonicalJson(removal.preimage))
      throw new Error(`Committed removal stage preimage changed: ${removal.operationId}`);
  }
  for (const edit of plan.symlinkTargetEdits) {
    const staged = join2(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    if (lstatOrNull(staged) && (!lstatSync(staged).isSymbolicLink() || readlinkSync(staged) !== edit.oldTarget))
      throw new Error(`Committed symlink stage preimage changed: ${edit.operationId}`);
  }
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind !== "file")
      continue;
    const stored = join2(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 4095) !== backup.mode || stat.size !== backup.size)
      throw new Error(`Committed typed backup changed: ${path}`);
  }
  const backupStat = lstatOrNull(backupRoot);
  if (backupStat) {
    if (!backupStat.isDirectory() || backupStat.isSymbolicLink())
      throw new Error("Committed backup root must be a no-follow directory");
    const expected = Object.values(journal.backups).filter((entry) => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
    if (canonicalJson(readdirSync2(backupRoot).sort(generatorPathCompare)) !== canonicalJson(expected))
      throw new Error("Committed backup root has unexpected or missing evidence");
  } else if (Object.values(journal.backups).some((entry) => entry.kind === "file"))
    throw new Error("Committed backup root is missing frozen file evidence");
  if (stagingStat)
    durableRemove(stagingRoot, true);
  if (backupStat)
    durableRemove(backupRoot, true);
}
function cleanupRolledBackTransaction(repoRoot, journal, plan) {
  if (journal.state !== "rolled-back")
    throw new Error("Rolled-back cleanup requires a terminal rolled-back journal");
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stagingStat = lstatOrNull(stagingRoot);
  const backupStat = lstatOrNull(backupRoot);
  if (!stagingStat && !backupStat)
    return;
  if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest)
    throw new Error("Rolled-back cleanup pre-state digest changed");
  if (stagingStat) {
    if (!stagingStat.isDirectory() || stagingStat.isSymbolicLink())
      throw new Error("Rolled-back staging root is not a no-follow directory");
    if (readdirSync2(stagingRoot).length > 0)
      throw new Error("Rolled-back staging root contains unexpected evidence");
  }
  if (backupStat) {
    if (!backupStat.isDirectory() || backupStat.isSymbolicLink())
      throw new Error("Rolled-back backup root is not a no-follow directory");
    const expected = Object.values(journal.backups).filter((entry) => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
    const actual = readdirSync2(backupRoot).sort(generatorPathCompare);
    if (canonicalJson(actual) !== canonicalJson(expected))
      throw new Error("Rolled-back backup root contains unexpected or missing evidence");
    for (const [path, backup] of Object.entries(journal.backups)) {
      if (backup.kind !== "file")
        continue;
      const stored = join2(backupRoot, backup.backupPath);
      const stat = lstatOrNull(stored);
      if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 4095) !== backup.mode || stat.size !== backup.size)
        throw new Error(`Rolled-back backup changed: ${path}`);
    }
  }
  if (stagingStat)
    durableRemove(stagingRoot, true);
  if (backupStat)
    durableRemove(backupRoot, true);
}
function assertActiveTransactionEvidence(repoRoot, plan, journal, exact) {
  const stagingRoot = absolutePath(repoRoot, journal.stagingRoot);
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const stage = lstatOrNull(stagingRoot), backup = lstatOrNull(backupRoot);
  if (!stage?.isDirectory() || stage.isSymbolicLink() || !backup?.isDirectory() || backup.isSymbolicLink())
    throw new Error("Active transaction stage/backup roots must be direct no-follow directories");
  const allowedStage = new Set([
    ...plan.moves.map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`),
    journal.journalWriteDirectory
  ]);
  const actualStage = readdirSync2(stagingRoot).sort(generatorPathCompare);
  const unexpectedStage = actualStage.find((name) => !allowedStage.has(name));
  if (unexpectedStage)
    throw new Error(`Active transaction staging root contains unexpected evidence: ${unexpectedStage}`);
  const expectedBackups = Object.values(journal.backups).filter((entry) => entry.kind === "file").map((entry) => entry.backupPath).sort(generatorPathCompare);
  const actualBackups = readdirSync2(backupRoot).sort(generatorPathCompare);
  if (canonicalJson(actualBackups) !== canonicalJson(expectedBackups))
    throw new Error("Active transaction backup root contains unexpected or missing evidence");
  if (!exact)
    return;
  const expectedStage = [
    ...plan.moves.filter((entry) => journal.stagedMoveIds.includes(entry.operationId) && !journal.installedMoveIds.includes(entry.operationId)).map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.filter((entry) => journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && !journal.installedEmbeddedRelocationIds.includes(entry.operationId)).map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.filter((entry) => journal.stagedEvidenceRemovalIds.includes(entry.operationId)).map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.filter((entry) => journal.stagedEmbeddedRootIds.includes(entry.operationId)).map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.filter((entry) => journal.stagedSymlinkTargetEditIds.includes(entry.operationId)).map((entry) => `symlink-${entry.operationId}`)
  ].sort(generatorPathCompare);
  if (canonicalJson(actualStage) !== canonicalJson(expectedStage))
    throw new Error("Active transaction staging root does not match its exact journal tuple set");
}

class TaxonomyStartedRegenerationPartialError extends Error {
  regenerationId;
  constructor(regenerationId) {
    super(`resume-state-drift: started regeneration has a transaction-owned partial output tree: ${regenerationId}`);
    this.regenerationId = regenerationId;
  }
}
function validateResumeTuples(repoRoot, plan, journal, taxonomy) {
  let reconciled = false;
  const present = (...paths) => paths.map((path) => Boolean(lstatOrNull(path)));
  for (const move of plan.moves) {
    const source = absolutePath(repoRoot, move.sourcePath), stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination = absolutePath(repoRoot, move.destinationPath);
    const states = present(source, stage, destination);
    if (!journal.installedMoveIds.includes(move.operationId) && journal.stagedMoveIds.includes(move.operationId) && !states[0] && !states[1] && states[2] && canonicalJson(leafPreimage(destination)) === canonicalJson(move.sourcePreimage)) {
      journal.installedMoveIds.push(move.operationId);
      reconciled = true;
    }
    if (!journal.stagedMoveIds.includes(move.operationId) && journal.preparedMoveIds.includes(move.operationId) && !states[0] && states[1] && !states[2]) {
      journal.stagedMoveIds.push(move.operationId);
      reconciled = true;
    }
    const expected = journal.installedMoveIds.includes(move.operationId) ? 2 : journal.stagedMoveIds.includes(move.operationId) ? 1 : journal.preparedMoveIds.includes(move.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected])
      throw new Error(`resume-state-drift: move ${move.operationId}`);
    const current = [source, stage, destination][expected];
    const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
    if (!journal.appliedEditPaths.includes(move.destinationPath) && canonicalJson(leafPreimage(current)) !== canonicalJson(retargetedMovePreimage(move, installedLink)))
      throw new Error(`resume-state-drift: move preimage ${move.operationId}`);
  }
  for (const entry of plan.embeddedTicketRootRelocations) {
    const states = present(absolutePath(repoRoot, entry.sourcePath), join2(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), absolutePath(repoRoot, entry.destinationPath));
    if (!journal.installedEmbeddedRelocationIds.includes(entry.operationId) && journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && !states[0] && !states[1] && states[2] && canonicalJson(leafPreimage(absolutePath(repoRoot, entry.destinationPath))) === canonicalJson(entry.preimage)) {
      journal.installedEmbeddedRelocationIds.push(entry.operationId);
      reconciled = true;
    }
    if (!journal.stagedEmbeddedRelocationIds.includes(entry.operationId) && journal.preparedEmbeddedRelocationIds.includes(entry.operationId) && !states[0] && states[1] && !states[2]) {
      journal.stagedEmbeddedRelocationIds.push(entry.operationId);
      reconciled = true;
    }
    const expected = journal.installedEmbeddedRelocationIds.includes(entry.operationId) ? 2 : journal.stagedEmbeddedRelocationIds.includes(entry.operationId) ? 1 : journal.preparedEmbeddedRelocationIds.includes(entry.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected])
      throw new Error(`resume-state-drift: embedded relocation ${entry.operationId}`);
    if (canonicalJson(leafPreimage([absolutePath(repoRoot, entry.sourcePath), join2(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), absolutePath(repoRoot, entry.destinationPath)][expected])) !== canonicalJson(entry.preimage))
      throw new Error(`resume-state-drift: embedded relocation preimage ${entry.operationId}`);
  }
  for (const entry of plan.evidenceRemovals) {
    const states = present(absolutePath(repoRoot, entry.sourcePath), join2(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`));
    if (!journal.stagedEvidenceRemovalIds.includes(entry.operationId) && journal.preparedEvidenceRemovalIds.includes(entry.operationId) && !states[0] && states[1]) {
      journal.stagedEvidenceRemovalIds.push(entry.operationId);
      reconciled = true;
    }
    const expected = journal.stagedEvidenceRemovalIds.includes(entry.operationId) ? 1 : journal.preparedEvidenceRemovalIds.includes(entry.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected])
      throw new Error(`resume-state-drift: evidence removal ${entry.operationId}`);
    if (canonicalJson(leafPreimage([absolutePath(repoRoot, entry.sourcePath), join2(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`)][expected])) !== canonicalJson(entry.preimage))
      throw new Error(`resume-state-drift: evidence removal preimage ${entry.operationId}`);
    if (entry.authority.kind === "byte-and-mode-identical")
      for (const member of entry.authority.members.filter((candidate) => candidate.disposition === "retain")) {
        const owningMove = plan.moves.find((move) => move.sourcePath === member.sourcePath && move.destinationPath === member.finalPath);
        const retained = owningMove ? journal.installedMoveIds.includes(owningMove.operationId) ? absolutePath(repoRoot, owningMove.destinationPath) : journal.stagedMoveIds.includes(owningMove.operationId) ? join2(absolutePath(repoRoot, journal.stagingRoot), owningMove.operationId) : absolutePath(repoRoot, owningMove.sourcePath) : absolutePath(repoRoot, member.finalPath);
        if (!lstatOrNull(retained) || canonicalJson(leafPreimage(retained)) !== canonicalJson(member.preimage))
          throw new Error(`resume-state-drift: retained evidence ${member.finalPath}`);
      }
    else if (entry.authority.kind === "serialized-path-sentinel") {
      const fixture = serializedSentinelCases(repoRoot);
      const sentinel = fixture?.cases.find((candidate) => candidate.id === entry.authority.caseId);
      if (!fixture || fixture.fixtureContentHash !== entry.authority.fixtureContentHash || !sentinel || sentinel.inputPath !== entry.authority.serializedInputPath || sentinel.physicalSourcePath !== entry.sourcePath || sentinel.expectedViolationCode !== entry.authority.expectedViolationCode || sentinel.sourceContentHash !== entry.preimage.contentHash)
        throw new Error(`resume-state-drift: serialized sentinel authority ${entry.operationId}`);
    }
  }
  for (const root of plan.embeddedTicketRoots) {
    const states = present(absolutePath(repoRoot, root.sourceMetadataRoot), join2(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`));
    if (!journal.stagedEmbeddedRootIds.includes(root.operationId) && journal.preparedEmbeddedRootIds.includes(root.operationId) && !states[0] && states[1]) {
      journal.stagedEmbeddedRootIds.push(root.operationId);
      reconciled = true;
    }
    const expected = journal.stagedEmbeddedRootIds.includes(root.operationId) ? 1 : journal.preparedEmbeddedRootIds.includes(root.operationId) && states[1] ? 1 : 0;
    if (states.filter(Boolean).length !== 1 || !states[expected])
      throw new Error(`resume-state-drift: embedded root ${root.operationId}`);
    const current = states[1] ? normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`) : root.sourceMetadataRoot;
    const children = [...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath), ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath)];
    const currentTree = states[1] ? noFollowTreeDigest(repoRoot, current) : noFollowTreeDigestExcluding(repoRoot, current, children);
    if (canonicalJson(currentTree) !== canonicalJson(root.residualTreeDigest))
      throw new Error(`resume-state-drift: embedded root tree ${root.operationId}`);
  }
  for (const edit of plan.symlinkTargetEdits) {
    const link = absolutePath(repoRoot, edit.finalPath), stage = join2(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    const linkStat = lstatOrNull(link), stageStat = lstatOrNull(stage);
    if (!journal.installedSymlinkTargetEditIds.includes(edit.operationId) && journal.stagedSymlinkTargetEditIds.includes(edit.operationId) && linkStat?.isSymbolicLink() && readlinkSync(link) === edit.newTarget && stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget) {
      journal.installedSymlinkTargetEditIds.push(edit.operationId);
      reconciled = true;
    }
    if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId) && journal.preparedSymlinkTargetEditIds.includes(edit.operationId) && !linkStat && stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget) {
      journal.stagedSymlinkTargetEditIds.push(edit.operationId);
      reconciled = true;
    }
    if (journal.installedSymlinkTargetEditIds.includes(edit.operationId)) {
      if (!linkStat?.isSymbolicLink() || readlinkSync(link) !== edit.newTarget || !stageStat?.isSymbolicLink() || readlinkSync(stage) !== edit.oldTarget)
        throw new Error(`resume-state-drift: symlink edit ${edit.operationId}`);
    } else if (journal.stagedSymlinkTargetEditIds.includes(edit.operationId) || journal.preparedSymlinkTargetEditIds.includes(edit.operationId) && stageStat) {
      if (linkStat || !stageStat?.isSymbolicLink() || readlinkSync(stage) !== edit.oldTarget)
        throw new Error(`resume-state-drift: symlink stage ${edit.operationId}`);
    } else if (!linkStat?.isSymbolicLink() || readlinkSync(link) !== edit.oldTarget)
      throw new Error(`resume-state-drift: symlink source ${edit.operationId}`);
    const targetMove = plan.moves.find((move) => move.sourcePath === edit.logicalTargetSourcePath && move.destinationPath === edit.logicalTargetFinalPath);
    const targetPath = targetMove ? journal.installedMoveIds.includes(targetMove.operationId) ? absolutePath(repoRoot, targetMove.destinationPath) : journal.stagedMoveIds.includes(targetMove.operationId) ? join2(absolutePath(repoRoot, journal.stagingRoot), targetMove.operationId) : absolutePath(repoRoot, targetMove.sourcePath) : absolutePath(repoRoot, edit.logicalTargetSourcePath);
    const targetStat = lstatOrNull(targetPath);
    if (edit.logicalTargetPreimage.state === "absent") {
      if (targetStat)
        throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    } else if (edit.logicalTargetPreimage.state === "directory") {
      if (!targetStat?.isDirectory() || targetStat.isSymbolicLink())
        throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    } else if (!journal.appliedEditPaths.includes(edit.logicalTargetFinalPath)) {
      const targetLinkEdit = plan.symlinkTargetEdits.find((candidate) => candidate.sourcePath === edit.logicalTargetSourcePath && candidate.finalPath === edit.logicalTargetFinalPath && journal.installedSymlinkTargetEditIds.includes(candidate.operationId));
      if (targetLinkEdit) {
        if (!targetStat?.isSymbolicLink() || readlinkSync(targetPath) !== targetLinkEdit.newTarget)
          throw new Error(`resume-state-drift: nested symlink logical target ${edit.operationId}`);
      } else if (!targetStat || canonicalJson(leafPathPreimage(targetPath)) !== canonicalJson(edit.logicalTargetPreimage))
        throw new Error(`resume-state-drift: symlink logical target ${edit.operationId}`);
    }
  }
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind !== "file")
      continue;
    const stored = join2(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || hashPath(stored) !== backup.contentHash || (stat.mode & 4095) !== backup.mode || stat.size !== backup.size)
      throw new Error(`resume-state-drift: backup ${path}`);
  }
  for (const path of journal.appliedEditPaths) {
    const backup = journal.backups[path];
    if (!backup || backup.kind !== "file")
      throw new Error(`resume-state-drift: applied edit backup ${path}`);
    const edits = plan.edits.filter((edit) => edit.path === path);
    const expected = applyEditsToContent(readFileSync2(join2(backupRoot, backup.backupPath), "utf8"), edits);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    if (!stat?.isFile() || stat.isSymbolicLink() || readFileSync2(current, "utf8") !== expected || (stat.mode & 4095) !== backup.mode || stat.size !== Buffer.byteLength(expected))
      throw new Error(`resume-state-drift: applied edit ${path}`);
  }
  const editPaths = [...new Set(plan.edits.map((entry) => entry.path))].sort(generatorPathCompare);
  for (const path of editPaths.filter((entry) => !journal.appliedEditPaths.includes(entry))) {
    const backup = journal.backups[path];
    if (!backup)
      continue;
    if (backup.kind !== "file")
      throw new Error(`resume-state-drift: edit backup kind ${path}`);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    const edits = plan.edits.filter((entry) => entry.path === path);
    const expected = applyEditsToContent(readFileSync2(join2(backupRoot, backup.backupPath), "utf8"), edits);
    if (stat?.isFile() && !stat.isSymbolicLink() && readFileSync2(current, "utf8") === expected && (stat.mode & 4095) === backup.mode && stat.size === Buffer.byteLength(expected)) {
      journal.appliedEditPaths.push(path);
      reconciled = true;
    } else if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(current) !== backup.contentHash || (stat.mode & 4095) !== backup.mode || stat.size !== backup.size)
      throw new Error(`resume-state-drift: prepared edit ${path}`);
  }
  const startedOutputs = new Set(plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id)).flatMap((entry) => entry.preOutputs.map((output) => output.path)));
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (journal.appliedEditPaths.includes(path) || startedOutputs.has(path) || backup.kind === "file")
      continue;
    const current = lstatOrNull(absolutePath(repoRoot, path));
    if (backup.kind === "absent" && current || backup.kind === "symlink" && (!current?.isSymbolicLink() || canonicalJson(leafPreimage(absolutePath(repoRoot, path))) !== canonicalJson({ nodeKind: "symlink", contentHash: backup.targetHash, mode: backup.mode, size: backup.size, target: backup.target })))
      throw new Error(`resume-state-drift: typed backup source ${path}`);
  }
  for (const regeneration of plan.regenerations) {
    const inputs = regeneration.inputs.map((input) => generatorNodeRecord(repoRoot, input.path, taxonomy));
    if (canonicalJson(inputs) !== canonicalJson(regeneration.inputs))
      throw new Error(`resume-state-drift: regeneration inputs ${regeneration.id}`);
    const outputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
    const pre = canonicalJson(outputs) === canonicalJson(regeneration.preOutputs);
    const post = canonicalJson(outputs) === canonicalJson(regeneration.outputs);
    if (journal.completedRegenerationIds.includes(regeneration.id) && !post)
      throw new Error(`resume-state-drift: regeneration outputs ${regeneration.id}`);
    if (journal.startedRegenerationIds.includes(regeneration.id) && !journal.completedRegenerationIds.includes(regeneration.id) && !pre && !post)
      throw new TaxonomyStartedRegenerationPartialError(regeneration.id);
    if (!journal.startedRegenerationIds.includes(regeneration.id) && !pre)
      throw new Error(`resume-state-drift: regeneration outputs ${regeneration.id}`);
    if (journal.startedRegenerationIds.includes(regeneration.id) && !journal.completedRegenerationIds.includes(regeneration.id) && post) {
      journal.completedRegenerationIds.push(regeneration.id);
      reconciled = true;
    }
  }
  return reconciled;
}
function assertJournalPhaseMembership(plan, journal) {
  if (journal.state === "rolling-back" || journal.state === "rolled-back")
    return;
  const rank = { prepared: 0, staging: 1, disposing: 2, installing: 3, retargeting: 4, editing: 5, regenerating: 6, verifying: 7, committed: 8, "rolling-back": -1, "rolled-back": -1 };
  const exact = (actual, expected, label) => {
    const sorted = [...expected].sort(generatorPathCompare);
    if (canonicalJson(actual) !== canonicalJson(sorted))
      throw new Error(`Resume journal ${label} is incomplete for state ${journal.state}`);
  };
  const phase = rank[journal.state];
  if (phase >= 2) {
    exact(journal.stagedMoveIds, plan.moves.map((entry) => entry.operationId), "staged moves");
    exact(journal.stagedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations.map((entry) => entry.operationId), "staged embedded relocations");
    exact(journal.stagedEvidenceRemovalIds, plan.evidenceRemovals.map((entry) => entry.operationId), "staged evidence removals");
  }
  if (phase >= 3)
    exact(journal.stagedEmbeddedRootIds, plan.embeddedTicketRoots.map((entry) => entry.operationId), "staged embedded roots");
  if (phase >= 4) {
    exact(journal.installedMoveIds, plan.moves.map((entry) => entry.operationId), "installed moves");
    exact(journal.installedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations.map((entry) => entry.operationId), "installed embedded relocations");
  }
  if (phase >= 5)
    exact(journal.installedSymlinkTargetEditIds, plan.symlinkTargetEdits.map((entry) => entry.operationId), "installed symlink target edits");
  if (phase >= 6)
    exact(journal.appliedEditPaths, [...new Set(plan.edits.map((entry) => entry.path))], "applied edit paths");
  if (phase >= 7)
    exact(journal.completedRegenerationIds, plan.regenerations.map((entry) => entry.id), "completed regenerations");
}
function assertJournalPlanMembership(plan, journal) {
  const subset = (child, parent) => child.every((id) => parent.includes(id));
  const exactPlanIds = (ids, records) => ids.every((id) => records.some((record2) => record2.operationId === id));
  if (!subset(journal.stagedMoveIds, journal.preparedMoveIds) || !subset(journal.installedMoveIds, journal.stagedMoveIds) || !exactPlanIds(journal.preparedMoveIds, plan.moves) || !subset(journal.stagedEmbeddedRelocationIds, journal.preparedEmbeddedRelocationIds) || !subset(journal.installedEmbeddedRelocationIds, journal.stagedEmbeddedRelocationIds) || !exactPlanIds(journal.preparedEmbeddedRelocationIds, plan.embeddedTicketRootRelocations) || !subset(journal.stagedEvidenceRemovalIds, journal.preparedEvidenceRemovalIds) || !exactPlanIds(journal.preparedEvidenceRemovalIds, plan.evidenceRemovals) || !subset(journal.stagedEmbeddedRootIds, journal.preparedEmbeddedRootIds) || !exactPlanIds(journal.preparedEmbeddedRootIds, plan.embeddedTicketRoots) || !subset(journal.stagedSymlinkTargetEditIds, journal.preparedSymlinkTargetEditIds) || !subset(journal.installedSymlinkTargetEditIds, journal.stagedSymlinkTargetEditIds) || !exactPlanIds(journal.preparedSymlinkTargetEditIds, plan.symlinkTargetEdits) || !subset(journal.completedRegenerationIds, journal.startedRegenerationIds) || journal.startedRegenerationIds.some((id) => !plan.regenerations.some((record2) => record2.id === id)) || journal.appliedEditPaths.some((path) => !plan.edits.some((edit) => edit.path === path)))
    throw new Error("Resume journal operation state does not match the plan");
}
function assertJournalBackupAuthority(plan, journal) {
  const editPaths = new Set(plan.edits.map((entry) => entry.path));
  const generatorPreimages = new Map(plan.regenerations.filter((entry) => journal.state === "rolled-back" || journal.startedRegenerationIds.includes(entry.id)).flatMap((entry) => entry.preOutputs.filter((output) => output.nodeKind !== "directory").map((output) => [output.path, output])));
  const editBackupsAllowed = ["editing", "regenerating", "verifying", "committed", "rolling-back", "rolled-back"].includes(journal.state);
  const seenStored = new Set;
  for (const [path, backup] of Object.entries(journal.backups)) {
    const generatorPreimage = generatorPreimages.get(path);
    if (!editPaths.has(path) && !generatorPreimage)
      throw new Error(`Resume journal has an unauthorized backup path: ${path}`);
    if (editPaths.has(path) && !editBackupsAllowed)
      throw new Error(`Resume journal contains a reference-edit backup before the editing phase: ${path}`);
    if (editPaths.has(path) && backup.kind !== "file")
      throw new Error(`Reference-edit backup must be a regular file: ${path}`);
    if (editPaths.has(path)) {
      const preimages = new Map(plan.edits.filter((entry) => entry.path === path).map((entry) => [canonicalJson(entry.preimage), entry.preimage]));
      const preimage = [...preimages.values()][0];
      if (preimages.size !== 1 || !preimage || backup.kind !== "file" || backup.contentHash !== preimage.contentHash || backup.mode !== preimage.mode || backup.size !== preimage.size)
        throw new Error(`Reference-edit backup does not match its frozen preimage: ${path}`);
    }
    if (generatorPreimage) {
      const matches = generatorPreimage.nodeKind === "file" ? backup.kind === "file" && backup.contentHash === generatorPreimage.contentHash && backup.mode === generatorPreimage.mode && backup.size === generatorPreimage.size : generatorPreimage.nodeKind === "symlink" ? backup.kind === "symlink" && backup.targetHash === generatorPreimage.contentHash && backup.mode === generatorPreimage.mode && backup.size === generatorPreimage.size && backup.target === generatorPreimage.target : false;
      if (!matches)
        throw new Error(`Generator backup does not match its frozen preOutput: ${path}`);
    }
    if (backup.kind !== "file")
      continue;
    const expected = `${sha256(path).slice(0, 24)}.backup`;
    if (backup.backupPath !== expected || seenStored.has(backup.backupPath))
      throw new Error(`Resume journal backup storage identity is invalid: ${path}`);
    seenStored.add(backup.backupPath);
  }
  for (const path of journal.appliedEditPaths)
    if (!journal.backups[path])
      throw new Error(`Applied reference edit has no frozen typed backup: ${path}`);
  for (const regeneration of plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id))) {
    for (const output of regeneration.preOutputs.filter((entry) => entry.nodeKind !== "directory"))
      if (!journal.backups[output.path])
        throw new Error(`Started regeneration has no frozen typed backup: ${regeneration.id}:${output.path}`);
  }
}
function transactionBackupAuthorities(plan) {
  const byPath = new Map;
  const add = (path, expected, edit, regenerationId) => {
    const prior = byPath.get(path);
    if (prior) {
      if (canonicalJson(prior.expected) !== canonicalJson(expected))
        throw new Error(`Transaction backup path has incompatible frozen authorities: ${path}`);
      prior.edit ||= edit;
      if (regenerationId && !prior.regenerationIds.includes(regenerationId))
        prior.regenerationIds.push(regenerationId);
      return;
    }
    byPath.set(path, { path, expected, edit, regenerationIds: regenerationId ? [regenerationId] : [] });
  };
  for (const edit of plan.edits)
    add(edit.path, edit.preimage, true);
  for (const regeneration of plan.regenerations)
    for (const output of regeneration.preOutputs)
      if (output.nodeKind !== "directory")
        add(output.path, output, false, regeneration.id);
  const byIdentity = new Map;
  for (const authority of byPath.values()) {
    const identity = sha256(authority.path).slice(0, 24);
    const prior = byIdentity.get(identity);
    if (prior && prior.path !== authority.path)
      throw new Error(`Transaction backup storage identity collision: ${prior.path} <> ${authority.path}`);
    byIdentity.set(identity, authority);
  }
  return byIdentity;
}
function expectedBackupRecord(path, expected) {
  if (expected.nodeKind === "symlink") {
    if (sha256(expected.target) !== expected.contentHash || Buffer.byteLength(expected.target) !== expected.size)
      throw new Error(`Frozen symlink backup preimage is incomplete: ${path}`);
    return { kind: "symlink", target: expected.target, targetHash: expected.contentHash, mode: expected.mode, size: expected.size };
  }
  return { kind: "file", backupPath: `${sha256(path).slice(0, 24)}.backup`, contentHash: expected.contentHash, mode: expected.mode, size: expected.size };
}
function assertStoredFileBackup(path, record2) {
  const stat = lstatOrNull(path);
  if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(path) !== record2.contentHash || (stat.mode & 4095) !== record2.mode || stat.size !== record2.size)
    throw new Error(`Stored transaction backup does not match its frozen preimage: ${path}`);
}
function transactionBinaryWritePreparations(container, preparationName, candidateName) {
  const preparations = [];
  for (const name of readdirSync2(container).sort(generatorPathCompare)) {
    const match = /^write-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match)
      continue;
    const pid = Number.parseInt(match[1], 10), token = match[2];
    if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== preparationName(pid, token))
      throw new Error(`Transaction binary write preparation name is invalid: ${name}`);
    const root = join2(container, name), stat = lstatOrNull(root);
    if (!stat?.isDirectory() || stat.isSymbolicLink())
      throw new Error(`Transaction binary write preparation must be a no-follow directory: ${name}`);
    const children = readdirSync2(root).sort(generatorPathCompare);
    if (children.length > 1 || children.length === 1 && children[0] !== candidateName)
      throw new Error(`Transaction binary write preparation contains unexpected evidence: ${name}`);
    const leaf = children.length === 1 ? join2(root, candidateName) : undefined;
    const leafStat = leaf ? lstatOrNull(leaf) : undefined;
    if (leaf && (!leafStat?.isFile() || leafStat.isSymbolicLink()))
      throw new Error(`Transaction binary write candidate must be a regular no-follow file: ${leaf}`);
    preparations.push({ root, leaf });
  }
  if (preparations.length > 1)
    throw new Error(`Transaction binary writer has duplicate preparations: ${container}`);
  return preparations;
}
function writeTransactionBinaryCandidate(container, preparationName, candidateName, bytes, mode) {
  const root = join2(container, preparationName(process.pid, randomUUID()));
  mkdirSync(root);
  fsyncDirectory(container);
  const leaf = join2(root, candidateName), descriptor = openSync(leaf, "wx", mode);
  try {
    writeFileSync(descriptor, bytes);
    fsyncSync(descriptor);
  } finally {
    closeSync(descriptor);
  }
  chmodSync(leaf, mode);
  fsyncFile(leaf);
  fsyncDirectory(root);
  return { root, leaf };
}
function backupPath(repoRoot, logicalPath, backupRoot, journal, expected, preparationName, writePreparationName, writeCandidateName) {
  if (journal.backups[logicalPath] !== undefined)
    return;
  const source = absolutePath(repoRoot, logicalPath);
  const stat = lstatOrNull(source);
  if (!stat)
    throw new Error(`Backup source is absent: ${logicalPath}`);
  if (stat.isSymbolicLink()) {
    const target = readlinkSync(source);
    const actual = { nodeKind: "symlink", contentHash: sha256(target), mode: stat.mode & 4095, size: Buffer.byteLength(target), target };
    if (expected.nodeKind !== "symlink" || canonicalJson(actual) !== canonicalJson(expected))
      throw new Error(`Backup source changed from its frozen symlink preimage: ${logicalPath}`);
    journal.backups[logicalPath] = expectedBackupRecord(logicalPath, expected);
    return;
  }
  if (!stat.isFile() || expected.nodeKind !== "file")
    throw new Error(`Backup target kind changed: ${logicalPath}`);
  const bytes = readFileSync2(source);
  const record2 = expectedBackupRecord(logicalPath, expected);
  if (record2.kind !== "file" || sha256(bytes) !== record2.contentHash || bytes.byteLength !== record2.size || (stat.mode & 4095) !== record2.mode)
    throw new Error(`Backup source changed from its frozen file preimage: ${logicalPath}`);
  const token = randomUUID();
  const candidate = join2(backupRoot, preparationName(record2.backupPath.slice(0, 24), process.pid, token));
  const candidateLeaf = join2(candidate, record2.backupPath);
  const destination = join2(backupRoot, record2.backupPath);
  mkdirSync(candidate);
  fsyncDirectory(backupRoot);
  const writer = writeTransactionBinaryCandidate(candidate, writePreparationName, writeCandidateName, bytes, record2.mode);
  assertStoredFileBackup(writer.leaf, record2);
  const sourceAfter = leafPreimage(source);
  if (sourceAfter.nodeKind !== "file" || sourceAfter.contentHash !== record2.contentHash || sourceAfter.mode !== record2.mode || sourceAfter.size !== record2.size)
    throw new Error(`Backup source changed during frozen snapshot publication: ${logicalPath}`);
  durableRename(writer.leaf, candidateLeaf);
  durableRemove(writer.root, true);
  const published = lstatOrNull(destination);
  if (published)
    assertStoredFileBackup(destination, record2);
  else {
    try {
      linkSync(candidateLeaf, destination);
    } catch (error) {
      if (error.code !== "EEXIST")
        throw error;
      assertStoredFileBackup(destination, record2);
    }
    fsyncDirectory(backupRoot);
  }
  durableRemove(candidate, true);
  journal.backups[logicalPath] = record2;
}
function recoverTransactionBackups(repoRoot, plan, journal, preparationName, writePreparationName, writeCandidateName, validateOnly = false) {
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const authorities = transactionBackupAuthorities(plan);
  let changed = false;
  const rootChildren = readdirSync2(backupRoot).sort(generatorPathCompare);
  const recordForStored = (identity, path) => {
    const authority = authorities.get(identity);
    if (!authority || authority.expected.nodeKind !== "file")
      throw new Error(`Transaction backup evidence has no unique frozen file authority: ${basename2(path)}`);
    const stat = lstatOrNull(path);
    const record2 = { kind: "file", backupPath: `${identity}.backup`, contentHash: authority.expected.contentHash, mode: authority.expected.mode, size: stat?.size ?? -1 };
    if (!stat?.isFile() || stat.isSymbolicLink() || authority.expected.size !== undefined && stat.size !== authority.expected.size)
      throw new Error(`Transaction backup evidence is not a regular exact-size file: ${path}`);
    assertStoredFileBackup(path, record2);
    return record2;
  };
  const candidates = [];
  const records = new Map;
  for (const name of rootChildren) {
    const rest = splitLeadingEmoji(name).rest;
    const final = /^([0-9a-f]{24})\.backup$/u.exec(name);
    if (final) {
      const record3 = recordForStored(final[1], join2(backupRoot, name));
      records.set(final[1], record3);
      continue;
    }
    const match = /^backup-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3])) {
      if (rest.startsWith("restore-") || rest.startsWith("lease-"))
        continue;
      throw new Error(`Transaction backup root contains unauthorized evidence: ${name}`);
    }
    const identity = match[1], pid = Number.parseInt(match[2], 10), token = match[3];
    if (name !== preparationName(identity, pid, token))
      throw new Error(`Transaction backup preparation is not canonical: ${name}`);
    if (candidates.some((entry) => entry.identity === identity))
      throw new Error(`Transaction backup root contains duplicate candidates for ${identity}`);
    const candidate = join2(backupRoot, name);
    const candidateStat = lstatOrNull(candidate);
    if (!candidateStat?.isDirectory() || candidateStat.isSymbolicLink())
      throw new Error(`Transaction backup preparation must be a no-follow directory: ${name}`);
    const expectedLeaf = `${identity}.backup`;
    const writers = transactionBinaryWritePreparations(candidate, writePreparationName, writeCandidateName);
    const writerNames = new Set(writers.map((entry) => basename2(entry.root)));
    const candidateChildren = readdirSync2(candidate).sort(generatorPathCompare);
    if (candidateChildren.some((child) => child !== expectedLeaf && !writerNames.has(child)))
      throw new Error(`Transaction backup preparation has unexpected evidence: ${name}`);
    const candidateLeaf = join2(candidate, expectedLeaf);
    const authority = authorities.get(identity);
    if (!authority || authority.expected.nodeKind !== "file")
      throw new Error(`Transaction backup preparation has no frozen file authority: ${name}`);
    const record2 = expectedBackupRecord(authority.path, authority.expected);
    if (record2.kind !== "file")
      throw new Error(`Transaction backup preparation kind is invalid: ${name}`);
    const outerPresent = Boolean(lstatOrNull(candidateLeaf));
    if (outerPresent)
      assertStoredFileBackup(candidateLeaf, record2);
    const writer = writers[0];
    const writerExact = Boolean(writer?.leaf && (() => {
      try {
        assertStoredFileBackup(writer.leaf, record2);
        return true;
      } catch {
        return false;
      }
    })());
    if (!outerPresent && !writerExact) {
      const current = absolutePath(repoRoot, authority.path), currentStat = lstatOrNull(current);
      const pre = currentStat?.isFile() && !currentStat.isSymbolicLink() && hashPath(current) === record2.contentHash && (currentStat.mode & 4095) === record2.mode && currentStat.size === record2.size;
      if (!pre)
        throw new Error(`Incomplete transaction backup writer has no exact source preimage: ${name}`);
      candidates.push({ root: candidate, writer, identity, record: record2, discard: true });
      continue;
    }
    const destination = join2(backupRoot, expectedLeaf);
    if (lstatOrNull(destination)) {
      assertStoredFileBackup(destination, record2);
      const finalRecord = records.get(identity) ?? recordForStored(identity, destination);
      if (canonicalJson(finalRecord) !== canonicalJson(record2))
        throw new Error(`Transaction backup candidate differs from its published leaf: ${identity}`);
      records.set(identity, finalRecord);
    } else
      records.set(identity, record2);
    candidates.push({ root: candidate, leaf: outerPresent ? candidateLeaf : writer.leaf, writer, identity, record: record2, discard: false });
  }
  for (const [identity, record2] of records) {
    const authority = authorities.get(identity);
    if (!authority)
      throw new Error(`Transaction backup leaf has no unique plan authority: ${identity}.backup`);
    const prior = journal.backups[authority.path];
    if (prior) {
      if (canonicalJson(prior) !== canonicalJson(record2))
        throw new Error(`Transaction backup journal evidence differs from its stored leaf: ${authority.path}`);
      continue;
    }
    if (!(authority.edit && journal.state === "editing") && !(authority.regenerationIds.length > 0 && journal.state === "regenerating"))
      throw new Error(`Transaction backup orphan is unreachable in journal state ${journal.state}: ${authority.path}`);
    const current = absolutePath(repoRoot, authority.path);
    const currentStat = lstatOrNull(current);
    const pre = currentStat?.isFile() && !currentStat.isSymbolicLink() && hashPath(current) === record2.contentHash && (currentStat.mode & 4095) === record2.mode && currentStat.size === record2.size;
    if (!pre)
      throw new Error(`Transaction backup orphan source is not its frozen preimage: ${authority.path}`);
  }
  if (!validateOnly)
    for (const candidate of candidates) {
      if (candidate.discard) {
        durableRemove(candidate.root, true);
        continue;
      }
      const outerLeaf = join2(candidate.root, candidate.record.backupPath);
      if (!lstatOrNull(outerLeaf))
        durableRename(candidate.leaf, outerLeaf);
      if (candidate.writer && lstatOrNull(candidate.writer.root))
        durableRemove(candidate.writer.root, true);
      const destination = join2(backupRoot, candidate.record.backupPath);
      if (!lstatOrNull(destination)) {
        try {
          linkSync(outerLeaf, destination);
        } catch (error) {
          if (error.code !== "EEXIST")
            throw error;
          assertStoredFileBackup(destination, candidate.record);
        }
        fsyncDirectory(backupRoot);
      }
      durableRemove(candidate.root, true);
    }
  for (const [identity, record2] of records) {
    const authority = authorities.get(identity);
    if (!journal.backups[authority.path] && !validateOnly) {
      journal.backups[authority.path] = record2;
      changed = true;
    } else if (!journal.backups[authority.path])
      changed = true;
  }
  for (const regeneration of plan.regenerations) {
    const outputs = regeneration.preOutputs.filter((entry) => entry.nodeKind !== "directory");
    const hasOrphan = outputs.some((entry) => journal.backups[entry.path] !== undefined);
    if (!hasOrphan || journal.startedRegenerationIds.includes(regeneration.id))
      continue;
    if (journal.state !== "regenerating")
      throw new Error(`Transaction generator backup orphan is outside its regenerating phase: ${regeneration.id}`);
    if (!validateOnly) {
      for (const output of outputs)
        backupPath(repoRoot, output.path, backupRoot, journal, output, preparationName, writePreparationName, writeCandidateName);
      journal.startedRegenerationIds.push(regeneration.id);
    }
    changed = true;
  }
  return changed;
}
function restoreBackup(repoRoot, plan, logicalPath, backupRoot, encoded, preparationName) {
  const destination = absolutePath(repoRoot, logicalPath);
  mkdirSync(dirname2(destination), { recursive: true });
  const current = lstatOrNull(destination);
  if (encoded.kind === "absent") {
    if (current?.isDirectory())
      throw new Error(`Cannot remove directory while restoring absent backup: ${logicalPath}`);
    if (current)
      durableRemove(destination);
    return;
  }
  if (current?.isDirectory())
    throw new Error(`Cannot replace directory while restoring backup: ${logicalPath}`);
  const identity = sha256(logicalPath).slice(0, 24), token = randomUUID();
  const candidateRoot = join2(backupRoot, preparationName(identity, process.pid, token));
  const candidateLeaf = join2(candidateRoot, `${identity}.backup`);
  const postLeaf = join2(candidateRoot, `${identity}.post`);
  mkdirSync(candidateRoot);
  fsyncDirectory(backupRoot);
  if (encoded.kind === "symlink") {
    if (encoded.targetHash !== sha256(encoded.target))
      throw new Error(`Symlink backup target hash changed: ${logicalPath}`);
    symlinkSync(encoded.target, candidateLeaf);
    fsyncDirectory(candidateRoot);
  } else {
    const source = join2(backupRoot, encoded.backupPath);
    assertStoredFileBackup(source, encoded);
    linkSync(source, candidateLeaf);
    fsyncDirectory(candidateRoot);
  }
  const candidatePreimage = leafPreimage(candidateLeaf);
  const expected = encoded.kind === "symlink" ? { nodeKind: "symlink", contentHash: encoded.targetHash, mode: encoded.mode, size: encoded.size, target: encoded.target } : { nodeKind: "file", contentHash: encoded.contentHash, mode: encoded.mode, size: encoded.size };
  if (canonicalJson(candidatePreimage) !== canonicalJson(expected))
    throw new Error(`Restore candidate does not match its typed backup: ${logicalPath}`);
  if (current && canonicalJson(leafPreimage(destination)) === canonicalJson(expected)) {
    durableRemove(candidateRoot, true);
    return;
  }
  if (current) {
    const edits = plan.edits.filter((entry) => entry.path === logicalPath);
    const transactionOwned = edits.length > 0 && encoded.kind === "file" && (() => {
      const rendered = applyEditsToContent(readFileSync2(join2(backupRoot, encoded.backupPath), "utf8"), edits);
      return current.isFile() && !current.isSymbolicLink() && readFileSync2(destination, "utf8") === rendered && (current.mode & 4095) === encoded.mode;
    })();
    if (!transactionOwned)
      throw new Error(`Restore destination is not an exact transaction-owned postimage: ${logicalPath}`);
    durableRename(destination, postLeaf);
  }
  durableRename(candidateLeaf, destination);
  if (lstatOrNull(postLeaf))
    durableRemove(postLeaf);
  durableRemove(candidateRoot, true);
}
function recoverRestorePreparations(repoRoot, plan, journal, preparationName, validateOnly = false) {
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const byIdentity = new Map;
  for (const [path, backup] of Object.entries(journal.backups)) {
    const identity = sha256(path).slice(0, 24);
    if (byIdentity.has(identity))
      throw new Error(`Restore preparation identity is ambiguous: ${identity}`);
    byIdentity.set(identity, { path, backup });
  }
  const actions = [];
  for (const name of readdirSync2(backupRoot).sort(generatorPathCompare)) {
    const rest = splitLeadingEmoji(name).rest;
    if (!rest.startsWith("restore-"))
      continue;
    const match = /^restore-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3]) || name !== preparationName(match[1], Number.parseInt(match[2], 10), match[3]))
      throw new Error(`Restore preparation name is invalid: ${name}`);
    const authority = byIdentity.get(match[1]);
    if (!authority || journal.state !== "rolling-back")
      throw new Error(`Restore preparation has no rolling-back journal authority: ${name}`);
    const root = join2(backupRoot, name), leaf = join2(root, `${match[1]}.backup`), postLeaf = join2(root, `${match[1]}.post`), destination = absolutePath(repoRoot, authority.path);
    const stat = lstatOrNull(root);
    const children = stat?.isDirectory() && !stat.isSymbolicLink() ? readdirSync2(root).sort(generatorPathCompare) : [];
    if (!stat?.isDirectory() || stat.isSymbolicLink() || children.some((child) => child !== basename2(leaf) && child !== basename2(postLeaf)) || new Set(children).size !== children.length)
      throw new Error(`Restore preparation has incomplete or unexpected evidence: ${name}`);
    const backupPresent = children.includes(basename2(leaf)), postPresent = children.includes(basename2(postLeaf));
    const candidate = backupPresent ? leafPreimage(leaf) : undefined;
    const expected = authority.backup.kind === "file" ? { nodeKind: "file", contentHash: authority.backup.contentHash, mode: authority.backup.mode, size: authority.backup.size } : authority.backup.kind === "symlink" ? { nodeKind: "symlink", contentHash: authority.backup.targetHash, mode: authority.backup.mode, size: authority.backup.size, target: authority.backup.target } : undefined;
    if (!expected || candidate && canonicalJson(candidate) !== canonicalJson(expected))
      throw new Error(`Restore preparation bytes differ from journal backup authority: ${name}`);
    const currentStat = lstatOrNull(destination);
    const restored = Boolean(currentStat && !currentStat.isDirectory() && canonicalJson(leafPreimage(destination)) === canonicalJson(expected));
    let transactionOwned = false;
    const edits = plan.edits.filter((entry) => entry.path === authority.path);
    if (edits.length > 0 && currentStat?.isFile() && !currentStat.isSymbolicLink() && authority.backup.kind === "file") {
      const rendered = applyEditsToContent(readFileSync2(join2(backupRoot, authority.backup.backupPath), "utf8"), edits);
      transactionOwned = readFileSync2(destination, "utf8") === rendered && (currentStat.mode & 4095) === authority.backup.mode;
    }
    if (postPresent) {
      if (edits.length === 0 || authority.backup.kind !== "file")
        throw new Error(`Restore postimage has no exact edit authority: ${name}`);
      const rendered = applyEditsToContent(readFileSync2(join2(backupRoot, authority.backup.backupPath), "utf8"), edits), post = lstatOrNull(postLeaf);
      if (!post?.isFile() || post.isSymbolicLink() || readFileSync2(postLeaf, "utf8") !== rendered || (post.mode & 4095) !== authority.backup.mode)
        throw new Error(`Restore postimage differs from transaction output: ${name}`);
    }
    const startedGeneratorAbsent = !currentStat && plan.regenerations.some((entry) => journal.startedRegenerationIds.includes(entry.id) && entry.preOutputs.some((output) => output.path === authority.path));
    const stateValid = !backupPresent && !postPresent ? transactionOwned || restored || startedGeneratorAbsent : backupPresent && postPresent ? !currentStat : backupPresent ? transactionOwned || !currentStat || restored : restored;
    if (!stateValid)
      throw new Error(`Restore preparation has an impossible exchange tuple: ${name}`);
    actions.push({ root, backupLeaf: backupPresent ? leaf : undefined, postLeaf: postPresent ? postLeaf : undefined, destination, destinationPre: restored, destinationPost: transactionOwned });
  }
  if (!validateOnly)
    for (const action of actions) {
      if (action.backupLeaf && !action.destinationPre) {
        if (action.destinationPost && !action.postLeaf) {
          const post = join2(action.root, basename2(action.backupLeaf).replace(/\.backup$/u, ".post"));
          durableRename(action.destination, post);
          action.postLeaf = post;
        }
        mkdirSync(dirname2(action.destination), { recursive: true });
        durableRename(action.backupLeaf, action.destination);
      }
      if (action.postLeaf && lstatOrNull(action.postLeaf))
        durableRemove(action.postLeaf);
      durableRemove(action.root, true);
    }
}
function referenceEditResult(repoRoot, plan, journal, path) {
  const edits = plan.edits.filter((entry) => entry.path === path);
  const preimages = new Map(edits.map((entry) => [canonicalJson(entry.preimage), entry.preimage]));
  const preimage = [...preimages.values()][0];
  const backup = journal.backups[path];
  if (preimages.size !== 1 || !preimage || backup?.kind !== "file" || canonicalJson(backup) !== canonicalJson(expectedBackupRecord(path, preimage)))
    throw new Error(`Reference edit lacks one exact frozen preimage and backup: ${path}`);
  return { bytes: Buffer.from(applyEditsToContent(readFileSync2(join2(absolutePath(repoRoot, journal.backupRoot), backup.backupPath), "utf8"), edits)), preimage };
}
function applyReferenceEditAtomically(repoRoot, plan, journal, path, preparationName, writePreparationName, writeCandidateName) {
  const target = absolutePath(repoRoot, path), identity = sha256(path).slice(0, 24), result = referenceEditResult(repoRoot, plan, journal, path);
  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage))
    throw new Error(`Reference edit preimage changed: ${path}`);
  const root = join2(absolutePath(repoRoot, journal.stagingRoot), preparationName(identity, process.pid, randomUUID()));
  const leaf = join2(root, `${identity}.edit`);
  const preLeaf = join2(root, `${identity}.pre`);
  mkdirSync(root);
  fsyncDirectory(dirname2(root));
  const writer = writeTransactionBinaryCandidate(root, writePreparationName, writeCandidateName, result.bytes, result.preimage.mode);
  const candidate = leafPreimage(writer.leaf);
  if (candidate.nodeKind !== "file" || candidate.contentHash !== sha256(result.bytes) || candidate.mode !== result.preimage.mode || candidate.size !== result.bytes.byteLength)
    throw new Error(`Reference edit candidate differs from rendered bytes: ${path}`);
  if (canonicalJson(leafPreimage(target)) !== canonicalJson(result.preimage))
    throw new Error(`Reference edit source changed during candidate publication: ${path}`);
  durableRename(writer.leaf, leaf);
  durableRemove(writer.root, true);
  durableRename(target, preLeaf);
  durableRename(leaf, target);
  durableRemove(preLeaf);
  durableRemove(root, true);
}
function recoverReferenceEditPreparations(repoRoot, plan, journal, preparationName, writePreparationName, writeCandidateName, validateOnly = false) {
  const stageRoot = absolutePath(repoRoot, journal.stagingRoot);
  const editPaths = new Map;
  for (const path of new Set(plan.edits.map((entry) => entry.path))) {
    const identity = sha256(path).slice(0, 24);
    if (editPaths.has(identity))
      throw new Error(`Reference edit preparation identity collision: ${identity}`);
    editPaths.set(identity, path);
  }
  const actions = [];
  for (const name of readdirSync2(stageRoot).sort(generatorPathCompare)) {
    const rest = splitLeadingEmoji(name).rest;
    if (!rest.startsWith("edit-"))
      continue;
    const match = /^edit-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3]) || name !== preparationName(match[1], Number.parseInt(match[2], 10), match[3]) || journal.state !== "editing")
      throw new Error(`Reference edit preparation is invalid or unreachable: ${name}`);
    const path = editPaths.get(match[1]);
    if (!path || !journal.backups[path])
      throw new Error(`Reference edit preparation has no frozen plan backup: ${name}`);
    const root = join2(stageRoot, name), target = absolutePath(repoRoot, path), rootStat = lstatOrNull(root), expectedLeaf = `${match[1]}.edit`, expectedPreLeaf = `${match[1]}.pre`;
    if (!rootStat?.isDirectory() || rootStat.isSymbolicLink())
      throw new Error(`Reference edit preparation must be a no-follow directory: ${name}`);
    const writers = transactionBinaryWritePreparations(root, writePreparationName, writeCandidateName);
    const writerNames = new Set(writers.map((entry) => basename2(entry.root)));
    const children = readdirSync2(root).sort(generatorPathCompare);
    if (children.some((child) => child !== expectedLeaf && child !== expectedPreLeaf && !writerNames.has(child)) || children.length > 3)
      throw new Error(`Reference edit preparation contains unexpected evidence: ${name}`);
    const result = referenceEditResult(repoRoot, plan, journal, path), targetStat = lstatOrNull(target);
    const pre = targetStat?.isFile() && !targetStat.isSymbolicLink() && canonicalJson(leafPreimage(target)) === canonicalJson(result.preimage);
    const post = targetStat?.isFile() && !targetStat.isSymbolicLink() && sha256(readFileSync2(target)) === sha256(result.bytes) && (targetStat.mode & 4095) === result.preimage.mode && targetStat.size === result.bytes.byteLength;
    const outerLeaf = children.includes(expectedLeaf) ? join2(root, expectedLeaf) : undefined;
    const preLeaf = children.includes(expectedPreLeaf) ? join2(root, expectedPreLeaf) : undefined;
    if (preLeaf && canonicalJson(leafPreimage(preLeaf)) !== canonicalJson(result.preimage))
      throw new Error(`Reference edit preimage exchange bytes are forged: ${name}`);
    const exactCandidate = (candidatePath) => {
      const candidate = leafPreimage(candidatePath);
      return candidate.nodeKind === "file" && candidate.contentHash === sha256(result.bytes) && candidate.mode === result.preimage.mode && candidate.size === result.bytes.byteLength;
    };
    if (outerLeaf && !exactCandidate(outerLeaf))
      throw new Error(`Reference edit preparation bytes are forged: ${name}`);
    const writer = writers[0], writerExact = Boolean(writer?.leaf && exactCandidate(writer.leaf));
    const leaf = outerLeaf ?? (writerExact ? writer.leaf : undefined);
    const discard = Boolean(writer && !outerLeaf && !writerExact);
    if (discard && (!pre || preLeaf))
      throw new Error(`Incomplete reference edit writer has an unreachable target tuple: ${name}`);
    const stateValid = !leaf && !preLeaf ? pre || post : leaf && preLeaf ? !targetStat : leaf ? pre : post;
    if (!stateValid)
      throw new Error(`Reference edit preparation has an impossible target tuple: ${name}`);
    actions.push({ root, leaf, preLeaf, writer, publishWriter: Boolean(!outerLeaf && writerExact), target, targetPre: Boolean(pre), targetPost: Boolean(post), discard });
  }
  if (!validateOnly)
    for (const action of actions) {
      if (action.discard) {
        durableRemove(action.root, true);
        continue;
      }
      if (action.publishWriter) {
        const outerLeaf = join2(action.root, `${sha256(normalizeRelative(relative2(repoRoot, action.target).replaceAll("\\", "/"))).slice(0, 24)}.edit`);
        durableRename(action.leaf, outerLeaf);
        action.leaf = outerLeaf;
      }
      if (action.writer && lstatOrNull(action.writer.root))
        durableRemove(action.writer.root, true);
      if (!action.discard && action.leaf && !action.targetPost) {
        if (action.targetPre && !action.preLeaf) {
          const preLeaf = join2(action.root, basename2(action.leaf).replace(/\.edit$/u, ".pre"));
          durableRename(action.target, preLeaf);
          action.preLeaf = preLeaf;
        }
        durableRename(action.leaf, action.target);
      }
      if (action.preLeaf && lstatOrNull(action.preLeaf))
        durableRemove(action.preLeaf);
      durableRemove(action.root, true);
    }
}
function assertRecoveryRootNames(repoRoot, plan, journal, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName) {
  const stageKnown = new Set([
    journal.journalWriteDirectory,
    ...plan.moves.map((entry) => entry.operationId),
    ...plan.embeddedTicketRootRelocations.map((entry) => `relocation-${entry.operationId}`),
    ...plan.evidenceRemovals.map((entry) => `removal-${entry.operationId}`),
    ...plan.embeddedTicketRoots.map((entry) => `root-${entry.operationId}`),
    ...plan.symlinkTargetEdits.map((entry) => `symlink-${entry.operationId}`)
  ]);
  for (const name of readdirSync2(absolutePath(repoRoot, journal.stagingRoot)).sort(generatorPathCompare)) {
    if (stageKnown.has(name))
      continue;
    const match = /^edit-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match || !TRANSACTION_LEASE_TOKEN.test(match[3]) || name !== editPreparationName(match[1], Number.parseInt(match[2], 10), match[3]))
      throw new Error(`Transaction recovery staging root contains unauthorized evidence: ${name}`);
  }
  for (const name of readdirSync2(absolutePath(repoRoot, journal.backupRoot)).sort(generatorPathCompare)) {
    if (/^[0-9a-f]{24}\.backup$/u.test(name))
      continue;
    const rest = splitLeadingEmoji(name).rest;
    const leaf = /^(backup|restore)-([0-9a-f]{24})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(rest);
    if (leaf && TRANSACTION_LEASE_TOKEN.test(leaf[4])) {
      const expected = leaf[1] === "backup" ? backupPreparationName(leaf[2], Number.parseInt(leaf[3], 10), leaf[4]) : restorePreparationName(leaf[2], Number.parseInt(leaf[3], 10), leaf[4]);
      if (name === expected)
        continue;
    }
    const lease = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(rest);
    if (lease && TRANSACTION_LEASE_TOKEN.test(lease[2]) && name === leasePreparationName(Number.parseInt(lease[1], 10), lease[2], lease[3]))
      continue;
    throw new Error(`Transaction recovery backup root contains unauthorized evidence: ${name}`);
  }
}
function validateLeasePreparationEvidence(backupRoot, leasePreparationName, jsonWritePreparationName, filename, previousName, planDigest, attemptOrdinal) {
  for (const name of readdirSync2(backupRoot).sort(generatorPathCompare)) {
    const match = /^lease-([1-9][0-9]*)-([0-9a-f-]+)-(preparing|stale)$/u.exec(splitLeadingEmoji(name).rest);
    if (!match)
      continue;
    const pid = Number.parseInt(match[1], 10), token = match[2], state = match[3];
    if (!TRANSACTION_LEASE_TOKEN.test(token) || name !== leasePreparationName(pid, token, state))
      throw new Error(`Transaction lease preparation name is invalid: ${name}`);
    const root = join2(backupRoot, name), stat = lstatOrNull(root);
    if (!stat?.isDirectory() || stat.isSymbolicLink())
      throw new Error(`Transaction lease preparation must be a no-follow directory: ${name}`);
    recoverCanonicalJsonCandidates(root, filename, previousName, jsonWritePreparationName, (path) => {
      const record2 = parseTransactionLease(path, planDigest, attemptOrdinal, token);
      if (record2.pid !== pid)
        throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
    }, false, true);
    const canonical = join2(root, filename);
    if (lstatOrNull(canonical)) {
      const record2 = parseTransactionLease(canonical, planDigest, attemptOrdinal, token);
      if (record2.pid !== pid)
        throw new Error(`Transaction lease preparation pid is invalid: ${name}`);
    }
  }
}
function actualAffectedDigest(repoRoot, plan, taxonomy) {
  const row = (path) => {
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat)
      return { path, state: "absent" };
    if (stat.isSymbolicLink()) {
      const target = readlinkSync(absolute);
      return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) };
    }
    if (stat.isFile())
      return { path, state: "file", contentHash: sha256(readFileSync2(absolute)), mode: stat.mode & 4095, size: stat.size };
    return { path, state: "directory-tree", tree: noFollowTreeDigest(repoRoot, path) };
  };
  const rows = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    const stat = lstatOrNull(absolutePath(repoRoot, ancestor.path));
    rows.push(!stat ? { path: ancestor.path, state: "absent" } : stat.isDirectory() && !stat.isSymbolicLink() ? { path: ancestor.path, state: "directory" } : row(ancestor.path));
  }
  for (const move of plan.moves)
    rows.push(row(move.sourcePath), row(move.destinationPath));
  for (const relocation of plan.embeddedTicketRootRelocations)
    rows.push(row(relocation.sourcePath), row(relocation.destinationPath));
  for (const removal of plan.evidenceRemovals) {
    rows.push(row(removal.sourcePath));
    if (removal.authority.kind === "byte-and-mode-identical")
      for (const member of removal.authority.members.filter((member2) => member2.disposition !== "remove"))
        rows.push(row(member.finalPath));
    else if (removal.authority.kind === "serialized-path-sentinel")
      rows.push(row(removal.authority.fixturePath));
  }
  for (const root of plan.embeddedTicketRoots)
    rows.push(row(root.sourceMetadataRoot));
  for (const edit of plan.symlinkTargetEdits)
    rows.push(row(edit.finalPath), row(edit.logicalTargetFinalPath));
  for (const path of new Set(plan.edits.map((edit) => edit.path)))
    rows.push(row(path));
  for (const regeneration of plan.regenerations)
    rows.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy))) });
  return affectedStateDigest(rows);
}
function actualAffectedPreDigest(repoRoot, plan) {
  const row = (path) => {
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat)
      return { path, state: "absent" };
    if (stat.isSymbolicLink()) {
      const target = readlinkSync(absolute);
      return { path, state: "symlink", targetHash: sha256(target), targetSize: Buffer.byteLength(target) };
    }
    if (stat.isFile())
      return { path, state: "file", contentHash: sha256(readFileSync2(absolute)), mode: stat.mode & 4095, size: stat.size };
    return { path, state: "directory-tree", tree: noFollowTreeDigest(repoRoot, path) };
  };
  const rows = [];
  for (const ancestor of plan.destinationAncestorPreimages) {
    const stat = lstatOrNull(absolutePath(repoRoot, ancestor.path));
    rows.push(!stat ? { path: ancestor.path, state: "absent" } : stat.isDirectory() && !stat.isSymbolicLink() ? { path: ancestor.path, state: "directory" } : row(ancestor.path));
  }
  for (const move of plan.moves)
    rows.push(row(move.sourcePath), row(move.destinationPath));
  for (const relocation of plan.embeddedTicketRootRelocations)
    rows.push(row(relocation.sourcePath), row(relocation.destinationPath));
  for (const removal of plan.evidenceRemovals) {
    rows.push(row(removal.sourcePath));
    if (removal.authority.kind === "byte-and-mode-identical")
      for (const member of removal.authority.members.filter((member2) => member2.disposition !== "remove"))
        rows.push(row(member.sourcePath));
    else if (removal.authority.kind === "serialized-path-sentinel")
      rows.push(row(removal.authority.fixturePath));
  }
  for (const root of plan.embeddedTicketRoots)
    rows.push(row(root.sourceMetadataRoot));
  for (const edit of plan.symlinkTargetEdits)
    rows.push(row(edit.sourcePath), row(edit.logicalTargetSourcePath));
  for (const path of new Set(plan.edits.map((edit) => plan.moves.find((move) => move.destinationPath === edit.path)?.sourcePath ?? edit.path)))
    rows.push(row(path));
  for (const regeneration of plan.regenerations)
    rows.push({ path: `@generator/${regeneration.id}`, state: "generator", contentHash: sha256(canonicalJson(regeneration.preOutputs)) });
  return affectedStateDigest(rows);
}
function isProjectionConsumerPath(contract, path) {
  return contract.sourcePathIdentities.includes(path) && new RegExp(contract.sourcePathPattern, "u").test(path);
}
function artifactStaleGroups(paths, taxonomy) {
  const values = [...new Set(paths)];
  const artifacts = canonicalDirectoryName(taxonomy, "artifacts", "artifacts");
  const consumers = Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts);
  const rows = new Map;
  for (const { id, contract } of artifactProjectionContracts(taxonomy))
    for (const path of values) {
      const segments = path.split("/");
      for (let index = 1;index < segments.length; index++) {
        if (segments[index - 1] !== artifacts || segments[index] !== contract.sourceArtifactMemberName)
          continue;
        const ownerRoot = segments.slice(0, index - 1).join("/");
        if (!ownerRoot)
          continue;
        const markers = [...new Set(consumers.filter((consumer) => consumer.projectionContractId === id).flatMap((consumer) => consumer.staleMarkers))].sort(generatorPathCompare);
        if (markers.length === 0)
          continue;
        rows.set(`${id}\x00${ownerRoot}`, { id, rationaleRule: contract.rationaleRule, ownerRoot, markers });
      }
    }
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.ownerRoot, right.ownerRoot) || left.id.localeCompare(right.id));
}
function canonicalMutationProjectionPresent(paths, taxonomy) {
  const projection = taxonomy.schema.semanticPathProjectionContracts[taxonomy.schema.mutationCatalogProjection.projectionContractId];
  const renderer = taxonomy.schema.semanticPathProjectionProfileRenderers[projection.profileRendererId];
  const tests = canonicalDirectoryName(taxonomy, "tests", "tests");
  for (const path of paths) {
    const segments = path.split("/");
    for (let index = 0;index + 1 < segments.length; index++)
      if (segments[index] === tests && matchDirectoryKind(segments[index + 1], taxonomy, "tests").kind?.id === renderer.directoryKindId)
        return true;
  }
  return false;
}
function staleProjectionContentViolations(path, content, groups, taxonomy, mutationActive) {
  const rows = [];
  if (mutationActive) {
    const pattern = new RegExp(OLD_MUTATION_TEST_PREFIX_SOURCE, "gu");
    for (const match of content.matchAll(pattern))
      if (match.index !== undefined)
        rows.push(violation("projection-old-token-stale", path, `Old artifact mutation test hierarchy remains at raw offset ${match.index}`));
  }
  const consumers = Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts);
  for (const group of groups) {
    const internal = path === group.ownerRoot || path.startsWith(`${group.ownerRoot}/`);
    const external = consumers.some((contract) => contract.projectionContractId === group.id && isProjectionConsumerPath(contract, path));
    if (!internal && !external)
      continue;
    for (const marker of group.markers)
      for (let index = content.indexOf(marker);index >= 0; index = content.indexOf(marker, index + marker.length))
        rows.push(violation("projection-old-token-stale", path, `Old ${group.rationaleRule} token remains at raw offset ${index}`));
  }
  return rows;
}
function projectionStaleViolations(repoRoot, plan, taxonomy, inventory) {
  if (inventory) {
    const moveBySource = new Map(plan.moves.map((move) => [move.sourcePath, move.destinationPath]));
    const finalPaths = inventory.entries.map((entry) => moveBySource.get(entry.sourcePath) ?? entry.normalizedPath);
    const mutationActive2 = plan.moves.some((move) => move.rationaleRule === "artifact-mutation-test-projection-v1") || canonicalMutationProjectionPresent(finalPaths, taxonomy);
    const groups2 = artifactStaleGroups(finalPaths, taxonomy);
    const rows2 = [];
    for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "file" && textualPath(candidate.sourcePath))) {
      const path = moveBySource.get(entry.sourcePath) ?? entry.normalizedPath;
      let content;
      try {
        content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync2(absolutePath(repoRoot, entry.sourcePath)));
        const edits = plan.edits.filter((edit) => edit.path === path);
        if (edits.length > 0)
          content = applyEditsToContent(content, edits);
      } catch {
        continue;
      }
      rows2.push(...staleProjectionContentViolations(path, content, groups2, taxonomy, mutationActive2));
    }
    return stableViolations(rows2);
  }
  const paths = new Set;
  for (const row of gitRows(repoRoot, taxonomy))
    if (!isExcluded(row.path, taxonomy) && lstatOrNull(absolutePath(repoRoot, row.path)))
      paths.add(row.path);
  for (const move of plan.moves)
    paths.add(move.destinationPath);
  for (const edit of plan.edits)
    paths.add(edit.path);
  for (const regeneration of plan.regenerations)
    for (const output of regeneration.outputs)
      paths.add(output.path);
  const mutationActive = plan.moves.some((move) => move.rationaleRule === "artifact-mutation-test-projection-v1") || canonicalMutationProjectionPresent(paths, taxonomy);
  const groups = artifactStaleGroups(paths, taxonomy);
  if (!mutationActive && groups.length === 0)
    return [];
  const rows = [];
  for (const path of [...paths].filter(textualPath).sort(generatorPathCompare)) {
    if (isExcluded(path, taxonomy))
      continue;
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat?.isFile() || stat.size > 16 * 1024 * 1024)
      continue;
    try {
      const content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync2(absolute));
      rows.push(...staleProjectionContentViolations(path, content, groups, taxonomy, mutationActive));
    } catch {}
  }
  return stableViolations(rows);
}
function projectionPostApplyViolations(repoRoot, plan, taxonomy) {
  const moves = plan.moves.filter((move) => move.rationaleRule === "artifact-mutation-test-projection-v1");
  if (moves.length === 0)
    return [];
  const ids = taxonomy.schema.mutationCatalogProjection;
  const descendant = mutationDescendantContract(taxonomy);
  const groups = new Map;
  for (const move of moves) {
    const artifactRoot = artifactRootForPath(move.sourcePath);
    if (!artifactRoot || !move.destinationPath.startsWith(`${artifactRoot}/`))
      continue;
    const relativeSegments = move.destinationPath.slice(artifactRoot.length + 1).split("/");
    if (relativeSegments.length < 5)
      continue;
    const scenarioRoot = `${artifactRoot}/${relativeSegments.slice(0, 4).join("/")}`;
    groups.set(scenarioRoot, [...groups.get(scenarioRoot) ?? [], move]);
  }
  const rows = [];
  const expectedRequired = descendant.requiredNodes.map((node) => `${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`);
  for (const [scenarioRoot, group] of [...groups.entries()].sort(([left], [right]) => left.localeCompare(right))) {
    if (group.length !== 6) {
      rows.push(violation("projection-apply-move-count", scenarioRoot, `Projected scenario has ${group.length} file moves, expected 6`));
      continue;
    }
    for (const move of group)
      if (lstatOrNull(absolutePath(repoRoot, move.sourcePath)))
        rows.push(violation("projection-source-file-stale", move.sourcePath, "Projected source file remains after staged move installation"));
    const actual = new Set;
    const walk = (path) => {
      if (isExcluded(path, taxonomy))
        throw new Error(`Projection destination crosses opaque path ${path}`);
      const stat = lstatOrNull(absolutePath(repoRoot, path));
      if (!stat)
        return;
      const relativePath = path === scenarioRoot ? "" : path.slice(scenarioRoot.length + 1);
      if (stat.isSymbolicLink()) {
        rows.push(violation("projection-bundle-symlink", path, "Projected bundle contains a symlink"));
        return;
      }
      actual.add(`${stat.isDirectory() ? "directory" : "file"}\x00${relativePath}`);
      if (stat.isDirectory())
        for (const name of readdirSync2(absolutePath(repoRoot, path)).sort((left, right) => Buffer.from(left).compare(Buffer.from(right))))
          walk(`${path}/${name}`);
    };
    walk(scenarioRoot);
    const alternatives = descendant.exclusiveAlternatives.map((alternative) => alternative.nodes.map((node) => `${node.nodeType}\x00${projectionDescendantPath(node, taxonomy)}`).filter((key) => actual.has(key)));
    if (actual.size !== descendant.realizedNodeCount || expectedRequired.some((key) => !actual.has(key)) || alternatives.some((matches) => matches.length !== 1))
      rows.push(violation("projection-apply-bundle-invalid", scenarioRoot, `Projected destination does not realize the exact ${descendant.realizedNodeCount}-node descendant contract`));
  }
  if (groups.size * 6 !== moves.length)
    rows.push(violation("projection-apply-group-unresolved", moves[0].sourcePath, `${moves.length - groups.size * 6} projection move(s) do not resolve to an exact artifact scenario root`));
  return stableViolations(rows);
}
function artifactProjectionPostApplyViolations(repoRoot, plan, taxonomy) {
  const rows = [];
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const moves = plan.moves.filter((move) => move.rationaleRule === contract.rationaleRule);
    const groups = new Map;
    for (const move of moves) {
      const location = artifactProjectionSourceLocation(move.sourcePath, contract, taxonomy);
      if (location)
        groups.set(location.sourceRoot, [...groups.get(location.sourceRoot) ?? [], move]);
    }
    for (const [sourceRoot, group] of [...groups].sort(([left], [right]) => generatorPathCompare(left, right))) {
      const location = artifactProjectionSourceLocation(sourceRoot, contract, taxonomy);
      if (!location) {
        rows.push(violation("projection-apply-group-unresolved", sourceRoot, `${id} source root cannot be reconstructed from its frozen contract`));
        continue;
      }
      const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot }, taxonomy.schema);
      if (rendered.problems.length > 0) {
        rows.push(violation("projection-apply-group-unresolved", sourceRoot, rendered.problems.join(" | ")));
        continue;
      }
      const expected = new Set([`directory\x00${rendered.destinationRoot}`]);
      for (const move of group) {
        if (lstatOrNull(absolutePath(repoRoot, move.sourcePath)))
          rows.push(violation("projection-source-file-stale", move.sourcePath, "Projected source file remains after staged move installation"));
        const destination = lstatOrNull(absolutePath(repoRoot, move.destinationPath));
        if (!destination?.isFile() || destination.isSymbolicLink())
          rows.push(violation("projection-destination-file-invalid", move.destinationPath, "Projected destination is missing, non-file, or a symlink"));
        expected.add(`file\x00${move.destinationPath}`);
        for (let path = dirname2(move.destinationPath);path === rendered.destinationRoot || path.startsWith(`${rendered.destinationRoot}/`); path = dirname2(path)) {
          expected.add(`directory\x00${path}`);
          if (path === rendered.destinationRoot)
            break;
        }
      }
      const actual = new Set;
      const walk = (path) => {
        if (isExcluded(path, taxonomy))
          throw new Error(`Artifact projection destination crosses opaque path ${path}`);
        const stat = lstatOrNull(absolutePath(repoRoot, path));
        if (!stat)
          return;
        if (stat.isSymbolicLink()) {
          rows.push(violation("projection-bundle-symlink", path, "Projected destination contains a symlink"));
          return;
        }
        actual.add(`${stat.isDirectory() ? "directory" : "file"}\x00${path}`);
        if (stat.isDirectory())
          for (const name of readdirSync2(absolutePath(repoRoot, path)).sort((left, right) => Buffer.from(left).compare(Buffer.from(right))))
            walk(`${path}/${name}`);
      };
      walk(rendered.destinationRoot);
      const missing = [...expected].filter((row) => !actual.has(row));
      const unexpected = [...actual].filter((row) => !expected.has(row));
      if (missing.length > 0 || unexpected.length > 0)
        rows.push(violation("projection-apply-descendants-invalid", rendered.destinationRoot, `${id} exact descendant mismatch: ${missing.length} missing, ${unexpected.length} unexpected`));
    }
    if ([...groups.values()].reduce((count, group) => count + group.length, 0) !== moves.length && moves.length > 0)
      rows.push(violation("projection-apply-group-unresolved", moves[0].sourcePath, `${id} has moves outside its exact source root`));
  }
  return stableViolations(rows);
}
function injectFailure(options, stage) {
  if (options.injectFailureAt === stage)
    throw new Error(`Injected taxonomy failure at ${stage}`);
}
function pruneEmptySourceParents(repoRoot, plan, ticketRoot) {
  const candidates = new Set;
  for (const move of plan.moves) {
    let parent = dirname2(absolutePath(repoRoot, move.sourcePath));
    while (parent !== repoRoot && parent !== dirname2(parent) && !parent.startsWith(`${ticketRoot}/`)) {
      candidates.add(parent);
      parent = dirname2(parent);
    }
  }
  for (const path of [...candidates].sort((a, b) => b.length - a.length)) {
    try {
      rmdirSync(path);
    } catch (error) {
      if (!["ENOTEMPTY", "ENOENT", "EEXIST"].includes(String(error.code)))
        throw error;
    }
  }
}
function rollbackDestinationAncestors(repoRoot, plan) {
  for (const ancestor of plan.destinationAncestorPreimages.filter((entry) => entry.state === "absent").sort((left, right) => right.path.split("/").length - left.path.split("/").length || generatorPathCompare(right.path, left.path))) {
    const path = absolutePath(repoRoot, ancestor.path);
    const stat = lstatOrNull(path);
    if (!stat)
      continue;
    if (!stat.isDirectory() || stat.isSymbolicLink() || readdirSync2(path).length > 0)
      throw new Error(`Rollback-created destination ancestor is occupied: ${ancestor.path}`);
    rmdirSync(path);
    fsyncDirectory(dirname2(path));
  }
}
function reconcileRollbackTuples(repoRoot, plan, journal, taxonomy) {
  let changed = false;
  const remove = (array, id) => {
    const next = array.filter((entry) => entry !== id);
    if (next.length !== array.length) {
      array.splice(0, array.length, ...next);
      changed = true;
    }
  };
  const add = (array, id) => {
    if (!array.includes(id)) {
      array.push(id);
      changed = true;
    }
  };
  const present = (path) => Boolean(lstatOrNull(path));
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  for (const [path, backup] of Object.entries(journal.backups)) {
    if (backup.kind === "symlink" && (backup.targetHash !== sha256(backup.target) || backup.size !== Buffer.byteLength(backup.target)))
      throw new Error(`rollback-state-drift: symlink backup ${path}`);
    if (backup.kind !== "file")
      continue;
    const stored = join2(backupRoot, backup.backupPath);
    const stat = lstatOrNull(stored);
    if (!stat?.isFile() || stat.isSymbolicLink() || hashPath(stored) !== backup.contentHash || (stat.mode & 4095) !== backup.mode || stat.size !== backup.size)
      throw new Error(`rollback-state-drift: file backup ${path}`);
  }
  for (const move of plan.moves) {
    const source = absolutePath(repoRoot, move.sourcePath), stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination = absolutePath(repoRoot, move.destinationPath);
    const states = [present(source), present(stage), present(destination)];
    if (states.filter(Boolean).length !== 1)
      throw new Error(`rollback-state-drift: move ${move.operationId}`);
    const index = states.findIndex(Boolean);
    const current = [source, stage, destination][index];
    const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
    const edited = index === 2 && journal.appliedEditPaths.includes(move.destinationPath);
    let expectedPreimage = retargetedMovePreimage(move, installedLink);
    if (edited) {
      const backup = journal.backups[move.destinationPath];
      if (!backup || backup.kind !== "file")
        throw new Error(`rollback-state-drift: move edit backup ${move.operationId}`);
      const result = applyEditsToContent(readFileSync2(join2(absolutePath(repoRoot, journal.backupRoot), backup.backupPath), "utf8"), plan.edits.filter((edit) => edit.path === move.destinationPath));
      expectedPreimage = { nodeKind: "file", contentHash: sha256(result), mode: backup.mode, size: Buffer.byteLength(result) };
    }
    if (canonicalJson(leafPreimage(current)) !== canonicalJson(expectedPreimage))
      throw new Error(`rollback-state-drift: move preimage ${move.operationId}`);
    if (states[0]) {
      remove(journal.installedMoveIds, move.operationId);
      remove(journal.stagedMoveIds, move.operationId);
      remove(journal.preparedMoveIds, move.operationId);
    } else if (states[1]) {
      remove(journal.installedMoveIds, move.operationId);
      add(journal.stagedMoveIds, move.operationId);
      add(journal.preparedMoveIds, move.operationId);
    } else if (!journal.installedMoveIds.includes(move.operationId))
      throw new Error(`rollback-state-drift: unowned move destination ${move.operationId}`);
  }
  for (const entry of plan.embeddedTicketRootRelocations) {
    const source = absolutePath(repoRoot, entry.sourcePath), stage = join2(absolutePath(repoRoot, journal.stagingRoot), `relocation-${entry.operationId}`), destination = absolutePath(repoRoot, entry.destinationPath);
    const states = [present(source), present(stage), present(destination)];
    if (states.filter(Boolean).length !== 1)
      throw new Error(`rollback-state-drift: relocation ${entry.operationId}`);
    const current = [source, stage, destination][states.findIndex(Boolean)];
    if (canonicalJson(leafPreimage(current)) !== canonicalJson(entry.preimage))
      throw new Error(`rollback-state-drift: relocation preimage ${entry.operationId}`);
    if (states[0]) {
      remove(journal.installedEmbeddedRelocationIds, entry.operationId);
      remove(journal.stagedEmbeddedRelocationIds, entry.operationId);
      remove(journal.preparedEmbeddedRelocationIds, entry.operationId);
    } else if (states[1]) {
      remove(journal.installedEmbeddedRelocationIds, entry.operationId);
      add(journal.stagedEmbeddedRelocationIds, entry.operationId);
      add(journal.preparedEmbeddedRelocationIds, entry.operationId);
    } else if (!journal.installedEmbeddedRelocationIds.includes(entry.operationId))
      throw new Error(`rollback-state-drift: unowned relocation destination ${entry.operationId}`);
  }
  for (const entry of plan.evidenceRemovals) {
    const source = absolutePath(repoRoot, entry.sourcePath), stage = join2(absolutePath(repoRoot, journal.stagingRoot), `removal-${entry.operationId}`);
    const states = [present(source), present(stage)];
    if (states.filter(Boolean).length !== 1 || canonicalJson(leafPreimage(states[0] ? source : stage)) !== canonicalJson(entry.preimage))
      throw new Error(`rollback-state-drift: removal ${entry.operationId}`);
    if (states[0]) {
      remove(journal.stagedEvidenceRemovalIds, entry.operationId);
      remove(journal.preparedEvidenceRemovalIds, entry.operationId);
    } else {
      add(journal.stagedEvidenceRemovalIds, entry.operationId);
      add(journal.preparedEvidenceRemovalIds, entry.operationId);
    }
  }
  for (const root of plan.embeddedTicketRoots) {
    const source = absolutePath(repoRoot, root.sourceMetadataRoot), stage = join2(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
    const states = [present(source), present(stage)];
    if (states.filter(Boolean).length !== 1)
      throw new Error(`rollback-state-drift: embedded root ${root.operationId}`);
    const current = states[0] ? root.sourceMetadataRoot : normalizeRelative(`${journal.stagingRoot}/root-${root.operationId}`);
    const children = [...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath), ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath)];
    const tree = states[0] ? noFollowTreeDigestExcluding(repoRoot, current, children) : noFollowTreeDigest(repoRoot, current);
    if (canonicalJson(tree) !== canonicalJson(root.residualTreeDigest))
      throw new Error(`rollback-state-drift: embedded root tree ${root.operationId}`);
    if (states[0]) {
      remove(journal.stagedEmbeddedRootIds, root.operationId);
      remove(journal.preparedEmbeddedRootIds, root.operationId);
    } else {
      add(journal.stagedEmbeddedRootIds, root.operationId);
      add(journal.preparedEmbeddedRootIds, root.operationId);
    }
  }
  for (const edit of plan.symlinkTargetEdits) {
    const link = absolutePath(repoRoot, edit.finalPath), stage = join2(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    const linkStat = lstatOrNull(link), stageStat = lstatOrNull(stage);
    const oldAtLink = linkStat?.isSymbolicLink() && readlinkSync(link) === edit.oldTarget;
    const newAtLink = linkStat?.isSymbolicLink() && readlinkSync(link) === edit.newTarget;
    const oldAtStage = stageStat?.isSymbolicLink() && readlinkSync(stage) === edit.oldTarget;
    if (oldAtLink && !stageStat) {
      remove(journal.installedSymlinkTargetEditIds, edit.operationId);
      remove(journal.stagedSymlinkTargetEditIds, edit.operationId);
      remove(journal.preparedSymlinkTargetEditIds, edit.operationId);
    } else if ((!linkStat || newAtLink) && oldAtStage) {
      add(journal.stagedSymlinkTargetEditIds, edit.operationId);
      add(journal.preparedSymlinkTargetEditIds, edit.operationId);
      if (newAtLink)
        add(journal.installedSymlinkTargetEditIds, edit.operationId);
      else
        remove(journal.installedSymlinkTargetEditIds, edit.operationId);
    } else
      throw new Error(`rollback-state-drift: symlink edit ${edit.operationId}`);
  }
  for (const path of [...new Set(plan.edits.map((entry) => entry.path))].filter((entry) => journal.backups[entry])) {
    const backup = journal.backups[path];
    if (!backup || backup.kind !== "file")
      throw new Error(`rollback-state-drift: edit backup ${path}`);
    const current = absolutePath(repoRoot, path);
    const stat = lstatOrNull(current);
    const pre = stat?.isFile() && !stat.isSymbolicLink() && hashPath(current) === backup.contentHash && (stat.mode & 4095) === backup.mode && stat.size === backup.size;
    const result = applyEditsToContent(readFileSync2(join2(backupRoot, backup.backupPath), "utf8"), plan.edits.filter((edit) => edit.path === path));
    const post = stat?.isFile() && !stat.isSymbolicLink() && readFileSync2(current, "utf8") === result && (stat.mode & 4095) === backup.mode && stat.size === Buffer.byteLength(result);
    if (pre) {
      if (!journal.appliedEditPaths.includes(path))
        throw new Error(`rollback-state-drift: unowned restored edit ${path}`);
    } else if (post)
      add(journal.appliedEditPaths, path);
    else
      throw new Error(`rollback-state-drift: edit ${path}`);
  }
  for (const regeneration of plan.regenerations) {
    if (journal.startedRegenerationIds.includes(regeneration.id))
      continue;
    if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs))
      throw new Error(`rollback-state-drift: unstarted regeneration ${regeneration.id}`);
  }
  return changed;
}
function rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options) {
  if (journal.state === "rolling-back") {
    if (reconcileRollbackTuples(repoRoot, plan, journal, taxonomy))
      persistJournal(repoRoot, journalPath, journal);
  } else {
    if (reconcileRollbackTuples(repoRoot, plan, journal, taxonomy))
      persistJournal(repoRoot, journalPath, journal);
    journal.state = "rolling-back";
    persistJournal(repoRoot, journalPath, journal);
  }
  const backupRoot = absolutePath(repoRoot, journal.backupRoot);
  const restorePreparationName = (identity, pid, token) => canonicalDirectoryName(taxonomy, "transaction-restore-preparation", `restore-${identity}-${pid}-${token}`, "transaction-backup");
  const started = new Set(journal.startedRegenerationIds);
  for (const regeneration of [...plan.regenerations].reverse()) {
    if (!started.has(regeneration.id))
      continue;
    for (const root of [...regeneration.outputRoots].sort((left, right) => right.length - left.length || generatorPathCompare(right, left)))
      durableRemove(absolutePath(repoRoot, root), true);
    for (const directory of regeneration.preOutputs.filter((entry) => entry.nodeKind === "directory").sort((left, right) => left.path.split("/").length - right.path.split("/").length || generatorPathCompare(left.path, right.path))) {
      const path = absolutePath(repoRoot, directory.path);
      mkdirSync(path, { recursive: true });
      chmodSync(path, directory.mode);
      fsyncDirectory(path);
      fsyncDirectory(dirname2(path));
    }
  }
  for (const [path, backup] of Object.entries(journal.backups).sort(([a], [b]) => generatorPathCompare(b, a))) {
    restoreBackup(repoRoot, plan, path, backupRoot, backup, restorePreparationName);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const regeneration of plan.regenerations.filter((entry) => journal.startedRegenerationIds.includes(entry.id))) {
    if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs))
      throw new Error(`Rollback regeneration pre-state is incomplete: ${regeneration.id}`);
  }
  for (const edit of [...plan.symlinkTargetEdits].reverse()) {
    if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId))
      continue;
    const link = absolutePath(repoRoot, edit.finalPath);
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
    if (lstatOrNull(link))
      durableRemove(link);
    if (lstatOrNull(stage)) {
      mkdirSync(dirname2(link), { recursive: true });
      durableRename(stage, link);
    }
    report(options.progress, "apply", "rolling-back-symlink-target-edits", 1, plan.symlinkTargetEdits.length, edit.finalPath);
    journal.installedSymlinkTargetEditIds = journal.installedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    journal.stagedSymlinkTargetEditIds = journal.stagedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    journal.preparedSymlinkTargetEditIds = journal.preparedSymlinkTargetEditIds.filter((id) => id !== edit.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const relocation of [...plan.embeddedTicketRootRelocations].reverse()) {
    if (!journal.installedEmbeddedRelocationIds.includes(relocation.operationId))
      continue;
    const destination = absolutePath(repoRoot, relocation.destinationPath);
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
    if (!lstatOrNull(stage) && lstatOrNull(destination)) {
      mkdirSync(dirname2(stage), { recursive: true });
      durableRename(destination, stage);
    }
    journal.installedEmbeddedRelocationIds = journal.installedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  const activeIds = new Set([...journal.stagedMoveIds, ...journal.installedMoveIds]);
  for (const move of [...plan.moves].reverse()) {
    if (!journal.installedMoveIds.includes(move.operationId))
      continue;
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const destination = absolutePath(repoRoot, move.destinationPath);
    if (!lstatOrNull(stage) && lstatOrNull(destination)) {
      mkdirSync(dirname2(stage), { recursive: true });
      durableRename(destination, stage);
    }
    journal.installedMoveIds = journal.installedMoveIds.filter((id) => id !== move.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const root of [...plan.embeddedTicketRoots].reverse()) {
    if (!journal.stagedEmbeddedRootIds.includes(root.operationId))
      continue;
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
    const source = absolutePath(repoRoot, root.sourceMetadataRoot);
    if (lstatOrNull(stage)) {
      if (lstatOrNull(source))
        throw new Error(`Rollback embedded root source is occupied: ${root.sourceMetadataRoot}`);
      mkdirSync(dirname2(source), { recursive: true });
      durableRename(stage, source);
    }
    journal.stagedEmbeddedRootIds = journal.stagedEmbeddedRootIds.filter((id) => id !== root.operationId);
    journal.preparedEmbeddedRootIds = journal.preparedEmbeddedRootIds.filter((id) => id !== root.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const relocation of [...plan.embeddedTicketRootRelocations].reverse()) {
    if (!journal.stagedEmbeddedRelocationIds.includes(relocation.operationId))
      continue;
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
    const source = absolutePath(repoRoot, relocation.sourcePath);
    if (lstatOrNull(stage)) {
      if (lstatOrNull(source))
        throw new Error(`Rollback relocation source is occupied: ${relocation.sourcePath}`);
      mkdirSync(dirname2(source), { recursive: true });
      durableRename(stage, source);
    }
    journal.stagedEmbeddedRelocationIds = journal.stagedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    journal.preparedEmbeddedRelocationIds = journal.preparedEmbeddedRelocationIds.filter((id) => id !== relocation.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const removal of [...plan.evidenceRemovals].reverse()) {
    if (!journal.stagedEvidenceRemovalIds.includes(removal.operationId))
      continue;
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
    const source = absolutePath(repoRoot, removal.sourcePath);
    if (lstatOrNull(stage)) {
      if (lstatOrNull(source))
        throw new Error(`Rollback removal source is occupied: ${removal.sourcePath}`);
      mkdirSync(dirname2(source), { recursive: true });
      durableRename(stage, source);
    }
    journal.stagedEvidenceRemovalIds = journal.stagedEvidenceRemovalIds.filter((id) => id !== removal.operationId);
    journal.preparedEvidenceRemovalIds = journal.preparedEvidenceRemovalIds.filter((id) => id !== removal.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  for (const move of [...plan.moves].reverse()) {
    if (!activeIds.has(move.operationId))
      continue;
    const stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
    const source = absolutePath(repoRoot, move.sourcePath);
    if (lstatOrNull(stage)) {
      mkdirSync(dirname2(source), { recursive: true });
      if (lstatOrNull(source))
        throw new Error(`Rollback source is occupied: ${move.sourcePath}`);
      durableRename(stage, source);
    }
    journal.stagedMoveIds = journal.stagedMoveIds.filter((id) => id !== move.operationId);
    journal.preparedMoveIds = journal.preparedMoveIds.filter((id) => id !== move.operationId);
    persistJournal(repoRoot, journalPath, journal);
  }
  journal.appliedEditPaths = [];
  journal.startedRegenerationIds = [];
  journal.completedRegenerationIds = [];
  journal.installedMoveIds = [];
  journal.stagedMoveIds = [];
  journal.preparedMoveIds = [];
  journal.preparedEmbeddedRelocationIds = [];
  journal.stagedEmbeddedRelocationIds = [];
  journal.installedEmbeddedRelocationIds = [];
  journal.preparedEvidenceRemovalIds = [];
  journal.stagedEvidenceRemovalIds = [];
  journal.preparedEmbeddedRootIds = [];
  journal.stagedEmbeddedRootIds = [];
  journal.preparedSymlinkTargetEditIds = [];
  journal.stagedSymlinkTargetEditIds = [];
  journal.installedSymlinkTargetEditIds = [];
  rollbackDestinationAncestors(repoRoot, plan);
  if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest)
    throw new Error("Rollback did not restore the affected pre-state digest");
  journal.state = "rolled-back";
  persistJournal(repoRoot, journalPath, journal);
  cleanupRolledBackTransaction(repoRoot, journal, plan);
}
function applyTaxonomyPlan(plan, options) {
  plan = parseTaxonomyPlan(plan);
  const repoRoot = resolve2(options.repoRoot);
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1))
    throw new Error("workers must be a positive integer");
  const digest = taxonomyPlanDigest(plan);
  if (plan.planDigest !== digest)
    throw new Error("Plan digest does not match canonical plan bytes");
  if (options.expectedPlanDigest !== undefined && options.expectedPlanDigest !== digest)
    throw new Error("Plan digest does not match expectedPlanDigest");
  if (plan.unresolved.some((entry) => entry.severity === "error"))
    throw new Error("Plan has unresolved blocking violations");
  const taxonomy = loadTaxonomy2({ repoRoot, taxonomyPath: options.taxonomyPath });
  const ticketRelative = normalizeRelative(isAbsolute(options.ticketDir) ? relative2(repoRoot, resolve2(options.ticketDir)) : options.ticketDir);
  if (isExcluded(ticketRelative, taxonomy))
    throw new Error(`Ticket directory is opaque: ${ticketRelative}`);
  const ticketRoot = absolutePath(repoRoot, ticketRelative);
  const transactionDirectory = canonicalDirectoryName(taxonomy, "taxonomy-transaction", "taxonomy-transaction");
  const digestDirectory = canonicalDirectoryName(taxonomy, "transaction-digest", digest, "taxonomy-transaction");
  const transactionRootRelative = normalizeRelative(`${ticketRelative}/${transactionDirectory}`);
  const transactionRelative = normalizeRelative(`${transactionRootRelative}/${digestDirectory}`);
  const attemptsDirectory = canonicalDirectoryName(taxonomy, "transaction-attempts", "attempts", "transaction-digest");
  const stageDirectory = canonicalDirectoryName(taxonomy, "transaction-stage", "stage", "transaction-attempt");
  const backupDirectory = canonicalDirectoryName(taxonomy, "transaction-backup", "backup", "transaction-attempt");
  const leaseDirectory = canonicalDirectoryName(taxonomy, "transaction-lease", "lease", "transaction-attempt");
  const journalWriteDirectory = canonicalDirectoryName(taxonomy, "transaction-journal-write", "journal", "transaction-stage");
  const attemptPreparationName = (ordinal, pid, token) => canonicalDirectoryName(taxonomy, "transaction-attempt-preparation", `prepare-${ordinal}-${pid}-${token}`, "transaction-attempts");
  const leasePreparationName = (pid, token, state) => canonicalDirectoryName(taxonomy, "transaction-lease-preparation", `lease-${pid}-${token}-${state}`, "transaction-backup");
  const journalJsonWritePreparationName = (pid, token) => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-journal-write");
  const leaseJsonWritePreparationName = (pid, token) => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-lease-preparation");
  const backupPreparationName = (identity, pid, token) => canonicalDirectoryName(taxonomy, "transaction-backup-preparation", `backup-${identity}-${pid}-${token}`, "transaction-backup");
  const backupWritePreparationName = (pid, token) => canonicalDirectoryName(taxonomy, "transaction-backup-write-preparation", `write-${pid}-${token}`, "transaction-backup-preparation");
  const restorePreparationName = (identity, pid, token) => canonicalDirectoryName(taxonomy, "transaction-restore-preparation", `restore-${identity}-${pid}-${token}`, "transaction-backup");
  const editPreparationName = (identity, pid, token) => canonicalDirectoryName(taxonomy, "transaction-edit-preparation", `edit-${identity}-${pid}-${token}`, "transaction-stage");
  const editWritePreparationName = (pid, token) => canonicalDirectoryName(taxonomy, "transaction-edit-write-preparation", `write-${pid}-${token}`, "transaction-edit-preparation");
  const attemptsRelative = normalizeRelative(`${transactionRelative}/${attemptsDirectory}`);
  const journalFilename = canonicalKindOnlyFilename(taxonomy, "json", ".json");
  const jsonPreviousName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-json-previous", "transaction-json-write-preparation", ".json");
  const backupWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-backup-write-candidate", "transaction-backup-write-preparation", ".backup");
  const editWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-edit-write-candidate", "transaction-edit-write-preparation", ".edit");
  const planBytes = Buffer.from(`${canonicalJson(plan)}
`);
  const planAuthority = (() => {
    const candidateRelative = options.planArtifactPath ? normalizeRelative(relative2(repoRoot, assertLexicalInputOutsideOpaque(repoRoot, options.planArtifactPath, "planArtifactPath", true)).replaceAll("\\", "/")) : normalizeRelative(`${ticketRelative}/\uD83D\uDCCA\uFE0Ftaxonomy-plan/\uD83D\uDD23\uFE0F.json`);
    const candidate = absolutePath(repoRoot, candidateRelative);
    if (!options.planArtifactPath)
      assertNoFollowAncestors(repoRoot, candidate, "canonical plan artifact", true);
    const stat = lstatOrNull(candidate);
    if (options.planArtifactPath && (!stat?.isFile() || stat.isSymbolicLink() || !readFileSync2(candidate).equals(planBytes)))
      throw new Error("planArtifactPath must be a regular no-follow file containing the exact canonical plan bytes");
    return stat?.isFile() && !stat.isSymbolicLink() && readFileSync2(candidate).equals(planBytes) ? { path: candidateRelative, bytes: planBytes } : undefined;
  })();
  const resumeRelative = options.resumeJournal ? normalizeRelative(isAbsolute(options.resumeJournal) ? relative2(repoRoot, resolve2(options.resumeJournal)) : options.resumeJournal) : undefined;
  assertPlanOutsideTransaction(plan, transactionRootRelative, taxonomy, repoRoot);
  if (options.cancelFile) {
    const cancelAbsolute = assertLexicalInputOutsideOpaque(repoRoot, options.cancelFile, "cancelFile", true);
    const cancelRelative = normalizeRelative(relative2(repoRoot, cancelAbsolute).replaceAll("\\", "/"));
    const mutationPaths = [
      ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]),
      ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...entry.authority.kind === "byte-and-mode-identical" ? entry.authority.members.flatMap((member) => [member.sourcePath, member.finalPath]) : [entry.authority.fixturePath]]),
      ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
      ...plan.edits.map((entry) => entry.path),
      ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals])
    ];
    if (pathsOverlap(cancelRelative, transactionRootRelative) || mutationPaths.some((path) => pathsOverlap(cancelRelative, path)))
      throw new Error(`cancelFile overlaps transaction or mutation authority: ${cancelRelative}`);
  }
  const existingAttempts = [];
  const unpublishedAttempts = [];
  const attemptsAbsolute = absolutePath(repoRoot, attemptsRelative);
  assertNoFollowAncestors(repoRoot, attemptsAbsolute, "transaction attempts root", true);
  const transactionRootAbsolute = absolutePath(repoRoot, transactionRootRelative);
  const transactionRootStat = lstatOrNull(transactionRootAbsolute);
  if (transactionRootStat) {
    if (!transactionRootStat.isDirectory() || transactionRootStat.isSymbolicLink())
      throw new Error("Taxonomy transaction root must be a no-follow directory");
    for (const name of readdirSync2(transactionRootAbsolute).sort(generatorPathCompare)) {
      const childDigest = splitLeadingEmoji(name).rest;
      if (!PLAN_HASH.test(childDigest) || name !== canonicalDirectoryName(taxonomy, "transaction-digest", childDigest, "taxonomy-transaction"))
        throw new Error(`Unexpected taxonomy transaction-root entry: ${name}`);
      const stat = lstatOrNull(join2(transactionRootAbsolute, name));
      if (!stat?.isDirectory() || stat.isSymbolicLink())
        throw new Error(`Transaction digest must be a no-follow directory: ${name}`);
    }
  }
  const digestAbsolute = absolutePath(repoRoot, transactionRelative);
  const digestStat = lstatOrNull(digestAbsolute);
  if (digestStat) {
    if (!digestStat.isDirectory() || digestStat.isSymbolicLink())
      throw new Error("Selected transaction digest must be a no-follow directory");
    const digestChildren = readdirSync2(digestAbsolute).sort(generatorPathCompare);
    if (digestChildren.some((name) => name !== attemptsDirectory))
      throw new Error("Selected transaction digest contains an unexpected artifact");
  }
  const attemptsStat = lstatOrNull(attemptsAbsolute);
  if (attemptsStat) {
    if (!attemptsStat.isDirectory() || attemptsStat.isSymbolicLink())
      throw new Error("Transaction attempts authority must be a no-follow directory");
    for (const name of readdirSync2(attemptsAbsolute).sort(generatorPathCompare)) {
      const childSlug = splitLeadingEmoji(name).rest;
      const preparation = /^prepare-([0-9]{6})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(childSlug);
      if (preparation) {
        const ordinal2 = preparation[1], pid = Number.parseInt(preparation[2], 10), token = preparation[3];
        if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== attemptPreparationName(ordinal2, pid, token))
          throw new Error(`Unexpected transaction attempt preparation: ${name}`);
        const path = join2(attemptsAbsolute, name);
        const stat = lstatOrNull(path);
        if (!stat?.isDirectory() || stat.isSymbolicLink())
          throw new Error(`Transaction attempt preparation must be a no-follow directory: ${name}`);
        unpublishedAttempts.push({ ordinal: ordinal2, pid, token, path });
        continue;
      }
      const ordinal = childSlug;
      if (!/^[0-9]{6}$/u.test(ordinal) || name !== canonicalDirectoryName(taxonomy, "transaction-attempt", ordinal, "transaction-attempts"))
        throw new Error(`Unexpected transaction attempt entry: ${name}`);
      const attemptRelative2 = normalizeRelative(`${attemptsRelative}/${name}`);
      const attemptAbsolute = absolutePath(repoRoot, attemptRelative2);
      const attemptStat = lstatOrNull(attemptAbsolute);
      if (!attemptStat?.isDirectory() || attemptStat.isSymbolicLink())
        throw new Error(`Transaction attempt must be a no-follow directory: ${attemptRelative2}`);
      const childNames = readdirSync2(attemptAbsolute).sort(generatorPathCompare);
      if (!childNames.includes(journalFilename))
        throw new Error(`Transaction attempt has no durable journal: ${attemptRelative2}`);
      if (childNames.some((child) => child !== journalFilename && child !== stageDirectory && child !== backupDirectory && child !== leaseDirectory))
        throw new Error(`Transaction attempt contains an unexpected artifact: ${attemptRelative2}`);
      const attemptJournalRelative = normalizeRelative(`${attemptRelative2}/${journalFilename}`);
      const attemptJournalAbsolute = absolutePath(repoRoot, attemptJournalRelative);
      const journalStat = lstatOrNull(attemptJournalAbsolute);
      if (!journalStat?.isFile() || journalStat.isSymbolicLink())
        throw new Error(`Transaction attempt journal must be a regular no-follow file: ${attemptJournalRelative}`);
      const attemptJournal = readJournal(attemptJournalAbsolute, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
      if (attemptJournal.planDigest !== digest || attemptJournal.attemptOrdinal !== ordinal)
        throw new Error(`Transaction attempt identity does not match its canonical path: ${attemptJournalRelative}`);
      const expectedStaging = normalizeRelative(`${attemptRelative2}/${stageDirectory}`);
      const expectedBackup = normalizeRelative(`${attemptRelative2}/${backupDirectory}`);
      if (attemptJournal.stagingRoot !== expectedStaging || attemptJournal.backupRoot !== expectedBackup)
        throw new Error(`Transaction attempt roots do not match ordinal ${ordinal}`);
      const stageStat = lstatOrNull(absolutePath(repoRoot, expectedStaging));
      const backupStat = lstatOrNull(absolutePath(repoRoot, expectedBackup));
      const leaseStat = lstatOrNull(join2(attemptAbsolute, leaseDirectory));
      if (stageStat && (!stageStat.isDirectory() || stageStat.isSymbolicLink()) || backupStat && (!backupStat.isDirectory() || backupStat.isSymbolicLink()))
        throw new Error(`Transaction attempt stage/backup must be direct no-follow directories: ${ordinal}`);
      if (leaseStat && (!leaseStat.isDirectory() || leaseStat.isSymbolicLink()))
        throw new Error(`Transaction attempt lease must be a direct no-follow directory: ${ordinal}`);
      if (attemptJournal.state !== "rolled-back" && attemptJournal.state !== "committed" && (!stageStat || !backupStat))
        throw new Error(`Active transaction attempt is missing stage/backup roots: ${ordinal}`);
      assertJournalPhaseMembership(plan, attemptJournal);
      assertJournalBackupAuthority(plan, attemptJournal);
      existingAttempts.push({ ordinal, attemptRelative: attemptRelative2, journal: attemptJournal, journalRelative: attemptJournalRelative });
    }
  }
  for (let index = 0;index < existingAttempts.length; index++) {
    if (existingAttempts[index].ordinal !== String(index + 1).padStart(6, "0"))
      throw new Error("Transaction attempt ordinals are not contiguous");
    if (index < existingAttempts.length - 1 && existingAttempts[index].journal.state !== "rolled-back")
      throw new Error("Only rolled-back attempts may precede another transaction attempt");
  }
  const validateUnpublishedAttempt = (preparation) => {
    if (transactionLeaseProcessIsAlive(preparation.pid))
      throw new Error(`Transaction attempt preparation is active for pid ${preparation.pid}`);
    const allowed = new Set([stageDirectory, backupDirectory, leaseDirectory, journalFilename]);
    const children = readdirSync2(preparation.path).sort(generatorPathCompare);
    if (children.some((name) => !allowed.has(name)))
      throw new Error(`Dead transaction attempt preparation contains unexpected evidence: ${basename2(preparation.path)}`);
    const finalAttemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", preparation.ordinal, "transaction-attempts");
    const finalAttempt = normalizeRelative(`${attemptsRelative}/${finalAttemptDirectory}`);
    const assertPreparedIdentity = (prepared, path) => {
      if (prepared.revision !== 0 || prepared.state !== "prepared" || prepared.planDigest !== digest || prepared.attemptOrdinal !== preparation.ordinal || prepared.stagingRoot !== normalizeRelative(`${finalAttempt}/${stageDirectory}`) || prepared.backupRoot !== normalizeRelative(`${finalAttempt}/${backupDirectory}`))
        throw new Error(`Dead attempt preparation journal identity is invalid: ${path}`);
      assertJournalPlanMembership(plan, prepared);
      assertJournalPhaseMembership(plan, prepared);
      assertJournalBackupAuthority(plan, prepared);
    };
    for (const name of children) {
      const path = join2(preparation.path, name);
      const stat = lstatOrNull(path);
      if (name === journalFilename) {
        if (!stat?.isFile() || stat.isSymbolicLink())
          throw new Error(`Dead attempt preparation journal is not a no-follow file: ${path}`);
        assertPreparedIdentity(readJournal(path, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), path);
      } else {
        if (!stat?.isDirectory() || stat.isSymbolicLink())
          throw new Error(`Dead attempt preparation child is not a no-follow directory: ${path}`);
        if (name === leaseDirectory) {
          recoverCanonicalJsonCandidates(path, journalFilename, jsonPreviousName, leaseJsonWritePreparationName, (candidate) => {
            const lease = parseTransactionLease(candidate, digest, preparation.ordinal);
            if (lease.pid !== preparation.pid)
              throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`);
          }, false, true);
          if (lstatOrNull(join2(path, journalFilename))) {
            const lease = parseTransactionLease(join2(path, journalFilename), digest, preparation.ordinal);
            if (lease.pid !== preparation.pid)
              throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`);
          }
        } else if (name === stageDirectory && readdirSync2(path).length > 0) {
          const nested = readdirSync2(path).sort(generatorPathCompare);
          if (canonicalJson(nested) !== canonicalJson([journalWriteDirectory]))
            throw new Error(`Dead attempt preparation stage contains unexpected evidence: ${path}`);
          const wal = join2(path, journalWriteDirectory);
          const walStat = lstatOrNull(wal);
          if (!walStat?.isDirectory() || walStat.isSymbolicLink())
            throw new Error(`Dead attempt preparation WAL is invalid: ${wal}`);
          recoverCanonicalJsonCandidates(wal, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidate) => assertPreparedIdentity(readJournal(candidate, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), candidate), false, true);
          const walChildren = readdirSync2(wal).sort(generatorPathCompare);
          if (walChildren.length > 0) {
            if (children.includes(journalFilename))
              throw new Error(`Dead attempt preparation has both a durable journal and a pending initial WAL: ${wal}`);
            const walPath = join2(wal, journalFilename);
            if (lstatOrNull(walPath)) {
              if (lstatSync(walPath).isSymbolicLink())
                throw new Error(`Dead attempt preparation WAL snapshot is not a no-follow file: ${walPath}`);
              assertPreparedIdentity(readJournal(walPath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), walPath);
            }
          }
        } else if (readdirSync2(path).length > 0)
          throw new Error(`Dead attempt preparation executor root is not empty: ${path}`);
      }
    }
  };
  for (const preparation of unpublishedAttempts)
    validateUnpublishedAttempt(preparation);
  for (const preparation of unpublishedAttempts)
    durableRemove(preparation.path, true);
  for (const attempt of existingAttempts) {
    if (attempt.journal.state !== "rolled-back" && attempt.journal.state !== "committed")
      continue;
    let terminalJournal = attempt.journal;
    const attemptAbsolute = absolutePath(repoRoot, attempt.attemptRelative);
    const stageAbsolute = absolutePath(repoRoot, attempt.journal.stagingRoot);
    const backupAbsolute = absolutePath(repoRoot, attempt.journal.backupRoot);
    const leaseAbsolute = join2(attemptAbsolute, leaseDirectory);
    const hasResidue = Boolean(lstatOrNull(stageAbsolute) || lstatOrNull(backupAbsolute));
    let terminalLease;
    let createdLeaseBackup = false;
    if (hasResidue || lstatOrNull(leaseAbsolute)) {
      if (!hasResidue && attempt.journal.state === "committed" && actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest)
        throw new Error(`Committed attempt post-state changed before stale lease recovery: ${attempt.ordinal}`);
      if (!lstatOrNull(backupAbsolute)) {
        mkdirSync(backupAbsolute);
        fsyncDirectory(attemptAbsolute);
        createdLeaseBackup = true;
      }
      terminalLease = acquireTransactionLease(repoRoot, attempt.attemptRelative, attempt.journal.backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attempt.ordinal);
      if (!hasResidue && createdLeaseBackup)
        durableRemove(backupAbsolute, true);
      terminalJournal = reconcileJournalWal(repoRoot, absolutePath(repoRoot, attempt.journalRelative), terminalJournal, plan, taxonomy);
    }
    try {
      if (terminalJournal.state === "rolled-back")
        cleanupRolledBackTransaction(repoRoot, terminalJournal, plan);
      else {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest)
          throw new Error(`Committed attempt post-state changed: ${attempt.ordinal}`);
        cleanupCommittedTransaction(repoRoot, terminalJournal, plan);
      }
    } finally {
      if (terminalLease)
        releaseTransactionLease(terminalLease);
    }
    if (canonicalJson(readdirSync2(attemptAbsolute).sort(generatorPathCompare)) !== canonicalJson([journalFilename]))
      throw new Error(`Terminal transaction attempt is not closed: ${attempt.ordinal}`);
  }
  let attemptOrdinal;
  let journalRelative;
  let selectedAttempt;
  if (resumeRelative) {
    const match = existingAttempts.find((entry) => entry.journalRelative === resumeRelative);
    if (!match)
      throw new Error(`Resume journal is not an exact existing canonical attempt for plan ${digest}`);
    selectedAttempt = match;
    attemptOrdinal = match.ordinal;
    journalRelative = match.journalRelative;
  } else {
    const active = existingAttempts.find((entry) => entry.journal.state !== "rolled-back" && entry.journal.state !== "committed");
    if (active)
      throw new Error(`Transaction attempt ${active.ordinal} is active and must be resumed`);
    if (existingAttempts.some((entry) => entry.journal.state === "committed"))
      throw new Error("Plan already has a committed transaction attempt");
    const next = existingAttempts.length === 0 ? 1 : Math.max(...existingAttempts.map((entry) => Number.parseInt(entry.ordinal, 10))) + 1;
    if (next > 999999)
      throw new Error("Transaction attempt ordinal space is exhausted");
    attemptOrdinal = String(next).padStart(6, "0");
    const attemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", attemptOrdinal, "transaction-attempts");
    journalRelative = normalizeRelative(`${attemptsRelative}/${attemptDirectory}/${journalFilename}`);
  }
  const attemptRelative = posix.dirname(journalRelative);
  const journalPath = absolutePath(repoRoot, journalRelative);
  let leaseHandle;
  const acquireLease = (backupRoot, beforePublish) => {
    leaseHandle = acquireTransactionLease(repoRoot, attemptRelative, backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attemptOrdinal, beforePublish);
  };
  const releaseLease = () => {
    if (!leaseHandle)
      return;
    releaseTransactionLease(leaseHandle);
    leaseHandle = undefined;
  };
  for (const edit of plan.symlinkTargetEdits) {
    const localTarget = repositoryLocalSymlinkTargetPath(repoRoot, edit.oldTarget);
    if (localTarget !== edit.logicalTargetSourcePath)
      throw new Error(`Symlink target authority is not repository-local or does not match its logical source: ${edit.sourcePath}`);
    const linkMoves = plan.moves.filter((move) => move.sourcePath === edit.sourcePath);
    const targetMoves = plan.moves.filter((move) => move.sourcePath === edit.logicalTargetSourcePath);
    const expectedFinalPath = linkMoves.length === 0 ? edit.sourcePath : linkMoves.length === 1 ? linkMoves[0].destinationPath : "";
    const expectedTargetFinalPath = targetMoves.length === 0 ? edit.logicalTargetSourcePath : targetMoves.length === 1 ? targetMoves[0].destinationPath : "";
    if (edit.finalPath !== expectedFinalPath || edit.logicalTargetFinalPath !== expectedTargetFinalPath)
      throw new Error(`Symlink target projection does not match exact plan moves: ${edit.sourcePath}`);
    const expectedTarget = posix.relative(posix.dirname(expectedFinalPath), expectedTargetFinalPath);
    if (!expectedTarget || expectedTarget !== edit.newTarget || expectedTarget.startsWith("/") || posix.normalize(posix.join(posix.dirname(expectedFinalPath), expectedTarget)) !== expectedTargetFinalPath)
      throw new Error(`Symlink relative target does not resolve to its frozen logical target: ${edit.sourcePath}`);
    if (edit.logicalTargetPreimage.state === "directory" || edit.windowsLinkType !== "file")
      throw new Error(`Symlink directory target lacks recursive no-follow authority: ${edit.sourcePath}`);
    if (edit.logicalTargetPreimage.state === "absent" && !resolveFileKind(edit.logicalTargetSourcePath, taxonomy, [], []).kind)
      throw new Error(`Broken symlink target kind cannot be proven: ${edit.sourcePath}`);
    const targetDigestible = { sourcePath: edit.sourcePath, finalPath: edit.finalPath, oldTarget: edit.oldTarget, newTarget: edit.newTarget, logicalTargetSourcePath: edit.logicalTargetSourcePath, logicalTargetFinalPath: edit.logicalTargetFinalPath, logicalTargetPreimage: edit.logicalTargetPreimage };
    if (edit.sourceTargetDigest !== sha256(canonicalJson(targetDigestible)))
      throw new Error(`Symlink source-target authority digest changed: ${edit.sourcePath}`);
  }
  execFileSync("git", ["cat-file", "-e", `${plan.baselineCommit}^{commit}`], { cwd: repoRoot, stdio: "ignore" });
  if (plan.excludedTreeDigests.length > 0)
    throw new Error("Opaque digest filesystem access is disabled; replan with empty excludedTreeDigests");
  if (!options.resumeJournal)
    checkCancellation(repoRoot, options.cancelFile);
  if (!options.resumeJournal) {
    if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest)
      throw new Error("Affected pre-state digest does not match plan expectation");
    for (const path of [...plan.moves.map((entry) => entry.destinationPath), ...plan.embeddedTicketRootRelocations.map((entry) => entry.destinationPath), ...plan.symlinkTargetEdits.map((entry) => entry.finalPath), ...plan.edits.map((entry) => entry.path), ...plan.regenerations.flatMap((entry) => entry.outputRoots)])
      assertWritableAncestors(repoRoot, path);
    const referenceInventory = inventoryTaxonomy({ repoRoot, scope: plan.scope, ticketDir: options.ticketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile });
    const moveSources = new Set(plan.moves.map((move) => move.sourcePath));
    for (const move of plan.moves) {
      const source = absolutePath(repoRoot, move.sourcePath);
      assertLeafPreimage(repoRoot, move.sourcePath, move.sourcePreimage);
      if (lstatOrNull(absolutePath(repoRoot, move.destinationPath)) && !moveSources.has(move.destinationPath))
        throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      assertLeafPreimage(repoRoot, relocation.sourcePath, relocation.preimage);
      if (lstatOrNull(absolutePath(repoRoot, relocation.destinationPath)))
        throw new Error(`Embedded relocation destination is occupied: ${relocation.destinationPath}`);
    }
    for (const removal of plan.evidenceRemovals)
      assertLeafPreimage(repoRoot, removal.sourcePath, removal.preimage);
    for (const removal of plan.evidenceRemovals) {
      if (removal.authority.kind === "byte-and-mode-identical")
        for (const member of removal.authority.members)
          assertLeafPreimage(repoRoot, member.sourcePath, member.preimage);
      else {
        const fixture = serializedSentinelCases(repoRoot);
        const sentinel = fixture?.cases.find((entry) => entry.id === removal.authority.caseId);
        if (removal.authority.fixturePath !== TRANSACTION_DISPOSITIONS_FIXTURE_PATH || !fixture || fixture.fixtureContentHash !== removal.authority.fixtureContentHash || !sentinel || sentinel.inputPath !== removal.authority.serializedInputPath || sentinel.physicalSourcePath !== removal.sourcePath || sentinel.expectedViolationCode !== removal.authority.expectedViolationCode || sentinel.sourceContentHash !== removal.preimage.contentHash)
          throw new Error(`Serialized sentinel authority changed: ${removal.authority.caseId}`);
      }
    }
    for (const edit of plan.symlinkTargetEdits) {
      const link = absolutePath(repoRoot, edit.sourcePath);
      if (!lstatOrNull(link)?.isSymbolicLink() || readlinkSync(link) !== edit.oldTarget)
        throw new Error(`Symlink target preimage changed: ${edit.sourcePath}`);
      const logical = lstatOrNull(absolutePath(repoRoot, edit.logicalTargetSourcePath));
      if (edit.logicalTargetPreimage.state === "absent" && logical || edit.logicalTargetPreimage.state === "directory" && !logical?.isDirectory() || (edit.logicalTargetPreimage.state === "file" || edit.logicalTargetPreimage.state === "symlink") && (!logical || canonicalJson(leafPathPreimage(absolutePath(repoRoot, edit.logicalTargetSourcePath))) !== canonicalJson(edit.logicalTargetPreimage)))
        throw new Error(`Logical symlink target preimage changed: ${edit.logicalTargetSourcePath}`);
    }
    for (const root of plan.embeddedTicketRoots) {
      if (canonicalJson(noFollowTreeDigest(repoRoot, root.sourceMetadataRoot)) !== canonicalJson(root.sourceTreeDigest))
        throw new Error(`Embedded root tree preimage changed: ${root.sourceMetadataRoot}`);
      const children = [
        ...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath),
        ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath)
      ].sort(generatorPathCompare);
      if (children.some((path) => !path.startsWith(`${root.sourceTicketRoot}/`) || !path.startsWith(`${root.sourceMetadataRoot}/`)) || canonicalJson(noFollowTreeDigestExcluding(repoRoot, root.sourceMetadataRoot, children)) !== canonicalJson(root.residualTreeDigest))
        throw new Error(`Embedded root residual authority changed: ${root.sourceMetadataRoot}`);
      const incoming = incomingEmbeddedReferences(referenceInventory, root.sourceMetadataRoot).filter((row) => {
        const source = row.split("\x00")[1];
        if (!planAuthority || source !== planAuthority.path)
          return true;
        const stat = lstatOrNull(absolutePath(repoRoot, source));
        return !stat?.isFile() || stat.isSymbolicLink() || !readFileSync2(absolutePath(repoRoot, source)).equals(planAuthority.bytes);
      });
      if (sha256(`sha256-taxonomy-reference-set-v1\x00${canonicalJson(incoming)}`) !== root.incomingReferenceDigest || incoming.length > 0)
        throw new Error(`Embedded root incoming reference set changed: ${root.sourceMetadataRoot}`);
      const lexicalIncoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.ticketDir, planAuthority, transactionRootRelative);
      if (lexicalIncoming.length > 0)
        throw new Error(`Embedded root structured incoming reference set changed: ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      const incoming = lexicalTargetIncomingReferences(repoRoot, new Set([removal.sourcePath]), [removal.sourcePath], taxonomy, options.ticketDir, planAuthority, transactionRootRelative);
      if (incoming.length > 0)
        throw new Error(`Evidence-removal structured incoming reference set changed: ${removal.sourcePath}`);
    }
    {
      const authorityInventory = inventoryWithoutTransactionEvidence(referenceInventory, transactionRootRelative, planAuthority?.path);
      const authorityPlan = planTaxonomy(authorityInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile });
      const operationSets = [
        ["affected pre-state digest", plan.expectedAffectedPreStateDigest, authorityPlan.expectedAffectedPreStateDigest],
        ["affected post-state digest", plan.expectedPostStateDigest, authorityPlan.expectedPostStateDigest],
        ["moves", plan.moves, authorityPlan.moves],
        ["embedded roots", plan.embeddedTicketRoots, authorityPlan.embeddedTicketRoots],
        ["embedded relocations", plan.embeddedTicketRootRelocations, authorityPlan.embeddedTicketRootRelocations],
        ["symlink target edits", plan.symlinkTargetEdits, authorityPlan.symlinkTargetEdits],
        ["evidence removals", plan.evidenceRemovals, authorityPlan.evidenceRemovals],
        ["destination ancestor preimages", plan.destinationAncestorPreimages, authorityPlan.destinationAncestorPreimages],
        ["reference edits", plan.edits, authorityPlan.edits],
        ["regenerations", plan.regenerations, authorityPlan.regenerations],
        ["unresolved findings", plan.unresolved, authorityPlan.unresolved]
      ];
      const mismatch = operationSets.find(([, submitted, derived]) => canonicalJson(submitted) !== canonicalJson(derived));
      if (mismatch)
        throw new Error(`Plan ${mismatch[0]} cannot be rederived exactly from current schema-owned authority`);
    }
  }
  let journal;
  if (options.resumeJournal) {
    try {
      const journalStat = lstatOrNull(journalPath);
      if (!journalStat?.isFile() || journalStat.isSymbolicLink())
        throw new Error(`Resume journal must be a regular no-follow file: ${journalRelative}`);
      if (!selectedAttempt)
        throw new Error("Resume journal attempt was not selected from canonical history");
      journal = selectedAttempt.journal;
      if (journal.state !== "committed" && journal.state !== "rolled-back") {
        const validateRecoveryEvidence = () => {
          assertRecoveryRootNames(repoRoot, plan, journal, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
          validateLeasePreparationEvidence(absolutePath(repoRoot, journal.backupRoot), leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attemptOrdinal);
          recoverTransactionBackups(repoRoot, plan, journal, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
          recoverRestorePreparations(repoRoot, plan, journal, restorePreparationName, true);
          recoverReferenceEditPreparations(repoRoot, plan, journal, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
          reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy, true);
          const tupleProbe = { ...journal, ...Object.fromEntries(JOURNAL_OPERATION_ARRAYS.map((key) => [key, [...journal[key]]])), backups: { ...journal.backups } };
          try {
            if (journal.state === "rolling-back")
              reconcileRollbackTuples(repoRoot, plan, tupleProbe, taxonomy);
            else
              validateResumeTuples(repoRoot, plan, tupleProbe, taxonomy);
          } catch (error) {
            if (!(error instanceof TaxonomyStartedRegenerationPartialError))
              throw error;
          }
        };
        validateRecoveryEvidence();
        acquireLease(journal.backupRoot, validateRecoveryEvidence);
        validateRecoveryEvidence();
        const recovered = { ...journal, startedRegenerationIds: [...journal.startedRegenerationIds], backups: { ...journal.backups } };
        const recoveredBackups = recoverTransactionBackups(repoRoot, plan, recovered, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
        recoverRestorePreparations(repoRoot, plan, recovered, restorePreparationName);
        recoverReferenceEditPreparations(repoRoot, plan, recovered, editPreparationName, editWritePreparationName, editWriteCandidateName);
        journal = reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy);
        if (recoveredBackups) {
          let changed = false;
          for (const [path, backup] of Object.entries(recovered.backups)) {
            if (journal.backups[path]) {
              if (canonicalJson(journal.backups[path]) !== canonicalJson(backup))
                throw new Error(`Recovered backup differs from promoted journal authority: ${path}`);
            } else {
              journal.backups[path] = backup;
              changed = true;
            }
          }
          for (const id of recovered.startedRegenerationIds)
            if (!journal.startedRegenerationIds.includes(id)) {
              journal.startedRegenerationIds.push(id);
              changed = true;
            }
          if (changed)
            persistJournal(repoRoot, journalPath, journal);
        }
      }
      assertJournalBackupAuthority(plan, journal);
      const expectedStagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
      const expectedBackupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
      if (journal.attemptOrdinal !== attemptOrdinal || journal.stagingRoot !== expectedStagingRoot || journal.backupRoot !== expectedBackupRoot)
        throw new Error("Resume journal attempt identity and transaction roots do not match the canonical plan attempt");
      assertJournalPlanMembership(plan, journal);
      assertJournalPhaseMembership(plan, journal);
      if (journal.planDigest !== digest)
        throw new Error("Resume journal belongs to a different plan");
      if (journal.state !== "committed" && journal.state !== "rolled-back")
        assertActiveTransactionEvidence(repoRoot, plan, journal, false);
      if (journal.state === "committed") {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest)
          throw new Error("Committed resume post-state digest changed");
        cleanupCommittedTransaction(repoRoot, journal, plan);
        return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
      }
      if (journal.state === "rolled-back")
        throw new Error(`Cannot resume journal in state ${journal.state}`);
      if (journal.state === "rolling-back") {
        try {
          rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options);
        } finally {
          releaseLease();
        }
        return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
      }
      if (cancellationRequested(repoRoot, options.cancelFile)) {
        try {
          rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options);
        } finally {
          releaseLease();
        }
        return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
      }
      for (const root of plan.embeddedTicketRoots) {
        const incoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.ticketDir, planAuthority, transactionRootRelative);
        if (incoming.length > 0)
          throw new Error(`resume-state-drift: embedded incoming references ${root.sourceMetadataRoot}`);
      }
      for (const removal of plan.evidenceRemovals) {
        const incoming = lexicalTargetIncomingReferences(repoRoot, new Set([removal.sourcePath]), [removal.sourcePath], taxonomy, options.ticketDir, planAuthority, transactionRootRelative);
        if (incoming.length > 0)
          throw new Error(`resume-state-drift: evidence-removal incoming references ${removal.sourcePath}`);
      }
      try {
        if (validateResumeTuples(repoRoot, plan, journal, taxonomy))
          persistJournal(repoRoot, journalPath, journal);
      } catch (error) {
        if (!(error instanceof TaxonomyStartedRegenerationPartialError))
          throw error;
        journal.state = "rolling-back";
        journal.error = error.message;
        persistJournal(repoRoot, journalPath, journal);
        try {
          rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options);
        } finally {
          releaseLease();
        }
        return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
      }
      assertActiveTransactionEvidence(repoRoot, plan, journal, true);
    } catch (error) {
      releaseLease();
      throw error;
    }
  } else {
    for (const regeneration of plan.regenerations) {
      const actualInputs = regeneration.inputs.map((input) => generatorNodeRecord(repoRoot, input.path, taxonomy));
      if (canonicalJson(actualInputs) !== canonicalJson(regeneration.inputs))
        throw new Error(`Regeneration input preimage changed: ${regeneration.id}`);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.preOutputs))
        throw new Error(`Regeneration output preimage changed: ${regeneration.id}`);
    }
    const stagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
    const backupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
    journal = { schemaVersion: 2, revision: 0, planDigest: digest, attemptOrdinal, state: "prepared", stagingRoot, backupRoot, journalWriteDirectory, jsonWritePreparationName: journalJsonWritePreparationName, preparedMoveIds: [], stagedMoveIds: [], installedMoveIds: [], preparedEmbeddedRelocationIds: [], stagedEmbeddedRelocationIds: [], installedEmbeddedRelocationIds: [], preparedEvidenceRemovalIds: [], stagedEvidenceRemovalIds: [], preparedEmbeddedRootIds: [], stagedEmbeddedRootIds: [], preparedSymlinkTargetEditIds: [], stagedSymlinkTargetEditIds: [], installedSymlinkTargetEditIds: [], appliedEditPaths: [], startedRegenerationIds: [], completedRegenerationIds: [], backups: {} };
    checkCancellation(repoRoot, options.cancelFile);
    assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, attemptsRelative), "transaction attempts root", true);
    mkdirSync(attemptsAbsolute, { recursive: true });
    fsyncDirectory(dirname2(attemptsAbsolute));
    const preparationToken = randomUUID();
    const preparationRelative = normalizeRelative(`${attemptsRelative}/${attemptPreparationName(attemptOrdinal, process.pid, preparationToken)}`);
    const preparationRoot = absolutePath(repoRoot, preparationRelative);
    const preparationStage = join2(preparationRoot, stageDirectory);
    const preparationBackup = join2(preparationRoot, backupDirectory);
    const preparationLease = join2(preparationRoot, leaseDirectory);
    const leaseRecord = { schemaVersion: 1, planDigest: digest, attemptOrdinal, token: randomUUID(), pid: process.pid };
    try {
      mkdirSync(preparationRoot);
    } catch (error) {
      if (error.code === "EEXIST")
        throw new Error(`Transaction attempt preparation collision at ${preparationRelative}`);
      throw error;
    }
    fsyncDirectory(attemptsAbsolute);
    try {
      mkdirSync(preparationStage);
      mkdirSync(preparationBackup);
      mkdirSync(preparationLease);
      publishCanonicalJsonCandidate(preparationLease, journalFilename, jsonPreviousName, leaseRecord, leaseJsonWritePreparationName);
      const initialWalRoot = join2(preparationStage, journalWriteDirectory);
      const initialWal = join2(initialWalRoot, journalFilename);
      mkdirSync(initialWalRoot);
      publishCanonicalJsonCandidate(initialWalRoot, journalFilename, jsonPreviousName, journalSnapshot(journal), journalJsonWritePreparationName);
      durableRename(initialWal, join2(preparationRoot, journalFilename));
      durableRemove(initialWalRoot, true);
      fsyncFile(join2(preparationRoot, journalFilename));
      fsyncDirectory(preparationStage);
      fsyncDirectory(preparationBackup);
      fsyncDirectory(preparationRoot);
      durableRename(preparationRoot, absolutePath(repoRoot, attemptRelative));
    } catch (error) {
      if (lstatOrNull(preparationRoot))
        durableRemove(preparationRoot, true);
      if (error.code === "EEXIST" || error.code === "ENOTEMPTY")
        throw new Error(`Transaction attempt allocation race at ${attemptRelative}`);
      throw error;
    }
    leaseHandle = { root: absolutePath(repoRoot, `${attemptRelative}/${leaseDirectory}`), filename: journalFilename, record: leaseRecord };
  }
  const sourceSet = new Set(plan.moves.map((move) => move.sourcePath));
  try {
    checkCancellation(repoRoot, options.cancelFile);
    for (const move of plan.moves) {
      if (journal.stagedMoveIds.includes(move.operationId)) {
        const candidates = journal.installedMoveIds.includes(move.operationId) ? [absolutePath(repoRoot, move.destinationPath), join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)] : [join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)];
        const resumedPath = candidates.find((path) => lstatOrNull(path));
        if (!resumedPath)
          throw new Error(`Resume move state is invalid: ${move.operationId}`);
        const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
        if (!journal.appliedEditPaths.includes(move.destinationPath) && canonicalJson(leafPreimage(resumedPath)) !== canonicalJson(retargetedMovePreimage(move, installedLink)))
          throw new Error(`Resume move preimage changed: ${move.operationId}`);
        continue;
      }
      const source = absolutePath(repoRoot, move.sourcePath);
      const destination = absolutePath(repoRoot, move.destinationPath);
      const sourceStat = lstatOrNull(source);
      if (!sourceStat)
        throw new Error(`Move source is missing: ${move.sourcePath}`);
      if (canonicalJson(leafPreimage(source)) !== canonicalJson(move.sourcePreimage))
        throw new Error(`Move source preimage changed: ${move.sourcePath}`);
      if (lstatOrNull(destination) && !sourceSet.has(move.destinationPath))
        throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    journal.state = "staging";
    persistJournal(repoRoot, journalPath, journal);
    for (let index = 0;index < plan.moves.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const move = plan.moves[index];
      const stage = join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
      if (!journal.preparedMoveIds.includes(move.operationId)) {
        journal.preparedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!lstatOrNull(stage) && !journal.installedMoveIds.includes(move.operationId)) {
        mkdirSync(dirname2(stage), { recursive: true });
        durableRename(absolutePath(repoRoot, move.sourcePath), stage);
      }
      if (!journal.stagedMoveIds.includes(move.operationId)) {
        journal.stagedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "staging", index + 1, plan.moves.length, move.sourcePath);
    }
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
      if (!journal.preparedEmbeddedRelocationIds.includes(relocation.operationId)) {
        journal.preparedEmbeddedRelocationIds.push(relocation.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!lstatOrNull(stage) && !journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) {
        mkdirSync(dirname2(stage), { recursive: true });
        durableRename(absolutePath(repoRoot, relocation.sourcePath), stage);
      }
      if (!journal.stagedEmbeddedRelocationIds.includes(relocation.operationId)) {
        journal.stagedEmbeddedRelocationIds.push(relocation.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "staging-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.sourcePath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    for (const [index, removal] of plan.evidenceRemovals.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
      if (!journal.preparedEvidenceRemovalIds.includes(removal.operationId)) {
        journal.preparedEvidenceRemovalIds.push(removal.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!lstatOrNull(stage) && !journal.stagedEvidenceRemovalIds.includes(removal.operationId)) {
        mkdirSync(dirname2(stage), { recursive: true });
        durableRename(absolutePath(repoRoot, removal.sourcePath), stage);
      }
      if (!journal.stagedEvidenceRemovalIds.includes(removal.operationId)) {
        journal.stagedEvidenceRemovalIds.push(removal.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "staging-evidence-removals", index + 1, plan.evidenceRemovals.length, removal.sourcePath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-staging");
    journal.state = "disposing";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, root] of plan.embeddedTicketRoots.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
      if (!journal.preparedEmbeddedRootIds.includes(root.operationId)) {
        journal.preparedEmbeddedRootIds.push(root.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!lstatOrNull(stage) && !journal.stagedEmbeddedRootIds.includes(root.operationId)) {
        assertDirectoryOnlyTree(absolutePath(repoRoot, root.sourceMetadataRoot));
        if (canonicalJson(noFollowTreeDigest(repoRoot, root.sourceMetadataRoot)) !== canonicalJson(root.residualTreeDigest))
          throw new Error(`Embedded root residual tree differs from frozen structure: ${root.sourceMetadataRoot}`);
        mkdirSync(dirname2(stage), { recursive: true });
        durableRename(absolutePath(repoRoot, root.sourceMetadataRoot), stage);
      }
      if (!journal.stagedEmbeddedRootIds.includes(root.operationId)) {
        journal.stagedEmbeddedRootIds.push(root.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "disposing-embedded-roots", index + 1, plan.embeddedTicketRoots.length, root.sourceMetadataRoot);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-embedded-root-staging");
    journal.state = "installing";
    persistJournal(repoRoot, journalPath, journal);
    for (let index = 0;index < plan.moves.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const move = plan.moves[index];
      const destination = absolutePath(repoRoot, move.destinationPath);
      const installed = journal.installedMoveIds.includes(move.operationId);
      if (!lstatOrNull(destination)) {
        mkdirSync(dirname2(destination), { recursive: true });
        durableRename(join2(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination);
      }
      if (!installed) {
        journal.installedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "moves", index + 1, plan.moves.length, move.destinationPath);
    }
    injectFailure(options, "after-moves");
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const destination = absolutePath(repoRoot, relocation.destinationPath);
      if (!journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) {
        if (!lstatOrNull(destination)) {
          mkdirSync(dirname2(destination), { recursive: true });
          durableRename(join2(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`), destination);
        }
        journal.installedEmbeddedRelocationIds.push(relocation.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "installing-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.destinationPath);
    }
    injectFailure(options, "after-relocations");
    journal.state = "retargeting";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, edit] of plan.symlinkTargetEdits.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const link = absolutePath(repoRoot, edit.finalPath);
      const stage = join2(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
      if (!journal.preparedSymlinkTargetEditIds.includes(edit.operationId)) {
        journal.preparedSymlinkTargetEditIds.push(edit.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId)) {
        if (!lstatOrNull(stage)) {
          mkdirSync(dirname2(stage), { recursive: true });
          durableRename(link, stage);
        }
        journal.stagedSymlinkTargetEditIds.push(edit.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!journal.installedSymlinkTargetEditIds.includes(edit.operationId)) {
        if (!lstatOrNull(link))
          durableSymlink(edit.newTarget, link, process.platform === "win32" ? edit.windowsLinkType : undefined);
        if (readlinkSync(link) !== edit.newTarget)
          throw new Error(`Symlink retarget verification failed: ${edit.finalPath}`);
        journal.installedSymlinkTargetEditIds.push(edit.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "retargeting-symlinks", index + 1, plan.symlinkTargetEdits.length, edit.finalPath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-symlink-retargeting");
    journal.state = "editing";
    persistJournal(repoRoot, journalPath, journal);
    const editGroups = new Map;
    for (const edit of plan.edits)
      editGroups.set(edit.path, [...editGroups.get(edit.path) ?? [], edit]);
    const sortedEditGroups = [...editGroups.entries()].sort(([a], [b]) => a.localeCompare(b));
    for (let index = 0;index < sortedEditGroups.length; index++) {
      checkCancellation(repoRoot, options.cancelFile);
      const [path, edits] = sortedEditGroups[index];
      if (!journal.appliedEditPaths.includes(path)) {
        const preimages = new Map(edits.map((edit) => [canonicalJson(edit.preimage), edit.preimage]));
        const preimage = [...preimages.values()][0];
        if (preimages.size !== 1 || !preimage || canonicalJson(leafPreimage(absolutePath(repoRoot, path))) !== canonicalJson(preimage))
          throw new Error(`Reference edit preimage changed: ${path}`);
        backupPath(repoRoot, path, absolutePath(repoRoot, journal.backupRoot), journal, preimage, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
        persistJournal(repoRoot, journalPath, journal);
        applyReferenceEditAtomically(repoRoot, plan, journal, path, editPreparationName, editWritePreparationName, editWriteCandidateName);
        journal.appliedEditPaths.push(path);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "edits", index + 1, sortedEditGroups.length, path);
    }
    checkCancellation(repoRoot, options.cancelFile);
    injectFailure(options, "after-edits");
    journal.state = "regenerating";
    persistJournal(repoRoot, journalPath, journal);
    for (let index = 0;index < plan.regenerations.length; index++) {
      const regeneration = plan.regenerations[index];
      checkCancellation(repoRoot, options.cancelFile);
      if (journal.completedRegenerationIds.includes(regeneration.id)) {
        if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.outputs))
          throw new Error(`Completed regeneration output changed: ${regeneration.id}`);
        if (regeneration.verifyCommand)
          execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), stdio: "inherit" });
        report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
        continue;
      }
      if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs))
        throw new Error(`Regeneration output preimage changed before execution: ${regeneration.id}`);
      if (!journal.startedRegenerationIds.includes(regeneration.id)) {
        for (const output of regeneration.preOutputs)
          if (output.nodeKind !== "directory")
            backupPath(repoRoot, output.path, absolutePath(repoRoot, journal.backupRoot), journal, output, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
        journal.startedRegenerationIds.push(regeneration.id);
        persistJournal(repoRoot, journalPath, journal);
      }
      execFileSync(regeneration.command[0], [...regeneration.command.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.outputs))
        throw new Error(`Regeneration ${regeneration.id} produced missing, stale, unexpected, byte-different, or mode-different output`);
      durablySyncGeneratorRecords(repoRoot, actualOutputs);
      if (regeneration.verifyCommand)
        execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      journal.completedRegenerationIds.push(regeneration.id);
      persistJournal(repoRoot, journalPath, journal);
      report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
    }
    injectFailure(options, "after-regenerations");
    checkCancellation(repoRoot, options.cancelFile);
    journal.state = "verifying";
    persistJournal(repoRoot, journalPath, journal);
    injectFailure(options, "before-verify");
    const projectionState = [...projectionPostApplyViolations(repoRoot, plan, taxonomy), ...artifactProjectionPostApplyViolations(repoRoot, plan, taxonomy)];
    if (projectionState.length > 0)
      throw new Error(`Projection verification failed: ${projectionState[0].code} at ${projectionState[0].path}`);
    const staleProjectionTokens = projectionStaleViolations(repoRoot, plan, taxonomy);
    if (staleProjectionTokens.length > 0)
      throw new Error(`Projection verification found ${staleProjectionTokens.length} stale old-hierarchy token(s): ${staleProjectionTokens[0].path}`);
    if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest)
      throw new Error("Post-state digest does not match plan expectation");
    const oldTargets = new Set([...plan.moves.map((entry) => entry.sourcePath), ...plan.evidenceRemovals.map((entry) => entry.sourcePath)]);
    for (const root of plan.embeddedTicketRoots)
      for (const path of embeddedTargetPaths(plan, root))
        oldTargets.add(path);
    const staleTransactionReferences = lexicalTargetIncomingReferences(repoRoot, oldTargets, [], taxonomy, options.ticketDir, planAuthority, transactionRootRelative);
    if (staleTransactionReferences.length > 0)
      throw new Error(`Post-state contains ${staleTransactionReferences.length} structured reference(s) to disposed source paths`);
    const postInventoryRaw = inventoryTaxonomy({ repoRoot, scope: plan.scope, ticketDir: options.ticketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile });
    const exactPlanArtifact = (() => {
      if (!planAuthority)
        return false;
      const stat = lstatOrNull(absolutePath(repoRoot, planAuthority.path));
      return Boolean(stat?.isFile() && !stat.isSymbolicLink() && readFileSync2(absolutePath(repoRoot, planAuthority.path)).equals(planAuthority.bytes));
    })();
    const postInventory = inventoryWithoutTransactionEvidence(postInventoryRaw, transactionRootRelative, exactPlanArtifact ? planAuthority.path : undefined);
    const postPlan = planTaxonomy(postInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile });
    const pendingPostOperations = postPlan.moves.length + postPlan.embeddedTicketRoots.length + postPlan.embeddedTicketRootRelocations.length + postPlan.symlinkTargetEdits.length + postPlan.evidenceRemovals.length + postPlan.edits.length + postPlan.regenerations.length;
    if (pendingPostOperations > 0 || postPlan.unresolved.some((entry) => entry.severity === "error"))
      throw new Error(`Post-state does not converge to an empty plan: ${pendingPostOperations} operation(s), ${postPlan.unresolved.length} finding(s)`);
    checkCancellation(repoRoot, options.cancelFile);
    journal.state = "committed";
    persistJournal(repoRoot, journalPath, journal);
    pruneEmptySourceParents(repoRoot, plan, ticketRoot);
    cleanupCommittedTransaction(repoRoot, journal, plan);
    const appliedOperations = plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length;
    report(options.progress, "apply", "complete", appliedOperations, appliedOperations);
    releaseLease();
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
  } catch (error) {
    journal.error = error instanceof Error ? error.message : String(error);
    if (journal.state === "committed") {
      releaseLease();
      throw error;
    }
    try {
      rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options);
    } catch (rollbackError) {
      journal.error = `${journal.error}; rollback failed: ${rollbackError instanceof Error ? rollbackError.message : String(rollbackError)}`;
      persistJournal(repoRoot, journalPath, journal);
      releaseLease();
      throw new Error(journal.error);
    }
    report(options.progress, "apply", "rolled-back", 0, plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length);
    releaseLease();
    return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
  }
}
export {
  verifyTaxonomy,
  taxonomyPlatformPathViolationCodes,
  taxonomyPlanDigest,
  repositoryLocalSymlinkTargetPath,
  planTaxonomy,
  parseTaxonomyPlan,
  parseGeneratorPreviewManifest,
  opaqueTreeDigest,
  noFollowTreeDigest,
  inventoryTaxonomy,
  canonicalJson,
  artifactProjectionTail,
  applyTaxonomyPlan
};
