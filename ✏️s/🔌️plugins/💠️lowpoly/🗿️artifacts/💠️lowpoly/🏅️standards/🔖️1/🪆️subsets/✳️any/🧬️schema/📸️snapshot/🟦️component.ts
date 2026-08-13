/** 🧬️ Lowpoly snapshot schema — persistent fields only. */

export interface LowpolySnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objects: LowpolyObject[];
}

export interface LowpolySelectionTargets {
  mesh: boolean;
  vertex: boolean;
  edge: boolean;
  face: boolean;
}

export interface LowpolySelection {
  targets: LowpolySelectionTargets;
  keys: string[];
  mode: string;
  ids: number[];
}

export interface LowpolyTransform {
  position: [number, number, number];
  rotation: [number, number, number];
  scale: [number, number, number];
}

export interface LowpolyPaintLayer {
  name: string;
  visible: boolean;
  opacity: number;
  blendMode: string;
  pixels: string;
}

export interface LowpolyObject {
  id: string;
  name: string;
  transform: LowpolyTransform;
  smoothShading: boolean;
  meshJson: string;
  paintLayers: LowpolyPaintLayer[];
}
