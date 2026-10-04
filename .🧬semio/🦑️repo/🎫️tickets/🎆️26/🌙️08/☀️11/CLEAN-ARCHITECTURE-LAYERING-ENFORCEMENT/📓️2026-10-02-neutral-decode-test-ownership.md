# Neutral Decode Test Ownership

This source cut assigns the two existing native decode-control laws and original TypeScript schema/SQLite reference corpus to their actual neutral Value owner. Their old names, complete bodies, fixtures, schema and independent reference implementations remain unchanged. Record binding laws and actual ArtifactChild product scenarios are separate outstanding ownership work; no source removal or generic replacement is proposed here.

The higher duplicate control-law mount will be retired only after the lower package actually discovers and executes the original two native laws. Full Value default native route remains unfiltered. New package test-decode-ownership will execute ownership admission and the existing two TypeScript decode laws; existing construction portable law stays selected. No new runtime dependency or compatibility API is introduced.

## Original Inputs

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🦀️.rs

Bytes6778; SHA-256 3f59fead0f187444a9c4bb6fd049b43762966bc2222c3706ce3beb7a7a85209d

````text
//! 🛬️ Cumulative ownership admission shared by native parsers and typed field construction.

/// ⏱️ Decoder work units and admitted owned bytes at one cancellation boundary.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeProgress { pub completed:usize, pub total:usize, pub owned_bytes:usize }

/// 🧮️ One caller-owned budget persists from input scanning through final typed construction.
pub struct NativeDecodeControl<'a> { maximum_bytes:usize, owned_bytes:usize, completed:usize, total:usize, started:bool, stage:u64, depth:usize, callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool }

impl<'a> NativeDecodeControl<'a> {
    /// 🚦️ Binds the explicit allocation ceiling and cancellation callback.
    pub fn new(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Self { Self{maximum_bytes,owned_bytes:0,completed:0,total:0,started:false,stage:0,depth:0,callback} }
    /// 🪆️ Bounds recursive typed construction independently of physical input parsing.
    pub fn scoped_depth<T,E:From<String>>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        if self.depth>=maximum{return Err(E::from("native typed construction exceeds depth limit".into()))}
        self.depth+=1;let result=operation(self);self.depth-=1;result
    }
    /// 📊️ Returns the cumulative ownership already admitted for this operation.
    pub fn owned_bytes(&self)->usize { self.owned_bytes }
    /// 📏️ Returns the caller's complete native allocation allowance.
    pub fn maximum_bytes(&self)->usize { self.maximum_bytes }
    /// 🛑️ Checks cancellation before an explicitly owned expensive operation.
    pub fn checkpoint(&mut self)->Result<(),String> { if(self.callback)(NativeDecodeProgress{completed:self.completed,total:self.total,owned_bytes:self.owned_bytes}){self.started=true;Ok(())}else{Err("native decoding canceled".into())} }
    /// 🧭️ Begins a known stage workload while retaining every previously admitted byte.
    pub fn begin_stage(&mut self,total:usize)->Result<(),String>{self.stage=self.stage.checked_add(1).ok_or("native decoding stage overflow")?;self.completed=0;self.total=total;self.started=false;self.checkpoint()}
    /// 🪆️ Restores a parent workload after a child begins its own stage, preserving cumulative ownership.
    pub fn scoped_stage<T,E>(&mut self,operation:impl FnOnce(&mut Self)->Result<T,E>)->Result<T,E>{
        let parent=(self.completed,self.total,self.started,self.stage);let result=operation(self);
        if self.stage!=parent.3{self.completed=parent.0;self.total=parent.1;self.started=parent.2;self.stage=parent.3;}
        result
    }
    /// 📐️ Restricts one owned decoder stage to its domain ceiling, retaining cumulative charges afterward.
    pub fn scoped_maximum<T>(&mut self,maximum:usize,operation:impl FnOnce(&mut Self)->Result<T,String>)->Result<T,String>{let parent=self.maximum_bytes;self.maximum_bytes=parent.min(maximum);if self.owned_bytes>self.maximum_bytes{self.maximum_bytes=parent;return Err("native decoding ownership exceeds stage limit".into());}let result=operation(self);self.maximum_bytes=parent;result}
    /// 📍️ Advances consumed stage units and checks cancellation across256-unit boundaries.
    pub fn advance(&mut self,units:usize)->Result<(),String>{if !self.started{self.checkpoint()?;}let previous=self.completed;self.completed=self.completed.checked_add(units).ok_or("native decoding work overflow")?;if self.total!=0&&self.completed>self.total{return Err("native decoding exceeded declared stage workload".into());}if previous/256!=self.completed/256||(self.total!=0&&self.completed==self.total){self.checkpoint()?;}Ok(())}
    /// 🔢️ Advances decoder work and publishes a checkpoint every256units.
    pub fn step(&mut self)->Result<(),String> {self.advance(1)}
    /// 📦️ Admits cumulative storage before a caller copies or reserves it.
    pub fn charge(&mut self,bytes:usize)->Result<(),String> { let next=self.owned_bytes.checked_add(bytes).filter(|next|*next<=self.maximum_bytes).ok_or("native decoding ownership exceeds caller limit")?;if !self.started||bytes>65536 {self.checkpoint()?;}self.owned_bytes=next;Ok(()) }
    /// 🗂️ Reserves typed collection slots after overflow and caller-bound admission.
    pub fn allocate_vec<T>(&mut self,count:usize)->Result<Vec<T>,String> { let bytes=count.checked_mul(std::mem::size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or("native decoding collection size overflow")?;self.charge(bytes)?;let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|"native decoding collection allocation failed")?;Ok(output) }
    fn copy_checkpoint(&mut self,completed:usize,total:usize)->Result<(),String>{if(self.callback)(NativeDecodeProgress{completed,total,owned_bytes:self.owned_bytes}){Ok(())}else{Err("native decoding canceled".into())}}
    /// 🔎️ Validates borrowed UTF-8 in bounded spans without owning a second text buffer.
    pub fn borrow_text<'text>(&mut self,bytes:&'text[u8])->Result<&'text str,String>{
        self.copy_checkpoint(0,bytes.len())?;let mut position=0;
        while position<bytes.len(){
            let mut end=position.saturating_add(65536).min(bytes.len());
            loop{match std::str::from_utf8(&bytes[position..end]){Ok(_)=>break,Err(error) if error.error_len().is_none()&&end<bytes.len()=>{end+=1;},Err(_)=>return Err("invalid native UTF-8".into())}}
            position=end;self.copy_checkpoint(position,bytes.len())?;
        }
        Ok(unsafe{std::str::from_utf8_unchecked(bytes)})
    }
    /// 🔤️ Copies owned UTF-8 in cancellable64KiB spans, preserving the outer work stage.
    pub fn copy_text(&mut self,text:&str)->Result<String,String> {
        self.charge(text.len())?;self.copy_checkpoint(0,text.len())?;
        let mut output=String::new();output.try_reserve_exact(text.len()).map_err(|_|"native decoding text allocation failed")?;
        let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[position..end]);position=end;self.copy_checkpoint(position,text.len())?;}Ok(output)
    }
    /// 🧬️ Copies intrinsic octets in cancellable64KiB spans after complete ownership admission.
    pub fn copy_bytes(&mut self,bytes:&[u8])->Result<Vec<u8>,String> {
        self.charge(bytes.len())?;self.copy_checkpoint(0,bytes.len())?;
        let mut output=Vec::new();output.try_reserve_exact(bytes.len()).map_err(|_|"native decoding collection allocation failed")?;
        for chunk in bytes.chunks(65536){output.extend_from_slice(chunk);self.copy_checkpoint(output.len(),bytes.len())?;}Ok(output)
    }
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts

Bytes6263; SHA-256 3b513e0477b3d72090de256fa26460520f25a0e7ab3a0460f7e10c42bd54d8af

````text
/** 🛬️ Cumulative ownership admission spanning native parsing and typed construction. */
export interface NativeDecodeProgress { readonly completed:number;readonly total:number;readonly ownedBytes:number }
/** 🧮️ One explicit native allocation budget and cancellable decoder work cursor. */
export class NativeDecodeControl {
  private owned=0;
  private completed=0;
  private total=0;
  private started=false;
  private stage=0;
  private maximum:number;
  constructor(maximumBytes:number,private readonly progress:(event:NativeDecodeProgress)=>boolean,private readonly signal?:AbortSignal) {
    if(!Number.isSafeInteger(maximumBytes)||maximumBytes<0)throw new Error("Invalid native decoding allocation ceiling");
    this.maximum=maximumBytes;
  }
  /** 📏️ Current domain allocation ceiling within the caller allowance. */
  get maximumBytes():number{return this.maximum;}
  /** 📊️ Ownership admitted throughout the complete decoding operation. */
  get ownedBytes():number{return this.owned;}
  /** 🛑️ Yield before expensive ownership and receive caller cancellation. */
  async checkpoint():Promise<void>{
    if(this.signal?.aborted||!this.progress({completed:this.completed,total:this.total,ownedBytes:this.owned}))throw new Error("native decoding canceled");
    this.started=true;
    await new Promise<void>(resolve=>setTimeout(resolve,0));
    if(this.signal?.aborted)throw new Error("native decoding canceled");
  }
  /** 🔢️ Advance decoder work and publish every256units. */
  async step():Promise<void>{await this.advance(1);}
  /** 🧭️ Begin a known stage while retaining the complete ownership budget. */
  async beginStage(total:number):Promise<void>{if(!Number.isSafeInteger(total)||total<0||!Number.isSafeInteger(this.stage+1))throw new Error("Invalid native decoding workload");this.stage++;this.completed=0;this.total=total;this.started=false;await this.checkpoint();}
  /** 🪆️ Preserve parent work after a child begins a stage while retaining cumulative ownership. */
  async scopedStage<T>(operation:(control:NativeDecodeControl)=>Promise<T>):Promise<T>{const parent={completed:this.completed,total:this.total,started:this.started,stage:this.stage};try{return await operation(this);}finally{if(this.stage!==parent.stage){this.completed=parent.completed;this.total=parent.total;this.started=parent.started;this.stage=parent.stage;}}}
  /** 📐️ Restrict one stage's owned allocation without resetting cumulative charges. */
  async scopedMaximum<T>(maximum:number,operation:(control:NativeDecodeControl)=>Promise<T>):Promise<T>{if(!Number.isSafeInteger(maximum)||maximum<0)throw new Error("Invalid native stage allowance");const parent=this.maximum;this.maximum=Math.min(parent,maximum);try{if(this.owned>this.maximum)throw new Error("native decoding ownership exceeds stage limit");return await operation(this);}finally{this.maximum=parent;}}
  /** 📍️ Advance consumed units across bounded cancellation frontiers. */
  async advance(units:number):Promise<void>{if(!Number.isSafeInteger(units)||units<0)throw new Error("Invalid native decoding work");if(!this.started)await this.checkpoint();const previous=this.completed;this.completed+=units;if(!Number.isSafeInteger(this.completed)||this.total!==0&&this.completed>this.total)throw new Error("native decoding exceeded declared stage workload");if(Math.floor(previous/256)!==Math.floor(this.completed/256)||this.total!==0&&this.completed===this.total)await this.checkpoint();}
  /** 📦️ Admit cumulative bytes before a copy or reservation. */
  async charge(bytes:number):Promise<void>{
    const next=this.owned+bytes;
    if(!Number.isSafeInteger(bytes)||bytes<0||!Number.isSafeInteger(next)||next>this.maximumBytes)throw new Error("native decoding ownership exceeds caller limit");
    if(!this.started||bytes>65536)await this.checkpoint();
    const committed=this.owned+bytes;
    if(!Number.isSafeInteger(committed)||committed>this.maximumBytes)throw new Error("native decoding ownership exceeds caller limit");
    this.owned=committed;
  }
  /** 🗂️ Admit explicitly declared typed collection slot width before allocation. */
  async admitSlots(count:number,width:number):Promise<void>{if(!Number.isSafeInteger(count)||count<0||!Number.isSafeInteger(width)||width<0)throw new Error("native decoding collection size overflow");await this.charge(count*width);}
  private async copyCheckpoint(completed:number,total:number):Promise<void>{if(this.signal?.aborted||!this.progress({completed,total,ownedBytes:this.owned}))throw new Error("native decoding canceled");await new Promise<void>(resolve=>setTimeout(resolve,0));if(this.signal?.aborted)throw new Error("native decoding canceled");}
  /** 🔎️ Validate borrowed UTF-8 without constructing a second owned text buffer. */
  async validateUtf8(bytes:Uint8Array):Promise<void>{
    await this.copyCheckpoint(0,bytes.byteLength);let position=0,last=0;
    while(position<bytes.byteLength){
      const first=bytes[position++];let count=0,low=0x80,high=0xbf;
      if(first<=0x7f)count=0;else if(first>=0xc2&&first<=0xdf)count=1;else if(first>=0xe0&&first<=0xef){count=2;if(first===0xe0)low=0xa0;if(first===0xed)high=0x9f;}else if(first>=0xf0&&first<=0xf4){count=3;if(first===0xf0)low=0x90;if(first===0xf4)high=0x8f;}else throw new Error("invalid native UTF-8");
      for(let i=0;i<count;i++){if(position===bytes.byteLength)throw new Error("invalid native UTF-8");const next=bytes[position++];if(next<(i===0?low:0x80)||next>(i===0?high:0xbf))throw new Error("invalid native UTF-8");}
      if(position-last>=65536){await this.copyCheckpoint(position,bytes.byteLength);last=position;}
    }
    if(last!==position)await this.copyCheckpoint(position,bytes.byteLength);
  }
  /** 🧬️ Copy intrinsic octets in cancellable64KiB spans after complete ownership admission. */
  async copyBytes(bytes:Uint8Array):Promise<Uint8Array>{await this.charge(bytes.byteLength);await this.copyCheckpoint(0,bytes.byteLength);const output=new Uint8Array(bytes.byteLength);for(let offset=0;offset<bytes.byteLength;offset+=65536){const end=Math.min(offset+65536,bytes.byteLength);output.set(bytes.subarray(offset,end),offset);await this.copyCheckpoint(end,bytes.byteLength);}return output;}
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🦀️.rs

Bytes9703; SHA-256 92342ea0ca5411eaaa369685bcc8167cf4c8b5cd44b0aeb9d02d7de7c94fdb3e

````text
use semio_framework_os_kernel::native_decoding::*;

#[test]
fn sqlite_snapshot_nested_native_stages_preserve_parent_and_cumulative_ownership(){
    use std::{cell::Cell,io::Write,process::{Command,Stdio}};
    fn label(error:&str)->&str{match error{"native decoding exceeded declared stage workload"=>"overrun","native decoding canceled"=>"canceled","native decoding ownership exceeds caller limit"=>"limit","owned child rejected"=>"rejected",_=>panic!("unexpected decode error: {error}")}}
    fn run(control:&mut NativeDecodeControl<'_>,operations:&serde_json::Value,errors:&mut Vec<String>)->Result<(),String>{
        for op in operations.as_array().unwrap(){match op["kind"].as_str().unwrap(){
            "begin"=>control.begin_stage(op["units"].as_u64().unwrap() as usize)?,
            "advance"=>control.advance(op["units"].as_u64().unwrap() as usize)?,
            "charge"=>control.charge(op["units"].as_u64().unwrap() as usize)?,
            "reject"=>return Err("owned child rejected".into()),
            "scope"=>{if let Err(error)=control.scoped_stage(|child|run(child,&op["operations"],errors)){errors.push(label(&error).to_owned());}},
            kind=>panic!("unexpected stage operation: {kind}")
        }}Ok(())
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪆️stage/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🪆️stage/🔣️.json")).unwrap();
    let mut actual=Vec::new();
    for case in fixture["cases"].as_array().unwrap(){
        let progress=Cell::new(NativeDecodeProgress{completed:0,total:0,owned_bytes:0});
        let mut callback=|event:NativeDecodeProgress|{progress.set(event);case["cancel"].is_null()||event.total!=case["cancel"]["total"].as_u64().unwrap() as usize||event.completed<(case["cancel"]["at"].as_u64().unwrap() as usize)};
        let mut control=NativeDecodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize,&mut callback);let mut errors=Vec::new();
        if let Err(error)=run(&mut control,&case["operations"],&mut errors){errors.push(label(&error).to_owned());}control.checkpoint().unwrap();
        let event=progress.get();let result=serde_json::json!({"progress":{"completed":event.completed,"total":event.total,"ownedBytes":event.owned_bytes},"errors":errors});
        assert_eq!(result,case["expected"],"{}",case["id"]);actual.push(result);
    }
    let script=format!("{}\nimport Ajv from 'ajv/dist/2020.js';const input=JSON.parse(await Bun.stdin.text());const ajv=new Ajv({{strict:true}});if(!ajv.validate(input.schema,input.fixture))throw Error('stage fixture');const admit=ajv.getSchema(input.schema.$id+'#/$defs/case');if(input.fixture.cases.some(c=>!admit(c))||input.fixture.refusals.some(c=>admit(c.value)))throw Error('stage admission');await Bun.write(Bun.stdout,JSON.stringify(input.fixture.cases.map(stageOracle)));",include_str!("🪆️stage/🟦️.ts"));
    let mut child=Command::new("bun").args(["-e",&script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
}

#[test]
fn sqlite_snapshot_native_materialization_preserves_caller_budget_and_cancellation() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let mut actual=Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let mut progress=|_:NativeDecodeProgress|true;
        let mut control=NativeDecodeControl::new(case["maximumBytes"].as_u64().unwrap() as usize,&mut progress);
        let mut accepted=0;
        for bytes in case["charges"].as_array().unwrap() { if control.charge(bytes.as_u64().unwrap() as usize).is_err(){break} accepted+=1; }
        assert_eq!(accepted,case["acceptedCharges"].as_u64().unwrap());assert_eq!(control.owned_bytes(),case["ownedBytes"].as_u64().unwrap() as usize);
        actual.push(serde_json::json!({"acceptedCharges":accepted,"ownedBytes":control.owned_bytes()}));
    }
    let script="import {Database} from 'bun:sqlite';import Ajv from 'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const db=new Database(':memory:');db.run('CREATE TABLE charge(position INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)');const actual=x.fixture.cases.map(c=>{db.run('DELETE FROM charge');let acceptedCharges=0,ownedBytes=0;for(const bytes of c.charges){db.run('INSERT INTO charge VALUES(?,?)',acceptedCharges+1,bytes);const n=db.query('SELECT SUM(bytes) AS n FROM charge').get().n;if(n>c.maximumBytes)break;ownedBytes=n;acceptedCharges++;}return {acceptedCharges,ownedBytes};});db.close();await Bun.write(Bun.stdout,JSON.stringify(actual));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual,serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap());
    let copy_total=fixture["copyWorkload"]["total"].as_u64().unwrap() as usize;let copy_cancel=fixture["copyWorkload"]["cancelAt"].as_u64().unwrap() as usize;
    for text in [false,true]{let mut reached=Vec::new();let mut progress=|event:NativeDecodeProgress|{if event.total!=copy_total{return true;}reached.push(event.completed);event.completed<copy_cancel};let mut control=NativeDecodeControl::new(1_000_000,&mut progress);if text{let input=fixture["copyWorkload"]["text"].as_str().unwrap().repeat(fixture["copyWorkload"]["repetitions"].as_u64().unwrap() as usize);assert_eq!(input.len(),copy_total);assert!(control.copy_text(&input).is_err());}else{assert!(control.copy_bytes(&vec![1;copy_total]).is_err());}assert_eq!(reached,vec![0,copy_cancel]);assert!(copy_cancel<copy_total);}
    let mut canceled=|_:NativeDecodeProgress|false;
    let mut control=NativeDecodeControl::new(1_000_000,&mut canceled);
    assert!(control.copy_bytes(&vec![1;65537]).is_err());assert_eq!(control.owned_bytes(),0);
    assert!(control.copy_bytes(&[1]).is_err());assert_eq!(control.owned_bytes(),0);
    let total=fixture["workload"]["total"].as_u64().unwrap() as usize;let cancel_at=fixture["workload"]["cancelAt"].as_u64().unwrap() as usize;
    let mut reached=Vec::new();let mut progress=|event:NativeDecodeProgress|{assert_eq!(event.total,total);reached.push(event.completed);event.completed<cancel_at};
    let mut control=NativeDecodeControl::new(1_000_000,&mut progress);
    control.begin_stage(total).unwrap();for _ in 0..cancel_at-1 {control.step().unwrap();}assert!(control.step().is_err());assert!(cancel_at<total);assert_eq!(serde_json::json!(reached),fixture["workload"]["checkpoints"]);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(16,&mut accepted);
    assert!(control.allocate_vec::<Vec<u8>>(2048).is_err());assert_eq!(control.owned_bytes(),0);
    control.charge(12).unwrap();control.begin_stage(4).unwrap();assert!(control.charge(8).is_err());assert_eq!(control.owned_bytes(),12);
    let text="🧬".repeat(copy_total/4);let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(0,&mut accepted);assert_eq!(control.borrow_text(text.as_bytes()).unwrap(),text);assert_eq!(control.owned_bytes(),0);for bytes in [&[0xc0,0x80][..],&[0xed,0xa0,0x80][..],&[0xf4,0x90,0x80,0x80][..],&[0xf0,0x9f][..]]{assert!(control.borrow_text(bytes).is_err());}
    let mut interior=false;let mut cancel=|event:NativeDecodeProgress|{if event.completed==copy_cancel&&event.total==copy_total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(0,&mut cancel);assert!(control.borrow_text(text.as_bytes()).is_err());assert!(interior);
    let n=&fixture["nestedStage"];let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(n["parentTotal"].as_u64().unwrap() as usize).unwrap();control.advance(n["parentBefore"].as_u64().unwrap() as usize).unwrap();control.scoped_stage(|child|->Result<(),String>{child.begin_stage(n["childTotal"].as_u64().unwrap() as usize)?;child.charge(n["childBytes"].as_u64().unwrap() as usize)?;child.advance(n["childTotal"].as_u64().unwrap() as usize)}).unwrap();control.advance(n["parentAfter"].as_u64().unwrap() as usize).unwrap();assert_eq!(control.owned_bytes(),n["childBytes"].as_u64().unwrap() as usize);assert!(control.scoped_stage(|child|->Result<(),String>{child.begin_stage(2)?;Err("owned child failed".into())}).is_err());control.advance((n["parentTotal"].as_u64().unwrap()-n["parentBefore"].as_u64().unwrap()-n["parentAfter"].as_u64().unwrap()) as usize).unwrap();assert!(control.step().is_err());
    let mut control=NativeDecodeControl::new(0,&mut accepted);for case in fixture["utf8"].as_array().unwrap(){let bytes:Vec<u8>=serde_json::from_value(case["bytes"].clone()).unwrap();assert_eq!(control.borrow_text(&bytes).is_ok(),case["accepted"].as_bool().unwrap());assert_eq!(control.owned_bytes(),0);}
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🟦️.ts

Bytes7767; SHA-256 af804919e25d27447a764efc1e3edb15326e6c5dc21c5acb3945317f7a13cc80

````text
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {NativeDecodeControl} from "../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import stageFixture from "../🧫️fixtures/🪆️stage/🔣️.json";
import stageSchema from "../🧬️schema/🪆️stage/🔣️.json";
import {stageOracle,type StageCase,type StageOperation,type StageResult} from "./🪆️stage/🟦️.ts";

test("nested native stages preserve parent workloads, cancellation and cumulative ownership",async()=>{
  const ajv=new Ajv({strict:true});expect(ajv.validate(stageSchema,stageFixture)).toBe(true);
  const admit=ajv.getSchema(`${stageSchema.$id}#/$defs/case`)!;for(const c of stageFixture.cases)expect(admit(c)).toBe(true);for(const c of stageFixture.refusals)expect(admit(c.value)).toBe(false);
  const classify=(error:unknown)=>{const message=(error as Error).message;const labels=new Map([["native decoding exceeded declared stage workload","overrun"],["native decoding canceled","canceled"],["native decoding ownership exceeds caller limit","limit"],["owned child rejected","rejected"]]);const label=labels.get(message);if(!label)throw error;return label;};
  for(const c of stageFixture.cases as StageCase[]){
    let progress:StageResult["progress"]={completed:0,total:0,ownedBytes:0};const errors:string[]=[];
    const control=new NativeDecodeControl(c.maximumBytes,p=>{progress=p;return !c.cancel||p.total!==c.cancel.total||p.completed<c.cancel.at;});
    const run=async(operations:StageOperation[]):Promise<void>=>{for(const op of operations){
      if(op.kind==="begin")await control.beginStage(op.units);
      else if(op.kind==="advance")await control.advance(op.units);
      else if(op.kind==="charge")await control.charge(op.units);
      else if(op.kind==="reject")throw new Error("owned child rejected");
      else try{await control.scopedStage(()=>run(op.operations));}catch(error){errors.push(classify(error));}
    }};
    try{await run(c.operations);}catch(error){errors.push(classify(error));}await control.checkpoint();
    const actual={progress,errors};expect(actual).toEqual(c.expected);expect(actual).toEqual(stageOracle(c));
  }
});

test("native materialization retains cumulative bounds and interior cancellation",async()=>{
  expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
  const database=new Database(":memory:");database.run("CREATE TABLE charge(position INTEGER PRIMARY KEY,bytes INTEGER NOT NULL)");
  try{for(const c of fixture.cases){
    const control=new NativeDecodeControl(c.maximumBytes,()=>true);let acceptedCharges=0;
    for(const bytes of c.charges){try{await control.charge(bytes);}catch{break;}acceptedCharges++;}
    database.run("DELETE FROM charge");let independentAccepted=0,independentBytes=0;
    for(const bytes of c.charges){database.run("INSERT INTO charge VALUES(?,?)",independentAccepted+1,bytes);const total=(database.query("SELECT SUM(bytes) AS n FROM charge").get() as {n:number}).n;if(total>c.maximumBytes)break;independentAccepted++;independentBytes=total;}
    expect([acceptedCharges,control.ownedBytes]).toEqual([independentAccepted,independentBytes]);expect([acceptedCharges,control.ownedBytes]).toEqual([c.acceptedCharges,c.ownedBytes]);
  }}finally{database.close();}
  const scopeCase=fixture.cases.find(c=>c.id==="cumulative")!,scoped=new NativeDecodeControl(fixture.workload.total,()=>true);await expect(scoped.scopedMaximum(scopeCase.maximumBytes,async control=>{for(const bytes of scopeCase.charges)await control.charge(bytes);})).rejects.toThrow("caller limit");expect(scoped.ownedBytes).toBe(scopeCase.ownedBytes);expect(scoped.maximumBytes).toBe(fixture.workload.total);await scoped.charge(scopeCase.charges[1]);expect(scoped.ownedBytes).toBe(scopeCase.ownedBytes+scopeCase.charges[1]);await expect(scoped.scopedMaximum(0,async()=>{throw new Error("operation must not run");})).rejects.toThrow("stage limit");expect(scoped.maximumBytes).toBe(fixture.workload.total);
  const copyEvents:number[]=[];const copyCanceled=new NativeDecodeControl(1_000_000,p=>{if(p.total!==fixture.copyWorkload.total)return true;copyEvents.push(p.completed);return p.completed<fixture.copyWorkload.cancelAt;});await expect(copyCanceled.copyBytes(new Uint8Array(fixture.copyWorkload.total))).rejects.toThrow("canceled");expect(copyEvents).toEqual([0,fixture.copyWorkload.cancelAt]);expect(fixture.copyWorkload.cancelAt).toBeLessThan(fixture.copyWorkload.total);
  const denied=new NativeDecodeControl(1_000_000,()=>false);await expect(denied.copyBytes(new Uint8Array(65537))).rejects.toThrow("canceled");expect(denied.ownedBytes).toBe(0);
  const early=new NativeDecodeControl(16,()=>false);await expect(early.copyBytes(new Uint8Array(1))).rejects.toThrow("canceled");expect(early.ownedBytes).toBe(0);
  const observed:number[]=[];const canceled=new NativeDecodeControl(1_000_000,p=>{expect(p.total).toBe(fixture.workload.total);observed.push(p.completed);return p.completed<fixture.workload.cancelAt;});await canceled.beginStage(fixture.workload.total);for(let i=0;i<fixture.workload.cancelAt-1;i++)await canceled.step();await expect(canceled.step()).rejects.toThrow("canceled");expect(observed).toEqual(fixture.workload.checkpoints);expect(fixture.workload.cancelAt).toBeLessThan(fixture.workload.total);
  const limited=new NativeDecodeControl(16,()=>true);await expect(limited.admitSlots(2048,24)).rejects.toThrow("caller limit");expect(limited.ownedBytes).toBe(0);
  await limited.charge(12);await limited.beginStage(4);await expect(limited.charge(8)).rejects.toThrow("caller limit");expect(limited.ownedBytes).toBe(12);
  const signal=new AbortController(),interrupted=new NativeDecodeControl(1_000_000,()=>true,signal.signal);const pending=interrupted.copyBytes(new Uint8Array(65537));signal.abort();await expect(pending).rejects.toThrow("canceled");expect(interrupted.ownedBytes).toBe(0);
  const shared=new NativeDecodeControl(100000,()=>true);const simultaneous=await Promise.allSettled([shared.copyBytes(new Uint8Array(65537)),shared.copyBytes(new Uint8Array(65537))]);expect(simultaneous.filter(result=>result.status==="fulfilled")).toHaveLength(1);expect(shared.ownedBytes).toBe(65537);
  const parent=new NativeDecodeControl(64,()=>true),n=fixture.nestedStage;await parent.beginStage(n.parentTotal);await parent.advance(n.parentBefore);await parent.scopedStage(async child=>{await child.beginStage(n.childTotal);await child.charge(n.childBytes);await child.advance(n.childTotal);});await parent.advance(n.parentAfter);expect(parent.ownedBytes).toBe(n.childBytes);await expect(parent.scopedStage(async child=>{await child.beginStage(n.childTotal);throw new Error("owned child failed");})).rejects.toThrow("owned child failed");await parent.advance(n.parentTotal-n.parentBefore-n.parentAfter);await expect(parent.step()).rejects.toThrow("declared stage workload");
  for(const c of fixture.utf8){const control=new NativeDecodeControl(0,()=>true),bytes=new Uint8Array(c.bytes);let independent=true,actual=true;try{new TextDecoder("utf-8",{fatal:true}).decode(bytes);}catch{independent=false;}try{await control.validateUtf8(bytes);}catch{actual=false;}expect(actual).toBe(independent);expect(actual).toBe(c.accepted);expect(control.ownedBytes).toBe(0);}
  let utf8Interior=false;const utf8Canceled=new NativeDecodeControl(0,p=>{if(p.completed===fixture.copyWorkload.cancelAt&&p.total===fixture.copyWorkload.total){utf8Interior=true;return false;}return true;});await expect(utf8Canceled.validateUtf8(new TextEncoder().encode(fixture.copyWorkload.text.repeat(fixture.copyWorkload.repetitions)))).rejects.toThrow("canceled");expect(utf8Interior).toBe(true);
});

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🪆️stage/🟦️.ts

Bytes2073; SHA-256 dee56718bb6e9b676f01e7435fd298848430dde29c1becbdabd7fbad92645686

````text
import {Database} from "bun:sqlite";

export type StageOperation={kind:"begin";units:number}|{kind:"advance";units:number}|{kind:"charge";units:number}|{kind:"scope";operations:StageOperation[]}|{kind:"reject"};
export interface StageResult {progress:{completed:number;total:number;ownedBytes:number};errors:string[]}
export interface StageCase {id:string;maximumBytes:number;cancel:null|{total:number;at:number};operations:StageOperation[];expected:StageResult}

/** 🪆️ Independent workload stack and SQLite cumulative ownership oracle. */
export function stageOracle(c:StageCase):StageResult {
  const db=new Database(":memory:");db.run("CREATE TABLE charges(bytes INTEGER NOT NULL)");
  let completed=0,total=0,started=false,stage=0;const errors:string[]=[];
  const owned=()=>Number((db.query("SELECT COALESCE(SUM(bytes),0) AS n FROM charges").get() as {n:number}).n);
  const checkpoint=()=>{if(c.cancel&&total===c.cancel.total&&completed>=c.cancel.at)throw new Error("canceled");started=true;};
  const run=(operations:StageOperation[])=>{for(const op of operations){
    if(op.kind==="begin"){stage++;completed=0;total=op.units;started=false;checkpoint();}
    else if(op.kind==="advance"){if(!started)checkpoint();const previous=completed;completed+=op.units;if(total!==0&&completed>total)throw new Error("overrun");if(Math.floor(previous/256)!==Math.floor(completed/256)||total!==0&&completed===total)checkpoint();}
    else if(op.kind==="charge"){if(owned()+op.units>c.maximumBytes)throw new Error("limit");if(!started||op.units>65536)checkpoint();db.run("INSERT INTO charges VALUES(?)",op.units);}
    else if(op.kind==="reject")throw new Error("rejected");
    else {const parent=[completed,total,started,stage] as const;try{run(op.operations);}catch(error){errors.push((error as Error).message);}finally{if(stage!==parent[3]){[completed,total,started,stage]=parent;}}}
  }};
  try{try{run(c.operations);}catch(error){errors.push((error as Error).message);}checkpoint();return {progress:{completed,total,ownedBytes:owned()},errors};}finally{db.close();}
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🪆️binding/🦀️.rs

Bytes26754; SHA-256 017c1cf9b082c4350185acfc32d8d233db23a2ae319d52119124d6ee401b2364

````text
use semio_framework_os_kernel::{native_decoding::*,DslField,FieldValue,RecordValue};

#[derive(Debug,PartialEq,dsl::DslRecord)]
struct BoundFields { title:String, chunks:Vec<Vec<u8>>, labels:Vec<String> }

#[test]
fn sqlite_snapshot_native_binding_controls_typed_field_materialization() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🪆️binding/🔣️.json")).unwrap();
    let title=fixture["title"].as_str().unwrap().to_string();
    let chunks:Vec<Vec<u8>>=serde_json::from_value(fixture["chunks"].clone()).unwrap();
    let labels:Vec<String>=serde_json::from_value(fixture["labels"].clone()).unwrap();
    let expected=BoundFields{title,chunks,labels};let record=expected.__dsl_to_record();
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(fixture["maximumBytes"].as_u64().unwrap() as usize,&mut accepted);
    let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>actual.title.len());
    let script="import {Database} from 'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());import Ajv from 'ajv/dist/2020.js';if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const f=x.fixture;const d=new Database(':memory:');d.run('CREATE TABLE chunk(id INTEGER PRIMARY KEY,bytes BLOB NOT NULL)');f.chunks.forEach((b,i)=>d.run('INSERT INTO chunk VALUES(?,?)',i+1,new Uint8Array(b)));const rows=d.query('SELECT bytes FROM chunk ORDER BY id').all();await Bun.write(Bun.stdout,JSON.stringify(rows.map(r=>Array.from(r.bytes))));d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(actual.chunks,serde_json::from_slice::<Vec<Vec<u8>>>(&output.stdout).unwrap());
    let mut tiny=NativeDecodeControl::new(1,&mut accepted);assert!(BoundFields::__dsl_from_record_controlled(&record,&mut tiny).is_err());assert_eq!(tiny.owned_bytes(),0);
    let count=fixture["count"].as_u64().unwrap() as usize;let cancel_at=fixture["cancelAfter"].as_u64().unwrap() as usize;
    let many=BoundFields{title:String::new(),chunks:vec![],labels:(0..count).map(|_|"x".into()).collect()}.__dsl_to_record();
    let mut reached=0;let mut cancel=|event:NativeDecodeProgress|{reached=event.completed;event.completed<cancel_at};let mut control=NativeDecodeControl::new(1_000_000,&mut cancel);control.begin_stage(0).unwrap();assert!(BoundFields::__dsl_from_record_controlled(&many,&mut control).is_err());assert_eq!(reached,cancel_at);assert!(reached<count);
    struct Unbound;impl DslField for Unbound {fn shape()->dsl::Shape{dsl::Shape::Text}fn to_value(&self)->FieldValue{FieldValue::Text(String::new())}fn from_value(_:&FieldValue)->Result<Self,String>{panic!("ordinary custom constructor must not run")}}
    let mut control=NativeDecodeControl::new(1_000,&mut accepted);assert!(Unbound::from_value_controlled(&FieldValue::Text("owner".into()),&mut control).is_err());
    let empty=RecordValue::default();assert!(BoundFields::__dsl_from_record_controlled(&empty,&mut control).is_err());
}

#[derive(Debug,PartialEq,dsl::DslEnum)]
enum BoundVariant { Label { text:String }, Empty }
#[derive(Debug,PartialEq,dsl::DslScalar)]
enum BoundOrdinal { First, Second }

thread_local! { static FIELD_RETIREMENTS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)}; }
struct RetainedField { retired:bool }
impl Drop for RetainedField { fn drop(&mut self){if !self.retired{FIELD_RETIREMENTS.with(|count|count.set(count.get()+1000));}} }
impl DslField for RetainedField {
    fn shape()->dsl::Shape{dsl::Shape::Text}
    fn to_value(&self)->FieldValue{FieldValue::Text("owned".into())}
    fn from_value(_:&FieldValue)->Result<Self,String>{Err("ordinary retained binding disabled".into())}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,String>{control.step()?;match value{FieldValue::Text(_)=>Ok(Self{retired:false}),_=>Err("expected retained Text".into())}}
    fn retire_decoded(mut self){self.retired=true;FIELD_RETIREMENTS.with(|count|count.set(count.get()+1));}
}
struct StagedField;
impl DslField for StagedField {
    fn shape()->dsl::Shape{dsl::Shape::Text}
    fn to_value(&self)->FieldValue{FieldValue::Text("stage".into())}
    fn from_value(_:&FieldValue)->Result<Self,String>{Err("ordinary staged binding disabled".into())}
    fn from_value_controlled(_:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,String>{control.begin_stage(2)?;control.charge(8)?;control.step()?;control.step()?;Ok(Self)}
}
#[derive(dsl::DslRecord)]
struct StagedRecord { child:StagedField, later:bool }

#[test]
fn sqlite_snapshot_native_binding_restores_declared_parent_workload() {
    let expected=StagedRecord{child:StagedField,later:true};let record=expected.__dsl_to_record();
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(4).unwrap();control.step().unwrap();let actual=StagedRecord::__dsl_from_record_controlled(&record,&mut control).unwrap();assert!(actual.later);control.step().unwrap();control.step().unwrap();assert!(control.step().is_err());assert_eq!(control.owned_bytes(),8);
    let mut incomplete=record.clone();incomplete.fields.remove(&StagedRecord::__dsl_spec().fields[1].id);let mut control=NativeDecodeControl::new(64,&mut accepted);control.begin_stage(4).unwrap();control.step().unwrap();let error=StagedRecord::__dsl_from_record_controlled(&incomplete,&mut control).err().unwrap();assert!(error.message.contains("missing field"),"{}",error.message);control.advance(3).unwrap();assert_eq!(control.owned_bytes(),8);
}
#[derive(dsl::DslRecord)]
struct RetainedRecord { fields:Vec<RetainedField>, later:bool }

#[test]
fn sqlite_snapshot_native_binding_retires_partial_owned_fields() {
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🪆️binding/🔣️.json")).unwrap();
    let script="import{Database}from'bun:sqlite';import Ajv from'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const f=x.fixture;const d=new Database(':memory:');d.run('CREATE TABLE work(id INTEGER PRIMARY KEY,completed INTEGER NOT NULL)');for(let i=1;i<=f.count;i++)d.run('INSERT INTO work VALUES(?,?)',i,i+f.retirement.initialWork);await Bun.write(Bun.stdout,JSON.stringify(d.query('SELECT COUNT(*) AS count FROM work WHERE completed <= ?').get(f.cancelAfter)));d.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let independent:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(independent["count"],fixture["retirement"]["canceledFields"]);
    let count=fixture["retirement"]["completedFields"].as_u64().unwrap() as usize;
    let mut accepted=|_:NativeDecodeProgress|true;
    let spec=RetainedRecord::__dsl_spec();let fields_id=spec.fields[0].id;let later_id=spec.fields[1].id;
    let mut record=RecordValue::default();record.fields.insert(fields_id,FieldValue::List(vec![FieldValue::Text("owned".into());count]));
    FIELD_RETIREMENTS.with(|value|value.set(0));let mut control=NativeDecodeControl::new(65536,&mut accepted);assert!(RetainedRecord::__dsl_from_record_controlled(&record,&mut control).is_err());assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),count);
    record.fields.insert(later_id,FieldValue::Bool(true));FIELD_RETIREMENTS.with(|value|value.set(0));let mut control=NativeDecodeControl::new(65536,&mut accepted);let actual=RetainedRecord::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),0);DslField::retire_decoded(actual);assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),count);
    record.fields.insert(fields_id,FieldValue::List(vec![FieldValue::Text("owned".into()),FieldValue::Bool(false)]));FIELD_RETIREMENTS.with(|value|value.set(0));let mut control=NativeDecodeControl::new(65536,&mut accepted);assert!(RetainedRecord::__dsl_from_record_controlled(&record,&mut control).is_err());assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),1);
    record.fields.insert(fields_id,FieldValue::List(vec![FieldValue::Text("owned".into());fixture["count"].as_u64().unwrap() as usize]));FIELD_RETIREMENTS.with(|value|value.set(0));let boundary=fixture["cancelAfter"].as_u64().unwrap() as usize;let mut observed=None;let mut canceled=|event:NativeDecodeProgress|{if event.completed>=boundary{observed=Some(event);false}else{true}};let mut control=NativeDecodeControl::new(65536,&mut canceled);assert!(RetainedRecord::__dsl_from_record_controlled(&record,&mut control).is_err());drop(control);let observed=observed.unwrap();assert_eq!(observed.completed,boundary);assert_eq!(observed.total,fixture["count"].as_u64().unwrap() as usize);assert_eq!(FIELD_RETIREMENTS.with(|value|value.get()),observed.completed);assert_eq!(observed.completed,fixture["retirement"]["canceledFields"].as_u64().unwrap() as usize);
}

#[test]
fn sqlite_snapshot_native_binding_controls_declared_variants() {
    use dsl::DslVariants;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let mut progress=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(65536,&mut progress);
    for row in fixture["variants"].as_array().unwrap(){let expected=if row["keyword"]=="label"{BoundVariant::Label{text:row["text"].as_str().unwrap().into()}}else{BoundVariant::Empty};let (keyword,record)=expected.to_named_record();assert_eq!(BoundVariant::from_named_record_controlled(&keyword,&record,&mut control).unwrap(),expected);}
    assert_eq!(BoundOrdinal::from_value_controlled(&FieldValue::Enum(1),&mut control).unwrap(),BoundOrdinal::Second);
    assert!(BoundOrdinal::from_value_controlled(&FieldValue::Enum(2),&mut control).is_err());
    let mut canceled=|_:NativeDecodeProgress|false;let mut control=NativeDecodeControl::new(65536,&mut canceled);assert!(BoundVariant::from_named_record_controlled("empty",&RecordValue::default(),&mut control).is_err());assert!(BoundOrdinal::from_value_controlled(&FieldValue::Enum(0),&mut control).is_err());
}

#[test]
fn sqlite_snapshot_native_binding_controlled_pack_keeps_one_cumulative_budget() {
    use dsl::{PackDecodeOptions,PackEncodeOptions,PackLimits,pack_rt};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let expected=BoundFields{title:fixture["title"].as_str().unwrap().into(),chunks:serde_json::from_value(fixture["chunks"].clone()).unwrap(),labels:serde_json::from_value(fixture["labels"].clone()).unwrap()};
    let bytes=pack_rt::encode_record_body(&BoundFields::__dsl_spec(),&expected.__dsl_to_record(),&PackEncodeOptions::default()).unwrap();
    let mut progress=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(65536,&mut progress);
    let record=pack_rt::decode_record_body_exact_controlled(&bytes,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).unwrap();let parsed=control.owned_bytes();
    let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>parsed);
    let count=fixture["count"].as_u64().unwrap() as usize;let cutoff=fixture["cancelAfter"].as_u64().unwrap() as usize;let many=BoundFields{title:String::new(),chunks:vec![],labels:vec!["x".into();count]};let bytes=pack_rt::encode_record_body(&BoundFields::__dsl_spec(),&many.__dsl_to_record(),&PackEncodeOptions::default()).unwrap();
    let mut reached=0;let mut canceled=|event:NativeDecodeProgress|{reached=event.completed;event.completed<cutoff};let mut control=NativeDecodeControl::new(1_000_000,&mut canceled);assert!(pack_rt::decode_record_body_exact_controlled(&bytes,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).is_err());assert_eq!(reached,cutoff);assert!(reached<count);
    let mut control=NativeDecodeControl::new(16384,&mut progress);let options=PackDecodeOptions{limits:PackLimits{max_total_alloc:16384,..Default::default()},..Default::default()};assert!(pack_rt::decode_record_body_exact_controlled(&bytes,&BoundFields::__dsl_spec(),&options,&mut control).is_err());assert!(control.owned_bytes()<=16384);
}

#[test]
fn sqlite_snapshot_native_binding_controlled_document_cancels_during_inflation() {
    use dsl::{PackDecodeOptions,PackEncodeOptions,pack_rt};
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let expected=BoundFields{title:fixture["title"].as_str().unwrap().repeat(fixture["count"].as_u64().unwrap() as usize*16),chunks:serde_json::from_value(fixture["chunks"].clone()).unwrap(),labels:serde_json::from_value(fixture["labels"].clone()).unwrap()};
    let encoded=pack_rt::encode_document(&BoundFields::__dsl_spec(),&expected.__dsl_to_record(),&PackEncodeOptions::default()).unwrap();
    let script="import{inflateRawSync}from'node:zlib';const x=JSON.parse(await Bun.stdin.text());const b=Buffer.from(x.bytes);let p=32;function v(){let n=0,s=0;for(;;){const x=b[p++];n+=(x&127)*2**s;if(!(x&128))return n;s+=7;if(s>63)throw Error('varint')}}let found=false;while(p<b.length-84){const kind=b[p++],flags=b[p++],stored=v(),raw=flags&1?v():stored;const payload=b.subarray(p,p+stored);p+=stored+4;const decoded=flags&1?inflateRawSync(payload):payload;if(decoded.length!==raw)throw Error('length');if(decoded.includes(Buffer.from(x.title)))found=true;}await Bun.write(Bun.stdout,JSON.stringify(found));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"bytes":encoded,"title":expected.title}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(serde_json::from_slice::<bool>(&output.stdout).unwrap(),true);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(16_000_000,&mut accepted);let (record,_)=pack_rt::decode_document_controlled(&encoded,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).unwrap();let parsed=control.owned_bytes();let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>parsed);
    let mut interior=false;let mut cancel=|event:NativeDecodeProgress|{if event.completed>=256&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(16_000_000,&mut cancel);assert!(pack_rt::decode_document_controlled(&encoded,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).is_err());assert!(interior);
    let mut control=NativeDecodeControl::new(64,&mut accepted);assert!(pack_rt::decode_document_controlled(&encoded,&BoundFields::__dsl_spec(),&PackDecodeOptions::default(),&mut control).is_err());assert!(control.owned_bytes()<=64);
}

#[test]
fn sqlite_snapshot_native_binding_controls_artifact_child_identity() {
    use semio_framework_os_kernel::{ArtifactChild,io_schema::ArtifactRef};
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔗️child/🔣️.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧬️schema/🔗️child/🔣️.json")).unwrap();
    let strings=["childId","artifactId","artifactKind","standard","subset"].map(|key|fixture[key].as_str().unwrap());let total=strings.iter().map(|text|text.len()).sum::<usize>();let expected=ArtifactRef{artifact_id:strings[1].into(),dialect:semio_framework_os_kernel::io_schema::ArtifactDialect{artifact_kind:strings[2].into(),standard:strings[3].into(),subset:strings[4].into()}};
    let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(strings[0].into()));record.fields.insert(1,<ArtifactRef as dsl::DslField>::to_value(&expected));let value=FieldValue::Record(record);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(total,&mut accepted);let actual=ArtifactChild::<()>::from_value_controlled(&value,&mut control).unwrap();assert_eq!(actual.child_id,strings[0]);assert_eq!(actual.target,expected);assert_eq!(control.owned_bytes(),total);
    let mut control=NativeDecodeControl::new(total-1,&mut accepted);assert!(ArtifactChild::<()>::from_value_controlled(&value,&mut control).is_err());assert!(control.owned_bytes()<=total-1);
    let repeated=strings[1].repeat(fixture["repeatCount"].as_u64().unwrap() as usize);let long_target=ArtifactRef{artifact_id:repeated,dialect:expected.dialect.clone()};let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(strings[0].into()));record.fields.insert(1,<ArtifactRef as dsl::DslField>::to_value(&long_target));let mut interior=false;let cutoff=fixture["cancelAt"].as_u64().unwrap() as usize;let mut cancel=|event:NativeDecodeProgress|{if event.completed>=cutoff&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(1_000_000,&mut cancel);assert!(ArtifactChild::<()>::from_value_controlled(&FieldValue::Record(record),&mut control).is_err());assert!(interior);
    let script="import{Database}from'bun:sqlite';import Ajv from'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const d=new Database(':memory:');d.run('CREATE TABLE identity(child TEXT,id TEXT,kind TEXT,standard TEXT,subset TEXT)');const f=x.fixture;d.run('INSERT INTO identity VALUES(?,?,?,?,?)',f.childId,f.artifactId,f.artifactKind,f.standard,f.subset);await Bun.write(Bun.stdout,JSON.stringify(d.query('SELECT child,id,kind,standard,subset FROM identity').get()));d.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture,"schema":schema}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let row:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(row,serde_json::json!({"child":actual.child_id,"id":actual.target.artifact_id,"kind":actual.target.dialect.artifact_kind,"standard":actual.target.dialect.standard,"subset":actual.target.dialect.subset}));
}

#[test]
fn sqlite_snapshot_native_binding_controlled_text_preserves_owned_literals() {
    use dsl::schema::{parse_exact_controlled,ParseOptions,JoinMode};
    use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();
    let expected=BoundFields{title:fixture["title"].as_str().unwrap().repeat(16384),chunks:serde_json::from_value(fixture["chunks"].clone()).unwrap(),labels:serde_json::from_value(fixture["labels"].clone()).unwrap()};let spec=BoundFields::__dsl_spec();let text=dsl::schema::print(&expected.__dsl_to_record(),&spec,JoinMode::Document);
    let script=r#"const x=JSON.parse(await Bun.stdin.text());const m=x.text.match(/title=("(?:[^"\\]|\\.)*")/);if(!m)throw Error('title');const literal=m[1].replace(/\\u\{([0-9a-fA-F]+)\}/g,(_,n)=>{const v=parseInt(n,16);return v<65536?'\\u'+v.toString(16).padStart(4,'0'):String.fromCodePoint(v)});await Bun.write(Bun.stdout,JSON.stringify(JSON.parse(literal)===x.title));"#;
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"text":text,"title":expected.title}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));assert_eq!(serde_json::from_slice::<bool>(&output.stdout).unwrap(),true);
    let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(16_000_000,&mut accepted);let record=parse_exact_controlled(&text,&spec,&ParseOptions::default(),&mut control).unwrap();let parsed=control.owned_bytes();let actual=BoundFields::__dsl_from_record_controlled(&record,&mut control).unwrap();assert_eq!(actual,expected);assert!(control.owned_bytes()>parsed);
    let mut interior=false;let mut canceled=|event:NativeDecodeProgress|{if event.completed>=256&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(16_000_000,&mut canceled);assert!(parse_exact_controlled(&text,&spec,&ParseOptions::default(),&mut control).is_err());assert!(interior);
    let mut control=NativeDecodeControl::new(64,&mut accepted);assert!(parse_exact_controlled(&text,&spec,&ParseOptions::default(),&mut control).is_err());assert!(control.owned_bytes()<=64);
    let mut control=NativeDecodeControl::new(16_000_000,&mut accepted);assert!(parse_exact_controlled(&(text+" foreign=1"),&spec,&ParseOptions::default(),&mut control).is_err());
}

#[derive(Debug,PartialEq,dsl::DslRecord)]
struct ChunkField { #[dsl(base64)] bytes:Vec<u8> }
#[derive(Debug,PartialEq,dsl::DslRecord)]
struct ChunkFields { chunks:Vec<ChunkField> }

#[test]
fn sqlite_snapshot_native_binding_controls_chunk_reconstruction(){
    use dsl::{PackDecodeOptions,PackEncodeOptions,pack_rt};use std::{io::Write,process::{Command,Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap();let expected=ChunkFields{chunks:serde_json::from_value::<Vec<Vec<u8>>>(fixture["chunks"].clone()).unwrap().into_iter().map(|bytes|ChunkField{bytes}).collect()};let mut options=PackEncodeOptions::default();options.chunk_threshold=1;options.chunk_size=2;let spec=ChunkFields::__dsl_spec();let bytes=pack_rt::encode_document(&spec,&expected.__dsl_to_record(),&options).unwrap();let mut accepted=|_:NativeDecodeProgress|true;let mut control=NativeDecodeControl::new(1_000_000,&mut accepted);let mut verified=PackDecodeOptions::default();verified.verification=semio_framework_os_kernel::VerificationLevel::Full;let(record,_)=pack_rt::decode_document_controlled(&bytes,&spec,&verified,&mut control).unwrap();assert_eq!(ChunkFields::__dsl_from_record_controlled(&record,&mut control).unwrap(),expected);
    let repeated=expected.chunks[0].bytes.repeat(fixture["count"].as_u64().unwrap() as usize*64);let length=repeated.len();let large=ChunkFields{chunks:vec![ChunkField{bytes:repeated}]};let mut identity=options.clone();identity.codec=semio_framework_os_kernel::CodecId(0);identity.chunk_size=length as u64;let large_bytes=pack_rt::encode_document(&spec,&large.__dsl_to_record(),&identity).unwrap();
    for verification in [semio_framework_os_kernel::VerificationLevel::Trusted,semio_framework_os_kernel::VerificationLevel::Standard]{let mut interior=false;let mut canceled=|event:NativeDecodeProgress|{if event.total==length&&event.completed>=4096&&event.completed<event.total{interior=true;false}else{true}};let mut control=NativeDecodeControl::new(1_000_000,&mut canceled);let mut decode=PackDecodeOptions::default();decode.verification=verification;assert!(pack_rt::decode_document_controlled(&large_bytes,&spec,&decode,&mut control).is_err());assert!(interior);}
    let mut large_work=false;let mut observed=|event:NativeDecodeProgress|{large_work|=event.total==length;true};let mut control=NativeDecodeControl::new(64_000,&mut observed);assert!(pack_rt::decode_document_controlled(&large_bytes,&spec,&verified,&mut control).is_err());assert!(control.owned_bytes()<=64_000);drop(control);assert!(!large_work);
    let file=semio_framework_os_kernel::os_io::resolve_ready(semio_framework_os_kernel::os_pack::format::PackFile::open_manifest(bytes.as_slice(),&verified.limits,verified.verification)).unwrap();let range=file.chunk_range(semio_framework_os_kernel::ChunkId(0)).unwrap();let mut corrupted=bytes.clone();corrupted[range.offset as usize]^=1;let mut control=NativeDecodeControl::new(1_000_000,&mut accepted);assert!(pack_rt::decode_document_controlled(&corrupted,&spec,&verified,&mut control).is_err());
    let script="import{inflateRawSync}from'node:zlib';const x=JSON.parse(await Bun.stdin.text()),b=Buffer.from(x.bytes);let p=32;function v(){let n=0,s=0;for(;;){const x=b[p++];n+=(x&127)*2**s;if(!(x&128))return n;s+=7;if(s>63)throw Error('varint')}}const chunks=[];while(p<b.length-84){const kind=b[p++],flags=b[p++],stored=v(),raw=flags&1?v():stored,payload=b.subarray(p,p+stored);p+=stored+4;const decoded=flags&1?inflateRawSync(payload):payload;if(decoded.length!==raw)throw Error('length');if(kind===5)chunks.push(Array.from(decoded));}await Bun.write(Bun.stdout,JSON.stringify(chunks));";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"bytes":bytes}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let chunks:Vec<Vec<u8>>=serde_json::from_slice(&output.stdout).unwrap();let expected_chunks:Vec<Vec<u8>>=expected.chunks.iter().flat_map(|chunk|chunk.bytes.chunks(2).map(|bytes|bytes.to_vec())).collect();assert_eq!(chunks,expected_chunks);
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧫️fixtures/🔣️.json

Bytes986; SHA-256 16b454cd403a1baa5439336dade9a1879253bf0261261e1d926a62edb4e3e20f

````text
{"schema":"semio.native-decoding-materialization/v1","workload":{"total":1024,"cancelAt":512,"checkpoints":[0,256,512]},"cases":[{"id":"exact","maximumBytes":16,"charges":[8,8],"acceptedCharges":2,"ownedBytes":16},{"id":"cumulative","maximumBytes":16,"charges":[12,8],"acceptedCharges":1,"ownedBytes":12},{"id":"before-large-copy","maximumBytes":65536,"charges":[65537],"acceptedCharges":0,"ownedBytes":0},{"id":"zero-length","maximumBytes":0,"charges":[0,0],"acceptedCharges":2,"ownedBytes":0}],"copyWorkload":{"total":131072,"cancelAt":65536,"text":"🧬","repetitions":32768},"nestedStage":{"parentTotal":4,"parentBefore":1,"childTotal":2,"childBytes":8,"parentAfter":2},"utf8":[{"bytes":[],"accepted":true},{"bytes":[65,0],"accepted":true},{"bytes":[240,159,167,172],"accepted":true},{"bytes":[194,162],"accepted":true},{"bytes":[192,128],"accepted":false},{"bytes":[237,160,128],"accepted":false},{"bytes":[244,144,128,128],"accepted":false},{"bytes":[240,159],"accepted":false}]}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧬️schema/🔣️.json

Bytes1822; SHA-256 fc7895e6ba4d113b33a945394c8b0646e682f58bf6fb7cd91bcbedcd7f78b0bc

````text
{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","additionalProperties":false,"required":["schema","cases","workload","copyWorkload","nestedStage","utf8"],"properties":{"schema":{"const":"semio.native-decoding-materialization/v1"},"workload":{"type":"object","additionalProperties":false,"required":["total","cancelAt","checkpoints"],"properties":{"total":{"type":"integer","minimum":1},"cancelAt":{"type":"integer","minimum":1},"checkpoints":{"type":"array","items":{"type":"integer","minimum":0}}}},"cases":{"type":"array","items":{"type":"object","additionalProperties":false,"required":["id","maximumBytes","charges","acceptedCharges","ownedBytes"],"properties":{"id":{"type":"string"},"maximumBytes":{"type":"integer","minimum":0},"charges":{"type":"array","items":{"type":"integer","minimum":0}},"acceptedCharges":{"type":"integer","minimum":0},"ownedBytes":{"type":"integer","minimum":0}}}},"copyWorkload":{"type":"object","additionalProperties":false,"required":["total","cancelAt","text","repetitions"],"properties":{"total":{"type":"integer","minimum":1},"cancelAt":{"type":"integer","minimum":1},"text":{"type":"string"},"repetitions":{"type":"integer","minimum":1}}},"nestedStage":{"type":"object","additionalProperties":false,"required":["parentTotal","parentBefore","childTotal","childBytes","parentAfter"],"properties":{"parentTotal":{"type":"integer","minimum":0},"parentBefore":{"type":"integer","minimum":0},"childTotal":{"type":"integer","minimum":0},"childBytes":{"type":"integer","minimum":0},"parentAfter":{"type":"integer","minimum":0}}},"utf8":{"type":"array","items":{"type":"object","additionalProperties":false,"required":["bytes","accepted"],"properties":{"bytes":{"type":"array","items":{"type":"integer","minimum":0,"maximum":255}},"accepted":{"type":"boolean"}}}}}}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧫️fixtures/🪆️stage/🔣️.json

Bytes14241; SHA-256 7290b0a8f674bcffd9b3042db07285ee43b1ee0706e254190c2f1f1c9daacd8e

````text
{
  "schema": "semio.native-decoding-stage/v1",
  "cases": [
    {
      "id": "completed-child-restores-parent",
      "maximumBytes": 64,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 3
            },
            {
              "kind": "charge",
              "units": 8
            },
            {
              "kind": "advance",
              "units": 3
            }
          ]
        },
        {
          "kind": "advance",
          "units": 1
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 12
        },
        "errors": []
      }
    },
    {
      "id": "inherited-stage-progress-is-retained",
      "maximumBytes": 64,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "advance",
              "units": 1
            }
          ]
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 4
        },
        "errors": []
      }
    },
    {
      "id": "rejected-child-retains-admitted-bytes",
      "maximumBytes": 64,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 3
            },
            {
              "kind": "charge",
              "units": 8
            },
            {
              "kind": "reject"
            }
          ]
        },
        {
          "kind": "advance",
          "units": 1
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 12
        },
        "errors": [
          "rejected"
        ]
      }
    },
    {
      "id": "child-canceled-before-admission",
      "maximumBytes": 64,
      "cancel": {
        "total": 3,
        "at": 0
      },
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 3
            },
            {
              "kind": "charge",
              "units": 8
            },
            {
              "kind": "advance",
              "units": 3
            }
          ]
        },
        {
          "kind": "advance",
          "units": 1
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 4
        },
        "errors": [
          "canceled"
        ]
      }
    },
    {
      "id": "child-canceled-after-admission",
      "maximumBytes": 64,
      "cancel": {
        "total": 3,
        "at": 3
      },
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 3
            },
            {
              "kind": "charge",
              "units": 8
            },
            {
              "kind": "advance",
              "units": 3
            }
          ]
        },
        {
          "kind": "advance",
          "units": 1
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 12
        },
        "errors": [
          "canceled"
        ]
      }
    },
    {
      "id": "child-overrun-restores-parent",
      "maximumBytes": 64,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 3
            },
            {
              "kind": "advance",
              "units": 4
            }
          ]
        },
        {
          "kind": "advance",
          "units": 1
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 4
        },
        "errors": [
          "overrun"
        ]
      }
    },
    {
      "id": "nested-ownership-cannot-reset-limit",
      "maximumBytes": 16,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 3
            },
            {
              "kind": "charge",
              "units": 13
            }
          ]
        },
        {
          "kind": "advance",
          "units": 1
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 4
        },
        "errors": [
          "limit"
        ]
      }
    },
    {
      "id": "parent-overrun-remains-visible",
      "maximumBytes": 64,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 3
            },
            {
              "kind": "charge",
              "units": 8
            },
            {
              "kind": "advance",
              "units": 3
            }
          ]
        },
        {
          "kind": "advance",
          "units": 2
        }
      ],
      "expected": {
        "progress": {
          "completed": 3,
          "total": 2,
          "ownedBytes": 12
        },
        "errors": [
          "overrun"
        ]
      }
    },
    {
      "id": "grandchild-restores-both-workloads",
      "maximumBytes": 64,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 3
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 2
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 2
            },
            {
              "kind": "advance",
              "units": 1
            },
            {
              "kind": "charge",
              "units": 3
            },
            {
              "kind": "scope",
              "operations": [
                {
                  "kind": "begin",
                  "units": 4
                },
                {
                  "kind": "advance",
                  "units": 4
                },
                {
                  "kind": "charge",
                  "units": 5
                }
              ]
            },
            {
              "kind": "advance",
              "units": 1
            }
          ]
        },
        {
          "kind": "advance",
          "units": 2
        }
      ],
      "expected": {
        "progress": {
          "completed": 3,
          "total": 3,
          "ownedBytes": 10
        },
        "errors": []
      }
    },
    {
      "id": "zero-work-child-restores-parent",
      "maximumBytes": 64,
      "cancel": null,
      "operations": [
        {
          "kind": "begin",
          "units": 2
        },
        {
          "kind": "advance",
          "units": 1
        },
        {
          "kind": "charge",
          "units": 4
        },
        {
          "kind": "scope",
          "operations": [
            {
              "kind": "begin",
              "units": 0
            },
            {
              "kind": "charge",
              "units": 8
            }
          ]
        },
        {
          "kind": "advance",
          "units": 1
        }
      ],
      "expected": {
        "progress": {
          "completed": 2,
          "total": 2,
          "ownedBytes": 12
        },
        "errors": []
      }
    }
  ],
  "refusals": [
    {
      "id": "unknown-stage-operation",
      "value": {
        "id": "completed-child-restores-parent",
        "maximumBytes": 64,
        "cancel": null,
        "operations": [
          {
            "kind": "begin",
            "units": 2
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "charge",
            "units": 4
          },
          {
            "kind": "scope",
            "operations": [
              {
                "kind": "begin",
                "units": 3
              },
              {
                "kind": "charge",
                "units": 8
              },
              {
                "kind": "advance",
                "units": 3
              }
            ]
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "reset",
            "units": 0
          }
        ],
        "expected": {
          "progress": {
            "completed": 2,
            "total": 2,
            "ownedBytes": 12
          },
          "errors": []
        }
      }
    },
    {
      "id": "missing-work-units",
      "value": {
        "id": "completed-child-restores-parent",
        "maximumBytes": 64,
        "cancel": null,
        "operations": [
          {
            "kind": "begin",
            "units": 2
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "charge",
            "units": 4
          },
          {
            "kind": "scope",
            "operations": [
              {
                "kind": "begin",
                "units": 3
              },
              {
                "kind": "charge",
                "units": 8
              },
              {
                "kind": "advance",
                "units": 3
              }
            ]
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "advance"
          }
        ],
        "expected": {
          "progress": {
            "completed": 2,
            "total": 2,
            "ownedBytes": 12
          },
          "errors": []
        }
      }
    },
    {
      "id": "negative-child-work",
      "value": {
        "id": "completed-child-restores-parent",
        "maximumBytes": 64,
        "cancel": null,
        "operations": [
          {
            "kind": "begin",
            "units": 2
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "charge",
            "units": 4
          },
          {
            "kind": "scope",
            "operations": [
              {
                "kind": "begin",
                "units": 3
              },
              {
                "kind": "charge",
                "units": 8
              },
              {
                "kind": "advance",
                "units": 3
              }
            ]
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "scope",
            "operations": [
              {
                "kind": "advance",
                "units": -1
              }
            ]
          }
        ],
        "expected": {
          "progress": {
            "completed": 2,
            "total": 2,
            "ownedBytes": 12
          },
          "errors": []
        }
      }
    },
    {
      "id": "undeclared-allocation-reset",
      "value": {
        "id": "completed-child-restores-parent",
        "maximumBytes": 64,
        "cancel": null,
        "operations": [
          {
            "kind": "begin",
            "units": 2
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "charge",
            "units": 4
          },
          {
            "kind": "scope",
            "operations": [
              {
                "kind": "begin",
                "units": 3
              },
              {
                "kind": "charge",
                "units": 8
              },
              {
                "kind": "advance",
                "units": 3
              }
            ]
          },
          {
            "kind": "advance",
            "units": 1
          },
          {
            "kind": "charge",
            "units": 8,
            "resetOwned": true
          }
        ],
        "expected": {
          "progress": {
            "completed": 2,
            "total": 2,
            "ownedBytes": 12
          },
          "errors": []
        }
      }
    }
  ]
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧬️schema/🪆️stage/🔣️.json

Bytes4705; SHA-256 ef6a2dd9e0d445cc753ee0d9552f261d27a816afb9e355c35472c3f5ac8a4f6a

````text
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "semio.native-decoding-stage/v1",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "schema",
    "cases",
    "refusals"
  ],
  "properties": {
    "schema": {
      "const": "semio.native-decoding-stage/v1"
    },
    "cases": {
      "type": "array",
      "minItems": 1,
      "maxItems": 64,
      "items": {
        "$ref": "#/$defs/case"
      }
    },
    "refusals": {
      "type": "array",
      "minItems": 1,
      "maxItems": 64,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": [
          "id",
          "value"
        ],
        "properties": {
          "id": {
            "type": "string",
            "minLength": 1
          },
          "value": {
            "type": "object"
          }
        }
      }
    }
  },
  "$defs": {
    "operation": {
      "oneOf": [
        {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "kind",
            "units"
          ],
          "properties": {
            "kind": {
              "enum": [
                "begin",
                "advance",
                "charge"
              ]
            },
            "units": {
              "type": "integer",
              "minimum": 0,
              "maximum": 1000000
            }
          }
        },
        {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "kind",
            "operations"
          ],
          "properties": {
            "kind": {
              "const": "scope"
            },
            "operations": {
              "type": "array",
              "maxItems": 64,
              "items": {
                "$ref": "#/$defs/operation"
              }
            }
          }
        },
        {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "kind"
          ],
          "properties": {
            "kind": {
              "const": "reject"
            }
          }
        }
      ]
    },
    "case": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "id",
        "maximumBytes",
        "cancel",
        "operations",
        "expected"
      ],
      "properties": {
        "id": {
          "type": "string",
          "minLength": 1
        },
        "maximumBytes": {
          "type": "integer",
          "minimum": 0,
          "maximum": 1000000
        },
        "cancel": {
          "oneOf": [
            {
              "type": "null"
            },
            {
              "type": "object",
              "additionalProperties": false,
              "required": [
                "total",
                "at"
              ],
              "properties": {
                "total": {
                  "type": "integer",
                  "minimum": 0,
                  "maximum": 1000000
                },
                "at": {
                  "type": "integer",
                  "minimum": 0,
                  "maximum": 1000000
                }
              }
            }
          ]
        },
        "operations": {
          "type": "array",
          "maxItems": 64,
          "items": {
            "$ref": "#/$defs/operation"
          }
        },
        "expected": {
          "type": "object",
          "additionalProperties": false,
          "required": [
            "progress",
            "errors"
          ],
          "properties": {
            "progress": {
              "type": "object",
              "additionalProperties": false,
              "required": [
                "completed",
                "total",
                "ownedBytes"
              ],
              "properties": {
                "completed": {
                  "type": "integer",
                  "minimum": 0,
                  "maximum": 1000000
                },
                "total": {
                  "type": "integer",
                  "minimum": 0,
                  "maximum": 1000000
                },
                "ownedBytes": {
                  "type": "integer",
                  "minimum": 0,
                  "maximum": 1000000
                }
              }
            },
            "errors": {
              "type": "array",
              "maxItems": 64,
              "items": {
                "enum": [
                  "overrun",
                  "canceled",
                  "limit",
                  "rejected"
                ]
              }
            }
          }
        }
      }
    }
  }
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧫️fixtures/🪆️binding/🔣️.json

Bytes325; SHA-256 41180186b36428e72dcd62555da21cd2a6df1fb1aad01f5cad7950f7a45b2aaa

````text
{"title":"Unicode🧬\u0000","chunks":[[0,255,128],[],[9,10]],"labels":["first","second"],"cancelAfter":256,"count":1024,"maximumBytes":65536,"variants":[{"keyword":"label","text":"Unicode🧬\u0000"},{"keyword":"empty","text":null}],"retirement":{"completedFields":2,"partialFields":1,"canceledFields":256,"initialWork":0}}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧬️schema/🪆️binding/🔣️.json

Bytes1072; SHA-256 7d4b818df387689c05d8ff084e04ecda20e0091654337d3c4718789ea1ccbe8a

````text
{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","additionalProperties":false,"required":["title","chunks","labels","cancelAfter","count","maximumBytes","variants","retirement"],"properties":{"title":{"type":"string"},"chunks":{"type":"array","items":{"type":"array","items":{"type":"integer","minimum":0,"maximum":255}}},"labels":{"type":"array","items":{"type":"string"}},"cancelAfter":{"type":"integer","minimum":1},"count":{"type":"integer","minimum":1},"maximumBytes":{"type":"integer","minimum":0},"variants":{"type":"array","items":{"type":"object","additionalProperties":false,"required":["keyword","text"],"properties":{"keyword":{"enum":["label","empty"]},"text":{"type":["string","null"]}}}},"retirement":{"type":"object","additionalProperties":false,"required":["completedFields","partialFields","canceledFields","initialWork"],"properties":{"completedFields":{"type":"integer","minimum":0},"partialFields":{"type":"integer","minimum":0},"canceledFields":{"type":"integer","minimum":0},"initialWork":{"type":"integer","minimum":0}}}}}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧫️fixtures/🔗️child/🔣️.json

Bytes180; SHA-256 a665c90245ccd2de29dbcff3c0c9efd25c5d9bc968497db5976f54c8e5041411

````text
{
  "childId": "child🧬\u0000",
  "artifactId": "node!@/",
  "artifactKind": "s.stdio.semio",
  "standard": "1",
  "subset": "table",
  "repeatCount": 32768,
  "cancelAt": 256
}

````

### 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧬️schema/🔗️child/🔣️.json

Bytes661; SHA-256 24b16a1e19aa8936ed65a540723322fecb058980b1361123df3d6ce018ce1416

````text
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "additionalProperties": false,
  "required": [
    "childId",
    "artifactId",
    "artifactKind",
    "standard",
    "subset",
    "repeatCount",
    "cancelAt"
  ],
  "properties": {
    "childId": {
      "type": "string"
    },
    "artifactId": {
      "type": "string"
    },
    "artifactKind": {
      "type": "string"
    },
    "standard": {
      "type": "string"
    },
    "subset": {
      "type": "string"
    },
    "repeatCount": {
      "type": "integer",
      "minimum": 1
    },
    "cancelAt": {
      "type": "integer",
      "minimum": 1
    }
  }
}

````

### 🧰️framework/🔨️modules/🌱️value/🦀️.rs

Bytes18696; SHA-256 82ab3779de00ed4f965ff156577469f9c8b1363f1efbac2cefa315d9f256067c

````text
//! 🌱️ `DslValue` — the schema-erased dynamic value both sides of a replication link speak.
//!
//! Lives beside the wire contract rather than inside the os DSL because it is what a schema-less
//! payload decodes to: the authority validates it, the optimistic replica applies it, and the
//! pathmap bodies `db` stores are trees of it. The DSL's own record/field/wire types build on it
//! and stay product-side.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

//#region 🗂️OrderedOwnership
#[path = "🗂️ordered/🦀️.rs"]
pub mod ordered;
#[path = "📋️list/🦀️.rs"]
pub mod list;
#[path = "📦️paged/🦀️.rs"]
pub mod paged;
//#endregion 🗂️OrderedOwnership

#[path = "🧬️bytes/🦀️.rs"]
pub mod bytes;

#[path = "🧬️clone/🦀️.rs"]
pub mod bounded_clone;

#[path = "🛬️decode/🦀️.rs"]
pub mod native_decoding;
pub use native_decoding::NativeDecodeControl;

#[path = "🛫️encode/🦀️.rs"]
pub mod native_encoding;
pub use native_encoding::NativeEncodeControl;

//#region 🔁️Codec
#[path = "🔁️codec/🦀️.rs"]
mod codec;
pub use codec::{edit_through_value, ControlledValueHasher, DecodedValue, FromValue, ToValue, ValueEdit, ValueError, ValueShape};
//#endregion 🔁️Codec

//#region 🔖️Number
/// 🔢️ A JSON-equivalent number that keeps the writer's-eye distinction a bare `f64` erases:
/// an integer literal (`UInt`/`Int`) round-trips without a decimal point, `Float` always keeps one
/// (or an exponent). Mirrors [`pack::json::Number`]'s shape exactly so the wire bridge between them
/// (`🎒️pack/🔤️json/🦀️.rs`) is a straight variant-to-variant map, never a widen-then-guess. See
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
/// 🔍️research/📓️dslvalue-integer-fidelity.md`.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    UInt(u64),
    Int(i64),
    Float(f64),
}

impl Number {
    /// 🔎️ Widens to `f64` regardless of variant — lossy for `u64`/`i64` magnitudes beyond 2^53.
    pub fn as_f64(&self) -> f64 {
        match *self {
            Number::UInt(v) => v as f64,
            Number::Int(v) => v as f64,
            Number::Float(v) => v,
        }
    }

    /// 🔎️ Exact `i64`, only for the `Int` variant and `UInt` values that fit.
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Number::Int(v) => Some(v),
            Number::UInt(v) => i64::try_from(v).ok(),
            Number::Float(_) => None,
        }
    }

    /// 🔎️ Exact `u64`, only for the `UInt` variant and non-negative `Int` values.
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Number::UInt(v) => Some(v),
            Number::Int(v) => u64::try_from(v).ok(),
            Number::Float(_) => None,
        }
    }

    /// 🔎️ Whether this literal was written without a decimal point or exponent.
    pub fn is_integer(&self) -> bool {
        !matches!(self, Number::Float(_))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Number::UInt(a), Number::UInt(b)) => a == b,
            (Number::Int(a), Number::Int(b)) => a == b,
            (Number::Float(a), Number::Float(b)) => a == b,
            (Number::UInt(a), Number::Int(b)) | (Number::Int(b), Number::UInt(a)) => i64::try_from(a).is_ok_and(|a| a == b),
            _ => false,
        }
    }
}

impl From<u64> for Number {
    fn from(v: u64) -> Self {
        Number::UInt(v)
    }
}
impl From<i64> for Number {
    fn from(v: i64) -> Self {
        Number::Int(v)
    }
}
impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number::Float(v)
    }
}

/// 🔢️ The integer one finite JSON number reads back as — [`json_integer`]'s answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonInteger {
    Unsigned(u64),
    Signed(i64),
}

/// 🔢️ The integer an `f64` reads back as from its JSON text: `JSON.stringify(7.0)` is `"7"`, so a finite, integral number
/// within the safe integer range (`Number.isSafeInteger`, |v| ≤ 2^53 − 1) is an integer — unsigned unless negative — and
/// anything else (a fraction, a non-finite value, an integral value past the safe range) is not. The ONE rule every Rust
/// boundary that carries a JSON number as `f64` reads it by (`DslValue::json_number`, the UI contract's `UiValue` serializer).
pub fn json_integer(v: f64) -> Option<JsonInteger> {
    const SAFE_INTEGER_MAX: f64 = 9_007_199_254_740_991.0;
    if !v.is_finite() || v.fract() != 0.0 || v.abs() > SAFE_INTEGER_MAX {
        return None;
    }
    Some(if v >= 0.0 { JsonInteger::Unsigned(v as u64) } else { JsonInteger::Signed(v as i64) })
}
//#endregion 🔖️Number

//#region 🔖️Value
/// 🌱️ Dynamic JSON-equivalent literal for schema-less fields (`Shape::Value`).
#[derive(Clone, Debug, PartialEq)]
pub enum DslValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Bytes(Vec<u8>),
    Array(Vec<DslValue>),
    Object(Vec<(String, DslValue)>),
}

/// 🪪️ Immutable framework value selected from one retained domain owner.
pub trait DslValueSource {
    fn value(&self) -> &DslValue;
}

impl DslValueSource for std::sync::Arc<DslValue> {
    fn value(&self) -> &DslValue { self.as_ref() }
}

impl DslValue {
    pub fn null() -> Self {
        Self::Null
    }

    /// 🔢️ Whole-number constructor — the fidelity-preserving choice for ids/counts/indices/ms.
    pub fn uint(v: u64) -> Self {
        Self::Number(Number::UInt(v))
    }

    /// 🔢️ Signed whole-number constructor.
    pub fn int(v: i64) -> Self {
        Self::Number(Number::Int(v))
    }

    /// 🔢️ Fractional constructor — the wire always keeps an explicit `.0` for a whole float so it
    /// never collapses onto its integer twin.
    pub fn float(v: f64) -> Self {
        Self::Number(Number::Float(v))
    }

    /// 🔢️ A JSON number carried as `f64` (the UI contract's `UiValue::Number`), read the way its JSON text reads back
    /// ([`json_integer`]): an integer stays an integer, a non-finite value is `Null` (JSON text has neither NaN nor Infinity),
    /// anything else a float — so an integer argument a renderer hands back decodes as the integer React's wire delivers.
    pub fn json_number(v: f64) -> Self {
        match json_integer(v) {
            Some(JsonInteger::Unsigned(value)) => Self::uint(value),
            Some(JsonInteger::Signed(value)) => Self::int(value),
            None if v.is_finite() => Self::float(v),
            None => Self::Null,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// 🔎️ Widens to `f64` regardless of variant — lossy for `u64`/`i64` magnitudes beyond 2^53. Use
    /// [`DslValue::as_i64`]/[`DslValue::as_u64`] when exactness matters.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(n.as_f64()),
            _ => None,
        }
    }

    /// 🔎️ Exact `i64`, only when the underlying [`Number`] is representable as one.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(n) => n.as_i64(),
            _ => None,
        }
    }

    /// 🔎️ Exact `u64`, only when the underlying [`Number`] is representable as one.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(n) => n.as_u64(),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// 🧬️ Borrows intrinsic octets without projecting one numeric node per byte.
    pub fn as_bytes(&self)->Option<&[u8]>{match self{Self::Bytes(bytes)=>Some(bytes),_=>None}}

    pub fn as_array(&self) -> Option<&[DslValue]> {
        match self {
            Self::Array(items) => Some(items.as_slice()),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&[(String, DslValue)]> {
        match self {
            Self::Object(entries) => Some(entries.as_slice()),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&DslValue> {
        let Self::Object(entries) = self else {
            return None;
        };
        entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn object(entries: impl IntoIterator<Item = (String, DslValue)>) -> Self {
        Self::Object(entries.into_iter().collect())
    }
}

impl std::ops::Index<&str> for DslValue {
    type Output = DslValue;
    fn index(&self, key: &str) -> &Self::Output {
        static NULL: DslValue = DslValue::Null;
        self.get(key).unwrap_or(&NULL)
    }
}

impl std::ops::Index<usize> for DslValue {
    type Output = DslValue;
    fn index(&self, index: usize) -> &Self::Output {
        static NULL: DslValue = DslValue::Null;
        match self {
            DslValue::Array(items) => items.get(index).unwrap_or(&NULL),
            _ => &NULL,
        }
    }
}

impl From<&DslValue> for serde_json::Value {
    fn from(val: &DslValue) -> Self {
        match val {
            DslValue::Null => serde_json::Value::Null,
            DslValue::Bool(b) => serde_json::Value::Bool(*b),
            DslValue::Number(Number::UInt(v)) => serde_json::Value::Number((*v).into()),
            DslValue::Number(Number::Int(v)) => serde_json::Value::Number((*v).into()),
            DslValue::Number(Number::Float(v)) => serde_json::json!(*v),
            DslValue::String(s) => serde_json::Value::String(s.clone()),
            DslValue::Bytes(bytes) => serde_json::Value::Array(bytes.iter().map(|byte|serde_json::Value::Number((*byte).into())).collect()),
            DslValue::Array(arr) => serde_json::Value::Array(arr.iter().map(serde_json::Value::from).collect()),
            DslValue::Object(obj) => {
                let map = obj.iter().map(|(k, v)| (k.clone(), serde_json::Value::from(v))).collect();
                serde_json::Value::Object(map)
            }
        }
    }
}

impl From<DslValue> for serde_json::Value {
    fn from(val: DslValue) -> Self {
        serde_json::Value::from(&val)
    }
}

/// 🌉️ The reverse bridge: a plugin decoding `ArtifactEditor::command_from_action`/
/// `host_configuration_mutation`'s trait-mandated `Option<&serde_json::Value>` args into a
/// `ToValue`/`FromValue` domain type routes through here — preserves whichever of `serde_json`'s
/// own `u64`/`i64`/`f64` storage the number parsed into (same convention `pack::json`'s
/// `to_dsl_value` bridge uses for its sibling `Value` type), never widening an integer to `f64`.
impl From<&serde_json::Value> for DslValue {
    fn from(val: &serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => DslValue::Null,
            serde_json::Value::Bool(b) => DslValue::Bool(*b),
            serde_json::Value::Number(n) => {
                if let Some(v) = n.as_u64().filter(|_| n.is_u64()) {
                    DslValue::Number(Number::UInt(v))
                } else if let Some(v) = n.as_i64().filter(|_| n.is_i64()) {
                    DslValue::Number(Number::Int(v))
                } else {
                    DslValue::Number(Number::Float(n.as_f64().unwrap_or(f64::NAN)))
                }
            }
            serde_json::Value::String(s) => DslValue::String(s.clone()),
            serde_json::Value::Array(items) => DslValue::Array(items.iter().map(DslValue::from).collect()),
            serde_json::Value::Object(obj) => DslValue::object(obj.iter().map(|(k, v)| (k.clone(), DslValue::from(v)))),
        }
    }
}

impl From<serde_json::Value> for DslValue {
    fn from(val: serde_json::Value) -> Self {
        DslValue::from(&val)
    }
}

/// 🌉️ Serializes an owned value through the explicitly exposed Serde boundary.
impl serde::Serialize for DslValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&serde_json::Value::from(self), serializer)
    }
}

/// 🌉️ Deserializes a Serde JSON boundary into an owned value tree.
impl<'de> serde::Deserialize<'de> for DslValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        serde_json::Value::deserialize(deserializer).map(DslValue::from)
    }
}

impl PartialEq<serde_json::Value> for DslValue {
    fn eq(&self, other: &serde_json::Value) -> bool {
        &serde_json::Value::from(self) == other
    }
}

impl PartialEq<DslValue> for serde_json::Value {
    fn eq(&self, other: &DslValue) -> bool {
        self == &serde_json::Value::from(other)
    }
}

//#region 🔖️Literal
/// 🧾️ Builds a [`DslValue`] from a JSON-shaped literal — the first-party replacement for
/// `serde_json::json!` + `DslValue::from` at every action-argument site. Grammar: `null`, `true`,
/// `false`, `[ … ]`, `{ key: value, … }` (a key is a string literal or any expression convertible
/// `Into<String>`, e.g. a `const`), and any other expression, converted through [`ToValue`] by method call
/// (so a borrowed `&String`/`&[T]` auto-derefs to its implementation, as `serde_json::json!` accepts it). Object
/// entries keep their written order (a `DslValue::Object` is ordered); trailing commas are
/// accepted. The equivalence law against `serde_json::json!` lives in `🧪️tests/🔬️unit/🦀️.rs`.
#[macro_export]
macro_rules! dsl_value {
    (@array [$($elements:expr,)*]) => { ::std::vec![$($elements,)*] };
    (@array [$($elements:expr),*]) => { ::std::vec![$($elements),*] };
    (@array [$($elements:expr,)*] null $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!(null)] $($rest)*) };
    (@array [$($elements:expr,)*] true $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!(true)] $($rest)*) };
    (@array [$($elements:expr,)*] false $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!(false)] $($rest)*) };
    (@array [$($elements:expr,)*] [$($array:tt)*] $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!([$($array)*])] $($rest)*) };
    (@array [$($elements:expr,)*] {$($object:tt)*} $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!({$($object)*})] $($rest)*) };
    (@array [$($elements:expr,)*] $next:expr, $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!($next),] $($rest)*) };
    (@array [$($elements:expr,)*] $last:expr) => { $crate::dsl_value!(@array [$($elements,)* $crate::dsl_value!($last)]) };
    (@array [$($elements:expr),*] , $($rest:tt)*) => { $crate::dsl_value!(@array [$($elements,)*] $($rest)*) };
    (@array [$($elements:expr),*] $unexpected:tt $($rest:tt)*) => { ::std::compile_error!("dsl_value!: unexpected token in array") };
    (@object $entries:ident () () ()) => {};
    (@object $entries:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $entries.push((::std::string::String::from($($key)+), $value));
        $crate::dsl_value!(@object $entries () ($($rest)*) ($($rest)*));
    };
    (@object $entries:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => { ::std::compile_error!("dsl_value!: expected `,` after an object entry") };
    (@object $entries:ident [$($key:tt)+] ($value:expr)) => { $entries.push((::std::string::String::from($($key)+), $value)); };
    (@object $entries:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!(null)) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!(true)) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!(false)) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!([$($array)*])) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: {$($object:tt)*} $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!({$($object)*})) $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!($value)) , $($rest)*); };
    (@object $entries:ident ($($key:tt)+) (: $value:expr) $copy:tt) => { $crate::dsl_value!(@object $entries [$($key)+] ($crate::dsl_value!($value))); };
    (@object $entries:ident ($($key:tt)+) (:) $copy:tt) => { ::std::compile_error!("dsl_value!: missing value after `:`") };
    (@object $entries:ident ($($key:tt)+) () $copy:tt) => { ::std::compile_error!("dsl_value!: missing `:` after an object key") };
    (@object $entries:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => { ::std::compile_error!("dsl_value!: unexpected `:`") };
    (@object $entries:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => { ::std::compile_error!("dsl_value!: unexpected `,`") };
    (@object $entries:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries ($key) (: $($rest)*) (: $($rest)*)); };
    (@object $entries:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => { $crate::dsl_value!(@object $entries ($($key)* $tt) ($($rest)*) ($($rest)*)); };
    (null) => { $crate::value::DslValue::Null };
    (true) => { $crate::value::DslValue::Bool(true) };
    (false) => { $crate::value::DslValue::Bool(false) };
    ([]) => { $crate::value::DslValue::Array(::std::vec::Vec::new()) };
    ([ $($tt:tt)+ ]) => { $crate::value::DslValue::Array($crate::dsl_value!(@array [] $($tt)+)) };
    ({}) => { $crate::value::DslValue::Object(::std::vec::Vec::new()) };
    ({ $($tt:tt)+ }) => {
        $crate::value::DslValue::Object({
            let mut entries: ::std::vec::Vec<(::std::string::String, $crate::value::DslValue)> = ::std::vec::Vec::new();
            $crate::dsl_value!(@object entries () ($($tt)+) ($($tt)+));
            entries
        })
    };
    ($other:expr) => {{
        use $crate::value::ToValue as _;
        (&$other).to_value()
    }};
}
//#endregion 🔖️Literal



#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

````

### 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs

Bytes763; SHA-256 9116f2cfbbd1d441df1411c705420a80abc9669df267535d5b6dbe2ada86593d

````text
//! 🌱️ Actual neutral value package; types are mounted once below replication and products.
extern crate self as semio_framework_value;
#[path = "../../🦀️.rs"]
pub mod value;
pub use value::*;
pub use serde;
pub use serde_json;
pub use semio_framework_io_base64::{base64_standard_encode,base64_standard_decode};
pub use semio_framework_value_derive::{FromValue, RetainedClone, RetireOwned, ToValue};

#[path = "../../♻️retirement/🧬️contract/🦀️.rs"]
mod retirement_contract;
pub use retirement_contract::*;
#[path = "../../♻️retirement/🦀️.rs"]
pub mod retirement;
#[path = "../../🧬️retained-clone/🦀️.rs"]
pub mod retained_clone;

#[path = "../../🏷️type/🦀️.rs"]
mod types;
pub use types::{ValueKind, ValueType};

````

### 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml

Bytes729; SHA-256 2f10ca5328d67a68f407d2ab20b63f7b17ed332d1ec53119885fdebc3f244057

````text
[package]
workspace = "../../../../.."
name = "semio-framework-value"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Neutral typed value conversion, ordered ownership, bounded retained cloning and explicit retirement."

[package.metadata.semio]
role = "framework"
id = "value"

[lints]
workspace = true

[lib]
name = "semio_framework_value"
path = "🦀️.rs"

[features]
default = []
ordered-set-serde = []

[dependencies]
semio-framework-value-derive = { path = "../../✨️derive/📦️packages/🦀️rust" }
semio-framework-io-base64 = { path = "../../../🚪️io/🔤️base64/📦️packages/🦀️rust" }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"

````

### 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts

Bytes3294; SHA-256 5ad6a0eba90922593ae86baa346311b465d9aa247c06bf00cf94c438f8dab3d4

````text
#!/usr/bin/env bun
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";

/** 🌱️ Tests the actual neutral value package under its explicitly supplied native policy. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] === "portable") {
      if (args.length !== 1) throw Error("Expected test portable");
      await runOwnedCommand(process.execPath, ["test", resolve(this.root, "../../🔁️codec/🧪️tests/🛬️controlled/🟦️.ts")], this.repoRoot, "value:portable:construction", 15_000);
      return;
    }
    const { rest } = resolveTestLevel(args);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-value"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}
/** 🛬️ Exercises canonical borrowed construction, allocation admission and owner retirement. */
class ControlledValueTestScript extends BundleScript {
  async run(args:string[]):Promise<void>{
    const {rest}=resolveTestLevel(args);
    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_",...rest]},readCargoTestPolicyV1(process.env));
  }
}
/** 🛫️ Exercises explicit borrowed output construction and cumulative encoding admission. */
class ControlledEncodingTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{const{rest}=resolveTestLevel(args);await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-value"],cwd:this.root,extraArgs:["--lib","controlled_value_encoding_",...rest]},readCargoTestPolicyV1(process.env));}
}
/** 🏷️ Validates the canonical type corpus and direct lower ownership. */
class TypeOwnershipTestScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("Expected test-type-ownership");
  const source=resolve(this.root,"../../🏷️type/🟦️.ts"),tests=resolve(this.root,"../../🏷️type/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source,tests],this.repoRoot,"value:type:types",15000);
  await runOwnedCommand(process.execPath,["test",tests],this.repoRoot,"value:type:ownership",15000);
 }
}
await runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-controlled-construction", ControlledValueTestScript).register("test-controlled-encoding",ControlledEncodingTestScript), { defaultCommand: "test" });

````

### 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json

Bytes1958; SHA-256 ae94cbe08e09c9d95a269a1605a50f2729aba2a3937d7a8d5c7462edd4f00397

````text
{
  "name": "@semio-tech/value-rs",
  "projectType": "library",
  "namedInputs": {
    "default": [
      "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*",
      "sharedGlobals"
    ]
  },
  "targets": {
    "test": {
      "executor": "nx:run-commands",
      "cache": true,
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-controlled-construction": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-controlled-construction",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-controlled-encoding": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-controlled-encoding",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-portable": {
      "executor": "nx:run-commands",
      "cache": false,
      "inputs": [
        "default",
        "^production"
      ],
      "outputs": [],
      "dependsOn": [],
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test portable"
      }
    },
    "test-type-ownership": {
      "executor": "nx:run-commands",
      "cache": false,
      "inputs": [
        "default"
      ],
      "outputs": [],
      "dependsOn": [],
      "options": {
        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-type-ownership"
      }
    }
  }
}

````

### 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/package.json

Bytes280; SHA-256 b978b61b6fd11e91845c32bb721e65c24ee7bc0fd3019e77010a47ebf94c2b5f

````text
{
  "name": "@semio-tech/value-rs",
  "private": true,
  "type": "module",
  "nx": {
    "includedScripts": []
  },
  "scripts": {
    "test-portable": "nx run @semio-tech/value-rs:test-portable",
    "test-type-ownership": "nx run @semio-tech/value-rs:test-type-ownership"
  }
}

````

### 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🦀️.rs

Bytes8194; SHA-256 31670e3e29f9cc4832c14b9c03e696ae21c2c6de604dffc056d0b806e033f310

````text
use semio_framework_os_kernel::*;
extern crate semio_framework_os_kernel as dsl;
pub use semio_framework_os_kernel as store;
#[path = "../../../../../../../../../🔨️modules/🌱️value/🛬️decode/🧪️tests/🦀️.rs"]
mod native_materialization;
#[path = "../../../../../../../../../🔨️modules/🌱️value/🛬️decode/🧪️tests/🪆️binding/🦀️.rs"]
mod native_binding;
pub use semio_framework_os_kernel::{io_schema, os_dsl, sqlite_snapshot};
pub use semio_framework_os_kernel::{os_io, os_pack};
pub use semio_framework_os_kernel::os_pack::codec;

#[path = "../🦀️.rs"]
mod decoding;
#[path = "../../../🪶️native-encoding/🧪️tests/🦀️.rs"]
mod encoding;
#[path = "../../../🪶️native-retirement/🧪️tests/🦀️.rs"]
mod retirement;

#[path = "🧬️octets/🦀️.rs"]
mod octets;

#[path = "../../../../../../🧬️semio/🧪️tests/🚦️controlled/🦀️.rs"]
mod envelope;

#[path = "../../../../../../🗣️dsl/🧬️schema/🧪️tests/🛬️decoding/🦀️.rs"]
mod controlled_text;

#[path = "../../../../../../🗣️dsl/🧬️schema/🏭️producer/🧪️tests/🦀️.rs"]
mod schema_metadata;

#[test]
fn sqlite_snapshot_link_controlled_binding_preserves_all_pin_branches_and_cancels_before_copies(){
    use dsl::{DslField,FieldValue,NativeDecodeControl};

    let references:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🚪️io/🧬️schema/🔗️reference/🧫️fixtures/🔣️.json" )).unwrap();
    for value in references["references"].as_array().unwrap(){
        let target=store::os_io::ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:store::os_io::ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}};
        let expected=store::ArtifactLink{target,pin:store::LinkPin::Head,role:"literal".into()};let field=<store::ArtifactLink as DslField>::to_value(&expected);
        assert_eq!(<store::ArtifactLink as DslField>::from_value(&field).unwrap(),expected);
        assert_eq!(<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut NativeDecodeControl::new(4096,&mut |_|true)).unwrap(),expected);
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧩️composed-pack-schema/🔣️.json" )).unwrap();let model=&fixture["documents"][0]["snapshot"];
    for row in [ &model["cover"], &model["links"][0], &model["links"][1] ]{
        let pin=match row["pin"]["kind"].as_str().unwrap(){"head"=>store::LinkPin::Head,"checkpoint"=>store::LinkPin::Checkpoint{id:row["pin"]["id"].as_str().unwrap().into()},"snapshot"=>store::LinkPin::Snapshot{blob:store::BlobRef{hash:row["pin"]["hash"].as_str().unwrap().into(),size:u64::MAX,media_type:row["pin"]["mediaType"].as_str().unwrap().into()}},_=>unreachable!()};
        let expected=store::ArtifactLink{target:store::os_io::ArtifactRef::parse_uri(row["target"].as_str().unwrap()).unwrap(),pin,role:row["role"].as_str().unwrap().into()};let field=<store::ArtifactLink as DslField>::to_value(&expected);
        let decoded=<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut NativeDecodeControl::new(4096,&mut |_|true)).unwrap();assert_eq!(decoded,expected);
        let dsl::Shape::Record(spec)=<store::ArtifactLink as DslField>::shape()else{unreachable!()};let FieldValue::Record(record)=&field else{unreachable!()};
        let bytes=os_pack::encode_document(&(spec.ordinary)(),record,&store::PackEncodeOptions::default()).unwrap();let(decoded_record,report)=os_pack::decode_document(&bytes,&(spec.ordinary)(),&store::PackDecodeOptions::default()).unwrap();assert!(!report.schema_drift);let mut decoded_field=FieldValue::Record(decoded_record);
        assert_eq!(<store::ArtifactLink as DslField>::from_value(&decoded_field).unwrap(),expected);assert_eq!(<store::ArtifactLink as DslField>::from_value_controlled(&decoded_field,&mut NativeDecodeControl::new(4096,&mut |_|true)).unwrap(),expected);
        let FieldValue::Record(root)=&mut decoded_field else{unreachable!()};let Some(FieldValue::Record(pin))=root.fields.get_mut(&1)else{unreachable!()};pin.fields.insert(99,FieldValue::Absent);assert!(<store::ArtifactLink as DslField>::from_value_controlled(&decoded_field,&mut NativeDecodeControl::new(4096,&mut |_|true)).is_err());

        assert!(<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut NativeDecodeControl::new(1,&mut |_|true)).is_err());
        assert!(<store::ArtifactLink as DslField>::from_value_controlled(&field,&mut NativeDecodeControl::new(4096,&mut |_|false)).is_err());
        let mut malformed=field;let FieldValue::Record(record)=&mut malformed else{unreachable!()};record.fields.insert(99,FieldValue::Text("unknown".into()));assert!(<store::ArtifactLink as DslField>::from_value_controlled(&malformed,&mut NativeDecodeControl::new(4096,&mut |_|true)).is_err());
    }
}

#[test]
fn sqlite_snapshot_reference_controlled_projection_preserves_literal_identity_and_stops_before_ownership(){
    use dsl::{DslField,NativeEncodeControl};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🚪️io/🧬️schema/🔗️reference/🧫️fixtures/🔣️.json")).unwrap();
    for value in fixture["references"].as_array().unwrap(){
        let reference=store::os_io::ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:store::os_io::ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}};
        let field=<store::os_io::ArtifactRef as DslField>::to_value_controlled(&reference,&mut NativeEncodeControl::new(8192,&mut |_|true)).unwrap();assert_eq!(<store::os_io::ArtifactRef as DslField>::from_value(&field).unwrap(),reference);
        let mut admit=|_|true;let mut control=NativeEncodeControl::new(1,&mut admit);assert!(<store::os_io::ArtifactRef as DslField>::to_value_controlled(&reference,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
        let mut cancel=|_|false;let mut control=NativeEncodeControl::new(8192,&mut cancel);assert!(<store::os_io::ArtifactRef as DslField>::to_value_controlled(&reference,&mut control).is_err());assert_eq!(control.owned_bytes(),0);
        for pin in [store::LinkPin::Head,store::LinkPin::Checkpoint{id:String::new()},store::LinkPin::Snapshot{blob:store::BlobRef{hash:"!/@\0".into(),size:u64::MAX,media_type:String::new()}}]{let expected=store::ArtifactLink{target:reference.clone(),pin,role:"\0 !/@".into()};let field=<store::ArtifactLink as DslField>::to_value_controlled(&expected,&mut NativeEncodeControl::new(8192,&mut |_|true)).unwrap();assert_eq!(<store::ArtifactLink as DslField>::from_value(&field).unwrap(),expected);}
    }
}

#[test]
fn sqlite_snapshot_reference_controlled_value_preserves_derived_literal_fields(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🚪️io/🧬️schema/🔗️reference/🧫️fixtures/🔣️.json")).unwrap();
    for value in fixture["references"].as_array().unwrap(){let reference=store::os_io::ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:store::os_io::ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}};let encoded=<store::os_io::ArtifactRef as dsl::ToValue>::to_value_controlled(&reference,&mut dsl::NativeEncodeControl::new(8192,&mut |_|true)).unwrap();assert_eq!(<store::os_io::ArtifactRef as dsl::FromValue>::from_value(encoded).unwrap(),reference);let mut admit=|_|true;let mut control=dsl::NativeEncodeControl::new(1,&mut admit);assert!(<store::os_io::ArtifactRef as dsl::ToValue>::to_value_controlled(&reference,&mut control).is_err());assert_eq!(control.owned_bytes(),0);}
}



````

### 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts

Bytes186159; SHA-256 ff54eb7b22feaf40d366bd1acaf4ffbd429d536ed1324a79a1053c8d4ec8dcb7

````text
#!/usr/bin/env bun
import { runVitestV1, readVitestPolicyV1 } from "../../../../🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { runBudgetedTestCommand } from "../../../../🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { resolveTestLevel } from "../../../../🔨️modules/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🦀️ `@semio-tech/framework-os-kernel` task router. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { type ValidateFunction } from "ajv";
import Ajv2020 from "ajv/dist/2020.js";
import { runCargo, runRepositoryCargoTests, runRepositoryTestCommand, runRepositoryExactCargoLaws } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runNestedCargoPackageAdapter } from "../../../🦑️repo/🔨️modules/📚️library/📽️projection/🧩️package-adapter/📦️publication/🟦️.ts";
import { blake3Hex } from "../../../../🔨️modules/🔏️hash/🟦️.ts";
import { semioSchemaAjvV1 } from "../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

/** 🧬️ A compiled owned-schema export, typed as a boolean runtime check so `assert` never narrows its validated subject to `unknown`. */
type SchemaCheck = ((data: unknown) => boolean) & Pick<ValidateFunction, "errors">;

//#region 🧬️OwnedSchemaExports
const OS_MODULE_SCHEMAS = {
  "db.engine": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧬️schema/🔣️.json",
  "db.wal": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🧬️schema/🔣️.json",
  "db.storage": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧬️schema/🔣️.json",
  "db.storage.writer": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧬️schema/🔣️.json",
  "db.compact": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🧬️schema/🔣️.json",
  "db.artifact": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧬️schema/🔣️.json",
  directory: "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json",
} as const;

/** 🧬️ Compiles one named `$defs` export of an owning `🧬️schema/` module against its draft-07 `$id`. */
function ownedExport(repoRoot: string, scope: keyof typeof OS_MODULE_SCHEMAS, exportId: string): SchemaCheck {
  const doc = JSON.parse(readFileSync(join(repoRoot, OS_MODULE_SCHEMAS[scope]), "utf8")) as { $id: string };
  const compiled = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(doc).getSchema(`${doc.$id}#/$defs/${exportId}`);
  if (!compiled) throw new Error(`${scope} schema module publishes no export ${exportId}`);
  return compiled as ValidateFunction;
}
//#endregion 🧬️OwnedSchemaExports


function exactCargoStageEnvironments() {
  return {
    env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}

/** ↔️ Admits portable paged traversal and independently checks array/deque ordering. */
class PagedHistoryStackScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length > 1 || (args.length && args[0] !== "--native")) throw Error("paged-history-stack-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📸️paged-history-stack/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/📸️paged-history-stack/🔣️.json"), "utf8"));
    const admit = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
    assert(admit(fixture), JSON.stringify(admit.errors));
    for (const vector of fixture.vectors) {
      const values = Array.from({ length: vector.pushes }, (_, i) => String(i));
      if (vector.removeLogical !== null) values.splice(vector.removeLogical, 1);
      if (vector.replacement !== null) values.push(vector.replacement);
      assert.deepEqual(values, vector.forward);
      assert.deepEqual([...values].reverse(), vector.reverse);
      assert.deepEqual(vector.directions.map((direction: string) => (direction === "front" ? values.shift() : values.pop()) ?? null), vector.expected);
      assert.deepEqual(values, vector.remaining);
    }
    const branchSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🌿️branch-provenance/🔣️.json"), "utf8"));
    const branchFixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🌿️branch-provenance/🔣️.json"), "utf8"));
    const branchAjv = semioSchemaAjvV1({ strict: true, allErrors: true });
    assert(branchAjv.compile(branchSchema)(branchFixture));
    const editCheck = branchAjv.compile(branchSchema.$defs.Edit);
    for (const vector of branchFixture.vectors) {
      assert.equal(editCheck(vector.edit), vector.valid, vector.id);
      assert.equal(Object.hasOwn(vector.edit, "line") && (vector.edit.line === null || typeof vector.edit.line === "string"), vector.valid, vector.id);
    }
    const { testCanonicalEditFixtures } = await import("../../🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts");
    const { storeCanonicalEditSealerSelfTests } = await import("../../🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts");
    testCanonicalEditFixtures();
    const canonical = storeCanonicalEditSealerSelfTests();
    console.log("paged-history-canonical-oracles: " + JSON.stringify(canonical));
    if (args[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({ cwd: this.repoRoot, ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib", name: "semio_framework_os_kernel" }, laws: [
          "os_vcs::tests::paged_history_stack_traversal_follows_the_portable_deque_vectors",
          "os_vcs::tests::history_branch_provenance_follows_portable_required_wire_vectors",
          "os_store::component::canonical_edit::tests::edit_digest_chains_match_the_neutral_vectors_and_extend_incrementally",
          "os_store::component::canonical_edit::tests::canonical_authority_final_unicode_strings_retire_under_single_byte_grants",
        ] }], artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: 3_600_000, listBudgetMs: 60_000, lawBudgetMs: 120_000 });
      for (const receipt of receipts) console.log("paged-history-stack-native-receipt: " + JSON.stringify(receipt));
    }
    console.log(`paged-history-stack-check: vectors=${fixture.vectors.length} independent-array/Ajv=passed`);
  }
}

/** 📜️ Verifies history-result publication and exact replay-owner retirement. */
class DatabaseHistoryCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-history-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/📜️history-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "HistoryCompletionV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.publication.map((row: { name: string }) => row.name), ["before-poll", "before-registration", "after-pending"].flatMap(stage => ["success", "cancelled"].map(outcome => stage + "-" + outcome)));
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE handoff (payload TEXT, location TEXT, waiter INTEGER, wakes INTEGER, admission INTEGER, registry INTEGER)");
      for (const row of fixture.publication) {
        oracle.run("DELETE FROM handoff");
        const payload = row.outcome === "success" ? fixture.operations : { error: "Closed" };
        oracle.run("INSERT INTO handoff VALUES (?1, 'producer', 0, 0, 1, 1)", [JSON.stringify(payload)]);
        const snapshot = () => oracle.query("SELECT payload, location, waiter, wakes, admission, registry FROM handoff").get() as { payload: string; location: string; waiter: number; wakes: number; admission: number; registry: number };
        const publish = () => oracle.run("UPDATE handoff SET location = 'completion', wakes = wakes + waiter, waiter = 0 WHERE location = 'producer'");
        if (row.publication === "before-poll") publish();
        let firstReady = snapshot().location === "completion";
        if (!firstReady) {
          if (row.publication === "before-registration") publish();
          oracle.run("UPDATE handoff SET waiter = 1");
          firstReady = snapshot().location === "completion";
        }
        assert.equal(firstReady, row.firstReady, row.name);
        if (!firstReady) publish();
        assert.equal(oracle.run("UPDATE handoff SET location = 'consumer', waiter = 0 WHERE location = 'completion'").changes, 1, row.name);
        assert.equal(JSON.stringify(JSON.parse(snapshot().payload)) === JSON.stringify(payload), row.exactResult, row.name);
        assert.equal(snapshot().waiter === 0, row.waiterEmpty, row.name);
        assert.equal(snapshot().wakes, row.wakes, row.name);
        assert.equal(snapshot().waiter === 0, row.wakeLockReleased, row.name);
        oracle.run("UPDATE handoff SET location = 'empty', admission = 0, registry = 0 WHERE location = 'consumer'");
        assert.equal(snapshot().admission === 0, row.admissionReleased, row.name);
        assert.equal(snapshot().registry === 0, row.registryEmpty, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-history-completion-check: AJV=1 sqlite-publication=" + fixture.publication.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-os-kernel-db",
        target: { kind: "lib", name: "db" },
        cargoArgs: ["--all-features"],
        laws: [
          "artifact_history_completion_interleavings_preserve_result_and_wake",
          "artifact_history_empty_and_two_batch_replay_are_deterministic",
          "artifact_history_empty_one_cap_plus_one_admission_returns_exact_request",
          "artifact_history_cancel_before_handoff_retires_full_reservation_before_credit_release",
          "artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-history-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-history-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📖️ Verifies catalog-root ownership independently of scalar capability opening. */
class DatabaseCatalogReadOwnershipCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-catalog-read-ownership-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/📖️catalog-read-ownership");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "CatalogReadOwnershipV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.transfers.map((row: { phase: string }) => row.phase), ["Handoff", "RetainWork", "Poll"]);
    assert.deepEqual(fixture.completion.map((row: { name: string }) => row.name), ["synchronous", "before-wake", "after-finalizer-check", "refused-finalizer"].flatMap(stage => ["pages", "backend-fault"].map(outcome => stage + "-" + outcome)));
    assert.deepEqual(fixture.recovery.map((row: { name: string }) => row.name), ["retry", "terminal-completion", "terminal-result", "spent-work"].flatMap(path => ["pages", "backend-fault"].map(outcome => path + "-" + outcome)));
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE ownership (active INTEGER, location TEXT, admission INTEGER, waiter INTEGER, releases INTEGER, payload TEXT)");
      const reset = (active: number, location: string, payload: unknown) => {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (?1, ?2, 1, 1, 0, ?3)", [active, location, JSON.stringify(payload)]);
      };
      const finalize = () => oracle.run("UPDATE ownership SET active = 4, admission = 0, releases = releases + 1 WHERE active = 0 AND location = 'empty' AND waiter = 0 AND admission = 1");
      for (const row of fixture.transfers) {
        reset(1, "stack", fixture.root);
        const close = oracle.run("UPDATE ownership SET admission = 0 WHERE active = 0 AND location = 'empty'").changes;
        assert.equal(close === 0, row.closeBlocked, row.name);
        const paused = oracle.query("SELECT admission, location FROM ownership").get() as { admission: number; location: string };
        assert.equal(paused.admission === 1, row.admissionRetained, row.name);
        assert.equal(paused.location === "stack", row.storageRetained, row.name);
        const submissions = oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes;
        assert.equal(submissions, row.submissionsWhileActive, row.name);
        oracle.run("UPDATE ownership SET active = 0, location = 'empty', waiter = 0");
        finalize();
        assert.equal((oracle.query("SELECT admission = 0 AS empty FROM ownership").get() as { empty: number }).empty === 1, row.finalEmpty, row.name);
      }
      for (const row of fixture.completion) {
        const payload = row.outcome === "pages" ? fixture.root : { key: fixture.root.key, error: "fixture-root-fault" };
        reset(row.publication === "synchronous" ? 0 : 1, "completion", payload);
        assert.deepEqual(JSON.parse((oracle.query("SELECT payload FROM ownership").get() as { payload: string }).payload), payload, row.name);
        oracle.run("UPDATE ownership SET location = 'empty', waiter = 0");
        finalize();
        assert.equal((oracle.query("SELECT admission FROM ownership").get() as { admission: number }).admission === 1, row.admissionDuringPublication, row.name);
        const retirement = row.publication === "after-finalizer-check" || row.publication === "refused-finalizer";
        const submissions = retirement ? oracle.run("UPDATE ownership SET active = 2 WHERE active = 1").changes : 0;
        assert.equal(submissions, row.retirementSubmissions, row.name);
        oracle.run("UPDATE ownership SET active = 0 WHERE active IN (1, 2)");
        finalize();
        assert.equal(oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes, row.lateSubmissions, row.name);
        const final = oracle.query("SELECT admission = 0 AND releases = 1 AS empty FROM ownership").get() as { empty: number };
        assert.equal(final.empty === 1, row.terminalEmpty, row.name);
      }
      oracle.run("CREATE TABLE recovery (generation INTEGER, checked_out INTEGER, admission INTEGER, deliveries INTEGER, payload TEXT)");
      for (const row of fixture.recovery) {
        oracle.run("DELETE FROM recovery");
        const payload = row.outcome === "pages" ? fixture.root : { key: fixture.root.key, error: "fixture-root-fault" };
        oracle.run("INSERT INTO recovery VALUES (1, 0, 1, 0, ?1)", [JSON.stringify(payload)]);
        if (row.path === "retry") {
          oracle.run("UPDATE recovery SET generation = generation + 1");
          assert.equal(oracle.run("UPDATE recovery SET deliveries = deliveries + 1 WHERE generation = 1").changes, row.staleRetrySubmissions, row.name);
        }
        if (row.path === "terminal-result" || row.path === "spent-work") {
          oracle.run("UPDATE recovery SET checked_out = 1");
          assert.equal(oracle.run("UPDATE recovery SET admission = 0 WHERE checked_out = 0").changes === 0, row.checkoutBlocksRetirement, row.name);
          oracle.run("UPDATE recovery SET checked_out = 0");
        } else {
          assert.equal(row.checkoutBlocksRetirement, false, row.name);
        }
        oracle.run("UPDATE recovery SET deliveries = deliveries + 1, admission = 0 WHERE checked_out = 0");
        const actual = oracle.query("SELECT deliveries, admission, payload FROM recovery").get() as { deliveries: number; admission: number; payload: string };
        assert.equal(actual.deliveries, 1, row.name);
        assert.deepEqual(JSON.parse(actual.payload), payload, row.name);
        assert.equal(actual.admission === 0, row.terminalEmpty, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-catalog-read-ownership-check: AJV=1 sqlite-transfers=" + fixture.transfers.length + " sqlite-completion=" + fixture.completion.length + " sqlite-recovery=" + fixture.recovery.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-os-kernel-db",
        target: { kind: "lib", name: "db" },
        cargoArgs: ["--all-features"],
        laws: [
          "database_catalog_read_paused_transfers_exclude_successors_and_public_cleanup",
          "database_catalog_read_consumed_publication_preserves_exact_root_and_retires",
          "database_catalog_read_retry_and_terminal_resume_preserve_exact_root",
          "database_catalog_read_fixed_cap_plus_one_and_generation_aba",
          "database_catalog_read_success_returns_exact_storage_key_and_root",
          "database_catalog_read_controlled_wakes_coalesce_and_terminal_never_repolls",
          "database_catalog_read_publication_between_check_and_waker_registration_is_observed",
          "database_catalog_read_rejected_mount_retires_storage_and_key_on_distinct_grants",
          "database_catalog_read_cancel_stale_and_rejection_preserve_exact_storage_key",
          "database_catalog_read_terminal_result_drop_hands_back_exact_result",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-catalog-read-ownership-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-catalog-read-ownership-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 📬️ Verifies retained capability completion handoff at every public waiter boundary. */
class DatabaseCapabilityCompletionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-capability-completion-check accepts only --native");
    const root = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/📬️capability-completion");
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "CapabilityCompletionV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(
      fixture.publication.map((row: { name: string }) => row.name),
      ["before-poll", "before-registration", "after-pending"].flatMap((stage) => ["success", "fault"].map((outcome) => stage + "-" + outcome)),
    );
    assert.deepEqual(
      fixture.retirement.map((row: { name: string }) => row.name),
      ["consumed-before-wake-success", "consumed-before-wake-fault"],
    );
    assert.deepEqual(fixture.driveOwnership.map((row: { phase: string }) => row.phase), ["Handoff", "RetainWork", "Poll"]);
    assert.deepEqual(fixture.leaseCompletion.map((row: { publication: string }) => row.publication), ["synchronous", "active-publisher", "after-finalizer-check", "after-finalizer-check-refused"]);
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    try {
      oracle.run("CREATE TABLE handoff (id INTEGER PRIMARY KEY, completion TEXT, waiter INTEGER NOT NULL, wakes INTEGER NOT NULL, admission INTEGER NOT NULL)");
      const snapshot = () => oracle.query("SELECT completion, waiter, wakes, admission FROM handoff WHERE id = 1").get() as { completion: string | null; waiter: number; wakes: number; admission: number };
      const reset = () => {
        oracle.run("DELETE FROM handoff");
        oracle.run("INSERT INTO handoff VALUES (1, NULL, 0, 0, 1)");
      };
      const publish = (outcome: string) => oracle.run("UPDATE handoff SET completion = ?1, wakes = wakes + waiter, waiter = 0 WHERE id = 1", [outcome]);
      const consume = () => oracle.run("UPDATE handoff SET completion = NULL, waiter = 0, admission = 0 WHERE id = 1 AND completion IS NOT NULL");
      for (const row of fixture.publication) {
        reset();
        if (row.publication === "before-poll") publish(row.outcome);
        let firstReady = snapshot().completion !== null;
        if (!firstReady) {
          if (row.publication === "before-registration") publish(row.outcome);
          oracle.run("UPDATE handoff SET waiter = 1 WHERE id = 1");
          firstReady = snapshot().completion !== null;
        }
        assert.equal(firstReady, row.firstReady, row.name);
        if (!firstReady) publish(row.outcome);
        assert.equal(snapshot().completion, row.outcome, row.name);
        consume();
        assert.deepEqual(snapshot(), { completion: null, waiter: 0, wakes: row.wakes, admission: 0 }, row.name);
      }
      for (const row of fixture.retirement) {
        reset();
        oracle.run("UPDATE handoff SET waiter = 1, completion = ?1 WHERE id = 1", [row.outcome]);
        consume();
        assert.equal(snapshot().admission === 0, row.terminalBeforePublisherWake, row.name);
        oracle.run("UPDATE handoff SET wakes = wakes + waiter, waiter = 0 WHERE id = 1");
        assert.equal(snapshot().wakes, row.wakes, row.name);
      }
      oracle.run("CREATE TABLE ownership (active INTEGER NOT NULL, location TEXT NOT NULL, admission INTEGER NOT NULL, releases INTEGER NOT NULL)");
      for (const row of fixture.driveOwnership) {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (1, 'stack', 1, 0)");
        const close = () => oracle.run("UPDATE ownership SET admission = 0, releases = releases + 1 WHERE active = 0 AND location = 'empty' AND admission = 1").changes;
        assert.equal(close() === 0, row.closeBlocked, row.name);
        const paused = oracle.query("SELECT admission, location FROM ownership").get() as { admission: number; location: string };
        assert.equal(paused.admission === 1, row.admissionRetained, row.name);
        assert.equal(paused.location === "stack", row.storageRetained, row.name);
        oracle.run("UPDATE ownership SET location = 'terminal', active = 0");
        assert.equal(close(), 0, row.name);
        oracle.run("UPDATE ownership SET location = 'empty'");
        assert.equal(close(), 1, row.name);
        assert.equal(close(), 0, row.name);
        assert.equal((oracle.query("SELECT releases FROM ownership").get() as { releases: number }).releases, row.finalReleases, row.name);
      }
      for (const row of fixture.leaseCompletion) {
        oracle.run("DELETE FROM ownership");
        oracle.run("INSERT INTO ownership VALUES (?1, 'completion', 1, 0)", [row.publication === "synchronous" ? 0 : 1]);
        const checkedBeforeConsumption = row.publication.startsWith("after-finalizer-check");
        const readyAtFirstCheck = (oracle.query("SELECT location = 'empty' AS ready FROM ownership").get() as { ready: number }).ready === 1;
        oracle.run("UPDATE ownership SET location = 'empty'");
        const finalize = () => oracle.run("UPDATE ownership SET active = 4, admission = 0, releases = releases + 1 WHERE location = 'empty' AND active = 0 AND admission = 1");
        finalize();
        assert.equal((oracle.query("SELECT admission FROM ownership").get() as { admission: number }).admission === 1, row.admissionDuringPublication, row.name);
        let retirementSubmissions = 0;
        if (checkedBeforeConsumption && !readyAtFirstCheck) {
          oracle.run("UPDATE ownership SET active = active | 2 WHERE active = 1");
          oracle.run("UPDATE ownership SET active = active & ~1 WHERE active = 3");
          retirementSubmissions = (oracle.query("SELECT active = 2 AS queued FROM ownership").get() as { queued: number }).queued;
          oracle.run("UPDATE ownership SET active = 0 WHERE active = 2");
        } else {
          oracle.run("UPDATE ownership SET active = 0 WHERE active = 1");
        }
        assert.equal(retirementSubmissions, row.retirementSubmissions, row.name);
        finalize();
        assert.equal(oracle.run("UPDATE ownership SET active = 2 WHERE active = 0").changes, row.lateSubmissions, row.name);
        assert.equal((oracle.query("SELECT releases FROM ownership").get() as { releases: number }).releases, 1, row.name);
      }
    } finally {
      oracle.close();
    }
    console.log("database-capability-completion-check: AJV=1 sqlite-publication=" + fixture.publication.length + " sqlite-retirement=" + fixture.retirement.length + " sqlite-drive-ownership=" + fixture.driveOwnership.length + " sqlite-lease-completion=" + fixture.leaseCompletion.length);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "database_capability_open_paused_transfer_blocks_public_close",
            "database_capability_open_lease_successors_and_active_publication_retire_once",
            "database_capability_open_completion_interleavings_preserve_result_and_wake",
            "database_capability_open_consumed_completion_retires_before_publisher_wake",
            "database_capability_open_fixed_admission_cap_plus_one_and_generation_aba",
            "database_capability_open_success_returns_exact_storage_owner_and_scalar",
            "database_capability_open_cancel_and_stale_generation_retain_exact_owner_for_public_close",
            "database_capability_open_saturation_and_shutdown_keep_retry_job_and_public_terminal",
            "database_capability_open_poll_publication_precedes_wake_rearm_at_every_boundary",
            "database_capability_open_post_ready_cancel_and_stale_retain_public_exact_result",
            "database_capability_open_rejection_take_retry_and_close_preserve_exact_storage",
            "database_capability_open_terminal_result_take_resume_and_checked_out_drop_handback",
            "database_capability_open_retry_contention_is_one_compare_exchange_per_callback",
            "database_catalog_read_publication_between_check_and_waker_registration_is_observed",
            "database_catalog_bootstrap_publication_race_and_queue_pressure_keep_exact_successor",
            "database_create_catalog_publication_check_register_recheck_has_no_lost_wake",
            "open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("database-capability-completion-native " + event.stage + ": " + (event.law ?? "") + " artifacts=" + event.artifactDir);
      },
    });
    for (const receipt of receipts) console.log("database-capability-completion-native-receipt: " + JSON.stringify(receipt));
  }
}

/** 🔐️ Checks fixed writer capabilities and retained local backend integration. */
class WalWriterAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-writer-authority-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.storage.writer", "WriterAuthorityV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const writerDeferredWake = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔔️deferred-wake/🔣️.json"), "utf8"));
    const writerDeferredWakeSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔔️deferred-wake/🔣️.json"), "utf8"));
    const validateWriterDeferredWake = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(writerDeferredWakeSchema) as SchemaCheck;
    assert(validateWriterDeferredWake(writerDeferredWake), JSON.stringify(validateWriterDeferredWake.errors));
    const neutralDeferredWake = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧫️fixtures/🔣️.json"), "utf8"));
    const neutralDeferredWakeSchema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧬️schema/🔣️.json"), "utf8"));
    const validateNeutralDeferredWake = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(neutralDeferredWakeSchema).getSchema(`${neutralDeferredWakeSchema.$id}#/$defs/DeferredWakeFixture`) as SchemaCheck;
    assert(validateNeutralDeferredWake(neutralDeferredWake), JSON.stringify(validateNeutralDeferredWake.errors));
    assert.equal(neutralDeferredWake.capacity.partitions, writerDeferredWake.capacity.backendControls);
    assert.equal(neutralDeferredWake.capacity.slotsPerPartition, writerDeferredWake.capacity.writersPerBackend);
    assert.equal(neutralDeferredWake.capacity.totalWaiters, writerDeferredWake.capacity.backendControls * writerDeferredWake.capacity.writersPerBackend * writerDeferredWake.capacity.waitersPerWriter);
    {
    let queuedOwner = true;
    let faulted = true;
    let retryEpoch = 0;
    let queuedOwners = 1;
    let maximumQueuedOwners = queuedOwners;
    assert.equal(faulted && queuedOwner ? "pending" : "ready", writerDeferredWake.retryEpoch.hostileTrace[2]);
    assert.equal(writerDeferredWake.retryEpoch.readyBeforeOldSlotDrains, false);
    queuedOwner = false;
    assert.equal(faulted && queuedOwner ? "pending" : "fault-ready", writerDeferredWake.retryEpoch.hostileTrace[4]);
    faulted = false;
    retryEpoch += 1;
    queuedOwner = true;
    queuedOwners = Number(queuedOwner);
    maximumQueuedOwners = Math.max(maximumQueuedOwners, queuedOwners);
    assert.equal(retryEpoch, 1);
    assert.equal(maximumQueuedOwners, writerDeferredWake.retryEpoch.maximumQueuedOwnersPerSignal);
    assert.equal(writerDeferredWake.retryEpoch.admission, "fault-ready-after-exact-slot-drain");
    }
    for (const row of writerDeferredWake.cases) {
      assert.equal(row.expected.retainedFaults, row.activeRequested, row.id);
      assert.equal(row.expected.retainedGuards, row.activeRequested, row.id);
      assert.equal(row.expected.terminalEpochs, 0, row.id);
    }
    const writerDeferredStorageSource = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    const writerDeferredSource = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const writerDeferredReleaseSource = readFileSync(join(owner, "🔔️release/🦀️.rs"), "utf8");
    assert(writerDeferredStorageSource.includes(`const DB_IO_BACKEND_CONTROLS: usize = ${writerDeferredWake.capacity.backendControls};`));
    assert(writerDeferredSource.includes(`const WAL_WRITER_CAPACITY: usize = ${writerDeferredWake.capacity.writersPerBackend};`));
    assert(writerDeferredReleaseSource.includes("fn request_controller(") && writerDeferredReleaseSource.includes("fn notify_faults("));
    const missingWriter = writerDeferredWake.runtimeMarkers.writer.filter((marker: string) => !writerDeferredReleaseSource.includes(marker));
    assert.deepEqual(missingWriter, [], `missing writer runtime markers: ${missingWriter.join(", ")}`);
    console.log(`[DEBUG] writer-deferred-wake-independent-oracle: AJV=1 cases=${writerDeferredWake.cases.length} capacities=64*32*1 runtime-markers=${writerDeferredWake.runtimeMarkers.writer.length}`);
    const remoteOwner = join(owner, "🧫️fixtures/🌐️remote-guard");
    const remoteFixture = JSON.parse(readFileSync(join(remoteOwner, "🔣️.json"), "utf8"));
    const validateRemote = ownedExport(this.repoRoot, "db.storage.writer", "WalWriterFenceV1");
    assert(validateRemote(remoteFixture), JSON.stringify(validateRemote.errors));
    assert.deepEqual(remoteFixture.mutations, ["create", "append", "sync", "seal", "truncateTail", "delete"]);
    for (const row of remoteFixture.postgres.lockKeys) {
      const digest = createHash("sha256").update(Buffer.concat([Buffer.from(remoteFixture.postgres.lockNamespace, "utf8"), Buffer.from([0]), Buffer.from(row.document, "utf8")])).digest();
      assert.equal(digest.readBigInt64BE(0).toString(), row.key, `postgres advisory-lock key oracle for ${row.document}`);
    }
    assert(remoteFixture.neo4j.renewEveryMs * 2 < remoteFixture.neo4j.leaseTtlMs, "a Neo4j writer lease must survive one missed renewal");
    for (const backend of ["sqlite", "postgres", "neo4j"]) assert(remoteFixture.laws.filter((law: { backends: string[] }) => law.backends.includes(backend)).length >= 4, `${backend} runs every shared fence law`);
    const sourceOf = (path: string) => readFileSync(join(owner, "..", path, "🦀️.rs"), "utf8");
    for (const [marker, path] of [
      ["pg_try_advisory_lock($1)", "🐘️postgres"],
      ["fn fenced_wal_mutation", "🐘️postgres"],
      [`"${remoteFixture.postgres.lockNamespace}"`, "🐘️postgres"],
      ["CYPHER_WAL_WRITER_FENCE", "🌐️neo4j"],
      ["fn fenced_wal_mutation", "🌐️neo4j"],
      [`WAL_WRITER_LEASE_TTL_MS: i64 = ${remoteFixture.neo4j.leaseTtlMs.toLocaleString("en-US").replaceAll(",", "_")}`, "🌐️neo4j"],
    ] as const)
      assert(sourceOf(path).includes(marker), `missing cross-process writer fence primitive ${marker} in ${path}`);
    console.log(`wal-writer-fence-oracle: AJV=1 lockKeys=${remoteFixture.postgres.lockKeys.length} laws=${remoteFixture.laws.length}`);
    const memoryOwner = join(owner, "..", "🧫️fixtures", "🧮️memory-backing");
    const memoryFixture = JSON.parse(readFileSync(join(memoryOwner, "🔣️.json"), "utf8"));
    const validateMemory = ownedExport(this.repoRoot, "db.storage", "MemoryBackingV1");
    assert(validateMemory(memoryFixture), JSON.stringify(validateMemory.errors));
    const poolUseOwner = join(owner, "..", "🧫️fixtures", "🔐️backend-pool-use");
    const poolUseFixture = JSON.parse(readFileSync(join(poolUseOwner, "🔣️.json"), "utf8"));
    const validatePoolUse = ownedExport(this.repoRoot, "db.storage", "BackendPoolUseV1");
    assert(validatePoolUse(poolUseFixture), JSON.stringify(validatePoolUse.errors));
    assert.deepEqual(
      poolUseFixture.cases.map((row: { name: string }) => row.name),
      [
        "registered-backend-blocks-shutdown",
        "task-derives-registered-pool",
        "dropped-facade-retains-use",
        "compaction-acquires-before-admission",
        "forged-kind-rejected-before-page-admission",
        "rollback-reserved-before-owner-transfer",
        "all-retirement-tiers-full-return-exact-executor",
        "committed-rollback-retains-use-until-terminal",
        "prepared-post-transfer-refusal-returns-close-owner",
      ],
    );
    const { Database } = await import("bun:sqlite");
    const openingOracle = new Database(":memory:");
    try {
      openingOracle.run("CREATE TABLE opening_owner (backend TEXT PRIMARY KEY, retained INTEGER NOT NULL, close_requested INTEGER NOT NULL)");
      for (const row of poolUseFixture.opening) {
        openingOracle.run("INSERT INTO opening_owner VALUES (?1, 1, 0)", [row.backend]);
        openingOracle.run("UPDATE opening_owner SET close_requested = 1 WHERE backend = ?1", [row.backend]);
        const requested = openingOracle.query("SELECT close_requested FROM opening_owner WHERE backend = ?1").get(row.backend) as { close_requested: number };
        assert.equal(!!requested.close_requested, row.closeRequestedBeforeRetry);
        openingOracle.run("DELETE FROM opening_owner WHERE backend = ?1 AND close_requested = 1", [row.backend]);
        const terminal = openingOracle.query("SELECT count(*) AS retained FROM opening_owner").get() as { retained: number };
        assert.equal(terminal.retained === 0, row.ledgerBaseline);
        assert.equal(terminal.retained === 0, row.poolShutdownAfterDrain);
        assert.equal(terminal.retained === 0, row.reopens);
      }
    } finally {
      openingOracle.close();
    }
    assert.deepEqual(memoryFixture.writerTable, { slots: fixture.capacity, separateBox: true });
    assert.deepEqual(memoryFixture.controllerCredit, { items: 1, controls: 1, bytesFormula: "wake-plus-two-usize" });
    assert.equal(memoryFixture.retainedPageResults, 44);
    assert.equal(memoryFixture.sameSlotReuseBeforeOldResultClose, true);
    assert.equal(memoryFixture.taskSlotReleasedWhileResultRetained, true);
    assert.equal(memoryFixture.droppedResultRetirement, "mounted-io-maintenance");
    const directoryOwner = join(owner, "..", "🧫️fixtures", "📁️directory-durability");
    const directoryFixture = JSON.parse(readFileSync(join(directoryOwner, "🔣️.json"), "utf8"));
    const validateDirectory = ownedExport(this.repoRoot, "db.storage", "DirectoryDurabilityV1");
    assert(validateDirectory(directoryFixture), JSON.stringify(validateDirectory.errors));
    const openOwner = join(owner, "..", "..", "📝️wal", "🧫️fixtures", "🚪️open-rejection");
    const openFixture = JSON.parse(readFileSync(join(openOwner, "🔣️.json"), "utf8"));
    const validateOpen = ownedExport(this.repoRoot, "db.wal", "OpenRejectionV1");
    assert(validateOpen(openFixture), JSON.stringify(validateOpen.errors));
    let openOwnerState = "exact-writer-permit";
    assert.equal(openFixture.acquisition[1].owner, openOwnerState);
    openOwnerState = "exact-writer-release";
    assert.equal(openFixture.acquisition[2].owner, openOwnerState);
    assert.equal(openFixture.acquisition[2].releaseActivation, "first-explicit-close-poll");
    const exactRelease = Symbol("exact-writer-release");
    let retainedRelease: symbol | undefined = exactRelease;
    for (const [index, close] of openFixture.explicitClose.entries()) {
      assert.equal(close.attempt, index + 1);
      assert.equal(close.openCause, "original-open-error");
      if (close.outcome === "fault") assert.equal(retainedRelease, exactRelease);
      else retainedRelease = undefined;
      assert.equal(close.sameDocumentAcquire, retainedRelease ? "conflict" : "available");
    }
    assert.equal(openFixture.droppedRejection.behavior === "transfer" && !openFixture.droppedRejection.panics ? "backend-release-cell" : "discarded", openFixture.droppedRejection.location);
    const directoryNames = new Set(["segment", "marker"]);
    directoryNames.delete("segment");
    assert.equal(!directoryNames.has("segment") && directoryNames.has("marker") ? "not-found-with-retained-marker" : "invalid", directoryFixture.deleteFaultState);
    directoryNames.delete("marker");
    assert.equal(directoryNames.size, 0);
    assert(directoryFixture.delete.indexOf("segment-delete-parent-synced") < directoryFixture.delete.indexOf("marker-deleted"));
    const pinnedWriter = { operation: fixture.controllerBinding.operation, releaseRequested: true };
    assert.equal(pinnedWriter.releaseRequested && pinnedWriter.operation !== fixture.controllerBinding.contender ? "closed" : "ok", fixture.controllerBinding.fenceBeforeCallback);
    assert.equal(pinnedWriter.operation === fixture.controllerBinding.operation ? "ok" : "closed", fixture.controllerBinding.pinnedResume);
    assert.equal(fixture.controllerBinding.tasksAddedOnRelease, 0);
    assert.equal(fixture.controllerBinding.guardFactoryCallsOnConflict, 0);
    assert.equal(fixture.controllerBinding.faultRetryPreservesKey, true);
    const pendingWriters = new Map([
      ["fault", "retained"],
      ["healthy", "retained"],
    ]);
    const faultTrace = ["fault-retained"];
    pendingWriters.delete("healthy");
    faultTrace.push("healthy-terminal");
    assert.equal(pendingWriters.get("fault"), "retained");
    pendingWriters.delete("fault");
    faultTrace.push("fault-retry-terminal");
    assert.deepEqual(faultTrace, fixture.controllerFaults.coalesced);
    assert.equal(fixture.controllerFaults.outerPanic.executorTurns, 1);
    assert.equal(fixture.controllerFaults.outerPanic.wakesPerOwner, 1);
    const followerOwners = new Set(["document"]);
    assert.equal(followerOwners.has("document") ? "conflict" : "ok", fixture.replication.occupiedFollower);
    assert.deepEqual(fixture.replication.inventoryAfterConflict, []);
    const transfer = fixture.replication.snapshotTransfer;
    const sourceSnapshot = Buffer.concat(Array.from({ length: transfer.repetitions }, () => Buffer.from(transfer.pattern)));
    const copiedSnapshot = Buffer.from(sourceSnapshot);
    assert.equal(sourceSnapshot.length, transfer.bytes);
    sourceSnapshot.fill(0);
    assert.deepEqual(
      [...copiedSnapshot],
      Array.from({ length: transfer.bytes }, (_, index) => transfer.pattern[index % transfer.pattern.length]),
    );
    assert.equal(Symbol("source-result") === Symbol("independent-input"), transfer.sameOperation);
    const clusterSource = readFileSync(join(owner, "..", "..", "🌐️cluster", "🦀️.rs"), "utf8");
    assert(clusterSource.includes("db_io_copy_page_owner(&pages)") && clusterSource.includes("close_replication_pages(&mut pages)"), "snapshot replication must retire source result before a distinct write owner");
    const walSource = readFileSync(join(owner, "..", "..", "📝️wal", "🦀️.rs"), "utf8");
    for (const marker of ["struct ArtifactWalAcquiredRejected", "enum ArtifactWalOpenRejected", "retry_open(", "retry_close(", "open_acquired(", "into_open_rejected"])
      assert(walSource.includes(marker), "missing retained WAL-open owner primitive: " + marker);
    assert(!walSource.includes("release_failed_open"), "WAL open rejection must not await and flatten its writer release");
    const artifactSource = readFileSync(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"), "utf8");
    for (const marker of ["enum ArtifactEngineOpenRejected", "RetainedWal", "has_retained_writer", "Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>>"])
      assert(artifactSource.includes(marker), "missing engine retained-open propagation: " + marker);
    const engineSource = readFileSync(join(owner, "..", "..", "⚙️engine", "🦀️.rs"), "utf8");
    for (const marker of ["enum DatabaseDocumentOpenRejected", "Result<ArtifactHandle, DatabaseDocumentOpenRejected>", "rejected.retry_close().await"]) assert(engineSource.includes(marker), "missing database retained-open propagation: " + marker);
    for (const marker of ["enum ReplicationRejected", "ArtifactWal::open_acquired", "ReplicationRejected::WalOpen"]) assert(clusterSource.includes(marker), "missing cluster acquired-writer propagation: " + marker);
    for (const outcome of fixture.replication.releaseAfter) {
      const owner = new Set(["document"]);
      try {
        assert(["tail", "up-to-date", "snapshot", "leader-corrupt"].includes(outcome));
      } finally {
        owner.delete("document");
      }
      assert.equal(owner.size, 0);
    }
    for (const row of fixture.cases) {
      let generation = BigInt(row.firstGeneration);
      const live = new Map<string, bigint>();
      const owners = new Map<string, { document: string; generation: bigint }>();
      for (const step of row.steps) {
        let actual = "ok";
        const permit = owners.get(step.owner);
        if (step.action === "acquire") {
          if (live.has(step.document)) actual = "conflict";
          else if (generation === 0xffffffffffffffffn) actual = "exhausted";
          else {
            live.set(step.document, generation);
            owners.set(step.owner, { document: step.document, generation: generation++ });
          }
        } else if (step.backend !== 0 || permit?.document !== step.document || live.get(step.document) !== permit?.generation) actual = "fenced";
        else if (step.action === "release") live.delete(step.document);
        assert.equal(actual, step.expected, `${row.name}: ${JSON.stringify(step)}`);
      }
      assert.equal(live.size, 0, row.name);
    }
    assert.deepEqual(
      fixture.resultRetirement.terminal,
      Array.from({ length: fixture.resultRetirement.pages + 3 }, (_, index) => index === fixture.resultRetirement.pages + 2),
    );
    assert.deepEqual(
      fixture.guardRetirement.terminal,
      fixture.guardRetirement.stages.map((stage: string) => stage === "terminal"),
    );
    const rejected = new Set(Array.from({ length: fixture.backendPressure.capacity }, (_, index) => index));
    let retained = rejected.size === fixture.backendPressure.capacity;
    assert.equal(retained, fixture.backendPressure.retainedWhenFull);
    rejected.delete(0);
    if (rejected.size < fixture.backendPressure.capacity) retained = false;
    assert.equal(!retained, fixture.backendPressure.terminalAfterCapacityReturns);
    const closing = new Map([
      [fixture.fairRetirement.pinnedSlot, "pinned"],
      [fixture.fairRetirement.releasingSlot, "releasing"],
    ]);
    for (let cursor = 0; cursor < fixture.fairRetirement.maximumOpportunities; cursor++) if (closing.get(cursor) === "releasing") closing.delete(cursor);
    assert.equal(closing.has(fixture.fairRetirement.pinnedSlot), fixture.fairRetirement.pinnedRetained);
    assert.equal(!closing.has(fixture.fairRetirement.releasingSlot), fixture.fairRetirement.releasingRetired);
    for (const trace of [fixture.maintenanceFairness.continuouslyReady, fixture.maintenanceFairness.firstClassFaults])
      assert.deepEqual(
        trace,
        trace.map((_: unknown, index: number) => index % fixture.maintenanceFairness.classes.length),
      );
    let terminalEpoch = BigInt(fixture.releaseSignal.firstEpoch);
    const waits: bigint[] = [];
    for (const writer of fixture.releaseSignal.writers) {
      const required = terminalEpoch + 1n;
      assert.equal(terminalEpoch >= required, fixture.releaseSignal.requestCompletesRelease);
      terminalEpoch += 1n;
      waits.push(required);
      assert.equal(terminalEpoch.toString(), writer.terminalEpoch);
      assert.equal(
        waits.every((wait) => terminalEpoch >= wait),
        fixture.releaseSignal.oldWaitSurvivesReuse,
      );
    }
    console.log(
      `wal-writer-authority-independent-oracle: AJV=6 exact-u64=1 cases=${fixture.cases.length} mutations=${fixture.mutations.length} fence-laws=${remoteFixture.laws.length} writer-slots=${memoryFixture.writerTable.slots} retained-result=1 directory-barriers=4 wal-open-owner=1 backend-pool-use=${poolUseFixture.cases.length} physical-opening=${poolUseFixture.opening.length}`,
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WalWriterPermit", "struct WalWriterTable", "struct WalFileWriterGuard", "try_lock()", "checked_add(1)", "active_operation", "fn release_step"])
      assert(source.includes(marker), `missing writer capability primitive: ${marker}`);
    const explicitRelease = source.slice(source.indexOf("pub fn release(mut self)"), source.indexOf("fn request_release(&self)"));
    assert(!explicitRelease.includes("self.request_release()"), "explicit retained release must stay dormant until its first poll");
    const storageSource = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    for (const marker of ["WalWriterTable<WalFileWriterGuard>", "fn writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(storageSource.includes(marker), `missing filesystem writer integration: ${marker}`);
    for (const marker of [
      "pool_use: Option<Arc<WorkerPoolUse>>",
      "fn db_io_backend_admit_operation",
      "Result<Arc<WorkerPool>, DbError>",
      "pub fn submit_db_io_task(task: DbIoTask)",
      "db_io_backend_control(owner.kind, slot, generation) != control",
      "struct DbIoBackendRollbackReservation",
      "struct DbIoBackendRegistrationRejected",
      "pub enum DbStorageOpenRejected",
      "pub async fn retry_close(self) -> Result<DbError, Self>",
      "register_db_io_backend_prepared_with_use",
      "Result<DbIoBackendControl, DbIoBackendRegistrationRejected>",
      "reserved: bool",
    ])
      assert(storageSource.includes(marker), `missing backend-owned WorkerPool use boundary: ${marker}`);
    assert(!storageSource.includes("pub fn submit_db_io_task(pool:"), "DB I/O tasks must derive the exact registered backend pool");
    const registrationSource = storageSource.slice(storageSource.indexOf("pub fn register_db_io_backend("), storageSource.indexOf("fn db_io_writer_release_lane_step"));
    assert(!registrationSource.includes("let _ = db_io_park_lost_owner"), "backend registration must not discard a saturated retirement owner");
    for (const marker of ["MemoryDbIoExecutor::backing_bytes()", "checked_add(writer::release::controller_credit())"]) assert(storageSource.includes(marker), `missing memory writer backing integration: ${marker}`);
    const sqliteSource = readFileSync(join(owner, "..", "🪶️sqlite", "🦀️.rs"), "utf8");
    for (const marker of ["WalWriterTable<SqliteWalWriterGuard>", "canonical_database", "fn physical_writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(sqliteSource.includes(marker), `missing SQLite writer integration: ${marker}`);
    const releaseSource = readFileSync(join(owner, "🔔️release/🦀️.rs"), "utf8");
    for (const marker of [
      "struct WalWriterSignalCell",
      "requested: bool",
      "fn request_release(&mut self)",
      "impl Drop for WalWriterRelease",
      "deferred_fault_waiter",
      "defer_fault_notifications",
      "suspend_controller_for_refusal",
      "deferred_wake_pending_for_test",
    ])
      assert(releaseSource.includes(marker), `missing bounded deferred refusal primitive: ${marker}`);
    assert.equal((storageSource.match(/writer::release::notify_faults/g) ?? []).length, 0, "public DB handback paths must not invoke writer wakers directly");
    const retainedFixtureLaws = readFileSync(join(owner, "..", "🧪️tests", "🔬️db-io-retained-fixtures", "🦀️.rs"), "utf8");
    assert(retainedFixtureLaws.includes("fn wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch()"), "missing mounted stale-controller refusal law");
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen",
            "db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry",
            "wal_writer_table_matches_neutral_exact_scope_and_aba_rejection",
            "wal_writer_table_capacity_recycles_slots_without_reusing_generations",
            "wal_writer_file_lock_excludes_independent_instances_and_processes",
            "db_io_lost_result_lease_retains_every_page_and_final_handback",
            "wal_writer_release_retains_pinned_operation_and_faulted_guard",
            "db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure",
            "wal_writer_table_close_advances_other_guards_while_first_operation_is_pinned",
            "db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation",
            "wal_writer_release_signal_preserves_exact_waits_across_writer_and_backend_reuse",
            "wal_writer_mounted_controller_fences_at_signal_and_wakes_outside_registry_without_tasks",
            "wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer",
            "wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch",
            "wal_writer_mounted_controller_rerequests_after_async_executor_handback",
            "wal_writer_mounted_controller_coalesced_fault_does_not_strand_healthy_release",
            "wal_writer_mounted_controller_outer_panic_faults_waiters_once_and_stops",
            "db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return",
            "db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit",
            "fs_storage_canonical_alias_writer_fences_all_six_mutations",
            "sqlite_wal_writer_real_database_alias_and_crash_are_exclusive",
            "fs_wal_directory_barriers_match_neutral_order_and_duplicate_create_is_atomic",
            "fs_wal_directory_faults_retain_seal_and_delete_order_until_explicit_retry",
            "fs_replacement_reports_failure_until_renamed_parent_is_synced",
            "fs_wal_reopen_repairs_unacknowledged_segment_namespace_before_header_ack",
            "replicate_document_fences_occupied_follower_before_inventory_or_up_to_date",
            "replicate_document_releases_follower_after_leader_replay_failure",
            "replicate_document_applies_missing_tail_commands_to_a_fresh_follower",
            "replicate_document_reports_up_to_date_once_a_follower_catches_up",
            "replicate_document_transfers_a_snapshot_when_the_follower_is_below_the_retained_floor",
            "artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry",
            "artifact_engine_create_rejection_propagates_exact_wal_release_owner",
            "database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout",
            "db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close",
            "db_io_backend_registration_saturation_returns_exact_executor_before_pool_use",
            "db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal",
            "db_io_task_uses_registered_backend_pool_not_caller_pool",
            "db_io_backend_drop_retains_pool_until_deferred_close_terminal",
            "db_io_forged_backend_kind_is_rejected_before_task_page_admission",
            "database_compaction_future_acquires_pool_use_before_admission",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log(`wal-writer-authority-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    for (const receipt of receipts) console.log(`wal-writer-authority-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🤺️ The cross-process WAL writer fence laws of `db_storage::writer::fence_conformance`, one lane per backend:
 * `sqlite` on a scratch file; `postgres` and `neo4j` on the ONE shared development server claimed by
 * `os-hub-ts backend run <postgres|neo4j> -- …` (its `OS_HUB_*` environment selects the server, `SEMIO_BACKEND_CLIENT` is the
 * server's own client the laws cross-check the lock through), so a server lane runs only under that claim.
 * `wal-writer-fence-live [sqlite|postgres|neo4j]…`: without a lane, `sqlite` plus every claimed lane whose environment
 * is present. */
const WAL_WRITER_FENCE_LANES: Readonly<Record<string, Readonly<{ law: string; claimed?: string }>>> = {
  sqlite: { law: "db_storage::writer::fence_conformance::sqlite_wal_writer_fence_holds_every_shared_law" },
  postgres: { law: "db_storage::writer::fence_conformance::postgres_wal_writer_fence_holds_every_shared_law", claimed: "OS_HUB_DATABASE_URL" },
  neo4j: { law: "db_storage::writer::fence_conformance::neo4j_wal_writer_fence_holds_every_shared_law", claimed: "OS_HUB_NEO4J_URI" },
};

class WalWriterFenceLiveScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const lanes = segments.length ? segments : Object.keys(WAL_WRITER_FENCE_LANES).filter((name) => !WAL_WRITER_FENCE_LANES[name]!.claimed || process.env[WAL_WRITER_FENCE_LANES[name]!.claimed!]);
    const unknown = lanes.filter((name) => !(name in WAL_WRITER_FENCE_LANES));
    if (unknown.length) throw new Error(`wal-writer-fence-live accepts ${Object.keys(WAL_WRITER_FENCE_LANES).join(" | ")}, got ${unknown.join(",")}`);
    const unclaimed = lanes.filter((name) => WAL_WRITER_FENCE_LANES[name]!.claimed && !process.env[WAL_WRITER_FENCE_LANES[name]!.claimed!]);
    if (unclaimed.length) throw new Error(`wal-writer-fence-live ${unclaimed.join(",")} needs the claimed shared server: run it under \`os-hub-ts backend run ${unclaimed[0]} -- …\``);
    for (const lane of lanes) {
      console.log(`wal-writer-fence-live ${lane}: ${WAL_WRITER_FENCE_LANES[lane]!.law}`);
      await runCargo(["test", "--manifest-path", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml"), "--features", "sqlite,postgres,neo4j", "--lib", "--", "--exact", WAL_WRITER_FENCE_LANES[lane]!.law, "--include-ignored", "--test-threads=1"], this.root);
    }
  }
}

/** 🐘️ `postgres-round-trips-live` — the PostgreSQL storage laws of `db_storage_postgres::round_trips` on the ONE shared
 * development server claimed by `os-hub-ts backend run postgres -- …` (a list is one statement, concurrent writers wait for
 * the backend's operation slot, concurrent openers of a fresh database all find its schema). */
class PostgresRoundTripsLiveScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("postgres-round-trips-live takes no arguments");
    if (!process.env.OS_HUB_DATABASE_URL) throw new Error("postgres-round-trips-live needs the claimed shared server: run it under `os-hub-ts backend run postgres -- …`");
    await runCargo(["test", "--manifest-path", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml"), "--features", "sqlite,postgres", "--lib", "--", "db_storage_postgres::round_trips::", "--include-ignored"], this.root);
  }
}

/** 🧾️ Proves the logical commit firewall with an independent neutral grammar evaluator. */
class WalCommittedTransactionsCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-transactions-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧾️committed-transactions/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.wal", "CommittedTransactionsV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.historyProjection.expected, { entries: 1, operationIds: fixture.historyProjection.commands.map((command: any) => command.id), headSeq: fixture.historyProjection.commands.length, commitSeq: 1 });
    assert.equal(new Set(fixture.historyProjection.expected.operationIds).size, fixture.historyProjection.commands.length);
    for (const row of fixture.historyProjection.rejections) {
      const frontiers = row.records.flatMap((record: any, index: number) => (record.kind === "frontier" ? [index] : []));
      const accepted = row.records.every((record: any) => record.document === "current") && frontiers.length === 1 && frontiers[0] === row.records.length - 1;
      assert.equal(accepted, row.accepted, row.name);
    }
    for (const row of fixture.cases) {
      let next = 1n;
      let observed = false;
      let recoverAbort: string | null = null;
      const transactions: { id: string; kinds: string[] }[] = [];
      let error: string | null = null;
      try {
        for (const [index, segment] of row.segments.entries()) {
          if (index !== row.segments.length - 1 && segment.state !== "sealed") throw "corrupt";
          assert.deepEqual(
            [...segment.physicalCommitsAfter].sort((a: number, b: number) => a - b),
            segment.physicalCommitsAfter,
          );
          const frames = segment.frames.flatMap((frame: any) => Array.from({ length: frame.repeat ?? 1 }, () => frame));
          assert.equal(segment.physicalCommitsAfter.at(-1), frames.length - 1);
          let current: { id: string; kinds: string[] } | null = null;
          for (const [ordinal, frame] of frames.entries()) {
            if (frame.kind === "header") {
              if (ordinal !== 0 || current !== null) throw "corrupt";
              continue;
            }
            if (ordinal === 0) throw "corrupt";
            if (frame.kind === "begin") {
              if (current !== null || BigInt(frame.id) < next || (observed && BigInt(frame.id) !== next)) throw "corrupt";
              const payload = Buffer.alloc(8);
              payload.writeBigUInt64LE(BigInt(frame.id));
              const id = payload.readBigUInt64LE();
              if (id === 0xffffffffffffffffn) throw "sequence";
              next = id + 1n;
              observed = true;
              current = { id: id.toString(), kinds: [] };
            } else if (frame.kind === "commit" || frame.kind === "abort") {
              if (current === null || current.id !== frame.id || (frame.kind === "commit" && current.kinds.length !== frame.count)) throw "corrupt";
              if (frame.kind === "commit") transactions.push(current);
              current = null;
            } else {
              if (current === null) throw "corrupt";
              if (current.kinds.length === fixture.maximumRecords) throw "capacity";
              current.kinds.push(frame.kind);
            }
          }
          if (current !== null) {
            if (segment.state !== "active" || index !== row.segments.length - 1) throw "corrupt";
            recoverAbort = current.id;
          }
        }
      } catch (caught) {
        if (!["corrupt", "capacity", "sequence"].includes(String(caught))) throw caught;
        error = String(caught);
      }
      assert.deepEqual({ accepted: error === null, transactions: error === null ? transactions : [], nextTxId: error === null ? next.toString() : null, recoverAbort: error === null ? recoverAbort : null, error }, row.expected, row.name);
    }
    console.log(`wal-committed-transactions-independent-oracle: AJV=1 u64=1 vectors=${fixture.cases.length}`);
    const faults = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    const validateFaults = ownedExport(this.repoRoot, "db.wal", "FailStopV1");
    assert(validateFaults(faults), JSON.stringify(validateFaults.errors));
    assert.deepEqual(
      faults.cases.filter((row: any) => row.fault !== "successorAppendError").map((row: any) => [row.fault, row.expectedPhysicalSuffix]),
      [
        ["shortAppend", "torn"],
        ["appendError", "absent"],
        ["syncError", "complete"],
      ],
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("struct WalTransactionGate") && source.includes("frames: Vec<WalRecordFrame>") && source.includes("self.frames.len() >= WAL_TRANSACTION_RECORDS_MAX") && source.includes("count > WAL_TRANSACTION_RECORDS_MAX"), "logical admission must retain frame spans (not owned decoded records) up to the writer's own transaction bound");
    assert(source.includes("WalCommittedCursor") && source.includes("WalCommittedTransaction"), "materializers need one shared borrowed committed cursor");
    assert(source.includes("enum WalVerifiedFrameStep"), "verified replay must yield after each physical frame without repeating whole-frame CRC work");
    assert(source.includes("trait WalImmutableByteSource"), "History must share the authenticated frame source without borrowing another field across polls");
    assert(source.includes("struct WalAuthenticatedSource<S>") && source.includes("source: S"), "History authentication and committed spans must retain the same immutable source owner");
    assert(source.includes("recovered_abort_tx_id") && source.includes("wal recovery abort exceeds retained segment budget"), "active recovery must durably abort within the retained segment budget");
    const history = readFileSync(join(owner, "../🗿️artifact/🦀️.rs"), "utf8");
    assert(history.includes("WalAuthenticatedSource<HistoryPageSet>") && !history.includes("struct HistoryFrameCursor"), "History must consume authenticated committed spans, with no independent frame grammar");
    for (const check of ["history envelope document differs", "history frontier document differs", "history committed frontier is not terminal"]) assert(history.includes(check), `History admission is missing ${check}`);
    const decoder = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📖️retained-decoder/🔣️.json"), "utf8"));
    const validateDecoder = ownedExport(this.repoRoot, "db.wal", "RetainedDecoderV1");
    assert(validateDecoder(decoder), JSON.stringify(validateDecoder.errors));
    const { default: leb } = await import("@webassemblyjs/leb128/lib/leb.js");
    for (const row of decoder.varints) {
      const bytes = Buffer.from(row.hex, "hex");
      let value: string | null = null;
      let consumed: number | null = null;
      const terminal = bytes.findIndex((byte) => byte < 128);
      if (terminal >= 0 && terminal < 10) {
        const number = bytes.subarray(0, terminal + 1).reduceRight((value, byte) => value * 128n + BigInt(byte & 127), 0n);
        if (number <= 0xffffffffffffffffn) {
          const storage = Buffer.alloc(8);
          storage.writeBigUInt64LE(number);
          if (Buffer.from(leb.encodeUIntBuffer(storage)).equals(bytes.subarray(0, terminal + 1))) {
            value = number.toString();
            consumed = terminal + 1;
          }
        }
      }
      assert.deepEqual({ value, consumed }, { value: row.value, consumed: row.consumed }, row.name);
    }
    assert(source.includes("fn wal_read_canonical_varint"), "retained readers must reject noncanonical and overflowing u64 fields");
    console.log(`wal-retained-decoder-independent-oracle: AJV=1 LEB128=1 vectors=${decoder.varints.length}`);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            cargoArgs: ["--all-features"],
            laws: [
              "wal_transaction_gate_matches_neutral_committed_spans",
              "wal_retained_decoder_fuel_resumes_exact_fragmented_bytes",
              "wal_retained_decoder_cancel_close_preserves_source_and_returns_owner",
              "wal_committed_cursor_cancel_resume_keeps_transaction_position",
              "wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close",
              "artifact_open_ignores_neutral_aborted_command_snapshot_and_cas",
              "sync_replay_ignores_neutral_aborted_command_snapshot_and_cas",
              "cli_verify_checks_neutral_logical_commit_boundaries",
              "open_replays_the_wal_and_reconstructs_state_and_frontier_identically",
              "wal_retained_varints_match_neutral_exact_u64_and_atomic_interruption",
              "wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions",
              "sync_retained_reads_resume_neutral_varints_without_renewing_overall_deadline",
              "wal_immutable_source_fragmentation_matches_neutral_transactions",
              "artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner",
              "artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources",
              "artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order",
              "wal_recovery_aborts_only_incomplete_active_transactions_idempotently",
              "wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens",
              "wal_recovery_abort_faults_retry_without_duplicate_abort",
              "wal_recovery_abort_cancellation_has_one_durable_boundary",
              "wal_recovery_abort_capacity_exact_and_plus_one_preserves_source",
              "artifact_history_panic_at_each_phase_transition_retains_then_fault_retires",
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
            ],
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) {
          console.log(`wal-committed-transactions-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      for (const receipt of receipts) console.log(`wal-committed-transactions-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧹️ Proves compaction observes only logically committed WAL effects. */
class WalCommittedCompactionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-compaction-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🧾️committed-effects/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.compact", "CommittedEffectsV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(
      fixture.segments.map((row: any) => row.index),
      fixture.segments.map((_: any, index: number) => index),
    );
    assert.equal(fixture.segments.at(-1).state, "active");
    const committed = fixture.segments.flatMap((segment: any) =>
      segment.transactions.filter((transaction: any) => transaction.outcome === "commit").flatMap((transaction: any) => transaction.records.map((record: any) => ({ ...record, segment: segment.index }))),
    );
    const horizons = fixture.segments.map((segment: any) => ({
      segment: segment.index,
      head: committed.filter((record: any) => record.segment === segment.index && ["frontier", "snapshot"].includes(record.kind)).reduce((head: number | null, record: any) => (head === null ? record.headSeq : Math.max(head, record.headSeq)), null),
    }));
    const highest = fixture.segments.at(-1).index;
    const deletedSegments = horizons.filter((row: any) => row.segment !== highest && row.head !== null && row.head <= fixture.floorHeadSeq).map((row: any) => row.segment);
    const deletedPayloads =
      fixture.actor.payloadReclamation === "deferred-global-reference-authority"
        ? []
        : committed
            .filter((record: any) => record.kind === "payload" && deletedSegments.includes(record.segment) && !committed.some((live: any) => live.kind === "payload" && live.payload === record.payload && !deletedSegments.includes(live.segment)))
            .map((record: any) => record.payload);
    const allPayloads = new Set(fixture.segments.flatMap((segment: any) => segment.transactions.flatMap((transaction: any) => transaction.records.filter((record: any) => record.kind === "payload").map((record: any) => record.payload))));
    assert.deepEqual(
      {
        deletedSegments: deletedSegments.length,
        deletedPayloads: new Set(deletedPayloads).size,
        remainingSegments: fixture.segments.map((row: any) => row.index).filter((index: number) => !deletedSegments.includes(index)),
        retainedPayloads: [...allPayloads].filter((payload) => !deletedPayloads.includes(payload)),
      },
      fixture.expected,
    );
    assert.deepEqual(fixture.actor, {
      priority: "command",
      writerAuthority: "retained-artifact-wal",
      activeSegmentAuthority: "artifact-wal",
      leaseBefore: ["snapshot-floor", "wal-horizon", "wal-delete"],
      indexBudgetContinuation: "same-owned-future-cooperative-yield",
      payloadReclamation: "deferred-global-reference-authority",
      queuedSubmitDuring: "pending",
      queuedSubmitAfter: "accepted",
    });
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const committedCut = source.slice(source.indexOf("async fn committed_compaction_horizons"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(committedCut.includes("replay_committed_document") && committedCut.includes("close_record_step") && committedCut.includes("transaction.finish()") && committedCut.includes("close_compaction_replay"));
    assert(source.slice(source.indexOf("async fn close_compaction_replay"), source.indexOf("async fn close_compaction_owner")).includes("close_owner_step"));
    assert(!committedCut.includes("WalReplayCursor") && !committedCut.includes("replay_document"));
    const liveCut = source.slice(source.indexOf("pub async fn retained_compaction_with_wal"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(liveCut.indexOf("CompactionLease::acquire") < liveCut.indexOf("SnapshotFloor"));
    assert(!liveCut.includes("acquire_writer"));
    const underLeaseCut = source.slice(source.indexOf("async fn retained_compaction_under_lease"), source.indexOf("async fn retained_compaction_snapshot"));
    assert(!underLeaseCut.includes("loop {\n            let deadline") && underLeaseCut.includes("handle.compact(&mut control).await"));
    const walSource = readFileSync(join(owner, "..", "📝️wal", "🦀️.rs"), "utf8");
    assert(walSource.includes("delete_compacted_sealed_segment"));
    const artifactSource = readFileSync(join(owner, "..", "🗿️artifact", "🦀️.rs"), "utf8");
    assert(artifactSource.includes("ArtifactMessage::Compact") && artifactSource.includes("Priority::Command"));
    const laws = readFileSync(join(owner, "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    assert(laws.includes("fn compaction_applies_only_committed_frontier_snapshot_and_payload_effects("));
    console.log("wal-committed-compaction-independent-oracle: abort effects excluded, global payloads retained, header-only highest preserved");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
              "db_compact::tests::document_compaction_retains_shared_and_private_cas_without_global_reference_authority",
              "db_engine::tests::compact_document_uses_live_actor_writer_and_restores_submits",
            ],
          },
        ],
        progress(event) {
          console.log(`wal-committed-compaction-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-committed-compaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

/** 🚪️ Proves retained database shutdown keeps exact retry owners across interruption and shared authority. */
class DatabaseShutdownCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("database-shutdown-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🚪️shutdown/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "ShutdownV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(fixture.phases, ["authority", "emit", "complete"]);
    assert.equal(fixture.maximumAuthorityStepsPerTurn, 1);
    for (const row of fixture.cases) {
      let authorityOwners = row.initial.authorityOwners;
      let externalAuthorityOwners = row.initial.externalAuthorityOwners;
      let closing = false;
      let cancelled = false;
      let terminal = false;
      let emitCount = 0;
      let authorityRetained = false;
      for (const action of row.trace) {
        if (action === "cancel") {
          cancelled = true;
          authorityRetained ||= authorityOwners > 0 || closing;
          continue;
        }
        if (action === "retry") {
          cancelled = false;
          continue;
        }
        if (action === "release-shared-owner") {
          externalAuthorityOwners = 0;
          continue;
        }
        if (cancelled || terminal) continue;
        if (authorityOwners > 0) {
          if (externalAuthorityOwners > 0) {
            authorityRetained = true;
          } else if (!closing) {
            closing = true;
          } else {
            closing = false;
            authorityOwners -= 1;
          }
        } else if (emitCount === 0) {
          emitCount = 1;
        } else {
          terminal = true;
        }
      }
      assert.deepEqual({ terminal, authorityRetained, emitCount }, row.expected, row.name);
    }
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const artifact = readFileSync(join(owner, "..", "🗿️artifact", "🦀️.rs"), "utf8");
    assert(source.includes("closing_authority: Option<") && source.includes("pub async fn shutdown_step(&mut self"));
    assert(source.includes("DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities"));
    assert(source.includes("shutdown_emit_started") && !source.includes("shutdown_graph_complete"));
    assert(!source.includes("pub async fn shutdown(self"));
    assert(artifact.includes("pub fn shutdown_step(&self) -> bool") && artifact.includes("handoff.terminal"));
    console.log(`database-shutdown-independent-oracle: AJV=1 cases=${fixture.cases.length} retained-authority=1 terminal-ack=1`);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_engine::tests::database_shutdown_cancellation_preserves_exact_retry_owners",
              "db_engine::tests::database_shutdown_shared_authority_blocks_without_closing_live_handle",
            ],
          },
        ],
        progress(event) {
          console.log(`database-shutdown-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`database-shutdown-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

/** 🪢️ Proves one generation-fenced retained mount owner serves every concurrent document opener. */
class DocumentMountSingleFlightCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("document-mount-single-flight-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🚪️document-mount");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "DocumentMountV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const mountedPoolUse = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔐️pool-use/🔣️.json"), "utf8"));
    const mountedPoolUseSchema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔐️pool-use/🔣️.json"), "utf8"));
    const validateMountedPoolUse = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(mountedPoolUseSchema);
    assert(validateMountedPoolUse(mountedPoolUse), JSON.stringify(validateMountedPoolUse.errors));
    for (const row of mountedPoolUse.cases) {
      const retainedUses = row.externalUses + (row.pool === "open" && (row.database === "open" || row.database === "opening-non-runnable") ? 1 : 0);
      const shutdown = retainedUses ? `busy-${retainedUses}` : "stopped";
      assert.equal(shutdown, row.expectedShutdown, row.id);
      if (row.authority === "ready") assert.equal(row.database, "open", row.id);
      if (row.database === "terminal" || row.database === "absent") assert.equal(row.databaseActivities, "closed", row.id);
    }
    const mountedEngine = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs"), "utf8");
    const mountedArtifact = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"), "utf8");
    const mountedSync = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"), "utf8");
    assert(mountedEngine.includes('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'), "missing actual database pool-use test mount");
    const engineLaws = readFileSync(join(owner, "..", "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    for (const marker of ["pool_use: Option<Arc<WorkerPoolUse>>", "let pool_use = pool.acquire_use()", "fn require_open_use(&self)", "self.pool_use.take()", "DatabaseRetainedActivityRejected::Closed", "DatabaseDocumentMountDriver::NonRunnable", "DatabaseShutdownBlock::Executor(kind)"]) assert(mountedEngine.includes(marker), `missing mounted pool-use marker ${marker}`);
    const catalogStateStart = mountedEngine.indexOf("struct DatabaseCreateCatalogState {");
    const catalogState = mountedEngine.slice(catalogStateStart, mountedEngine.indexOf("\n}", catalogStateStart));
    assert.equal((mountedEngine.match(/_pool_use: Arc<WorkerPoolUse>/g)?.length ?? 0) + Number(catalogState.includes("pool_use: Mutex<Option<Arc<WorkerPoolUse>>>")), 4, "every retained Database capability/catalog state must own the use cell");
    assert(mountedEngine.includes("pool_use: Mutex::new(Some(pool_use))"), "catalog publication must retain its admitted use cell");
    assert(mountedEngine.includes("self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();"), "catalog terminal drain must release its exact admitted use cell");
    for (const marker of ["_pool_use: Arc<semio_framework_async::WorkerPoolUse>", "spawn_with_pool_use", "pool.acquire_use()"] ) assert(mountedArtifact.includes(marker), `missing authority pool-use marker ${marker}`);
    for (const marker of ["_pool_use: std::sync::Arc<semio_framework_async::WorkerPoolUse>", "let pool_use = match pool.acquire_use()"] ) assert(mountedSync.includes(marker), `missing sync-hello pool-use marker ${marker}`);
    for (const law of ["database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack", "database_worker_pool_use_is_admitted_before_the_first_storage_probe", "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer"]) assert(engineLaws.includes(`fn ${law}(`), `missing mounted pool-use law ${law}`);
    console.log(`[DEBUG] database-mounted-pool-use-independent-oracle: AJV=1 cases=${mountedPoolUse.cases.length} retained-source-owners=3`);
    for (const row of fixture.cases) {
      let activeGeneration: bigint | undefined;
      let waiters = 0;
      let ready = false;
      let terminalCleanup = true;
      let emitting = false;
      let events = 0;
      let fanoutPrepared = false;
      let registryUnlocked = true;
      let ownerPollActive = false;
      for (const step of row.steps) {
        const generation = BigInt(step.generation);
        if (step.action === "install") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          ready = false;
        } else if (step.action === "join") {
          assert.equal(generation, activeGeneration, row.name);
          waiters += 1;
        } else if (step.action === "cancel-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "catalog-published" || step.action === "shutdown-interrupted") {
          assert.equal(generation, activeGeneration, row.name);
        } else if (step.action === "emit-begin") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!emitting && events === 0, row.name);
          emitting = true;
        } else if (step.action === "emit-complete") {
          assert.equal(generation, activeGeneration, row.name);
          assert(events === 0, row.name);
          emitting = false;
          events = 1;
        } else if (step.action === "fill-waiters") {
          assert.equal(generation, activeGeneration, row.name);
          waiters = fixture.capacity.waitersPerDocument;
        } else if (step.action === "reject-over-capacity") {
          assert.equal(generation, activeGeneration, row.name);
          assert.equal(waiters, fixture.capacity.waitersPerDocument, row.name);
        } else if (step.action === "release-slot") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "close-fault") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = false;
        } else if (step.action === "resume-cleanup") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "explicit-retirement-retry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "close-terminal") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = true;
          activeGeneration = undefined;
        } else if (step.action === "prepare-fanout") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = true;
          registryUnlocked = false;
        } else if (step.action === "unlock-registry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && !registryUnlocked, row.name);
          registryUnlocked = true;
        } else if (step.action === "wake-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = false;
        } else if (step.action === "owner-poll-active") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!ownerPollActive, row.name);
          ownerPollActive = true;
        } else if (step.action === "request-drive") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
        } else if (step.action === "owner-poll-release") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
          ownerPollActive = false;
        } else if (step.action === "retry") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          events = 0;
        } else if (step.action === "ready") {
          assert.equal(generation, activeGeneration, row.name);
          assert(terminalCleanup && !emitting && events === 1, row.name);
          ready = true;
        }
      }
      assert.equal(ready ? 1 : 0, row.expected.actors, row.name);
      assert.equal(row.expected.owners, 1, row.name);
      assert.equal(events, row.expected.events, row.name);
      assert.equal(waiters, row.expected.waitersReleased, row.name);
      assert.equal(terminalCleanup, row.expected.terminalBeforeFanout, row.name);
      assert(!fanoutPrepared && registryUnlocked && !ownerPollActive, row.name);
    }
    for (const row of fixture.driverCases) {
      let state = "idle";
      let wakeRequested = false;
      let resumeRequested = false;
      let queued = 0;
      let maximumQueued = 0;
      let cleanupParked = false;
      let cleanupResumes = 0;
      for (const step of row.steps) {
        if (step === "request" || step === "request-resume") {
          wakeRequested = true;
          resumeRequested ||= step === "request-resume";
          if (state === "idle") {
            state = "queued";
            queued = 1;
            maximumQueued = Math.max(maximumQueued, queued);
          }
        } else if (step === "poll-start") {
          assert.equal(state, "queued", row.name);
          state = "polling";
          queued = 0;
          wakeRequested = false;
        } else if (step === "cleanup-fault") {
          assert.equal(state, "polling", row.name);
          cleanupParked = true;
        } else if (step === "cleanup-resume") {
          assert(cleanupParked && resumeRequested, row.name);
          cleanupParked = false;
          resumeRequested = false;
          cleanupResumes += 1;
        } else if (step === "hard-submit-rejected") {
          assert.equal(state, "queued", row.name);
          state = "non-runnable";
        } else if (step === "poll-pending") {
          assert.equal(state, "polling", row.name);
          if (wakeRequested) wakeRequested = false;
          else state = "idle";
        } else if (step === "complete") {
          state = "terminal";
        }
      }
      assert.equal(state, row.expected.state, row.name);
      assert.equal(maximumQueued, row.expected.maximumQueuedPollers, row.name);
      assert.equal(cleanupResumes, row.expected.cleanupResumes, row.name);
    }
    const source = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum DatabaseDocumentMountSlot",
      "Opening {",
      "struct DatabaseDocumentMountOwner",
      "DatabaseDocumentMountWork",
      "DATABASE_DOCUMENT_MOUNT_WAITERS",
      "fn take_generation",
      "pub async fn ensure_document",
      "retained_mount_rejection",
      "DatabaseDocumentMountWait",
      "impl Drop for DatabaseDocumentMountWait",
    ])
      assert(source.includes(marker), `missing retained document-mount marker: ${marker}`);
    assert(!source.includes("open_artifacts: Mutex<HashMap<String, Arc<db_artifact::ArtifactAuthority>>>") && source.includes("open_artifacts: Arc<Mutex<DatabaseDocumentMountRegistry>>"));
    const retainedMount = source.slice(source.indexOf("async fn run_document_mount("), source.indexOf("async fn mount_document("));
    assert(retainedMount.indexOf("emit.emit(") >= 0 && retainedMount.indexOf("emit.emit(") < retainedMount.indexOf("Ok(DatabaseDocumentMountReply"));
    assert(!source.slice(source.indexOf("struct DatabaseDocumentMountReply"), source.indexOf("struct DatabaseDocumentMountWaiter")).includes("emission:"));
    const complete = source.slice(source.indexOf("fn complete(&self, result: Result<DatabaseDocumentMountReply, DbError>)"), source.indexOf("//#region 🔖️Database"));
    assert(complete.includes("let mut fanout: [Option<(") && complete.includes("DATABASE_DOCUMENT_MOUNT_WAITERS"));
    assert(complete.indexOf("drop(authority);") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(complete.indexOf("self.terminal.store(true") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    const registryScope = complete.slice(complete.indexOf("let mut registry = registry.lock()"), complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(registryScope.trimEnd().endsWith("self.terminal.store(true, Ordering::Release);\n        }"), "waiters must be woken only after the registry guard's scope has closed");
    const mountOwner = source.slice(source.indexOf("impl DatabaseDocumentMountOwner"), source.indexOf("//#region 🔖️Database"));
    assert(
      mountOwner.includes("DatabaseDocumentMountDriver::Idle") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Queued") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Polling") &&
        mountOwner.includes("DatabaseDocumentMountDriver::NonRunnable"),
    );
    assert(mountOwner.includes("resume_requested") && mountOwner.includes("wake_requested"));
    assert(mountOwner.includes("Arc::downgrade(self)") && !mountOwner.includes("let owner = self.clone();\n        self.submit_exact"));
    const databaseOpen = source.slice(source.indexOf("async fn open_with("), source.indexOf("fn document_engine_config("));
    assert.equal(databaseOpen.match(/acquire_use\(\)/g)?.length, 1);
    for (const marker of ["DatabaseCapabilityOpenFuture::try_prepare_with_use", "DatabaseCatalogReadFuture::try_prepare_with_use", "DatabaseCatalogBootstrapFuture::try_prepare_with_use"])
      assert(databaseOpen.includes(marker), `database open minted an untracked pool use instead of retaining ${marker}`);
    const catalogPublication = source.slice(source.indexOf("async fn publish_mount_catalog("), source.indexOf("async fn run_open_document_mount("));
    assert(catalogPublication.includes("DatabaseCreateCatalogFuture::try_prepare_with_use(pool, pool_use"));
    const catalogDriveStart = source.indexOf("fn drive_one(self: Arc<Self>, generation: u64)", source.indexOf("//#region 🔖️CreateDocumentCatalogCas"));
    const catalogDrive = source.slice(catalogDriveStart, source.indexOf("fn drive_claimed(self: &Arc<Self>, generation: u64)", catalogDriveStart));
    assert(!catalogDrive.includes("release_success()"));
    const catalogTerminal = source.slice(source.indexOf("pub struct DatabaseCreateCatalogTerminalHandle"), source.indexOf("pub fn take_database_create_catalog_terminal"));
    assert(catalogTerminal.includes("drive_explicit_close_one()"), "explicit catalog terminal cleanup must make one bounded turn without a wall-clock callback");
    const helloStart = source.indexOf("pub fn hello_retained(");
    const hello = source.slice(helloStart, source.indexOf("pub async fn hello(", helloStart));
    assert(hello.includes("DatabaseSyncHelloFuture::try_submit_with_use") && !hello.includes("DatabaseSyncHelloFuture::try_submit("));
    const requestDrive = mountOwner.slice(mountOwner.indexOf("fn request_drive(self: &Arc<Self>"), mountOwner.indexOf("fn resume_parked("));
    assert(!requestDrive.includes("self.work.try_lock()") && !requestDrive.includes("self.work.lock()"));
    const artifact = readFileSync(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum ArtifactRunnerDriver",
      "RunnableIdle",
      "Queued",
      "PollingWake",
      "Parked",
      "ClosingReady",
      "ClosingPollingWake",
      "ClosingParked",
      "Terminal",
      "fn park_terminal_job",
      "struct ArtifactRunnerClosePoll",
      "struct ArtifactRunnerPoll",
      "struct ArtifactRunnerRetirementReservation",
      "WorkerMaintenanceStep::Retire",
      "impl Drop for ArtifactAuthority",
      "pub fn close(mut self) -> Result<(), Self>",
      "pub fn resume(mut self) -> Result<(), Self>",
    ]) {
      assert(artifact.includes(marker), `missing artifact terminal-authority marker: ${marker}`);
    }
    for (const marker of ["close_error", "WorkerMaintenanceStep::Fault"]) {
      assert(artifact.includes(marker), `missing retained artifact close-fault marker: ${marker}`);
    }
    const artifactLaws = readFileSync(join(owner, "..", "..", "🗿️artifact", "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    for (const law of [
      "artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal",
      "artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission",
      "artifact_engine_close_fault_cancel_stops_the_timer_until_readmission",
    ]) {
      assert(artifactLaws.includes(`fn ${law}(`), `missing retained artifact close-fault law ${law}`);
    }
    const artifactSchedule = artifact.slice(artifact.indexOf("fn schedule(self: &Arc<Self>)"), artifact.indexOf("fn submit_exact(self: &Arc<Self>"));
    assert(artifactSchedule.includes("compare_exchange") && artifactSchedule.includes("ArtifactRunnerDriver::RunnableIdle as u8") && artifactSchedule.includes("ArtifactRunnerDriver::Queued as u8"));
    assert(!artifactSchedule.includes("scheduled.compare_exchange"));
    const observe = readFileSync(join(owner, "..", "..", "👁️observe", "🦀️.rs"), "utf8");
    assert(observe.includes("fn emit(&self, event: EmitEvent) -> impl Future<Output = ()> + Send;"));
    const hub = readFileSync(join(this.repoRoot, "🌎️hub/🏗️bootstrap/🦀️.rs"), "utf8");
    const ensure = hub.slice(hub.indexOf("async fn ensure_document(&self"), hub.indexOf("fn bearer("));
    assert(ensure.includes("self.db.ensure_document(id).await") && ensure.includes("rejected.retry_close().await"));
    assert(!ensure.includes("self.db.create_document") && !ensure.includes("self.db.document(id)"));
    for (const law of [
      "database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
      "database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
      "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
    ]) {
      assert(engineLaws.includes(`fn ${law}(`), `missing exact mount driver law ${law}`);
    }
    console.log(`document-mount-single-flight-independent-oracle: AJV=1 cases=${fixture.cases.length} waiters=${fixture.capacity.waitersPerDocument} owner-futures=${fixture.capacity.ownerFuturesPerDocument}`);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_engine::tests::database_concurrent_ensure_mounts_one_actor_and_one_writer",
            "db_engine::tests::database_published_opening_joins_without_actor_overwrite",
            "db_engine::tests::database_cancelled_ensure_waiter_does_not_cancel_mount_owner",
            "db_engine::tests::database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation",
            "db_engine::tests::database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation",
            "db_engine::tests::database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot",
            "db_engine::tests::database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff",
            "db_engine::tests::database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready",
            "db_engine::tests::database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume",
            "db_engine::tests::database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
            "db_engine::tests::database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
            "db_engine::tests::database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack",
            "db_engine::tests::database_worker_pool_use_is_admitted_before_the_first_storage_probe",
            "db_engine::tests::database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
            "db_engine::tests::database_create_catalog_resolved_drop_retains_use_until_terminal_drain",
            "db_artifact::tests::artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume",
            "db_artifact::tests::artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn",
            "db_artifact::tests::artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake",
            "db_artifact::tests::artifact_runner_terminal_resume_refusal_returns_exact_cursor_for_close",
            "db_artifact::tests::artifact_authority_drop_transfers_parked_terminal_job_to_registered_close_owner",
            "db_artifact::tests::artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry",
            "db_artifact::tests::artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal",
            "db_artifact::tests::artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission",
            "db_artifact::tests::artifact_engine_close_fault_cancel_stops_the_timer_until_readmission",
            "db_artifact::tests::more_live_authorities_than_pool_maintenance_hooks_retire_through_one_shared_hook",
          ],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 180_000,
      progress(event) {
        console.log(`document-mount-single-flight-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`document-mount-single-flight-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🗺️ Proves the fixed owned Map decision envelope with independent AJV and SHA-256 oracles. */
class DurableOwnedGroupDecisionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-owned-group-decision-check accepts only --native");
    const { testDurableOwnedGroupDecisionFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts");
    testDurableOwnedGroupDecisionFixture();
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [
          {
            package: "semio-framework-os-kernel",
            target: { kind: "lib" },
            laws: [
              "durable_group::tests::durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds",
              "durable_group::tests::durable_group_journal_record_projects_edits_only_after_all_three_bound_outcomes_verify",
              "durable_group::tests::durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes",
              "durable_group::tests::durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity",
              "durable_group::tests::durable_decision_rejects_deflate_expansion_before_document_body_allocation",
              "durable_group::tests::durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners",
              "durable_group::tests::durable_store_private_committed_record_recovers_all_three_stores_without_reappending_journal",
              "durable_group::tests::durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess",
              "durable_group::tests::durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once",
              "durable_group::tests::durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots",
              "durable_group::tests::durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty",
              "durable_group::tests::durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change",
              "durable_group::tests::durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence",
              "durable_group::tests::durable_map_fixed_host_slot_retains_every_live_owner_across_request_error_until_terminal_handoff",
              "durable_group::tests::durable_map_fixed_host_slot_cancellation_after_uncertain_io_waits_for_trusted_absence",
            ],
          },
        ],
        artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
        buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
        listBudgetMs: 60_000,
        lawBudgetMs: 120_000,
        progress(event) {
          console.log(`durable-owned-group-decision-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      for (const receipt of receipts) console.log(`durable-owned-group-decision-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧾️ Proves the typed Store-decision journal boundary and exact physical WAL reservation. */
class DurableGroupJournalCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-group-journal-check accepts only --native");
    const artifactOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact");
    const fixtureOwner = join(artifactOwner, "🧫️fixtures/📓️durable-group-journal");
    const fixture = JSON.parse(readFileSync(join(fixtureOwner, "🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.artifact", "DurableGroupJournalV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
    for (const row of fixture.committedDecisionWitnessCases) {
      const events = row.recordKinds.filter((kind: string) => kind === "event").length;
      const expected = row.transaction === "aborted" || events === 0 ? "ignored" : row.replayDocument === "foreign" || events !== 1 || row.recordKinds.length !== 1 ? "rejected" : "witness";
      assert.equal(row.expected, expected);
    }
    assert.deepEqual(fixture.committedRecovery.authority, {
      consumer: "db-wal-witness",
      recordEscape: false,
      receiptSource: "derived",
      cancelAfterCommit: false,
      errorSurface: "retaining-fault",
      terminalHandoff: "stores-and-ack-once",
    });
    assert.equal(new Set(fixture.committedRecovery.cases.map((row: any) => row.id)).size, fixture.committedRecovery.cases.length);
    for (const row of fixture.committedRecovery.cases) {
      const expected = row.frontier === "base" ? ["complete", 3] : row.frontier === "post" ? ["already-applied", 0] : ["fault", 0];
      assert.deepEqual([row.expected, row.mutations], expected);
      assert.equal(row.returnedStoreOwners, 3);
    }
    const storeOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group");
    const storeFixture = JSON.parse(readFileSync(join(storeOwner, "🧫️fixtures/🔣️.json"), "utf8"));
    assert.equal(createHash("sha256").update(storeFixture.expected.unsignedJson).digest("hex"), fixture.record.decisionSha256);
    assert.equal(storeFixture.expected.anchorSha256, fixture.record.anchorSha256);
    const varintBytes = (value: number) => {
      let bytes = 1;
      while (value >= 128) {
        value = Math.floor(value / 128);
        bytes += 1;
      }
      return bytes;
    };
    const frameBytes = (payload: number) => varintBytes(payload + 2) + payload + 10;
    const fieldBytes = (value: string) => varintBytes(Buffer.byteLength(value)) + Buffer.byteLength(value);
    const successorHeaderBytes = 32 + frameBytes(fieldBytes(fixture.record.document) + 41) + 75;
    const transactionBytes = (eventBytes: number) => frameBytes(8) + frameBytes(eventBytes) + frameBytes(12) + 75;
    assert.equal(successorHeaderBytes, fixture.limits.successorHeaderBytes);
    assert.equal(transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumTransactionBytes);
    assert.equal(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumSegmentBytes);
    let maximumEventBytes = fixture.limits.walSegmentBytes;
    while (successorHeaderBytes + transactionBytes(maximumEventBytes) > fixture.limits.walSegmentBytes) maximumEventBytes -= 1;
    assert.equal(maximumEventBytes, fixture.limits.maximumEventBytes);
    assert(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes) <= fixture.limits.walSegmentBytes);
    assert(successorHeaderBytes + transactionBytes(maximumEventBytes + 1) > fixture.limits.walSegmentBytes);
    const storeSource = readFileSync(join(storeOwner, "🦀️.rs"), "utf8");
    const artifactSource = readFileSync(join(artifactOwner, "🦀️.rs"), "utf8");
    const walSource = readFileSync(join(artifactOwner, "../📝️wal/🦀️.rs"), "utf8");
    const engineSource = readFileSync(join(artifactOwner, "../⚙️engine/🦀️.rs"), "utf8");
    assert(storeSource.includes("pub struct DurableOwnedGroupJournalRecordV1") && storeSource.includes("DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&canonical_pack)"));
    assert(storeSource.includes("pub enum DurableOwnedGroupJournalAdvanceV1") && storeSource.includes("Rejected(String)"));
    const append = artifactSource.slice(artifactSource.indexOf("async fn append_durable_group_decision("), artifactSource.indexOf("pub(crate) async fn compact_retained", artifactSource.indexOf("async fn append_durable_group_decision(")));
    assert(append.includes("WalRecord::Event") && append.includes("DurabilityClass::Fsync"));
    assert(append.indexOf("preflight_submit") < append.indexOf("self.wal.submit"));
    assert(append.includes("ArtifactDurableGroupJournalAppendV1::Absent") && append.includes("ArtifactDurableGroupJournalAppendV1::Rejected"));
    const witness = artifactSource.slice(artifactSource.indexOf("struct ArtifactCommittedDurableGroupDecisionV1"), artifactSource.indexOf("//#endregion 🔖️Receipt"));
    const lowerRecovery = storeSource.slice(
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryHostV1"),
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryHostV1"),
    );
    assert(witness.includes("WalCommittedTransaction") && witness.includes("record_count != 1") && witness.includes("transaction.finish()?"));
    for (const marker of [
      "ArtifactCommittedDurableGroupRecoveryV1",
      "ArtifactCommittedDurableGroupRecoveryStateV1::Admitting",
      "ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault",
      "into_store_owned_recovery",
      "restore_untrusted_committed_record",
      "acknowledge_restoration(&receipt)",
      "take_terminal",
      "take_rejected_terminal",
    ])
      assert(witness.includes(marker), `missing committed Store recovery marker ${marker}`);
    assert(!witness.includes("pub fn begin_store_owned_recovery"));
    assert(!witness.includes("fn into_record(") && !witness.includes("fn record(&self)"), "a committed WAL witness has no raw record escape");
    assert(!witness.includes("fn cancel("), "committed Store recovery remains non-cancellable after WAL visibility");
    assert(
      lowerRecovery.includes("pub fn advance(&mut self, grant: super::ArtifactStoreOneItemGrant) -> DurableOwnedMapRecoveryAdvanceV1") && lowerRecovery.includes("DurableOwnedMapRecoveryAdvanceV1::Fault(error)"),
      "lower Store recovery faults must retain their exact host instead of exposing Result/? owner loss",
    );
    assert(lowerRecovery.includes("pub fn capture_snapshot(&self) -> Option<"), "lower Store recovery observation must not expose a Result/? owner-loss path");
    const sink = artifactSource.slice(artifactSource.indexOf("struct ArtifactDurableGroupJournalSinkV1"), artifactSource.indexOf("type ArtifactBuildFuture"));
    assert(sink.includes("NotSubmitted") && sink.includes("Awaiting") && sink.includes("Failed") && sink.includes("Committed") && sink.includes("terminal_is_empty"));
    assert(!sink.includes("block_on"));
    assert(walSource.includes("pub(crate) fn preflight_submit(&self, commands: &[Vec<u8>], records: &WalRecordBatch)") && walSource.includes("wal transaction exceeds readable segment"));
    assert(engineSource.includes("pub fn durable_group_journal_sink(&self, now_ms: u64)"));
    const laws = [
      "db_artifact::tests::document_authority_durable_group_journal_commits_one_exact_fsync_event",
      "db_artifact::tests::committed_durable_group_decision_accepts_only_one_exact_event_transaction",
      "db_artifact::tests::committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection",
      "db_artifact::tests::document_authority_durable_group_journal_cancellation_before_handoff_is_absent",
      "db_artifact::tests::document_authority_durable_group_journal_rejects_hash_before_mailbox",
    ];
    for (const law of laws) assert(artifactSource.includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    console.log(
      `durable-group-journal-independent-oracle: AJV=1 cases=${fixture.cases.length} witnesses=${fixture.committedDecisionWitnessCases.length} recovery=${fixture.committedRecovery.cases.length} max-event=${maximumEventBytes} store-margin=${fixture.limits.walSegmentBytes - fixture.limits.storeMaximumSegmentBytes}`,
    );
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 180_000,
      progress(event) {
        console.log(`durable-group-journal-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`durable-group-journal-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🚑️ Proves exact WAL commit boundaries with independent CRC/LEB128 and schema oracles. */
class WalRecoveryCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-recovery-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🚑️recovery/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.wal", "TailOnlyRecoveryV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const failStop = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    const validateFailStop = ownedExport(this.repoRoot, "db.wal", "FailStopV1");
    assert(validateFailStop(failStop), JSON.stringify(validateFailStop.errors));
    assert.deepEqual(
      failStop.cases.map((row: any) => [row.name, row.fault, row.expectedPhysicalSuffix]),
      [
        ["short-append", "shortAppend", "torn"],
        ["append-error", "appendError", "absent"],
        ["sync-error", "syncError", "complete"],
        ["successor-append-error", "successorAppendError", "complete"],
      ],
    );
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb = await import("@webassemblyjs/leb128");
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const checksum = (bytes: Uint8Array) => crc.buf(bytes) >>> 0;
    const fragmented = Buffer.from(Array.from({ length: 49152 }, (_, index) => (index * 17 + 3) % 251));
    for (const row of fixture.fragmentCopies) assert.equal(checksum(fragmented.subarray(row.offset, row.offset + row.length)), row.crc32c);
    const hash = (bytes: Buffer) => Buffer.from(blake3Hex(bytes), "hex");
    const u64 = (value: number) => {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64LE(BigInt(value));
      return bytes;
    };
    const frame = (kind: number, payload: Buffer) => {
      const body = Buffer.concat([Buffer.from([kind, 2]), payload]);
      const size = Buffer.from(leb.encodeU32(body.length));
      const trailer = Buffer.alloc(8);
      trailer.writeUInt32LE(checksum(body));
      trailer.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, trailer]);
    };
    const header = Buffer.alloc(32);
    Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
    header.writeUInt16LE(1, 8);
    header.writeUInt32LE(1, 12);
    header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    let bytes = header;
    let chain = hash(header);
    let previousOffset = 0;
    const batches = [
      [frame(64, Buffer.concat([Buffer.from([1, 100]), u64(0), Buffer.from([0])]))],
      ...fixture.commands.map((command: string, index: number) => [frame(65, u64(index + 1)), frame(68, Buffer.from(command)), frame(66, Buffer.concat([u64(index + 1), Buffer.from([1, 0, 0, 0])]))]),
    ];
    for (const [index, records] of batches.entries()) {
      const payload = Buffer.alloc(64);
      const recordsLength = records.reduce((sum: number, value: Buffer) => sum + value.length, 0);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      payload.writeBigUInt64LE(BigInt(index + 1));
      payload.writeBigUInt64LE(BigInt(previousOffset), 8);
      payload.writeBigUInt64LE(BigInt(recordsLength), 16);
      payload.writeUInt32LE(records.length, 24);
      chain.copy(payload, 32);
      previousOffset = bytes.length + recordsLength;
      bytes = Buffer.concat([bytes, ...records, frame(12, payload)]);
      assert.equal(bytes.length, fixture.commitEnds[index]);
    }
    for (const row of fixture.cuts) {
      const prefix = bytes.subarray(0, row.cut);
      const span = row.cut < 32 ? { end: 0, sequence: 0 } : inspectRetainedSprNeutral(prefix, checksum, hash);
      assert.equal(span.end, row.trustedEnd);
      assert.equal(Math.max(129, span.end), row.recoveredEnd);
      assert.equal(Math.max(1, span.sequence), row.nextTxId);
    }
    const expectedAccepted = new Set(["missing", "highest-sealed", "successor-empty", "successor-partial", "successor-header", "compacted-clean"]);
    assert.equal(new Set(fixture.lifecycle.map((row: any) => row.name)).size, fixture.lifecycle.length);
    for (const row of fixture.lifecycle) assert.equal(row.accepted, expectedAccepted.has(row.name), row.name);
    console.log(`wal-recovery-independent-oracle: ${fixture.cuts.length} exact CRC/hash-chain prefixes, ${fixture.lifecycle.length} lifecycle rows`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const open = source.slice(source.indexOf("pub async fn open(storage:", source.indexOf("impl ArtifactWal")), source.indexOf("pub async fn document(&self)", source.indexOf("impl ArtifactWal")));
    assert(!open.includes("delete_segment"), "recovery must not delete a committed segment");
    assert(source.includes("resume_verified") && source.includes("RetainedSprVerification"), "full verification and exact writer resume are required");
    const replayClose = source.slice(
      source.indexOf("pub fn close_owner_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
      source.indexOf("pub async fn close_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
    );
    assert(replayClose.includes("pages.close_step()") && replayClose.includes("segments.close_step()"), "replay close must retire retained page and list owners");
    assert(!replayClose.includes("control.grant()"), "terminal replay close must remain available after cancellation");
    const artifactClose = source.slice(source.indexOf("pub fn close_step(&mut self)", source.indexOf("impl ArtifactWal")), source.indexOf("//#endregion 🔖️ArtifactWal"));
    assert(artifactClose.includes("self.active.close_step()") && artifactClose.includes("self.active.terminal_is_empty()"), "artifact WAL must expose explicit terminal owner retirement");
    const segmentClose = source.slice(source.indexOf("fn close_step(&mut self)", source.indexOf("impl SegmentWriter")), source.indexOf("//#endregion 🔖️Segment"));
    assert(segmentClose.indexOf("self.writer.take()") < segmentClose.indexOf("buf.close_step()"), "segment close must relinquish the retained writer before retiring its page buffer");
    assert(segmentClose.includes("force_flush is required before close"), "segment close must reject pending records");
    const segmentFlush = source.slice(source.indexOf("async fn commit_and_flush", source.indexOf("impl SegmentWriter")), source.indexOf("async fn tip_chain_hash", source.indexOf("impl SegmentWriter")));
    assert(segmentFlush.includes("new_len != expected_len") && segmentFlush.includes("self.flushed_len = new_len"), "WAL flush must verify the exact appended length before acknowledging it");
    assert(segmentFlush.indexOf("self.flushed_len = new_len") < segmentFlush.indexOf("storage.sync"), "a failed sync must retain knowledge that its append already landed");
    assert(segmentFlush.includes("self.poison()"), "every uncertain post-commit failure must poison the live writer");
    const rotate = source.slice(source.indexOf("async fn rotate", source.indexOf("impl ArtifactWal")), source.indexOf("pub fn close_step", source.indexOf("impl ArtifactWal")));
    assert(rotate.indexOf("storage.seal") < rotate.indexOf("self.active.poison()"), "a sealed segment must poison its old live writer before successor creation");
    const laws = [
      "db_wal::tests::wal_recovery_preserves_neutral_committed_prefixes",
      "db_wal::tests::wal_recovery_matches_neutral_lifecycle_without_prefix_replacement",
      "db_wal::retained_tests::wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty",
      "db_wal::retained_tests::artifact_wal_repeated_open_close_is_page_budget_neutral",
      "db_wal::retained_tests::artifact_wal_close_rejects_pending_records_and_closed_writes",
      "db_wal::retained_tests::artifact_wal_short_append_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_append_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_sync_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_successor_failure_after_seal_is_fail_stop_until_reopen",
      "db_fault_testing::tests::fault_storage_fail_nth_sync_fails_once_after_the_preceding_append",
    ];
    laws.push(
      ...[
        "single_segment_write_commit_flush_recovers_cleanly",
        "group_commit_batches_until_policy_threshold_then_commits",
        "fsync_durability_forces_immediate_commit_regardless_of_policy",
        "torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix",
        "recovery_resumes_next_tx_id_and_accepts_further_submits",
        "multi_segment_rotation_chains_prev_hash_and_replay_spans_segments",
        "recovery_rejects_a_torn_non_active_sealed_segment",
        "empty_document_open_creates_a_fresh_wal",
      ].map((law) => `db_wal::tests::${law}`),
    );
    const faultStorageLawsSource = readFileSync(join(owner, "../🧪️tests/🧯️fault-storage-laws/🦀️.rs"), "utf8");
    for (const law of laws) assert((law.startsWith("db_fault_testing::") ? faultStorageLawsSource : source).includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws }],
        progress(event) {
          console.log(`wal-recovery ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-recovery-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-recovery-check: ${fixture.cuts.length + fixture.lifecycle.length + fixture.fragmentCopies.length + failStop.cases.length} checks clean`);
  }
}

/** 📏️ Proves complete transaction reservations fit the shared readable storage span. */
class WalCapacityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-capacity-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📏️capacity/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.wal", "SegmentCapacityV1");
    assert(validate(fixture), JSON.stringify(validate.errors));
    const leb = await import("@webassemblyjs/leb128");
    const frame = (payload: number) => leb.encodeU32(payload + 2).length + payload + 10;
    const txn = (payload: number) => frame(8) + frame(payload) + frame(12);
    for (const row of fixture.cases) {
      const lengths = [129];
      const segments: number[] = [];
      let pending = false;
      for (let index = 0; index < 3; index++) {
        if (lengths.at(-1)! + txn(fixture.payloadBytes) + 75 > fixture.maxSegmentBytes) {
          if (pending) lengths[lengths.length - 1] += 75;
          lengths.push(161);
          pending = false;
        }
        segments.push(lengths.length - 1);
        lengths[lengths.length - 1] += txn(fixture.payloadBytes);
        pending = true;
        if (row.durability === "fsync") {
          lengths[lengths.length - 1] += 75;
          pending = false;
        }
      }
      if (pending) lengths[lengths.length - 1] += 75;
      assert.deepEqual(segments, row.segments);
      assert.deepEqual(lengths, row.lengths);
    }
    assert.equal(129 + txn(fixture.exactPayloadBytes) + 75, fixture.maxSegmentBytes);
    assert.equal(129 + txn(fixture.oversizedPayloadBytes) + 75, fixture.maxSegmentBytes + 1);
    console.log("wal-capacity-independent-oracle: Fsync/grouped rotation and exact/one-over capacity confirmed by LEB128");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("fn wal_transaction_frame_bytes("), "missing transaction byte preflight before writes");
    assert(source.includes("const DEFAULT_MAX_SEGMENT_BYTES: u64 = db_storage::DB_IO_MAX_READ_BYTES;"), "WAL and storage must share one byte ceiling");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws: ["db_wal::tests::wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries"] }],
        progress(event) {
          console.log(`wal-capacity ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-capacity-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log("wal-capacity-check: 6 checks clean");
  }
}

//#region 🔎️ScalarWireSource
class ScalarWireSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { testScalarRecordWireFixture } = await import("../../🔨️modules/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts");
    testScalarRecordWireFixture();
  }
}
//#endregion 🔎️ScalarWireSource

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}

/** 🏛️ Verifies shared policy and mutation publication authority through exact native laws. */
class CanonicalArchitectureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("canonical-architecture accepts no arguments");
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-os-kernel",
        target: { kind: "lib", name: "semio_framework_os_kernel" },
        laws: [
          "plugin_module_schema_exports_match_declared_formats",
          "declared_access_policy_matches_the_language_neutral_truth_table",
          "access_policy_is_closed_by_default_and_deny_overrides_allow",
          "one_item_publication_fixture_matches_the_third_party_json_oracle",
          "artifact_snapshot_root_is_o1_and_generation_stable_until_the_next_event",
          "presence_local_read_is_o1_and_never_clones_the_payload_at_capture",
          "transient_root_is_o1_and_retains_the_exact_pre_reset_value",
          "artifact_store_batch_publication_of_one_mutation_is_the_single_item_case",
          "artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step",
          "artifact_store_batch_cancel_mid_flight_retires_every_staged_owner_without_publishing",
          "artifact_store_one_item_digest_helper_matches_validation_and_rejects_forged_cursor_history",
          "artifact_store_one_item_stale_saturation_and_cancel_leave_root_generation_and_revision_unchanged",
          "retained_member_publication_rejects_wrong_owner_staleness_and_cancels_without_commit",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log("canonical-architecture-native " + event.stage + ": " + (event.law ?? ""));
      },
    });
    console.log("canonical-architecture-native receipts=" + receipts.length);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", ...segments], this.root);
  }
}

class ListRecordNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-list-record-boundaries accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"],this.repoRoot,["--lib","--no-fail-fast","list_record_"]);}
}

class ComposedPackSchemaTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-composed-pack-schema accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"],this.repoRoot,["--lib","composed_pack_schema_tests::"]);}
}

class IeeePayloadNativeTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-native accepts no arguments");await runRepositoryCargoTests(["semio-framework-os-kernel"],this.repoRoot,["--lib","ieee_payload_"]);}
}

class IeeePayloadSourceTestScript extends BundleScript {
  async run(segments:string[]):Promise<void>{if(segments.length)throw new Error("test-ieee-payload-source accepts no arguments");const root=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔢️ieee754");await runRepositoryTestCommand(process.execPath,["x","--no-install","tsc","--project",join(root,"🧪️tests/📋️tsconfig.json")],{cwd:this.repoRoot});await runRepositoryTestCommand(process.execPath,["test",join(root,"🧪️tests/🟦️.ts")],{cwd:this.repoRoot});}
}

//#region 🧬️RetainedCloneFixtures
/** 🎭️ One `choice` arm of `🧬️retained-clone/🧫️fixtures/📦️nested/🧬️schema/🔣️.json` (`#/$defs/choice`). */
type RetainedCloneChoiceV1 =
  | { readonly kind: "unit" }
  | { readonly kind: "text"; readonly text: string }
  | { readonly kind: "nested"; readonly rows: readonly { readonly id: number; readonly text: string }[] };

/** 🎟️ The copy grant a retained-clone corpus hands one step. */
type RetainedCloneGrantV1 = { readonly maximumItems: number; readonly maximumCopyBytes: number; readonly maximumCapacityBytes: number; readonly maximumDepth: number };

/** 📦️ `🧬️retained-clone/🧫️fixtures/📦️nested` — the nested-value clone corpus its schema admits. */
type RetainedCloneNestedFixtureV1 = {
  readonly source: {
    readonly title: string;
    readonly optional: string | null;
    readonly choice: RetainedCloneChoiceV1;
    readonly choices: readonly RetainedCloneChoiceV1[];
    readonly fixedArray: readonly [number, number, number, number];
    readonly pair: readonly [string, number];
    readonly triple: readonly [string, number, string];
    readonly labels: Readonly<Record<string, string>>;
  };
  readonly payloadByteLength: number;
  readonly payloadModulo: number;
  readonly recursiveDepth: number;
  readonly grant: RetainedCloneGrantV1;
  readonly insufficientCapacityBytes: number;
  readonly immutableLease: { readonly captured: string; readonly externalAfterCapture: string; readonly expected: string };
  readonly cancellationStops: readonly number[];
};

/** 🎯️ The state an ordered-map operation row must leave behind. */
type RetainedOrderedMapExpectationV1 = { readonly found: boolean; readonly ordinal: number; readonly entryCount: number };

/** 🗺️ `🗺️ordered-map/🧫️fixtures/📦️paging` — a lookup carries no value; an insert or duplicate carries the value it offers. */
type RetainedOrderedMapOperationV1 =
  | { readonly kind: "lookup"; readonly key: string; readonly expected: RetainedOrderedMapExpectationV1 }
  | { readonly kind: "insert" | "duplicate"; readonly key: string; readonly value: string; readonly expected: RetainedOrderedMapExpectationV1 };

/** 🗺️ `🗺️ordered-map/🧫️fixtures/📦️paging` — the paging, growth and immutable-lookup corpus its schema admits. */
type RetainedOrderedMapPagingFixtureV1 = {
  readonly pageCapacity: 16;
  readonly entryCount: number;
  readonly keyPrefix: string;
  readonly valuePrefix: string;
  readonly longKeyByteLength: number;
  readonly comparisonGrant: { readonly maximumItems: number; readonly maximumBytes: number };
  readonly progressChannels: { readonly comparisonOnly: true; readonly capacityOnly: true; readonly minimumMovedItems: number };
  readonly repeatedGrowth: { readonly entryCount: 32; readonly insertions: 17; readonly keyPrefix: string; readonly valuePrefix: string; readonly expectedEntryCount: 49 };
  readonly immutableLookup: { readonly capturedTarget: string; readonly externalAfterCapture: string; readonly expectedOrdinal: number };
  readonly operations: readonly RetainedOrderedMapOperationV1[];
};

/** 🧩️ `🧩preparation/🧪️fixtures/📦️lifecycle` — one preparation outcome row. */
type RetainedClonePreparationCaseV1 = {
  readonly id: string;
  readonly kind: "success" | "rejection" | "cancel" | "stale" | "fault" | "overBudget";
  readonly initial: number;
  readonly value: number;
  readonly interruptAfterTurns: number;
  readonly expected: { readonly published: boolean; readonly value: number; readonly history: number; readonly terminalEmpty: true };
};

/** 🧩️ `🧩preparation/🧪️fixtures/📦️lifecycle` — the preparation lifecycle corpus its schema admits. */
type RetainedClonePreparationFixtureV1 = {
  readonly grant: { readonly maximumItems: 1; readonly maximumBytes: number; readonly maximumDepth: number };
  readonly largeCapacity: { readonly stringByteLength: number; readonly expectedCode: "retained-clone.step-grant-too-small" };
  readonly cases: readonly RetainedClonePreparationCaseV1[];
};

/** 📋️ `📋️paged-list/🧫️fixtures/📦️copy` — the paged-list copy corpus its schema admits. */
type RetainedPagedListCopyFixtureV1 = {
  readonly maximumEntries: 1024;
  readonly entryCount: 513;
  readonly valuePrefix: string;
  readonly grant: { readonly maximumItems: 5; readonly maximumCopyBytes: 32; readonly maximumCapacityBytes: 4096; readonly maximumDepth: 64 };
  readonly cancellationAfterEntries: 173;
  readonly expected: { readonly ordered: true; readonly sourcePreserved: true; readonly copyRequiresMultipleTurns: true; readonly closeRequiresMultipleTurns: true; readonly terminalEmpty: true };
};
//#endregion 🧬️RetainedCloneFixtures

/** 🧬️ Validates the retained-clone resource contract and neutral corpus with Ajv and the platform structured-clone oracle. */
class RetainedCloneCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("retained-clone-check accepts no arguments");
    const root = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧫️fixtures/📦️nested");
    const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedCloneNestedFixtureV1>(schema);
    assert(validate(fixture), JSON.stringify(validate.errors));
    assert.deepEqual(structuredClone(fixture), fixture);
    assert.equal(fixture.source.choices.length, 3);
    assert.equal(fixture.source.fixedArray.length, 4);
    assert(fixture.grant.maximumDepth >= fixture.recursiveDepth);
    const capturedLease = structuredClone(fixture.immutableLease.captured);
    let externalLease = fixture.immutableLease.captured;
    externalLease = fixture.immutableLease.externalAfterCapture;
    assert.equal(capturedLease, fixture.immutableLease.expected);
    assert.notEqual(externalLease, capturedLease);
    assert(fixture.cancellationStops.some((stop: number) => stop > fixture.payloadByteLength / fixture.grant.maximumCopyBytes));
    const mapRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🧫️fixtures/📦️paging");
    const mapSchema = JSON.parse(readFileSync(join(mapRoot, "🧬️schema/🔣️.json"), "utf8"));
    const mapFixture = JSON.parse(readFileSync(join(mapRoot, "🔣️.json"), "utf8"));
    const validateMap = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedOrderedMapPagingFixtureV1>(mapSchema);
    assert(validateMap(mapFixture), JSON.stringify(validateMap.errors));
    const entries = Array.from({ length: mapFixture.entryCount }, (_, ordinal) => [`${mapFixture.keyPrefix}${ordinal.toString().padStart(4, "0")}`, `${mapFixture.valuePrefix}${ordinal}`] as [string, string]);
    const oracle = new Map(entries);
    const ordered = [...oracle.entries()].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0);
    const lowerBound = (key: string): number => {
      let low = 0;
      let high = ordered.length;
      while (low < high) {
        const middle = low + Math.floor((high - low) / 2);
        if (ordered[middle][0] < key) low = middle + 1;
        else high = middle;
      }
      return low;
    };
    for (const operation of mapFixture.operations) {
      const ordinal = lowerBound(operation.key);
      const found = ordinal < ordered.length && ordered[ordinal][0] === operation.key;
      if (operation.kind === "insert") ordered.splice(ordinal, 0, [operation.key, operation.value]);
      assert.equal(operation.kind === "insert" ? true : found, operation.expected.found, operation.kind);
      assert.equal(ordinal, operation.expected.ordinal, operation.kind);
      assert.equal(ordered.length, operation.expected.entryCount, operation.kind);
    }
    const longKey = "k".repeat(mapFixture.longKeyByteLength);
    assert.equal(new TextEncoder().encode(longKey).byteLength, mapFixture.longKeyByteLength);
    assert(longKey < `${longKey}z`);
    assert.equal(mapFixture.progressChannels.comparisonOnly, true);
    assert.equal(mapFixture.progressChannels.capacityOnly, true);
    assert(mapFixture.progressChannels.minimumMovedItems >= 1);
    const growth = new Map(Array.from({ length: mapFixture.repeatedGrowth.entryCount }, (_, ordinal) => [`${mapFixture.repeatedGrowth.keyPrefix}${(ordinal * 2).toString().padStart(4, "0")}`, `${mapFixture.repeatedGrowth.valuePrefix}${ordinal}`]));
    for (let ordinal = 0; ordinal < mapFixture.repeatedGrowth.insertions; ordinal += 1) growth.set(`${mapFixture.repeatedGrowth.keyPrefix}${(ordinal * 2 + 1).toString().padStart(4, "0")}`, `${mapFixture.repeatedGrowth.valuePrefix}insert-${ordinal}`);
    assert.equal(growth.size, mapFixture.repeatedGrowth.expectedEntryCount);
    const capturedTarget = structuredClone(mapFixture.immutableLookup.capturedTarget);
    let externalTarget = mapFixture.immutableLookup.capturedTarget;
    externalTarget = mapFixture.immutableLookup.externalAfterCapture;
    assert.equal(lowerBound(capturedTarget), mapFixture.immutableLookup.expectedOrdinal);
    assert.notEqual(externalTarget, capturedTarget);
    const preparationRoot = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧪️fixtures/📦️lifecycle");
    const preparationSchema = JSON.parse(readFileSync(join(preparationRoot, "🧬️schema/🔣️.json"), "utf8"));
    const preparationFixture = JSON.parse(readFileSync(join(preparationRoot, "🔣️.json"), "utf8"));
    const validatePreparation = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedClonePreparationFixtureV1>(preparationSchema);
    assert(validatePreparation(preparationFixture), JSON.stringify(validatePreparation.errors));
    assert(preparationFixture.largeCapacity.stringByteLength > preparationFixture.grant.maximumBytes);
    assert.equal(preparationFixture.largeCapacity.expectedCode, "retained-clone.step-grant-too-small");
    for (const row of preparationFixture.cases) {
      let value = row.initial;
      let history = 0;
      if (row.kind === "success") {
        value = row.value;
        history = 1;
      } else if (row.kind === "stale") {
        value += 1;
        history = 1;
      }
      assert.equal(value, row.expected.value, row.id);
      assert.equal(history, row.expected.history, row.id);
      assert.equal(row.expected.published, row.kind === "success", row.id);
      assert.equal(row.expected.terminalEmpty, true, row.id);
    }
    const pagedRoot = join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧫️fixtures/📦️copy");
    const pagedSchema = JSON.parse(readFileSync(join(pagedRoot, "🧬️schema/🔣️.json"), "utf8"));
    const pagedFixture = JSON.parse(readFileSync(join(pagedRoot, "🔣️.json"), "utf8"));
    const validatePaged = new Ajv2020({ strict: true, allErrors: true }).compile<RetainedPagedListCopyFixtureV1>(pagedSchema);
    assert(validatePaged(pagedFixture), JSON.stringify(validatePaged.errors));
    const pagedValues = Array.from({ length: pagedFixture.entryCount }, (_, ordinal) => `${pagedFixture.valuePrefix}${ordinal}`);
    const pagedOracle = structuredClone(pagedValues);
    assert.deepEqual(pagedOracle, pagedValues);
    assert.equal(pagedOracle.length, pagedFixture.entryCount);
    assert(pagedFixture.entryCount <= pagedFixture.maximumEntries);
    assert(pagedFixture.cancellationAfterEntries > 0 && pagedFixture.cancellationAfterEntries < pagedFixture.entryCount);
    assert(new TextEncoder().encode(pagedOracle.join("")).byteLength > pagedFixture.grant.maximumCapacityBytes);
    assert.equal(pagedFixture.expected.ordered, true);
    assert.equal(pagedFixture.expected.sourcePreserved, true);
    assert.equal(pagedFixture.expected.copyRequiresMultipleTurns, true);
    assert.equal(pagedFixture.expected.closeRequiresMultipleTurns, true);
    assert.equal(pagedFixture.expected.terminalEmpty, true);
    console.log(`retained-clone-check: choices=${fixture.source.choices.length} payload=${fixture.payloadByteLength} cancellation=${fixture.cancellationStops.length} orderedMap=${ordered.length} growth=${growth.size} preparation=${preparationFixture.cases.length} paged=${pagedOracle.length}`);
  }
}

/** 🪪️ Executes the native outer opening-attempt wire law without broadening the browser patch contract. */
class DocumentOpeningAttemptNativeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("document-opening-attempt-native-check accepts no arguments");
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          cargoArgs: ["--features", "sync"],
          laws: ["os_store::sync::tests::document_opening_attempt_wire_preserves_outer_owner_without_widening_actor_messages"],
        },
      ],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) {
        console.log(`document-opening-attempt-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      },
    });
    console.log(`document-opening-attempt-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

class NativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, ["--lib", "--features", "sync,ureq", ...rest]);
  }
}

class SnapshotNativeAdmissionTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runRepositoryTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🔨️modules/🌱️value/🛬️decode/🧪️tests/🟦️.ts"), join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/📡️literal/🧪️tests/🟦️.ts"), join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🧪️tests/🚦️controlled/🟦️.ts"), join(this.repoRoot, "🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️reference/🧪️tests/🟦️.ts")], { cwd: this.repoRoot });
    if (segments.length === 1 && segments[0] === "portable") return;
    const { rest } = resolveTestLevel(segments);
    await runRepositoryCargoTests(["semio-framework-os-kernel"], this.repoRoot, ["--test", "sqlite_snapshot_native_admission", ...rest, ...(rest.includes("--no-fail-fast") ? [] : ["--no-fail-fast"])]);
  }
}

class DirectoryRuntimeSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-runtime-source accepts no arguments");
    const { testDirectoryRuntimeIdentityFixture } = await import("../../🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts");
    testDirectoryRuntimeIdentityFixture();
  }
}

/** 🪪️ Proves the broker-visible identity is canonically bound to one exact server session. */
export async function directorySessionAuthorityOracle(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1");
  const schema = JSON.parse(readFileSync(join(root, "🧬️.schema.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const validate: SchemaCheck = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
  const contract = await import("../../🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts");
  for (const row of fixture.rows) {
    assert.equal(validate(row.value), row.accepted, `${row.id}: ${JSON.stringify(validate.errors)}`);
    let accepted = true;
    try { contract.parseDirectorySessionAuthorityJsonV1(JSON.stringify(row.value)); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, `${row.id}: TypeScript`);
  }
  for (const row of fixture.raw) {
    let accepted = true;
    try { contract.parseDirectorySessionAuthorityJsonV1(row.source); } catch { accepted = false; }
    assert.equal(accepted, row.accepted, `${row.id}: canonical`);
  }
  for (const row of fixture.bindingGoldens) {
    const hash = createHash("sha256");
    const length = (value: string) => { const bytes = Buffer.alloc(4); bytes.writeUInt32BE(Buffer.byteLength(value)); return bytes; };
    const generation = Buffer.alloc(8);
    const expiresAt = Buffer.alloc(8);
    generation.writeBigUInt64BE(BigInt(row.authorizationGeneration));
    expiresAt.writeBigInt64BE(BigInt(row.expiresAt));
    hash.update("semio/hub/directory-event-page/session-binding/v1\0");
    hash.update(length(row.sessionId));
    hash.update(row.sessionId);
    hash.update(length(row.userId));
    hash.update(row.userId);
    hash.update(generation);
    hash.update(expiresAt);
    assert.equal(hash.digest("hex"), row.sessionBindingSha256, row.id);
  }
  for (const row of fixture.authorityLifecycle) {
    assert(["installed", "retained", "replaced", "ignored", "retired"].includes(row.outcome), `${row.id}: outcome`);
    assert.equal(row.epochDelta === 1, row.retireMountedAuthorities, `${row.id}: retirement epoch`);
    if (row.outcome === "replaced" || row.outcome === "retired") assert.equal(row.retainIndeterminateJobs, true, `${row.id}: uncertain jobs`);
    if (!row.brokerAdmissionCurrent) assert.equal(row.outcome, "ignored", `${row.id}: stale admission`);
  }
  const rust = readFileSync(join(root, "🦀️.rs"), "utf8");
  const client = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  const hub = readFileSync(join(repoRoot, "🌎️hub/🏗️bootstrap/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectorySessionAuthorityV1") && rust.includes("parse_canonical_json"), "Rust session authority contract missing");
  assert(client.includes("DirectorySessionAuthorityV1::parse_canonical_json") && client.includes("DIRECTORY_SESSION_AUTHORITY_MAX_BYTES"), "Rust client does not enforce canonical session authority");
  assert(hub.includes("directory_event_page_session_binding_v1(&caller)") && hub.includes("Json<DirectorySessionAuthorityV1>"), "Hub session response is not bound to the existing session digest");
  return fixture.rows.length + fixture.raw.length + fixture.bindingGoldens.length + fixture.authorityLifecycle.length + 3;
}

class DirectorySessionAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-session-authority-check accepts only --native");
    const checks = await directorySessionAuthorityOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          laws: [
            "os_directory::schema::tests::directory_session_authority_v1_matches_neutral_corpus_and_binding_goldens",
            "os_directory::client::tests::session_authority_client_preserves_canonical_binding_and_rejects_reordered_body",
            "os_directory::schema::tests::inference_current_hub_wire_preserves_required_nullable_hash",
            "os_directory::schema::tests::inference_indeterminate_lifecycle_matches_neutral_corpus",
            "os_directory::client::tests::inference_client_refuses_substituted_hub_receipt_and_page_coordinates",
          ],
        }],
      });
      console.log(`directory-session-authority-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-session-authority-check: checks=${checks} clean`);
  }
}

/** 📃️ Proves the shared event-page envelope against an independent JSON Schema and SHA-256 oracle. */
export async function directoryEventPageContractOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator: SchemaCheck = semioSchemaAjvV1({ strict: false, allErrors: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  assert(validator(fixture.valid), JSON.stringify(validator.errors));
  assert.equal(new TextEncoder().encode(fixture.canonicalUnsigned).length, 474);
  assert.equal(createHash("sha256").update(fixture.canonicalUnsigned).digest("hex"), fixture.expectedReceiptSha256);
  const contract = await import("../../🔨️modules/📇️directory/🧬️schema/🟦️.ts");
  const parsed = await contract.parseDirectoryEventPageV1(JSON.stringify(fixture.valid));
  assert.deepEqual(parsed, fixture.valid);
  const setPath = (value: any, path: string, replacement: unknown): any => {
    const copy = structuredClone(value);
    const parts = path.split(".");
    let parent = copy;
    for (const part of parts.slice(0, -1)) parent = parent[Number.isInteger(Number(part)) ? Number(part) : part];
    parent[parts.at(-1)!] = replacement;
    return copy;
  };
  for (const hostile of fixture.hostileMutations) await assert.rejects(() => contract.parseDirectoryEventPageV1(JSON.stringify(setPath(fixture.valid, hostile.path, hostile.value))), hostile.name);
  const canonical = JSON.stringify(fixture.valid);
  await assert.rejects(() => contract.parseDirectoryEventPageV1(`${canonical} `), "trailing-byte");
  await assert.rejects(() => contract.parseDirectoryEventPageV1(canonical.replace('{"schema":', '{"schema":"duplicate","schema":')), "duplicate-key");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"), "utf8");
  assert(rust.includes("pub struct DirectoryEventPageV1") && rust.includes("pub fn receipt_matches(&self) -> bool"), "Rust event-page contract missing");
  return 5 + fixture.hostileMutations.length + fixture.rawHostiles.length;
}

/** 🔌️ Proves both directory clients preserve one canonical event-page response and its bounded header. */
export async function directoryEventPageClientOracle(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/📃️event-page-v1.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔣️.json"), "utf8"));
  const validator: SchemaCheck = semioSchemaAjvV1({ strict: false, allErrors: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/DirectoryEventPageV1" });
  const canonical = JSON.stringify(fixture.valid);
  const accept = (raw: string, after: number) => {
    if (!Number.isSafeInteger(after) || after < 0 || new TextEncoder().encode(raw).byteLength > 65_536) throw new Error("client admission");
    const parsed = JSON.parse(raw);
    if (JSON.stringify(parsed) !== raw || !validator(parsed)) throw new Error("client admission");
    if (parsed.afterSeqExclusive !== after) throw new Error("frontier substitution");
    const { receiptSha256, ...unsigned } = parsed;
    if (createHash("sha256").update(JSON.stringify(unsigned)).digest("hex") !== receiptSha256) throw new Error("receipt substitution");
    return { canonicalJson: raw, throughSeqInclusive: parsed.throughSeqInclusive, receiptSha256: parsed.receiptSha256 };
  };
  const page = accept(canonical, 3);
  assert.equal(page.canonicalJson, canonical);
  assert.equal(page.throughSeqInclusive, fixture.valid.throughSeqInclusive);
  assert.equal(page.receiptSha256, fixture.expectedReceiptSha256);
  assert.throws(() => accept("x".repeat(65_537), 0));
  assert.throws(() => accept(canonical, 4));
  assert.throws(() => accept(canonical, -1));
  assert.throws(() => accept(`${canonical} `, 3));
  const typescript = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🟦️.ts"), "utf8");
  const method = typescript.slice(typescript.indexOf("async eventPage("), typescript.indexOf("stream(since", typescript.indexOf("async eventPage(")));
  assert(
    method.includes("response.text()") && method.includes("parseDirectoryEventPageV1(canonicalJson)") && method.includes("page.afterSeqExclusive !== after") && !method.includes("response.json()"),
    "TypeScript canonical page transport is incomplete",
  );
  assert(typescript.includes("streamAcknowledged(since:") && typescript.includes("acknowledge: (through: number)"), "TypeScript acknowledged directory frontier is missing");
  const rust = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8");
  assert(rust.includes("pub async fn event_page") && rust.includes("CanonicalDirectoryEventPageV1") && rust.includes("DIRECTORY_EVENT_PAGE_MAX_BYTES"), "Rust canonical page transport is incomplete");
  assert(rust.includes("pub fn stream_acknowledged") && rust.includes("pub fn acknowledge(&mut self, through: u64)"), "Rust acknowledged directory frontier is missing");
  assert(rust.includes("pub struct DirectoryEventPageBootstrapV1") && rust.includes("pub enum DirectoryBootstrapTransition"), "Rust directory bootstrap owner is missing");
  return 11;
}

class DirectoryEventPageContractCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-contract-check accepts only --native");
    const checks = await directoryEventPageContractOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::schema::tests::directory_event_page_v1_matches_language_neutral_receipt_and_rejects_hostiles"] }],
        progress(event) {
          console.log(`directory-event-page-contract ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`directory-event-page-contract-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-contract-check: checks=${checks} clean`);
  }
}

class DirectoryEventPageClientCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("directory-event-page-client-check accepts only --native");
    const checks = await directoryEventPageClientOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        groups: [{ package: "semio-framework-os-kernel", target: { kind: "lib" }, laws: ["os_directory::client::tests::directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io"] }],
        progress(event) {
          console.log(`directory-event-page-client ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`directory-event-page-client-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`directory-event-page-client-check: checks=${checks} clean`);
  }
}

/** 🧭️ Proves fetch, exact Home ACK, next-page, and live-cursor ordering independently of either shell. */
export function directoryEventPageBootstrapOracle(repoRoot: string): number {
  const trace = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🚀️event-page-bootstrap-v1.json"), "utf8"));
    const validator = ownedExport(repoRoot, "directory", "DirectoryEventPageBootstrapTraceV1");
  assert(validator(trace), JSON.stringify(validator.errors));
  let cursor = trace.initialAfter;
  let pending: any = null;
  let live = false;
  const present = (page: any): void => {
    assert(!pending && !live && page.afterSeqExclusive === cursor && page.throughSeqInclusive >= cursor);
    pending = page;
  };
  const acknowledge = (page: any, epoch = trace.bootstrapEpoch): "fetch" | "live" => {
    assert(pending && epoch === trace.bootstrapEpoch);
    for (const key of ["receiptSha256", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive"]) assert.equal(page[key], pending[key]);
    cursor = pending.throughSeqInclusive;
    const hasMore = pending.hasMore;
    pending = null;
    live = !hasMore;
    return hasMore ? "fetch" : "live";
  };
  present(trace.pages[0]);
  assert.throws(() => present(trace.pages[1]), "page 2 before ACK");
  assert.throws(() => acknowledge({ ...trace.pages[0], receiptSha256: "d".repeat(64) }), "forged ACK");
  assert.equal(cursor, trace.initialAfter);
  assert.equal(acknowledge(trace.pages[0]), "fetch");
  present(trace.pages[1]);
  assert.throws(() => acknowledge(trace.pages[1], trace.bootstrapEpoch + 1), "stale epoch");
  assert.equal(acknowledge(trace.pages[1]), "live");
  for (const wakeup of trace.wakeups) assert(wakeup > cursor && cursor === trace.expectedSocketSince);
  const worker = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"), "utf8");
  assert(worker.includes("DirectoryEventPageBootstrapV1") && worker.includes("directory-bootstrap-ack") && worker.includes("directory-event-page"), "browser worker bootstrap owner missing");
  return 11;
}

class DirectoryEventPageBootstrapCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("directory-event-page-bootstrap-check accepts no arguments");
    console.log(`directory-event-page-bootstrap-check: checks=${directoryEventPageBootstrapOracle(this.repoRoot)} clean`);
  }
}

/** 🚦️ Proves every WAL backend exposes the same read-only active/sealed contract. */
export function walSegmentStateOracle(repoRoot: string): number {
  const storageRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db");
  const core = readFileSync(join(storageRoot, "🗄️storage/🦀️.rs"), "utf8");
  const sqlite = readFileSync(join(storageRoot, "🗄️storage/🪶️sqlite/🦀️.rs"), "utf8");
  const postgres = readFileSync(join(storageRoot, "🗄️storage/🐘️postgres/🦀️.rs"), "utf8");
  const neo4j = readFileSync(join(storageRoot, "🗄️storage/🌐️neo4j/🦀️.rs"), "utf8");
  const faultStorage = readFileSync(join(storageRoot, "🧪️tests/🧯️fault-storage/🦀️.rs"), "utf8");
  assert(core.includes("pub enum WalSegmentState") && core.includes("Active,") && core.includes("Sealed,"));
  assert(core.includes("async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError>"));
  assert(core.includes("WalState { backend: DbIoBackendControl") && core.includes("WalSegmentState(WalSegmentState)"));
  assert(core.includes("Err(error) if error.kind() == std::io::ErrorKind::NotFound => WalSegmentState::Active") && core.includes("Err(error) => return Err(io_err(error))"));
  assert(sqlite.includes("SELECT sealed FROM wal_segment") && sqlite.includes("Err(DbError::Corrupt") && sqlite.includes("DbIoTask::WalState"));
  assert(postgres.includes("POSTGRES_WAL_STATE_QUERY") && postgres.includes("fetch_optional") && !postgres.slice(postgres.indexOf("const POSTGRES_WAL_STATE_QUERY"), postgres.indexOf("const POSTGRES_WAL_STATE_QUERY") + 240).includes("FOR UPDATE"));
  assert(neo4j.includes("const CYPHER_WAL_STATE") && neo4j.includes("RETURN n.sealed AS sealed") && !neo4j.slice(neo4j.indexOf("const CYPHER_WAL_STATE"), neo4j.indexOf("const CYPHER_WAL_STATE") + 240).includes("bytes"));
  const faultStorageLaws = readFileSync(join(storageRoot, "🧪️tests/🧯️fault-storage-laws/🦀️.rs"), "utf8");
  assert(faultStorage.includes("async fn segment_state") && faultStorageLaws.includes("fault_storage_segment_state_is_observational_and_counter_neutral"));
  return 12;
}

class WalSegmentStateCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("wal-segment-state-check accepts only --native");
    const checks = walSegmentStateOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({
        cwd: this.repoRoot,
        ...exactCargoStageEnvironments(),
        cargoArgs: ["--all-features"],
        groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib" },
            laws: [
              "memory_storage_satisfies_wal_storage_laws",
              "fs_storage_satisfies_wal_storage_laws",
              "fs_storage_stale_seal_marker_does_not_resurrect_missing_segment",
              "memory_storage_db_backend_accessors_and_capabilities",
              "wal_segment_state_observes_active_sealed_and_missing_rows",
              "wal_segment_state_decoder_rejects_non_boolean_storage_values",
              "wal_segment_state_query_and_mapper_are_read_only_and_byte_neutral",
              "wal_cypher_statements_reference_the_expected_label_and_keys",
              "fault_storage_segment_state_is_observational_and_counter_neutral",
            ],
          },
        ],
        progress(event) {
          console.log(`wal-segment-state ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        },
      });
      console.log(`wal-segment-state-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-segment-state-check: checks=${checks} clean`);
  }
}

class CodecSendSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-codec-send-source accepts no arguments");
    const { testNativeCodecSendFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts");
    testNativeCodecSendFixture();
  }
}

class GroupVisibilitySourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-group-visibility-source accepts no arguments");
    const { testGroupVisibilityFixtures } = await import("../../🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts");
    testGroupVisibilityFixtures();
  }
}

class BackboneDetachSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-backbone-detach-source accepts no arguments");
    const { testBackboneDetachFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts");
    testBackboneDetachFixture();
  }
}

class MemberDialectSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-member-dialect-source accepts no arguments");
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts");
    testFixtureProjectionRetirement();
  }
}

class MemberDialectCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { testMemberDialectFixture } = await import("../../🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts");
    testMemberDialectFixture();
    const { testFixtureProjectionRetirement } = await import("../../🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/🟦️.ts");
    testFixtureProjectionRetirement();
    const receipts = await runRepositoryExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      cargoArgs: segments,
      buildBudgetMs: 3_600_000,
      groups: [
        {
          package: "semio-framework-schema",
          target: { kind: "lib" },
          laws: [
            "artifact_composition_fields_derive_emits_expected_slot_tables",
            "artifact_composition_fields_default_to_empty_for_leaf_artifacts",
            "artifact_composition_projection_walks_aliases_nested_options_and_cancels",
            "artifact_composition_projection_real_child_alias_has_fixed_admission_bounds",
          ],
        },
        {
          package: "semio-framework-os-kernel",
          target: { kind: "lib" },
          laws: [
            "member_factory_closed_dialect_matches_neutral_admission_corpus",
            "member_factory_closed_dialect_rejects_identity_and_owner_substitution",
            "member_factory_closed_dialect_graph_admission_matches_neutral_corpus",
            "member_factory_closed_dialect_graph_sync_preserves_prior_state_on_rejection",
            "member_factory_closed_dialect_parent_projection_matches_neutral_corpus",
            "initial_child_identity_matches_neutral_coordinates_and_blake3",
          ],
        },
        {
          package: "semio-framework-plugin",
          target: { kind: "lib" },
          laws: [
            "fixture_projection_retires_exact_tree_before_return_error_or_panic",
            "member_factory_parent_snapshot_restore_matches_neutral_corpus",
            "member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member",
            "member_factory_closed_dialect_register_rejects_pin_without_mutating_member",
            "member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner",
          ],
        },
      ],
    });
    console.log(`exact member admission laws: ${receipts.reduce((sum, receipt) => sum + receipt.assertions, 0)} executed across ${receipts.length} verified test executables`);
  }
}

//#region 🧩️JCO Package Adapter
class GenerateJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "generate");
  }
}
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "preview");
  }
}
class CheckJcoPackageAdapterScript extends BundleScript {
  run(): void {
    runNestedCargoPackageAdapter(this.repoRoot, "check");
  }
}
//#endregion 🧩️JCO Package Adapter

//#region 🌪️ReopenStorm
/** 🌪️ The reopen-storm laws of the db engine: a hub restart's reconnect storm, measured where the storage lives. `unit`
 * greets two dozen grown documents at once after a reopen (bound 30 s, the growth e2e's reopen bound); `fs`, `sqlite`,
 * `postgres` and `neo4j` are the throughput laws that grow, reopen alone and reopen as a storm and bound the storm against
 * the solo reopen (`🛢️db/⚙️engine/🧫️fixtures/⏱️throughput`). `postgres`/`neo4j` run against the ONE shared development
 * server claimed by `os-hub-ts backend run <postgres|neo4j> -- …` (its `OS_HUB_*` environment selects it), so they are
 * selected only by name and `all` never includes them. */
const REOPEN_STORM_LAWS: Readonly<Record<string, Readonly<{ law: string; features: string; claimed?: string }>>> = {
  unit: { law: "db_engine::tests::long::two_dozen_grown_documents_greeted_at_once_after_a_reopen_are_welcomed_within_the_reopen_bound", features: "sqlite" },
  fs: { law: "db_engine::throughput_tests::fs_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite" },
  sqlite: { law: "db_engine::throughput_tests::sqlite_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite" },
  postgres: { law: "db_engine::throughput_tests::postgres_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite,postgres", claimed: "OS_HUB_DATABASE_URL" },
  neo4j: { law: "db_engine::throughput_tests::neo4j_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite,neo4j", claimed: "OS_HUB_NEO4J_URI" },
};

/** 🌪️ `reopen-storm-check [unit|fs|sqlite|all]…` — runs each selected law alone in its own cargo test process (in place,
 * `SEMIO_DB_ISOLATED_LAW`), streams its output, reads the storm welcome distribution it prints and publishes the
 * acceptance record. Ctrl-C stops the running law. */
class ReopenStormCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const wanted = segments.length === 0 || segments.includes("all") ? Object.keys(REOPEN_STORM_LAWS).filter((name) => !REOPEN_STORM_LAWS[name]!.claimed) : segments;
    const unknown = wanted.filter((name) => !(name in REOPEN_STORM_LAWS));
    if (unknown.length) throw new Error(`reopen-storm-check accepts ${Object.keys(REOPEN_STORM_LAWS).join(" | ")} | all, got ${unknown.join(",")}`);
    const throughput = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧫️fixtures/⏱️throughput/🔣️.json"), "utf8"));
    const validate = ownedExport(this.repoRoot, "db.engine", "ThroughputV1");
    assert(validate(throughput), `throughput fixture: ${JSON.stringify(validate.errors)}`);
    console.log(`[reopen-storm] throughput fixture valid (ThroughputV1, AJV): storm ${throughput.storm.documents}×${throughput.storm.batches}×${throughput.storm.batchEdits}, storm/serial ≤ ${throughput.bounds.stormToSerialRatioMax}`);
    const unclaimed = wanted.filter((name) => REOPEN_STORM_LAWS[name]!.claimed && !process.env[REOPEN_STORM_LAWS[name]!.claimed!]);
    if (unclaimed.length) throw new Error(`reopen-storm-check ${unclaimed.join(",")} needs the claimed shared server: run it under \`os-hub-ts backend run ${unclaimed[0]} -- …\``);
    const { acceptanceCheckResult, publishAcceptanceCheckResult, runLawProcess } = await import("../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts");
    const startedAt = new Date();
    const measured: Record<string, number | string | boolean> = {};
    const failed: string[] = [];
    let cancelled = false;
    for (const [index, name] of wanted.entries()) {
      if (cancelled) break;
      const { law, features, claimed } = REOPEN_STORM_LAWS[name]!;
      console.log(`[reopen-storm] ${index + 1}/${wanted.length} ${name}: ${law}`);
      const { status, lines } = await runLawProcess(
        "cargo",
        ["test", "-p", "semio-framework-os-kernel-db", "--features", features, "--lib", "--no-fail-fast", "--", "--exact", law, "--nocapture", "--test-threads=1", ...(claimed ? ["--include-ignored"] : [])],
        { cwd: this.repoRoot, env: { ...process.env, CARGO_INCREMENTAL: "0", RUST_MIN_STACK: "268435456", SEMIO_DB_ISOLATED_LAW: law } },
        () => {
          cancelled = true;
        },
      );
      const passed = status === 0 && lines.some((line) => line.includes("test result: ok. 1 passed"));
      measured[`${name}Pass`] = passed;
      const storm = lines.map((line) => /storm of (\d+) in (\d+) ms: welcome p50 ([\d.]+) ms .*? max ([\d.]+) ms(?:.*solo ([\d.]+) ms)?/u.exec(line)).find((match) => match !== null);
      if (storm) Object.assign(measured, { [`${name}Documents`]: Number(storm[1]), [`${name}StormMs`]: Number(storm[2]), [`${name}WelcomeP50Ms`]: Number(storm[3]), [`${name}WelcomeMaxMs`]: Number(storm[4]) }, storm[5] === undefined ? {} : { [`${name}SoloMs`]: Number(storm[5]) });
      if (!passed) failed.push(name);
    }
    const status = cancelled ? "skipped" : failed.length === 0 ? "pass" : "fail";
    publishAcceptanceCheckResult(
      this.repoRoot,
      acceptanceCheckResult({
        check: "hub-reopen-storm",
        status,
        startedAt,
        measured: { ...measured, laws: wanted.join(","), cancelled },
        summary: {
          en: `reopen storm laws ${wanted.length - failed.length}/${wanted.length} pass (${wanted.join(", ")})${failed.length ? `; failing: ${failed.join(", ")}` : ""}${cancelled ? "; cancelled" : ""}`,
          de: `Wiederöffnungssturm-Gesetze ${wanted.length - failed.length}/${wanted.length} bestanden (${wanted.join(", ")})${failed.length ? `; fehlgeschlagen: ${failed.join(", ")}` : ""}${cancelled ? "; abgebrochen" : ""}`,
        },
      }),
    );
    if (status !== "pass") process.exitCode = 1;
  }
}
//#endregion 🌪️ReopenStorm

/** 🌐️ Checks owner removal with neutral schema vectors and independent Ajv validation. */
class DocumentHttpCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🌐️document-http/🧫️fixtures/🔣️.json"), "utf8"));
    const ajv = semioSchemaAjvV1({ strict: false });
    const declarationSchema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧬️schema/🔣️.json"), "utf8"));
    const declared = ajv.compile(declarationSchema);
    assert(declared(fixture.neutral));
    assert(declared(fixture.secondary));
    const replyBounds=ajv.compile(fixture.replyNodeBounds.oracleSchema);
    for(const vector of fixture.replyNodeBounds.vectors) assert.equal(replyBounds(Array(vector.items).fill(null)),vector.valid);
    const validate = ajv.compile(JSON.parse(fixture.neutral.operations[0].inputSchema));
    for (const vector of fixture.vectors) assert.equal(validate(vector.value), vector.valid, vector.name);
    for (const law of ["os_directory::client::document_http::tests::owner_removal_preserves_neutral_document_transport", "os_directory::client::document_http::tests::schema_vectors_match_owned_validator", "os_directory::client::document_http::tests::decoded_replies_obey_the_same_node_bounds_as_owner_inputs"]) {
      await runRepositoryCargoTests(["semio-framework-os-kernel"], this.root, ["--lib", "--", law, "--exact", "--nocapture"]);
    }
    await runRepositoryCargoTests(["semio-framework-os-kernel"], this.root, ["--lib", "--", "os_directory::client::tests::document_http_transport_preserves_scope_bounds_and_owner_decode", "--exact"]);
    console.log("document-http: Ajv vectors, removal/reinstall, scope, bounds and owner decoding passed");
  }
}


const router = new ScriptRouter(import.meta.dir)
  .register("check", CheckScript)
  .register("retained-clone-check", RetainedCloneCheckScript)
  .register("test", TestScript)
  .register("test-list-record-boundaries", ListRecordNativeTestScript)
  .register("test-composed-pack-schema", ComposedPackSchemaTestScript)
  .register("test-ieee-payload-native", IeeePayloadNativeTestScript)
  .register("test-ieee-payload-source", IeeePayloadSourceTestScript)
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("document-opening-attempt-native-check", DocumentOpeningAttemptNativeCheckScript)
  .register("test-scalar-wire-source", ScalarWireSourceScript)
  .register("generate-jco-package-adapter", GenerateJcoPackageAdapterScript)
  .register("preview-generated", PreviewGeneratedScript)
  .register("check-jco-package-adapter", CheckJcoPackageAdapterScript)
  .register("test-native", NativeTestScript)
  .register("test-snapshot-native-admission", SnapshotNativeAdmissionTestScript)
  .register("test-directory-runtime-source", DirectoryRuntimeSourceScript)
  .register("directory-session-authority-check", DirectorySessionAuthorityCheckScript)
  .register("directory-event-page-contract-check", DirectoryEventPageContractCheckScript)
  .register("directory-event-page-client-check", DirectoryEventPageClientCheckScript)
  .register("directory-event-page-bootstrap-check", DirectoryEventPageBootstrapCheckScript)
  .register("wal-segment-state-check", WalSegmentStateCheckScript)
  .register("test-codec-send-source", CodecSendSourceScript)
  .register("test-group-visibility-source", GroupVisibilitySourceScript)
  .register("test-backbone-detach-source", BackboneDetachSourceScript)
  .register("test-member-dialect-source", MemberDialectSourceScript)
  .register("member-dialect-check", MemberDialectCheckScript);

router.register("document-http-check", DocumentHttpCheckScript);
router.register("paged-history-stack-check", PagedHistoryStackScript);
router.register("wal-recovery-check", WalRecoveryCheckScript);
router.register("wal-capacity-check", WalCapacityCheckScript);
router.register("wal-committed-transactions-check", WalCommittedTransactionsCheckScript);
router.register("wal-writer-authority-check", WalWriterAuthorityCheckScript);
router.register("wal-writer-fence-live", WalWriterFenceLiveScript);
router.register("postgres-round-trips-live", PostgresRoundTripsLiveScript);
router.register("database-history-completion-check", DatabaseHistoryCompletionCheckScript);
router.register("database-catalog-read-ownership-check", DatabaseCatalogReadOwnershipCheckScript);
router.register("database-capability-completion-check", DatabaseCapabilityCompletionCheckScript);
router.register("wal-committed-compaction-check", WalCommittedCompactionCheckScript);
router.register("database-shutdown-check", DatabaseShutdownCheckScript);
router.register("document-mount-single-flight-check", DocumentMountSingleFlightCheckScript);
router.register("durable-owned-group-decision-check", DurableOwnedGroupDecisionCheckScript);
router.register("durable-group-journal-check", DurableGroupJournalCheckScript);
router.register("reopen-storm-check", ReopenStormCheckScript);

/** 🧱️ Runs the retained owned fixture law against its independent oracle. */
class DirectoryLeaseFixtureScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-directory-lease-fixture accepts no arguments");
    await runBudgetedTestCommand(process.execPath, ["test", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🔏️document-execution-target-lease-v1/🟦️.ts")], { cwd: this.repoRoot, env: process.env, budgetMs: 15_000, throwOnFailure: true });
  }
}

router.register("test-directory-lease-fixture", DirectoryLeaseFixtureScript);


await runScriptMain(router, { defaultCommand: "check" });

````

### 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml

Bytes11119; SHA-256 5b6986779d5e1470affa28e0f02246068ed8e818617eafb14b9be778bce8b712

````text
[package]
workspace = "../../../../.."
name = "semio-framework-os-kernel"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Semio framework OS kernel — wasm-safe store/spr/dsl/pack document model"

[package.metadata.semio]
channel-version-consumers = "🧫️fixtures/📡️channel/📇️consumers.json"
role = "framework"
id = "os-kernel"

[lints]
workspace = true

[lib]
name = "semio_framework_os_kernel"
crate-type = ["rlib", "cdylib"]
path = "🦀️.rs"

[[test]]
name = "pack_schema_hash"
path = "../../🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️schema-hash/🦀️.rs"

[[test]]
name = "sqlite_snapshot_native_admission"
path = "../../🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🦀️.rs"

[features]
protocol-laws = ["record-codec-laws"]
record-codec-laws = ["semio-framework-pack/corruption-testing"]
durable-group-testing = []
default = ["deflate"]
deflate = ["semio-framework-replication/deflate", "semio-framework-pack/deflate"]
# 🌀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet terra-directory-and-run): the native
# `📇️directory/🔌️client`'s `NativeDirectoryTransport` routes its HTTP calls through
# `semio-framework-os-services`'s `HttpPool`/`ComputePool` (bounded, cancellable, deadline-aware).
# The services edge now belongs to `sync` as the shared host boundary for Store file watching too.
ureq = ["dep:ureq", "dep:semio-framework-actor"]
# 🧵️ No `tokio/net` reference here (unlike `rt`/`time`): `tokio` is now a
# `not(all(target_arch = "wasm32", target_env = "p2"))`-gated dependency (presentation on native and
# browser wasm32, absent from wasip2), so a feature-flag `tokio/net` would still unify onto browser
# wasm32, and tokio's own build script hard-errors on `net` for wasm32 (unlike `rt`/`time`, which it
# explicitly supports there). `net` is instead added unconditionally for non-wasm32 targets only, via
# the `[target.cfg(not(wasm32))]` `tokio` entry below — see its docstring. WASI-P2 guest plugins
# never link the sync actor's native (tokio-tungstenite) transport — see `🔄️sync/🦀️.rs`'s
# `native_actor` vs `wasm_actor` split — so `net` has no business reaching wasm32 at all.
sync = ["dep:tokio-tungstenite", "dep:semio-framework-os-services", "dep:ureq", "tokio/rt", "tokio/time"]
worker = ["sync"]
# 🧬️ Keeps the boundary/protocol type-generation capability explicit at this layer. The current
# wire declarations are owned by `semio-framework`, so enabling this feature adds no dependency.
typegen = []
# ⌨️ The native command-line binaries (`pack`, `spr`, `semio`). Their `cli` modules are mounted only
# off wasm32 (`os_pack::cli`, `os_spr::cli` need the native file layer), so the bins are opt-in: a
# wasm32 test build of this crate (`--target wasm32-wasip2 --test pack_schema_hash`) never tries to
# link them, and a native caller asks for them with `--features native-bin`.
native-bin = []

[dependencies]
semio-framework-dsl = { path = "../../../../🔨️modules/🗣️dsl/📦️packages/🦀️rust" }
semio-framework-ui-locale = { path = "../../../../🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust" }
semio-framework-schema-state = { path = "../../../../🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust" }
semio-framework-schema-composition = { path = "../../../../🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust" }
semio-framework-schema-validator = { workspace = true }
semio-framework-value = { path = "../../../../🔨️modules/🌱️value/📦️packages/🦀️rust" }
dsl_derive = { path = "../../🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust", package = "semio-framework-os-kernel-dsl-derive" }
semio-framework-value-derive = { path = "../../../../🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust", package = "semio-framework-value-derive" }
semio-framework-pack = { workspace = true }
semio-framework-replication = { workspace = true }
semio-framework-schema-registry = { workspace = true }
semio-framework-ui-viewport = { workspace = true }
# 🪪️ `OperationContext`/`CancelToken`/`ScopeHandle` vocabulary (ticket
# 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME) — pure, zero-tokio, wasm32-safe, so this is an
# unconditional (not target-gated) dependency: `📇️directory/🔌️client`'s `DirectoryTransport` trait
# carries an `OperationContext` on every implementation, native AND browser alike.
semio-framework-async = { workspace = true, features = ["entrypoint"] }
# 🔤️ `pack_value_to_base64`/`pack_value_from_base64` (`🏪️store/🦀️.rs`) are the only base64
# call sites this crate owns, and they are genuinely guest-reachable (scene `*Json` field encoding),
# not host-only — so this re-points at the first-party RFC 4648 codec instead of target-gating a
# third-party dependency away. Replaces the former `base64 = "0.22.1"` entry.
semio-framework-io-base64 = { path = "../../../../🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust" }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
semio-framework-hash = { path = "../../../../🔨️modules/🔏️hash/📦️packages/🦀️rust", package = "semio-framework-hash" }
semio-framework-job = { workspace = true }
# 🧮️ `GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`/`GUEST_HOST_ANSWER_CEILING_BYTES` — the one declared
# linear-memory budget `📡️spr/🧵️channel`'s paged command ingress derives its page and byte
# authorities from, so no transport here re-chooses a ceiling the guest allocator already fixes.
# Zero-dependency, pure std, wasm32-wasip2-safe (the guest plugin crate already links it).
semio-framework-trace = { workspace = true }

# 🌉️ Browser-only bridges reached through this crate's mounted modules (`🪪️identity`'s
# `crypto.getRandomValues` entropy arm, `📇️directory/🔌️client`'s experimental `browser` transport,
# `🏪️store/🔄️sync`'s `wasm_actor`, `🏪️store/👷️worker`'s Web Worker `postMessage` bridge) plus the
# `wasm_bindgen_futures` alias their async-fn codegen needs. Excluded from `wasm32-wasip2` because
# `target_arch = "wasm32"` is TRUE for the WASI component target too. `fill_entropy`/`now_ms`
# needed genuine wasip2 arms instead — see their docstrings.
[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]
js-sys = "0.3.83"
wasm-bindgen = "0.2.106"
web-sys = { version = "0.3.98", features = ["Window", "Storage", "console", "WebSocket", "MessageEvent", "CloseEvent", "BinaryType", "Request", "RequestInit", "Response", "Headers"] }

# 🧵️ Host-only tokio + zip, presentation on native AND browser wasm32, absent only from the shipped
# `wasm32-wasip2` component target — `target_arch = "wasm32"` is TRUE for wasm32-wasip2 too, so the
# full `not(all(target_arch = "wasm32", target_env = "p2")))` form is required (see the module-level
# docstring on `[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]`
# above for why a bare arch gate is wrong). `tokio` backs the `sync` cargo feature's native
# `tokio-tungstenite` transport and (base `sync`/`macros` tokio features only) the browser
# `📇️directory/🔌️client::browser` WebSocket bridge's `tokio::sync::mpsc` channel — both entirely
# outside a WASI guest's reach (`🔄️sync/🦀️.rs`'s own docstring: "WASI-P2 plugins never link
# this crate ... This actor is a host-side concern only"; the browser bridge is
# `cfg(all(target_arch = "wasm32", not(target_env = "p2")))`-gated already). `semio-framework-deflate` (its first-party
# `zip_archive`) backs `🧩️extension`'s `.sxt` package pack/unpack/verify — installing a runtime extension is host tooling
# (only caller repo-wide: `semio-framework-os`'s native host crate), never something a guest
# component does to itself; its module mount is gated the same way in `🦀️.rs`.
[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]
tokio = { version = "1", features = ["sync", "macros"], default-features = false }
semio-framework-deflate = { path = "../../../../🔨️modules/🗜️deflate/📦️packages/🦀️rust" }

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
futures = "0.3"
tokio-tungstenite = { version = "0.26", optional = true, features = ["rustls-tls-webpki-roots"] }
# 🧵️ Same crate key as the `not(all(target_arch = "wasm32", target_env = "p2"))` table above — cargo
# unifies same-name deps declared in multiple matching `[target.cfg(...)]` tables into one edge for
# a given target, unioning features (non-optional here to match that table's non-optional bit; a
# rename-alias or a mixed optional/non-optional pairing under one name both hit cargo's "multiple
# times with different names" / conflicting-optionality rejections). `net` is therefore always
# presentation for non-wasm32 builds of this crate, independent of the `sync` feature toggle — harmless
# (tokio without `sync`'s other deps never spins up a listener) and the only way to keep `net` off
# wasm32 unconditionally (both browser and wasip2), which is the actual requirement: no wasm32 target
# of this crate should ever link tokio's networking.
tokio = { version = "1", default-features = false, features = ["net"] }
# 🌀️ `HttpPool`/`ComputePool` and the owned file-change watcher. This remains optional because all
# consumers are native sync-runtime paths; this crate names only OS-services-owned types.
semio-framework-os-services = { workspace = true, optional = true }
# 🪪️ `PackageId`/`ActorId` — `HttpPool::request`'s own per-package/per-actor quota identity
# (`semio-framework-os-services`'s doc: "consumed from here rather than reinvented locally"). Pure,
# no tokio, but only the native `HttpPool`-backed transport constructs one, so it rides the same
# `ureq` feature as `semio-framework-os-services` above.
semio-framework-actor = { workspace = true, optional = true }
# 🌐️ The native `NativeDirectoryTransport`'s blocking HTTP client (rustls + ring). Every use site is
# `cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))`, and ring's `getrandom 0.2`
# does not build for browser wasm32, so the `sync`/`ureq` features name it as a native-only edge.
ureq = { version = "2", optional = true }

[[bin]]
name = "pack"
path = "../../🔨️modules/🎒️pack/⌨️cli/💾️binary/🦀️.rs"
required-features = ["native-bin"]

[[bin]]
name = "spr"
path = "../../🔨️modules/📡️spr/⌨️cli/💾️binary/🦀️.rs"
required-features = ["native-bin"]

[[bin]]
name = "semio"
path = "../../🔨️modules/🧬️semio/🏗️bootstrap/🦀️.rs"
required-features = ["native-bin"]

[dev-dependencies]
semio-framework-schema = { workspace = true }
blake3 = "1.8.2"
semio-framework-pack = { workspace = true, features = ["corruption-testing"] }
semio-framework-async-macros = { path = "../../../../🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }

````

## Registered Initial RED And Exact Authored Route

Actual uncached test-decode-ownership ran allfour ownership laws plus both original TypeScript decode laws:4pass/2intendedfail/132assertions/256msBun/2.1sNx. Missing lower native mount and retained higher duplicate mount are the two REDs. Schema/AJV/owned validator, whole native/control bytes, original fixtures and two actualSQLite/reference behavioral laws passed. Preparation also detected launch19already occupied by the concurrent lower DSL route; it refused to overwrite, preserved DSL19/Compiler20, and registers this route at verified-vacant21. The original defaultValue portable TestScript guard now specifically inspects that class, preserving its original construction test and requiring the decode corpus there; earlier simplewhole-script substring onlyproved the newownership route. A fresh registered RED precedes that additional route fix.

## Stronger Registered RED And Lower Native Source-Ready

The improved real portable-class discovery assertion produced3pass/3intendedfail/133assertions/234msBun/1.6sNx before source changes. The neutral NativeDecodeControl module now mounts its two original native laws under cfg(test); those laws explicitly import the two owned neutral control/progress types. Removing only that import transform and additive mount reproduces both complete original files. Default portable Value execution retains its original construction law and adds the existing complete decode corpus with its own30s stage. Fifteen current input records are stable in the source-ready receipt, with the original unfiltered113-law package cohort plus two original previously-higher control laws. Higher duplicate mount remains pending until actual independent lower discovery/execution. Native runtime and final higher retirement are not yet proved.

## Lower Discovery And Closed Admission Follow-Up

After the two declared lower edits and portable ownership fix, actual original source/reference route is5pass/1expectedfail/137assertions/379msBun/3.0sNx. Only intentionally retained higher duplicate mount remains before the independent native proof. Independent Low validated the two whole native/control originals, seven unchanged inputs, five actual Rust include targets, self crate alias and Node module lookup; native execution remains required. Its closed-schema counterexamples exposed empty paths/duplicate names: new actual schema RED4pass/2fail/138assertions/232msBun/1.7sNx, with schema admission and pending duplicate retirement failures. Paths now require nonempty input and native roster requires both known unique names; excess roster, nested unknown field and malformed digest are tested against AJV and owned validation. Original behavioral corpus remains unchanged.

## Current Closed Lower Proof and Actual Physical Absence

The lower proof previously imported a Repo product Rust scanner and read an OS product source. Both dependencies are removed: higher duplicate retirement is asserted under the actual OS snapshot owner, preserving the existing higher portable roster. The lower closed witness now binds the actual Cargo library path (Bun.TOML agrees with @iarna/toml), captured package/value roots, original two full native law bodies, five unchanged full inputs and the original two complete SQLite reference sources under exactly one owned binding-array literal edit each. The entire original two portable control laws are included.

Actual strict TypeScript admission first failed with TS2345 at both original SQLite Database.run variadic calls; both are now passed explicit typed binding arrays with unchanged SQL and scalar arguments. Their complete original sources are captured at sqlite-bindings-before.json and reproduced by the registered inverse law. No hash reset hides this source change. Actual registered cargo-root-green-attempt-2.log: strict TSC passed,8 portable laws passed/0 failed/169 assertions,542ms Bun and3.4s uncached Nx. The physically products/S/Hub-absent child executed7 laws/0 failures/166 assertions in214ms Bun. This is an actual standalone Bun child, not isolated Nx or whole-framework deletion. The input projection borrows third-party test packages through normal ancestor module lookup, copies no product or specialization sources, rejects symlinks and is removed after terminal execution.

NativeSourceREADY2 includes19 current full length/SHA records, including all original15 physical inputs and four proof files; all Rust source hashes match SourceREADY1 exactly. Script and only two SQLite reference sources changed among the original15. Full original Value113 plus the two actual lower control law discovery/runtime remains pending native execution; higher duplicate remains pending.

## Full Original Value Runtime and Higher Duplicate Retirement

Native queue independently executed the original complete uncached Value target:115 actual laws run/pass,0 skipped,2.02s compile,0.499s runtime,5.1s Nx. Executed binary roster comparison against earlier full113 cohort proves zero original removals and exactly the two previously-higher lower control law names, each once. All58 actual rustc.d owned input length/BLAKE3 rows and19 source-ready2 inputs matched current bytes before/after. Actual native binary receipt and roster comparisons are in current-native-worker generated receipts.

Only after that runtime proof, Root removed exactly the two-line higher native_materialization mount. The complete current pre-retirement higher caller is captured in higher-retirement-before.json, and reinserting that exact mount after its unchanged store export reproduces every byte. All retained native_binding/product controlled binding/projection/retirement/encoding/octet/envelope/schema mounts and law bodies stay exact. This retires duplicate discovery without dropping the original laws. Actual higher portable admission before removal was RED; native higher composition rerun remains queued.
