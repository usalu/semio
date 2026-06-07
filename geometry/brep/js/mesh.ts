// #region 🧲Header
/** @emoji 🖼️ Mesh transfer validation and GPU-ready geometry data. */
// #endregion 🧲Header

import type { MeshTransfer } from "./contracts.ts";

// #region 🖼️MeshValidation
/** @emoji ✅ True when mesh buffers are non-empty and finite. */
export function isRenderableMeshTransfer(mesh: MeshTransfer): boolean {
	if (mesh.position.length === 0) return false;
	if (mesh.position.length % 3 !== 0) return false;
	if (mesh.normal.length !== mesh.position.length) return false;
	if (mesh.edges.length % 3 !== 0) return false;
	for (const value of mesh.position) {
		if (!Number.isFinite(value)) return false;
	}
	for (const value of mesh.normal) {
		if (!Number.isFinite(value)) return false;
	}
	for (const value of mesh.edges) {
		if (!Number.isFinite(value)) return false;
	}
	const vertexCount = mesh.position.length / 3;
	for (const value of mesh.index) {
		if (!Number.isFinite(value) || value < 0 || value >= vertexCount) return false;
	}
	return true;
}
// #endregion 🖼️MeshValidation

// #region 🖼️MeshGeometryData
/** @emoji 📦 Three.js-free grouped mesh buffers for R3F upload. */
export interface MeshGeometryData {
	readonly position: Float32Array;
	readonly normal: Float32Array;
	readonly index: Uint32Array;
	readonly faceGroups: readonly { readonly start: number; readonly count: number }[];
}

/** @emoji 🔧 Converts `MeshTransfer` to grouped buffer geometry data. */
export function meshTransferToGeometryData(data: MeshTransfer): MeshGeometryData {
	if (!isRenderableMeshTransfer(data)) {
		return { position: new Float32Array(0), normal: new Float32Array(0), index: new Uint32Array(0), faceGroups: [] };
	}
	return {
		position: data.position,
		normal: data.normal,
		index: data.index,
		faceGroups: data.faceGroups.map((g) => ({ start: g.start, count: g.count })),
	};
}
// #endregion 🖼️MeshGeometryData
