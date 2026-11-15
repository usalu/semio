import sys,os
HERE=os.path.dirname(os.path.abspath(__file__))
def rd(name):return open(os.path.join(HERE,"parts",name),encoding="utf8").read()
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
PUB=LIB+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication"
BOOT=LIB+"/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"
PUBTS=PUB+"/🟦️.ts"
SETUP=PUB+"/⚙️setup/🟦️.ts"
FUNDING=PUB+"/⚙️setup/💰️funding/🟦️.ts"
SCHEMA=PUB+"/⚙️setup/🧬️schema.json"
TEXTS=LIB+"/⚡️caching/📖️texts/🔣️.json"
BUSYTEST=PUB+"/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts"
DRIVERTEST=PUB+"/⚙️setup/♻️retirement/🧪️tests/🔁️busy-driver/🟦️.ts"
OBSTEST="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📸️observation/🧪️tests/🟦️.ts"
RETLIVE=PUB+"/⚙️setup/♻️retirement/🟦️.ts"
NEWTEST=PUB+"/⚙️setup/🧪️tests/🌊️moving-tree/🟦️.ts"
TESTS=[NEWTEST,PUB+"/⚙️setup/🧪️tests/🟦️.ts",BUSYTEST,DRIVERTEST,PUB+"/⚙️setup/♻️retirement/🧪️tests/🟦️.ts",LIB+"/⚡️caching/📖️texts/🧪️tests/🟦️.ts",OBSTEST]
TITLE="R6b landing (the dependency funding is bound only to what it consumes: a package directory is judged by its identity, not by its size, links and times, so an edit anywhere in the shared tree no longer fails a 17-minute install; a real change is named and re-run a bounded number of times, then ends in a typed exit 75 refusal)"

BOOT_OLD='export function sameJavascriptDependencyParentIdentityV1('
BOOT_NEW='''/** 🧭️ Names the first thing that moved between the Source binding a producer captured and the one it observes now, judging only what the producer consumes: an endpoint row and an ancestor must be unchanged, a package directory by its identity alone when `parents` is "identity" (an edit in it changes its size, links and times, never the endpoint) and exactly otherwise; null when nothing consumed moved. */
export function javascriptDependencySourceMovementV1(expected:Readonly<{rows:readonly unknown[];parents:readonly Readonly<{path:string;identity:string}>[];ancestors?:readonly Readonly<{path:string;identity:string}>[]}>,observed:Readonly<{rows:readonly unknown[];parents:readonly Readonly<{path:string;identity:string}>[];ancestors?:readonly Readonly<{path:string;identity:string}>[]}>,parents:"identity"|"stamp"):Readonly<{what:"rows"|"parents"|"ancestors";path:string}>|null{
 const nameOf=(row:unknown,index:number):string=>{const value=row as{sourceDirectory?:unknown;path?:unknown}|undefined;return String(value?.sourceDirectory??value?.path??"#"+index);},first=<T>(left:readonly T[],right:readonly T[],same:(a:T,b:T)=>boolean):string|null=>{for(let index=0;index<Math.max(left.length,right.length);index++){const a=left[index],b=right[index];if(a===undefined||b===undefined||!same(a,b))return nameOf(a??b,index);}return null;};
 const rows=first(expected.rows,observed.rows,(a,b)=>JSON.stringify(a)===JSON.stringify(b));if(rows!==null)return Object.freeze({what:"rows"as const,path:rows});
 const parent=first(expected.parents,observed.parents,(a,b)=>a.path===b.path&&(parents==="identity"?sameJavascriptDependencyParentIdentityV1(a.identity,b.identity):a.identity===b.identity));if(parent!==null)return Object.freeze({what:"parents"as const,path:parent});
 const ancestor=first(expected.ancestors??[],observed.ancestors??[],(a,b)=>a.path===b.path&&a.identity===b.identity);return ancestor===null?null:Object.freeze({what:"ancestors"as const,path:ancestor});
}
export function sameJavascriptDependencyParentIdentityV1('''

PUB_IMPORT_OLD='import{readJavascriptDependencyPreservationSourceV1,javascriptDependencyPreservationIntentV1,readJavascriptDependencyPreservingCompletionRequestV1}from'
PUB_IMPORT_NEW='import{javascriptDependencySourceMovementV1}from"../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";\nimport{refuseCachingV1}from"../../../../../📖️texts/🟦️.ts";\n'+PUB_IMPORT_OLD
BOUNDARY_OLD=rd("boundary-old.txt")
BOUNDARY_NEW=rd("boundary-new.txt")

FUND_OLD='/** 🔒️ Owns one freshly captured genuine parent for one attempt'
FUND_NEW='''/** 🌊️ How a funding is re-run when the shared tree moved under it: bounded by the schema, with the pauses it declares. */
export type DependencyFundingMovePortsV1=Readonly<{sleep(milliseconds:number,signal:AbortSignal):Promise<void>;say(attempt:number,limit:number,what:string,path:string,seconds:number):void}>;
const fundingMovement=setupSchema.definitions.FundingMovementProtocol.const,fundingMovePorts:DependencyFundingMovePortsV1=Object.freeze({sleep:(milliseconds,signal)=>dependencyStoreWaitPorts.sleep(milliseconds,signal),say:(attempt,limit,what,path,seconds)=>sayCachingV1("dependency-store.funding-rerun",{attempt,limit,what,path,seconds})});
/** 🔭️ Whether a failure of the funding says that something it consumes moved under it, found in the typed movement refusal or in the stable text of the input recheck anywhere in the chain of causes and aggregate members; names what moved. */
export function dependencyFundingMovedV1(error:unknown):Readonly<{what:string;path:string}>|undefined{
  const seen=new Set<unknown>(),walk=(value:unknown,depth:number):Readonly<{what:string;path:string}>|undefined=>{
    if(depth>16||value===null||typeof value!=="object"||seen.has(value))return undefined;seen.add(value);
    if(isRepoErrorV1(value)&&value.key===fundingMovement.key)return Object.freeze({what:String(value.values.what),path:String(value.values.path)});
    const row=value as{message?:unknown;cause?:unknown;errors?:unknown};
    if(typeof row.message==="string"&&fundingMovement.signals.some(text=>(row.message as string).includes(text)))return Object.freeze({what:"inputs",path:row.message.slice(0,160)});
    for(const next of[row.cause,...Array.isArray(row.errors)?row.errors:[]]){const found=walk(next,depth+1);if(found)return found;}
    return undefined;
  };
  return walk(error,0);
}
/** 🔁️ Runs a funding again when what it consumes moved under it, a bounded number of times with the declared pauses; anything else ends it at once, and past the bound it ends in a typed bilingual exit 75 refusal that names what moved. */
export async function retryWhileDependencyFundingMovesV1<T>(control:ResourceLeaseControl,attempt:()=>Promise<T>,ports:DependencyFundingMovePortsV1=fundingMovePorts):Promise<T>{
  for(let run=0;;run++){
    checkResourceLeaseControl(control);
    try{return await attempt();}catch(error){
      const moved=dependencyFundingMovedV1(error);if(!moved)throw error;
      if(run>=fundingMovement.maximumReruns)throw refuseCachingV1("unserved","dependency-store.moving",{attempts:run+1,what:moved.what,path:moved.path},{cause:error});
      const pause=fundingMovement.pausesMilliseconds[Math.min(run,fundingMovement.pausesMilliseconds.length-1)]!;ports.say(run+1,fundingMovement.maximumReruns,moved.what,moved.path,Math.round(pause/1000));await ports.sleep(pause,control.signal);
    }
  }
}
'''+FUND_OLD
FUND_IMPORT_OLD='import{repositoryCacheImagePublicationOptionsV1}from"../../🟦️.ts";'
FUND_IMPORT_NEW=FUND_IMPORT_OLD+'\nimport setupSchema from"../🧬️schema.json"with{type:"json"};\nimport{isRepoErrorV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";'

SETUP_IMPORT_OLD='retryWhileDependencyStoreBusyV1,dependencyStoreWaitPortsForV1,'
SETUP_IMPORT_NEW='retryWhileDependencyStoreBusyV1,retryWhileDependencyFundingMovesV1,dependencyStoreWaitPortsForV1,'
SETUP_OLD=' await retryWhileDependencyStoreBusyV1(control,async()=>{const parent=await observeRepositoryDependencyMaintenanceParentV1'
SETUP_NEW=' await retryWhileDependencyFundingMovesV1(control,()=>retryWhileDependencyStoreBusyV1(control,async()=>{const parent=await observeRepositoryDependencyMaintenanceParentV1'
SETUP_TAIL_OLD='resource:"native-fixed-image:"+options.directory})));checkResourceLeaseControl(control);\n await retireObsolete'
SETUP_TAIL_NEW='resource:"native-fixed-image:"+options.directory}))));checkResourceLeaseControl(control);\n await retireObsolete'

SCHEMA_OLD='  "definitions": {\n'
SCHEMA_NEW='''  "definitions": {
    "FundingMovementProtocol": {
      "const": {
        "key": "caching.dependency-store.funding-moving",
        "maximumReruns": 3,
        "pausesMilliseconds": [2000, 8000, 30000],
        "signals": [
          "Original Bun dependency input changed during installation",
          "Original Bun dependency input appeared during installation"
        ]
      }
    },
'''

TEXTS_OLD='    "dependency-store.holder-gone": {'
TEXTS_NEW='''    "dependency-store.funding-moving": {
      "en": "The {what} of the dependency funding moved while it ran ({path})",
      "de": "{what} der Abhängigkeitsfüllung hat sich während des Laufs verändert ({path})"
    },
    "dependency-store.funding-rerun": {
      "en": "The {what} of the dependency funding moved ({path}); running it again, attempt {attempt} of {limit}, in {seconds} s",
      "de": "{what} der Abhängigkeitsfüllung hat sich verändert ({path}); sie läuft erneut, Versuch {attempt} von {limit}, in {seconds} s"
    },
    "dependency-store.moving": {
      "en": "The {what} of the dependency funding kept moving ({path}) and held still in none of {attempts} attempts; try again when the files it reads (manifests, lockfiles, patches) are not being edited",
      "de": "{what} der Abhängigkeitsfüllung hat sich immer wieder verändert ({path}) und blieb in keinem von {attempts} Versuchen stabil; erneut versuchen, wenn die gelesenen Dateien (Manifeste, Sperrdateien, Patches) nicht bearbeitet werden"
    },
'''+TEXTS_OLD

OLDSUB='await retryWhileDependencyStoreBusyV1(control,async()=>{const parent=await observeRepositoryDependencyMaintenanceParentV1'
NEWSUB='retryWhileDependencyStoreBusyV1(control,async()=>{const parent=await observeRepositoryDependencyMaintenanceParentV1'
EDITS=[(BOOT,BOOT_OLD,BOOT_NEW),(PUBTS,PUB_IMPORT_OLD,PUB_IMPORT_NEW),(PUBTS,BOUNDARY_OLD,BOUNDARY_NEW),(FUNDING,FUND_IMPORT_OLD,FUND_IMPORT_NEW),(FUNDING,FUND_OLD,FUND_NEW),(SETUP,SETUP_IMPORT_OLD,SETUP_IMPORT_NEW),(SETUP,SETUP_OLD,SETUP_NEW),(SETUP,SETUP_TAIL_OLD,SETUP_TAIL_NEW),(SCHEMA,SCHEMA_OLD,SCHEMA_NEW),(TEXTS,TEXTS_OLD,TEXTS_NEW),
 (BUSYTEST,'expect(setup).toContain("'+OLDSUB+'")','expect(setup).toContain("'+NEWSUB+'")'),
 (DRIVERTEST,'wait=setup.indexOf("'+OLDSUB+'")','wait=setup.indexOf("'+NEWSUB+'")'),
 (OBSTEST,'retryWhileDependencyStoreBusyV1:async(_actual:unknown,attempt:()=>Promise<{value:unknown}>)=>(await attempt()).value,','retryWhileDependencyStoreBusyV1:async(_actual:unknown,attempt:()=>Promise<{value:unknown}>)=>(await attempt()).value,retryWhileDependencyFundingMovesV1:async(_actual:unknown,attempt:()=>Promise<unknown>)=>attempt(),')]
NEWFILES={NEWTEST:os.path.join(HERE,"parts","moving-tree.ts")}
