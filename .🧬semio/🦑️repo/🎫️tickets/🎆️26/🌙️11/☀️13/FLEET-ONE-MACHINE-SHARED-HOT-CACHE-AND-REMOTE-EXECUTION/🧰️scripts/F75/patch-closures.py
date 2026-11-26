import re,sys,os
F=sys.argv[1]
t=open(F,encoding="utf8").read()
orig=t
imp='import{descriptorFactsOfStampV1,descriptorFactsV1,descriptorRefusalV1,settleDescriptorV1,type DescriptorLabelV1}from"../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧷️descriptor/🟦️.ts";\n'
anchor='import{directoryChangeV1,runClosureReadCohortsV1}from"./📡️session/🟦️.ts";\n'
assert t.count(anchor)==1
t=t.replace(anchor,imp+anchor)
labels={
 "Closure index descriptor changed":("Closure index","Abschlussindex"),
 "Original retiring closure descriptor changed":("Retiring closure leaf","Abzuräumendes Abschlussblatt"),
 "Original retiring closure identity changed":("Retiring closure leaf","Abzuräumendes Abschlussblatt"),
 "Original group retirement descriptor changed":("Group retirement leaf","Gruppen-Abräumblatt"),
 "Original terminal retirement intent changed":("Terminal retirement intent","Abschließende Abräumabsicht"),
}
pat=re.compile(r'if\(stamp\(fstatSync\(fd,\{bigint:true\}\)\)!==row\.stamp(\|\|physicalRow\((\w+),row\.path,row\.kind\)\.stamp!==row\.stamp)?\)throw Error\("([^"]+)"\);')
count=0
def sub(m):
    global count
    text=m.group(3)
    if text not in labels: raise SystemExit("unknown "+text)
    if text=="Closure index descriptor changed": return m.group(0)
    en,de=labels[text];count+=1
    p=m.group(2)
    if p: return f'settleLeaf({p},row,fd,{{en:"{en}",de:"{de}"}},true);'
    return f'settleLeaf(proof,row,fd,{{en:"{en}",de:"{de}"}},false);'
t=pat.sub(sub,t)
print("leaf sites",count)
helper='''/** 🧷️ Verifies one opened closure leaf against the stamp its proof row recorded: a leaf nobody may touch while it is open (class `immutable` of the descriptor protocol), observed again a bounded number of times and then refused as a concurrent edit in both languages. */
function settleLeaf(proof:Proof,row:Row,fd:number,label:DescriptorLabelV1,named:boolean){settleDescriptorV1({kind:"immutable",path:row.path,label,expected:descriptorFactsOfStampV1(row.stamp),descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true}))});if(named&&physicalRow(proof,row.path,row.kind).stamp!==row.stamp)throw descriptorRefusalV1("immutable",row.path,label);}
const closureIndexLabelV1:DescriptorLabelV1={en:"Closure index",de:"Abschlussindex"};
/** 📒️ Reads one closure index through a descriptor. A group's member index is append-only (class `append-only`): a sibling's append or a metadata-only touch between the lstat of the proof and the open adopts the fresh identity of the same inode and reads again, bounded; every other index is immutable. */
function smallBytes(proof:Proof,row:Row){
 check(proof);if(row.bytes>P.maximumIndexBytes)throw Error("Bounded closure index required");
 const appendOnly=row.kind==="member-index",adopt=(facts:ReturnType<typeof descriptorFactsV1>)=>{const fresh=physicalRow(proof,row.path,row.kind);if(descriptorFactsOfStampV1(fresh.stamp).ino!==facts.ino||fresh.bytes<row.bytes)throw descriptorRefusalV1("append-only",row.path,closureIndexLabelV1);row.stamp=fresh.stamp;row.bytes=fresh.bytes;row.allocatedBytes=fresh.allocatedBytes;row.modified=fresh.modified;if(row.bytes>P.maximumIndexBytes)throw Error("Bounded closure index required");};
 for(let attempt=0;;attempt++){
  const fd=openSync(row.path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));let failed=false,again=false;
  try{
   const settle=()=>settleDescriptorV1({kind:appendOnly?"append-only":"immutable",path:row.path,label:closureIndexLabelV1,expected:descriptorFactsOfStampV1(row.stamp),descriptor:()=>descriptorFactsV1(fstatSync(fd,{bigint:true}))});
   const first=settle();if(first.state!=="same")adopt(first.facts);
   proof.work.bytes+=row.bytes;if(proof.work.bytes>P.maximumProofBytes)throw Error("Bounded closure proof read bytes required");
   const bytes=Buffer.alloc(row.bytes);for(let at=0;at<bytes.length;){check(proof);const count=readSync(fd,bytes,at,Math.min(P.chunkBytes,bytes.length-at),at);if(!count)throw Error("Closure index shortened");at+=count;}
   if(appendOnly){const last=settle();if(last.state!=="same"){if(attempt>=3)throw descriptorRefusalV1("append-only",row.path,closureIndexLabelV1);adopt(last.facts);again=true;}}
   else if(readSync(fd,Buffer.alloc(1),0,1,bytes.length))throw descriptorRefusalV1("immutable",row.path,closureIndexLabelV1);
   else settle();
   if(!again){if(physicalRow(proof,row.path,row.kind).stamp!==row.stamp)throw descriptorRefusalV1(appendOnly?"append-only":"immutable",row.path,closureIndexLabelV1);return bytes;}
  }catch(error){failed=true;throw error;}finally{try{closeSync(fd);}catch(error){if(!failed)throw error;}}
 }
}
'''
a=t.index("function smallBytes(proof:Proof,row:Row){")
b=t.index("\nfunction pairs(proof:Proof)")
t=t[:a]+helper.rstrip("\n")+t[b:]
tmp=F+".f75tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,F)
print("ok")
