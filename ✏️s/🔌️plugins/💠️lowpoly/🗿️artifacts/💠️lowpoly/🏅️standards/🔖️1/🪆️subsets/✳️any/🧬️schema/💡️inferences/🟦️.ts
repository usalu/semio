/** 💡️ Lowpoly inference schema — object count + 3d bounding box across every object's transform
 * position. */

export interface LowpolyBounds {
  min: [number, number, number];
  max: [number, number, number];
}

export interface LowpolyInference {
  /** @derived */
  objectCount: number;
  /** @derived */
  bounds: LowpolyBounds | null;
}
