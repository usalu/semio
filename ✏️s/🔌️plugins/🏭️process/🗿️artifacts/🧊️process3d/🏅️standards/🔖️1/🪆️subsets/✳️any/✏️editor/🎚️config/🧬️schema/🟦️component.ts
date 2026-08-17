/** 🧬️ Process3dConfig */
export interface Process3dConfig {
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraPosition: number[];
  /** @state config */
  cameraTarget: number[];
  /** @state config */
  cameraFov: number;
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
  /** @state config */
  contributionsJson: string;
}
