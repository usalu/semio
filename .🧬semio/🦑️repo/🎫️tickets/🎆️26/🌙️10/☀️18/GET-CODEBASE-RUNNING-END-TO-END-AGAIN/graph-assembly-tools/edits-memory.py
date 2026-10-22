import sys,os
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
apply(L+"/🏗️build/🧠️memory/🟨️.mjs",[
 ('import v8 from "node:v8";\n',''),
 ('sample:()=>{const statistics=v8.getHeapStatistics();return statistics.used_heap_size+statistics.external_memory;},','sample:()=>{const memory=process.memoryUsage();return memory.heapUsed+memory.external;},'),
])
