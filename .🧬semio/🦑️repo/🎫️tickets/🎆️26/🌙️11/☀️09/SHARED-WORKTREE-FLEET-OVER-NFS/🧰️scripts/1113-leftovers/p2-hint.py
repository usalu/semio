import sys
w=sys.argv[1]
p=w+"/🔁️replica/🟦️.ts"
s=open(p,encoding="utf8").read()
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(s.count(old),old[:90])
    s=s.replace(old,new)
sub(" shared=0;\n constructor(readonly book:ReplicaBookV1,readonly local:boolean){}"," shared=0;\n writes=0;\n constructor(readonly book:ReplicaBookV1,readonly local:boolean){}")
sub(" write(id:number,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):void{\n  this.grow(id+1);"," write(id:number,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):void{\n  this.writes++;this.grow(id+1);")
sub(" clearId(id:number):boolean{\n  if(!this.live(id))return false;\n"," clearId(id:number):boolean{\n  if(!this.live(id))return false;\n  this.writes++;\n")
sub(" clear():void{this.flags.fill(0);"," clear():void{this.writes++;this.flags.fill(0);")
i=s.index(" adopt(columns:Readonly<{flags:Uint8Array;ms:Float64Array")
j=s.index("\n",i)
line=s[i:j]
assert line.endswith("{")
s=s[:j+1]+"  this.writes++;\n"+s[j+1:]
sub("const directoryKeys=new WeakMap<ReplicaV1,Map<string,number>>();","const directoryKeys=new WeakMap<ReplicaV1,{index:Map<string,number>;writes:number}>();")
old=s[s.index("/** 🗂️ How many live local files lie below each directory"):s.index("function countBelow")]
new='''/** 🗂️ How many live local files lie below each directory, counted by the fold key of the directory, so that a file or link at the path of a directory spelled in another case finds the files below it. Built on first use from the registers, kept by `setFile` and `forget`, and rebuilt when any other code wrote the registers meanwhile (the write counter of the `files` table moved: adopt, join, takeover, load, retire); it only decides whether the exact scan of `settleClashes` runs. */
function foldedDirectories(replica:ReplicaV1):Map<string,number>{
 let held=directoryKeys.get(replica);
 if(held===undefined||held.writes!==replica.files.writes){
  held={index:new Map(),writes:replica.files.writes};directoryKeys.set(replica,held);
  for(const id of replica.files.ids())if(replica.files.operationAt(id)!=="delete")countBelow(held.index,replica.book.name(id),1);
 }
 return held.index;
}

'''
s=s.replace(old,new)
old2=s[s.index("function setFile(replica:ReplicaV1"):s.index("const repairRequest=")]
new2='''function setFile(replica:ReplicaV1,path:string,entry:StateEntry):void{
 const id=replica.book.intern(path),held=directoryKeys.get(replica),index=held!==undefined&&held.writes===replica.files.writes?held.index:undefined;
 if(index!==undefined&&replica.files.live(id)&&replica.files.operationAt(id)!=="delete")countBelow(index,path,-1);
 if(index!==undefined&&entry.operation!=="delete")countBelow(index,path,1);
 replica.files.write(id,entry);
 if(held!==undefined)held.writes=index===undefined?-1:replica.files.writes;
 if(entry.operation==="delete")replica.folds.remove(id);else replica.folds.add(id);
 replica.changed.add(filesKey(id));replica.dirty=true;
}

function forget(replica:ReplicaV1,path:string):void{
 const id=replica.book.find(path);
 if(id===undefined||!replica.files.live(id))return;
 const held=directoryKeys.get(replica),index=held!==undefined&&held.writes===replica.files.writes?held.index:undefined;
 if(index!==undefined&&replica.files.operationAt(id)!=="delete")countBelow(index,path,-1);
 replica.folds.remove(id);replica.files.clearId(id);
 if(held!==undefined)held.writes=index===undefined?-1:replica.files.writes;
 replica.changed.add(filesKey(id));replica.dirty=true;
}

'''
s=s.replace(old2,new2)
open(p,"w",encoding="utf8").write(s)
