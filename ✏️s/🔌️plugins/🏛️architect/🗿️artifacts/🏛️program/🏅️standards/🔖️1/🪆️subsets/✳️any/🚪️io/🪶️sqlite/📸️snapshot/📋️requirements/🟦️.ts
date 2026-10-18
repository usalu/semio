/** 📋️ Requirement registers of the Architect SQLite projection. */
import type { EntityHeader } from "../../../../🧬️schema/🪪️entity-foundation/🟦️.ts";
import type { AccessibilityRequirement, CommunicationRequirement, CostRequirement, DeliveryConstraint, EnvironmentalRequirement, Equipment, FlexibilityRequirement, GrowthPlan, HumanFactorRequirement, InformationRequirement, InfrastructureRequirement, OperationalRequirement, OrganizationalRequirement, PrivacyRequirement, RegulatoryRequirement, ResilienceRequirement, Resource, SafetyRequirement, ScheduleRequirement, SecurityRequirement, ServiceRequirement, SiteContext, StorageRequirement, SustainabilityRequirement, WayfindingRequirement } from "../../../../🧬️schema/📋️requirements/🟦️.ts";
import { ArtifactSqliteProjection } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import type { SqliteRow } from "@semio-tech/framework/io/sqlite-snapshot";
import { AccessLevel, CostBasis, DeliveryPhase, EnvironmentalParameter, HumanFactorAspect, LifecycleStatus, PrivacyKind, RiskLevel, SafetyDomain, SecurityControlKind, StorageClass, ValidationStatus } from "../🏷️symbols/🟦️.ts";
import { boolean, Cursor, float, note, optional, symbol, text, textField, uint32 } from "../📏️cells/🟦️.ts";
import { ordered, quantity, Reader } from "../💰️backing/🟦️.ts";

export async function projectDeliveryConstraint(p:ArtifactSqliteProjection,id:bigint,v:DeliveryConstraint):Promise<void>{
 await p.insert("architect_delivery_constraint",[...text(v.constraintType),...textField(v.constraintDetails),...symbol(v.phase,DeliveryPhase),...optional(v.hardDeadline,1,text),...optional(v.softDeadline,1,text),...optional(v.workHours,1,text),...optional(v.procurementLeadTime,1,text),...optional(v.ownerId,1,text),...symbol(v.constraintStatus,LifecycleStatus)],id);
 await ordered(p,"architect_delivery_impacted_element",id,v.impactedElementIds,text);
 await ordered(p,"architect_delivery_impacted_requirement",id,v.impactedRequirementIds,text);
 await ordered(p,"architect_delivery_noise_restriction",id,v.noiseRestrictions,text);
 await ordered(p,"architect_delivery_access_restriction",id,v.accessRestrictions,text);
 await ordered(p,"architect_delivery_site_logistics",id,v.siteLogistics,text);
 await ordered(p,"architect_delivery_approval_gate",id,v.approvalGates,text);
 await ordered(p,"architect_delivery_occupancy_constraint",id,v.occupancyConstraints,text);
 await ordered(p,"architect_delivery_weather_window",id,v.weatherWindows,text);
 await ordered(p,"architect_delivery_penalty_clause",id,v.penaltyClauses,text);
 await ordered(p,"architect_delivery_mitigation_option",id,v.mitigationOptions,text);
 await ordered(p,"architect_delivery_risk",id,v.riskIds,text);
}
export async function restoreDeliveryConstraint(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<DeliveryConstraint>{
 const row=await r.body("architect_delivery_constraint",hrow.rowid,11),c=new Cursor(row!);
 const constraintType=c.text();
 const constraintDetails=c.textField();
 const phase=c.symbol(DeliveryPhase);
 const hardDeadline=c.optional(1,()=>c.text());
 const softDeadline=c.optional(1,()=>c.text());
 const workHours=c.optional(1,()=>c.text());
 const procurementLeadTime=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const constraintStatus=c.symbol(LifecycleStatus);
 c.end();
 const impactedElementIds=await r.list("architect_delivery_impacted_element",hrow.rowid,1,c=>c.text());
 const impactedRequirementIds=await r.list("architect_delivery_impacted_requirement",hrow.rowid,1,c=>c.text());
 const noiseRestrictions=await r.list("architect_delivery_noise_restriction",hrow.rowid,1,c=>c.text());
 const accessRestrictions=await r.list("architect_delivery_access_restriction",hrow.rowid,1,c=>c.text());
 const siteLogistics=await r.list("architect_delivery_site_logistics",hrow.rowid,1,c=>c.text());
 const approvalGates=await r.list("architect_delivery_approval_gate",hrow.rowid,1,c=>c.text());
 const occupancyConstraints=await r.list("architect_delivery_occupancy_constraint",hrow.rowid,1,c=>c.text());
 const weatherWindows=await r.list("architect_delivery_weather_window",hrow.rowid,1,c=>c.text());
 const penaltyClauses=await r.list("architect_delivery_penalty_clause",hrow.rowid,1,c=>c.text());
 const mitigationOptions=await r.list("architect_delivery_mitigation_option",hrow.rowid,1,c=>c.text());
 const riskIds=await r.list("architect_delivery_risk",hrow.rowid,1,c=>c.text());
 return{...h,constraintType,constraintDetails,phase,hardDeadline,softDeadline,workHours,procurementLeadTime,ownerId,constraintStatus,impactedElementIds,impactedRequirementIds,noiseRestrictions,accessRestrictions,siteLogistics,approvalGates,occupancyConstraints,weatherWindows,penaltyClauses,mitigationOptions,riskIds};
}
export async function projectSustainabilityRequirement(p:ArtifactSqliteProjection,id:bigint,v:SustainabilityRequirement):Promise<void>{
 await p.insert("architect_sustainability_requirement",[...text(v.topic),...optional(v.target,1,text),...optional(v.metric,1,text),...optional(v.baseline,3,float),...optional(v.targetValue,3,float),...optional(v.unit,1,text),...optional(v.embodiedCarbon,3,float),...optional(v.operationalCarbon,3,float),...optional(v.verificationPlan,1,text),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_sustainability_certification",id,v.certification,text);
 await ordered(p,"architect_sustainability_standard",id,v.standards,text);
 await ordered(p,"architect_sustainability_element",id,v.elementIds,text);
 await ordered(p,"architect_sustainability_strategy",id,v.strategies,text);
 await ordered(p,"architect_sustainability_material_preference",id,v.materialsPreferences,text);
 await ordered(p,"architect_sustainability_energy_strategy",id,v.energyStrategy,text);
 await ordered(p,"architect_sustainability_water_strategy",id,v.waterStrategy,text);
 await ordered(p,"architect_sustainability_waste_strategy",id,v.wasteStrategy,text);
 await ordered(p,"architect_sustainability_biodiversity",id,v.biodiversity,text);
 await ordered(p,"architect_sustainability_reporting_requirement",id,v.reportingRequirements,text);
}
export async function restoreSustainabilityRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<SustainabilityRequirement>{
 const row=await r.body("architect_sustainability_requirement",hrow.rowid,19),c=new Cursor(row!);
 const topic=c.text();
 const target=c.optional(1,()=>c.text());
 const metric=c.optional(1,()=>c.text());
 const baseline=c.optional(3,()=>c.float());
 const targetValue=c.optional(3,()=>c.float());
 const unit=c.optional(1,()=>c.text());
 const embodiedCarbon=c.optional(3,()=>c.float());
 const operationalCarbon=c.optional(3,()=>c.float());
 const verificationPlan=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const certification=await r.list("architect_sustainability_certification",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_sustainability_standard",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_sustainability_element",hrow.rowid,1,c=>c.text());
 const strategies=await r.list("architect_sustainability_strategy",hrow.rowid,1,c=>c.text());
 const materialsPreferences=await r.list("architect_sustainability_material_preference",hrow.rowid,1,c=>c.text());
 const energyStrategy=await r.list("architect_sustainability_energy_strategy",hrow.rowid,1,c=>c.text());
 const waterStrategy=await r.list("architect_sustainability_water_strategy",hrow.rowid,1,c=>c.text());
 const wasteStrategy=await r.list("architect_sustainability_waste_strategy",hrow.rowid,1,c=>c.text());
 const biodiversity=await r.list("architect_sustainability_biodiversity",hrow.rowid,1,c=>c.text());
 const reportingRequirements=await r.list("architect_sustainability_reporting_requirement",hrow.rowid,1,c=>c.text());
 return{...h,topic,target,metric,baseline,targetValue,unit,embodiedCarbon,operationalCarbon,verificationPlan,ownerId,certification,standards,elementIds,strategies,materialsPreferences,energyStrategy,waterStrategy,wasteStrategy,biodiversity,reportingRequirements};
}
export async function projectResilienceRequirement(p:ArtifactSqliteProjection,id:bigint,v:ResilienceRequirement):Promise<void>{
 await p.insert("architect_resilience_requirement",[...text(v.hazard),...symbol(v.riskLevel,RiskLevel),...optional(v.scenario,1,text),...optional(v.recoveryTime,1,text),...optional(v.recoveryPoint,1,text),...optional(v.ownerId,1,text),...optional(v.verificationPlan,1,text)],id);
 await ordered(p,"architect_resilience_redundancy",id,v.redundancy,text);
 await ordered(p,"architect_resilience_hardening_measure",id,v.hardeningMeasures,text);
 await ordered(p,"architect_resilience_backup_system",id,v.backupSystems,text);
 await ordered(p,"architect_resilience_alternate_site",id,v.alternateSites,text);
 await ordered(p,"architect_resilience_supply_chain",id,v.supplyChain,text);
 await ordered(p,"architect_resilience_communication_plan",id,v.communicationPlan,text);
 await ordered(p,"architect_resilience_drill_requirement",id,v.drillRequirements,text);
 await ordered(p,"architect_resilience_element",id,v.elementIds,text);
 await ordered(p,"architect_resilience_infrastructure",id,v.infrastructureIds,text);
 await ordered(p,"architect_resilience_standard",id,v.standards,text);
 await ordered(p,"architect_resilience_insurance_implication",id,v.insuranceImplications,text);
 await ordered(p,"architect_resilience_climate_adaptation",id,v.climateAdaptation,text);
}
export async function restoreResilienceRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ResilienceRequirement>{
 const row=await r.body("architect_resilience_requirement",hrow.rowid,8),c=new Cursor(row!);
 const hazard=c.text();
 const riskLevel=c.symbol(RiskLevel);
 const scenario=c.optional(1,()=>c.text());
 const recoveryTime=c.optional(1,()=>c.text());
 const recoveryPoint=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const verificationPlan=c.optional(1,()=>c.text());
 c.end();
 const redundancy=await r.list("architect_resilience_redundancy",hrow.rowid,1,c=>c.text());
 const hardeningMeasures=await r.list("architect_resilience_hardening_measure",hrow.rowid,1,c=>c.text());
 const backupSystems=await r.list("architect_resilience_backup_system",hrow.rowid,1,c=>c.text());
 const alternateSites=await r.list("architect_resilience_alternate_site",hrow.rowid,1,c=>c.text());
 const supplyChain=await r.list("architect_resilience_supply_chain",hrow.rowid,1,c=>c.text());
 const communicationPlan=await r.list("architect_resilience_communication_plan",hrow.rowid,1,c=>c.text());
 const drillRequirements=await r.list("architect_resilience_drill_requirement",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_resilience_element",hrow.rowid,1,c=>c.text());
 const infrastructureIds=await r.list("architect_resilience_infrastructure",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_resilience_standard",hrow.rowid,1,c=>c.text());
 const insuranceImplications=await r.list("architect_resilience_insurance_implication",hrow.rowid,1,c=>c.text());
 const climateAdaptation=await r.list("architect_resilience_climate_adaptation",hrow.rowid,1,c=>c.text());
 return{...h,hazard,riskLevel,scenario,recoveryTime,recoveryPoint,ownerId,verificationPlan,redundancy,hardeningMeasures,backupSystems,alternateSites,supplyChain,communicationPlan,drillRequirements,elementIds,infrastructureIds,standards,insuranceImplications,climateAdaptation};
}
export async function projectCostRequirement(p:ArtifactSqliteProjection,id:bigint,v:CostRequirement):Promise<void>{
 await p.insert("architect_cost_requirement",[...text(v.costItem),...symbol(v.basis,CostBasis),...optional(v.amount,3,float),...text(v.currency),...optional(v.quantityBasis,1,text),...optional(v.unitCost,3,float),...optional(v.contingencyPercent,3,float),...optional(v.escalationRate,3,float),...optional(v.fundingSource,1,text),...optional(v.phase,1,value=>symbol(value,DeliveryPhase)),...optional(v.benchmarkRef,1,text),...symbol(v.approvalStatus,ValidationStatus),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_cost_element",id,v.elementIds,text);
 await ordered(p,"architect_cost_requirement_ref",id,v.requirementIds,text);
 await ordered(p,"architect_cost_cash_flow",id,v.cashFlowProfile,text);
 await ordered(p,"architect_cost_value_engineering_note",id,v.valueEngineeringNotes,text);
 await ordered(p,"architect_cost_assumption",id,v.assumptions,text);
 await ordered(p,"architect_cost_sensitivity_factor",id,v.sensitivityFactors,text);
}
export async function restoreCostRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<CostRequirement>{
 const row=await r.body("architect_cost_requirement",hrow.rowid,22),c=new Cursor(row!);
 const costItem=c.text();
 const basis=c.symbol(CostBasis);
 const amount=c.optional(3,()=>c.float());
 const currency=c.text();
 const quantityBasis=c.optional(1,()=>c.text());
 const unitCost=c.optional(3,()=>c.float());
 const contingencyPercent=c.optional(3,()=>c.float());
 const escalationRate=c.optional(3,()=>c.float());
 const fundingSource=c.optional(1,()=>c.text());
 const phase=c.optional(1,()=>c.symbol(DeliveryPhase));
 const benchmarkRef=c.optional(1,()=>c.text());
 const approvalStatus=c.symbol(ValidationStatus);
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const elementIds=await r.list("architect_cost_element",hrow.rowid,1,c=>c.text());
 const requirementIds=await r.list("architect_cost_requirement_ref",hrow.rowid,1,c=>c.text());
 const cashFlowProfile=await r.list("architect_cost_cash_flow",hrow.rowid,1,c=>c.text());
 const valueEngineeringNotes=await r.list("architect_cost_value_engineering_note",hrow.rowid,1,c=>c.text());
 const assumptions=await r.list("architect_cost_assumption",hrow.rowid,1,c=>c.text());
 const sensitivityFactors=await r.list("architect_cost_sensitivity_factor",hrow.rowid,1,c=>c.text());
 return{...h,costItem,basis,amount,currency,quantityBasis,unitCost,contingencyPercent,escalationRate,fundingSource,phase,benchmarkRef,approvalStatus,ownerId,elementIds,requirementIds,cashFlowProfile,valueEngineeringNotes,assumptions,sensitivityFactors};
}
export async function projectGrowthPlan(p:ArtifactSqliteProjection,id:bigint,v:GrowthPlan):Promise<void>{
 await p.insert("architect_growth_plan",[...uint32(v.horizonYears),...optional(v.growthRate,3,float),...optional(v.budgetEnvelope,3,float),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_growth_phase",id,v.phases,text);
 await ordered(p,"architect_growth_trigger_event",id,v.triggerEvents,text);
 await ordered(p,"architect_growth_expansion_element",id,v.expansionElementIds,text);
 await ordered(p,"architect_growth_reserve_area",id,v.reserveAreas,text);
 await ordered(p,"architect_growth_infrastructure_headroom",id,v.infrastructureHeadroom,text);
 await ordered(p,"architect_growth_funding_source",id,v.fundingSources,text);
 await ordered(p,"architect_growth_risk_factor",id,v.riskFactors,text);
 await ordered(p,"architect_growth_decision_point",id,v.decisionPoints,text);
 await ordered(p,"architect_growth_scenario",id,v.scenarioIds,text);
 await ordered(p,"architect_growth_decommission_plan",id,v.decommissionPlan,text);
 await ordered(p,"architect_growth_relocation_strategy",id,v.relocationStrategy,text);
 await ordered(p,"architect_growth_stakeholder_impact",id,v.stakeholderImpact,text);
 await ordered(p,"architect_growth_regulatory_consideration",id,v.regulatoryConsiderations,text);
 await quantity(p,id,"headcount_growth","architect_growth_headcount",v.headcountGrowth);
 await quantity(p,id,"area_growth","architect_growth_area",v.areaGrowth);
}
export async function restoreGrowthPlan(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<GrowthPlan>{
 const row=await r.body("architect_growth_plan",hrow.rowid,9),c=new Cursor(row!);
 const horizonYears=c.uint32();
 const growthRate=c.optional(3,()=>c.float());
 const budgetEnvelope=c.optional(3,()=>c.float());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const phases=await r.list("architect_growth_phase",hrow.rowid,1,c=>c.text());
 const triggerEvents=await r.list("architect_growth_trigger_event",hrow.rowid,1,c=>c.text());
 const expansionElementIds=await r.list("architect_growth_expansion_element",hrow.rowid,1,c=>c.text());
 const reserveAreas=await r.list("architect_growth_reserve_area",hrow.rowid,1,c=>c.text());
 const infrastructureHeadroom=await r.list("architect_growth_infrastructure_headroom",hrow.rowid,1,c=>c.text());
 const fundingSources=await r.list("architect_growth_funding_source",hrow.rowid,1,c=>c.text());
 const riskFactors=await r.list("architect_growth_risk_factor",hrow.rowid,1,c=>c.text());
 const decisionPoints=await r.list("architect_growth_decision_point",hrow.rowid,1,c=>c.text());
 const scenarioIds=await r.list("architect_growth_scenario",hrow.rowid,1,c=>c.text());
 const decommissionPlan=await r.list("architect_growth_decommission_plan",hrow.rowid,1,c=>c.text());
 const relocationStrategy=await r.list("architect_growth_relocation_strategy",hrow.rowid,1,c=>c.text());
 const stakeholderImpact=await r.list("architect_growth_stakeholder_impact",hrow.rowid,1,c=>c.text());
 const regulatoryConsiderations=await r.list("architect_growth_regulatory_consideration",hrow.rowid,1,c=>c.text());
 const headcountGrowth=await r.quantity(hrow.rowid,"headcount_growth","architect_growth_headcount");
 const areaGrowth=await r.quantity(hrow.rowid,"area_growth","architect_growth_area");
 return{...h,horizonYears,growthRate,budgetEnvelope,ownerId,phases,triggerEvents,expansionElementIds,reserveAreas,infrastructureHeadroom,fundingSources,riskFactors,decisionPoints,scenarioIds,decommissionPlan,relocationStrategy,stakeholderImpact,regulatoryConsiderations,headcountGrowth,areaGrowth};
}
export async function projectWayfindingRequirement(p:ArtifactSqliteProjection,id:bigint,v:WayfindingRequirement):Promise<void>{
 await p.insert("architect_wayfinding_requirement",[...boolean(v.tactileRequired),...boolean(v.audioRequired),...boolean(v.digitalWayfinding),...optional(v.maximumSignageDistanceM,3,float),...optional(v.maintenancePlan,1,text)],id);
 await ordered(p,"architect_wayfinding_user_profile",id,v.userProfileIds,text);
 await ordered(p,"architect_wayfinding_element",id,v.elementIds,text);
 await ordered(p,"architect_wayfinding_destination_type",id,v.destinationTypes,text);
 await ordered(p,"architect_wayfinding_signage_type",id,v.signageTypes,text);
 await ordered(p,"architect_wayfinding_language",id,v.languages,text);
 await ordered(p,"architect_wayfinding_landmark_strategy",id,v.landmarkStrategy,text);
 await ordered(p,"architect_wayfinding_color_coding",id,v.colorCoding,text);
 await ordered(p,"architect_wayfinding_symbol_standard",id,v.symbolStandards,text);
 await ordered(p,"architect_wayfinding_decision_point",id,v.decisionPoints,text);
 await ordered(p,"architect_wayfinding_lighting_requirement",id,v.lightingRequirements,text);
 await ordered(p,"architect_wayfinding_emergency_egress",id,v.emergencyEgress,text);
 await ordered(p,"architect_wayfinding_visitor_journey",id,v.visitorJourney,text);
 await ordered(p,"architect_wayfinding_staff_journey",id,v.staffJourney,text);
 await ordered(p,"architect_wayfinding_brand_integration",id,v.brandIntegration,text);
}
export async function restoreWayfindingRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<WayfindingRequirement>{
 const row=await r.body("architect_wayfinding_requirement",hrow.rowid,8),c=new Cursor(row!);
 const tactileRequired=c.boolean();
 const audioRequired=c.boolean();
 const digitalWayfinding=c.boolean();
 const maximumSignageDistanceM=c.optional(3,()=>c.float());
 const maintenancePlan=c.optional(1,()=>c.text());
 c.end();
 const userProfileIds=await r.list("architect_wayfinding_user_profile",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_wayfinding_element",hrow.rowid,1,c=>c.text());
 const destinationTypes=await r.list("architect_wayfinding_destination_type",hrow.rowid,1,c=>c.text());
 const signageTypes=await r.list("architect_wayfinding_signage_type",hrow.rowid,1,c=>c.text());
 const languages=await r.list("architect_wayfinding_language",hrow.rowid,1,c=>c.text());
 const landmarkStrategy=await r.list("architect_wayfinding_landmark_strategy",hrow.rowid,1,c=>c.text());
 const colorCoding=await r.list("architect_wayfinding_color_coding",hrow.rowid,1,c=>c.text());
 const symbolStandards=await r.list("architect_wayfinding_symbol_standard",hrow.rowid,1,c=>c.text());
 const decisionPoints=await r.list("architect_wayfinding_decision_point",hrow.rowid,1,c=>c.text());
 const lightingRequirements=await r.list("architect_wayfinding_lighting_requirement",hrow.rowid,1,c=>c.text());
 const emergencyEgress=await r.list("architect_wayfinding_emergency_egress",hrow.rowid,1,c=>c.text());
 const visitorJourney=await r.list("architect_wayfinding_visitor_journey",hrow.rowid,1,c=>c.text());
 const staffJourney=await r.list("architect_wayfinding_staff_journey",hrow.rowid,1,c=>c.text());
 const brandIntegration=await r.list("architect_wayfinding_brand_integration",hrow.rowid,1,c=>c.text());
 return{...h,tactileRequired,audioRequired,digitalWayfinding,maximumSignageDistanceM,maintenancePlan,userProfileIds,elementIds,destinationTypes,signageTypes,languages,landmarkStrategy,colorCoding,symbolStandards,decisionPoints,lightingRequirements,emergencyEgress,visitorJourney,staffJourney,brandIntegration};
}
export async function projectScheduleRequirement(p:ArtifactSqliteProjection,id:bigint,v:ScheduleRequirement):Promise<void>{
 await p.insert("architect_schedule_requirement",[...text(v.milestone),...symbol(v.phase,DeliveryPhase),...optional(v.startDate,1,text),...optional(v.endDate,1,text),...optional(v.duration,1,text),...boolean(v.critical),...optional(v.floatDays,1,uint32),...optional(v.phasingStrategy,1,text),...optional(v.commissioningWindow,1,text),...optional(v.contingencyDays,1,uint32),...optional(v.reportingCadence,1,text),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_schedule_dependency",id,v.dependencies,text);
 await ordered(p,"architect_schedule_predecessor",id,v.predecessors,text);
 await ordered(p,"architect_schedule_successor",id,v.successors,text);
 await ordered(p,"architect_schedule_resource_requirement",id,v.resourceRequirements,text);
 await ordered(p,"architect_schedule_occupancy_impact",id,v.occupancyImpact,text);
 await ordered(p,"architect_schedule_decant_requirement",id,v.decantRequirements,text);
 await ordered(p,"architect_schedule_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_schedule_risk",id,v.riskIds,text);
}
export async function restoreScheduleRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ScheduleRequirement>{
 const row=await r.body("architect_schedule_requirement",hrow.rowid,13),c=new Cursor(row!);
 const milestone=c.text();
 const phase=c.symbol(DeliveryPhase);
 const startDate=c.optional(1,()=>c.text());
 const endDate=c.optional(1,()=>c.text());
 const duration=c.optional(1,()=>c.text());
 const critical=c.boolean();
 const floatDays=c.optional(1,()=>c.uint32());
 const phasingStrategy=c.optional(1,()=>c.text());
 const commissioningWindow=c.optional(1,()=>c.text());
 const contingencyDays=c.optional(1,()=>c.uint32());
 const reportingCadence=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const dependencies=await r.list("architect_schedule_dependency",hrow.rowid,1,c=>c.text());
 const predecessors=await r.list("architect_schedule_predecessor",hrow.rowid,1,c=>c.text());
 const successors=await r.list("architect_schedule_successor",hrow.rowid,1,c=>c.text());
 const resourceRequirements=await r.list("architect_schedule_resource_requirement",hrow.rowid,1,c=>c.text());
 const occupancyImpact=await r.list("architect_schedule_occupancy_impact",hrow.rowid,1,c=>c.text());
 const decantRequirements=await r.list("architect_schedule_decant_requirement",hrow.rowid,1,c=>c.text());
 const stakeholderIds=await r.list("architect_schedule_stakeholder",hrow.rowid,1,c=>c.text());
 const riskIds=await r.list("architect_schedule_risk",hrow.rowid,1,c=>c.text());
 return{...h,milestone,phase,startDate,endDate,duration,critical,floatDays,phasingStrategy,commissioningWindow,contingencyDays,reportingCadence,ownerId,dependencies,predecessors,successors,resourceRequirements,occupancyImpact,decantRequirements,stakeholderIds,riskIds};
}
export async function projectFlexibilityRequirement(p:ArtifactSqliteProjection,id:bigint,v:FlexibilityRequirement):Promise<void>{
 await p.insert("architect_flexibility_requirement",[...text(v.flexibilityType),...optional(v.modularityLevel,1,text),...optional(v.reconfigurationTime,1,text),...optional(v.costOfChange,3,float),...optional(v.technologyReadiness,1,text),...boolean(v.demountablePartitions),...boolean(v.raisedFloor),...boolean(v.overheadServices),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_flexibility_element",id,v.elementIds,text);
 await ordered(p,"architect_flexibility_adaptation_scenario",id,v.adaptationScenarios,text);
 await ordered(p,"architect_flexibility_future_function",id,v.futureFunctionIds,text);
 await ordered(p,"architect_flexibility_expansion_direction",id,v.expansionDirection,text);
 await ordered(p,"architect_flexibility_contraction_scenario",id,v.contractionScenario,text);
 await ordered(p,"architect_flexibility_multi_use_potential",id,v.multiUsePotential,text);
 await ordered(p,"architect_flexibility_furniture_strategy",id,v.furnitureStrategy,text);
 await ordered(p,"architect_flexibility_spare_capacity",id,v.infrastructureSpareCapacity,text);
 await ordered(p,"architect_flexibility_lease_implication",id,v.leaseImplications,text);
}
export async function restoreFlexibilityRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<FlexibilityRequirement>{
 const row=await r.body("architect_flexibility_requirement",hrow.rowid,12),c=new Cursor(row!);
 const flexibilityType=c.text();
 const modularityLevel=c.optional(1,()=>c.text());
 const reconfigurationTime=c.optional(1,()=>c.text());
 const costOfChange=c.optional(3,()=>c.float());
 const technologyReadiness=c.optional(1,()=>c.text());
 const demountablePartitions=c.boolean();
 const raisedFloor=c.boolean();
 const overheadServices=c.boolean();
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const elementIds=await r.list("architect_flexibility_element",hrow.rowid,1,c=>c.text());
 const adaptationScenarios=await r.list("architect_flexibility_adaptation_scenario",hrow.rowid,1,c=>c.text());
 const futureFunctionIds=await r.list("architect_flexibility_future_function",hrow.rowid,1,c=>c.text());
 const expansionDirection=await r.list("architect_flexibility_expansion_direction",hrow.rowid,1,c=>c.text());
 const contractionScenario=await r.list("architect_flexibility_contraction_scenario",hrow.rowid,1,c=>c.text());
 const multiUsePotential=await r.list("architect_flexibility_multi_use_potential",hrow.rowid,1,c=>c.text());
 const furnitureStrategy=await r.list("architect_flexibility_furniture_strategy",hrow.rowid,1,c=>c.text());
 const infrastructureSpareCapacity=await r.list("architect_flexibility_spare_capacity",hrow.rowid,1,c=>c.text());
 const leaseImplications=await r.list("architect_flexibility_lease_implication",hrow.rowid,1,c=>c.text());
 return{...h,flexibilityType,modularityLevel,reconfigurationTime,costOfChange,technologyReadiness,demountablePartitions,raisedFloor,overheadServices,ownerId,elementIds,adaptationScenarios,futureFunctionIds,expansionDirection,contractionScenario,multiUsePotential,furnitureStrategy,infrastructureSpareCapacity,leaseImplications};
}
export async function projectInfrastructureRequirement(p:ArtifactSqliteProjection,id:bigint,v:InfrastructureRequirement):Promise<void>{
 await p.insert("architect_infrastructure_requirement",[...text(v.system),...text(v.category),...optional(v.redundancy,1,text),...optional(v.utilitySource,1,text),...boolean(v.standbyPower),...optional(v.peakDemand,3,float),...optional(v.diversityFactor,3,float),...optional(v.lifecycleCost,3,float),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_infrastructure_distribution",id,v.distribution,text);
 await ordered(p,"architect_infrastructure_entry_point",id,v.entryPoints,text);
 await ordered(p,"architect_infrastructure_monitoring",id,v.monitoring,text);
 await ordered(p,"architect_infrastructure_maintenance_access",id,v.maintenanceAccess,text);
 await ordered(p,"architect_infrastructure_standard",id,v.standards,text);
 await ordered(p,"architect_infrastructure_element",id,v.elementIds,text);
 await ordered(p,"architect_infrastructure_future_expansion",id,v.futureExpansion,text);
 await ordered(p,"architect_infrastructure_interface_requirement",id,v.interfaceRequirements,text);
 await ordered(p,"architect_infrastructure_commissioning",id,v.commissioning,text);
 await quantity(p,id,"capacity","architect_infrastructure_capacity",v.capacity);
}
export async function restoreInfrastructureRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<InfrastructureRequirement>{
 const row=await r.body("architect_infrastructure_requirement",hrow.rowid,16),c=new Cursor(row!);
 const system=c.text();
 const category=c.text();
 const redundancy=c.optional(1,()=>c.text());
 const utilitySource=c.optional(1,()=>c.text());
 const standbyPower=c.boolean();
 const peakDemand=c.optional(3,()=>c.float());
 const diversityFactor=c.optional(3,()=>c.float());
 const lifecycleCost=c.optional(3,()=>c.float());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const distribution=await r.list("architect_infrastructure_distribution",hrow.rowid,1,c=>c.text());
 const entryPoints=await r.list("architect_infrastructure_entry_point",hrow.rowid,1,c=>c.text());
 const monitoring=await r.list("architect_infrastructure_monitoring",hrow.rowid,1,c=>c.text());
 const maintenanceAccess=await r.list("architect_infrastructure_maintenance_access",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_infrastructure_standard",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_infrastructure_element",hrow.rowid,1,c=>c.text());
 const futureExpansion=await r.list("architect_infrastructure_future_expansion",hrow.rowid,1,c=>c.text());
 const interfaceRequirements=await r.list("architect_infrastructure_interface_requirement",hrow.rowid,1,c=>c.text());
 const commissioning=await r.list("architect_infrastructure_commissioning",hrow.rowid,1,c=>c.text());
 const capacity=await r.quantity(hrow.rowid,"capacity","architect_infrastructure_capacity");
 return{...h,system,category,redundancy,utilitySource,standbyPower,peakDemand,diversityFactor,lifecycleCost,ownerId,distribution,entryPoints,monitoring,maintenanceAccess,standards,elementIds,futureExpansion,interfaceRequirements,commissioning,capacity};
}
export async function projectInformationRequirement(p:ArtifactSqliteProjection,id:bigint,v:InformationRequirement):Promise<void>{
 await p.insert("architect_information_requirement",[...text(v.informationType),...optional(v.format,1,text),...optional(v.sourceSystem,1,text),...optional(v.updateFrequency,1,text),...optional(v.retentionPeriod,1,text),...optional(v.classification,1,text),...boolean(v.auditTrail),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_information_destination_system",id,v.destinationSystems,text);
 await ordered(p,"architect_information_access_control",id,v.accessControls,text);
 await ordered(p,"architect_information_quality_criterion",id,v.qualityCriteria,text);
 await ordered(p,"architect_information_metadata_requirement",id,v.metadataRequirements,text);
 await ordered(p,"architect_information_integration_point",id,v.integrationPoints,text);
 await ordered(p,"architect_information_backup_requirement",id,v.backupRequirements,text);
 await ordered(p,"architect_information_disaster_recovery",id,v.disasterRecovery,text);
 await ordered(p,"architect_information_privacy_control",id,v.privacyControls,text);
 await ordered(p,"architect_information_element",id,v.elementIds,text);
 await ordered(p,"architect_information_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_information_standard",id,v.standards,text);
}
export async function restoreInformationRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<InformationRequirement>{
 const row=await r.body("architect_information_requirement",hrow.rowid,9),c=new Cursor(row!);
 const informationType=c.text();
 const format=c.optional(1,()=>c.text());
 const sourceSystem=c.optional(1,()=>c.text());
 const updateFrequency=c.optional(1,()=>c.text());
 const retentionPeriod=c.optional(1,()=>c.text());
 const classification=c.optional(1,()=>c.text());
 const auditTrail=c.boolean();
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const destinationSystems=await r.list("architect_information_destination_system",hrow.rowid,1,c=>c.text());
 const accessControls=await r.list("architect_information_access_control",hrow.rowid,1,c=>c.text());
 const qualityCriteria=await r.list("architect_information_quality_criterion",hrow.rowid,1,c=>c.text());
 const metadataRequirements=await r.list("architect_information_metadata_requirement",hrow.rowid,1,c=>c.text());
 const integrationPoints=await r.list("architect_information_integration_point",hrow.rowid,1,c=>c.text());
 const backupRequirements=await r.list("architect_information_backup_requirement",hrow.rowid,1,c=>c.text());
 const disasterRecovery=await r.list("architect_information_disaster_recovery",hrow.rowid,1,c=>c.text());
 const privacyControls=await r.list("architect_information_privacy_control",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_information_element",hrow.rowid,1,c=>c.text());
 const stakeholderIds=await r.list("architect_information_stakeholder",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_information_standard",hrow.rowid,1,c=>c.text());
 return{...h,informationType,format,sourceSystem,updateFrequency,retentionPeriod,classification,auditTrail,ownerId,destinationSystems,accessControls,qualityCriteria,metadataRequirements,integrationPoints,backupRequirements,disasterRecovery,privacyControls,elementIds,stakeholderIds,standards};
}
export async function projectCommunicationRequirement(p:ArtifactSqliteProjection,id:bigint,v:CommunicationRequirement):Promise<void>{
 await p.insert("architect_communication_requirement",[...text(v.channel),...optional(v.frequency,1,text),...boolean(v.emergencyUse),...boolean(v.twoWay),...optional(v.recordingPolicy,1,text),...boolean(v.feedbackLoop),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_communication_audience",id,v.audienceIds,text);
 await ordered(p,"architect_communication_message_type",id,v.messageTypes,text);
 await ordered(p,"architect_communication_medium",id,v.medium,text);
 await ordered(p,"architect_communication_language",id,v.language,text);
 await ordered(p,"architect_communication_accessibility",id,v.accessibility,text);
 await ordered(p,"architect_communication_signage_location",id,v.signageLocations,text);
 await ordered(p,"architect_communication_technology",id,v.technology,text);
 await ordered(p,"architect_communication_escalation_path",id,v.escalationPath,text);
 await ordered(p,"architect_communication_privacy_control",id,v.privacyControls,text);
 await ordered(p,"architect_communication_element",id,v.elementIds,text);
 await ordered(p,"architect_communication_standard",id,v.standards,text);
 await ordered(p,"architect_communication_template",id,v.templates,text);
}
export async function restoreCommunicationRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<CommunicationRequirement>{
 const row=await r.body("architect_communication_requirement",hrow.rowid,8),c=new Cursor(row!);
 const channel=c.text();
 const frequency=c.optional(1,()=>c.text());
 const emergencyUse=c.boolean();
 const twoWay=c.boolean();
 const recordingPolicy=c.optional(1,()=>c.text());
 const feedbackLoop=c.boolean();
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const audienceIds=await r.list("architect_communication_audience",hrow.rowid,1,c=>c.text());
 const messageTypes=await r.list("architect_communication_message_type",hrow.rowid,1,c=>c.text());
 const medium=await r.list("architect_communication_medium",hrow.rowid,1,c=>c.text());
 const language=await r.list("architect_communication_language",hrow.rowid,1,c=>c.text());
 const accessibility=await r.list("architect_communication_accessibility",hrow.rowid,1,c=>c.text());
 const signageLocations=await r.list("architect_communication_signage_location",hrow.rowid,1,c=>c.text());
 const technology=await r.list("architect_communication_technology",hrow.rowid,1,c=>c.text());
 const escalationPath=await r.list("architect_communication_escalation_path",hrow.rowid,1,c=>c.text());
 const privacyControls=await r.list("architect_communication_privacy_control",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_communication_element",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_communication_standard",hrow.rowid,1,c=>c.text());
 const templates=await r.list("architect_communication_template",hrow.rowid,1,c=>c.text());
 return{...h,channel,frequency,emergencyUse,twoWay,recordingPolicy,feedbackLoop,ownerId,audienceIds,messageTypes,medium,language,accessibility,signageLocations,technology,escalationPath,privacyControls,elementIds,standards,templates};
}
export async function projectSiteContext(p:ArtifactSqliteProjection,id:bigint,v:SiteContext):Promise<void>{
 await p.insert("architect_site_context",[...text(v.siteName),...optional(v.address,1,text),...optional(v.latitude,3,float),...optional(v.longitude,3,float),...optional(v.elevationM,3,float),...optional(v.climateZone,1,text),...optional(v.seismicZone,1,text),...optional(v.floodRisk,1,text),...optional(v.zoning,1,text),...optional(v.maxHeightM,3,float),...optional(v.maxCoverage,3,float)],id);
 await ordered(p,"architect_site_soil_condition",id,v.soilConditions,text);
 await ordered(p,"architect_site_utility",id,v.utilitiesAvailable,text);
 await ordered(p,"architect_site_access_road",id,v.accessRoads,text);
 await ordered(p,"architect_site_public_transit",id,v.publicTransit,text);
 await ordered(p,"architect_site_neighbor",id,v.neighbors,text);
 await ordered(p,"architect_site_view",id,v.views,text);
 await ordered(p,"architect_site_noise_source",id,v.noiseSources,text);
 await ordered(p,"architect_site_environmental_constraint",id,v.environmentalConstraints,text);
 await ordered(p,"architect_site_heritage_constraint",id,v.heritageConstraints,text);
}
export async function restoreSiteContext(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<SiteContext>{
 const row=await r.body("architect_site_context",hrow.rowid,22),c=new Cursor(row!);
 const siteName=c.text();
 const address=c.optional(1,()=>c.text());
 const latitude=c.optional(3,()=>c.float());
 const longitude=c.optional(3,()=>c.float());
 const elevationM=c.optional(3,()=>c.float());
 const climateZone=c.optional(1,()=>c.text());
 const seismicZone=c.optional(1,()=>c.text());
 const floodRisk=c.optional(1,()=>c.text());
 const zoning=c.optional(1,()=>c.text());
 const maxHeightM=c.optional(3,()=>c.float());
 const maxCoverage=c.optional(3,()=>c.float());
 c.end();
 const soilConditions=await r.list("architect_site_soil_condition",hrow.rowid,1,c=>c.text());
 const utilitiesAvailable=await r.list("architect_site_utility",hrow.rowid,1,c=>c.text());
 const accessRoads=await r.list("architect_site_access_road",hrow.rowid,1,c=>c.text());
 const publicTransit=await r.list("architect_site_public_transit",hrow.rowid,1,c=>c.text());
 const neighbors=await r.list("architect_site_neighbor",hrow.rowid,1,c=>c.text());
 const views=await r.list("architect_site_view",hrow.rowid,1,c=>c.text());
 const noiseSources=await r.list("architect_site_noise_source",hrow.rowid,1,c=>c.text());
 const environmentalConstraints=await r.list("architect_site_environmental_constraint",hrow.rowid,1,c=>c.text());
 const heritageConstraints=await r.list("architect_site_heritage_constraint",hrow.rowid,1,c=>c.text());
 return{...h,siteName,address,latitude,longitude,elevationM,climateZone,seismicZone,floodRisk,zoning,maxHeightM,maxCoverage,soilConditions,utilitiesAvailable,accessRoads,publicTransit,neighbors,views,noiseSources,environmentalConstraints,heritageConstraints};
}
export async function projectOrganizationalRequirement(p:ArtifactSqliteProjection,id:bigint,v:OrganizationalRequirement):Promise<void>{
 await p.insert("architect_organizational_requirement",[...text(v.department),...optional(v.reportingLine,1,text),...optional(v.growthPlanId,1,text),...optional(v.collaborationModel,1,text),...optional(v.changeReadiness,1,text),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_organization_work_pattern",id,v.workPatterns,text);
 await ordered(p,"architect_organization_hierarchy_level",id,v.hierarchyLevels,text);
 await ordered(p,"architect_organization_decision_making",id,v.decisionMaking,text);
 await ordered(p,"architect_organization_culture_note",id,v.cultureNotes,text);
 await ordered(p,"architect_organization_union_consideration",id,v.unionConsiderations,text);
 await ordered(p,"architect_organization_training_need",id,v.trainingNeeds,text);
 await ordered(p,"architect_organization_element",id,v.elementIds,text);
 await ordered(p,"architect_organization_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_organization_service_requirement",id,v.serviceRequirementIds,text);
 await ordered(p,"architect_organization_branding_requirement",id,v.brandingRequirements,text);
 await ordered(p,"architect_organization_wellness_plugin",id,v.wellnessPlugins,text);
 await ordered(p,"architect_organization_diversity_goal",id,v.diversityGoals,text);
 await quantity(p,id,"headcount","architect_organization_headcount",v.headcount);
}
export async function restoreOrganizationalRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<OrganizationalRequirement>{
 const row=await r.body("architect_organizational_requirement",hrow.rowid,7),c=new Cursor(row!);
 const department=c.text();
 const reportingLine=c.optional(1,()=>c.text());
 const growthPlanId=c.optional(1,()=>c.text());
 const collaborationModel=c.optional(1,()=>c.text());
 const changeReadiness=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const workPatterns=await r.list("architect_organization_work_pattern",hrow.rowid,1,c=>c.text());
 const hierarchyLevels=await r.list("architect_organization_hierarchy_level",hrow.rowid,1,c=>c.text());
 const decisionMaking=await r.list("architect_organization_decision_making",hrow.rowid,1,c=>c.text());
 const cultureNotes=await r.list("architect_organization_culture_note",hrow.rowid,1,c=>c.text());
 const unionConsiderations=await r.list("architect_organization_union_consideration",hrow.rowid,1,c=>c.text());
 const trainingNeeds=await r.list("architect_organization_training_need",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_organization_element",hrow.rowid,1,c=>c.text());
 const stakeholderIds=await r.list("architect_organization_stakeholder",hrow.rowid,1,c=>c.text());
 const serviceRequirementIds=await r.list("architect_organization_service_requirement",hrow.rowid,1,c=>c.text());
 const brandingRequirements=await r.list("architect_organization_branding_requirement",hrow.rowid,1,c=>c.text());
 const wellnessPlugins=await r.list("architect_organization_wellness_plugin",hrow.rowid,1,c=>c.text());
 const diversityGoals=await r.list("architect_organization_diversity_goal",hrow.rowid,1,c=>c.text());
 const headcount=await r.quantity(hrow.rowid,"headcount","architect_organization_headcount");
 return{...h,department,reportingLine,growthPlanId,collaborationModel,changeReadiness,ownerId,workPatterns,hierarchyLevels,decisionMaking,cultureNotes,unionConsiderations,trainingNeeds,elementIds,stakeholderIds,serviceRequirementIds,brandingRequirements,wellnessPlugins,diversityGoals,headcount};
}
export async function projectServiceRequirement(p:ArtifactSqliteProjection,id:bigint,v:ServiceRequirement):Promise<void>{
 await p.insert("architect_service_requirement",[...text(v.serviceName),...text(v.serviceType),...optional(v.provider,1,text),...optional(v.serviceLevel,1,text),...optional(v.operatingHours,1,text),...optional(v.responseTime,1,text),...optional(v.costModel,1,text),...optional(v.failureImpact,1,text)],id);
 await ordered(p,"architect_service_queue_management",id,v.queueManagement,text);
 await ordered(p,"architect_service_customer_profile",id,v.customerProfiles,text);
 await ordered(p,"architect_service_element",id,v.elementIds,text);
 await ordered(p,"architect_service_equipment",id,v.equipmentIds,text);
 await ordered(p,"architect_service_quality_metric",id,v.qualityMetrics,text);
 await ordered(p,"architect_service_contract_reference",id,v.contractRefs,text);
 await ordered(p,"architect_service_dependency",id,v.dependencies,text);
 await ordered(p,"architect_service_backup",id,v.backupService,text);
 await ordered(p,"architect_service_feedback_channel",id,v.feedbackChannels,text);
 await quantity(p,id,"capacity","architect_service_capacity",v.capacity);
 await quantity(p,id,"staffing","architect_service_staffing",v.staffing);
}
export async function restoreServiceRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<ServiceRequirement>{
 const row=await r.body("architect_service_requirement",hrow.rowid,9),c=new Cursor(row!);
 const serviceName=c.text();
 const serviceType=c.text();
 const provider=c.optional(1,()=>c.text());
 const serviceLevel=c.optional(1,()=>c.text());
 const operatingHours=c.optional(1,()=>c.text());
 const responseTime=c.optional(1,()=>c.text());
 const costModel=c.optional(1,()=>c.text());
 const failureImpact=c.optional(1,()=>c.text());
 c.end();
 const queueManagement=await r.list("architect_service_queue_management",hrow.rowid,1,c=>c.text());
 const customerProfiles=await r.list("architect_service_customer_profile",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_service_element",hrow.rowid,1,c=>c.text());
 const equipmentIds=await r.list("architect_service_equipment",hrow.rowid,1,c=>c.text());
 const qualityMetrics=await r.list("architect_service_quality_metric",hrow.rowid,1,c=>c.text());
 const contractRefs=await r.list("architect_service_contract_reference",hrow.rowid,1,c=>c.text());
 const dependencies=await r.list("architect_service_dependency",hrow.rowid,1,c=>c.text());
 const backupService=await r.list("architect_service_backup",hrow.rowid,1,c=>c.text());
 const feedbackChannels=await r.list("architect_service_feedback_channel",hrow.rowid,1,c=>c.text());
 const capacity=await r.quantity(hrow.rowid,"capacity","architect_service_capacity");
 const staffing=await r.quantity(hrow.rowid,"staffing","architect_service_staffing");
 return{...h,serviceName,serviceType,provider,serviceLevel,operatingHours,responseTime,costModel,failureImpact,queueManagement,customerProfiles,elementIds,equipmentIds,qualityMetrics,contractRefs,dependencies,backupService,feedbackChannels,capacity,staffing};
}
export async function projectSafetyRequirement(p:ArtifactSqliteProjection,id:bigint,v:SafetyRequirement):Promise<void>{
 await p.insert("architect_safety_requirement",[...symbol(v.safetyDomain,SafetyDomain),...text(v.hazard),...symbol(v.riskLevel,RiskLevel),...optional(v.inspectionFrequency,1,text),...optional(v.residualRisk,1,text)],id);
 await ordered(p,"architect_safety_element",id,v.affectedElementIds,text);
 await ordered(p,"architect_safety_user",id,v.affectedUserIds,text);
 await ordered(p,"architect_safety_mitigation_measure",id,v.mitigationMeasures,text);
 await ordered(p,"architect_safety_ppe_requirement",id,v.ppeRequirements,text);
 await ordered(p,"architect_safety_emergency_procedure",id,v.emergencyProcedures,text);
 await ordered(p,"architect_safety_evacuation_requirement",id,v.evacuationRequirements,text);
 await ordered(p,"architect_safety_fire_protection",id,v.fireProtection,text);
 await ordered(p,"architect_safety_structural_safety",id,v.structuralSafety,text);
 await ordered(p,"architect_safety_slip_trip_fall",id,v.slipTripFall,text);
 await ordered(p,"architect_safety_chemical_safety",id,v.chemicalSafety,text);
 await ordered(p,"architect_safety_electrical_safety",id,v.electricalSafety,text);
 await ordered(p,"architect_safety_machinery_safety",id,v.machinerySafety,text);
 await ordered(p,"architect_safety_standard",id,v.standards,text);
 await ordered(p,"architect_safety_training_requirement",id,v.trainingRequirements,text);
 await ordered(p,"architect_safety_incident_reporting",id,v.incidentReporting,text);
}
export async function restoreSafetyRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<SafetyRequirement>{
 const row=await r.body("architect_safety_requirement",hrow.rowid,6),c=new Cursor(row!);
 const safetyDomain=c.symbol(SafetyDomain);
 const hazard=c.text();
 const riskLevel=c.symbol(RiskLevel);
 const inspectionFrequency=c.optional(1,()=>c.text());
 const residualRisk=c.optional(1,()=>c.text());
 c.end();
 const affectedElementIds=await r.list("architect_safety_element",hrow.rowid,1,c=>c.text());
 const affectedUserIds=await r.list("architect_safety_user",hrow.rowid,1,c=>c.text());
 const mitigationMeasures=await r.list("architect_safety_mitigation_measure",hrow.rowid,1,c=>c.text());
 const ppeRequirements=await r.list("architect_safety_ppe_requirement",hrow.rowid,1,c=>c.text());
 const emergencyProcedures=await r.list("architect_safety_emergency_procedure",hrow.rowid,1,c=>c.text());
 const evacuationRequirements=await r.list("architect_safety_evacuation_requirement",hrow.rowid,1,c=>c.text());
 const fireProtection=await r.list("architect_safety_fire_protection",hrow.rowid,1,c=>c.text());
 const structuralSafety=await r.list("architect_safety_structural_safety",hrow.rowid,1,c=>c.text());
 const slipTripFall=await r.list("architect_safety_slip_trip_fall",hrow.rowid,1,c=>c.text());
 const chemicalSafety=await r.list("architect_safety_chemical_safety",hrow.rowid,1,c=>c.text());
 const electricalSafety=await r.list("architect_safety_electrical_safety",hrow.rowid,1,c=>c.text());
 const machinerySafety=await r.list("architect_safety_machinery_safety",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_safety_standard",hrow.rowid,1,c=>c.text());
 const trainingRequirements=await r.list("architect_safety_training_requirement",hrow.rowid,1,c=>c.text());
 const incidentReporting=await r.list("architect_safety_incident_reporting",hrow.rowid,1,c=>c.text());
 return{...h,safetyDomain,hazard,riskLevel,inspectionFrequency,residualRisk,affectedElementIds,affectedUserIds,mitigationMeasures,ppeRequirements,emergencyProcedures,evacuationRequirements,fireProtection,structuralSafety,slipTripFall,chemicalSafety,electricalSafety,machinerySafety,standards,trainingRequirements,incidentReporting};
}
export async function projectSecurityRequirement(p:ArtifactSqliteProjection,id:bigint,v:SecurityRequirement):Promise<void>{
 await p.insert("architect_security_requirement",[...symbol(v.controlKind,SecurityControlKind),...text(v.threat),...symbol(v.riskLevel,RiskLevel),...symbol(v.accessLevel,AccessLevel),...optional(v.drillFrequency,1,text),...optional(v.classifiedLevel,1,text)],id);
 await ordered(p,"architect_security_asset",id,v.assetIds,text);
 await ordered(p,"architect_security_zone",id,v.zoneIds,text);
 await ordered(p,"architect_security_perimeter_control",id,v.perimeterControls,text);
 await ordered(p,"architect_security_surveillance",id,v.surveillance,text);
 await ordered(p,"architect_security_intrusion_detection",id,v.intrusionDetection,text);
 await ordered(p,"architect_security_cybersecurity",id,v.cybersecurity,text);
 await ordered(p,"architect_security_screening",id,v.screening,text);
 await ordered(p,"architect_security_visitor_management",id,v.visitorManagement,text);
 await ordered(p,"architect_security_key_management",id,v.keyManagement,text);
 await ordered(p,"architect_security_standard",id,v.standards,text);
 await ordered(p,"architect_security_response_procedure",id,v.responseProcedures,text);
 await ordered(p,"architect_security_liaison_contact",id,v.liaisonContacts,text);
 await ordered(p,"architect_security_redundancy",id,v.redundancy,text);
 await ordered(p,"architect_security_audit_requirement",id,v.auditRequirements,text);
}
export async function restoreSecurityRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<SecurityRequirement>{
 const row=await r.body("architect_security_requirement",hrow.rowid,7),c=new Cursor(row!);
 const controlKind=c.symbol(SecurityControlKind);
 const threat=c.text();
 const riskLevel=c.symbol(RiskLevel);
 const accessLevel=c.symbol(AccessLevel);
 const drillFrequency=c.optional(1,()=>c.text());
 const classifiedLevel=c.optional(1,()=>c.text());
 c.end();
 const assetIds=await r.list("architect_security_asset",hrow.rowid,1,c=>c.text());
 const zoneIds=await r.list("architect_security_zone",hrow.rowid,1,c=>c.text());
 const perimeterControls=await r.list("architect_security_perimeter_control",hrow.rowid,1,c=>c.text());
 const surveillance=await r.list("architect_security_surveillance",hrow.rowid,1,c=>c.text());
 const intrusionDetection=await r.list("architect_security_intrusion_detection",hrow.rowid,1,c=>c.text());
 const cybersecurity=await r.list("architect_security_cybersecurity",hrow.rowid,1,c=>c.text());
 const screening=await r.list("architect_security_screening",hrow.rowid,1,c=>c.text());
 const visitorManagement=await r.list("architect_security_visitor_management",hrow.rowid,1,c=>c.text());
 const keyManagement=await r.list("architect_security_key_management",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_security_standard",hrow.rowid,1,c=>c.text());
 const responseProcedures=await r.list("architect_security_response_procedure",hrow.rowid,1,c=>c.text());
 const liaisonContacts=await r.list("architect_security_liaison_contact",hrow.rowid,1,c=>c.text());
 const redundancy=await r.list("architect_security_redundancy",hrow.rowid,1,c=>c.text());
 const auditRequirements=await r.list("architect_security_audit_requirement",hrow.rowid,1,c=>c.text());
 return{...h,controlKind,threat,riskLevel,accessLevel,drillFrequency,classifiedLevel,assetIds,zoneIds,perimeterControls,surveillance,intrusionDetection,cybersecurity,screening,visitorManagement,keyManagement,standards,responseProcedures,liaisonContacts,redundancy,auditRequirements};
}
export async function projectRegulatoryRequirement(p:ArtifactSqliteProjection,id:bigint,v:RegulatoryRequirement):Promise<void>{
 await p.insert("architect_regulatory_requirement",[...text(v.jurisdiction),...text(v.code),...optional(v.clause,1,text),...text(v.title),...textField(v.requirementText),...optional(v.complianceMethod,1,text),...optional(v.authority,1,text),...optional(v.effectiveDate,1,text),...optional(v.expiryDate,1,text),...symbol(v.verificationStatus,ValidationStatus),...optional(v.updateSource,1,text)],id);
 await ordered(p,"architect_regulatory_applicability",id,v.applicability,text);
 await ordered(p,"architect_regulatory_element",id,v.elementIds,text);
 await ordered(p,"architect_regulatory_required_evidence",id,v.evidenceRequired,text);
 await ordered(p,"architect_regulatory_penalty",id,v.penalties,text);
 await ordered(p,"architect_regulatory_exemption",id,v.exemptions,text);
 await ordered(p,"architect_regulatory_related_requirement",id,v.relatedRequirementIds,text);
 await ordered(p,"architect_regulatory_interpretation_note",id,v.interpretationNotes,note);
 await ordered(p,"architect_regulatory_consultant_ref",id,v.consultantRefs,text);
}
export async function restoreRegulatoryRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<RegulatoryRequirement>{
 const row=await r.body("architect_regulatory_requirement",hrow.rowid,13),c=new Cursor(row!);
 const jurisdiction=c.text();
 const code=c.text();
 const clause=c.optional(1,()=>c.text());
 const title=c.text();
 const requirementText=c.textField();
 const complianceMethod=c.optional(1,()=>c.text());
 const authority=c.optional(1,()=>c.text());
 const effectiveDate=c.optional(1,()=>c.text());
 const expiryDate=c.optional(1,()=>c.text());
 const verificationStatus=c.symbol(ValidationStatus);
 const updateSource=c.optional(1,()=>c.text());
 c.end();
 const applicability=await r.list("architect_regulatory_applicability",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_regulatory_element",hrow.rowid,1,c=>c.text());
 const evidenceRequired=await r.list("architect_regulatory_required_evidence",hrow.rowid,1,c=>c.text());
 const penalties=await r.list("architect_regulatory_penalty",hrow.rowid,1,c=>c.text());
 const exemptions=await r.list("architect_regulatory_exemption",hrow.rowid,1,c=>c.text());
 const relatedRequirementIds=await r.list("architect_regulatory_related_requirement",hrow.rowid,1,c=>c.text());
 const interpretationNotes=await r.list("architect_regulatory_interpretation_note",hrow.rowid,2,c=>c.note());
 const consultantRefs=await r.list("architect_regulatory_consultant_ref",hrow.rowid,1,c=>c.text());
 return{...h,jurisdiction,code,clause,title,requirementText,complianceMethod,authority,effectiveDate,expiryDate,verificationStatus,updateSource,applicability,elementIds,evidenceRequired,penalties,exemptions,relatedRequirementIds,interpretationNotes,consultantRefs};
}
export async function projectAccessibilityRequirement(p:ArtifactSqliteProjection,id:bigint,v:AccessibilityRequirement):Promise<void>{
 await p.insert("architect_accessibility_requirement",[...text(v.standard),...optional(v.level,1,text),...optional(v.clearWidthM,3,float),...optional(v.clearHeightM,3,float),...optional(v.turningCircleM,3,float),...optional(v.rampSlope,3,float),...boolean(v.liftRequired),...boolean(v.tactileGuidance),...boolean(v.hearingLoop),...boolean(v.visualContrast),...optional(v.controlsHeight,1,text),...optional(v.serviceAnimalPolicy,1,text),...boolean(v.companionSeating),...optional(v.verificationPlan,1,text),...optional(v.wcagConformance,1,text)],id);
 await ordered(p,"architect_accessibility_user_profile",id,v.userProfileIds,text);
 await ordered(p,"architect_accessibility_element",id,v.elementIds,text);
 await ordered(p,"architect_accessibility_route",id,v.routeIds,text);
 await ordered(p,"architect_accessibility_signage_requirement",id,v.signageRequirements,text);
 await ordered(p,"architect_accessibility_emergency_evacuation",id,v.emergencyEvacuation,text);
 await ordered(p,"architect_accessibility_exception",id,v.exceptions,text);
 await ordered(p,"architect_accessibility_universal_design_principle",id,v.universalDesignPrinciples,text);
}
export async function restoreAccessibilityRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<AccessibilityRequirement>{
 const row=await r.body("architect_accessibility_requirement",hrow.rowid,24),c=new Cursor(row!);
 const standard=c.text();
 const level=c.optional(1,()=>c.text());
 const clearWidthM=c.optional(3,()=>c.float());
 const clearHeightM=c.optional(3,()=>c.float());
 const turningCircleM=c.optional(3,()=>c.float());
 const rampSlope=c.optional(3,()=>c.float());
 const liftRequired=c.boolean();
 const tactileGuidance=c.boolean();
 const hearingLoop=c.boolean();
 const visualContrast=c.boolean();
 const controlsHeight=c.optional(1,()=>c.text());
 const serviceAnimalPolicy=c.optional(1,()=>c.text());
 const companionSeating=c.boolean();
 const verificationPlan=c.optional(1,()=>c.text());
 const wcagConformance=c.optional(1,()=>c.text());
 c.end();
 const userProfileIds=await r.list("architect_accessibility_user_profile",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_accessibility_element",hrow.rowid,1,c=>c.text());
 const routeIds=await r.list("architect_accessibility_route",hrow.rowid,1,c=>c.text());
 const signageRequirements=await r.list("architect_accessibility_signage_requirement",hrow.rowid,1,c=>c.text());
 const emergencyEvacuation=await r.list("architect_accessibility_emergency_evacuation",hrow.rowid,1,c=>c.text());
 const exceptions=await r.list("architect_accessibility_exception",hrow.rowid,1,c=>c.text());
 const universalDesignPrinciples=await r.list("architect_accessibility_universal_design_principle",hrow.rowid,1,c=>c.text());
 return{...h,standard,level,clearWidthM,clearHeightM,turningCircleM,rampSlope,liftRequired,tactileGuidance,hearingLoop,visualContrast,controlsHeight,serviceAnimalPolicy,companionSeating,verificationPlan,wcagConformance,userProfileIds,elementIds,routeIds,signageRequirements,emergencyEvacuation,exceptions,universalDesignPrinciples};
}
export async function projectPrivacyRequirement(p:ArtifactSqliteProjection,id:bigint,v:PrivacyRequirement):Promise<void>{
 await p.insert("architect_privacy_requirement",[...symbol(v.privacyKind,PrivacyKind),...text(v.privacyType),...optional(v.level,1,text),...boolean(v.screeningRequired),...boolean(v.enclosureRequired),...optional(v.observationRisk,1,text),...optional(v.retentionPolicy,1,text),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_privacy_subject",id,v.subjectIds,text);
 await ordered(p,"architect_privacy_element",id,v.elementIds,text);
 await ordered(p,"architect_privacy_visual",id,v.visualPrivacy,text);
 await ordered(p,"architect_privacy_acoustic",id,v.acousticPrivacy,text);
 await ordered(p,"architect_privacy_data",id,v.dataPrivacy,text);
 await ordered(p,"architect_privacy_access_restriction",id,v.accessRestrictions,text);
 await ordered(p,"architect_privacy_regulatory_basis",id,v.regulatoryBasis,text);
 await ordered(p,"architect_privacy_cultural_consideration",id,v.culturalConsiderations,text);
 await ordered(p,"architect_privacy_technology_control",id,v.technologyControls,text);
 await ordered(p,"architect_privacy_signage",id,v.signage,text);
 await ordered(p,"architect_privacy_monitoring_restriction",id,v.monitoringRestrictions,text);
 await ordered(p,"architect_privacy_breach_response",id,v.breachResponse,text);
}
export async function restorePrivacyRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<PrivacyRequirement>{
 const row=await r.body("architect_privacy_requirement",hrow.rowid,9),c=new Cursor(row!);
 const privacyKind=c.symbol(PrivacyKind);
 const privacyType=c.text();
 const level=c.optional(1,()=>c.text());
 const screeningRequired=c.boolean();
 const enclosureRequired=c.boolean();
 const observationRisk=c.optional(1,()=>c.text());
 const retentionPolicy=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const subjectIds=await r.list("architect_privacy_subject",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_privacy_element",hrow.rowid,1,c=>c.text());
 const visualPrivacy=await r.list("architect_privacy_visual",hrow.rowid,1,c=>c.text());
 const acousticPrivacy=await r.list("architect_privacy_acoustic",hrow.rowid,1,c=>c.text());
 const dataPrivacy=await r.list("architect_privacy_data",hrow.rowid,1,c=>c.text());
 const accessRestrictions=await r.list("architect_privacy_access_restriction",hrow.rowid,1,c=>c.text());
 const regulatoryBasis=await r.list("architect_privacy_regulatory_basis",hrow.rowid,1,c=>c.text());
 const culturalConsiderations=await r.list("architect_privacy_cultural_consideration",hrow.rowid,1,c=>c.text());
 const technologyControls=await r.list("architect_privacy_technology_control",hrow.rowid,1,c=>c.text());
 const signage=await r.list("architect_privacy_signage",hrow.rowid,1,c=>c.text());
 const monitoringRestrictions=await r.list("architect_privacy_monitoring_restriction",hrow.rowid,1,c=>c.text());
 const breachResponse=await r.list("architect_privacy_breach_response",hrow.rowid,1,c=>c.text());
 return{...h,privacyKind,privacyType,level,screeningRequired,enclosureRequired,observationRisk,retentionPolicy,ownerId,subjectIds,elementIds,visualPrivacy,acousticPrivacy,dataPrivacy,accessRestrictions,regulatoryBasis,culturalConsiderations,technologyControls,signage,monitoringRestrictions,breachResponse};
}
export async function projectEnvironmentalRequirement(p:ArtifactSqliteProjection,id:bigint,v:EnvironmentalRequirement):Promise<void>{
 await p.insert("architect_environmental_requirement",[...symbol(v.parameterKind,EnvironmentalParameter),...text(v.parameter),...optional(v.targetValue,3,float),...optional(v.unit,1,text),...optional(v.minValue,3,float),...optional(v.maxValue,3,float),...optional(v.comfortBand,1,text),...optional(v.measurementMethod,1,text),...optional(v.monitoringFrequency,1,text),...optional(v.occupancyBasis,1,text),...optional(v.ventilationStrategy,1,text),...optional(v.daylightTarget,1,text),...optional(v.acousticTarget,1,text),...optional(v.iaqTarget,1,text),...optional(v.verificationPlan,1,text)],id);
 await ordered(p,"architect_environmental_element",id,v.elementIds,text);
 await ordered(p,"architect_environmental_seasonal_variation",id,v.seasonalVariation,text);
 await ordered(p,"architect_environmental_energy_implication",id,v.energyImplications,text);
 await ordered(p,"architect_environmental_standard",id,v.standards,text);
 await ordered(p,"architect_environmental_certification_target",id,v.certificationTargets,text);
 await ordered(p,"architect_environmental_outdoor_condition",id,v.outdoorConditions,text);
}
export async function restoreEnvironmentalRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<EnvironmentalRequirement>{
 const row=await r.body("architect_environmental_requirement",hrow.rowid,22),c=new Cursor(row!);
 const parameterKind=c.symbol(EnvironmentalParameter);
 const parameter=c.text();
 const targetValue=c.optional(3,()=>c.float());
 const unit=c.optional(1,()=>c.text());
 const minValue=c.optional(3,()=>c.float());
 const maxValue=c.optional(3,()=>c.float());
 const comfortBand=c.optional(1,()=>c.text());
 const measurementMethod=c.optional(1,()=>c.text());
 const monitoringFrequency=c.optional(1,()=>c.text());
 const occupancyBasis=c.optional(1,()=>c.text());
 const ventilationStrategy=c.optional(1,()=>c.text());
 const daylightTarget=c.optional(1,()=>c.text());
 const acousticTarget=c.optional(1,()=>c.text());
 const iaqTarget=c.optional(1,()=>c.text());
 const verificationPlan=c.optional(1,()=>c.text());
 c.end();
 const elementIds=await r.list("architect_environmental_element",hrow.rowid,1,c=>c.text());
 const seasonalVariation=await r.list("architect_environmental_seasonal_variation",hrow.rowid,1,c=>c.text());
 const energyImplications=await r.list("architect_environmental_energy_implication",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_environmental_standard",hrow.rowid,1,c=>c.text());
 const certificationTargets=await r.list("architect_environmental_certification_target",hrow.rowid,1,c=>c.text());
 const outdoorConditions=await r.list("architect_environmental_outdoor_condition",hrow.rowid,1,c=>c.text());
 return{...h,parameterKind,parameter,targetValue,unit,minValue,maxValue,comfortBand,measurementMethod,monitoringFrequency,occupancyBasis,ventilationStrategy,daylightTarget,acousticTarget,iaqTarget,verificationPlan,elementIds,seasonalVariation,energyImplications,standards,certificationTargets,outdoorConditions};
}
export async function projectHumanFactorRequirement(p:ArtifactSqliteProjection,id:bigint,v:HumanFactorRequirement):Promise<void>{
 await p.insert("architect_human_factor_requirement",[...symbol(v.aspect,HumanFactorAspect),...text(v.factor),...optional(v.cognitiveLoad,1,text),...optional(v.reachEnvelope,1,text),...optional(v.verificationMethod,1,text)],id);
 await ordered(p,"architect_human_factor_user_profile",id,v.userProfileIds,text);
 await ordered(p,"architect_human_factor_activity",id,v.activityIds,text);
 await ordered(p,"architect_human_factor_ergonomic_criterion",id,v.ergonomicCriteria,text);
 await ordered(p,"architect_human_factor_visual_demand",id,v.visualDemands,text);
 await ordered(p,"architect_human_factor_auditory_demand",id,v.auditoryDemands,text);
 await ordered(p,"architect_human_factor_posture_requirement",id,v.postureRequirements,text);
 await ordered(p,"architect_human_factor_task_lighting",id,v.lightingForTasks,text);
 await ordered(p,"architect_human_factor_thermal_comfort",id,v.thermalComfort,text);
 await ordered(p,"architect_human_factor_privacy_need",id,v.privacyNeeds,text);
 await ordered(p,"architect_human_factor_social_interaction",id,v.socialInteraction,text);
 await ordered(p,"architect_human_factor_stress_factor",id,v.stressFactors,text);
 await ordered(p,"architect_human_factor_mitigation_measure",id,v.mitigationMeasures,text);
 await ordered(p,"architect_human_factor_training_need",id,v.trainingNeeds,text);
 await ordered(p,"architect_human_factor_standard",id,v.standards,text);
 await ordered(p,"architect_human_factor_research_basis",id,v.researchBasis,text);
 await ordered(p,"architect_human_factor_element",id,v.elementIds,text);
}
export async function restoreHumanFactorRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<HumanFactorRequirement>{
 const row=await r.body("architect_human_factor_requirement",hrow.rowid,6),c=new Cursor(row!);
 const aspect=c.symbol(HumanFactorAspect);
 const factor=c.text();
 const cognitiveLoad=c.optional(1,()=>c.text());
 const reachEnvelope=c.optional(1,()=>c.text());
 const verificationMethod=c.optional(1,()=>c.text());
 c.end();
 const userProfileIds=await r.list("architect_human_factor_user_profile",hrow.rowid,1,c=>c.text());
 const activityIds=await r.list("architect_human_factor_activity",hrow.rowid,1,c=>c.text());
 const ergonomicCriteria=await r.list("architect_human_factor_ergonomic_criterion",hrow.rowid,1,c=>c.text());
 const visualDemands=await r.list("architect_human_factor_visual_demand",hrow.rowid,1,c=>c.text());
 const auditoryDemands=await r.list("architect_human_factor_auditory_demand",hrow.rowid,1,c=>c.text());
 const postureRequirements=await r.list("architect_human_factor_posture_requirement",hrow.rowid,1,c=>c.text());
 const lightingForTasks=await r.list("architect_human_factor_task_lighting",hrow.rowid,1,c=>c.text());
 const thermalComfort=await r.list("architect_human_factor_thermal_comfort",hrow.rowid,1,c=>c.text());
 const privacyNeeds=await r.list("architect_human_factor_privacy_need",hrow.rowid,1,c=>c.text());
 const socialInteraction=await r.list("architect_human_factor_social_interaction",hrow.rowid,1,c=>c.text());
 const stressFactors=await r.list("architect_human_factor_stress_factor",hrow.rowid,1,c=>c.text());
 const mitigationMeasures=await r.list("architect_human_factor_mitigation_measure",hrow.rowid,1,c=>c.text());
 const trainingNeeds=await r.list("architect_human_factor_training_need",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_human_factor_standard",hrow.rowid,1,c=>c.text());
 const researchBasis=await r.list("architect_human_factor_research_basis",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_human_factor_element",hrow.rowid,1,c=>c.text());
 return{...h,aspect,factor,cognitiveLoad,reachEnvelope,verificationMethod,userProfileIds,activityIds,ergonomicCriteria,visualDemands,auditoryDemands,postureRequirements,lightingForTasks,thermalComfort,privacyNeeds,socialInteraction,stressFactors,mitigationMeasures,trainingNeeds,standards,researchBasis,elementIds};
}
export async function projectStorageRequirement(p:ArtifactSqliteProjection,id:bigint,v:StorageRequirement):Promise<void>{
 await p.insert("architect_storage_requirement",[...text(v.storedItem),...symbol(v.storageClass,StorageClass),...optional(v.volumeM3,3,float),...optional(v.weightKg,3,float),...optional(v.temperatureRange,1,text),...optional(v.humidityRange,1,text),...symbol(v.securityLevel,AccessLevel),...optional(v.hazardClass,1,text),...optional(v.retentionPeriod,1,text),...optional(v.accessFrequency,1,text),...optional(v.ventilation,1,text),...optional(v.organizationSystem,1,text),...optional(v.growthAllowance,3,float),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_storage_element",id,v.elementIds,text);
 await ordered(p,"architect_storage_equipment",id,v.equipmentIds,text);
 await ordered(p,"architect_storage_handling_equipment",id,v.handlingEquipment,text);
 await ordered(p,"architect_storage_fire_protection",id,v.fireProtection,text);
 await ordered(p,"architect_storage_regulatory_reference",id,v.regulatoryRefs,text);
 await quantity(p,id,"quantity","architect_storage_quantity",v.quantity);
}
export async function restoreStorageRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<StorageRequirement>{
 const row=await r.body("architect_storage_requirement",hrow.rowid,21),c=new Cursor(row!);
 const storedItem=c.text();
 const storageClass=c.symbol(StorageClass);
 const volumeM3=c.optional(3,()=>c.float());
 const weightKg=c.optional(3,()=>c.float());
 const temperatureRange=c.optional(1,()=>c.text());
 const humidityRange=c.optional(1,()=>c.text());
 const securityLevel=c.symbol(AccessLevel);
 const hazardClass=c.optional(1,()=>c.text());
 const retentionPeriod=c.optional(1,()=>c.text());
 const accessFrequency=c.optional(1,()=>c.text());
 const ventilation=c.optional(1,()=>c.text());
 const organizationSystem=c.optional(1,()=>c.text());
 const growthAllowance=c.optional(3,()=>c.float());
 const ownerId=c.optional(1,()=>c.text());
 c.end();
 const elementIds=await r.list("architect_storage_element",hrow.rowid,1,c=>c.text());
 const equipmentIds=await r.list("architect_storage_equipment",hrow.rowid,1,c=>c.text());
 const handlingEquipment=await r.list("architect_storage_handling_equipment",hrow.rowid,1,c=>c.text());
 const fireProtection=await r.list("architect_storage_fire_protection",hrow.rowid,1,c=>c.text());
 const regulatoryRefs=await r.list("architect_storage_regulatory_reference",hrow.rowid,1,c=>c.text());
 const quantity=await r.quantity(hrow.rowid,"quantity","architect_storage_quantity");
 return{...h,storedItem,storageClass,volumeM3,weightKg,temperatureRange,humidityRange,securityLevel,hazardClass,retentionPeriod,accessFrequency,ventilation,organizationSystem,growthAllowance,ownerId,elementIds,equipmentIds,handlingEquipment,fireProtection,regulatoryRefs,quantity};
}
export async function projectEquipment(p:ArtifactSqliteProjection,id:bigint,v:Equipment):Promise<void>{
 await p.insert("architect_equipment",[...text(v.code),...text(v.category),...optional(v.manufacturer,1,text),...optional(v.model,1,text),...optional(v.dimensions,1,text),...optional(v.weightKg,3,float),...optional(v.powerKw,3,float),...optional(v.ventilation,1,text),...optional(v.noiseLevelDb,3,float),...optional(v.clearance,1,text),...optional(v.mounting,1,text),...optional(v.lifecycleYears,1,uint32),...optional(v.replacementCost,3,float),...optional(v.supplier,1,text)],id);
 await ordered(p,"architect_equipment_utility_connection",id,v.utilityConnections,text);
 await ordered(p,"architect_equipment_element",id,v.elementIds,text);
 await ordered(p,"architect_equipment_activity",id,v.activityIds,text);
 await ordered(p,"architect_equipment_maintenance_access",id,v.maintenanceAccess,text);
 await ordered(p,"architect_equipment_standard",id,v.standards,text);
 await ordered(p,"architect_equipment_activity_link",id,v.activityLinkIds,text);
 await ordered(p,"architect_equipment_installation_requirement",id,v.installationRequirements,text);
 await ordered(p,"architect_equipment_commissioning_note",id,v.commissioningNotes,text);
 await ordered(p,"architect_equipment_spare_part",id,v.spareParts,text);
 await quantity(p,id,"quantity","architect_equipment_quantity",v.quantity);
}
export async function restoreEquipment(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Equipment>{
 const row=await r.body("architect_equipment",hrow.rowid,23),c=new Cursor(row!);
 const code=c.text();
 const category=c.text();
 const manufacturer=c.optional(1,()=>c.text());
 const model=c.optional(1,()=>c.text());
 const dimensions=c.optional(1,()=>c.text());
 const weightKg=c.optional(3,()=>c.float());
 const powerKw=c.optional(3,()=>c.float());
 const ventilation=c.optional(1,()=>c.text());
 const noiseLevelDb=c.optional(3,()=>c.float());
 const clearance=c.optional(1,()=>c.text());
 const mounting=c.optional(1,()=>c.text());
 const lifecycleYears=c.optional(1,()=>c.uint32());
 const replacementCost=c.optional(3,()=>c.float());
 const supplier=c.optional(1,()=>c.text());
 c.end();
 const utilityConnections=await r.list("architect_equipment_utility_connection",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_equipment_element",hrow.rowid,1,c=>c.text());
 const activityIds=await r.list("architect_equipment_activity",hrow.rowid,1,c=>c.text());
 const maintenanceAccess=await r.list("architect_equipment_maintenance_access",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_equipment_standard",hrow.rowid,1,c=>c.text());
 const activityLinkIds=await r.list("architect_equipment_activity_link",hrow.rowid,1,c=>c.text());
 const installationRequirements=await r.list("architect_equipment_installation_requirement",hrow.rowid,1,c=>c.text());
 const commissioningNotes=await r.list("architect_equipment_commissioning_note",hrow.rowid,1,c=>c.text());
 const spareParts=await r.list("architect_equipment_spare_part",hrow.rowid,1,c=>c.text());
 const quantity=await r.quantity(hrow.rowid,"quantity","architect_equipment_quantity");
 return{...h,code,category,manufacturer,model,dimensions,weightKg,powerKw,ventilation,noiseLevelDb,clearance,mounting,lifecycleYears,replacementCost,supplier,utilityConnections,elementIds,activityIds,maintenanceAccess,standards,activityLinkIds,installationRequirements,commissioningNotes,spareParts,quantity};
}
export async function projectResource(p:ArtifactSqliteProjection,id:bigint,v:Resource):Promise<void>{
 await p.insert("architect_resource",[...text(v.code),...text(v.category),...text(v.resourceType),...optional(v.mobility,1,text),...optional(v.sharingModel,1,text),...optional(v.allocation,1,text),...optional(v.storageRequirementId,1,text),...optional(v.durability,1,text),...optional(v.replacementCycle,1,text),...optional(v.costPerUnit,3,float),...optional(v.supplier,1,text),...optional(v.furnitureClass,1,text),...optional(v.ergonomicsRating,1,text),...optional(v.sharingRatio,3,float)],id);
 await ordered(p,"architect_resource_element",id,v.elementIds,text);
 await ordered(p,"architect_resource_activity",id,v.activityIds,text);
 await ordered(p,"architect_resource_user_profile",id,v.userProfileIds,text);
 await ordered(p,"architect_resource_cleaning_requirement",id,v.cleaningRequirements,text);
 await ordered(p,"architect_resource_standard",id,v.standards,text);
 await ordered(p,"architect_resource_ergonomic_note",id,v.ergonomicNotes,text);
 await ordered(p,"architect_resource_customization",id,v.customization,text);
 await ordered(p,"architect_resource_disposal_note",id,v.disposalNotes,text);
 await quantity(p,id,"quantity","architect_resource_quantity",v.quantity);
}
export async function restoreResource(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<Resource>{
 const row=await r.body("architect_resource",hrow.rowid,19),c=new Cursor(row!);
 const code=c.text();
 const category=c.text();
 const resourceType=c.text();
 const mobility=c.optional(1,()=>c.text());
 const sharingModel=c.optional(1,()=>c.text());
 const allocation=c.optional(1,()=>c.text());
 const storageRequirementId=c.optional(1,()=>c.text());
 const durability=c.optional(1,()=>c.text());
 const replacementCycle=c.optional(1,()=>c.text());
 const costPerUnit=c.optional(3,()=>c.float());
 const supplier=c.optional(1,()=>c.text());
 const furnitureClass=c.optional(1,()=>c.text());
 const ergonomicsRating=c.optional(1,()=>c.text());
 const sharingRatio=c.optional(3,()=>c.float());
 c.end();
 const elementIds=await r.list("architect_resource_element",hrow.rowid,1,c=>c.text());
 const activityIds=await r.list("architect_resource_activity",hrow.rowid,1,c=>c.text());
 const userProfileIds=await r.list("architect_resource_user_profile",hrow.rowid,1,c=>c.text());
 const cleaningRequirements=await r.list("architect_resource_cleaning_requirement",hrow.rowid,1,c=>c.text());
 const standards=await r.list("architect_resource_standard",hrow.rowid,1,c=>c.text());
 const ergonomicNotes=await r.list("architect_resource_ergonomic_note",hrow.rowid,1,c=>c.text());
 const customization=await r.list("architect_resource_customization",hrow.rowid,1,c=>c.text());
 const disposalNotes=await r.list("architect_resource_disposal_note",hrow.rowid,1,c=>c.text());
 const quantity=await r.quantity(hrow.rowid,"quantity","architect_resource_quantity");
 return{...h,code,category,resourceType,mobility,sharingModel,allocation,storageRequirementId,durability,replacementCycle,costPerUnit,supplier,furnitureClass,ergonomicsRating,sharingRatio,elementIds,activityIds,userProfileIds,cleaningRequirements,standards,ergonomicNotes,customization,disposalNotes,quantity};
}
export async function projectOperationalRequirement(p:ArtifactSqliteProjection,id:bigint,v:OperationalRequirement):Promise<void>{
 await p.insert("architect_operational_requirement",[...text(v.operation),...optional(v.serviceLevel,1,text),...optional(v.operatingHours,1,text),...optional(v.maintenanceInterval,1,text),...optional(v.cleaningRegime,1,text),...optional(v.turnaroundTime,1,text),...optional(v.redundancy,1,text),...optional(v.uptimeTarget,3,float),...optional(v.responseTime,1,text),...optional(v.ownerId,1,text),...optional(v.serviceCategory,1,text),...optional(v.shiftPattern,1,text),...optional(v.slaTarget,1,text),...optional(v.escalationContactId,1,text)],id);
 await ordered(p,"architect_operation_equipment",id,v.equipmentIds,text);
 await ordered(p,"architect_operation_element",id,v.elementIds,text);
 await ordered(p,"architect_operation_process",id,v.processIds,text);
 await ordered(p,"architect_operation_utility",id,v.utilities,text);
 await ordered(p,"architect_operation_waste_stream",id,v.wasteStreams,text);
 await ordered(p,"architect_operation_contingency_plan",id,v.contingencyPlan,text);
 await ordered(p,"architect_operation_training_requirement",id,v.trainingRequirements,text);
 await ordered(p,"architect_operation_sop_reference",id,v.sopReferences,text);
 await ordered(p,"architect_operation_kpi_target",id,v.kpiTargets,text);
 await quantity(p,id,"staffing","architect_operation_staffing",v.staffing);
}
export async function restoreOperationalRequirement(r:Reader,hrow:SqliteRow,h:EntityHeader):Promise<OperationalRequirement>{
 const row=await r.body("architect_operational_requirement",hrow.rowid,17),c=new Cursor(row!);
 const operation=c.text();
 const serviceLevel=c.optional(1,()=>c.text());
 const operatingHours=c.optional(1,()=>c.text());
 const maintenanceInterval=c.optional(1,()=>c.text());
 const cleaningRegime=c.optional(1,()=>c.text());
 const turnaroundTime=c.optional(1,()=>c.text());
 const redundancy=c.optional(1,()=>c.text());
 const uptimeTarget=c.optional(3,()=>c.float());
 const responseTime=c.optional(1,()=>c.text());
 const ownerId=c.optional(1,()=>c.text());
 const serviceCategory=c.optional(1,()=>c.text());
 const shiftPattern=c.optional(1,()=>c.text());
 const slaTarget=c.optional(1,()=>c.text());
 const escalationContactId=c.optional(1,()=>c.text());
 c.end();
 const equipmentIds=await r.list("architect_operation_equipment",hrow.rowid,1,c=>c.text());
 const elementIds=await r.list("architect_operation_element",hrow.rowid,1,c=>c.text());
 const processIds=await r.list("architect_operation_process",hrow.rowid,1,c=>c.text());
 const utilities=await r.list("architect_operation_utility",hrow.rowid,1,c=>c.text());
 const wasteStreams=await r.list("architect_operation_waste_stream",hrow.rowid,1,c=>c.text());
 const contingencyPlan=await r.list("architect_operation_contingency_plan",hrow.rowid,1,c=>c.text());
 const trainingRequirements=await r.list("architect_operation_training_requirement",hrow.rowid,1,c=>c.text());
 const sopReferences=await r.list("architect_operation_sop_reference",hrow.rowid,1,c=>c.text());
 const kpiTargets=await r.list("architect_operation_kpi_target",hrow.rowid,1,c=>c.text());
 const staffing=await r.quantity(hrow.rowid,"staffing","architect_operation_staffing");
 return{...h,operation,serviceLevel,operatingHours,maintenanceInterval,cleaningRegime,turnaroundTime,redundancy,uptimeTarget,responseTime,ownerId,serviceCategory,shiftPattern,slaTarget,escalationContactId,equipmentIds,elementIds,processIds,utilities,wasteStreams,contingencyPlan,trainingRequirements,sopReferences,kpiTargets,staffing};
}
