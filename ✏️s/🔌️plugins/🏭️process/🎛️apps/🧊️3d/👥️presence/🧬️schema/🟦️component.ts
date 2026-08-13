/** 🧬️ Process3dPresence */
export interface Process3dPresence {
  /** @state presence */
  selectedId?: string;
  /** @state presence */
  hoveredId?: string;
  /** @state presence */
  selectedFaceId?: number;
  /** @state presence */
  selectionMethod: string;
  /** @state presence */
  engagementInput: string;
  /** @state presence */
  cameraPosition: number[];
  /** @state presence */
  cameraTarget: number[];
  /** @state presence */
  cameraFov: number;
  /** @state presence */
  activeUtilityId: string;
}
