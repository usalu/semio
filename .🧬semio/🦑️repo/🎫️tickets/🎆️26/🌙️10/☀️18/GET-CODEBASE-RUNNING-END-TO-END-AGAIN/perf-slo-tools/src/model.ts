import schema from "./@@SCHEMA@@/@@JSON@@" with {type:"json"};
import {validateJsonSchemaSubset} from "@@SEVEN@@/@@MODULES@@/@@SCHEMA@@/@@VALIDATOR@@/@@PACKAGES@@/@@TYPESCRIPT@@/@@TS@@";
import {concurrentMutateV1} from "../@@CONC@@/@@TS@@";

/** 🌱 A text in every language the bench speaks. */
export type BenchTextV1=Readonly<{en:string;de:string}>;
/** 🔧 The build implementations behind the tool-neutral interface that a probe can run with. */
export type BenchToolV1="nx"|"bazel";
/** 🌳 What an objective is measured with: one implementation, or `neutral` for a command that does not depend on it. */
export type BenchSloToolV1=BenchToolV1|"neutral";
/** 🌰 The kinds of probe. */
export type BenchKindV1="cli-help"|"noop"|"edit"|"graph"|"cache"|"concurrency";
/** 🌴 The path a command takes: one process per command, or the resident service. */
export type BenchLaneV1="oneshot"|"resident";
/** 🌵 What a probe measures. */
export type BenchMetricV1="wallMs"|"startMs"|"greenMs"|"peakRssBytes"|"hitRatio"|"makespanMs"|"maxUnrelatedWaitMs"|"speedup";
/** 🌶 How the samples of a series are condensed to one number. */
export type BenchStatisticV1="min"|"p50"|"p95"|"p99"|"max"|"mean";
/** 🌷 The verdict of an objective: `warn` met the gate but not the goal, `recorded` has neither. */
export type BenchStatusV1="pass"|"warn"|"fail"|"unproven"|"recorded";
/** 🌸 Why an objective stays unproven or a comparison is not made. */
export type BenchReasonV1="no-samples"|"too-few-samples"|"probe-failed"|"unavailable"|"not-cold"|"host-saturated"|"service-unavailable"|"unsupported"|"host-class"|"schema"|"start-unobserved";
/** 🌹 How busy the host was while a series was measured. */
export type BenchLoadClassV1="quiet"|"busy"|"saturated"|"unknown";
/** 🌺 The verdict of one comparison row against the stored baseline. */
export type BenchCompareStatusV1="improved"|"same"|"regressed"|"inconclusive"|"incomparable"|"new"|"missing";
/** 🌻 What ended the wait for the rebuild to start. */
export type BenchStartKindV1="marker"|"first-output"|"exit";
/** 🌼 A source file edit that makes one language unit rebuild: the file, the kind of edit and the target that must follow it. */
export type BenchUnitV1=Readonly<{id:string;language:"ts"|"rs"|"go"|"py";target:string;file:string;op:"comment"|"private-body"|"public-api";title:BenchTextV1}>;
/** 🌽 One measurement: a kind, the target or unit it works on, how often it runs and how it starts. */
export type BenchProbeV1=Readonly<{id:string;kind:BenchKindV1;title:BenchTextV1;lane?:BenchLaneV1;target?:string;unit?:string;workload?:string;sessions?:number;runs:number;warmup:number;cold?:boolean;memory?:boolean}>;
/** 🌾 The allowed drift against a baseline: a share of the baseline value plus an absolute margin in the unit of the metric. */
export type BenchToleranceV1=Readonly<{relative:number;absolute:number}>;
/** 🌿 One objective: the statistic of a metric of a probe, its goal and its gate, who enforces it and how far it may drift. */
export type BenchSloV1=Readonly<{id:string;title:BenchTextV1;probe:string;metric:BenchMetricV1;statistic:BenchStatisticV1;direction:"lower"|"higher";goal:number|null;gate:number|null;tools:readonly BenchSloToolV1[];enforcement:"advisory"|"blocking";tolerance:BenchToleranceV1;source:string}>;
/** 🍀 The SLO table: labels, edit units, probes and objectives. */
export type BenchSloSetV1=Readonly<{version:1;labels:Readonly<Record<string,BenchTextV1>>;units:readonly BenchUnitV1[];probes:readonly BenchProbeV1[];slos:readonly BenchSloV1[]}>;
/** 🍁 One run of a probe: whether it succeeded, the values it measured and what ended the wait for the start. */
export type BenchSampleV1=Readonly<{index:number;ok:boolean;exit:number|null;values:Readonly<Partial<Record<BenchMetricV1,number>>>;startKind:BenchStartKindV1|null;loadPerCore:number|null}>;
/** 🍂 The condensed samples of one metric. */
export type BenchStatsV1=Readonly<{n:number;min:number;p50:number;p95:number;p99:number;max:number;mean:number}>;
/** 🍃 All samples of one probe with one implementation, their statistics and the load of the host while they ran. */
export type BenchSeriesV1=Readonly<{probe:string;tool:BenchSloToolV1;lane:BenchLaneV1|null;status:"measured"|"unavailable";reason:BenchReasonV1|null;samples:readonly BenchSampleV1[];failed:number;stats:Readonly<Partial<Record<BenchMetricV1,BenchStatsV1>>>;loadClass:BenchLoadClassV1;startKinds:Readonly<Record<BenchStartKindV1,number>>}>;
/** 🍄 What the host looked like when a measurement started or ended. */
export type BenchHostV1=Readonly<{platform:string;arch:string;cpus:number;memoryBytes:number;load1:number|null;loadPerCore:number|null;loadClass:BenchLoadClassV1;hostClass:string}>;
/** 🍅 The verdict of one objective with one implementation. */
export type BenchRowV1=Readonly<{slo:string;tool:BenchSloToolV1;probe:string;metric:BenchMetricV1;statistic:BenchStatisticV1;direction:"lower"|"higher";unit:"ms"|"bytes"|"ratio";value:number|null;goal:number|null;gate:number|null;n:number;status:BenchStatusV1;reason:BenchReasonV1|null;inflated:boolean;enforcement:"advisory"|"blocking";loadClass:BenchLoadClassV1}>;
/** 🍆 The verdicts counted. */
export type BenchTallyV1=Readonly<{pass:number;warn:number;fail:number;unproven:number;recorded:number;blockingFailed:number}>;
/** 🍇 One objective compared with the stored baseline. */
export type BenchCompareRowV1=Readonly<{slo:string;tool:BenchSloToolV1;metric:BenchMetricV1;statistic:BenchStatisticV1;direction:"lower"|"higher";unit:"ms"|"bytes"|"ratio";base:number|null;current:number|null;delta:number|null;relative:number|null;toleranceRelative:number;toleranceAbsolute:number;status:BenchCompareStatusV1;enforcement:"advisory"|"blocking";reason:BenchReasonV1|null}>;
/** 🍈 The comparisons counted. */
export type BenchCompareTallyV1=Readonly<{improved:number;same:number;regressed:number;inconclusive:number;incomparable:number;new:number;missing:number;blockingRegressed:number}>;
/** 🍉 A report compared with a baseline. */
export type BenchComparisonV1=Readonly<{version:1;currentHostClass:string;baselineHostClass:string;rows:readonly BenchCompareRowV1[];tally:BenchCompareTallyV1}>;
/** 🍊 Everything a bench run measured and judged; it is also the format of a stored baseline. */
export type BenchReportV1=Readonly<{version:1;kind:"bench-report";startedAt:string;finishedAt:string;host:BenchHostV1;hostAfter:BenchHostV1;tools:readonly BenchToolV1[];runsOverride:number|null;series:readonly BenchSeriesV1[];rows:readonly BenchRowV1[];tally:BenchTallyV1;comparison:BenchComparisonV1|null}>;
/** 🍋 A probe with one implementation and the objectives that read its samples. */
export type BenchPairV1=Readonly<{probe:BenchProbeV1;tool:BenchSloToolV1;slos:readonly BenchSloV1[]}>;

const policy=schema.definitions.Policy.const,order=(left:string,right:string)=>left<right?-1:left>right?1:0;
const both=(english:string,german:string)=>`${english}\n${german}`;
const closed=(name:keyof typeof schema.definitions,value:unknown,message:string):void=>{if(validateJsonSchemaSubset(schema.definitions[name],value,schema).length)throw Error(message);};
const metrics=schema.definitions.Metric.enum as readonly BenchMetricV1[],unitOf=(metric:BenchMetricV1)=>policy.units[metric] as "ms"|"bytes"|"ratio";

/** 🍌 Reads a percentile of sorted values by linear interpolation between ranks (the inclusive method of Python's `statistics.quantiles` and NumPy's default). */
export function benchPercentileV1(sorted:readonly number[],fraction:number):number{
 const position=(sorted.length-1)*fraction,low=Math.floor(position),high=Math.min(low+1,sorted.length-1);
 return sorted[low]!+(position-low)*(sorted[high]!-sorted[low]!);
}

/** 🍍 Condenses samples to count, minimum, median, 95th and 99th percentile, maximum and mean; no samples give nothing. */
export function benchStatsV1(values:readonly number[]):BenchStatsV1|null{
 if(!values.length)return null;
 const sorted=[...values].sort((left,right)=>left-right);
 return Object.freeze({n:sorted.length,min:sorted[0]!,p50:benchPercentileV1(sorted,0.5),p95:benchPercentileV1(sorted,0.95),p99:benchPercentileV1(sorted,0.99),max:sorted[sorted.length-1]!,mean:sorted.reduce((sum,value)=>sum+value,0)/sorted.length});
}

/** 🍎 Names how busy a host is from its one-minute load per core; a host that does not report a load is `unknown`. */
export function benchLoadClassV1(perCore:number|null):BenchLoadClassV1{
 if(perCore===null||!Number.isFinite(perCore))return "unknown";
 return perCore<policy.loadPerCore.quietBelow?"quiet":perCore>=policy.loadPerCore.saturatedFrom?"saturated":"busy";
}

/** 🍏 Describes the host: platform, architecture, cores, memory and the load per core with its class; the host class names the machines whose numbers can be compared. */
export function benchHostV1(input:Readonly<{platform:string;arch:string;cpus:number;memoryBytes:number;load1:number|null}>):BenchHostV1{
 const perCore=input.load1===null?null:input.load1/input.cpus;
 return Object.freeze({platform:input.platform,arch:input.arch,cpus:input.cpus,memoryBytes:input.memoryBytes,load1:input.load1,loadPerCore:perCore,loadClass:benchLoadClassV1(perCore),hostClass:`${input.platform}-${input.arch}-${input.cpus}`});
}

/** 🍐 Builds a series from its samples: failed samples are counted and left out of the statistics, the load class is the busiest moment of the series. */
export function benchSeriesV1(input:Readonly<{probe:string;tool:BenchSloToolV1;lane:BenchLaneV1|null;status:"measured"|"unavailable";reason:BenchReasonV1|null;samples:readonly BenchSampleV1[]}>):BenchSeriesV1{
 const good=input.samples.filter(sample=>sample.ok),stats:Partial<Record<BenchMetricV1,BenchStatsV1>>={};
 for(const metric of metrics){const found=benchStatsV1(good.flatMap(sample=>sample.values[metric]===undefined?[]:[sample.values[metric]!]));if(found)stats[metric]=found;}
 const loads=input.samples.flatMap(sample=>sample.loadPerCore===null?[]:[sample.loadPerCore]),kinds={marker:0,"first-output":0,exit:0};
 for(const sample of good)if(sample.startKind)kinds[sample.startKind]++;
 return Object.freeze({probe:input.probe,tool:input.tool,lane:input.lane,status:input.status,reason:input.reason,samples:input.samples,failed:input.samples.length-good.length,stats,loadClass:benchLoadClassV1(loads.length?Math.max(...loads):null),startKinds:kinds});
}

/** 🥭 Judges one objective with one implementation against its series: the statistic must be backed by enough samples; a time that misses its gate on a saturated host is unproven because load only ever inflates it. */
export function benchJudgeV1(slo:BenchSloV1,tool:BenchSloToolV1,series:BenchSeriesV1|undefined):BenchRowV1{
 const head={slo:slo.id,tool,probe:slo.probe,metric:slo.metric,statistic:slo.statistic,direction:slo.direction,unit:unitOf(slo.metric),goal:slo.goal,gate:slo.gate,enforcement:slo.enforcement,loadClass:series?.loadClass??"unknown" as BenchLoadClassV1};
 const unproven=(reason:BenchReasonV1,n:number,value:number|null=null):BenchRowV1=>Object.freeze({...head,value,n,status:"unproven" as const,reason,inflated:false});
 if(!series||series.status!=="measured")return unproven(series?.reason??"unavailable",0);
 const stats=series.stats[slo.metric],n=stats?.n??0;
 if(!stats)return unproven(series.failed>0?"probe-failed":"no-samples",0);
 if(n<policy.minimumSamples[slo.statistic])return unproven("too-few-samples",n,stats[slo.statistic]);
 const value=stats[slo.statistic],inflated=head.unit==="ms"&&series.loadClass==="saturated",lower=slo.direction==="lower";
 const within=(limit:number)=>lower?value<=limit:value>=limit;
 const kinds=series.startKinds,seen=kinds.marker+kinds["first-output"]+kinds.exit,bound=slo.metric!=="startMs"||kinds.marker===seen?"exact":kinds["first-output"]===seen?"lower":kinds.exit===seen?"upper":"mixed";
 let status:BenchStatusV1;
 if(slo.goal===null&&slo.gate===null)status="recorded";
 else if(slo.goal!==null&&within(slo.goal))status="pass";
 else if(slo.gate!==null&&within(slo.gate))status=slo.goal===null?"pass":"warn";
 else status="fail";
 if(bound!=="exact"&&status!=="recorded"&&!(bound==="lower"&&status==="fail"||bound==="upper"&&status!=="fail"))return unproven("start-unobserved",n,value);
 if(status==="fail"&&inflated)return Object.freeze({...head,value,n,status:"unproven" as const,reason:"host-saturated" as const,inflated:true});
 return Object.freeze({...head,value,n,status,reason:null,inflated:inflated&&status!=="pass"&&status!=="recorded"});
}

/** 🥝 Counts the verdicts; a blocking objective that failed is what makes `repo bench run` fail. */
export function benchTallyV1(rows:readonly BenchRowV1[]):BenchTallyV1{
 const count=(status:BenchStatusV1)=>rows.filter(row=>row.status===status).length;
 return Object.freeze({pass:count("pass"),warn:count("warn"),fail:count("fail"),unproven:count("unproven"),recorded:count("recorded"),blockingFailed:rows.filter(row=>row.status==="fail"&&row.enforcement==="blocking").length});
}

/** 🍓 Reads a stored report as a baseline and refuses anything that is not a closed report of this version. */
export function benchBaselineV1(value:unknown):BenchReportV1{
 if(validateJsonSchemaSubset(schema.definitions.Report,value,schema).length)throw Error(both("A closed bench report of this version is required as the baseline","Als Referenz wird ein geschlossener Bench-Bericht dieser Version benötigt"));
 return value as BenchReportV1;
}

const drift=(direction:"lower"|"higher",base:number,current:number,tolerance:BenchToleranceV1):"improved"|"same"|"regressed"=>{
 const up=base*(1+tolerance.relative)+tolerance.absolute,down=base*(1-tolerance.relative)-tolerance.absolute,worse=direction==="lower"?current>up:current<down,better=direction==="lower"?current<down:current>up;
 return worse?"regressed":better?"improved":"same";
};

/** 🥑 Compares the verdict rows of a report with a stored baseline of the same host class: a change beyond the tolerance of the objective is an improvement or a regression, a regression of a time on a saturated host is inconclusive, rows without a value on either side are incomparable. */
export function benchCompareV1(set:BenchSloSetV1,current:BenchReportV1,baseline:BenchReportV1):BenchComparisonV1{
 const slos=new Map(set.slos.map(row=>[row.id,row])),key=(row:Readonly<{slo:string;tool:string}>)=>`${row.slo}\0${row.tool}`,before=new Map(baseline.rows.map(row=>[key(row),row])),now=new Set(current.rows.map(key));
 const sameHost=current.host.hostClass===baseline.host.hostClass,rows:BenchCompareRowV1[]=[];
 const saturated=(report:BenchReportV1,row:BenchRowV1)=>report.series.find(series=>series.probe===row.probe&&series.tool===row.tool)?.loadClass==="saturated";
 for(const row of current.rows){
  const slo=slos.get(row.slo),tolerance=slo?.tolerance??policy.defaultTolerance,old=before.get(key(row)),head={slo:row.slo,tool:row.tool,metric:row.metric,statistic:row.statistic,direction:row.direction,unit:row.unit,toleranceRelative:tolerance.relative,toleranceAbsolute:tolerance.absolute,enforcement:row.enforcement};
  const made=(status:BenchCompareStatusV1,base:number|null,value:number|null,reason:BenchReasonV1|null):BenchCompareRowV1=>Object.freeze({...head,base,current:value,delta:base===null||value===null?null:value-base,relative:base===null||value===null||base===0?null:(value-base)/base,status,reason});
  if(!old)rows.push(made("new",null,row.value,null));
  else if(!sameHost)rows.push(made("incomparable",old.value,row.value,"host-class"));
  else if(old.status==="unproven"||row.status==="unproven"||old.value===null||row.value===null)rows.push(made("incomparable",old.value,row.value,row.reason??old.reason??"unavailable"));
  else{
   const verdict=drift(row.direction,old.value,row.value,tolerance);
   rows.push(verdict==="regressed"&&row.unit==="ms"&&(saturated(current,row)||saturated(baseline,old))?made("inconclusive",old.value,row.value,"host-saturated"):made(verdict,old.value,row.value,null));
  }
 }
 const probes=new Set(current.series.map(series=>series.probe));
 for(const row of baseline.rows){
  if(now.has(key(row))||!probes.has(row.probe))continue;
  const tolerance=slos.get(row.slo)?.tolerance??policy.defaultTolerance;
  rows.push(Object.freeze({slo:row.slo,tool:row.tool,metric:row.metric,statistic:row.statistic,direction:row.direction,unit:row.unit,base:row.value,current:null,delta:null,relative:null,toleranceRelative:tolerance.relative,toleranceAbsolute:tolerance.absolute,status:"missing" as const,enforcement:row.enforcement,reason:null}));
 }
 const count=(status:BenchCompareStatusV1)=>rows.filter(row=>row.status===status).length;
 return Object.freeze({version:1 as const,currentHostClass:current.host.hostClass,baselineHostClass:baseline.host.hostClass,rows,tally:Object.freeze({improved:count("improved"),same:count("same"),regressed:count("regressed"),inconclusive:count("inconclusive"),incomparable:count("incomparable"),new:count("new"),missing:count("missing"),blockingRegressed:rows.filter(row=>row.status==="regressed"&&row.enforcement==="blocking").length})});
}

/** 🍒 Builds the closed report of a run from its series and verdict rows; a stored report is the baseline of later runs. */
export function benchReportV1(input:Readonly<{startedAt:string;finishedAt:string;host:BenchHostV1;hostAfter:BenchHostV1;tools:readonly BenchToolV1[];runsOverride:number|null;series:readonly BenchSeriesV1[];rows:readonly BenchRowV1[];comparison:BenchComparisonV1|null}>):BenchReportV1{
 const report={version:1 as const,kind:"bench-report" as const,...input,tally:benchTallyV1(input.rows)};
 closed("Report",report,"Closed bench report required");
 return Object.freeze(report);
}

const kindRank=new Map((policy.order as readonly string[]).map((kind,at)=>[kind,at]));

/** 🥥 Admits the authored SLO table and lists every inconsistency: duplicate ids, a probe without its target, unit or workload, a metric the probe does not yield, a statistic no run count can back, a goal beyond its gate, a blocking objective without a gate, an edit the unit's language has no template for. */
export function benchSloIssuesV1(value:unknown):string[]{
 const issues=validateJsonSchemaSubset(schema.definitions.SloSet,value,schema).map(String);
 if(issues.length)return issues;
 const set=value as BenchSloSetV1,found:string[]=[],unique=(kind:string,ids:readonly string[])=>{const seen=new Set<string>();for(const id of ids){if(seen.has(id))found.push(`duplicate ${kind} ${id}`);seen.add(id);}};
 unique("unit",set.units.map(row=>row.id));unique("probe",set.probes.map(row=>row.id));unique("objective",set.slos.map(row=>row.id));
 const units=new Set(set.units.map(row=>row.id)),probes=new Map(set.probes.map(row=>[row.id,row]));
 for(const unit of set.units){
  try{concurrentMutateV1("x\n",unit.op,unit.language,"abcdef");}catch{found.push(`${unit.id}: no ${unit.op} template for ${unit.language}`);}
  if(unit.file.length>policy.maximumPathLength)found.push(`${unit.id}: path exceeds ${policy.maximumPathLength}`);
 }
 for(const probe of set.probes){
  const need=(field:keyof BenchProbeV1,wanted:boolean)=>{if((probe[field]!==undefined)!==wanted)found.push(`${probe.id}: ${field} ${wanted?"is required":"is not allowed"} for ${probe.kind}`);};
  need("target",probe.kind==="noop"||probe.kind==="graph"||probe.kind==="cache");need("unit",probe.kind==="edit");need("workload",probe.kind==="concurrency");need("sessions",probe.kind==="concurrency");need("lane",probe.kind==="noop"||probe.kind==="edit"||probe.kind==="graph");
  if(probe.unit!==undefined&&!units.has(probe.unit))found.push(`${probe.id}: unknown unit ${probe.unit}`);
  if(probe.lane==="resident"&&probe.kind==="graph")found.push(`${probe.id}: a graph probe has no resident lane`);
  if(probe.cold&&probe.kind!=="graph")found.push(`${probe.id}: only a graph probe can be cold`);
  if(probe.cold&&probe.runs!==1)found.push(`${probe.id}: a cold probe runs once`);
  if(probe.memory&&probe.kind!=="noop"&&probe.kind!=="edit")found.push(`${probe.id}: only noop and edit probes measure memory`);
 }
 for(const slo of set.slos){
  const probe=probes.get(slo.probe);
  if(!probe){found.push(`${slo.id}: unknown probe ${slo.probe}`);continue;}
  const yielded=(policy.metrics[probe.kind][probe.memory?"memory":"plain"] as readonly string[]);
  if(!yielded.includes(slo.metric))found.push(`${slo.id}: ${probe.id} does not yield ${slo.metric}`);
  if(policy.minimumSamples[slo.statistic]>probe.runs)found.push(`${slo.id}: ${slo.statistic} needs ${policy.minimumSamples[slo.statistic]} samples but ${probe.id} runs ${probe.runs}`);
  if(slo.metric==="startMs"&&slo.direction!=="lower")found.push(`${slo.id}: a start time is judged downwards`);
  if(slo.goal!==null&&slo.gate===null)found.push(`${slo.id}: a goal needs a gate`);
  if(slo.goal!==null&&slo.gate!==null&&(slo.direction==="lower"?slo.goal>slo.gate:slo.goal<slo.gate))found.push(`${slo.id}: the goal lies beyond the gate`);
  if(slo.enforcement==="blocking"&&slo.gate===null)found.push(`${slo.id}: a blocking objective needs a gate`);
  if(slo.tools.includes("neutral")!==(probe.kind==="cli-help")||probe.kind==="cli-help"&&slo.tools.length!==1)found.push(`${slo.id}: neutral belongs to cli-help probes only`);
 }
 return found;
}

/** 🥦 Selects objectives by id, `probe:<id>`, `kind:<kind>` or `all` and pairs every probe with the chosen implementations (a neutral probe runs once); cold probes run first, then probes in the order of their kind. */
export function benchSelectV1(set:BenchSloSetV1,ids:readonly string[]|"all",tools:readonly BenchToolV1[]):BenchPairV1[]{
 const chosen=new Set<string>();
 if(ids==="all")for(const slo of set.slos)chosen.add(slo.id);
 else for(const id of ids){
  const matches=id.startsWith("kind:")?set.slos.filter(slo=>set.probes.find(probe=>probe.id===slo.probe)?.kind===id.slice(5)):id.startsWith("probe:")?set.slos.filter(slo=>slo.probe===id.slice(6)):set.slos.filter(slo=>slo.id===id);
  if(!matches.length)throw Error(both(`Unknown objective: ${id} (an id, probe:<id>, kind:<${policy.kinds.join("|")}> or all)`,`Unbekanntes Ziel: ${id} (eine Kennung, probe:<id>, kind:<${policy.kinds.join("|")}> oder all)`));
  for(const slo of matches)chosen.add(slo.id);
 }
 const pairs=new Map<string,{probe:BenchProbeV1;tool:BenchSloToolV1;slos:BenchSloV1[]}>();
 for(const slo of set.slos){
  if(!chosen.has(slo.id))continue;
  const probe=set.probes.find(row=>row.id===slo.probe)!;
  for(const tool of slo.tools){
   if(tool!=="neutral"&&!tools.includes(tool))continue;
   const key=`${probe.id}\0${tool}`,pair=pairs.get(key)??{probe,tool,slos:[]};pair.slos.push(slo);pairs.set(key,pair);
  }
 }
 if(!pairs.size)throw Error(both("No objective is left for the chosen implementations","Für die gewählten Implementierungen bleibt kein Ziel übrig"));
 const position=new Map(set.probes.map((probe,at)=>[probe.id,at]));
 return [...pairs.values()].sort((left,right)=>(right.probe.cold?1:0)-(left.probe.cold?1:0)||kindRank.get(left.probe.kind)!-kindRank.get(right.probe.kind)!||position.get(left.probe.id)!-position.get(right.probe.id)!||order(left.tool,right.tool));
}

/** 🥬 Renders one edit of a source text for a unit: appended at the end, never changing an existing line, always different from the input. */
export function benchMutateV1(text:string,unit:BenchUnitV1,nonce:string):string{return concurrentMutateV1(text,unit.op,unit.language,nonce);}

/** 🥒 Decides which start line pattern list applies to an implementation. */
export function benchMarkersV1(tool:BenchSloToolV1):readonly RegExp[]{return (policy.markers[tool]??[]).map(pattern=>new RegExp(pattern,"u"));}

const text=(set:BenchSloSetV1,key:string,language:"en"|"de")=>set.labels[key]?.[language]??key;
const trim=(value:number,digits:number)=>String(Number(value.toFixed(digits)));
/** 🥧 Formats a value for its metric: milliseconds with one decimal, bytes in MiB, the hit ratio as a percentage, other ratios with two decimals. */
export function benchFormatV1(metric:BenchMetricV1,value:number|null):string{
 if(value===null)return "-";
 if(metric==="hitRatio")return `${trim(value*100,1)} %`;
 if(metric==="peakRssBytes")return `${trim(value/1048576,1)} MiB`;
 if(metric==="speedup")return trim(value,2);
 return trim(value,value<10?2:1);
}

const grid=(head:readonly string[],rows:readonly (readonly string[])[],left:readonly number[],format:"text"|"markdown"):string[]=>{
 if(format==="markdown")return [`| ${head.join(" | ")} |`,`| ${head.map((_,at)=>left.includes(at)?":--":"--:").join(" | ")} |`,...rows.map(row=>`| ${row.map(cell=>cell.replaceAll("|","\\|")).join(" | ")} |`)];
 const width=head.map((cell,at)=>Math.max([...cell].length,...rows.map(row=>[...row[at]!].length))),pad=(cell:string,at:number)=>left.includes(at)?cell+" ".repeat(width[at]!-[...cell].length):" ".repeat(width[at]!-[...cell].length)+cell;
 return [head.map(pad).join("  "),...rows.map(row=>row.map(pad).join("  "))];
};
const heading=(value:string,format:"text"|"markdown",level=2)=>format==="markdown"?`${"#".repeat(level)} ${value}`:value;
const limit=(metric:BenchMetricV1,value:number|null)=>value===null?"-":benchFormatV1(metric,value);
const toolName=(set:BenchSloSetV1,tool:BenchSloToolV1,language:"en"|"de")=>text(set,tool,language);

/** 🥗 Renders a report in a language as plain text or markdown: the host, the verdict table with value, goal, gate, sample count and status, the reasons of everything unproven, how the start was observed and the comparison with the baseline when there is one. */
export function renderBenchReportV1(report:BenchReportV1,set:BenchSloSetV1,language:"en"|"de",format:"text"|"markdown"):string{
 const t=(key:string)=>text(set,key,language),out:string[]=[];
 const title=(id:string)=>set.slos.find(slo=>slo.id===id)?.title[language]??id;
 out.push(heading(t("title"),format,1),"");
 out.push(`${t("started")}: ${report.startedAt}, ${t("finished")}: ${report.finishedAt}`,`${t("host")}: ${report.host.hostClass}, ${t("cpus")}: ${report.host.cpus}, ${t("memory")}: ${benchFormatV1("peakRssBytes",report.host.memoryBytes)}, ${t("load")}: ${report.host.loadPerCore===null?"-":trim(report.host.loadPerCore,2)} (${t(report.host.loadClass)}) -> ${report.hostAfter.loadPerCore===null?"-":trim(report.hostAfter.loadPerCore,2)} (${t(report.hostAfter.loadClass)})`,"");
 const tally=report.tally;
 out.push(`${t("summary")}: ${t("pass")} ${tally.pass}, ${t("warn")} ${tally.warn}, ${t("fail")} ${tally.fail}, ${t("unproven")} ${tally.unproven}, ${t("recorded")} ${tally.recorded}; ${t("blockingFailed")}: ${tally.blockingFailed}`,"");
 const rows=report.rows.map(row=>[title(row.slo),toolName(set,row.tool,language),row.statistic,benchFormatV1(row.metric,row.value),limit(row.metric,row.goal),limit(row.metric,row.gate),String(row.n),`${t(row.status)}${row.inflated?"*":""}${row.enforcement==="blocking"?" !":""}`,t(row.loadClass)]);
 out.push(...grid([t("slo"),t("tool"),t("statistic"),t("value"),t("goal"),t("gate"),t("samples"),t("status"),t("load")],rows,[0,1,2,7,8],format));
 const why=report.rows.filter(row=>row.reason!==null);
 if(why.length){out.push("",heading(t("notes"),format));for(const row of why)out.push(`- ${title(row.slo)} / ${toolName(set,row.tool,language)}: ${t(row.reason!)}`);}
 const kinds=report.series.filter(series=>series.status==="measured"&&(series.startKinds.marker+series.startKinds["first-output"]+series.startKinds.exit)>0);
 if(kinds.length){out.push("",heading(t("startKinds"),format));for(const series of kinds)out.push(`- ${series.probe} / ${toolName(set,series.tool,language)}: ${t("marker")} ${series.startKinds.marker}, ${t("first-output")} ${series.startKinds["first-output"]}, ${t("exit")} ${series.startKinds.exit}`);}
 if(report.comparison)out.push("",renderBenchComparisonV1(report.comparison,set,language,format));
 out.push("",`${t("note-marks")}`,`${t("note-load")}`,`${t("note-percentile")}`,`${t("note-start")}`,`${t("note-recorded")}`);
 return out.join("\n");
}

/** 🥘 Renders a comparison with a baseline: per objective the baseline and current value, the change, the tolerance and the verdict. */
export function renderBenchComparisonV1(comparison:BenchComparisonV1,set:BenchSloSetV1,language:"en"|"de",format:"text"|"markdown"):string{
 const t=(key:string)=>text(set,key,language),title=(id:string)=>set.slos.find(slo=>slo.id===id)?.title[language]??id,out:string[]=[heading(`${t("comparison")} (${comparison.baselineHostClass} -> ${comparison.currentHostClass})`,format),""];
 const tally=comparison.tally;
 out.push(`${t("summary")}: ${t("improved")} ${tally.improved}, ${t("same")} ${tally.same}, ${t("regressed")} ${tally.regressed}, ${t("inconclusive")} ${tally.inconclusive}, ${t("incomparable")} ${tally.incomparable}, ${t("new")} ${tally.new}, ${t("missing")} ${tally.missing}; ${t("blockingRegressed")}: ${tally.blockingRegressed}`,"");
 out.push(...grid([t("slo"),t("tool"),t("statistic"),t("baseline"),t("current"),t("relative"),t("tolerance"),t("status")],comparison.rows.map(row=>[title(row.slo),toolName(set,row.tool,language),row.statistic,benchFormatV1(row.metric,row.base),benchFormatV1(row.metric,row.current),row.relative===null?"-":`${trim(row.relative*100,1)} %`,`${trim(row.toleranceRelative*100,1)} % + ${benchFormatV1(row.metric,row.toleranceAbsolute)}`,`${t(row.status)}${row.enforcement==="blocking"?" !":""}`]),[0,1,2,7],format));
 const why=comparison.rows.filter(row=>row.reason!==null);
 for(const row of why)out.push(`- ${title(row.slo)} / ${toolName(set,row.tool,language)}: ${t(row.reason!)}`);
 return out.join("\n");
}

/** 🥙 Renders the objectives and their probes in a language: id, title, probe, metric, statistic, goal, gate, enforcement and source. */
export function renderBenchListV1(set:BenchSloSetV1,language:"en"|"de",format:"text"|"markdown"):string{
 const t=(key:string)=>text(set,key,language),probes=new Map(set.probes.map(row=>[row.id,row]));
 const rows=set.slos.map(slo=>[slo.id,slo.title[language],`${slo.probe}${probes.get(slo.probe)?.lane?` (${t(probes.get(slo.probe)!.lane!)})`:""}`,t(slo.metric),slo.statistic,limit(slo.metric,slo.goal),limit(slo.metric,slo.gate),slo.tools.map(tool=>t(tool)).join(","),t(slo.enforcement),slo.source]);
 return [heading(t("list"),format,1),"",...grid([t("slo"),t("name"),t("probe"),t("metric"),t("statistic"),t("goal"),t("gate"),t("tool"),t("enforcement"),t("source")],rows,[0,1,2,3,4,7,8,9],format)].join("\n");
}
