p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🧹️prune/🟦️.ts"
s=open(p,encoding="utf8").read()
a=s.index('test("the due pass is claimed through a stamp')
b=s.index('test("pruning never throws')
new='''test("the due pass retires the whole backlog in one pass with bounded slices and is claimed through a stamp: concurrent or early callers skip it, an aborted pass comes due sooner than a finished registry",async()=>{
 const world=build(),policy=leaseOrderPolicyV1({}),tight={...policy,registry:{...policy.registry,prune:{...policy.registry.prune,maximumFolders:1,budgetMilliseconds:1000,idleMilliseconds:0,intervalMilliseconds:100000,backlogMilliseconds:1000}}};
 try{
  const first=await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now);
  expect(first?.more).toBe(false);
  expect(first?.examined).toBe(world.folders.length);
  expect(world.folders.filter(row=>existsSync(row.name)).map(row=>row.name).sort()).toEqual(survivors(world.folders));
  expect(JSON.parse(readFileSync(join(world.registry,"prune.json"),"utf8")).more).toBe(false);
  expect(await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+10)).toBeUndefined();
  expect(await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+1001)).toBeUndefined();
  expect(await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+100001)).toBeDefined();
 }finally{rmSync(world.root,{recursive:true,force:true});}
});

test("an aborted pass leaves the stamp saying more so the next pass resumes soon, and slices yield to the event loop",async()=>{
 const world=build(),policy=leaseOrderPolicyV1({}),tight={...policy,registry:{...policy.registry,prune:{...policy.registry.prune,maximumFolders:1,budgetMilliseconds:1000,idleMilliseconds:0,intervalMilliseconds:100000,backlogMilliseconds:1000}}};
 try{
  const controller=new AbortController();let ticks=0;
  const timer=setInterval(()=>{ticks++;},0);
  const pending=pruneLeaseRegistryIfDueV1(world.registry,tight,world.now,controller.signal);
  controller.abort();
  const cut=await pending;clearInterval(timer);
  expect(cut?.more).toBe(true);
  expect(JSON.parse(readFileSync(join(world.registry,"prune.json"),"utf8")).more).toBe(true);
  const rest=await pruneLeaseRegistryIfDueV1(world.registry,tight,world.now+1001);
  expect(rest?.more).toBe(false);
  expect(world.folders.filter(row=>existsSync(row.name)).map(row=>row.name).sort()).toEqual(survivors(world.folders));
 }finally{rmSync(world.root,{recursive:true,force:true});}
});

'''
s=s[:a]+new+s[b:]
open(p,"w",encoding="utf8").write(s)
