/** 🧬️ Process3d diff schema — sparse field delta over the artifact. */

export interface Process3dDiff {
  /** @state artifact */
  artifact?: Process3dArtifact;
  /** @state artifact */
  workshop?: Process3dWorkshop;
  /** @state artifact */
  stockId?: string;
  stockLabel?: string;
  stockPose?: Record<string, unknown>;
  stockPayload?: Process3dStock;
  stockSolid?: ArtifactChildHandle;
  /** @state artifact */
  steps?: ArtifactChildHandle;
  stepPayloads?: Process3dStep[];
  toolSolids?: ArtifactChildHandle[];
  /** @state artifact */
  resolvedUpTo?: number | null;
  /** @state presence */
  selectedId?: string | null;
  /** @state presence */
  selectedFaceId?: number | null;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  cameraPositionX?: number;
  /** @state config */
  cameraPositionY?: number;
  /** @state config */
  cameraPositionZ?: number;
  /** @state config */
  cameraTargetX?: number;
  /** @state config */
  cameraTargetY?: number;
  /** @state config */
  cameraTargetZ?: number;
  /** @state config */
  cameraFov?: number;
  /** @state config */
  sunEnabled?: boolean;
  /** @state config */
  sunAzimuth?: number;
  /** @state config */
  sunElevation?: number;
  /** @state config */
  sunIntensity?: number;
  /** @state config */
  sunColor?: string;
  /** @state config */
  locale?: string;
  /** @state config */
  contributionsJson?: string;
  /** @state artifact */
  hoveredId?: string | null;
}

export interface Process3dArtifact {
  workshop: Process3dWorkshop;
  stockId: string;
  stockLabel: string;
  stockPose: Record<string, unknown>;
  stockPayload: Process3dStock;
  stockSolid: ArtifactChildHandle;
  steps: ArtifactChildHandle;
  stepPayloads: Process3dStep[];
  toolSolids: ArtifactChildHandle[];
  resolvedUpTo?: number;
}
export interface Process3dWorkshop { machines: unknown[]; }
export interface Process3dStock { id: string; label: string; solid: Record<string, unknown>; pose: Record<string, unknown>; }
export interface Process3dStep { id: string; label: string; enabled: boolean; }
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
export interface Process3dStepsDelta {
  added: Process3dStep[];
  removed: string[];
  patched: Process3dStepPatchEntry[];
  reordered?: string[];
}
export interface Process3dStepPatchEntry { id: string; patch: Record<string, unknown>; }
