/** 🧬️ CadPresence */

export interface CadPresence {
  /** @state shared-ui */
  selectedObjectIds: string[];
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  hoveredObjectId?: string;
  /** @state shared-ui */
  hoveredTargetObjectId?: string;
  /** @state shared-ui */
  hoveredTargetMode?: string;
  /** @state shared-ui */
  hoveredTargetId?: number;
  /** @state shared-ui */
  activeObjectId?: string;
  /** @state shared-ui */
  componentSelectionMode: string;
  /** @state shared-ui */
  componentSelectionIds: number[];
  /** @state shared-ui */
  componentSelectionTargetsMesh: boolean;
  /** @state shared-ui */
  componentSelectionTargetsVertex: boolean;
  /** @state shared-ui */
  componentSelectionTargetsEdge: boolean;
  /** @state shared-ui */
  componentSelectionTargetsFace: boolean;
  /** @state shared-ui */
  cameraPosition: number[];
  /** @state shared-ui */
  cameraTarget: number[];
  /** @state shared-ui */
  cameraZoom: number;
  /** @state shared-ui */
  cameraFov: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  engagementStep: string;
  /** @state shared-ui */
  engagementPane?: string;
}
