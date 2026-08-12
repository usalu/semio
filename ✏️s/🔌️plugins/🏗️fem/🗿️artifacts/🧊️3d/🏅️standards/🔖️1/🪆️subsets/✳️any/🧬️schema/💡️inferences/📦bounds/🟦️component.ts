/** 📦 `bounds` — 3d extent plus node/element counts, derived from `nodes`/`elements`. */

export interface Fem3dBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Fem3dBounds {
  boundingBox: Fem3dBoundingBox;
  nodeCount: number;
  elementCount: number;
}
