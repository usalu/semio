/**
 * ☠️ Any `.rs` under `📚️examples` must be reachable via `#[path]` from a `📦️glue.rs` — dead definition
 * or test shims are forbidden.
 */
function policyCollectGluePathTargets(glueAbs: string): Set<string> {
  const declared = new Set<string>();
  if (!existsSync(glueAbs)) return declared;
  const libDir = dirname(glueAbs);
  const libText = readFileSync(glueAbs, "utf8");
  const baseStack: string[] = [libDir];
  let pendingPath: string | null = null;
  for (const rawLine of libText.split(/\r?\n/)) {
    const line = rawLine.trim();
    const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/);
    if (pathMatch) {
      pendingPath = pathMatch[1] ?? null;
      continue;
    }
    const modMatch = line.match(/^(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)/);
    if (modMatch) {
      const modName = modMatch[1]!;
      const base = baseStack[baseStack.length - 1] ?? libDir;
      let resolved: string;
      if (pendingPath === null) {
        resolved = join(base, modName);
      } else if (pendingPath === ".") {
        resolved = base;
      } else {
        resolved = join(base, pendingPath);
      }
      pendingPath = null;
      const asFile = resolved.endsWith(".rs") ? resolved : `${resolved}.rs`;
      const asModFile = join(resolved, "mod.rs");
      if (existsSync(asFile)) declared.add(resolve(asFile));
      else if (existsSync(asModFile)) declared.add(resolve(asModFile));
      else declared.add(resolve(asFile));
      if (line.includes("{")) baseStack.push(resolved.endsWith(".rs") ? dirname(resolved) : resolved);
      continue;
    }
    pendingPath = null;
    const opens = (line.match(/\{/g) ?? []).length;
    const closes = (line.match(/\}/g) ?? []).length;
    for (let i = 0; i < opens; i++) baseStack.push(baseStack[baseStack.length - 1] ?? libDir);
    for (let i = 0; i < closes; i++) {
      if (baseStack.length > 1) baseStack.pop();
    }
  }
  return declared;
}

function policyDeadExampleLeafBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const reachable = new Set<string>();
  const owners = [...new Set(crates.filter((crate) => crate.shape === "taxonomy").map((crate) => crate.ownerRel).filter(Boolean))];
  const glueRoots = owners.length > 0 ? owners : ["✏️s/🔌️plugins"];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    for (const target of policyCollectGluePathTargets(join(repoRoot, crate.libRelPath))) reachable.add(target);
  }
  for (const glueRel of policyWalkRelFiles(repoRoot, glueRoots, (_p, name) => name === "📦️glue.rs")) {
    for (const target of policyCollectGluePathTargets(join(repoRoot, glueRel))) reachable.add(target);
  }
  const fw = readdirSync(repoRoot).find((name) => name.endsWith("framework"));
  const exampleRoots = owners.length > 0 ? owners : fw ? ["✏️s/🔌️plugins", fw] : ["✏️s/🔌️plugins"];
  const exampleRs = policyWalkRelFiles(repoRoot, exampleRoots, (relPath, name) => {
    if (!name.endsWith(".rs")) return false;
    return relPath.replaceAll("\\", "/").includes("/📚️examples/");
  });
  for (const relPath of exampleRs) {
    const abs = resolve(join(repoRoot, relPath));
    if (reachable.has(abs)) continue;
    breaches.push({
      id: `dead-example-leaf-${relPath}`,
      summary: `"${relPath}" is not reachable via #[path] from any 📦️glue.rs`,
      kind: "taxonomy/dead-example-leaf",
      scope: relPath,
      priority: "high",
      reason: "Every .rs under 📚️examples must be wired from the plugin 📦️glue.rs (definition leaf or cfg(test) test leaf).",
      solution: `Add a #[path] mod declaration for ${relPath} under //#region 📚️Examples in the owning 📦️glue.rs, or delete the dead file.`,
    });
  }
  return breaches;
}

