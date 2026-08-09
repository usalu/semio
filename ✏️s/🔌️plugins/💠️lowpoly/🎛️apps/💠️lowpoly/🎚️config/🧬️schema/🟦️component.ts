/** 🧬️ LowpolyConfig */
export interface LowpolyConfig {
  /** @state local-ui */
  activeObjectId: string;
  /** @state local-ui */
  selectionMode: string;
  /** @state local-ui */
  selectionIds: number[];
  /** @state local-ui */
  selectionTargetsMesh: boolean;
  /** @state local-ui */
  selectionTargetsVertex: boolean;
  /** @state local-ui */
  selectionTargetsEdge: boolean;
  /** @state local-ui */
  selectionTargetsFace: boolean;
  /** @state local-ui */
  selectionKeys: string[];
  /** @state local-ui */
  paintUtility: string;
  /** @state local-ui */
  activePaintLayer: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  selectionModeDefault: string;
  /** @state local-ui */
  selectedObjectIds: string[];
  /** @state local-ui */
  hoveredObjectId?: string;
  /** @state local-ui */
  hoveredTargetObjectId?: string;
  /** @state local-ui */
  hoveredTargetMode?: string;
  /** @state local-ui */
  hoveredTargetId?: number;
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
  worldCameraPosition: number[];
  /** @state local-ui */
  worldCameraTarget: number[];
  /** @state local-ui */
  worldCameraFov: number;
  /** @state local-ui */
  engagementInput: string;
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
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
}
