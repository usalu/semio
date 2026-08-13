/** 🧬️ Process3d artifact schema — every field with its state class. */

export interface Process3dArtifact {
  /** @state artifact */
  workshop: Process3dWorkshop;
  /** @state artifact */
  stock: Process3dStock;
  /** @state artifact */
  steps: ProcessStep[];
  /** @state artifact */
  resolvedUpTo?: number;
  /** @state presence */
  selectedId?: string;
  /** @state presence */
  selectedFaceId?: number;
  /** @state presence */
  activeUtilityId: string;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  cameraPositionX: number;
  /** @state config */
  cameraPositionY: number;
  /** @state config */
  cameraPositionZ: number;
  /** @state config */
  cameraTargetX: number;
  /** @state config */
  cameraTargetY: number;
  /** @state config */
  cameraTargetZ: number;
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
  locale: string;
  /** @state config */
  contributionsJson: string;
  /** @state artifact */
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
