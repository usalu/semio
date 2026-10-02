/** 🔣️ Declared Program JSON transport scalars map explicitly to canonical owned fields. */
import { architectProgramArtifactGuardObject, architectProgramArtifactGuardExactObject, architectProgramArtifactGuardArray, architectProgramArtifactGuardBinary64, architectProgramArtifactGuardUnsigned64, parseProgramArtifact, PROGRAM_ARTIFACT_FIELDS, type ProgramArtifact } from "../🟦️.ts";
import { binary64 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { parseProgramDiff, type ProgramDiff } from "../🔺️diff/🟦️.ts";

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
function header(value:unknown,decode:boolean):unknown {const row=architectProgramArtifactGuardObject(value,"Program JSON entity");return decode?{...row,tags:row.tags===undefined?[]:row.tags,notes:row.notes===undefined?[]:row.notes}:row}
function rows(value:unknown,convert:(value:unknown,decode:boolean)=>unknown,decode:boolean):unknown[]{return architectProgramArtifactGuardArray(value,"Program JSON register").map(item=>convert(header(item,decode),decode))}

function accessibilityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON AccessibilityRequirement"),out={...row};
 if(Object.hasOwn(row,"clearWidthM")&&row.clearWidthM!==null)out.clearWidthM=optionalFloat(row.clearWidthM,decode);
 if(Object.hasOwn(row,"clearHeightM")&&row.clearHeightM!==null)out.clearHeightM=optionalFloat(row.clearHeightM,decode);
 if(Object.hasOwn(row,"turningCircleM")&&row.turningCircleM!==null)out.turningCircleM=optionalFloat(row.turningCircleM,decode);
 if(Object.hasOwn(row,"rampSlope")&&row.rampSlope!==null)out.rampSlope=optionalFloat(row.rampSlope,decode);
 return out;
}

function adjacency(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Adjacency"),out={...row};
 if(Object.hasOwn(row,"weight")&&row.weight!==null)out.weight=float(row.weight,decode);
 if(Object.hasOwn(row,"distanceMaxM")&&row.distanceMaxM!==null)out.distanceMaxM=optionalFloat(row.distanceMaxM,decode);
 if(Object.hasOwn(row,"distanceMinM")&&row.distanceMinM!==null)out.distanceMinM=optionalFloat(row.distanceMinM,decode);
 return out;
}

function benchmarkRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON BenchmarkRecord"),out={...row};
 if(Object.hasOwn(row,"value")&&row.value!==null)out.value=float(row.value,decode);
 return out;
}

function changeRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON ChangeRecord"),out={...row};
 if(Object.hasOwn(row,"costImpact")&&row.costImpact!==null)out.costImpact=optionalFloat(row.costImpact,decode);
 return out;
}

function conflict(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Conflict"),out={...row};
 if(Object.hasOwn(row,"costImpact")&&row.costImpact!==null)out.costImpact=optionalFloat(row.costImpact,decode);
 return out;
}

function costRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON CostRequirement"),out={...row};
 if(Object.hasOwn(row,"amount")&&row.amount!==null)out.amount=optionalFloat(row.amount,decode);
 if(Object.hasOwn(row,"unitCost")&&row.unitCost!==null)out.unitCost=optionalFloat(row.unitCost,decode);
 if(Object.hasOwn(row,"contingencyPercent")&&row.contingencyPercent!==null)out.contingencyPercent=optionalFloat(row.contingencyPercent,decode);
 if(Object.hasOwn(row,"escalationRate")&&row.escalationRate!==null)out.escalationRate=optionalFloat(row.escalationRate,decode);
 return out;
}

function decision(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Decision"),out={...row};
 if(Object.hasOwn(row,"costImpact")&&row.costImpact!==null)out.costImpact=optionalFloat(row.costImpact,decode);
 return out;
}

function environmentalRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON EnvironmentalRequirement"),out={...row};
 if(Object.hasOwn(row,"targetValue")&&row.targetValue!==null)out.targetValue=optionalFloat(row.targetValue,decode);
 if(Object.hasOwn(row,"minValue")&&row.minValue!==null)out.minValue=optionalFloat(row.minValue,decode);
 if(Object.hasOwn(row,"maxValue")&&row.maxValue!==null)out.maxValue=optionalFloat(row.maxValue,decode);
 return out;
}

function equipment(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Equipment"),out={...row};
 if(Object.hasOwn(row,"weightKg")&&row.weightKg!==null)out.weightKg=optionalFloat(row.weightKg,decode);
 if(Object.hasOwn(row,"powerKw")&&row.powerKw!==null)out.powerKw=optionalFloat(row.powerKw,decode);
 if(Object.hasOwn(row,"noiseLevelDb")&&row.noiseLevelDb!==null)out.noiseLevelDb=optionalFloat(row.noiseLevelDb,decode);
 if(Object.hasOwn(row,"replacementCost")&&row.replacementCost!==null)out.replacementCost=optionalFloat(row.replacementCost,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

function flexibilityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON FlexibilityRequirement"),out={...row};
 if(Object.hasOwn(row,"costOfChange")&&row.costOfChange!==null)out.costOfChange=optionalFloat(row.costOfChange,decode);
 return out;
}

function flowRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON FlowRequirement"),out={...row};
 if(Object.hasOwn(row,"peakRate")&&row.peakRate!==null)out.peakRate=optionalFloat(row.peakRate,decode);
 if(Object.hasOwn(row,"clearWidthM")&&row.clearWidthM!==null)out.clearWidthM=optionalFloat(row.clearWidthM,decode);
 if(Object.hasOwn(row,"clearHeightM")&&row.clearHeightM!==null)out.clearHeightM=optionalFloat(row.clearHeightM,decode);
 if(Object.hasOwn(row,"volume")&&row.volume!==null)out.volume=quantity(row.volume,decode);
 return out;
}

function growthPlan(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON GrowthPlan"),out={...row};
 if(Object.hasOwn(row,"growthRate")&&row.growthRate!==null)out.growthRate=optionalFloat(row.growthRate,decode);
 if(Object.hasOwn(row,"budgetEnvelope")&&row.budgetEnvelope!==null)out.budgetEnvelope=optionalFloat(row.budgetEnvelope,decode);
 if(Object.hasOwn(row,"headcountGrowth")&&row.headcountGrowth!==null)out.headcountGrowth=quantity(row.headcountGrowth,decode);
 if(Object.hasOwn(row,"areaGrowth")&&row.areaGrowth!==null)out.areaGrowth=quantity(row.areaGrowth,decode);
 return out;
}

function infrastructureRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON InfrastructureRequirement"),out={...row};
 if(Object.hasOwn(row,"peakDemand")&&row.peakDemand!==null)out.peakDemand=optionalFloat(row.peakDemand,decode);
 if(Object.hasOwn(row,"diversityFactor")&&row.diversityFactor!==null)out.diversityFactor=optionalFloat(row.diversityFactor,decode);
 if(Object.hasOwn(row,"lifecycleCost")&&row.lifecycleCost!==null)out.lifecycleCost=optionalFloat(row.lifecycleCost,decode);
 if(Object.hasOwn(row,"capacity")&&row.capacity!==null)out.capacity=quantity(row.capacity,decode);
 return out;
}

function operationalRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON OperationalRequirement"),out={...row};
 if(Object.hasOwn(row,"uptimeTarget")&&row.uptimeTarget!==null)out.uptimeTarget=optionalFloat(row.uptimeTarget,decode);
 if(Object.hasOwn(row,"staffing")&&row.staffing!==null)out.staffing=quantity(row.staffing,decode);
 return out;
}

function optionEvaluation(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON OptionEvaluation"),out={...row};
 if(Object.hasOwn(row,"scores")&&row.scores!==null)out.scores=architectProgramArtifactGuardArray(row.scores,"Program JSON OptionEvaluation.scores").map(value=>float(value,decode));
 if(Object.hasOwn(row,"weightedScore")&&row.weightedScore!==null)out.weightedScore=optionalFloat(row.weightedScore,decode);
 if(Object.hasOwn(row,"costEstimate")&&row.costEstimate!==null)out.costEstimate=optionalFloat(row.costEstimate,decode);
 return out;
}

function performanceCriterion(value:unknown,decode:boolean):unknown {
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

function priorityRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON PriorityRecord"),out={...row};
 if(Object.hasOwn(row,"weight")&&row.weight!==null)out.weight=optionalFloat(row.weight,decode);
 if(Object.hasOwn(row,"score")&&row.score!==null)out.score=optionalFloat(row.score,decode);
 return out;
}

function quantityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON QuantityRequirement"),out={...row};
 if(Object.hasOwn(row,"tolerancePercent")&&row.tolerancePercent!==null)out.tolerancePercent=optionalFloat(row.tolerancePercent,decode);
 if(Object.hasOwn(row,"peakFactor")&&row.peakFactor!==null)out.peakFactor=optionalFloat(row.peakFactor,decode);
 if(Object.hasOwn(row,"growthFactor")&&row.growthFactor!==null)out.growthFactor=optionalFloat(row.growthFactor,decode);
 if(Object.hasOwn(row,"unitCost")&&row.unitCost!==null)out.unitCost=optionalFloat(row.unitCost,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

function relationship(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Relationship"),out={...row};
 if(Object.hasOwn(row,"strength")&&row.strength!==null)out.strength=optionalFloat(row.strength,decode);
 if(Object.hasOwn(row,"distanceConstraintM")&&row.distanceConstraintM!==null)out.distanceConstraintM=optionalFloat(row.distanceConstraintM,decode);
 return out;
}

function requirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Requirement"),out={...row};
 if(Object.hasOwn(row,"costEstimate")&&row.costEstimate!==null)out.costEstimate=optionalFloat(row.costEstimate,decode);
 return out;
}

function resource(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Resource"),out={...row};
 if(Object.hasOwn(row,"costPerUnit")&&row.costPerUnit!==null)out.costPerUnit=optionalFloat(row.costPerUnit,decode);
 if(Object.hasOwn(row,"sharingRatio")&&row.sharingRatio!==null)out.sharingRatio=optionalFloat(row.sharingRatio,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

function risk(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Risk"),out={...row};
 if(Object.hasOwn(row,"riskScore")&&row.riskScore!==null)out.riskScore=optionalFloat(row.riskScore,decode);
 return out;
}

function scenario(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Scenario"),out={...row};
 if(Object.hasOwn(row,"probability")&&row.probability!==null)out.probability=optionalFloat(row.probability,decode);
 if(Object.hasOwn(row,"costDelta")&&row.costDelta!==null)out.costDelta=optionalFloat(row.costDelta,decode);
 if(Object.hasOwn(row,"areaDelta")&&row.areaDelta!==null)out.areaDelta=optionalFloat(row.areaDelta,decode);
 if(Object.hasOwn(row,"headcountDelta")&&row.headcountDelta!==null)out.headcountDelta=optionalFloat(row.headcountDelta,decode);
 return out;
}

function siteContext(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON SiteContext"),out={...row};
 if(Object.hasOwn(row,"latitude")&&row.latitude!==null)out.latitude=optionalFloat(row.latitude,decode);
 if(Object.hasOwn(row,"longitude")&&row.longitude!==null)out.longitude=optionalFloat(row.longitude,decode);
 if(Object.hasOwn(row,"elevationM")&&row.elevationM!==null)out.elevationM=optionalFloat(row.elevationM,decode);
 if(Object.hasOwn(row,"maxHeightM")&&row.maxHeightM!==null)out.maxHeightM=optionalFloat(row.maxHeightM,decode);
 if(Object.hasOwn(row,"maxCoverage")&&row.maxCoverage!==null)out.maxCoverage=optionalFloat(row.maxCoverage,decode);
 return out;
}

function statusRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON StatusRecord"),out={...row};
 if(Object.hasOwn(row,"progressPercent")&&row.progressPercent!==null)out.progressPercent=optionalFloat(row.progressPercent,decode);
 return out;
}

function storageRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON StorageRequirement"),out={...row};
 if(Object.hasOwn(row,"volumeM3")&&row.volumeM3!==null)out.volumeM3=optionalFloat(row.volumeM3,decode);
 if(Object.hasOwn(row,"weightKg")&&row.weightKg!==null)out.weightKg=optionalFloat(row.weightKg,decode);
 if(Object.hasOwn(row,"growthAllowance")&&row.growthAllowance!==null)out.growthAllowance=optionalFloat(row.growthAllowance,decode);
 if(Object.hasOwn(row,"quantity")&&row.quantity!==null)out.quantity=quantity(row.quantity,decode);
 return out;
}

function survey(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Survey"),out={...row};
 if(Object.hasOwn(row,"responseRate")&&row.responseRate!==null)out.responseRate=optionalFloat(row.responseRate,decode);
 return out;
}

function sustainabilityRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON SustainabilityRequirement"),out={...row};
 if(Object.hasOwn(row,"baseline")&&row.baseline!==null)out.baseline=optionalFloat(row.baseline,decode);
 if(Object.hasOwn(row,"targetValue")&&row.targetValue!==null)out.targetValue=optionalFloat(row.targetValue,decode);
 if(Object.hasOwn(row,"embodiedCarbon")&&row.embodiedCarbon!==null)out.embodiedCarbon=optionalFloat(row.embodiedCarbon,decode);
 if(Object.hasOwn(row,"operationalCarbon")&&row.operationalCarbon!==null)out.operationalCarbon=optionalFloat(row.operationalCarbon,decode);
 return out;
}

function wayfindingRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON WayfindingRequirement"),out={...row};
 if(Object.hasOwn(row,"maximumSignageDistanceM")&&row.maximumSignageDistanceM!==null)out.maximumSignageDistanceM=optionalFloat(row.maximumSignageDistanceM,decode);
 return out;
}

function workshop(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Workshop"),out={...row};
 if(Object.hasOwn(row,"budget")&&row.budget!==null)out.budget=optionalFloat(row.budget,decode);
 return out;
}

function activity(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Activity"),out={...row};
 if(Object.hasOwn(row,"participants")&&row.participants!==null)out.participants=quantity(row.participants,decode);
 return out;
}

function functionRow(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON Function"),out={...row};
 if(Object.hasOwn(row,"staffing")&&row.staffing!==null)out.staffing=quantity(row.staffing,decode);
 return out;
}

function programElement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON ProgramElement"),out={...row};
 if(Object.hasOwn(row,"area")&&row.area!==null)out.area=quantity(row.area,decode);
 if(Object.hasOwn(row,"volume")&&row.volume!==null)out.volume=quantity(row.volume,decode);
 if(Object.hasOwn(row,"height")&&row.height!==null)out.height=quantity(row.height,decode);
 if(Object.hasOwn(row,"occupancy")&&row.occupancy!==null)out.occupancy=quantity(row.occupancy,decode);
 return out;
}

function organizationalRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON OrganizationalRequirement"),out={...row};
 if(Object.hasOwn(row,"headcount")&&row.headcount!==null)out.headcount=quantity(row.headcount,decode);
 return out;
}

function serviceRequirement(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON ServiceRequirement"),out={...row};
 if(Object.hasOwn(row,"capacity")&&row.capacity!==null)out.capacity=quantity(row.capacity,decode);
 if(Object.hasOwn(row,"staffing")&&row.staffing!==null)out.staffing=quantity(row.staffing,decode);
 return out;
}

function analysisRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON AnalysisRecord"),out={...row};
 if(Object.hasOwn(row,"durationMs")&&row.durationMs!==null)out.durationMs=row.durationMs===null?null:unsigned(row.durationMs,decode);
 return out;
}

function searchFilter(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON SearchFilter"),out={...row};
 if(Object.hasOwn(row,"useCount")&&row.useCount!==null)out.useCount=unsigned(row.useCount,decode);
 return out;
}

function templateRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON TemplateRecord"),out={...row};
 if(Object.hasOwn(row,"usageCount")&&row.usageCount!==null)out.usageCount=unsigned(row.usageCount,decode);
 return out;
}

function knowledgeRecord(value:unknown,decode:boolean):unknown {
 const row=architectProgramArtifactGuardObject(value,"Program JSON KnowledgeRecord"),out={...row};
 if(Object.hasOwn(row,"usageCount")&&row.usageCount!==null)out.usageCount=unsigned(row.usageCount,decode);
 return out;
}

function program(value:unknown,decode:boolean):Record<string,unknown> {
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

function delta(value:unknown,convert:(value:unknown,decode:boolean)=>unknown,decode:boolean):unknown{
 const row=architectProgramArtifactGuardExactObject(value,"Program JSON delta",["added","removed","patched","reordered"]);
 return{...row,added:rows(row.added,convert,decode),patched:architectProgramArtifactGuardArray(row.patched,"Program JSON patched").map(value=>{const entry=architectProgramArtifactGuardExactObject(value,"Program JSON patch",["id","patch"]);return{...entry,patch:convert===header?entry.patch:convert(entry.patch,decode)}})};
}
function diff(value:unknown,decode:boolean):Record<string,unknown>{
 const row=architectProgramArtifactGuardExactObject(value,"Program JSON diff",["artifact",...PROGRAM_ARTIFACT_FIELDS]),out={...row};
 if(row.artifact!==null)out.artifact=program(row.artifact,decode);
 if(row.accessibility!==null)out.accessibility=delta(row.accessibility,accessibilityRequirement,decode);
 if(row.adjacencies!==null)out.adjacencies=delta(row.adjacencies,adjacency,decode);
 if(row.benchmarksPayload!==null)out.benchmarksPayload=rows(row.benchmarksPayload,benchmarkRecord,decode);
 if(row.changes!==null)out.changes=delta(row.changes,changeRecord,decode);
 if(row.conflicts!==null)out.conflicts=delta(row.conflicts,conflict,decode);
 if(row.costs!==null)out.costs=delta(row.costs,costRequirement,decode);
 if(row.decisions!==null)out.decisions=delta(row.decisions,decision,decode);
 if(row.environmental!==null)out.environmental=delta(row.environmental,environmentalRequirement,decode);
 if(row.equipment!==null)out.equipment=delta(row.equipment,equipment,decode);
 if(row.flexibility!==null)out.flexibility=delta(row.flexibility,flexibilityRequirement,decode);
 if(row.flows!==null)out.flows=delta(row.flows,flowRequirement,decode);
 if(row.growth!==null)out.growth=delta(row.growth,growthPlan,decode);
 if(row.infrastructure!==null)out.infrastructure=delta(row.infrastructure,infrastructureRequirement,decode);
 if(row.operations!==null)out.operations=delta(row.operations,operationalRequirement,decode);
 if(row.options!==null)out.options=delta(row.options,optionEvaluation,decode);
 if(row.performance!==null)out.performance=delta(row.performance,performanceCriterion,decode);
 if(row.priorities!==null)out.priorities=delta(row.priorities,priorityRecord,decode);
 if(row.quantities!==null)out.quantities=delta(row.quantities,quantityRequirement,decode);
 if(row.relationships!==null)out.relationships=delta(row.relationships,relationship,decode);
 if(row.requirements!==null)out.requirements=delta(row.requirements,requirement,decode);
 if(row.resources!==null)out.resources=delta(row.resources,resource,decode);
 if(row.risks!==null)out.risks=delta(row.risks,risk,decode);
 if(row.scenarios!==null)out.scenarios=delta(row.scenarios,scenario,decode);
 if(row.siteContext!==null)out.siteContext=delta(row.siteContext,siteContext,decode);
 if(row.statusRecords!==null)out.statusRecords=delta(row.statusRecords,statusRecord,decode);
 if(row.storage!==null)out.storage=delta(row.storage,storageRequirement,decode);
 if(row.surveys!==null)out.surveys=delta(row.surveys,survey,decode);
 if(row.sustainability!==null)out.sustainability=delta(row.sustainability,sustainabilityRequirement,decode);
 if(row.wayfinding!==null)out.wayfinding=delta(row.wayfinding,wayfindingRequirement,decode);
 if(row.workshops!==null)out.workshops=delta(row.workshops,workshop,decode);
 if(row.activities!==null)out.activities=delta(row.activities,activity,decode);
 if(row.functions!==null)out.functions=delta(row.functions,functionRow,decode);
 if(row.elements!==null)out.elements=delta(row.elements,programElement,decode);
 if(row.organizational!==null)out.organizational=delta(row.organizational,organizationalRequirement,decode);
 if(row.services!==null)out.services=delta(row.services,serviceRequirement,decode);
 if(row.analyses!==null)out.analyses=delta(row.analyses,analysisRecord,decode);
 if(row.searchFilters!==null)out.searchFilters=delta(row.searchFilters,searchFilter,decode);
 if(row.templates!==null)out.templates=delta(row.templates,templateRecord,decode);
 if(row.knowledgePayload!==null)out.knowledgePayload=rows(row.knowledgePayload,knowledgeRecord,decode);
 if(row.approvals!==null)out.approvals=delta(row.approvals,header,decode);
 if(row.meetings!==null)out.meetings=delta(row.meetings,header,decode);
 if(row.assumptions!==null)out.assumptions=delta(row.assumptions,header,decode);
 if(row.constraints!==null)out.constraints=delta(row.constraints,header,decode);
 if(row.complianceRecords!==null)out.complianceRecords=delta(row.complianceRecords,header,decode);
 if(row.issues!==null)out.issues=delta(row.issues,header,decode);
 if(row.collaboration!==null)out.collaboration=delta(row.collaboration,header,decode);
 if(row.reports!==null)out.reports=delta(row.reports,header,decode);
 if(row.quality!==null)out.quality=delta(row.quality,header,decode);
 if(row.artifacts!==null)out.artifacts=delta(row.artifacts,header,decode);
 if(row.validations!==null)out.validations=delta(row.validations,header,decode);
 if(row.delivery!==null)out.delivery=delta(row.delivery,header,decode);
 if(row.resilience!==null)out.resilience=delta(row.resilience,header,decode);
 if(row.schedules!==null)out.schedules=delta(row.schedules,header,decode);
 if(row.information!==null)out.information=delta(row.information,header,decode);
 if(row.communication!==null)out.communication=delta(row.communication,header,decode);
 if(row.safety!==null)out.safety=delta(row.safety,header,decode);
 if(row.security!==null)out.security=delta(row.security,header,decode);
 if(row.regulatory!==null)out.regulatory=delta(row.regulatory,header,decode);
 if(row.privacy!==null)out.privacy=delta(row.privacy,header,decode);
 if(row.humanFactors!==null)out.humanFactors=delta(row.humanFactors,header,decode);
 if(row.accessRules!==null)out.accessRules=delta(row.accessRules,header,decode);
 if(row.processes!==null)out.processes=delta(row.processes,header,decode);
 if(row.users!==null)out.users=delta(row.users,header,decode);
 if(row.stakeholders!==null)out.stakeholders=delta(row.stakeholders,header,decode);
 if(row.auditEvents!==null)out.auditEvents=delta(row.auditEvents,header,decode);
 return out;
}
/** 🧬️ Decode each literal diff register's declared JSON numeric transport. */
export function programDiffFromJson(value:unknown):ProgramDiff{return parseProgramDiff(diff(value,true))}
/** 🔺️ Encode each literal canonical diff register without numeric coercion. */
export function programDiffToJson(value:ProgramDiff):Readonly<Record<string,unknown>>{return diff(parseProgramDiff(value),false)}
