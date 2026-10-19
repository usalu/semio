/** 🧬️ ProgramSnapshot artifact schema — every field with its state class. */

import { parseArtifactChild, type ArtifactChild } from "@semio-tech/framework/schema/artifact-reference";
import { parseSchemaRecord } from "@semio-tech/framework/schema/record";
import { parseBinary64, type Binary64 } from "@semio-tech/framework/value/ieee754";
import type { ProgramMeta, ProjectDefinition, QuantitySpec, TraceLink } from "./🪪️entity-foundation/🟦️.ts";
import type { AccessRule, Activity, Adjacency, AdjacencyKind, FlowRequirement, Function, Process, ProgramElement, QuantityRequirement, Relationship, Stakeholder, UserProfile } from "./🏗️program-structure/🟦️.ts";
import type { AccessibilityRequirement, CommunicationRequirement, CostRequirement, DeliveryConstraint, EnvironmentalRequirement, Equipment, FlexibilityRequirement, GrowthPlan, HumanFactorRequirement, InformationRequirement, InfrastructureRequirement, OperationalRequirement, OrganizationalRequirement, PrivacyRequirement, RegulatoryRequirement, ResilienceRequirement, Resource, SafetyRequirement, ScheduleRequirement, SecurityRequirement, ServiceRequirement, SiteContext, StorageRequirement, SustainabilityRequirement, WayfindingRequirement } from "./📋️requirements/🟦️.ts";
import type { AnalysisRecord, ApprovalRecord, ArtifactRecord, Assumption, AuditEvent, BenchmarkRecord, ChangeRecord, CollaborationRecord, ComplianceRecord, Conflict, ConstraintRecord, Decision, Governance, Issue, KnowledgeRecord, MeetingRecord, OptionEvaluation, PerformanceCriterion, PriorityRecord, QualityRecord, ReportRecord, Requirement, Risk, Scenario, SearchFilter, StatusRecord, Survey, TemplateRecord, ValidationRecord, Workshop } from "./🗃️decision-records/🟦️.ts";
export { parseArtifactChild, type ArtifactChild } from "@semio-tech/framework/schema/artifact-reference";
export type { RegisterKey, RegisterRow } from "./🗄️registers/🗂️catalog/🟦️.ts";

export const PROGRAM_ARTIFACT_FIELDS = [
  "schema", "meta", "project", "stakeholders", "users", "activities", "functions", "elements", "quantities", "relationships", "adjacencies", "processes", "flows", "accessRules", "operations", "equipment", "resources", "storage", "environmental", "humanFactors", "accessibility", "privacy", "safety", "security", "regulatory", "siteContext", "organizational", "services", "infrastructure", "information", "communication", "wayfinding", "schedules", "flexibility", "growth", "sustainability", "resilience", "costs", "delivery", "risks", "conflicts", "requirements", "priorities", "scenarios", "options", "decisions", "validations", "performance", "quality", "artifacts", "assumptions", "constraints", "complianceRecords", "approvals", "meetings", "changes", "collaboration", "analyses", "reports", "searchFilters", "statusRecords", "workshops", "surveys", "issues", "auditEvents", "templates", "knowledgePayload", "knowledge", "benchmarksPayload", "benchmarks", "traces", "governance",
] as const;

export interface ProgramArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  meta: ProgramMeta;
  /** @state artifact */
  project: ProjectDefinition;
  /** @state artifact */
  stakeholders: Stakeholder[];
  /** @state artifact */
  users: UserProfile[];
  /** @state artifact */
  activities: Activity[];
  /** @state artifact */
  functions: Function[];
  /** @state artifact */
  elements: ProgramElement[];
  /** @state artifact */
  quantities: QuantityRequirement[];
  /** @state artifact */
  relationships: Relationship[];
  /** @state artifact */
  adjacencies: Adjacency[];
  /** @state artifact */
  processes: Process[];
  /** @state artifact */
  flows: FlowRequirement[];
  /** @state artifact */
  accessRules: AccessRule[];
  /** @state artifact */
  operations: OperationalRequirement[];
  /** @state artifact */
  equipment: Equipment[];
  /** @state artifact */
  resources: Resource[];
  /** @state artifact */
  storage: StorageRequirement[];
  /** @state artifact */
  environmental: EnvironmentalRequirement[];
  /** @state artifact */
  humanFactors: HumanFactorRequirement[];
  /** @state artifact */
  accessibility: AccessibilityRequirement[];
  /** @state artifact */
  privacy: PrivacyRequirement[];
  /** @state artifact */
  safety: SafetyRequirement[];
  /** @state artifact */
  security: SecurityRequirement[];
  /** @state artifact */
  regulatory: RegulatoryRequirement[];
  /** @state artifact */
  siteContext: SiteContext[];
  /** @state artifact */
  organizational: OrganizationalRequirement[];
  /** @state artifact */
  services: ServiceRequirement[];
  /** @state artifact */
  infrastructure: InfrastructureRequirement[];
  /** @state artifact */
  information: InformationRequirement[];
  /** @state artifact */
  communication: CommunicationRequirement[];
  /** @state artifact */
  wayfinding: WayfindingRequirement[];
  /** @state artifact */
  schedules: ScheduleRequirement[];
  /** @state artifact */
  flexibility: FlexibilityRequirement[];
  /** @state artifact */
  growth: GrowthPlan[];
  /** @state artifact */
  sustainability: SustainabilityRequirement[];
  /** @state artifact */
  resilience: ResilienceRequirement[];
  /** @state artifact */
  costs: CostRequirement[];
  /** @state artifact */
  delivery: DeliveryConstraint[];
  /** @state artifact */
  risks: Risk[];
  /** @state artifact */
  conflicts: Conflict[];
  /** @state artifact */
  requirements: Requirement[];
  /** @state artifact */
  priorities: PriorityRecord[];
  /** @state artifact */
  scenarios: Scenario[];
  /** @state artifact */
  options: OptionEvaluation[];
  /** @state artifact */
  decisions: Decision[];
  /** @state artifact */
  validations: ValidationRecord[];
  /** @state artifact */
  performance: PerformanceCriterion[];
  /** @state artifact */
  quality: QualityRecord[];
  /** @state artifact */
  artifacts: ArtifactRecord[];
  /** @state artifact */
  assumptions: Assumption[];
  /** @state artifact */
  constraints: ConstraintRecord[];
  /** @state artifact */
  complianceRecords: ComplianceRecord[];
  /** @state artifact */
  approvals: ApprovalRecord[];
  /** @state artifact */
  meetings: MeetingRecord[];
  /** @state artifact */
  changes: ChangeRecord[];
  /** @state artifact */
  collaboration: CollaborationRecord[];
  /** @state artifact */
  analyses: AnalysisRecord[];
  /** @state artifact */
  reports: ReportRecord[];
  /** @state artifact */
  searchFilters: SearchFilter[];
  /** @state artifact */
  statusRecords: StatusRecord[];
  /** @state artifact */
  workshops: Workshop[];
  /** @state artifact */
  surveys: Survey[];
  /** @state artifact */
  issues: Issue[];
  /** @state artifact */
  auditEvents: AuditEvent[];
  /** @state artifact */
  templates: TemplateRecord[];
  /** @state artifact */
  knowledgePayload: KnowledgeRecord[];
  /** @state artifact */
  knowledge: ArtifactChild;
  /** @state artifact */
  benchmarksPayload: BenchmarkRecord[];
  /** @state artifact */
  benchmarks: ArtifactChild;
  /** @state artifact */
  traces: TraceLink[];
  /** @state artifact */
  governance: Governance;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramArtifactGuardReject = (at: string, why: string): never => {
  throw new architectProgramArtifactGuardRefusal(at, why);
};

type architectProgramArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramArtifactGuardReject(at, "value is not an object");
export const architectProgramArtifactGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = parseSchemaRecord(value, fields, at);
  for (const field of required) if (!Object.hasOwn(row, field)) architectProgramArtifactGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const architectProgramArtifactGuardArray = (value: unknown, at: string, bounds: architectProgramArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramArtifactGuardString = (value: unknown, at: string, bounds: architectProgramArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramArtifactGuardReject(at, "value is not a boolean"));
export const architectProgramArtifactGuardNumber = (value: unknown, at: string, bounds: architectProgramArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramArtifactGuardInteger = (value: unknown, at: string, bounds: architectProgramArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramArtifactGuardNumber(value, at, bounds) : architectProgramArtifactGuardReject(at, "value is not an integer");
/** 🔢️ Admits the canonical complete native unsigned64 domain without coercion. */
export const architectProgramArtifactGuardUnsigned64 = (value:unknown,at:string):bigint =>
  typeof value==="bigint"&&value>=0n&&value<=18446744073709551615n?value:architectProgramArtifactGuardReject(at,"value is not an unsigned64 bigint");
/** 🧮️ Canonical native doubles own exact words independently of JSON numeric transport. */
export function architectProgramArtifactGuardBinary64(value:unknown,at:string):Binary64 {
  const row=architectProgramArtifactGuardExactObject(value,at,["bits"]);
  try{return parseBinary64(row);}catch{return architectProgramArtifactGuardReject(at,"value is not an exact binary64 word");}
}
function nullableBinary64(value:unknown,at:string):Binary64|null{return value===null?null:architectProgramArtifactGuardBinary64(value,at);}
/** 📏️ Admit each literal quantity field as a bit-backed optional native double. */
export function parseQuantitySpec(value:unknown,at="$"):QuantitySpec{
  const row=architectProgramArtifactGuardExactObject(value,at,["min","max","target","current","forecast","peak","average","unit"],["unit"]);
  const out:QuantitySpec={unit:architectProgramArtifactGuardString(row.unit,at+".unit")};
  if(row.min!==undefined)out.min=architectProgramArtifactGuardBinary64(row.min,at+".min");
  if(row.max!==undefined)out.max=architectProgramArtifactGuardBinary64(row.max,at+".max");
  if(row.target!==undefined)out.target=architectProgramArtifactGuardBinary64(row.target,at+".target");
  if(row.current!==undefined)out.current=architectProgramArtifactGuardBinary64(row.current,at+".current");
  if(row.forecast!==undefined)out.forecast=architectProgramArtifactGuardBinary64(row.forecast,at+".forecast");
  if(row.peak!==undefined)out.peak=architectProgramArtifactGuardBinary64(row.peak,at+".peak");
  if(row.average!==undefined)out.average=architectProgramArtifactGuardBinary64(row.average,at+".average");
  return out;
}
export const architectProgramArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramArtifactGuardReject(at, `value is not ${String(expected)}`);

//#endregion 🚪️Parsers
function architectProgramArtifactGuardHeaderObject(value:unknown,at:string,fields:readonly string[],required:readonly string[]):Readonly<Record<string,unknown>>{
 const row=architectProgramArtifactGuardExactObject(value,at,fields,[...required,"tags","notes"]);
 architectProgramArtifactGuardArray(row.tags,at+".tags").forEach((item,index)=>architectProgramArtifactGuardString(item,at+".tags["+index+"]"));
 architectProgramArtifactGuardArray(row.notes,at+".notes").forEach((item,index)=>{const note=architectProgramArtifactGuardExactObject(item,at+".notes["+index+"]",["tag","text"]);architectProgramArtifactGuardString(note.tag,at+".notes["+index+"].tag");architectProgramArtifactGuardString(note.text,at+".notes["+index+"].text")});
 if(row.description!==undefined){const text=architectProgramArtifactGuardExactObject(row.description,at+".description",["text","format"],["text"]);architectProgramArtifactGuardString(text.text,at+".description.text");if(text.format!==undefined)architectProgramArtifactGuardString(text.format,at+".description.format")}
 const timestamp=architectProgramArtifactGuardExactObject(row.timestamps,at+".timestamps",["created","updated","createdBy","updatedBy"],["created","updated"]);
 architectProgramArtifactGuardString(timestamp.created,at+".timestamps.created");architectProgramArtifactGuardString(timestamp.updated,at+".timestamps.updated");
 if(timestamp.createdBy!==undefined)architectProgramArtifactGuardString(timestamp.createdBy,at+".timestamps.createdBy");
 if(timestamp.updatedBy!==undefined)architectProgramArtifactGuardString(timestamp.updatedBy,at+".timestamps.updatedBy");
 return row;
}

export function parseProgramArtifact(value: unknown, at = "$"): ProgramArtifact {
  const row = architectProgramArtifactGuardExactObject(value, at, PROGRAM_ARTIFACT_FIELDS);
  return {
    schema: architectProgramArtifactGuardString(row["schema"], `${at}.schema`),
    meta: parseProgramMeta(row["meta"], `${at}.meta`),
    project: parseProjectDefinition(row["project"], `${at}.project`),
    stakeholders: architectProgramArtifactGuardArray(row["stakeholders"], `${at}.stakeholders`).map((item, index) => parseStakeholder(item, `${at}.stakeholders[${index}]`)),
    users: architectProgramArtifactGuardArray(row["users"], `${at}.users`).map((item, index) => parseUserProfile(item, `${at}.users[${index}]`)),
    activities: architectProgramArtifactGuardArray(row["activities"], `${at}.activities`).map((item, index) => parseActivity(item, `${at}.activities[${index}]`)),
    functions: architectProgramArtifactGuardArray(row["functions"], `${at}.functions`).map((item, index) => parseFunction(item, `${at}.functions[${index}]`)),
    elements: architectProgramArtifactGuardArray(row["elements"], `${at}.elements`).map((item, index) => parseProgramElement(item, `${at}.elements[${index}]`)),
    quantities: architectProgramArtifactGuardArray(row["quantities"], `${at}.quantities`).map((item, index) => parseQuantityRequirement(item, `${at}.quantities[${index}]`)),
    relationships: architectProgramArtifactGuardArray(row["relationships"], `${at}.relationships`).map((item, index) => parseRelationship(item, `${at}.relationships[${index}]`)),
    adjacencies: architectProgramArtifactGuardArray(row["adjacencies"], `${at}.adjacencies`).map((item, index) => parseAdjacency(item, `${at}.adjacencies[${index}]`)),
    processes: architectProgramArtifactGuardArray(row["processes"], `${at}.processes`).map((item, index) => parseProcess(item, `${at}.processes[${index}]`)),
    flows: architectProgramArtifactGuardArray(row["flows"], `${at}.flows`).map((item, index) => parseFlowRequirement(item, `${at}.flows[${index}]`)),
    accessRules: architectProgramArtifactGuardArray(row["accessRules"], `${at}.accessRules`).map((item, index) => parseAccessRule(item, `${at}.accessRules[${index}]`)),
    operations: architectProgramArtifactGuardArray(row["operations"], `${at}.operations`).map((item, index) => parseOperationalRequirement(item, `${at}.operations[${index}]`)),
    equipment: architectProgramArtifactGuardArray(row["equipment"], `${at}.equipment`).map((item, index) => parseEquipment(item, `${at}.equipment[${index}]`)),
    resources: architectProgramArtifactGuardArray(row["resources"], `${at}.resources`).map((item, index) => parseResource(item, `${at}.resources[${index}]`)),
    storage: architectProgramArtifactGuardArray(row["storage"], `${at}.storage`).map((item, index) => parseStorageRequirement(item, `${at}.storage[${index}]`)),
    environmental: architectProgramArtifactGuardArray(row["environmental"], `${at}.environmental`).map((item, index) => parseEnvironmentalRequirement(item, `${at}.environmental[${index}]`)),
    humanFactors: architectProgramArtifactGuardArray(row["humanFactors"], `${at}.humanFactors`).map((item, index) => parseHumanFactorRequirement(item, `${at}.humanFactors[${index}]`)),
    accessibility: architectProgramArtifactGuardArray(row["accessibility"], `${at}.accessibility`).map((item, index) => parseAccessibilityRequirement(item, `${at}.accessibility[${index}]`)),
    privacy: architectProgramArtifactGuardArray(row["privacy"], `${at}.privacy`).map((item, index) => parsePrivacyRequirement(item, `${at}.privacy[${index}]`)),
    safety: architectProgramArtifactGuardArray(row["safety"], `${at}.safety`).map((item, index) => parseSafetyRequirement(item, `${at}.safety[${index}]`)),
    security: architectProgramArtifactGuardArray(row["security"], `${at}.security`).map((item, index) => parseSecurityRequirement(item, `${at}.security[${index}]`)),
    regulatory: architectProgramArtifactGuardArray(row["regulatory"], `${at}.regulatory`).map((item, index) => parseRegulatoryRequirement(item, `${at}.regulatory[${index}]`)),
    siteContext: architectProgramArtifactGuardArray(row["siteContext"], `${at}.siteContext`).map((item, index) => parseSiteContext(item, `${at}.siteContext[${index}]`)),
    organizational: architectProgramArtifactGuardArray(row["organizational"], `${at}.organizational`).map((item, index) => parseOrganizationalRequirement(item, `${at}.organizational[${index}]`)),
    services: architectProgramArtifactGuardArray(row["services"], `${at}.services`).map((item, index) => parseServiceRequirement(item, `${at}.services[${index}]`)),
    infrastructure: architectProgramArtifactGuardArray(row["infrastructure"], `${at}.infrastructure`).map((item, index) => parseInfrastructureRequirement(item, `${at}.infrastructure[${index}]`)),
    information: architectProgramArtifactGuardArray(row["information"], `${at}.information`).map((item, index) => parseInformationRequirement(item, `${at}.information[${index}]`)),
    communication: architectProgramArtifactGuardArray(row["communication"], `${at}.communication`).map((item, index) => parseCommunicationRequirement(item, `${at}.communication[${index}]`)),
    wayfinding: architectProgramArtifactGuardArray(row["wayfinding"], `${at}.wayfinding`).map((item, index) => parseWayfindingRequirement(item, `${at}.wayfinding[${index}]`)),
    schedules: architectProgramArtifactGuardArray(row["schedules"], `${at}.schedules`).map((item, index) => parseScheduleRequirement(item, `${at}.schedules[${index}]`)),
    flexibility: architectProgramArtifactGuardArray(row["flexibility"], `${at}.flexibility`).map((item, index) => parseFlexibilityRequirement(item, `${at}.flexibility[${index}]`)),
    growth: architectProgramArtifactGuardArray(row["growth"], `${at}.growth`).map((item, index) => parseGrowthPlan(item, `${at}.growth[${index}]`)),
    sustainability: architectProgramArtifactGuardArray(row["sustainability"], `${at}.sustainability`).map((item, index) => parseSustainabilityRequirement(item, `${at}.sustainability[${index}]`)),
    resilience: architectProgramArtifactGuardArray(row["resilience"], `${at}.resilience`).map((item, index) => parseResilienceRequirement(item, `${at}.resilience[${index}]`)),
    costs: architectProgramArtifactGuardArray(row["costs"], `${at}.costs`).map((item, index) => parseCostRequirement(item, `${at}.costs[${index}]`)),
    delivery: architectProgramArtifactGuardArray(row["delivery"], `${at}.delivery`).map((item, index) => parseDeliveryConstraint(item, `${at}.delivery[${index}]`)),
    risks: architectProgramArtifactGuardArray(row["risks"], `${at}.risks`).map((item, index) => parseRisk(item, `${at}.risks[${index}]`)),
    conflicts: architectProgramArtifactGuardArray(row["conflicts"], `${at}.conflicts`).map((item, index) => parseConflict(item, `${at}.conflicts[${index}]`)),
    requirements: architectProgramArtifactGuardArray(row["requirements"], `${at}.requirements`).map((item, index) => parseRequirement(item, `${at}.requirements[${index}]`)),
    priorities: architectProgramArtifactGuardArray(row["priorities"], `${at}.priorities`).map((item, index) => parsePriorityRecord(item, `${at}.priorities[${index}]`)),
    scenarios: architectProgramArtifactGuardArray(row["scenarios"], `${at}.scenarios`).map((item, index) => parseScenario(item, `${at}.scenarios[${index}]`)),
    options: architectProgramArtifactGuardArray(row["options"], `${at}.options`).map((item, index) => parseOptionEvaluation(item, `${at}.options[${index}]`)),
    decisions: architectProgramArtifactGuardArray(row["decisions"], `${at}.decisions`).map((item, index) => parseDecision(item, `${at}.decisions[${index}]`)),
    validations: architectProgramArtifactGuardArray(row["validations"], `${at}.validations`).map((item, index) => parseValidationRecord(item, `${at}.validations[${index}]`)),
    performance: architectProgramArtifactGuardArray(row["performance"], `${at}.performance`).map((item, index) => parsePerformanceCriterion(item, `${at}.performance[${index}]`)),
    quality: architectProgramArtifactGuardArray(row["quality"], `${at}.quality`).map((item, index) => parseQualityRecord(item, `${at}.quality[${index}]`)),
    artifacts: architectProgramArtifactGuardArray(row["artifacts"], `${at}.artifacts`).map((item, index) => parseArtifactRecord(item, `${at}.artifacts[${index}]`)),
    assumptions: architectProgramArtifactGuardArray(row["assumptions"], `${at}.assumptions`).map((item, index) => parseAssumption(item, `${at}.assumptions[${index}]`)),
    constraints: architectProgramArtifactGuardArray(row["constraints"], `${at}.constraints`).map((item, index) => parseConstraintRecord(item, `${at}.constraints[${index}]`)),
    complianceRecords: architectProgramArtifactGuardArray(row["complianceRecords"], `${at}.complianceRecords`).map((item, index) => parseComplianceRecord(item, `${at}.complianceRecords[${index}]`)),
    approvals: architectProgramArtifactGuardArray(row["approvals"], `${at}.approvals`).map((item, index) => parseApprovalRecord(item, `${at}.approvals[${index}]`)),
    meetings: architectProgramArtifactGuardArray(row["meetings"], `${at}.meetings`).map((item, index) => parseMeetingRecord(item, `${at}.meetings[${index}]`)),
    changes: architectProgramArtifactGuardArray(row["changes"], `${at}.changes`).map((item, index) => parseChangeRecord(item, `${at}.changes[${index}]`)),
    collaboration: architectProgramArtifactGuardArray(row["collaboration"], `${at}.collaboration`).map((item, index) => parseCollaborationRecord(item, `${at}.collaboration[${index}]`)),
    analyses: architectProgramArtifactGuardArray(row["analyses"], `${at}.analyses`).map((item, index) => parseAnalysisRecord(item, `${at}.analyses[${index}]`)),
    reports: architectProgramArtifactGuardArray(row["reports"], `${at}.reports`).map((item, index) => parseReportRecord(item, `${at}.reports[${index}]`)),
    searchFilters: architectProgramArtifactGuardArray(row["searchFilters"], `${at}.searchFilters`).map((item, index) => parseSearchFilter(item, `${at}.searchFilters[${index}]`)),
    statusRecords: architectProgramArtifactGuardArray(row["statusRecords"], `${at}.statusRecords`).map((item, index) => parseStatusRecord(item, `${at}.statusRecords[${index}]`)),
    workshops: architectProgramArtifactGuardArray(row["workshops"], `${at}.workshops`).map((item, index) => parseWorkshop(item, `${at}.workshops[${index}]`)),
    surveys: architectProgramArtifactGuardArray(row["surveys"], `${at}.surveys`).map((item, index) => parseSurvey(item, `${at}.surveys[${index}]`)),
    issues: architectProgramArtifactGuardArray(row["issues"], `${at}.issues`).map((item, index) => parseIssue(item, `${at}.issues[${index}]`)),
    auditEvents: architectProgramArtifactGuardArray(row["auditEvents"], `${at}.auditEvents`).map((item, index) => parseAuditEvent(item, `${at}.auditEvents[${index}]`)),
    templates: architectProgramArtifactGuardArray(row["templates"], `${at}.templates`).map((item, index) => parseTemplateRecord(item, `${at}.templates[${index}]`)),
    knowledgePayload: architectProgramArtifactGuardArray(row["knowledgePayload"], `${at}.knowledgePayload`).map((item, index) => parseKnowledgeRecord(item, `${at}.knowledgePayload[${index}]`)),
    knowledge: parseArtifactChild(row["knowledge"]),
    benchmarksPayload: architectProgramArtifactGuardArray(row["benchmarksPayload"], `${at}.benchmarksPayload`).map((item, index) => parseBenchmarkRecord(item, `${at}.benchmarksPayload[${index}]`)),
    benchmarks: parseArtifactChild(row["benchmarks"]),
    traces: architectProgramArtifactGuardArray(row["traces"], `${at}.traces`).map((item, index) => parseTraceLink(item, `${at}.traces[${index}]`)),
    governance: parseGovernance(row["governance"], `${at}.governance`),
  };
}

export function parseAccessRule(value: unknown, at = "$"): AccessRule {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectIds","resourceIds","accessLevel","accessMode","authentication","authorization","timeRestrictions","escortPolicy","visitorPolicy","emergencyOverride","auditRequired","badgeRequired","biometricRequired","zoneIds","exceptions","regulatoryBasis","enforcementMethod","revocationPolicy","trainingRequired","ownerId"], ["id","name","status","priority","ownership","timestamps","subjectIds","resourceIds","accessLevel","accessMode","authentication","authorization","timeRestrictions","escortPolicy","visitorPolicy","emergencyOverride","auditRequired","badgeRequired","biometricRequired","zoneIds","exceptions","regulatoryBasis","enforcementMethod","revocationPolicy","trainingRequired","ownerId"]) as unknown as AccessRule;
}

export function parseAccessibilityRequirement(value: unknown, at = "$"): AccessibilityRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","standard","level","userProfileIds","elementIds","routeIds","clearWidthM","clearHeightM","turningCircleM","rampSlope","liftRequired","tactileGuidance","hearingLoop","visualContrast","signageRequirements","controlsHeight","emergencyEvacuation","serviceAnimalPolicy","companionSeating","verificationPlan","exceptions","wcagConformance","universalDesignPrinciples"], ["id","name","status","priority","ownership","timestamps","standard","level","userProfileIds","elementIds","routeIds","clearWidthM","clearHeightM","turningCircleM","rampSlope","liftRequired","tactileGuidance","hearingLoop","visualContrast","signageRequirements","controlsHeight","emergencyEvacuation","serviceAnimalPolicy","companionSeating","verificationPlan","exceptions","wcagConformance","universalDesignPrinciples"]);
  nullableBinary64(row.clearWidthM, at+".clearWidthM");
  nullableBinary64(row.clearHeightM, at+".clearHeightM");
  nullableBinary64(row.turningCircleM, at+".turningCircleM");
  nullableBinary64(row.rampSlope, at+".rampSlope");
  return row as unknown as AccessibilityRequirement;
}

export function parseActivity(value: unknown, at = "$"): Activity {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","frequency","duration","intensity","participants","equipmentIds","spaceRequirements","environmentalNeeds","privacyNeeds","accessibilityNeeds","adjacentActivities","sequencing","peakPeriods","workflowSteps","inputs","outputs","userProfileIds","functionIds","performanceIndicators","activityType","locationContext","temporalPattern","supervisionLevel"], ["id","name","status","priority","ownership","timestamps","code","category","frequency","duration","intensity","participants","equipmentIds","spaceRequirements","environmentalNeeds","privacyNeeds","accessibilityNeeds","adjacentActivities","sequencing","peakPeriods","workflowSteps","inputs","outputs","userProfileIds","functionIds","performanceIndicators","activityType","locationContext","temporalPattern","supervisionLevel"]);
  parseQuantitySpec(row.participants,at+".participants");
  return row as unknown as Activity;
}

export function parseAdjacency(value: unknown, at = "$"): Adjacency {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","elementAId","elementBId","kind","connection","separations","weight","rationale","distanceMaxM","distanceMinM","levelConstraint","accessPath","sharedWall","sharedEntry","trafficIsolation","circulationOverlap","conflictIds","normalized","verificationStatus","sourceRelationshipId","internalExternalAccess"], ["id","name","status","priority","ownership","timestamps","elementAId","elementBId","kind","connection","separations","weight","rationale","distanceMaxM","distanceMinM","levelConstraint","accessPath","sharedWall","sharedEntry","trafficIsolation","circulationOverlap","conflictIds","normalized","verificationStatus","sourceRelationshipId","internalExternalAccess"]);
  architectProgramArtifactGuardBinary64(row.weight, at+".weight");
  nullableBinary64(row.distanceMaxM, at+".distanceMaxM");
  nullableBinary64(row.distanceMinM, at+".distanceMinM");
  return row as unknown as Adjacency;
}

export function parseAdjacencyKind(value: unknown, at = "$"): AdjacencyKind {
  return architectProgramArtifactGuardMember(value, at, ["required","preferred","optional","prohibited"]) as AdjacencyKind;
}

export function parseAnalysisRecord(value: unknown, at = "$"): AnalysisRecord {
  const row=architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","kind","title","parameters","inputEntityIds","outputSummary","findings","metrics","charts","runBy","runAt","durationMs","toolVersion","scenarioId","reportId","confidence","limitations","recommendations","rawResultRef"], ["id","name","status","priority","ownership","timestamps","kind","title","parameters","inputEntityIds","outputSummary","findings","metrics","charts","runBy","runAt","durationMs","toolVersion","scenarioId","reportId","confidence","limitations","recommendations","rawResultRef"]);
  if(row["durationMs"]!==null)architectProgramArtifactGuardUnsigned64(row["durationMs"],`${at}.durationMs`);
  return row as unknown as AnalysisRecord;
}

export function parseApprovalRecord(value: unknown, at = "$"): ApprovalRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","approvalType","subjectId","approverIds","approvalDate","conditions","approvalStatus","expiryDate","delegationChain","evidenceRefs","relatedDecisionId","relatedChangeId","authorityBasis","signatureMethod","rejectionReason","resubmissionDate","notificationList","workflowStep","version","auditTrailRef"], ["id","name","status","priority","ownership","timestamps","approvalType","subjectId","approverIds","approvalDate","conditions","approvalStatus","expiryDate","delegationChain","evidenceRefs","relatedDecisionId","relatedChangeId","authorityBasis","signatureMethod","rejectionReason","resubmissionDate","notificationList","workflowStep","version","auditTrailRef"]) as unknown as ApprovalRecord;
}

export function parseAssumption(value: unknown, at = "$"): Assumption {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","statement","basis","confidenceLevel","impactIfFalse","relatedEntityIds","validationStatus","validatedBy","validationDate","ownerId","reviewCycle","source","category","dependencies","mitigation","linkedRequirementIds","linkedRiskIds","expirationDate","statusNotes","artifactRefs"], ["id","name","status","priority","ownership","timestamps","statement","basis","confidenceLevel","impactIfFalse","relatedEntityIds","validationStatus","validatedBy","validationDate","ownerId","reviewCycle","source","category","dependencies","mitigation","linkedRequirementIds","linkedRiskIds","expirationDate","statusNotes","artifactRefs"]) as unknown as Assumption;
}

export function parseAuditEvent(value: unknown, at = "$"): AuditEvent {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","action","actorId","subjectId","subjectKind","timestamp","details","beforeState","afterState","ipAddress","client","sessionId","changeRecordId","traceLink","success","errorMessage","correlationId","complianceTags","retentionUntil"], ["id","name","status","priority","ownership","timestamps","action","actorId","subjectId","subjectKind","timestamp","details","beforeState","afterState","ipAddress","client","sessionId","changeRecordId","traceLink","success","errorMessage","correlationId","complianceTags","retentionUntil"]) as unknown as AuditEvent;
}

export function parseBenchmarkRecord(value: unknown, at = "$"): BenchmarkRecord {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","benchmarkName","sector","metric","value","unit","sampleSize","source","collectionYear","geography","buildingType","confidence","methodology","applicableElementKinds","relatedRequirementIds","comparisonNotes","limitations","license","knowledgeId","lastVerified"], ["id","name","status","priority","ownership","timestamps","benchmarkName","sector","metric","value","unit","sampleSize","source","collectionYear","geography","buildingType","confidence","methodology","applicableElementKinds","relatedRequirementIds","comparisonNotes","limitations","license","knowledgeId","lastVerified"]);
  if (row["sampleSize"] !== null) architectProgramArtifactGuardInteger(row["sampleSize"], `${at}.sampleSize`, { minimum: 0, maximum: 4294967295 });
  if (row["collectionYear"] !== null) architectProgramArtifactGuardInteger(row["collectionYear"], `${at}.collectionYear`, { minimum: 0, maximum: 4294967295 });
  architectProgramArtifactGuardBinary64(row.value, at+".value");
  return row as unknown as BenchmarkRecord;
}

export function parseChangeRecord(value: unknown, at = "$"): ChangeRecord {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","changeType","summary","reason","requestedBy","approvedBy","changeDate","effectiveDate","impactedEntityIds","beforeSnapshot","afterSnapshot","costImpact","scheduleImpact","riskImpact","approvalStatus","rollbackPlan","communicationPlan","versionFrom","versionTo","auditEventIds"], ["id","name","status","priority","ownership","timestamps","changeType","summary","reason","requestedBy","approvedBy","changeDate","effectiveDate","impactedEntityIds","beforeSnapshot","afterSnapshot","costImpact","scheduleImpact","riskImpact","approvalStatus","rollbackPlan","communicationPlan","versionFrom","versionTo","auditEventIds"]);
  nullableBinary64(row.costImpact, at+".costImpact");
  return row as unknown as ChangeRecord;
}

export function parseCollaborationRecord(value: unknown, at = "$"): CollaborationRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","sessionType","title","participants","facilitatorId","startTime","endTime","location","agenda","outcomes","actionItems","decisionIds","issueIds","documentIds","recordingRef","feedback","followUpDate","workshopId","surveyId"], ["id","name","status","priority","ownership","timestamps","sessionType","title","participants","facilitatorId","startTime","endTime","location","agenda","outcomes","actionItems","decisionIds","issueIds","documentIds","recordingRef","feedback","followUpDate","workshopId","surveyId"]) as unknown as CollaborationRecord;
}

export function parseCommunicationRequirement(value: unknown, at = "$"): CommunicationRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","channel","audienceIds","messageTypes","frequency","medium","language","accessibility","emergencyUse","twoWay","recordingPolicy","signageLocations","technology","escalationPath","feedbackLoop","privacyControls","elementIds","standards","ownerId","templates"], ["id","name","status","priority","ownership","timestamps","channel","audienceIds","messageTypes","frequency","medium","language","accessibility","emergencyUse","twoWay","recordingPolicy","signageLocations","technology","escalationPath","feedbackLoop","privacyControls","elementIds","standards","ownerId","templates"]) as unknown as CommunicationRequirement;
}

export function parseComplianceRecord(value: unknown, at = "$"): ComplianceRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","standardRef","obligation","complianceStatus","evidenceRefs","auditorId","auditDate","nextReview","affectedEntityIds","gapAnalysis","remediationPlan","ownerId","severity","regulatoryBody","certificationTarget","waiverStatus","relatedRequirementIds","monitoringMethod","reportingFrequency","penalties","correctiveActions","artifactRefs"], ["id","name","status","priority","ownership","timestamps","standardRef","obligation","complianceStatus","evidenceRefs","auditorId","auditDate","nextReview","affectedEntityIds","gapAnalysis","remediationPlan","ownerId","severity","regulatoryBody","certificationTarget","waiverStatus","relatedRequirementIds","monitoringMethod","reportingFrequency","penalties","correctiveActions","artifactRefs"]) as unknown as ComplianceRecord;
}

export function parseConflict(value: unknown, at = "$"): Conflict {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","kind","summary","entityAId","entityBId","severity","detectedBy","detectionDate","tradeOffOptions","recommendedResolution","decisionId","stakeholderIds","requirementIds","costImpact","scheduleImpact","qualityImpact","resolutionStatus","ownerId","escalationLevel","relatedRiskIds"], ["id","name","status","priority","ownership","timestamps","kind","summary","entityAId","entityBId","severity","detectedBy","detectionDate","tradeOffOptions","recommendedResolution","decisionId","stakeholderIds","requirementIds","costImpact","scheduleImpact","qualityImpact","resolutionStatus","ownerId","escalationLevel","relatedRiskIds"]);
  nullableBinary64(row.costImpact, at+".costImpact");
  return row as unknown as Conflict;
}

export function parseConstraintRecord(value: unknown, at = "$"): ConstraintRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","constraintType","summary","severity","affectedEntityIds","source","regulatoryBasis","mitigationOptions","ownerId","effectiveDate","expiryDate","waiverStatus","waiverApprover","impactAssessment","resolutionPlan","relatedRequirementIds","relatedDecisionIds","monitoringFrequency","complianceStatus","exceptions","traceLinks","escalationContactId"], ["id","name","status","priority","ownership","timestamps","constraintType","summary","severity","affectedEntityIds","source","regulatoryBasis","mitigationOptions","ownerId","effectiveDate","expiryDate","waiverStatus","waiverApprover","impactAssessment","resolutionPlan","relatedRequirementIds","relatedDecisionIds","monitoringFrequency","complianceStatus","exceptions","traceLinks","escalationContactId"]) as unknown as ConstraintRecord;
}

export function parseCostRequirement(value: unknown, at = "$"): CostRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","costItem","basis","amount","currency","quantityBasis","unitCost","contingencyPercent","escalationRate","fundingSource","elementIds","requirementIds","phase","cashFlowProfile","valueEngineeringNotes","benchmarkRef","approvalStatus","ownerId","assumptions","sensitivityFactors"], ["id","name","status","priority","ownership","timestamps","costItem","basis","amount","currency","quantityBasis","unitCost","contingencyPercent","escalationRate","fundingSource","elementIds","requirementIds","phase","cashFlowProfile","valueEngineeringNotes","benchmarkRef","approvalStatus","ownerId","assumptions","sensitivityFactors"]);
  nullableBinary64(row.amount, at+".amount");
  nullableBinary64(row.unitCost, at+".unitCost");
  nullableBinary64(row.contingencyPercent, at+".contingencyPercent");
  nullableBinary64(row.escalationRate, at+".escalationRate");
  return row as unknown as CostRequirement;
}

export function parseDecision(value: unknown, at = "$"): Decision {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","decisionStatement","context","optionsConsidered","selectedOptionId","rationale","decisionMakerIds","consultedIds","informedIds","decisionDate","effectiveDate","reversalConditions","impactedRequirementIds","impactedElementIds","costImpact","scheduleImpact","riskImpact","approvalStatus","meetingRef","artifactRefs"], ["id","name","status","priority","ownership","timestamps","decisionStatement","context","optionsConsidered","selectedOptionId","rationale","decisionMakerIds","consultedIds","informedIds","decisionDate","effectiveDate","reversalConditions","impactedRequirementIds","impactedElementIds","costImpact","scheduleImpact","riskImpact","approvalStatus","meetingRef","artifactRefs"]);
  nullableBinary64(row.costImpact, at+".costImpact");
  return row as unknown as Decision;
}

export function parseDeliveryConstraint(value: unknown, at = "$"): DeliveryConstraint {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","constraintType","constraintDetails","phase","hardDeadline","softDeadline","impactedElementIds","impactedRequirementIds","workHours","noiseRestrictions","accessRestrictions","siteLogistics","procurementLeadTime","approvalGates","occupancyConstraints","weatherWindows","penaltyClauses","mitigationOptions","ownerId","riskIds","constraintStatus"], ["id","name","status","priority","ownership","timestamps","constraintType","constraintDetails","phase","hardDeadline","softDeadline","impactedElementIds","impactedRequirementIds","workHours","noiseRestrictions","accessRestrictions","siteLogistics","procurementLeadTime","approvalGates","occupancyConstraints","weatherWindows","penaltyClauses","mitigationOptions","ownerId","riskIds","constraintStatus"]) as unknown as DeliveryConstraint;
}

export function parseArtifactRecord(value: unknown, at = "$"): ArtifactRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","documentType","title","version","fileRef","format","authorIds","reviewerIds","approverIds","issueDate","revisionDate","distributionList","relatedEntityIds","classification","retentionPeriod","accessControls","supersedes","documentStatus","checksum","sourceSystem"], ["id","name","status","priority","ownership","timestamps","documentType","title","version","fileRef","format","authorIds","reviewerIds","approverIds","issueDate","revisionDate","distributionList","relatedEntityIds","classification","retentionPeriod","accessControls","supersedes","documentStatus","checksum","sourceSystem"]) as unknown as ArtifactRecord;
}

export function parseEnvironmentalRequirement(value: unknown, at = "$"): EnvironmentalRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","parameterKind","parameter","targetValue","unit","minValue","maxValue","comfortBand","measurementMethod","monitoringFrequency","elementIds","occupancyBasis","seasonalVariation","energyImplications","standards","certificationTargets","outdoorConditions","ventilationStrategy","daylightTarget","acousticTarget","iaqTarget","verificationPlan"], ["id","name","status","priority","ownership","timestamps","parameterKind","parameter","targetValue","unit","minValue","maxValue","comfortBand","measurementMethod","monitoringFrequency","elementIds","occupancyBasis","seasonalVariation","energyImplications","standards","certificationTargets","outdoorConditions","ventilationStrategy","daylightTarget","acousticTarget","iaqTarget","verificationPlan"]);
  nullableBinary64(row.targetValue, at+".targetValue");
  nullableBinary64(row.minValue, at+".minValue");
  nullableBinary64(row.maxValue, at+".maxValue");
  return row as unknown as EnvironmentalRequirement;
}

export function parseEquipment(value: unknown, at = "$"): Equipment {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","manufacturer","model","quantity","dimensions","weightKg","powerKw","utilityConnections","ventilation","noiseLevelDb","clearance","mounting","elementIds","activityIds","maintenanceAccess","lifecycleYears","replacementCost","standards","supplier","activityLinkIds","installationRequirements","commissioningNotes","spareParts"], ["id","name","status","priority","ownership","timestamps","code","category","manufacturer","model","quantity","dimensions","weightKg","powerKw","utilityConnections","ventilation","noiseLevelDb","clearance","mounting","elementIds","activityIds","maintenanceAccess","lifecycleYears","replacementCost","standards","supplier","activityLinkIds","installationRequirements","commissioningNotes","spareParts"]);
  if (row["lifecycleYears"] !== null) architectProgramArtifactGuardInteger(row["lifecycleYears"], `${at}.lifecycleYears`, { minimum: 0, maximum: 4294967295 });
  nullableBinary64(row.weightKg, at+".weightKg");
  nullableBinary64(row.powerKw, at+".powerKw");
  nullableBinary64(row.noiseLevelDb, at+".noiseLevelDb");
  nullableBinary64(row.replacementCost, at+".replacementCost");
  parseQuantitySpec(row.quantity,at+".quantity");
  return row as unknown as Equipment;
}

export function parseFlexibilityRequirement(value: unknown, at = "$"): FlexibilityRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","flexibilityType","elementIds","adaptationScenarios","modularityLevel","reconfigurationTime","costOfChange","technologyReadiness","futureFunctionIds","demountablePartitions","raisedFloor","overheadServices","expansionDirection","contractionScenario","multiUsePotential","furnitureStrategy","infrastructureSpareCapacity","leaseImplications","ownerId"], ["id","name","status","priority","ownership","timestamps","flexibilityType","elementIds","adaptationScenarios","modularityLevel","reconfigurationTime","costOfChange","technologyReadiness","futureFunctionIds","demountablePartitions","raisedFloor","overheadServices","expansionDirection","contractionScenario","multiUsePotential","furnitureStrategy","infrastructureSpareCapacity","leaseImplications","ownerId"]);
  nullableBinary64(row.costOfChange, at+".costOfChange");
  return row as unknown as FlexibilityRequirement;
}

export function parseFlowRequirement(value: unknown, at = "$"): FlowRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","fromElementId","toElementId","kind","flowType","direction","volume","peakRate","clearWidthM","clearHeightM","separationRequirements","accessLevel","timeWindows","equipmentClearance","signageRequired","escortRequired","emergencyRoute","barrierFree","monitoringRequired","processId","conflictIds","verificationMethod"], ["id","name","status","priority","ownership","timestamps","fromElementId","toElementId","kind","flowType","direction","volume","peakRate","clearWidthM","clearHeightM","separationRequirements","accessLevel","timeWindows","equipmentClearance","signageRequired","escortRequired","emergencyRoute","barrierFree","monitoringRequired","processId","conflictIds","verificationMethod"]);
  nullableBinary64(row.peakRate, at+".peakRate");
  nullableBinary64(row.clearWidthM, at+".clearWidthM");
  nullableBinary64(row.clearHeightM, at+".clearHeightM");
  parseQuantitySpec(row.volume,at+".volume");
  return row as unknown as FlowRequirement;
}

export function parseFunction(value: unknown, at = "$"): Function {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","kind","purpose","criticality","performanceTargets","serviceLevel","operatingHours","staffing","equipmentIds","resourceIds","activityIds","elementIds","dependencies","interfaces","constraints","qualityCriteria","regulatoryRefs","futureChanges","ownerStakeholderId","successMetrics","hierarchyParentId","conflictIds"], ["id","name","status","priority","ownership","timestamps","code","kind","purpose","criticality","performanceTargets","serviceLevel","operatingHours","staffing","equipmentIds","resourceIds","activityIds","elementIds","dependencies","interfaces","constraints","qualityCriteria","regulatoryRefs","futureChanges","ownerStakeholderId","successMetrics","hierarchyParentId","conflictIds"]);
  parseQuantitySpec(row.staffing,at+".staffing");
  return row as unknown as Function;
}

export function parseGovernance(value: unknown, at = "$"): Governance {
  return architectProgramArtifactGuardExactObject(value, at, ["id","framework","roles","responsibilities","approvalMatrix","escalationPaths","meetingCadence","decisionRights","changeControlProcess","qualityPolicy","riskAppetite","complianceObligations","auditSchedule","documentControl","stakeholderEngagementPlan","ethicsPolicy","dataGovernance","ownerId","reviewCycle","reviewHierarchy","policyOwnershipId","requirementOwnershipId","riskOwnershipId","reportingFrequency","accountabilityRules","exceptionManagement","governancePerformance"], ["id","framework","roles","responsibilities","approvalMatrix","escalationPaths","meetingCadence","decisionRights","changeControlProcess","qualityPolicy","riskAppetite","complianceObligations","auditSchedule","documentControl","stakeholderEngagementPlan","ethicsPolicy","dataGovernance","ownerId","reviewCycle","reviewHierarchy","policyOwnershipId","requirementOwnershipId","riskOwnershipId","reportingFrequency","accountabilityRules","exceptionManagement","governancePerformance"]) as unknown as Governance;
}

export function parseGrowthPlan(value: unknown, at = "$"): GrowthPlan {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","horizonYears","growthRate","headcountGrowth","areaGrowth","phases","triggerEvents","expansionElementIds","reserveAreas","infrastructureHeadroom","budgetEnvelope","fundingSources","riskFactors","decisionPoints","scenarioIds","decommissionPlan","relocationStrategy","stakeholderImpact","regulatoryConsiderations","ownerId"], ["id","name","status","priority","ownership","timestamps","horizonYears","growthRate","headcountGrowth","areaGrowth","phases","triggerEvents","expansionElementIds","reserveAreas","infrastructureHeadroom","budgetEnvelope","fundingSources","riskFactors","decisionPoints","scenarioIds","decommissionPlan","relocationStrategy","stakeholderImpact","regulatoryConsiderations","ownerId"]);
  architectProgramArtifactGuardInteger(row["horizonYears"], `${at}.horizonYears`, { minimum: 0, maximum: 4294967295 });
  nullableBinary64(row.growthRate, at+".growthRate");
  nullableBinary64(row.budgetEnvelope, at+".budgetEnvelope");
  parseQuantitySpec(row.headcountGrowth,at+".headcountGrowth");
  parseQuantitySpec(row.areaGrowth,at+".areaGrowth");
  return row as unknown as GrowthPlan;
}

export function parseHumanFactorRequirement(value: unknown, at = "$"): HumanFactorRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","aspect","factor","userProfileIds","activityIds","ergonomicCriteria","cognitiveLoad","visualDemands","auditoryDemands","postureRequirements","reachEnvelope","lightingForTasks","thermalComfort","privacyNeeds","socialInteraction","stressFactors","mitigationMeasures","trainingNeeds","standards","researchBasis","elementIds","verificationMethod"], ["id","name","status","priority","ownership","timestamps","aspect","factor","userProfileIds","activityIds","ergonomicCriteria","cognitiveLoad","visualDemands","auditoryDemands","postureRequirements","reachEnvelope","lightingForTasks","thermalComfort","privacyNeeds","socialInteraction","stressFactors","mitigationMeasures","trainingNeeds","standards","researchBasis","elementIds","verificationMethod"]) as unknown as HumanFactorRequirement;
}

export function parseInformationRequirement(value: unknown, at = "$"): InformationRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","informationType","format","sourceSystem","destinationSystems","updateFrequency","retentionPeriod","accessControls","classification","qualityCriteria","metadataRequirements","integrationPoints","backupRequirements","disasterRecovery","privacyControls","auditTrail","elementIds","stakeholderIds","standards","ownerId"], ["id","name","status","priority","ownership","timestamps","informationType","format","sourceSystem","destinationSystems","updateFrequency","retentionPeriod","accessControls","classification","qualityCriteria","metadataRequirements","integrationPoints","backupRequirements","disasterRecovery","privacyControls","auditTrail","elementIds","stakeholderIds","standards","ownerId"]) as unknown as InformationRequirement;
}

export function parseInfrastructureRequirement(value: unknown, at = "$"): InfrastructureRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","system","category","capacity","redundancy","distribution","entryPoints","utilitySource","standbyPower","monitoring","maintenanceAccess","standards","elementIds","peakDemand","diversityFactor","futureExpansion","interfaceRequirements","commissioning","lifecycleCost","ownerId"], ["id","name","status","priority","ownership","timestamps","system","category","capacity","redundancy","distribution","entryPoints","utilitySource","standbyPower","monitoring","maintenanceAccess","standards","elementIds","peakDemand","diversityFactor","futureExpansion","interfaceRequirements","commissioning","lifecycleCost","ownerId"]);
  nullableBinary64(row.peakDemand, at+".peakDemand");
  nullableBinary64(row.diversityFactor, at+".diversityFactor");
  nullableBinary64(row.lifecycleCost, at+".lifecycleCost");
  parseQuantitySpec(row.capacity,at+".capacity");
  return row as unknown as InfrastructureRequirement;
}

export function parseIssue(value: unknown, at = "$"): Issue {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","issueType","summary","issueDescription","severity","issuePriority","reporterId","assigneeId","affectedEntityIds","rootCause","resolution","workaround","dueDate","resolvedDate","relatedConflictIds","relatedRiskIds","decisionId","comments","attachments","escalationLevel"], ["id","name","status","priority","ownership","timestamps","issueType","summary","issueDescription","severity","issuePriority","reporterId","assigneeId","affectedEntityIds","rootCause","resolution","workaround","dueDate","resolvedDate","relatedConflictIds","relatedRiskIds","decisionId","comments","attachments","escalationLevel"]) as unknown as Issue;
}

export function parseKnowledgeRecord(value: unknown, at = "$"): KnowledgeRecord {
  const row=architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","topic","category","summary","content","sources","references","lessonsLearned","bestPractices","applicableSectors","relatedEntityKinds","authorIds","expertiseLevel","validationStatus","lastReviewed","keywords","attachments","citations","usageCount"], ["id","name","status","priority","ownership","timestamps","topic","category","summary","content","sources","references","lessonsLearned","bestPractices","applicableSectors","relatedEntityKinds","authorIds","expertiseLevel","validationStatus","lastReviewed","keywords","attachments","citations","usageCount"]);
  architectProgramArtifactGuardUnsigned64(row["usageCount"],`${at}.usageCount`);
  return row as unknown as KnowledgeRecord;
}

export function parseMeetingRecord(value: unknown, at = "$"): MeetingRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","meetingType","scheduledDate","duration","location","chairId","attendeeIds","agendaItems","minutes","actionItems","decisionsMade","artifactRefs","followUpDate","recordingRef","quorumMet","meetingStatus","workshopId","stakeholderIds","requirementIds","issueIds","approvalIds"], ["id","name","status","priority","ownership","timestamps","meetingType","scheduledDate","duration","location","chairId","attendeeIds","agendaItems","minutes","actionItems","decisionsMade","artifactRefs","followUpDate","recordingRef","quorumMet","meetingStatus","workshopId","stakeholderIds","requirementIds","issueIds","approvalIds"]) as unknown as MeetingRecord;
}

export function parseOperationalRequirement(value: unknown, at = "$"): OperationalRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","operation","serviceLevel","operatingHours","staffing","maintenanceInterval","cleaningRegime","turnaroundTime","redundancy","uptimeTarget","responseTime","equipmentIds","elementIds","processIds","utilities","wasteStreams","contingencyPlan","trainingRequirements","sopReferences","kpiTargets","ownerId","serviceCategory","shiftPattern","slaTarget","escalationContactId"], ["id","name","status","priority","ownership","timestamps","operation","serviceLevel","operatingHours","staffing","maintenanceInterval","cleaningRegime","turnaroundTime","redundancy","uptimeTarget","responseTime","equipmentIds","elementIds","processIds","utilities","wasteStreams","contingencyPlan","trainingRequirements","sopReferences","kpiTargets","ownerId","serviceCategory","shiftPattern","slaTarget","escalationContactId"]);
  nullableBinary64(row.uptimeTarget, at+".uptimeTarget");
  parseQuantitySpec(row.staffing,at+".staffing");
  return row as unknown as OperationalRequirement;
}

export function parseOptionEvaluation(value: unknown, at = "$"): OptionEvaluation {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","optionName","optionDescription","scenarioId","criteriaIds","scores","weightedScore","costEstimate","scheduleEstimate","riskSummary","benefits","drawbacks","assumptions","dependencies","stakeholderFeedback","recommendation","decisionId","evaluationStatus","evaluatorIds","evaluationDate"], ["id","name","status","priority","ownership","timestamps","optionName","optionDescription","scenarioId","criteriaIds","scores","weightedScore","costEstimate","scheduleEstimate","riskSummary","benefits","drawbacks","assumptions","dependencies","stakeholderFeedback","recommendation","decisionId","evaluationStatus","evaluatorIds","evaluationDate"]);
  architectProgramArtifactGuardArray(row.scores, at+".scores").forEach((value,index)=>architectProgramArtifactGuardBinary64(value,at+".scores["+index+"]"));
  nullableBinary64(row.weightedScore, at+".weightedScore");
  nullableBinary64(row.costEstimate, at+".costEstimate");
  return row as unknown as OptionEvaluation;
}

export function parseOrganizationalRequirement(value: unknown, at = "$"): OrganizationalRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","department","reportingLine","headcount","growthPlanId","workPatterns","collaborationModel","hierarchyLevels","decisionMaking","cultureNotes","changeReadiness","unionConsiderations","trainingNeeds","elementIds","stakeholderIds","serviceRequirementIds","brandingRequirements","wellnessPlugins","diversityGoals","ownerId"], ["id","name","status","priority","ownership","timestamps","department","reportingLine","headcount","growthPlanId","workPatterns","collaborationModel","hierarchyLevels","decisionMaking","cultureNotes","changeReadiness","unionConsiderations","trainingNeeds","elementIds","stakeholderIds","serviceRequirementIds","brandingRequirements","wellnessPlugins","diversityGoals","ownerId"]);
  parseQuantitySpec(row.headcount,at+".headcount");
  return row as unknown as OrganizationalRequirement;
}

export function parsePerformanceCriterion(value: unknown, at = "$"): PerformanceCriterion {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","criterion","metric","target","unit","minimum","maximum","measurementMethod","frequency","requirementIds","elementIds","baseline","benchmarkRef","weight","dataSource","reportingCadence","ownerId","verificationPlan","penaltyThreshold","incentiveThreshold"], ["id","name","status","priority","ownership","timestamps","criterion","metric","target","unit","minimum","maximum","measurementMethod","frequency","requirementIds","elementIds","baseline","benchmarkRef","weight","dataSource","reportingCadence","ownerId","verificationPlan","penaltyThreshold","incentiveThreshold"]);
  nullableBinary64(row.target, at+".target");
  nullableBinary64(row.minimum, at+".minimum");
  nullableBinary64(row.maximum, at+".maximum");
  nullableBinary64(row.baseline, at+".baseline");
  nullableBinary64(row.weight, at+".weight");
  nullableBinary64(row.penaltyThreshold, at+".penaltyThreshold");
  nullableBinary64(row.incentiveThreshold, at+".incentiveThreshold");
  return row as unknown as PerformanceCriterion;
}

export function parsePriorityRecord(value: unknown, at = "$"): PriorityRecord {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectId","subjectKind","rankedPriority","rank","weight","rationale","decisionId","stakeholderIds","effectiveFrom","effectiveUntil","reviewCycle","dependencies","conflicts","scoringMethod","score","criteria","approvedBy","approvalDate","rankingNotes"], ["id","name","status","priority","ownership","timestamps","subjectId","subjectKind","rankedPriority","rank","weight","rationale","decisionId","stakeholderIds","effectiveFrom","effectiveUntil","reviewCycle","dependencies","conflicts","scoringMethod","score","criteria","approvedBy","approvalDate","rankingNotes"]);
  if (row["rank"] !== null) architectProgramArtifactGuardInteger(row["rank"], `${at}.rank`, { minimum: 0, maximum: 4294967295 });
  nullableBinary64(row.weight, at+".weight");
  nullableBinary64(row.score, at+".score");
  return row as unknown as PriorityRecord;
}

export function parsePrivacyRequirement(value: unknown, at = "$"): PrivacyRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","privacyKind","privacyType","level","subjectIds","elementIds","visualPrivacy","acousticPrivacy","dataPrivacy","screeningRequired","enclosureRequired","accessRestrictions","observationRisk","regulatoryBasis","culturalConsiderations","technologyControls","signage","monitoringRestrictions","retentionPolicy","breachResponse","ownerId"], ["id","name","status","priority","ownership","timestamps","privacyKind","privacyType","level","subjectIds","elementIds","visualPrivacy","acousticPrivacy","dataPrivacy","screeningRequired","enclosureRequired","accessRestrictions","observationRisk","regulatoryBasis","culturalConsiderations","technologyControls","signage","monitoringRestrictions","retentionPolicy","breachResponse","ownerId"]) as unknown as PrivacyRequirement;
}

export function parseProcess(value: unknown, at = "$"): Process {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","trigger","inputs","outputs","steps","actors","equipmentIds","elementIds","duration","frequency","criticalPath","bottlenecks","dependencies","kpis","automationLevel","failureModes","improvementOpportunities","regulatoryRefs","ownerId","workflowType","handoffPoints","qualityGates"], ["id","name","status","priority","ownership","timestamps","code","category","trigger","inputs","outputs","steps","actors","equipmentIds","elementIds","duration","frequency","criticalPath","bottlenecks","dependencies","kpis","automationLevel","failureModes","improvementOpportunities","regulatoryRefs","ownerId","workflowType","handoffPoints","qualityGates"]) as unknown as Process;
}

export function parseProgramElement(value: unknown, at = "$"): ProgramElement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","kind","parentId","level","area","volume","height","occupancy","functionIds","activityIds","userProfileIds","adjacencyIds","quantityIds","requirementIds","locationHint","orientation","daylightRequirement","acousticClass","securityZone","flexibilityNotes","growthAllocation","circulationRole","visibilityLevel","adjacencyPreferences","environmentalZone"], ["id","name","status","priority","ownership","timestamps","code","kind","parentId","level","area","volume","height","occupancy","functionIds","activityIds","userProfileIds","adjacencyIds","quantityIds","requirementIds","locationHint","orientation","daylightRequirement","acousticClass","securityZone","flexibilityNotes","growthAllocation","circulationRole","visibilityLevel","adjacencyPreferences","environmentalZone"]);
  parseQuantitySpec(row.area,at+".area");
  parseQuantitySpec(row.volume,at+".volume");
  parseQuantitySpec(row.height,at+".height");
  parseQuantitySpec(row.occupancy,at+".occupancy");
  return row as unknown as ProgramElement;
}

export function parseProgramMeta(value: unknown, at = "$"): ProgramMeta {
  return architectProgramArtifactGuardExactObject(value, at, ["schema","documentId","title","subtitle","purpose","terminology","classification","industrySector","projectType","locale","revision","authorIds","sourceSystem","exportProfile","timestamps"], ["schema","documentId","title","subtitle","purpose","terminology","classification","industrySector","projectType","locale","revision","authorIds","sourceSystem","exportProfile","timestamps"]) as unknown as ProgramMeta;
}

export function parseProjectDefinition(value: unknown, at = "$"): ProjectDefinition {
  return architectProgramArtifactGuardExactObject(value, at, ["id","code","clientName","ownerOrganization","briefSummary","problemStatement","vision","mission","objectives","successCriteria","projectPriorities","completionCriteria","decisionCriteria","scopeInclusions","scopeExclusions","assumptions","constraintsSummary","dependencies","deliverables","phases","geographicContext","developmentContext","operationalContext","regulatoryContext","fundingModel","ownership","timestamps"], ["id","code","clientName","ownerOrganization","briefSummary","problemStatement","vision","mission","objectives","successCriteria","projectPriorities","completionCriteria","decisionCriteria","scopeInclusions","scopeExclusions","assumptions","constraintsSummary","dependencies","deliverables","phases","geographicContext","developmentContext","operationalContext","regulatoryContext","fundingModel","ownership","timestamps"]) as unknown as ProjectDefinition;
}

export function parseQualityRecord(value: unknown, at = "$"): QualityRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","qualityTopic","standard","targetLevel","inspectionPoints","acceptanceCriteria","testingRequirements","sampleRate","defectCategories","correctiveActionProcess","elementIds","requirementIds","supplierRequirements","documentationRequirements","trainingRequirements","auditSchedule","kpis","ownerId","certificationTargets","continuousImprovement"], ["id","name","status","priority","ownership","timestamps","qualityTopic","standard","targetLevel","inspectionPoints","acceptanceCriteria","testingRequirements","sampleRate","defectCategories","correctiveActionProcess","elementIds","requirementIds","supplierRequirements","documentationRequirements","trainingRequirements","auditSchedule","kpis","ownerId","certificationTargets","continuousImprovement"]) as unknown as QualityRecord;
}

export function parseQuantityRequirement(value: unknown, at = "$"): QuantityRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","targetElementId","metric","quantity","basis","calculationMethod","source","benchmarkRef","tolerancePercent","peakFactor","growthFactor","unitCost","currency","verificationMethod","relatedRequirementIds","assumptions","constraints","schedulePhase","responsibleParty","lastVerified","varianceNotes"], ["id","name","status","priority","ownership","timestamps","targetElementId","metric","quantity","basis","calculationMethod","source","benchmarkRef","tolerancePercent","peakFactor","growthFactor","unitCost","currency","verificationMethod","relatedRequirementIds","assumptions","constraints","schedulePhase","responsibleParty","lastVerified","varianceNotes"]);
  nullableBinary64(row.tolerancePercent, at+".tolerancePercent");
  nullableBinary64(row.peakFactor, at+".peakFactor");
  nullableBinary64(row.growthFactor, at+".growthFactor");
  nullableBinary64(row.unitCost, at+".unitCost");
  parseQuantitySpec(row.quantity,at+".quantity");
  return row as unknown as QuantityRequirement;
}

export function parseRegulatoryRequirement(value: unknown, at = "$"): RegulatoryRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","jurisdiction","code","clause","title","requirementText","applicability","elementIds","complianceMethod","evidenceRequired","authority","effectiveDate","expiryDate","penalties","exemptions","relatedRequirementIds","interpretationNotes","verificationStatus","consultantRefs","updateSource"], ["id","name","status","priority","ownership","timestamps","jurisdiction","code","clause","title","requirementText","applicability","elementIds","complianceMethod","evidenceRequired","authority","effectiveDate","expiryDate","penalties","exemptions","relatedRequirementIds","interpretationNotes","verificationStatus","consultantRefs","updateSource"]) as unknown as RegulatoryRequirement;
}

export function parseRelationship(value: unknown, at = "$"): Relationship {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","sourceId","targetId","kind","strength","directional","rationale","constraints","conditions","relationshipPriority","validFrom","validUntil","evidence","conflictIds","traceLinks","bidirectional","distanceConstraintM","capacityConstraint","regulatoryBasis","reviewCycle","ownerId","proximityRequirement","compatibilityRequirement","incompatibilityRequirement","separationRequirements"], ["id","name","status","priority","ownership","timestamps","sourceId","targetId","kind","strength","directional","rationale","constraints","conditions","relationshipPriority","validFrom","validUntil","evidence","conflictIds","traceLinks","bidirectional","distanceConstraintM","capacityConstraint","regulatoryBasis","reviewCycle","ownerId","proximityRequirement","compatibilityRequirement","incompatibilityRequirement","separationRequirements"]);
  nullableBinary64(row.strength, at+".strength");
  nullableBinary64(row.distanceConstraintM, at+".distanceConstraintM");
  return row as unknown as Relationship;
}

export function parseReportRecord(value: unknown, at = "$"): ReportRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","kind","title","audience","sections","generatedAt","generatedBy","analysisIds","format","fileRef","distributionList","approvalStatus","approverId","version","templateId","parameters","confidentiality","expiryDate","relatedDecisionIds"], ["id","name","status","priority","ownership","timestamps","kind","title","audience","sections","generatedAt","generatedBy","analysisIds","format","fileRef","distributionList","approvalStatus","approverId","version","templateId","parameters","confidentiality","expiryDate","relatedDecisionIds"]) as unknown as ReportRecord;
}

export function parseRequirement(value: unknown, at = "$"): Requirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","kind","statement","rationale","source","stakeholderIds","elementIds","functionIds","parentRequirementId","childRequirementIds","acceptanceCriteria","verificationMethod","validationStatus","conflictIds","riskIds","costEstimate","scheduleConstraint","regulatoryRefs","traceLinks","supersededBy"], ["id","name","status","priority","ownership","timestamps","code","kind","statement","rationale","source","stakeholderIds","elementIds","functionIds","parentRequirementId","childRequirementIds","acceptanceCriteria","verificationMethod","validationStatus","conflictIds","riskIds","costEstimate","scheduleConstraint","regulatoryRefs","traceLinks","supersededBy"]);
  nullableBinary64(row.costEstimate, at+".costEstimate");
  return row as unknown as Requirement;
}

export function parseResilienceRequirement(value: unknown, at = "$"): ResilienceRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","hazard","riskLevel","scenario","recoveryTime","recoveryPoint","redundancy","hardeningMeasures","backupSystems","alternateSites","supplyChain","communicationPlan","drillRequirements","elementIds","infrastructureIds","standards","insuranceImplications","climateAdaptation","ownerId","verificationPlan"], ["id","name","status","priority","ownership","timestamps","hazard","riskLevel","scenario","recoveryTime","recoveryPoint","redundancy","hardeningMeasures","backupSystems","alternateSites","supplyChain","communicationPlan","drillRequirements","elementIds","infrastructureIds","standards","insuranceImplications","climateAdaptation","ownerId","verificationPlan"]) as unknown as ResilienceRequirement;
}

export function parseResource(value: unknown, at = "$"): Resource {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","category","resourceType","quantity","mobility","sharingModel","allocation","elementIds","activityIds","userProfileIds","storageRequirementId","durability","cleaningRequirements","replacementCycle","costPerUnit","supplier","standards","ergonomicNotes","customization","disposalNotes","furnitureClass","ergonomicsRating","sharingRatio"], ["id","name","status","priority","ownership","timestamps","code","category","resourceType","quantity","mobility","sharingModel","allocation","elementIds","activityIds","userProfileIds","storageRequirementId","durability","cleaningRequirements","replacementCycle","costPerUnit","supplier","standards","ergonomicNotes","customization","disposalNotes","furnitureClass","ergonomicsRating","sharingRatio"]);
  nullableBinary64(row.costPerUnit, at+".costPerUnit");
  nullableBinary64(row.sharingRatio, at+".sharingRatio");
  parseQuantitySpec(row.quantity,at+".quantity");
  return row as unknown as Resource;
}

export function parseRisk(value: unknown, at = "$"): Risk {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","riskStatement","category","probability","impact","riskScore","causes","effects","affectedElementIds","affectedRequirementIds","mitigation","contingency","ownerId","reviewDate","triggerIndicators","residualProbability","residualImpact","relatedConflictIds","escalationPath","monitoringPlan"], ["id","name","status","priority","ownership","timestamps","riskStatement","category","probability","impact","riskScore","causes","effects","affectedElementIds","affectedRequirementIds","mitigation","contingency","ownerId","reviewDate","triggerIndicators","residualProbability","residualImpact","relatedConflictIds","escalationPath","monitoringPlan"]);
  nullableBinary64(row.riskScore, at+".riskScore");
  return row as unknown as Risk;
}

export function parseSafetyRequirement(value: unknown, at = "$"): SafetyRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","safetyDomain","hazard","riskLevel","affectedElementIds","affectedUserIds","mitigationMeasures","ppeRequirements","emergencyProcedures","evacuationRequirements","fireProtection","structuralSafety","slipTripFall","chemicalSafety","electricalSafety","machinerySafety","standards","inspectionFrequency","trainingRequirements","incidentReporting","residualRisk"], ["id","name","status","priority","ownership","timestamps","safetyDomain","hazard","riskLevel","affectedElementIds","affectedUserIds","mitigationMeasures","ppeRequirements","emergencyProcedures","evacuationRequirements","fireProtection","structuralSafety","slipTripFall","chemicalSafety","electricalSafety","machinerySafety","standards","inspectionFrequency","trainingRequirements","incidentReporting","residualRisk"]) as unknown as SafetyRequirement;
}

export function parseScenario(value: unknown, at = "$"): Scenario {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","code","hypothesis","assumptions","variables","elementIds","requirementIds","growthPlanId","probability","impactSummary","costDelta","areaDelta","headcountDelta","scheduleDelta","riskIds","optionIds","baseline","preferred","analysisIds","ownerId"], ["id","name","status","priority","ownership","timestamps","code","hypothesis","assumptions","variables","elementIds","requirementIds","growthPlanId","probability","impactSummary","costDelta","areaDelta","headcountDelta","scheduleDelta","riskIds","optionIds","baseline","preferred","analysisIds","ownerId"]);
  nullableBinary64(row.probability, at+".probability");
  nullableBinary64(row.costDelta, at+".costDelta");
  nullableBinary64(row.areaDelta, at+".areaDelta");
  nullableBinary64(row.headcountDelta, at+".headcountDelta");
  return row as unknown as Scenario;
}

export function parseScheduleRequirement(value: unknown, at = "$"): ScheduleRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","milestone","phase","startDate","endDate","duration","dependencies","predecessors","successors","critical","floatDays","resourceRequirements","occupancyImpact","phasingStrategy","decantRequirements","commissioningWindow","stakeholderIds","riskIds","contingencyDays","reportingCadence","ownerId"], ["id","name","status","priority","ownership","timestamps","milestone","phase","startDate","endDate","duration","dependencies","predecessors","successors","critical","floatDays","resourceRequirements","occupancyImpact","phasingStrategy","decantRequirements","commissioningWindow","stakeholderIds","riskIds","contingencyDays","reportingCadence","ownerId"]);
  if (row["floatDays"] !== null) architectProgramArtifactGuardInteger(row["floatDays"], `${at}.floatDays`, { minimum: 0, maximum: 4294967295 });
  if (row["contingencyDays"] !== null) architectProgramArtifactGuardInteger(row["contingencyDays"], `${at}.contingencyDays`, { minimum: 0, maximum: 4294967295 });
  return row as unknown as ScheduleRequirement;
}

export function parseSearchFilter(value: unknown, at = "$"): SearchFilter {
  const row=architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","filterName","filterDescription","keywords","categories","ownerIds","statuses","priorities","sources","dateFrom","dateTo","entityKinds","tagFilters","sortField","sortDirection","isPublic","createdBy","lastUsed","useCount","pinned"], ["id","name","status","priority","ownership","timestamps","filterName","filterDescription","keywords","categories","ownerIds","statuses","priorities","sources","dateFrom","dateTo","entityKinds","tagFilters","sortField","sortDirection","isPublic","createdBy","lastUsed","useCount","pinned"]);
  architectProgramArtifactGuardUnsigned64(row["useCount"],`${at}.useCount`);
  return row as unknown as SearchFilter;
}

export function parseSecurityRequirement(value: unknown, at = "$"): SecurityRequirement {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","controlKind","threat","riskLevel","assetIds","zoneIds","accessLevel","perimeterControls","surveillance","intrusionDetection","cybersecurity","screening","visitorManagement","keyManagement","standards","responseProcedures","drillFrequency","liaisonContacts","classifiedLevel","redundancy","auditRequirements"], ["id","name","status","priority","ownership","timestamps","controlKind","threat","riskLevel","assetIds","zoneIds","accessLevel","perimeterControls","surveillance","intrusionDetection","cybersecurity","screening","visitorManagement","keyManagement","standards","responseProcedures","drillFrequency","liaisonContacts","classifiedLevel","redundancy","auditRequirements"]) as unknown as SecurityRequirement;
}

export function parseServiceRequirement(value: unknown, at = "$"): ServiceRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","serviceName","serviceType","provider","serviceLevel","operatingHours","capacity","responseTime","queueManagement","customerProfiles","elementIds","equipmentIds","staffing","qualityMetrics","costModel","contractRefs","dependencies","failureImpact","backupService","feedbackChannels"], ["id","name","status","priority","ownership","timestamps","serviceName","serviceType","provider","serviceLevel","operatingHours","capacity","responseTime","queueManagement","customerProfiles","elementIds","equipmentIds","staffing","qualityMetrics","costModel","contractRefs","dependencies","failureImpact","backupService","feedbackChannels"]);
  parseQuantitySpec(row.capacity,at+".capacity");
  parseQuantitySpec(row.staffing,at+".staffing");
  return row as unknown as ServiceRequirement;
}

export function parseSiteContext(value: unknown, at = "$"): SiteContext {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","siteName","address","latitude","longitude","elevationM","climateZone","seismicZone","floodRisk","soilConditions","utilitiesAvailable","accessRoads","publicTransit","neighbors","views","noiseSources","environmentalConstraints","heritageConstraints","zoning","maxHeightM","maxCoverage"], ["id","name","status","priority","ownership","timestamps","siteName","address","latitude","longitude","elevationM","climateZone","seismicZone","floodRisk","soilConditions","utilitiesAvailable","accessRoads","publicTransit","neighbors","views","noiseSources","environmentalConstraints","heritageConstraints","zoning","maxHeightM","maxCoverage"]);
  nullableBinary64(row.latitude, at+".latitude");
  nullableBinary64(row.longitude, at+".longitude");
  nullableBinary64(row.elevationM, at+".elevationM");
  nullableBinary64(row.maxHeightM, at+".maxHeightM");
  nullableBinary64(row.maxCoverage, at+".maxCoverage");
  return row as unknown as SiteContext;
}

export function parseStakeholder(value: unknown, at = "$"): Stakeholder {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","role","organization","department","contactEmail","contactPhone","influence","interest","engagement","expectations","concerns","requirementIds","decisionAuthority","communicationPreferences","reportingFrequency","involvementPhases","availability","representativeOf","delegatedTo","relationshipToClient","powerInterestNotes","stakeholderType","influenceStrategy","communicationChannels","successMetrics"], ["id","name","status","priority","ownership","timestamps","role","organization","department","contactEmail","contactPhone","influence","interest","engagement","expectations","concerns","requirementIds","decisionAuthority","communicationPreferences","reportingFrequency","involvementPhases","availability","representativeOf","delegatedTo","relationshipToClient","powerInterestNotes","stakeholderType","influenceStrategy","communicationChannels","successMetrics"]) as unknown as Stakeholder;
}

export function parseStatusRecord(value: unknown, at = "$"): StatusRecord {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectId","subjectKind","recordStatus","previousStatus","changedBy","changedAt","reason","blockers","nextActions","dueDate","progressPercent","health","escalationLevel","relatedIssueIds","relatedRiskIds","milestoneId","reportingPeriod","statusNotes"], ["id","name","status","priority","ownership","timestamps","subjectId","subjectKind","recordStatus","previousStatus","changedBy","changedAt","reason","blockers","nextActions","dueDate","progressPercent","health","escalationLevel","relatedIssueIds","relatedRiskIds","milestoneId","reportingPeriod","statusNotes"]);
  nullableBinary64(row.progressPercent, at+".progressPercent");
  return row as unknown as StatusRecord;
}

export function parseStorageRequirement(value: unknown, at = "$"): StorageRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","storedItem","storageClass","quantity","volumeM3","weightKg","temperatureRange","humidityRange","securityLevel","hazardClass","retentionPeriod","accessFrequency","elementIds","equipmentIds","handlingEquipment","fireProtection","ventilation","organizationSystem","growthAllowance","regulatoryRefs","ownerId"], ["id","name","status","priority","ownership","timestamps","storedItem","storageClass","quantity","volumeM3","weightKg","temperatureRange","humidityRange","securityLevel","hazardClass","retentionPeriod","accessFrequency","elementIds","equipmentIds","handlingEquipment","fireProtection","ventilation","organizationSystem","growthAllowance","regulatoryRefs","ownerId"]);
  nullableBinary64(row.volumeM3, at+".volumeM3");
  nullableBinary64(row.weightKg, at+".weightKg");
  nullableBinary64(row.growthAllowance, at+".growthAllowance");
  parseQuantitySpec(row.quantity,at+".quantity");
  return row as unknown as StorageRequirement;
}

export function parseSurvey(value: unknown, at = "$"): Survey {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","surveyType","title","objectives","questions","targetAudience","distributionChannels","launchDate","closeDate","responseCount","responseRate","findings","themes","recommendations","confidentiality","consentProcess","analysisId","workshopId","ownerId","surveyStatus"], ["id","name","status","priority","ownership","timestamps","surveyType","title","objectives","questions","targetAudience","distributionChannels","launchDate","closeDate","responseCount","responseRate","findings","themes","recommendations","confidentiality","consentProcess","analysisId","workshopId","ownerId","surveyStatus"]);
  architectProgramArtifactGuardInteger(row["responseCount"], `${at}.responseCount`, { minimum: 0, maximum: 4294967295 });
  nullableBinary64(row.responseRate, at+".responseRate");
  return row as unknown as Survey;
}

export function parseSustainabilityRequirement(value: unknown, at = "$"): SustainabilityRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","topic","target","metric","baseline","targetValue","unit","certification","standards","elementIds","strategies","materialsPreferences","energyStrategy","waterStrategy","wasteStrategy","biodiversity","embodiedCarbon","operationalCarbon","reportingRequirements","verificationPlan","ownerId"], ["id","name","status","priority","ownership","timestamps","topic","target","metric","baseline","targetValue","unit","certification","standards","elementIds","strategies","materialsPreferences","energyStrategy","waterStrategy","wasteStrategy","biodiversity","embodiedCarbon","operationalCarbon","reportingRequirements","verificationPlan","ownerId"]);
  nullableBinary64(row.baseline, at+".baseline");
  nullableBinary64(row.targetValue, at+".targetValue");
  nullableBinary64(row.embodiedCarbon, at+".embodiedCarbon");
  nullableBinary64(row.operationalCarbon, at+".operationalCarbon");
  return row as unknown as SustainabilityRequirement;
}

export function parseTemplateRecord(value: unknown, at = "$"): TemplateRecord {
  const row=architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","templateType","sector","projectType","version","contentRef","entityKinds","defaultFields","checklists","standards","applicability","authorId","approvalStatus","usageCount","lastApplied","customizationNotes","relatedKnowledgeIds","benchmarkIds","license","sourceOrganization"], ["id","name","status","priority","ownership","timestamps","templateType","sector","projectType","version","contentRef","entityKinds","defaultFields","checklists","standards","applicability","authorId","approvalStatus","usageCount","lastApplied","customizationNotes","relatedKnowledgeIds","benchmarkIds","license","sourceOrganization"]);
  architectProgramArtifactGuardUnsigned64(row["usageCount"],`${at}.usageCount`);
  return row as unknown as TemplateRecord;
}

export function parseTraceLink(value: unknown, at = "$"): TraceLink {
  return architectProgramArtifactGuardExactObject(value, at, ["id","fromId","toId","kind","label"], ["id","fromId","toId","kind"]) as unknown as TraceLink;
}

export function parseUserProfile(value: unknown, at = "$"): UserProfile {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","category","demographic","ageRange","abilities","disabilities","occupation","roleTitle","department","mobilityProfile","sensoryProfile","cognitiveProfile","behavioralPatterns","usageFrequency","usageDuration","peakUsageTimes","technologyProficiency","preferences","painPoints","goals","activityIds","researchMethod","personaArchetype","validated","stakeholderIds"], ["id","name","status","priority","ownership","timestamps","category","demographic","ageRange","abilities","disabilities","occupation","roleTitle","department","mobilityProfile","sensoryProfile","cognitiveProfile","behavioralPatterns","usageFrequency","usageDuration","peakUsageTimes","technologyProficiency","preferences","painPoints","goals","activityIds","researchMethod","personaArchetype","validated","stakeholderIds"]) as unknown as UserProfile;
}

export function parseValidationRecord(value: unknown, at = "$"): ValidationRecord {
  return architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","subjectId","subjectKind","validationType","method","criteria","result","evidence","validatorIds","validationDate","nextReviewDate","findings","nonConformities","correctiveActions","waivers","standards","traceLinks","reportId","confidenceLevel","validationNotes"], ["id","name","status","priority","ownership","timestamps","subjectId","subjectKind","validationType","method","criteria","result","evidence","validatorIds","validationDate","nextReviewDate","findings","nonConformities","correctiveActions","waivers","standards","traceLinks","reportId","confidenceLevel","validationNotes"]) as unknown as ValidationRecord;
}

export function parseWayfindingRequirement(value: unknown, at = "$"): WayfindingRequirement {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","userProfileIds","elementIds","destinationTypes","signageTypes","languages","tactileRequired","audioRequired","digitalWayfinding","landmarkStrategy","colorCoding","symbolStandards","decisionPoints","maximumSignageDistanceM","lightingRequirements","maintenancePlan","emergencyEgress","visitorJourney","staffJourney","brandIntegration"], ["id","name","status","priority","ownership","timestamps","userProfileIds","elementIds","destinationTypes","signageTypes","languages","tactileRequired","audioRequired","digitalWayfinding","landmarkStrategy","colorCoding","symbolStandards","decisionPoints","maximumSignageDistanceM","lightingRequirements","maintenancePlan","emergencyEgress","visitorJourney","staffJourney","brandIntegration"]);
  nullableBinary64(row.maximumSignageDistanceM, at+".maximumSignageDistanceM");
  return row as unknown as WayfindingRequirement;
}

export function parseWorkshop(value: unknown, at = "$"): Workshop {
  const row = architectProgramArtifactGuardHeaderObject(value, at, ["id","name","description","status","priority","ownership","tags","notes","timestamps","workshopType","objectives","agenda","facilitatorId","participants","scheduledStart","scheduledEnd","location","materials","methods","outputs","decisions","issues","followUpActions","feedback","recordingRef","budget","workshopStatus","surveyIds"], ["id","name","status","priority","ownership","timestamps","workshopType","objectives","agenda","facilitatorId","participants","scheduledStart","scheduledEnd","location","materials","methods","outputs","decisions","issues","followUpActions","feedback","recordingRef","budget","workshopStatus","surveyIds"]);
  nullableBinary64(row.budget, at+".budget");
  return row as unknown as Workshop;
}
