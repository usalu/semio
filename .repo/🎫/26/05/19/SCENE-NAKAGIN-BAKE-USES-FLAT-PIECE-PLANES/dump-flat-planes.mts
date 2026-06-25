#!/usr/bin/env bun
/** @emoji 🧾 Emits `elements/.../nakagin-capsule-tower.flat-planes.v1.json` via `@compose/js` flatten only (no algorithms UI barrel). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { openSession, type Store as JsStore } from "../../../../../../compose/client/lib/js/index.ts";

type GqlWireObject = { readonly [k: string]: unknown };

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const kitPath = join(repoRoot, "assets/fixtures/metabolism.shallow.kit.compose.json");
const nakaginShallowDesignPath = join(repoRoot, "assets/fixtures/nakagin-capsule-tower.shallow.design.compose.json");
const outPath = join(repoRoot, "elements/client/lib/scene/fixtures/nakagin-capsule-tower.flat-planes.v1.json");
const nakaginDesignId = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

function toBootstrap(kit: unknown): GqlWireObject {
	return JSON.parse(JSON.stringify(kit)) as GqlWireObject;
}

async function withJsStore<T>(kit: unknown, fn: (store: JsStore) => Promise<T>): Promise<T> {
	const session = await openSession(JSON.stringify(toBootstrap(kit)));
	try {
		const stores = await session.stores();
		const store = stores[0];
		if (!store) throw new Error("withJsStore: session has no stores");
		return await fn(store);
	} finally {
		await session.dispose();
	}
}

async function readFlattenPlanesByPieceId(
	store: JsStore,
	designId: string,
): Promise<Record<string, unknown>> {
	const flat = await store.design(designId).flatten();
	if (!flat.ok) throw new Error(flat.error.message);
	const sel = `design(id: ${JSON.stringify(designId)}) { pieces { edges { node { id flatPosition { plane { origin { x y z } xAxis { x y z } yAxis { x y z } } } } } } }`;
	const frag = (await store.readKitInner(sel)) as GqlWireObject | null;
	const design = frag?.["design"] as GqlWireObject | undefined;
	const pieces = design?.["pieces"] as GqlWireObject | undefined;
	const edges = (pieces?.["edges"] as readonly GqlWireObject[] | undefined) ?? [];
	const byPieceId: Record<string, unknown> = {};
	for (const e of edges) {
		const n = e["node"] as GqlWireObject | undefined;
		if (!n) continue;
		const id = String(n["id"] ?? "");
		const fp = n["flatPosition"] as GqlWireObject | undefined;
		const plane = fp?.["plane"];
		if (!id || plane == null || typeof plane !== "object") continue;
		const p = plane as { origin?: unknown; xAxis?: unknown; yAxis?: unknown };
		if (p.origin && p.xAxis && p.yAxis) byPieceId[id] = plane;
	}
	return byPieceId;
}

const kit = JSON.parse(readFileSync(kitPath, "utf8")) as Record<string, unknown>;
const nakShallow = JSON.parse(readFileSync(nakaginShallowDesignPath, "utf8")) as Record<string, unknown>;
const designs = [...((kit.designs as Record<string, unknown>[]) ?? [])];
const di = designs.findIndex((d) => String(d["id"] ?? "") === nakaginDesignId);
if (di < 0) throw new Error("nakagin design missing from shallow kit");
designs[di] = {
	...designs[di],
	pieces: nakShallow["pieces"] ?? [],
	connections: nakShallow["connections"] ?? [],
};
kit.designs = designs;
const byPieceId = await withJsStore(kit, (s) => readFlattenPlanesByPieceId(s, nakaginDesignId));
const doc = {
	schema: "elements.scene.flat-planes/v1",
	designId: nakaginDesignId,
	byPieceId,
};
writeFileSync(outPath, JSON.stringify(doc, null, 2));
console.log(`[dump-flat-planes] wrote ${outPath} (${Object.keys(byPieceId).length} planes)`);
