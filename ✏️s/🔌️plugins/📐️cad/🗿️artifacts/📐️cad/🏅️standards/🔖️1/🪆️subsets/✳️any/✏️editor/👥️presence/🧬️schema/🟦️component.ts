/** 🧬️ CadPresence */

export interface CadPresence {
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  hoveredObjectId?: string;
  /** @state presence */
  hoveredTargetObjectId?: string;
  /** @state presence */
  hoveredTargetMode?: string;
  /** @state presence */
  hoveredTargetId?: number;
  /** @state presence */
  activeObjectId?: string;
  /** @state presence */
  componentSelectionMode: string;
  /** @state presence */
  componentSelectionIds: number[];
  /** @state presence */
  componentSelectionTargetsMesh: boolean;
  /** @state presence */
  componentSelectionTargetsVertex: boolean;
  /** @state presence */
  componentSelectionTargetsEdge: boolean;
  /** @state presence */
  componentSelectionTargetsFace: boolean;
  /** @state presence */
  cameraPosition: number[];
  /** @state presence */
  cameraTarget: number[];
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  cameraFov: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  engagementStep: string;
  /** @state presence */
  engagementPane?: string;
}
