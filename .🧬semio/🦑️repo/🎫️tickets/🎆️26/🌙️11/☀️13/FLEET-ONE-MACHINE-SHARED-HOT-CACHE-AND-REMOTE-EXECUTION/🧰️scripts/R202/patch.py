import sys,re,json
K=sys.argv[1];OUT=sys.argv[2]
def rd(n): return open(K+'/'+n,encoding='utf8').read()
def wr(n,t): open(OUT+'/'+n,'w',encoding='utf8').write(t)
def sub(t,old,new,count=1):
    assert t.count(old)==count,(old[:60],t.count(old))
    return t.replace(old,new)
pool=rd('pool.base.mjs')
pool=sub(pool,'const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;\n','''const P = GRAPH_FARM_POLICY_V1, MEGABYTE = 1048576;

/** ⏱️ What a job is expected to cost in a farm child, in milliseconds, from its kind and the cost table: a look of up to six owners, a verification, a warm settling, or an inference that grows with the megabytes the cost table knows for its owner (`costMegabytes`). The one source of the order of the queue.
 * @see ./🧪️tests/⏱️queue-order */
export function farmJobExpectedMillisecondsV1(payload, policy = P) {
  const q = policy.queue ?? P.queue, known = Number.isFinite(payload?.costMegabytes) && payload.costMegabytes > 0 ? payload.costMegabytes : 0;
  if (payload?.look === true) return q.lookMilliseconds * Math.max(1, Array.isArray(payload.owners) ? payload.owners.length : 1);
  if (payload?.warm === true) return q.warmMilliseconds;
  if (payload?.observe === true) return q.observeMilliseconds;
  return q.inferMilliseconds + q.perMegabyteMilliseconds * known;
}

/** 🥇 The queued jobs in the order the farm serves them: the shortest expected job first, every millisecond a job has waited counting `agingPerMille` thousandths of a millisecond against its expected cost, so no long owner waits longer than about its own cost; a job that was sent back (`front`: lost, or capped to run alone) stays in front. Equal scores keep their arrival order.
 * @see ./🧪️tests/⏱️queue-order */
export function farmQueueOrderV1(jobs, now, policy = P) {
  const q = policy.queue ?? P.queue, score = job => job.front === true ? -Infinity : job.expected - (now - job.queued) * q.agingPerMille / 1000;
  return jobs.map((job, index) => ({ job, index, score: score(job) })).sort((a, b) => a.score === b.score ? a.index - b.index : a.score < b.score ? -1 : 1).map(row => row.job);
}
''')
pool=sub(pool,'if (job.attempts >= policy.maximumAttempts) settle(job, { lost: why }); else queue.unshift(job); }','if (job.attempts >= policy.maximumAttempts) settle(job, { lost: why }); else { job.front = true; queue.unshift(job); } }')
pool=sub(pool,'else { job.alone = true; job.attempts--; queue.unshift(job); }','else { job.alone = true; job.front = true; job.attempts--; queue.unshift(job); }')
pool=sub(pool,'''    if (closed || disabled !== undefined) return;
    let held = false;''','''    if (closed || disabled !== undefined) return;
    if (queue.length > 1) queue.splice(0, queue.length, ...farmQueueOrderV1(queue, performance.now(), policy));
    let held = false;''')
pool=sub(pool,'queued: performance.now(), dispatched: 0 };\n        queue.push(job);','queued: performance.now(), dispatched: 0, expected: farmJobExpectedMillisecondsV1(payload, policy), front: false };\n        queue.push(job);')
wr('pool.new.mjs',pool)
d=rd('pool.base.d.mts')
d=d.rstrip('\n')+'''
/** ⏱️ The expected cost of a farm job in milliseconds. */
export function farmJobExpectedMillisecondsV1(payload:Readonly<Record<string,unknown>>,policy?:GraphFarmPolicyV1):number;
/** 🥇 The queued jobs, shortest expected first with aging. */
export function farmQueueOrderV1<T extends{readonly expected:number;readonly queued:number;readonly front?:boolean}>(jobs:readonly T[],now:number,policy?:GraphFarmPolicyV1):T[];
'''
wr('pool.new.d.mts',d)
s=rd('poolschema.base.json')
s=sub(s,'        "holdRecheckMilliseconds": 500,\n','        "holdRecheckMilliseconds": 500,\n        "queue": {\n          "lookMilliseconds": 30000,\n          "observeMilliseconds": 60000,\n          "warmMilliseconds": 90000,\n          "inferMilliseconds": 300000,\n          "perMegabyteMilliseconds": 400,\n          "agingPerMille": 1000\n        },\n')
wr('poolschema.new.json',s)
t=rd('take.base.mjs')
t=sub(t,'''/** 🛤️ The two lanes of a take pass: the owners whose record is stored here (`local`, decided by the CPU of this process) and the others (`remote`, decided by the fleet cache and the network), which run beside each other so that an owner the fleet holds does not wait behind thousands of local looks. Every name is in exactly one lane, in its order; an empty lane is left out. */
export function takeLanesV1(names, isPresent) {
  const local = [], remote = [];
  for (const name of names) (isPresent(name) ? local : remote).push(name);
  return [local, remote].filter(lane => lane.length > 0);
}''','''/** 🛤️ The lanes of a take pass: the owners the pools of the fleet say they stored and that are not stored here (`hot`, first: their look finds a record), the owners whose record is stored here (`local`, decided by the CPU of this process) and the others (`remote`, decided by the fleet cache and the network), which run beside each other so that an owner the fleet holds does not wait behind thousands of local looks. Every name is in exactly one lane, in its order; an empty lane is left out.
 * @see ./🧪️tests/🔥️hot-lane */
export function takeLanesV1(names, isPresent, isPublished = () => false) {
  const hot = [], local = [], remote = [];
  for (const name of names) (isPresent(name) ? local : isPublished(name) ? hot : remote).push(name);
  return [hot, local, remote].filter(lane => lane.length > 0);
}''')
wr('take.new.mjs',t)
td=rd('take.base.d.mts')
td=sub(td,'export function takeLanesV1(names: readonly string[], isPresent: (name: string) => boolean): string[][];','export function takeLanesV1(names: readonly string[], isPresent: (name: string) => boolean, isPublished?: (name: string) => boolean): string[][];')
wr('take.new.d.mts',td)
p=rd('pn.base.mjs')
p=sub(p,'const lanes=takeLanesV1(split.light,name=>present.has(name));lookQueue?.start','const published=fleetPublishedNamesV1(process.env.SEMIO_FLEET_STORE_ROOT),lanes=takeLanesV1(split.light,name=>present.has(name),name=>published.has(name));lookQueue?.start')
p=sub(p,'await primeSharedClosuresInChildV1({workspaceRoot,storeDirectory,signal:context.phaseControl?.signal});}','const primed=await primeSharedClosuresInChildV1({workspaceRoot,storeDirectory,signal:context.phaseControl?.signal});try{const seconds=Math.round((performance.now()-since)/1000);for(const language of selectGraphMemoryLanguagesV1(process.env))process.stderr.write("[owners] "+(language==="de"?"gemeinsame Closures vorbereitet in "+seconds+" s: "+(primed?.recipes??"?")+" Rezepte, "+(primed?.derived??"?")+" abgeleitet, "+(primed?.loaded??"?")+" geladen, "+(primed?.stale??"?")+" veraltet":"shared closures primed in "+seconds+" s: "+(primed?.recipes??"?")+" recipes, "+(primed?.derived??"?")+" derived, "+(primed?.loaded??"?")+" loaded, "+(primed?.stale??"?")+" stale")+"\\n");}catch{}}')
p=sub(p,'farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,verdicts:closureVerdictScopeV1()})','farm.prepare({name,root,json,native,pipeline,nativeRoot,selectedInputs,head,cacheKey,verdicts:closureVerdictScopeV1(),costMegabytes:Math.round(enforcement.known?.(name)??0)})')
wr('pn.new.mjs',p)
print('patched')
