// [derive-g] memory/time budget law of the environment-move scenario, checked on the log of one `delta.mjs` run (SLICE=0, one touched digest row, LOADMARKS=1 optional). Exit 1 when a budget is exceeded. usage: bun budget-law.mjs <log> 
import{readFileSync}from"node:fs";
const rows=readFileSync(process.argv[2],"utf8").split("\n").filter(line=>line.startsWith("[DEBUG] {\"label\"")).map(line=>JSON.parse(line.slice(8)));
const at=label=>rows.find(row=>row.label===label||row.label.startsWith(label));const rss=label=>at(label)?.rss;
const done=JSON.parse(readFileSync(process.argv[2],"utf8").split("\n").find(line=>line.startsWith("{\"wallMs\""))??"{}");
const budgets=[
 ["re-edge stretch keeps the process flat (RSS growth part:testedges -> part:edges, MB)",rss("part:edges")-rss("part:testedges"),200],
 ["test discovery after a move (tests.projects wall, ms; the old code needed 69,000)",rows.find(row=>row.label==="tests.projects:done")?.ms,15000],
 ["re-edge wall (part:edges, ms; the old code needed 24,000-48,000)",at("part:edges")?.ms,8000],
 ["shape keeps the process flat (RSS growth, MB; the old code grew 850)",(rss("load:shaped")??rss("build:compute-start"))-(rss("load:before-shape")??rss("part:captured")),300],
 ["peak RSS of the whole scenario (MB; the old code peaked at 4660-5000)",done.peakRssMB,4700],
];
let failed=0;for(const[name,value,limit]of budgets){const ok=Number.isFinite(value)&&value<=limit;if(!ok)failed++;console.log((ok?"PASS ":"FAIL ")+name+": "+value+" <= "+limit);}
process.exit(failed?1:0);
