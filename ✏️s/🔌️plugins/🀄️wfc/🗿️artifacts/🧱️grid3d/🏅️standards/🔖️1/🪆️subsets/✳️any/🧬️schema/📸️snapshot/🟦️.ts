/** 🧬 s.wfc.grid3d snapshot — the TypeScript twin of the normative JSON Schema, ported field by
 * field (never generated). The wire form is camelCase, exactly as the Rust `#[value(rename_all =
 * "camelCase")]` records emit it. */

export const WFC_GRID3D_DOCUMENT_SCHEMA = "s.wfc.grid3d";

export interface Grid3dColor {
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface Grid3dMesh {
  positions: number[];
  indices: number[];
  color?: Grid3dColor;
}

export interface ArtifactChildHandle {
  childId: string;
  target: { artifactId: string; dialect: { artifactKind: string; standard: string; subset: string } };
}

export type Grid3dTileMedia = { kind: "mesh"; mesh: Grid3dMesh } | { kind: "meshChild"; child: ArtifactChildHandle };

export interface Grid3dTile {
  id: string;
  label?: string;
  weight: number;
  media: Grid3dTileMedia;
}

export type Grid3dDirection = "LEFT" | "RIGHT" | "FRONT" | "BACK" | "BOTTOM" | "TOP";
export type Grid3dAxis = "x" | "y" | "z";

export interface Grid3dRule {
  id: string;
  tileAId: string;
  tileBId: string;
  direction: Grid3dDirection;
  allowed: boolean;
}

export interface Grid3dPinnedCell {
  x: number;
  y: number;
  z: number;
  tileId: string;
}

export interface Grid3dCell {
  x: number;
  y: number;
  z: number;
}

export interface Grid3dSnapshot {
  schema: string;
  seed: number;
  width: number;
  height: number;
  depth: number;
  cellSizesX: number[];
  cellSizesY: number[];
  cellSizesZ: number[];
  periodicX: boolean;
  periodicY: boolean;
  periodicZ: boolean;
  tiles: Grid3dTile[];
  rules: Grid3dRule[];
  pinned: Grid3dPinnedCell[];
  masked: Grid3dCell[];
}

/** 📐 Cell `index`'s lower world coordinate on one axis — the cumulative sum of every size before it.
 * The twin of the Rust `axis_offset`, so a TypeScript consumer places a non-uniform cell identically. */
export function axisOffset(sizes: number[], index: number): number {
  let total = 0;
  for (let cursor = 0; cursor < Math.min(index, sizes.length); cursor += 1) total += sizes[cursor];
  return total;
}

/** 📐 Cell `index`'s own size, defaulting to a unit cell for an index the array does not reach. */
export function axisSize(sizes: number[], index: number): number {
  const size = sizes[index];
  return Number.isFinite(size) && size > 0 ? size : 1;
}

/** 📐 The canonical sort key of one pinned or masked cell. */
export function cellKey(x: number, y: number, z: number): string {
  return `${x}:${y}:${z}`;
}

export function emptyGrid3dSnapshot(): Grid3dSnapshot {
  return {
    schema: WFC_GRID3D_DOCUMENT_SCHEMA,
    seed: 0,
    width: 1,
    height: 1,
    depth: 1,
    cellSizesX: [1],
    cellSizesY: [1],
    cellSizesZ: [1],
    periodicX: false,
    periodicY: false,
    periodicZ: false,
    tiles: [],
    rules: [],
    pinned: [],
    masked: [],
  };
}
