/** 🧬️ Lowpoly snapshot schema — artifact-lane fields only. */

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
  /** `null` when the object owns no mesh yet — confirmed against the `create-object` mutation fixture. */
  mesh: LowpolyMeshHandle | null;
  paintLayers: LowpolyPaintLayer[];
}

export interface LowpolyMeshHandle {
  childId: string;
  target: ArtifactRef;
}

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
