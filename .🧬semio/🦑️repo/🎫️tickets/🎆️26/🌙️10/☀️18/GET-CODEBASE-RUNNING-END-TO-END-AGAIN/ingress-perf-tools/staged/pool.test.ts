import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import fixture from "./@JSON@" with {type:"json"};
import {createCustodyPoolV1} from "../@TS@";

type Step={op:string;id?:string;ms?:number;fail?:boolean};
const settle=()=>new Promise<void>(done=>setTimeout(done,1));

async function replay(row:{steps:Step[]}){
 let clock=0,counter=0;const timers=new Map<number,{due:number;run:()=>void}>(),open:Array<{id:number;closed:boolean}>=[];
 const pool=createCustodyPoolV1<number>({idleMilliseconds:fixture.idleMilliseconds,maximumLifetimeMilliseconds:fixture.maximumLifetimeMilliseconds,
  timer:{now:()=>clock,set:(ms,run)=>{const handle=++counter;timers.set(handle,{due:clock+ms,run});return handle;},clear:handle=>{timers.delete(handle as number);}},
  open:async()=>{const entry={id:open.length+1,closed:false};open.push(entry);return{session:entry.id,close:async()=>{entry.closed=true;}};}});
 const running=new Map<string,{release:()=>void;done:Promise<unknown>}>();
 for(const step of row.steps){
  if(step.op==="advance"){clock+=step.ms!;for(const[handle,timer]of[...timers])if(timer.due<=clock){timers.delete(handle);timer.run();}await settle();}
  else if(step.op==="with"){if(step.fail)await expect(pool.with(async()=>{throw Error("custody lost");})).rejects.toThrow("custody lost");else await pool.with(async session=>session);await settle();}
  else if(step.op==="start"){let release!:()=>void;const gate=new Promise<void>(done=>{release=done;});running.set(step.id!,{release,done:pool.with(async()=>{await gate;})});await settle();}
  else if(step.op==="finish"){const entry=running.get(step.id!)!;entry.release();await entry.done;await settle();}
  else if(step.op==="close"){await pool.close();await settle();}
 }
 return{opens:pool.opens(),closes:pool.closes(),live:pool.live()};
}

const python=["python3","python"].find(name=>spawnSync(name,["-c","pass"]).status===0);
const oracleSource=`
import json,sys
fixture=json.load(open(sys.argv[1]));idle=fixture["idleMilliseconds"];life=fixture["maximumLifetimeMilliseconds"];out=[]
for case in fixture["cases"]:
    clock=0;opened=None;opens=closes=0;inflight=0;idle_due=None;pending={}
    def fire():
        global opened,closes,idle_due
        if idle_due is not None and idle_due<=clock and inflight==0 and opened is not None:
            opened=None;closes+=1;idle_due=None
    def begin():
        global opened,opens,closes,idle_due,inflight
        idle_due=None
        if opened is not None and inflight==0 and clock-opened>=life: opened=None;closes+=1
        if opened is None: opened=clock;opens+=1
        inflight+=1
    def end(fail=False):
        global opened,closes,idle_due,inflight
        inflight-=1
        if fail and opened is not None: opened=None;closes+=1
        if opened is not None and inflight==0: idle_due=clock+idle
    for step in case["steps"]:
        op=step["op"]
        if op=="advance": clock+=step["ms"];fire()
        elif op=="with": begin();end(step.get("fail",False))
        elif op=="start": begin()
        elif op=="finish": end()
        elif op=="close":
            if opened is not None: opened=None;closes+=1
            idle_due=None
    out.append({"opens":opens,"closes":closes,"live":opened is not None})
print(json.dumps(out))
`;
const oracle=python?JSON.parse(spawnSync(python,["-I","-c",oracleSource,fileURLToPath(new URL("./@JSON@",import.meta.url))],{encoding:"utf8"}).stdout||"null"):null;

for(const[index,row]of fixture.cases.entries())test("custody pool: "+row.name,async()=>{
 const got=await replay(row as {steps:Step[]});
 expect(got).toEqual({opens:row.opens,closes:row.closes,live:row.live});
 if(oracle)expect(got).toEqual(oracle[index]);
});

test("custody pool: lifetimes must be bounded",()=>{
 const timer={now:()=>0,set:()=>1,clear:()=>undefined},open=async()=>({session:1,close:async()=>undefined});
 expect(()=>createCustodyPoolV1({open,idleMilliseconds:0,maximumLifetimeMilliseconds:10,timer})).toThrow("Bounded custody pool lifetimes required");
 expect(()=>createCustodyPoolV1({open,idleMilliseconds:20,maximumLifetimeMilliseconds:10,timer})).toThrow("Bounded custody pool lifetimes required");
});
