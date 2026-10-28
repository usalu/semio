/** 📏 Read-only cost of the parent-side roster recheck: workspace scope + members for a manifest (bun roster-cost.ts <root> <manifest-rel> [package...]). */
const [root, manifest, ...packages] = process.argv.slice(2);
const cargo = await import(root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts");
for (let round = 0; round < 3; round++) {
  const t0 = performance.now(), scope = cargo.cargoWorkspaceForManifest(root, manifest), t1 = performance.now(), members = cargo.cargoWorkspaceMembers(root, scope), t2 = performance.now();
  console.log(JSON.stringify({ round, scope: scope.manifest, members: members.length, scopeMs: +(t1 - t0).toFixed(1), membersMs: +(t2 - t1).toFixed(1), rssMB: Math.round(process.memoryUsage().rss / 1048576) }));
}
