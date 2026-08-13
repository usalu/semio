/** 🧬️ Process3d snapshot schema — persistent fields only. */

export interface Process3dSnapshot {
  /** @state artifact */
  workshop: Process3dWorkshop;
  /** @state artifact */
  stock: Process3dStock;
  /** @state artifact */
  steps: Process3dStep[];
  /** @state artifact */
  resolvedUpTo?: number;
}

export interface Process3dWorkshop { machines: Process3dWorkshopMachine[]; }
export interface Process3dWorkshopMachine { id: string; label: string; iconId: string; catalogId?: string; capabilities: unknown[]; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Process3dPose; }
export interface Process3dPose { position: [number, number, number]; axis: [number, number, number]; angle: number; }
export interface Process3dStep { id: string; label: string; enabled: boolean; origin?: { machineId: string; capabilityId: string }; measure: Record<string, unknown>; }
