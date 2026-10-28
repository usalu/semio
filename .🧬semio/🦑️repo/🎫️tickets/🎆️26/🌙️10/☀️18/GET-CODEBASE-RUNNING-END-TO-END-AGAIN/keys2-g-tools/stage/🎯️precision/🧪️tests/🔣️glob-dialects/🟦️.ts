/** 🔣 Contract: the three glob compilers of the build (workspace TypeScript units, parity paths, input precision) select exactly the paths authored in `./🔣️.json`, agree with an independent Python translation on every authored and every swept path, agree with minimatch where the dialects coincide and differ from it exactly where the corpus says so, treat a line break in a path like any other character and refuse an unterminated brace group. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import {requireTestToolV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {globRegexV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪞️parity/🟦️.ts";
import {workspaceGlobRegExpV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/🧩️units/🟦️.ts";
import {inputGlobRegExpV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(cases:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({cases}),encoding:"utf8",maxBuffer:1<<28});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const engines:Record<string,(pattern:string)=>RegExp>={ws:workspaceGlobRegExpV1,parity:globRegexV1,input:inputGlobRegExpV1};
const mini=(pattern:string,path:string)=>minimatch(path,pattern,{dot:true,nonegate:true,nocomment:true,noext:true});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,rows:[{...corpus.rows[0],engines:["glob"]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,rows:[{...corpus.rows[0],minimatch:"maybe"}]})).toBe(false);
});

test("every engine of a row selects its authored paths and rejects its authored non-paths",()=>{
 for(const row of corpus.rows)for(const engine of row.engines){
  const expression=engines[engine]!(row.pattern);
  for(const path of row.matches)expect(expression.test(path),`${engine} ${JSON.stringify(row.pattern)} selects ${JSON.stringify(path)}`).toBe(true);
  for(const path of row.rejects)expect(expression.test(path),`${engine} ${JSON.stringify(row.pattern)} rejects ${JSON.stringify(path)}`).toBe(false);
 }
});

test("an independent Python translation gives the same verdicts on every authored path",()=>{
 const cases=corpus.rows.flatMap(row=>row.engines.map(engine=>({engine,pattern:row.pattern,paths:[...row.matches,...row.rejects],expected:[...row.matches.map(()=>true),...row.rejects.map(()=>false)]})));
 const answers=ask(cases.map(({engine,pattern,paths})=>({engine,pattern,paths})));
 for(const[index,row]of cases.entries())expect(answers[index].matches,`${row.engine} ${JSON.stringify(row.pattern)}`).toEqual(row.expected);
});

test("minimatch gives the same verdicts where the dialects coincide and different ones where the corpus says they differ",()=>{
 for(const row of corpus.rows){
  const verdicts=[...row.matches.map(path=>[path,true] as const),...row.rejects.map(path=>[path,false] as const)];
  const different=verdicts.filter(([path,expected])=>mini(row.pattern,path)!==expected);
  if(row.minimatch==="agrees")expect(different,JSON.stringify(row.pattern)).toEqual([]);
  else expect(different.length,JSON.stringify(row.pattern)+" "+(row.note??"")).toBeGreaterThan(0);
 }
});

const alphabet=["a","b","/","."],sweep:string[]=[];
for(let length=0;length<=6;length++){const walk=(prefix:string)=>{if(prefix.length===length){sweep.push(prefix);return;}for(const character of alphabet)walk(prefix+character);};walk("");}
const regular=(path:string)=>path!==""&&!path.startsWith("/")&&!path.endsWith("/")&&!path.includes("//")&&!path.split("/").some(segment=>segment==="."||segment==="..");
const sweepPatterns=["*","**","a/**","a/**/b","**/b","**/*.a","*/b","a/*","a*b","*a","b/*/a","**/**","a/**/b/**"];

test("the three engines, the Python translation and minimatch agree on every short path over a small alphabet",()=>{
 const answers=ask(sweepPatterns.map(pattern=>({engine:"input",pattern,paths:sweep})));
 for(const[index,pattern]of sweepPatterns.entries()){
  const reference=engines.input!(pattern),others=[engines.ws!(pattern),engines.parity!(pattern)];
  for(const[position,path]of sweep.entries()){
   const verdict=reference.test(path);
   if(others.some(expression=>expression.test(path)!==verdict))throw Error(`engines disagree on ${JSON.stringify(pattern)} for ${JSON.stringify(path)}`);
   if(answers[index].matches[position]!==verdict)throw Error(`oracle disagrees on ${JSON.stringify(pattern)} for ${JSON.stringify(path)}`);
   if(!pattern.includes("**")||/(^|\/)\*\*(\/|$)/u.test(pattern))if(regular(path)&&!/\*\*/u.test(pattern.replace(/(^|\/)\*\*(\/|$)/gu,"")))if(mini(pattern,path)!==verdict)throw Error(`minimatch disagrees on ${JSON.stringify(pattern)} for ${JSON.stringify(path)}`);
  }
 }
});

test("an unterminated brace group is a syntax error of the expression compiler",()=>{
 for(const row of corpus.errors)expect(()=>engines[row.engine]!(row.pattern),row.name).toThrow(SyntaxError);
 expect(inputGlobRegExpV1("a/*.ts")).toBe(inputGlobRegExpV1("a/*.ts"));
});
