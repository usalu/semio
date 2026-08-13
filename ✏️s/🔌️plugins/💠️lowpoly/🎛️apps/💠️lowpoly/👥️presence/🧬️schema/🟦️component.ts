/** 🧬️ LowpolyPresence */
export interface LowpolyPresence {
  /** @state presence */
  selectionMode: string;
  /** @state presence */
  selectionIds: number[];
  /** @state presence */
  selectionTargetsMesh: boolean;
  /** @state presence */
  selectionTargetsVertex: boolean;
  /** @state presence */
  selectionTargetsEdge: boolean;
  /** @state presence */
  selectionTargetsFace: boolean;
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  hoveredObjectId?: string;
  /** @state presence */
  hoveredTargetObjectId?: string;
  /** @state presence */
  hoveredTargetMode?: string;
  /** @state presence */
  hoveredTargetId?: number;
  /** @state presence */
  worldCameraPosition: number[];
  /** @state presence */
  worldCameraTarget: number[];
  /** @state presence */
  worldCameraFov: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  paintUtility: string;
}
