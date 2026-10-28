/** 🧪️ Laws that keep the numbers of the disk budget in one place: the machine total and its weights derive from the declared capacities, the logical limits and the control directory agree with what lives in them, and the documentation states what the declarations say (English and German). */
import {spawnSync} from "node:child_process";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {expect,test} from "bun:test";
import metadata from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/📦️metadata/🧬️schema/🔣️.json" with {type:"json"};
import unitCache from "../../../../🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json" with {type:"json"};
import budgetPolicy from "../../../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {cacheLedgerV1} from "../../🟦️.ts";
import {machinePolicyV1} from "../../🎚️machine/🟦️.ts";

function workspace():string{
 if(process.env.SEMIO_CACHE_LAW_ROOT)return resolve(process.env.SEMIO_CACHE_LAW_ROOT);
 for(let path=import.meta.dir;;path=dirname(path)){if(existsSync(join(path,"bunfig.toml"))&&existsSync(join(path,"bun.lock")))return path;if(dirname(path)===path)throw Error("Repository root not found");}
}
const GIB=1024**3,MIB=1024**2,ledger=cacheLedgerV1(),machine=machinePolicyV1(),partitions=budgetPolicy.partitions,repo=join(workspace(),"🧰️framework/🛍️products/🦑️repo");
const english=(bytes:number):string=>bytes>=GIB?String(+(bytes/GIB).toFixed(1))+" GiB":String(bytes/MIB)+" MiB",german=(bytes:number):string=>english(bytes).replace(".",",");
const guide=readFileSync(join(repo,"📓️build-guide.md"),"utf8"),readme=readFileSync(join(repo,"README.md"),"utf8");

test("the machine default is the sum of the declared capacities and loose roots and every weight is a store's share of it (independent Python largest-remainder split)",()=>{
 const stores=machine.stores.map(store=>({id:store.id,capacity:store.partition!==undefined?partitions[store.partition as keyof typeof partitions].capacityBytes:null}));
 const loose=ledger.roots.repository.maximumBytes+ledger.roots.native.maximumBytes;
 const program="import json,sys\nv=json.load(sys.stdin)\nrows=[(s['id'],s['capacity'] if s['capacity'] is not None else v['loose']) for s in v['stores']]\ntotal=sum(c for _,c in rows)\nraw=[(i,c*10000/total) for i,c in rows]\nout={i:int(r) for i,r in raw}\nleft=10000-sum(out.values())\nfor i,r in sorted(raw,key=lambda x:(-(x[1]-int(x[1])),x[0]))[:left]: out[i]+=1\nprint(json.dumps({'total':total,'weights':out}))";
 const run=spawnSync("python3",["-I","-c",program],{input:JSON.stringify({stores,loose}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as {total:number;weights:Record<string,number>};
 expect(machine.defaultTotalBytes,"defaultTotalBytes").toBe(expected.total);
 expect(Object.fromEntries(machine.stores.map(store=>[store.id,store.weightBasisPoints]))).toEqual(expected.weights);
 expect(expected.total).toBeLessThanOrEqual(budgetPolicy.maximumBytes+loose);expect(machine.stores.reduce((sum,store)=>sum+store.floorBytes,0)).toBeLessThan(machine.defaultTotalBytes);
});

test("a logical limit never exceeds its image, except for the tools image whose hardlinked toolchains are counted once per name, and the control directory holds its headroom file",()=>{
 for(const[id,row]of Object.entries(partitions)){
  if(id==="tools"){expect(row.maximumLogicalBytes).toBeGreaterThan(row.capacityBytes);expect(row.maximumLogicalBytes).toBeLessThanOrEqual(8*row.capacityBytes);}
  else expect(row.maximumLogicalBytes,id).toBeLessThanOrEqual(row.capacityBytes);
 }
 expect(budgetPolicy.maximumControlBytes).toBeGreaterThanOrEqual(metadata.properties.capacityBytes.const+MIB);
});

test("the documentation states the declared numbers in English and German",()=>{
 const total=machine.defaultTotalBytes;
 expect(guide).toContain("default "+english(total));expect(guide).toContain("Standard "+german(total));
 const marks=[unitCache.definitions.Policy.const.gc.highWaterShare,unitCache.definitions.Policy.const.gc.lowWaterShare].map(share=>+(share*100).toFixed(2));
 for(const mark of marks){expect(readme).toContain(mark+" percent");expect(readme).toContain(String(mark).replace(".",",")+" Prozent");}
 const letters:Record<string,string>={d:"dependencies",t:"tools",g:"graph",o:"diagnostics",p:"products",l:"leases",n:"nx"};
 for(const[letter,id]of Object.entries(letters)){
  const size=partitions[id as keyof typeof partitions].capacityBytes;
  expect(readme,"table row of "+letter).toMatch(new RegExp("\\| `"+letter+"` \\| [^|]+ \\| "+english(size).replace(".","\\.")+" \\|","u"));
  expect(readme,"German sentence of "+letter).toMatch(new RegExp("`"+letter+"` [^(`]{1,60}\\("+german(size)+"\\)","u"));
 }
 expect(readme).toContain("partition `g`, "+english(partitions.graph.capacityBytes));expect(readme.match(new RegExp("Partition `g`, "+english(partitions.graph.capacityBytes),"gu"))?.length).toBe(1);
 expect(readme).toContain("the rust partition, "+english(partitions.rust.capacityBytes));expect(readme).toContain("die Rust-Partition, "+english(partitions.rust.capacityBytes));
 expect(readme).toContain("`repository` capped at "+english(ledger.roots.repository.maximumBytes)+", `native` at "+english(ledger.roots.native.maximumBytes));
 expect(readme).toContain("`repository` gedeckelt bei "+english(ledger.roots.repository.maximumBytes)+", `native` bei "+english(ledger.roots.native.maximumBytes));
 expect(readme).toContain("at most "+english(budgetPolicy.maximumControlBytes)+"; it holds");expect(readme).toContain("höchstens "+english(budgetPolicy.maximumControlBytes)+"; es hält");
});
