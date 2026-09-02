/** 🧬️ Lowpoly diff schema — sparse field delta. */

export interface LowpolyDiff {
  /** @state artifact */
  artifact?: LowpolyArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  objects?: LowpolyObjectsDelta;
  /** @state presence */
  activeObjectId?: string | null;
  /** @state presence */
  selection?: LowpolySelection;
  /** @state presence */
  selectedObjectIds?: LowpolyStringList;
  /** @state presence */
  paintUtility?: string;
  /** @state presence */
  activePaintLayer?: number;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  showEdges?: boolean;
  /** @state config */
  sunEnabled?: boolean;
  /** @state config */
  sunAzimuth?: number;
  /** @state config */
  sunElevation?: number;
  /** @state config */
  sunIntensity?: number;
  /** @state config */
  sunColor?: string;
  /** @state config */
  worldCameraPositionX?: number;
  /** @state config */
  worldCameraPositionY?: number;
  /** @state config */
  worldCameraPositionZ?: number;
  /** @state config */
  worldCameraTargetX?: number;
  /** @state config */
  worldCameraTargetY?: number;
  /** @state config */
  worldCameraTargetZ?: number;
  /** @state config */
  worldCameraFov?: number;
  /** @state config */
  utilityParamsJson?: string;
  /** @state config */
  paintColorR?: number;
  /** @state config */
  paintColorG?: number;
  /** @state config */
  paintColorB?: number;
  /** @state config */
  paintColorA?: number;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  selectionModeDefault?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  locale?: string;
  /** @state artifact */
  hoveredObjectId?: string | null;
  /** @state artifact */
  hoveredTargetObjectId?: string | null;
  /** @state artifact */
  hoveredTargetMode?: string | null;
  /** @state artifact */
  hoveredTargetId?: number | null;
  /** @state artifact */
  strokeDragActive?: boolean;
  /** @state artifact */
  transformDragActive?: boolean;
  /** @state artifact */
  previewSeq?: number;
}

export interface LowpolyStringList {
  values: string[];
}

export interface LowpolyObjectsDelta {
  added: LowpolyObject[];
  removed: string[];
  patched: LowpolyObjectPatchEntry[];
  reordered?: string[];
}

export interface LowpolyObjectPatchEntry {
  id: string;
  patch: LowpolyObjectPatch;
  paintLayers?: LowpolyPaintLayersDelta;
}

export interface LowpolyObjectPatch {
  name?: string;
  smoothShading?: boolean;
  transform?: LowpolyTransform;
  /** Double-optional on the wire: absent = untouched, `null` = cleared, present = new handle. */
  mesh?: LowpolyMeshHandle | null;
}

export interface LowpolyPaintLayersDelta {
  added: LowpolyIndexedPaintLayer[];
  removed: number[];
  patched: LowpolyIndexedPaintLayerPatch[];
  strokes: LowpolyPaintStrokeAt[];
}

export interface LowpolyIndexedPaintLayer {
  index: number;
  layer: LowpolyPaintLayer;
}

export interface LowpolyIndexedPaintLayerPatch {
  index: number;
  patch: LowpolyPaintLayerPatch;
}

export interface LowpolyPaintLayerPatch {
  name?: string;
  visible?: boolean;
  opacity?: number;
  blendMode?: string;
}

export interface LowpolyPaintStrokeAt {
  layerIndex: number;
  runs: PixelRun[];
}

export interface PixelRun {
  offset: number;
  bytes: string;
}

export interface LowpolyArtifact {
  schema: string;
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
