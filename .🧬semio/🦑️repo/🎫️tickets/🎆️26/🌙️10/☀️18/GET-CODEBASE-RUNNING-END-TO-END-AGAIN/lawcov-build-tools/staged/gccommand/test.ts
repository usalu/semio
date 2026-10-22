/** 🎹️ Contract: the `repo cache` argument grammar parses and refuses exactly like an independent Python implementation for every systematic vector of `./🔣️.json`, every refusal is worded in both languages from the authored catalogue and the command reports a cancelled run with the exit code 130 in the requested language. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {cacheTextV1,readCacheLedgerV1} from "../../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1} from "../../../🏃️runner/🟦️.ts";
import {cacheArgumentsV1,runCacheLedgerCommandV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),ledger=readCacheLedgerV1(corpus.ledger),ids=[...ledger.areas.map(area=>area.id),"retired","undeclared"];
const ask=(vectors:string[][]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({ids,vectors}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const both=(key:string,variables:Record<string,string>)=>cacheTextV1("en",key,variables)+"\n"+cacheTextV1("de",key,variables);
const outcome=(args:string[])=>{try{const parsed=cacheArgumentsV1(args,ledger);return{parsed:{operation:parsed.operation,json:parsed.json,dryRun:parsed.dryRun,only:parsed.only===undefined?null:[...parsed.only],language:parsed.language??null}};}catch(error){return{error:(error as Error).message};}};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,refused:[{args:[],key:"error.other",variables:{}}]})).toBe(false);
 expect(ids).toEqual(["t1","t2","retired","undeclared"]);
});

test("every systematic vector is parsed or refused like the Python grammar, with both languages in every refusal",()=>{
 const answers=ask(corpus.vectors);let parsedRows=0,refusedRows=0;const keys=new Set<string>();
 for(const[index,args]of corpus.vectors.entries()){
  const actual=outcome(args),expected=answers[index],label=JSON.stringify(args);
  if(expected.key!==undefined){refusedRows++;keys.add(expected.key);expect(actual.error,label).toBe(both(expected.key,expected.variables));expect(actual.error!.includes("\n"),label).toBe(true);}
  else{parsedRows++;expect(actual.parsed,label).toEqual({...expected.parsed,language:expected.parsed.language});}
 }
 expect(parsedRows).toBeGreaterThan(15);expect(refusedRows).toBeGreaterThan(80);expect([...keys].sort()).toEqual(["error.area","error.areaEmpty","error.option","error.repeated","error.syntax","error.value"]);
});

test("the authored parses and refusals hold",()=>{
 for(const row of corpus.authored)expect(outcome(row.args).parsed,row.args.join(" ")).toEqual(row.parsed);
 for(const row of corpus.refused)expect(outcome(row.args).error,row.args.join(" ")).toBe(both(row.key,row.variables));
 expect(cacheTextV1("en","error.syntax").startsWith("Expected repo cache status|gc")).toBe(true);
});

test("a cancelled run reports in the requested language with exit code 130 and an unknown operation is refused before anything runs",async()=>{
 const stopped=new AbortController();stopped.abort(Error("stop"));
 const ports=(signal:AbortSignal)=>cacheRunnerPortsV1({signal,progress:async()=>undefined,exclusive:async()=>({release:()=>undefined})});
 for(const[args,language]of[[["status"],"en"],[["status","--lang=de"],"de"],[["gc","--dry-run","--lang","de"],"de"]] as [string[],"en"|"de"][]){
  const lines:string[]=[],code=await runCacheLedgerCommandV1(args,{workspace:"/nonexistent-cache-workspace",environment:{},write:line=>lines.push(line),ledger},ports(stopped.signal));
  expect(code,args.join(" ")).toBe(130);expect(lines.at(-1),args.join(" ")).toBe(cacheTextV1(language,"error.cancelled"));
 }
 await expect(runCacheLedgerCommandV1(["bogus"],{workspace:"/x",environment:{},write:()=>undefined,ledger},ports(new AbortController().signal))).rejects.toThrow("Expected repo cache status|gc");
});
