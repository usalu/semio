import { SemioKitClient } from "/home/user/semio/semio/client/lib/react/index.ts";
const file = process.argv[2] ?? "/home/user/semio/semio/fixtures/nakagin-capsule-tower.filtered.kit.semio.json";
const t0 = Date.now();
const client = await SemioKitClient.open({ kind: "bytes", data: new Uint8Array(await Bun.file(file).arrayBuffer()) });
console.log("[DEBUG] open", Date.now() - t0, "ms", client.kitId);
for (const design of client.getSnapshot().designs) {
  const t1 = Date.now();
  const refs = await Promise.race([client.designReferences(design.id), new Promise((r) => setTimeout(() => r("TIMEOUT"), 90000))]);
  console.log("[DEBUG] refs", design.name, design.pieces.length, Date.now() - t1, "ms", JSON.stringify(refs).slice(0, 200));
}
await client.dispose();
