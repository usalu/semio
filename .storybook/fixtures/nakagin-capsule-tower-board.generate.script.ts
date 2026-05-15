// #region 🧲Header
// 💻 .storybook/fixtures/nakagin-capsule-tower-board.generate.script.ts
// Specs: Regenerate `nakagin-capsule-tower.board.json` from `metabolism.kit.semio.json` parent Nakagin design (180 pieces, 179 connections).
// Summary: Force-layout screen positions; handle ids use type connector names (`link` when unnamed); edges mirror kit `connections`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

//#region 🔖Kit
const __dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dir, "../..");
const kitPath = join(repoRoot, "semio/assets/fixtures/metabolism.kit.semio.json");
const outPath = join(__dir, "nakagin-capsule-tower.board.json");

interface KitConnector {
	id: string;
	name?: string;
}

interface KitType {
	connectors?: { items?: KitConnector[] };
	id: string;
}

interface KitPiece {
	id: string;
	name: string;
	type: { id: string };
}

interface KitConnection {
	connected: { connector: { id: string }; piece: { id: string } };
	connecting: { connector: { id: string }; piece: { id: string } };
	id: string;
}

function loadKit(): {
	connections: KitConnection[];
	pieceById: Record<string, KitPiece>;
	typeById: Record<string, KitType>;
} {
	const raw = JSON.parse(readFileSync(kitPath, "utf8")) as {
		wip: { initialKit: { designs: { items: { connections: { items: KitConnection[] }; id: string; pieces: { items: KitPiece[] } }[] }; types: { items: KitType[] } } };
	};
	const kit = raw.wip.initialKit;
	const nak = kit.designs.items.find((d) => d.id === "9a890dd4-0a9c-48ac-920a-9e62666465ef");
	if (!nak) {
		throw new Error("Nakagin Capsule Tower design not found in metabolism kit.");
	}
	const pieces = nak.pieces.items;
	const pieceById = Object.fromEntries(pieces.map((p) => [p.id, p]));
	const typeById = Object.fromEntries(kit.types.items.map((t) => [t.id, t]));
	return { connections: nak.connections.items, pieceById, typeById };
}
//#endregion 🔖Kit

//#region 🔖Layout
function connectorName(typeById: Record<string, KitType>, piece: KitPiece, connectorId: string): string {
	const typ = typeById[piece.type.id];
	const list = typ.connectors?.items ?? [];
	const found = list.find((c) => c.id === connectorId);
	return found?.name ?? "link";
}

function handleId(typeById: Record<string, KitType>, pieceId: string, connectorId: string, pieceById: Record<string, KitPiece>): string {
	const piece = pieceById[pieceId];
	return `${pieceId}:${connectorName(typeById, piece, connectorId)}`;
}

function pieceRadius(p: KitPiece): number {
	if (p.name === "b") {
		return 20;
	}
	if (p.name.startsWith("cs_")) {
		return 10;
	}
	if (p.name.startsWith("t_")) {
		return 14;
	}
	return 12;
}

function mulberry32(seed: number): () => number {
	return () => {
		let t = (seed += 0x6d2b79f5);
		t = Math.imul(t ^ (t >>> 15), t | 1);
		t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
		return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
	};
}

function forceLayout(
	pieceIds: string[],
	edgePairs: [string, string][],
	anchorId: string | null,
): Record<string, { x: number; y: number }> {
	const rand = mulberry32(0x9a89_0dd4);
	const pos: Record<string, { x: number; y: number; vx: number; vy: number }> = {};
	for (const id of pieceIds) {
		pos[id] = { vx: 0, vy: 0, x: (rand() - 0.5) * 40, y: (rand() - 0.5) * 40 };
	}
	if (anchorId && pos[anchorId]) {
		pos[anchorId].x = 0;
		pos[anchorId].y = 0;
	}
	const kRep = 120_000;
	const kAttr = 0.018;
	const dt = 0.45;
	const damping = 0.88;
	for (let iter = 0; iter < 420; iter += 1) {
		const fx: Record<string, number> = Object.fromEntries(pieceIds.map((id) => [id, 0]));
		const fy: Record<string, number> = Object.fromEntries(pieceIds.map((id) => [id, 0]));
		for (let i = 0; i < pieceIds.length; i += 1) {
			for (let j = i + 1; j < pieceIds.length; j += 1) {
				const a = pieceIds[i];
				const b = pieceIds[j];
				let dx = pos[a].x - pos[b].x;
				let dy = pos[a].y - pos[b].y;
				let dist = Math.hypot(dx, dy);
				if (dist < 0.01) {
					dist = 0.01;
					dx = 0.01;
					dy = 0;
				}
				const f = kRep / (dist * dist);
				const ux = (dx / dist) * f;
				const uy = (dy / dist) * f;
				fx[a] += ux;
				fy[a] += uy;
				fx[b] -= ux;
				fy[b] -= uy;
			}
		}
		for (const [a, b] of edgePairs) {
			const dx = pos[b].x - pos[a].x;
			const dy = pos[b].y - pos[a].y;
			const dist = Math.hypot(dx, dy) + 0.01;
			const f = kAttr * dist;
			const ux = (dx / dist) * f;
			const uy = (dy / dist) * f;
			fx[a] += ux;
			fy[a] += uy;
			fx[b] -= ux;
			fy[b] -= uy;
		}
		for (const id of pieceIds) {
			pos[id].vx = (pos[id].vx + fx[id] * dt) * damping;
			pos[id].vy = (pos[id].vy + fy[id] * dt) * damping;
			pos[id].x += pos[id].vx * dt;
			pos[id].y += pos[id].vy * dt;
		}
	}
	const out: Record<string, { x: number; y: number }> = {};
	for (const id of pieceIds) {
		out[id] = { x: pos[id].x, y: pos[id].y };
	}
	return out;
}

function normalizeLayout(pos: Record<string, { x: number; y: number }>, pieceById: Record<string, KitPiece>): {
	camera: { x: number; y: number; zoom: number };
	pos: Record<string, { x: number; y: number }>;
} {
	const ids = Object.keys(pos);
	let minX = Infinity;
	let minY = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	for (const id of ids) {
		const r = pieceRadius(pieceById[id]);
		minX = Math.min(minX, pos[id].x - r);
		maxX = Math.max(maxX, pos[id].x + r);
		minY = Math.min(minY, pos[id].y - r);
		maxY = Math.max(maxY, pos[id].y + r);
	}
	const cx = (minX + maxX) / 2;
	const cy = (minY + maxY) / 2;
	const span = Math.max(maxX - minX, maxY - minY, 1);
	const target = 2000;
	const s = target / span;
	const next: Record<string, { x: number; y: number }> = {};
	for (const id of ids) {
		next[id] = {
			x: Math.round((pos[id].x - cx) * s * 1000) / 1000,
			y: Math.round((pos[id].y - cy) * s * 1000) / 1000,
		};
	}
	const zoom = Math.min(720, 520) / (target * 1.08);
	return { camera: { x: 0, y: 0, zoom: Math.round(zoom * 10_000) / 10_000 }, pos: next };
}
//#endregion 🔖Layout

//#region 🔖Main
function main(): void {
	const { connections, pieceById, typeById } = loadKit();
	const pieceIds = Object.keys(pieceById);
	const anchor = pieceIds.find((id) => pieceById[id].name === "b") ?? null;
	const edgePairs: [string, string][] = connections.map((c) => [c.connected.piece.id, c.connecting.piece.id]);
	let pos = forceLayout(pieceIds, edgePairs, anchor);
	const { camera, pos: posN } = normalizeLayout(pos, pieceById);
	pos = posN;

	const handleAngles = new Map<string, number>();
	function setAngle(hid: string, ax: number, ay: number, bx: number, by: number): void {
		const ang = Math.atan2(by - ay, bx - ax);
		handleAngles.set(hid, Math.round(ang * 1e6) / 1e6);
	}

	for (const c of connections) {
		const pa = c.connected.piece.id;
		const ca = c.connected.connector.id;
		const pb = c.connecting.piece.id;
		const cb = c.connecting.connector.id;
		const ha = handleId(typeById, pa, ca, pieceById);
		const hb = handleId(typeById, pb, cb, pieceById);
		const ax = pos[pa].x;
		const ay = pos[pa].y;
		const bx = pos[pb].x;
		const by = pos[pb].y;
		setAngle(ha, ax, ay, bx, by);
		setAngle(hb, bx, by, ax, ay);
	}

	const nodes = pieceIds
		.map((id) => {
			const p = pieceById[id];
			const handles = [...handleAngles.keys()]
				.filter((h) => h.startsWith(`${id}:`))
				.map((hid) => ({ angle: handleAngles.get(hid)!, id: hid }))
				.sort((a, b) => a.id.localeCompare(b.id));
			return {
				handles,
				id,
				label: p.name,
				radius: pieceRadius(p),
				x: pos[id].x,
				y: pos[id].y,
			};
		})
		.sort((a, b) => a.id.localeCompare(b.id));

	const edges = connections
		.map((c) => ({
			from: handleId(typeById, c.connected.piece.id, c.connected.connector.id, pieceById),
			id: c.id,
			to: handleId(typeById, c.connecting.piece.id, c.connecting.connector.id, pieceById),
		}))
		.sort((a, b) => a.id.localeCompare(b.id));

	const fixture = {
		schema: "elements.board.fixture/v1",
		camera,
		nodes,
		edges,
	};

	writeFileSync(outPath, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
	console.log(`[nakagin-board] wrote ${outPath} nodes=${nodes.length} edges=${edges.length}`);
}

main();
//#endregion 🔖Main
