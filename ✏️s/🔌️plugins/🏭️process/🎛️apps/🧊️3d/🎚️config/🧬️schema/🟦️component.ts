/** 🧬️ Process3dConfig */
export interface Process3dConfig {
  /** @state local-ui */
  selectedId?: string;
  /** @state local-ui */
  hoveredId?: string;
  /** @state local-ui */
  selectedFaceId?: number;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  cameraPosition: number[];
  /** @state local-ui */
  cameraTarget: number[];
  /** @state local-ui */
  cameraFov: number;
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
  /** @state local-ui */
  contributionsJson: string;
}
