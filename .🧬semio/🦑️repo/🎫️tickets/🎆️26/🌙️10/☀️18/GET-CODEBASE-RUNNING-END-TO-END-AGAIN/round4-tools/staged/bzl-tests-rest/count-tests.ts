// 🔢️ Counts how the test-family tasks of the static graph reach Bazel before and after the witness: reads the input-precision census (declared files per task, before and after precision derivation) and the merged graph (distinct declared input sets). Usage: bun count-tests.ts <input-precision generated dir>
import {readFileSync} from "node:fs";
const G=process.argv[2]!;
const census=(name:string)=>JSON.parse(readFileSync(`${G}/${name}`,"utf8")).all.filter((row:any)=>row.target==="test"||row.target.startsWith("test-")) as {id:string;project:string;target:string;files:number}[];
const before=census("inputs-census-before.json"),after=census("inputs-census-real.json");
const limit=10000,from=2000;
const over=(rows:typeof after,n:number)=>rows.filter(row=>row.files>n);
const sum=(rows:typeof after)=>rows.reduce((total,row)=>total+row.files,0);
const graph=JSON.parse(readFileSync(`${G}/graph-merged.json`,"utf8"));
const sets=new Map<string,string[]>();
const wanted=new Set(over(after,from).map(row=>row.id));
for(const [project,row] of Object.entries<any>(graph.projects))for(const [target,value] of Object.entries<any>(row.targets)){const id=project+":"+target;if(!wanted.has(id))continue;const key=JSON.stringify(value.inputs);sets.set(key,[...(sets.get(key)??[]),id]);}
const witnessed=over(after,from),runfilesBefore=sum(after.filter(row=>row.files<=limit)),runfilesAfter=sum(after.filter(row=>row.files<=from))+witnessed.length;
console.log(JSON.stringify({
  testFamilyTasks:{beforePrecision:before.length,afterPrecision:after.length},
  overRunfilesLimit10000:{beforePrecision:over(before,limit).length,afterPrecision:over(after,limit).length},
  bazelTestBefore:{runfilesLimit10000:after.length-over(after,limit).length,fallbackBuildActions:over(after,limit).length},
  bazelTestAfter:{bazelTests:after.length,fallbackBuildActions:0,witnessed:witnessed.length,distinctDeclaredInputSetsAmongWitnessed:sets.size,direct:after.length-witnessed.length},
  declaredFiles:{testsTotal:sum(after),largest:Math.max(...after.map(row=>row.files)),inWitnessedTests:sum(witnessed)},
  runfilesLinkedPerFullSuiteRun:{before:runfilesBefore,after:runfilesAfter},
  perProject:Object.fromEntries([...over(after,limit).reduce((map,row)=>map.set(row.project,(map.get(row.project)??0)+1),new Map<string,number>())].sort((a,b)=>b[1]-a[1]).slice(0,12)),
},null,1));
