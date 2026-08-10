/** 🧬️ Lowpoly diff schema — sparse field delta. */

export interface LowpolyDiff {
  /** @state persistent */
  artifact?: LowpolyArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  objects?: LowpolyObjectsDelta;
  /** @state shared-ui */
  activeObjectId?: string | null;
  /** @state shared-ui */
  selection?: LowpolySelection;
  /** @state shared-ui */
  selectedObjectIds?: LowpolyStringList;
  /** @state shared-ui */
  paintUtility?: string;
  /** @state shared-ui */
  activePaintLayer?: number;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  showEdges?: boolean;
  /** @state local-ui */
  sunEnabled?: boolean;
  /** @state local-ui */
  sunAzimuth?: number;
  /** @state local-ui */
  sunElevation?: number;
  /** @state local-ui */
  sunIntensity?: number;
  /** @state local-ui */
  sunColor?: string;
  /** @state local-ui */
  worldCameraPositionX?: number;
  /** @state local-ui */
  worldCameraPositionY?: number;
  /** @state local-ui */
  worldCameraPositionZ?: number;
  /** @state local-ui */
  worldCameraTargetX?: number;
  /** @state local-ui */
  worldCameraTargetY?: number;
  /** @state local-ui */
  worldCameraTargetZ?: number;
  /** @state local-ui */
  worldCameraFov?: number;
  /** @state local-ui */
  utilityParamsJson?: string;
  /** @state local-ui */
  paintColorR?: number;
  /** @state local-ui */
  paintColorG?: number;
  /** @state local-ui */
  paintColorB?: number;
  /** @state local-ui */
  paintColorA?: number;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  selectionModeDefault?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  hoveredObjectId?: string | null;
  /** @state preview */
  hoveredTargetObjectId?: string | null;
  /** @state preview */
  hoveredTargetMode?: string | null;
  /** @state preview */
  hoveredTargetId?: number | null;
  /** @state preview */
  strokeDragActive?: boolean;
  /** @state preview */
  transformDragActive?: boolean;
  /** @state preview */
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
}

export interface LowpolyObjectPatch {
  name?: string;
  smoothShading?: boolean;
  transform?: LowpolyTransform;
  meshJson?: string;
  paintLayers?: LowpolyPaintLayersDelta;
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
  meshJson: string;
  paintLayers: LowpolyPaintLayer[];
}
