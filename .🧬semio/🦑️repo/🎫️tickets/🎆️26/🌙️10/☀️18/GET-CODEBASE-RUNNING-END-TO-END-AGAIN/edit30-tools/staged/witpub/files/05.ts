/** 🧾️ Contract: the Bazel input witness is the same digest whether it is computed by the reference function over the bytes, by the publisher from captured rows (path, size, content sha256) of a written plan, or by an independent Python implementation, for every case of `./🔣️.json`; the rules hold no witness action any more. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {bazelStarlarkRulesV1} from "../../../../📜️script.ts";
import {bazelWitnessDigestV1,bazelWitnessFilesOfPlanV1,bazelWitnessFilesV1,bazelWitnessPlanTextV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({rows}),encoding:"utf8",maxBuffer:128*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const bytesOf=(content:any):Buffer=>Buffer.from("repeat" in content?content.repeat.repeat(content.times):content.text);
let workspace="";
beforeAll(()=>{workspace=realpathSync(mkdtempSync(join(tmpdir(),"witness-")));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],expected:{version:2,files:0,sha256:"x"}}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[1],files:[{path:"a\nb",content:{text:"x"}}]}]})).toBe(false);
});

test("the reference digest equals the authored digest and the Python digest for every case",()=>{
 const answers=ask(corpus.cases);
 for(const[index,row]of corpus.cases.entries()){
  const files=row.files.map(file=>({path:file.path,bytes:bytesOf(file.content)}));
  expect<unknown>(bazelWitnessDigestV1(files),row.name).toEqual(row.expected);expect(answers[index],row.name).toEqual(row.expected);
  expect<unknown>(bazelWitnessDigestV1([...files].reverse()),row.name+" reversed").toEqual(row.expected);
 }
 const changed=bazelWitnessDigestV1([{path:"a.ts",bytes:Buffer.from("abc")}]),other=bazelWitnessDigestV1([{path:"a.ts",bytes:Buffer.from("abd")}]);
 expect(changed.sha256).not.toBe(other.sha256);
});

test("the publisher writes exactly that digest for every case from the captured rows of a written plan",()=>{
 for(const[index,row]of corpus.cases.entries()){
  const rows=new Map(row.files.map(file=>{const bytes=bytesOf(file.content);return[file.path,{path:file.path,bytes:bytes.length,sha256:createHash("sha256").update(bytes).digest("hex")}] as const;}));
  const plan=JSON.parse(bazelWitnessPlanTextV1(new Map([["w_case"+index,row.files.map(file=>file.path)]])));
  const files=bazelWitnessFilesOfPlanV1(plan,rows);expect(files.length,row.name).toBe(1);
  const[name,text]=files[0]!;expect(name,row.name).toBe("w_case"+index+bazelWitnessFilesV1.suffix);expect(text.endsWith("\n"),row.name).toBe(true);expect(JSON.parse(text),row.name).toEqual(row.expected);
 }
});

test("the rules hold no witness action: a witnessed test takes the witness as a source file",()=>{
 const rules=bazelStarlarkRulesV1();
 for(const gone of["RepoInputsWitness","repo_inputs_witness","witness.mjs"])expect(rules,gone).not.toContain(gone);
 expect(rules).toContain('"witnesses": attr.label_list(allow_files = True)');
 expect(rules.includes("\t")).toBe(false);
});
