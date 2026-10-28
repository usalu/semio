import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {benchHostV1,benchObserverV1,benchPercentileV1,type BenchHostV1,type BenchLoadClassV1} from "../🟦️.ts";

const policy=schema.definitions.Policy.const,both=(english:string,german:string)=>`${english}\n${german}`;
const closed=(name:keyof typeof schema.definitions,value:unknown,message:string):void=>{if(validateJsonSchemaSubset(schema.definitions[name],value,schema).length)throw Error(message);};

/** 🥼️ A text in every language the acceptance speaks. */
export type AcceptanceTextV1=Readonly<{en:string;de:string}>;
/** 🥋️ The kinds of scenario: a hot no-op, an edit of another agent's file, a one-line edit with its byte-exact revert. */
export type AcceptanceKindV1="noop"|"peer"|"edit";
/** 🤿️ What one official run is for: the unjudged prime, the judged hot runs, the baseline before a peer edit, the run the peer edit lands in, the run after its restore, the edit run and the revert run. */
export type AcceptanceRoleV1="prime"|"hot"|"control"|"peer"|"follow"|"edit"|"revert";
/** 🥷️ What a budget bounds: the exact wall clock, the physical footprint of the process tree in megabytes, the actions the tool executed. */
export type AcceptanceMetricV1="wallMs"|"peakFootprintMb"|"executedActions";
/** 🦂️ How the samples of a role are condensed to one number. */
export type AcceptanceStatisticV1="max"|"min"|"p50";
/** 🦁️ The verdict of a row; `unproven` is neither a pass nor a breach. */
export type AcceptanceStatusV1="pass"|"fail"|"unproven";
/** 🐯️ Why a row stays unproven. */
export type AcceptanceReasonV1="no-samples"|"run-failed"|"clock-grain"|"host-saturated"|"resident-only"|"not-landed"|"foreign-write"|"skipped"|"aborted";
/** 🐺️ A file one product edit or peer edit touches and the comment syntax that appends a line to it harmlessly. */
export type AcceptanceFileV1=Readonly<{file:string;comment:"//"|"#"}>;
/** 🐻️ A product: the target the official route builds and the source file whose one-line edit it must absorb. */
export type AcceptanceProductV1=Readonly<{id:string;title:AcceptanceTextV1;target:string;edit:AcceptanceFileV1}>;
/** 🐼️ The peer edits: the output line that tells the run is inside its graph phase (the edit lands then) and the two groups of files other agents touch. */
export type AcceptancePeerV1=Readonly<{trigger:string;unrelated:readonly AcceptanceFileV1[];scheduling:readonly AcceptanceFileV1[]}>;
/** 🐰️ One scenario of one product. */
export type AcceptanceScenarioV1=Readonly<{id:string;kind:AcceptanceKindV1;product:string;title:AcceptanceTextV1;hot?:number;files?:"unrelated"|"scheduling";timeoutMs:number}>;
/** 🐱️ One declared budget: the statistic of a metric over the runs of a role of a scenario must not exceed the maximum; `slo` names the objective of the bench table it restates. */
export type AcceptanceBudgetV1=Readonly<{id:string;title:AcceptanceTextV1;scenario:string;role:AcceptanceRoleV1;metric:AcceptanceMetricV1;statistic:AcceptanceStatisticV1;maximum:number;enforcement:"blocking"|"advisory";source:string;slo?:string;note?:AcceptanceTextV1}>;
/** 🐴️ The authored table: labels, products, peer edits, scenarios and budgets. */
export type AcceptanceSetV1=Readonly<{version:1;labels:Readonly<Record<string,AcceptanceTextV1>>;products:readonly AcceptanceProductV1[];peer:AcceptancePeerV1;scenarios:readonly AcceptanceScenarioV1[];budgets:readonly AcceptanceBudgetV1[]}>;
/** 🐵️ The clock a wall time was taken with: an exact one, or a poll that rounds up to its grain. */
export type AcceptanceClockV1=Readonly<{kind:"exact"|"poll";grainMs:number}>;
/** 🐷️ One official run: its command, exit, exact wall clock, peak memory with the measure it is in, the actions it executed, whether the graph came from the memo, the projects it re-inferred, its phases and the load of the host at its start. */
export type AcceptanceRecordV1=Readonly<{scenario:string;role:AcceptanceRoleV1;index:number;command:string;exit:number|null;killed:"timeout"|"abort"|null;wallMs:number;clock:AcceptanceClockV1;peakMb:number|null;measure:"footprint"|"resident"|null;executed:number|null;served:boolean;inferred:number;phases:Readonly<Record<string,number>>;loadPerCore:number|null}>;
/** 🐮️ How one scenario ended: whether it ran, why it did not, whether its edits were restored byte for byte, whether the peer edit landed while a run executed and whether a foreign write was merged. */
export type AcceptanceOutcomeV1=Readonly<{scenario:string;status:"ran"|"skipped"|"failed";reason:string|null;restored:boolean|null;landed:boolean|null;merged:boolean|null}>;
/** 🐲️ One verdict row of a budget or of an invariant. */
export type AcceptanceRowV1=Readonly<{id:string;scenario:string;kind:"budget"|"invariant";role:AcceptanceRoleV1|null;metric:AcceptanceMetricV1|null;statistic:AcceptanceStatisticV1|null;value:number|null;maximum:number|null;n:number;status:AcceptanceStatusV1;reason:AcceptanceReasonV1|null;enforcement:"blocking"|"advisory";bound:"exact"|"upper"}>;
/** 🐦️ The verdicts counted. */
export type AcceptanceTallyV1=Readonly<{pass:number;fail:number;unproven:number;blockingFailed:number;advisoryFailed:number;provenBlocking:number}>;
/** 🐔️ Everything one acceptance run measured and judged; it is also the stored result of the nightly job. */
export type AcceptanceReportV1=Readonly<{version:1;kind:"acceptance-report";startedAt:string;finishedAt:string;host:BenchHostV1;hostAfter:BenchHostV1;language:"en"|"de";selected:readonly string[];records:readonly AcceptanceRecordV1[];outcomes:readonly AcceptanceOutcomeV1[];rows:readonly AcceptanceRowV1[];tally:AcceptanceTallyV1;verdict:AcceptanceStatusV1}>;
/** 🐓️ What reading an output tells: the actions the tool executed (`null` when it printed no summary), whether the graph was served from the memo, the projects re-inferred and the wall time per phase. */
export type AcceptanceObservationV1=Readonly<{executed:number|null;served:boolean;inferred:number;phases:Readonly<Record<string,number>>}>;
/** 🐐️ Reads the output lines of a command one by one and keeps only counters. */
export type AcceptanceObserverV1=Readonly<{feed(at:number,line:string):void;finish(wallMs:number):AcceptanceObservationV1}>;

/** 🐏️ Admits the authored table and lists every inconsistency: duplicate ids, a scenario of an unknown product, a budget of an unknown scenario or of a role the scenario never runs, a statistic its sample count cannot back, a peer scenario without its file group, an identical English and German text, a path beyond the limit. */
export function acceptanceSetIssuesV1(value:unknown):string[]{
 const issues=validateJsonSchemaSubset(schema.definitions.Set,value,schema).map(String);
 if(issues.length)return issues;
 const set=value as AcceptanceSetV1,found:string[]=[],unique=(kind:string,ids:readonly string[])=>{const seen=new Set<string>();for(const id of ids){if(seen.has(id))found.push(`duplicate ${kind} ${id}`);seen.add(id);}};
 unique("product",set.products.map(row=>row.id));unique("scenario",set.scenarios.map(row=>row.id));unique("budget",set.budgets.map(row=>row.id));
 const products=new Set(set.products.map(row=>row.id)),scenarios=new Map(set.scenarios.map(row=>[row.id,row] as const)),rolesOf=policy.rolesOfKind as Record<string,readonly string[]>;
 const distinct=(where:string,text:AcceptanceTextV1)=>{if(text.en===text.de)found.push(`${where}: the English and the German text are identical`);};
 for(const [key,text] of Object.entries(set.labels))distinct(`label ${key}`,text);
 for(const product of set.products){distinct(`product ${product.id}`,product.title);if(product.edit.file.length>policy.maximumPathLength)found.push(`${product.id}: path exceeds ${policy.maximumPathLength}`);}
 for(const file of [...set.peer.unrelated,...set.peer.scheduling])if(file.file.length>policy.maximumPathLength)found.push(`${file.file}: path exceeds ${policy.maximumPathLength}`);
 try{new RegExp(set.peer.trigger,"u");}catch{found.push("peer trigger is not a regular expression");}
 for(const scenario of set.scenarios){
  distinct(`scenario ${scenario.id}`,scenario.title);
  if(!products.has(scenario.product))found.push(`${scenario.id}: unknown product ${scenario.product}`);
  if((scenario.kind==="noop")!==(scenario.hot!==undefined))found.push(`${scenario.id}: hot belongs to no-op scenarios only`);
  if((scenario.kind==="peer")!==(scenario.files!==undefined))found.push(`${scenario.id}: files belong to peer scenarios only`);
 }
 for(const budget of set.budgets){
  distinct(`budget ${budget.id}`,budget.title);
  const scenario=scenarios.get(budget.scenario);
  if(!scenario){found.push(`${budget.id}: unknown scenario ${budget.scenario}`);continue;}
  if(!rolesOf[scenario.kind]!.includes(budget.role)||budget.role==="prime")found.push(`${budget.id}: ${scenario.id} never runs a judged ${budget.role}`);
  if(budget.metric==="executedActions"&&!Number.isInteger(budget.maximum))found.push(`${budget.id}: a count has a whole maximum`);
  const samples=scenario.kind==="noop"&&budget.role==="hot"?scenario.hot!:1;
  if(samples<(policy.minimumSamples as Record<string,number>)[budget.statistic]!)found.push(`${budget.id}: ${budget.statistic} needs ${(policy.minimumSamples as Record<string,number>)[budget.statistic]} samples but ${scenario.id} runs ${samples}`);
  if(budget.note)distinct(`budget ${budget.id} note`,budget.note);
 }
 return found;
}

/** 🐗️ Selects scenarios by id, `kind:<kind>`, `product:<id>` or `all`, in the order noop, peer, edit and the declared order inside a kind. Scenarios that edit files are only run with `inPlace`: an explicit selection of one without it is refused, `all` leaves them out and names them as skipped. */
export function acceptanceSelectV1(set:AcceptanceSetV1,ids:readonly string[]|"all",inPlace:boolean):Readonly<{run:readonly AcceptanceScenarioV1[];skipped:readonly AcceptanceScenarioV1[]}>{
 const chosen=new Set<string>();
 if(ids==="all")for(const scenario of set.scenarios)chosen.add(scenario.id);
 else for(const id of ids){
  const matches=id.startsWith("kind:")?set.scenarios.filter(row=>row.kind===id.slice(5)):id.startsWith("product:")?set.scenarios.filter(row=>row.product===id.slice(8)):set.scenarios.filter(row=>row.id===id);
  if(!matches.length)throw Error(both(`Unknown scenario: ${id} (an id, kind:<${policy.kinds.join("|")}>, product:<id> or all)`,`Unbekanntes Szenario: ${id} (eine Kennung, kind:<${policy.kinds.join("|")}>, product:<id> oder all)`));
  for(const row of matches)chosen.add(row.id);
 }
 const rank=(kind:AcceptanceKindV1)=>policy.kinds.indexOf(kind),ordered=set.scenarios.filter(row=>chosen.has(row.id)).map((row,at)=>({row,at})).sort((left,right)=>rank(left.row.kind)-rank(right.row.kind)||left.at-right.at).map(entry=>entry.row);
 const editing=ordered.filter(row=>row.kind!=="noop");
 if(!inPlace&&ids!=="all"&&editing.length)throw Error(both(`${editing.map(row=>row.id).join(", ")} edit and restore source files of this checkout; pass --in-place on a checkout nobody else edits (a CI runner, a private clone)`,`${editing.map(row=>row.id).join(", ")} ändern Quelldateien dieses Checkouts und setzen sie zurück; --in-place nur auf einem Checkout übergeben, den niemand sonst bearbeitet (CI-Runner, private Kopie)`));
 return Object.freeze({run:inPlace?ordered:ordered.filter(row=>row.kind==="noop"),skipped:inPlace?[]:editing});
}

/** 🐖️ The line a peer or product edit appends: the comment syntax of the file, the marker and a nonce. */
export function acceptanceLineV1(comment:"//"|"#",nonce:string):string{
 if(!new RegExp(policy.edit.nonce,"u").test(nonce))throw Error("Closed acceptance nonce required");
 return `${comment} ${policy.edit.marker} ${nonce}\n`;
}

/** 🐆️ Appends the edit line to the bytes of a file: a missing final newline is added first, no existing byte changes, the result always differs from the input. Reports whether it added the newline. */
export function acceptanceMutateV1(original:Uint8Array,line:string):Readonly<{bytes:Uint8Array;addedNewline:boolean}>{
 const addedNewline=original.length>0&&original[original.length-1]!==10,tail=Buffer.from((addedNewline?"\n":"")+line,"utf8");
 return Object.freeze({bytes:Buffer.concat([original,tail]),addedNewline});
}

/** 🐅️ Takes the edit line back out of the bytes a file holds now: exact when the file still is what the edit wrote; a foreign write after the edit is kept and reported as merged. */
export function acceptanceRestoreV1(original:Uint8Array,edited:Readonly<{bytes:Uint8Array;addedNewline:boolean}>,line:string,current:Uint8Array|null):Readonly<{bytes:Uint8Array|null;exact:boolean;merged:boolean}>{
 if(current===null)return Object.freeze({bytes:null,exact:false,merged:true});
 if(Buffer.from(current).equals(Buffer.from(edited.bytes)))return Object.freeze({bytes:original,exact:true,merged:false});
 const text=Buffer.from(current),needle=Buffer.from((edited.addedNewline?"\n":"")+line,"utf8"),at=text.indexOf(needle);
 if(at<0)return Object.freeze({bytes:null,exact:Buffer.from(current).equals(Buffer.from(original)),merged:!Buffer.from(current).equals(Buffer.from(original))});
 const rest=Buffer.concat([text.subarray(0,at),text.subarray(at+needle.length)]);
 return Object.freeze({bytes:rest,exact:rest.equals(Buffer.from(original)),merged:!rest.equals(Buffer.from(original))});
}

/** 🐄️ Digests bytes with sha256, the independent identity of a restored file. */
export function acceptanceDigestV1(bytes:Uint8Array):string{return createHash("sha256").update(bytes).digest("hex");}

/** 🐉️ Reads the output of one command line by line, as the bench does (executed actions, re-inferred projects, wall time per phase), and notes whether the graph came from the memo; nothing but counters is kept. */
export function acceptanceObserverV1():AcceptanceObserverV1{
 const inner=benchObserverV1(),served=policy.served.map(pattern=>new RegExp(pattern,"u"));let hit=false;
 return Object.freeze({
  feed(at:number,line:string):void{inner.feed(at,line);if(!hit&&served.some(pattern=>pattern.test(line)))hit=true;},
  finish(wallMs:number):AcceptanceObservationV1{const seen=inner.finish(wallMs);return Object.freeze({executed:seen.executed,served:hit,inferred:seen.inferred,phases:seen.phases});}
 });
}

/** 🐊️ Builds the record of one run from what the spawner measured and what the output told. */
export function acceptanceRecordV1(input:Readonly<{scenario:string;role:AcceptanceRoleV1;index:number;command:string;exit:number|null;killed:"timeout"|"abort"|null;wallMs:number;clock:AcceptanceClockV1;peakMb:number|null;measure:"footprint"|"resident"|null;loadPerCore:number|null}>,observation:AcceptanceObservationV1):AcceptanceRecordV1{
 const record={...input,executed:observation.executed,served:observation.served,inferred:observation.inferred,phases:observation.phases};
 closed("Record",record,"Closed acceptance record required");
 return Object.freeze(record);
}

/** 🐡️ Replays a recorded output as a run: the lines are read with the observer, the recorded exit, wall clock and memory complete the record. */
export function acceptanceReplayV1(input:Readonly<{scenario:string;role:AcceptanceRoleV1;index:number;command:string;exit:number;wallMs:number;clock:AcceptanceClockV1;peakMb:number|null;measure:"footprint"|"resident"|null;loadPerCore:number|null}>,lines:Iterable<string>):AcceptanceRecordV1{
 const observer=acceptanceObserverV1();
 for(const line of lines)observer.feed(0,line);
 return acceptanceRecordV1({...input,killed:null},observer.finish(input.wallMs));
}

/** 🐥️ Expands a recorded log in the run-length form of the fixtures: a plain string is one line, `[line, n]` is the line n times. */
export function acceptanceExpandV1(lines:readonly (string|readonly [string,number])[]):string[]{
 return lines.flatMap(row=>typeof row==="string"?[row]:Array.from({length:row[1]},()=>row[0]));
}

const loadClassOf=(perCore:number|null):BenchLoadClassV1=>perCore===null?"unknown":perCore<policy.quietLoadPerCore?"quiet":perCore>=policy.saturatedLoadPerCore?"saturated":"busy";
const statisticOf=(statistic:AcceptanceStatisticV1,values:readonly number[]):number=>{
 const sorted=[...values].sort((left,right)=>left-right);
 return statistic==="max"?sorted[sorted.length-1]!:statistic==="min"?sorted[0]!:benchPercentileV1(sorted,0.5);
};
const measured=(record:AcceptanceRecordV1,metric:AcceptanceMetricV1):number|null=>metric==="wallMs"?record.wallMs:metric==="peakFootprintMb"?record.peakMb:record.executed;

/** 🐩️ Judges one budget against the runs of its role. A value within the maximum passes (a poll clock and a resident-only memory read are upper bounds, so a pass stays a pass); a value beyond it fails unless the grain of the clock still allows a pass, only the resident set was read, or the host was saturated while a time was taken. */
export function acceptanceJudgeBudgetV1(budget:AcceptanceBudgetV1,records:readonly AcceptanceRecordV1[],outcome:AcceptanceOutcomeV1|undefined):AcceptanceRowV1{
 const head={id:budget.id,scenario:budget.scenario,kind:"budget" as const,role:budget.role,metric:budget.metric,statistic:budget.statistic,maximum:budget.maximum,enforcement:budget.enforcement};
 const unproven=(reason:AcceptanceReasonV1,n=0,value:number|null=null,bound:"exact"|"upper"="exact"):AcceptanceRowV1=>Object.freeze({...head,value,n,status:"unproven" as const,reason,bound});
 if(outcome?.status==="skipped")return unproven("skipped");
 const mine=records.filter(row=>row.scenario===budget.scenario&&row.role===budget.role);
 if(!mine.length)return unproven(outcome?.status==="failed"?"run-failed":"no-samples");
 const good=mine.filter(row=>row.exit===0&&row.killed===null),used=good.flatMap(row=>{const value=measured(row,budget.metric);return value===null?[]:[{row,value}];});
 if(!good.length)return unproven("run-failed",mine.length);
 if(used.length<(policy.minimumSamples as Record<string,number>)[budget.statistic]!)return unproven("no-samples",used.length);
 const value=statisticOf(budget.statistic,used.map(entry=>entry.value)),n=used.length;
 const slack=budget.metric==="wallMs"?Math.max(...used.map(entry=>entry.row.clock.kind==="poll"?entry.row.clock.grainMs:0)):0,resident=budget.metric==="peakFootprintMb"&&used.some(entry=>entry.row.measure==="resident"),bound=slack>0||resident?"upper" as const:"exact" as const;
 if(value<=budget.maximum)return Object.freeze({...head,value,n,status:"pass" as const,reason:null,bound});
 if(resident)return unproven("resident-only",n,value,bound);
 if(slack>0&&value-slack<=budget.maximum)return unproven("clock-grain",n,value,bound);
 if(budget.metric==="wallMs"&&used.some(entry=>loadClassOf(entry.row.loadPerCore)==="saturated"))return unproven("host-saturated",n,value,bound);
 return Object.freeze({...head,value,n,status:"fail" as const,reason:null,bound});
}

/** 🦃️ Judges a whole run: every budget of every scenario that was selected, plus the invariants of each scenario that ran (every official run exits with 0, edited files are restored byte for byte, the peer edit landed while a run executed). */
export function acceptanceJudgeV1(set:AcceptanceSetV1,selected:readonly string[],records:readonly AcceptanceRecordV1[],outcomes:readonly AcceptanceOutcomeV1[]):AcceptanceRowV1[]{
 const rows:AcceptanceRowV1[]=[],byOutcome=new Map(outcomes.map(row=>[row.scenario,row] as const)),chosen=new Set(selected);
 for(const scenario of set.scenarios){
  if(!chosen.has(scenario.id))continue;
  const outcome=byOutcome.get(scenario.id);
  for(const budget of set.budgets)if(budget.scenario===scenario.id)rows.push(acceptanceJudgeBudgetV1(budget,records,outcome));
  if(!outcome||outcome.status==="skipped")continue;
  const mine=records.filter(row=>row.scenario===scenario.id),broken=mine.filter(row=>row.exit!==0||row.killed!==null).length,invariant=(suffix:string,status:AcceptanceStatusV1,reason:AcceptanceReasonV1|null,value:number|null,maximum:number|null,n:number):AcceptanceRowV1=>Object.freeze({id:`${scenario.id}-${suffix}`,scenario:scenario.id,kind:"invariant" as const,role:null,metric:null,statistic:null,value,maximum,n,status,reason,enforcement:"blocking" as const,bound:"exact" as const});
  rows.push(invariant("exit",broken?"fail":mine.length?"pass":"unproven",mine.length?null:"no-samples",broken,0,mine.length));
  if(scenario.kind!=="noop")rows.push(outcome.restored===null?invariant("restored","unproven","no-samples",null,null,0):outcome.merged?invariant("restored","unproven","foreign-write",null,null,1):invariant("restored",outcome.restored?"pass":"fail",null,outcome.restored?1:0,1,1));
  if(scenario.kind==="peer")rows.push(outcome.landed===null?invariant("landed","unproven","no-samples",null,null,0):invariant("landed",outcome.landed?"pass":"unproven",outcome.landed?null:"not-landed",outcome.landed?1:0,1,1));
 }
 return rows;
}

/** 🥔️ Counts the verdicts; a failed blocking row is what fails the run. */
export function acceptanceTallyV1(rows:readonly AcceptanceRowV1[]):AcceptanceTallyV1{
 const count=(status:AcceptanceStatusV1)=>rows.filter(row=>row.status===status).length;
 return Object.freeze({pass:count("pass"),fail:count("fail"),unproven:count("unproven"),blockingFailed:rows.filter(row=>row.status==="fail"&&row.enforcement==="blocking").length,advisoryFailed:rows.filter(row=>row.status==="fail"&&row.enforcement==="advisory").length,provenBlocking:rows.filter(row=>row.kind==="budget"&&row.status==="pass"&&row.enforcement==="blocking").length});
}

/** 🥕️ The verdict of a tally: a broken blocking budget fails; with none broken the run passes only when at least one blocking budget was proven, so a run that proved nothing cannot go green. */
export function acceptanceVerdictV1(tally:AcceptanceTallyV1):AcceptanceStatusV1{return tally.blockingFailed>0?"fail":tally.provenBlocking>0?"pass":"unproven";}

/** 🥛️ Builds the closed report of a run. */
export function acceptanceReportV1(input:Readonly<{startedAt:string;finishedAt:string;host:BenchHostV1;hostAfter:BenchHostV1;language:"en"|"de";selected:readonly string[];records:readonly AcceptanceRecordV1[];outcomes:readonly AcceptanceOutcomeV1[];rows:readonly AcceptanceRowV1[]}>):AcceptanceReportV1{
 const tally=acceptanceTallyV1(input.rows),report={version:1 as const,kind:"acceptance-report" as const,...input,tally,verdict:acceptanceVerdictV1(tally)};
 closed("Report",report,"Closed acceptance report required");
 return Object.freeze(report);
}

/** 🥯️ The failure a report makes the command raise, in both languages, or null when the run passes. */
export function acceptanceFailureV1(report:AcceptanceReportV1):string|null{
 if(report.tally.blockingFailed>0)return both(`${report.tally.blockingFailed} blocking acceptance budgets are broken`,`${report.tally.blockingFailed} verbindliche Abnahmebudgets sind verletzt`);
 if(report.verdict==="unproven")return both("No blocking acceptance budget was proven","Kein verbindliches Abnahmebudget wurde bewiesen");
 return null;
}

/** 🍖️ Describes the host with the bench's host class and load classes. */
export function acceptanceHostV1(input:Parameters<typeof benchHostV1>[0]):BenchHostV1{return benchHostV1(input);}

const text=(set:AcceptanceSetV1,key:string,language:"en"|"de")=>set.labels[key]?.[language]??key;
const trim=(value:number,digits:number)=>String(Number(value.toFixed(digits)));
/** 🍗️ Formats a value for its metric: seconds with one decimal for times, whole megabytes, whole counts. */
export function acceptanceFormatV1(metric:AcceptanceMetricV1|null,value:number|null):string{
 if(value===null)return "-";
 if(metric==="wallMs")return `${trim(value/1000,1)} s`;
 if(metric==="peakFootprintMb")return `${Math.round(value)} MB`;
 return String(Math.round(value));
}
const grid=(head:readonly string[],rows:readonly (readonly string[])[],left:readonly number[],format:"text"|"markdown"):string[]=>{
 if(format==="markdown")return [`| ${head.join(" | ")} |`,`| ${head.map((_,at)=>left.includes(at)?":--":"--:").join(" | ")} |`,...rows.map(row=>`| ${row.map(cell=>cell.replaceAll("|","\\|")).join(" | ")} |`)];
 const width=head.map((cell,at)=>Math.max([...cell].length,...rows.map(row=>[...row[at]!].length))),pad=(cell:string,at:number)=>left.includes(at)?cell+" ".repeat(width[at]!-[...cell].length):" ".repeat(width[at]!-[...cell].length)+cell;
 return [head.map(pad).join("  "),...rows.map(row=>row.map(pad).join("  "))];
};
const heading=(value:string,format:"text"|"markdown",level=2)=>format==="markdown"?`${"#".repeat(level)} ${value}`:value;

/** 🍟️ Renders a report in a language as plain text or markdown: the host, the verdict, the table of budgets and invariants with value, maximum, sample count and status, the reasons of everything unproven, every official run and the outcome of every scenario. */
export function renderAcceptanceReportV1(report:AcceptanceReportV1,set:AcceptanceSetV1,language:"en"|"de",format:"text"|"markdown"):string{
 const t=(key:string)=>text(set,key,language),out:string[]=[],budgets=new Map(set.budgets.map(row=>[row.id,row] as const)),scenarios=new Map(set.scenarios.map(row=>[row.id,row] as const));
 const titleOf=(row:AcceptanceRowV1)=>row.kind==="budget"?budgets.get(row.id)?.title[language]??row.id:t("invariant"+row.id.slice(row.scenario.length+1).replace(/^./u,letter=>letter.toUpperCase()));
 out.push(heading(t("title"),format,1),"",`${t("started")}: ${report.startedAt}, ${t("finished")}: ${report.finishedAt}`,`${t("host")}: ${report.host.hostClass}, ${t("cpus")}: ${report.host.cpus}, ${t("memory")}: ${Math.round(report.host.memoryBytes/1048576)} MB, ${t("load")}: ${report.host.loadPerCore===null?"-":trim(report.host.loadPerCore,2)} (${t(report.host.loadClass)}) -> ${report.hostAfter.loadPerCore===null?"-":trim(report.hostAfter.loadPerCore,2)} (${t(report.hostAfter.loadClass)})`,"");
 const tally=report.tally;
 out.push(`${t("summary")}: ${t("pass")} ${tally.pass}, ${t("fail")} ${tally.fail}, ${t("unproven")} ${tally.unproven}; ${t("blockingFailed")}: ${tally.blockingFailed}; ${t("advisoryFailed")}: ${tally.advisoryFailed}; ${t("provenBlocking")}: ${tally.provenBlocking}`,"");
 const verdict=t(report.verdict==="pass"?"verdictPass":report.verdict==="fail"?"verdictFail":"verdictUnproven");
 out.push(format==="markdown"?`**${verdict}**`:verdict,"");
 const rows=report.rows.map(row=>[titleOf(row),row.scenario,row.role===null?"-":t(row.role),row.statistic??"-",`${row.bound==="upper"&&row.value!==null?"≤ ":""}${acceptanceFormatV1(row.metric,row.value)}`,row.maximum===null?"-":acceptanceFormatV1(row.metric,row.maximum),String(row.n),`${t(row.status)}${row.enforcement==="blocking"?" !":""}`]);
 out.push(...grid([t("budget"),t("scenario"),t("run"),t("statistic"),t("value"),t("maximum"),t("samples"),t("status")],rows,[0,1,2,3,7],format));
 const why=report.rows.filter(row=>row.reason!==null);
 if(why.length){out.push("",heading(t("notes"),format));for(const row of why)out.push(`- ${titleOf(row)} / ${row.scenario}: ${t(row.reason!)}`);}
 const noted=set.budgets.filter(row=>row.note&&report.rows.some(entry=>entry.id===row.id&&entry.status!=="pass"));
 for(const row of noted)out.push(`- ${row.title[language]}: ${row.note![language]}`);
 if(report.records.length){
  out.push("",heading(t("runs"),format));
  out.push(...grid([t("scenario"),t("run"),"#",t("exit"),t("wall"),t("footprint"),t("executed"),t("memo"),t("inferred")],report.records.map(row=>[row.scenario,t(row.role),String(row.index+1),row.exit===null?(row.killed??"-"):String(row.exit),`${row.clock.kind==="poll"?"≤ ":""}${acceptanceFormatV1("wallMs",row.wallMs)}`,row.peakMb===null?"-":`${row.measure==="resident"?"≤ ":""}${acceptanceFormatV1("peakFootprintMb",row.peakMb)}`,row.executed===null?"-":String(row.executed),t(row.served?"yes":"no"),String(row.inferred)]),[0,1],format));
 }
 if(report.outcomes.length){
  out.push("",heading(t("outcomes"),format));
  out.push(...grid([t("scenario"),t("status"),t("restored"),t("landed"),t("notes")],report.outcomes.map(row=>[scenarios.get(row.scenario)?.title[language]??row.scenario,t(row.status),row.restored===null?"-":t(row.restored?"yes":"no"),row.landed===null?"-":t(row.landed?"yes":"no"),row.status==="skipped"?t("skipped-reason"):row.reason??""]),[0,1,4],format));
 }
 out.push("",t("noteMeasure"),t("noteBounds"),t("noteMarks"));
 return out.join("\n");
}

/** 🍡️ Renders the scenarios and their budgets in a language: id, title, role, metric, statistic, maximum, enforcement and source. */
export function renderAcceptanceListV1(set:AcceptanceSetV1,language:"en"|"de",format:"text"|"markdown"):string{
 const t=(key:string)=>text(set,key,language);
 const rows=set.budgets.map(row=>[row.id,row.title[language],row.scenario,t(row.role),row.statistic,acceptanceFormatV1(row.metric,row.maximum),t(row.enforcement),row.source]);
 return [heading(t("list"),format,1),"",...grid([t("budget"),t("name"),t("scenario"),t("run"),t("statistic"),t("maximum"),t("enforcement"),t("source")],rows,[0,1,2,3,4,6,7],format)].join("\n");
}

