import { SemioKitClient } from "/home/user/semio/semio/client/lib/react/index.ts";
const client = await SemioKitClient.open({ kind: "empty", name: "Hub Kit" });
await client.execute((kit) => kit.createType("Capsule"));
await client.execute((kit) => kit.createDesign("Tower"));
const snap = client.getSnapshot();
console.log("[DEBUG] kit", snap.id, "types", snap.types?.map((t) => t.id), "designs", snap.designs?.map((d) => d.id));
const seen = new Map<string, string>();
const walk = async (parent: any, depth: number) => {
	if (depth > 6) { console.log("[DEBUG] depth limit at", parent); return; }
	const rows = await client.fileSystemChildren(parent);
	for (const row of rows) {
		console.log("[DEBUG]", " ".repeat(depth * 2), row.kind, row.id, row.name, "hasChildren", (row as any).hasChildren);
		if (row.id === parent.id) console.log("[DEBUG] SELF CYCLE", row.id);
		if ((row as any).hasChildren) await walk({ kind: row.kind, id: row.id }, depth + 1);
	}
};
await walk({ kind: "KIT", id: snap.id }, 0);
await client.dispose();
