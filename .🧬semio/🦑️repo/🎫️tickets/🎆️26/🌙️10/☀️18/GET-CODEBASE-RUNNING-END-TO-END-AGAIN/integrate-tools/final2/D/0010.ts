import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {appendFileSync,linkSync,mkdtempSync,renameSync,rmSync,unlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import fixture from "./🔣️.json" with {type:"json"};
import {createJournalReceiptV1,observeJournalRevisionV1} from "../🟦️.ts";

const pause=()=>new Promise<void>(done=>setTimeout(done,15));

async function replay(directory:string){
 const path=join(directory,"events.jsonl"),seen:Array<boolean|null>=[];let previous:string|undefined;
 for(const step of fixture.steps as Array<{op:string;text?:string}>){
  await pause();
  if(step.op==="write"||step.op==="rewrite")writeFileSync(path,step.text!);
  else if(step.op==="append")appendFileSync(path,step.text!);
  else if(step.op==="touch")utimesSync(path,new Date(),new Date(Date.now()+1000));
  else if(step.op==="replace"){const next=path+"~";writeFileSync(next,step.text!);renameSync(next,path);}
  else if(step.op==="link")linkSync(path,path+".link");
  else if(step.op==="unlink"){unlinkSync(path+".link");unlinkSync(path);}
  const revision=observeJournalRevisionV1(path);
  seen.push(revision===undefined?null:previous===undefined?null:revision===previous);
  previous=revision;
 }
 return seen;
}

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const oracleSource=`
import json,os,sys,time
directory=sys.argv[1];fixture=json.load(open(sys.argv[2]));path=os.path.join(directory,"events.jsonl");seen=[];previous=None
def revision():
    try:
        row=os.lstat(path)
    except OSError:
        return None
    import stat
    if not stat.S_ISREG(row.st_mode) or row.st_nlink!=1: return None
    return (row.st_dev,row.st_ino,row.st_size,row.st_mtime_ns,row.st_ctime_ns)
for step in fixture["steps"]:
    time.sleep(0.015);op=step["op"]
    if op in("write","rewrite"): open(path,"w").write(step["text"])
    elif op=="append": open(path,"a").write(step["text"])
    elif op=="touch": os.utime(path,(time.time(),time.time()+1))
    elif op=="replace":
        open(path+"~","w").write(step["text"]);os.replace(path+"~",path)
    elif op=="link": os.link(path,path+".link")
    elif op=="unlink": os.unlink(path+".link");os.unlink(path)
    current=revision()
    seen.append(None if current is None or previous is None else current==previous)
    previous=current
print(json.dumps(seen))
`;

test("journal revision: every mutation changes it and a plain re-observation does not",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"journal-revision-"));
 try{
  const seen=await replay(directory);
  expect(seen).toEqual(fixture.steps.map(step=>step.equalToPrevious));
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test.skipIf(!python)("journal revision: agrees with an independent Python stat model",async()=>{
 const first=mkdtempSync(join(tmpdir(),"journal-revision-a-")),second=mkdtempSync(join(tmpdir(),"journal-revision-b-"));
 try{
  const ours=await replay(first),theirs=JSON.parse(spawnSync(python!,["-I","-c",oracleSource,second,fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"}).stdout);
  expect(ours).toEqual(theirs);
 }finally{rmSync(first,{recursive:true,force:true});rmSync(second,{recursive:true,force:true});}
});

test("journal receipt: recalls a state only for the revision it was read under",()=>{
 const directory=mkdtempSync(join(tmpdir(),"journal-receipt-")),path=join(directory,"events.jsonl");
 try{
  writeFileSync(path,"a\n");const receipt=createJournalReceiptV1<string>(path),first=receipt.revision()!;
  expect(receipt.recall(first)).toBeUndefined();
  receipt.remember(first,"state-a");
  expect(receipt.recall(first)).toBe("state-a");
  appendFileSync(path,"b\n");
  expect(receipt.recall(receipt.revision()!)).toBeUndefined();
  expect(createJournalReceiptV1<string>(join(directory,"absent")).revision()).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
