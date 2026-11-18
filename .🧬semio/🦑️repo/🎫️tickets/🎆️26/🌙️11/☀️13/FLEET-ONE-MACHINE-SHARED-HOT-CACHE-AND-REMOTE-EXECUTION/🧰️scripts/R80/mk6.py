import sys;sys.path.insert(0,__import__("os").path.dirname(__import__("os").path.abspath(__file__)))
from edit import *
s=Stage(SCR+"stage6");PN=B+"🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
s.edit(PN,[
('const stagesV1 = new Map();','''const stagesV1 = new Map();
const projectSizesV1 = { count: 0, bytes: 0, top: [] };
/** 📏️ Notes the real JSON size of one resolved project for the slice memory report: the total and the five largest (a resolved project is what a cold part keeps per owner). */
const noteProjectSizeV1 = (name, value) => { let bytes = 0; try { bytes = JSON.stringify(value).length; } catch {} projectSizesV1.count++; projectSizesV1.bytes += bytes; projectSizesV1.top.push({ name: String(name).slice(0, 80), bytes }); projectSizesV1.top.sort((a, b) => b.bytes - a.bytes); if (projectSizesV1.top.length > 5) projectSizesV1.top.length = 5; };''',1),
('    result.projects[name].targets={...inactive,...result.projects[name].targets};','    result.projects[name].targets={...inactive,...result.projects[name].targets};noteProjectSizeV1(name,result.projects[name]);',1),
('return[{name:"owners:results.projects",entries:all.length,sample:()=>all.slice(0,16)},','jobsBytes??=jobs.reduce((sum,job)=>sum+JSON.stringify(job.json).length,0);return[{name:"owners:results.projects",entries:all.length,sample:()=>all.slice(0,16)},{name:"owners:jobs (real JSON)",entries:jobs.length,bytes:jobsBytes},{name:"owners:resolvedProjects (real JSON of "+projectSizesV1.count+" finished: "+projectSizesV1.top.map(row=>row.name.split("/").pop()+" "+row.bytes).join(", ")+")",entries:projectSizesV1.count,bytes:projectSizesV1.bytes},',1),
('  const collectionsOff=registerSliceCollectionsV1("owners",','  let jobsBytes;const collectionsOff=registerSliceCollectionsV1("owners",',1),
])
s.finish([B+"🧠️memory/📋️slice-report/🧪️tests/🟦️.ts",B+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/📥️take/🟦️.ts"],"- Step M1 (diagnostic): the real JSON size of every resolved project (total and five largest) and of the jobs is reported as collections.")
