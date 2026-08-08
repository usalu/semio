/** 🧬️ Lowpoly artifact schema — every field with its state class. */

export interface LowpolyArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  objects: LowpolyObject[];
  /** @state shared-ui */
  activeObjectId?: string;
  /** @state shared-ui */
  selection: LowpolySelection;
  /** @state shared-ui */
  selectedObjectIds: string[];
  /** @state shared-ui */
  paintUtility: string;
  /** @state shared-ui */
  activePaintLayer: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  showEdges: boolean;
  /** @state local-ui */
  sunEnabled: boolean;
  /** @state local-ui */
  sunAzimuth: number;
  /** @state local-ui */
  sunElevation: number;
  /** @state local-ui */
  sunIntensity: number;
  /** @state local-ui */
  sunColor: string;
  /** @state local-ui */
  worldCameraPositionX: number;
  /** @state local-ui */
  worldCameraPositionY: number;
  /** @state local-ui */
  worldCameraPositionZ: number;
  /** @state local-ui */
  worldCameraTargetX: number;
  /** @state local-ui */
  worldCameraTargetY: number;
  /** @state local-ui */
  worldCameraTargetZ: number;
  /** @state local-ui */
  worldCameraFov: number;
  /** @state local-ui */
  utilityParamsJson: string;
  /** @state local-ui */
  paintColorR: number;
  /** @state local-ui */
  paintColorG: number;
  /** @state local-ui */
  paintColorB: number;
  /** @state local-ui */
  paintColorA: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  selectionModeDefault: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  hoveredObjectId?: string;
  /** @state preview */
  hoveredTargetObjectId?: string;
  /** @state preview */
  hoveredTargetMode?: string;
  /** @state preview */
  hoveredTargetId?: number;
  /** @state preview */
  strokeDragActive: boolean;
  /** @state preview */
  transformDragActive: boolean;
  /** @state preview */
  previewSeq: number;
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
