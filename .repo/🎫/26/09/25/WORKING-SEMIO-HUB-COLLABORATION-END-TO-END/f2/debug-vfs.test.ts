import { it } from "vitest";
import { buildSketchpadPlatform, getSketchpadShellController } from "/home/user/semio/semio/client/lib/sketchpad/js/index.ts";
import { buildVirtualFileSystemModelRows } from "@framework/platform/core";
it("debug vfs", async () => {
	await buildSketchpadPlatform();
	const shell = getSketchpadShellController()!;
	const kitId = await shell.createTemporaryKit("Dbg");
	const store = shell.getKitStore(kitId)!;
	await store.client!.execute((kit) => kit.createDesign("Tower"));
	const designId = store.getSnapshot().kit.designs![0]!.id;
	console.log("[DEBUG] kit", kitId, "design", designId);
	shell.navigateTo(`/kits/${kitId}/designs/${designId}`);
	await new Promise((r) => setTimeout(r, 2000));
	const anyShell = shell as unknown as { childrenByScope: Map<string, { getSnapshot(): Record<string, { id: string }[]> }>; expandedStore(scope: unknown): { getSnapshot(): string[] } };
	for (const [scope, children] of anyShell.childrenByScope) {
		const snap = children.getSnapshot();
		for (const [parent, rows] of Object.entries(snap)) console.log("[DEBUG] scope", scope, "parent", parent, "->", rows.map((r) => r.id).join(","));
	}
}, 60_000);
