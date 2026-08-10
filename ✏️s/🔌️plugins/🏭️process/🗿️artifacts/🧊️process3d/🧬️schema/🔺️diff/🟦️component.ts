/** 🧬️ Process3d diff schema — sparse field delta over the artifact. */

export interface Process3dDiff {
  /** @state persistent */
  artifact?: Process3dArtifact;
  /** @state persistent */
  workshop?: Process3dWorkshop;
  /** @state persistent */
  stock?: Process3dStock;
  /** @state persistent */
  steps?: Process3dStepsDelta;
  /** @state persistent */
  resolvedUpTo?: number | null;
  /** @state shared-ui */
  selectedId?: string | null;
  /** @state shared-ui */
  selectedFaceId?: number | null;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  cameraPositionX?: number;
  /** @state local-ui */
  cameraPositionY?: number;
  /** @state local-ui */
  cameraPositionZ?: number;
  /** @state local-ui */
  cameraTargetX?: number;
  /** @state local-ui */
  cameraTargetY?: number;
  /** @state local-ui */
  cameraTargetZ?: number;
  /** @state local-ui */
  cameraFov?: number;
  /** @state local-ui */
  sunEnabled?: boolean;
  /** @state local-ui */
  sunAzimuth?: number;
  /** @state local-ui */
  sunElevation?: number;
  /** @state local-ui */
  sunIntensity?: number;
  /** @state local-ui */
  sunColor?: string;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
  contributionsJson?: string;
  /** @state preview */
  hoveredId?: string | null;
}

export interface Process3dArtifact { workshop: Process3dWorkshop; stock: Process3dStock; steps: Process3dStep[]; resolvedUpTo?: number; }
export interface Process3dWorkshop { machines: unknown[]; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Record<string, unknown>; }
export interface Process3dStep { id: string; label: string; enabled: boolean; }
export interface Process3dStepsDelta {
  added: Process3dStep[];
  removed: string[];
  patched: Process3dStepPatchEntry[];
  reordered?: string[];
}
export interface Process3dStepPatchEntry { id: string; patch: Record<string, unknown>; }
