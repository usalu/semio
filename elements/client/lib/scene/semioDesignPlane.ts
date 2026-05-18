// #region 🧾SemioDesignPlane
/** @emoji 🧾 Authoring-space plane (semio Z-up) before three.js conversion. */
export interface SemioAuthoringPlane {
	readonly origin: { x: number; y: number; z: number };
	readonly xAxis: { x: number; y: number; z: number };
	readonly yAxis: { x: number; y: number; z: number };
}

function isVec3ish(v: unknown): v is { x: number; y: number; z: number } {
	if (!v || typeof v !== "object") return false;
	const o = v as Record<string, unknown>;
	return typeof o.x === "number" && typeof o.y === "number" && typeof o.z === "number";
}

function isSemioAuthoringPlane(v: unknown): v is SemioAuthoringPlane {
	if (!v || typeof v !== "object") return false;
	const p = v as Record<string, unknown>;
	return isVec3ish(p.origin) && isVec3ish(p.xAxis) && isVec3ish(p.yAxis);
}

/** @emoji 🧾 Pulls a piece plane from shallow/flat design JSON (`pose.plane`, top-level `plane`, or `semio.plane` attribute). */
export function semioAuthoringPlaneFromDesignPiece(piece: Record<string, unknown>): SemioAuthoringPlane | null {
	const pose = piece["pose"] as Record<string, unknown> | undefined;
	const posePlane = pose?.["plane"];
	if (isSemioAuthoringPlane(posePlane)) return posePlane;
	const top = piece["plane"];
	if (isSemioAuthoringPlane(top)) return top;
	const attrs = piece["attributes"] as unknown[] | undefined;
	if (!Array.isArray(attrs)) return null;
	for (const a of attrs) {
		if (!a || typeof a !== "object") continue;
		const row = a as Record<string, unknown>;
		if (row["key"] !== "semio.plane" || typeof row["value"] !== "string") continue;
		try {
			const parsed = JSON.parse(row["value"]) as unknown;
			if (isSemioAuthoringPlane(parsed)) return parsed;
		} catch {
			/* ignore */
		}
	}
	return null;
}

/** @emoji 🧾 Materialized flat layout sidecar: `elements.scene.flat-layout-planes/v1` with `byPieceName` → authoring plane (board `label` matches piece `name`). */
export function mergeAuthoringPlanesFromFlatLayoutPlanesV1Doc(doc: unknown, into: Map<string, SemioAuthoringPlane>): void {
	if (!doc || typeof doc !== "object") return;
	const r = doc as Record<string, unknown>;
	if (r["schema"] !== "elements.scene.flat-layout-planes/v1") return;
	const by = r["byPieceName"] as Record<string, unknown> | undefined;
	if (!by || typeof by !== "object") return;
	for (const [name, plane] of Object.entries(by)) {
		if (!name || !isSemioAuthoringPlane(plane)) continue;
		into.set(name, plane);
	}
}

/** @emoji 🧾 Optional WASM dump: `elements.scene.flat-planes/v1` with `byPieceId` authoring planes (overrides JSON-derived planes). */
export function mergeAuthoringPlanesFromFlatPlanesV1Doc(doc: unknown, into: Map<string, SemioAuthoringPlane>): void {
	if (!doc || typeof doc !== "object") return;
	const r = doc as Record<string, unknown>;
	if (r["schema"] !== "elements.scene.flat-planes/v1") return;
	const by = r["byPieceId"] as Record<string, unknown> | undefined;
	if (!by || typeof by !== "object") return;
	for (const [id, plane] of Object.entries(by)) {
		if (isSemioAuthoringPlane(plane)) into.set(id, plane);
	}
}

/** @emoji 🧾 Indexes every `pieces[]` entry that carries a resolvable plane (later ids override earlier). */
export function mergeAuthoringPlanesFromDesignDoc(doc: Record<string, unknown>, into: Map<string, SemioAuthoringPlane>): void {
	const pieces = doc["pieces"] as unknown[] | undefined;
	if (!Array.isArray(pieces)) return;
	for (const raw of pieces) {
		if (!raw || typeof raw !== "object") continue;
		const piece = raw as Record<string, unknown>;
		const id = String(piece["id"] ?? "");
		if (!id) continue;
		const pl = semioAuthoringPlaneFromDesignPiece(piece);
		if (pl) into.set(id, pl);
	}
}
// #endregion 🧾SemioDesignPlane
