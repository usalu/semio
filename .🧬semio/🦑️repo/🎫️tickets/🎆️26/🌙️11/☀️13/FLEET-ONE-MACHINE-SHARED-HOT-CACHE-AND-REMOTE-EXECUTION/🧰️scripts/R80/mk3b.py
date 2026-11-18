import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage3b");SR=B+"🧠️memory/📋️slice-report/"
s.edit(SR+"🟨️.mjs",[
(' const taken=[];let previous=0;',' const taken=[];let previous=0,collected=false;',1),
(' const timer=setInterval(()=>{probe();',''' const collect=()=>{
  if(collected||process.memoryUsage().heapUsed/MEGABYTE<P.collectMinimumHeapMegabytes)return;
  collected=true;const started=performance.now();
  try{globalThis.Bun?.gc?.(true);}catch{}
  const usage=process.memoryUsage(),stats=heap?.();
  sources.set("afterCollect",()=>({heapUsed:megabytes(usage.heapUsed),external:megabytes(usage.external),seconds:Math.round((performance.now()-started)/100)/10,...stats===undefined?{}:{types:Object.entries(stats.objectTypeCounts??{}).sort((a,b)=>b[1]-a[1]).slice(0,P.topTypes).map(([type,count])=>({type,count}))}}));
 };
 const timer=setInterval(()=>{probe();collect();''',1),
])
s.edit(SR+"🧬️schema/🔣️.json",[('"collectionSample": 16,','"collectionSample": 16,\n        "collectMinimumHeapMegabytes": 2500,',1)])
s.finish([SR+"🧪️tests/🟦️.ts"],"- Step 2c (diagnostic): ONE full collection per worker at 2500 MB heap, with the object types after it (`afterCollect`), to tell live promises and frames from garbage.")
