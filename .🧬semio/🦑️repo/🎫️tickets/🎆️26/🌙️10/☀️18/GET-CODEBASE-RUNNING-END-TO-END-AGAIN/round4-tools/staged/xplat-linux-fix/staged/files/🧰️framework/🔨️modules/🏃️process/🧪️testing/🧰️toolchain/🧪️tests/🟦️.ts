import{expect,test}from"bun:test";
import Ajv from"ajv";
import{chmodSync,lstatSync,mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{join,resolve}from"node:path";
import{tmpdir}from"node:os";
import fixture from"./🧫️fixtures/🔣️.json";
import{describeMissingTestToolV1,makeTestTreeWritableV1,requireTestToolV1,resolveTestToolV1,testPythonV1,testToolPathV1,type TestToolchainPortV1}from"../🟦️.ts";

const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["name","platform","which","runs","tool","path","tried"],properties:{name:{type:"string"},platform:{enum:["linux","darwin","win32"]},which:{type:"object",additionalProperties:{type:"string"}},runs:{type:"object",additionalProperties:{type:"array"}},tool:{enum:["python","ripgrep"]},path:{type:["string","null"]},tried:{type:"array",items:{type:"string"}}}});
const port=(row:{platform:string;which:Record<string,string>;runs:Record<string,unknown[]>},written:string[]=[],environment:Record<string,string>={}):TestToolchainPortV1=>({platform:row.platform as NodeJS.Platform,which:command=>row.which[command]??null,run:(command,args)=>{const answer=row.runs[[command,...args].join(" ")];return answer?{status:answer[0] as number,stdout:String(answer[1])}:{status:1,stdout:""};},environment,write:text=>{written.push(text);}});

test("a tool is the first declared candidate that answers its probe like the wanted program, on every host",()=>{
 for(const row of fixture.resolution){
  expect(check(row),row.name+JSON.stringify(check.errors)).toBe(true);
  const result=resolveTestToolV1(row.tool as "python",port(row as never));
  expect(result.path,row.name).toBe(row.path);expect(result.tried,row.name).toEqual(row.tried);
 }
});

test("a missing tool is announced once with the reason in the declared language and a required tool throws it",()=>{
 const row=fixture.resolution[3]!,written:string[]=[];
 const english=port(row as never,written,{SEMIO_LANGUAGE:"en"});
 expect(testToolPathV1("python",english)).toBeNull();expect(written).toHaveLength(1);expect(written[0]).toContain("python3, python");expect(written[0]).toContain("Test tool python not found");
 const german=port(row as never,[],{SEMIO_LANGUAGE:"de"});
 expect(describeMissingTestToolV1(resolveTestToolV1("python",german),german.environment)).toContain("Test-Werkzeug python nicht gefunden");
 expect(describeMissingTestToolV1(resolveTestToolV1("python",english),{LANG:"de_DE.UTF-8"})).toContain("nicht gefunden");
 expect(describeMissingTestToolV1(resolveTestToolV1("python",english),{})).toContain("Test tool");
 expect(()=>requireTestToolV1("python",english)).toThrow("not found");
});

test("on this host the resolved Python really is a Python 3 and ripgrep resolves or is reported",()=>{
 const python=testPythonV1();
 if(python!==null){const answer=Bun.spawnSync([python,"-c","import sys;print(sys.version_info[0])"],{stdout:"pipe"});expect(answer.stdout.toString().trim()).toBe("3");}
 expect(testPythonV1()).toBe(python);
});

test("making a tree writable needs no external program, skips links and equals chmod -R u+w where chmod exists",()=>{
 const base=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"wr-"));try{
  const build=(name:string)=>{const root=join(base,name);mkdirSync(join(root,"a","b"),{recursive:true});writeFileSync(join(root,"a","b","f"),"x");writeFileSync(join(root,"a","g"),"y");symlinkSync(join(base),join(root,"escape"));for(const path of[join(root,"a","b","f"),join(root,"a","g")])chmodSync(path,0o444);for(const path of[join(root,"a","b"),join(root,"a")])chmodSync(path,0o555);return root;};
  const ours=build("ours");makeTestTreeWritableV1(ours);
  expect(lstatSync(join(ours,"a","b")).mode&0o200).toBe(0o200);expect(lstatSync(join(ours,"a","b","f")).mode&0o200).toBe(0o200);
  rmSync(ours,{recursive:true,force:false});
  if(process.platform!=="win32"){const twin=build("twin"),modes=(root:string)=>["a","a/b","a/b/f","a/g"].map(path=>lstatSync(join(root,path)).mode&0o777);Bun.spawnSync(["chmod","-R","u+w",twin]);const reference=modes(twin);const again=build("again");makeTestTreeWritableV1(again);expect(modes(again)).toEqual(reference);rmSync(twin,{recursive:true});rmSync(again,{recursive:true});}
  expect(()=>makeTestTreeWritableV1(join(base,"absent"))).not.toThrow();
 }finally{for(const name of["ours","twin","again"]){try{makeTestTreeWritableV1(join(base,name));}catch{}}rmSync(base,{recursive:true,force:true});}
});
