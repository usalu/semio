/** 🧬️ Process3dPresence */
export interface Process3dPresence {
  /** @state shared-ui */
  selectedId?: string;
  /** @state shared-ui */
  hoveredId?: string;
  /** @state shared-ui */
  selectedFaceId?: number;
  /** @state shared-ui */
  selectionMethod: string;
  /** @state shared-ui */
  engagementInput: string;
  /** @state shared-ui */
  cameraPosition: number[];
  /** @state shared-ui */
  cameraTarget: number[];
  /** @state shared-ui */
  cameraFov: number;
  /** @state shared-ui */
  activeUtilityId: string;
}
