/** 💡️ Process3d inference schema — stockBounds (world-space AABB) + stepCount. */

export interface BoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

export interface Process3dInference {
  /** @state inferred */
  stockBounds: BoundingBox;
  /** @state inferred */
  stepCount: number;
}
