import assert from "node:assert/strict";
import {createRequire} from "node:module";
import {mkdir,readFile,writeFile,stat} from "node:fs/promises";
import {dirname,join,resolve} from "node:path";

const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch]=process.argv.slice(2);
assert.equal(command,"verify");assert.match(epoch??"",/^\d+$/);
const output=join(ticket,"🗑️generated/dependency-coordinates",epoch),path=join(root,"🔒️dependencies.json"),producer=await readFile(import.meta.path,"utf8"),before=await readFile(join(import.meta.dir,"before.json"),"utf8"),current=await readFile(path,"utf8"),corpus=JSON.parse(await readFile(join(import.meta.dir,"corpus.json"),"utf8")),schema=JSON.parse(await readFile(join(import.meta.dir,"schema.json"),"utf8"));
await mkdir(output,{recursive:true});const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),oracle=new Ajv({strict:true}).compile(schema);
let code=1,error:string|undefined,expected=before;
try{
 assert.equal(oracle(corpus),true,JSON.stringify(oracle.errors));
 assert.equal(corpus.version,1);assert.ok(Array.isArray(corpus.moves)&&corpus.moves.length>0);
 for(const move of corpus.moves){assert.equal(typeof move.from,"string");assert.equal(typeof move.to,"string");assert.ok(Number.isInteger(move.occurrences)&&move.occurrences>0);const from=JSON.stringify(move.from),to=JSON.stringify(move.to);assert.equal(expected.split(from).length-1,move.occurrences);expected=expected.replaceAll(from,to);assert.equal((await stat(join(root,move.to))).isFile(),true);}
 const beforeValue=JSON.parse(before),afterValue=JSON.parse(current),expectedValue=JSON.parse(expected),parse=require("jsonc-parser").parse;
 assert.deepEqual(parse(before),beforeValue);assert.deepEqual(parse(current),afterValue);assert.deepEqual(parse(expected),expectedValue);
 assert.deepEqual(afterValue,expectedValue,"Only the seven admitted dependency user coordinates may move");
 assert.equal(current,expected,"Every non-coordinate byte must remain unchanged");
 assert.equal(beforeValue.entries.length,afterValue.entries.length);
 assert.equal(await readFile(path,"utf8"),current);assert.equal(await readFile(import.meta.path,"utf8"),producer);
 console.log("[DEBUG] Canonical dependency coordinates="+corpus.moves.reduce((total:number,row:any)=>total+row.occurrences,0)+"; all foreign entries and non-coordinate bytes conserved");code=0;
}catch(failure){error=String(failure);}finally{await writeFile(join(output,"terminal.json"),JSON.stringify({code,error,producer,before,current,expected,corpus,schema,sourceExact:await readFile(path,"utf8")===current,producerExact:await readFile(import.meta.path,"utf8")===producer,atomicSnapshotClaimed:false}));process.exitCode=code;}
