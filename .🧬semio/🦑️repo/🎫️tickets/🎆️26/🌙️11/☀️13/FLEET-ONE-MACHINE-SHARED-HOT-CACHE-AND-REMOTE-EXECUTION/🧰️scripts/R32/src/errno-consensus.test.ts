/** 🎯️ The failure number of a read-only relative call is read by a second native call, so it counts only when two failed calls in a row report it; the loop equals an independent Python reading of the same rule on seeded random sequences, and a real helper keeps answering ENOENT for absent entries and names the directory of a refused entry. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {getSystemErrorName} from "node:util";
import {settleNativeCallV1,withArtifactRelativeDirectoryV1} from "../../📁️descriptor/🟦️.ts";

const transient=(number:number)=>number===4||getSystemErrorName(-number).startsWith("Unknown");

const script=(steps:readonly(number|"ok")[],attempts=4)=>{
  let at=0,reads=0,checks=0;
  const outcome=settleNativeCallV1(()=>{const step=steps[Math.min(at++,steps.length-1)]!;return step==="ok"?0:-1;},()=>{reads++;return steps[Math.min(at-1,steps.length-1)] as number;},transient,attempts,()=>{checks++;});
  return{outcome,calls:at,reads,checks};
};

test("a number counts when two failed calls in a row report it, a single odd reading never decides (the cause of the fake EPERM for an absent entry)",()=>{
  expect(script([2,2]).outcome).toEqual({status:-1,number:2,reads:2});
  expect(script([1,2,2])).toMatchObject({outcome:{status:-1,number:2,reads:2},calls:3,checks:2});
  expect(script([316,2,2]).outcome).toEqual({status:-1,number:2,reads:2});
  expect(script([1,1]).outcome).toEqual({status:-1,number:1,reads:2});
  expect(script([1,2,1,2]).outcome).toEqual({status:-1,number:2,reads:1});
});

test("a success on a repeat is the answer, EINTR and numbers without a name are retried without counting, the attempts bound the loop",()=>{
  expect(script(["ok"])).toMatchObject({outcome:{status:0,number:0,reads:0},calls:1,reads:0,checks:0});
  expect(script([1,"ok"]).outcome).toEqual({status:0,number:0,reads:0});
  expect(script([4,4,2,2]).outcome).toEqual({status:-1,number:2,reads:2});
  expect(script([2,4,2]).outcome).toEqual({status:-1,number:2,reads:2});
  expect(script([316,316,316,316,316]).outcome).toEqual({status:-1,number:316,reads:0});
  expect(script([4,4,4,4,4],3)).toMatchObject({calls:3});
  expect(script([13,13,13]).outcome).toEqual({status:-1,number:13,reads:2});
});

test("the loop equals an independent Python reading of the rule on 400 seeded random sequences",()=>{
  let seed=20261115;const next=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed;};
  const alphabet:(number|"ok")[]=[1,2,2,2,4,13,316,"ok"];
  const cases=Array.from({length:400},()=>({attempts:1+next()%6,steps:Array.from({length:7},()=>alphabet[next()%alphabet.length]!)}));
  const mine=cases.map(({steps,attempts})=>script(steps,attempts).outcome);
  const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️oracle.py",import.meta.url))],{input:JSON.stringify(cases),encoding:"utf8"});
  expect(oracle.status,oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual(mine);
});

test("a real helper answers ENOENT for 20000 absent entries in a row and keeps the directory of a refused entry on the error",async()=>{
  const root=mkdtempSync(join(tmpdir(),"errno-consensus-"));
  mkdirSync(join(root,"capsule"));writeFileSync(join(root,"capsule","image.json"),"{}");
  try{
    await withArtifactRelativeDirectoryV1(root,()=>{},async anchor=>anchor.withDirectory("capsule",async current=>{
      for(let index=0;index<20000;index++)expect(await current.entry(".reset.json")).toBeNull();
      expect((await current.entry("image.json"))?.kind).toBe("file");
    }));
    if(process.platform!=="win32"&&process.getuid?.()!==0){
      mkdirSync(join(root,"capsule","inner"));chmodSync(join(root,"capsule","inner"),0o600);
      let caught:(Error&{code?:string;root?:string;entry?:string;directory?:string;reads?:number})|undefined;
      await withArtifactRelativeDirectoryV1(root,()=>{},async anchor=>anchor.withDirectory("capsule",async current=>current.withDirectory("inner",async inner=>{await inner.entry(".reset.json");}))).catch(error=>{caught=error;});
      expect(caught).toBeDefined();
      expect(caught!.message).toMatch(/^Relative filesystem fstatat failed \((EACCES|EPERM)\)$/u);
      expect(caught!.root).toBe(root);expect(caught!.entry).toBe(".reset.json");
      expect(caught!.directory).toBe(join(root,"capsule","inner"));expect(caught!.reads).toBe(2);
    }
  }finally{try{chmodSync(join(root,"capsule","inner"),0o700);}catch{}rmSync(root,{recursive:true,force:true});}
});
