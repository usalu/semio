/** 🗂️ Current Nx workspace-data location for immutable source facts. */
function workspaceDataDirectory(workspaceRoot) {
  return process.env.NX_WORKSPACE_DATA_DIRECTORY || join(workspaceRoot, ".nx", "workspace-data");
}

/** 🧬️ Keeps parser revisions and source identities apart from resolved project authority. */
function moduleSourceFactCacheRoot(workspaceRoot) {
  return join(workspaceDataDirectory(workspaceRoot), "emoji-module-source-facts", commandInputHash);
}

const MODULE_FACT_BYTES = 1024 * 1024;

/** 🔑️ Identifies immutable content at one normalized source path under the actual parser revision. */
function moduleSourceFactIdentity(file, hash) {
  const identity = { parser: commandInputHash, file: nxPath(file), hash: String(hash) };
  return { ...identity, key: createHash("sha256").update(JSON.stringify(identity)).digest("hex") };
}

/** 📥️ Admits a bounded closed source fact record without granting any project target authority. */
function readCachedModuleImports(cacheRoot, identity) {
  try {
    const path = join(cacheRoot, `${identity.key}.json`);
    if (statSync(path).size > MODULE_FACT_BYTES) return undefined;
    const value = JSON.parse(readFileSync(path, "utf8"));
    if (!value || Object.keys(value).length !== 4 || value.parser !== identity.parser || value.file !== identity.file || value.hash !== identity.hash || !Array.isArray(value.imports) || value.imports.length > 16384 || !value.imports.every(specifier => typeof specifier === "string")) return undefined;
    return value.imports;
  } catch {
    return undefined;
  }
}

/** 📤️ Persists only bounded source facts; partial cache reads simply cause a fresh parse. */
function writeCachedModuleImports(cacheRoot, identity, imports) {
  if (imports.length > 16384) return;
  const value = JSON.stringify({ parser: identity.parser, file: identity.file, hash: identity.hash, imports });
  if (Buffer.byteLength(value) > MODULE_FACT_BYTES) return;
  mkdirSync(cacheRoot, { recursive: true });
  writeFileSync(join(cacheRoot, `${identity.key}.json`), value);
}

/** 🏛️ Snapshots current project roots once, in deterministic ownership precedence. */
function importProjectOwners(projects) {
  return Object.entries(projects).filter(([name]) => name !== "workspace").map(([name, project]) => ({ name, root: nxPath(project.root) })).sort((a, b) => b.root.length - a.root.length || a.name.localeCompare(b.name));
}

/** 🔗️ Resolves immutable runtime import strings against current ownership, package names, and lock scope. */
function importTargetsFromFacts(imports, file, workspaceRoot, owners, byPackage, locked) {
  const targets = new Set();
  for (const specifier of imports) {
    if (specifier.startsWith(".")) {
      const path = nxPath(relative(workspaceRoot, resolve(dirname(join(workspaceRoot, file)), specifier)));
      const target = owners.find(project => owned(path, project.root))?.name;
      if (target) targets.add(target);
      continue;
    }
    const packageName = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
    const workspaceTarget = byPackage.get(packageName);
    if (workspaceTarget) { targets.add(workspaceTarget); continue; }
    if (isBuiltin(packageName)) continue;
    const key = locked?.resolveImport?.(file, packageName);
    if (key && locked?.externalNodes?.[`npm:${key}`]) targets.add(`npm:${key}`);
  }
  return [...targets].sort();
}

/** 🔍️ Parses one module through the canonical owned string interface and current project authority. */
function importTargetsFromSource(text, file, workspaceRoot, projects, byPackage, locked) {
  return importTargetsFromFacts(moduleSourceImports(file, text), nxPath(file), workspaceRoot, importProjectOwners(projects), byPackage, locked);
}

/** ⚡️ Reuses bounded source facts while resolving every full or incremental file against current authority. */
async function collectImportEdges(workspaceRoot, projectFiles, projects, byPackage, locked, add) {
  const cacheRoot = moduleSourceFactCacheRoot(workspaceRoot), owners = importProjectOwners(projects), jobs = [];
  for (const [name, files] of Object.entries(projectFiles)) {
    if (name === "workspace") continue;
    for (const file of files) if (/\.[cm]?[jt]sx?$/.test(file.file)) jobs.push({ name, file });
  }
  let cursor = 0;
  await Promise.all(Array.from({ length: Math.min(32, jobs.length) }, async () => {
    for (;;) {
      const index = cursor++;
      if (index >= jobs.length) return;
      const { name, file } = jobs[index], sourceFile = nxPath(file.file);
      let identity = file.hash ? moduleSourceFactIdentity(sourceFile, file.hash) : undefined;
      let imports = identity ? readCachedModuleImports(cacheRoot, identity) : undefined;
      if (!imports) {
        let text;
        try { text = await readFile(join(workspaceRoot, sourceFile), "utf8"); }
        catch (error) { if (error.code === "ENOENT") continue; throw error; }
        identity ??= moduleSourceFactIdentity(sourceFile, createHash("sha256").update(text).digest("hex"));
        imports = moduleSourceImports(sourceFile, text);
        writeCachedModuleImports(cacheRoot, identity, imports);
      }
      for (const target of importTargetsFromFacts(imports, sourceFile, workspaceRoot, owners, byPackage, locked)) add(name, target, sourceFile);
    }
  }));
}

