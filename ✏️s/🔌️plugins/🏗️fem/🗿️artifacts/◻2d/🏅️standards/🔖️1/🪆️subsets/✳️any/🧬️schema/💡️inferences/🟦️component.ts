/** 💡️ Fem2d inference schema — bounds (plan-view extent + node/element counts). */

export interface Fem2dBoundingBox {
  min: [number, number];
  max: [number, number];
}

export interface Fem2dBounds {
  boundingBox: Fem2dBoundingBox;
  nodeCount: number;
  elementCount: number;
}

export interface Fem2dInference {
  /** @state inferred */
  bounds: Fem2dBounds;
}
