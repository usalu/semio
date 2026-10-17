/** 🪪️ ProgramSnapshot artifact schema: the program meta, project definition, entity header and the vocabulary every register family shares. */
import { type Binary64 } from "@semio-tech/framework/value/ieee754";

//#region 🔖️Entities
/** 🧬️ Register entity types, mirrored field-for-field from the `🗄️registers/<family>/🦀️.rs` families and
 * `🧱️kernel/🦀️.rs` (`EntityHeader` etc, `#[serde(flatten)]`d onto every register row —
 * hence `extends EntityHeader` below instead of a nested `header` field). `ArtifactRecord` mirrors
 * Rust `ArtifactRecord` (this schema's own field name for that register). */
export interface TextField {
  text: string;
  format?: string;
}

export interface TimestampMeta {
  created: string;
  updated: string;
  createdBy?: string;
  updatedBy?: string;
}

export interface ProgramMeta {
  schema: string;
  documentId: string;
  title: string;
  subtitle: string | null;
  purpose: TextField;
  terminology: string[];
  classification: string[];
  industrySector: string;
  projectType: string;
  locale: string;
  revision: string;
  authorIds: string[];
  sourceSystem: string | null;
  exportProfile: string | null;
  timestamps: TimestampMeta;
}

export type Priority = "mandatory" | "essential" | "preferred" | "optional" | "deferred" | "prohibited";

export interface Ownership {
  ownerId: string | null;
  authorityId: string | null;
  consultantIds: string[];
  participantIds: string[];
}

export interface ProjectDefinition {
  id: string;
  code: string;
  clientName: string;
  ownerOrganization: string;
  briefSummary: TextField;
  problemStatement: TextField;
  vision: TextField;
  mission: TextField;
  objectives: string[];
  successCriteria: string[];
  projectPriorities: Priority[];
  completionCriteria: string[];
  decisionCriteria: string[];
  scopeInclusions: string[];
  scopeExclusions: string[];
  assumptions: string[];
  constraintsSummary: string[];
  dependencies: string[];
  deliverables: string[];
  phases: string[];
  geographicContext: TextField;
  developmentContext: TextField;
  operationalContext: TextField;
  regulatoryContext: string[];
  fundingModel: string;
  ownership: Ownership;
  timestamps: TimestampMeta;
}

export type LifecycleStatus = "draft" | "proposed" | "underReview" | "validated" | "approved" | "rejected" | "deferred" | "superseded" | "archived" | "open" | "closed" | "atRisk" | "blocked" | "inProgress" | "complete";

export interface TaggedNote {
  tag: string;
  text: string;
}

export interface EntityHeader {
  id: string;
  name: string;
  description?: TextField;
  status: LifecycleStatus;
  priority: Priority;
  ownership: Ownership;
  tags: string[];
  notes: TaggedNote[];
  timestamps: TimestampMeta;
}

export interface QuantitySpec {
  min?: Binary64;
  max?: Binary64;
  target?: Binary64;
  current?: Binary64;
  forecast?: Binary64;
  peak?: Binary64;
  average?: Binary64;
  unit: string;
}

export type TraceKind = "objectiveToRequirement" | "stakeholderToRequirement" | "userToActivity" | "activityToFunction" | "functionToProgramElement" | "requirementToDecision" | "requirementToRisk" | "requirementToStandard" | "requirementToValidation" | "requirementToApproval" | "requirementToChange" | "equipmentToActivity" | "processToResource" | "constraintToImpact" | "scenarioToDecision" | "issueToAction" | "actionToOwner" | "decisionToOutcome" | "versionToChange" | "fullAuditTrail";

export interface TraceLink {
  id: string;
  fromId: string;
  toId: string;
  kind: TraceKind;
  label?: string;
}

export type SeparationKind = "acoustic" | "visual" | "security" | "olfactory" | "thermal" | "fire" | "hygienic" | "circulation" | "operational" | "infectionControl";

export type ValidationStatus = "pending" | "passed" | "failed" | "waived" | "deferred";

export type AccessLevel = "public" | "restricted" | "controlled" | "private" | "secure" | "emergencyOnly";

export type RiskLevel = "negligible" | "low" | "medium" | "high" | "critical";

export type DeliveryPhase = "concept" | "schematic" | "designDevelopment" | "constructionDocuments" | "procurement" | "construction" | "commissioning" | "occupancy";

export type IssueSeverity = "cosmetic" | "minor" | "major" | "critical" | "blocker";
//#endregion 🔖️Entities
