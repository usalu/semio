#!/usr/bin/env bun
/** @emoji 🧪 One-off: split metabolism initialKit monolith into kit shell + types/ + designs/. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const srcRoot = join(repoRoot, "compose/fixtures/kit/dev/metabolism/wip/initialKit");
const dstRoot = join(repoRoot, "compose/fixtures/stores/metabolism/wip/initialKit");

const HASH_STUB = "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262";

function itemsOf(node: unknown): Record<string, unknown>[] {
	if (Array.isArray(node)) return node as Record<string, unknown>[];
	if (node && typeof node === "object" && Array.isArray((node as { items?: unknown[] }).items)) {
		return (node as { items: Record<string, unknown>[] }).items;
	}
	return [];
}

function wrapItems(items: Record<string, unknown>[]): { hash: string; items: Record<string, unknown>[] } {
	return { hash: HASH_STUB, items };
}

function slugify(name: string): string {
	return name
		.trim()
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "");
}

function typeFileName(type: Record<string, unknown>, used: Set<string>): string {
	const base = slugify(String(type.name ?? type.id ?? "type"));
	let candidate = `${base}.type.compose.json`;
	let n = 2;
	while (used.has(candidate)) {
		candidate = `${base}-${n}.type.compose.json`;
		n += 1;
	}
	used.add(candidate);
	return candidate;
}

function designFileName(design: Record<string, unknown>, used: Set<string>): string {
	const base = slugify(String(design.name ?? design.id ?? "design"));
	let candidate = `${base}.design.compose.json`;
	let n = 2;
	while (used.has(candidate)) {
		candidate = `${base}-${n}.design.compose.json`;
		n += 1;
	}
	used.add(candidate);
	return candidate;
}

const TYPE_STUB_KEYS = new Set([
	"id",
	"name",
	"hash",
	"isAbstract",
	"createdAt",
	"updatedAt",
	"typology",
	"families",
	"parent",
]);

const DESIGN_STUB_KEYS = new Set([
	"id",
	"name",
	"hash",
	"parent",
	"createdAt",
	"updatedAt",
	"typology",
	"families",
	"isAbstract",
]);

function shallowEntity(row: Record<string, unknown>, keys: Set<string>): Record<string, unknown> {
	const out: Record<string, unknown> = {};
	for (const key of keys) {
		if (key in row) out[key] = row[key];
	}
	if (typeof out.id === "string" && !("hash" in out)) out.hash = HASH_STUB;
	return out;
}

const kit = JSON.parse(readFileSync(join(srcRoot, "kit.compose.json"), "utf8")) as Record<string, unknown>;
mkdirSync(join(dstRoot, "types"), { recursive: true });
mkdirSync(join(dstRoot, "designs"), { recursive: true });

const typeUsed = new Set<string>();
const designUsed = new Set<string>();
const typeById = new Map<string, Record<string, unknown>>();
const designById = new Map<string, Record<string, unknown>>();
const typeIndex: { id: string; file: string }[] = [];
const designIndex: { id: string; file: string }[] = [];

for (const topo of itemsOf(kit.typologies)) {
	for (const type of itemsOf(topo.types)) {
		const id = String(type.id);
		if (!typeById.has(id)) {
			typeById.set(id, type);
			const file = typeFileName(type, typeUsed);
			writeFileSync(join(dstRoot, "types", file), `${JSON.stringify(type, null, 4)}\n`);
			typeIndex.push({ id, file: `types/${file}` });
		}
	}
	for (const design of itemsOf(topo.designs)) {
		const id = String(design.id);
		if (!designById.has(id)) {
			designById.set(id, design);
			const file = designFileName(design, designUsed);
			writeFileSync(join(dstRoot, "designs", file), `${JSON.stringify(design, null, 4)}\n`);
			designIndex.push({ id, file: `designs/${file}` });
		}
	}
}

const rebuiltTopologies: Record<string, unknown>[] = [];
for (const topo of itemsOf(kit.typologies)) {
	const next: Record<string, unknown> = { ...topo };
	const shallowTypes = itemsOf(topo.types).map((t) => shallowEntity(typeById.get(String(t.id)) ?? t, TYPE_STUB_KEYS));
	if (shallowTypes.length > 0) next.types = wrapItems(shallowTypes);
	else delete next.types;
	const shallowDesigns = itemsOf(topo.designs).map((d) => shallowEntity(designById.get(String(d.id)) ?? d, DESIGN_STUB_KEYS));
	if (shallowDesigns.length > 0) next.designs = wrapItems(shallowDesigns);
	else delete next.designs;
	rebuiltTopologies.push(next);
}

kit.typologies = wrapItems(rebuiltTopologies);
delete kit.types;
delete kit.designs;

writeFileSync(join(dstRoot, "kit.compose.json"), `${JSON.stringify(kit, null, 4)}\n`);
writeFileSync(join(dstRoot, "index.compose.json"), `${JSON.stringify({ types: typeIndex, designs: designIndex }, null, 2)}\n`);

const shellMiB = readFileSync(join(dstRoot, "kit.compose.json")).length / 1024 / 1024;
console.log(`[split] wrote ${dstRoot}`);
console.log(`[split] types ${typeIndex.length} designs ${designIndex.length} shell ${shellMiB.toFixed(2)} MiB`);
