/** 🧾 Turns the measure.py records of one directory into a closed `bench-report` (the library's own series, judging and report builders), so that `repo bench compare` can judge the live-tree baseline.
 *
 * `bun to-report.ts <records-dir> <report.json> [<markdown.md>]` reads `<product>.<tool>.<noop|edit|peer-X>.<label>.json` records (the `prime` and `r*` labels are warm-ups and not counted) and `concurrent.*.json` summaries.
 * [Bench library](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench/🟦️.ts) */
import {readdirSync,readFileSync,writeFileSync} from "node:fs";
import {arch,cpus,loadavg,platform,totalmem} from "node:os";
import {join,resolve} from "node:path";

const root=resolve(import.meta.dir,"../../../../../../../..");
const bench=`${root}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📏️incremental/🏎️bench`;
const lib=await import(`${bench}/🟦️.ts`);
const set=(await import(`${bench}/🎯️slo/🔣️.json`,{with:{type:"json"}})).default;

const [dir,out,markdown]=process.argv.slice(2);
if(!dir||!out)throw Error("usage: to-report.ts <records-dir> <report.json> [<markdown.md>]");
const host=()=>lib.benchHostV1({platform:platform(),arch:arch(),cpus:cpus().length,memoryBytes:totalmem(),load1:loadavg()[0]});
const records:any[]=readdirSync(dir).filter(name=>name.endsWith(".json")&&!name.endsWith(".all.json")).map(name=>JSON.parse(readFileSync(join(dir,name),"utf8")));
const probes=new Map<string,any[]>();
for(const record of records){
 if(record.scenario===undefined||/^(prime|r\d+)$/u.test(record.label))continue;
 const kind=record.scenario==="peer"?`peer-${record.peer==="scheduling"?"scheduling":"unrelated"}`:record.scenario;
 const id=`product-${record.product}-${kind}`;
 probes.set(id,[...(probes.get(id)??[]),record]);
}
const sampleOf=(record:any,index:number)=>{
 const phases=record.phases??{},bazel=record.actions?.bazel,executed=bazel?bazel.executed:null,inferred=record.graph?.inferredProjects??0;
 const values:Record<string,number>=lib.benchObservedValuesV1({phases,executed,inferred});
 values.wallMs=record.wallMs;
 if(record.scenario==="edit")values.greenMs=record.wallMs;
 values.startMs=record.executeStartMs??record.firstOutputMs??record.wallMs;
 const memory=process.env.BENCH_MEMORY=="rss"?record.peakRssBytes:record.peakFootprintBytes??record.peakRssBytes;
 if(memory)values.peakRssBytes=memory;
 const perCore=record.load1Start==null?null:record.load1Start/cpus().length;
 return {index,ok:record.exit===0,exit:record.exit,values,startKind:record.executeStartMs!=null?"marker":record.firstOutputMs!=null?"first-output":"exit",loadPerCore:perCore};
};
const series=[...probes.entries()].map(([id,rows])=>lib.benchSeriesV1({probe:id,tool:"bazel",lane:"oneshot",status:"measured",reason:null,samples:rows.sort((a,b)=>a.startedAt<b.startedAt?-1:1).map((record,at)=>sampleOf(record,at))}));
const started=records.map(record=>record.startedAt).filter(Boolean).sort()[0]??new Date().toISOString();
const pairs=lib.benchSelectV1(set,series.map(one=>`probe:${one.probe}`),["bazel"]);
const rows:any[]=[];
for(const pair of pairs)for(const slo of pair.slos)rows.push(lib.benchJudgeV1(slo,pair.tool,series.find(one=>one.probe===pair.probe.id)));
const position=new Map(set.slos.map((slo:any,at:number)=>[slo.id,at]));
rows.sort((left,right)=>(position.get(left.slo) as number)-(position.get(right.slo) as number));
const report=lib.benchReportV1({startedAt:started.replace(/\+00:00$/u,"Z"),finishedAt:new Date().toISOString(),host:host(),hostAfter:host(),tools:["bazel"],runsOverride:null,series,rows,comparison:null});
writeFileSync(out,JSON.stringify(report,null,1));
if(markdown)writeFileSync(markdown,lib.renderBenchReportV1(report,set,"en","markdown"));
console.log(JSON.stringify({probes:series.map(one=>({probe:one.probe,samples:one.samples.length,failed:one.failed})),tally:report.tally,hostClass:report.host.hostClass}));
