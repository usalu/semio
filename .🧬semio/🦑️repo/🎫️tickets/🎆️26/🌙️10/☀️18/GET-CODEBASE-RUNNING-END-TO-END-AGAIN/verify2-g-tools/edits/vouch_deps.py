"""Batch `vouch`, part 2: the dependency generation verification asks the vouch instead of walking, and ensureBuildEnvironmentV1 shares ONE vouch between its `fast` and `recheck` passes."""
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
G = L + "/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
# neutral verifier
sub(G + "/🟦️.ts", ' readonly producer:()=>GenerationProducerV1;readonly recheck?:boolean;\n',
    ' readonly producer:()=>GenerationProducerV1;readonly recheck?:boolean;\n /** 🤝️ Answers which of the given workspace-relative paths a change journal proves untouched since the receipt was last verified; the verifier then skips their rows. Without it, or with an empty answer, every row is looked at. */\n readonly vouched?:(paths:readonly string[])=>Promise<ReadonlySet<string>>|ReadonlySet<string>;\n')
sub(G + "/🟦️.ts", '''  for(const group of receipt.inputs)for(const source of group.sources){
   await advance();const path=generationResolveV1(source.path,workspace,storeRoot),facts=lstatBig(path);''',
'''  const asRelative=(absolute:string):string|undefined=>{const from=relative(workspace,absolute).split(sep).join("/");return from===""||from===".."||from.startsWith("../")||isAbsolute(from)?undefined:from;};
  const known=input.vouched?await input.vouched([...receipt.inputs.flatMap(group=>group.sources.map(source=>asRelative(generationResolveV1(source.path,workspace,storeRoot)))),...receipt.membership.texts.map(row=>asRelative(resolve(workspace,row.path)))].filter((path):path is string=>path!==undefined)):new Set<string>();
  for(const group of receipt.inputs)for(const source of group.sources){
   await advance();const path=generationResolveV1(source.path,workspace,storeRoot);if(known.has(asRelative(path)??""))continue;const facts=lstatBig(path);''')
sub(G + "/🟦️.ts", '''  for(const row of receipt.membership.texts){
   await advance();let text:string;''', '''  for(const row of receipt.membership.texts){
   await advance();if(known.has(asRelative(resolve(workspace,row.path))??""))continue;let text:string;''')
sub(G + "/🟦️.ts", '''  for(const row of rows)if(row[0]==="d")flags.set(row[1]as string,row[10]as number);''', '''  for(const row of rows)if(row[0]==="d")flags.set(row[1]as string,row[10]as number);
  const settled=input.vouched?await input.vouched(rows.map(row=>row[1]as string)):new Set<string>();''')
sub(G + "/🟦️.ts", '''  for(const row of rows){
   await advance();const path=resolve(workspace,row[1]as string),facts=lstatBig(path);''', '''  for(const row of rows){
   if(settled.has(row[1]as string)){if(++work%protocol.checkWork===0)control.check();if(work%protocol.progressWork===0)await control.tick(work);continue;}
   await advance();const path=resolve(workspace,row[1]as string),facts=lstatBig(path);''')
# storage glue
S = G + "/💾️storage/🟦️.ts"
sub(S, 'import type {RepositoryCacheStoreV1} from', 'import type {VouchV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";\nimport type {RepositoryCacheStoreV1} from')
sub(S, 'receipt:GenerationReceiptV1,subject:JavascriptDependencyGenerationSubjectV1,recheck:boolean)=>Object.freeze({\n workspace,storeRoot:context.root,receipt,recheck,',
       'receipt:GenerationReceiptV1,subject:JavascriptDependencyGenerationSubjectV1,recheck:boolean,vouched?:(paths:readonly string[])=>Promise<ReadonlySet<string>>)=>Object.freeze({\n workspace,storeRoot:context.root,receipt,recheck,...(vouched?{vouched}:{}),')
sub(S, 'options:Readonly<{recheck?:boolean}>={}):Promise<JavascriptDependencyGenerationVerifyV1>{', 'options:Readonly<{recheck?:boolean;vouch?:(receiptSha256:string)=>Promise<VouchV1|undefined>}>={}):Promise<JavascriptDependencyGenerationVerifyV1>{')
sub(S, '  if(receipt.scope!==scope)continue;\n  const result=await verifyGenerationReceiptV1(portsOf(root,context,control,controlOf(context,control,"shared"),', '  if(receipt.scope!==scope)continue;\n  const vouch=options.vouch?await options.vouch(receipt.sha256):undefined,vouched=vouch?(paths:readonly string[])=>options.recheck===true?vouch.settled(paths,control.signal):Promise.resolve(vouch.skippable(paths)):undefined;\n  const result=await verifyGenerationReceiptV1(portsOf(root,context,control,controlOf(context,control,"shared"),')
sub(S, 'receipt,subject,options.recheck===true));', 'receipt,subject,options.recheck===true,vouched));')
# library wiring (new file)
put(J + "/🪪️vouch/🟦️.ts", '''import {join} from "node:path";
import {canonicalRootV1} from "../../📨️protocol/📍️endpoint/🟨️.mjs";
import {VOUCH_POLICY_V1,openVouchV1,type VouchV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";
import {changeJournalDirectoryV1,connectChangeJournalV1} from "../🟦️.ts";

/** 🪪️ Opens the vouch of one verification of a workspace against its change journal: the record lives in the private runtime directory of the journal, so it dies with the journal's epoch and never reaches the checkout. */
export function openJournalVouchV1(root:string,consumer:string,key:string,environment:Readonly<Record<string,string|undefined>>=process.env,signal?:AbortSignal):Promise<VouchV1>{
 const canonical=canonicalRootV1(root);
 return openVouchV1({client:connectChangeJournalV1(canonical,environment),directory:join(changeJournalDirectoryV1(canonical,environment),VOUCH_POLICY_V1.directory),consumer,key,...(signal?{signal}:{})});
}
''')
# bootstrap
BS = B + "/🚀️bootstrap/🟦️.ts"
sub(BS, 'import {profilePhaseV1} from "../⏱️profile/🟨️.mjs";', 'import {profileMarkV1,profilePhaseV1} from "../⏱️profile/🟨️.mjs";')
sub(BS, 'fast=await profilePhaseV1("dependencies.verify",()=>generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,scope));',
        'journal=publishReceipt||cheap?undefined:dependencyVouchV1(workspace,scope,actualControl.signal),fast=await profilePhaseV1("dependencies.verify",()=>generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,scope,undefined,journal?{vouch:journal.open}:{}));if(journal)profileMarkV1("dependencies.vouch",await journal.describe());')
sub(BS, 'generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,scope,undefined,{recheck:true}));if(("sha256" in confirmed?confirmed.sha256:undefined)!==owner.sha256)throw new Error("Current parser Source-owner custody changed");',
        'generation.verifyJavascriptDependencyGenerationReceiptV1(workspace,context,actualControl,scope,undefined,{recheck:true,...(journal?{vouch:journal.open}:{})}));if(("sha256" in confirmed?confirmed.sha256:undefined)!==owner.sha256)throw new Error("Current parser Source-owner custody changed");if(journal&&fast.status==="match")await journal.seal();')
sub(BS, '\nimport {javascriptInstallSchemaV1 as dependencyInstallSchema}', '''
/** 🪪️ The vouch of one `ensureBuildEnvironmentV1` look at the dependency receipt: `open` hands the same vouch to the complete pass and to the recheck of one receipt, `describe` names how it was obtained for the profile, `seal` stores the journal position once both passes held. A journal that cannot vouch (switched off, no service, gap, overflow, restart) leaves both passes complete walks. */
export function dependencyVouchV1(workspace:string,scope:string,signal:AbortSignal):Readonly<{open:(key:string)=>Promise<import("../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts").VouchV1|undefined>;describe:()=>Promise<string>;seal:()=>Promise<void>}>{
 const opened=new Map<string,Promise<import("../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts").VouchV1|undefined>>();let last:Promise<import("../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts").VouchV1|undefined>|undefined;
 return Object.freeze({
  open:(key:string)=>{let vouch=opened.get(key);if(!vouch){vouch=import("../🏡️resident/📜️journal/🪪️vouch/🟦️.ts").then(module=>module.openJournalVouchV1(workspace,"dependencies:"+scope,key,process.env,signal)).catch(()=>{signal.throwIfAborted();return undefined;});opened.set(key,vouch);}last=vouch;return vouch;},
  describe:async()=>{const vouch=await last;return vouch===undefined?"off":vouch.mode+(vouch.refusal===undefined?"":":"+vouch.refusal);},
  seal:async()=>{try{await(await last)?.seal();}catch{}},
 });
}

import {javascriptInstallSchemaV1 as dependencyInstallSchema}''')

# decoded evidence reused by the second look of the same pass
sub(G + "/🟦️.ts", '''const decodeRows=(bytes:Uint8Array,expected:GenerationEvidenceV1):(readonly unknown[])[]=>{
 if(bytes.length!==expected.bytes||generationSha256V1(bytes)!==expected.sha256)throw Error("Sealed generation evidence required");
 const text=new TextDecoder("utf-8",{fatal:true}).decode(bytes),rows=text.length?text.slice(0,-1).split("\\n").map(line=>JSON.parse(line)as unknown[]):[];
 if(rows.length!==expected.rows||rows.some(row=>!Array.isArray(row)))throw Error("Complete generation evidence rows required");
 return rows;
};''', '''let decodedLast:{sha256:string;rows:(readonly unknown[])[];timer:ReturnType<typeof setTimeout>}|undefined;
const decodeRows=(bytes:Uint8Array,expected:GenerationEvidenceV1):(readonly unknown[])[]=>{
 if(bytes.length!==expected.bytes||generationSha256V1(bytes)!==expected.sha256)throw Error("Sealed generation evidence required");
 if(decodedLast?.sha256===expected.sha256)return decodedLast.rows;
 const text=new TextDecoder("utf-8",{fatal:true}).decode(bytes),rows=text.length?text.slice(0,-1).split("\\n").map(line=>JSON.parse(line)as unknown[]):[];
 if(rows.length!==expected.rows||rows.some(row=>!Array.isArray(row)))throw Error("Complete generation evidence rows required");
 if(decodedLast)clearTimeout(decodedLast.timer);
 const timer=setTimeout(()=>{decodedLast=undefined;},protocol.decodedRowsRetainMilliseconds);timer.unref?.();
 decodedLast={sha256:expected.sha256,rows,timer};
 return rows;
};''')
sub(G + "/🧬️schema/🔣️.json", '        "progressWork": 4096,\n', '        "progressWork": 4096,\n        "decodedRowsRetainMilliseconds": 30000,\n')
