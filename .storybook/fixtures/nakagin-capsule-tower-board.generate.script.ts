// #region 🧲Header
// 💻 .storybook/fixtures/nakagin-capsule-tower-board.generate.script.ts
// Specs: Regenerate `nakagin-capsule-tower.board.json` from `metabolism.kit.semio.json` Nakagin parent + Flat designs; attach `nodeKind` / `handleKind` + `meta.kindCatalogs` from `metabolism.kit.light.semio.json` (kit types + Nakagin family ports).
// Summary: Geometry and connector **names** still come from the full kit (Flat `pose.center` only exists there); every emitted node gets the light kit **`nodeKind`** for its piece `type.id`, each handle gets **`port.handleKind`** keyed by `(typeId, connectorName)`; `meta.kindCatalogs` lists one **node** catalog row per unique `nodeKind` (id=`nodeKind`, label=name) and one **handle** row per Nakagin family port (id=`handleKind`, label=port name, stable HSL color, `defaultWireKind`=`board.wire.link`). Uniform 40×40 footprint and icon stems unchanged.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

//#region 🔖Kit
const __dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dir, "../..");
const kitPath = join(repoRoot, "semio/assets/fixtures/metabolism.kit.semio.json");
const lightKitPath = join(repoRoot, "semio/assets/fixtures/metabolism.kit.light.semio.json");
const outPath = join(__dir, "nakagin-capsule-tower.board.json");

/** @emoji 🔗 Matches {@link BOARD_BUILTIN_LINK_WIRE_KIND} in elements board (gesture default for catalog handles). */
const LIGHT_PATCH_DEFAULT_WIRE_KIND = "board.wire.link";

/** @emoji 🗼 Parent graph (stable piece ids and connections) — same id as `MetabolismKitNakaginCapsuleTowerDesigns` primary record. */
const NAKAGIN_PARENT_DESIGN_ID = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

/** @emoji 🗺️ Flat layout design carrying pose.center u/v for every Nakagin piece name (180 pieces, matches parent names). */
const NAKAGIN_FLAT_DESIGN_ID = "79fa8945-b47d-4896-965f-f921067cbae2";

interface KitConnector {
	id: string;
	name?: string;
}

interface KitType {
	connectors?: { items?: KitConnector[] };
	icon?: string;
	id: string;
}

interface KitPiece {
	id: string;
	name: string;
	pose?: { center?: { u: number; v: number } };
	type: { id: string };
}

type KitConnectionEnd = { connector: { id: string }; piece: { id: string } };

interface KitConnection {
	id: string;
	connected?: KitConnectionEnd;
	connecting?: KitConnectionEnd;
	parent?: KitConnectionEnd;
	child?: KitConnectionEnd;
}

interface LightFamilyPort {
	handleKind: string;
	id: string;
	name: string;
}

interface LightKitConnector {
	id: string;
	name?: string;
	port?: { handleKind?: string };
}

interface LightKitType {
	connectors?: { items?: LightKitConnector[] };
	id: string;
	name: string;
	nodeKind?: string;
}

/** @emoji 🔁 Normalizes kit connection endpoints across legacy (`connected`/`connecting`) and current (`parent`/`child`) shapes. */
function kitConnectionEnds(c: KitConnection): { from: KitConnectionEnd; to: KitConnectionEnd } {
	if (c.connected && c.connecting) {
		return { from: c.connected, to: c.connecting };
	}
	if (c.parent && c.child) {
		return { from: c.parent, to: c.child };
	}
	throw new Error(`[nakagin-board] connection ${c.id}: expected connected/connecting or parent/child`);
}

function loadKit(): {
	centerUvByName: Record<string, { u: number; v: number }>;
	connections: KitConnection[];
	pieceById: Record<string, KitPiece>;
	typeById: Record<string, KitType>;
} {
	const raw = JSON.parse(readFileSync(kitPath, "utf8")) as {
		wip: { initialKit: { designs: { items: { connections: { items: KitConnection[] }; id: string; pieces: { items: KitPiece[] } }[] }; types: { items: KitType[] } } };
	};
	const kit = raw.wip.initialKit;
	const nak = kit.designs.items.find((d) => d.id === NAKAGIN_PARENT_DESIGN_ID);
	if (!nak) {
		throw new Error("Nakagin Capsule Tower parent design not found in metabolism kit.");
	}
	const flat = kit.designs.items.find((d) => d.id === NAKAGIN_FLAT_DESIGN_ID);
	if (!flat) {
		throw new Error("Nakagin Capsule Tower Flat design not found in metabolism kit.");
	}
	const centerUvByName: Record<string, { u: number; v: number }> = {};
	for (const p of flat.pieces.items) {
		const c = p.pose?.center;
		if (!c || !Number.isFinite(c.u) || !Number.isFinite(c.v)) {
			throw new Error(`[nakagin-board] Flat design piece ${p.name} missing pose.center u/v`);
		}
		centerUvByName[p.name] = { u: c.u, v: c.v };
	}
	const pieces = nak.pieces.items;
	const pieceById = Object.fromEntries(pieces.map((p) => [p.id, p]));
	for (const p of pieces) {
		if (!centerUvByName[p.name]) {
			throw new Error(`[nakagin-board] Flat layout missing center for piece name ${p.name}`);
		}
	}
	const typeById = Object.fromEntries(kit.types.items.map((t) => [t.id, t]));
	return { centerUvByName, connections: nak.connections.items, pieceById, typeById };
}

/** @emoji 🧬 Maps light-kit `port.handleKind` by `(typeId, connectorDisplayName)`; separator is newline so connector names never collide. */
function loadLightSemantics(): {
	handleKindByTypeConnectorName: Map<string, string>;
	kindCatalogs: {
		handles: { color: string; defaultWireKind: string; id: string; label: string; name: string }[];
		nodes: { id: string; label: string; name: string }[];
	};
	nodeKindByTypeId: Record<string, string>;
} {
	const raw = JSON.parse(readFileSync(lightKitPath, "utf8")) as {
		wip: {
			families: { items: Array<{ name?: string; ports?: { items?: LightFamilyPort[] } }> };
			initialKit: { types: { items: LightKitType[] } };
		};
	};
	const types = raw.wip.initialKit.types.items;
	const handleKindByTypeConnectorName = new Map<string, string>();
	for (const t of types) {
		for (const c of t.connectors?.items ?? []) {
			const hk = typeof c.port?.handleKind === "string" ? c.port.handleKind.trim() : "";
			if (hk === "") continue;
			const nm = (c.name ?? "link").trim() || "link";
			handleKindByTypeConnectorName.set(`${t.id}\n${nm}`, hk);
		}
	}
	const nodeKindByTypeId: Record<string, string> = {};
	const nodeRows: { id: string; label: string; name: string }[] = [];
	const seenNodeKind = new Set<string>();
	for (const t of types) {
		const nk = typeof t.nodeKind === "string" ? t.nodeKind.trim() : "";
		if (nk === "") continue;
		nodeKindByTypeId[t.id] = nk;
		if (seenNodeKind.has(nk)) continue;
		seenNodeKind.add(nk);
		nodeRows.push({ id: nk, label: t.name, name: t.name });
	}
	let portItems: LightFamilyPort[] = [];
	for (const fam of raw.wip.families.items) {
		if (fam.name === "Nakagin Capsule Tower" && fam.ports?.items?.length) {
			portItems = fam.ports.items;
			break;
		}
	}
	if (portItems.length === 0) {
		throw new Error("[nakagin-board] Nakagin Capsule Tower family ports not found in light kit.");
	}
	const handleRows = portItems.map((p) => ({
		color: stableCatalogColorForKindId(p.handleKind),
		defaultWireKind: LIGHT_PATCH_DEFAULT_WIRE_KIND,
		id: p.handleKind,
		label: p.name,
		name: p.name,
	}));
	handleRows.sort((a, b) => a.id.localeCompare(b.id));
	nodeRows.sort((a, b) => a.id.localeCompare(b.id));
	return {
		handleKindByTypeConnectorName,
		kindCatalogs: { handles: handleRows, nodes: nodeRows },
		nodeKindByTypeId,
	};
}

/** @emoji 🎨 Deterministic pleasant HSL for WASM handle catalog rows (must include `color`). */
function stableCatalogColorForKindId(id: string): string {
	let h = 2166136261;
	for (let i = 0; i < id.length; i++) {
		h = Math.imul(h ^ id.charCodeAt(i), 16777619);
	}
	const hue = Math.abs(h) % 360;
	return `hsl(${hue} 52% 48%)`;
}

/** @emoji 🏷️ Kit `icon` like `icons/capsule_p.svg` → catalog stem `capsule_p` baked in `board_metabolism_icon_match`. */
function catalogKeyFromKitIconPath(icon: string | undefined): string | undefined {
	if (icon === undefined || typeof icon !== "string") {
		return undefined;
	}
	const t = icon.trim();
	if (!t) {
		return undefined;
	}
	const base = t.replace(/^.*\//, "").replace(/\.svg$/i, "");
	return base.length > 0 ? base : undefined;
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

function handleKindForBoardHandleId(
	handleKindByTypeConnectorName: Map<string, string>,
	pieceById: Record<string, KitPiece>,
	handleFullId: string,
): string {
	const colon = handleFullId.indexOf(":");
	const pieceId = colon >= 0 ? handleFullId.slice(0, colon) : handleFullId;
	const connectorDisplayName = colon >= 0 ? handleFullId.slice(colon + 1) : "link";
	const piece = pieceById[pieceId];
	if (!piece) {
		throw new Error(`[nakagin-board] missing piece for handle id ${handleFullId}`);
	}
	const hk = handleKindByTypeConnectorName.get(`${piece.type.id}\n${connectorDisplayName}`);
	if (!hk) {
		throw new Error(
			`[nakagin-board] missing light handleKind for type ${piece.type.id} connector "${connectorDisplayName}" (handle ${handleFullId})`,
		);
	}
	return hk;
}

type PieceLayout =
	| { bounds: "circle"; radius: number }
	| { bounds: "rectangle"; height: number; width: number };

/** @emoji 📐 `t_*` cluster circles use this u/v radius; `cs_*` axis-aligned squares use 2× as width/height so default footprint matches cluster circle diameter. */
const NAKAGIN_CLUSTER_NODE_RADIUS_UV = 0.22;

/** @emoji 📐 Bounds in sheet u/v units (same as Flat pose.center); cs_* footprint matches t_* headline circle diameter. */
function pieceLayout(p: KitPiece): PieceLayout {
	if (p.name === "b") {
		return { bounds: "circle", radius: 0.36 };
	}
	if (p.name.startsWith("cs_")) {
		const d = NAKAGIN_CLUSTER_NODE_RADIUS_UV * 2;
		return { bounds: "rectangle", height: d, width: d };
	}
	if (p.name.startsWith("t_")) {
		return { bounds: "circle", radius: NAKAGIN_CLUSTER_NODE_RADIUS_UV };
	}
	return { bounds: "circle", radius: 0.16 };
}

/** @emoji 🧭 Handle angle toward neighbor: rectangle stores north-zero CCW atan2(-dx,-dy); circle stores east-zero atan2(dy,dx); both wrapped to [0,2π). */
function handleAngleToward(from: KitPiece, ax: number, ay: number, bx: number, by: number): number {
	const dx = bx - ax;
	const dy = by - ay;
	const L = pieceLayout(from);
	let ang = L.bounds === "rectangle" ? Math.atan2(-dx, -dy) : Math.atan2(dy, dx);
	const tau = Math.PI * 2;
	ang %= tau;
	if (ang < 0) {
		ang += tau;
	}
	return Math.round(ang * 1e6) / 1e6;
}

/** @emoji 🌍 Board world: x=u, y=-v so increasing flat v moves up on screen. */
function sheetToWorld(uv: { u: number; v: number }): { x: number; y: number } {
	return { x: uv.u, y: -uv.v };
}

function normalizeLayout(pos: Record<string, { x: number; y: number }>, pieceById: Record<string, KitPiece>): {
	camera: { x: number; y: number; zoom: number };
	layoutScale: number;
	pos: Record<string, { x: number; y: number }>;
} {
	const ids = Object.keys(pos);
	let minX = Infinity;
	let minY = Infinity;
	let maxX = -Infinity;
	let maxY = -Infinity;
	for (const id of ids) {
		const L = pieceLayout(pieceById[id]);
		const p = pos[id];
		if (L.bounds === "rectangle") {
			const hw = L.width / 2;
			const hh = L.height / 2;
			minX = Math.min(minX, p.x - hw);
			maxX = Math.max(maxX, p.x + hw);
			minY = Math.min(minY, p.y - hh);
			maxY = Math.max(maxY, p.y + hh);
		} else {
			const r = L.radius;
			minX = Math.min(minX, p.x - r);
			maxX = Math.max(maxX, p.x + r);
			minY = Math.min(minY, p.y - r);
			maxY = Math.max(maxY, p.y + r);
		}
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
	return {
		camera: { x: 0, y: 0, zoom: Math.round(zoom * 10_000) / 10_000 },
		layoutScale: s,
		pos: next,
	};
}
//#endregion 🔖Layout

/** @emoji 📐 Uniform world footprint for every node (rectangles 40×40; circles radius 20). */
const NAKAGIN_BOARD_UNIFORM_NODE_SIZE_PX = 40;

type GeneratedBoardHandle = { angle: number; handleKind: string; id: string; radius: number };

type GeneratedBoardNode = (
	| {
			handles: GeneratedBoardHandle[];
			height: number;
			id: string;
			label: string;
			nodeKind: string;
			shape: "rectangle";
			width: number;
			x: number;
			y: number;
	  }
	| { handles: GeneratedBoardHandle[]; id: string; label: string; nodeKind: string; radius: number; x: number; y: number }
) & { iconKind?: string };

/** @emoji 📐 Replaces scaled per-kind sizes with {@link NAKAGIN_BOARD_UNIFORM_NODE_SIZE_PX} and matching handle hit radii. */
function applyUniformNodeFootprint(nodes: GeneratedBoardNode[]): void {
	const d = NAKAGIN_BOARD_UNIFORM_NODE_SIZE_PX;
	const circleR = d / 2;
	for (const n of nodes) {
		if ("shape" in n && n.shape === "rectangle") {
			n.width = d;
			n.height = d;
			const rh = Math.max(2.5, d * 0.065);
			const rounded = Math.round(rh * 1000) / 1000;
			for (const h of n.handles) {
				h.radius = rounded;
			}
		} else {
			n.radius = circleR;
			const rh = Math.max(2.5, circleR * 0.28);
			const rounded = Math.round(rh * 1000) / 1000;
			for (const h of n.handles) {
				h.radius = rounded;
			}
		}
	}
}

//#region 🔖Main
function main(): void {
	const light = loadLightSemantics();
	const { centerUvByName, connections, pieceById, typeById } = loadKit();
	const pieceIds = Object.keys(pieceById);
	const pos: Record<string, { x: number; y: number }> = {};
	for (const id of pieceIds) {
		pos[id] = sheetToWorld(centerUvByName[pieceById[id].name]);
	}
	const { camera, layoutScale, pos: posN } = normalizeLayout(pos, pieceById);
	Object.assign(pos, posN);

	function scaledLayout(pid: string): PieceLayout {
		const L = pieceLayout(pieceById[pid]);
		if (L.bounds === "rectangle") {
			return {
				bounds: "rectangle",
				height: Math.round(L.height * layoutScale * 1000) / 1000,
				width: Math.round(L.width * layoutScale * 1000) / 1000,
			};
		}
		return { bounds: "circle", radius: Math.round(L.radius * layoutScale * 1000) / 1000 };
	}

	function handleRadiusWorld(pid: string): number {
		const L = scaledLayout(pid);
		if (L.bounds === "rectangle") {
			return Math.max(2.5, Math.min(L.width, L.height) * 0.065);
		}
		return Math.max(2.5, L.radius * 0.28);
	}

	const handleAngles = new Map<string, number>();
	for (const c of connections) {
		const { from: endA, to: endB } = kitConnectionEnds(c);
		const pa = endA.piece.id;
		const pb = endB.piece.id;
		const ha = handleId(typeById, pa, endA.connector.id, pieceById);
		const hb = handleId(typeById, pb, endB.connector.id, pieceById);
		const ax = pos[pa].x;
		const ay = pos[pa].y;
		const bx = pos[pb].x;
		const by = pos[pb].y;
		handleAngles.set(ha, handleAngleToward(pieceById[pa], ax, ay, bx, by));
		handleAngles.set(hb, handleAngleToward(pieceById[pb], bx, by, ax, ay));
	}

	const nodes = pieceIds
		.map((id) => {
			const p = pieceById[id];
			const nk = light.nodeKindByTypeId[p.type.id];
			if (!nk) {
				throw new Error(`[nakagin-board] missing light nodeKind for piece type ${p.type.id} (piece ${id})`);
			}
			const iconKind = catalogKeyFromKitIconPath(typeById[p.type.id]?.icon);
			const L = scaledLayout(id);
			const rh = Math.round(handleRadiusWorld(id) * 1000) / 1000;
			const handles = [...handleAngles.keys()]
				.filter((h) => h.startsWith(`${id}:`))
				.map((hid) => ({
					angle: handleAngles.get(hid)!,
					handleKind: handleKindForBoardHandleId(light.handleKindByTypeConnectorName, pieceById, hid),
					id: hid,
					radius: rh,
				}))
				.sort((a, b) => a.id.localeCompare(b.id));
			if (L.bounds === "rectangle") {
				return {
					handles,
					height: L.height,
					id,
					...(iconKind !== undefined ? { iconKind } : {}),
					label: p.name,
					nodeKind: nk,
					shape: "rectangle" as const,
					width: L.width,
					x: pos[id].x,
					y: pos[id].y,
				};
			}
			return {
				handles,
				id,
				...(iconKind !== undefined ? { iconKind } : {}),
				label: p.name,
				nodeKind: nk,
				radius: L.radius,
				x: pos[id].x,
				y: pos[id].y,
			};
		})
		.sort((a, b) => a.id.localeCompare(b.id));

	applyUniformNodeFootprint(nodes);

	const edges = connections
		.map((c) => {
			const { from: endA, to: endB } = kitConnectionEnds(c);
			return {
				from: handleId(typeById, endA.piece.id, endA.connector.id, pieceById),
				id: c.id,
				to: handleId(typeById, endB.piece.id, endB.connector.id, pieceById),
			};
		})
		.sort((a, b) => a.id.localeCompare(b.id));

	const fixture = {
		schema: "elements.board.fixture/v1",
		camera,
		edges,
		meta: { kindCatalogs: light.kindCatalogs },
		nodes,
	};

	writeFileSync(outPath, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
	console.log(`[nakagin-board] wrote ${outPath} nodes=${nodes.length} edges=${edges.length}`);
}

main();
//#endregion 🔖Main
