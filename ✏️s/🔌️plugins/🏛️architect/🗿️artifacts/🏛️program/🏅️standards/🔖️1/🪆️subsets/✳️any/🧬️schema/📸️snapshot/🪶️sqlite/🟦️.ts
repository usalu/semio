/** 🏛️ Literal Architect entities and relationships under the owned semantic schema. */
import * as model from "../../🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteTables, artifactSqliteCheckpoint, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { type SqliteDatabase, type SqliteRow, type SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { binary64Value, type Binary64 } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { NativeDecodeControl } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import { PROGRAM_SQLITE_SCHEMA } from "./🗄️schema/🟦️.ts";

type Cells=readonly SqliteValue[];
const fail=(message:string):never=>{throw new Error("Architect SQLite "+message)};
const text=(value:string):Cells=>typeof value==="string"?[value]:fail("requires TEXT");
const boolean=(value:boolean):Cells=>typeof value==="boolean"?[value?1n:0n]:fail("requires Boolean");
const uint32=(value:number):Cells=>[BigInt(model.architectProgramArtifactGuardInteger(value,"unsigned32",{minimum:0,maximum:4294967295}))];
const uint64=(value:bigint):Cells=>{const word=model.architectProgramArtifactGuardUnsigned64(value,"unsigned64");return[word.toString(),BigInt.asIntN(64,word)]};
function float(value:Binary64):Cells{const word=model.architectProgramArtifactGuardBinary64(value,"binary64").bits,query=binary64Value(value),kind=Number.isNaN(query)?"nan":query===Infinity?"positiveInfinity":query===-Infinity?"negativeInfinity":"finite";return[Number.isNaN(query)?null:query,BigInt.asIntN(64,word),kind]}
const optional=<T>(value:T|null,width:number,encode:(value:T)=>Cells):Cells=>value===null?Array<null>(width).fill(null):encode(value);
const textField=(value:model.TextField):Cells=>[...text(value.text),...optional(value.format??null,1,text)];
const optionalTextField=(value:model.TextField|null):Cells=>value===null?[0n,null,null]:[1n,...textField(value)];
const note=(value:model.TaggedNote):Cells=>[...text(value.tag),...text(value.text)];
const symbol=<T extends string>(value:T,members:readonly T[]):Cells=>text(model.architectProgramArtifactGuardMember(value,"enum",members));
const timestamp=(value:model.TimestampMeta):Cells=>[...text(value.created),...text(value.updated),...optional(value.createdBy??null,1,text),...optional(value.updatedBy??null,1,text)];
class Cursor{
 private index:number;
 constructor(private readonly row:SqliteRow,index:number=1){this.index=index}
 next():SqliteValue{if(this.index>=this.row.values.length)return fail("scalar is absent");return this.row.values[this.index++]!}
 text():string{const value=this.next();return typeof value==="string"?value:fail("requires TEXT")}
 integer():bigint{const value=this.next();return typeof value==="bigint"?value:fail("requires INTEGER")}
 boolean():boolean{const value=this.integer();return value===0n?false:value===1n?true:fail("noncanonical Boolean")}
 uint32():number{const value=this.integer();return value>=0n&&value<=4294967295n?Number(value):fail("unsigned32 exceeds domain")}
 uint64():bigint{const decimal=this.text(),word=BigInt.asUintN(64,this.integer());return decimal===word.toString()?word:fail("unsigned64 decimal and word differ")}
 float():Binary64{const query=this.next(),word=BigInt.asUintN(64,this.integer()),kind=this.text(),value={bits:word},number=binary64Value(value),expected=Number.isNaN(number)?"nan":number===Infinity?"positiveInfinity":number===-Infinity?"negativeInfinity":"finite";if(kind!==expected)return fail("binary64 class differs");if(typeof query==="bigint"){if(!Number.isFinite(number)||!Number.isInteger(number)||BigInt(number)!==query)return fail("binary64 query INTEGER differs")}else if(typeof query==="number"){if(Number.isNaN(number)||query!==number)return fail("binary64 query REAL differs")}else if(query!==null||!Number.isNaN(number))return fail("binary64 query differs");return value}
 optional<T>(width:number,decode:()=>T):T|null{const end=this.index+width;if(end>this.row.values.length)return fail("optional scalar is absent");if(this.row.values.slice(this.index,end).every(value=>value===null)){this.index=end;return null}return decode()}
 textField():model.TextField{const value={text:this.text()},format=this.optional(1,()=>this.text());return format===null?value:{...value,format}}
 optionalTextField():model.TextField|null{if(this.boolean())return this.textField();if(this.next()!==null||this.next()!==null)return fail("absent text owns a payload");return null}
 note():model.TaggedNote{return{tag:this.text(),text:this.text()}}
 symbol<T extends string>(members:readonly T[]):T{return model.architectProgramArtifactGuardMember(this.text(),"enum",members)}
 timestamp():model.TimestampMeta{const created=this.text(),updated=this.text(),createdBy=this.optional(1,()=>this.text()),updatedBy=this.optional(1,()=>this.text());return{created,updated,...createdBy===null?{}:{createdBy},...updatedBy===null?{}:{updatedBy}}}
 end():void{if(this.index!==this.row.values.length)fail("unused scalar columns")}
}
async function ordered<T>(p:ArtifactSqliteProjection,table:string,parent:bigint,values:readonly T[],encode:(value:T)=>Cells):Promise<void>{for(let ordinal=0;ordinal<values.length;ordinal++)await p.insert(table,[parent,BigInt(ordinal),...encode(values[ordinal]!)])}
class Reader{
 private readonly used=new Set<SqliteRow>();
 private readonly bodies=new Map<string,Map<bigint,SqliteRow>>();
 private readonly groups=new Map<string,Map<bigint,SqliteRow[]>>();
 private headers:Map<string,SqliteRow[]>|undefined;
 constructor(private readonly tables:ReadonlyMap<string,readonly SqliteRow[]>,readonly control:NativeDecodeControl){}
 rows(table:string):readonly SqliteRow[]{return this.tables.get(table)??fail("table is absent")}
 private shape(row:SqliteRow,width:number):void{if(row.rowid<=0n||row.values.length!==width||row.values[0]!==row.rowid)fail("row shape or identity differs")}
 async use(row:SqliteRow):Promise<void>{await this.control.charge(96);await this.control.step();if(this.used.has(row))fail("entity is multiply owned");this.used.add(row)}
 async singleton(table:string,width:number):Promise<SqliteRow>{const rows=this.rows(table);if(rows.length!==1||rows[0]!.rowid!==1n)fail("singleton shape differs");this.shape(rows[0]!,width);await this.use(rows[0]!);return rows[0]!}
 async body(table:string,parent:bigint,width:number,required=true):Promise<SqliteRow|null>{let index=this.bodies.get(table);if(index===undefined){const rows=this.rows(table);await this.control.admitSlots(rows.length,128);index=new Map();for(const row of rows){await this.control.step();this.shape(row,width);if(index.has(row.rowid))fail("body identity is repeated");index.set(row.rowid,row)}this.bodies.set(table,index)}const row=index.get(parent);if(!row){if(required)fail("typed body is absent");return null}this.shape(row,width);index.delete(parent);await this.use(row);return row}
 async list<T>(table:string,parent:bigint,width:number,decode:(cursor:Cursor)=>T):Promise<T[]>{let groups=this.groups.get(table);if(groups===undefined){const rows=this.rows(table);await this.control.admitSlots(rows.length,256);groups=new Map();for(const row of rows){await this.control.step();this.shape(row,width+3);const cursor=new Cursor(row),owner=cursor.integer(),ordinal=cursor.integer();if(ordinal<0n||ordinal>BigInt(Number.MAX_SAFE_INTEGER))fail("relationship ordinal exceeds domain");let group=groups.get(owner);if(!group){group=[];groups.set(owner,group)}const index=Number(ordinal);if(index>rows.length||group[index])fail("relationship ordinal differs");group[index]=row}this.groups.set(table,groups)}const rows=groups.get(parent)??[];groups.delete(parent);await this.control.admitSlots(rows.length,32);const result:T[]=[];for(let ordinal=0;ordinal<rows.length;ordinal++){const row=rows[ordinal];if(!row)fail("relationship ordinals are not dense");await this.use(row!);const cursor=new Cursor(row!,3);result.push(decode(cursor));cursor.end()}return result}
 async register<T>(name:string,restore:(reader:Reader,row:SqliteRow,header:model.EntityHeader)=>Promise<T>):Promise<T[]>{if(this.headers===undefined){const rows=this.rows("architect_entity_header");await this.control.admitSlots(rows.length,256);this.headers=new Map();for(const row of rows){await this.control.step();this.shape(row,17);const cursor=new Cursor(row),document=cursor.integer(),register=cursor.text(),ordinal=cursor.integer();if(document!==1n||ordinal<0n||ordinal>BigInt(rows.length))fail("header owner or ordinal differs");let group=this.headers.get(register);if(!group){group=[];this.headers.set(register,group)}if(group[Number(ordinal)])fail("header ordinal is repeated");group[Number(ordinal)]=row}}const rows=this.headers.get(name)??[];this.headers.delete(name);await this.control.admitSlots(rows.length,64);const result:T[]=[];for(const row of rows){if(!row)fail("register ordinals are not dense");await this.use(row);result.push(await restore(this,row,await this.header(row)))}return result}
 async header(row:SqliteRow):Promise<model.EntityHeader>{const c=new Cursor(row,4),id=c.text(),name=c.text(),description=c.optionalTextField(),status=c.symbol(LifecycleStatus),priority=c.symbol(Priority),ownerId=c.optional(1,()=>c.text()),authorityId=c.optional(1,()=>c.text()),timestamps=c.timestamp();c.end();return{id,name,...description===null?{}:{description},status,priority,ownership:{ownerId,authorityId,consultantIds:await this.list("architect_entity_consultant",row.rowid,1,c=>c.text()),participantIds:await this.list("architect_entity_participant",row.rowid,1,c=>c.text())},timestamps,tags:await this.list("architect_entity_tag",row.rowid,1,c=>c.text()),notes:await this.list("architect_entity_note",row.rowid,2,c=>c.note())}}
 async quantity(entity:bigint,slot:string,table:string):Promise<model.QuantitySpec>{const edge=await this.body(table,entity,2),link=new Cursor(edge!),row=await this.body("architect_quantity_spec",link.integer(),25),c=new Cursor(row!);link.end();if(c.integer()!==entity||c.text()!==slot)fail("quantity belongs to another entity or slot");const unit=c.text(),min=c.optional(3,()=>c.float()),max=c.optional(3,()=>c.float()),target=c.optional(3,()=>c.float()),current=c.optional(3,()=>c.float()),forecast=c.optional(3,()=>c.float()),peak=c.optional(3,()=>c.float()),average=c.optional(3,()=>c.float());c.end();return{unit,...min===null?{}:{min},...max===null?{}:{max},...target===null?{}:{target},...current===null?{}:{current},...forecast===null?{}:{forecast},...peak===null?{}:{peak},...average===null?{}:{average}}}
 async finish():Promise<void>{for(const rows of this.tables.values())for(const row of rows){await this.control.step();if(!this.used.has(row))fail("contains unowned entities")}await this.control.checkpoint()}
}
async function header(p:ArtifactSqliteProjection,register:string,ordinal:number,v:model.EntityHeader):Promise<bigint>{const id=await p.insert("architect_entity_header",[1n,register,BigInt(ordinal),...text(v.id),...text(v.name),...optionalTextField(v.description??null),...symbol(v.status,LifecycleStatus),...symbol(v.priority,Priority),...optional(v.ownership.ownerId,1,text),...optional(v.ownership.authorityId,1,text),...timestamp(v.timestamps)]);await ordered(p,"architect_entity_consultant",id,v.ownership.consultantIds,text);await ordered(p,"architect_entity_participant",id,v.ownership.participantIds,text);await ordered(p,"architect_entity_tag",id,v.tags,text);await ordered(p,"architect_entity_note",id,v.notes,note);return id}
async function quantity(p:ArtifactSqliteProjection,entity:bigint,slot:string,table:string,v:model.QuantitySpec):Promise<void>{const id=await p.insert("architect_quantity_spec",[entity,slot,...text(v.unit),...optional(v.min??null,3,float),...optional(v.max??null,3,float),...optional(v.target??null,3,float),...optional(v.current??null,3,float),...optional(v.forecast??null,3,float),...optional(v.peak??null,3,float),...optional(v.average??null,3,float)]);await p.insert(table,[id],entity)}
const Priority=["mandatory","essential","preferred","optional","deferred","prohibited"] as const;
const LifecycleStatus=["draft","proposed","underReview","validated","approved","rejected","deferred","superseded","archived","open","closed","atRisk","blocked","inProgress","complete"] as const;
const InfluenceLevel=["low","medium","high","critical"] as const;
const EngagementLevel=["unaware","resistant","neutral","supportive","leading"] as const;
const UserCategory=["primary","secondary","occasional","service","visitor","staff","public"] as const;
const ProgramElementKind=["building","campus","floor","zone","room","suite","department","system","circulation","support","outdoor","furnitureGroup","other"] as const;
const FunctionKind=["primary","secondary","support","administrative","service","technical","public","private","shared","restricted","temporary","future","operational","circulation"] as const;
const FlowKind=["people","material","information","service","equipment","waste","emergency","vehicle"] as const;
const PrivacyKind=["public","semiPublic","semiPrivate","private","confidential","restricted","anonymous"] as const;
const SafetyDomain=["lifeSafety","occupationalHealth","fire","structural","electrical","chemical","radiation","ergonomics","biological","environmental"] as const;
const SecurityControlKind=["accessControl","surveillance","perimeter","cyber","personnel","information","physical","procedural","screening","keyManagement"] as const;
const StorageClass=["general","secure","climateControlled","hazardous","archive","mobile","fixed","shared","coldChain","flammable"] as const;
const EnvironmentalParameter=["temperature","humidity","airQuality","lighting","acoustics","ventilation","radiation","vibration","pressure","iaq"] as const;
const HumanFactorAspect=["ergonomics","cognition","sensory","social","cultural","behavioral","physical","psychological","fatigue","stress"] as const;
const AccessMode=["unrestricted","cardControlled","biometric","keyed","escortRequired","timeRestricted","roleBased","emergencyOnly"] as const;
const RelationshipKind=["contains","serves","supports","dependsOn","conflictsWith","equivalentTo","adjacentTo","feeds","receives","controls","monitors","functional","operational","organizational","user","service","information","access","security","supervision","communication","dependency","sequential","sharedResource"] as const;
const AdjacencyKind=["required","preferred","optional","prohibited"] as const;
const ConnectionKind=["direct","indirect","controlled","sharedAccess","none"] as const;
const SeparationKind=["acoustic","visual","security","olfactory","thermal","fire","hygienic","circulation","operational","infectionControl"] as const;
const FlowDirection=["oneWay","twoWay","bidirectionalPeak","restricted"] as const;
const AccessLevel=["public","restricted","controlled","private","secure","emergencyOnly"] as const;
const RiskLevel=["negligible","low","medium","high","critical"] as const;
const ConflictKind=["adjacency","capacity","schedule","budget","regulatory","operational","environmental","security","priority"] as const;
const RequirementKind=["functional","spatial","performance","regulatory","operational","technical","aesthetic","sustainability"] as const;
const ValidationStatus=["pending","passed","failed","waived","deferred"] as const;
const AnalysisKind=["gap","conflict","dependency","capacity","demand","utilization","workflow","risk","cost","scenario","sensitivity","impact","trend","requirementComparison","requirementClustering","requirementFiltering","requirementSorting","requirementScoring","requirementWeighting","relationshipAnalysis"] as const;
const ReportKind=["executiveSummary","programOverview","stakeholderSummary","requirementsMatrix","adjacencyMatrix","gapAnalysis","riskRegister","decisionLog","validationSummary","recommendation","userSummary","functionalSummary","capacitySummary","workflowSummary","complianceSummary","costSummary","scheduleSummary","changeSummary","openIssueSummary","prioritySummary","scenarioSummary"] as const;
const IssueSeverity=["cosmetic","minor","major","critical","blocker"] as const;
const AuditAction=["created","updated","deleted","reviewed","approved","rejected","exported","imported","merged","archived"] as const;
const CostBasis=["capital","operational","lifecycle","replacement","maintenance"] as const;
const DeliveryPhase=["concept","schematic","designDevelopment","constructionDocuments","procurement","construction","commissioning","occupancy"] as const;
const TraceKind=["objectiveToRequirement","stakeholderToRequirement","userToActivity","activityToFunction","functionToProgramElement","requirementToDecision","requirementToRisk","requirementToStandard","requirementToValidation","requirementToApproval","requirementToChange","equipmentToActivity","processToResource","constraintToImpact","scenarioToDecision","issueToAction","actionToOwner","decisionToOutcome","versionToChange","fullAuditTrail"] as const;
const trace=(value:model.TraceLink):Cells=>[...text(value.id),...text(value.fromId),...text(value.toId),...symbol(value.kind,TraceKind),...optional(value.label??null,1,text)];
function readTrace(c:Cursor):model.TraceLink{const id=c.text(),fromId=c.text(),toId=c.text(),kind=c.symbol(TraceKind),label=c.optional(1,()=>c.text());return{id,fromId,toId,kind,...label===null?{}:{label}}}
async function projectApprovalRecord(p:ArtifactSqliteProjection,id:bigint,v:model.ApprovalRecord):Promise<void>{
 await p.insert("architect_approval_record",[...text(v.approvalType),...text(v.subjectId),...optional(v.approvalDate,1,text),...symbol(v.approvalStatus,LifecycleStatus),...optional(v.expiryDate,1,text),...optional(v.relatedDecisionId,1,text),...optional(v.relatedChangeId,1,text),...optional(v.signatureMethod,1,text),...optionalTextField(v.rejectionReason),...optional(v.resubmissionDate,1,text),...optional(v.workflowStep,1,text),...optional(v.version,1,text),...optional(v.auditTrailRef,1,text)],id);
 await ordered(p,"architect_approval_approver",id,v.approverIds,text);
 await ordered(p,"architect_approval_condition",id,v.conditions,text);
 await ordered(p,"architect_approval_delegation",id,v.delegationChain,text);
 await ordered(p,"architect_approval_evidence_ref",id,v.evidenceRefs,text);
 await ordered(p,"architect_approval_authority_basis",id,v.authorityBasis,text);
 await ordered(p,"architect_approval_notification",id,v.notificationList,text);
}
async function restoreApprovalRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ApprovalRecord>{
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
async function projectMeetingRecord(p:ArtifactSqliteProjection,id:bigint,v:model.MeetingRecord):Promise<void>{
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
async function restoreMeetingRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.MeetingRecord>{
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
async function projectAssumption(p:ArtifactSqliteProjection,id:bigint,v:model.Assumption):Promise<void>{
 await p.insert("architect_assumption",[...textField(v.statement),...optionalTextField(v.basis),...optional(v.confidenceLevel,1,text),...optionalTextField(v.impactIfFalse),...symbol(v.validationStatus,ValidationStatus),...optional(v.validatedBy,1,text),...optional(v.validationDate,1,text),...optional(v.ownerId,1,text),...optional(v.reviewCycle,1,text),...optional(v.source,1,text),...optional(v.category,1,text),...optional(v.expirationDate,1,text)],id);
 await ordered(p,"architect_assumption_related_entity",id,v.relatedEntityIds,text);
 await ordered(p,"architect_assumption_dependency",id,v.dependencies,text);
 await ordered(p,"architect_assumption_mitigation",id,v.mitigation,text);
 await ordered(p,"architect_assumption_linked_requirement",id,v.linkedRequirementIds,text);
 await ordered(p,"architect_assumption_linked_risk",id,v.linkedRiskIds,text);
 await ordered(p,"architect_assumption_status_note",id,v.statusNotes,note);
 await ordered(p,"architect_assumption_artifact_ref",id,v.artifactRefs,text);
}
async function restoreAssumption(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Assumption>{
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
async function projectConstraintRecord(p:ArtifactSqliteProjection,id:bigint,v:model.ConstraintRecord):Promise<void>{
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
async function restoreConstraintRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ConstraintRecord>{
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
async function projectComplianceRecord(p:ArtifactSqliteProjection,id:bigint,v:model.ComplianceRecord):Promise<void>{
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
async function restoreComplianceRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ComplianceRecord>{
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
async function projectTemplateRecord(p:ArtifactSqliteProjection,id:bigint,v:model.TemplateRecord):Promise<void>{
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
async function restoreTemplateRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.TemplateRecord>{
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
async function projectKnowledgeRecord(p:ArtifactSqliteProjection,id:bigint,v:model.KnowledgeRecord):Promise<void>{
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
async function restoreKnowledgeRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.KnowledgeRecord>{
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
async function projectBenchmarkRecord(p:ArtifactSqliteProjection,id:bigint,v:model.BenchmarkRecord):Promise<void>{
 await p.insert("architect_benchmark_record",[...text(v.benchmarkName),...text(v.sector),...text(v.metric),...float(v.value),...text(v.unit),...optional(v.sampleSize,1,uint32),...optional(v.source,1,text),...optional(v.collectionYear,1,uint32),...optional(v.geography,1,text),...optional(v.buildingType,1,text),...optional(v.confidence,1,text),...optional(v.methodology,1,text),...optional(v.license,1,text),...optional(v.knowledgeId,1,text),...optional(v.lastVerified,1,text)],id);
 await ordered(p,"architect_benchmark_applicable_element_kind",id,v.applicableElementKinds,text);
 await ordered(p,"architect_benchmark_related_requirement",id,v.relatedRequirementIds,text);
 await ordered(p,"architect_benchmark_comparison_note",id,v.comparisonNotes,text);
 await ordered(p,"architect_benchmark_limitation",id,v.limitations,text);
}
async function restoreBenchmarkRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.BenchmarkRecord>{
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
async function projectIssue(p:ArtifactSqliteProjection,id:bigint,v:model.Issue):Promise<void>{
 await p.insert("architect_issue",[...text(v.issueType),...textField(v.summary),...textField(v.issueDescription),...symbol(v.severity,IssueSeverity),...symbol(v.issuePriority,Priority),...optional(v.reporterId,1,text),...optional(v.assigneeId,1,text),...optionalTextField(v.rootCause),...optionalTextField(v.resolution),...optionalTextField(v.workaround),...optional(v.dueDate,1,text),...optional(v.resolvedDate,1,text),...optional(v.decisionId,1,text),...optional(v.escalationLevel,1,text)],id);
 await ordered(p,"architect_issue_affected_entity",id,v.affectedEntityIds,text);
 await ordered(p,"architect_issue_related_conflict",id,v.relatedConflictIds,text);
 await ordered(p,"architect_issue_related_risk",id,v.relatedRiskIds,text);
 await ordered(p,"architect_issue_comment",id,v.comments,note);
 await ordered(p,"architect_issue_attachment",id,v.attachments,text);
}
async function restoreIssue(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Issue>{
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
async function projectStatusRecord(p:ArtifactSqliteProjection,id:bigint,v:model.StatusRecord):Promise<void>{
 await p.insert("architect_status_record",[...text(v.subjectId),...text(v.subjectKind),...symbol(v.recordStatus,LifecycleStatus),...optional(v.previousStatus,1,value=>symbol(value,LifecycleStatus)),...optional(v.changedBy,1,text),...optional(v.changedAt,1,text),...optionalTextField(v.reason),...optional(v.dueDate,1,text),...optional(v.progressPercent,3,float),...optional(v.health,1,text),...optional(v.escalationLevel,1,text),...optional(v.milestoneId,1,text),...optional(v.reportingPeriod,1,text)],id);
 await ordered(p,"architect_status_blocker",id,v.blockers,text);
 await ordered(p,"architect_status_next_action",id,v.nextActions,text);
 await ordered(p,"architect_status_related_issue",id,v.relatedIssueIds,text);
 await ordered(p,"architect_status_related_risk",id,v.relatedRiskIds,text);
 await ordered(p,"architect_status_note",id,v.statusNotes,note);
}
async function restoreStatusRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.StatusRecord>{
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
async function projectWorkshop(p:ArtifactSqliteProjection,id:bigint,v:model.Workshop):Promise<void>{
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
async function restoreWorkshop(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Workshop>{
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
async function projectSurvey(p:ArtifactSqliteProjection,id:bigint,v:model.Survey):Promise<void>{
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
async function restoreSurvey(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Survey>{
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
async function projectSearchFilter(p:ArtifactSqliteProjection,id:bigint,v:model.SearchFilter):Promise<void>{
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
async function restoreSearchFilter(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.SearchFilter>{
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
async function projectCollaborationRecord(p:ArtifactSqliteProjection,id:bigint,v:model.CollaborationRecord):Promise<void>{
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
async function restoreCollaborationRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.CollaborationRecord>{
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
async function projectAnalysisRecord(p:ArtifactSqliteProjection,id:bigint,v:model.AnalysisRecord):Promise<void>{
 await p.insert("architect_analysis_record",[...symbol(v.kind,AnalysisKind),...text(v.title),...textField(v.outputSummary),...optional(v.runBy,1,text),...optional(v.runAt,1,text),...optional(v.durationMs,2,uint64),...optional(v.toolVersion,1,text),...optional(v.scenarioId,1,text),...optional(v.reportId,1,text),...optional(v.confidence,1,text),...optional(v.rawResultRef,1,text)],id);
 await ordered(p,"architect_analysis_parameter",id,v.parameters,text);
 await ordered(p,"architect_analysis_input_entity",id,v.inputEntityIds,text);
 await ordered(p,"architect_analysis_finding",id,v.findings,text);
 await ordered(p,"architect_analysis_metric",id,v.metrics,text);
 await ordered(p,"architect_analysis_chart",id,v.charts,text);
 await ordered(p,"architect_analysis_limitation",id,v.limitations,text);
 await ordered(p,"architect_analysis_recommendation",id,v.recommendations,text);
}
async function restoreAnalysisRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.AnalysisRecord>{
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
async function projectReportRecord(p:ArtifactSqliteProjection,id:bigint,v:model.ReportRecord):Promise<void>{
 await p.insert("architect_report_record",[...symbol(v.kind,ReportKind),...text(v.title),...optional(v.generatedAt,1,text),...optional(v.generatedBy,1,text),...optional(v.format,1,text),...optional(v.fileRef,1,text),...symbol(v.approvalStatus,ValidationStatus),...optional(v.approverId,1,text),...text(v.version),...optional(v.templateId,1,text),...optional(v.confidentiality,1,text),...optional(v.expiryDate,1,text)],id);
 await ordered(p,"architect_report_audience",id,v.audience,text);
 await ordered(p,"architect_report_section",id,v.sections,text);
 await ordered(p,"architect_report_analysis",id,v.analysisIds,text);
 await ordered(p,"architect_report_distribution",id,v.distributionList,text);
 await ordered(p,"architect_report_parameter",id,v.parameters,text);
 await ordered(p,"architect_report_related_decision",id,v.relatedDecisionIds,text);
}
async function restoreReportRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ReportRecord>{
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
async function projectChangeRecord(p:ArtifactSqliteProjection,id:bigint,v:model.ChangeRecord):Promise<void>{
 await p.insert("architect_change_record",[...text(v.changeType),...textField(v.summary),...textField(v.reason),...optional(v.requestedBy,1,text),...optional(v.approvedBy,1,text),...optional(v.changeDate,1,text),...optional(v.effectiveDate,1,text),...optional(v.beforeSnapshot,1,text),...optional(v.afterSnapshot,1,text),...optional(v.costImpact,3,float),...optional(v.scheduleImpact,1,text),...symbol(v.approvalStatus,ValidationStatus),...optional(v.versionFrom,1,text),...optional(v.versionTo,1,text)],id);
 await ordered(p,"architect_change_impacted_entity",id,v.impactedEntityIds,text);
 await ordered(p,"architect_change_risk_impact",id,v.riskImpact,text);
 await ordered(p,"architect_change_rollback_plan",id,v.rollbackPlan,text);
 await ordered(p,"architect_change_communication_plan",id,v.communicationPlan,text);
 await ordered(p,"architect_change_audit_event",id,v.auditEventIds,text);
}
async function restoreChangeRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ChangeRecord>{
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
async function projectPerformanceCriterion(p:ArtifactSqliteProjection,id:bigint,v:model.PerformanceCriterion):Promise<void>{
 await p.insert("architect_performance_criterion",[...text(v.criterion),...text(v.metric),...optional(v.target,3,float),...optional(v.unit,1,text),...optional(v.minimum,3,float),...optional(v.maximum,3,float),...optional(v.measurementMethod,1,text),...optional(v.frequency,1,text),...optional(v.baseline,3,float),...optional(v.benchmarkRef,1,text),...optional(v.weight,3,float),...optional(v.dataSource,1,text),...optional(v.reportingCadence,1,text),...optional(v.ownerId,1,text),...optional(v.verificationPlan,1,text),...optional(v.penaltyThreshold,3,float),...optional(v.incentiveThreshold,3,float)],id);
 await ordered(p,"architect_performance_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_performance_element",id,v.elementIds,text);
}
async function restorePerformanceCriterion(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.PerformanceCriterion>{
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
async function projectQualityRecord(p:ArtifactSqliteProjection,id:bigint,v:model.QualityRecord):Promise<void>{
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
async function restoreQualityRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.QualityRecord>{
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
async function projectArtifactRecord(p:ArtifactSqliteProjection,id:bigint,v:model.ArtifactRecord):Promise<void>{
 await p.insert("architect_artifact_record",[...text(v.documentType),...text(v.title),...text(v.version),...optional(v.fileRef,1,text),...optional(v.format,1,text),...optional(v.issueDate,1,text),...optional(v.revisionDate,1,text),...optional(v.classification,1,text),...optional(v.retentionPeriod,1,text),...optional(v.supersedes,1,text),...symbol(v.documentStatus,LifecycleStatus),...optional(v.checksum,1,text),...optional(v.sourceSystem,1,text)],id);
 await ordered(p,"architect_artifact_author",id,v.authorIds,text);
 await ordered(p,"architect_artifact_reviewer",id,v.reviewerIds,text);
 await ordered(p,"architect_artifact_approver",id,v.approverIds,text);
 await ordered(p,"architect_artifact_distribution",id,v.distributionList,text);
 await ordered(p,"architect_artifact_related_entity",id,v.relatedEntityIds,text);
 await ordered(p,"architect_artifact_access_control",id,v.accessControls,text);
}
async function restoreArtifactRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ArtifactRecord>{
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
async function projectValidationRecord(p:ArtifactSqliteProjection,id:bigint,v:model.ValidationRecord):Promise<void>{
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
async function restoreValidationRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ValidationRecord>{
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
async function projectScenario(p:ArtifactSqliteProjection,id:bigint,v:model.Scenario):Promise<void>{
 await p.insert("architect_scenario",[...text(v.code),...textField(v.hypothesis),...optional(v.growthPlanId,1,text),...optional(v.probability,3,float),...optionalTextField(v.impactSummary),...optional(v.costDelta,3,float),...optional(v.areaDelta,3,float),...optional(v.headcountDelta,3,float),...optional(v.scheduleDelta,1,text),...boolean(v.baseline),...boolean(v.preferred),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_scenario_assumption",id,v.assumptions,text);
 await ordered(p,"architect_scenario_variable",id,v.variables,text);
 await ordered(p,"architect_scenario_element",id,v.elementIds,text);
 await ordered(p,"architect_scenario_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_scenario_risk",id,v.riskIds,text);
 await ordered(p,"architect_scenario_option",id,v.optionIds,text);
 await ordered(p,"architect_scenario_analysis",id,v.analysisIds,text);
}
async function restoreScenario(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Scenario>{
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
async function projectOptionEvaluation(p:ArtifactSqliteProjection,id:bigint,v:model.OptionEvaluation):Promise<void>{
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
async function restoreOptionEvaluation(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.OptionEvaluation>{
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
async function projectDecision(p:ArtifactSqliteProjection,id:bigint,v:model.Decision):Promise<void>{
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
async function restoreDecision(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Decision>{
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
async function projectPriorityRecord(p:ArtifactSqliteProjection,id:bigint,v:model.PriorityRecord):Promise<void>{
 await p.insert("architect_priority_record",[...text(v.subjectId),...text(v.subjectKind),...symbol(v.rankedPriority,Priority),...optional(v.rank,1,uint32),...optional(v.weight,3,float),...optionalTextField(v.rationale),...optional(v.decisionId,1,text),...optional(v.effectiveFrom,1,text),...optional(v.effectiveUntil,1,text),...optional(v.reviewCycle,1,text),...optional(v.scoringMethod,1,text),...optional(v.score,3,float),...optional(v.approvedBy,1,text),...optional(v.approvalDate,1,text)],id);
 await ordered(p,"architect_priority_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_priority_dependency",id,v.dependencies,text);
 await ordered(p,"architect_priority_conflict",id,v.conflicts,text);
 await ordered(p,"architect_priority_criterion",id,v.criteria,text);
 await ordered(p,"architect_priority_ranking_note",id,v.rankingNotes,note);
}
async function restorePriorityRecord(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.PriorityRecord>{
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
async function projectRisk(p:ArtifactSqliteProjection,id:bigint,v:model.Risk):Promise<void>{
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
async function restoreRisk(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Risk>{
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
async function projectConflict(p:ArtifactSqliteProjection,id:bigint,v:model.Conflict):Promise<void>{
 await p.insert("architect_conflict",[...symbol(v.kind,ConflictKind),...textField(v.summary),...text(v.entityAId),...text(v.entityBId),...symbol(v.severity,IssueSeverity),...optional(v.detectedBy,1,text),...optional(v.detectionDate,1,text),...optionalTextField(v.recommendedResolution),...optional(v.decisionId,1,text),...optional(v.costImpact,3,float),...optional(v.scheduleImpact,1,text),...symbol(v.resolutionStatus,ValidationStatus),...optional(v.ownerId,1,text),...optional(v.escalationLevel,1,text)],id);
 await ordered(p,"architect_conflict_trade_off_option",id,v.tradeOffOptions,text);
 await ordered(p,"architect_conflict_stakeholder",id,v.stakeholderIds,text);
 await ordered(p,"architect_conflict_requirement",id,v.requirementIds,text);
 await ordered(p,"architect_conflict_quality_impact",id,v.qualityImpact,text);
 await ordered(p,"architect_conflict_related_risk",id,v.relatedRiskIds,text);
}
async function restoreConflict(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Conflict>{
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
async function projectRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.Requirement):Promise<void>{
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
async function restoreRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Requirement>{
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
async function projectDeliveryConstraint(p:ArtifactSqliteProjection,id:bigint,v:model.DeliveryConstraint):Promise<void>{
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
async function restoreDeliveryConstraint(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.DeliveryConstraint>{
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
async function projectSustainabilityRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.SustainabilityRequirement):Promise<void>{
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
async function restoreSustainabilityRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.SustainabilityRequirement>{
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
async function projectResilienceRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.ResilienceRequirement):Promise<void>{
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
async function restoreResilienceRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ResilienceRequirement>{
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
async function projectCostRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.CostRequirement):Promise<void>{
 await p.insert("architect_cost_requirement",[...text(v.costItem),...symbol(v.basis,CostBasis),...optional(v.amount,3,float),...text(v.currency),...optional(v.quantityBasis,1,text),...optional(v.unitCost,3,float),...optional(v.contingencyPercent,3,float),...optional(v.escalationRate,3,float),...optional(v.fundingSource,1,text),...optional(v.phase,1,value=>symbol(value,DeliveryPhase)),...optional(v.benchmarkRef,1,text),...symbol(v.approvalStatus,ValidationStatus),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_cost_element",id,v.elementIds,text);
 await ordered(p,"architect_cost_requirement_ref",id,v.requirementIds,text);
 await ordered(p,"architect_cost_cash_flow",id,v.cashFlowProfile,text);
 await ordered(p,"architect_cost_value_engineering_note",id,v.valueEngineeringNotes,text);
 await ordered(p,"architect_cost_assumption",id,v.assumptions,text);
 await ordered(p,"architect_cost_sensitivity_factor",id,v.sensitivityFactors,text);
}
async function restoreCostRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.CostRequirement>{
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
async function projectGrowthPlan(p:ArtifactSqliteProjection,id:bigint,v:model.GrowthPlan):Promise<void>{
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
async function restoreGrowthPlan(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.GrowthPlan>{
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
async function projectWayfindingRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.WayfindingRequirement):Promise<void>{
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
async function restoreWayfindingRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.WayfindingRequirement>{
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
async function projectScheduleRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.ScheduleRequirement):Promise<void>{
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
async function restoreScheduleRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ScheduleRequirement>{
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
async function projectFlexibilityRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.FlexibilityRequirement):Promise<void>{
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
async function restoreFlexibilityRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.FlexibilityRequirement>{
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
async function projectInfrastructureRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.InfrastructureRequirement):Promise<void>{
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
async function restoreInfrastructureRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.InfrastructureRequirement>{
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
async function projectInformationRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.InformationRequirement):Promise<void>{
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
async function restoreInformationRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.InformationRequirement>{
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
async function projectCommunicationRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.CommunicationRequirement):Promise<void>{
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
async function restoreCommunicationRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.CommunicationRequirement>{
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
async function projectSiteContext(p:ArtifactSqliteProjection,id:bigint,v:model.SiteContext):Promise<void>{
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
async function restoreSiteContext(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.SiteContext>{
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
async function projectOrganizationalRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.OrganizationalRequirement):Promise<void>{
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
async function restoreOrganizationalRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.OrganizationalRequirement>{
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
async function projectServiceRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.ServiceRequirement):Promise<void>{
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
async function restoreServiceRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ServiceRequirement>{
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
async function projectSafetyRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.SafetyRequirement):Promise<void>{
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
async function restoreSafetyRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.SafetyRequirement>{
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
async function projectSecurityRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.SecurityRequirement):Promise<void>{
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
async function restoreSecurityRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.SecurityRequirement>{
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
async function projectRegulatoryRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.RegulatoryRequirement):Promise<void>{
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
async function restoreRegulatoryRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.RegulatoryRequirement>{
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
async function projectAccessibilityRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.AccessibilityRequirement):Promise<void>{
 await p.insert("architect_accessibility_requirement",[...text(v.standard),...optional(v.level,1,text),...optional(v.clearWidthM,3,float),...optional(v.clearHeightM,3,float),...optional(v.turningCircleM,3,float),...optional(v.rampSlope,3,float),...boolean(v.liftRequired),...boolean(v.tactileGuidance),...boolean(v.hearingLoop),...boolean(v.visualContrast),...optional(v.controlsHeight,1,text),...optional(v.serviceAnimalPolicy,1,text),...boolean(v.companionSeating),...optional(v.verificationPlan,1,text),...optional(v.wcagConformance,1,text)],id);
 await ordered(p,"architect_accessibility_user_profile",id,v.userProfileIds,text);
 await ordered(p,"architect_accessibility_element",id,v.elementIds,text);
 await ordered(p,"architect_accessibility_route",id,v.routeIds,text);
 await ordered(p,"architect_accessibility_signage_requirement",id,v.signageRequirements,text);
 await ordered(p,"architect_accessibility_emergency_evacuation",id,v.emergencyEvacuation,text);
 await ordered(p,"architect_accessibility_exception",id,v.exceptions,text);
 await ordered(p,"architect_accessibility_universal_design_principle",id,v.universalDesignPrinciples,text);
}
async function restoreAccessibilityRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.AccessibilityRequirement>{
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
async function projectPrivacyRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.PrivacyRequirement):Promise<void>{
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
async function restorePrivacyRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.PrivacyRequirement>{
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
async function projectEnvironmentalRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.EnvironmentalRequirement):Promise<void>{
 await p.insert("architect_environmental_requirement",[...symbol(v.parameterKind,EnvironmentalParameter),...text(v.parameter),...optional(v.targetValue,3,float),...optional(v.unit,1,text),...optional(v.minValue,3,float),...optional(v.maxValue,3,float),...optional(v.comfortBand,1,text),...optional(v.measurementMethod,1,text),...optional(v.monitoringFrequency,1,text),...optional(v.occupancyBasis,1,text),...optional(v.ventilationStrategy,1,text),...optional(v.daylightTarget,1,text),...optional(v.acousticTarget,1,text),...optional(v.iaqTarget,1,text),...optional(v.verificationPlan,1,text)],id);
 await ordered(p,"architect_environmental_element",id,v.elementIds,text);
 await ordered(p,"architect_environmental_seasonal_variation",id,v.seasonalVariation,text);
 await ordered(p,"architect_environmental_energy_implication",id,v.energyImplications,text);
 await ordered(p,"architect_environmental_standard",id,v.standards,text);
 await ordered(p,"architect_environmental_certification_target",id,v.certificationTargets,text);
 await ordered(p,"architect_environmental_outdoor_condition",id,v.outdoorConditions,text);
}
async function restoreEnvironmentalRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.EnvironmentalRequirement>{
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
async function projectHumanFactorRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.HumanFactorRequirement):Promise<void>{
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
async function restoreHumanFactorRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.HumanFactorRequirement>{
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
async function projectStorageRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.StorageRequirement):Promise<void>{
 await p.insert("architect_storage_requirement",[...text(v.storedItem),...symbol(v.storageClass,StorageClass),...optional(v.volumeM3,3,float),...optional(v.weightKg,3,float),...optional(v.temperatureRange,1,text),...optional(v.humidityRange,1,text),...symbol(v.securityLevel,AccessLevel),...optional(v.hazardClass,1,text),...optional(v.retentionPeriod,1,text),...optional(v.accessFrequency,1,text),...optional(v.ventilation,1,text),...optional(v.organizationSystem,1,text),...optional(v.growthAllowance,3,float),...optional(v.ownerId,1,text)],id);
 await ordered(p,"architect_storage_element",id,v.elementIds,text);
 await ordered(p,"architect_storage_equipment",id,v.equipmentIds,text);
 await ordered(p,"architect_storage_handling_equipment",id,v.handlingEquipment,text);
 await ordered(p,"architect_storage_fire_protection",id,v.fireProtection,text);
 await ordered(p,"architect_storage_regulatory_reference",id,v.regulatoryRefs,text);
 await quantity(p,id,"quantity","architect_storage_quantity",v.quantity);
}
async function restoreStorageRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.StorageRequirement>{
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
async function projectEquipment(p:ArtifactSqliteProjection,id:bigint,v:model.Equipment):Promise<void>{
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
async function restoreEquipment(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Equipment>{
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
async function projectResource(p:ArtifactSqliteProjection,id:bigint,v:model.Resource):Promise<void>{
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
async function restoreResource(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Resource>{
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
async function projectFlowRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.FlowRequirement):Promise<void>{
 await p.insert("architect_flow_requirement",[...text(v.fromElementId),...text(v.toElementId),...symbol(v.kind,FlowKind),...text(v.flowType),...symbol(v.direction,FlowDirection),...optional(v.peakRate,3,float),...optional(v.clearWidthM,3,float),...optional(v.clearHeightM,3,float),...symbol(v.accessLevel,AccessLevel),...optional(v.equipmentClearance,1,text),...boolean(v.signageRequired),...boolean(v.escortRequired),...boolean(v.emergencyRoute),...boolean(v.barrierFree),...boolean(v.monitoringRequired),...optional(v.processId,1,text),...optional(v.verificationMethod,1,text)],id);
 await ordered(p,"architect_flow_separation",id,v.separationRequirements,value=>symbol(value,SeparationKind));
 await ordered(p,"architect_flow_time_window",id,v.timeWindows,text);
 await ordered(p,"architect_flow_conflict",id,v.conflictIds,text);
 await quantity(p,id,"volume","architect_flow_volume",v.volume);
}
async function restoreFlowRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.FlowRequirement>{
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
async function projectAccessRule(p:ArtifactSqliteProjection,id:bigint,v:model.AccessRule):Promise<void>{
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
async function restoreAccessRule(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.AccessRule>{
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
async function projectOperationalRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.OperationalRequirement):Promise<void>{
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
async function restoreOperationalRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.OperationalRequirement>{
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
async function projectAdjacency(p:ArtifactSqliteProjection,id:bigint,v:model.Adjacency):Promise<void>{
 await p.insert("architect_adjacency",[...text(v.elementAId),...text(v.elementBId),...symbol(v.kind,AdjacencyKind),...symbol(v.connection,ConnectionKind),...float(v.weight),...optionalTextField(v.rationale),...optional(v.distanceMaxM,3,float),...optional(v.distanceMinM,3,float),...optional(v.levelConstraint,1,text),...optional(v.accessPath,1,text),...boolean(v.sharedWall),...boolean(v.sharedEntry),...boolean(v.trafficIsolation),...boolean(v.circulationOverlap),...boolean(v.normalized),...symbol(v.verificationStatus,ValidationStatus),...optional(v.sourceRelationshipId,1,text),...optional(v.internalExternalAccess,1,text)],id);
 await ordered(p,"architect_adjacency_separation",id,v.separations,value=>symbol(value,SeparationKind));
 await ordered(p,"architect_adjacency_conflict",id,v.conflictIds,text);
}
async function restoreAdjacency(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Adjacency>{
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
async function projectProcess(p:ArtifactSqliteProjection,id:bigint,v:model.Process):Promise<void>{
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
async function restoreProcess(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Process>{
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
async function projectQuantityRequirement(p:ArtifactSqliteProjection,id:bigint,v:model.QuantityRequirement):Promise<void>{
 await p.insert("architect_quantity_requirement",[...text(v.targetElementId),...text(v.metric),...optional(v.basis,1,text),...optional(v.calculationMethod,1,text),...optional(v.source,1,text),...optional(v.benchmarkRef,1,text),...optional(v.tolerancePercent,3,float),...optional(v.peakFactor,3,float),...optional(v.growthFactor,3,float),...optional(v.unitCost,3,float),...optional(v.currency,1,text),...optional(v.verificationMethod,1,text),...optional(v.schedulePhase,1,text),...optional(v.responsibleParty,1,text),...optional(v.lastVerified,1,text)],id);
 await ordered(p,"architect_quantity_requirement_related_requirement",id,v.relatedRequirementIds,text);
 await ordered(p,"architect_quantity_requirement_assumption",id,v.assumptions,text);
 await ordered(p,"architect_quantity_requirement_constraint",id,v.constraints,text);
 await ordered(p,"architect_quantity_requirement_variance_note",id,v.varianceNotes,note);
 await quantity(p,id,"quantity","architect_quantity_requirement_quantity",v.quantity);
}
async function restoreQuantityRequirement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.QuantityRequirement>{
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
async function projectRelationship(p:ArtifactSqliteProjection,id:bigint,v:model.Relationship):Promise<void>{
 await p.insert("architect_relationship",[...text(v.sourceId),...text(v.targetId),...symbol(v.kind,RelationshipKind),...optional(v.strength,3,float),...boolean(v.directional),...optionalTextField(v.rationale),...symbol(v.relationshipPriority,Priority),...optional(v.validFrom,1,text),...optional(v.validUntil,1,text),...boolean(v.bidirectional),...optional(v.distanceConstraintM,3,float),...optional(v.capacityConstraint,1,text),...optional(v.reviewCycle,1,text),...optional(v.ownerId,1,text),...optionalTextField(v.proximityRequirement),...optionalTextField(v.compatibilityRequirement),...optionalTextField(v.incompatibilityRequirement)],id);
 await ordered(p,"architect_relationship_constraint",id,v.constraints,text);
 await ordered(p,"architect_relationship_condition",id,v.conditions,text);
 await ordered(p,"architect_relationship_evidence",id,v.evidence,text);
 await ordered(p,"architect_relationship_conflict",id,v.conflictIds,text);
 await ordered(p,"architect_relationship_regulatory_basis",id,v.regulatoryBasis,text);
 await ordered(p,"architect_relationship_separation",id,v.separationRequirements,value=>symbol(value,SeparationKind));
 await ordered(p,"architect_relationship_trace",id,v.traceLinks,trace);
}
async function restoreRelationship(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Relationship>{
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
async function projectUserProfile(p:ArtifactSqliteProjection,id:bigint,v:model.UserProfile):Promise<void>{
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
async function restoreUserProfile(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.UserProfile>{
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
async function projectActivity(p:ArtifactSqliteProjection,id:bigint,v:model.Activity):Promise<void>{
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
async function restoreActivity(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Activity>{
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
async function projectFunction(p:ArtifactSqliteProjection,id:bigint,v:model.Function):Promise<void>{
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
async function restoreFunction(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Function>{
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
async function projectProgramElement(p:ArtifactSqliteProjection,id:bigint,v:model.ProgramElement):Promise<void>{
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
async function restoreProgramElement(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.ProgramElement>{
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
async function projectStakeholder(p:ArtifactSqliteProjection,id:bigint,v:model.Stakeholder):Promise<void>{
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
async function restoreStakeholder(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.Stakeholder>{
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
async function projectProgramMeta(p:ArtifactSqliteProjection,v:model.ProgramMeta):Promise<void>{
 await p.insert("architect_meta",[1n,...text(v.schema),...text(v.documentId),...text(v.title),...optional(v.subtitle,1,text),...textField(v.purpose),...text(v.industrySector),...text(v.projectType),...text(v.locale),...text(v.revision),...optional(v.sourceSystem,1,text),...optional(v.exportProfile,1,text),...timestamp(v.timestamps)],1n);
 await ordered(p,"architect_meta_term",1n,v.terminology,text);
 await ordered(p,"architect_meta_classification",1n,v.classification,text);
 await ordered(p,"architect_meta_author",1n,v.authorIds,text);
}
async function restoreProgramMeta(r:Reader):Promise<model.ProgramMeta>{
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
async function projectProjectDefinition(p:ArtifactSqliteProjection,v:model.ProjectDefinition):Promise<void>{
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
async function restoreProjectDefinition(r:Reader):Promise<model.ProjectDefinition>{
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
async function projectGovernance(p:ArtifactSqliteProjection,v:model.Governance):Promise<void>{
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
async function restoreGovernance(r:Reader):Promise<model.Governance>{
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

async function projectChild(p:ArtifactSqliteProjection,slot:string,value:model.ArtifactChild):Promise<void>{await p.insert("architect_child",[1n,slot,...text(value.childId),...text(value.target.artifactId),...text(value.target.dialect.artifactKind),...text(value.target.dialect.standard),...text(value.target.dialect.subset)])}
async function restoreChild(r:Reader,slot:string):Promise<model.ArtifactChild>{let found:SqliteRow|undefined;for(const row of r.rows("architect_child")){await r.control.step();if(row.values.length!==8||row.rowid<=0n||row.values[0]!==row.rowid||row.values[1]!==1n)fail("child shape or owner differs");if(row.values[2]===slot){if(found)fail("child slot is repeated");found=row}}if(!found)return fail("required child slot is absent");await r.use(found);const c=new Cursor(found,3),childId=c.text(),artifactId=c.text(),artifactKind=c.text(),standard=c.text(),subset=c.text();c.end();return{childId,target:{artifactId,dialect:{artifactKind,standard,subset}}}}
async function projectAuditEvent(p:ArtifactSqliteProjection,id:bigint,v:model.AuditEvent):Promise<void>{await p.insert("architect_audit_event",[...symbol(v.action,AuditAction),...optional(v.actorId,1,text),...text(v.subjectId),...text(v.subjectKind),...text(v.timestamp),...textField(v.details),...optional(v.beforeState,1,text),...optional(v.afterState,1,text),...optional(v.ipAddress,1,text),...optional(v.client,1,text),...optional(v.sessionId,1,text),...optional(v.changeRecordId,1,text),...boolean(v.success),...optional(v.errorMessage,1,text),...optional(v.correlationId,1,text),...optional(v.retentionUntil,1,text)],id);await ordered(p,"architect_audit_compliance_tag",id,v.complianceTags,text);if(v.traceLink!==null)await p.insert("architect_audit_trace",trace(v.traceLink),id)}
async function restoreAuditEvent(r:Reader,hrow:SqliteRow,h:model.EntityHeader):Promise<model.AuditEvent>{const row=await r.body("architect_audit_event",hrow.rowid,18),c=new Cursor(row!),action=c.symbol(AuditAction),actorId=c.optional(1,()=>c.text()),subjectId=c.text(),subjectKind=c.text(),timestamp=c.text(),details=c.textField(),beforeState=c.optional(1,()=>c.text()),afterState=c.optional(1,()=>c.text()),ipAddress=c.optional(1,()=>c.text()),client=c.optional(1,()=>c.text()),sessionId=c.optional(1,()=>c.text()),changeRecordId=c.optional(1,()=>c.text()),success=c.boolean(),errorMessage=c.optional(1,()=>c.text()),correlationId=c.optional(1,()=>c.text()),retentionUntil=c.optional(1,()=>c.text());c.end();const complianceTags=await r.list("architect_audit_compliance_tag",hrow.rowid,1,c=>c.text()),traceRow=await r.body("architect_audit_trace",hrow.rowid,6,false),tc=traceRow?new Cursor(traceRow):null,traceLink=tc?readTrace(tc):null;tc?.end();return{...h,action,actorId,subjectId,subjectKind,timestamp,details,beforeState,afterState,ipAddress,client,sessionId,changeRecordId,success,errorMessage,correlationId,retentionUntil,complianceTags,traceLink}}
function count(value:number,extra:number,maximum:number):number{const result=value+extra;return Number.isSafeInteger(extra)&&extra>=0&&Number.isSafeInteger(result)&&result<=maximum?result:fail("semantic row limit")}
async function forecast(s:model.ProgramArtifact,options:ArtifactSqliteOptions):Promise<number>{await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);const maximum=options.maxRows??1000000;let total=count(0,6,maximum),visits=0;
 total=count(total,s.meta.terminology.length,maximum);
 total=count(total,s.meta.classification.length,maximum);
 total=count(total,s.meta.authorIds.length,maximum);
 total=count(total,s.project.objectives.length,maximum);
 total=count(total,s.project.successCriteria.length,maximum);
 total=count(total,s.project.projectPriorities.length,maximum);
 total=count(total,s.project.completionCriteria.length,maximum);
 total=count(total,s.project.decisionCriteria.length,maximum);
 total=count(total,s.project.scopeInclusions.length,maximum);
 total=count(total,s.project.scopeExclusions.length,maximum);
 total=count(total,s.project.assumptions.length,maximum);
 total=count(total,s.project.constraintsSummary.length,maximum);
 total=count(total,s.project.dependencies.length,maximum);
 total=count(total,s.project.deliverables.length,maximum);
 total=count(total,s.project.phases.length,maximum);
 total=count(total,s.project.regulatoryContext.length,maximum);
 total=count(total,s.project.ownership.consultantIds.length,maximum);
 total=count(total,s.project.ownership.participantIds.length,maximum);
 total=count(total,s.governance.roles.length,maximum);
 total=count(total,s.governance.responsibilities.length,maximum);
 total=count(total,s.governance.approvalMatrix.length,maximum);
 total=count(total,s.governance.escalationPaths.length,maximum);
 total=count(total,s.governance.meetingCadence.length,maximum);
 total=count(total,s.governance.decisionRights.length,maximum);
 total=count(total,s.governance.changeControlProcess.length,maximum);
 total=count(total,s.governance.complianceObligations.length,maximum);
 total=count(total,s.governance.documentControl.length,maximum);
 total=count(total,s.governance.stakeholderEngagementPlan.length,maximum);
 total=count(total,s.governance.ethicsPolicy.length,maximum);
 total=count(total,s.governance.dataGovernance.length,maximum);
 total=count(total,s.governance.reviewHierarchy.length,maximum);
 total=count(total,s.governance.accountabilityRules.length,maximum);
 total=count(total,s.governance.exceptionManagement.length,maximum);
 total=count(total,s.governance.governancePerformance.length,maximum);
 total=count(total,s.traces.length,maximum);
 for(const row of s.approvals){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.approverIds.length+row.conditions.length+row.delegationChain.length+row.evidenceRefs.length+row.authorityBasis.length+row.notificationList.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.meetings){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.attendeeIds.length+row.agendaItems.length+row.actionItems.length+row.decisionsMade.length+row.artifactRefs.length+row.stakeholderIds.length+row.requirementIds.length+row.issueIds.length+row.approvalIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.assumptions){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.relatedEntityIds.length+row.dependencies.length+row.mitigation.length+row.linkedRequirementIds.length+row.linkedRiskIds.length+row.statusNotes.length+row.artifactRefs.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.constraints){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.affectedEntityIds.length+row.regulatoryBasis.length+row.mitigationOptions.length+row.resolutionPlan.length+row.relatedRequirementIds.length+row.relatedDecisionIds.length+row.exceptions.length+row.traceLinks.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.complianceRecords){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.evidenceRefs.length+row.affectedEntityIds.length+row.gapAnalysis.length+row.remediationPlan.length+row.relatedRequirementIds.length+row.penalties.length+row.correctiveActions.length+row.artifactRefs.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.templates){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.entityKinds.length+row.defaultFields.length+row.checklists.length+row.standards.length+row.applicability.length+row.customizationNotes.length+row.relatedKnowledgeIds.length+row.benchmarkIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.knowledgePayload){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.sources.length+row.references.length+row.lessonsLearned.length+row.bestPractices.length+row.applicableSectors.length+row.relatedEntityKinds.length+row.authorIds.length+row.keywords.length+row.attachments.length+row.citations.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.benchmarksPayload){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.applicableElementKinds.length+row.relatedRequirementIds.length+row.comparisonNotes.length+row.limitations.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.issues){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.affectedEntityIds.length+row.relatedConflictIds.length+row.relatedRiskIds.length+row.comments.length+row.attachments.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.statusRecords){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.blockers.length+row.nextActions.length+row.relatedIssueIds.length+row.relatedRiskIds.length+row.statusNotes.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.workshops){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.objectives.length+row.agenda.length+row.participants.length+row.materials.length+row.methods.length+row.outputs.length+row.decisions.length+row.issues.length+row.followUpActions.length+row.feedback.length+row.surveyIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.surveys){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.objectives.length+row.questions.length+row.targetAudience.length+row.distributionChannels.length+row.findings.length+row.themes.length+row.recommendations.length+row.consentProcess.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.searchFilters){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.keywords.length+row.categories.length+row.ownerIds.length+row.statuses.length+row.priorities.length+row.sources.length+row.entityKinds.length+row.tagFilters.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.collaboration){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.participants.length+row.agenda.length+row.outcomes.length+row.actionItems.length+row.decisionIds.length+row.issueIds.length+row.documentIds.length+row.feedback.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.analyses){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.parameters.length+row.inputEntityIds.length+row.findings.length+row.metrics.length+row.charts.length+row.limitations.length+row.recommendations.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.reports){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.audience.length+row.sections.length+row.analysisIds.length+row.distributionList.length+row.parameters.length+row.relatedDecisionIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.changes){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.impactedEntityIds.length+row.riskImpact.length+row.rollbackPlan.length+row.communicationPlan.length+row.auditEventIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.performance){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.requirementIds.length+row.elementIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.quality){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.inspectionPoints.length+row.acceptanceCriteria.length+row.testingRequirements.length+row.defectCategories.length+row.correctiveActionProcess.length+row.elementIds.length+row.requirementIds.length+row.supplierRequirements.length+row.documentationRequirements.length+row.trainingRequirements.length+row.kpis.length+row.certificationTargets.length+row.continuousImprovement.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.artifacts){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.authorIds.length+row.reviewerIds.length+row.approverIds.length+row.distributionList.length+row.relatedEntityIds.length+row.accessControls.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.validations){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.criteria.length+row.evidence.length+row.validatorIds.length+row.findings.length+row.nonConformities.length+row.correctiveActions.length+row.waivers.length+row.standards.length+row.traceLinks.length+row.validationNotes.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.scenarios){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.assumptions.length+row.variables.length+row.elementIds.length+row.requirementIds.length+row.riskIds.length+row.optionIds.length+row.analysisIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.options){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.criteriaIds.length+row.scores.length+row.riskSummary.length+row.benefits.length+row.drawbacks.length+row.assumptions.length+row.dependencies.length+row.stakeholderFeedback.length+row.evaluatorIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.decisions){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.optionsConsidered.length+row.decisionMakerIds.length+row.consultedIds.length+row.informedIds.length+row.reversalConditions.length+row.impactedRequirementIds.length+row.impactedElementIds.length+row.riskImpact.length+row.artifactRefs.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.priorities){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.stakeholderIds.length+row.dependencies.length+row.conflicts.length+row.criteria.length+row.rankingNotes.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.risks){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.causes.length+row.effects.length+row.affectedElementIds.length+row.affectedRequirementIds.length+row.mitigation.length+row.contingency.length+row.triggerIndicators.length+row.relatedConflictIds.length+row.escalationPath.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.conflicts){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.tradeOffOptions.length+row.stakeholderIds.length+row.requirementIds.length+row.qualityImpact.length+row.relatedRiskIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.requirements){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.stakeholderIds.length+row.elementIds.length+row.functionIds.length+row.childRequirementIds.length+row.acceptanceCriteria.length+row.conflictIds.length+row.riskIds.length+row.regulatoryRefs.length+row.traceLinks.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.delivery){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.impactedElementIds.length+row.impactedRequirementIds.length+row.noiseRestrictions.length+row.accessRestrictions.length+row.siteLogistics.length+row.approvalGates.length+row.occupancyConstraints.length+row.weatherWindows.length+row.penaltyClauses.length+row.mitigationOptions.length+row.riskIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.sustainability){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.certification.length+row.standards.length+row.elementIds.length+row.strategies.length+row.materialsPreferences.length+row.energyStrategy.length+row.waterStrategy.length+row.wasteStrategy.length+row.biodiversity.length+row.reportingRequirements.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.resilience){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.redundancy.length+row.hardeningMeasures.length+row.backupSystems.length+row.alternateSites.length+row.supplyChain.length+row.communicationPlan.length+row.drillRequirements.length+row.elementIds.length+row.infrastructureIds.length+row.standards.length+row.insuranceImplications.length+row.climateAdaptation.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.costs){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.elementIds.length+row.requirementIds.length+row.cashFlowProfile.length+row.valueEngineeringNotes.length+row.assumptions.length+row.sensitivityFactors.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.growth){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.phases.length+row.triggerEvents.length+row.expansionElementIds.length+row.reserveAreas.length+row.infrastructureHeadroom.length+row.fundingSources.length+row.riskFactors.length+row.decisionPoints.length+row.scenarioIds.length+row.decommissionPlan.length+row.relocationStrategy.length+row.stakeholderImpact.length+row.regulatoryConsiderations.length+4,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.wayfinding){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.userProfileIds.length+row.elementIds.length+row.destinationTypes.length+row.signageTypes.length+row.languages.length+row.landmarkStrategy.length+row.colorCoding.length+row.symbolStandards.length+row.decisionPoints.length+row.lightingRequirements.length+row.emergencyEgress.length+row.visitorJourney.length+row.staffJourney.length+row.brandIntegration.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.schedules){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.dependencies.length+row.predecessors.length+row.successors.length+row.resourceRequirements.length+row.occupancyImpact.length+row.decantRequirements.length+row.stakeholderIds.length+row.riskIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.flexibility){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.elementIds.length+row.adaptationScenarios.length+row.futureFunctionIds.length+row.expansionDirection.length+row.contractionScenario.length+row.multiUsePotential.length+row.furnitureStrategy.length+row.infrastructureSpareCapacity.length+row.leaseImplications.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.infrastructure){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.distribution.length+row.entryPoints.length+row.monitoring.length+row.maintenanceAccess.length+row.standards.length+row.elementIds.length+row.futureExpansion.length+row.interfaceRequirements.length+row.commissioning.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.information){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.destinationSystems.length+row.accessControls.length+row.qualityCriteria.length+row.metadataRequirements.length+row.integrationPoints.length+row.backupRequirements.length+row.disasterRecovery.length+row.privacyControls.length+row.elementIds.length+row.stakeholderIds.length+row.standards.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.communication){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.audienceIds.length+row.messageTypes.length+row.medium.length+row.language.length+row.accessibility.length+row.signageLocations.length+row.technology.length+row.escalationPath.length+row.privacyControls.length+row.elementIds.length+row.standards.length+row.templates.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.siteContext){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.soilConditions.length+row.utilitiesAvailable.length+row.accessRoads.length+row.publicTransit.length+row.neighbors.length+row.views.length+row.noiseSources.length+row.environmentalConstraints.length+row.heritageConstraints.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.organizational){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.workPatterns.length+row.hierarchyLevels.length+row.decisionMaking.length+row.cultureNotes.length+row.unionConsiderations.length+row.trainingNeeds.length+row.elementIds.length+row.stakeholderIds.length+row.serviceRequirementIds.length+row.brandingRequirements.length+row.wellnessPlugins.length+row.diversityGoals.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.services){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.queueManagement.length+row.customerProfiles.length+row.elementIds.length+row.equipmentIds.length+row.qualityMetrics.length+row.contractRefs.length+row.dependencies.length+row.backupService.length+row.feedbackChannels.length+4,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.safety){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.affectedElementIds.length+row.affectedUserIds.length+row.mitigationMeasures.length+row.ppeRequirements.length+row.emergencyProcedures.length+row.evacuationRequirements.length+row.fireProtection.length+row.structuralSafety.length+row.slipTripFall.length+row.chemicalSafety.length+row.electricalSafety.length+row.machinerySafety.length+row.standards.length+row.trainingRequirements.length+row.incidentReporting.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.security){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.assetIds.length+row.zoneIds.length+row.perimeterControls.length+row.surveillance.length+row.intrusionDetection.length+row.cybersecurity.length+row.screening.length+row.visitorManagement.length+row.keyManagement.length+row.standards.length+row.responseProcedures.length+row.liaisonContacts.length+row.redundancy.length+row.auditRequirements.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.regulatory){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.applicability.length+row.elementIds.length+row.evidenceRequired.length+row.penalties.length+row.exemptions.length+row.relatedRequirementIds.length+row.interpretationNotes.length+row.consultantRefs.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.accessibility){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.userProfileIds.length+row.elementIds.length+row.routeIds.length+row.signageRequirements.length+row.emergencyEvacuation.length+row.exceptions.length+row.universalDesignPrinciples.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.privacy){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.subjectIds.length+row.elementIds.length+row.visualPrivacy.length+row.acousticPrivacy.length+row.dataPrivacy.length+row.accessRestrictions.length+row.regulatoryBasis.length+row.culturalConsiderations.length+row.technologyControls.length+row.signage.length+row.monitoringRestrictions.length+row.breachResponse.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.environmental){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.elementIds.length+row.seasonalVariation.length+row.energyImplications.length+row.standards.length+row.certificationTargets.length+row.outdoorConditions.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.humanFactors){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.userProfileIds.length+row.activityIds.length+row.ergonomicCriteria.length+row.visualDemands.length+row.auditoryDemands.length+row.postureRequirements.length+row.lightingForTasks.length+row.thermalComfort.length+row.privacyNeeds.length+row.socialInteraction.length+row.stressFactors.length+row.mitigationMeasures.length+row.trainingNeeds.length+row.standards.length+row.researchBasis.length+row.elementIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.storage){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.elementIds.length+row.equipmentIds.length+row.handlingEquipment.length+row.fireProtection.length+row.regulatoryRefs.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.equipment){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.utilityConnections.length+row.elementIds.length+row.activityIds.length+row.maintenanceAccess.length+row.standards.length+row.activityLinkIds.length+row.installationRequirements.length+row.commissioningNotes.length+row.spareParts.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.resources){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.elementIds.length+row.activityIds.length+row.userProfileIds.length+row.cleaningRequirements.length+row.standards.length+row.ergonomicNotes.length+row.customization.length+row.disposalNotes.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.flows){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.separationRequirements.length+row.timeWindows.length+row.conflictIds.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.accessRules){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.subjectIds.length+row.resourceIds.length+row.authentication.length+row.authorization.length+row.timeRestrictions.length+row.zoneIds.length+row.exceptions.length+row.regulatoryBasis.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.operations){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.equipmentIds.length+row.elementIds.length+row.processIds.length+row.utilities.length+row.wasteStreams.length+row.contingencyPlan.length+row.trainingRequirements.length+row.sopReferences.length+row.kpiTargets.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.adjacencies){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.separations.length+row.conflictIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.processes){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.inputs.length+row.outputs.length+row.steps.length+row.actors.length+row.equipmentIds.length+row.elementIds.length+row.bottlenecks.length+row.dependencies.length+row.kpis.length+row.failureModes.length+row.improvementOpportunities.length+row.regulatoryRefs.length+row.handoffPoints.length+row.qualityGates.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.quantities){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.relatedRequirementIds.length+row.assumptions.length+row.constraints.length+row.varianceNotes.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.relationships){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.constraints.length+row.conditions.length+row.evidence.length+row.conflictIds.length+row.regulatoryBasis.length+row.separationRequirements.length+row.traceLinks.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.users){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.abilities.length+row.disabilities.length+row.mobilityProfile.length+row.sensoryProfile.length+row.cognitiveProfile.length+row.behavioralPatterns.length+row.peakUsageTimes.length+row.preferences.length+row.painPoints.length+row.goals.length+row.activityIds.length+row.stakeholderIds.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.activities){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.equipmentIds.length+row.spaceRequirements.length+row.environmentalNeeds.length+row.privacyNeeds.length+row.accessibilityNeeds.length+row.adjacentActivities.length+row.sequencing.length+row.peakPeriods.length+row.workflowSteps.length+row.inputs.length+row.outputs.length+row.userProfileIds.length+row.functionIds.length+row.performanceIndicators.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.functions){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.performanceTargets.length+row.equipmentIds.length+row.resourceIds.length+row.activityIds.length+row.elementIds.length+row.dependencies.length+row.interfaces.length+row.constraints.length+row.qualityCriteria.length+row.regulatoryRefs.length+row.futureChanges.length+row.successMetrics.length+row.conflictIds.length+2,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.elements){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.functionIds.length+row.activityIds.length+row.userProfileIds.length+row.adjacencyIds.length+row.quantityIds.length+row.requirementIds.length+row.flexibilityNotes.length+row.adjacencyPreferences.length+8,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.stakeholders){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.expectations.length+row.concerns.length+row.requirementIds.length+row.communicationPreferences.length+row.involvementPhases.length+row.powerInterestNotes.length+row.communicationChannels.length+row.successMetrics.length,maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 for(const row of s.auditEvents){total=count(total,2+row.tags.length+row.notes.length+row.ownership.consultantIds.length+row.ownership.participantIds.length+row.complianceTags.length+(row.traceLink===null?0:1),maximum);if(++visits%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",visits,0)}
 await artifactSqliteCheckpoint(options,"projectSnapshot",visits,visits);return total;
}
/** 📤️ Projects every typed register after a borrowed exact row forecast. */
export async function programSnapshotToSqliteDatabase(s:model.ProgramArtifact,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const total=await forecast(s,options),p=await ArtifactSqliteProjection.create(PROGRAM_SQLITE_SCHEMA,options);p.checkRowsAdditional(total);
 await p.insert("architect_document",text(s.schema),1n);await projectProgramMeta(p,s.meta);await projectProjectDefinition(p,s.project);await projectGovernance(p,s.governance);await projectChild(p,"knowledge",s.knowledge);await projectChild(p,"benchmarks",s.benchmarks);await ordered(p,"architect_trace",1n,s.traces,trace);
 for(let ordinal=0;ordinal<s.approvals.length;ordinal++){const value=s.approvals[ordinal]!,id=await header(p,"approvals",ordinal,value);await projectApprovalRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.meetings.length;ordinal++){const value=s.meetings[ordinal]!,id=await header(p,"meetings",ordinal,value);await projectMeetingRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.assumptions.length;ordinal++){const value=s.assumptions[ordinal]!,id=await header(p,"assumptions",ordinal,value);await projectAssumption(p,id,value)}
 for(let ordinal=0;ordinal<s.constraints.length;ordinal++){const value=s.constraints[ordinal]!,id=await header(p,"constraints",ordinal,value);await projectConstraintRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.complianceRecords.length;ordinal++){const value=s.complianceRecords[ordinal]!,id=await header(p,"complianceRecords",ordinal,value);await projectComplianceRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.templates.length;ordinal++){const value=s.templates[ordinal]!,id=await header(p,"templates",ordinal,value);await projectTemplateRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.knowledgePayload.length;ordinal++){const value=s.knowledgePayload[ordinal]!,id=await header(p,"knowledgePayload",ordinal,value);await projectKnowledgeRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.benchmarksPayload.length;ordinal++){const value=s.benchmarksPayload[ordinal]!,id=await header(p,"benchmarksPayload",ordinal,value);await projectBenchmarkRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.issues.length;ordinal++){const value=s.issues[ordinal]!,id=await header(p,"issues",ordinal,value);await projectIssue(p,id,value)}
 for(let ordinal=0;ordinal<s.statusRecords.length;ordinal++){const value=s.statusRecords[ordinal]!,id=await header(p,"statusRecords",ordinal,value);await projectStatusRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.workshops.length;ordinal++){const value=s.workshops[ordinal]!,id=await header(p,"workshops",ordinal,value);await projectWorkshop(p,id,value)}
 for(let ordinal=0;ordinal<s.surveys.length;ordinal++){const value=s.surveys[ordinal]!,id=await header(p,"surveys",ordinal,value);await projectSurvey(p,id,value)}
 for(let ordinal=0;ordinal<s.searchFilters.length;ordinal++){const value=s.searchFilters[ordinal]!,id=await header(p,"searchFilters",ordinal,value);await projectSearchFilter(p,id,value)}
 for(let ordinal=0;ordinal<s.collaboration.length;ordinal++){const value=s.collaboration[ordinal]!,id=await header(p,"collaboration",ordinal,value);await projectCollaborationRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.analyses.length;ordinal++){const value=s.analyses[ordinal]!,id=await header(p,"analyses",ordinal,value);await projectAnalysisRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.reports.length;ordinal++){const value=s.reports[ordinal]!,id=await header(p,"reports",ordinal,value);await projectReportRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.changes.length;ordinal++){const value=s.changes[ordinal]!,id=await header(p,"changes",ordinal,value);await projectChangeRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.performance.length;ordinal++){const value=s.performance[ordinal]!,id=await header(p,"performance",ordinal,value);await projectPerformanceCriterion(p,id,value)}
 for(let ordinal=0;ordinal<s.quality.length;ordinal++){const value=s.quality[ordinal]!,id=await header(p,"quality",ordinal,value);await projectQualityRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.artifacts.length;ordinal++){const value=s.artifacts[ordinal]!,id=await header(p,"artifacts",ordinal,value);await projectArtifactRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.validations.length;ordinal++){const value=s.validations[ordinal]!,id=await header(p,"validations",ordinal,value);await projectValidationRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.scenarios.length;ordinal++){const value=s.scenarios[ordinal]!,id=await header(p,"scenarios",ordinal,value);await projectScenario(p,id,value)}
 for(let ordinal=0;ordinal<s.options.length;ordinal++){const value=s.options[ordinal]!,id=await header(p,"options",ordinal,value);await projectOptionEvaluation(p,id,value)}
 for(let ordinal=0;ordinal<s.decisions.length;ordinal++){const value=s.decisions[ordinal]!,id=await header(p,"decisions",ordinal,value);await projectDecision(p,id,value)}
 for(let ordinal=0;ordinal<s.priorities.length;ordinal++){const value=s.priorities[ordinal]!,id=await header(p,"priorities",ordinal,value);await projectPriorityRecord(p,id,value)}
 for(let ordinal=0;ordinal<s.risks.length;ordinal++){const value=s.risks[ordinal]!,id=await header(p,"risks",ordinal,value);await projectRisk(p,id,value)}
 for(let ordinal=0;ordinal<s.conflicts.length;ordinal++){const value=s.conflicts[ordinal]!,id=await header(p,"conflicts",ordinal,value);await projectConflict(p,id,value)}
 for(let ordinal=0;ordinal<s.requirements.length;ordinal++){const value=s.requirements[ordinal]!,id=await header(p,"requirements",ordinal,value);await projectRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.delivery.length;ordinal++){const value=s.delivery[ordinal]!,id=await header(p,"delivery",ordinal,value);await projectDeliveryConstraint(p,id,value)}
 for(let ordinal=0;ordinal<s.sustainability.length;ordinal++){const value=s.sustainability[ordinal]!,id=await header(p,"sustainability",ordinal,value);await projectSustainabilityRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.resilience.length;ordinal++){const value=s.resilience[ordinal]!,id=await header(p,"resilience",ordinal,value);await projectResilienceRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.costs.length;ordinal++){const value=s.costs[ordinal]!,id=await header(p,"costs",ordinal,value);await projectCostRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.growth.length;ordinal++){const value=s.growth[ordinal]!,id=await header(p,"growth",ordinal,value);await projectGrowthPlan(p,id,value)}
 for(let ordinal=0;ordinal<s.wayfinding.length;ordinal++){const value=s.wayfinding[ordinal]!,id=await header(p,"wayfinding",ordinal,value);await projectWayfindingRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.schedules.length;ordinal++){const value=s.schedules[ordinal]!,id=await header(p,"schedules",ordinal,value);await projectScheduleRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.flexibility.length;ordinal++){const value=s.flexibility[ordinal]!,id=await header(p,"flexibility",ordinal,value);await projectFlexibilityRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.infrastructure.length;ordinal++){const value=s.infrastructure[ordinal]!,id=await header(p,"infrastructure",ordinal,value);await projectInfrastructureRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.information.length;ordinal++){const value=s.information[ordinal]!,id=await header(p,"information",ordinal,value);await projectInformationRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.communication.length;ordinal++){const value=s.communication[ordinal]!,id=await header(p,"communication",ordinal,value);await projectCommunicationRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.siteContext.length;ordinal++){const value=s.siteContext[ordinal]!,id=await header(p,"siteContext",ordinal,value);await projectSiteContext(p,id,value)}
 for(let ordinal=0;ordinal<s.organizational.length;ordinal++){const value=s.organizational[ordinal]!,id=await header(p,"organizational",ordinal,value);await projectOrganizationalRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.services.length;ordinal++){const value=s.services[ordinal]!,id=await header(p,"services",ordinal,value);await projectServiceRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.safety.length;ordinal++){const value=s.safety[ordinal]!,id=await header(p,"safety",ordinal,value);await projectSafetyRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.security.length;ordinal++){const value=s.security[ordinal]!,id=await header(p,"security",ordinal,value);await projectSecurityRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.regulatory.length;ordinal++){const value=s.regulatory[ordinal]!,id=await header(p,"regulatory",ordinal,value);await projectRegulatoryRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.accessibility.length;ordinal++){const value=s.accessibility[ordinal]!,id=await header(p,"accessibility",ordinal,value);await projectAccessibilityRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.privacy.length;ordinal++){const value=s.privacy[ordinal]!,id=await header(p,"privacy",ordinal,value);await projectPrivacyRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.environmental.length;ordinal++){const value=s.environmental[ordinal]!,id=await header(p,"environmental",ordinal,value);await projectEnvironmentalRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.humanFactors.length;ordinal++){const value=s.humanFactors[ordinal]!,id=await header(p,"humanFactors",ordinal,value);await projectHumanFactorRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.storage.length;ordinal++){const value=s.storage[ordinal]!,id=await header(p,"storage",ordinal,value);await projectStorageRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.equipment.length;ordinal++){const value=s.equipment[ordinal]!,id=await header(p,"equipment",ordinal,value);await projectEquipment(p,id,value)}
 for(let ordinal=0;ordinal<s.resources.length;ordinal++){const value=s.resources[ordinal]!,id=await header(p,"resources",ordinal,value);await projectResource(p,id,value)}
 for(let ordinal=0;ordinal<s.flows.length;ordinal++){const value=s.flows[ordinal]!,id=await header(p,"flows",ordinal,value);await projectFlowRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.accessRules.length;ordinal++){const value=s.accessRules[ordinal]!,id=await header(p,"accessRules",ordinal,value);await projectAccessRule(p,id,value)}
 for(let ordinal=0;ordinal<s.operations.length;ordinal++){const value=s.operations[ordinal]!,id=await header(p,"operations",ordinal,value);await projectOperationalRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.adjacencies.length;ordinal++){const value=s.adjacencies[ordinal]!,id=await header(p,"adjacencies",ordinal,value);await projectAdjacency(p,id,value)}
 for(let ordinal=0;ordinal<s.processes.length;ordinal++){const value=s.processes[ordinal]!,id=await header(p,"processes",ordinal,value);await projectProcess(p,id,value)}
 for(let ordinal=0;ordinal<s.quantities.length;ordinal++){const value=s.quantities[ordinal]!,id=await header(p,"quantities",ordinal,value);await projectQuantityRequirement(p,id,value)}
 for(let ordinal=0;ordinal<s.relationships.length;ordinal++){const value=s.relationships[ordinal]!,id=await header(p,"relationships",ordinal,value);await projectRelationship(p,id,value)}
 for(let ordinal=0;ordinal<s.users.length;ordinal++){const value=s.users[ordinal]!,id=await header(p,"users",ordinal,value);await projectUserProfile(p,id,value)}
 for(let ordinal=0;ordinal<s.activities.length;ordinal++){const value=s.activities[ordinal]!,id=await header(p,"activities",ordinal,value);await projectActivity(p,id,value)}
 for(let ordinal=0;ordinal<s.functions.length;ordinal++){const value=s.functions[ordinal]!,id=await header(p,"functions",ordinal,value);await projectFunction(p,id,value)}
 for(let ordinal=0;ordinal<s.elements.length;ordinal++){const value=s.elements[ordinal]!,id=await header(p,"elements",ordinal,value);await projectProgramElement(p,id,value)}
 for(let ordinal=0;ordinal<s.stakeholders.length;ordinal++){const value=s.stakeholders[ordinal]!,id=await header(p,"stakeholders",ordinal,value);await projectStakeholder(p,id,value)}
 for(let ordinal=0;ordinal<s.auditEvents.length;ordinal++){const value=s.auditEvents[ordinal]!,id=await header(p,"auditEvents",ordinal,value);await projectAuditEvent(p,id,value)}
 return p.finish();
}
/** 📥️ Restores literal typed fields while admitting relation indexes and every owned row. */
export async function programSnapshotFromSqliteDatabase(database:SqliteDatabase,settings:ArtifactSqliteOptions={}):Promise<model.ProgramArtifact>{
 const tables=await artifactSqliteTables(database,PROGRAM_SQLITE_SCHEMA,settings),control=new NativeDecodeControl(settings.maxValueBytes??268435456,event=>{settings.onProgress?.({phase:"reconstructSnapshot",completed:event.completed,total:event.total});return !settings.signal?.aborted},settings.signal);
 await control.charge(2048);await control.admitSlots(tables.length,32);const names=new Map<string,readonly SqliteRow[]>();for(let index=0;index<tables.length;index++){await control.step();names.set(database.tables[index]!.name,tables[index]!)}const r=new Reader(names,control),root=await r.singleton("architect_document",2),schema=new Cursor(root).text(),meta=await restoreProgramMeta(r),project=await restoreProjectDefinition(r),governance=await restoreGovernance(r),knowledge=await restoreChild(r,"knowledge"),benchmarks=await restoreChild(r,"benchmarks"),traces=await r.list("architect_trace",1n,5,readTrace);
 const approvals=await r.register("approvals",restoreApprovalRecord);
 const meetings=await r.register("meetings",restoreMeetingRecord);
 const assumptions=await r.register("assumptions",restoreAssumption);
 const constraints=await r.register("constraints",restoreConstraintRecord);
 const complianceRecords=await r.register("complianceRecords",restoreComplianceRecord);
 const templates=await r.register("templates",restoreTemplateRecord);
 const knowledgePayload=await r.register("knowledgePayload",restoreKnowledgeRecord);
 const benchmarksPayload=await r.register("benchmarksPayload",restoreBenchmarkRecord);
 const issues=await r.register("issues",restoreIssue);
 const statusRecords=await r.register("statusRecords",restoreStatusRecord);
 const workshops=await r.register("workshops",restoreWorkshop);
 const surveys=await r.register("surveys",restoreSurvey);
 const searchFilters=await r.register("searchFilters",restoreSearchFilter);
 const collaboration=await r.register("collaboration",restoreCollaborationRecord);
 const analyses=await r.register("analyses",restoreAnalysisRecord);
 const reports=await r.register("reports",restoreReportRecord);
 const changes=await r.register("changes",restoreChangeRecord);
 const performance=await r.register("performance",restorePerformanceCriterion);
 const quality=await r.register("quality",restoreQualityRecord);
 const artifacts=await r.register("artifacts",restoreArtifactRecord);
 const validations=await r.register("validations",restoreValidationRecord);
 const scenarios=await r.register("scenarios",restoreScenario);
 const options=await r.register("options",restoreOptionEvaluation);
 const decisions=await r.register("decisions",restoreDecision);
 const priorities=await r.register("priorities",restorePriorityRecord);
 const risks=await r.register("risks",restoreRisk);
 const conflicts=await r.register("conflicts",restoreConflict);
 const requirements=await r.register("requirements",restoreRequirement);
 const delivery=await r.register("delivery",restoreDeliveryConstraint);
 const sustainability=await r.register("sustainability",restoreSustainabilityRequirement);
 const resilience=await r.register("resilience",restoreResilienceRequirement);
 const costs=await r.register("costs",restoreCostRequirement);
 const growth=await r.register("growth",restoreGrowthPlan);
 const wayfinding=await r.register("wayfinding",restoreWayfindingRequirement);
 const schedules=await r.register("schedules",restoreScheduleRequirement);
 const flexibility=await r.register("flexibility",restoreFlexibilityRequirement);
 const infrastructure=await r.register("infrastructure",restoreInfrastructureRequirement);
 const information=await r.register("information",restoreInformationRequirement);
 const communication=await r.register("communication",restoreCommunicationRequirement);
 const siteContext=await r.register("siteContext",restoreSiteContext);
 const organizational=await r.register("organizational",restoreOrganizationalRequirement);
 const services=await r.register("services",restoreServiceRequirement);
 const safety=await r.register("safety",restoreSafetyRequirement);
 const security=await r.register("security",restoreSecurityRequirement);
 const regulatory=await r.register("regulatory",restoreRegulatoryRequirement);
 const accessibility=await r.register("accessibility",restoreAccessibilityRequirement);
 const privacy=await r.register("privacy",restorePrivacyRequirement);
 const environmental=await r.register("environmental",restoreEnvironmentalRequirement);
 const humanFactors=await r.register("humanFactors",restoreHumanFactorRequirement);
 const storage=await r.register("storage",restoreStorageRequirement);
 const equipment=await r.register("equipment",restoreEquipment);
 const resources=await r.register("resources",restoreResource);
 const flows=await r.register("flows",restoreFlowRequirement);
 const accessRules=await r.register("accessRules",restoreAccessRule);
 const operations=await r.register("operations",restoreOperationalRequirement);
 const adjacencies=await r.register("adjacencies",restoreAdjacency);
 const processes=await r.register("processes",restoreProcess);
 const quantities=await r.register("quantities",restoreQuantityRequirement);
 const relationships=await r.register("relationships",restoreRelationship);
 const users=await r.register("users",restoreUserProfile);
 const activities=await r.register("activities",restoreActivity);
 const functions=await r.register("functions",restoreFunction);
 const elements=await r.register("elements",restoreProgramElement);
 const stakeholders=await r.register("stakeholders",restoreStakeholder);
 const auditEvents=await r.register("auditEvents",restoreAuditEvent);
 await r.finish();return{schema,meta,project,governance,knowledge,benchmarks,traces,approvals,meetings,assumptions,constraints,complianceRecords,templates,knowledgePayload,benchmarksPayload,issues,statusRecords,workshops,surveys,searchFilters,collaboration,analyses,reports,changes,performance,quality,artifacts,validations,scenarios,options,decisions,priorities,risks,conflicts,requirements,delivery,sustainability,resilience,costs,growth,wayfinding,schedules,flexibility,infrastructure,information,communication,siteContext,organizational,services,safety,security,regulatory,accessibility,privacy,environmental,humanFactors,storage,equipment,resources,flows,accessRules,operations,adjacencies,processes,quantities,relationships,users,activities,functions,elements,stakeholders,auditEvents};
}
