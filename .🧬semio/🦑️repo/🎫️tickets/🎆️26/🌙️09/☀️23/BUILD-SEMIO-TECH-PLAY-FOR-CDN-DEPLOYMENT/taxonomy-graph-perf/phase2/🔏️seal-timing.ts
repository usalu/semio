/** 🔏️ Times one graph seal check (and optionally one seal write) against a workspace data directory. argv: <root> <workspaceData> */
const [root, workspaceData] = process.argv.slice(2);
const seal = await import(`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🔏️seal/🟦️.ts`);
for (let round = 0; round < 3; round++) {
  const start = performance.now(), holds = await seal.nxGraphSealHolds(root, workspaceData);
  console.log(`[DEBUG] seal holds=${holds} ${(performance.now() - start).toFixed(0)}ms`);
}
