import re
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R187"
t=open(S+"/base/d.ts").read()
def once(a,b):
    global t
    assert t.count(a)==1,(a[:80],t.count(a));t=t.replace(a,b)
docs=re.findall(r'/\*\* (\S+) ',t)
for e in ['⏳️','🧮️','🚧️','📝️']: assert docs.count(e)==0,e
once('\n  const call=<R>(operation:()=>R):R=>{checkpoint();const result=operation();','\n  const lineage=new WeakMap<ArtifactWindowsRelativeHandleV1,{parent:ArtifactWindowsRelativeHandleV1;name:string}>();\n  const call=<R>(operation:()=>R):R=>{checkpoint();const result=operation();')
once('const owned=handle(value);state.uncertain=false;state.retained=state.retained.filter(value=>!buffers.includes(value));return owned;},info=','const owned=handle(value);lineage.set(owned,{parent,name});state.uncertain=false;state.retained=state.retained.filter(value=>!buffers.includes(value));return owned;},info=')
once('const result=handle(output);state.uncertain=false;state.retained=state.retained.filter(value=>!buffers.includes(value));return result;};','const result=handle(output);lineage.set(result,{parent:owned,name});state.uncertain=false;state.retained=state.retained.filter(value=>!buffers.includes(value));return result;};')
once('role==="directory"||role==="ancestor"?p.shareRead|p.shareWrite:p.shareRead,p.fileOpen,options','role==="directory"||role==="ancestor"?p.shareRead|p.shareWrite|p.shareDelete:p.shareRead|p.shareDelete,p.fileOpen,options')
once('isDirectory?p.shareRead|p.shareWrite:p.shareRead,c.fileCreate','isDirectory?p.shareRead|p.shareWrite|p.shareDelete:p.shareRead|p.shareDelete,c.fileCreate')
once('p.genericRead,p.shareRead,null,3,p.openReparseWin32','p.genericRead,p.shareRead|p.shareDelete,null,3,p.openReparseWin32')
old='const c=exportedSchema.definitions.RelativeBindingProtocol.const.windows,encoded=Buffer.from(preservationPolicy.payload,"utf16le"),packet=Buffer.alloc(c.renameMinimumBytes+encoded.length),io=Buffer.alloc(p.statusBytes);packet.writeBigUInt64LE(child.owned.value,c.renameRootOffset);packet.writeUInt32LE(encoded.length,c.renameNameLengthOffset);encoded.copy(packet,c.renameNameOffset);const status=statusCall(source,[packet,io],()=>nt.symbols.NtSetInformationFile!(source.value,io,packet,packet.length,c.renameClass)as number);if(status!==0||io.readBigUInt64LE(8)!==0n)throw failure("preservation rename",status);bindingAdopt();child.adopt();'
new='''const c=exportedSchema.definitions.RelativeBindingProtocol.const.windows,settlement=exportedSchema.definitions.RelativeRenameSettlement.const,encoded=Buffer.from(preservationPolicy.payload,"utf16le"),base=root.replace(/\\\\+$/u,""),sourceText=base+"\\\\"+name;
      await settleArtifactWindowsRelativeRenameV1({source:sourceText,target:base+"\\\\"+capsule+"\\\\"+preservationPolicy.payload,ownHandles:()=>artifactWindowsRelativeHandlesBelowV1(state.handles,item=>lineage.get(item),source),
       attempt:method=>{guard();const packet=Buffer.alloc(c.renameMinimumBytes+encoded.length),io=Buffer.alloc(p.statusBytes);if(method==="posix-ex")packet.writeUInt32LE((settlement.flags.posixSemantics|settlement.flags.ignoreReadonlyAttribute)>>>0,0);packet.writeBigUInt64LE(child.owned.value,c.renameRootOffset);packet.writeUInt32LE(encoded.length,c.renameNameLengthOffset);encoded.copy(packet,c.renameNameOffset);const status=statusCall(source,[packet,io],()=>nt.symbols.NtSetInformationFile!(source.value,io,packet,packet.length,method==="posix-ex"?c.renameExClass:c.renameClass)as number);if(status===0&&io.readBigUInt64LE(8)!==0n)throw failure("preservation rename",status);return status;},
       sleep:milliseconds=>new Promise<void>(done=>setTimeout(done,milliseconds)),now:()=>performance.now(),checkpoint:guard,fail:status=>failure("preservation rename",status),
       holders:()=>{const bytes=Buffer.alloc(8+8*settlement.maximumHolders*4),io=Buffer.alloc(p.statusBytes),status=statusCall(source,[bytes,io],()=>nt.symbols.NtQueryInformationFile!(source.value,io,bytes,bytes.length,settlement.holderClass)as number);if(status!==0)return[];const rows:string[]=[];for(let index=0,count=Math.min(bytes.readUInt32LE(0),(bytes.length-8)/8);index<count;index++){const pid=bytes.readBigUInt64LE(8+index*8);if(pid!==BigInt(process.pid))rows.push("process "+pid);}return rows;},
       report:row=>{process.stderr.write("[DATA] Windows preservation rename waiting "+JSON.stringify({source:sourceText,...row})+"\\n");}});
      bindingAdopt();child.adopt();'''
once(old,new)
block='''/** 🚧️ Names one Windows move that stayed refused: both texts, the NT status, the evidence and whether the holder is this command or a foreign program. */
export class ArtifactWindowsRelativeRenameRefusedErrorV1 extends Error{
 readonly code="SEMIO_WINDOWS_RENAME_REFUSED";readonly reason:"foreign-handles"|"own-handles";readonly texts:Readonly<{en:string;de:string}>;readonly status:number;readonly source:string;readonly target:string;readonly attempts:number;readonly waitedMilliseconds:number;readonly holders:readonly string[];
 constructor(row:ArtifactWindowsRelativeRenameRowV1){const texts=artifactWindowsRelativeRenameTextsV1(row);super(texts.en);this.name="WindowsRenameRefused";this.reason=row.reason;this.texts=texts;this.status=row.status>>>0;this.source=row.source;this.target=row.target;this.attempts=row.attempts;this.waitedMilliseconds=row.waitedMilliseconds;this.holders=row.holders;}
}
/** 📊️ The evidence of one refused move. */
export type ArtifactWindowsRelativeRenameRowV1=Readonly<{reason:"foreign-handles"|"own-handles";status:number;source:string;target:string;attempts:number;waitedMilliseconds:number;holders:readonly string[];count:number}>;
/** 📝️ Renders the refusal in English and German from the schema texts; the status is named by its NT name and in words, unknown holders are said so. */
export function artifactWindowsRelativeRenameTextsV1(row:ArtifactWindowsRelativeRenameRowV1):Readonly<{en:string;de:string}>{
 const policy=exportedSchema.definitions.RelativeRenameSettlement.const,names=exportedSchema.definitions.RelativeWindowsFailureNames.const.nt as Record<string,string>,words=policy.statusWords as Record<string,Record<string,string>>,status=row.status>>>0,hex="0x"+status.toString(16),clean=(item:string,units:number)=>item.replace(/[\\u0000-\\u001f\\u007f]/gu,"?").slice(0,units),text=(language:"en"|"de")=>{const catalog=policy.texts[language],values:Record<string,string>={source:clean(row.source,300),target:clean(row.target,300),status:hex,statusName:names[String(status)]??hex,words:words[String(status)]?.[language]??hex,attempts:String(row.attempts),waited:String(Math.round(row.waitedMilliseconds)),holders:row.holders.length?row.holders.slice(0,policy.maximumHolders).map(item=>clean(item,policy.maximumHolderUnits)).join(", "):catalog.unknown,count:String(row.count)};return catalog[row.reason==="own-handles"?"own":"foreign"].replace(/\\{(\\w+)\\}/gu,(_,key:string)=>values[key]??"");};
 return Object.freeze({en:text("en"),de:text("de")});
}
/** 🧮️ Counts the handles this process still holds at the entry through another handle on the same name or anywhere below it; every one must be closed before the entry moves. */
export function artifactWindowsRelativeHandlesBelowV1<H extends object>(open:Iterable<H>,lineage:(item:H)=>Readonly<{parent:H;name:string}>|undefined,entry:H):number{
 const own=lineage(entry);let count=0;
 for(const item of open){if(item===entry)continue;let current:H|undefined=item;for(let depth=0;current&&depth<256;depth++){if(current===entry){count++;break;}const link=lineage(current);if(!link)break;if(own&&link.parent===own.parent&&link.name===own.name){count++;break;}current=link.parent;}}
 return count;
}
/** 🔌️ What one Windows move needs from the platform: one attempt per method, time, cancellation, own-handle census and the optional holder names. */
export type ArtifactWindowsRelativeRenamePortV1=Readonly<{source:string;target:string;ownHandles():number;attempt(method:"posix-ex"|"classic"):number;sleep(milliseconds:number):Promise<void>;now():number;checkpoint():void;fail(status:number):Error;holders?():readonly string[];report?(row:Readonly<{attempt:number;status:number;method:string;waitedMilliseconds:number;nextMilliseconds:number}>):void}>;
/** ⏳️ Moves with POSIX semantics where the volume has them, falls back to the classic rename, retries a transient refusal with a doubling pause up to the schema budget, and then refuses typed; a handle of this command below the source is refused at once. */
export async function settleArtifactWindowsRelativeRenameV1(port:ArtifactWindowsRelativeRenamePortV1):Promise<Readonly<{method:"posix-ex"|"classic";attempts:number;waitedMilliseconds:number}>>{
 const policy=exportedSchema.definitions.RelativeRenameSettlement.const,started=port.now();let method:"posix-ex"|"classic"="posix-ex",delay=policy.initialMilliseconds,attempts=0,status=0,reported=-Infinity;
 const refuse=(reason:"foreign-handles"|"own-handles",count:number)=>{let holders:readonly string[]=[];if(reason==="foreign-handles"&&port.holders)try{holders=[...port.holders()].slice(0,policy.maximumHolders);}catch{}return new ArtifactWindowsRelativeRenameRefusedErrorV1({reason,status,source:port.source,target:port.target,attempts,waitedMilliseconds:port.now()-started,holders,count});};
 for(;;){
  port.checkpoint();const own=port.ownHandles();if(own>0)throw refuse("own-handles",own);
  attempts++;status=port.attempt(method);if(method==="posix-ex"&&policy.fallbackStatuses.includes(status)){method="classic";status=port.attempt(method);}
  if(status===0)return Object.freeze({method,attempts,waitedMilliseconds:port.now()-started});
  if(!policy.transientStatuses.includes(status))throw port.fail(status);
  const waited=port.now()-started,remaining=policy.budgetMilliseconds-waited;if(remaining<=0)throw refuse("foreign-handles",0);
  const pause=Math.min(delay,remaining);if(port.report&&waited-reported>=policy.reportEveryMilliseconds){reported=waited;port.report({attempt:attempts,status,method,waitedMilliseconds:waited,nextMilliseconds:pause});}
  await port.sleep(pause);delay=Math.min(delay*policy.growth,policy.capMilliseconds);
 }
}
'''
once('/** 🩺️ Names a Windows relative failure by code',block+'/** 🩺️ Names a Windows relative failure by code')
open(S+"/stage/d.ts","w").write(t)
print("ok",len(t))
