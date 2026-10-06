import { createHash } from "node:crypto";
import {
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  renameSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { dirname, extname, join, relative, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";

type Change = {
  kind: "create" | "modify" | "move";
  oldPath: string;
  newPath: string;
  bytesBefore: number;
  bytesAfter: number;
  sha256Before: string;
  sha256After: string;
  bytePreserved: boolean;
};

const ticket = resolve(import.meta.dir, "..");
const repo = resolve(ticket, "../../../../../../..");
const scope = join(repo, "✏️s");
const output = join(ticket, "🗑️generated", "plugins");
const reportPath = join(output, "📊️moves.json");

const slash = (path: string) => path.split(sep).join("/");
const repoPath = (path: string) => slash(relative(repo, path));
const sha256 = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const textExtensions = new Set([
  ".c",
  ".cc",
  ".cpp",
  ".cs",
  ".css",
  ".ebnf",
  ".feature",
  ".g4",
  ".go",
  ".graphql",
  ".h",
  ".hpp",
  ".html",
  ".js",
  ".json",
  ".jsx",
  ".md",
  ".mjs",
  ".proto",
  ".py",
  ".rs",
  ".semio",
  ".toml",
  ".ts",
  ".tsx",
  ".txt",
  ".yaml",
  ".yml",
]);
const isTextSource = (path: string) => textExtensions.has(extname(path));

const trackedFiles = (): string[] => {
  const result = Bun.spawnSync(["rg", "--files", "-0", scope]);
  if (result.exitCode !== 0) throw new Error(result.stderr.toString());
  return result.stdout
    .toString()
    .split("\0")
    .filter(Boolean)
    .map((path) => resolve(path));
};

const referencingFiles = (pattern: string): string[] => {
  const globs = [...textExtensions].flatMap((extension) => ["--glob", `*${extension}`]);
  const result = Bun.spawnSync(["rg", "-l", "-0", ...globs, pattern, scope]);
  if (result.exitCode !== 0 && result.exitCode !== 1) throw new Error(result.stderr.toString());
  return result.stdout
    .toString()
    .split("\0")
    .filter(Boolean)
    .map((path) => resolve(path));
};

const fixtureLocation = (path: string) => {
  const parts = path.split(sep);
  const testAt = parts.findIndex((part, index) => part === "🧪️tests" && parts[index + 2] === "🧫️fixtures");
  if (testAt < 0) return undefined;
  const owner = parts.slice(0, testAt).join(sep) || sep;
  const testCase = parts[testAt + 1];
  const fixtureRelative = parts.slice(testAt + 3).join(sep);
  return { owner, testCase, fixtureRelative };
};

const writeReport = (changes: Change[]) => {
  mkdirSync(output, { recursive: true });
  const previous = existsSync(reportPath) ? (JSON.parse(readFileSync(reportPath, "utf8")) as Change[]) : [];
  const merged = new Map(previous.map((change) => [`${change.kind}:${change.oldPath}:${change.newPath}`, change]));
  for (const change of changes) merged.set(`${change.kind}:${change.oldPath}:${change.newPath}`, change);
  writeFileSync(reportPath, `${JSON.stringify([...merged.values()].sort((a, b) => a.oldPath.localeCompare(b.oldPath)), null, 2)}\n`);
};

const census = () => {
  const files = trackedFiles();
  const localFixtureFiles = files.filter((path) => fixtureLocation(path));
  const localFixtureDirectories = new Set(
    localFixtureFiles.map((path) => {
      const location = fixtureLocation(path)!;
      return join(location.owner, "🧪️tests", location.testCase, "🧫️fixtures");
    }),
  );
  const localReferenceFiles = files.filter((path) => isTextSource(path)).filter((path) => {
    const bytes = readFileSync(path);
    return !bytes.includes(0) && bytes.toString("utf8").includes("local://");
  });
  const assetReferenceFiles = files.filter((path) => isTextSource(path)).filter((path) => {
    const bytes = readFileSync(path);
    return !bytes.includes(0) && bytes.toString("utf8").includes("asset://");
  });
  mkdirSync(output, { recursive: true });
  writeFileSync(
    join(output, "📊️census.json"),
    `${JSON.stringify(
      {
        localFixtureDirectories: [...localFixtureDirectories].map(repoPath).sort(),
        localFixtureFiles: localFixtureFiles.map(repoPath).sort(),
        localReferenceFiles: localReferenceFiles.map(repoPath).sort(),
        assetReferenceFiles: assetReferenceFiles.map(repoPath).sort(),
      },
      null,
      2,
    )}\n`,
  );
  console.log(
    JSON.stringify({
      localFixtureDirectories: localFixtureDirectories.size,
      localFixtureFiles: localFixtureFiles.length,
      localReferenceFiles: localReferenceFiles.length,
      assetReferenceFiles: assetReferenceFiles.length,
    }),
  );
};

const migrateLocal = () => {
  const files = trackedFiles();
  const fixtureFiles = files.filter((path) => fixtureLocation(path));
  const fixtureRoots = new Map<string, { owner: string; testCase: string }>();
  const changes: Change[] = [];

  for (const oldPath of fixtureFiles) {
    const location = fixtureLocation(oldPath)!;
    const sourceRoot = join(location.owner, "🧪️tests", location.testCase, "🧫️fixtures");
    fixtureRoots.set(sourceRoot, { owner: location.owner, testCase: location.testCase });
    const newPath = join(location.owner, "🧫️fixtures", location.testCase, location.fixtureRelative);
    if (existsSync(newPath)) throw new Error(`Destination already exists: ${repoPath(newPath)}`);
  }

  for (const [sourceRoot, { owner, testCase }] of fixtureRoots) {
    const caseRoot = join(owner, "🧪️tests", testCase);
    for (const path of files.filter(
      (candidate) =>
        isTextSource(candidate) &&
        candidate.startsWith(`${caseRoot}${sep}`) &&
        !candidate.startsWith(`${sourceRoot}${sep}`),
    )) {
      const before = readFileSync(path);
      if (before.includes(0)) continue;
      const text = before.toString("utf8");
      if (!text.includes("local://")) continue;
      const after = Buffer.from(text.replaceAll("local://", `shared://${testCase}/`));
      writeFileSync(path, after);
      changes.push({
        kind: "modify",
        oldPath: repoPath(path),
        newPath: repoPath(path),
        bytesBefore: before.length,
        bytesAfter: after.length,
        sha256Before: sha256(before),
        sha256After: sha256(after),
        bytePreserved: false,
      });
    }
  }

  for (const oldPath of fixtureFiles) {
    const location = fixtureLocation(oldPath)!;
    const newPath = join(location.owner, "🧫️fixtures", location.testCase, location.fixtureRelative);
    const before = readFileSync(oldPath);
    mkdirSync(dirname(newPath), { recursive: true });
    renameSync(oldPath, newPath);
    const after = readFileSync(newPath);
    changes.push({
      kind: "move",
      oldPath: repoPath(oldPath),
      newPath: repoPath(newPath),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: before.equals(after),
    });
  }

  for (const sourceRoot of fixtureRoots.keys()) {
    if (existsSync(sourceRoot) && statSync(sourceRoot).isDirectory()) rmSync(sourceRoot, { recursive: true });
  }
  writeReport(changes.sort((a, b) => a.oldPath.localeCompare(b.oldPath)));
  console.log(
    JSON.stringify({
      movedFiles: changes.filter(({ kind }) => kind === "move").length,
      modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
      bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length,
    }),
  );
};

const cleanLocalMarkers = () => {
  const changes: Change[] = [];
  for (const path of referencingFiles("local://")) {
    const before = readFileSync(path);
    if (before.includes(0)) continue;
    const text = before.toString("utf8");
    if (!text.includes("local://")) continue;
    const afterText = text
      .replaceAll('("asset://", "local://", "shared://")', '("asset://", "shared://")')
      .replaceAll(
        '(token.starts_with("asset://") || token.starts_with("local://") || token.starts_with("shared://"))',
        '(token.starts_with("asset://") || token.starts_with("shared://"))',
      )
      .replaceAll("local://🔣️.json base graph", "shared://🌊️mutate-flow-1/🔣️.json base graph")
      .replaceAll("local://🔣️.json base", "shared://🎞️mutate-presentation-1/🔣️.json base")
      .replaceAll("local://", "shared://");
    const after = Buffer.from(afterText);
    writeFileSync(path, after);
    changes.push({
      kind: "modify",
      oldPath: repoPath(path),
      newPath: repoPath(path),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: false,
    });
  }
  writeReport(changes);
  console.log(JSON.stringify({ modifiedFiles: changes.length }));
};

const rebuildLocalReport = () => {
  const files = trackedFiles();
  const indexedResult = Bun.spawnSync(["git", "ls-files", "-z", scope], { cwd: repo });
  if (indexedResult.exitCode !== 0) throw new Error(indexedResult.stderr.toString());
  const indexed = new Set(indexedResult.stdout.toString().split("\0").filter(Boolean));
  const changes: Change[] = [];
  for (const newPath of files) {
    const parts = newPath.split(sep);
    const fixtureAt = parts.findIndex((part, index) => part === "🧫️fixtures" && index + 2 < parts.length);
    if (fixtureAt < 0) continue;
    const owner = parts.slice(0, fixtureAt).join(sep) || sep;
    const testCase = parts[fixtureAt + 1];
    const caseRoot = join(owner, "🧪️tests", testCase);
    if (!existsSync(caseRoot)) continue;
    const fixtureRelative = parts.slice(fixtureAt + 2).join(sep);
    const oldPath = join(caseRoot, "🧫️fixtures", fixtureRelative);
    if (existsSync(oldPath) || !indexed.has(repoPath(oldPath))) continue;
    const bytes = readFileSync(newPath);
    changes.push({
      kind: "move",
      oldPath: repoPath(oldPath),
      newPath: repoPath(newPath),
      bytesBefore: bytes.length,
      bytesAfter: bytes.length,
      sha256Before: sha256(bytes),
      sha256After: sha256(bytes),
      bytePreserved: true,
    });
  }
  for (const path of files.filter(isTextSource)) {
    const parts = path.split(sep);
    const testAt = parts.findIndex((part, index) => part === "🧪️tests" && index + 1 < parts.length);
    if (testAt < 0) continue;
    const testCase = parts[testAt + 1];
    const after = readFileSync(path);
    if (after.includes(0)) continue;
    const marker = `shared://${testCase}/`;
    const afterText = after.toString("utf8");
    if (!afterText.includes(marker)) continue;
    const before = Buffer.from(afterText.replaceAll(marker, "local://"));
    changes.push({
      kind: "modify",
      oldPath: repoPath(path),
      newPath: repoPath(path),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: false,
    });
  }
  const previous = existsSync(reportPath) ? (JSON.parse(readFileSync(reportPath, "utf8")) as Change[]) : [];
  mkdirSync(output, { recursive: true });
  writeFileSync(reportPath, `${JSON.stringify(previous.filter(({ kind }) => kind !== "move"), null, 2)}\n`);
  writeReport(changes);
  console.log(
    JSON.stringify({
      movedFiles: changes.filter(({ kind }) => kind === "move").length,
      modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
    }),
  );
};

const assetCensus = () => {
  const records = [] as Array<{
    sourcePath: string;
    line: number;
    uri: string;
    ownerPath?: string;
    currentTargetExists?: boolean;
    assetTargetExists?: boolean;
  }>;
  for (const path of referencingFiles("asset://")) {
    const parts = path.split(sep);
    const testAt = parts.findIndex((part) => part === "🧪️tests");
    const owner = testAt < 0 ? undefined : parts.slice(0, testAt).join(sep) || sep;
    const lines = readFileSync(path, "utf8").split("\n");
    for (const [index, line] of lines.entries()) {
      for (const match of line.matchAll(/asset:\/\/[^\s"'`]+/g)) {
        const uri = match[0].replace(/[),.;:]+$/, "");
        const relativeTarget = uri.slice("asset://".length);
        records.push({
          sourcePath: repoPath(path),
          line: index + 1,
          uri,
          ownerPath: owner && repoPath(owner),
          currentTargetExists: owner && !relativeTarget.includes("<") ? existsSync(join(owner, relativeTarget)) : undefined,
          assetTargetExists: owner && !relativeTarget.includes("<") ? existsSync(join(owner, "🖼️assets", relativeTarget)) : undefined,
        });
      }
    }
  }
  mkdirSync(output, { recursive: true });
  writeFileSync(join(output, "📊️asset-census.json"), `${JSON.stringify(records, null, 2)}\n`);
  console.log(
    JSON.stringify({
      records: records.length,
      sourceFiles: new Set(records.map(({ sourcePath }) => sourcePath)).size,
      distinctUris: new Set(records.map(({ uri }) => uri)).size,
      currentTargets: records.filter(({ currentTargetExists }) => currentTargetExists).length,
      assetTargets: records.filter(({ assetTargetExists }) => assetTargetExists).length,
    }),
  );
};

const migrateOwnerFixtureUris = () => {
  const changes: Change[] = [];
  for (const path of referencingFiles("asset://🧫️fixtures/")) {
    const before = readFileSync(path);
    const after = Buffer.from(before.toString("utf8").replaceAll("asset://🧫️fixtures/", "shared://"));
    writeFileSync(path, after);
    changes.push({
      kind: "modify",
      oldPath: repoPath(path),
      newPath: repoPath(path),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: false,
    });
  }
  writeReport(changes);
  console.log(JSON.stringify({ modifiedFiles: changes.length }));
};

const migrateSchemaVectorFamily = (ownerArgument: string | undefined, persist = true): Change[] => {
  if (!ownerArgument) throw new Error("migrate-schema-vector-family requires a repository-relative subset owner");
  const owner = resolve(repo, ownerArgument);
  if (!owner.startsWith(`${scope}${sep}`)) throw new Error(`Owner escapes ✏️s: ${ownerArgument}`);
  const mutationRoot = join(owner, "🧬️schema", "🧬️mutations");
  const implementationExtensions = new Set([".c", ".cc", ".cpp", ".cs", ".feature", ".go", ".js", ".jsx", ".py", ".rs", ".ts", ".tsx"]);
  const fixtureMutationRoot = join(owner, "🧫️fixtures", "🧬️mutations");
  if (existsSync(fixtureMutationRoot)) {
    const fixtureResult = Bun.spawnSync(["rg", "--files", "-0", fixtureMutationRoot]);
    if (fixtureResult.exitCode !== 0) throw new Error(fixtureResult.stderr.toString());
    const restoredFixturePaths: string[] = [];
    for (const fixturePath of fixtureResult.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path))) {
      const parts = relative(fixtureMutationRoot, fixturePath).split(sep);
      if (parts.length !== 3 || !implementationExtensions.has(extname(fixturePath))) continue;
      const [mutation, testCase, implementation] = parts;
      const restoredPath = join(mutationRoot, mutation, "🧪️tests", testCase, implementation);
      if (existsSync(restoredPath)) throw new Error(`Implementation already exists: ${repoPath(restoredPath)}`);
      mkdirSync(dirname(restoredPath), { recursive: true });
      renameSync(fixturePath, restoredPath);
      restoredFixturePaths.push(repoPath(fixturePath));
    }
    if (restoredFixturePaths.length > 0 && existsSync(reportPath)) {
      const previous = JSON.parse(readFileSync(reportPath, "utf8")) as Change[];
      writeFileSync(reportPath, `${JSON.stringify(previous.filter(({ newPath }) => !restoredFixturePaths.includes(newPath)), null, 2)}\n`);
    }
  }
  const result = Bun.spawnSync(["rg", "--files", "-0", mutationRoot]);
  if (result.exitCode !== 0) throw new Error(result.stderr.toString());
  const files = result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path));
  const moves = [] as Array<{ oldPath: string; newPath: string; mutation: string; testCase: string; rest: string }>;
  for (const oldPath of files) {
    const parts = relative(mutationRoot, oldPath).split(sep);
    if (parts[1] !== "🧪️tests" || parts.length < 4 || (parts.length === 4 && implementationExtensions.has(extname(oldPath)))) continue;
    const [mutation, , testCase, ...restParts] = parts;
    const rest = restParts.join(sep);
    const newPath = join(owner, "🧫️fixtures", "🧬️mutations", mutation, testCase, rest);
    if (existsSync(newPath)) throw new Error(`Destination already exists: ${repoPath(newPath)}`);
    moves.push({ oldPath, newPath, mutation, testCase, rest });
  }
  const changes: Change[] = [];
  const sourceDirectories = new Set<string>();
  for (const move of moves) {
    const bytes = readFileSync(move.oldPath);
    mkdirSync(dirname(move.newPath), { recursive: true });
    renameSync(move.oldPath, move.newPath);
    const after = readFileSync(move.newPath);
    sourceDirectories.add(join(mutationRoot, move.mutation, "🧪️tests", move.testCase, move.rest.split(sep)[0]));
    changes.push({
      kind: "move",
      oldPath: repoPath(move.oldPath),
      newPath: repoPath(move.newPath),
      bytesBefore: bytes.length,
      bytesAfter: after.length,
      sha256Before: sha256(bytes),
      sha256After: sha256(after),
      bytePreserved: bytes.equals(after),
    });
  }
  for (const directory of sourceDirectories) {
    if (existsSync(directory)) rmSync(directory, { recursive: true });
  }
  const replacements = new Map<string, Map<string, string>>();
  for (const move of moves) {
    const caseRoot = join(mutationRoot, move.mutation, "🧪️tests", move.testCase);
    for (const implementation of files.filter((path) => dirname(path) === caseRoot && isTextSource(path))) {
      const byFile = replacements.get(implementation) ?? new Map<string, string>();
      byFile.set(slash(move.rest), slash(relative(dirname(implementation), move.newPath)));
      replacements.set(implementation, byFile);
    }
  }
  const ownerResult = Bun.spawnSync(["rg", "--files", "-0", owner]);
  if (ownerResult.exitCode !== 0) throw new Error(ownerResult.stderr.toString());
  const ownerFiles = ownerResult.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path));
  for (const path of ownerFiles.filter(isTextSource)) {
    const before = readFileSync(path);
    if (before.includes(0)) continue;
    let text = before.toString("utf8");
    for (const [oldInclude, newInclude] of replacements.get(path) ?? []) {
      text = text.replaceAll(`include_str!("${oldInclude}")`, `include_str!("${newInclude}")`);
      text = text.replaceAll(`include_bytes!("${oldInclude}")`, `include_bytes!("${newInclude}")`);
    }
    text = text
      .replaceAll("asset://🧬️schema/🧬️mutations/", "shared://🧬️mutations/")
      .replaceAll("asset://🧬️schema/🧬️mutations", "shared://🧬️mutations");
    text = text.replace(/shared:\/\/🧬️mutations\/[^\s"'`]+/g, (uri) => uri.replaceAll("/🧪️tests/", "/"));
    for (const { mutation, testCase } of moves) {
      text = text.replaceAll(`${mutation}/🧪️tests/${testCase}`, `${mutation}/${testCase}`);
    }
    text = text.replaceAll("🧬️mutations/<slug>/🧪️tests/<fixture>/", "🧫️fixtures/🧬️mutations/<slug>/<fixture>/");
    const after = Buffer.from(text);
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({
      kind: "modify",
      oldPath: repoPath(path),
      newPath: repoPath(path),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: false,
    });
  }
  if (persist) {
    writeReport(changes);
    console.log(
      JSON.stringify({
        movedFiles: changes.filter(({ kind }) => kind === "move").length,
        modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
        bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length,
      }),
    );
  }
  return changes;
};

const migrateAllSchemaVectorFamilies = () => {
  const marker = `${sep}🧬️schema${sep}🧬️mutations${sep}`;
  const owners = new Set<string>();
  for (const path of trackedFiles()) {
    const markerAt = path.indexOf(marker);
    if (markerAt < 0) continue;
    const rest = path.slice(markerAt + marker.length).split(sep);
    if (rest[1] !== "🧪️tests" || rest.length < 4) continue;
    owners.add(path.slice(0, markerAt));
  }
  const changes: Change[] = [];
  const ordered = [...owners].sort();
  for (const [index, owner] of ordered.entries()) {
    const familyChanges = migrateSchemaVectorFamily(repoPath(owner), false);
    changes.push(...familyChanges);
    console.log(
      JSON.stringify({
        family: `${index + 1}/${ordered.length}`,
        owner: repoPath(owner),
        movedFiles: familyChanges.filter(({ kind }) => kind === "move").length,
        modifiedFiles: familyChanges.filter(({ kind }) => kind === "modify").length,
      }),
    );
  }
  writeReport(changes);
  console.log(
    JSON.stringify({
      families: ordered.length,
      movedFiles: changes.filter(({ kind }) => kind === "move").length,
      modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
      bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length,
    }),
  );
};

const migrateProductionAssets = () => {
  const cadFixture = join(
    scope,
    "🔌️plugins",
    "📐️cad",
    "🗿️artifacts",
    "📐️cad",
    "🏅️standards",
    "🔖️1",
    "🪆️subsets",
    "✳️any",
    "📚️examples",
    "🧫️fixtures",
    "🌲️concrete-forest-reference",
    "🖼️.png",
  );
  const cadAsset = cadFixture.replace(`${sep}🧫️fixtures${sep}`, `${sep}🖼️assets${sep}`);
  const changes: Change[] = [];
  if (existsSync(cadFixture)) {
    if (existsSync(cadAsset)) throw new Error(`Destination already exists: ${repoPath(cadAsset)}`);
    const before = readFileSync(cadFixture);
    mkdirSync(dirname(cadAsset), { recursive: true });
    renameSync(cadFixture, cadAsset);
    const after = readFileSync(cadAsset);
    changes.push({
      kind: "move",
      oldPath: repoPath(cadFixture),
      newPath: repoPath(cadAsset),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: before.equals(after),
    });
  }
  const replacements = [
    ["/📚️examples/🧫️fixtures", "/📚️examples/🖼️assets"],
    ["/cad-fixture", "/cad-assets"],
    ["♾️infinite/🧫️fixtures", "♾️infinite/🖼️assets"],
    ["/infinite-fixture", "/infinite-assets"],
  ] as const;
  const paths = new Set<string>();
  for (const [before] of replacements) {
    for (const path of referencingFiles(before)) paths.add(path);
  }
  for (const path of paths) {
    const before = readFileSync(path);
    let text = before.toString("utf8");
    for (const [from, to] of replacements) text = text.replaceAll(from, to);
    const after = Buffer.from(text);
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({
      kind: "modify",
      oldPath: repoPath(path),
      newPath: repoPath(path),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: false,
    });
  }
  const oldDirectory = dirname(dirname(cadFixture));
  if (existsSync(oldDirectory)) rmSync(oldDirectory, { recursive: true });
  writeReport(changes);
  console.log(
    JSON.stringify({
      movedFiles: changes.filter(({ kind }) => kind === "move").length,
      modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
      bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length,
    }),
  );
};

const canonicalizeSchemaFixtureCases = () => {
  const marker = `${sep}🧫️fixtures${sep}🧬️mutations${sep}`;
  const owners = new Set<string>();
  for (const path of trackedFiles()) {
    const markerAt = path.indexOf(marker);
    if (markerAt >= 0) owners.add(path.slice(0, markerAt));
  }
  const changes: Change[] = [];
  const report = existsSync(reportPath) ? (JSON.parse(readFileSync(reportPath, "utf8")) as Change[]) : [];
  const replacementsByOwner = new Map<string, Array<[string, string]>>();
  const unmatched: string[] = [];
  let renamedCases = 0;
  for (const owner of owners) {
    const manifestPath = [join(owner, "⚖️oracle", "🔣️.json"), join(owner, "🔮️oracle", "🔣️.json")].find(existsSync) ?? "";
    if (!existsSync(manifestPath)) {
      unmatched.push(`${repoPath(owner)}: no subset oracle manifest`);
      continue;
    }
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as {
      mutationCatalogs?: Array<{
        vectors?: Array<{
          sourceMutationDirectoryName?: string;
          scenarios?: Array<{ directoryName?: string }>;
        }>;
      }>;
    };
    for (const vector of manifest.mutationCatalogs?.flatMap(({ vectors }) => vectors ?? []) ?? []) {
      if (!vector.sourceMutationDirectoryName) continue;
      const fixtureMutation = join(owner, "🧫️fixtures", "🧬️mutations", vector.sourceMutationDirectoryName);
      if (!existsSync(fixtureMutation)) continue;
      const scenarios = vector.scenarios?.filter(({ directoryName }) => directoryName) ?? [];
      const current = readdirSync(fixtureMutation, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map(({ name }) => name);
      for (const scenario of scenarios) {
        const canonical = scenario.directoryName!;
        if (current.includes(canonical)) continue;
        const scenarioCandidates = current.filter((name) => name !== "🔬️unit" && name !== "🔬️mutation-law");
        const candidate = scenarios.length === 1 && scenarioCandidates.length === 1 ? scenarioCandidates[0] : undefined;
        if (!candidate) {
          unmatched.push(`${repoPath(fixtureMutation)}: cannot map ${canonical} among ${current.join(", ")}`);
          continue;
        }
        const oldCase = join(fixtureMutation, candidate);
        const newCase = join(fixtureMutation, canonical);
        if (existsSync(newCase)) throw new Error(`Destination already exists: ${repoPath(newCase)}`);
        renameSync(oldCase, newCase);
        renamedCases++;
        const oldPrefix = `${repoPath(oldCase)}/`;
        const newPrefix = `${repoPath(newCase)}/`;
        for (const entry of report) {
          if (entry.kind === "move" && entry.newPath.startsWith(oldPrefix)) entry.newPath = `${newPrefix}${entry.newPath.slice(oldPrefix.length)}`;
        }
        const ownerReplacements = replacementsByOwner.get(owner) ?? [];
        ownerReplacements.push([
          `🧫️fixtures/🧬️mutations/${slash(vector.sourceMutationDirectoryName)}/${candidate}`,
          `🧫️fixtures/🧬️mutations/${slash(vector.sourceMutationDirectoryName)}/${canonical}`,
        ]);
        replacementsByOwner.set(owner, ownerReplacements);
      }
    }
  }
  mkdirSync(output, { recursive: true });
  writeFileSync(reportPath, `${JSON.stringify(report.sort((a, b) => a.oldPath.localeCompare(b.oldPath)), null, 2)}\n`);
  for (const [owner, replacements] of replacementsByOwner) {
    const result = Bun.spawnSync(["rg", "--files", "-0", owner]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    for (const path of result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).filter(isTextSource)) {
      const before = readFileSync(path);
      if (before.includes(0)) continue;
      let text = before.toString("utf8");
      for (const [from, to] of replacements) text = text.replaceAll(from, to);
      if (text.includes("shared://🧬️mutations/")) text = text.replaceAll("/🧪️tests/", "/");
      const after = Buffer.from(text);
      if (before.equals(after)) continue;
      writeFileSync(path, after);
      changes.push({
        kind: "modify",
        oldPath: repoPath(path),
        newPath: repoPath(path),
        bytesBefore: before.length,
        bytesAfter: after.length,
        sha256Before: sha256(before),
        sha256After: sha256(after),
        bytePreserved: false,
      });
    }
  }
  writeReport(changes);
  writeFileSync(join(output, "📊️schema-fixture-canonicalization-unmatched.json"), `${JSON.stringify(unmatched.sort(), null, 2)}\n`);
  console.log(JSON.stringify({ owners: owners.size, renamedCases, modifiedFiles: changes.length, unmatched: unmatched.length }));
};

const canonicalizeSchemaImplementationCases = () => {
  const marker = `${sep}🧫️fixtures${sep}🧬️mutations${sep}`;
  const owners = new Set<string>();
  for (const path of trackedFiles()) {
    const markerAt = path.indexOf(marker);
    if (markerAt >= 0) owners.add(path.slice(0, markerAt));
  }
  const changes: Change[] = [];
  const replacementsByPlugin = new Map<string, Array<[string, string]>>();
  const unmatched: string[] = [];
  let renamedCases = 0;
  for (const owner of owners) {
    const manifestPath = [join(owner, "⚖️oracle", "🔣️.json"), join(owner, "🔮️oracle", "🔣️.json")].find(existsSync) ?? "";
    if (!existsSync(manifestPath)) continue;
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as {
      mutationCatalogs?: Array<{
        vectors?: Array<{
          sourceMutationDirectoryName?: string;
          scenarios?: Array<{ directoryName?: string }>;
        }>;
      }>;
    };
    for (const vector of manifest.mutationCatalogs?.flatMap(({ vectors }) => vectors ?? []) ?? []) {
      if (!vector.sourceMutationDirectoryName) continue;
      const tests = join(owner, "🧬️schema", "🧬️mutations", vector.sourceMutationDirectoryName, "🧪️tests");
      if (!existsSync(tests)) continue;
      const scenarios = vector.scenarios?.filter(({ directoryName }) => directoryName) ?? [];
      const current = readdirSync(tests, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map(({ name }) => name);
      for (const scenario of scenarios) {
        const canonical = scenario.directoryName!;
        if (current.includes(canonical)) continue;
        const scenarioCandidates = current.filter((name) => name !== "🔬️unit" && name !== "🔬️mutation-law");
        const candidate = scenarios.length === 1 && scenarioCandidates.length === 1 ? scenarioCandidates[0] : undefined;
        if (!candidate) {
          unmatched.push(`${repoPath(tests)}: cannot map ${canonical} among ${current.join(", ")}`);
          continue;
        }
        const oldCase = join(tests, candidate);
        const newCase = join(tests, canonical);
        const result = Bun.spawnSync(["rg", "--files", "-0", oldCase]);
        if (result.exitCode !== 0) throw new Error(result.stderr.toString());
        const moved = result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).map((oldPath) => ({ oldPath, bytes: readFileSync(oldPath) }));
        renameSync(oldCase, newCase);
        renamedCases++;
        for (const { oldPath, bytes } of moved) {
          const newPath = join(newCase, relative(oldCase, oldPath));
          const after = readFileSync(newPath);
          changes.push({
            kind: "move",
            oldPath: repoPath(oldPath),
            newPath: repoPath(newPath),
            bytesBefore: bytes.length,
            bytesAfter: after.length,
            sha256Before: sha256(bytes),
            sha256After: sha256(after),
            bytePreserved: bytes.equals(after),
          });
        }
        const ownerParts = owner.split(sep);
        const pluginsAt = ownerParts.indexOf("🔌️plugins");
        const pluginRoot = ownerParts.slice(0, pluginsAt + 2).join(sep);
        const pluginReplacements = replacementsByPlugin.get(pluginRoot) ?? [];
        pluginReplacements.push([
          `🧬️schema/🧬️mutations/${slash(vector.sourceMutationDirectoryName)}/🧪️tests/${candidate}`,
          `🧬️schema/🧬️mutations/${slash(vector.sourceMutationDirectoryName)}/🧪️tests/${canonical}`,
        ]);
        replacementsByPlugin.set(pluginRoot, pluginReplacements);
      }
    }
  }
  for (const [pluginRoot, replacements] of replacementsByPlugin) {
    const result = Bun.spawnSync(["rg", "--files", "-0", pluginRoot]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    for (const path of result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).filter(isTextSource)) {
      const before = readFileSync(path);
      if (before.includes(0)) continue;
      let text = before.toString("utf8");
      for (const [from, to] of replacements) text = text.replaceAll(from, to);
      const after = Buffer.from(text);
      if (before.equals(after)) continue;
      writeFileSync(path, after);
      changes.push({
        kind: "modify",
        oldPath: repoPath(path),
        newPath: repoPath(path),
        bytesBefore: before.length,
        bytesAfter: after.length,
        sha256Before: sha256(before),
        sha256After: sha256(after),
        bytePreserved: false,
      });
    }
  }
  writeReport(changes);
  writeFileSync(join(output, "📊️schema-implementation-canonicalization-unmatched.json"), `${JSON.stringify(unmatched.sort(), null, 2)}\n`);
  console.log(
    JSON.stringify({
      owners: owners.size,
      renamedCases,
      movedImplementations: changes.filter(({ kind }) => kind === "move").length,
      modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
      unmatched: unmatched.length,
    }),
  );
};

const fixtureResolutionCensus = async () => {
  const modulePath = join(repo, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🧪️test", "📦️packages", "🟦️typescript", "🟦️.ts");
  const platform = await import(pathToFileURL(modulePath).href);
  console.log("[DEBUG] fixture platform imported");
  const cases = platform.discoverTestCases(repo).filter(({ owner }: { owner: string }) => owner.startsWith("✏️s/"));
  console.log(`[DEBUG] discovered ${cases.length} scoped cases`);
  const registry = platform.loadOracleRegistry(repo);
  const records = [] as Array<Record<string, unknown>>;
  for (const discovered of cases) {
    try {
      const built = platform.buildCasePlan(repo, discovered, "exhaustive", registry);
      const uris = platform.fixtureUrisIn(built.feature);
      records.push({
        owner: discovered.owner,
        case: discovered.case,
        featurePath: discovered.featurePath,
        fixtureDirectory: discovered.sharedFixtureDir,
        uris,
        resolved: built.plan.fixtures.map(({ uri, path, digest }: { uri: string; path: string; digest: string }) => ({ uri, path, digest })),
        missing: built.missingFixtures,
      });
    } catch (error) {
      records.push({ owner: discovered.owner, case: discovered.case, featurePath: discovered.featurePath, error: String(error) });
    }
  }
  mkdirSync(output, { recursive: true });
  writeFileSync(join(output, "📊️fixture-resolution.json"), `${JSON.stringify(records, null, 2)}\n`);
  console.log(
    JSON.stringify({
      cases: records.length,
      casesWithUris: records.filter(({ uris }) => Array.isArray(uris) && uris.length > 0).length,
      resolved: records.flatMap(({ resolved }) => Array.isArray(resolved) ? resolved : []).length,
      missing: records.flatMap(({ missing }) => Array.isArray(missing) ? missing : []).length,
      errors: records.filter(({ error }) => error).length,
    }),
  );
};

const repairMissingSharedScenarioReferences = () => {
  const censusPath = join(output, "📊️fixture-resolution.json");
  const records = JSON.parse(readFileSync(censusPath, "utf8")) as Array<{
    featurePath: string;
    fixtureDirectory: string;
    missing?: string[];
  }>;
  const normalizeSegment = (segment: string) => segment.replace(/^[^\p{Letter}\p{Number}]*/u, "");
  const normalizePath = (path: string) => path.split("/").map(normalizeSegment).join("/");
  const replacements = new Map<string, Map<string, string>>();
  const unresolved: string[] = [];
  let matchedUris = 0;
  for (const record of records) {
    const missing = record.missing?.filter((uri) => uri.startsWith("shared://")) ?? [];
    if (missing.length === 0) continue;
    const fixtureDirectory = resolve(repo, record.fixtureDirectory);
    if (!existsSync(fixtureDirectory)) {
      unresolved.push(...missing.map((uri) => `${record.featurePath}: ${uri}: fixture directory absent`));
      continue;
    }
    const result = Bun.spawnSync(["rg", "--files", "-0", fixtureDirectory]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    const actualByNormalized = new Map<string, string[]>();
    for (const path of result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path))) {
      const relativePath = slash(relative(fixtureDirectory, path));
      const key = normalizePath(relativePath);
      actualByNormalized.set(key, [...(actualByNormalized.get(key) ?? []), relativePath]);
    }
    for (const uri of missing) {
      const oldRelative = uri.slice("shared://".length);
      const candidates = actualByNormalized.get(normalizePath(oldRelative)) ?? [];
      if (candidates.length !== 1) {
        unresolved.push(`${record.featurePath}: ${uri}: ${candidates.length} normalized candidates`);
        continue;
      }
      const oldSegments = oldRelative.split("/"), newSegments = candidates[0]!.split("/");
      const differences = oldSegments.flatMap((segment, index) => segment === newSegments[index] ? [] : [[segment, newSegments[index]!] as const]);
      if (differences.length === 0) {
        unresolved.push(`${record.featurePath}: ${uri}: physical target exists but resolver reported missing`);
        continue;
      }
      const byFeature = replacements.get(record.featurePath) ?? new Map<string, string>();
      for (const [from, to] of differences) {
        const prior = byFeature.get(from);
        if (prior && prior !== to) throw new Error(`Ambiguous feature cell ${record.featurePath}: ${from} -> ${prior} or ${to}`);
        byFeature.set(from, to);
      }
      replacements.set(record.featurePath, byFeature);
      matchedUris++;
    }
  }
  const changes: Change[] = [];
  for (const [featurePath, byCell] of replacements) {
    const path = resolve(repo, featurePath), before = readFileSync(path);
    const lines = before.toString("utf8").split("\n").map((line) => {
      if (!line.includes("|")) return line;
      const cells = line.split("|");
      return cells.map((cell) => {
        const value = cell.trim();
        const replaced = value.split("/").map((segment) => byCell.get(segment) ?? segment).join("/");
        return replaced === value ? cell : cell.replace(value, replaced);
      }).join("|");
    });
    const after = Buffer.from(lines.join("\n"));
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  }
  writeReport(changes);
  writeFileSync(join(output, "📊️shared-scenario-reference-unresolved.json"), `${JSON.stringify(unresolved.sort(), null, 2)}\n`);
  console.log(JSON.stringify({ matchedUris, modifiedFeatures: changes.length, unresolved: unresolved.length }));
};

const migrateScannerTestData = () => {
  const scannerPath = join(ticket, "🗑️generated", "coordinator", "fixture-layout-plugins.json");
  const rows = JSON.parse(readFileSync(scannerPath, "utf8")) as Array<{ code: string; path?: string }>;
  const mappings = new Map<string, string>();
  for (const { code, path } of rows) {
    if (code !== "test-data-in-case" || !path) continue;
    const oldPath = resolve(repo, path);
    if (!existsSync(oldPath)) continue;
    const parts = oldPath.split(sep), testsAt = parts.lastIndexOf("🧪️tests");
    if (testsAt < 0) continue;
    const rest = parts.slice(testsAt + 1);
    const fixtureRelative = rest[0] === "🧫️fixtures" ? rest.slice(1) : rest;
    const newPath = join(parts.slice(0, testsAt).join(sep), "🧫️fixtures", ...fixtureRelative);
    const prior = mappings.get(oldPath);
    if (prior && prior !== newPath) throw new Error(`Ambiguous destination for ${path}`);
    mappings.set(oldPath, newPath);
  }
  const changes: Change[] = [];
  for (const [oldPath, newPath] of mappings) {
    if (existsSync(newPath)) throw new Error(`Destination already exists: ${repoPath(newPath)}`);
    const before = readFileSync(oldPath);
    mkdirSync(dirname(newPath), { recursive: true });
    renameSync(oldPath, newPath);
    const after = readFileSync(newPath);
    changes.push({ kind: "move", oldPath: repoPath(oldPath), newPath: repoPath(newPath), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: before.equals(after) });
  }
  for (const path of trackedFiles().filter(isTextSource)) {
    const before = readFileSync(path);
    if (before.includes(0)) continue;
    let text = before.toString("utf8");
    for (const [oldPath, newPath] of mappings) {
      const oldRelative = slash(relative(dirname(path), oldPath));
      const newRelative = slash(relative(dirname(path), newPath));
      text = text.replaceAll(repoPath(oldPath), repoPath(newPath));
      text = text.replaceAll(oldRelative, newRelative);
    }
    const after = Buffer.from(text);
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  }
  writeReport(changes);
  console.log(JSON.stringify({ movedFiles: mappings.size, bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length, modifiedFiles: changes.filter(({ kind }) => kind === "modify").length }));
};

const moveFlowFixtureScript = () => {
  const editor = join(scope, "🔌️plugins", "🌊️flow", "🗿️artifacts", "🌊️flow", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor");
  const oldPath = join(editor, "🧫️fixtures", "📜️script.ts");
  const newPath = join(editor, "🧪️tests", "🔬️source-contract", "🟦️.ts");
  if (!existsSync(oldPath)) throw new Error(`Missing ${repoPath(oldPath)}`);
  if (existsSync(newPath)) throw new Error(`Destination exists ${repoPath(newPath)}`);
  const original = readFileSync(oldPath);
  mkdirSync(dirname(newPath), { recursive: true });
  renameSync(oldPath, newPath);
  const moved = readFileSync(newPath);
  const changes: Change[] = [{ kind: "move", oldPath: repoPath(oldPath), newPath: repoPath(newPath), bytesBefore: original.length, bytesAfter: moved.length, sha256Before: sha256(original), sha256After: sha256(moved), bytePreserved: original.equals(moved) }];
  const adjusted = moved.toString("utf8").replace(/(["'])(\.\.?\/[^"']+)\1/gu, (literal, quote: string, relativePath: string) => {
    const target = resolve(dirname(oldPath), relativePath);
    if (!existsSync(target)) return literal;
    const next = slash(relative(dirname(newPath), target));
    return `${quote}${next.startsWith(".") ? next : `./${next}`}${quote}`;
  });
  const after = Buffer.from(adjusted);
  if (!moved.equals(after)) {
    writeFileSync(newPath, after);
    changes.push({ kind: "modify", oldPath: repoPath(newPath), newPath: repoPath(newPath), bytesBefore: moved.length, bytesAfter: after.length, sha256Before: sha256(moved), sha256After: sha256(after), bytePreserved: false });
  }
  const packageScript = join(scope, "🔌️plugins", "🌊️flow", "📦️packages", "🦀️rust", "📜️script.ts");
  const packageBefore = readFileSync(packageScript);
  const oldImport = slash(relative(dirname(packageScript), oldPath));
  const newImport = slash(relative(dirname(packageScript), newPath));
  const packageAfter = Buffer.from(packageBefore.toString("utf8").replaceAll(oldImport, newImport));
  if (!packageBefore.equals(packageAfter)) {
    writeFileSync(packageScript, packageAfter);
    changes.push({ kind: "modify", oldPath: repoPath(packageScript), newPath: repoPath(packageScript), bytesBefore: packageBefore.length, bytesAfter: packageAfter.length, sha256Before: sha256(packageBefore), sha256After: sha256(packageAfter), bytePreserved: false });
  }
  writeReport(changes);
  console.log(JSON.stringify({ moved: 1, bytePreserved: original.equals(moved), modifiedFiles: changes.filter(({ kind }) => kind === "modify").length }));
};

const moveImplementationOwnedFixtures = () => {
  const scannerPath = join(ticket, "🗑️generated", "coordinator", "fixture-layout-plugins.json");
  const rows = JSON.parse(readFileSync(scannerPath, "utf8")) as Array<{ code: string; path?: string }>;
  const changes: Change[] = [];
  const mappings: Array<[string, string]> = [];
  for (const { code, path } of rows) {
    if (code !== "fixture-owner-delivery-scope" || !path) continue;
    const oldPath = resolve(repo, path);
    if (!existsSync(oldPath)) continue;
    const normalized = repoPath(oldPath), packageAt = normalized.indexOf("/📦️packages/"), fixtureAt = normalized.indexOf("/🧫️fixtures/");
    if (packageAt < 0 || fixtureAt < 0) throw new Error(`Unsupported implementation-owned fixture: ${normalized}`);
    const owner = resolve(repo, normalized.slice(0, packageAt));
    const newPath = join(owner, "🧫️fixtures", normalized.slice(fixtureAt + "/🧫️fixtures/".length));
    if (existsSync(newPath)) throw new Error(`Destination exists: ${repoPath(newPath)}`);
    const before = readFileSync(oldPath);
    mkdirSync(dirname(newPath), { recursive: true });
    renameSync(oldPath, newPath);
    const after = readFileSync(newPath);
    mappings.push([oldPath, newPath]);
    changes.push({ kind: "move", oldPath: normalized, newPath: repoPath(newPath), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: before.equals(after) });
  }
  for (const path of trackedFiles().filter(isTextSource)) {
    const before = readFileSync(path);
    if (before.includes(0)) continue;
    let text = before.toString("utf8");
    for (const [oldPath, newPath] of mappings) {
      text = text.replaceAll(repoPath(oldPath), repoPath(newPath));
      text = text.replaceAll(slash(relative(dirname(path), oldPath)), slash(relative(dirname(path), newPath)));
    }
    if (repoPath(path) === "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts") {
      text = text.replaceAll('join(root, "🧫️fixtures",', 'join(root, "../..", "🧫️fixtures",');
      text = text.replaceAll('join(root, "🧫️fixtures/', 'join(root, "../../🧫️fixtures/');
      text = text.replaceAll('join(this.root, "🧫️fixtures/', 'join(this.root, "../../🧫️fixtures/');
    }
    const after = Buffer.from(text);
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  }
  writeReport(changes);
  console.log(JSON.stringify({ movedFiles: mappings.length, bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length, modifiedFiles: changes.filter(({ kind }) => kind === "modify").length }));
};

const renameProductionFixtureCommands = () => {
  const roots = [
    {
      plugin: join(scope, "🔌️plugins", "🎥️shooting"),
      oldDirectory: join("🗿️artifacts", "🎥️shooting", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🎮️commands", "🗃️fixture"),
      newDirectory: join("🗿️artifacts", "🎥️shooting", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🎮️commands", "📄️document"),
      replacements: [
        ["🗃️fixture", "📄️document"],
        ["commands::fixture", "commands::document"],
        ["pub mod fixture", "pub mod document"],
        [", fixture,", ", document,"],
        ["use fixture::", "use document::"],
        ["whole-fixture", "whole-document"],
        ["dev fixture load", "dev document load"],
        ["fixture_text", "document_text"],
      ],
    },
    {
      plugin: join(scope, "🔌️plugins", "💠️lowpoly"),
      oldDirectory: join("🗿️artifacts", "💠️lowpoly", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🎮️commands", "🧫️fixture"),
      newDirectory: join("🗿️artifacts", "💠️lowpoly", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🎮️commands", "📄️document"),
      replacements: [
        ["🧫️fixture", "📄️document"],
        ["commands::fixture", "commands::document"],
        ["pub mod fixture", "pub mod document"],
        [", fixture,", ", document,"],
        ["use fixture::", "use document::"],
        ["SetFixtureJson", "ReplaceSnapshotJson"],
        ["set_fixture_json", "replace_snapshot_json"],
        ["setFixtureJson", "replaceSnapshotJson"],
        ["set-fixture-json", "replace-snapshot-json"],
        ['"fixture" =>', '"document" =>'],
        ["whole-projection", "whole-document"],
      ],
    },
  ] as const;
  const changes: Change[] = [];
  for (const { plugin, oldDirectory, newDirectory, replacements } of roots) {
    const oldPath = join(plugin, oldDirectory), newPath = join(plugin, newDirectory);
    if (!existsSync(oldPath)) throw new Error(`Missing ${repoPath(oldPath)}`);
    if (existsSync(newPath)) throw new Error(`Destination exists ${repoPath(newPath)}`);
    const result = Bun.spawnSync(["rg", "--files", "-0", oldPath]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    const originals = result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).map((path) => ({ path, bytes: readFileSync(path) }));
    mkdirSync(dirname(newPath), { recursive: true });
    renameSync(oldPath, newPath);
    for (const { path, bytes } of originals) {
      const destination = join(newPath, relative(oldPath, path)), after = readFileSync(destination);
      changes.push({ kind: "move", oldPath: repoPath(path), newPath: repoPath(destination), bytesBefore: bytes.length, bytesAfter: after.length, sha256Before: sha256(bytes), sha256After: sha256(after), bytePreserved: bytes.equals(after) });
    }
    const textFiles = Bun.spawnSync(["rg", "--files", "-0", plugin]).stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).filter(isTextSource);
    for (const path of textFiles) {
      const before = readFileSync(path);
      if (before.includes(0)) continue;
      let text = before.toString("utf8");
      for (const [from, to] of replacements) text = text.replaceAll(from, to);
      const after = Buffer.from(text);
      if (before.equals(after)) continue;
      writeFileSync(path, after);
      changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
    }
  }
  writeReport(changes);
  console.log(JSON.stringify({ movedFiles: changes.filter(({ kind }) => kind === "move").length, bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length, modifiedFiles: changes.filter(({ kind }) => kind === "modify").length }));
};

const repairLedgerRelativePaths = () => {
  const report = JSON.parse(readFileSync(reportPath, "utf8")) as Change[];
  const moves = report.filter(({ kind }) => kind === "move");
  const exact = new Map(moves.map((move) => [move.oldPath, move.newPath]));
  const withoutTests = new Map<string, string | null>();
  for (const move of moves) {
    const key = move.oldPath.replace("/🧪️tests/", "/");
    const prior = withoutTests.get(key);
    withoutTests.set(key, prior && prior !== move.newPath ? null : move.newPath);
  }
  const changes: Change[] = [];
  let replacements = 0;
  for (const path of trackedFiles().filter(isTextSource)) {
    const before = readFileSync(path);
    if (before.includes(0)) continue;
    const text = before.toString("utf8").replace(/(["'])([^"'\n]+)\1/gu, (literal, quote: string, value: string) => {
      if (!(value.includes("/") || value.startsWith("."))) return literal;
      const resolvedOld = repoPath(resolve(dirname(path), value));
      const destination = exact.get(resolvedOld) ?? withoutTests.get(resolvedOld);
      if (!destination) return literal;
      const next = slash(relative(dirname(path), resolve(repo, destination)));
      replacements++;
      return `${quote}${value.startsWith("./") && !next.startsWith(".") ? "./" : ""}${next}${quote}`;
    });
    const after = Buffer.from(text);
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  }
  writeReport(changes);
  console.log(JSON.stringify({ replacements, modifiedFiles: changes.length }));
};

const resolveFinalFixtureGaps = () => {
  const changes: Change[] = [];
  const move = (oldPath: string, newPath: string) => {
    if (!existsSync(oldPath)) throw new Error(`Missing ${repoPath(oldPath)}`);
    if (existsSync(newPath)) throw new Error(`Destination exists ${repoPath(newPath)}`);
    const before = readFileSync(oldPath);
    mkdirSync(dirname(newPath), { recursive: true });
    renameSync(oldPath, newPath);
    const after = readFileSync(newPath);
    changes.push({ kind: "move", oldPath: repoPath(oldPath), newPath: repoPath(newPath), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: before.equals(after) });
  };
  const modify = (path: string, replacements: ReadonlyArray<readonly [string, string]>) => {
    const before = readFileSync(path);
    let text = before.toString("utf8");
    for (const [from, to] of replacements) text = text.replaceAll(from, to);
    const after = Buffer.from(text);
    if (before.equals(after)) return;
    writeFileSync(path, after);
    changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  };
  const normalizeSegment = (segment: string) => segment.replace(/^[^\p{Letter}\p{Number}]*/u, "");
  for (const subset of ["🪪️asset", "💎️material"]) {
    const fixtures = join(scope, "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧊️gltf", "🏅️standards", "🔖️2.0", "🪆️subsets", subset, "🧫️fixtures");
    const result = Bun.spawnSync(["rg", "--files", "-0", fixtures]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    for (const oldPath of result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path))) {
      const relativePath = relative(fixtures, oldPath).split(sep).map(normalizeSegment);
      const newPath = join(fixtures, ...relativePath);
      if (oldPath !== newPath) move(oldPath, newPath);
    }
  }
  const noteFixtureRoot = join(scope, "🔌️plugins", "🗒️note", "🗿️artifacts", "🗒️note", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures");
  const nestedResult = Bun.spawnSync(["rg", "--files", "-0", noteFixtureRoot]);
  if (nestedResult.exitCode !== 0) throw new Error(nestedResult.stderr.toString());
  for (const oldPath of nestedResult.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).filter((path) => path.includes(`${sep}🧪️tests${sep}`))) {
    move(oldPath, oldPath.replace(`${sep}🧪️tests${sep}`, sep));
  }
  const dagAsset = join(scope, "🔌️plugins", "🕸️dag", "🗿️artifacts", "🕸️dag", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🖼️assets", "🎬️demo", "🗣️.dsl.semio");
  if (!existsSync(dagAsset)) {
    const historical = "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
    const restored = Bun.spawnSync(["git", "show", `HEAD:${historical}`], { cwd: repo });
    if (restored.exitCode !== 0) throw new Error(restored.stderr.toString());
    mkdirSync(dirname(dagAsset), { recursive: true });
    writeFileSync(dagAsset, restored.stdout);
    changes.push({ kind: "create", oldPath: historical, newPath: repoPath(dagAsset), bytesBefore: 0, bytesAfter: restored.stdout.length, sha256Before: "", sha256After: sha256(restored.stdout), bytePreserved: false });
  }
  const puzzleFixtureRoot = join(scope, "🔌️plugins", "🧩️puzzle", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures", "🧬️mutations");
  const puzzleCatalog = join(puzzleFixtureRoot, "🔣️.json");
  if (!existsSync(puzzleCatalog)) {
    const result = Bun.spawnSync(["rg", "--files", "-0", puzzleFixtureRoot]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    const files = result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).sort().map((path) => ({ path: slash(relative(puzzleFixtureRoot, path)), sha256: sha256(readFileSync(path)) }));
    const bytes = Buffer.from(`${JSON.stringify({ schema: "semio.fixture-catalog/v1", files }, null, 2)}\n`);
    writeFileSync(puzzleCatalog, bytes);
    changes.push({ kind: "create", oldPath: "", newPath: repoPath(puzzleCatalog), bytesBefore: 0, bytesAfter: bytes.length, sha256Before: "", sha256After: sha256(bytes), bytePreserved: false });
  }
  modify(join(scope, "🔌️plugins", "🧱️block", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧪️tests", "🧱️mutate-block-3d-1", "🥒️.feature"), [["/✏️renames-object-kind-to-pod/", "/🧪️renames-object-kind-to-pod/"]]);
  modify(join(scope, "🔌️plugins", "🧱️block", "🗿️artifacts", "🖐️5d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧪️tests", "🧱️mutate-block-5d-1", "🥒️.feature"), [["/✏️renames-part-kind-to-pod/", "/🧪️renames-part-kind-to-pod/"]]);
  for (const subset of ["🧱️baseline", "🧾️document"]) modify(join(scope, "🔌️plugins", "🗄️stdio", "🗿️artifacts", "📸️jpg", "🏅️standards", "🔖️jfif-1.01", "🪆️subsets", subset, "🧪️tests", subset === "🧱️baseline" ? "🛡️mutate-jpg-jfif-1-01-baseline" : "📸️mutate-jpg-jfif-1-01", "🥒️.feature"), [["shared://🧪️abbau-aufbau-masterarbeit-grundriss/", "shared://🏘️abbau-aufbau-masterarbeit-grundriss/"]]);
  modify(join(scope, "🔌️plugins", "🗒️note", "🗿️artifacts", "🗒️note", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧪️tests", "📜️mutate-note-1-a5bb7f", "🥒️.feature"), [["`shared://📜️mutate-note-1-a5bb7f/`", "this owner's `🧫️fixtures/📜️mutate-note-1-a5bb7f` tree"]]);
  modify(join(scope, "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧿️semio", "🏅️standards", "🔖️v1", "🪆️subsets", "📊️table", "🧪️tests", "📊️mutate-semio-table", "🥒️.feature"), [["`shared://📊️mutate-semio-table/`", "this owner's `🧫️fixtures/📊️mutate-semio-table` tree"]]);
  writeReport(changes);
  console.log(JSON.stringify({ movedFiles: changes.filter(({ kind }) => kind === "move").length, bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length, createdFiles: changes.filter(({ kind }) => kind === "create").length, modifiedFiles: changes.filter(({ kind }) => kind === "modify").length }));
};

const cleanSharedMutationPaths = () => {
  const changes: Change[] = [];
  for (const path of referencingFiles("shared://🧬️mutations/")) {
    const before = readFileSync(path);
    const after = Buffer.from(before.toString("utf8").replaceAll("/🧪️tests/", "/"));
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({
      kind: "modify",
      oldPath: repoPath(path),
      newPath: repoPath(path),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: false,
    });
  }
  writeReport(changes);
  console.log(JSON.stringify({ modifiedFiles: changes.length }));
};

const migrateOwnerAssets = () => {
  const censusPath = join(output, "📊️asset-census.json");
  if (!existsSync(censusPath)) throw new Error("Run asset-census before migrate-owner-assets");
  const records = JSON.parse(readFileSync(censusPath, "utf8")) as Array<{
    sourcePath: string;
    uri: string;
    ownerPath?: string;
    currentTargetExists?: boolean;
  }>;
  const mappings = new Map<string, { owner: string; uri: string; newUri: string; oldPath: string; newPath: string }>();
  const skipped: Array<{ ownerPath?: string; uri: string; reason: string }> = [];
  for (const record of records) {
    if (!record.ownerPath || !record.currentTargetExists) continue;
    const name = record.uri.slice("asset://".length);
    const match = name.match(/^📚️examples\/([^/]+)\/🖼️assets\/(.+)$/u);
    if (!match) {
      skipped.push({ ownerPath: record.ownerPath, uri: record.uri, reason: "not an owner-local example asset" });
      continue;
    }
    const [, example, rest] = match;
    const owner = join(repo, record.ownerPath);
    const oldPath = join(owner, name);
    const newRelative = join(example, rest);
    const newPath = join(owner, "🖼️assets", newRelative);
    mappings.set(`${record.ownerPath}:${record.uri}`, { owner, uri: record.uri, newUri: `asset://${slash(newRelative)}`, oldPath, newPath });
  }
  const changes: Change[] = [];
  for (const mapping of mappings.values()) {
    if (!existsSync(mapping.oldPath)) continue;
    if (existsSync(mapping.newPath)) throw new Error(`Destination already exists: ${repoPath(mapping.newPath)}`);
    const before = readFileSync(mapping.oldPath);
    mkdirSync(dirname(mapping.newPath), { recursive: true });
    renameSync(mapping.oldPath, mapping.newPath);
    const after = readFileSync(mapping.newPath);
    changes.push({
      kind: "move",
      oldPath: repoPath(mapping.oldPath),
      newPath: repoPath(mapping.newPath),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: before.equals(after),
    });
  }
  const byOwner = new Map<string, typeof mappings extends Map<string, infer T> ? T[] : never>();
  for (const mapping of mappings.values()) {
    const ownerMappings = byOwner.get(mapping.owner) ?? [];
    ownerMappings.push(mapping);
    byOwner.set(mapping.owner, ownerMappings);
  }
  for (const [owner, ownerMappings] of byOwner) {
    const result = Bun.spawnSync(["rg", "--files", "-0", owner]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    for (const path of result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).filter(isTextSource)) {
      const before = readFileSync(path);
      if (before.includes(0)) continue;
      let text = before.toString("utf8");
      for (const mapping of ownerMappings) {
        text = text.replaceAll(mapping.uri, mapping.newUri);
        text = text.replaceAll(slash(relative(dirname(path), mapping.oldPath)), slash(relative(dirname(path), mapping.newPath)));
        text = text.replaceAll(repoPath(mapping.oldPath), repoPath(mapping.newPath));
      }
      const after = Buffer.from(text);
      if (before.equals(after)) continue;
      writeFileSync(path, after);
      changes.push({
        kind: "modify",
        oldPath: repoPath(path),
        newPath: repoPath(path),
        bytesBefore: before.length,
        bytesAfter: after.length,
        sha256Before: sha256(before),
        sha256After: sha256(after),
        bytePreserved: false,
      });
    }
  }
  writeReport(changes);
  writeFileSync(join(output, "📊️asset-migration-skipped.json"), `${JSON.stringify(skipped, null, 2)}\n`);
  console.log(
    JSON.stringify({
      mappings: mappings.size,
      movedFiles: changes.filter(({ kind }) => kind === "move").length,
      modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
      bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length,
      skipped: skipped.length,
    }),
  );
};

const migrateRemainingSchemaCaseData = () => {
  const marker = `${sep}🧬️schema${sep}🧬️mutations${sep}`;
  const implementationExtensions = new Set([".c", ".cc", ".cpp", ".cs", ".feature", ".go", ".js", ".jsx", ".py", ".rs", ".ts", ".tsx"]);
  const moves = [] as Array<{ owner: string; oldPath: string; newPath: string; caseRoot: string; rest: string }>;
  for (const oldPath of trackedFiles()) {
    const markerAt = oldPath.indexOf(marker);
    if (markerAt < 0) continue;
    const owner = oldPath.slice(0, markerAt);
    const parts = oldPath.slice(markerAt + marker.length).split(sep);
    const testsAt = parts.indexOf("🧪️tests");
    if (testsAt < 0 || parts.length < testsAt + 3) continue;
    const caseName = parts[testsAt + 1];
    const restParts = parts.slice(testsAt + 2);
    if (restParts.length === 1 && implementationExtensions.has(extname(oldPath))) continue;
    const mutationParts = parts.slice(0, testsAt);
    const newPath = join(owner, "🧫️fixtures", "🧬️mutations", ...mutationParts, caseName, ...restParts);
    if (existsSync(newPath)) throw new Error(`Destination already exists: ${repoPath(newPath)}`);
    moves.push({ owner, oldPath, newPath, caseRoot: join(owner, "🧬️schema", "🧬️mutations", ...mutationParts, "🧪️tests", caseName), rest: restParts.join(sep) });
  }
  const changes: Change[] = [];
  const replacements = new Map<string, Map<string, string>>();
  const touchedOwners = new Set<string>();
  for (const move of moves) {
    const before = readFileSync(move.oldPath);
    mkdirSync(dirname(move.newPath), { recursive: true });
    renameSync(move.oldPath, move.newPath);
    const after = readFileSync(move.newPath);
    touchedOwners.add(move.owner);
    changes.push({
      kind: "move",
      oldPath: repoPath(move.oldPath),
      newPath: repoPath(move.newPath),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: before.equals(after),
    });
    if (!existsSync(move.caseRoot)) continue;
    for (const implementation of readdirSync(move.caseRoot, { withFileTypes: true }).filter((entry) => entry.isFile()).map(({ name }) => join(move.caseRoot, name)).filter(isTextSource)) {
      const byFile = replacements.get(implementation) ?? new Map<string, string>();
      byFile.set(slash(move.rest), slash(relative(dirname(implementation), move.newPath)));
      replacements.set(implementation, byFile);
    }
  }
  for (const [path, byFile] of replacements) {
    const before = readFileSync(path);
    let text = before.toString("utf8");
    for (const [oldInclude, newInclude] of byFile) {
      text = text.replaceAll(`include_str!("${oldInclude}")`, `include_str!("${newInclude}")`);
      text = text.replaceAll(`include_bytes!("${oldInclude}")`, `include_bytes!("${newInclude}")`);
    }
    const after = Buffer.from(text);
    if (before.equals(after)) continue;
    writeFileSync(path, after);
    changes.push({
      kind: "modify",
      oldPath: repoPath(path),
      newPath: repoPath(path),
      bytesBefore: before.length,
      bytesAfter: after.length,
      sha256Before: sha256(before),
      sha256After: sha256(after),
      bytePreserved: false,
    });
  }
  writeReport(changes);
  console.log(
    JSON.stringify({
      owners: touchedOwners.size,
      movedFiles: changes.filter(({ kind }) => kind === "move").length,
      modifiedFiles: changes.filter(({ kind }) => kind === "modify").length,
      bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length,
    }),
  );
};

const canonicalizeGroupedSchemaCases = async () => {
  const testModulePath = join(repo, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🧪️test", "📦️packages", "🟦️typescript", "🟦️.ts");
  const discoveryModulePath = join(repo, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔍️discovery", "🟦️.ts");
  const [testPlatform, discovery] = await Promise.all([import(pathToFileURL(testModulePath).href), import(pathToFileURL(discoveryModulePath).href)]);
  const taxonomy = testPlatform.testTaxonomy(repo);
  const marker = `${sep}🧫️fixtures${sep}🧬️mutations${sep}`;
  const owners = new Set<string>();
  for (const path of trackedFiles()) {
    const markerAt = path.indexOf(marker);
    if (markerAt >= 0) owners.add(path.slice(0, markerAt));
  }
  const report = existsSync(reportPath) ? (JSON.parse(readFileSync(reportPath, "utf8")) as Change[]) : [];
  const changes: Change[] = [];
  const replacementsByPlugin = new Map<string, Array<[string, string]>>();
  const unmatched: string[] = [];
  let fixtureCases = 0, implementationCases = 0;
  for (const owner of owners) {
    const relativeMutationRoot = `${repoPath(owner)}/🧬️schema/🧬️mutations`;
    if (!Object.hasOwn(taxonomy.mutationDomainOwners, relativeMutationRoot)) continue;
    const manifestPath = [join(owner, "⚖️oracle", "🔣️.json"), join(owner, "🔮️oracle", "🔣️.json")].find(existsSync);
    if (!manifestPath) {
      unmatched.push(`${repoPath(owner)}: no oracle manifest`);
      continue;
    }
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as {
      mutationCatalogs?: Array<{
        vectors?: Array<{
          mutationId: string;
          scenarios?: Array<{ directoryName?: string }>;
        }>;
      }>;
    };
    for (const vector of manifest.mutationCatalogs?.flatMap(({ vectors }) => vectors ?? []) ?? []) {
      const mutationPath = discovery.mutationOwnerRelativePath(relativeMutationRoot, vector.mutationId, taxonomy) as string | null;
      if (!mutationPath) {
        unmatched.push(`${relativeMutationRoot}: no registered owner for ${vector.mutationId}`);
        continue;
      }
      const relativeOperation = mutationPath;
      for (const scenario of vector.scenarios?.filter(({ directoryName }) => directoryName) ?? []) {
        const canonical = scenario.directoryName!;
        const fixtureOperation = join(owner, "🧫️fixtures", "🧬️mutations", relativeOperation);
        const tests = join(owner, "🧬️schema", "🧬️mutations", relativeOperation, "🧪️tests");
        const renamed: Array<[string, string]> = [];
        if (existsSync(fixtureOperation) && !existsSync(join(fixtureOperation, canonical))) {
          const candidates = readdirSync(fixtureOperation, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map(({ name }) => name);
          if (candidates.length !== 1) unmatched.push(`${repoPath(fixtureOperation)}: cannot map ${canonical} among ${candidates.join(", ")}`);
          else {
            const oldCase = join(fixtureOperation, candidates[0]), newCase = join(fixtureOperation, canonical);
            renameSync(oldCase, newCase);
            fixtureCases++;
            const oldPrefix = `${repoPath(oldCase)}/`, newPrefix = `${repoPath(newCase)}/`;
            for (const entry of report) if (entry.kind === "move" && entry.newPath.startsWith(oldPrefix)) entry.newPath = `${newPrefix}${entry.newPath.slice(oldPrefix.length)}`;
            renamed.push([`🧫️fixtures/🧬️mutations/${slash(relativeOperation)}/${candidates[0]}`, `🧫️fixtures/🧬️mutations/${slash(relativeOperation)}/${canonical}`]);
          }
        }
        if (existsSync(tests) && !existsSync(join(tests, canonical))) {
          const candidates = readdirSync(tests, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map(({ name }) => name).filter((name) => name !== "🔬️unit" && name !== "🔬️mutation-law");
          if (candidates.length !== 1) unmatched.push(`${repoPath(tests)}: cannot map ${canonical} among ${candidates.join(", ")}`);
          else {
            const oldCase = join(tests, candidates[0]), newCase = join(tests, canonical);
            const result = Bun.spawnSync(["rg", "--files", "-0", oldCase]);
            if (result.exitCode !== 0) throw new Error(result.stderr.toString());
            const moved = result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).map((oldPath) => ({ oldPath, bytes: readFileSync(oldPath) }));
            renameSync(oldCase, newCase);
            implementationCases++;
            for (const { oldPath, bytes } of moved) {
              const newPath = join(newCase, relative(oldCase, oldPath)), after = readFileSync(newPath);
              changes.push({ kind: "move", oldPath: repoPath(oldPath), newPath: repoPath(newPath), bytesBefore: bytes.length, bytesAfter: after.length, sha256Before: sha256(bytes), sha256After: sha256(after), bytePreserved: bytes.equals(after) });
            }
            renamed.push([`🧬️schema/🧬️mutations/${slash(relativeOperation)}/🧪️tests/${candidates[0]}`, `🧬️schema/🧬️mutations/${slash(relativeOperation)}/🧪️tests/${canonical}`]);
          }
        }
        if (renamed.length > 0) {
          const ownerParts = owner.split(sep), pluginsAt = ownerParts.indexOf("🔌️plugins"), pluginRoot = ownerParts.slice(0, pluginsAt + 2).join(sep);
          replacementsByPlugin.set(pluginRoot, [...(replacementsByPlugin.get(pluginRoot) ?? []), ...renamed]);
        }
      }
    }
  }
  mkdirSync(output, { recursive: true });
  writeFileSync(reportPath, `${JSON.stringify(report.sort((a, b) => a.oldPath.localeCompare(b.oldPath)), null, 2)}\n`);
  for (const [pluginRoot, replacements] of replacementsByPlugin) {
    const result = Bun.spawnSync(["rg", "--files", "-0", pluginRoot]);
    if (result.exitCode !== 0) throw new Error(result.stderr.toString());
    for (const path of result.stdout.toString().split("\0").filter(Boolean).map((path) => resolve(path)).filter(isTextSource)) {
      const before = readFileSync(path);
      if (before.includes(0)) continue;
      let text = before.toString("utf8");
      for (const [from, to] of replacements) text = text.replaceAll(from, to);
      const after = Buffer.from(text);
      if (before.equals(after)) continue;
      writeFileSync(path, after);
      changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
    }
  }
  writeReport(changes);
  writeFileSync(join(output, "📊️grouped-schema-canonicalization-unmatched.json"), `${JSON.stringify(unmatched.sort(), null, 2)}\n`);
  console.log(JSON.stringify({ fixtureCases, implementationCases, movedImplementations: changes.filter(({ kind }) => kind === "move").length, modifiedFiles: changes.filter(({ kind }) => kind === "modify").length, unmatched: unmatched.length }));
};

const resolveFinalPluginLayoutFindings = () => {
  const changes: Change[] = [];
  const move = (oldPath: string, newPath: string) => {
    const before = readFileSync(oldPath);
    mkdirSync(dirname(newPath), { recursive: true });
    renameSync(oldPath, newPath);
    const after = readFileSync(newPath);
    changes.push({ kind: "move", oldPath: repoPath(oldPath), newPath: repoPath(newPath), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: before.equals(after) });
  };
  const modify = (path: string, transform: (text: string) => string) => {
    const before = readFileSync(path);
    const after = Buffer.from(transform(before.toString("utf8")));
    if (before.equals(after)) return;
    writeFileSync(path, after);
    changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  };
  const create = (path: string, bytes: Uint8Array) => {
    if (existsSync(path)) throw new Error(`Destination already exists: ${repoPath(path)}`);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, bytes);
    changes.push({ kind: "create", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: 0, bytesAfter: bytes.length, sha256Before: sha256(Buffer.alloc(0)), sha256After: sha256(bytes), bytePreserved: false });
  };
  const extractInlineRustTests = (path: string) => {
    const before = readFileSync(path);
    const text = before.toString("utf8");
    const marker = "\n#[cfg(test)]\nmod tests {\n";
    const markerAt = text.lastIndexOf(marker);
    if (markerAt < 0 || text.slice(markerAt + marker.length).trimEnd().at(-1) !== "}") throw new Error(`Expected final inline test module: ${repoPath(path)}`);
    const moduleText = text.slice(markerAt + marker.length).trimEnd();
    const implementation = Buffer.from(`${moduleText.slice(0, -1).trim()}\n`);
    const testPath = join(dirname(path), "🧪️tests", "🔬️unit", "🦀️.rs");
    create(testPath, implementation);
    const after = Buffer.from(`${text.slice(0, markerAt).trimEnd()}\n\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n`);
    writeFileSync(path, after);
    changes.push({ kind: "modify", oldPath: repoPath(path), newPath: repoPath(path), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  };

  const retirement = join(scope, "🔌️plugins", "🔱️trinity", "🗿️artifacts", "♻️rewriting", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧬️schema", "♻️retirement");
  for (const implementation of ["🦀️.rs", "🟦️.ts"]) move(join(retirement, "🧪️tests", implementation), join(retirement, "🧪️tests", "🔬️document-retirement", implementation));
  modify(join(retirement, "🦀️.rs"), (text) => text.replace('🧪️tests/🦀️.rs', '🧪️tests/🔬️document-retirement/🦀️.rs'));
  for (const implementation of ["🦀️.rs", "🟦️.ts"]) modify(join(retirement, "🧪️tests", "🔬️document-retirement", implementation), (text) => text.replace('../🧫️fixtures/🔣️.json', '../../🧫️fixtures/🔣️.json'));

  const stdioBinary = join(scope, "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧿️semio", "🏅️standards", "🔖️v1", "🪆️subsets", "🌊️flow", "🧬️schema", "📸️snapshot", "💾️binary");
  for (const relativePath of ["🔣️.json", join("♻️lifecycle", "🔣️.json")]) move(join(stdioBinary, "🧫️fixture", relativePath), join(stdioBinary, "🧫️fixtures", relativePath));
  modify(join(stdioBinary, "🧪️tests", "💾️binary", "🦀️.rs"), (text) => text.replaceAll("../../🧫️fixture/", "../../🧫️fixtures/"));
  rmSync(join(stdioBinary, "🧫️fixture"), { recursive: true, force: true });

  const gisConfig = join(scope, "🔌️plugins", "🌍️gis", "🗿️artifacts", "🗺️gismap", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🎚️config", "🧬️schema", "🦀️.rs");
  modify(gisConfig, (text) => text.replace('../../👥️presence/🧬️schema/🧫️fixtures/🔣️.json', '../../👥️presence/🧬️schema/🔣️.json'));

  for (const path of [
    join(scope, "🔌️plugins", "🔱️trinity", "🗿️artifacts", "🔌️jack", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🎮️commands", "▶️run-query", "🧵️job", "🦀️.rs"),
    join(scope, "🔌️plugins", "🗒️note", "🗿️artifacts", "🗒️note", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🪟️window", "🦀️.rs"),
    join(scope, "🔌️plugins", "🧩️puzzle", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🪟️window", "🦀️.rs"),
    join(scope, "🔌️plugins", "🧩️puzzle", "🗿️artifacts", "🖐️5d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🪟️window", "🦀️.rs"),
    join(scope, "🔌️plugins", "🧩️puzzle", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "✏️editor", "🪟️window", "🦀️.rs"),
  ]) extractInlineRustTests(path);

  const energySubset = join(scope, "🔌️plugins", "🔋️energy", "🗿️artifacts", "🔋️model", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any");
  const malformedEnergySubset = join(scope, "🔌️plugins", "🔋️energy", "🗿️artifacts", "🔋️model", "🗿️artifacts", "🔋️model", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any");
  for (const entry of readdirSync(join(energySubset, "📚️examples"), { withFileTypes: true }).filter((entry) => entry.isDirectory() && entry.name.startsWith("🏛️bestest-"))) {
    const oldAsset = join(energySubset, "📚️examples", entry.name, "🖼️assets", "🗣️.dsl.semio");
    const asset = join(energySubset, "🖼️assets", entry.name, "🗣️.dsl.semio");
    move(oldAsset, asset);
    const generatedAsset = readFileSync(join(malformedEnergySubset, "🖼️assets", entry.name, "🗣️.dsl.semio"));
    const emptyAsset = readFileSync(asset);
    writeFileSync(asset, generatedAsset);
    changes.push({ kind: "modify", oldPath: repoPath(asset), newPath: repoPath(asset), bytesBefore: emptyAsset.length, bytesAfter: generatedAsset.length, sha256Before: sha256(emptyAsset), sha256After: sha256(generatedAsset), bytePreserved: false });
    create(join(energySubset, "🧫️fixtures", entry.name, "🔋️model.json"), readFileSync(join(malformedEnergySubset, "🧫️fixtures", entry.name, "🔋️model.json")));
  }
  rmSync(join(scope, "🔌️plugins", "🔋️energy", "🗿️artifacts", "🔋️model", "🗿️artifacts"), { recursive: true, force: true });
  const energyTest = join(scope, "🔌️plugins", "🔋️energy", "🔨️modules", "⚡️simulation", "⚙️engine", "🏛️bestest", "🧪️tests", "🔬️unit", "🦀️.rs");
  modify(energyTest, (text) => text.replace('../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any', '../../🏅️standards/🔖️1/🪆️subsets/✳️any'));

  writeReport(changes);
  console.log(JSON.stringify({ movedFiles: changes.filter(({ kind }) => kind === "move").length, createdFiles: changes.filter(({ kind }) => kind === "create").length, modifiedFiles: changes.filter(({ kind }) => kind === "modify").length, bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length }));
};

const resolveRemainingFixtureUris = () => {
  const changes: Change[] = [];
  const move = (oldPath: string, newPath: string) => {
    const before = readFileSync(oldPath);
    mkdirSync(dirname(newPath), { recursive: true });
    renameSync(oldPath, newPath);
    const after = readFileSync(newPath);
    changes.push({ kind: "move", oldPath: repoPath(oldPath), newPath: repoPath(newPath), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: before.equals(after) });
  };
  const gltfFixtures = join(scope, "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧊️gltf", "🏅️standards", "🔖️2.0", "🪆️subsets", "💎️material", "🧫️fixtures");
  const scenarios: Array<[string, string]> = [
    ["material/change-alpha", "change-material-alpha-mode-applied"],
    ["material/change-sides", "change-material-double-sided-applied"],
    ["material/create", "create-material-applied"],
    ["material/delete", "delete-material-applied"],
    ["material/move", "move-material-applied"],
    ["material/reorder", "reorder-materials-applied"],
    ["texture/create", "create-texture-applied"],
    ["texture/delete", "delete-texture-applied"],
    ["texture/move", "move-texture-applied"],
    ["texture/reorder", "reorder-textures-applied"],
    ["image/create", "create-image-applied"],
    ["image/delete", "delete-image-applied"],
    ["image/move", "move-image-applied"],
    ["image/reorder", "reorder-images-applied"],
    ["sampler/create", "create-sampler-applied"],
    ["sampler/delete", "delete-sampler-applied"],
    ["sampler/move", "move-sampler-applied"],
    ["sampler/reorder", "reorder-samplers-applied"],
  ];
  for (const [from, to] of scenarios) {
    for (const name of ["before.gltf", "after.gltf"]) move(join(gltfFixtures, ...from.split("/"), name), join(gltfFixtures, to, name));
  }
  for (const family of ["material", "texture", "image", "sampler"]) rmSync(join(gltfFixtures, family), { recursive: true, force: true });

  const noteFeature = join(scope, "🔌️plugins", "🗒️note", "🗿️artifacts", "🗒️note", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧪️tests", "📜️mutate-note-1-a5bb7f", "🥒️.feature");
  const before = readFileSync(noteFeature);
  const after = Buffer.from(before.toString("utf8").replaceAll("🏷️rename-note/🧪️tests/🏷️retitles-the-document", "🏷️rename-note/🏷️retitles-the-document"));
  writeFileSync(noteFeature, after);
  changes.push({ kind: "modify", oldPath: repoPath(noteFeature), newPath: repoPath(noteFeature), bytesBefore: before.length, bytesAfter: after.length, sha256Before: sha256(before), sha256After: sha256(after), bytePreserved: false });
  const noteFixtureTests = join(scope, "🔌️plugins", "🗒️note", "🗿️artifacts", "🗒️note", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures", "📜️mutate-note-1-a5bb7f", "🏷️rename-note", "🧪️tests");
  rmSync(noteFixtureTests, { recursive: true, force: true });
  writeReport(changes);
  console.log(JSON.stringify({ movedFiles: changes.filter(({ kind }) => kind === "move").length, modifiedFiles: changes.filter(({ kind }) => kind === "modify").length, bytePreservedMoves: changes.filter(({ kind, bytePreserved }) => kind === "move" && bytePreserved).length }));
};

const [command, argument] = Bun.argv.slice(2);
if (command === "census") census();
else if (command === "migrate-local") migrateLocal();
else if (command === "clean-local-markers") cleanLocalMarkers();
else if (command === "rebuild-local-report") rebuildLocalReport();
else if (command === "asset-census") assetCensus();
else if (command === "migrate-owner-fixture-uris") migrateOwnerFixtureUris();
else if (command === "migrate-schema-vector-family") migrateSchemaVectorFamily(argument);
else if (command === "migrate-all-schema-vector-families") migrateAllSchemaVectorFamilies();
else if (command === "migrate-production-assets") migrateProductionAssets();
else if (command === "canonicalize-schema-fixture-cases") canonicalizeSchemaFixtureCases();
else if (command === "canonicalize-schema-implementation-cases") canonicalizeSchemaImplementationCases();
else if (command === "fixture-resolution-census") await fixtureResolutionCensus();
else if (command === "repair-missing-shared-scenario-references") repairMissingSharedScenarioReferences();
else if (command === "migrate-scanner-test-data") migrateScannerTestData();
else if (command === "move-flow-fixture-script") moveFlowFixtureScript();
else if (command === "move-implementation-owned-fixtures") moveImplementationOwnedFixtures();
else if (command === "rename-production-fixture-commands") renameProductionFixtureCommands();
else if (command === "repair-ledger-relative-paths") repairLedgerRelativePaths();
else if (command === "resolve-final-fixture-gaps") resolveFinalFixtureGaps();
else if (command === "clean-shared-mutation-paths") cleanSharedMutationPaths();
else if (command === "migrate-owner-assets") migrateOwnerAssets();
else if (command === "migrate-remaining-schema-case-data") migrateRemainingSchemaCaseData();
else if (command === "canonicalize-grouped-schema-cases") await canonicalizeGroupedSchemaCases();
else if (command === "resolve-final-plugin-layout-findings") resolveFinalPluginLayoutFindings();
else if (command === "resolve-remaining-fixture-uris") resolveRemainingFixtureUris();
else throw new Error("Usage: bun 📜️script.ts <census|migrate-local|clean-local-markers|rebuild-local-report|asset-census|migrate-owner-fixture-uris|migrate-schema-vector-family|migrate-all-schema-vector-families|migrate-production-assets|canonicalize-schema-fixture-cases|canonicalize-schema-implementation-cases|fixture-resolution-census|repair-missing-shared-scenario-references|migrate-scanner-test-data|move-flow-fixture-script|move-implementation-owned-fixtures|rename-production-fixture-commands|repair-ledger-relative-paths|resolve-final-fixture-gaps|clean-shared-mutation-paths|migrate-owner-assets|migrate-remaining-schema-case-data|canonicalize-grouped-schema-cases|resolve-final-plugin-layout-findings|resolve-remaining-fixture-uris>");
