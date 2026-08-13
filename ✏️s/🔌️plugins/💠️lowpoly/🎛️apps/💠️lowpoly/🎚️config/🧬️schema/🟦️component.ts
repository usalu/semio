/** 🧬️ LowpolyConfig */
export interface LowpolyConfig {
  /** @state config */
  activeObjectId: string;
  /** @state config */
  selectionMode: string;
  /** @state config */
  selectionIds: number[];
  /** @state config */
  selectionTargetsMesh: boolean;
  /** @state config */
  selectionTargetsVertex: boolean;
  /** @state config */
  selectionTargetsEdge: boolean;
  /** @state config */
  selectionTargetsFace: boolean;
  /** @state config */
  selectionKeys: string[];
  /** @state config */
  paintUtility: string;
  /** @state config */
  activePaintLayer: number;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  selectionModeDefault: string;
  /** @state config */
  selectedObjectIds: string[];
  /** @state config */
  hoveredObjectId?: string;
  /** @state config */
  hoveredTargetObjectId?: string;
  /** @state config */
  hoveredTargetMode?: string;
  /** @state config */
  hoveredTargetId?: number;
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
  worldCameraPosition: number[];
  /** @state config */
  worldCameraTarget: number[];
  /** @state config */
  worldCameraFov: number;
  /** @state config */
  engagementInput: string;
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
  activeUtilityId: string;
  /** @state config */
  locale: string;
}
