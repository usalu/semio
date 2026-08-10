/** 🧬️ Process3d snapshot schema — persistent fields only. */

export interface Process3dSnapshot {
  /** @state persistent */
  workshop: Process3dWorkshop;
  /** @state persistent */
  stock: Process3dStock;
  /** @state persistent */
  steps: Process3dStep[];
  /** @state persistent */
  resolvedUpTo?: number;
}

export interface Process3dWorkshop { machines: Process3dWorkshopMachine[]; }
export interface Process3dWorkshopMachine { id: string; label: string; iconId: string; catalogId?: string; capabilities: unknown[]; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Process3dPose; }
export interface Process3dPose { position: [number, number, number]; axis: [number, number, number]; angle: number; }
export interface Process3dStep { id: string; label: string; enabled: boolean; origin?: { machineId: string; capabilityId: string }; measure: Record<string, unknown>; }
