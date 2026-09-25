import { SemioKitClient } from "/home/user/semio/semio/client/lib/react/index.ts";
const c = await SemioKitClient.open({ kind: "bytes", data: new Uint8Array(await Bun.file(process.argv[2] ?? "/home/user/semio/semio/fixtures/metabolism.zip").arrayBuffer()) });
const kit = c.getSnapshot();
const design = kit.designs!.find((d) => d.name === "Nakagin Capsule Tower")!;
const typeIds = new Set(design.pieces!.map((p) => p.type?.id));
console.log("[DEBUG] pieces", design.pieces!.length, "types", typeIds.size, "files", kit.files!.length);
for (const tid of typeIds) {
  const t = kit.types!.find((x) => x.id === tid);
  const reps = t?.representations ?? [];
  const summary = reps.map((r) => { const f = kit.files!.find((x) => x.id === r.file?.id); return `${f?.name ?? "?"}:${c.fileUrl(r.file?.id ?? "")?.slice(0, 12) ?? "none"}`; });
  console.log("[DEBUG]", t?.name, summary.slice(0, 4).join(" | "));
}
await c.dispose();
