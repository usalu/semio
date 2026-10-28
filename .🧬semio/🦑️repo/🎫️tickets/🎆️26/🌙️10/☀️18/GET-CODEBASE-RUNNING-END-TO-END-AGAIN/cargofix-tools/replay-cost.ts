/** ⏱️ Read-only cost of recording vs replaying the physical answers of a workspace roster (bun replay-cost.ts <code-root> <data-root> <manifest-rel>). */
const [code, root, manifest] = process.argv.slice(2);
const cargo = await import(code + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts");
const witness = await import(code + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/🟦️.ts");
const observations = new Map(), t0 = performance.now(), scope = cargo.cargoWorkspaceForManifest(root, manifest, observations), members = cargo.cargoWorkspaceMembers(root, scope, observations), t1 = performance.now(), rows = [...observations.values()];
const counts: Record<string, number> = {}; for (const row of rows) counts[row.operation] = (counts[row.operation] ?? 0) + 1;
console.log(JSON.stringify({ record: { ms: +(t1 - t0).toFixed(0), members: members.length, observations: rows.length, counts, bytes: Buffer.byteLength(JSON.stringify(rows)), poisoned: rows.filter((row: { stamp: string }) => !row.stamp).length } }));
for (let round = 0; round < 3; round++) { const s = performance.now(), holds = witness.cargoPhysicalObservationsHoldV1(root, rows); console.log(JSON.stringify({ replay: round, holds, ms: +(performance.now() - s).toFixed(1), rssMB: Math.round(process.memoryUsage().rss / 1048576) })); }
