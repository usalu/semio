/** 🧬️ Process3d artifact schema — every field with its state class. */

export interface Process3dArtifact {
  /** @state persistent */
  workshop: Process3dWorkshop;
  /** @state persistent */
  stock: Process3dStock;
  /** @state persistent */
  steps: ProcessStep[];
  /** @state persistent */
  resolvedUpTo?: number;
  /** @state shared-ui */
  selectedId?: string;
  /** @state shared-ui */
  selectedFaceId?: number;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  cameraPositionX: number;
  /** @state local-ui */
  cameraPositionY: number;
  /** @state local-ui */
  cameraPositionZ: number;
  /** @state local-ui */
  cameraTargetX: number;
  /** @state local-ui */
  cameraTargetY: number;
  /** @state local-ui */
  cameraTargetZ: number;
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
  locale: string;
  /** @state local-ui */
  contributionsJson: string;
  /** @state preview */
  hoveredId?: string;
}

export interface Process3dWorkshop { machines: Process3dWorkshopMachine[]; }
export interface Process3dWorkshopMachine { id: string; label: string; iconId: string; catalogId?: string; capabilities: Process3dCapability[]; }
export interface Process3dCapability { id: string; label: string; iconId: string; recipe: Record<string, unknown>; parameters: Process3dCapabilityParameter[]; rules: Record<string, unknown>[]; }
export interface Process3dCapabilityParameter { id: string; label: string; value: number; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Process3dPose; }
export interface Process3dPose { position: [number, number, number]; axis: [number, number, number]; angle: number; }
export interface Process3dStep { id: string; label: string; enabled: boolean; origin?: Process3dStepOrigin; measure: Record<string, unknown>; }
export interface Process3dStepOrigin { machineId: string; capabilityId: string; }
