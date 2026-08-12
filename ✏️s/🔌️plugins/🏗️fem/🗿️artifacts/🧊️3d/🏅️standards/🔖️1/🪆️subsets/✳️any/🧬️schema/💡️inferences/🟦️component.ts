/** 💡️ Fem3d inference schema — bounds (3d extent + node/element counts). */

export interface Fem3dBoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Fem3dBounds {
  boundingBox: Fem3dBoundingBox;
  nodeCount: number;
  elementCount: number;
}

export interface Fem3dInference {
  /** @state inferred */
  bounds: Fem3dBounds;
}
