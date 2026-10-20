
/** 🔭️ The observed directory reader declared globs and TypeScript projects list through. */
function declaredDirectoryV1(directory) {
  try { return observedFs.readdirSync(directory, { withFileTypes: true }).map(entry => ({ name: entry.name, directory: entry.isDirectory(), file: entry.isFile() })); }
  catch { return undefined; }
}

/** 🌿️ The module files the target's `metadata.semio.closure.modules` globs match, which join the target's command closure as entries. */
function declaredModuleGlobEntriesV1(globs, workspaceRoot, projectRoot, cache) {
  if (!Array.isArray(globs) || !globs.length) return [];
  if (!globalThis.Bun?.Glob) throw Error("Declared module globs require Bun.Glob");
  return memoizedObservedComputationV1(cache.declared, memoDigestV1("module-globs", workspaceRoot, projectRoot, globs), () => declaredModuleGlobsV1(globs, workspaceRoot, projectRoot, { readdir: declaredDirectoryV1, Glob: globalThis.Bun.Glob, join }).map(path => resolve(workspaceRoot, path)));
}

/** 🪁️ Joins the compiler's own program for each TypeScript project in `metadata.semio.closure.typescript` to the closure, records the generator targets whose outputs it reads and names the installed packages it resolved. */
function declaredProjectClosureV1(projects, workspaceRoot, projectRoot, cache, closure) {
  if (!Array.isArray(projects) || !projects.length) return undefined;
  const project = memoizedObservedComputationV1(cache.declared, memoDigestV1("typescript-project", workspaceRoot, projectRoot, projects), () => {
    const compiler = createRequire(import.meta.url)("typescript");
    return declaredTypeScriptProjectsV1(projects, workspaceRoot, projectRoot, { compiler, host: observedTypeScriptHost(compiler.sys), readdir: declaredDirectoryV1, join, dirname, isAbsolute });
  });
  if (!project.files.length) return undefined;
  const config = resolve(workspaceRoot, project.configs[0]), names = [];
  for (const file of project.files) {
    const produced = cache.generatedOutput?.(file);
    if (produced) { const rows = cache.generatedImports.get(config) ?? []; if (!rows.some(row => row.target === produced.target && row.output === produced.output)) rows.push(produced); cache.generatedImports.set(config, rows); continue; }
    readPhysicalSource(file);
    names.push(closurePathNameV1(cache, workspaceRoot, file));
  }
  return { closure: [...new Set([...closure, ...names])].sort(), packages: project.packages, config: closurePathNameV1(cache, workspaceRoot, config) };
}

