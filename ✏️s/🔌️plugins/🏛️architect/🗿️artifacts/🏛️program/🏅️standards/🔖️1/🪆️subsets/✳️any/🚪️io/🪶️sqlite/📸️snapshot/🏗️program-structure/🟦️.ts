/** 🏗️ Program structure registers of the Architect SQLite projection. */
import type { EntityHeader } from "../../../../🧬️schema/🪪️entity-foundation/🟦️.ts";
import type { AccessRule, Activity, Adjacency, FlowRequirement, Function, Process, ProgramElement, QuantityRequirement, Relationship, Stakeholder, UserProfile } from "../../../../🧬️schema/🏗️program-structure/🟦️.ts";
import { ArtifactSqliteProjection } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import type { SqliteRow } from "@semio-tech/framework/io/sqlite-snapshot";
import { AccessLevel, AccessMode, AdjacencyKind, ConnectionKind, EngagementLevel, FlowDirection, FlowKind, FunctionKind, InfluenceLevel, Priority, ProgramElementKind, RelationshipKind, SeparationKind, UserCategory, ValidationStatus } from "../🏷️symbols/🟦️.ts";
import { boolean, Cursor, float, note, optional, optionalTextField, symbol, text, textField } from "../📏️cells/🟦️.ts";
import { ordered, quantity, Reader } from "../💰️backing/🟦️.ts";
import { readTrace, trace } from "../🔗️trace/🟦️.ts";

export async function projectFlowRequirement(p:ArtifactSqliteProjection,id:bigint,v:FlowRequirement):Promise<void>{
 await p.insert("architect_flow_requirement",[...text(v.fromElementId),...text(v.toElementId),...symbol(v.kind,FlowKind),...text(v.flowType),...symbol(v.direction,FlowDirection),...optional(v.peakRate,3,float),...optional(v.clearWidthM,3,float),...optional(v.clearHeightM,3,float),...symbol(v.accessLevel,AccessLevel),...optional(v.equipmentClearance,1,text),...boolean(v.signageRequired),...boolean(v.escortRequired),...boolean(v.emergencyRoute),...boolean(v.barrierFree),...boolean(v.monitoringRequired),...optional(v.processId,1,text),...optional(v.verificationMethod,1,text)],id);
 await ordered(p,"architect_flow_separation",id,v.separationRequirements,value=>symbol(value,SeparationKind));
 await ordered(p,"architect_flow_time_window",id,v.timeWindows,text);
 await ordered(p,"architect_flow_conflict",id,v.conflictIds,text);
 await quantity(p,id,"volume","architect_flow_volume",v.volume);
}
export async function restoreFlowRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<FlowRequirement>{
 const row=await r.body("architect_flow_requirement",hrow.rowid,24),c=new Cursor(row!);
 const fromElementId=c.text();
 const toElementId=c.text();
 const kind=c.symbol(FlowKind);
 const flowType=c.text();
 const direction=c.symbol(FlowDirection);
 const peakRate=c.optional(3,()=>c.float());
 const clearWidthM=c.optional(3,()=>c.float());
 const clearHeightM=c.optional(3,()=>c.float());
 const accessLevel=c.symbol(AccessLevel);
 const equipmentClearance=c.optional(1,()=>c.text());
 const signageRequired=c.boolean();
 const escortRequired=c.boolean();
 const emergencyRoute=c.boolean();
 const barrierFree=c.boolean();
 const monitoringRequired=c.boolean();
 const processId=c.optional(1,()=>c.text());
 const verificationMethod=c.optional(1,()=>c.text());
 c.end();
 const separationRequirements=await r.list("architect_flow_separation",hrow.rowid,1,c=>c.symbol(SeparationKind));
 const timeWindows=await r.list("architect_flow_time_window",hrow.rowid,1,c=>c.text());
 const conflictIds=await r.list("architect_flow_conflict",hrow.rowid,1,c=>c.text());
 const volume=await r.quantity(hrow.rowid,"volume","architect_flow_volume");
 return{...h,fromElementId,toElementId,kind,flowType,direction,peakRate,clearWidthM,clearHeightM,accessLevel,equipmentClearance,signageRequired,escortRequired,emergencyRoute,barrierFree,monitoringRequired,processId,verificationMethod,separationRequirements,timeWindows,conflictIds,volume};
}
export async function projectAccessRule(p:ArtifactSqliteProjection,id:bigint,v:AccessRule):Promise<void>{
 await p.insert("architect_access_rule",[...symbol(v.accessLevel,AccessLevel),...symbol(v.accessMode,AccessMode),...optional(v.escortPolicy,1,text),...optional(v.visitorPolicy,1,text),...boolean(v.emergencyOverride),...boolean(v.auditRequired),...boolean(v.badgeRequired),...boolean(v.biometricRequired),...optional(v.enforcementMethod,1,text),...optional(v.revocationPolicy,1,text),...boolean(v.trainingRequired),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_access_subject",id,v.subjectIds,text);
 await ordered(p,"architect_access_resource",id,v.resourceIds,text);
 await ordered(p,"architect_access_authentication",id,v.authentication,text);
 await ordered(p,"architect_access_authorization",id,v.authorization,text);
 await ordered(p,"architect_access_time_restriction",id,v.timeRestrictions,text);
 await ordered(p,"architect_access_zone",id,v.zoneIds,text);
 await ordered(p,"architect_access_exception",id,v.exceptions,text);
 await ordered(p,"architect_access_regulatory_basis",id,v.regulatoryBasis,text);
}
export async function restoreAccessRule(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<AccessRule>{
 const row=await r.body("architect_access_rule",hrow.rowid,13),c=new Cursor(row!);
 const accessLevel=c.symbol(AccessLevel);
 const accessMode=c.symbol(AccessMode);
 const escortPolicy=c.optional(1,()=>c.text());
 const visitorPolicy=c.optional(1,()=>c.text());
 const emergencyOverride=c.boolean();
 const auditRequired=c.boolean();
 const badgeRequired=c.boolean();
 const biometricRequired=c.boolean();
 const enforcementMethod=c.optional(1,()=>c.text());
 const revocationPolicy=c.optional(1,()=>c.text());
 const trainingRequired=c.boolean();
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const subjectIds=await r.list("architect_access_subject",hrow.rowid,1,c=>c.text());
 const resourceIds=await r.list("architect_access_resource",hrow.rowid,1,c=>c.text());
 const authentication=await r.list("architect_access_authentication",hrow.rowid,1,c=>c.text());
 const authorization=await r.list("architect_access_authorization",hrow.rowid,1,c=>c.text());
 const timeRestrictions=await r.list("architect_access_time_restriction",hrow.rowid,1,c=>c.text());
 const zoneIds=await r.list("architect_access_zone",hrow.rowid,1,c=>c.text());
 const exceptions=await r.list("architect_access_exception",hrow.rowid,1,c=>c.text());
 const regulatoryBasis=await r.list("architect_access_regulatory_basis",hrow.rowid,1,c=>c.text());
 return{...h,accessLevel,accessMode,escortPolicy,visitorPolicy,emergencyOverride,auditRequired,badgeRequired,biometricRequired,enforcementMethod,revocationPolicy,trainingRequired,ownerId,subjectIds,resourceIds,authentication,authorization,timeRestrictions,zoneIds,exceptions,regulatoryBasis};
}
export async function projectAdjacency(p:ArtifactSqliteProjection,id:bigint,v:Adjacency):Promise<void>{
 await p.insert("architect_adjacency",[...text(v.elementAId),...text(v.elementBId),...symbol(v.kind,AdjacencyKind),...symbol(v.connection,ConnectionKind),...float(v.weight),...optionalTextField(v.rationale),...optional(v.distanceMaxM,3,float),...optional(v.distanceMinM,3,float),...optional(v.levelConstraint,1,text),...optional(v.accessPath,1,text),...boolean(v.sharedWall),...boolean(v.sharedEntry),...boolean(v.trafficIsolation),...boolean(v.circulationOverlap),...boolean(v.normalized),...symbol(v.verificationStatus,ValidationStatus),...optional(v.sourceRelationshipId,1,text),...optional(v.internalExternalAccess,1,text)],id);
 await ordered(p,"architect_adjacency_separation",id,v.separations,value=>symbol(value,SeparationKind));
 await ordered(p,"architect_adjacency_conflict",id,v.conflictIds,text);
}
export async function restoreAdjacency(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Adjacency>{
 const row=await r.body("architect_adjacency",hrow.rowid,27),c=new Cursor(row!);
 const elementAId=c.text();
 const elementBId=c.text();
 const kind=c.symbol(AdjacencyKind);
 const connection=c.symbol(ConnectionKind);
 const weight=c.float();
 const rationale=c.optionalTextField();
 const distanceMaxM=c.optional(3,()=>c.float());
 const distanceMinM=c.optional(3,()=>c.float());
 const levelConstraint=c.optional(1,()=>c.text());
 const accessPath=c.optional(1,()=>c.text());
 const sharedWall=c.boolean();
 const sharedEntry=c.boolean();
 const trafficIsolation=c.boolean();
 const circulationOverlap=c.boolean();
 const normalized=c.boolean();
 const verificationStatus=c.symbol(ValidationStatus);
 const sourceRelationshipId=c.optional(1,()=>c.text());
 const internalExternalAccess=c.optional(1,()=>c.text());
 c.end();
 const separations=await r.list("architect_adjacency_separation",hrow.rowid,1,c=>c.symbol(SeparationKind));
 const conflictIds=await r.list("architect_adjacency_conflict",hrow.rowid,1,c=>c.text());
 return{...h,elementAId,elementBId,kind,connection,weight,rationale,distanceMaxM,distanceMinM,levelConstraint,accessPath,sharedWall,sharedEntry,trafficIsolation,circulationOverlap,normalized,verificationStatus,sourceRelationshipId,internalExternalAccess,separations,conflictIds};
}
export async function projectProcess(p:ArtifactSqliteProjection,id:bigint,v:Process):Promise<void>{
 await p.insert("architect_process",[...text(v.code),...text(v.category),...optional(v.trigger,1,text),...optional(v.duration,1,text),...optional(v.frequency,1,text),...boolean(v.criticalPath),...optional(v.automationLevel,1,text),...optional(v.ownerId,1,text),...optional(v.workflowType,1,text)],id);
 await ordered(p,"architect_process_input",id,v.inputs,text);
 await ordered(p,"architect_process_output",id,v.outputs,text);
 await ordered(p,"architect_process_step",id,v.steps,text);
 await ordered(p,"architect_process_actor",id,v.actors,text);
 await ordered(p,"architect_process_equipment",id,v.equipmentIds,text);
 await ordered(p,"architect_process_element",id,v.elementIds,text);
 await ordered(p,"architect_process_bottleneck",id,v.bottlenecks,text);
 await ordered(p,"architect_process_dependency",id,v.dependencies,text);
 await ordered(p,"architect_process_kpi",id,v.kpis,text);
 await ordered(p,"architect_process_failure_mode",id,v.failureModes,text);
 await ordered(p,"architect_process_improvement_opportunity",id,v.improvementOpportunities,text);
 await ordered(p,"architect_process_regulatory_reference",id,v.regulatoryRefs,text);
 await ordered(p,"architect_process_handoff_point",id,v.handoffPoints,text);
 await ordered(p,"architect_process_quality_gate",id,v.qualityGates,text);
}
export async function restoreProcess(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Process>{
 const row=await r.body("architect_process",hrow.rowid,10),c=new Cursor(row!);
 const code=c.text();
 const category=c.text();
 const trigger=c.optional(1,()=>c.text());
 const duration=c.optional(1,()=>c.text());
 const frequency=c.optional(1,()=>c.text());
 const criticalPath=c.boolean();
 const automationLevel=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const workflowType=c.optional(1,()=>c.text());
 c.end();
 const inputs=await r.list("architect_process_input",hrow.rowid,1,c=>c.text());
 const outputs=await r.list("architect_process_output",hrow.rowid,1,c=>c.text());
 const steps=await r.list("architect_process_step",hrow.rowid,1,c=>c.text());
 const actors=await r.list("architect_process_actor",hrow.rowid,1,c=>c.text());
 const equipmentIds=await r.list("architect_process_equipment",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_process_element",hrow.rowid,1,c=>c.text());
 const bottlenecks=await r.list("architect_process_bottleneck",hrow.rowid,1,c=>c.text());
 const dependencies=await r.list("architect_process_dependency",hrow.rowid,1,c=>c.text());
 const kpis=await r.list("architect_process_kpi",hrow.rowid,1,c=>c.text());
 const failureModes=await r.list("architect_process_failure_mode",hrow.rowid,1,c=>c.text());
 const improvementOpportunities=await r.list("architect_process_improvement_opportunity",hrow.rowid,1,c=>c.text());
 const regulatoryRefs=await r.list("architect_process_regulatory_reference",hrow.rowid,1,c=>c.text());
 const handoffPoints=await r.list("architect_process_handoff_point",hrow.rowid,1,c=>c.text());
 const qualityGates=await r.list("architect_process_quality_gate",hrow.rowid,1,c=>c.text());
 return{...h,code,category,trigger,duration,frequency,criticalPath,automationLevel,ownerId,workflowType,inputs,outputs,steps,actors,equipmentIds,elementIds,bottlenecks,dependencies,kpis,failureModes,improvementOpportunities,regulatoryRefs,handoffPoints,qualityGates};
}
export async function projectQuantityRequirement(p:ArtifactSqliteProjection,id:bigint,v:QuantityRequirement):Promise<void>{
 await p.insert("architect_quantity_requirement",[...text(v.targetElementId),...text(v.metric),...optional(v.basis,1,text),...optional(v.calculationMethod,1,text),...optional(v.source,1,text),...optional(v.benchmarkRef,1,text),...optional(v.tolerancePercent,3,float),...optional(v.peakFactor,3,float),...optional(v.growthFactor,3,float),...optional(v.unitCost,3,float),...optional(v.currency,1,text),...optional(v.verificationMethod,1,text),...optional(v.schedulePhase,1,text),...optional(v.responsibleParty,1,text),...optional(v.lastVerified,1,text)],id);
 await ordered(p,"architect_quantity_requirement_related_requirement",id,v.relatedRequirementIds,text);
 await ordered(p,"architect_quantity_requirement_assumption",id,v.assumptions,text);
 await ordered(p,"architect_quantity_requirement_constraint",id,v.constraints,text);
 await ordered(p,"architect_quantity_requirement_variance_note",id,v.varianceNotes,note);
 await quantity(p,id,"quantity","architect_quantity_requirement_quantity",v.quantity);
}
export async function restoreQuantityRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<QuantityRequirement>{
 const row=await r.body("architect_quantity_requirement",hrow.rowid,24),c=new Cursor(row!);
 const targetElementId=c.text();
 const metric=c.text();
 const basis=c.optional(1,()=>c.text());
 const calculationMethod=c.optional(1,()=>c.text());
 const source=c.optional(1,()=>c.text());
 const benchmarkRef=c.optional(1,()=>c.text());
 const tolerancePercent=c.optional(3,()=>c.float());
 const peakFactor=c.optional(3,()=>c.float());
 const growthFactor=c.optional(3,()=>c.float());
 const unitCost=c.optional(3,()=>c.float());
 const currency=c.optional(1,()=>c.text());
 const verificationMethod=c.optional(1,()=>c.text());
 const schedulePhase=c.optional(1,()=>c.text());
 const responsibleParty=c.optional(1,()=>c.text());
 const lastVerified=c.optional(1,()=>c.text());
 c.end();
 const relatedRequirementIds=await r.list("architect_quantity_requirement_related_requirement",hrow.rowid,1,c=>c.text());
 const assumptions=await r.list("architect_quantity_requirement_assumption",hrow.rowid,1,c=>c.text());
 const constraints=await r.list("architect_quantity_requirement_constraint",hrow.rowid,1,c=>c.text());
 const varianceNotes=await r.list("architect_quantity_requirement_variance_note",hrow.rowid,2,c=>c.note());
 const quantity=await r.quantity(hrow.rowid,"quantity","architect_quantity_requirement_quantity");
 return{...h,targetElementId,metric,basis,calculationMethod,source,benchmarkRef,tolerancePercent,peakFactor,growthFactor,unitCost,currency,verificationMethod,schedulePhase,responsibleParty,lastVerified,relatedRequirementIds,assumptions,constraints,varianceNotes,quantity};
}
export async function projectRelationship(p:ArtifactSqliteProjection,id:bigint,v:Relationship):Promise<void>{
 await p.insert("architect_relationship",[...text(v.sourceId),...text(v.targetId),...symbol(v.kind,RelationshipKind),...optional(v.strength,3,float),...boolean(v.directional),...optionalTextField(v.rationale),...symbol(v.relationshipPriority,Priority),...optional(v.validFrom,1,text),...optional(v.validUntil,1,text),...boolean(v.bidirectional),...optional(v.distanceConstraintM,3,float),...optional(v.capacityConstraint,1,text),...optional(v.reviewCycle,1,text),...optional(v.ownerId,1,text),...optionalTextField(v.proximityRequirement),...optionalTextField(v.compatibilityRequirement),...optionalTextField(v.incompatibilityRequirement)],id);
 await ordered(p,"architect_relationship_constraint",id,v.constraints,text);
 await ordered(p,"architect_relationship_condition",id,v.conditions,text);
 await ordered(p,"architect_relationship_evidence",id,v.evidence,text);
 await ordered(p,"architect_relationship_conflict",id,v.conflictIds,text);
 await ordered(p,"architect_relationship_regulatory_basis",id,v.regulatoryBasis,text);
 await ordered(p,"architect_relationship_separation",id,v.separationRequirements,value=>symbol(value,SeparationKind));
 await ordered(p,"architect_relationship_trace",id,v.traceLinks,trace);
}
export async function restoreRelationship(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Relationship>{
 const row=await r.body("architect_relationship",hrow.rowid,30),c=new Cursor(row!);
 const sourceId=c.text();
 const targetId=c.text();
 const kind=c.symbol(RelationshipKind);
 const strength=c.optional(3,()=>c.float());
 const directional=c.boolean();
 const rationale=c.optionalTextField();
 const relationshipPriority=c.symbol(Priority);
 const validFrom=c.optional(1,()=>c.text());
 const validUntil=c.optional(1,()=>c.text());
 const bidirectional=c.boolean();
 const distanceConstraintM=c.optional(3,()=>c.float());
 const capacityConstraint=c.optional(1,()=>c.text());
 const reviewCycle=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const proximityRequirement=c.optionalTextField();
 const compatibilityRequirement=c.optionalTextField();
 const incompatibilityRequirement=c.optionalTextField();
 c.end();
 const constraints=await r.list("architect_relationship_constraint",hrow.rowid,1,c=>c.text());
 const conditions=await r.list("architect_relationship_condition",hrow.rowid,1,c=>c.text());
 const evidence=await r.list("architect_relationship_evidence",hrow.rowid,1,c=>c.text());
 const conflictIds=await r.list("architect_relationship_conflict",hrow.rowid,1,c=>c.text());
 const regulatoryBasis=await r.list("architect_relationship_regulatory_basis",hrow.rowid,1,c=>c.text());
 const separationRequirements=await r.list("architect_relationship_separation",hrow.rowid,1,c=>c.symbol(SeparationKind));
 const traceLinks=await r.list("architect_relationship_trace",hrow.rowid,5,c=>readTrace(c));
 return{...h,sourceId,targetId,kind,strength,directional,rationale,relationshipPriority,validFrom,validUntil,bidirectional,distanceConstraintM,capacityConstraint,reviewCycle,ownerId,proximityRequirement,compatibilityRequirement,incompatibilityRequirement,constraints,conditions,evidence,conflictIds,regulatoryBasis,separationRequirements,traceLinks};
}
export async function projectUserProfile(p:ArtifactSqliteProjection,id:bigint,v:UserProfile):Promise<void>{
 await p.insert("architect_user_profile",[...symbol(v.category,UserCategory),...optional(v.demographic,1,text),...optional(v.ageRange,1,text),...optional(v.occupation,1,text),...optional(v.roleTitle,1,text),...optional(v.department,1,text),...optional(v.usageFrequency,1,text),...optional(v.usageDuration,1,text),...optional(v.technologyProficiency,1,text),...optional(v.researchMethod,1,text),...optional(v.personaArchetype,1,text),...boolean(v.validated)],id);
 await ordered(p,"architect_user_ability",id,v.abilities,text);
 await ordered(p,"architect_user_disability",id,v.disabilities,text);
 await ordered(p,"architect_user_mobility_profile",id,v.mobilityProfile,text);
 await ordered(p,"architect_user_sensory_profile",id,v.sensoryProfile,text);
 await ordered(p,"architect_user_cognitive_profile",id,v.cognitiveProfile,text);
 await ordered(p,"architect_user_behavioral_pattern",id,v.behavioralPatterns,text);
 await ordered(p,"architect_user_peak_usage_time",id,v.peakUsageTimes,text);
 await ordered(p,"architect_user_preference",id,v.preferences,text);
 await ordered(p,"architect_user_pain_point",id,v.painPoints,text);
 await ordered(p,"architect_user_goal",id,v.goals,text);
 await ordered(p,"architect_user_activity",id,v.activityIds,text);
 await ordered(p,"architect_user_stakeholder",id,v.stakeholderIds,text);
}
export async function restoreUserProfile(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<UserProfile>{
 const row=await r.body("architect_user_profile",hrow.rowid,13),c=new Cursor(row!);
 const category=c.symbol(UserCategory);
 const demographic=c.optional(1,()=>c.text());
 const ageRange=c.optional(1,()=>c.text());
 const occupation=c.optional(1,()=>c.text());
 const roleTitle=c.optional(1,()=>c.text());
 const department=c.optional(1,()=>c.text());
 const usageFrequency=c.optional(1,()=>c.text());
 const usageDuration=c.optional(1,()=>c.text());
 const technologyProficiency=c.optional(1,()=>c.text());
 const researchMethod=c.optional(1,()=>c.text());
 const personaArchetype=c.optional(1,()=>c.text());
 const validated=c.boolean();
 c.end();
 const abilities=await r.list("architect_user_ability",hrow.rowid,1,c=>c.text());
 const disabilities=await r.list("architect_user_disability",hrow.rowid,1,c=>c.text());
 const mobilityProfile=await r.list("architect_user_mobility_profile",hrow.rowid,1,c=>c.text());
 const sensoryProfile=await r.list("architect_user_sensory_profile",hrow.rowid,1,c=>c.text());
 const cognitiveProfile=await r.list("architect_user_cognitive_profile",hrow.rowid,1,c=>c.text());
 const behavioralPatterns=await r.list("architect_user_behavioral_pattern",hrow.rowid,1,c=>c.text());
 const peakUsageTimes=await r.list("architect_user_peak_usage_time",hrow.rowid,1,c=>c.text());
 const preferences=await r.list("architect_user_preference",hrow.rowid,1,c=>c.text());
 const painPoints=await r.list("architect_user_pain_point",hrow.rowid,1,c=>c.text());
 const goals=await r.list("architect_user_goal",hrow.rowid,1,c=>c.text());
 const activityIds=await r.list("architect_user_activity",hrow.rowid,1,c=>c.text());
 const stakeholderIds=await r.list("architect_user_stakeholder",hrow.rowid,1,c=>c.text());
 return{...h,category,demographic,ageRange,occupation,roleTitle,department,usageFrequency,usageDuration,technologyProficiency,researchMethod,personaArchetype,validated,abilities,disabilities,mobilityProfile,sensoryProfile,cognitiveProfile,behavioralPatterns,peakUsageTimes,preferences,painPoints,goals,activityIds,stakeholderIds};
}
export async function projectActivity(p:ArtifactSqliteProjection,id:bigint,v:Activity):Promise<void>{
 await p.insert("architect_activity",[...text(v.code),...text(v.category),...optional(v.frequency,1,text),...optional(v.duration,1,text),...optional(v.intensity,1,text),...text(v.activityType),...optional(v.locationContext,1,text),...optional(v.temporalPattern,1,text),...optional(v.supervisionLevel,1,text)],id);
 await ordered(p,"architect_activity_equipment",id,v.equipmentIds,text);
 await ordered(p,"architect_activity_space_requirement",id,v.spaceRequirements,text);
 await ordered(p,"architect_activity_environmental_need",id,v.environmentalNeeds,text);
 await ordered(p,"architect_activity_privacy_need",id,v.privacyNeeds,text);
 await ordered(p,"architect_activity_accessibility_need",id,v.accessibilityNeeds,text);
 await ordered(p,"architect_activity_adjacency",id,v.adjacentActivities,text);
 await ordered(p,"architect_activity_sequence",id,v.sequencing,text);
 await ordered(p,"architect_activity_peak_period",id,v.peakPeriods,text);
 await ordered(p,"architect_activity_workflow_step",id,v.workflowSteps,text);
 await ordered(p,"architect_activity_input",id,v.inputs,text);
 await ordered(p,"architect_activity_output",id,v.outputs,text);
 await ordered(p,"architect_activity_user_profile",id,v.userProfileIds,text);
 await ordered(p,"architect_activity_function",id,v.functionIds,text);
 await ordered(p,"architect_activity_performance_indicator",id,v.performanceIndicators,text);
 await quantity(p,id,"participants","architect_activity_participants",v.participants);
}
export async function restoreActivity(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Activity>{
 const row=await r.body("architect_activity",hrow.rowid,10),c=new Cursor(row!);
 const code=c.text();
 const category=c.text();
 const frequency=c.optional(1,()=>c.text());
 const duration=c.optional(1,()=>c.text());
 const intensity=c.optional(1,()=>c.text());
 const activityType=c.text();
 const locationContext=c.optional(1,()=>c.text());
 const temporalPattern=c.optional(1,()=>c.text());
 const supervisionLevel=c.optional(1,()=>c.text());
 c.end();
 const equipmentIds=await r.list("architect_activity_equipment",hrow.rowid,1,c=>c.text());
 const spaceRequirements=await r.list("architect_activity_space_requirement",hrow.rowid,1,c=>c.text());
 const environmentalNeeds=await r.list("architect_activity_environmental_need",hrow.rowid,1,c=>c.text());
 const privacyNeeds=await r.list("architect_activity_privacy_need",hrow.rowid,1,c=>c.text());
 const accessibilityNeeds=await r.list("architect_activity_accessibility_need",hrow.rowid,1,c=>c.text());
 const adjacentActivities=await r.list("architect_activity_adjacency",hrow.rowid,1,c=>c.text());
 const sequencing=await r.list("architect_activity_sequence",hrow.rowid,1,c=>c.text());
 const peakPeriods=await r.list("architect_activity_peak_period",hrow.rowid,1,c=>c.text());
 const workflowSteps=await r.list("architect_activity_workflow_step",hrow.rowid,1,c=>c.text());
 const inputs=await r.list("architect_activity_input",hrow.rowid,1,c=>c.text());
 const outputs=await r.list("architect_activity_output",hrow.rowid,1,c=>c.text());
 const userProfileIds=await r.list("architect_activity_user_profile",hrow.rowid,1,c=>c.text());
 const functionIds=await r.list("architect_activity_function",hrow.rowid,1,c=>c.text());
 const performanceIndicators=await r.list("architect_activity_performance_indicator",hrow.rowid,1,c=>c.text());
 const participants=await r.quantity(hrow.rowid,"participants","architect_activity_participants");
 return{...h,code,category,frequency,duration,intensity,activityType,locationContext,temporalPattern,supervisionLevel,equipmentIds,spaceRequirements,environmentalNeeds,privacyNeeds,accessibilityNeeds,adjacentActivities,sequencing,peakPeriods,workflowSteps,inputs,outputs,userProfileIds,functionIds,performanceIndicators,participants};
}
export async function projectFunction(p:ArtifactSqliteProjection,id:bigint,v:Function):Promise<void>{
 await p.insert("architect_function",[...text(v.code),...symbol(v.kind,FunctionKind),...textField(v.purpose),...symbol(v.criticality,Priority),...optional(v.serviceLevel,1,text),...optional(v.operatingHours,1,text),...optional(v.ownerStakeholderId,1,text),...optional(v.hierarchyParentId,1,text)],id);
 await ordered(p,"architect_function_performance_target",id,v.performanceTargets,text);
 await ordered(p,"architect_function_equipment",id,v.equipmentIds,text);
 await ordered(p,"architect_function_resource",id,v.resourceIds,text);
 await ordered(p,"architect_function_activity",id,v.activityIds,text);
 await ordered(p,"architect_function_element",id,v.elementIds,text);
 await ordered(p,"architect_function_dependency",id,v.dependencies,text);
 await ordered(p,"architect_function_interface",id,v.interfaces,text);
 await ordered(p,"architect_function_constraint",id,v.constraints,text);
 await ordered(p,"architect_function_quality_criterion",id,v.qualityCriteria,text);
 await ordered(p,"architect_function_regulatory_reference",id,v.regulatoryRefs,text);
 await ordered(p,"architect_function_future_change",id,v.futureChanges,text);
 await ordered(p,"architect_function_success_metric",id,v.successMetrics,text);
 await ordered(p,"architect_function_conflict",id,v.conflictIds,text);
 await quantity(p,id,"staffing","architect_function_staffing",v.staffing);
}
export async function restoreFunction(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Function>{
 const row=await r.body("architect_function",hrow.rowid,10),c=new Cursor(row!);
 const code=c.text();
 const kind=c.symbol(FunctionKind);
 const purpose=c.textField();
 const criticality=c.symbol(Priority);
 const serviceLevel=c.optional(1,()=>c.text());
 const operatingHours=c.optional(1,()=>c.text());
 const ownerStakeholderId=c.optional(1,()=>c.text());
 const hierarchyParentId=c.optional(1,()=>c.text());
 c.end();
 const performanceTargets=await r.list("architect_function_performance_target",hrow.rowid,1,c=>c.text());
 const equipmentIds=await r.list("architect_function_equipment",hrow.rowid,1,c=>c.text());
 const resourceIds=await r.list("architect_function_resource",hrow.rowid,1,c=>c.text());
 const activityIds=await r.list("architect_function_activity",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_function_element",hrow.rowid,1,c=>c.text());
 const dependencies=await r.list("architect_function_dependency",hrow.rowid,1,c=>c.text());
 const interfaces=await r.list("architect_function_interface",hrow.rowid,1,c=>c.text());
 const constraints=await r.list("architect_function_constraint",hrow.rowid,1,c=>c.text());
 const qualityCriteria=await r.list("architect_function_quality_criterion",hrow.rowid,1,c=>c.text());
 const regulatoryRefs=await r.list("architect_function_regulatory_reference",hrow.rowid,1,c=>c.text());
 const futureChanges=await r.list("architect_function_future_change",hrow.rowid,1,c=>c.text());
 const successMetrics=await r.list("architect_function_success_metric",hrow.rowid,1,c=>c.text());
 const conflictIds=await r.list("architect_function_conflict",hrow.rowid,1,c=>c.text());
 const staffing=await r.quantity(hrow.rowid,"staffing","architect_function_staffing");
 return{...h,code,kind,purpose,criticality,serviceLevel,operatingHours,ownerStakeholderId,hierarchyParentId,performanceTargets,equipmentIds,resourceIds,activityIds,elementIds,dependencies,interfaces,constraints,qualityCriteria,regulatoryRefs,futureChanges,successMetrics,conflictIds,staffing};
}
export async function projectProgramElement(p:ArtifactSqliteProjection,id:bigint,v:ProgramElement):Promise<void>{
 await p.insert("architect_program_element",[...text(v.code),...symbol(v.kind,ProgramElementKind),...optional(v.parentId,1,text),...optional(v.level,1,text),...optional(v.locationHint,1,text),...optional(v.orientation,1,text),...optional(v.daylightRequirement,1,text),...optional(v.acousticClass,1,text),...optional(v.securityZone,1,text),...optional(v.growthAllocation,1,text),...optional(v.circulationRole,1,text),...optional(v.visibilityLevel,1,text),...optional(v.environmentalZone,1,text)],id);
 await ordered(p,"architect_element_function",id,v.functionIds,text);
 await ordered(p,"architect_element_activity",id,v.activityIds,text);
 await ordered(p,"architect_element_user_profile",id,v.userProfileIds,text);
 await ordered(p,"architect_element_adjacency",id,v.adjacencyIds,text);
 await ordered(p,"architect_element_quantity",id,v.quantityIds,text);
 await ordered(p,"architect_element_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_element_flexibility_note",id,v.flexibilityNotes,text);
 await ordered(p,"architect_element_adjacency_preference",id,v.adjacencyPreferences,text);
 await quantity(p,id,"area","architect_element_area",v.area);
 await quantity(p,id,"volume","architect_element_volume",v.volume);
 await quantity(p,id,"height","architect_element_height",v.height);
 await quantity(p,id,"occupancy","architect_element_occupancy",v.occupancy);
}
export async function restoreProgramElement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ProgramElement>{
 const row=await r.body("architect_program_element",hrow.rowid,14),c=new Cursor(row!);
 const code=c.text();
 const kind=c.symbol(ProgramElementKind);
 const parentId=c.optional(1,()=>c.text());
 const level=c.optional(1,()=>c.text());
 const locationHint=c.optional(1,()=>c.text());
 const orientation=c.optional(1,()=>c.text());
 const daylightRequirement=c.optional(1,()=>c.text());
 const acousticClass=c.optional(1,()=>c.text());
 const securityZone=c.optional(1,()=>c.text());
 const growthAllocation=c.optional(1,()=>c.text());
 const circulationRole=c.optional(1,()=>c.text());
 const visibilityLevel=c.optional(1,()=>c.text());
 const environmentalZone=c.optional(1,()=>c.text());
 c.end();
 const functionIds=await r.list("architect_element_function",hrow.rowid,1,c=>c.text());
 const activityIds=await r.list("architect_element_activity",hrow.rowid,1,c=>c.text());
 const userProfileIds=await r.list("architect_element_user_profile",hrow.rowid,1,c=>c.text());
 const adjacencyIds=await r.list("architect_element_adjacency",hrow.rowid,1,c=>c.text());
 const quantityIds=await r.list("architect_element_quantity",hrow.rowid,1,c=>c.text());
 const requirementIds=await r.list("architect_element_requirement",hrow.rowid,1,c=>c.text());
 const flexibilityNotes=await r.list("architect_element_flexibility_note",hrow.rowid,1,c=>c.text());
 const adjacencyPreferences=await r.list("architect_element_adjacency_preference",hrow.rowid,1,c=>c.text());
 const area=await r.quantity(hrow.rowid,"area","architect_element_area");
 const volume=await r.quantity(hrow.rowid,"volume","architect_element_volume");
 const height=await r.quantity(hrow.rowid,"height","architect_element_height");
 const occupancy=await r.quantity(hrow.rowid,"occupancy","architect_element_occupancy");
 return{...h,code,kind,parentId,level,locationHint,orientation,daylightRequirement,acousticClass,securityZone,growthAllocation,circulationRole,visibilityLevel,environmentalZone,functionIds,activityIds,userProfileIds,adjacencyIds,quantityIds,requirementIds,flexibilityNotes,adjacencyPreferences,area,volume,height,occupancy};
}
export async function projectStakeholder(p:ArtifactSqliteProjection,id:bigint,v:Stakeholder):Promise<void>{
 await p.insert("architect_stakeholder",[...text(v.role),...text(v.organization),...optional(v.department,1,text),...optional(v.contactEmail,1,text),...optional(v.contactPhone,1,text),...symbol(v.influence,InfluenceLevel),...symbol(v.interest,InfluenceLevel),...symbol(v.engagement,EngagementLevel),...boolean(v.decisionAuthority),...optional(v.reportingFrequency,1,text),...optional(v.availability,1,text),...optional(v.representativeOf,1,text),...optional(v.delegatedTo,1,text),...optional(v.relationshipToClient,1,text),...text(v.stakeholderType),...optional(v.influenceStrategy,1,text)],id);
 await ordered(p,"architect_stakeholder_expectation",id,v.expectations,text);
 await ordered(p,"architect_stakeholder_concern",id,v.concerns,text);
 await ordered(p,"architect_stakeholder_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_stakeholder_communication_preference",id,v.communicationPreferences,text);
 await ordered(p,"architect_stakeholder_involvement_phase",id,v.involvementPhases,text);
 await ordered(p,"architect_stakeholder_power_interest_note",id,v.powerInterestNotes,note);
 await ordered(p,"architect_stakeholder_communication_channel",id,v.communicationChannels,text);
 await ordered(p,"architect_stakeholder_success_metric",id,v.successMetrics,text);
}
export async function restoreStakeholder(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Stakeholder>{
 const row=await r.body("architect_stakeholder",hrow.rowid,17),c=new Cursor(row!);
 const role=c.text();
 const organization=c.text();
 const department=c.optional(1,()=>c.text());
 const contactEmail=c.optional(1,()=>c.text());
 const contactPhone=c.optional(1,()=>c.text());
 const influence=c.symbol(InfluenceLevel);
 const interest=c.symbol(InfluenceLevel);
 const engagement=c.symbol(EngagementLevel);
 const decisionAuthority=c.boolean();
 const reportingFrequency=c.optional(1,()=>c.text());
 const availability=c.optional(1,()=>c.text());
 const representativeOf=c.optional(1,()=>c.text());
 const delegatedTo=c.optional(1,()=>c.text());
 const relationshipToClient=c.optional(1,()=>c.text());
 const stakeholderType=c.text();
 const influenceStrategy=c.optional(1,()=>c.text());
 c.end();
 const expectations=await r.list("architect_stakeholder_expectation",hrow.rowid,1,c=>c.text());
 const concerns=await r.list("architect_stakeholder_concern",hrow.rowid,1,c=>c.text());
 const requirementIds=await r.list("architect_stakeholder_requirement",hrow.rowid,1,c=>c.text());
 const communicationPreferences=await r.list("architect_stakeholder_communication_preference",hrow.rowid,1,c=>c.text());
 const involvementPhases=await r.list("architect_stakeholder_involvement_phase",hrow.rowid,1,c=>c.text());
 const powerInterestNotes=await r.list("architect_stakeholder_power_interest_note",hrow.rowid,2,c=>c.note());
 const communicationChannels=await r.list("architect_stakeholder_communication_channel",hrow.rowid,1,c=>c.text());
 const successMetrics=await r.list("architect_stakeholder_success_metric",hrow.rowid,1,c=>c.text());
 return{...h,role,organization,department,contactEmail,contactPhone,influence,interest,engagement,decisionAuthority,reportingFrequency,availability,representativeOf,delegatedTo,relationshipToClient,stakeholderType,influenceStrategy,expectations,concerns,requirementIds,communicationPreferences,involvementPhases,powerInterestNotes,communicationChannels,successMetrics};
}
