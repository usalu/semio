/** 🗃️ Decision record registers of the Architect SQLite projection. */
import type { EntityHeader } from "../../../../🧬️schema/🪪️entity-foundation/🟦️.ts";
import type { AnalysisRecord, ApprovalRecord, ArtifactRecord, Assumption, AuditEvent, BenchmarkRecord, ChangeRecord, CollaborationRecord, ComplianceRecord, Conflict, ConstraintRecord, Decision, Issue, KnowledgeRecord, MeetingRecord, OptionEvaluation, PerformanceCriterion, PriorityRecord, QualityRecord, ReportRecord, Requirement, Risk, Scenario, SearchFilter, StatusRecord, Survey, TemplateRecord, ValidationRecord, Workshop } from "../../../../🧬️schema/🗃️decision-records/🟦️.ts";
import { ArtifactSqliteProjection } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import type { SqliteRow } from "@semio-tech/framework/io/sqlite-snapshot";
import { AnalysisKind, AuditAction, ConflictKind, IssueSeverity, LifecycleStatus, Priority, ReportKind, RequirementKind, RiskLevel, ValidationStatus } from "../🏷️symbols/🟦️.ts";
import { boolean, Cursor, float, note, optional, optionalTextField, symbol, text, textField, uint32, uint64 } from "../📏️cells/🟦️.ts";
import { ordered, Reader } from "../💰️backing/🟦️.ts";
import { readTrace, trace } from "../🔗️trace/🟦️.ts";

export async function projectApprovalRecord(p:ArtifactSqliteProjection,id:bigint,v:ApprovalRecord):Promise<void>{
 await p.insert("architect_approval_record",[...text(v.approvalType),...text(v.subjectId),...optional(v.approvalDate,1,text),...symbol(v.approvalStatus,LifecycleStatus),...optional(v.expiryDate,1,text),...optional(v.relatedDecisionId,1,text),...optional(v.relatedChangeId,1,text),...optional(v.signatureMethod,1,text),...optionalTextField(v.rejectionReason),...optional(v.resubmissionDate,1,text),...optional(v.workflowStep,1,text),...optional(v.version,1,text),...optional(v.auditTrailRef,1,text)],id);
 await ordered(p,"architect_approval_approver",id,v.approverIds,text);
 await ordered(p,"architect_approval_condition",id,v.conditions,text);
 await ordered(p,"architect_approval_delegation",id,v.delegationChain,text);
 await ordered(p,"architect_approval_evidence_ref",id,v.evidenceRefs,text);
 await ordered(p,"architect_approval_authority_basis",id,v.authorityBasis,text);
 await ordered(p,"architect_approval_notification",id,v.notificationList,text);
}
export async function restoreApprovalRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ApprovalRecord>{
 const row=await r.body("architect_approval_record",hrow.rowid,16),c=new Cursor(row!);
 const approvalType=c.text();
 const subjectId=c.text();
 const approvalDate=c.optional(1,()=>c.text());
 const approvalStatus=c.symbol(LifecycleStatus);
 const expiryDate=c.optional(1,()=>c.text());
 const relatedDecisionId=c.optional(1,()=>c.text());
 const relatedChangeId=c.optional(1,()=>c.text());
 const signatureMethod=c.optional(1,()=>c.text());
 const rejectionReason=c.optionalTextField();
 const resubmissionDate=c.optional(1,()=>c.text());
 const workflowStep=c.optional(1,()=>c.text());
 const version=c.optional(1,()=>c.text());
 const auditTrailRef=c.optional(1,()=>c.text());
 c.end();
 const approverIds=await r.list("architect_approval_approver",hrow.rowid,1,c=>c.text());
 const conditions=await r.list("architect_approval_condition",hrow.rowid,1,c=>c.text());
 const delegationChain=await r.list("architect_approval_delegation",hrow.rowid,1,c=>c.text());
 const evidenceRefs=await r.list("architect_approval_evidence_ref",hrow.rowid,1,c=>c.text());
 const authorityBasis=await r.list("architect_approval_authority_basis",hrow.rowid,1,c=>c.text());
 const notificationList=await r.list("architect_approval_notification",hrow.rowid,1,c=>c.text());
 return{...h,approvalType,subjectId,approvalDate,approvalStatus,expiryDate,relatedDecisionId,relatedChangeId,signatureMethod,rejectionReason,resubmissionDate,workflowStep,version,auditTrailRef,approverIds,conditions,delegationChain,evidenceRefs,authorityBasis,notificationList};
}
export async function projectMeetingRecord(p:ArtifactSqliteProjection,id:bigint,v:MeetingRecord):Promise<void>{
 await p.insert("architect_meeting_record",[...text(v.meetingType),...optional(v.scheduledDate,1,text),...optional(v.duration,1,text),...optional(v.location,1,text),...optional(v.chairId,1,text),...optionalTextField(v.minutes),...optional(v.followUpDate,1,text),...optional(v.recordingRef,1,text),...boolean(v.quorumMet),...symbol(v.meetingStatus,LifecycleStatus),...optional(v.workshopId,1,text)],id);
 await ordered(p,"architect_meeting_attendee",id,v.attendeeIds,text);
 await ordered(p,"architect_meeting_agenda_item",id,v.agendaItems,text);
 await ordered(p,"architect_meeting_action_item",id,v.actionItems,text);
 await ordered(p,"architect_meeting_decision",id,v.decisionsMade,text);
 await ordered(p,"architect_meeting_artifact_ref",id,v.artifactRefs,text);
 await ordered(p,"architect_meeting_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_meeting_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_meeting_issue",id,v.issueIds,text);
 await ordered(p,"architect_meeting_approval",id,v.approvalIds,text);
}
export async function restoreMeetingRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<MeetingRecord>{
 const row=await r.body("architect_meeting_record",hrow.rowid,14),c=new Cursor(row!);
 const meetingType=c.text();
 const scheduledDate=c.optional(1,()=>c.text());
 const duration=c.optional(1,()=>c.text());
 const location=c.optional(1,()=>c.text());
 const chairId=c.optional(1,()=>c.text());
 const minutes=c.optionalTextField();
 const followUpDate=c.optional(1,()=>c.text());
 const recordingRef=c.optional(1,()=>c.text());
 const quorumMet=c.boolean();
 const meetingStatus=c.symbol(LifecycleStatus);
 const workshopId=c.optional(1,()=>c.text());
 c.end();
 const attendeeIds=await r.list("architect_meeting_attendee",hrow.rowid,1,c=>c.text());
 const agendaItems=await r.list("architect_meeting_agenda_item",hrow.rowid,1,c=>c.text());
 const actionItems=await r.list("architect_meeting_action_item",hrow.rowid,1,c=>c.text());
 const decisionsMade=await r.list("architect_meeting_decision",hrow.rowid,1,c=>c.text());
 const artifactRefs=await r.list("architect_meeting_artifact_ref",hrow.rowid,1,c=>c.text());
 const stakeholderIds=await r.list("architect_meeting_stakeholder",hrow.rowid,1,c=>c.text());
 const requirementIds=await r.list("architect_meeting_requirement",hrow.rowid,1,c=>c.text());
 const issueIds=await r.list("architect_meeting_issue",hrow.rowid,1,c=>c.text());
 const approvalIds=await r.list("architect_meeting_approval",hrow.rowid,1,c=>c.text());
 return{...h,meetingType,scheduledDate,duration,location,chairId,minutes,followUpDate,recordingRef,quorumMet,meetingStatus,workshopId,attendeeIds,agendaItems,actionItems,decisionsMade,artifactRefs,stakeholderIds,requirementIds,issueIds,approvalIds};
}
export async function projectAssumption(p:ArtifactSqliteProjection,id:bigint,v:Assumption):Promise<void>{
 await p.insert("architect_assumption",[...textField(v.statement),...optionalTextField(v.basis),...optional(v.confidenceLevel,1,text),...optionalTextField(v.impactIfFalse),...symbol(v.validationStatus,ValidationStatus),...optional(v.validatedBy,1,text),...optional(v.validationDate,1,text),...optional(v.ownerId,1,text),...optional(v.reviewCycle,1,text),...optional(v.source,1,text),...optional(v.category,1,text),...optional(v.expirationDate,1,text)],id);
 await ordered(p,"architect_assumption_related_entity",id,v.relatedEntityIds,text);
 await ordered(p,"architect_assumption_dependency",id,v.dependencies,text);
 await ordered(p,"architect_assumption_mitigation",id,v.mitigation,text);
 await ordered(p,"architect_assumption_linked_requirement",id,v.linkedRequirementIds,text);
 await ordered(p,"architect_assumption_linked_risk",id,v.linkedRiskIds,text);
 await ordered(p,"architect_assumption_status_note",id,v.statusNotes,note);
 await ordered(p,"architect_assumption_artifact_ref",id,v.artifactRefs,text);
}
export async function restoreAssumption(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Assumption>{
 const row=await r.body("architect_assumption",hrow.rowid,18),c=new Cursor(row!);
 const statement=c.textField();
 const basis=c.optionalTextField();
 const confidenceLevel=c.optional(1,()=>c.text());
 const impactIfFalse=c.optionalTextField();
 const validationStatus=c.symbol(ValidationStatus);
 const validatedBy=c.optional(1,()=>c.text());
 const validationDate=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const reviewCycle=c.optional(1,()=>c.text());
 const source=c.optional(1,()=>c.text());
 const category=c.optional(1,()=>c.text());
 const expirationDate=c.optional(1,()=>c.text());
 c.end();
 const relatedEntityIds=await r.list("architect_assumption_related_entity",hrow.rowid,1,c=>c.text());
 const dependencies=await r.list("architect_assumption_dependency",hrow.rowid,1,c=>c.text());
 const mitigation=await r.list("architect_assumption_mitigation",hrow.rowid,1,c=>c.text());
 const linkedRequirementIds=await r.list("architect_assumption_linked_requirement",hrow.rowid,1,c=>c.text());
 const linkedRiskIds=await r.list("architect_assumption_linked_risk",hrow.rowid,1,c=>c.text());
 const statusNotes=await r.list("architect_assumption_status_note",hrow.rowid,2,c=>c.note());
 const artifactRefs=await r.list("architect_assumption_artifact_ref",hrow.rowid,1,c=>c.text());
 return{...h,statement,basis,confidenceLevel,impactIfFalse,validationStatus,validatedBy,validationDate,ownerId,reviewCycle,source,category,expirationDate,relatedEntityIds,dependencies,mitigation,linkedRequirementIds,linkedRiskIds,statusNotes,artifactRefs};
}
export async function projectConstraintRecord(p:ArtifactSqliteProjection,id:bigint,v:ConstraintRecord):Promise<void>{
 await p.insert("architect_constraint_record",[...text(v.constraintType),...textField(v.summary),...symbol(v.severity,RiskLevel),...optional(v.source,1,text),...optional(v.ownerId,1,text),...optional(v.effectiveDate,1,text),...optional(v.expiryDate,1,text),...optional(v.waiverStatus,1,text),...optional(v.waiverApprover,1,text),...optionalTextField(v.impactAssessment),...optional(v.monitoringFrequency,1,text),...symbol(v.complianceStatus,ValidationStatus),...optional(v.escalationContactId,1,text)],id);
 await ordered(p,"architect_constraint_affected_entity",id,v.affectedEntityIds,text);
 await ordered(p,"architect_constraint_regulatory_basis",id,v.regulatoryBasis,text);
 await ordered(p,"architect_constraint_mitigation_option",id,v.mitigationOptions,text);
 await ordered(p,"architect_constraint_resolution_plan",id,v.resolutionPlan,text);
 await ordered(p,"architect_constraint_related_requirement",id,v.relatedRequirementIds,text);
 await ordered(p,"architect_constraint_related_decision",id,v.relatedDecisionIds,text);
 await ordered(p,"architect_constraint_exception",id,v.exceptions,text);
 await ordered(p,"architect_constraint_trace",id,v.traceLinks,trace);
}
export async function restoreConstraintRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ConstraintRecord>{
 const row=await r.body("architect_constraint_record",hrow.rowid,17),c=new Cursor(row!);
 const constraintType=c.text();
 const summary=c.textField();
 const severity=c.symbol(RiskLevel);
 const source=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const effectiveDate=c.optional(1,()=>c.text());
 const expiryDate=c.optional(1,()=>c.text());
 const waiverStatus=c.optional(1,()=>c.text());
 const waiverApprover=c.optional(1,()=>c.text());
 const impactAssessment=c.optionalTextField();
 const monitoringFrequency=c.optional(1,()=>c.text());
 const complianceStatus=c.symbol(ValidationStatus);
 const escalationContactId=c.optional(1,()=>c.text());
 c.end();
 const affectedEntityIds=await r.list("architect_constraint_affected_entity",hrow.rowid,1,c=>c.text());
 const regulatoryBasis=await r.list("architect_constraint_regulatory_basis",hrow.rowid,1,c=>c.text());
 const mitigationOptions=await r.list("architect_constraint_mitigation_option",hrow.rowid,1,c=>c.text());
 const resolutionPlan=await r.list("architect_constraint_resolution_plan",hrow.rowid,1,c=>c.text());
 const relatedRequirementIds=await r.list("architect_constraint_related_requirement",hrow.rowid,1,c=>c.text());
 const relatedDecisionIds=await r.list("architect_constraint_related_decision",hrow.rowid,1,c=>c.text());
 const exceptions=await r.list("architect_constraint_exception",hrow.rowid,1,c=>c.text());
 const traceLinks=await r.list("architect_constraint_trace",hrow.rowid,5,c=>readTrace(c));
 return{...h,constraintType,summary,severity,source,ownerId,effectiveDate,expiryDate,waiverStatus,waiverApprover,impactAssessment,monitoringFrequency,complianceStatus,escalationContactId,affectedEntityIds,regulatoryBasis,mitigationOptions,resolutionPlan,relatedRequirementIds,relatedDecisionIds,exceptions,traceLinks};
}
export async function projectComplianceRecord(p:ArtifactSqliteProjection,id:bigint,v:ComplianceRecord):Promise<void>{
 await p.insert("architect_compliance_record",[...text(v.standardRef),...textField(v.obligation),...symbol(v.complianceStatus,ValidationStatus),...optional(v.auditorId,1,text),...optional(v.auditDate,1,text),...optional(v.nextReview,1,text),...optional(v.ownerId,1,text),...symbol(v.severity,RiskLevel),...optional(v.regulatoryBody,1,text),...optional(v.certificationTarget,1,text),...optional(v.waiverStatus,1,text),...optional(v.monitoringMethod,1,text),...optional(v.reportingFrequency,1,text)],id);
 await ordered(p,"architect_compliance_evidence_ref",id,v.evidenceRefs,text);
 await ordered(p,"architect_compliance_affected_entity",id,v.affectedEntityIds,text);
 await ordered(p,"architect_compliance_gap_analysis",id,v.gapAnalysis,text);
 await ordered(p,"architect_compliance_remediation_plan",id,v.remediationPlan,text);
 await ordered(p,"architect_compliance_related_requirement",id,v.relatedRequirementIds,text);
 await ordered(p,"architect_compliance_penalty",id,v.penalties,text);
 await ordered(p,"architect_compliance_corrective_action",id,v.correctiveActions,text);
 await ordered(p,"architect_compliance_artifact_ref",id,v.artifactRefs,text);
}
export async function restoreComplianceRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ComplianceRecord>{
 const row=await r.body("architect_compliance_record",hrow.rowid,15),c=new Cursor(row!);
 const standardRef=c.text();
 const obligation=c.textField();
 const complianceStatus=c.symbol(ValidationStatus);
 const auditorId=c.optional(1,()=>c.text());
 const auditDate=c.optional(1,()=>c.text());
 const nextReview=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const severity=c.symbol(RiskLevel);
 const regulatoryBody=c.optional(1,()=>c.text());
 const certificationTarget=c.optional(1,()=>c.text());
 const waiverStatus=c.optional(1,()=>c.text());
 const monitoringMethod=c.optional(1,()=>c.text());
 const reportingFrequency=c.optional(1,()=>c.text());
 c.end();
 const evidenceRefs=await r.list("architect_compliance_evidence_ref",hrow.rowid,1,c=>c.text());
 const affectedEntityIds=await r.list("architect_compliance_affected_entity",hrow.rowid,1,c=>c.text());
 const gapAnalysis=await r.list("architect_compliance_gap_analysis",hrow.rowid,1,c=>c.text());
 const remediationPlan=await r.list("architect_compliance_remediation_plan",hrow.rowid,1,c=>c.text());
 const relatedRequirementIds=await r.list("architect_compliance_related_requirement",hrow.rowid,1,c=>c.text());
 const penalties=await r.list("architect_compliance_penalty",hrow.rowid,1,c=>c.text());
 const correctiveActions=await r.list("architect_compliance_corrective_action",hrow.rowid,1,c=>c.text());
 const artifactRefs=await r.list("architect_compliance_artifact_ref",hrow.rowid,1,c=>c.text());
 return{...h,standardRef,obligation,complianceStatus,auditorId,auditDate,nextReview,ownerId,severity,regulatoryBody,certificationTarget,waiverStatus,monitoringMethod,reportingFrequency,evidenceRefs,affectedEntityIds,gapAnalysis,remediationPlan,relatedRequirementIds,penalties,correctiveActions,artifactRefs};
}
export async function projectTemplateRecord(p:ArtifactSqliteProjection,id:bigint,v:TemplateRecord):Promise<void>{
 await p.insert("architect_template_record",[...text(v.templateType),...optional(v.sector,1,text),...optional(v.projectType,1,text),...text(v.version),...optional(v.contentRef,1,text),...optional(v.authorId,1,text),...symbol(v.approvalStatus,ValidationStatus),...uint64(v.usageCount),...optional(v.lastApplied,1,text),...optional(v.license,1,text),...optional(v.sourceOrganization,1,text)],id);
 await ordered(p,"architect_template_entity_kind",id,v.entityKinds,text);
 await ordered(p,"architect_template_default_field",id,v.defaultFields,text);
 await ordered(p,"architect_template_checklist",id,v.checklists,text);
 await ordered(p,"architect_template_standard",id,v.standards,text);
 await ordered(p,"architect_template_applicability",id,v.applicability,text);
 await ordered(p,"architect_template_customization_note",id,v.customizationNotes,text);
 await ordered(p,"architect_template_related_knowledge",id,v.relatedKnowledgeIds,text);
 await ordered(p,"architect_template_benchmark",id,v.benchmarkIds,text);
}
export async function restoreTemplateRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<TemplateRecord>{
 const row=await r.body("architect_template_record",hrow.rowid,13),c=new Cursor(row!);
 const templateType=c.text();
 const sector=c.optional(1,()=>c.text());
 const projectType=c.optional(1,()=>c.text());
 const version=c.text();
 const contentRef=c.optional(1,()=>c.text());
 const authorId=c.optional(1,()=>c.text());
 const approvalStatus=c.symbol(ValidationStatus);
 const usageCount=c.uint64();
 const lastApplied=c.optional(1,()=>c.text());
 const license=c.optional(1,()=>c.text());
 const sourceOrganization=c.optional(1,()=>c.text());
 c.end();
 const entityKinds=await r.list("architect_template_entity_kind",hrow.rowid,1,c=>c.text());
 const defaultFields=await r.list("architect_template_default_field",hrow.rowid,1,c=>c.text());
 const checklists=await r.list("architect_template_checklist",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_template_standard",hrow.rowid,1,c=>c.text());
 const applicability=await r.list("architect_template_applicability",hrow.rowid,1,c=>c.text());
 const customizationNotes=await r.list("architect_template_customization_note",hrow.rowid,1,c=>c.text());
 const relatedKnowledgeIds=await r.list("architect_template_related_knowledge",hrow.rowid,1,c=>c.text());
 const benchmarkIds=await r.list("architect_template_benchmark",hrow.rowid,1,c=>c.text());
 return{...h,templateType,sector,projectType,version,contentRef,authorId,approvalStatus,usageCount,lastApplied,license,sourceOrganization,entityKinds,defaultFields,checklists,standards,applicability,customizationNotes,relatedKnowledgeIds,benchmarkIds};
}
export async function projectKnowledgeRecord(p:ArtifactSqliteProjection,id:bigint,v:KnowledgeRecord):Promise<void>{
 await p.insert("architect_knowledge_record",[...text(v.topic),...text(v.category),...textField(v.summary),...textField(v.content),...optional(v.expertiseLevel,1,text),...symbol(v.validationStatus,ValidationStatus),...optional(v.lastReviewed,1,text),...uint64(v.usageCount)],id);
 await ordered(p,"architect_knowledge_source",id,v.sources,text);
 await ordered(p,"architect_knowledge_reference",id,v.references,text);
 await ordered(p,"architect_knowledge_lesson_learned",id,v.lessonsLearned,text);
 await ordered(p,"architect_knowledge_best_practice",id,v.bestPractices,text);
 await ordered(p,"architect_knowledge_applicable_sector",id,v.applicableSectors,text);
 await ordered(p,"architect_knowledge_related_entity_kind",id,v.relatedEntityKinds,text);
 await ordered(p,"architect_knowledge_author",id,v.authorIds,text);
 await ordered(p,"architect_knowledge_keyword",id,v.keywords,text);
 await ordered(p,"architect_knowledge_attachment",id,v.attachments,text);
 await ordered(p,"architect_knowledge_citation",id,v.citations,text);
}
export async function restoreKnowledgeRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<KnowledgeRecord>{
 const row=await r.body("architect_knowledge_record",hrow.rowid,12),c=new Cursor(row!);
 const topic=c.text();
 const category=c.text();
 const summary=c.textField();
 const content=c.textField();
 const expertiseLevel=c.optional(1,()=>c.text());
 const validationStatus=c.symbol(ValidationStatus);
 const lastReviewed=c.optional(1,()=>c.text());
 const usageCount=c.uint64();
 c.end();
 const sources=await r.list("architect_knowledge_source",hrow.rowid,1,c=>c.text());
 const references=await r.list("architect_knowledge_reference",hrow.rowid,1,c=>c.text());
 const lessonsLearned=await r.list("architect_knowledge_lesson_learned",hrow.rowid,1,c=>c.text());
 const bestPractices=await r.list("architect_knowledge_best_practice",hrow.rowid,1,c=>c.text());
 const applicableSectors=await r.list("architect_knowledge_applicable_sector",hrow.rowid,1,c=>c.text());
 const relatedEntityKinds=await r.list("architect_knowledge_related_entity_kind",hrow.rowid,1,c=>c.text());
 const authorIds=await r.list("architect_knowledge_author",hrow.rowid,1,c=>c.text());
 const keywords=await r.list("architect_knowledge_keyword",hrow.rowid,1,c=>c.text());
 const attachments=await r.list("architect_knowledge_attachment",hrow.rowid,1,c=>c.text());
 const citations=await r.list("architect_knowledge_citation",hrow.rowid,1,c=>c.text());
 return{...h,topic,category,summary,content,expertiseLevel,validationStatus,lastReviewed,usageCount,sources,references,lessonsLearned,bestPractices,applicableSectors,relatedEntityKinds,authorIds,keywords,attachments,citations};
}
export async function projectBenchmarkRecord(p:ArtifactSqliteProjection,id:bigint,v:BenchmarkRecord):Promise<void>{
 await p.insert("architect_benchmark_record",[...text(v.benchmarkName),...text(v.sector),...text(v.metric),...float(v.value),...text(v.unit),...optional(v.sampleSize,1,uint32),...optional(v.source,1,text),...optional(v.collectionYear,1,uint32),...optional(v.geography,1,text),...optional(v.buildingType,1,text),...optional(v.confidence,1,text),...optional(v.methodology,1,text),...optional(v.license,1,text),...optional(v.knowledgeId,1,text),...optional(v.lastVerified,1,text)],id);
 await ordered(p,"architect_benchmark_applicable_element_kind",id,v.applicableElementKinds,text);
 await ordered(p,"architect_benchmark_related_requirement",id,v.relatedRequirementIds,text);
 await ordered(p,"architect_benchmark_comparison_note",id,v.comparisonNotes,text);
 await ordered(p,"architect_benchmark_limitation",id,v.limitations,text);
}
export async function restoreBenchmarkRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<BenchmarkRecord>{
 const row=await r.body("architect_benchmark_record",hrow.rowid,18),c=new Cursor(row!);
 const benchmarkName=c.text();
 const sector=c.text();
 const metric=c.text();
 const value=c.float();
 const unit=c.text();
 const sampleSize=c.optional(1,()=>c.uint32());
 const source=c.optional(1,()=>c.text());
 const collectionYear=c.optional(1,()=>c.uint32());
 const geography=c.optional(1,()=>c.text());
 const buildingType=c.optional(1,()=>c.text());
 const confidence=c.optional(1,()=>c.text());
 const methodology=c.optional(1,()=>c.text());
 const license=c.optional(1,()=>c.text());
 const knowledgeId=c.optional(1,()=>c.text());
 const lastVerified=c.optional(1,()=>c.text());
 c.end();
 const applicableElementKinds=await r.list("architect_benchmark_applicable_element_kind",hrow.rowid,1,c=>c.text());
 const relatedRequirementIds=await r.list("architect_benchmark_related_requirement",hrow.rowid,1,c=>c.text());
 const comparisonNotes=await r.list("architect_benchmark_comparison_note",hrow.rowid,1,c=>c.text());
 const limitations=await r.list("architect_benchmark_limitation",hrow.rowid,1,c=>c.text());
 return{...h,benchmarkName,sector,metric,value,unit,sampleSize,source,collectionYear,geography,buildingType,confidence,methodology,license,knowledgeId,lastVerified,applicableElementKinds,relatedRequirementIds,comparisonNotes,limitations};
}
export async function projectIssue(p:ArtifactSqliteProjection,id:bigint,v:Issue):Promise<void>{
 await p.insert("architect_issue",[...text(v.issueType),...textField(v.summary),...textField(v.issueDescription),...symbol(v.severity,IssueSeverity),...symbol(v.issuePriority,Priority),...optional(v.reporterId,1,text),...optional(v.assigneeId,1,text),...optionalTextField(v.rootCause),...optionalTextField(v.resolution),...optionalTextField(v.workaround),...optional(v.dueDate,1,text),...optional(v.resolvedDate,1,text),...optional(v.decisionId,1,text),...optional(v.escalationLevel,1,text)],id);
 await ordered(p,"architect_issue_affected_entity",id,v.affectedEntityIds,text);
 await ordered(p,"architect_issue_related_conflict",id,v.relatedConflictIds,text);
 await ordered(p,"architect_issue_related_risk",id,v.relatedRiskIds,text);
 await ordered(p,"architect_issue_comment",id,v.comments,note);
 await ordered(p,"architect_issue_attachment",id,v.attachments,text);
}
export async function restoreIssue(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Issue>{
 const row=await r.body("architect_issue",hrow.rowid,23),c=new Cursor(row!);
 const issueType=c.text();
 const summary=c.textField();
 const issueDescription=c.textField();
 const severity=c.symbol(IssueSeverity);
 const issuePriority=c.symbol(Priority);
 const reporterId=c.optional(1,()=>c.text());
 const assigneeId=c.optional(1,()=>c.text());
 const rootCause=c.optionalTextField();
 const resolution=c.optionalTextField();
 const workaround=c.optionalTextField();
 const dueDate=c.optional(1,()=>c.text());
 const resolvedDate=c.optional(1,()=>c.text());
 const decisionId=c.optional(1,()=>c.text());
 const escalationLevel=c.optional(1,()=>c.text());
 c.end();
 const affectedEntityIds=await r.list("architect_issue_affected_entity",hrow.rowid,1,c=>c.text());
 const relatedConflictIds=await r.list("architect_issue_related_conflict",hrow.rowid,1,c=>c.text());
 const relatedRiskIds=await r.list("architect_issue_related_risk",hrow.rowid,1,c=>c.text());
 const comments=await r.list("architect_issue_comment",hrow.rowid,2,c=>c.note());
 const attachments=await r.list("architect_issue_attachment",hrow.rowid,1,c=>c.text());
 return{...h,issueType,summary,issueDescription,severity,issuePriority,reporterId,assigneeId,rootCause,resolution,workaround,dueDate,resolvedDate,decisionId,escalationLevel,affectedEntityIds,relatedConflictIds,relatedRiskIds,comments,attachments};
}
export async function projectStatusRecord(p:ArtifactSqliteProjection,id:bigint,v:StatusRecord):Promise<void>{
 await p.insert("architect_status_record",[...text(v.subjectId),...text(v.subjectKind),...symbol(v.recordStatus,LifecycleStatus),...optional(v.previousStatus,1,value=>symbol(value,LifecycleStatus)),...optional(v.changedBy,1,text),...optional(v.changedAt,1,text),...optionalTextField(v.reason),...optional(v.dueDate,1,text),...optional(v.progressPercent,3,float),...optional(v.health,1,text),...optional(v.escalationLevel,1,text),...optional(v.milestoneId,1,text),...optional(v.reportingPeriod,1,text)],id);
 await ordered(p,"architect_status_blocker",id,v.blockers,text);
 await ordered(p,"architect_status_next_action",id,v.nextActions,text);
 await ordered(p,"architect_status_related_issue",id,v.relatedIssueIds,text);
 await ordered(p,"architect_status_related_risk",id,v.relatedRiskIds,text);
 await ordered(p,"architect_status_note",id,v.statusNotes,note);
}
export async function restoreStatusRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<StatusRecord>{
 const row=await r.body("architect_status_record",hrow.rowid,18),c=new Cursor(row!);
 const subjectId=c.text();
 const subjectKind=c.text();
 const recordStatus=c.symbol(LifecycleStatus);
 const previousStatus=c.optional(1,()=>c.symbol(LifecycleStatus));
 const changedBy=c.optional(1,()=>c.text());
 const changedAt=c.optional(1,()=>c.text());
 const reason=c.optionalTextField();
 const dueDate=c.optional(1,()=>c.text());
 const progressPercent=c.optional(3,()=>c.float());
 const health=c.optional(1,()=>c.text());
 const escalationLevel=c.optional(1,()=>c.text());
 const milestoneId=c.optional(1,()=>c.text());
 const reportingPeriod=c.optional(1,()=>c.text());
 c.end();
 const blockers=await r.list("architect_status_blocker",hrow.rowid,1,c=>c.text());
 const nextActions=await r.list("architect_status_next_action",hrow.rowid,1,c=>c.text());
 const relatedIssueIds=await r.list("architect_status_related_issue",hrow.rowid,1,c=>c.text());
 const relatedRiskIds=await r.list("architect_status_related_risk",hrow.rowid,1,c=>c.text());
 const statusNotes=await r.list("architect_status_note",hrow.rowid,2,c=>c.note());
 return{...h,subjectId,subjectKind,recordStatus,previousStatus,changedBy,changedAt,reason,dueDate,progressPercent,health,escalationLevel,milestoneId,reportingPeriod,blockers,nextActions,relatedIssueIds,relatedRiskIds,statusNotes};
}
export async function projectWorkshop(p:ArtifactSqliteProjection,id:bigint,v:Workshop):Promise<void>{
 await p.insert("architect_workshop",[...text(v.workshopType),...optional(v.facilitatorId,1,text),...optional(v.scheduledStart,1,text),...optional(v.scheduledEnd,1,text),...optional(v.location,1,text),...optional(v.recordingRef,1,text),...optional(v.budget,3,float),...symbol(v.workshopStatus,LifecycleStatus)],id);
 await ordered(p,"architect_workshop_objective",id,v.objectives,text);
 await ordered(p,"architect_workshop_agenda",id,v.agenda,text);
 await ordered(p,"architect_workshop_participant",id,v.participants,text);
 await ordered(p,"architect_workshop_material",id,v.materials,text);
 await ordered(p,"architect_workshop_method",id,v.methods,text);
 await ordered(p,"architect_workshop_output",id,v.outputs,text);
 await ordered(p,"architect_workshop_decision",id,v.decisions,text);
 await ordered(p,"architect_workshop_issue",id,v.issues,text);
 await ordered(p,"architect_workshop_follow_up_action",id,v.followUpActions,text);
 await ordered(p,"architect_workshop_feedback",id,v.feedback,note);
 await ordered(p,"architect_workshop_survey",id,v.surveyIds,text);
}
export async function restoreWorkshop(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Workshop>{
 const row=await r.body("architect_workshop",hrow.rowid,11),c=new Cursor(row!);
 const workshopType=c.text();
 const facilitatorId=c.optional(1,()=>c.text());
 const scheduledStart=c.optional(1,()=>c.text());
 const scheduledEnd=c.optional(1,()=>c.text());
 const location=c.optional(1,()=>c.text());
 const recordingRef=c.optional(1,()=>c.text());
 const budget=c.optional(3,()=>c.float());
 const workshopStatus=c.symbol(LifecycleStatus);
 c.end();
 const objectives=await r.list("architect_workshop_objective",hrow.rowid,1,c=>c.text());
 const agenda=await r.list("architect_workshop_agenda",hrow.rowid,1,c=>c.text());
 const participants=await r.list("architect_workshop_participant",hrow.rowid,1,c=>c.text());
 const materials=await r.list("architect_workshop_material",hrow.rowid,1,c=>c.text());
 const methods=await r.list("architect_workshop_method",hrow.rowid,1,c=>c.text());
 const outputs=await r.list("architect_workshop_output",hrow.rowid,1,c=>c.text());
 const decisions=await r.list("architect_workshop_decision",hrow.rowid,1,c=>c.text());
 const issues=await r.list("architect_workshop_issue",hrow.rowid,1,c=>c.text());
 const followUpActions=await r.list("architect_workshop_follow_up_action",hrow.rowid,1,c=>c.text());
 const feedback=await r.list("architect_workshop_feedback",hrow.rowid,2,c=>c.note());
 const surveyIds=await r.list("architect_workshop_survey",hrow.rowid,1,c=>c.text());
 return{...h,workshopType,facilitatorId,scheduledStart,scheduledEnd,location,recordingRef,budget,workshopStatus,objectives,agenda,participants,materials,methods,outputs,decisions,issues,followUpActions,feedback,surveyIds};
}
export async function projectSurvey(p:ArtifactSqliteProjection,id:bigint,v:Survey):Promise<void>{
 await p.insert("architect_survey",[...text(v.surveyType),...text(v.title),...optional(v.launchDate,1,text),...optional(v.closeDate,1,text),...uint32(v.responseCount),...optional(v.responseRate,3,float),...optional(v.confidentiality,1,text),...optional(v.analysisId,1,text),...optional(v.workshopId,1,text),...optional(v.ownerId,1,text),...symbol(v.surveyStatus,LifecycleStatus)],id);
 await ordered(p,"architect_survey_objective",id,v.objectives,text);
 await ordered(p,"architect_survey_question",id,v.questions,text);
 await ordered(p,"architect_survey_target_audience",id,v.targetAudience,text);
 await ordered(p,"architect_survey_distribution_channel",id,v.distributionChannels,text);
 await ordered(p,"architect_survey_finding",id,v.findings,text);
 await ordered(p,"architect_survey_theme",id,v.themes,text);
 await ordered(p,"architect_survey_recommendation",id,v.recommendations,text);
 await ordered(p,"architect_survey_consent_process",id,v.consentProcess,text);
}
export async function restoreSurvey(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Survey>{
 const row=await r.body("architect_survey",hrow.rowid,14),c=new Cursor(row!);
 const surveyType=c.text();
 const title=c.text();
 const launchDate=c.optional(1,()=>c.text());
 const closeDate=c.optional(1,()=>c.text());
 const responseCount=c.uint32();
 const responseRate=c.optional(3,()=>c.float());
 const confidentiality=c.optional(1,()=>c.text());
 const analysisId=c.optional(1,()=>c.text());
 const workshopId=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const surveyStatus=c.symbol(LifecycleStatus);
 c.end();
 const objectives=await r.list("architect_survey_objective",hrow.rowid,1,c=>c.text());
 const questions=await r.list("architect_survey_question",hrow.rowid,1,c=>c.text());
 const targetAudience=await r.list("architect_survey_target_audience",hrow.rowid,1,c=>c.text());
 const distributionChannels=await r.list("architect_survey_distribution_channel",hrow.rowid,1,c=>c.text());
 const findings=await r.list("architect_survey_finding",hrow.rowid,1,c=>c.text());
 const themes=await r.list("architect_survey_theme",hrow.rowid,1,c=>c.text());
 const recommendations=await r.list("architect_survey_recommendation",hrow.rowid,1,c=>c.text());
 const consentProcess=await r.list("architect_survey_consent_process",hrow.rowid,1,c=>c.text());
 return{...h,surveyType,title,launchDate,closeDate,responseCount,responseRate,confidentiality,analysisId,workshopId,ownerId,surveyStatus,objectives,questions,targetAudience,distributionChannels,findings,themes,recommendations,consentProcess};
}
export async function projectSearchFilter(p:ArtifactSqliteProjection,id:bigint,v:SearchFilter):Promise<void>{
 await p.insert("architect_search_filter",[...text(v.filterName),...optionalTextField(v.filterDescription),...optional(v.dateFrom,1,text),...optional(v.dateTo,1,text),...optional(v.sortField,1,text),...optional(v.sortDirection,1,text),...boolean(v.isPublic),...optional(v.createdBy,1,text),...optional(v.lastUsed,1,text),...uint64(v.useCount),...boolean(v.pinned)],id);
 await ordered(p,"architect_search_keyword",id,v.keywords,text);
 await ordered(p,"architect_search_category",id,v.categories,text);
 await ordered(p,"architect_search_owner",id,v.ownerIds,text);
 await ordered(p,"architect_search_status",id,v.statuses,value=>symbol(value,LifecycleStatus));
 await ordered(p,"architect_search_priority",id,v.priorities,value=>symbol(value,Priority));
 await ordered(p,"architect_search_source",id,v.sources,text);
 await ordered(p,"architect_search_entity_kind",id,v.entityKinds,text);
 await ordered(p,"architect_search_tag_filter",id,v.tagFilters,text);
}
export async function restoreSearchFilter(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<SearchFilter>{
 const row=await r.body("architect_search_filter",hrow.rowid,15),c=new Cursor(row!);
 const filterName=c.text();
 const filterDescription=c.optionalTextField();
 const dateFrom=c.optional(1,()=>c.text());
 const dateTo=c.optional(1,()=>c.text());
 const sortField=c.optional(1,()=>c.text());
 const sortDirection=c.optional(1,()=>c.text());
 const isPublic=c.boolean();
 const createdBy=c.optional(1,()=>c.text());
 const lastUsed=c.optional(1,()=>c.text());
 const useCount=c.uint64();
 const pinned=c.boolean();
 c.end();
 const keywords=await r.list("architect_search_keyword",hrow.rowid,1,c=>c.text());
 const categories=await r.list("architect_search_category",hrow.rowid,1,c=>c.text());
 const ownerIds=await r.list("architect_search_owner",hrow.rowid,1,c=>c.text());
 const statuses=await r.list("architect_search_status",hrow.rowid,1,c=>c.symbol(LifecycleStatus));
 const priorities=await r.list("architect_search_priority",hrow.rowid,1,c=>c.symbol(Priority));
 const sources=await r.list("architect_search_source",hrow.rowid,1,c=>c.text());
 const entityKinds=await r.list("architect_search_entity_kind",hrow.rowid,1,c=>c.text());
 const tagFilters=await r.list("architect_search_tag_filter",hrow.rowid,1,c=>c.text());
 return{...h,filterName,filterDescription,dateFrom,dateTo,sortField,sortDirection,isPublic,createdBy,lastUsed,useCount,pinned,keywords,categories,ownerIds,statuses,priorities,sources,entityKinds,tagFilters};
}
export async function projectCollaborationRecord(p:ArtifactSqliteProjection,id:bigint,v:CollaborationRecord):Promise<void>{
 await p.insert("architect_collaboration_record",[...text(v.sessionType),...text(v.title),...optional(v.facilitatorId,1,text),...optional(v.startTime,1,text),...optional(v.endTime,1,text),...optional(v.location,1,text),...optional(v.recordingRef,1,text),...optional(v.followUpDate,1,text),...optional(v.workshopId,1,text),...optional(v.surveyId,1,text)],id);
 await ordered(p,"architect_collaboration_participant",id,v.participants,text);
 await ordered(p,"architect_collaboration_agenda",id,v.agenda,text);
 await ordered(p,"architect_collaboration_outcome",id,v.outcomes,text);
 await ordered(p,"architect_collaboration_action_item",id,v.actionItems,text);
 await ordered(p,"architect_collaboration_decision",id,v.decisionIds,text);
 await ordered(p,"architect_collaboration_issue",id,v.issueIds,text);
 await ordered(p,"architect_collaboration_document",id,v.documentIds,text);
 await ordered(p,"architect_collaboration_feedback",id,v.feedback,note);
}
export async function restoreCollaborationRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<CollaborationRecord>{
 const row=await r.body("architect_collaboration_record",hrow.rowid,11),c=new Cursor(row!);
 const sessionType=c.text();
 const title=c.text();
 const facilitatorId=c.optional(1,()=>c.text());
 const startTime=c.optional(1,()=>c.text());
 const endTime=c.optional(1,()=>c.text());
 const location=c.optional(1,()=>c.text());
 const recordingRef=c.optional(1,()=>c.text());
 const followUpDate=c.optional(1,()=>c.text());
 const workshopId=c.optional(1,()=>c.text());
 const surveyId=c.optional(1,()=>c.text());
 c.end();
 const participants=await r.list("architect_collaboration_participant",hrow.rowid,1,c=>c.text());
 const agenda=await r.list("architect_collaboration_agenda",hrow.rowid,1,c=>c.text());
 const outcomes=await r.list("architect_collaboration_outcome",hrow.rowid,1,c=>c.text());
 const actionItems=await r.list("architect_collaboration_action_item",hrow.rowid,1,c=>c.text());
 const decisionIds=await r.list("architect_collaboration_decision",hrow.rowid,1,c=>c.text());
 const issueIds=await r.list("architect_collaboration_issue",hrow.rowid,1,c=>c.text());
 const documentIds=await r.list("architect_collaboration_document",hrow.rowid,1,c=>c.text());
 const feedback=await r.list("architect_collaboration_feedback",hrow.rowid,2,c=>c.note());
 return{...h,sessionType,title,facilitatorId,startTime,endTime,location,recordingRef,followUpDate,workshopId,surveyId,participants,agenda,outcomes,actionItems,decisionIds,issueIds,documentIds,feedback};
}
export async function projectAnalysisRecord(p:ArtifactSqliteProjection,id:bigint,v:AnalysisRecord):Promise<void>{
 await p.insert("architect_analysis_record",[...symbol(v.kind,AnalysisKind),...text(v.title),...textField(v.outputSummary),...optional(v.runBy,1,text),...optional(v.runAt,1,text),...optional(v.durationMs,2,uint64),...optional(v.toolVersion,1,text),...optional(v.scenarioId,1,text),...optional(v.reportId,1,text),...optional(v.confidence,1,text),...optional(v.rawResultRef,1,text)],id);
 await ordered(p,"architect_analysis_parameter",id,v.parameters,text);
 await ordered(p,"architect_analysis_input_entity",id,v.inputEntityIds,text);
 await ordered(p,"architect_analysis_finding",id,v.findings,text);
 await ordered(p,"architect_analysis_metric",id,v.metrics,text);
 await ordered(p,"architect_analysis_chart",id,v.charts,text);
 await ordered(p,"architect_analysis_limitation",id,v.limitations,text);
 await ordered(p,"architect_analysis_recommendation",id,v.recommendations,text);
}
export async function restoreAnalysisRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<AnalysisRecord>{
 const row=await r.body("architect_analysis_record",hrow.rowid,14),c=new Cursor(row!);
 const kind=c.symbol(AnalysisKind);
 const title=c.text();
 const outputSummary=c.textField();
 const runBy=c.optional(1,()=>c.text());
 const runAt=c.optional(1,()=>c.text());
 const durationMs=c.optional(2,()=>c.uint64());
 const toolVersion=c.optional(1,()=>c.text());
 const scenarioId=c.optional(1,()=>c.text());
 const reportId=c.optional(1,()=>c.text());
 const confidence=c.optional(1,()=>c.text());
 const rawResultRef=c.optional(1,()=>c.text());
 c.end();
 const parameters=await r.list("architect_analysis_parameter",hrow.rowid,1,c=>c.text());
 const inputEntityIds=await r.list("architect_analysis_input_entity",hrow.rowid,1,c=>c.text());
 const findings=await r.list("architect_analysis_finding",hrow.rowid,1,c=>c.text());
 const metrics=await r.list("architect_analysis_metric",hrow.rowid,1,c=>c.text());
 const charts=await r.list("architect_analysis_chart",hrow.rowid,1,c=>c.text());
 const limitations=await r.list("architect_analysis_limitation",hrow.rowid,1,c=>c.text());
 const recommendations=await r.list("architect_analysis_recommendation",hrow.rowid,1,c=>c.text());
 return{...h,kind,title,outputSummary,runBy,runAt,durationMs,toolVersion,scenarioId,reportId,confidence,rawResultRef,parameters,inputEntityIds,findings,metrics,charts,limitations,recommendations};
}
export async function projectReportRecord(p:ArtifactSqliteProjection,id:bigint,v:ReportRecord):Promise<void>{
 await p.insert("architect_report_record",[...symbol(v.kind,ReportKind),...text(v.title),...optional(v.generatedAt,1,text),...optional(v.generatedBy,1,text),...optional(v.format,1,text),...optional(v.fileRef,1,text),...symbol(v.approvalStatus,ValidationStatus),...optional(v.approverId,1,text),...text(v.version),...optional(v.templateId,1,text),...optional(v.confidentiality,1,text),...optional(v.expiryDate,1,text)],id);
 await ordered(p,"architect_report_audience",id,v.audience,text);
 await ordered(p,"architect_report_section",id,v.sections,text);
 await ordered(p,"architect_report_analysis",id,v.analysisIds,text);
 await ordered(p,"architect_report_distribution",id,v.distributionList,text);
 await ordered(p,"architect_report_parameter",id,v.parameters,text);
 await ordered(p,"architect_report_related_decision",id,v.relatedDecisionIds,text);
}
export async function restoreReportRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ReportRecord>{
 const row=await r.body("architect_report_record",hrow.rowid,13),c=new Cursor(row!);
 const kind=c.symbol(ReportKind);
 const title=c.text();
 const generatedAt=c.optional(1,()=>c.text());
 const generatedBy=c.optional(1,()=>c.text());
 const format=c.optional(1,()=>c.text());
 const fileRef=c.optional(1,()=>c.text());
 const approvalStatus=c.symbol(ValidationStatus);
 const approverId=c.optional(1,()=>c.text());
 const version=c.text();
 const templateId=c.optional(1,()=>c.text());
 const confidentiality=c.optional(1,()=>c.text());
 const expiryDate=c.optional(1,()=>c.text());
 c.end();
 const audience=await r.list("architect_report_audience",hrow.rowid,1,c=>c.text());
 const sections=await r.list("architect_report_section",hrow.rowid,1,c=>c.text());
 const analysisIds=await r.list("architect_report_analysis",hrow.rowid,1,c=>c.text());
 const distributionList=await r.list("architect_report_distribution",hrow.rowid,1,c=>c.text());
 const parameters=await r.list("architect_report_parameter",hrow.rowid,1,c=>c.text());
 const relatedDecisionIds=await r.list("architect_report_related_decision",hrow.rowid,1,c=>c.text());
 return{...h,kind,title,generatedAt,generatedBy,format,fileRef,approvalStatus,approverId,version,templateId,confidentiality,expiryDate,audience,sections,analysisIds,distributionList,parameters,relatedDecisionIds};
}
export async function projectChangeRecord(p:ArtifactSqliteProjection,id:bigint,v:ChangeRecord):Promise<void>{
 await p.insert("architect_change_record",[...text(v.changeType),...textField(v.summary),...textField(v.reason),...optional(v.requestedBy,1,text),...optional(v.approvedBy,1,text),...optional(v.changeDate,1,text),...optional(v.effectiveDate,1,text),...optional(v.beforeSnapshot,1,text),...optional(v.afterSnapshot,1,text),...optional(v.costImpact,3,float),...optional(v.scheduleImpact,1,text),...symbol(v.approvalStatus,ValidationStatus),...optional(v.versionFrom,1,text),...optional(v.versionTo,1,text)],id);
 await ordered(p,"architect_change_impacted_entity",id,v.impactedEntityIds,text);
 await ordered(p,"architect_change_risk_impact",id,v.riskImpact,text);
 await ordered(p,"architect_change_rollback_plan",id,v.rollbackPlan,text);
 await ordered(p,"architect_change_communication_plan",id,v.communicationPlan,text);
 await ordered(p,"architect_change_audit_event",id,v.auditEventIds,text);
}
export async function restoreChangeRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ChangeRecord>{
 const row=await r.body("architect_change_record",hrow.rowid,19),c=new Cursor(row!);
 const changeType=c.text();
 const summary=c.textField();
 const reason=c.textField();
 const requestedBy=c.optional(1,()=>c.text());
 const approvedBy=c.optional(1,()=>c.text());
 const changeDate=c.optional(1,()=>c.text());
 const effectiveDate=c.optional(1,()=>c.text());
 const beforeSnapshot=c.optional(1,()=>c.text());
 const afterSnapshot=c.optional(1,()=>c.text());
 const costImpact=c.optional(3,()=>c.float());
 const scheduleImpact=c.optional(1,()=>c.text());
 const approvalStatus=c.symbol(ValidationStatus);
 const versionFrom=c.optional(1,()=>c.text());
 const versionTo=c.optional(1,()=>c.text());
 c.end();
 const impactedEntityIds=await r.list("architect_change_impacted_entity",hrow.rowid,1,c=>c.text());
 const riskImpact=await r.list("architect_change_risk_impact",hrow.rowid,1,c=>c.text());
 const rollbackPlan=await r.list("architect_change_rollback_plan",hrow.rowid,1,c=>c.text());
 const communicationPlan=await r.list("architect_change_communication_plan",hrow.rowid,1,c=>c.text());
 const auditEventIds=await r.list("architect_change_audit_event",hrow.rowid,1,c=>c.text());
 return{...h,changeType,summary,reason,requestedBy,approvedBy,changeDate,effectiveDate,beforeSnapshot,afterSnapshot,costImpact,scheduleImpact,approvalStatus,versionFrom,versionTo,impactedEntityIds,riskImpact,rollbackPlan,communicationPlan,auditEventIds};
}
export async function projectPerformanceCriterion(p:ArtifactSqliteProjection,id:bigint,v:PerformanceCriterion):Promise<void>{
 await p.insert("architect_performance_criterion",[...text(v.criterion),...text(v.metric),...optional(v.target,3,float),...optional(v.unit,1,text),...optional(v.minimum,3,float),...optional(v.maximum,3,float),...optional(v.measurementMethod,1,text),...optional(v.frequency,1,text),...optional(v.baseline,3,float),...optional(v.benchmarkRef,1,text),...optional(v.weight,3,float),...optional(v.dataSource,1,text),...optional(v.reportingCadence,1,text),...optional(v.ownerId,1,text),...optional(v.verificationPlan,1,text),...optional(v.penaltyThreshold,3,float),...optional(v.incentiveThreshold,3,float)],id);
 await ordered(p,"architect_performance_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_performance_element",id,v.elementIds,text);
}
export async function restorePerformanceCriterion(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<PerformanceCriterion>{
 const row=await r.body("architect_performance_criterion",hrow.rowid,32),c=new Cursor(row!);
 const criterion=c.text();
 const metric=c.text();
 const target=c.optional(3,()=>c.float());
 const unit=c.optional(1,()=>c.text());
 const minimum=c.optional(3,()=>c.float());
 const maximum=c.optional(3,()=>c.float());
 const measurementMethod=c.optional(1,()=>c.text());
 const frequency=c.optional(1,()=>c.text());
 const baseline=c.optional(3,()=>c.float());
 const benchmarkRef=c.optional(1,()=>c.text());
 const weight=c.optional(3,()=>c.float());
 const dataSource=c.optional(1,()=>c.text());
 const reportingCadence=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const verificationPlan=c.optional(1,()=>c.text());
 const penaltyThreshold=c.optional(3,()=>c.float());
 const incentiveThreshold=c.optional(3,()=>c.float());
 c.end();
 const requirementIds=await r.list("architect_performance_requirement",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_performance_element",hrow.rowid,1,c=>c.text());
 return{...h,criterion,metric,target,unit,minimum,maximum,measurementMethod,frequency,baseline,benchmarkRef,weight,dataSource,reportingCadence,ownerId,verificationPlan,penaltyThreshold,incentiveThreshold,requirementIds,elementIds};
}
export async function projectQualityRecord(p:ArtifactSqliteProjection,id:bigint,v:QualityRecord):Promise<void>{
 await p.insert("architect_quality_record",[...text(v.qualityTopic),...optional(v.standard,1,text),...optional(v.targetLevel,1,text),...optional(v.sampleRate,1,text),...optional(v.auditSchedule,1,text),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_quality_inspection_point",id,v.inspectionPoints,text);
 await ordered(p,"architect_quality_acceptance_criterion",id,v.acceptanceCriteria,text);
 await ordered(p,"architect_quality_testing_requirement",id,v.testingRequirements,text);
 await ordered(p,"architect_quality_defect_category",id,v.defectCategories,text);
 await ordered(p,"architect_quality_corrective_action_process",id,v.correctiveActionProcess,text);
 await ordered(p,"architect_quality_element",id,v.elementIds,text);
 await ordered(p,"architect_quality_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_quality_supplier_requirement",id,v.supplierRequirements,text);
 await ordered(p,"architect_quality_documentation_requirement",id,v.documentationRequirements,text);
 await ordered(p,"architect_quality_training_requirement",id,v.trainingRequirements,text);
 await ordered(p,"architect_quality_kpi",id,v.kpis,text);
 await ordered(p,"architect_quality_certification_target",id,v.certificationTargets,text);
 await ordered(p,"architect_quality_continuous_improvement",id,v.continuousImprovement,text);
}
export async function restoreQualityRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<QualityRecord>{
 const row=await r.body("architect_quality_record",hrow.rowid,7),c=new Cursor(row!);
 const qualityTopic=c.text();
 const standard=c.optional(1,()=>c.text());
 const targetLevel=c.optional(1,()=>c.text());
 const sampleRate=c.optional(1,()=>c.text());
 const auditSchedule=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const inspectionPoints=await r.list("architect_quality_inspection_point",hrow.rowid,1,c=>c.text());
 const acceptanceCriteria=await r.list("architect_quality_acceptance_criterion",hrow.rowid,1,c=>c.text());
 const testingRequirements=await r.list("architect_quality_testing_requirement",hrow.rowid,1,c=>c.text());
 const defectCategories=await r.list("architect_quality_defect_category",hrow.rowid,1,c=>c.text());
 const correctiveActionProcess=await r.list("architect_quality_corrective_action_process",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_quality_element",hrow.rowid,1,c=>c.text());
 const requirementIds=await r.list("architect_quality_requirement",hrow.rowid,1,c=>c.text());
 const supplierRequirements=await r.list("architect_quality_supplier_requirement",hrow.rowid,1,c=>c.text());
 const documentationRequirements=await r.list("architect_quality_documentation_requirement",hrow.rowid,1,c=>c.text());
 const trainingRequirements=await r.list("architect_quality_training_requirement",hrow.rowid,1,c=>c.text());
 const kpis=await r.list("architect_quality_kpi",hrow.rowid,1,c=>c.text());
 const certificationTargets=await r.list("architect_quality_certification_target",hrow.rowid,1,c=>c.text());
 const continuousImprovement=await r.list("architect_quality_continuous_improvement",hrow.rowid,1,c=>c.text());
 return{...h,qualityTopic,standard,targetLevel,sampleRate,auditSchedule,ownerId,inspectionPoints,acceptanceCriteria,testingRequirements,defectCategories,correctiveActionProcess,elementIds,requirementIds,supplierRequirements,documentationRequirements,trainingRequirements,kpis,certificationTargets,continuousImprovement};
}
export async function projectArtifactRecord(p:ArtifactSqliteProjection,id:bigint,v:ArtifactRecord):Promise<void>{
 await p.insert("architect_artifact_record",[...text(v.documentType),...text(v.title),...text(v.version),...optional(v.fileRef,1,text),...optional(v.format,1,text),...optional(v.issueDate,1,text),...optional(v.revisionDate,1,text),...optional(v.classification,1,text),...optional(v.retentionPeriod,1,text),...optional(v.supersedes,1,text),...symbol(v.documentStatus,LifecycleStatus),...optional(v.checksum,1,text),...optional(v.sourceSystem,1,text)],id);
 await ordered(p,"architect_artifact_author",id,v.authorIds,text);
 await ordered(p,"architect_artifact_reviewer",id,v.reviewerIds,text);
 await ordered(p,"architect_artifact_approver",id,v.approverIds,text);
 await ordered(p,"architect_artifact_distribution",id,v.distributionList,text);
 await ordered(p,"architect_artifact_related_entity",id,v.relatedEntityIds,text);
 await ordered(p,"architect_artifact_access_control",id,v.accessControls,text);
}
export async function restoreArtifactRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ArtifactRecord>{
 const row=await r.body("architect_artifact_record",hrow.rowid,14),c=new Cursor(row!);
 const documentType=c.text();
 const title=c.text();
 const version=c.text();
 const fileRef=c.optional(1,()=>c.text());
 const format=c.optional(1,()=>c.text());
 const issueDate=c.optional(1,()=>c.text());
 const revisionDate=c.optional(1,()=>c.text());
 const classification=c.optional(1,()=>c.text());
 const retentionPeriod=c.optional(1,()=>c.text());
 const supersedes=c.optional(1,()=>c.text());
 const documentStatus=c.symbol(LifecycleStatus);
 const checksum=c.optional(1,()=>c.text());
 const sourceSystem=c.optional(1,()=>c.text());
 c.end();
 const authorIds=await r.list("architect_artifact_author",hrow.rowid,1,c=>c.text());
 const reviewerIds=await r.list("architect_artifact_reviewer",hrow.rowid,1,c=>c.text());
 const approverIds=await r.list("architect_artifact_approver",hrow.rowid,1,c=>c.text());
 const distributionList=await r.list("architect_artifact_distribution",hrow.rowid,1,c=>c.text());
 const relatedEntityIds=await r.list("architect_artifact_related_entity",hrow.rowid,1,c=>c.text());
 const accessControls=await r.list("architect_artifact_access_control",hrow.rowid,1,c=>c.text());
 return{...h,documentType,title,version,fileRef,format,issueDate,revisionDate,classification,retentionPeriod,supersedes,documentStatus,checksum,sourceSystem,authorIds,reviewerIds,approverIds,distributionList,relatedEntityIds,accessControls};
}
export async function projectValidationRecord(p:ArtifactSqliteProjection,id:bigint,v:ValidationRecord):Promise<void>{
 await p.insert("architect_validation_record",[...text(v.subjectId),...text(v.subjectKind),...text(v.validationType),...optional(v.method,1,text),...symbol(v.result,ValidationStatus),...optional(v.validationDate,1,text),...optional(v.nextReviewDate,1,text),...optional(v.reportId,1,text),...optional(v.confidenceLevel,1,text)],id);
 await ordered(p,"architect_validation_criterion",id,v.criteria,text);
 await ordered(p,"architect_validation_evidence",id,v.evidence,text);
 await ordered(p,"architect_validation_validator",id,v.validatorIds,text);
 await ordered(p,"architect_validation_finding",id,v.findings,text);
 await ordered(p,"architect_validation_non_conformity",id,v.nonConformities,text);
 await ordered(p,"architect_validation_corrective_action",id,v.correctiveActions,text);
 await ordered(p,"architect_validation_waiver",id,v.waivers,text);
 await ordered(p,"architect_validation_standard",id,v.standards,text);
 await ordered(p,"architect_validation_trace",id,v.traceLinks,trace);
 await ordered(p,"architect_validation_note",id,v.validationNotes,note);
}
export async function restoreValidationRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ValidationRecord>{
 const row=await r.body("architect_validation_record",hrow.rowid,10),c=new Cursor(row!);
 const subjectId=c.text();
 const subjectKind=c.text();
 const validationType=c.text();
 const method=c.optional(1,()=>c.text());
 const result=c.symbol(ValidationStatus);
 const validationDate=c.optional(1,()=>c.text());
 const nextReviewDate=c.optional(1,()=>c.text());
 const reportId=c.optional(1,()=>c.text());
 const confidenceLevel=c.optional(1,()=>c.text());
 c.end();
 const criteria=await r.list("architect_validation_criterion",hrow.rowid,1,c=>c.text());
 const evidence=await r.list("architect_validation_evidence",hrow.rowid,1,c=>c.text());
 const validatorIds=await r.list("architect_validation_validator",hrow.rowid,1,c=>c.text());
 const findings=await r.list("architect_validation_finding",hrow.rowid,1,c=>c.text());
 const nonConformities=await r.list("architect_validation_non_conformity",hrow.rowid,1,c=>c.text());
 const correctiveActions=await r.list("architect_validation_corrective_action",hrow.rowid,1,c=>c.text());
 const waivers=await r.list("architect_validation_waiver",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_validation_standard",hrow.rowid,1,c=>c.text());
 const traceLinks=await r.list("architect_validation_trace",hrow.rowid,5,c=>readTrace(c));
 const validationNotes=await r.list("architect_validation_note",hrow.rowid,2,c=>c.note());
 return{...h,subjectId,subjectKind,validationType,method,result,validationDate,nextReviewDate,reportId,confidenceLevel,criteria,evidence,validatorIds,findings,nonConformities,correctiveActions,waivers,standards,traceLinks,validationNotes};
}
export async function projectScenario(p:ArtifactSqliteProjection,id:bigint,v:Scenario):Promise<void>{
 await p.insert("architect_scenario",[...text(v.code),...textField(v.hypothesis),...optional(v.growthPlanId,1,text),...optional(v.probability,3,float),...optionalTextField(v.impactSummary),...optional(v.costDelta,3,float),...optional(v.areaDelta,3,float),...optional(v.headcountDelta,3,float),...optional(v.scheduleDelta,1,text),...boolean(v.baseline),...boolean(v.preferred),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_scenario_assumption",id,v.assumptions,text);
 await ordered(p,"architect_scenario_variable",id,v.variables,text);
 await ordered(p,"architect_scenario_element",id,v.elementIds,text);
 await ordered(p,"architect_scenario_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_scenario_risk",id,v.riskIds,text);
 await ordered(p,"architect_scenario_option",id,v.optionIds,text);
 await ordered(p,"architect_scenario_analysis",id,v.analysisIds,text);
}
export async function restoreScenario(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Scenario>{
 const row=await r.body("architect_scenario",hrow.rowid,24),c=new Cursor(row!);
 const code=c.text();
 const hypothesis=c.textField();
 const growthPlanId=c.optional(1,()=>c.text());
 const probability=c.optional(3,()=>c.float());
 const impactSummary=c.optionalTextField();
 const costDelta=c.optional(3,()=>c.float());
 const areaDelta=c.optional(3,()=>c.float());
 const headcountDelta=c.optional(3,()=>c.float());
 const scheduleDelta=c.optional(1,()=>c.text());
 const baseline=c.boolean();
 const preferred=c.boolean();
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const assumptions=await r.list("architect_scenario_assumption",hrow.rowid,1,c=>c.text());
 const variables=await r.list("architect_scenario_variable",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_scenario_element",hrow.rowid,1,c=>c.text());
 const requirementIds=await r.list("architect_scenario_requirement",hrow.rowid,1,c=>c.text());
 const riskIds=await r.list("architect_scenario_risk",hrow.rowid,1,c=>c.text());
 const optionIds=await r.list("architect_scenario_option",hrow.rowid,1,c=>c.text());
 const analysisIds=await r.list("architect_scenario_analysis",hrow.rowid,1,c=>c.text());
 return{...h,code,hypothesis,growthPlanId,probability,impactSummary,costDelta,areaDelta,headcountDelta,scheduleDelta,baseline,preferred,ownerId,assumptions,variables,elementIds,requirementIds,riskIds,optionIds,analysisIds};
}
export async function projectOptionEvaluation(p:ArtifactSqliteProjection,id:bigint,v:OptionEvaluation):Promise<void>{
 await p.insert("architect_option_evaluation",[...text(v.optionName),...textField(v.optionDescription),...optional(v.scenarioId,1,text),...optional(v.weightedScore,3,float),...optional(v.costEstimate,3,float),...optional(v.scheduleEstimate,1,text),...optional(v.recommendation,1,text),...optional(v.decisionId,1,text),...symbol(v.evaluationStatus,ValidationStatus),...optional(v.evaluationDate,1,text)],id);
 await ordered(p,"architect_option_criterion",id,v.criteriaIds,text);
 await ordered(p,"architect_option_score",id,v.scores,float);
 await ordered(p,"architect_option_risk_summary",id,v.riskSummary,text);
 await ordered(p,"architect_option_benefit",id,v.benefits,text);
 await ordered(p,"architect_option_drawback",id,v.drawbacks,text);
 await ordered(p,"architect_option_assumption",id,v.assumptions,text);
 await ordered(p,"architect_option_dependency",id,v.dependencies,text);
 await ordered(p,"architect_option_stakeholder_feedback",id,v.stakeholderFeedback,note);
 await ordered(p,"architect_option_evaluator",id,v.evaluatorIds,text);
}
export async function restoreOptionEvaluation(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<OptionEvaluation>{
 const row=await r.body("architect_option_evaluation",hrow.rowid,16),c=new Cursor(row!);
 const optionName=c.text();
 const optionDescription=c.textField();
 const scenarioId=c.optional(1,()=>c.text());
 const weightedScore=c.optional(3,()=>c.float());
 const costEstimate=c.optional(3,()=>c.float());
 const scheduleEstimate=c.optional(1,()=>c.text());
 const recommendation=c.optional(1,()=>c.text());
 const decisionId=c.optional(1,()=>c.text());
 const evaluationStatus=c.symbol(ValidationStatus);
 const evaluationDate=c.optional(1,()=>c.text());
 c.end();
 const criteriaIds=await r.list("architect_option_criterion",hrow.rowid,1,c=>c.text());
 const scores=await r.list("architect_option_score",hrow.rowid,3,c=>c.float());
 const riskSummary=await r.list("architect_option_risk_summary",hrow.rowid,1,c=>c.text());
 const benefits=await r.list("architect_option_benefit",hrow.rowid,1,c=>c.text());
 const drawbacks=await r.list("architect_option_drawback",hrow.rowid,1,c=>c.text());
 const assumptions=await r.list("architect_option_assumption",hrow.rowid,1,c=>c.text());
 const dependencies=await r.list("architect_option_dependency",hrow.rowid,1,c=>c.text());
 const stakeholderFeedback=await r.list("architect_option_stakeholder_feedback",hrow.rowid,2,c=>c.note());
 const evaluatorIds=await r.list("architect_option_evaluator",hrow.rowid,1,c=>c.text());
 return{...h,optionName,optionDescription,scenarioId,weightedScore,costEstimate,scheduleEstimate,recommendation,decisionId,evaluationStatus,evaluationDate,criteriaIds,scores,riskSummary,benefits,drawbacks,assumptions,dependencies,stakeholderFeedback,evaluatorIds};
}
export async function projectDecision(p:ArtifactSqliteProjection,id:bigint,v:Decision):Promise<void>{
 await p.insert("architect_decision",[...textField(v.decisionStatement),...textField(v.context),...optional(v.selectedOptionId,1,text),...textField(v.rationale),...optional(v.decisionDate,1,text),...optional(v.effectiveDate,1,text),...optional(v.costImpact,3,float),...optional(v.scheduleImpact,1,text),...symbol(v.approvalStatus,ValidationStatus),...optional(v.meetingRef,1,text)],id);
 await ordered(p,"architect_decision_option",id,v.optionsConsidered,text);
 await ordered(p,"architect_decision_maker",id,v.decisionMakerIds,text);
 await ordered(p,"architect_decision_consulted",id,v.consultedIds,text);
 await ordered(p,"architect_decision_informed",id,v.informedIds,text);
 await ordered(p,"architect_decision_reversal_condition",id,v.reversalConditions,text);
 await ordered(p,"architect_decision_impacted_requirement",id,v.impactedRequirementIds,text);
 await ordered(p,"architect_decision_impacted_element",id,v.impactedElementIds,text);
 await ordered(p,"architect_decision_risk_impact",id,v.riskImpact,text);
 await ordered(p,"architect_decision_artifact_ref",id,v.artifactRefs,text);
}
export async function restoreDecision(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Decision>{
 const row=await r.body("architect_decision",hrow.rowid,16),c=new Cursor(row!);
 const decisionStatement=c.textField();
 const context=c.textField();
 const selectedOptionId=c.optional(1,()=>c.text());
 const rationale=c.textField();
 const decisionDate=c.optional(1,()=>c.text());
 const effectiveDate=c.optional(1,()=>c.text());
 const costImpact=c.optional(3,()=>c.float());
 const scheduleImpact=c.optional(1,()=>c.text());
 const approvalStatus=c.symbol(ValidationStatus);
 const meetingRef=c.optional(1,()=>c.text());
 c.end();
 const optionsConsidered=await r.list("architect_decision_option",hrow.rowid,1,c=>c.text());
 const decisionMakerIds=await r.list("architect_decision_maker",hrow.rowid,1,c=>c.text());
 const consultedIds=await r.list("architect_decision_consulted",hrow.rowid,1,c=>c.text());
 const informedIds=await r.list("architect_decision_informed",hrow.rowid,1,c=>c.text());
 const reversalConditions=await r.list("architect_decision_reversal_condition",hrow.rowid,1,c=>c.text());
 const impactedRequirementIds=await r.list("architect_decision_impacted_requirement",hrow.rowid,1,c=>c.text());
 const impactedElementIds=await r.list("architect_decision_impacted_element",hrow.rowid,1,c=>c.text());
 const riskImpact=await r.list("architect_decision_risk_impact",hrow.rowid,1,c=>c.text());
 const artifactRefs=await r.list("architect_decision_artifact_ref",hrow.rowid,1,c=>c.text());
 return{...h,decisionStatement,context,selectedOptionId,rationale,decisionDate,effectiveDate,costImpact,scheduleImpact,approvalStatus,meetingRef,optionsConsidered,decisionMakerIds,consultedIds,informedIds,reversalConditions,impactedRequirementIds,impactedElementIds,riskImpact,artifactRefs};
}
export async function projectPriorityRecord(p:ArtifactSqliteProjection,id:bigint,v:PriorityRecord):Promise<void>{
 await p.insert("architect_priority_record",[...text(v.subjectId),...text(v.subjectKind),...symbol(v.rankedPriority,Priority),...optional(v.rank,1,uint32),...optional(v.weight,3,float),...optionalTextField(v.rationale),...optional(v.decisionId,1,text),...optional(v.effectiveFrom,1,text),...optional(v.effectiveUntil,1,text),...optional(v.reviewCycle,1,text),...optional(v.scoringMethod,1,text),...optional(v.score,3,float),...optional(v.approvedBy,1,text),...optional(v.approvalDate,1,text)],id);
 await ordered(p,"architect_priority_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_priority_dependency",id,v.dependencies,text);
 await ordered(p,"architect_priority_conflict",id,v.conflicts,text);
 await ordered(p,"architect_priority_criterion",id,v.criteria,text);
 await ordered(p,"architect_priority_ranking_note",id,v.rankingNotes,note);
}
export async function restorePriorityRecord(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<PriorityRecord>{
 const row=await r.body("architect_priority_record",hrow.rowid,21),c=new Cursor(row!);
 const subjectId=c.text();
 const subjectKind=c.text();
 const rankedPriority=c.symbol(Priority);
 const rank=c.optional(1,()=>c.uint32());
 const weight=c.optional(3,()=>c.float());
 const rationale=c.optionalTextField();
 const decisionId=c.optional(1,()=>c.text());
 const effectiveFrom=c.optional(1,()=>c.text());
 const effectiveUntil=c.optional(1,()=>c.text());
 const reviewCycle=c.optional(1,()=>c.text());
 const scoringMethod=c.optional(1,()=>c.text());
 const score=c.optional(3,()=>c.float());
 const approvedBy=c.optional(1,()=>c.text());
 const approvalDate=c.optional(1,()=>c.text());
 c.end();
 const stakeholderIds=await r.list("architect_priority_stakeholder",hrow.rowid,1,c=>c.text());
 const dependencies=await r.list("architect_priority_dependency",hrow.rowid,1,c=>c.text());
 const conflicts=await r.list("architect_priority_conflict",hrow.rowid,1,c=>c.text());
 const criteria=await r.list("architect_priority_criterion",hrow.rowid,1,c=>c.text());
 const rankingNotes=await r.list("architect_priority_ranking_note",hrow.rowid,2,c=>c.note());
 return{...h,subjectId,subjectKind,rankedPriority,rank,weight,rationale,decisionId,effectiveFrom,effectiveUntil,reviewCycle,scoringMethod,score,approvedBy,approvalDate,stakeholderIds,dependencies,conflicts,criteria,rankingNotes};
}
export async function projectRisk(p:ArtifactSqliteProjection,id:bigint,v:Risk):Promise<void>{
 await p.insert("architect_risk",[...textField(v.riskStatement),...text(v.category),...symbol(v.probability,RiskLevel),...symbol(v.impact,RiskLevel),...optional(v.riskScore,3,float),...optional(v.ownerId,1,text),...optional(v.reviewDate,1,text),...optional(v.residualProbability,1,value=>symbol(value,RiskLevel)),...optional(v.residualImpact,1,value=>symbol(value,RiskLevel)),...optional(v.monitoringPlan,1,text)],id);
 await ordered(p,"architect_risk_cause",id,v.causes,text);
 await ordered(p,"architect_risk_effect",id,v.effects,text);
 await ordered(p,"architect_risk_affected_element",id,v.affectedElementIds,text);
 await ordered(p,"architect_risk_affected_requirement",id,v.affectedRequirementIds,text);
 await ordered(p,"architect_risk_mitigation",id,v.mitigation,text);
 await ordered(p,"architect_risk_contingency",id,v.contingency,text);
 await ordered(p,"architect_risk_trigger_indicator",id,v.triggerIndicators,text);
 await ordered(p,"architect_risk_related_conflict",id,v.relatedConflictIds,text);
 await ordered(p,"architect_risk_escalation_path",id,v.escalationPath,text);
}
export async function restoreRisk(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Risk>{
 const row=await r.body("architect_risk",hrow.rowid,14),c=new Cursor(row!);
 const riskStatement=c.textField();
 const category=c.text();
 const probability=c.symbol(RiskLevel);
 const impact=c.symbol(RiskLevel);
 const riskScore=c.optional(3,()=>c.float());
 const ownerId=c.optional(1,()=>c.text());
 const reviewDate=c.optional(1,()=>c.text());
 const residualProbability=c.optional(1,()=>c.symbol(RiskLevel));
 const residualImpact=c.optional(1,()=>c.symbol(RiskLevel));
 const monitoringPlan=c.optional(1,()=>c.text());
 c.end();
 const causes=await r.list("architect_risk_cause",hrow.rowid,1,c=>c.text());
 const effects=await r.list("architect_risk_effect",hrow.rowid,1,c=>c.text());
 const affectedElementIds=await r.list("architect_risk_affected_element",hrow.rowid,1,c=>c.text());
 const affectedRequirementIds=await r.list("architect_risk_affected_requirement",hrow.rowid,1,c=>c.text());
 const mitigation=await r.list("architect_risk_mitigation",hrow.rowid,1,c=>c.text());
 const contingency=await r.list("architect_risk_contingency",hrow.rowid,1,c=>c.text());
 const triggerIndicators=await r.list("architect_risk_trigger_indicator",hrow.rowid,1,c=>c.text());
 const relatedConflictIds=await r.list("architect_risk_related_conflict",hrow.rowid,1,c=>c.text());
 const escalationPath=await r.list("architect_risk_escalation_path",hrow.rowid,1,c=>c.text());
 return{...h,riskStatement,category,probability,impact,riskScore,ownerId,reviewDate,residualProbability,residualImpact,monitoringPlan,causes,effects,affectedElementIds,affectedRequirementIds,mitigation,contingency,triggerIndicators,relatedConflictIds,escalationPath};
}
export async function projectConflict(p:ArtifactSqliteProjection,id:bigint,v:Conflict):Promise<void>{
 await p.insert("architect_conflict",[...symbol(v.kind,ConflictKind),...textField(v.summary),...text(v.entityAId),...text(v.entityBId),...symbol(v.severity,IssueSeverity),...optional(v.detectedBy,1,text),...optional(v.detectionDate,1,text),...optionalTextField(v.recommendedResolution),...optional(v.decisionId,1,text),...optional(v.costImpact,3,float),...optional(v.scheduleImpact,1,text),...symbol(v.resolutionStatus,ValidationStatus),...optional(v.ownerId,1,text),...optional(v.escalationLevel,1,text)],id);
 await ordered(p,"architect_conflict_trade_off_option",id,v.tradeOffOptions,text);
 await ordered(p,"architect_conflict_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_conflict_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_conflict_quality_impact",id,v.qualityImpact,text);
 await ordered(p,"architect_conflict_related_risk",id,v.relatedRiskIds,text);
}
export async function restoreConflict(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Conflict>{
 const row=await r.body("architect_conflict",hrow.rowid,20),c=new Cursor(row!);
 const kind=c.symbol(ConflictKind);
 const summary=c.textField();
 const entityAId=c.text();
 const entityBId=c.text();
 const severity=c.symbol(IssueSeverity);
 const detectedBy=c.optional(1,()=>c.text());
 const detectionDate=c.optional(1,()=>c.text());
 const recommendedResolution=c.optionalTextField();
 const decisionId=c.optional(1,()=>c.text());
 const costImpact=c.optional(3,()=>c.float());
 const scheduleImpact=c.optional(1,()=>c.text());
 const resolutionStatus=c.symbol(ValidationStatus);
 const ownerId=c.optional(1,()=>c.text());
 const escalationLevel=c.optional(1,()=>c.text());
 c.end();
 const tradeOffOptions=await r.list("architect_conflict_trade_off_option",hrow.rowid,1,c=>c.text());
 const stakeholderIds=await r.list("architect_conflict_stakeholder",hrow.rowid,1,c=>c.text());
 const requirementIds=await r.list("architect_conflict_requirement",hrow.rowid,1,c=>c.text());
 const qualityImpact=await r.list("architect_conflict_quality_impact",hrow.rowid,1,c=>c.text());
 const relatedRiskIds=await r.list("architect_conflict_related_risk",hrow.rowid,1,c=>c.text());
 return{...h,kind,summary,entityAId,entityBId,severity,detectedBy,detectionDate,recommendedResolution,decisionId,costImpact,scheduleImpact,resolutionStatus,ownerId,escalationLevel,tradeOffOptions,stakeholderIds,requirementIds,qualityImpact,relatedRiskIds};
}
export async function projectRequirement(p:ArtifactSqliteProjection,id:bigint,v:Requirement):Promise<void>{
 await p.insert("architect_requirement",[...text(v.code),...symbol(v.kind,RequirementKind),...textField(v.statement),...optionalTextField(v.rationale),...optional(v.source,1,text),...optional(v.parentRequirementId,1,text),...optional(v.verificationMethod,1,text),...symbol(v.validationStatus,ValidationStatus),...optional(v.costEstimate,3,float),...optional(v.scheduleConstraint,1,text),...optional(v.supersededBy,1,text)],id);
 await ordered(p,"architect_requirement_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_requirement_element",id,v.elementIds,text);
 await ordered(p,"architect_requirement_function",id,v.functionIds,text);
 await ordered(p,"architect_requirement_child",id,v.childRequirementIds,text);
 await ordered(p,"architect_requirement_acceptance_criterion",id,v.acceptanceCriteria,text);
 await ordered(p,"architect_requirement_conflict",id,v.conflictIds,text);
 await ordered(p,"architect_requirement_risk",id,v.riskIds,text);
 await ordered(p,"architect_requirement_regulatory_ref",id,v.regulatoryRefs,text);
 await ordered(p,"architect_requirement_trace",id,v.traceLinks,trace);
}
export async function restoreRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Requirement>{
 const row=await r.body("architect_requirement",hrow.rowid,17),c=new Cursor(row!);
 const code=c.text();
 const kind=c.symbol(RequirementKind);
 const statement=c.textField();
 const rationale=c.optionalTextField();
 const source=c.optional(1,()=>c.text());
 const parentRequirementId=c.optional(1,()=>c.text());
 const verificationMethod=c.optional(1,()=>c.text());
 const validationStatus=c.symbol(ValidationStatus);
 const costEstimate=c.optional(3,()=>c.float());
 const scheduleConstraint=c.optional(1,()=>c.text());
 const supersededBy=c.optional(1,()=>c.text());
 c.end();
 const stakeholderIds=await r.list("architect_requirement_stakeholder",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_requirement_element",hrow.rowid,1,c=>c.text());
 const functionIds=await r.list("architect_requirement_function",hrow.rowid,1,c=>c.text());
 const childRequirementIds=await r.list("architect_requirement_child",hrow.rowid,1,c=>c.text());
 const acceptanceCriteria=await r.list("architect_requirement_acceptance_criterion",hrow.rowid,1,c=>c.text());
 const conflictIds=await r.list("architect_requirement_conflict",hrow.rowid,1,c=>c.text());
 const riskIds=await r.list("architect_requirement_risk",hrow.rowid,1,c=>c.text());
 const regulatoryRefs=await r.list("architect_requirement_regulatory_ref",hrow.rowid,1,c=>c.text());
 const traceLinks=await r.list("architect_requirement_trace",hrow.rowid,5,c=>readTrace(c));
 return{...h,code,kind,statement,rationale,source,parentRequirementId,verificationMethod,validationStatus,costEstimate,scheduleConstraint,supersededBy,stakeholderIds,elementIds,functionIds,childRequirementIds,acceptanceCriteria,conflictIds,riskIds,regulatoryRefs,traceLinks};
}
export async function projectAuditEvent(p:ArtifactSqliteProjection,id:bigint,v:AuditEvent):Promise<void>{await p.insert("architect_audit_event",[...symbol(v.action,AuditAction),...optional(v.actorId,1,text),...text(v.subjectId),...text(v.subjectKind),...text(v.timestamp),...textField(v.details),...optional(v.beforeState,1,text),...optional(v.afterState,1,text),...optional(v.ipAddress,1,text),...optional(v.client,1,text),...optional(v.sessionId,1,text),...optional(v.changeRecordId,1,text),...boolean(v.success),...optional(v.errorMessage,1,text),...optional(v.correlationId,1,text),...optional(v.retentionUntil,1,text)],id);await ordered(p,"architect_audit_compliance_tag",id,v.complianceTags,text);if(v.traceLink!==null)await p.insert("architect_audit_trace",trace(v.traceLink),id)}
export async function restoreAuditEvent(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<AuditEvent>{const row=await r.body("architect_audit_event",hrow.rowid,18),c=new Cursor(row!),action=c.symbol(AuditAction),actorId=c.optional(1,()=>c.text()),subjectId=c.text(),subjectKind=c.text(),timestamp=c.text(),details=c.textField(),beforeState=c.optional(1,()=>c.text()),afterState=c.optional(1,()=>c.text()),ipAddress=c.optional(1,()=>c.text()),client=c.optional(1,()=>c.text()),sessionId=c.optional(1,()=>c.text()),changeRecordId=c.optional(1,()=>c.text()),success=c.boolean(),errorMessage=c.optional(1,()=>c.text()),correlationId=c.optional(1,()=>c.text()),retentionUntil=c.optional(1,()=>c.text());c.end();const complianceTags=await r.list("architect_audit_compliance_tag",hrow.rowid,1,c=>c.text()),traceRow=await r.body("architect_audit_trace",hrow.rowid,6,false),tc=traceRow?new Cursor(traceRow):null,traceLink=tc?readTrace(tc):null;tc?.end();return{...h,action,actorId,subjectId,subjectKind,timestamp,details,beforeState,afterState,ipAddress,client,sessionId,changeRecordId,success,errorMessage,correlationId,retentionUntil,complianceTags,traceLink}}
