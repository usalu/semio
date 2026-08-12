/** 💡️ Cad inference schema — object/vertex counts + 3d bounding box across every pane. */

export interface CadBounds {
  min: [number, number, number];
  max: [number, number, number];
}

export interface CadInference {
  /** @state inferred */
  objectCount: number;
  /** @state inferred */
  vertexCount: number;
  /** @state inferred */
  bounds: CadBounds | null;
}
