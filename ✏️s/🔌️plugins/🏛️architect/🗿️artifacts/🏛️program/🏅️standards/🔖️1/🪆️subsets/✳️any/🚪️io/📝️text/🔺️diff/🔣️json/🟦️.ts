import {architectProgramArtifactGuardExactObject,architectProgramArtifactGuardArray} from "../../../../🧬️schema/🟦️.ts";
import {parseProgramDiff,PROGRAM_DIFF_FIELDS,type ProgramDiff} from "../../../../🧬️schema/🔺️diff/🟦️.ts";
import {header,rows,accessibilityRequirement,adjacency,benchmarkRecord,changeRecord,conflict,costRequirement,decision,environmentalRequirement,equipment,flexibilityRequirement,flowRequirement,growthPlan,infrastructureRequirement,operationalRequirement,optionEvaluation,performanceCriterion,priorityRecord,quantityRequirement,relationship,requirement,resource,risk,scenario,siteContext,statusRecord,storageRequirement,survey,sustainabilityRequirement,wayfindingRequirement,workshop,activity,functionRow,programElement,organizationalRequirement,serviceRequirement,analysisRecord,searchFilter,templateRecord,knowledgeRecord} from "../../📸️snapshot/🔣️json/🟦️.ts";


function delta(value:unknown,convert:(value:unknown,decode:boolean)=>unknown,decode:boolean):unknown{
 const row=architectProgramArtifactGuardExactObject(value,"Program JSON delta",["added","removed","patched","reordered"]);
 return{...row,added:rows(row.added,convert,decode),patched:architectProgramArtifactGuardArray(row.patched,"Program JSON patched").map(value=>{const entry=architectProgramArtifactGuardExactObject(value,"Program JSON patch",["id","patch"]);return{...entry,patch:convert===header?entry.patch:convert(entry.patch,decode)}})};
}

function diff(value:unknown,decode:boolean):Record<string,unknown>{
 const row=architectProgramArtifactGuardExactObject(value,"Program JSON diff",PROGRAM_DIFF_FIELDS),out={...row};
 if(row.accessibility!==null)out.accessibility=delta(row.accessibility,accessibilityRequirement,decode);
 if(row.adjacencies!==null)out.adjacencies=delta(row.adjacencies,adjacency,decode);
 if(row.benchmarks!==null)out.benchmarks=delta(row.benchmarks,benchmarkRecord,decode);
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
 if(row.knowledge!==null)out.knowledge=delta(row.knowledge,knowledgeRecord,decode);
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
