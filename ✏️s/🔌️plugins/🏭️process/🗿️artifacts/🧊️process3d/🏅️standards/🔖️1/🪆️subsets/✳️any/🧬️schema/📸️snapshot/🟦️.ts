/** 🧬️ Process3d snapshot schema — artifact-lane fields only. */

export interface Process3dSnapshot {
  /** @state artifact */
  workshop: Process3dWorkshop;
  /** @state artifact */
  stockId: string;
  stockLabel: string;
  stockPose: Process3dPose;
  stockPayload: Process3dStock;
  stockSolid: ArtifactChildHandle;
  /** @state artifact */
  steps: ArtifactChildHandle;
  stepPayloads: Process3dStep[];
  toolSolids: ArtifactChildHandle[];
  /** @state artifact */
  resolvedUpTo?: number;
}

export interface Process3dWorkshop { machines: Process3dWorkshopMachine[]; }
export interface Process3dWorkshopMachine { id: string; label: string; iconId: string; catalogId?: string; capabilities: unknown[]; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Process3dPose; }
export interface Process3dPose { position: [number, number, number]; axis: [number, number, number]; angle: number; }
export interface Process3dStep { id: string; label: string; enabled: boolean; origin?: { machineId: string; capabilityId: string }; measure: Record<string, unknown>; }
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}
