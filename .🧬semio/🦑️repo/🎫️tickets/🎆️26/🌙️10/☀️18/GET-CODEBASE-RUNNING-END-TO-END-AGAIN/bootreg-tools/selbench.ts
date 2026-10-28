/** 🧮 Times the synchronous pre-selection steps of runRepositoryCargoTests on a root (read-only): package inventory, workspace scope, preparation request + receipt witness. `bun selbench.ts <root>` */
const root = process.argv[2]!;
const L = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const t0 = performance.now();
const ws = await import(L + "/🗂️workspaces/🦀️cargo/🟦️.ts");
const prep = await import(L + "/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts");
const t1 = performance.now();
console.log(`[DEBUG] import ${(t1 - t0).toFixed(0)} ms`);
for (let i = 0; i < 2; i++) {
  const a = performance.now();
  const disc = ws.discoverCargoWorkspaces ? ws.discoverCargoWorkspaces(root) : [];
  const b = performance.now();
  const rows = ws.cargoRepositoryPackageSelections(root, ["semio-framework-print-tests"]);
  const c = performance.now();
  const scope = ws.cargoWorkspaceForManifest(root, rows[0].manifest);
  const d = performance.now();
  const storage = prep.repositoryCargoPreparationStorageV1(root);
  const req = ws.cargoWorkspacePreparationInvocationV1(storage, root, ["test", "--manifest-path", root + "/" + rows[0].manifest, "-p", rows[0].name], root + "/🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust", { ...process.env });
  const e = performance.now();
  const current = req ? ws.cargoPreparationInvocationCurrentV1(storage, root, req) : "none";
  const f = performance.now();
  console.log(`[DEBUG] run ${i}: discover ${(b - a).toFixed(0)} ms (${disc.length} workspaces) selections ${(c - b).toFixed(0)} ms (${rows[0].manifest}) scope ${(d - c).toFixed(0)} ms (${scope.directory}) request ${(e - d).toFixed(0)} ms witness ${(f - e).toFixed(0)} ms current=${current}`);
}
