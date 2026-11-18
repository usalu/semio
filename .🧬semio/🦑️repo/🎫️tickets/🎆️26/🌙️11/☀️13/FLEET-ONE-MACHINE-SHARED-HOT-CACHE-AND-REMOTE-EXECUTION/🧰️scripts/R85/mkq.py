import shutil
R="/Users/ueli/Documents/semio/"
RD="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
files={"A":RD+"🟦️.ts","B":RD+"🧪️tests/🟦️.ts"}
open("paths.txt","w",encoding="utf8").write("\n".join(k+" "+v for k,v in files.items())+"\n")
for k,v in files.items():shutil.copyfile(R+v,"base."+k);shutil.copyfile(R+v,"new."+k)
def edit(k,pairs):
    p="new."+k;t=open(p,encoding="utf8").read()
    for a,b in pairs:
        assert t.count(a)==1,(k,a[:90],t.count(a));t=t.replace(a,b)
    open(p,"w",encoding="utf8").write(t)
edit("A",[
 ('if(tracker.observe(recent))stallAt=Date.now();','if(tracker.observe(recent)){stallAt=Date.now();progressAt=Date.now();}'),
 ("/** 🧾️ The lines of a refusal","""/** 💾️ Writes a file in the store so a reader never sees half of it: a temporary file renamed over the target; where the rename is refused (Windows with an open reader, a network share) the text is written to the target directly and the temporary file is removed, so a mirror never goes stale and no `.tmp` is left. */
export function readyWriteStoreFileV1(file:string,text:string,rename:(from:string,to:string)=>void=renameSync):void{
 const temporary=`${file}.${process.pid}.tmp`;
 writeFileSync(temporary,text);
 try{rename(temporary,file);}catch{try{writeFileSync(file,text);}catch{}try{rmSync(temporary,{force:true});}catch{}}
}

/** 🧾️ The lines of a refusal"""),
 ("file=join(dir,`${P.mirrorPrefix}${stage}.txt`),temporary=`${file}.${process.pid}.tmp`;\n    mkdirSync(dir,{recursive:true});writeFileSync(temporary,text);renameSync(temporary,file);","file=join(dir,`${P.mirrorPrefix}${stage}.txt`);\n    mkdirSync(dir,{recursive:true});readyWriteStoreFileV1(file,text);"),
 ("if(bytes.length<=P.sliceReport.fileBytes){const temporary=`${target}.${process.pid}.tmp`;writeFileSync(temporary,bytes);renameSync(temporary,target);}","if(bytes.length<=P.sliceReport.fileBytes)readyWriteStoreFileV1(target,bytes.toString(\"utf8\"));"),
 ("file=join(dir,P.timingsName),temporary=`${file}.${process.pid}.tmp`;mkdirSync(dir,{recursive:true});writeFileSync(temporary,JSON.stringify(doc));renameSync(temporary,file);","file=join(dir,P.timingsName);mkdirSync(dir,{recursive:true});readyWriteStoreFileV1(file,JSON.stringify(doc));")])
t=open("new.B",encoding="utf8").read()
assert t.count("createReadyStatusV1,")==1
t=t.replace("createReadyStatusV1,","createReadyStatusV1,readyWriteStoreFileV1,",1)
t+=open("tail.ts",encoding="utf8").read()
open("new.B","w",encoding="utf8").write(t)
