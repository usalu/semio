p="work/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
s=open(p,encoding="utf8").read()
def rep(old,new,count=1):
    global s
    assert s.count(old)==count,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('import { openProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";\n','import { openProjectFarmV1 } from "./🏊️project-farm/🟨️.mjs";\nimport { ownerRecordKeyV1, ownerRecordsPresentTextV1, ownerRecordsPresentV1 } from "./📋️present/🟨️.mjs";\n')
rep('function ownerCacheKeyV1(root) { return createHash("sha256").update(root).digest("hex").slice(0, 24); }','function ownerCacheKeyV1(root) { return ownerRecordKeyV1(root); }')
rep('''/** 🧭️ The order owners start in: the owners the cost table says need a solo grant (`alone`) last, before them the ones recorded as slow (`slow`), the held requests first (`rank`) within each class; the order inside a class is kept. */
export function orderOwnerJobsV1(jobs, { alone = new Set(), slow = new Set(), rank } = {}) {
  const tier = job => alone.has(job.name) ? 2 : slow.has(job.name) ? 1 : 0;
  return jobs.sort((a, b) => tier(a) - tier(b) || (rank === undefined ? 0 : rank(a.name) - rank(b.name)));
}''','''/** 🧭️ The order owners start in: the owners the cost table says need a solo grant (`alone`) last, before them the ones recorded as slow (`slow`), the held requests first (`rank`) within each class, then the owners whose stored record is present (`present`, names) before those without one, so the cheap ones finish and publish first; the order inside a class is kept. */
export function orderOwnerJobsV1(jobs, { alone = new Set(), slow = new Set(), rank, present } = {}) {
  const tier = job => alone.has(job.name) ? 2 : slow.has(job.name) ? 1 : 0, cold = job => present === undefined || present.has(job.name) ? 0 : 1;
  return jobs.sort((a, b) => tier(a) - tier(b) || (rank === undefined ? 0 : rank(a.name) - rank(b.name)) || cold(a) - cold(b));
}''')
rep('orderOwnerJobsV1(jobs,{alone,slow:new Set(enforcement.slow?.(jobs.map(job=>job.name))??[]),rank});const lightEnd=','''const census=ownerRecordsPresentV1(join(workspaceDataDirectory(workspaceRoot),"np"),jobs.map(job=>job.json.root)),present=new Set(jobs.filter(job=>census.present.has(job.json.root)).map(job=>job.name));try{for(const language of selectGraphMemoryLanguagesV1(process.env))process.stderr.write("[owners] "+ownerRecordsPresentTextV1(language,census)+"\\n");}catch{}orderOwnerJobsV1(jobs,{alone,slow:new Set(enforcement.slow?.(jobs.map(job=>job.name))??[]),rank,present});const lightEnd=''')
open(p,"w",encoding="utf8").write(s)
