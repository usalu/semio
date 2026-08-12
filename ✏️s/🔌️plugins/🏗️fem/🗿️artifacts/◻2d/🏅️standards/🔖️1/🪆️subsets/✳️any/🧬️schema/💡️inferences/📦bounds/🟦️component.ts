/** 📦 `bounds` — plan-view extent plus node/element counts, derived from `nodes`/`elements`. */

export interface Fem2dBoundingBox {
  min: [number, number];
  max: [number, number];
}

export interface Fem2dBounds {
  boundingBox: Fem2dBoundingBox;
  nodeCount: number;
  elementCount: number;
}
