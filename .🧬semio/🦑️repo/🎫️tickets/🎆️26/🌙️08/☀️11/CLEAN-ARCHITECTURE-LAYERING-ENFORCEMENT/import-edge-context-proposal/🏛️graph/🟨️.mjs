/** 🧬️ Identifies current dependency resolution authority using only owned immutable source facts.
 * @param {string} workspaceRoot
 * @param {Record<string, { root: string }>} projects
 * @returns {string}
 */
function dependencyResolutionAuthorityImplementation(workspaceRoot, projects) {
  const hash = createHash("sha256").update(implementationRevision());
  const source = (path) => { hash.update(JSON.stringify(nxPath(relative(workspaceRoot, path)))); hash.update(existsSync(path) ? readPhysicalSource(path) : "\0absent\0"); };
  for (const path of ["nx.json", "Cargo.toml", "go.work", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs"]) source(join(workspaceRoot, path));
  if (existsSync(join(workspaceRoot, "bun.lock"))) { readBunLockGraph(workspaceRoot); hash.update(BUN_LOCK_CACHE.hash); }
  else hash.update("\0no-bun-lock\0");
  for (const [name, project] of Object.entries(projects).sort(([a], [b]) => a.localeCompare(b))) {
    hash.update(JSON.stringify([name, nxPath(project.root)]));
    for (const filename of ["package.json", "📋️project.json", "Cargo.toml"]) source(join(workspaceRoot, project.root, filename));
    const go = goManifest(project.root, workspaceRoot);
    if (go) source(go);
  }
  return hash.digest("hex");
}

/** 🏛️ Supplies current roots, manifests, lock scope, and actual implementation identity behind an owned digest interface. */
export function dependencyResolutionAuthority(...args) { return invokeCurrentImplementation("authority", args); }
