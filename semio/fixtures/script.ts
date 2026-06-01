#!/usr/bin/env bun
/** @emoji 🧪 `@semio/fixtures` router: `bun ./script.ts regenerate-metabolism-light`. */
import fs from "node:fs";
import path from "node:path";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBun, runBundleScriptMain } from "../../repo/lib/js/src/index.ts";

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
		if (typeof type.id === "string") type.nodeKind = `semio.metabolism.light.node.${type.id}`;
		for (const connector of (type.connectors ?? []) as Record<string, unknown>[]) {
			const port = connector.port;
			if (port != null && typeof port === "object" && !Array.isArray(port)) {
				const portRow = port as Record<string, unknown>;
				if (typeof portRow.id === "string") portRow.handleKind = `semio.metabolism.light.handle.${portRow.id}`;
			}
		}
	}
	for (const family of (kit.families ?? []) as Record<string, unknown>[]) {
		for (const port of (family.ports ?? []) as Record<string, unknown>[]) {
			if (typeof port.id === "string") port.handleKind = `semio.metabolism.light.handle.${port.id}`;
		}
	}
}

class RegenerateMetabolismLightScript extends BundleScript {
	run(): void {
		const snapshotPath = path.join(this.root, "metabolism.kit.snapshot.semio.json");
		const outPath = path.join(this.root, "metabolism.kit.light.semio.json");
		const snapshot = JSON.parse(fs.readFileSync(snapshotPath, "utf8")) as Record<string, unknown>;
		annotateKitSemantics(snapshot);
		const initialKit = annotateValue(snapshot);
		const bundle = {
			schema: SCHEMA,
			wip: { id: KIT_ID, hash: HASH, authors: { hash: HASH, items: [] }, initialKit },
		};
		fs.writeFileSync(outPath, `${JSON.stringify(bundle)}\n`);
		console.log(`[semio/fixtures] wrote ${outPath}`);
	}
}

class ReconcileKindNamesScript extends BundleScript {
	run(): void {
		runBun([join(this.root, "reconcile-kind-names.ts")], this.root);
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("regenerate-metabolism-light", RegenerateMetabolismLightScript)
	.register("reconcile-kind-names", ReconcileKindNamesScript);

if (import.meta.main) {
	await runBundleScriptMain(router, import.meta.url);
}
