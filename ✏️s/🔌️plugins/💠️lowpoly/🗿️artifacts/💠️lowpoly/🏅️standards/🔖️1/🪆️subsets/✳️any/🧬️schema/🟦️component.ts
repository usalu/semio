/** 🧬️ Lowpoly artifact schema — every field with its state class. */

export interface LowpolyArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objects: LowpolyObject[];
  /** @state presence */
  activeObjectId?: string;
  /** @state presence */
  selection: LowpolySelection;
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  paintUtility: string;
  /** @state presence */
  activePaintLayer: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state config */
  showEdges: boolean;
  /** @state config */
  sunEnabled: boolean;
  /** @state config */
  sunAzimuth: number;
  /** @state config */
  sunElevation: number;
  /** @state config */
  sunIntensity: number;
  /** @state config */
  sunColor: string;
  /** @state config */
  worldCameraPositionX: number;
  /** @state config */
  worldCameraPositionY: number;
  /** @state config */
  worldCameraPositionZ: number;
  /** @state config */
  worldCameraTargetX: number;
  /** @state config */
  worldCameraTargetY: number;
  /** @state config */
  worldCameraTargetZ: number;
  /** @state config */
  worldCameraFov: number;
  /** @state config */
  utilityParamsJson: string;
  /** @state config */
  paintColorR: number;
  /** @state config */
  paintColorG: number;
  /** @state config */
  paintColorB: number;
  /** @state config */
  paintColorA: number;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  selectionModeDefault: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  locale: string;
  /** @state artifact */
  hoveredObjectId?: string;
  /** @state artifact */
  hoveredTargetObjectId?: string;
  /** @state artifact */
  hoveredTargetMode?: string;
  /** @state artifact */
  hoveredTargetId?: number;
  /** @state artifact */
  strokeDragActive: boolean;
  /** @state artifact */
  transformDragActive: boolean;
  /** @state artifact */
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
