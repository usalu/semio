import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🔧️ The build implementations the neutral entry can select. */
export type CiToolV1="nx"|"bazel";

/** 📈️ Build activity the neutral cache status records (Bazel only). */
export type CiActivityV1=Readonly<{builds:number;actions:number;executed:number;cacheHits:number}>;

/** 🗄️ The CacheStatus document `cache status` prints, shaped by `🔣️.json`. */
export type CiCacheStatusV1=Readonly<{version:1;tool:CiToolV1;available:boolean;directory:string|null;entries:number;bytes:number;activity?:CiActivityV1}>;

/** 📉️ What one stretch of work added to the cache. */
export type CiCacheDeltaV1=Readonly<{tool:CiToolV1;entries:number;bytes:number;builds:number;actions:number;executed:number;cacheHits:number;hitRate:number|null}>;

/** ✅️ The verdict of one cache proof. */
export type CiProofV1=Readonly<{tool:CiToolV1;ok:boolean;rule:"log-marker"|"activity"|"spawn-marker"|"none";entries:readonly number[];bytes:readonly number[];problems:readonly string[]}>;

/** 🗺️ The DryRunPlan document `--dry-run` prints. */
export type CiPlanV1=Readonly<{tasks:Readonly<{roots:readonly string[];tasks:Readonly<Record<string,Readonly<{id:string;target:Readonly<{project:string;target:string;configuration?:string}>}>>>;dependencies:Readonly<Record<string,readonly string[]>>}>}>;

/** 🧮️ Size of a selection. */
export type CiPlanSummaryV1=Readonly<{tasks:number;selected:number;projects:number;targets:readonly string[]}>;

/** 🪞️ Whether two implementations selected the same tasks. */
export type CiParityV1=Readonly<{equal:boolean;first:number;second:number;onlyFirst:readonly string[];onlySecond:readonly string[]}>;

/** 🧪️ One row of the exported test results directory. */
export type CiTestResultV1=Readonly<{label:string;status:"passed"|"failed"|"flaky"|"timeout"|"error";cached:boolean;durationMillis:number;file:string}>;

/** 📚️ The `index.json` of the test results directory. */
export type CiTestResultsIndexV1=Readonly<{version:1;tool:CiToolV1;results:readonly CiTestResultV1[]}>;

/** 🧾️ Counts of one JUnit XML document. */
export type CiJUnitV1=Readonly<{tests:number;failures:number;errors:number;skipped:number}>;

/** 🧪️ Counts of all exported test results. */
export type CiTestsSummaryV1=Readonly<{files:number;tests:number;failures:number;errors:number;skipped:number;cached:number;failed:readonly string[]}>;

/** 🌍️ A report language; English first, German second. */
export type CiLanguageV1="en"|"de";

const definitions=schema.definitions;
const texts={
 en:{
  deltaTitle:"Cache activity ({label})",deltaRate:"hit rate {rate} ({hits} cache hits, {executed} executed, {actions} actions in {builds} builds)",deltaNoRate:"hit rate not available ({builds} builds recorded, nothing executed or restored)",deltaStore:"cache store grew by {entries} entries and {bytes} bytes",deltaNoActivity:"this tool records no build activity; hit rate not available",
  proofOk:"cache proof passed under {tool}: hit recognized by {rule} (entries {entries}, bytes {bytes})",proofFail:"cache proof failed under {tool}",proofNoGrowth:"the first run stored nothing (entries {entries}, bytes {bytes})",proofGrowth:"the second run stored new entries (entries {entries})",proofNoHit:"the second run was not served from the cache",
  planTitle:"Selection ({label})",planLine:"{selected} selected tasks of {tasks} planned, in {projects} projects, targets {targets}",planEmpty:"nothing is affected; no work is needed",
  parityOk:"selections agree: {first} selected tasks",parityFail:"selections differ: {first} against {second} selected tasks",parityOnlyFirst:"only in the first: {list}",parityOnlySecond:"only in the second: {list}",
  testsTitle:"Test results",testsLine:"{files} result files, {tests} tests, {failures} failures, {errors} errors, {skipped} skipped, {cached} restored from the cache",testsNone:"no test results were exported",testsFailed:"failed: {list}"
 },
 de:{
  deltaTitle:"Cache-Aktivität ({label})",deltaRate:"Trefferquote {rate} ({hits} Cache-Treffer, {executed} ausgeführt, {actions} Aktionen in {builds} Builds)",deltaNoRate:"Trefferquote nicht verfügbar ({builds} Builds aufgezeichnet, nichts ausgeführt oder wiederhergestellt)",deltaStore:"der Cache wuchs um {entries} Einträge und {bytes} Bytes",deltaNoActivity:"dieses Werkzeug zeichnet keine Build-Aktivität auf; Trefferquote nicht verfügbar",
  proofOk:"Cache-Beweis unter {tool} bestanden: Treffer erkannt durch {rule} (Einträge {entries}, Bytes {bytes})",proofFail:"Cache-Beweis unter {tool} fehlgeschlagen",proofNoGrowth:"der erste Lauf speicherte nichts (Einträge {entries}, Bytes {bytes})",proofGrowth:"der zweite Lauf speicherte neue Einträge (Einträge {entries})",proofNoHit:"der zweite Lauf wurde nicht aus dem Cache bedient",
  planTitle:"Auswahl ({label})",planLine:"{selected} ausgewählte Tasks von {tasks} geplanten, in {projects} Projekten, Targets {targets}",planEmpty:"nichts ist betroffen; es ist keine Arbeit nötig",
  parityOk:"Auswahlen stimmen überein: {first} ausgewählte Tasks",parityFail:"Auswahlen unterscheiden sich: {first} gegen {second} ausgewählte Tasks",parityOnlyFirst:"nur in der ersten: {list}",parityOnlySecond:"nur in der zweiten: {list}",
  testsTitle:"Testergebnisse",testsLine:"{files} Ergebnisdateien, {tests} Tests, {failures} Fehlschläge, {errors} Fehler, {skipped} übersprungen, {cached} aus dem Cache wiederhergestellt",testsNone:"es wurden keine Testergebnisse exportiert",testsFailed:"fehlgeschlagen: {list}"
 }
} as const;

/** 🔤️ The message keys every language must define. */
export const CI_TEXT_KEYS_V1:readonly string[]=Object.freeze(Object.keys(texts.en));

/** 🌍️ Picks the report language: the flag, then SEMIO_LANGUAGE, then the locale; there is no default, so an unresolved language is refused. */
export function resolveCiLanguageV1(flag:string|undefined,environment:Readonly<Record<string,string|undefined>>):CiLanguageV1{
 for(const value of [flag,environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LANG]){
  const prefix=(value??"").slice(0,2).toLowerCase();
  if(prefix==="en"||prefix==="de")return prefix;
 }
 throw Error("Language required: pass --lang en|de or set SEMIO_LANGUAGE");
}

/** 💬️ Renders one message with `{name}` placeholders. */
export function ciTextV1(language:CiLanguageV1,key:keyof typeof texts.en,values:Readonly<Record<string,string|number>>={}):string{
 return texts[language][key].replace(/\{([a-z]+)\}/gu,(_,name:string)=>String(values[name]??`{${name}}`));
}

const requireShape=(definition:string,value:unknown,message:string):void=>{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
};

function balancedEnd(source:string,start:number):number{
 let depth=0,quoted=false;
 for(let index=start;index<source.length;index++){
  const character=source[index];
  if(quoted){if(character==="\\")index++;else if(character==="\"")quoted=false;continue;}
  if(character==="\"")quoted=true;
  else if(character==="{")depth++;
  else if(character==="}"&&--depth===0)return index;
 }
 return -1;
}

/** 🔎️ Finds every top-level JSON object that begins at the start of a line of a console capture; log lines around it are ignored. */
export function extractJsonObjectsV1(text:string):readonly unknown[]{
 const source=text.replace(/\r\n/gu,"\n"),starts:number[]=source.startsWith("{")?[0]:[],found:unknown[]=[];
 for(let index=source.indexOf("\n{");index>=0;index=source.indexOf("\n{",index+1))starts.push(index+1);
 let covered=-1;
 for(const start of starts){
  if(start<=covered)continue;
  const end=balancedEnd(source,start);
  if(end<0)continue;
  try{found.push(JSON.parse(source.slice(start,end+1)));covered=end;}catch{}
 }
 return found;
}

function lastValid<T>(text:string,definition:string,message:string):T{
 const objects=extractJsonObjectsV1(text).filter(value=>validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema).length===0);
 if(!objects.length)throw Error(message);
 return objects[objects.length-1] as T;
}

/** 🗄️ Reads the last CacheStatus of a console capture. */
export function readCiCacheStatusV1(text:string):CiCacheStatusV1{return lastValid<CiCacheStatusV1>(text,"CacheStatus","Closed CacheStatus required");}

/** 🗺️ Reads the last DryRunPlan of a console capture. */
export function readCiPlanV1(text:string):CiPlanV1{return lastValid<CiPlanV1>(text,"DryRunPlan","Closed DryRunPlan required");}

/** 📉️ Subtracts two cache statuses; the hit rate is restored over restored plus executed work, and absent when the tool records no activity or did no work. */
export function ciCacheDeltaV1(before:CiCacheStatusV1,after:CiCacheStatusV1):CiCacheDeltaV1{
 if(before.tool!==after.tool)throw Error("Cache statuses of one tool required");
 const from=before.activity,to=after.activity,subtract=(key:keyof CiActivityV1)=>to===undefined?0:to[key]-(from?.[key]??0);
 const hits=subtract("cacheHits"),executed=subtract("executed"),worked=hits+executed;
 const delta:CiCacheDeltaV1={tool:after.tool,entries:after.entries-before.entries,bytes:after.bytes-before.bytes,builds:subtract("builds"),actions:subtract("actions"),executed,cacheHits:hits,hitRate:to===undefined||worked<=0?null:hits/worked};
 requireShape("Delta",delta,"Closed cache delta required");return Object.freeze(delta);
}

/** 📝️ Renders the cache delta as Markdown lines. */
export function renderCiCacheDeltaV1(delta:CiCacheDeltaV1,label:string,hasActivity:boolean,language:CiLanguageV1):string{
 const rate=delta.hitRate===null?undefined:`${(delta.hitRate*100).toFixed(1)}%`;
 const lines=[`### ${ciTextV1(language,"deltaTitle",{label})}`,""];
 if(!hasActivity)lines.push(`- ${ciTextV1(language,"deltaNoActivity")}`);
 else if(rate===undefined)lines.push(`- ${ciTextV1(language,"deltaNoRate",{builds:delta.builds})}`);
 else lines.push(`- ${ciTextV1(language,"deltaRate",{rate,hits:delta.cacheHits,executed:delta.executed,actions:delta.actions,builds:delta.builds})}`);
 lines.push(`- ${ciTextV1(language,"deltaStore",{entries:delta.entries,bytes:delta.bytes})}`,"");
 return lines.join("\n");
}

const nxMarkers=/read the output from the cache|\[local cache\]|\[existing outputs match the cache\]/u;
const bazelMarkers=/(?:disk|remote) cache hit/u;
const spawned=/[0-9]+ (?:linux-sandbox|darwin-sandbox|processwrapper-sandbox|windows-sandbox|local|worker)/u;

/** ✅️ Judges a cache proof: run 1 must grow the store, run 2 must not, and run 2 must be recognized as served from the cache by the implementation's own evidence. */
export function judgeCiCacheProofV1(input:Readonly<{tool:CiToolV1;statuses:readonly [CiCacheStatusV1,CiCacheStatusV1,CiCacheStatusV1];logs:readonly [string,string]}>):CiProofV1{
 const [s0,s1,s2]=input.statuses,[log1,log2]=input.logs,problems:string[]=[];
 const entries=[s0.entries,s1.entries,s2.entries],bytes=[s0.bytes,s1.bytes,s2.bytes];
 if(!(s1.entries>s0.entries&&s1.bytes>s0.bytes))problems.push("proofNoGrowth");
 if(s2.entries!==s1.entries)problems.push("proofGrowth");
 let rule:CiProofV1["rule"]="none";
 if(input.tool==="nx"){if(nxMarkers.test(log2))rule="log-marker";}
 else if(bazelMarkers.test(log2))rule="log-marker";
 else if(s0.activity!==undefined&&s1.activity!==undefined&&s2.activity!==undefined&&s1.activity.executed-s0.activity.executed>0&&s2.activity.executed===s1.activity.executed&&s2.activity.builds>s1.activity.builds)rule="activity";
 else if(spawned.test(log1)&&!spawned.test(log2)&&/Build completed successfully/u.test(log2))rule="spawn-marker";
 if(rule==="none")problems.push("proofNoHit");
 const proof:CiProofV1={tool:input.tool,ok:problems.length===0,rule,entries,bytes,problems:[...problems]};
 requireShape("Proof",proof,"Closed cache proof required");return Object.freeze(proof);
}

/** 📝️ Renders a proof verdict. */
export function renderCiProofV1(proof:CiProofV1,language:CiLanguageV1):string{
 const values={tool:proof.tool,rule:proof.rule,entries:proof.entries.join(" > "),bytes:proof.bytes.join(" > ")};
 if(proof.ok)return ciTextV1(language,"proofOk",values);
 return [ciTextV1(language,"proofFail",values),...proof.problems.map(problem=>`- ${ciTextV1(language,problem as keyof typeof texts.en,values)}`)].join("\n");
}

/** 🎯️ Compiles a comma list of target names with `*` wildcards into a matcher. */
export function ciTargetMatcherV1(patterns:string):(name:string)=>boolean{
 const expressions=patterns.split(",").map(item=>item.trim()).filter(Boolean).map(item=>new RegExp(`^${item.split("*").map(part=>part.replace(/[.+?^${}()|[\]\\]/gu,"\\$&")).join(".*")}$`,"u"));
 if(!expressions.length)throw Error("Target list required");
 return name=>expressions.some(expression=>expression.test(name));
}

/** 🎯️ The sorted `project:target` tasks of a plan whose target the list names; configurations are part of the identity. */
export function ciPlanSelectionV1(plan:CiPlanV1,targets:string):readonly string[]{
 const matches=ciTargetMatcherV1(targets),selected=new Set<string>();
 for(const task of Object.values(plan.tasks.tasks))if(matches(task.target.target))selected.add(`${task.target.project}:${task.target.target}${task.target.configuration===undefined?"":`:${task.target.configuration}`}`);
 return [...selected].sort();
}

/** 🧮️ Counts a plan. */
export function summarizeCiPlanV1(plan:CiPlanV1,targets:string):CiPlanSummaryV1{
 const selection=ciPlanSelectionV1(plan,targets),summary={tasks:Object.keys(plan.tasks.tasks).length,selected:selection.length,projects:new Set(selection.map(item=>item.split(":")[0])).size,targets:targets.split(",").map(item=>item.trim()).filter(Boolean)};
 requireShape("PlanSummary",summary,"Closed plan summary required");return Object.freeze(summary);
}

/** 📝️ Renders a plan summary. */
export function renderCiPlanV1(summary:CiPlanSummaryV1,label:string,language:CiLanguageV1):string{
 return [`### ${ciTextV1(language,"planTitle",{label})}`,"",`- ${summary.selected===0?ciTextV1(language,"planEmpty"):ciTextV1(language,"planLine",{selected:summary.selected,tasks:summary.tasks,projects:summary.projects,targets:summary.targets.join(",")})}`,""].join("\n");
}

/** 🪞️ Compares the tasks two implementations selected for the same targets. */
export function compareCiPlansV1(first:CiPlanV1,second:CiPlanV1,targets:string):CiParityV1{
 const a=ciPlanSelectionV1(first,targets),b=ciPlanSelectionV1(second,targets),inA=new Set(a),inB=new Set(b);
 const parity={equal:a.length===b.length&&a.every(item=>inB.has(item)),first:a.length,second:b.length,onlyFirst:a.filter(item=>!inB.has(item)),onlySecond:b.filter(item=>!inA.has(item))};
 requireShape("Parity",parity,"Closed parity required");return Object.freeze(parity);
}

/** 📝️ Renders a parity verdict; lists are cut after ten entries. */
export function renderCiParityV1(parity:CiParityV1,language:CiLanguageV1):string{
 const list=(items:readonly string[])=>items.slice(0,10).join(", ")+(items.length>10?`, … (${items.length})`:"");
 if(parity.equal)return ciTextV1(language,"parityOk",{first:parity.first});
 return [ciTextV1(language,"parityFail",{first:parity.first,second:parity.second}),...parity.onlyFirst.length?[`- ${ciTextV1(language,"parityOnlyFirst",{list:list(parity.onlyFirst)})}`]:[],...parity.onlySecond.length?[`- ${ciTextV1(language,"parityOnlySecond",{list:list(parity.onlySecond)})}`]:[]].join("\n");
}

/** 🧾️ Counts the test cases, failures, errors and skips of one JUnit XML document; CDATA sections and comments are ignored and nested suites are never double counted because only cases and their outcome elements are counted. */
export function summarizeCiJUnitV1(xml:string):CiJUnitV1{
 const text=xml.replace(/<!\[CDATA\[[\s\S]*?\]\]>/gu,"").replace(/<!--[\s\S]*?-->/gu,""),count=(name:string)=>(text.match(new RegExp(`<${name}(?=[\\s/>])`,"gu"))??[]).length;
 const counts={tests:count("testcase"),failures:count("failure"),errors:count("error"),skipped:count("skipped")};
 requireShape("JUnit",counts,"Closed JUnit counts required");return Object.freeze(counts);
}

/** 📚️ Validates an exported index. */
export function readCiTestResultsIndexV1(text:string):CiTestResultsIndexV1{
 const value=JSON.parse(text);requireShape("TestResultsIndex",value,"Closed test results index required");return value as CiTestResultsIndexV1;
}

/** 🧪️ Sums the exported files; a row names its file, and a file without a row still counts. */
export function summarizeCiTestsV1(files:readonly Readonly<{file:string;xml:string}>[],index:CiTestResultsIndexV1|undefined):CiTestsSummaryV1{
 let tests=0,failures=0,errors=0,skipped=0;const failed:string[]=[],rows=new Map((index?.results??[]).map(row=>[row.file,row]));
 for(const entry of files){
  const counts=summarizeCiJUnitV1(entry.xml);tests+=counts.tests;failures+=counts.failures;errors+=counts.errors;skipped+=counts.skipped;
  const row=rows.get(entry.file);
  if(counts.failures+counts.errors>0||(row!==undefined&&row.status!=="passed"&&row.status!=="flaky"))failed.push(row?.label??entry.file);
 }
 const summary={files:files.length,tests,failures,errors,skipped,cached:(index?.results??[]).filter(row=>row.cached).length,failed:[...new Set(failed)].sort()};
 requireShape("TestsSummary",summary,"Closed tests summary required");return Object.freeze(summary);
}

/** 📝️ Renders the test summary. */
export function renderCiTestsV1(summary:CiTestsSummaryV1,language:CiLanguageV1):string{
 const lines=[`### ${ciTextV1(language,"testsTitle")}`,""];
 if(summary.files===0)lines.push(`- ${ciTextV1(language,"testsNone")}`);
 else{
  lines.push(`- ${ciTextV1(language,"testsLine",{files:summary.files,tests:summary.tests,failures:summary.failures,errors:summary.errors,skipped:summary.skipped,cached:summary.cached})}`);
  if(summary.failed.length)lines.push(`- ${ciTextV1(language,"testsFailed",{list:summary.failed.slice(0,20).join(", ")+(summary.failed.length>20?`, … (${summary.failed.length})`:"")})}`);
 }
 lines.push("");return lines.join("\n");
}
