// #region 🧲Header
/** @emoji 🖼️ Mesh transfer validation and GPU-ready geometry data. */
// #endregion 🧲Header

import type { MeshTransfer } from "./contracts.ts";

// #region 🖼️MeshValidation
function isFiniteBuffer(buf: Float32Array | Uint32Array | undefined): boolean {
	if (!buf || buf.length === 0) return true;
	for (const value of buf) {
		if (!Number.isFinite(value)) return false;
	}
	return true;
}

/** @emoji ✅ True when mesh buffers are non-empty and finite. */
export function isRenderableMeshTransfer(mesh: MeshTransfer): boolean {
	const hasTris = mesh.position.length > 0 && mesh.index.length > 0;
	const hasEdges = mesh.edges.length > 0;
	const hasPoints = (mesh.points?.length ?? 0) > 0;
	if (!hasTris && !hasEdges && !hasPoints) return false;
	if (hasTris) {
		if (mesh.position.length % 3 !== 0) return false;
		if (mesh.normal.length !== mesh.position.length) return false;
		const vertexCount = mesh.position.length / 3;
		for (const value of mesh.index) {
			if (!Number.isFinite(value) || value < 0 || value >= vertexCount) return false;
		}
	}
	if (hasEdges && mesh.edges.length % 3 !== 0) return false;
	if (hasPoints && (mesh.points!.length % 3 !== 0)) return false;
	return isFiniteBuffer(mesh.position) && isFiniteBuffer(mesh.normal) && isFiniteBuffer(mesh.edges) && isFiniteBuffer(mesh.points);
}
// #endregion 🖼️MeshValidation

// #region 🖼️MeshGeometryData
/** @emoji 📦 Three.js-free grouped mesh buffers for R3F upload. */
export interface MeshGeometryData {
	readonly position: Float32Array;
	readonly normal: Float32Array;
	readonly index: Uint32Array;
	readonly edges: Float32Array;
	readonly points: Float32Array;
	readonly faceGroups: readonly { readonly start: number; readonly count: number }[];
}

/** @emoji 🔧 Converts `MeshTransfer` to grouped buffer geometry data. */
export function meshTransferToGeometryData(data: MeshTransfer): MeshGeometryData {
	if (!isRenderableMeshTransfer(data)) {
		return { position: new Float32Array(0), normal: new Float32Array(0), index: new Uint32Array(0), edges: new Float32Array(0), points: new Float32Array(0), faceGroups: [] };
	}
	return {
		position: data.position,
		normal: data.normal,
		index: data.index,
		edges: data.edges,
		points: data.points ?? new Float32Array(0),
		faceGroups: data.faceGroups.map((g) => ({ start: g.start, count: g.count })),
	};
}
// #endregion 🖼️MeshGeometryData
