import { it } from "vitest";
import { appendFileSync } from "node:fs";
const log = (...a: unknown[]) => appendFileSync("/home/user/semio/.repo/🎫/26/09/25/WORKING-SEMIO-HUB-COLLABORATION-END-TO-END/f2/debug-vfs-rebind.log", a.map(String).join(" ") + "\n");
import {
	buildSketchpadPlatform,
	getSketchpadShellController,
	SKETCHPAD_KIT_APP_ID,
	SemioKitStore,
} from "/home/user/semio/semio/client/lib/sketchpad/js/index.ts";
import type { Kit } from "@semio/react";
import { virtualFileSystemScopeKey, virtualFileSystemSurfaceId } from "@framework/platform/core";
const sketchpadVfsScope = (appId: string) => ({ appId, surfaceId: virtualFileSystemSurfaceId(appId) });

const dump = (label: string) => {
	const shell = getSketchpadShellController()! as unknown as Record<string, Map<string, unknown>>;
	const key = virtualFileSystemScopeKey(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID));
	const children = shell.childrenByScope.get(key) as { getSnapshot(): Record<string, { id: string }[]> } | undefined;
	log("[DEBUG]", label, "root", shell.vfsRouteRootByScope.get(key), "expanded", JSON.stringify((shell as unknown as { expandedStore(s: unknown): { getSnapshot(): string[] } }).expandedStore(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID)).getSnapshot()), "children", JSON.stringify(children ? Object.fromEntries(Object.entries(children.getSnapshot()).map(([k, v]) => [k, v.map((r) => r.id)])) : null), "pending", JSON.stringify([...((shell.pendingChildrenLoadsByScope.get(key) as Set<string> | undefined) ?? [])]));
};

it("first", async () => {
	const kitId = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
	const platform = await buildSketchpadPlatform();
	const ctrl = getSketchpadShellController()!;
	ctrl.registerKitStore(kitId, SemioKitStore.fromSnapshot({ id: kitId, name: "VFS Kit", types: [{ id: "11111111-2222-3333-4444-555555555555", name: "Base" }] } as Kit));
	ctrl.navigateTo(`/kits/${kitId}`);
	platform.uri = `/kits/${kitId}`;
	const vfs = { buildSnapshot: () => { ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), ctrl.getStore<{ navigationPath: string }>("shell") ? (platform.uri.split("/")[2] ?? "") : (platform.uri.split("/")[2] ?? "")); return ctrl.buildVirtualFileSystemModel(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID)); } };
	await new Promise<void>((resolve) => setTimeout(resolve, 0));
	dump("first");
	log("[DEBUG] first rows", vfs.buildSnapshot().rows.map((r) => r.id).join(","));
	ctrl.dispose();
});

it("second", async () => {
	const kitA = "aaaaaaaa-bbbb-cccc-dddd-111111111111";
	const typeA = "11111111-2222-3333-4444-aaaaaaaaaaaa";
	const platform = await buildSketchpadPlatform();
	const ctrl = getSketchpadShellController()!;
	dump("second-start");
	ctrl.registerKitStore(kitA, SemioKitStore.fromSnapshot({ id: kitA, name: "Kit A", types: [{ id: typeA, name: "Type A" }] } as Kit));
	ctrl.navigateTo(`/kits/${kitA}`);
	platform.uri = `/kits/${kitA}`;
	dump("second-nav");
	const vfs = { buildSnapshot: () => { ctrl.syncVirtualFileSystemRoute(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID), ctrl.getStore<{ navigationPath: string }>("shell") ? (platform.uri.split("/")[2] ?? "") : (platform.uri.split("/")[2] ?? "")); return ctrl.buildVirtualFileSystemModel(sketchpadVfsScope(SKETCHPAD_KIT_APP_ID)); } };
	await new Promise<void>((resolve) => setTimeout(resolve, 0));
	dump("second-after");
	log("[DEBUG] second rows", vfs.buildSnapshot().rows.map((r) => r.id).join(","));
	await new Promise<void>((resolve) => setTimeout(resolve, 50));
	dump("second-later");
	log("[DEBUG] second rows later", vfs.buildSnapshot().rows.map((r) => r.id).join(","));
	ctrl.dispose();
});
