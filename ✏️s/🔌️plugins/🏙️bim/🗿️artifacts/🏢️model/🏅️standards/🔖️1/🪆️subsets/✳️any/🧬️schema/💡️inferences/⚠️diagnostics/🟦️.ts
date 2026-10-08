/** ⚠️ `diagnostics`: every finding of the model with a severity, a stable code, the element ids involved, the English and German message key and the numbers of the message. */

export type Severity = "Info" | "Warning" | "Error";

export type DiagnosticCode = "ClashWallWall" | "ClashWallColumn" | "ClashColumnColumn" | "ClashWallBeam" | "ClashBeamColumn" | "ClashBeamBeam" | "ClashBeamSlab" | "ClashStairWall" | "ClashStairColumn" | "ClashStairBeam" | "ClashStairStair" | "ClashSlabSlab" | "RefWallType" | "RefColumnType" | "RefBeamType" | "RefSlabType" | "RefRoofType" | "RefWindowType" | "RefDoorType" | "RefTopStorey" | "RefOpeningHost" | "RefElementStorey" | "RefStoreyBuilding" | "RefBuildingSite" | "RefGridBuilding" | "RefLayerMaterial" | "RefTypeMaterial" | "RefPropertyElement" | "DuplicateId" | "OpeningOutsideHost" | "OpeningBelowBase" | "OpeningAboveTop" | "OpeningOverlap" | "OpeningSize" | "DegenerateAxis" | "DegenerateThickness" | "DegenerateHeight" | "DegenerateProfile" | "DegenerateLoop" | "SelfIntersectingLoop" | "DegeneratePath" | "NonFinite" | "DegenerateSpacing" | "DegenerateStorey" | "StoreyLevelGap" | "StoreyLevelDuplicate" | "StoreyNoDatum" | "StairNoRise" | "StairRiserHeight" | "StairTreadDepth" | "StairComfort" | "SpaceNotEnclosed" | "SpaceSeedInWall" | "SpaceDuplicateNumber";

export interface Diagnostic {
  code: DiagnosticCode;
  severity: Severity;
  message_key: string;
  elements: string[];
  missing: string[];
  storey?: string;
  values: Record<string, number>;
}
