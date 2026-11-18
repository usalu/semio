import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage1b");SR=B+"🧠️memory/📋️slice-report/"
s.edit(SR+"🟨️.mjs",[
(' const taken=[];let previous=0;',' const taken=[];let previous=0,lastCollect=-Infinity;',1),
(' const timer=setInterval(()=>{probe();writeSliceReportV1(root,{reason:"interval",heap});},intervalMilliseconds);',
 ''' const collect=()=>{
  const used=process.memoryUsage().heapUsed/MEGABYTE,now=performance.now();
  if(used<P.collectMinimumHeapMegabytes||now-lastCollect<P.collectEveryMilliseconds)return;
  lastCollect=now;const before=process.memoryUsage();
  try{globalThis.Bun?.gc?.(true);}catch{}
  const after=process.memoryUsage();sources.set("afterCollect",()=>({beforeHeapUsed:megabytes(before.heapUsed),beforeExternal:megabytes(before.external),heapUsed:megabytes(after.heapUsed),external:megabytes(after.external),seconds:Math.round((performance.now()-now)/100)/10}));
 };
 const timer=setInterval(()=>{probe();collect();writeSliceReportV1(root,{reason:"interval",heap});},intervalMilliseconds);''',1),
])
s.edit(SR+"🧬️schema/🔣️.json",[('"collectionSample": 16,','"collectionSample": 16,\n        "collectMinimumHeapMegabytes": 1000,\n        "collectEveryMilliseconds": 120000,',1)])
s.finish([SR+"🧪️tests/🟦️.ts"],"- Step 1b: the report collects once (at most every 120 s, from 1000 MB heap) and writes `afterCollect` (heap before and after), so garbage and live data are told apart.")
