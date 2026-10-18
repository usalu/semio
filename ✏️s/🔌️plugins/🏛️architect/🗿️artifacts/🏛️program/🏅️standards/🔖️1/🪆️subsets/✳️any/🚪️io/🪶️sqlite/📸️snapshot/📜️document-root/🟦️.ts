/** 📜️ Document level singletons of the Architect SQLite projection: meta, project definition, governance and child slots. */
import type { ArtifactChild } from "../../../../🧬️schema/🟦️.ts";
import type { ProgramMeta, ProjectDefinition } from "../../../../🧬️schema/🪪️entity-foundation/🟦️.ts";
import type { Governance } from "../../../../🧬️schema/🗃️decision-records/🟦️.ts";
import { ArtifactSqliteProjection } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import type { SqliteRow } from "@semio-tech/framework/io/sqlite-snapshot";
import { Cursor, fail, optional, symbol, text, textField, timestamp } from "../📏️cells/🟦️.ts";
import { Priority } from "../🏷️symbols/🟦️.ts";
import { ordered, Reader } from "../💰️backing/🟦️.ts";

export async function projectProgramMeta(p:ArtifactSqliteProjection,v:ProgramMeta):Promise<void>{
 await p.insert("architect_meta",[1n,...text(v.schema),...text(v.documentId),...text(v.title),...optional(v.subtitle,1,text),...textField(v.purpose),...text(v.industrySector),...text(v.projectType),...text(v.locale),...text(v.revision),...optional(v.sourceSystem,1,text),...optional(v.exportProfile,1,text),...timestamp(v.timestamps)],1n);
 await ordered(p,"architect_meta_term",1n,v.terminology,text);
 await ordered(p,"architect_meta_classification",1n,v.classification,text);
 await ordered(p,"architect_meta_author",1n,v.authorIds,text);
}
export async function restoreProgramMeta(r:Reader):Promise<ProgramMeta>{
 const row=await r.singleton("architect_meta",18),c=new Cursor(row!);if(c.integer()!==1n)fail("core document owner differs");
 const schema=c.text();
 const documentId=c.text();
 const title=c.text();
 const subtitle=c.optional(1,()=>c.text());
 const purpose=c.textField();
 const industrySector=c.text();
 const projectType=c.text();
 const locale=c.text();
 const revision=c.text();
 const sourceSystem=c.optional(1,()=>c.text());
 const exportProfile=c.optional(1,()=>c.text());
 const timestamps=c.timestamp();
 c.end();
 const terminology=await r.list("architect_meta_term",1n,1,c=>c.text());
 const classification=await r.list("architect_meta_classification",1n,1,c=>c.text());
 const authorIds=await r.list("architect_meta_author",1n,1,c=>c.text());
 return{schema,documentId,title,subtitle,purpose,industrySector,projectType,locale,revision,sourceSystem,exportProfile,timestamps,terminology,classification,authorIds};
}
export async function projectProjectDefinition(p:ArtifactSqliteProjection,v:ProjectDefinition):Promise<void>{
 await p.insert("architect_project",[1n,...text(v.id),...text(v.code),...text(v.clientName),...text(v.ownerOrganization),...textField(v.briefSummary),...textField(v.problemStatement),...textField(v.vision),...textField(v.mission),...textField(v.geographicContext),...textField(v.developmentContext),...textField(v.operationalContext),...text(v.fundingModel),...optional(v.ownership.ownerId,1,text),...optional(v.ownership.authorityId,1,text),...timestamp(v.timestamps)],1n);
 await ordered(p,"architect_project_objective",1n,v.objectives,text);
 await ordered(p,"architect_project_success_criterion",1n,v.successCriteria,text);
 await ordered(p,"architect_project_priority",1n,v.projectPriorities,value=>symbol(value,Priority));
 await ordered(p,"architect_project_completion_criterion",1n,v.completionCriteria,text);
 await ordered(p,"architect_project_decision_criterion",1n,v.decisionCriteria,text);
 await ordered(p,"architect_project_scope_inclusion",1n,v.scopeInclusions,text);
 await ordered(p,"architect_project_scope_exclusion",1n,v.scopeExclusions,text);
 await ordered(p,"architect_project_assumption",1n,v.assumptions,text);
 await ordered(p,"architect_project_constraint",1n,v.constraintsSummary,text);
 await ordered(p,"architect_project_dependency",1n,v.dependencies,text);
 await ordered(p,"architect_project_deliverable",1n,v.deliverables,text);
 await ordered(p,"architect_project_phase",1n,v.phases,text);
 await ordered(p,"architect_project_regulatory_context",1n,v.regulatoryContext,text);
 await ordered(p,"architect_project_consultant",1n,v.ownership.consultantIds,text);
 await ordered(p,"architect_project_participant",1n,v.ownership.participantIds,text);
}
export async function restoreProjectDefinition(r:Reader):Promise<ProjectDefinition>{
 const row=await r.singleton("architect_project",27),c=new Cursor(row!);if(c.integer()!==1n)fail("core document owner differs");
 const id=c.text();
 const code=c.text();
 const clientName=c.text();
 const ownerOrganization=c.text();
 const briefSummary=c.textField();
 const problemStatement=c.textField();
 const vision=c.textField();
 const mission=c.textField();
 const geographicContext=c.textField();
 const developmentContext=c.textField();
 const operationalContext=c.textField();
 const fundingModel=c.text();
 const ownerId=c.optional(1,()=>c.text());
 const authorityId=c.optional(1,()=>c.text());
 const timestamps=c.timestamp();
 c.end();
 const objectives=await r.list("architect_project_objective",1n,1,c=>c.text());
 const successCriteria=await r.list("architect_project_success_criterion",1n,1,c=>c.text());
 const projectPriorities=await r.list("architect_project_priority",1n,1,c=>c.symbol(Priority));
 const completionCriteria=await r.list("architect_project_completion_criterion",1n,1,c=>c.text());
 const decisionCriteria=await r.list("architect_project_decision_criterion",1n,1,c=>c.text());
 const scopeInclusions=await r.list("architect_project_scope_inclusion",1n,1,c=>c.text());
 const scopeExclusions=await r.list("architect_project_scope_exclusion",1n,1,c=>c.text());
 const assumptions=await r.list("architect_project_assumption",1n,1,c=>c.text());
 const constraintsSummary=await r.list("architect_project_constraint",1n,1,c=>c.text());
 const dependencies=await r.list("architect_project_dependency",1n,1,c=>c.text());
 const deliverables=await r.list("architect_project_deliverable",1n,1,c=>c.text());
 const phases=await r.list("architect_project_phase",1n,1,c=>c.text());
 const regulatoryContext=await r.list("architect_project_regulatory_context",1n,1,c=>c.text());
 const consultantIds=await r.list("architect_project_consultant",1n,1,c=>c.text());
 const participantIds=await r.list("architect_project_participant",1n,1,c=>c.text());
 return{id,code,clientName,ownerOrganization,briefSummary,problemStatement,vision,mission,geographicContext,developmentContext,operationalContext,fundingModel,timestamps,objectives,successCriteria,projectPriorities,completionCriteria,decisionCriteria,scopeInclusions,scopeExclusions,assumptions,constraintsSummary,dependencies,deliverables,phases,regulatoryContext,ownership:{ownerId,authorityId,consultantIds,participantIds}};
}
export async function projectGovernance(p:ArtifactSqliteProjection,v:Governance):Promise<void>{
 await p.insert("architect_governance",[1n,...text(v.id),...text(v.framework),...textField(v.qualityPolicy),...optional(v.riskAppetite,1,text),...optional(v.auditSchedule,1,text),...optional(v.ownerId,1,text),...optional(v.reviewCycle,1,text),...optional(v.policyOwnershipId,1,text),...optional(v.requirementOwnershipId,1,text),...optional(v.riskOwnershipId,1,text),...optional(v.reportingFrequency,1,text)],1n);
 await ordered(p,"architect_governance_role",1n,v.roles,text);
 await ordered(p,"architect_governance_responsibility",1n,v.responsibilities,text);
 await ordered(p,"architect_governance_approval",1n,v.approvalMatrix,text);
 await ordered(p,"architect_governance_escalation",1n,v.escalationPaths,text);
 await ordered(p,"architect_governance_meeting_cadence",1n,v.meetingCadence,text);
 await ordered(p,"architect_governance_decision_right",1n,v.decisionRights,text);
 await ordered(p,"architect_governance_change_control",1n,v.changeControlProcess,text);
 await ordered(p,"architect_governance_compliance_obligation",1n,v.complianceObligations,text);
 await ordered(p,"architect_governance_document_control",1n,v.documentControl,text);
 await ordered(p,"architect_governance_stakeholder_engagement",1n,v.stakeholderEngagementPlan,text);
 await ordered(p,"architect_governance_ethics_policy",1n,v.ethicsPolicy,text);
 await ordered(p,"architect_governance_data_governance",1n,v.dataGovernance,text);
 await ordered(p,"architect_governance_review_hierarchy",1n,v.reviewHierarchy,text);
 await ordered(p,"architect_governance_accountability",1n,v.accountabilityRules,text);
 await ordered(p,"architect_governance_exception",1n,v.exceptionManagement,text);
 await ordered(p,"architect_governance_performance",1n,v.governancePerformance,text);
}
export async function restoreGovernance(r:Reader):Promise<Governance>{
 const row=await r.singleton("architect_governance",14),c=new Cursor(row!);if(c.integer()!==1n)fail("core document owner differs");
 const id=c.text();
 const framework=c.text();
 const qualityPolicy=c.textField();
 const riskAppetite=c.optional(1,()=>c.text());
 const auditSchedule=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const reviewCycle=c.optional(1,()=>c.text());
 const policyOwnershipId=c.optional(1,()=>c.text());
 const requirementOwnershipId=c.optional(1,()=>c.text());
 const riskOwnershipId=c.optional(1,()=>c.text());
 const reportingFrequency=c.optional(1,()=>c.text());
 c.end();
 const roles=await r.list("architect_governance_role",1n,1,c=>c.text());
 const responsibilities=await r.list("architect_governance_responsibility",1n,1,c=>c.text());
 const approvalMatrix=await r.list("architect_governance_approval",1n,1,c=>c.text());
 const escalationPaths=await r.list("architect_governance_escalation",1n,1,c=>c.text());
 const meetingCadence=await r.list("architect_governance_meeting_cadence",1n,1,c=>c.text());
 const decisionRights=await r.list("architect_governance_decision_right",1n,1,c=>c.text());
 const changeControlProcess=await r.list("architect_governance_change_control",1n,1,c=>c.text());
 const complianceObligations=await r.list("architect_governance_compliance_obligation",1n,1,c=>c.text());
 const documentControl=await r.list("architect_governance_document_control",1n,1,c=>c.text());
 const stakeholderEngagementPlan=await r.list("architect_governance_stakeholder_engagement",1n,1,c=>c.text());
 const ethicsPolicy=await r.list("architect_governance_ethics_policy",1n,1,c=>c.text());
 const dataGovernance=await r.list("architect_governance_data_governance",1n,1,c=>c.text());
 const reviewHierarchy=await r.list("architect_governance_review_hierarchy",1n,1,c=>c.text());
 const accountabilityRules=await r.list("architect_governance_accountability",1n,1,c=>c.text());
 const exceptionManagement=await r.list("architect_governance_exception",1n,1,c=>c.text());
 const governancePerformance=await r.list("architect_governance_performance",1n,1,c=>c.text());
 return{id,framework,qualityPolicy,riskAppetite,auditSchedule,ownerId,reviewCycle,policyOwnershipId,requirementOwnershipId,riskOwnershipId,reportingFrequency,roles,responsibilities,approvalMatrix,escalationPaths,meetingCadence,decisionRights,changeControlProcess,complianceObligations,documentControl,stakeholderEngagementPlan,ethicsPolicy,dataGovernance,reviewHierarchy,accountabilityRules,exceptionManagement,governancePerformance};
}
export async function projectChild(p:ArtifactSqliteProjection,slot:string,value:ArtifactChild):Promise<void>{await p.insert("architect_child",[1n,slot,...text(value.childId),...text(value.target.artifactId),...text(value.target.dialect.artifactKind),...text(value.target.dialect.standard),...text(value.target.dialect.subset)])}
export async function restoreChild(r:Reader,slot:string):Promise<ArtifactChild>{let found:SqliteRow|undefined;for(const row of r.rows("architect_child")){await r.control.step();if(row.values.length!==8||row.rowid<=0n||row.values[0]!==row.rowid||row.values[1]!==1n)fail("child shape or owner differs");if(row.values[2]===slot){if(found)fail("child slot is repeated");found=row}}if(!found)return fail("required child slot is absent");await r.use(found);const c=new Cursor(found,3),childId=c.text(),artifactId=c.text(),artifactKind=c.text(),standard=c.text(),subset=c.text();c.end();return{childId,target:{artifactId,dialect:{artifactKind,standard,subset}}}}
