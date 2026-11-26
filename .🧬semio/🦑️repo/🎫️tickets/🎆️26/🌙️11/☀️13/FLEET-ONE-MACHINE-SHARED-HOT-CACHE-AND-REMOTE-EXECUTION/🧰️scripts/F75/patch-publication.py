import sys,os
F=sys.argv[1]
CH="🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/"
t=open(F,encoding="utf8").read()
a='function readRepositoryGraphMaintenanceFileV1(path:string,maximum:number,check:()=>void):NxGraphCurrentMaintenanceFileV1{'
assert t.count(a)==1
t=t.replace(a,'''function readRepositoryGraphMaintenanceFileV1(path:string,maximum:number,check:()=>void):NxGraphCurrentMaintenanceFileV1{
  const pauses=descriptorProtocolV1().pausesMilliseconds;
  for(let attempt=0;;attempt++){
    try{return readRepositoryGraphMaintenanceFileOnceV1(path,maximum,check);}
    catch(error){
      if(!isConcurrentEditV1(error))throw error;
      if(attempt>=pauses.length)throw descriptorRefusalV1("renamed",path,{en:"Current maintenance file",de:"Aktuelle Wartungsdatei"});
      Atomics.wait(maintenancePause,0,0,pauses[attempt]!);
    }
  }
}
const maintenancePause=new Int32Array(new SharedArrayBuffer(4));
function readRepositoryGraphMaintenanceFileOnceV1(path:string,maximum:number,check:()=>void):NxGraphCurrentMaintenanceFileV1{''')
b='throw Error("Exact opened current file identity required")'
assert t.count(b)==1
t=t.replace(b,'throw concurrentEditV1("descriptor",path,"Exact opened current file identity required")')
a2='/** 📄️ Reads only one bounded ordinary descriptor and preserves primary plus close failures. */\n'
assert t.count(a2)==1
t=t.replace(a2,'/** 📄️ Reads only one bounded ordinary descriptor and preserves primary plus close failures; a file replaced by rename between its name and its descriptor (class `renamed` of the descriptor protocol) is opened by name again, bounded by the declared pauses. */\n')
rel=lambda target:os.path.relpath(CH+target,os.path.dirname(F)).replace(os.sep,"/")
imp='import{descriptorProtocolV1,descriptorRefusalV1}from"%s";\nimport{concurrentEditV1,isConcurrentEditV1}from"%s";\n'%(rel("🧷️descriptor/🟦️.ts"),rel("🟦️.ts"))
if 'descriptorRefusalV1}from' not in t:t=imp+t
open(F+".f75tmp","w",encoding="utf8").write(t);os.replace(F+".f75tmp",F);print("ok")
