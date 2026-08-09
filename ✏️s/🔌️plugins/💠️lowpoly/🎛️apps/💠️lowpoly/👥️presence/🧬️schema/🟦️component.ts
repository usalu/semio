/** 🧬️ LowpolyPresence */
export interface LowpolyPresence {
  /** @state shared-ui */
  selectionMode: string;
  /** @state shared-ui */
  selectionIds: number[];
  /** @state shared-ui */
  selectionTargetsMesh: boolean;
  /** @state shared-ui */
  selectionTargetsVertex: boolean;
  /** @state shared-ui */
  selectionTargetsEdge: boolean;
  /** @state shared-ui */
  selectionTargetsFace: boolean;
  /** @state shared-ui */
  selectedObjectIds: string[];
  /** @state shared-ui */
  hoveredObjectId?: string;
  /** @state shared-ui */
  hoveredTargetObjectId?: string;
  /** @state shared-ui */
  hoveredTargetMode?: string;
  /** @state shared-ui */
  hoveredTargetId?: number;
  /** @state shared-ui */
  worldCameraPosition: number[];
  /** @state shared-ui */
  worldCameraTarget: number[];
  /** @state shared-ui */
  worldCameraFov: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  paintUtility: string;
}
