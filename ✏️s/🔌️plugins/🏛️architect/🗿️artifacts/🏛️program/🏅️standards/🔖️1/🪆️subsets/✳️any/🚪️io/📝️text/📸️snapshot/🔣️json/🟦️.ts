/** 🔣️ Declared Program JSON transport scalars map explicitly to canonical owned fields. */
import {architectProgramArtifactGuardObject,architectProgramArtifactGuardExactObject,architectProgramArtifactGuardArray,architectProgramArtifactGuardBinary64,architectProgramArtifactGuardUnsigned64,parseProgramArtifact,PROGRAM_ARTIFACT_FIELDS,type ProgramArtifact} from "../../../../🧬️schema/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

function float(value:unknown,decode:boolean):unknown {
 if(!decode)return{bits:architectProgramArtifactGuardBinary64(value,"Program JSON binary64").bits.toString(16).padStart(16,"0")};
 if(typeof value==="number"){if(!Number.isFinite(value))throw new Error("Program JSON numeric transport requires a finite double");return binary64(value)}
 const row=architectProgramArtifactGuardExactObject(value,"Program JSON binary64",["bits"]);
 if(typeof row.bits!=="string"||!/^[0-9a-f]{16}$/.test(row.bits))throw new Error("Program JSON binary64 requires exactly sixteen lowercase hexadecimal digits");
 return{bits:BigInt("0x"+row.bits)};
}
function optionalFloat(value:unknown,decode:boolean):unknown{return value===null?null:float(value,decode)}
function unsigned(value:unknown,decode:boolean):unknown {
 if(!decode)return architectProgramArtifactGuardUnsigned64(value,"Program JSON unsigned64").toString();
 let word:bigint;
 if(typeof value==="number"){if(!Number.isSafeInteger(value)||value<0)throw new Error("Program JSON numeric unsigned64 transport requires a safe unsigned integer");word=BigInt(value)}
 else if(typeof value==="string"&&/^(0|[1-9][0-9]{0,19})$/.test(value))word=BigInt(value);
 else throw new Error("Program JSON unsigned64 requires a canonical decimal string");
 return architectProgramArtifactGuardUnsigned64(word,"Program JSON unsigned64");
}
function quantity(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON quantity"),out={...row};
 if(row.min!==undefined)out.min=float(row.min,decode);
 if(row.max!==undefined)out.max=float(row.max,decode);
 if(row.target!==undefined)out.target=float(row.target,decode);
 if(row.current!==undefined)out.current=float(row.current,decode);
 if(row.forecast!==undefined)out.forecast=float(row.forecast,decode);
 if(row.peak!==undefined)out.peak=float(row.peak,decode);
 if(row.average!==undefined)out.average=float(row.average,decode);
 return out;
}
export function header(value:unknown,decode:boolean):unknown {const row=architectProgramArtifactGuardObject(value,"Program JSON entity");return decode?{...row,tags:row.tags===undefined?[]:row.tags,notes:row.notes===undefined?[]:row.notes}:row}
export function rows(value:unknown,convert:(value:unknown,decode:boolean)=>unknown,decode:boolean):unknown[]{return architectProgramArtifactGuardArray(value,"Program JSON register").map(item=>convert(header(item,decode),decode))}

export function accessibilityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON AccessibilityRequirement"),out={...row};
 if(Object.hasOwn(row,"clearWidthM")&&row.clearWidthM!==null)out.clearWidthM=optionalFloat(row.clearWidthM,decode);
 if(Object.hasOwn(row,"clearHeightM")&&row.clearHeightM!==null)out.clearHeightM=optionalFloat(row.clearHeightM,decode);
 if(Object.hasOwn(row,"turningCircleM")&&row.turningCircleM!==null)out.turningCircleM=optionalFloat(row.turningCircleM,decode);
 if(Object.hasOwn(row,"rampSlope")&&row.rampSlope!==null)out.rampSlope=optionalFloat(row.rampSlope,decode);
 return out;
}

export function adjacency(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Adjacency"),out={...row};
 if(Object.hasOwn(row,"weight")&&row.weight!==null)out.weight=float(row.weight,decode);
 if(Object.hasOwn(row,"distanceMaxM")&&row.distanceMaxM!==null)out.distanceMaxM=optionalFloat(row.distanceMaxM,decode);
 if(Object.hasOwn(row,"distanceMinM")&&row.distanceMinM!==null)out.distanceMinM=optionalFloat(row.distanceMinM,decode);
 return out;
}

export function benchmarkRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON BenchmarkRecord"),out={...row};
 if(Object.hasOwn(row,"value")&&row.value!==null)out.value=float(row.value,decode);
 return out;
}

export function changeRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON ChangeRecord"),out={...row};
 if(Object.hasOwn(row,"costImpact")&&row.costImpact!==null)out.costImpact=optionalFloat(row.costImpact,decode);
 return out;
}

export function conflict(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Conflict"),out={...row};
 if(Object.hasOwn(row,"costImpact")&&row.costImpact!==null)out.costImpact=optionalFloat(row.costImpact,decode);
 return out;
}

export function costRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON CostRequirement"),out={...row};
 if(Object.hasOwn(row,"amount")&&row.amount!==null)out.amount=optionalFloat(row.amount,decode);
 if(Object.hasOwn(row,"unitCost")&&row.unitCost!==null)out.unitCost=optionalFloat(row.unitCost,decode);
 if(Object.hasOwn(row,"contingencyPercent")&&row.contingencyPercent!==null)out.contingencyPercent=optionalFloat(row.contingencyPercent,decode);
 if(Object.hasOwn(row,"escalationRate")&&row.escalationRate!==null)out.escalationRate=optionalFloat(row.escalationRate,decode);
 return out;
}

export function decision(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Decision"),out={...row};
 if(Object.hasOwn(row,"costImpact")&&row.costImpact!==null)out.costImpact=optionalFloat(row.costImpact,decode);
 return out;
}

export function environmentalRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON EnvironmentalRequirement"),out={...row};
 if(Object.hasOwn(row,"targetValue")&&row.targetValue!==null)out.targetValue=optionalFloat(row.targetValue,decode);
 if(Object.hasOwn(row,"minValue")&&row.minValue!==null)out.minValue=optionalFloat(row.minValue,decode);
 if(Object.hasOwn(row,"maxValue")&&row.maxValue!==null)out.maxValue=optionalFloat(row.maxValue,decode);
 return out;
}

export function equipment(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Equipment"),out={...row};
 if(Object.hasOwn(row,"weightKg")&&row.weightKg!==null)out.weightKg=optionalFloat(row.weightKg,decode);
 if(Object.hasOwn(row,"powerKw")&&row.powerKw!==null)out.powerKw=optionalFloat(row.powerKw,decode);
 if(Object.hasOwn(row,"noiseLevelDb")&&row.noiseLevelDb!==null)out.noiseLevelDb=optionalFloat(row.noiseLevelDb,decode);
 if(Object.hasOwn(row,"replacementCost")&&row.replacementCost!==null)out.replacementCost=optionalFloat(row.replacementCost,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

export function flexibilityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON FlexibilityRequirement"),out={...row};
 if(Object.hasOwn(row,"costOfChange")&&row.costOfChange!==null)out.costOfChange=optionalFloat(row.costOfChange,decode);
 return out;
}

export function flowRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON FlowRequirement"),out={...row};
 if(Object.hasOwn(row,"peakRate")&&row.peakRate!==null)out.peakRate=optionalFloat(row.peakRate,decode);
 if(Object.hasOwn(row,"clearWidthM")&&row.clearWidthM!==null)out.clearWidthM=optionalFloat(row.clearWidthM,decode);
 if(Object.hasOwn(row,"clearHeightM")&&row.clearHeightM!==null)out.clearHeightM=optionalFloat(row.clearHeightM,decode);
 if(Object.hasOwn(row,"volume")&&row.volume!==null)out.volume=quantity(row.volume,decode);
 return out;
}

export function growthPlan(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON GrowthPlan"),out={...row};
 if(Object.hasOwn(row,"growthRate")&&row.growthRate!==null)out.growthRate=optionalFloat(row.growthRate,decode);
 if(Object.hasOwn(row,"budgetEnvelope")&&row.budgetEnvelope!==null)out.budgetEnvelope=optionalFloat(row.budgetEnvelope,decode);
 if(Object.hasOwn(row,"headcountGrowth")&&row.headcountGrowth!==null)out.headcountGrowth=quantity(row.headcountGrowth,decode);
 if(Object.hasOwn(row,"areaGrowth")&&row.areaGrowth!==null)out.areaGrowth=quantity(row.areaGrowth,decode);
 return out;
}

export function infrastructureRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON InfrastructureRequirement"),out={...row};
 if(Object.hasOwn(row,"peakDemand")&&row.peakDemand!==null)out.peakDemand=optionalFloat(row.peakDemand,decode);
 if(Object.hasOwn(row,"diversityFactor")&&row.diversityFactor!==null)out.diversityFactor=optionalFloat(row.diversityFactor,decode);
 if(Object.hasOwn(row,"lifecycleCost")&&row.lifecycleCost!==null)out.lifecycleCost=optionalFloat(row.lifecycleCost,decode);
 if(Object.hasOwn(row,"capacity")&&row.capacity!==null)out.capacity=quantity(row.capacity,decode);
 return out;
}

export function operationalRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON OperationalRequirement"),out={...row};
 if(Object.hasOwn(row,"uptimeTarget")&&row.uptimeTarget!==null)out.uptimeTarget=optionalFloat(row.uptimeTarget,decode);
 if(Object.hasOwn(row,"staffing")&&row.staffing!==null)out.staffing=quantity(row.staffing,decode);
 return out;
}

export function optionEvaluation(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON OptionEvaluation"),out={...row};
 if(Object.hasOwn(row,"scores")&&row.scores!==null)out.scores=architectProgramArtifactGuardArray(row.scores,"Program JSON OptionEvaluation.scores").map(value=>float(value,decode));
 if(Object.hasOwn(row,"weightedScore")&&row.weightedScore!==null)out.weightedScore=optionalFloat(row.weightedScore,decode);
 if(Object.hasOwn(row,"costEstimate")&&row.costEstimate!==null)out.costEstimate=optionalFloat(row.costEstimate,decode);
 return out;
}

export function performanceCriterion(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON PerformanceCriterion"),out={...row};
 if(Object.hasOwn(row,"target")&&row.target!==null)out.target=optionalFloat(row.target,decode);
 if(Object.hasOwn(row,"minimum")&&row.minimum!==null)out.minimum=optionalFloat(row.minimum,decode);
 if(Object.hasOwn(row,"maximum")&&row.maximum!==null)out.maximum=optionalFloat(row.maximum,decode);
 if(Object.hasOwn(row,"baseline")&&row.baseline!==null)out.baseline=optionalFloat(row.baseline,decode);
 if(Object.hasOwn(row,"weight")&&row.weight!==null)out.weight=optionalFloat(row.weight,decode);
 if(Object.hasOwn(row,"penaltyThreshold")&&row.penaltyThreshold!==null)out.penaltyThreshold=optionalFloat(row.penaltyThreshold,decode);
 if(Object.hasOwn(row,"incentiveThreshold")&&row.incentiveThreshold!==null)out.incentiveThreshold=optionalFloat(row.incentiveThreshold,decode);
 return out;
}

export function priorityRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON PriorityRecord"),out={...row};
 if(Object.hasOwn(row,"weight")&&row.weight!==null)out.weight=optionalFloat(row.weight,decode);
 if(Object.hasOwn(row,"score")&&row.score!==null)out.score=optionalFloat(row.score,decode);
 return out;
}

export function quantityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON QuantityRequirement"),out={...row};
 if(Object.hasOwn(row,"tolerancePercent")&&row.tolerancePercent!==null)out.tolerancePercent=optionalFloat(row.tolerancePercent,decode);
 if(Object.hasOwn(row,"peakFactor")&&row.peakFactor!==null)out.peakFactor=optionalFloat(row.peakFactor,decode);
 if(Object.hasOwn(row,"growthFactor")&&row.growthFactor!==null)out.growthFactor=optionalFloat(row.growthFactor,decode);
 if(Object.hasOwn(row,"unitCost")&&row.unitCost!==null)out.unitCost=optionalFloat(row.unitCost,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

export function relationship(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Relationship"),out={...row};
 if(Object.hasOwn(row,"strength")&&row.strength!==null)out.strength=optionalFloat(row.strength,decode);
 if(Object.hasOwn(row,"distanceConstraintM")&&row.distanceConstraintM!==null)out.distanceConstraintM=optionalFloat(row.distanceConstraintM,decode);
 return out;
}

export function requirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Requirement"),out={...row};
 if(Object.hasOwn(row,"costEstimate")&&row.costEstimate!==null)out.costEstimate=optionalFloat(row.costEstimate,decode);
 return out;
}

export function resource(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Resource"),out={...row};
 if(Object.hasOwn(row,"costPerUnit")&&row.costPerUnit!==null)out.costPerUnit=optionalFloat(row.costPerUnit,decode);
 if(Object.hasOwn(row,"sharingRatio")&&row.sharingRatio!==null)out.sharingRatio=optionalFloat(row.sharingRatio,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

export function risk(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Risk"),out={...row};
 if(Object.hasOwn(row,"riskScore")&&row.riskScore!==null)out.riskScore=optionalFloat(row.riskScore,decode);
 return out;
}

export function scenario(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Scenario"),out={...row};
 if(Object.hasOwn(row,"probability")&&row.probability!==null)out.probability=optionalFloat(row.probability,decode);
 if(Object.hasOwn(row,"costDelta")&&row.costDelta!==null)out.costDelta=optionalFloat(row.costDelta,decode);
 if(Object.hasOwn(row,"areaDelta")&&row.areaDelta!==null)out.areaDelta=optionalFloat(row.areaDelta,decode);
 if(Object.hasOwn(row,"headcountDelta")&&row.headcountDelta!==null)out.headcountDelta=optionalFloat(row.headcountDelta,decode);
 return out;
}

export function siteContext(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON SiteContext"),out={...row};
 if(Object.hasOwn(row,"latitude")&&row.latitude!==null)out.latitude=optionalFloat(row.latitude,decode);
 if(Object.hasOwn(row,"longitude")&&row.longitude!==null)out.longitude=optionalFloat(row.longitude,decode);
 if(Object.hasOwn(row,"elevationM")&&row.elevationM!==null)out.elevationM=optionalFloat(row.elevationM,decode);
 if(Object.hasOwn(row,"maxHeightM")&&row.maxHeightM!==null)out.maxHeightM=optionalFloat(row.maxHeightM,decode);
 if(Object.hasOwn(row,"maxCoverage")&&row.maxCoverage!==null)out.maxCoverage=optionalFloat(row.maxCoverage,decode);
 return out;
}

export function statusRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON StatusRecord"),out={...row};
 if(Object.hasOwn(row,"progressPercent")&&row.progressPercent!==null)out.progressPercent=optionalFloat(row.progressPercent,decode);
 return out;
}

export function storageRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON StorageRequirement"),out={...row};
 if(Object.hasOwn(row,"volumeM3")&&row.volumeM3!==null)out.volumeM3=optionalFloat(row.volumeM3,decode);
 if(Object.hasOwn(row,"weightKg")&&row.weightKg!==null)out.weightKg=optionalFloat(row.weightKg,decode);
 if(Object.hasOwn(row,"growthAllowance")&&row.growthAllowance!==null)out.growthAllowance=optionalFloat(row.growthAllowance,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

export function survey(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Survey"),out={...row};
 if(Object.hasOwn(row,"responseRate")&&row.responseRate!==null)out.responseRate=optionalFloat(row.responseRate,decode);
 return out;
}

export function sustainabilityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON SustainabilityRequirement"),out={...row};
 if(Object.hasOwn(row,"baseline")&&row.baseline!==null)out.baseline=optionalFloat(row.baseline,decode);
 if(Object.hasOwn(row,"targetValue")&&row.targetValue!==null)out.targetValue=optionalFloat(row.targetValue,decode);
 if(Object.hasOwn(row,"embodiedCarbon")&&row.embodiedCarbon!==null)out.embodiedCarbon=optionalFloat(row.embodiedCarbon,decode);
 if(Object.hasOwn(row,"operationalCarbon")&&row.operationalCarbon!==null)out.operationalCarbon=optionalFloat(row.operationalCarbon,decode);
 return out;
}

export function wayfindingRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON WayfindingRequirement"),out={...row};
 if(Object.hasOwn(row,"maximumSignageDistanceM")&&row.maximumSignageDistanceM!==null)out.maximumSignageDistanceM=optionalFloat(row.maximumSignageDistanceM,decode);
 return out;
}

export function workshop(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Workshop"),out={...row};
 if(Object.hasOwn(row,"budget")&&row.budget!==null)out.budget=optionalFloat(row.budget,decode);
 return out;
}

export function activity(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Activity"),out={...row};
 if(Object.hasOwn(row,"participants")&&row.participants!==null)out.participants=quantity(row.participants,decode);
 return out;
}

export function functionRow(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Function"),out={...row};
 if(Object.hasOwn(row,"staffing")&&row.staffing!==null)out.staffing=quantity(row.staffing,decode);
 return out;
}

export function programElement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON ProgramElement"),out={...row};
 if(Object.hasOwn(row,"area")&&row.area!==null)out.area=quantity(row.area,decode);
 if(Object.hasOwn(row,"volume")&&row.volume!==null)out.volume=quantity(row.volume,decode);
 if(Object.hasOwn(row,"height")&&row.height!==null)out.height=quantity(row.height,decode);
 if(Object.hasOwn(row,"occupancy")&&row.occupancy!==null)out.occupancy=quantity(row.occupancy,decode);
 return out;
}

export function organizationalRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON OrganizationalRequirement"),out={...row};
 if(Object.hasOwn(row,"headcount")&&row.headcount!==null)out.headcount=quantity(row.headcount,decode);
 return out;
}

export function serviceRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON ServiceRequirement"),out={...row};
 if(Object.hasOwn(row,"capacity")&&row.capacity!==null)out.capacity=quantity(row.capacity,decode);
 if(Object.hasOwn(row,"staffing")&&row.staffing!==null)out.staffing=quantity(row.staffing,decode);
 return out;
}

export function analysisRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON AnalysisRecord"),out={...row};
 if(Object.hasOwn(row,"durationMs")&&row.durationMs!==null)out.durationMs=row.durationMs===null?null:unsigned(row.durationMs,decode);
 return out;
}

export function searchFilter(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON SearchFilter"),out={...row};
 if(Object.hasOwn(row,"useCount")&&row.useCount!==null)out.useCount=unsigned(row.useCount,decode);
 return out;
}

export function templateRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON TemplateRecord"),out={...row};
 if(Object.hasOwn(row,"usageCount")&&row.usageCount!==null)out.usageCount=unsigned(row.usageCount,decode);
 return out;
}

export function knowledgeRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON KnowledgeRecord"),out={...row};
 if(Object.hasOwn(row,"usageCount")&&row.usageCount!==null)out.usageCount=unsigned(row.usageCount,decode);
 return out;
}

export function program(value:unknown,decode:boolean):Record<string,unknown> {
 const row=architectProgramArtifactGuardExactObject(value,"Program JSON",PROGRAM_ARTIFACT_FIELDS),out={...row};
 out.accessibility=rows(row.accessibility,accessibilityRequirement,decode);
 out.adjacencies=rows(row.adjacencies,adjacency,decode);
 out.benchmarksPayload=rows(row.benchmarksPayload,benchmarkRecord,decode);
 out.changes=rows(row.changes,changeRecord,decode);
 out.conflicts=rows(row.conflicts,conflict,decode);
 out.costs=rows(row.costs,costRequirement,decode);
 out.decisions=rows(row.decisions,decision,decode);
 out.environmental=rows(row.environmental,environmentalRequirement,decode);
 out.equipment=rows(row.equipment,equipment,decode);
 out.flexibility=rows(row.flexibility,flexibilityRequirement,decode);
 out.flows=rows(row.flows,flowRequirement,decode);
 out.growth=rows(row.growth,growthPlan,decode);
 out.infrastructure=rows(row.infrastructure,infrastructureRequirement,decode);
 out.operations=rows(row.operations,operationalRequirement,decode);
 out.options=rows(row.options,optionEvaluation,decode);
 out.performance=rows(row.performance,performanceCriterion,decode);
 out.priorities=rows(row.priorities,priorityRecord,decode);
 out.quantities=rows(row.quantities,quantityRequirement,decode);
 out.relationships=rows(row.relationships,relationship,decode);
 out.requirements=rows(row.requirements,requirement,decode);
 out.resources=rows(row.resources,resource,decode);
 out.risks=rows(row.risks,risk,decode);
 out.scenarios=rows(row.scenarios,scenario,decode);
 out.siteContext=rows(row.siteContext,siteContext,decode);
 out.statusRecords=rows(row.statusRecords,statusRecord,decode);
 out.storage=rows(row.storage,storageRequirement,decode);
 out.surveys=rows(row.surveys,survey,decode);
 out.sustainability=rows(row.sustainability,sustainabilityRequirement,decode);
 out.wayfinding=rows(row.wayfinding,wayfindingRequirement,decode);
 out.workshops=rows(row.workshops,workshop,decode);
 out.activities=rows(row.activities,activity,decode);
 out.functions=rows(row.functions,functionRow,decode);
 out.elements=rows(row.elements,programElement,decode);
 out.organizational=rows(row.organizational,organizationalRequirement,decode);
 out.services=rows(row.services,serviceRequirement,decode);
 out.analyses=rows(row.analyses,analysisRecord,decode);
 out.searchFilters=rows(row.searchFilters,searchFilter,decode);
 out.templates=rows(row.templates,templateRecord,decode);
 out.knowledgePayload=rows(row.knowledgePayload,knowledgeRecord,decode);
 out.approvals=rows(row.approvals,header,decode);
 out.meetings=rows(row.meetings,header,decode);
 out.assumptions=rows(row.assumptions,header,decode);
 out.constraints=rows(row.constraints,header,decode);
 out.complianceRecords=rows(row.complianceRecords,header,decode);
 out.issues=rows(row.issues,header,decode);
 out.collaboration=rows(row.collaboration,header,decode);
 out.reports=rows(row.reports,header,decode);
 out.quality=rows(row.quality,header,decode);
 out.artifacts=rows(row.artifacts,header,decode);
 out.validations=rows(row.validations,header,decode);
 out.delivery=rows(row.delivery,header,decode);
 out.resilience=rows(row.resilience,header,decode);
 out.schedules=rows(row.schedules,header,decode);
 out.information=rows(row.information,header,decode);
 out.communication=rows(row.communication,header,decode);
 out.safety=rows(row.safety,header,decode);
 out.security=rows(row.security,header,decode);
 out.regulatory=rows(row.regulatory,header,decode);
 out.privacy=rows(row.privacy,header,decode);
 out.humanFactors=rows(row.humanFactors,header,decode);
 out.accessRules=rows(row.accessRules,header,decode);
 out.processes=rows(row.processes,header,decode);
 out.users=rows(row.users,header,decode);
 out.stakeholders=rows(row.stakeholders,header,decode);
 out.auditEvents=rows(row.auditEvents,header,decode);
 return out;
}
/** 📥️ Admit declared numeric JSON transport into the complete canonical typed model. */
export function programArtifactFromJson(value:unknown):ProgramArtifact{return parseProgramArtifact(program(value,true))}
/** 📤️ Emit exact IEEE and unsigned word scalars in the declared Program JSON transport. */
export function programArtifactToJson(value:ProgramArtifact):Readonly<Record<string,unknown>>{return program(parseProgramArtifact(value),false)}
