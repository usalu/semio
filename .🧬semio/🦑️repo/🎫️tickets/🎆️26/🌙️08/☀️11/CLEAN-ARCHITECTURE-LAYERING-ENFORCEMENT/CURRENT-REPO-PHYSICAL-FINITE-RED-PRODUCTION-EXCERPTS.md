# Current Repo RED Relevant Production Excerpts

Read-only exact line excerpts; whole originals are already preserved in Root captures. These are current source epochs and are not substitutes for the earlier Native execution epoch.

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts

Whole-source SHA-256 `363271c9dfd4db2338b0b6254b6590ace4ab74c21c17ed55dd24f91dbf6ec69f`.

Lines 1000–1018:

```ts
}

function stableViolations(rows: readonly TaxonomyViolation[]): readonly TaxonomyViolation[] {
  return [...new Map(rows.map((entry) => [`${entry.path}\u0000${entry.code}\u0000${entry.severity}\u0000${entry.message}`, entry])).values()].sort((a, b) => a.path.localeCompare(b.path) || a.code.localeCompare(b.code) || a.message.localeCompare(b.message));
}

function checkCancellation(repoRoot: string, cancelFile?: string): void {
  if (!cancelFile) return;
  const path = assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true);
  if (existsSync(path)) throw new TaxonomyCancellationError();
}

function cancellationRequested(repoRoot: string, cancelFile?: string): boolean {
  if (!cancelFile) return false;
  return existsSync(assertLexicalInputOutsideOpaque(repoRoot, cancelFile, "cancelFile", true));
}
//#endregion 🧾️Source Admission

interface CandidatePath {
```

Lines 2450–2523:

```ts
function rustContextFiles(path: string, index: ReferencePathIndex): readonly string[] {
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const views = rustReferenceContextFiles.get(index) ?? new Map<string, readonly string[]>();
  rustReferenceContextFiles.set(index, views);
  const cached = views.get(coordinateRoot);
  if (cached) return cached;
  const files = index.contextPaths.filter((candidate) => (candidate.endsWith(".rs") || basename(candidate) === "Cargo.toml") && (ancestorReferenceCoordinateRoot(candidate, index.coordinateRootSet) ?? "") === coordinateRoot);
  views.set(coordinateRoot, files);
  return files;
}

function unprovenRustReferenceTargets(referencePath: string, value: string, index: ReferencePathIndex): readonly string[] {
  const cache = rustUnprovenReferenceTargets.get(index) ?? new Map<string, readonly string[]>();
  rustUnprovenReferenceTargets.set(index, cache);
  const coordinateRoot = ancestorReferenceCoordinateRoot(referencePath, index.coordinateRootSet), key = `${coordinateRoot ?? ""}\0${value}`;
  const cached = cache.get(key);
  if (cached) return cached;
  const decoded = unsupportedReferenceTokens(`"${value}"`, "rust")[0]?.targetValues?.[0] ?? value, suffix = decoded.replace(/^(?:\.\.?\/)+/u, "");
  const explicitEscape = /^(?:\.\.\/|\/|[A-Za-z]:[\\/])/u.test(decoded);
  const targets = [...index.exact].filter((candidate) => (!coordinateRoot || candidate.startsWith(`${coordinateRoot}/`) || explicitEscape) && suffix !== "" && (candidate === suffix || candidate.endsWith(`/${suffix}`)));
  cache.set(key, targets);
  return targets;
}

function rustReferenceNeedsOwnership(path: string, references: ReturnType<typeof inspectRustManifestPathReferences>, index: ReferencePathIndex, candidates: ReturnType<typeof inspectRustManifestPathCandidates> = []): boolean {
  if (index.affectedPaths.has(path)) return true;
  const affected = (candidate: string): boolean => index.affectedPaths.has(candidate) || index.affectedPaths.has(index.nfc.get(candidate.normalize("NFC")) ?? "");
  const manifests = rustContextFiles(path, index).filter((candidate) => basename(candidate) === "Cargo.toml");
  for (const reference of references) {
    if (unprovenRustReferenceTargets(path, reference.value, index).some(affected)) return true;
    for (const manifest of manifests) {
      try { if (affected(normalizeRelative(posix.join(posix.dirname(manifest), ...reference.base, reference.value)))) return true; } catch {}
    }
  }
  for (const candidate of candidates) {
    if (unprovenRustReferenceTargets(path, candidate.value, index).some(affected)) return true;
    for (const manifest of manifests) for (const parts of candidate.targets) {
      try { if (affected(normalizeRelative(posix.join(posix.dirname(manifest), ...parts)))) return true; } catch {}
    }
  }
  return false;
}

function rustReferenceGraph(path: string, index: ReferencePathIndex): RustReferenceGraphView | null {
  if (!index.repoRoot) return null;
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const views = rustReferenceGraphs.get(index) ?? new Map<string, RustReferenceGraphView>();
  rustReferenceGraphs.set(index, views);
  const cached = views.get(coordinateRoot);
  if (cached) return cached;
  const files = rustContextFiles(path, index);
  const contents = new Map<string, string | undefined>(), hashes = new Map<string, string>(), unreadableInputs = new Map<string, Error>();
  const read = (candidate: string): string | undefined => {
    checkCancellation(index.repoRoot!, index.cancelFile);
    if (contents.has(candidate)) return contents.get(candidate);
    let absolute: string;
    try { absolute = assertLexicalInputOutsideOpaque(index.repoRoot!, candidate, "Rust module ownership", true); }
    catch (error) { unreadableInputs.set(candidate, error instanceof Error ? error : new Error(String(error))); contents.set(candidate, undefined); return undefined; }
    const stat = lstatOrNull(absolute);
    if (!stat) { contents.set(candidate, undefined); return undefined; }
    if (!stat.isFile()) { unreadableInputs.set(candidate, new Error(`Rust module ownership is not a regular file: ${candidate}`)); contents.set(candidate, undefined); return undefined; }
    const bytes = readFileSync(absolute), after = lstatSync(absolute);
    if (after.mode !== stat.mode || after.size !== stat.size || after.mtimeMs !== stat.mtimeMs || bytes.byteLength !== stat.size) throw new Error(`Rust module ownership changed during its snapshot: ${candidate}`);
    hashes.set(candidate, sha256(bytes));
    contents.set(candidate, bytes.toString("utf8"));
    return contents.get(candidate);
  };
  const graph = inspectRustModuleGraph(files, read, { strictManifests: true, checkCancellation: () => checkCancellation(index.repoRoot!, index.cancelFile) });
  const view = { graph, hashes, unreadableInputs };
  views.set(coordinateRoot, view);
  return view;
}

/** 🧮️ Admits a complete finite interpretation only through one unchanged physical Cargo source chain. An ancestor's glob import (`use x::*`) is never disqualifying here — file participation comes only from `mod`/`#[path]` declarations, which `inspectRustModuleGraphFacts` already tracks completely regardless of glob re-exports; a glob only affects NAME resolution, never which physical files exist in the graph. A non-literal `.join(...)` argument is separately, structurally unrepresentable by every extractor this function consumes (`inspectRustManifestPathReferences`/`inspectRustManifestPathCandidates`/`inspectRustJoinArgumentSpans` all require a string literal, or a loop bound to string literals, to record anything at all) — so it can never reach this proof as a false `finite` positive regardless of glob imports. */
```

Lines 2606–2726:

```ts
function rustFiniteManifestTargets(path: string, content: string, candidates: ReturnType<typeof inspectRustManifestPathCandidates>, index: ReferencePathIndex, view: RustReferenceGraphView | null): ReadonlyMap<number, readonly string[]> {
  const result = new Map<number, readonly string[]>(), contexts = view?.graph.contexts.get(path) ?? [];
  if (!index.repoRoot || !view || !contexts.length || contexts.some((context) => context.manifestPath === null)) return result;
  const manifests = [...new Set(contexts.map((context) => context.manifestPath!))];
  if (manifests.length !== 1 || view.hashes.get(path) !== sha256(content)) return result;
  const coordinateRoot = ancestorReferenceCoordinateRoot(path, index.coordinateRootSet) ?? "";
  const sameRoot = (target: string): boolean => (index.coordinateRootSet.has(target) ? target : ancestorReferenceCoordinateRoot(target, index.coordinateRootSet) ?? "") === coordinateRoot;
  const proofPaths = [...new Set(contexts.flatMap((context) => [context.manifestPath!, ...context.sourceChain]))];
  if (!proofPaths.includes(path)) return result;
  const physicalPath = (base: string, parts: readonly string[], requireDirectory = false): string => {
    if (parts.length === 0 || parts.some((part) => posix.isAbsolute(part) || /^[A-Za-z]:/u.test(part) || part.includes("\\") || part.includes("\u0000"))) throw new Error("Rust finite path has no local physical identity");
    let current = normalizeRelative(base);
    if (!sameRoot(current) || !lstatOrNull(assertLexicalInputOutsideOpaque(index.repoRoot!, current, "Rust finite physical base", true))?.isDirectory()) throw new Error("Rust finite path has no coordinate-local physical base");
    const segments = parts.join("/").split("/");
    for (let step = 0; step < segments.length; step++) {
      checkCancellation(index.repoRoot!, index.cancelFile);
      current = normalizeRelative(posix.join(current, segments[step]!));
      if (!sameRoot(current)) throw new Error("Rust finite path step escapes its coordinate root");
      const absolute = assertLexicalInputOutsideOpaque(index.repoRoot!, current, "Rust finite path step", true), stat = lstatOrNull(absolute);
      const directory = requireDirectory || step + 1 < segments.length || ["", ".", ".."].includes(segments[step]!);
      if (!stat || (directory ? !stat.isDirectory() : !stat.isFile() && !stat.isDirectory())) throw new Error("Rust finite path step is not a physical file or directory");
    }
    return current;
  };
  const contents = new Map<string, string>();
  try {
    for (const source of proofPaths) {
      checkCancellation(index.repoRoot, index.cancelFile);
      if (!index.contextPathSet.has(source) || !sameRoot(source) || !view.hashes.has(source)) return result;
      const absolute = assertLexicalInputOutsideOpaque(index.repoRoot, source, "Rust finite source authority", true), before = lstatOrNull(absolute);
      if (!before?.isFile()) return result;
      const bytes = readFileSync(absolute), after = lstatSync(absolute);
      if (after.mode !== before.mode || after.size !== before.size || after.mtimeMs !== before.mtimeMs || bytes.byteLength !== before.size || sha256(bytes) !== view.hashes.get(source)) return result;
      contents.set(source, bytes.toString("utf8"));
    }
    const manifest = inspectRustCargoManifest(contents.get(manifests[0]!)!, true);
    if (!manifest.valid || manifest.dependencies.includes("std")) return result;
    const facts = new Map(proofPaths.filter((source) => source.endsWith(".rs")).map((source) => [source, inspectRustModuleGraphFacts(contents.get(source)!)]));
    const parentImports = facts.get(path)?.uses.some((use) => /^(?:super::)+\*$/u.test(use.specifier)) ?? false;
    for (const source of proofPaths.filter((source) => source.endsWith(".rs") && source !== path)) {
      const text = rustCodeOnlyTextForMacroTrust(contents.get(source)!);
      const withoutPathAttributes = text.replace(/#\s*\[\s*path\s*=\s*"[^"\\]*"\s*\]/gu, "");
      if (/[#!]/u.test(withoutPathAttributes) || /\bmacro\b/u.test(text) || parentImports && /\b(?:std|env)\b/u.test(text)) return result;
    }
    for (const context of contexts) {
      if (physicalPath(posix.dirname(manifests[0]!), [manifest.libPath ?? "src/lib.rs"]) !== context.crateRoot) return result;
      for (let chain = 0; chain + 1 < context.sourceChain.length; chain++) {
        const source = context.sourceChain[chain]!, next = context.sourceChain[chain + 1]!;
        const owners = (view.graph.contexts.get(source) ?? []).filter((owner) => owner.manifestPath === context.manifestPath && owner.crateRoot === context.crateRoot && owner.sourceChain.length === chain + 1 && owner.sourceChain.every((item, index) => item === context.sourceChain[index]) && owner.modulePath.every((item, index) => item === context.modulePath[index]));
        let proven = 0;
        for (const owner of owners) {
          const sourceFacts = facts.get(source);
          if (!sourceFacts || rustModuleScopeProof(sourceFacts, owner.sourceScope).state === "unresolved") return result;
          for (const module of sourceFacts.modules) {
          if (module.modulePath.length !== owner.sourceScope.length + 1 || !owner.sourceScope.every((item, index) => item === module.modulePath[index])) continue;
          const modulePath = [...owner.modulePath, module.name];
          if (!modulePath.every((item, index) => item === context.modulePath[index])) continue;
          if (module.unresolved) return result;
          if (module.inline) { if (module.pathTarget !== null) physicalPath(owner.moduleBase, [module.pathTarget], true); continue; }
          if (view.graph.targets.get(`${context.crateRoot}\0${modulePath.join("::")}`) !== next) continue;
          const base = module.pathTarget !== null && owner.sourceScope.length === 0 ? posix.dirname(source) : owner.moduleBase;
          const raw = module.pathTarget ?? (next === posix.join(base, `${module.name}.rs`) ? `${module.name}.rs` : `${module.name}/mod.rs`);
          if (physicalPath(base, [raw]) !== next) return result;
          proven++;
        }
        }
        if (proven !== 1) return result;
      }
    }
  } catch (error) {
    if (error instanceof TaxonomyCancellationError) throw error;
    return result;
  }
  for (const candidate of candidates) {
    if (!Number.isInteger(candidate.start) || !Number.isInteger(candidate.end) || candidate.start < 0 || candidate.end <= candidate.start || content.slice(candidate.start, candidate.end) !== candidate.value || candidate.targets.length === 0 || candidate.targets.length > 256) continue;
    try {
      const targets = new Set<string>();
      for (const parts of candidate.targets) {
        checkCancellation(index.repoRoot, index.cancelFile);
        const target = normalizeRelative(posix.join(posix.dirname(manifests[0]!), ...parts));
        if (!index.contextPathSet.has(target) || !sameRoot(target)) throw new Error("Rust finite target lacks coordinate-local admission");
        if (physicalPath(posix.dirname(manifests[0]!), parts) !== target) throw new Error("Rust finite target identity changed");
        targets.add(target);
      }
      result.set(candidate.start, [...targets].sort(generatorPathCompare));
    } catch (error) {
      if (error instanceof TaxonomyCancellationError) throw error;
    }
  }
  return result;
}

function rustManifestReferenceTokens(path: string, content: string, index: ReferencePathIndex): readonly ReferenceToken[] {
  const references = inspectRustManifestPathReferences(content);
  const arguments_ = inspectRustJoinArgumentSpans(content);
  const candidates = inspectRustManifestPathCandidates(content);
  if (references.length === 0 && arguments_.length === 0 && candidates.length === 0) return [];
  const view = !rustReferenceNeedsOwnership(path, references, index, candidates) ? null : rustReferenceGraph(path, index), contexts = view?.graph.contexts.get(path) ?? [];
  for (const [candidate, error] of view?.unreadableInputs ?? []) if (candidate === path || basename(candidate) === "Cargo.toml" && (posix.dirname(candidate) === "." || path.startsWith(`${posix.dirname(candidate)}/`))) throw error;
  if (view?.hashes.has(path) && view.hashes.get(path) !== sha256(content)) throw new Error(`Rust reference source changed during ownership resolution: ${path}`);
  const manifests = [...new Set(contexts.map((context) => context.manifestPath).filter((manifest): manifest is string => manifest !== null))];
  const proofPaths = [...new Set(contexts.flatMap((context) => [context.manifestPath!, ...context.sourceChain]))].sort(generatorPathCompare);
  const digest = sha256(canonicalJson(proofPaths.map((source) => ({ path: source, sha256: view?.hashes.get(source) }))));
  const conflicts = (left: Pick<ReferenceToken, "start" | "end">, right: Pick<ReferenceToken, "start" | "end">): boolean => left.start === right.start || left.start < right.end && right.start < left.end;
  const writableInputs = new Set(references.filter((reference, index) => !references.some((other, otherIndex) => index !== otherIndex && conflicts(reference, other))));
  const finiteInputs = new Set(candidates.filter((candidate, index) => !references.some((reference) => conflicts(candidate, reference)) && !candidates.some((other, otherIndex) => index !== otherIndex && conflicts(candidate, other))));
  const finite = rustFiniteManifestTargets(path, content, [...Array.from(writableInputs, (reference) => ({ start: reference.start, end: reference.end, value: reference.value, targets: [[...reference.base, reference.value]] })), ...finiteInputs], index, view);
  const rows: ReferenceToken[] = references.map((reference) => {
    let sourceBase: string | undefined, physicalTargets: string[] = [], unsupportedReason: string | undefined;
    const targets = writableInputs.has(reference) ? finite.get(reference.start) : undefined;
    try {
      if (manifests.length !== 1) unsupportedReason = `Rust manifest-relative path requires one proven Cargo owner, found ${manifests.length}`;
      // 🔀️ Two structurally distinct failures were previously conflated into one message: `targets`
      // is `undefined` when this reference never reached `rustFiniteManifestTargets`'s per-candidate
      // map at all (an early whole-file guard bailed, or its own try/catch threw for this candidate
      // specifically) — no proof was ever attempted. `targets.length !== 1` with `targets` DEFINED
      // means the proof ran to completion and found zero or several distinct physical targets — a
      // genuine ambiguity, not a missing proof. Three earlier diagnoses on this codebase (see
      // `rust-path-join` ticket history) mistook the first for the second; keep them apart.
      else if (targets === undefined) unsupportedReason = "Rust manifest-relative path was never admitted into a proven physical source chain";
      else if (targets.length !== 1) unsupportedReason = `Rust manifest-relative path resolved to ${targets.length} distinct physical targets, not exactly one`;
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts

Whole-source SHA-256 `4aa09dbf067d976284df77f9e073f11eb4d405698132597823fdad1a73f60292`.

Lines 8017–8052:

```ts
export function inspectRustModuleGraph(files: readonly string[], readSource: (path: string) => string | undefined, options: Readonly<{ conventionalRoots?: boolean; strictManifests?: boolean; checkCancellation?: () => void; compileReferences?: ReadonlyMap<string, readonly RustCompileReference[]> }> = {}): RustModuleGraph {
  const compare = (left: string, right: string): number => Buffer.from(left).compare(Buffer.from(right));
  const sourceFiles = new Set(files.filter((path) => path.endsWith(".rs"))), sources = new Map<string, string | undefined>(), factsBySource = new Map<string, RustModuleGraphFacts>();
  const targets = new Map<string, string>(), ambiguousTargets = new Set<string>(), unresolvedTargets = new Set<string>(), contexts = new Map<string, RustModuleContext[]>(), namedCrates = new Map<string, string[]>(), dependencies = new Map<string, readonly string[]>(), invalidManifests = new Set<string>();
  const origins: { target: Readonly<{ kind: "source"; path: string } | { kind: "unresolved"; rawPath: string }>; context: RustModuleContext; reason?: RustModuleParticipationReason }[] = [], manifestReasons = new Map<string, "invalid-manifest" | "unavailable-manifest">(), deniedKeys = new Map<string, RustModuleParticipationReason>();
  const precedence: readonly RustModuleParticipationReason[] = ["invalid-manifest", "unavailable-manifest", "nonportable-target", "unresolved-scope", "conflicting-target", "cyclic-target", "unresolved-target", "unavailable-source"];
  const read = (path: string): string | undefined => { if (!sources.has(path)) sources.set(path, readSource(path)); return sources.get(path); };
  const sourceFacts = (path: string): RustModuleGraphFacts => {
    if (!factsBySource.has(path)) { const source = read(path); factsBySource.set(path, source === undefined ? { modules: [], uses: [], includes: [], scopes: [] } : inspectRustModuleGraphFacts(source)); }
    return factsBySource.get(path)!;
  };
  const deny = (crateRoot: string, modulePath: readonly string[], reason: RustModuleParticipationReason): void => {
    const key = crateRoot + "\0" + modulePath.join("::"), prior = deniedKeys.get(key);
    unresolvedTargets.add(key);
    if (!prior || precedence.indexOf(reason) < precedence.indexOf(prior)) deniedKeys.set(key, reason);
  };
  const manifestRoots = files.filter((path) => path === "Cargo.toml" || path.endsWith("/Cargo.toml")).sort(compare).flatMap((manifest) => {
    options.checkCancellation?.();
    const source = read(manifest), facts = inspectRustCargoManifest(source ?? "", options.strictManifests === true);
    if (!facts.valid) { invalidManifests.add(manifest); manifestReasons.set(manifest, source === undefined ? "unavailable-manifest" : "invalid-manifest"); }
    if (facts.kind === "workspace" || !facts.valid && facts.crateName === null && facts.libPath === null) return [];
    const entry = posix.normalize(posix.join(posix.dirname(manifest), facts.libPath ?? "src/lib.rs"));
    if (entry.startsWith("../") || posix.isAbsolute(entry) || /^[A-Za-z]:/u.test(entry)) return [];
    return [{ path: entry, manifestPath: manifest, crateName: facts.crateName, dependencies: facts.dependencies }, ...facts.targetPaths.map((target) => posix.normalize(posix.join(posix.dirname(manifest), target))).filter((path) => path !== entry && !path.startsWith("../")).map((path) => ({ path, manifestPath: manifest, crateName: null, dependencies: facts.dependencies }))];
  });
  const manifestOwns = (manifest: string, path: string): boolean => posix.dirname(manifest) === "." || path.startsWith(posix.dirname(manifest) + "/");
  const conventionalRoots = options.conventionalRoots ? [...sourceFiles].filter((path) => /(?:^|\/)(?:lib|main)\.rs$/u.test(path) && !manifestRoots.some((root) => root.path === path) && ![...invalidManifests].some((manifest) => manifestOwns(manifest, path))).map((path) => ({ path, manifestPath: null, crateName: null, dependencies: [] as string[] })) : [];
  for (const root of [...manifestRoots, ...conventionalRoots].sort((left, right) => compare(left.path, right.path))) {
    const crateRoot = root.path, pending: { sourcePath: string; context: RustModuleContext }[] = [], seen = new Set<string>();
    if (root.crateName) namedCrates.set(root.crateName.replaceAll("-", "_"), [...(namedCrates.get(root.crateName.replaceAll("-", "_")) ?? []), crateRoot]);
    dependencies.set(crateRoot, root.dependencies);
    const observe = (path: string, context: RustModuleContext, reason?: RustModuleParticipationReason): void => {
      const identity = path + "\0" + JSON.stringify(context);
      if (seen.has(identity)) return;
      seen.add(identity);
      const localReason = reason ?? (!sourceFiles.has(path) || read(path) === undefined ? "unavailable-source" : rustModuleScopeProof(sourceFacts(path), context.sourceScope).state === "unresolved" || path !== crateRoot && sourceFacts(path).scopes.some((scope) => scope.crateCapabilities?.length) ? "unresolved-scope" : undefined);
```

## 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts

Whole-source SHA-256 `975a1727a386779fb16fc77e99b6ca97ac0c6c6af7fa1ca29bdeefc680a769b8`.

Lines 579–598:

```ts
    if (segments[0] === "transaction-fixture-key-exactness") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗝️transaction-fixture-key-exactness/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "testing-readme-coordinates") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "rust-finite-target-consumption") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "readme-current-source-revision") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🟦️.ts");
      await runRepositoryTestCommand(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
```
