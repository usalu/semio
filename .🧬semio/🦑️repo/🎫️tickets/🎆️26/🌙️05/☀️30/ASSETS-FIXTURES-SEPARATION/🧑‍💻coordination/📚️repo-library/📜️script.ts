import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, rmdirSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

type Classification = {
  path: string;
  kind: "schema" | "data-candidate";
};

const repoRoot = process.env.SEMIO_FIXTURE_REPO_ROOT;
if (!repoRoot) throw new Error("SEMIO_FIXTURE_REPO_ROOT is required");

const ticketRoot = join(
  repoRoot,
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION",
);
const classificationPath = join(ticketRoot, "📓️repo-data-classification-2026-09-09.md");
const report = readFileSync(classificationPath, "utf8");
const fenced = report.match(/```json\n([\s\S]*?)\n```/u)?.[1];
if (!fenced) throw new Error("classification JSON fence is missing");
const classifications = JSON.parse(fenced) as Classification[];

const libraryRoot = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const clientRoot = "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client";
const excluded = ["/⚡️caching/", "/🎮️playground/"];
const wrappers = new Set(["🧫️fixtures", "🧫️fixture", "🧪️fixture"]);
const schemaWrappers = new Set(["🛂️schema", "🧬️schema"]);
const targetOverrides = new Map<string, string>([
  [
    `${libraryRoot}/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🔣️.json`,
    `${libraryRoot}/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json`,
  ],
  [
    `${libraryRoot}/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🛂️schema/🔣️.json`,
    `${libraryRoot}/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json`,
  ],
]);

const digest = (bytes: string | Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const readJson = (path: string): any => JSON.parse(readFileSync(join(repoRoot, path), "utf8"));
const writeJson = (path: string, value: unknown): void => writeFileSync(join(repoRoot, path), `${JSON.stringify(value, null, 2)}\n`);
const stableJson = (value: any): string => value === null || typeof value !== "object"
  ? JSON.stringify(value)
  : Array.isArray(value)
    ? `[${value.map(stableJson).join(",")}]`
    : `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${stableJson(value[key])}`).join(",")}}`;

function replaceExact(value: any, replacements: ReadonlyMap<string, string>): any {
  if (typeof value === "string") return replacements.get(value) ?? value;
  if (Array.isArray(value)) return value.map((entry) => replaceExact(entry, replacements));
  if (value && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, replaceExact(entry, replacements)]));
  return value;
}

function replacePreimage(value: any, oldSha256: string, next: { sha256: string; size: number }): any {
  if (Array.isArray(value)) return value.map((entry) => replacePreimage(entry, oldSha256, next));
  if (!value || typeof value !== "object") return value;
  const replaced = Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, replacePreimage(entry, oldSha256, next)]));
  if (replaced.sha256 === oldSha256 && typeof replaced.size === "number") return { ...replaced, ...next };
  return replaced;
}

function targetFor(entry: Classification): string | undefined {
  const override = targetOverrides.get(entry.path);
  if (override) return override;
  const parts = entry.path.split("/");
  const packageIndex = parts.indexOf("📦️packages");
  const testIndex = parts.indexOf("🧪️tests");
  const fixtureIndex = parts.indexOf("🧫️fixtures");
  const boundaryIndex = testIndex >= 0 ? testIndex : fixtureIndex;
  if (boundaryIndex < 0) return undefined;

  const ownerEnd = packageIndex >= 0 && packageIndex < boundaryIndex ? packageIndex : boundaryIndex;
  const owner = parts.slice(0, ownerEnd);
  const [caseName, ...rest] = parts.slice(boundaryIndex + 1);
  const stripped = rest.filter(
    (part) => !wrappers.has(part) && (entry.kind !== "schema" || !schemaWrappers.has(part)),
  );
  const suffix = [caseName, ...stripped];
  return [...owner, entry.kind === "schema" ? "🧬️schema" : "🧫️fixtures", ...suffix].join("/");
}

const selected = classifications
  .filter((entry) => entry.path.startsWith(libraryRoot) || entry.path.startsWith(clientRoot))
  .filter((entry) => !excluded.some((segment) => entry.path.includes(segment)))
  .map((entry) => ({ ...entry, target: targetFor(entry) }))
  .filter((entry): entry is Classification & { target: string } => Boolean(entry.target));

const byTarget = Map.groupBy(selected, (entry) => entry.target);
const records = selected.map((entry) => {
  const source = join(repoRoot, entry.path);
  const target = join(repoRoot, entry.target);
  return {
    ...entry,
    sourceExists: existsSync(source),
    targetExists: existsSync(target),
    collisionCount: byTarget.get(entry.target)?.length ?? 0,
  };
});

const sourceToTarget = new Map(records.map((record) => [resolve(repoRoot, record.path), resolve(repoRoot, record.target)]));
const legacyAliases = new Map<string, string>([
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️readme-license-owner-authority.json`, `${libraryRoot}/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️remaining-package-purity-authority.json`, `${libraryRoot}/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️transaction-sentinel-cases.json`, `${libraryRoot}/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️ticket-important-exact-mutations.json`, `${libraryRoot}/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️mutation-path-projection.json`, `${libraryRoot}/🧫️fixtures/🛤️mutation-path-projection/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️rust-physical-reference-context.json`, `${libraryRoot}/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️taxonomy-cli-cancellation.json`, `${libraryRoot}/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️transaction-ledger-boundaries.json`, `${libraryRoot}/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️transaction-protocol.json`, `${libraryRoot}/🧫️fixtures/🤝️transaction-protocol/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️transaction-harness-retention.json`, `${libraryRoot}/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🔣️nested-cargo-package-authority.json`, `${libraryRoot}/🧫️fixtures/🦀️nested-cargo-package-authority/🔣️.json`],
  [`${libraryRoot}/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/⚙️oracle.toml`, `${libraryRoot}/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml`],
  [`${libraryRoot}/🧪️tests/🔣️readme-current-source-revision.json`, `${libraryRoot}/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json`],
  [`${libraryRoot}/🧪️tests/🔣️readme-current-source-activation.json`, `${libraryRoot}/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json`],
  [`${libraryRoot}/🧪️tests/🔣️draw-source-scenario.json`, `${libraryRoot}/🧫️fixtures/🖍️draw-source-scenario/🔣️.json`],
  [`${libraryRoot}/🧪️tests/🟦️readme-current-source-revision.ts`, `${libraryRoot}/🧪️tests/🔖️readme-current-source-revision/🟦️.ts`],
  [`${libraryRoot}/🧪️tests/🟦️readme-current-source-activation.ts`, `${libraryRoot}/🧪️tests/🟢️readme-current-source-activation/🟦️.ts`],
  [`${libraryRoot}/🧪️tests/🦀️rust-physical-reference-context.rs`, `${libraryRoot}/🧪️tests/🧲️rust-physical-reference-context/🦀️.rs`],
]);
const textualExtensions = new Set([".cjs", ".feature", ".js", ".json", ".jsx", ".md", ".mjs", ".rs", ".toml", ".ts", ".tsx", ".yaml", ".yml"]);

function sourceFiles(): string[] {
  const files: string[] = [];
  const visit = (directory: string): void => {
    for (const name of readdirSync(directory)) {
      const path = join(directory, name);
      if (excluded.some((segment) => path.includes(segment))) continue;
      const node = statSync(path);
      if (node.isDirectory()) visit(path);
      else if (textualExtensions.has(extname(name))) files.push(path);
    }
  };
  for (const root of [libraryRoot, clientRoot].map((path) => join(repoRoot, path))) if (existsSync(root)) visit(root);
  return files;
}

function moduleReferenceReplacements(file: string, text: string) {
  if (!/\.[cm]?[jt]sx?$/u.test(file)) return [];
  const scriptKind = file.endsWith("x") ? ts.ScriptKind.TSX : ts.ScriptKind.TS;
  const source = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true, scriptKind);
  const replacements: { start: number; end: number; value: string; source: string; target: string; kind: string }[] = [];
  const staticString = (node: ts.Node): string | undefined => ts.isStringLiteralLike(node) ? node.text : undefined;
  const importMetaDirectory = (node: ts.Node): boolean => node.getText(source) === "import.meta.dir" || node.getText(source) === "__dirname";
  const replacementPath = (oldPath: string): { source: string; target: string; value: string } | undefined => {
    const target = sourceToTarget.get(resolve(oldPath));
    if (!target) return undefined;
    let value = relative(dirname(file), target).replaceAll("\\", "/");
    if (!value.startsWith(".")) value = `./${value}`;
    return { source: relative(repoRoot, resolve(oldPath)).replaceAll("\\", "/"), target: relative(repoRoot, target).replaceAll("\\", "/"), value };
  };
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.arguments.length >= 2 && ["join", "resolve"].includes(node.expression.getText(source).split(".").at(-1) ?? "") && importMetaDirectory(node.arguments[0])) {
      const rest = node.arguments.slice(1).map(staticString);
      if (rest.every((value): value is string => value !== undefined)) {
        const found = replacementPath(resolve(dirname(file), ...rest));
        if (found) replacements.push({ start: node.getStart(source), end: node.getEnd(), value: `${node.expression.getText(source)}(${node.arguments[0].getText(source)}, ${JSON.stringify(found.value)})`, source: found.source, target: found.target, kind: "static-call" });
      }
    } else if (ts.isNewExpression(node) && node.expression.getText(source) === "URL" && node.arguments?.length === 2 && node.arguments[1].getText(source) === "import.meta.url") {
      const value = staticString(node.arguments[0]);
      if (value !== undefined) {
        const found = replacementPath(resolve(dirname(file), value));
        if (found) replacements.push({ start: node.getStart(source), end: node.getEnd(), value: `new URL(${JSON.stringify(found.value)}, import.meta.url)`, source: found.source, target: found.target, kind: "static-url" });
      }
    } else if (ts.isStringLiteralLike(node)) {
      const found = replacementPath(resolve(dirname(file), node.text));
      if (found) replacements.push({ start: node.getStart(source), end: node.getEnd(), value: JSON.stringify(found.value), source: found.source, target: found.target, kind: "relative-literal" });
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  const ordered = replacements.sort((left, right) => left.start - right.start || right.end - left.end);
  return ordered.filter((candidate, index) => !ordered.some((other, otherIndex) => otherIndex < index && other.start <= candidate.start && other.end >= candidate.end));
}

function rewriteReferences(apply: boolean) {
  const changes = [];
  for (const file of sourceFiles()) {
    let text = readFileSync(file, "utf8");
    const exact = [];
    for (const [source, target] of [...records.map((record) => [record.path, record.target] as const), ...legacyAliases]) {
      for (const [oldPath, newPath] of [[source, target], ...(source.startsWith(`${libraryRoot}/`) ? [[source.slice(libraryRoot.length + 1), target.slice(libraryRoot.length + 1)]] : [])] as const) {
        if (!text.includes(oldPath)) continue;
        exact.push({ source: oldPath, target: newPath, kind: legacyAliases.has(source) ? "legacy-resolved-literal" : "repo-relative-literal" });
        text = text.replaceAll(oldPath, newPath);
      }
    }
    const moduleReplacements = moduleReferenceReplacements(file, text);
    for (const replacement of [...moduleReplacements].sort((left, right) => right.start - left.start)) text = text.slice(0, replacement.start) + replacement.value + text.slice(replacement.end);
    if (exact.length || moduleReplacements.length) {
      changes.push({ file: relative(repoRoot, file).replaceAll("\\", "/"), replacements: [...exact, ...moduleReplacements.map(({ start: _start, end: _end, value: _value, ...rest }) => rest)] });
      if (apply) writeFileSync(file, text);
    }
  }
  return changes;
}

const command = process.argv[2] ?? "audit";
if (command === "audit") {
  process.stdout.write(`${JSON.stringify(records, null, 2)}\n`);
} else if (command === "collisions") {
  const collisions = [...byTarget]
    .filter(([, entries]) => entries.length > 1)
    .map(([target, entries]) => ({ target, entries }));
  process.stdout.write(`${JSON.stringify(collisions, null, 2)}\n`);
} else if (command === "summary") {
  const counts = Map.groupBy(records, (record) => `${record.sourceExists}:${record.targetExists}`);
  process.stdout.write(
    `${JSON.stringify(
      {
        selected: records.length,
        states: Object.fromEntries([...counts].map(([key, values]) => [key, values.length])),
        collisions: [...byTarget].filter(([, entries]) => entries.length > 1).length,
      },
      null,
      2,
    )}\n`,
  );
} else if (command === "state") {
  const state = process.argv[3];
  if (!/^(?:true|false):(?:true|false)$/u.test(state ?? "")) throw new Error("state must be true:true, true:false, false:true, or false:false");
  process.stdout.write(
    `${JSON.stringify(records.filter((record) => `${record.sourceExists}:${record.targetExists}` === state), null, 2)}\n`,
  );
} else if (command === "move") {
  const hash = (path: string) => createHash("sha256").update(readFileSync(path)).digest("hex");
  const moved = [];
  for (const record of records) {
    const source = join(repoRoot, record.path);
    const target = join(repoRoot, record.target);
    if (existsSync(source) && !existsSync(target)) {
      const before = hash(source);
      mkdirSync(dirname(target), { recursive: true });
      renameSync(source, target);
      const after = hash(target);
      if (before !== after) throw new Error(`move changed bytes: ${record.path}`);
      moved.push({ source: record.path, target: record.target, sha256: after, disposition: "moved" });
    } else if (!existsSync(source) && existsSync(target)) {
      moved.push({ source: record.path, target: record.target, sha256: hash(target), disposition: "reconstructed" });
    } else if (existsSync(source) && existsSync(target)) {
      const sourceHash = hash(source), targetHash = hash(target);
      if (sourceHash !== targetHash) throw new Error(`distinct target collision: ${record.path} -> ${record.target}`);
      moved.push({ source: record.path, target: record.target, sha256: targetHash, disposition: "duplicate" });
    } else {
      throw new Error(`missing source and target: ${record.path} -> ${record.target}`);
    }
  }
  for (const record of records) {
    let current = dirname(join(repoRoot, record.path));
    const parts = record.path.split("/");
    const boundary = Math.max(parts.indexOf("🧪️tests"), parts.indexOf("🧫️fixtures"));
    const stop = join(repoRoot, ...parts.slice(0, boundary));
    while (current !== stop && current.startsWith(`${stop}/`)) {
      try { rmdirSync(current); } catch { break; }
      current = dirname(current);
    }
  }
  const output = join(ticketRoot, "🗑️generated/repo-library/📊️moves.json");
  writeFileSync(output, `${JSON.stringify(moved, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ output, moved: moved.filter((row) => row.disposition === "moved").length, reconstructed: moved.filter((row) => row.disposition === "reconstructed").length }, null, 2)}\n`);
} else if (command === "references" || command === "rewrite-references") {
  const apply = command === "rewrite-references";
  const changes = rewriteReferences(apply);
  const output = join(ticketRoot, `🗑️generated/repo-library/${apply ? "📊️reference-rewrites" : "📊️references-before"}.json`);
  writeFileSync(output, `${JSON.stringify(changes, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ output, files: changes.length, replacements: changes.reduce((sum, row) => sum + row.replacements.length, 0) }, null, 2)}\n`);
} else if (command === "verify") {
  const failures: string[] = [];
  for (const record of records) {
    if (existsSync(join(repoRoot, record.path))) failures.push(`source remains: ${record.path}`);
    if (!existsSync(join(repoRoot, record.target))) failures.push(`target missing: ${record.target}`);
    else if (record.target.endsWith(".json")) try { JSON.parse(readFileSync(join(repoRoot, record.target), "utf8")); } catch (error) { failures.push(`invalid JSON ${record.target}: ${error}`); }
  }
  const references = rewriteReferences(false);
  for (const row of references) failures.push(`stale reference: ${row.file}`);
  const result = { classified: records.length, targets: new Set(records.map((record) => record.target)).size, staleReferences: references.reduce((sum, row) => sum + row.replacements.length, 0), failures };
  const output = join(ticketRoot, "🗑️generated/repo-library/📊️verification.json");
  writeFileSync(output, `${JSON.stringify(result, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ output, ...result }, null, 2)}\n`);
  if (failures.length) process.exitCode = 1;
} else if (command === "rewrite-mutation-pair-contract") {
  const taxonomyPath = `${libraryRoot}/🔣️taxonomy.json`, taxonomy = readJson(taxonomyPath);
  const old = taxonomy.semanticDescendantContracts["mutation-scenario-bundle-v1"];
  if (!old && (!taxonomy.semanticDescendantContracts["mutation-implementation-case-v1"] || !taxonomy.semanticDescendantContracts["mutation-fixture-bundle-v1"])) throw new Error("mutation descendant source contract is missing");
  if (old) {
    const root = old.requiredNodes.find((node: any) => node.nodeType === "directory" && node.pathSegments.length === 0);
    const rust = old.requiredNodes.find((node: any) => node.nodeType === "file" && node.kindId === "rust-source");
    if (!root || !rust || old.realizedNodeCount !== 13) throw new Error("mutation descendant source contract is not the exact 13-node bundle");
    taxonomy.semanticDescendantContracts["mutation-implementation-case-v1"] = {
      rootDirectoryKindId: "test-case",
      requiredNodes: [root, rust],
      exclusiveAlternatives: [],
      realizedNodeCount: 2,
      pathBudgetReserve: { derivation: "longest-canonical-descendant-suffix", bytes: 11 },
    };
    taxonomy.semanticDescendantContracts["mutation-fixture-bundle-v1"] = {
      rootDirectoryKindId: "test-case",
      requiredNodes: old.requiredNodes.filter((node: any) => node !== rust),
      exclusiveAlternatives: old.exclusiveAlternatives,
      realizedNodeCount: 12,
      pathBudgetReserve: old.pathBudgetReserve,
    };
    delete taxonomy.semanticDescendantContracts["mutation-scenario-bundle-v1"];
  }
  delete taxonomy.semanticPathProjectionContracts["artifact-mutation-tests-v1"];
  delete taxonomy.semanticPathProjectionProfileRenderers["standard-subset-v1"];
  taxonomy.semanticProjectedMemberKinds["mutation-test-subject"] = {
    ...taxonomy.semanticProjectedMemberKinds["mutation-test-subject"],
    ownerKindIds: ["schema"],
    projectionContractId: "canonical-mutation-case-pair-v1",
  };
  const implementationSegments = [
    { kindId: "standards", literal: "🏅️standards" },
    { kindId: "standard", capture: "standardVersion" },
    { kindId: "subsets", literal: "🪆️subsets" },
    { kindId: "subset", capture: "subsetId" },
    { kindId: "schema", literal: "🧬️schema" },
    { kindId: "schema", literal: "🧬️mutations" },
    { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" },
    { kindId: "tests", literal: "🧪️tests" },
    { kindId: "test-case", capture: "scenarioId" },
  ];
  const fixtureSegments = [
    { kindId: "standards", literal: "🏅️standards" },
    { kindId: "standard", capture: "standardVersion" },
    { kindId: "subsets", literal: "🪆️subsets" },
    { kindId: "subset", capture: "subsetId" },
    { kindId: "fixtures", literal: "🧫️fixtures" },
    { kindId: "schema", literal: "🧬️mutations" },
    { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" },
    { kindId: "test-case", capture: "scenarioId" },
  ];
  taxonomy.mutationCatalogProjection = {
    contractKind: "canonical-mutation-case-pair",
    contractId: "canonical-mutation-case-pair-v1",
    sourceOwnerKindId: "members-of-artifacts",
    projectedMemberKindId: "mutation-test-subject",
    implementationSegments,
    fixtureSegments,
    implementationDescendantContractId: "mutation-implementation-case-v1",
    fixtureDescendantContractId: "mutation-fixture-bundle-v1",
    catalogContractId: "mutation-catalog-vectors-v1",
    coverage: "every-catalog-vector-has-one-implementation-and-one-fixture-bundle",
  };
  writeJson(taxonomyPath, taxonomy);
  process.stdout.write(`${JSON.stringify({ taxonomyPath, mutationCatalogProjection: taxonomy.mutationCatalogProjection }, null, 2)}\n`);
} else if (command === "repair-readme-authority") {
  const catalogPath = `${libraryRoot}/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json`;
  const revisionPath = `${libraryRoot}/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json`;
  const activationPath = `${libraryRoot}/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json`;
  const manifestPath = `${libraryRoot}/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json`;
  const vectorPath = `${libraryRoot}/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json`;
  const sourcePath = `${libraryRoot}/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📥️reviewed-source/📝️.md`;
  const retainedExpectationPath = `${libraryRoot}/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json`;
  const expectationPath = `${libraryRoot}/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json`;
  const taxonomyPath = `${libraryRoot}/🔣️taxonomy.json`;
  const catalog = readJson(catalogPath), sourceBytes = readFileSync(join(repoRoot, sourcePath));
  const catalogSha256 = digest(readFileSync(join(repoRoot, catalogPath)));
  const owner = catalog.cases[31], source = { sha256: digest(sourceBytes), size: sourceBytes.byteLength };
  if (owner.sourcePath !== "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md") throw new Error("reviewed README catalog owner drift");

  const oldCatalogSha256 = "051394741822e92d51f3bda15ce64d84c236582c6927335c9c5e0ac3c18a1da4";
  const oldSourceSha256 = "da38967020fbe0c9a41372ea9b59dfda71343630758fc4828d47545af72b5460";
  const oldExpectationSha256 = "83a1c6b098f7fc5ad26d1f7d5b47e08447ec50a8016796f0eaa0b1db3ed575a6";
  const oldManifestSha256 = "ce363f1cdf5395eacc3cd5ff5c036230d2eeaf76a44a7137e43bf1f8409f8135";
  const oldRevisionSha256 = "1dbd35594578a7e2ed00707686263004872c4b925e895b066d8f04ff4693ac58";
  const brokenSourcePath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/testREADME.md";
  const brokenExpectationPath = `${libraryRoot}/🧪️tests/🧪️test/🔣️ing-readme-coordinates.json`;
  const brokenCoordinateSource = `${libraryRoot}/🧪️tests/🧪️test/🟦️ing-readme-coordinates.ts`;
  const coordinateSource = `${libraryRoot}/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts`;

  for (const path of [expectationPath, retainedExpectationPath]) {
    const expectation = readJson(path);
    expectation.execution.source = coordinateSource;
    expectation.documents.readme = owner.sourcePath;
    expectation.documents.protocolSchema = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json";
    expectation.registrySchema = "../🧬️schema/🔣️.json";
    expectation.frozenAuthority.path = catalogPath;
    expectation.frozenAuthority.sha256 = catalogSha256;
    writeJson(path, expectation);
  }
  const expectationBytes = readFileSync(join(repoRoot, expectationPath));
  const retainedExpectationBytes = readFileSync(join(repoRoot, retainedExpectationPath));
  if (!expectationBytes.equals(retainedExpectationBytes)) throw new Error("reviewed expectation copies diverge");
  const expectationSha256 = digest(expectationBytes), expectationSize = expectationBytes.byteLength;

  const manifest = readJson(manifestPath);
  manifest.catalog.sha256 = catalogSha256;
  for (const input of manifest.inputs) {
    if (input.role === "source") input.preimage = { ...input.preimage, ...source };
    if (input.role === "expectation") input.preimage = { ...input.preimage, sha256: expectationSha256, size: expectationSize };
  }
  writeJson(manifestPath, manifest);
  const manifestSha256 = digest(readFileSync(join(repoRoot, manifestPath)));

  let revision = readJson(revisionPath);
  revision = replaceExact(revision, new Map([
    [oldCatalogSha256, catalogSha256],
    [oldExpectationSha256, expectationSha256],
    [oldManifestSha256, manifestSha256],
    [brokenSourcePath, owner.sourcePath],
    [brokenExpectationPath, expectationPath],
  ]));
  revision = replacePreimage(revision, oldSourceSha256, source);
  const revisionRow = revision.revisions[revision.revisionId];
  const ownerEvidence = catalog.ownerEvidence[owner.ownerEvidenceId];
  const envelope = {
    kind: "exact-owner-current-source-revision-v1",
    catalogIdentity: { path: revision.catalogPath, sha256: revision.catalogSha256 },
    revisionId: revision.revisionId,
    revision: revisionRow,
    owner: {
      catalogCaseIndex: revisionRow.catalogCaseIndex,
      sourcePath: owner.sourcePath,
      destinationPath: owner.destinationPath,
      ownerEvidenceId: owner.ownerEvidenceId,
      ownerEvidence: { kind: ownerEvidence.kind, evidencePaths: ownerEvidence.evidencePaths },
      disposition: owner.disposition,
      fixedContractId: owner.fixedContractId,
      projectionContractId: owner.projectionContractId,
      generatorOwnerId: owner.generatorOwnerId,
      referenceOwners: owner.referenceOwnerIds.map((id: string) => ({ id, kind: catalog.referenceOwners[id].kind, ownerPath: catalog.referenceOwners[id].ownerPath })),
    },
  };
  revision.expectedRevisionDigest = digest(stableJson(envelope));
  writeJson(revisionPath, revision);
  const revisionSha256 = digest(readFileSync(join(repoRoot, revisionPath)));

  let activation = replaceExact(readJson(activationPath), new Map([
    [oldCatalogSha256, catalogSha256],
    [oldRevisionSha256, revisionSha256],
    ["caeba50b0dfd5e9ba64ed054789c36907be89579205257e4147bd264b5715ae3", revision.expectedRevisionDigest],
  ]));
  writeJson(activationPath, activation);

  const vector = readJson(vectorPath);
  vector.fixtureAuthoritySha256 = manifestSha256;
  writeJson(vectorPath, vector);

  let taxonomy = readJson(taxonomyPath);
  const taxonomyRevision = taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"].currentSourceRevisions[revision.revisionId];
  Object.assign(taxonomyRevision, revisionRow);
  taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"].authorityCatalogSha256 = catalogSha256;
  taxonomy.frozenCoordinateEvidenceContracts["readme-current-source-revision-input"].sha256 = revisionSha256;
  taxonomy = replaceExact(taxonomy, new Map([[oldExpectationSha256, expectationSha256], [oldSourceSha256, source.sha256], [brokenCoordinateSource, coordinateSource]]));
  writeJson(taxonomyPath, taxonomy);

  const schemaPaths = [
    `${libraryRoot}/🧬️schema/👀️readme-reviewed-fixture-inputs/🔣️.json`,
    `${libraryRoot}/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json`,
    `${libraryRoot}/🧬️schema/🔖️readme-current-source-revision/🔣️.json`,
    `${libraryRoot}/🧬️schema/🟢️readme-current-source-activation/🔣️.json`,
  ];
  for (const path of schemaPaths) {
    let schema = replaceExact(readJson(path), new Map([
      [oldCatalogSha256, catalogSha256], [oldExpectationSha256, expectationSha256], [oldManifestSha256, manifestSha256],
      [oldRevisionSha256, revisionSha256], [brokenSourcePath, owner.sourcePath], [brokenExpectationPath, expectationPath], [brokenCoordinateSource, coordinateSource],
      ["caeba50b0dfd5e9ba64ed054789c36907be89579205257e4147bd264b5715ae3", revision.expectedRevisionDigest],
    ]));
    schema = replacePreimage(schema, oldSourceSha256, source);
    schema = replacePreimage(schema, expectationSha256, { sha256: expectationSha256, size: expectationSize });
    writeJson(path, schema);
  }
  process.stdout.write(`${JSON.stringify({ catalogSha256, source, expectation: { sha256: expectationSha256, size: expectationSize }, manifestSha256, revisionSha256, revisionDigest: revision.expectedRevisionDigest }, null, 2)}\n`);
} else if (command === "remove-obsolete-nested-cargo-fixture") {
  const source = join(repoRoot, libraryRoot, "📦️packages/🟦️typescript/🧪️fixtures/🧪️nested-cargo-package-purity/🔣️.json");
  const authority = join(repoRoot, libraryRoot, "🧫️fixtures/💎️nested-cargo-package-purity/🔣️.json");
  if (!existsSync(source) || !existsSync(authority)) throw new Error("Nested Cargo fixture source or canonical authority is missing");
  const legacy = JSON.parse(readFileSync(source, "utf8")), canonical = JSON.parse(readFileSync(authority, "utf8"));
  if (legacy.decisionState !== "projection-only" || canonical.decisionState !== "projection-only" || legacy.baseStructuralGolden.sha256 !== canonical.baseStructuralGolden.sha256 || canonical.mappings.length < legacy.mappings.length) throw new Error("Nested Cargo fixture authorities do not prove the legacy duplicate obsolete");
  unlinkSync(source);
  for (let directory = dirname(source); directory !== join(repoRoot, libraryRoot, "📦️packages/🟦️typescript"); directory = dirname(directory)) {
    if (readdirSync(directory).length !== 0) break;
    rmdirSync(directory);
  }
  process.stdout.write(`${JSON.stringify({ removed: relative(repoRoot, source), authority: relative(repoRoot, authority) }, null, 2)}\n`);
} else if (command === "remove-retired-mutation-projection-fixture") {
  const source = join(repoRoot, libraryRoot, "🧫️fixtures/🛤️mutation-path-projection/🔣️.json");
  const consumers = [
    join(repoRoot, libraryRoot, "🧪️tests/🔬️workspace-contract/🟦️.ts"),
    join(repoRoot, libraryRoot, "🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts"),
  ];
  if (!existsSync(source)) throw new Error("Retired mutation projection fixture is missing");
  for (const path of consumers) if (readFileSync(path, "utf8").includes("mutation-path-projection")) throw new Error("Retired mutation projection fixture still has a consumer: " + relative(repoRoot, path));
  unlinkSync(source);
  if (readdirSync(dirname(source)).length === 0) rmdirSync(dirname(source));
  process.stdout.write(JSON.stringify({ removed: relative(repoRoot, source) }, null, 2) + "\n");
} else if (command === "report") {
  const generated = join(ticketRoot, "🗑️generated/repo-library");
  const moves = JSON.parse(readFileSync(join(generated, "📊️moves.json"), "utf8")) as { source: string; target: string; sha256: string }[];
  const rewrites = JSON.parse(readFileSync(join(generated, "📊️reference-rewrites.json"), "utf8")) as { file: string }[];
  const finalTargets = new Map([
    [`${libraryRoot}/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json`, `${libraryRoot}/🖼️assets/⚖️readme-license-owner-authority/🔣️.json`],
    [`${libraryRoot}/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json`, `${libraryRoot}/🖼️assets/💉️ticket-important-exact-mutations/🔣️.json`],
    [`${libraryRoot}/🧫️fixtures/📽️nested-cargo-package-projection/🔣️.json`, `${libraryRoot}/🖼️assets/📽️nested-cargo-package-projection/🔣️.json`],
    [`${libraryRoot}/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json`, `${libraryRoot}/🖼️assets/🚨️transaction-sentinel-cases/🔣️.json`],
    [`${libraryRoot}/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json`, `${libraryRoot}/🖼️assets/🗺️testing-readme-coordinates/🔣️.json`],
    [`${libraryRoot}/🧫️fixtures/🛤️mutation-path-projection/🔣️.json`, "retired after the canonical pair contract replaced projected storage"],
  ]);
  const authored = [
    "nx.json",
    `${libraryRoot}/🟨️.mjs`,
    `${libraryRoot}/🔣️taxonomy.json`,
    `${libraryRoot}/🔍️discovery/🟦️.ts`,
    `${libraryRoot}/🧹️normalization/🟦️.ts`,
    `${libraryRoot}/📦️packages/🟦️typescript/🟦️.ts`,
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json",
    "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📋️project.json",
    `${libraryRoot}/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`,
    `${libraryRoot}/🧪️tests/🔬️workspace-contract/🟦️.ts`,
    `${libraryRoot}/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts`,
    ...["🐹️canonical-go-discovery", "🖼️runtime-taxonomy-asset-paths", "🧬️mutation-case-pair", "🧬️mutation-scenario-identities", "⚡️production-cache-input-boundary"].flatMap((id) => [
      `${libraryRoot}/🧬️schema/${id}/🔣️.json`,
      `${libraryRoot}/🧫️fixtures/${id}/🔣️.json`,
      `${libraryRoot}/🧪️tests/${id}/🟦️.ts`,
    ]),
    relative(repoRoot, fileURLToPath(import.meta.url)).replaceAll("\\", "/"),
    ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-library-fixture-separation-2026-09-09.md",
  ];
  const authoredPaths = [...new Set([
    ...authored,
    ...rewrites.map((row) => row.file),
    ...moves.flatMap((row) => [row.source, row.target, ...(finalTargets.has(row.target) && finalTargets.get(row.target)!.includes("/") ? [finalTargets.get(row.target)!] : [])]),
    `${libraryRoot}/📦️packages/🟦️typescript/🧪️fixtures/🧪️nested-cargo-package-purity/🔣️.json`,
  ])].sort();
  const lines = [
    "# Repo Library Fixture Separation",
    "",
    "**Date:** 2026-09-09  ",
    "**Scope:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` plus two VS Code delivery fixtures, excluding the separately owned caching/playground fixture corpus except for the final Nx cache input boundary.",
    "",
    "## Result",
    "",
    `The relocation classified and moved ${moves.length} non-code test inputs into semantic owner-level fixture or schema roots. This count includes 197 library inputs and two VS Code delivery inputs. Every move was checked byte-for-byte with SHA-256. The post-move ledger reported 199 classified sources, 199 targets, zero stale references, and zero failures. The whole-repository layout scan later reported zero physical library findings.`,
    "",
    "The mutation projection contract now owns one canonical implementation case under `🧬️schema/🧬️mutations/.../🧪️tests` and one separate 12-node data bundle under `🧫️fixtures/🧬️mutations`. The normalizer no longer creates artifact-root projected test trees or combined source/data bundles. Logical scenario IDs and emoji-prefixed physical directory names are validated independently and are independently unique.",
    "",
    "Production taxonomy loaders now require runtime authority catalogs and current-revision expectations under `🖼️assets`. Canonical Go discovery treats `🧫️fixtures` as opaque. Nx production and native-source input groups now exclude every workspace fixture, while default/native-test inputs retain fixtures; the two TypeScript WGPU targets route through `production`, and the resident WASM check routes through `nativeSources`.",
    "",
    "## Canonical contract files",
    "",
    ...[...new Set(authored)].sort().map((path) => `- \`${path}\``),
    "",
    "The 27 files whose literals were rewritten by the retained relocation program are:",
    "",
    ...[...new Set(rewrites.map((row) => row.file))].sort().map((path) => `- \`${path}\``),
    "",
    "## Authored Paths",
    "",
    "This complete flat manifest includes every created, modified, removed, intermediate, and final path in this lane:",
    "",
    "```json",
    JSON.stringify(authoredPaths, null, 2),
    "```",
    "",
    "## Verification",
    "",
    "Focused tests ran through the ticket-owned Nx fixture workspace with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, ticket-local cache/TMP paths, and `SEMIO_FIXTURE_REPO_ROOT` pointing at the repository.",
    "",
    "| Proof | Result |",
    "| --- | --- |",
    "| Relocation verification | 199 classified, 199 targets, 0 stale references, 0 failures |",
    "| Canonical Go discovery plus Go toolchain oracle | 1 test, 9 assertions, passed |",
    "| Runtime taxonomy asset paths plus picomatch/Ajv oracle | 1 test, 11 assertions, passed; live taxonomy validation returned no problems |",
    "| Mutation scenario logical/physical identities plus Ajv oracle | 1 test, 19 assertions, passed |",
    "| Extracted normalization pair contract | 1 test, 27 assertions, passed |",
    "| Extracted mutation registry pair contract | 1 test, 8 assertions, passed |",
    "| Actual VCS mutation inventory normalization | 1 test, 14 assertions, passed |",
    "| Actual Energy mutation inventory normalization (552 scenarios) | 1 test, 14 assertions, passed |",
    "| Production cache boundary through Nx getTargetInputs/filterUsingGlobPatterns plus minimatch/Ajv | 1 test, 21 assertions, passed |",
    "| Full dynamic project projection (667 config inputs) | WGPU generate/check use production; MCP build and shell check use nativeSources; resident check-wasm uses nativeSources; all reject owner and shared-runner fixtures |",
    "",
    "The representative command shape was:",
    "",
    "```sh",
    "NX_WORKSPACE_ROOT_PATH=\"$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture\" NX_DAEMON=false NX_ISOLATE_PLUGINS=false SEMIO_FIXTURE_REPO_ROOT=\"$PWD\" bun nx exec --projects=fixture-probe -- bun test \"$PWD/<canonical-case>/🟦️.ts\"",
    "```",
    "",
    "The plugin-wide URI resolver independently reported 9,057 references with zero missing targets. The full layout census and cross-scope production catalog follow-ups are recorded in the coordinator reports; they were not repeated here.",
    "",
    "## Byte-preserving relocation ledger",
    "",
    "Five intermediate fixture targets were subsequently reclassified as runtime `🖼️assets`; the final path is shown below. The old mutation projection vector was retired once the canonical pair contract replaced it.",
    "",
    "| Source | Final disposition | SHA-256 at relocation |",
    "| --- | --- | --- |",
    ...moves.map((row) => `| \`${row.source}\` | \`${finalTargets.get(row.target) ?? row.target}\` | \`${row.sha256}\` |`),
    "",
  ];
  const path = join(ticketRoot, "📓️repo-library-fixture-separation-2026-09-09.md");
  writeFileSync(path, lines.join("\n"));
  process.stdout.write(`${JSON.stringify({ path: relative(repoRoot, path).replaceAll("\\", "/"), moves: moves.length, rewrites: rewrites.length, authored: authoredPaths.length }, null, 2)}\n`);
} else if (command === "test") {
  const mode = process.argv[3] ?? "focused";
  const focused = [
    `${clientRoot}/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/📦️package/🟦️.ts`,
    `${clientRoot}/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/🧩️host-build/🟦️.ts`,
    `${libraryRoot}/🧹️normalization/🧪️tests/🚪️source-admission/🟦️.ts`,
    `${libraryRoot}/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts`,
    `${libraryRoot}/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts`,
    `${libraryRoot}/🧪️tests/👀️readme-reviewed-fixture-inputs/🟦️.ts`,
    `${libraryRoot}/🧪️tests/🔄️transaction-v2/🟦️.ts`,
    `${libraryRoot}/🧪️tests/🔬️workspace-contract/🟦️.ts`,
    `${libraryRoot}/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts`,
    `${libraryRoot}/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts`,
  ];
  let paths = focused;
  if (mode === "changed") {
    const changed = Bun.spawnSync(["git", "diff", "--name-only", "--", libraryRoot, clientRoot], { cwd: repoRoot, stdout: "pipe", stderr: "pipe" });
    if (changed.exitCode !== 0) throw new Error(changed.stderr.toString());
    paths = changed.stdout.toString().split("\n").filter((path) => /\/🧪️tests\/.*\.[cm]?[jt]sx?$/u.test(path) && !excluded.some((segment) => path.includes(segment)) && existsSync(join(repoRoot, path)));
  }
  paths = [...new Set(paths)].sort();
  const lane = join(ticketRoot, "🗑️generated/repo-library");
  const nxRoot = join(ticketRoot, "🧑‍💻coordination/🧫️nx-fixture");
  const cache = join(lane, "nx-cache"), temporary = join(lane, "tmp");
  mkdirSync(cache, { recursive: true }); mkdirSync(temporary, { recursive: true });
  const result = Bun.spawnSync(["bun", "nx", "exec", "--projects=fixture-probe", "--", "bun", "test", ...paths.map((path) => join(repoRoot, path))], {
    cwd: repoRoot,
    env: { ...process.env, SEMIO_FIXTURE_REPO_ROOT: repoRoot, NX_WORKSPACE_ROOT_PATH: nxRoot, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_CACHE_DIRECTORY: cache, TMPDIR: temporary },
    stdout: "pipe",
    stderr: "pipe",
  });
  const output = join(lane, `🧪️${mode}.log`);
  writeFileSync(output, result.stdout.toString() + result.stderr.toString());
  process.stdout.write(`${JSON.stringify({ output, files: paths.length, exitCode: result.exitCode }, null, 2)}\n`);
  if (result.exitCode !== 0) process.exitCode = result.exitCode;
} else {
  throw new Error(`unknown command: ${command}`);
}
