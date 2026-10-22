import sys,os,json
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
M=L+"/🏗️build/🧠️memory/"
sp=M+"🧬️schema/🔣️.json"
raw=open(sp,encoding="utf8").read(); d=json.loads(raw)
pol=d["definitions"]["Policy"]["const"]
if "maximumCachedClosureEntries" not in pol:
    new={}
    for k,v in pol.items():
        new[k]=v
        if k=="maximumInternedCharacters": new["maximumCachedClosureEntries"]=4194304
    d["definitions"]["Policy"]["const"]=new
    ind=1 if json.dumps(json.loads(raw),ensure_ascii=False,indent=1)+"\n"==raw else 2
    open(sp,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=ind)+"\n")
    fx=M+"🧪️tests/🔣️.json"; f=json.load(open(fx,encoding="utf8")); f["policy"]=new
    open(fx,"w",encoding="utf8").write(json.dumps(f,ensure_ascii=False,indent=1)+"\n")
apply(M+"🟨️.d.mts",[('readonly maximumInternedCharacters:number;','readonly maximumInternedCharacters:number;readonly maximumCachedClosureEntries:number;')])
G=L+"/🏗️build/🕸️graph/🟦️.ts"
apply(G,[
 ('import{graphStringInternerV1}from"../🧠️memory/🟨️.mjs";','import{GRAPH_MEMORY_POLICY_V1,graphStringInternerV1}from"../🧠️memory/🟨️.mjs";'),
 ('const inventoryDirectoriesV1=new WeakMap<string[],string[]>();','''const inventoryDirectoriesV1=new WeakMap<string[],string[]>();

/** 🧊️ The expanded input lists of closure inventories this process already read, by repository and content address: a content addressed leaf expands to the same list in every chunk, so a chunked expansion reads each distinct closure once. Bounded by the declared entry count; over it the table starts again. */
const expandedClosuresV1=new Map<string,string[]>();let expandedClosureEntriesV1=0;
function rememberExpandedClosureV1(key:string,list:string[]):void{
  if(expandedClosureEntriesV1+list.length>GRAPH_MEMORY_POLICY_V1.maximumCachedClosureEntries){expandedClosuresV1.clear();expandedClosureEntriesV1=0;}
  expandedClosuresV1.set(key,list);expandedClosureEntriesV1+=list.length;
}'''),
 ('''        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        const directory=join(store.root,compact.directory),file=readInventoryRecordLeanV1(directory,compact.digest,exactSourceReadPolicyV1.maximumClosureBytes);
        if(file.sha256!==compact.digest)throw Error(`Build input closure content differs: ${compact.key}`);
        bytes+=file.byteLength;if(bytes>exactSourceReadPolicyV1.maximumSourceBytes)throw Error("Build input closure bytes exceeded");
        expanded.set(compact.key,compact.directory==="build-input-closures"?captureSourceInputInventoryV1(file.value,root).value.files.map(file=>graphStringInternerV1.share("{workspaceRoot}/"+file)):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+captureCargoLockInventoryV1(file.value,root).value.manifest]);
        rememberInventoryRecordV1(root,directory,compact.digest,file.stamp);''','''        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        const known=expandedClosuresV1.get(root+"\\0"+compact.key);
        if(known)expanded.set(compact.key,known);
        else{
          const directory=join(store.root,compact.directory),file=readInventoryRecordLeanV1(directory,compact.digest,exactSourceReadPolicyV1.maximumClosureBytes);
          if(file.sha256!==compact.digest)throw Error(`Build input closure content differs: ${compact.key}`);
          bytes+=file.byteLength;if(bytes>exactSourceReadPolicyV1.maximumSourceBytes)throw Error("Build input closure bytes exceeded");
          const list=compact.directory==="build-input-closures"?captureSourceInputInventoryV1(file.value,root).value.files.map(file=>graphStringInternerV1.share("{workspaceRoot}/"+file)):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+captureCargoLockInventoryV1(file.value,root).value.manifest];
          expanded.set(compact.key,list);rememberExpandedClosureV1(root+"\\0"+compact.key,list);
          rememberInventoryRecordV1(root,directory,compact.digest,file.stamp);
        }'''),
])
