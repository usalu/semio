#!/usr/bin/env bun
/** @emoji 🧪 `@semio-tech/compose-fixture` router: `bun ./script.ts regenerate-metabolism-light`. */
import fs from "node:fs";
import path from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../repo/lib/js/index.ts";

//#region 🔖AssembleSplitInitialKit
/** @emoji 📎 Reads `{ hash, items }` collection blocks from kit snapshot JSON. */
export function fixtureItemsOf(node: unknown): Record<string, unknown>[] {
	if (node && typeof node === "object" && Array.isArray((node as { items?: unknown[] }).items)) {
		return (node as { items: Record<string, unknown>[] }).items;
	}
	return [];
}

/** @emoji 🧩 Merges `types/*.type.compose.json` and `designs/*.design.compose.json` into a split `kit.compose.json` shell. */
export function assembleSplitInitialKitFromDirectory(initialKitDir: string): Record<string, unknown> {
	const shellPath = path.join(initialKitDir, "kit.compose.json");
	const kit = JSON.parse(fs.readFileSync(shellPath, "utf8")) as Record<string, unknown>;
	const typeById = new Map<string, Record<string, unknown>>();
	const designById = new Map<string, Record<string, unknown>>();
	const typesDir = fs.existsSync(path.join(initialKitDir, "type"))
		? path.join(initialKitDir, "type")
		: path.join(initialKitDir, "types");
	const designsDir = fs.existsSync(path.join(initialKitDir, "design"))
		? path.join(initialKitDir, "design")
		: path.join(initialKitDir, "designs");
	if (fs.existsSync(typesDir)) {
		for (const name of fs.readdirSync(typesDir)) {
			if (!name.endsWith(".type.compose.json")) continue;
			const row = JSON.parse(fs.readFileSync(path.join(typesDir, name), "utf8")) as Record<string, unknown>;
			if (typeof row.id === "string") typeById.set(row.id, row);
		}
	}
	if (fs.existsSync(designsDir)) {
		for (const name of fs.readdirSync(designsDir)) {
			if (!name.endsWith(".design.compose.json")) continue;
			const row = JSON.parse(fs.readFileSync(path.join(designsDir, name), "utf8")) as Record<string, unknown>;
			if (typeof row.id === "string") designById.set(row.id, row);
		}
	}
	const typologies = fixtureItemsOf(kit.typologies);
	for (const topo of typologies) {
		const mergedTypes = fixtureItemsOf(topo.types).map((stub) => {
			const id = String(stub.id ?? "");
			return typeById.get(id) ?? stub;
		});
		if (mergedTypes.length > 0) {
			const hash =
				topo.types && typeof topo.types === "object" && !Array.isArray(topo.types) && typeof (topo.types as { hash?: string }).hash === "string"
					? (topo.types as { hash: string }).hash
					: "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
			topo.types = { hash, items: mergedTypes };
		}
		const mergedDesigns = fixtureItemsOf(topo.designs).map((stub) => {
			const id = String(stub.id ?? "");
			return designById.get(id) ?? stub;
		});
		if (mergedDesigns.length > 0) {
			const hash =
				topo.designs && typeof topo.designs === "object" && !Array.isArray(topo.designs) && typeof (topo.designs as { hash?: string }).hash === "string"
					? (topo.designs as { hash: string }).hash
					: "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
			topo.designs = { hash, items: mergedDesigns };
		}
	}
	if (typologies.length > 0) {
		const hash =
			kit.typologies && typeof kit.typologies === "object" && !Array.isArray(kit.typologies) && typeof (kit.typologies as { hash?: string }).hash === "string"
				? (kit.typologies as { hash: string }).hash
				: "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";
		kit.typologies = { hash, items: typologies };
	}
	return kit;
}

/** @emoji 📦 Reads split or monolithic `kit.compose.json` (assembles sidecars when `types/` exists). */
export function readInitialKitFixtureFromPath(kitJsonPath: string): Record<string, unknown> {
	const initialKitDir = path.dirname(kitJsonPath);
	if (
		fs.existsSync(path.join(initialKitDir, "type")) ||
		fs.existsSync(path.join(initialKitDir, "types"))
	) {
		return assembleSplitInitialKitFromDirectory(initialKitDir);
	}
	return JSON.parse(fs.readFileSync(kitJsonPath, "utf8")) as Record<string, unknown>;
}
//#endregion 🔖AssembleSplitInitialKit

const HASH = "…";
const SCHEMA = "🎆26🌙06⬆️1";
const KIT_ID = "f042c2a4-3ba5-44b0-b22c-0ae8f568aacc";

/** @param {unknown} value */
function wrapCollection(value: unknown): unknown {
	if (Array.isArray(value)) {
		return { hash: HASH, items: (value as unknown[]).map((entry) => annotateValue(entry)) };
	}
	return annotateValue(value);
}

/** @param {unknown} value */
function annotateValue(value: unknown): unknown {
	if (value == null || typeof value !== "object") return value;
	if (Array.isArray(value)) return wrapCollection(value);
	const row = { ...(value as Record<string, unknown>) };
	if (typeof row.id === "string") row.hash = HASH;
	for (const [key, child] of Object.entries(row)) {
		if (Array.isArray(child)) row[key] = wrapCollection(child);
		else if (child != null && typeof child === "object") row[key] = annotateValue(child);
	}
	return row;
}

/** @param {Record<string, unknown>} kit */
function annotateKitSemantics(kit: Record<string, unknown>): void {
	for (const type of (kit.types ?? []) as Record<string, unknown>[]) {
		if (typeof type.id === "string") type.nodeKind = `compose.metabolism.light.node.${type.id}`;
		for (const connector of (type.connectors ?? []) as Record<string, unknown>[]) {
			const port = connector.port;
			if (port != null && typeof port === "object" && !Array.isArray(port)) {
				const portRow = port as Record<string, unknown>;
				if (typeof portRow.id === "string") portRow.handleKind = `compose.metabolism.light.handle.${portRow.id}`;
			}
		}
	}
	for (const family of (kit.families ?? []) as Record<string, unknown>[]) {
		for (const port of (family.ports ?? []) as Record<string, unknown>[]) {
			if (typeof port.id === "string") port.handleKind = `compose.metabolism.light.handle.${port.id}`;
		}
	}
}

class RegenerateMetabolismLightScript extends BundleScript {
	run(): void {
		const snapshotPath = path.join(this.root, "metabolism.kit.snapshot.compose.json");
		const outPath = path.join(this.root, "metabolism.kit.light.compose.json");
		const snapshot = JSON.parse(fs.readFileSync(snapshotPath, "utf8")) as Record<string, unknown>;
		annotateKitSemantics(snapshot);
		const initialKit = annotateValue(snapshot);
		const bundle = {
			schema: SCHEMA,
			wip: { id: KIT_ID, hash: HASH, authors: { hash: HASH, items: [] }, initialKit },
		};
		fs.writeFileSync(outPath, `${JSON.stringify(bundle)}\n`);
		console.log(`[compose/fixture] wrote ${outPath}`);
	}
}

if (import.meta.main) {
	const router = new ScriptRouter(import.meta.dir).register("regenerate-metabolism-light", RegenerateMetabolismLightScript);
	await runBundleScriptMain(router, import.meta.url);
}
