/** ⚠️ `diagnostics`: every finding of the model with a severity, a stable code, the element ids involved, the English and German message key and the numbers of the message. */

export type Severity = "Info" | "Warning" | "Error";

export type DiagnosticCode = "ClashWallWall" | "ClashWallColumn" | "ClashColumnColumn" | "ClashWallBeam" | "ClashBeamColumn" | "ClashBeamBeam" | "ClashBeamSlab" | "ClashStairWall" | "ClashStairColumn" | "ClashStairBeam" | "ClashStairStair" | "ClashSlabSlab" | "RefWallType" | "RefColumnType" | "RefBeamType" | "RefSlabType" | "RefRoofType" | "RefWindowType" | "RefDoorType" | "RefTopStorey" | "RefOpeningHost" | "RefElementStorey" | "RefStoreyBuilding" | "RefBuildingSite" | "RefGridBuilding" | "RefLayerMaterial" | "RefTypeMaterial" | "RefPropertyElement" | "DuplicateId" | "OpeningOutsideHost" | "OpeningBelowBase" | "OpeningAboveTop" | "OpeningOverlap" | "OpeningSize" | "OpeningOutsideTrimmed" | "DegenerateAxis" | "DegenerateThickness" | "DegenerateHeight" | "DegenerateProfile" | "DegenerateLoop" | "SelfIntersectingLoop" | "DegeneratePath" | "NonFinite" | "DegenerateSpacing" | "DegenerateStorey" | "StoreyLevelGap" | "StoreyLevelDuplicate" | "StoreyNoDatum" | "StairNoRise" | "StairRiserHeight" | "StairTreadDepth" | "StairComfort" | "StairStringerIgnored" | "SpaceNotEnclosed" | "SpaceSeedInWall" | "SpaceDuplicateNumber" | "RoofFlatCurved" | "RoofFlatSkeleton" | "RoofFlatDegenerate" | "RoofFlatPitch" | "RoofOverhangCollapsed" | "RoofGableToHip" | "AnnotationAnchorMissing" | "AnnotationAnchorUnresolved" | "AnnotationStyleMissing" | "DimensionZero" | "DimensionLockViolated" | "TagEmpty" | "ClashBeamCeiling" | "ClashCeilingCeiling" | "RefCeilingType" | "CurtainOverrideOutOfGrid" | "CurtainDoorNotAtBase" | "CurtainGridLineOutside" | "CurtainDuplicateOverride" | "RefCurtainWallType" | "RefCurtainPanel" | "RefCurtainOverrideHost" | "ColumnTiltInvalid" | "CeilingOutsideStorey" | "RampSlope" | "RampNoRun" | "RefRailingHost" | "RailingHostUnresolved";

export interface SeverityCounts {
  error: number;
  warning: number;
  info: number;
}

export interface ElementFindings {
  severity: Severity;
  count: number;
  codes: DiagnosticCode[];
}

export interface DiagnosticIndex {
  total: SeverityCounts;
  elements: Record<string, ElementFindings>;
  categories: Record<string, SeverityCounts>;
  codes: Record<string, number>;
  storeys: Record<string, SeverityCounts>;
}

export interface Diagnostic {
  code: DiagnosticCode;
  severity: Severity;
  message_key: string;
  elements: string[];
  missing: string[];
  storey?: string;
  values: Record<string, number>;
}
