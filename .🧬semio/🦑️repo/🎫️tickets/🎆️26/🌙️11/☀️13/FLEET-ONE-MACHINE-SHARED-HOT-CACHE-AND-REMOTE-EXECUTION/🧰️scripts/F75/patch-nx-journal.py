import sys,os
F=sys.argv[1]
CH="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/"
t=open(F,encoding="utf8").read()
a='function bytes(path:string,maximum:number,links:number|bigint=1){check();const before=regular(path,maximum,links)'
assert t.count(a)==1
t=t.replace(a,'''/** 🧷️ Reads one graph publication file that is replaced atomically by rename (class `renamed` of the descriptor protocol): a file replaced between the name and the descriptor is opened by name again, bounded by the declared pauses, and only a file that keeps being replaced is refused as a concurrent edit in both languages. */
function bytes(path:string,maximum:number,links:number|bigint=1){
  const pauses=descriptorProtocolV1().pausesMilliseconds;
  for(let attempt=0;;attempt++){
    try{return bytesOnce(path,maximum,links);}
    catch(error){
      if(!isConcurrentEditV1(error))throw error;
      if(attempt>=pauses.length)throw descriptorRefusalV1("renamed",path,{en:"Graph publication",de:"Graph-Veröffentlichung"});
      Atomics.wait(publicationPause,0,0,pauses[attempt]!);
    }
  }
}
const publicationPause=new Int32Array(new SharedArrayBuffer(4));
function bytesOnce(path:string,maximum:number,links:number|bigint=1){check();const before=regular(path,maximum,links)''')
assert t.count('throw Error("Graph publication file changed")')==2
t=t.replace('throw Error("Graph publication file changed")','throw concurrentEditV1("descriptor",path,"Graph publication file changed")')
import re
rel=lambda target:os.path.relpath(CH+target,os.path.dirname(F)).replace(os.sep,"/")
imp='import{descriptorProtocolV1,descriptorRefusalV1}from"%s";\nimport{concurrentEditV1,isConcurrentEditV1}from"%s";\n'%(rel("🧷️descriptor/🟦️.ts"),rel("🟦️.ts"))
t=imp+t
open(F+".f75tmp","w",encoding="utf8").write(t);os.replace(F+".f75tmp",F);print("ok")
