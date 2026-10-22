import sys,os
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
apply(L+"/🏗️build/⌨️command/⚙️runtime/🟦️.ts",[
 ('else console.log(JSON.stringify(result,null,2));\n      return;','else{const {writeJsonStreamV1}=await import("../../🕸️graph/🕰️generations/🌊️json-stream/🟨️.mjs");await writeJsonStreamV1(result,text=>new Promise<void>((resolve,reject)=>{process.stdout.write(text,error=>error?reject(error):resolve());}),{signal:invocation.control.signal});process.stdout.write("\\n");}\n      return;'),
])
