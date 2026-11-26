import sys,os
F=sys.argv[1]
t=open(F,encoding="utf8").read()
def once(a,b):
    global t
    assert t.count(a)==1,(t.count(a),a[:70]);t=t.replace(a,b)
once('import { resolve, dirname } from "node:path";\n','import { resolve, dirname } from "node:path";\nimport { descriptorProtocolV1, descriptorRefusalV1 } from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🧷️descriptor/🟦️.ts";\nimport { isConcurrentEditV1 } from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";\n')
once('/** 📄️ Reads a bounded single-owner retirement record through the same no-follow descriptor and stable identity. */\nexport function readNativeImageRetirementFileV1(path:string,maximumBytes:number):Readonly<{bytes:Buffer;stamp:string}>{',
'''const retirementPause=new Int32Array(new SharedArrayBuffer(4));
/** 📄️ Reads a bounded single-owner retirement record through the same no-follow descriptor and stable identity (class `immutable` of the descriptor protocol): a record that changed while it was read is read again after each declared pause, and only a record that keeps changing is refused as a concurrent edit in both languages. */
export function readNativeImageRetirementFileV1(path:string,maximumBytes:number):Readonly<{bytes:Buffer;stamp:string}>{
 const pauses=descriptorProtocolV1().pausesMilliseconds;
 for(let attempt=0;;attempt++){
  try{return readNativeImageRetirementFileOnceV1(path,maximumBytes);}
  catch(error){
   if((error as Error).message!=="Original image retirement record changed")throw error;
   if(attempt>=pauses.length)throw descriptorRefusalV1("immutable",path,{en:"Image retirement record",de:"Abräumvermerk des Abbilds"});
   Atomics.wait(retirementPause,0,0,pauses[attempt]!);
  }
 }
}
function readNativeImageRetirementFileOnceV1(path:string,maximumBytes:number):Readonly<{bytes:Buffer;stamp:string}>{''')
once('catch(error){if((error as Error).message==="Original image retirement record changed")continue;throw error;}','catch(error){if(isConcurrentEditV1(error))continue;throw error;}')
tmp=F+".f75tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,F);print("ok")
